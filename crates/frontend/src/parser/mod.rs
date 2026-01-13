use common::{
    ast::{Ast, Spanned, item::ItemKind},
    diagnostic::Diagnostic,
    source::SourceFileId,
    span::Span,
    token::TokenKind,
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

    pub(super) fn try_expect(&self, kinds: &[TokenKind]) -> bool {
        match self.stream().peek_kind() {
            Some(kind) => {
                for k in kinds {
                    if *k == kind {
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
                    | TokenKind::Model
                    | TokenKind::Import
                    | TokenKind::IntLiteral
                    | TokenKind::FloatLiteral
                    | TokenKind::StringLiteral
                    | TokenKind::Ident => return,

                    TokenKind::RBrace | TokenKind::Semicolon => {
                        self.stream.bump();
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
}
