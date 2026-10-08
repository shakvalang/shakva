use skc_ir::{
    source::{BytePos, Span},
    syntax::{LitConstKind, LiteralConst, Symbol, Token, TokenKind, Tokens},
};
use skc_lexer::Cursor;

use TokenKind::*;

use crate::parse::{NoDigitsLiteral, ParseSession, UnknownTokenStart, escaped_char};

#[derive(Debug)]
pub struct Lexer<'ci> {
    psess: ParseSession<'ci>,
    start_pos: BytePos,
    /// Byte position in the input stream.
    pos: BytePos,
    /// Source text to tokenize.
    src: &'ci str,
    /// Cursor for getting lexer tokens.
    cursor: Cursor<'ci>,
    /// When a "unknown start of token: \u{a0}" has already been emitted earlier
    /// in this file, it's safe to treat further occurrences of the non-breaking
    /// space character as whitespace.
    nbsp_is_ws: bool,
    /// Current token.
    token: Token,
}

impl<'ci> Lexer<'ci> {
    pub fn tokenize(src: &'ci str, start_pos: BytePos, psess: ParseSession<'ci>) -> Tokens {
        let mut lexer = Self::new(src, start_pos, psess);
        let mut tokens = vec![];

        loop {
            let token = lexer.bump();
            let is_eof = matches!(token.kind, TokenKind::EndOfFile);
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Tokens::new(tokens.into_iter())
    }

    pub fn new(src: &'ci str, start_pos: BytePos, psess: ParseSession<'ci>) -> Self {
        let mut lexer = Self {
            psess,
            start_pos,
            pos: start_pos,
            src,
            cursor: Cursor::new(src),
            nbsp_is_ws: false,
            token: Token::dummy(),
        };

        let _ = lexer.bump();

        lexer
    }

    pub fn first(&self) -> &Token {
        &self.token
    }

    pub fn bump(&mut self) -> Token {
        let next_tok = loop {
            let (next_tok, is_next_tok_preceded_by_ws) = self.next_token_from_cursor();

            if is_next_tok_preceded_by_ws {
                break next_tok;
            } else if let Some(glued) = self.token.glue(&next_tok) {
                self.token = glued;
            } else {
                break next_tok;
            }
        };
        std::mem::replace(&mut self.token, next_tok)
    }

    fn next_token_from_cursor(&mut self) -> (Token, bool) {
        let mut preceded_by_ws = false;
        let mut swallow_next_invalid = 0;
        // Skip trivial (whitespace & comments) tokens
        loop {
            let token = self.cursor.bump_token();
            let start = self.pos;
            self.pos += BytePos(token.len);

            let kind = match token.kind {
                skc_lexer::TokenKind::Whitespace => {
                    preceded_by_ws = true;
                    continue;
                }

                skc_lexer::TokenKind::Newline => Newline,
                skc_lexer::TokenKind::Semi => Semi,

                skc_lexer::TokenKind::Minus => Minus,
                skc_lexer::TokenKind::Plus => Plus,
                skc_lexer::TokenKind::Slash => Slash,
                skc_lexer::TokenKind::Star => Star,

                skc_lexer::TokenKind::LParen => LParen,
                skc_lexer::TokenKind::RParen => RParen,

                skc_lexer::TokenKind::LitConst { kind } => {
                    let (kind, sym) = self.cook_lexer_lit_const(start, self.pos, kind);
                    TokenKind::LitConst(LiteralConst { kind, sym })
                }

                skc_lexer::TokenKind::Unknown => {
                    // Don't emit diagnostics for sequences of the same invalid
                    // token
                    if swallow_next_invalid > 0 {
                        swallow_next_invalid -= 1;
                        continue;
                    }
                    let mut it = self.str_from_to_end(start).chars();
                    let c = it.next().unwrap();
                    if c == '\u{00a0}' {
                        // If an error has already been reported on non-breaking
                        // space characters earlier in the file, treat all
                        // subsequent occurrences as whitespace.
                        if self.nbsp_is_ws {
                            preceded_by_ws = true;
                            continue;
                        }
                        self.nbsp_is_ws = true;
                    }
                    let repeats = it.take_while(|c1| *c1 == c).count();
                    self.psess.dcx.accumulate(UnknownTokenStart {
                        span: Span::new(
                            start,
                            self.pos + BytePos::from_usize(repeats * c.len_utf8()),
                        ),
                        escaped: escaped_char(c),
                    });

                    swallow_next_invalid = repeats;
                    preceded_by_ws = true;
                    continue;
                }

                skc_lexer::TokenKind::EndOfFile => EndOfFile,
            };

            let span = Span::new(start, self.pos);
            return (Token::new(kind, span), preceded_by_ws);
        }
    }

    fn cook_lexer_lit_const(
        &self,
        start: BytePos,
        end: BytePos,
        kind: skc_lexer::LitConstKind,
    ) -> (LitConstKind, Symbol) {
        match kind {
            skc_lexer::LitConstKind::Int { empty_int } => {
                let mut kind = LitConstKind::Int;
                if empty_int {
                    self.psess.dcx.accumulate(NoDigitsLiteral {
                        span: Span::new(start, end),
                    });
                    kind = LitConstKind::Error;
                }
                (kind, self.symbol_from_to(start, end))
            }
            skc_lexer::LitConstKind::Float { empty_exp } => {
                let mut kind = LitConstKind::Float;
                if empty_exp {
                    self.psess.dcx.accumulate(NoDigitsLiteral {
                        span: Span::new(start, end),
                    });
                    kind = LitConstKind::Error;
                }
                (kind, self.symbol_from_to(start, end))
            }
        }
    }

    fn src_index(&self, pos: BytePos) -> usize {
        (pos - self.start_pos).to_usize()
    }

    fn str_from_to_end(&self, start: BytePos) -> &'ci str {
        &self.src[self.src_index(start)..]
    }

    fn symbol_from_to(&self, start: BytePos, end: BytePos) -> Symbol {
        Symbol::intern(self.psess.sym_interner, self.str_from_to(start, end))
    }

    fn str_from_to(&self, start: BytePos, end: BytePos) -> &'ci str {
        &self.src[self.src_index(start)..self.src_index(end)]
    }
}
