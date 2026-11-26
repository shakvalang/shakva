use std::str::Chars;

use crate::syntax::Token;

#[derive(Debug)]
pub struct Lexer<'src> {
    input: Chars<'src>,
    len_remaining: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input: input.chars(),
            len_remaining: input.len(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        todo!()
    }
}
