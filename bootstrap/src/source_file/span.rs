use crate::source_file::BytePos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl Span {
    pub fn new(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }
}

#[derive(Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}
