use crate::source_file::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub line_break_before: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    Literal(LitConstKind),

    /* keywords */
    /// `and`.
    AndKw,
    /// `catch`.
    CatchKw,
    /// `break`.
    BreakKw,
    /// `continue`.
    ContinueKw,
    /// `loop`.
    LoopKw,
    /// `do`.
    DoKw,
    /// `elif`.
    ElifKw,
    /// `else`.
    ElseKw,
    /// `false`.
    FalseKw,
    /// `finally`.
    FinallyKw,
    /// `for`.
    ForKw,
    /// `if`.
    IfKw,
    /// `in`.
    InKw,
    /// `match`.
    MatchKw,
    /// `not`.
    NotKw,
    /// `or`.
    OrKw,
    /// `return`.
    ReturnKw,
    /// `ref`.
    RefKw,
    /// `throw`.
    ThrowKw,
    /// `true`.
    TrueKw,
    /// `try`.
    TryKw,
    /// `while`.
    WhileKw,

    /* delimiters */
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `[`.
    LBracket,
    /// `]`.
    RBracket,
    /// `{`.
    LBrace,
    /// `}`.
    RBrace,

    /* punctuation */
    /// `,`.
    Comma,
    /// `;`.
    Semi,
    /// `:`.
    Colon,
    /// `.`.
    Dot,
    /// `?`.
    Quest,
    /// `!`.
    Excl,
    /// `_`.
    Wildcard,
    /// `->`.
    Arrow,
    /// `=>`.
    FArrow,

    /* arithmetic operators */
    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `%`.
    Percent,
    /// `**`.
    DStar,
    /// `//`.
    DSlash,

    /* comparison operators */
    /// `==`.
    Eq,
    /// `<`.
    Lt,
    /// `<=`.
    Le,
    /// `>`.
    Gt,
    /// `>=`.
    Ge,

    /* binding and assignment */
    /// `:=`.
    Bind,
    /// `=`.
    Rebind,
    /// `+=`.
    PlusEq,
    /// `-=`.
    MinusEq,
    /// `*=`.
    StarEq,
    /// `/=`.
    SlashEq,
    /// `%=`.
    PercentEq,
    /// `**=`.
    DStarEq,
    /// `//=`.
    DSlashEq,

    /* special */
    /// A character the lexer does not know.
    Unknown,
    /// End of input.
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LitConstKind {
    Int { base: Base },
    Float { base: Base },
    Str { terminated: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}
