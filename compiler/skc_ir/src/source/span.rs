use std::ops::Range;

use crate::source::BytePos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl Span {
    pub fn new(mut lo: BytePos, mut hi: BytePos) -> Self {
        if lo > hi {
            std::mem::swap(&mut lo, &mut hi);
        }
        Self { lo, hi }
    }

    pub fn dummy() -> Self {
        Self::new(BytePos(0), BytePos(0))
    }

    pub fn to(&self, other: &Self) -> Self {
        let lo = self.lo.min(other.lo);
        let hi = self.hi.max(other.hi);

        Self::new(lo, hi)
    }
}

impl From<Range<BytePos>> for Span {
    fn from(value: Range<BytePos>) -> Self {
        Self::new(value.start, value.end)
    }
}
