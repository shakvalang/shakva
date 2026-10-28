#[rustfmt::skip]
mod generated;

pub use generated::*;

use crate::syntax::{ShakvaKind, ShakvaNode};

pub trait ShakvaAstNode: Sized {
    fn can_cast(kind: ShakvaKind) -> bool;
    fn cast(syntax: ShakvaNode) -> Option<Self>;
    fn syntax(&self) -> &ShakvaNode;
}
