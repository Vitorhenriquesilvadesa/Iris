//! Source code span representation.
pub mod line_index;
pub mod source_file;
pub mod source_map;

/// Represents a half-open byte range within a source file.
///
/// A `Span` is defined by a start offset and a length, both in bytes.
/// It does not carry file identity, line, or column information.
///
/// Spans are immutable, lightweight, and copyable.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Span {
    /// Byte offset where the span starts.
    pub start: usize,

    /// Length of the span in bytes.
    pub length: usize,
}

impl Span {
    /// Creates a new span starting at `start` with the given `length`.
    #[inline]
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    /// Creates an empty span at a single byte position.
    #[inline]
    pub fn at(position: usize) -> Self {
        Self {
            start: position,
            length: 0,
        }
    }

    /// Returns the exclusive end offset of the span.
    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.length
    }

    /// Returns `true` if this span has zero length.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns `true` if the given byte offset lies within this span.
    #[inline]
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end()
    }

    /// Returns a span that covers both this span and `other`.
    ///
    /// The resulting span starts at the minimum start offset and
    /// ends at the maximum end offset of the two spans.
    pub fn merge(&self, other: &Span) -> Span {
        let start = self.start.min(other.start);
        let end = self.end().max(other.end());

        Span {
            start,
            length: end - start,
        }
    }

    /// Returns the source slice corresponding to this span.
    ///
    /// # Panics
    ///
    /// Panics if the span is out of bounds for the given source string.
    pub fn slice<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end()]
    }

    /// Computes a span covering all spans produced by the iterator.
    pub fn covering<I>(mut spans: I) -> Option<Span>
    where
        I: Iterator<Item = Span>,
    {
        let first = spans.next()?;
        Some(spans.fold(first, |acc, s| acc.merge(&s)))
    }
}
