use std::{ops::Range, str::Chars};

pub fn parse_file(input: &str) -> ParseFileResult<'_> {
    let mut parser = Parser {
        lexer: Lexer {
            input: input.chars(),
            len_remaining: input.len(),
        },
        errors: vec![],
        cur: Token {
            kind: TokenKind::Dummy,
            span: Span::default(),
            line_break_before: false,
        },
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

impl<'src> Lexer<'src> {}

#[derive(Debug)]
struct Parser<'src> {
    lexer: Lexer<'src>,
    errors: Vec<ParseError>,
    cur: Token,
}

impl<'src> Parser<'src> {
    fn parse_file(&mut self) -> ShakvaFile<'src> {
        let mut exprs = vec![];
        while !self.is_at_eof() {
            exprs.push(self.parse_expr(0));

            if self.eat(TokenKind::Semi) {
                continue;
            }
            if self.is_at_eof() || self.cur.line_break_before {
                continue;
            }

            self.expected_expr_sep();
            break;
        }

        ShakvaFile { exprs }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Expr<'src> {
        let mut lhs = self.parse_expr_start(min_bp);

        loop {
            if self.is_at_eof() {
                break;
            }
            let op = self.parse_infix_op();
            let (lbp, rbp) = op.value.bp();
            if lbp < min_bp {
                break;
            }
            self.bump();

            let rhs = self.parse_expr(rbp);
            let span = lhs.span.start..rhs.span.end;
            lhs = Expr {
                kind: ExprKind::Infix {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                },
                span: span.into(),
            };
        }

        lhs
    }

    fn parse_expr_start(&mut self, min_bp: u8) -> Expr<'src> {
        let tok = self.peek();

        todo!()
    }

    fn parse_infix_op(&mut self) -> Spanned<InfixOp> {
        todo!()
    }

    fn eat(&mut self, tok: TokenKind) -> bool {
        todo!()
    }

    fn is_at_eof(&self) -> bool {
        todo!()
    }

    fn bump(&mut self) {
        todo!()
    }

    fn peek(&self) -> &Token {
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
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedExprSep {
        actual: Token,
    },
    UnexpectedToken {
        expected: Vec<TokenKind>,
        actual: Token,
    },
}

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
    pub fn text(self, source: &str) -> &str {
        &source[self.start.0 as usize..self.end.0 as usize]
    }

    pub fn new(start: BytePos, end: BytePos) -> Self {
        Self { start, end }
    }
}

impl From<Range<BytePos>> for Span {
    fn from(value: Range<BytePos>) -> Self {
        Self::new(value.start, value.end)
    }
}

#[derive(Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug)]
pub struct ShakvaFile<'src> {
    pub exprs: Vec<Expr<'src>>,
}

#[derive(Debug)]
pub struct Expr<'src> {
    pub kind: ExprKind<'src>,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind<'src> {
    Infix {
        lhs: Box<Expr<'src>>,
        op: Spanned<InfixOp>,
        rhs: Box<Expr<'src>>,
    },
    Prefix {
        op: Spanned<PrefixOp>,
        rhs: Box<Expr<'src>>,
    },
    Paren {
        inner: Box<Expr<'src>>,
    },
    LitConst(LitConstData<'src>),
    Ref(&'src str),
    Call {
        callee: Box<Expr<'src>>,
        args: Vec<Expr<'src>>,
    },
    If {
        if_branch: Spanned<IfBranch<'src>>,
        elif_branches: Vec<Spanned<ElifBranch<'src>>>,
        else_branch: Option<Spanned<ElseBranch<'src>>>,
    },
    Lambda {
        params: Vec<Spanned<&'src str>>,
        body: Spanned<Body<'src>>,
    },
    Match {
        scrutinee: Box<Expr<'src>>,
        arms: Vec<Spanned<MatchArm<'src>>>,
    },
    For {
        pat: Spanned<Pattern<'src>>,
        iterable: Box<Expr<'src>>,
        guard: Option<Box<Expr<'src>>>,
        body: Spanned<Body<'src>>,
    },
    While {
        cond: Box<Expr<'src>>,
        body: Spanned<Body<'src>>,
    },
    DoWhile {
        body: Spanned<Body<'src>>,
        cond: Box<Expr<'src>>,
    },
    Loop {
        body: Spanned<Body<'src>>,
    },
    Break {
        expr: Option<Box<Expr<'src>>>,
    },
    Continue,
    Try {
        try_block: Spanned<Body<'src>>,
        catch_blocks: Vec<Spanned<CatchBlock<'src>>>,
        finally_block: Option<Spanned<Body<'src>>>,
    },
    Throw {
        expr: Box<Expr<'src>>,
    },
    Compare {
        first: Box<Expr<'src>>,
        rest: Vec<(Spanned<CompareOp>, Expr<'src>)>,
    },
    Return {
        expr: Option<Box<Expr<'src>>>,
    },
    Error,
}

#[derive(Debug)]
pub struct IfBranch<'src> {
    pub cond: Box<Expr<'src>>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct ElifBranch<'src> {
    pub cond: Expr<'src>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct ElseBranch<'src> {
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct Body<'src> {
    pub exprs: Vec<Expr<'src>>,
}

#[derive(Debug)]
pub struct MatchArm<'src> {
    pub pat: Spanned<Pattern<'src>>,
    pub guard: Option<Expr<'src>>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub enum Pattern<'src> {
    Ident(Spanned<&'src str>),
    LitConst {
        op: Option<Spanned<PrefixOp>>,
        data: Spanned<LitConstData<'src>>,
    },
}

#[derive(Debug)]
pub struct CatchBlock<'src> {
    pub pat: Spanned<Pattern<'src>>,
    pub body: Spanned<Body<'src>>,
}

#[derive(Debug)]
pub struct LitConstData<'src> {
    pub kind: LitConstKind,
    pub text: &'src str,
}

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
