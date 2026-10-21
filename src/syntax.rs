use ShakvaKind::*;
use rowan::{GreenNode, Language, NodeOrToken, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
enum ShakvaKind {
    Whitespace = 0,
    LineComment,
    BlockComment,

    /// End of the Shakva language syntax kinds. DO NOT add any variants below it.
    EndMarker,
}

impl From<ShakvaKind> for SyntaxKind {
    fn from(kind: ShakvaKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ShakvaLang {}
impl Language for ShakvaLang {
    type Kind = ShakvaKind;

    fn kind_from_raw(raw: SyntaxKind) -> Self::Kind {
        assert!(raw.0 < EndMarker as u16);
        // SAFETY: discriminants are contiguous from zero;
        // the assertion excludes the sentinel and out-of-range values.
        unsafe { std::mem::transmute::<u16, ShakvaKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> SyntaxKind {
        kind.into()
    }
}

type ShakvaNode = SyntaxNode<ShakvaLang>;
type ShakvaToken = SyntaxToken<ShakvaLang>;
type ShakvaElement = NodeOrToken<ShakvaNode, ShakvaToken>;

type ShakvaError = String;

struct Parse {
    green_node: GreenNode,
    errors: Vec<ShakvaError>,
}

pub fn parse(text: &str) -> Parse {
    todo!()
}
