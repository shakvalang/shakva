use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use skc_lexer::Cursor;

use indexmap::IndexMap;
use skc_ir::source::{BytePos, SourceFile, Span};
use thiserror::Error;

/// Thread-safe registry of loaded source files.
///
/// All files share one global byte-offset space ([`BytePos`]): each file gets a
/// disjoint range, so a single `BytePos` pins down both a file and an offset
/// within it.
#[derive(Debug)]
pub struct SourceMap {
    inner: RwLock<SourceMapImpl>,
}

impl SourceMap {
    /// Creates an empty source map.
    pub fn new() -> Self {
        let src_map_impl = SourceMapImpl::new();
        let inner = RwLock::new(src_map_impl);
        Self { inner }
    }

    /// Loads the file at `path` and returns it, or returns the already-loaded
    /// file if it was added before.
    ///
    /// Paths are matched verbatim - canonicalize first if relative paths or
    /// symlinks should dedup.
    pub fn add(&self, path: &Path) -> SourceMapResult<Arc<SourceFile>> {
        if let Some(src_file_ptr) = self.get_by_path(path) {
            return Ok(src_file_ptr);
        }

        Ok(self.register(read_source_file(path)?))
    }

    /// Returns the file whose range contains `pos`, or `None` if none does
    /// (a position past the end, or the gap byte between two files).
    pub fn get_by_pos(&self, pos: BytePos) -> Option<Arc<SourceFile>> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        let n = inner
            .files
            .as_slice()
            .partition_point(|_, f| f.span.lo <= pos);
        inner
            .files
            .get_index(n.checked_sub(1)?)
            .map(|(_, f)| f)
            .filter(|f| pos < f.span.hi)
            .cloned()
    }

    /// Returns the already-loaded file for `path` (matched verbatim), or
    /// `None`. Does not touch disk - use [`add`](Self::add) to load.
    pub fn get_by_path(&self, path: &Path) -> Option<Arc<SourceFile>> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.files.get(path).cloned()
    }

    /// Resolve a global span to its owning file and the file-local byte range
    /// `[lo, hi)` into that file's `contents`.
    ///
    /// Unlike [`get_by_pos`](Self::get_by_pos), a span starting exactly at a
    /// file's end (`span.lo == file.span.hi`) resolves to that file with a
    /// zero-width range at `contents.len()`, instead of falling off the end.
    pub fn resolve_span(&self, span: Span) -> Option<(Arc<SourceFile>, Range<usize>)> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        let n = inner
            .files
            .as_slice()
            .partition_point(|_, f| f.span.lo <= span.lo);
        let (_, file) = inner.files.get_index(n.checked_sub(1)?)?;

        // `<=`, not `<`: accept the end-of-file boundary position.
        if span.lo > file.span.hi {
            return None;
        }

        let base = file.span.lo;
        let lo = (span.lo - base).to_usize();
        // `.min(len)` defends against a malformed span that overshoots the
        // file.
        let hi = (span.hi - base).to_usize().min(file.contents.len());
        Some((file.clone(), lo..hi))
    }

    fn register(&self, mut src_file: SourceFile) -> Arc<SourceFile> {
        let mut inner = self.inner.write().expect("unable to acquire write lock");
        if let Some(src_file_ptr) = inner.files.get(&src_file.path).cloned() {
            // Somebody added file while we were waiting for write lock
            return src_file_ptr;
        }

        let lo = inner.bytes_len;
        src_file.span = Span::new(lo, lo + src_file.span.hi);
        for line_pos in &mut src_file.line_starts {
            *line_pos += lo;
        }
        inner.bytes_len = src_file.span.hi + BytePos(1);

        let src_file_ptr = Arc::new(src_file);

        inner
            .files
            .insert_full(src_file_ptr.path.clone(), src_file_ptr.clone());

        src_file_ptr
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct SourceMapImpl {
    files: IndexMap<PathBuf, Arc<SourceFile>>,
    bytes_len: BytePos,
}

impl SourceMapImpl {
    fn new() -> Self {
        Self {
            files: IndexMap::new(),
            bytes_len: BytePos(0),
        }
    }
}

impl Default for SourceMapImpl {
    fn default() -> Self {
        Self::new()
    }
}

type SourceMapResult<T> = Result<T, SourceMapError>;

#[derive(Error, Debug)]
pub enum SourceMapError {
    #[error("unable to read `{path}`: {source}")]
    UnableToReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
}

fn read_source_file(path: &Path) -> SourceMapResult<SourceFile> {
    let contents =
        std::fs::read_to_string(path).map_err(|err| SourceMapError::UnableToReadFile {
            path: path.to_path_buf(),
            source: err,
        })?;
    Ok(source_file_from_contents(path.to_path_buf(), contents))
}

/// Build a SourceFile with a *file-local* span `[0, len)` from in-memory
/// contents.
fn source_file_from_contents(path: PathBuf, contents: String) -> SourceFile {
    let line_starts = compute_line_starts(&contents);
    let hi = BytePos::from_usize(contents.len());
    SourceFile::new(path, contents, Span::new(BytePos(0), hi), line_starts)
}

/// Byte offsets of each line start. `lines[0]` is always `BytePos(0)` so every
/// file (even empty) has line 0. A new line starts after each terminator.
fn compute_line_starts(contents: &str) -> Vec<BytePos> {
    let tokens = Cursor::tokenize(contents);
    let mut pos = BytePos(0);
    let mut lines = vec![pos];

    for tok in tokens {
        pos += BytePos(tok.len);
        if tok.is_newline() {
            lines.push(pos);
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_source_file(path: &str, contents: &str) -> SourceFile {
        source_file_from_contents(PathBuf::from(path), contents.to_string())
    }

    fn bytes_len(map: &SourceMap) -> BytePos {
        map.inner.read().unwrap().bytes_len
    }

    fn file_count(map: &SourceMap) -> usize {
        map.inner.read().unwrap().files.len()
    }

    fn check_line_starts(input: &str, expected: &[u32]) {
        let expected: Vec<BytePos> = expected.iter().copied().map(BytePos).collect();
        assert_eq!(compute_line_starts(input), expected, "input = {input:?}");
    }

    #[test]
    fn line_starts_no_newline() {
        check_line_starts("", &[0]); // empty file still has line 0
        check_line_starts("abc", &[0]); // no trailing newline -> one line
        check_line_starts("ффф", &[0]); // multi-byte, no newline
    }

    #[test]
    fn line_starts_lf() {
        check_line_starts("a\nb", &[0, 2]);
        check_line_starts("a\n", &[0, 2]); // trailing newline -> empty last line
        check_line_starts("\n\n", &[0, 1, 2]); // consecutive blank lines
    }

    #[test]
    fn line_starts_crlf_is_single_break() {
        check_line_starts("a\r\nb", &[0, 3]); // \r\n counts as one break (+2)
        check_line_starts("a\r\n", &[0, 3]);
    }

    #[test]
    fn line_starts_lone_cr() {
        check_line_starts("a\rb", &[0, 2]); // lone \r breaks (matches the lexer)
        check_line_starts("a\r", &[0, 2]);
        check_line_starts("a\n\rb", &[0, 2, 3]); // LF then lone CR = two breaks
    }

    #[test]
    fn line_starts_multibyte_advances_by_utf8_len() {
        check_line_starts("ф\n", &[0, 3]); // 'ф' is 2 bytes
        check_line_starts("😀\n", &[0, 5]); // '😀' is 4 bytes
    }

    #[test]
    fn source_file_local_span_and_lines() {
        let sf = make_source_file("a.sk", "ab\ncd");
        assert_eq!(sf.span, Span::new(BytePos(0), BytePos(5)));
        assert_eq!(sf.line_starts, vec![BytePos(0), BytePos(3)]);
    }

    #[test]
    fn source_file_empty() {
        let sf = make_source_file("e.sk", "");
        assert_eq!(sf.span, Span::new(BytePos(0), BytePos(0)));
        assert_eq!(sf.line_starts, vec![BytePos(0)]);
    }

    #[test]
    fn span_hi_equals_byte_len_for_multibyte() {
        // The regression the `+1`-per-char version failed: `hi` is a *byte*
        // offset.
        for contents in ["ф\n", "😀\n", "ффф"] {
            let sf = make_source_file("m.sk", contents);
            assert_eq!(sf.span.hi, BytePos::from_usize(contents.len()));
        }
    }

    #[test]
    fn register_single_file() {
        let map = SourceMap::new();
        let sf = map.register(make_source_file("a.sk", "abc"));
        assert_eq!(sf.span, Span::new(BytePos(0), BytePos(3)));
        assert_eq!(sf.line_starts, vec![BytePos(0)]);
        assert_eq!(file_count(&map), 1);
        assert_eq!(bytes_len(&map), BytePos(4)); // len 3 + gap 1
    }

    #[test]
    fn register_two_files_reserves_gap() {
        let map = SourceMap::new();
        let a = map.register(make_source_file("a.sk", "abc")); // [0, 3)
        let b = map.register(make_source_file("b.sk", "de")); // [4, 6)
        assert_eq!(a.span, Span::new(BytePos(0), BytePos(3)));
        assert_eq!(b.span, Span::new(BytePos(4), BytePos(6))); // starts at first.hi + 1
        assert_eq!(bytes_len(&map), BytePos(7)); // 6 + gap 1
        assert_eq!(file_count(&map), 2);
    }

    #[test]
    fn register_rebases_line_starts() {
        let map = SourceMap::new();
        map.register(make_source_file("a.sk", "xy")); // [0, 2), bytes_len -> 3
        let b = map.register(make_source_file("b.sk", "a\nb")); // lo = 3
        assert_eq!(b.span, Span::new(BytePos(3), BytePos(6)));
        assert_eq!(b.line_starts, vec![BytePos(3), BytePos(5)]); // local [0, 2] + lo
    }

    #[test]
    fn register_same_path_is_idempotent() {
        let map = SourceMap::new();
        let first = map.register(make_source_file("a.sk", "abc"));
        let second = map.register(make_source_file("a.sk", "different contents"));
        assert!(Arc::ptr_eq(&first, &second)); // deduped, original kept
        assert_eq!(file_count(&map), 1);
        assert_eq!(bytes_len(&map), BytePos(4)); // advanced only once
    }

    fn two_file_map() -> (SourceMap, Arc<SourceFile>, Arc<SourceFile>) {
        let map = SourceMap::new();
        let a = map.register(make_source_file("a.sk", "abc")); // [0, 3)
        let b = map.register(make_source_file("b.sk", "de")); // [4, 6)
        (map, a, b)
    }

    #[test]
    fn get_by_pos_inside_files() {
        let (map, a, b) = two_file_map();
        assert!(Arc::ptr_eq(&map.get_by_pos(BytePos(0)).unwrap(), &a)); // start of a
        assert!(Arc::ptr_eq(&map.get_by_pos(BytePos(2)).unwrap(), &a)); // inside a
        assert!(Arc::ptr_eq(&map.get_by_pos(BytePos(4)).unwrap(), &b)); // start of b
        assert!(Arc::ptr_eq(&map.get_by_pos(BytePos(5)).unwrap(), &b)); // inside b
    }

    #[test]
    fn get_by_pos_gap_and_out_of_range() {
        let (map, _a, _b) = two_file_map();
        assert!(map.get_by_pos(BytePos(3)).is_none()); // gap byte between files
        assert!(map.get_by_pos(BytePos(6)).is_none()); // one past end of last file
        assert!(map.get_by_pos(BytePos(100)).is_none()); // far beyond all files
    }

    #[test]
    fn get_by_pos_empty_map() {
        let map = SourceMap::new();
        assert!(map.get_by_pos(BytePos(0)).is_none()); // checked_sub(1) underflow guard
    }

    #[test]
    fn resolve_span_inside_file_is_local() {
        let (map, a, b) = two_file_map(); // a=[0,3) "abc", b=[4,6) "de"
        let (f, r) = map.resolve_span(Span::new(BytePos(1), BytePos(3))).unwrap();
        assert!(Arc::ptr_eq(&f, &a));
        assert_eq!(r, 1..3); // "bc"
        let (f, r) = map.resolve_span(Span::new(BytePos(4), BytePos(5))).unwrap();
        assert!(Arc::ptr_eq(&f, &b));
        assert_eq!(r, 0..1); // "d", rebased to file-local
    }

    #[test]
    fn resolve_span_non_empty_ending_at_eof() {
        let (map, a, _b) = two_file_map();
        // "c" = last byte of a, global [2, 3); hi sits exactly at a.span.hi.
        let (f, r) = map.resolve_span(Span::new(BytePos(2), BytePos(3))).unwrap();
        assert!(Arc::ptr_eq(&f, &a));
        assert_eq!(r, 2..3);
    }

    #[test]
    fn resolve_span_eof_is_zero_width_at_end() {
        let (map, a, b) = two_file_map();
        let (f, r) = map.resolve_span(Span::new(a.span.hi, a.span.hi)).unwrap();
        assert!(Arc::ptr_eq(&f, &a));
        assert_eq!(r, 3..3); // a.contents.len()
        let (f, r) = map.resolve_span(Span::new(b.span.hi, b.span.hi)).unwrap();
        assert!(Arc::ptr_eq(&f, &b));
        assert_eq!(r, 2..2); // b.contents.len()
    }

    #[test]
    fn resolve_span_gap_byte_belongs_to_preceding_file() {
        // The 1-byte gap *is* the preceding file's `hi`, so (unlike get_by_pos)
        // it resolves to that file as an end-of-file position.
        let (map, a, _b) = two_file_map();
        let (f, r) = map.resolve_span(Span::new(BytePos(3), BytePos(3))).unwrap();
        assert!(Arc::ptr_eq(&f, &a));
        assert_eq!(r, 3..3);
    }

    #[test]
    fn resolve_span_overshoot_is_clamped_to_file_end() {
        // A malformed span whose hi runs past the file end is clamped, never
        // OOB.
        let (map, a, _b) = two_file_map();
        let (f, r) = map
            .resolve_span(Span::new(BytePos(1), BytePos(10)))
            .unwrap();
        assert!(Arc::ptr_eq(&f, &a));
        assert_eq!(r, 1..3); // hi clamped to a.contents.len()
    }

    #[test]
    fn resolve_span_uses_byte_offsets() {
        // Ranges are UTF-8 byte offsets, not char indices.
        let map = SourceMap::new();
        let m = map.register(make_source_file("m.sk", "фx")); // 'ф' = 2 bytes, 'x' = 1
        let (f, r) = map.resolve_span(Span::new(BytePos(2), BytePos(3))).unwrap();
        assert!(Arc::ptr_eq(&f, &m));
        assert_eq!(r, 2..3); // the 'x', after the 2-byte 'ф'
        let (_f, r) = map.resolve_span(Span::new(BytePos(3), BytePos(3))).unwrap();
        assert_eq!(r, 3..3); // EOF
    }

    #[test]
    fn resolve_span_past_end_is_none() {
        let (map, _a, b) = two_file_map();
        let past = b.span.hi + BytePos(1);
        assert!(map.resolve_span(Span::new(past, past)).is_none());
        assert!(
            map.resolve_span(Span::new(BytePos(100), BytePos(100)))
                .is_none()
        );
    }

    #[test]
    fn resolve_span_empty_map_is_none() {
        let map = SourceMap::new();
        assert!(
            map.resolve_span(Span::new(BytePos(0), BytePos(0)))
                .is_none()
        );
    }

    #[test]
    fn get_by_path_present_and_absent() {
        let map = SourceMap::new();
        let a = map.register(make_source_file("a.sk", "abc"));
        assert!(Arc::ptr_eq(
            &map.get_by_path(Path::new("a.sk")).unwrap(),
            &a
        ));
        assert!(map.get_by_path(Path::new("missing.sk")).is_none());
    }

    #[test]
    fn get_by_path_requires_exact_match() {
        // Canonicalization is the caller's responsibility: differing path
        // strings for the same file are intentionally distinct keys.
        let map = SourceMap::new();
        map.register(make_source_file("a.sk", "abc"));
        assert!(map.get_by_path(Path::new("./a.sk")).is_none());
    }

    #[test]
    fn register_concurrent_same_path_dedups() {
        let map = SourceMap::new();
        let ptrs = std::thread::scope(|s| {
            let handles: Vec<_> = (0..8)
                .map(|_| s.spawn(|| map.register(make_source_file("same.sk", "abc"))))
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        });

        // Every thread observes the same single registered file.
        for p in &ptrs {
            assert!(Arc::ptr_eq(p, &ptrs[0]));
        }
        assert_eq!(file_count(&map), 1);
    }
}
