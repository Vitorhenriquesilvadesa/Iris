pub(crate) mod cursor;
pub(crate) mod diagnostic;
pub(crate) mod error;
pub mod lex;

pub use lex::*;

use crate::lexer::cursor::Cursor;
use crate::lexer::diagnostic::map_lex_error;
use crate::lexer::error::LexError;
use common::diagnostic::Diagnostic;
use common::source::SourceFileId;
use common::span::Span;
use common::token::{Token, TokenKind};

#[derive(Debug)]
pub struct LexOutput {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    source_id: SourceFileId,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Lexer<'a> {
    pub fn new(source_id: SourceFileId, text: &'a str) -> Self {
        Self {
            cursor: Cursor::new(text),
            source_id,
            tokens: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn run(mut self) -> LexOutput {
        while !self.cursor.is_eof() {
            self.lex_next_token();
        }

        let eof_pos = self.cursor.position();
        self.tokens
            .push(Token::new(TokenKind::Eof, Span::new(eof_pos, 1)));

        LexOutput {
            tokens: self.tokens,
            diagnostics: self.diagnostics,
        }
    }

    fn lex_next_token(&mut self) {
        let start = self.cursor.position();

        let ch = match self.cursor.peek() {
            Some(c) => c,
            None => return,
        };

        match ch {
            c if c.is_whitespace() => {
                self.cursor.bump_while(char::is_whitespace);
            }

            c if is_ident_start(c) => {
                self.lex_identifier_or_keyword(start);
            }

            c if c.is_ascii_digit() => {
                self.lex_number(start);
            }

            '"' => {
                self.lex_string(start);
            }

            _ => {
                self.lex_operator_or_punctuation(start);
            }
        }
    }

    fn lex_identifier_or_keyword(&mut self, start: usize) {
        self.cursor.bump();
        self.cursor.bump_while(is_ident_continue);

        let end = self.cursor.position();
        let text = self.cursor.slice(start, end);

        // Single `_` is the wildcard token; `_foo` is a normal identifier.
        if text == "_" {
            self.push_token(TokenKind::Underscore, start, end);
            return;
        }

        let kind = match text {
            // ── Module system ──
            "module" => TokenKind::Module,
            "import" => TokenKind::Import,
            "as" => TokenKind::As,
            "pub" => TokenKind::Pub,

            // ── Declarations ──
            "type" => TokenKind::Type,
            "enum" => TokenKind::Enum,
            "interface" => TokenKind::Interface,
            "fn" => TokenKind::Fn,
            "task" => TokenKind::Task,
            "caps" => TokenKind::Caps,
            "impl" => TokenKind::Impl,

            // ── Bindings ──
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "const" => TokenKind::Const,

            // ── Control flow ──
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "in" => TokenKind::In,

            // ── Jumps ──
            "return" => TokenKind::Return,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,

            // ── Error handling ──
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "fail" => TokenKind::Fail,
            "defer" => TokenKind::Defer,

            // ── Logical operators (keyword form) ──
            "and" => TokenKind::And,
            "or" => TokenKind::Or,

            // ── Other keywords ──
            "using" => TokenKind::Using,

            // ── Boolean literals ──
            "true" | "false" => TokenKind::Boolean,

            _ => TokenKind::Ident,
        };

        self.push_token(kind, start, end);
    }

    fn lex_number(&mut self, start: usize) {
        self.cursor.bump_while(|c| c.is_ascii_digit());

        let mut kind = TokenKind::Int;

        let is_float = if self.cursor.peek() == Some('.') {
            let rest = self.cursor.remaining();

            matches!(rest.chars().nth(1), Some(c) if c.is_ascii_digit())
        } else {
            false
        };

        if is_float {
            self.cursor.bump();
            self.cursor.bump_while(|c| c.is_ascii_digit());
            kind = TokenKind::Float;
        }

        let end = self.cursor.position();
        self.push_token(kind, start, end);
    }

    fn lex_string(&mut self, start: usize) {
        self.cursor.bump();

        while let Some(ch) = self.cursor.peek() {
            match ch {
                '"' => {
                    self.cursor.bump();
                    let end = self.cursor.position();
                    self.push_token(TokenKind::String, start, end);
                    return;
                }
                '\\' => {
                    self.cursor.bump();
                    self.cursor.bump();
                }
                _ => {
                    self.cursor.bump();
                }
            }
        }

        self.report_error(LexError::UnterminatedString, start);
    }

    fn lex_operator_or_punctuation(&mut self, start: usize) {
        let first_char = self.cursor.bump().unwrap();

        match first_char {
            // ── Shift / comparison operators ──
            '>' => {
                if self.cursor.bump_if('>') {
                    if self.cursor.bump_if('=') {
                        self.push_token(TokenKind::RShiftEqual, start, self.cursor.position());
                    } else {
                        self.push_token(TokenKind::RShift, start, self.cursor.position());
                    }
                } else if self.cursor.bump_if('=') {
                    self.push_token(TokenKind::GreaterEq, start, self.cursor.position());
                } else {
                    self.push_simple(TokenKind::Greater, start);
                }
            }

            '<' => {
                if self.cursor.bump_if('<') {
                    if self.cursor.bump_if('=') {
                        self.push_token(TokenKind::LShiftEqual, start, self.cursor.position());
                    } else {
                        self.push_token(TokenKind::LShift, start, self.cursor.position());
                    }
                } else if self.cursor.bump_if('=') {
                    self.push_token(TokenKind::LessEq, start, self.cursor.position());
                } else {
                    self.push_simple(TokenKind::Less, start);
                }
            }

            // ── Pipe family: |>  ||  |=  | ──
            '|' => {
                if self.cursor.bump_if('>') {
                    self.push_token(TokenKind::PipeGt, start, self.cursor.position());
                } else if self.cursor.bump_if('|') {
                    self.push_token(TokenKind::PipePipe, start, self.cursor.position());
                } else if self.cursor.bump_if('=') {
                    self.push_token(TokenKind::BitwiseOrEqual, start, self.cursor.position());
                } else {
                    self.push_simple(TokenKind::Pipe, start);
                }
            }

            // ── Equality / assignment ──
            '=' => {
                if self.cursor.bump_if('>') {
                    self.push_token(TokenKind::FatArrow, start, self.cursor.position());
                } else if self.cursor.bump_if('=') {
                    self.push_token(TokenKind::EqualEqual, start, self.cursor.position());
                } else {
                    self.push_simple(TokenKind::Equal, start);
                }
            }

            '!' => {
                if self.cursor.bump_if('=') {
                    self.push_token(TokenKind::BangEqual, start, self.cursor.position());
                } else if self.cursor.bump_if('?') {
                    self.push_token(TokenKind::FallibleOptional, start, self.cursor.position());
                } else {
                    self.push_token(TokenKind::Not, start, self.cursor.position());
                }
            }

            // ── Arithmetic / compound assignment ──
            '+' => self.lex_compound(start, '=', TokenKind::PlusEqual, TokenKind::Plus),

            '-' => {
                if self.cursor.bump_if('>') {
                    self.push_token(TokenKind::Arrow, start, self.cursor.position());
                } else if self.cursor.bump_if('=') {
                    self.push_token(TokenKind::MinusEqual, start, self.cursor.position());
                } else {
                    self.push_simple(TokenKind::Minus, start);
                }
            }

            '*' => self.lex_compound(start, '=', TokenKind::StarEqual, TokenKind::Star),

            '/' => {
                // Line comment: skip until end of line.
                if self.cursor.bump_if('/') {
                    self.cursor.bump_while(|c| c != '\n');
                } else if self.cursor.bump_if('=') {
                    self.push_token(TokenKind::SlashEqual, start, self.cursor.position());
                } else {
                    self.push_simple(TokenKind::Slash, start);
                }
            }

            '%' => self.lex_compound(start, '=', TokenKind::PercentEqual, TokenKind::Percent),

            '&' => self.lex_compound(
                start,
                '=',
                TokenKind::BitwiseAndEqual,
                TokenKind::BitwiseAnd,
            ),

            '.' => self.lex_compound(start, '.', TokenKind::Range, TokenKind::Dot),

            // ── Delimiters ──
            '(' => self.push_simple(TokenKind::LParen, start),
            ')' => self.push_simple(TokenKind::RParen, start),
            '{' => self.push_simple(TokenKind::LBrace, start),
            '}' => self.push_simple(TokenKind::RBrace, start),
            '[' => self.push_simple(TokenKind::LBracket, start),
            ']' => self.push_simple(TokenKind::RBracket, start),

            // ── Punctuation ──
            ',' => self.push_simple(TokenKind::Comma, start),
            ':' => self.push_simple(TokenKind::Colon, start),
            ';' => self.push_simple(TokenKind::Semicolon, start),
            '#' => self.push_simple(TokenKind::Hash, start),
            '@' => self.push_simple(TokenKind::At, start),
            '?' => self.push_simple(TokenKind::Optional, start),

            other => {
                self.report_error(LexError::UnexpectedChar(other), start);
            }
        }
    }

    fn push_simple(&mut self, kind: TokenKind, start: usize) {
        let end = self.cursor.position();
        self.push_token(kind, start, end);
    }

    fn push_token(&mut self, kind: TokenKind, start: usize, end: usize) {
        self.tokens
            .push(Token::new(kind, Span::new(start, end - start)));
    }

    fn report_error(&mut self, error: LexError, start: usize) {
        let end = self.cursor.position();
        self.diagnostics.push(map_lex_error(
            error,
            self.source_id,
            Span::new(start, end - start),
        ));
    }

    fn lex_compound(
        &mut self,
        start: usize,
        next_char: char,
        compound_kind: TokenKind,
        simple_kind: TokenKind,
    ) {
        if self.cursor.bump_if(next_char) {
            self.push_token(compound_kind, start, self.cursor.position());
        } else {
            self.push_simple(simple_kind, start);
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) || c.is_ascii_digit()
}
