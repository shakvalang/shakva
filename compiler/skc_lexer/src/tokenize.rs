use crate::{Cursor, LitConstKind, Token, TokenKind};

use TokenKind::*;

impl<'src> Cursor<'src> {
    /// Creates an iterator that produces tokens from the input string.
    pub fn tokenize(input: &'src str) -> impl Iterator<Item = Token> {
        let mut cursor = Self::new(input);
        std::iter::from_fn(move || {
            let token = cursor.bump_token();
            if token.kind != EndOfFile {
                Some(token)
            } else {
                None
            }
        })
    }

    /// Parses a token from the input string.
    pub fn bump_token(&mut self) -> Token {
        let Some(first_char) = self.bump() else {
            return Token {
                kind: EndOfFile,
                len: 0,
            };
        };

        let kind = match first_char {
            CR_CHAR => match self.first() {
                LF_CHAR => {
                    self.bump();

                    Newline
                }
                _ => Newline,
            },
            LF_CHAR => Newline,

            c if is_whitespace(c) => self.whitespace(),

            '0'..='9' => LitConst {
                kind: self.number(first_char),
            },

            ';' => Semi,

            '-' => Minus,
            '+' => Plus,
            '/' => Slash,
            '*' => Star,
            '(' => LParen,
            ')' => RParen,

            _ => Unknown,
        };

        let len = self.bumped_len();
        self.reset_len_remaining();

        Token { kind, len }
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);

        Whitespace
    }

    fn number(&mut self, first_digit: char) -> LitConstKind {
        use LitConstKind::*;
        if first_digit == '0' {
            match self.first() {
                // Not a base prefix; eat additional digits
                '0'..='9' | '_' => {
                    self.eat_dec_digits();
                }
                // Also not a base prefix; nothing more to do here.
                '.' | 'e' | 'E' => {}
                // Just a 0.
                _ => {
                    return Int { empty_int: false };
                }
            }
        } else {
            self.eat_dec_digits();
        }

        match self.first() {
            // Don't be greedy if this is actually an
            // integer literal followed by a range pattern (`0..2`)
            '.' if self.second() != '.' => {
                // might have stuff after the ., and if it does, it needs to
                // start with a number
                self.bump();
                let mut empty_exp = false;
                if self.first().is_ascii_digit() {
                    self.eat_dec_digits();
                    match self.first() {
                        'e' | 'E' => {
                            self.bump();
                            empty_exp = !self.eat_float_exp();
                        }
                        _ => (),
                    }
                }
                Float { empty_exp }
            }
            'e' | 'E' => {
                self.bump();
                let empty_exp = !self.eat_float_exp();
                Float { empty_exp }
            }
            _ => Int { empty_int: false },
        }
    }

    fn eat_dec_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => return has_digits,
            }
        }
    }

    /// Eats the float exponent. Returns true if at least one digit was met,
    /// and returns false otherwise.
    fn eat_float_exp(&mut self) -> bool {
        if self.first() == '+' || self.first() == '-' {
            self.bump();
        }

        self.eat_dec_digits()
    }
}

const LF_CHAR: char = '\u{000A}';
const CR_CHAR: char = '\u{000D}';

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        | '\u{0020}' // space
        | '\u{0009}' // tab
        | '\u{000C}' // form feed
    )
}
