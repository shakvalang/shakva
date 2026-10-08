use std::sync::Arc;

use skc_ir::{source::SourceFile, syntax::ShakvaFile};

use crate::{ci::Shakva, parse::Parser, passes::lex_file};

pub fn parse_file(ci: &Shakva, src: Arc<SourceFile>) -> ShakvaFile {
    let tokens = lex_file(ci, src);

    let mut parser = Parser::new(&tokens, ci.psess());
    parser.parse_file()
}
