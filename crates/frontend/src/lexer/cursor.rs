#![allow(dead_code)]

use std::str::CharIndices;

/// A UTF-8–aware cursor for iterating over a string slice.
///
/// `Cursor` provides controlled traversal over a `&str`, allowing
/// peeking, consuming characters, and tracking the current byte position.
/// It is commonly useful in lexers and parsers.
#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    /// The original input string.
    input: &'a str,

    /// Iterator over character indices of the input.
    chars: CharIndices<'a>,

    /// Current byte position in the input.
    pos: usize,

    /// Cached result of a peeked character.
    peeked: Option<(usize, char)>,
}

impl<'a> Cursor<'a> {
    /// Creates a new `Cursor` over the given input string.
    ///
    /// The cursor starts at position `0` and no character is pre-peeked.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices(),
            pos: 0,
            peeked: None,
        }
    }

    /// Returns the next character without consuming it.
    ///
    /// This method caches the next character so that a subsequent
    /// call to [`bump`] will consume the same character.
    ///
    /// Returns `None` if the end of input has been reached.
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked.map(|(_, ch)| ch)
    }

    /// Consumes and returns the next character.
    ///
    /// Advances the internal byte position by the UTF-8 length of
    /// the consumed character.
    ///
    /// Returns `None` if the end of input has been reached.
    #[inline]
    pub fn bump(&mut self) -> Option<char> {
        let next = if let Some(peeked) = self.peeked.take() {
            Some(peeked)
        } else {
            self.chars.next()
        };

        match next {
            Some((idx, ch)) => {
                self.pos = idx + ch.len_utf8();
                Some(ch)
            }
            None => {
                self.pos = self.input.len();
                None
            }
        }
    }

    /// Returns `true` if the cursor has reached the end of the input.
    #[inline]
    pub fn is_eof(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Returns the current byte position in the input string.
    #[inline]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Returns a slice of the input string from `start` to `end`.
    ///
    /// Both `start` and `end` are byte indices and must lie on UTF-8
    /// character boundaries.
    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.input[start..end]
    }

    /// Consumes characters while the given predicate returns `true`.
    ///
    /// Stops at the first character for which the predicate returns `false`
    /// or at the end of input.
    pub fn bump_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(ch) = self.peek() {
            if !predicate(ch) {
                break;
            }
            self.bump();
        }
    }

    /// Consumes the next character only if it matches `expected`.
    ///
    /// Returns `true` if the character was consumed, or `false` otherwise.
    pub fn bump_if(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(ch) if ch == expected => {
                self.bump();
                true
            }
            _ => false,
        }
    }

    /// Returns the remaining unconsumed portion of the input string.
    pub fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }
}
