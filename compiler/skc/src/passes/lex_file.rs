use std::sync::Arc;

use skc_ir::{source::SourceFile, syntax::Tokens};

use crate::{ci::Shakva, parse::Lexer};

pub fn lex_file(ci: &Shakva, src: Arc<SourceFile>) -> Tokens {
    Lexer::tokenize(&src.contents, src.span.lo, ci.psess())
}
