#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum TokenType {
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
    LitConst,

    /// End of input.
    EndOfFile,
}
