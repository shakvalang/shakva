mod pos;
mod source_map;
mod span;

use std::path::PathBuf;

pub use pos::*;
pub use source_map::*;
pub use span::*;

#[derive(Debug)]
pub struct SourceFile {
    pub id: SourceFileId,
    pub path: PathBuf,
    pub text: String,
    pub span: Span,
}

impl SourceFile {
    pub fn new(id: SourceFileId, path: PathBuf, text: String, span: Span) -> Self {
        Self {
            id,
            path,
            text,
            span,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceFileId(u32);

impl From<u32> for SourceFileId {
    fn from(value: u32) -> Self {
        SourceFileId(value)
    }
}

impl From<usize> for SourceFileId {
    fn from(value: usize) -> Self {
        SourceFileId(value as u32)
    }
}
