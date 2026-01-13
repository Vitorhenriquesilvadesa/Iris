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
            .push(Token::new(TokenKind::Eof, Span::new(eof_pos, eof_pos)));

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
        self.cursor.bump(); // first char
        self.cursor.bump_while(is_ident_continue);

        let end = self.cursor.position();
        let text = self.cursor.slice(start, end);

        let kind = match text {
            "let" => TokenKind::Let,
            "import" => TokenKind::Import,
            "model" => TokenKind::Model,
            "extend" => TokenKind::Extend,
            "true" => TokenKind::BoolLiteral(true),
            "false" => TokenKind::BoolLiteral(false),
            _ => TokenKind::Ident,
        };

        self.push_token(kind, start, end);
    }

    fn lex_number(&mut self, start: usize) {
        self.cursor.bump_while(|c| c.is_ascii_digit());

        let mut kind = TokenKind::IntLiteral;

        let is_float = if self.cursor.peek() == Some('.') {
            let rest = self.cursor.remaining();

            matches!(rest.chars().nth(1), Some(c) if c.is_ascii_digit())
        } else {
            false
        };

        if is_float {
            self.cursor.bump();
            self.cursor.bump_while(|c| c.is_ascii_digit());
            kind = TokenKind::FloatLiteral;
        }

        let end = self.cursor.position();
        self.push_token(kind, start, end);
    }

    fn lex_string(&mut self, start: usize) {
        self.cursor.bump(); // opening quote

        while let Some(ch) = self.cursor.peek() {
            match ch {
                '"' => {
                    self.cursor.bump();
                    let end = self.cursor.position();
                    self.push_token(TokenKind::StringLiteral, start, end);
                    return;
                }
                '\\' => {
                    self.cursor.bump();
                    self.cursor.bump(); // escaped char
                }
                _ => {
                    self.cursor.bump();
                }
            }
        }

        self.report_error(LexError::UnterminatedString, start);
    }

    fn lex_operator_or_punctuation(&mut self, start: usize) {
        match self.cursor.bump().unwrap() {
            '|' if self.cursor.bump_if('>') => {
                self.push_token(TokenKind::Pipe, start, self.cursor.position());
            }

            '-' if self.cursor.bump_if('>') => {
                self.push_token(TokenKind::Arrow, start, self.cursor.position());
            }

            '=' if self.cursor.bump_if('=') => {
                self.push_token(TokenKind::EqualEqual, start, self.cursor.position());
            }

            '.' if self.cursor.bump_if('.') => {
                self.push_token(TokenKind::Range, start, self.cursor.position());
            }

            '>' if self.cursor.bump_if('=') => {
                self.push_token(TokenKind::GreaterEq, start, self.cursor.position());
            }

            '<' if self.cursor.bump_if('=') => {
                self.push_token(TokenKind::LessEq, start, self.cursor.position());
            }

            '(' => self.push_simple(TokenKind::LParen, start),
            ')' => self.push_simple(TokenKind::RParen, start),
            '{' => self.push_simple(TokenKind::LBrace, start),
            '}' => self.push_simple(TokenKind::RBrace, start),
            '[' => self.push_simple(TokenKind::LBracket, start),
            ']' => self.push_simple(TokenKind::RBracket, start),
            ',' => self.push_simple(TokenKind::Comma, start),
            ':' => self.push_simple(TokenKind::Colon, start),
            '.' => self.push_simple(TokenKind::Dot, start),
            '=' => self.push_simple(TokenKind::Equal, start),
            '+' => self.push_simple(TokenKind::Plus, start),
            '-' => self.push_simple(TokenKind::Minus, start),
            '*' => self.push_simple(TokenKind::Star, start),
            '/' => self.push_simple(TokenKind::Slash, start),
            '>' => self.push_simple(TokenKind::Greater, start),
            '<' => self.push_simple(TokenKind::Less, start),
            '!' => self.push_simple(TokenKind::Not, start),

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
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) || c.is_ascii_digit()
}
