use crate::{source::Span, syntax::LiteralConst};

#[derive(Debug)]
pub struct Nest {
    // TODO: will be changed in the future.
    pub file: ShakvaFile,
}

#[derive(Debug)]
pub struct Module {}

#[derive(Debug)]
pub struct ShakvaFile {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
    Expr(Expr),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
    Binary {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    },
    Grouped {
        lparen_sp: Span,
        expr: Box<Expr>,
        rparen_sp: Span,
    },
    LitConst {
        value: LiteralConst,
    },
    Error,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub kind: BinaryOpKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum BinaryOpKind {
    Sub,
    Add,
    Mul,
    Div,
    Error,
}
