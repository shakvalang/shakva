use rowan::{GreenNode, GreenNodeBuilder};

use crate::syntax::ShakvaKind;

pub fn parse(src: &str) -> Parse {
    todo!()
}

#[derive(Debug)]
pub struct Parse {
    g_node: GreenNode,
    errors: Vec<Error>,
}

#[derive(Debug)]
pub struct Error {}

#[derive(Debug)]
struct Parser<'src> {
    toks: Vec<(ShakvaKind, String)>,
    builder: GreenNodeBuilder<'src>,
    errors: Vec<Error>,
}

impl Parser<'_> {
    fn parse(mut self) -> Parse {
        todo!()
    }
}
