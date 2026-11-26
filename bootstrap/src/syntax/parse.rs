use crate::{
    source_file::{Span, Spanned},
    syntax::*,
};

pub fn parse_file(input: &str) -> ParseFileResult<'_> {
    let mut lexer = Lexer::new(input);
    let cur_tok = lexer.next_token();
    let mut parser = Parser {
        lexer,
        errors: vec![],
        cur_tok,
    };

    let file = parser.parse_file();

    ParseFileResult {
        file,
        errors: parser.errors,
    }
}

#[derive(Debug)]
struct Parser<'src> {
    lexer: Lexer<'src>,
    errors: Vec<ParseError>,
    cur_tok: Token,
}

impl<'src> Parser<'src> {
    fn parse_file(&mut self) -> ShakvaFile<'src> {
        let mut exprs = vec![];
        while !self.is_at_eof() {
            exprs.push(self.parse_expr(0));

            if self.eat(TokenKind::Semi) {
                continue;
            }
            if self.is_at_eof() || self.cur_tok.line_break_before {
                continue;
            }

            self.expected_expr_sep();
            break;
        }

        ShakvaFile { exprs }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Expr<'src> {
        let mut lhs = self.parse_expr_start(min_bp);

        loop {
            if self.is_at_eof() {
                break;
            }
            let op = self.parse_infix_op();
            let (lbp, rbp) = op.value.bp();
            if lbp < min_bp {
                break;
            }
            self.bump();

            let rhs = self.parse_expr(rbp);
            let span = lhs.span.lo..rhs.span.hi;
            lhs = Expr {
                kind: ExprKind::Infix {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                },
                span: Span::new(span.start, span.end),
            };
        }

        lhs
    }

    fn parse_expr_start(&mut self, min_bp: u8) -> Expr<'src> {
        let tok = self.peek();

        todo!()
    }

    fn parse_infix_op(&mut self) -> Spanned<InfixOp> {
        todo!()
    }

    fn eat(&mut self, tok: TokenKind) -> bool {
        todo!()
    }

    fn is_at_eof(&self) -> bool {
        todo!()
    }

    fn bump(&mut self) {
        todo!()
    }

    fn peek(&self) -> &Token {
        &self.cur_tok
    }

    fn expected_expr_sep(&mut self) {
        self.errors.push(ParseError::ExpectedExprSep {
            actual: self.cur_tok,
        });
    }
}

#[derive(Debug)]
pub struct ParseFileResult<'src> {
    pub file: ShakvaFile<'src>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedExprSep {
        actual: Token,
    },
    UnexpectedToken {
        expected: Vec<TokenKind>,
        actual: Token,
    },
}
