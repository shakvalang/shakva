pub mod ast;
#[cfg(feature = "sdk_dev_tools")]
pub mod codegen;
#[rustfmt::skip]
mod generated;

pub use generated::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShakvaLang {}
impl rowan::Language for ShakvaLang {
    type Kind = ShakvaKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        ShakvaKind::from_raw(raw)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type ShakvaNode = rowan::SyntaxNode<ShakvaLang>;
pub type ShakvaToken = rowan::SyntaxToken<ShakvaLang>;
pub type ShakvaElement = rowan::NodeOrToken<ShakvaNode, ShakvaToken>;
