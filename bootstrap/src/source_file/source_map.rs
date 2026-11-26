use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use indexmap::IndexMap;
use thiserror::Error;

use crate::source_file::{BytePos, SourceFile, SourceFileId, Span};

#[derive(Debug)]
pub struct SourceMap {
    inner: RwLock<SourceMapImpl>,
}

impl SourceMap {
    pub fn new() -> Self {
        let src_map_impl = SourceMapImpl::new();
        let inner = RwLock::new(src_map_impl);
        Self { inner }
    }

    pub fn add(&self, path: &Path) -> SourceMapResult<Arc<SourceFile>> {
        if let Some(src_file_ptr) = self.get_by_path(path) {
            return Ok(src_file_ptr);
        }

        let text =
            std::fs::read_to_string(path).map_err(|err| SourceMapError::UnableToReadFile {
                path: path.to_path_buf(),
                source: err,
            })?;

        Ok(self.register(path, text))
    }

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
            .filter(|f| pos <= f.span.hi)
            .cloned()
    }

    pub fn get_by_path(&self, path: &Path) -> Option<Arc<SourceFile>> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.files.get(path).cloned()
    }

    pub fn resolve_span(&self, span: Span) -> Option<(Arc<SourceFile>, Range<usize>)> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        let n = inner
            .files
            .as_slice()
            .partition_point(|_, f| f.span.lo <= span.lo);
        let (_, file) = inner.files.get_index(n.checked_sub(1)?)?;

        if span.lo > span.hi || span.hi > file.span.hi {
            return None;
        }

        let base = file.span.lo;
        let lo = usize::from(span.lo - base);
        let hi = usize::from(span.hi - base);
        Some((file.clone(), lo..hi))
    }

    fn register(&self, path: &Path, text: String) -> Arc<SourceFile> {
        let mut inner = self.inner.write().expect("unable to acquire write lock");
        if let Some(src_file_ptr) = inner.files.get(path).cloned() {
            // Somebody added file while we were waiting for write lock
            return src_file_ptr;
        }

        let id = inner.next_id();
        let span = Span::new(inner.bytes_len, inner.bytes_len + text.len().into());
        inner.bytes_len = span.hi + BytePos::from(1u32);

        let src_file_ptr = Arc::new(SourceFile::new(id, path.to_path_buf(), text, span));

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
            bytes_len: 0u32.into(),
        }
    }

    fn next_id(&self) -> SourceFileId {
        SourceFileId::from(self.files.len())
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
