use std::ops::Index;

use crate::{source::Span, syntax::Symbol};

use TokenKind::*;

#[derive(Clone, Debug)]
pub struct Tokens {
    inner: Vec<Token>,
}

impl Tokens {
    pub fn new(tokens: impl Iterator<Item = Token>) -> Self {
        Self {
            inner: tokens.collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Token> {
        self.inner.iter().copied()
    }
}

impl From<Vec<Token>> for Tokens {
    fn from(value: Vec<Token>) -> Self {
        Self { inner: value }
    }
}

impl Index<usize> for Tokens {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn dummy() -> Self {
        Self::new(Dummy, Span::dummy())
    }

    pub fn glue(&self, _: &Self) -> Option<Self> {
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    /// `LF | (CR [LF])`
    Newline,

    /// `;`
    Semi,

    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,

    /// `(`
    LParen,
    /// `)`
    RParen,

    /// A literal constant value, e.g. `42` or `321.123`.
    LitConst(LiteralConst),

    /// Dummy token for parser needs.
    Dummy,

    Error,

    /// End of input.
    EndOfFile,
}

#[derive(Clone, Copy, Debug)]
pub struct LiteralConst {
    pub kind: LitConstKind,
    pub sym: Symbol,
}

#[derive(Clone, Copy, Debug)]
pub enum LitConstKind {
    Int,
    Float,
    Error,
}
