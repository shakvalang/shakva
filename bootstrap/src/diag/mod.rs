mod ctx;

use DiagLvl::*;
pub use ctx::*;

use crate::source_file::Span;

#[derive(Debug, Clone)]
pub struct Diag {
    pub lvl: DiagLvl,
    pub msg: String,
    pub primary_span: Span,
    pub labels: Vec<Label>,
}

impl Diag {
    pub fn new(lvl: DiagLvl, primary_span: Span, msg: impl Into<String>) -> Self {
        Self {
            lvl,
            primary_span,
            msg: msg.into(),
            labels: vec![],
        }
    }

    pub fn error(primary_span: Span, msg: impl Into<String>) -> Self {
        Self::new(Error, primary_span, msg)
    }

    pub fn warning(primary_span: Span, msg: impl Into<String>) -> Self {
        Self::new(Warning, primary_span, msg)
    }

    pub fn note(primary_span: Span, msg: impl Into<String>) -> Self {
        Self::new(Note, primary_span, msg)
    }

    pub fn label(mut self, span: Span, msg: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            msg: msg.into(),
        });
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagLvl {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub msg: String,
    pub span: Span,
}
