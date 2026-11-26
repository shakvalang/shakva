use crate::{
    source_file::{Span, Spanned},
    syntax::LitConstKind,
};

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
    pub fn bp(&self) -> (u8, u8) {
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
    pub fn bp(&self) -> ((), u8) {
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
