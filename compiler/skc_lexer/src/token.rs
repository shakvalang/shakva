use crate::TokenKind::*;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    pub fn is_newline(&self) -> bool {
        self.kind == Newline
    }
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Any whitespace character sequence.
    Whitespace,

    /// `LF | ( CR [ LF ] )`
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
    LitConst { kind: LitConstKind },

    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,

    /// End of input.
    EndOfFile,
}

/// Enum representing the literal types supported by the lexer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LitConstKind {
    Int { empty_int: bool },
    Float { empty_exp: bool },
}
