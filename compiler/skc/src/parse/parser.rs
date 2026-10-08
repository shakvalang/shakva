use skc_ir::{
    source::BytePos,
    syntax::{Expr, ShakvaFile, Stmt, StmtKind, Token, TokenKind, Tokens},
};

use crate::parse::{Lexer, ParseSession};

#[derive(Debug)]
pub struct Parser<'ci> {
    psess: ParseSession<'ci>,
    tokens: &'ci Tokens,
    token: Token,
    pos: usize,
}

impl<'ci> Parser<'ci> {
    pub fn new(tokens: &'ci Tokens, psess: ParseSession<'ci>) -> Self {
        let mut parser = Self {
            psess,
            tokens,
            token: Token::dummy(),
            pos: 0,
        };
        let _ = parser.bump();
        parser
    }

    /// ```ebnf
    /// file = { NL } { stmt semi } EOF .
    /// ```
    pub fn parse_file(&mut self) -> ShakvaFile {
        // while self.eat(TokenType::NL) {}

        // let mut stmts = Vec::new();
        // while !self.at(TokenType::EOF) {
        //     let stmt = self.stmt();
        //     stmts.push(stmt);
        //     self.semi();
        // }

        // self.expect(TokenType::EOF);

        todo!()
    }

    /// ```ebnf
    /// stmt = expr .
    /// ```
    fn parse_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr(0);
        let span = expr.span;
        Stmt {
            kind: StmtKind::Expr(expr),
            span,
        }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Expr {
        todo!()
    }

    /// ```ebnf
    /// semi = ( ";" | NL ) { NL } .
    /// ```
    fn parse_semi(&mut self) {
        todo!()
    }

    // fn expect(&mut self, expected: TokenType) -> Option<ShakvaToken> {
    //     if self.at(expected) {
    //         return Some(self.bump());
    //     }

    //     None
    // }

    // fn eat(&mut self, expected: TokenType) -> bool {
    //     if self.at(expected) {
    //         self.bump();
    //         return true;
    //     }

    //     false
    // }

    // fn at_any(&self, expected: &[TokenType]) -> bool {
    //     expected.iter().any(|tt| *tt == self.first().kind)
    // }

    // fn at(&self, expected: TokenType) -> bool {
    //     self.first().kind == expected
    // }

    fn first(&self) -> Token {
        self.token
    }

    fn bump(&mut self) -> Token {
        if self.is_at_eof() {
            return self.token;
        }
        let res = std::mem::replace(&mut self.token, self.tokens[self.pos]);
        self.pos += 1;
        res
    }

    fn is_at_eof(&self) -> bool {
        matches!(self.token.kind, TokenKind::EndOfFile)
    }
}
