//! Lexical token definitions for Iris.

use std::fmt::Display;

use iris_span::Span;

/// A lexical category produced by the lexer.
///
/// `TokenKind` classifies source text without attaching semantic meaning.
/// Textual content is always retrieved via the token's `Span`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // ============ Delimiters ============
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // ============ Punctuation ============
    Comma,     // ,
    Dot,       // .
    Colon,     // :
    Semicolon, // ;
    At,        // @
    Hash,      // #

    // ============ Arithmetic Operators ============
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    // ============ Comparison Operators ============
    EqualEqual, // ==
    BangEqual,  // !=
    Less,       // <
    LessEq,     // <=
    Greater,    // >
    GreaterEq,  // >=

    // ============ Assignment Operators ============
    Equal,           // =
    PlusEqual,       // +=
    MinusEqual,      // -=
    StarEqual,       // *=
    SlashEqual,      // /=
    PercentEqual,    // %=
    BitwiseAndEqual, // &=
    BitwiseOrEqual,  // |=
    LShiftEqual,     // <<=
    RShiftEqual,     // >>=

    // ============ Logical Operators ============
    And, // and
    Or,  // or
    Not, // !

    // ============ Bitwise Operators ============
    BitwiseAnd, // &
    BitwiseOr,  // |
    LShift,     // <<
    RShift,     // >>

    // ============ Special Operators / Symbols ============
    Arrow,      // ->
    FatArrow,   // =>
    Range,      // ..
    Optional,   // ?   (Result/Option propagation)
    PipeGt,     // |>  (optional pipeline operator)
    Pipe,       // |   (closures: |x| ...)
    PipePipe,   // ||  (zero-arg closure delimiter)
    Underscore, // _  (wildcard)

    // ============ Keywords (core) ============
    Module, // module
    Import, // import
    As,     // as
    Pub,    // pub

    Type,      // type
    Enum,      // enum
    Interface, // interface

    Fn,   // fn
    Task, // task
    Caps, // caps

    Let,   // let
    Mut,   // mut
    Const, // const

    If,    // if
    Else,  // else
    Match, // match
    For,   // for
    While, // while
    In,    // in

    Return,   // return
    Break,    // break
    Continue, // continue

    Try,   // try
    Catch, // catch
    Fail,  // fail
    Defer, // defer

    Using, // using (optional sugar; harmless to keep)

    // ============ Literals ============
    Ident,
    Int,
    Float,
    String,
    Boolean, // true/false

    // ============ Type Tokens ============
    Fallible,
    FallibleOptional,

    // ============ Special Tokens ============
    Eof,
    Impl,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        match self {
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            LBracket => write!(f, "["),
            RBracket => write!(f, "]"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Colon => write!(f, ":"),
            Semicolon => write!(f, ";"),
            At => write!(f, "@"),
            Hash => write!(f, "#"),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Percent => write!(f, "%"),
            EqualEqual => write!(f, "=="),
            BangEqual => write!(f, "!="),
            Less => write!(f, "<"),
            LessEq => write!(f, "<="),
            Greater => write!(f, ">"),
            GreaterEq => write!(f, ">="),
            Equal => write!(f, "="),
            PlusEqual => write!(f, "+="),
            MinusEqual => write!(f, "-="),
            StarEqual => write!(f, "*="),
            SlashEqual => write!(f, "/="),
            PercentEqual => write!(f, "%="),
            BitwiseAndEqual => write!(f, "&="),
            BitwiseOrEqual => write!(f, "|="),
            LShiftEqual => write!(f, "<<="),
            RShiftEqual => write!(f, ">>="),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Not => write!(f, "!"),
            BitwiseAnd => write!(f, "&"),
            BitwiseOr => write!(f, "|"),
            LShift => write!(f, "<<"),
            RShift => write!(f, ">>"),
            Arrow => write!(f, "->"),
            FatArrow => write!(f, "=>"),
            Range => write!(f, ".."),
            Optional => write!(f, "?"),
            PipeGt => write!(f, "|>"),
            Pipe => write!(f, "|"),
            PipePipe => write!(f, "||"),
            Underscore => write!(f, "_"),
            Module => write!(f, "module"),
            Import => write!(f, "import"),
            As => write!(f, "as"),
            Pub => write!(f, "pub"),
            Type => write!(f, "type"),
            Enum => write!(f, "enum"),
            Interface => write!(f, "interface"),
            Fn => write!(f, "fn"),
            Task => write!(f, "task"),
            Caps => write!(f, "caps"),
            Let => write!(f, "let"),
            Mut => write!(f, "mut"),
            Const => write!(f, "const"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Match => write!(f, "match"),
            For => write!(f, "for"),
            While => write!(f, "while"),
            In => write!(f, "in"),
            Return => write!(f, "return"),
            Break => write!(f, "break"),
            Continue => write!(f, "continue"),
            Try => write!(f, "try"),
            Catch => write!(f, "catch"),
            Fail => write!(f, "fail"),
            Defer => write!(f, "defer"),
            Using => write!(f, "using"),
            Ident => write!(f, "identifier"),
            Int => write!(f, "integer"),
            Float => write!(f, "float"),
            String => write!(f, "string"),
            Boolean => write!(f, "boolean"),
            Eof => write!(f, "end of file"),
            Fallible => write!(f, "!"),
            FallibleOptional => write!(f, "!?"),
            Impl => write!(f, "impl"),
        }
    }
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
