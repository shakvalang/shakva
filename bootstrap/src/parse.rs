use std::{ops::Range, str::Chars};

pub fn parse_file(input: &str) -> ParseFileResult<'_> {
    let mut parser = Parser {
        lexer: Lexer {
            input: input.chars(),
            len_remaining: input.len(),
        },
        errors: vec![],
        line_break_before: false,
        cur: (Token::Dummy, Span::default()),
    };

    let file = parser.parse_file();

    ParseFileResult {
        file,
        errors: parser.errors,
    }
}

#[derive(Debug)]
struct Lexer<'src> {
    input: Chars<'src>,
    len_remaining: usize,
}

#[derive(Debug)]
struct Parser<'src> {
    lexer: Lexer<'src>,
    errors: Vec<ParseError<'src>>,
    line_break_before: bool,
    cur: Spanned<Token<'src>>,
}

impl<'src> Parser<'src> {
    fn parse_file(&mut self) -> ShakvaFile<'src> {
        let mut exprs = vec![];
        while !self.is_at_eof() {
            exprs.push(self.parse_expr(0));

            if self.eat(Token::Semi) {
                continue;
            }
            if self.is_at_eof() || self.line_break_before {
                continue;
            }

            self.expected_expr_sep();
            break;
        }

        ShakvaFile { exprs }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Spanned<Expr<'src>> {
        let mut lhs = self.parse_expr_start(min_bp);

        loop {
            if self.is_at_eof() {
                break;
            }
            let op = self.parse_infix_op();
            let (lbp, rbp) = op.0.bp();
            if lbp < min_bp {
                break;
            }
            self.bump();

            let rhs = self.parse_expr(rbp);
            let span = lhs.1.start..rhs.1.end;
            lhs = (
                Expr::Infix {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                },
                span.into(),
            );
        }

        lhs
    }

    fn parse_expr_start(&mut self, min_bp: u8) -> Spanned<Expr<'src>> {
        let (tok, sp) = self.peek();

        todo!()
    }

    fn parse_infix_op(&mut self) -> Spanned<InfixOp> {
        todo!()
    }

    fn eat(&mut self, tok: Token) -> bool {
        todo!()
    }

    fn is_at_eof(&self) -> bool {
        todo!()
    }

    fn bump(&mut self) {
        todo!()
    }

    fn peek(&self) -> &Spanned<Token<'src>> {
        &self.cur
    }

    fn expected_expr_sep(&mut self) {
        self.errors
            .push(ParseError::ExpectedExprSep { actual: self.cur });
    }
}

#[derive(Debug)]
pub struct ParseFileResult<'src> {
    file: ShakvaFile<'src>,
    errors: Vec<ParseError<'src>>,
}

#[derive(Debug)]
pub enum ParseError<'src> {
    ExpectedExprSep {
        actual: Spanned<Token<'src>>,
    },
    UnexpectedToken {
        expected: Vec<TokenType>,
        actual: Spanned<Token<'src>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /* identifiers and literals */
    Ident,
    LitConst,

    /* keywords */
    AndKw,
    CatchKw,
    DoKw,
    ElifKw,
    ElseKw,
    FalseKw,
    FinallyKw,
    ForKw,
    IfKw,
    InKw,
    MatchKw,
    NotKw,
    OrKw,
    ReturnKw,
    RefKw,
    ThrowKw,
    TrueKw,
    TryKw,
    WhileKw,

    /* delimiters */
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    /* punctuation */
    Comma,
    Semi,
    Colon,
    Dot,
    Quest,
    Excl,
    Wildcard,
    Arrow,
    FArrow,

    /* arithmetic operators */
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DStar,
    DSlash,

    /* comparison operators */
    Eq,
    Lt,
    Le,
    Gt,
    Ge,

    /* binding and assignment */
    Bind,
    Rebind,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    DStarEq,
    DSlashEq,

    /* special */
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'src> {
    /* trivia */
    /// A sequence of whitespace characters excluding CR (`\r`) and LF (`\n`).
    Ws,
    /// A comment starting with `#`, excluding the terminating line ending.
    Comment,

    /* identifiers and literals */
    Ident(&'src str),
    LitConst {
        kind: LitConstKind,
        value: &'src str,
    },

    /* keywords */
    /// `and`.
    AndKw,
    /// `catch`.
    CatchKw,
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
    Dummy,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BytePos(u32);

impl From<u32> for BytePos {
    fn from(value: u32) -> Self {
        BytePos(value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

impl Span {
    pub fn new(start: BytePos, end: BytePos) -> Self {
        Self { start, end }
    }
}

impl From<Range<BytePos>> for Span {
    fn from(value: Range<BytePos>) -> Self {
        Self::new(value.start, value.end)
    }
}

pub type Spanned<T> = (T, Span);

#[derive(Debug)]
pub struct ShakvaFile<'src> {
    pub exprs: Vec<Spanned<Expr<'src>>>,
}

#[derive(Debug)]
pub enum Expr<'src> {
    Infix {
        lhs: Box<Spanned<Expr<'src>>>,
        op: Spanned<InfixOp>,
        rhs: Box<Spanned<Expr<'src>>>,
    },
    Prefix {
        op: Spanned<PrefixOp>,
        rhs: Box<Spanned<Expr<'src>>>,
    },
    ParenExpr {
        inner: Box<Spanned<Expr<'src>>>,
    },
    LitConstExpr(LitConstData),
    RefExpr {
        name: Spanned<&'src str>,
    },
    CallExpr {
        callee: Box<Spanned<Expr<'src>>>,
        args: Vec<Spanned<Expr<'src>>>,
    },
    IfExpr {
        if_branch: Spanned<IfBranch<'src>>,
        elif_branches: Vec<Spanned<ElifBranch<'src>>>,
        else_branch: Option<Spanned<ElseBranch<'src>>>,
    },
    LambdaExpr {
        params: Vec<Spanned<&'src str>>,
        body: Spanned<Body<'src>>,
    },
    MatchExpr {
        scrutinee: Box<Spanned<Expr<'src>>>,
        arms: Vec<Spanned<MatchArm<'src>>>,
    },
    ForExpr {
        pat: Spanned<Pattern<'src>>,
        iterable: Box<Spanned<Expr<'src>>>,
        guard: Option<Box<Spanned<Expr<'src>>>>,
        body: Spanned<Body<'src>>,
    },
    WhileExpr {
        cond: Box<Spanned<Expr<'src>>>,
        body: Spanned<Body<'src>>,
    },
    DoWhileExpr {
        body: Spanned<Body<'src>>,
        cond: Box<Spanned<Expr<'src>>>,
    },
    LoopExpr {
        body: Spanned<Body<'src>>,
    },
    BreakExpr {
        expr: Option<Box<Spanned<Expr<'src>>>>,
    },
    ContinueExpr,
    TryExpr {
        try_block: Spanned<Body<'src>>,
        catch_blocks: Vec<Spanned<CatchBlock<'src>>>,
        finally_block: Option<Spanned<Body<'src>>>,
    },
    ThrowExpr {
        expr: Box<Spanned<Expr<'src>>>,
    },
    CompareExpr {
        first: Box<Spanned<Expr<'src>>>,
        rest: Vec<(Spanned<CompareOp>, Spanned<Expr<'src>>)>,
    },
    ReturnExpr {
        expr: Option<Box<Spanned<Expr<'src>>>>,
    },
    Error,
}

#[derive(Debug)]
pub struct IfBranch<'src> {
    pub cond: Box<Spanned<Expr<'src>>>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct ElifBranch<'src> {
    pub cond: Spanned<Expr<'src>>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct ElseBranch<'src> {
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct Body<'src> {
    pub exprs: Vec<Spanned<Expr<'src>>>,
}

#[derive(Debug)]
pub struct MatchArm<'src> {
    pub pat: Spanned<Pattern<'src>>,
    pub guard: Option<Spanned<Expr<'src>>>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub enum Pattern<'src> {
    Ident(Spanned<&'src str>),
    LitConst {
        op: Option<Spanned<PrefixOp>>,
        data: Spanned<LitConstData>,
    },
}

#[derive(Debug)]
pub struct CatchBlock<'src> {
    pub pat: Spanned<Pattern<'src>>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct LitConstData {}

#[derive(Debug)]
pub enum InfixOp {
    Or,
    And,
    Eq,
    Gt,
    Ge,
    Lt,
    Le,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    IntDiv,
    Pow,
    Error,
}

impl InfixOp {
    fn bp(&self) -> (u8, u8) {
        use InfixOp::*;
        match self {
            Or => (1, 2),
            And => (3, 4),
            // (5, 6) is taken by prefix `not`.
            Eq | Gt | Ge | Lt | Le => (7, 8),
            Add | Sub => (9, 10),
            Mul | Div | Mod | IntDiv => (11, 12),
            // (13, 14) is taken by prefix `-`
            Pow => (16, 15),
            Error => (0, 0),
        }
    }
}

#[derive(Debug)]
pub enum PrefixOp {
    Not,
    Neg,
    Error,
}

impl PrefixOp {
    fn bp(&self) -> ((), u8) {
        use PrefixOp::*;
        match self {
            Not => ((), 6),
            Neg => ((), 14),
            Error => ((), 0),
        }
    }
}

#[derive(Debug)]
pub enum CompareOp {
    Eq,
    Gt,
    Ge,
    Lt,
    Le,
    Error,
}
