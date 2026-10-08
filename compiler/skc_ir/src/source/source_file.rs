use std::path::PathBuf;

use crate::source::{BytePos, Span};

#[derive(Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub contents: String,
    pub span: Span,
    pub line_starts: Vec<BytePos>,
}

impl SourceFile {
    pub fn new(path: PathBuf, contents: String, span: Span, line_starts: Vec<BytePos>) -> Self {
        Self {
            path,
            contents,
            span,
            line_starts,
        }
    }

    pub fn line_col(&self, pos: BytePos) -> (usize, usize) {
        // `line_starts[0]` is the file base, so for any in-file `pos` the
        // partition point is >= 1 and the `-1` never underflows.
        let line = self.line_starts.partition_point(|&start| start <= pos) - 1;
        let line_start = (self.line_starts[line] - self.span.lo).to_usize();
        let offset = (pos - self.span.lo).to_usize();
        let col = self.contents[line_start..offset].chars().count();
        (line + 1, col + 1)
    }
}
