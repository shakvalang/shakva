use memchr::memchr;
use std::str::Chars;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
#[derive(Debug)]
pub struct Cursor<'src> {
    len_remaining: usize,
    /// Iterator over chars. Slightly faster than a &str.
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    pub fn as_str(&self) -> &'src str {
        self.chars.as_str()
    }

    /// Peeks the next symbol from the input stream without consuming it.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_at_eof` method.
    pub(super) fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub(super) fn second(&self) -> char {
        // `.next()` optimizes better than `.nth(1)`
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    /// Moves to the next character.
    pub(super) fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Eats symbols while predicate returns true or until the end of file is
    /// reached.
    pub(super) fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.first()) && !self.is_at_eof() {
            self.bump();
        }
    }

    pub(super) fn eat_until(&mut self, byte: u8) {
        self.chars = match memchr(byte, self.as_str().as_bytes()) {
            Some(index) => self.as_str()[index..].chars(),
            None => "".chars(),
        }
    }

    /// Returns amount of already bumped symbols.
    pub(super) fn bumped_len(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    /// Resets the number of bytes consumed to 0.
    pub(super) fn reset_len_remaining(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Checks if there is nothing more to consume.
    fn is_at_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}

const EOF_CHAR: char = '\0';
