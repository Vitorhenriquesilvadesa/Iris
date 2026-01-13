//! Lexical token definitions.

use crate::span::Span;

/// A lexical category produced by the lexer.
///
/// `TokenKind` classifies source text without attaching semantic meaning.
/// Textual content is always retrieved via the token's `Span`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Identifiers & literals
    Ident,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    BoolLiteral(bool),

    // Keywords
    Let,
    Import,
    Model,
    Extend,
    If,
    Else,
    For,
    While,
    Return,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Range,

    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Not,

    Arrow,
    Pipe,

    AndAnd,
    OrOr,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Punctuation
    Dot,
    Comma,
    Colon,
    Semicolon,

    // Special
    Eof,
}

/// A lexical token with its source location.
#[derive(Debug, Clone)]
pub struct Token {
    /// The token category.
    pub kind: TokenKind,

    /// The source span corresponding to this token.
    pub span: Span,
}

impl Token {
    /// Creates a new token with the given kind and span.
    #[inline]
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns `true` if this token is of the given kind.
    #[inline]
    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    /// Returns `true` if this token is the end-of-file marker.
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::Eof
    }
}
