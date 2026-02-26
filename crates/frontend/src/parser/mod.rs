use common::{
    ast::{Ast, Spanned, item::ItemKind},
    diagnostic::{Diagnostic, codes::UNEXPECTED_TOKEN},
    source::SourceFileId,
    span::Span,
    token::{Token, TokenKind},
};

mod diagnostic;
mod error;
mod expression;
mod item;
pub mod parse;
mod statement;
mod stream;

pub use parse::*;

use crate::parser::{diagnostic::map_parse_error, error::ParseError, stream::TokenStream};

type ParseResult<T> = Option<T>;

pub(crate) struct ParserOutput {
    pub ast: Ast,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct Parser<'a> {
    stream: TokenStream,
    items: Vec<Spanned<ItemKind>>,
    source_id: SourceFileId,
    diagnostics: Vec<Diagnostic>,
    pub(super) source_text: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(stream: TokenStream, source_text: &'a str, source_file_id: SourceFileId) -> Self {
        Self {
            stream,
            source_text,
            items: Vec::new(),
            diagnostics: Vec::new(),
            source_id: source_file_id,
        }
    }

    pub(crate) fn run(mut self) -> ParserOutput {
        while !self.stream().is_eof() {
            match self.parse_item() {
                Some(item) => self.items.push(item),
                None => self.recover(),
            }
        }

        ParserOutput {
            ast: Ast::new(self.items),
            diagnostics: self.diagnostics,
        }
    }

    pub(super) fn stream_mut(&mut self) -> &mut TokenStream {
        &mut self.stream
    }

    pub(super) fn stream(&self) -> &TokenStream {
        &self.stream
    }

    pub(super) fn expect_bump_with_flag(
        &mut self,
        kind: TokenKind,
        emmit_error: bool,
    ) -> Option<&Token> {
        let (current_kind, span) = match self.stream().peek() {
            Some(tok) => (tok.kind, tok.span),
            None => (TokenKind::Eof, Span::new(self.source_text.len(), 1)),
        };

        if current_kind == kind {
            return self.stream_mut().bump();
        }

        if emmit_error {
            self.report_error(
                ParseError::Expected {
                    expected: kind,
                    found: current_kind,
                    code: UNEXPECTED_TOKEN,
                },
                span,
            );
        }

        None
    }

    pub(super) fn expect_bump(&mut self, kind: TokenKind) -> Option<&Token> {
        self.expect_bump_with_flag(kind, true)
    }

    pub(super) fn try_expect(&mut self, kinds: &[TokenKind]) -> bool {
        match self.stream().peek_kind() {
            Some(kind) => {
                for k in kinds {
                    if *k == kind {
                        self.stream_mut().bump();
                        return true;
                    }
                }
            }
            None => {
                return false;
            }
        }

        false
    }

    pub(super) fn recover(&mut self) {
        while !self.stream.is_eof() {
            let kind = self.stream.peek_kind();

            match kind {
                Some(k) => match k {
                    TokenKind::Let
                    | TokenKind::Import
                    | TokenKind::Type
                    | TokenKind::Int
                    | TokenKind::Float
                    | TokenKind::String
                    | TokenKind::Ident => return,

                    TokenKind::RBrace => {
                        return;
                    }
                    _ => {
                        self.stream.bump();
                    }
                },
                None => break,
            }
        }
    }

    pub(super) fn report_error(&mut self, error: ParseError, span: Span) {
        let diagnostic = map_parse_error(error, self.source_id, span);
        self.diagnostics.push(diagnostic);
    }

    pub(super) fn report(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.stream()
            .peek()
            .map(|t| t.kind == kind)
            .unwrap_or(false)
    }
}
