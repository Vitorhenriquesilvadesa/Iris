#![allow(dead_code)]

use common::{
    ast::{
        Ast, Spanned,
        expression::{ExprKind, Literal},
        item::ItemKind,
        statement::StmtKind,
    },
    diagnostic::Diagnostic,
    source::SourceFileId,
    span::Span,
    token::TokenKind,
};

use crate::parser::{
    ParserOutput, diagnostic::map_parse_error, error::ParseError, stream::TokenStream,
};

type ParseResult<T> = Option<T>;

pub struct Parser<'a> {
    stream: TokenStream,
    source_id: SourceFileId,
    items: Vec<Spanned<ItemKind>>,
    diagnostics: Vec<Diagnostic>,
    source_text: &'a str,
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

        return ParserOutput {
            ast: Ast::new(self.items),
            diagnostics: self.diagnostics,
        };
    }

    fn parse_item(&mut self) -> ParseResult<Spanned<ItemKind>> {
        let stmt = self.parse_stmt()?;
        return Some(Spanned::new(ItemKind::Stmt(Box::new(stmt.node)), stmt.span));
    }

    fn parse_stmt(&mut self) -> ParseResult<Spanned<StmtKind>> {
        let expr = self.parse_expr()?;
        let span = expr.span;
        return Some(Spanned::new(StmtKind::Expr(expr), span));
    }

    fn parse_expr(&mut self) -> ParseResult<Spanned<ExprKind>> {
        let expr = self.parse_literal()?;
        return Some(expr);
    }

    fn parse_literal(&mut self) -> ParseResult<Spanned<ExprKind>> {
        let token = self.stream.peek()?;
        let span = token.span;
        match token.kind {
            TokenKind::IntLiteral => {
                self.stream.bump();
                let text = span.slice(self.source_text);
                let value = text.parse::<i64>().unwrap_or(0);
                Some(Spanned::new(ExprKind::Literal(Literal::Int(value)), span))
            }
            _ => {
                self.report_error(
                    ParseError::UnexpectedToken {
                        found: self.stream().peek_kind().unwrap(),
                    },
                    self.stream().peek().unwrap().span,
                );
                None
            }
        }
    }

    fn stream_mut(&mut self) -> &TokenStream {
        &mut self.stream
    }

    fn stream(&self) -> &TokenStream {
        &self.stream
    }

    fn recover(&mut self) {
        while !self.stream.is_eof() {
            let kind = self.stream.peek_kind();

            match kind {
                Some(k) => match k {
                    TokenKind::Let | TokenKind::Model | TokenKind::Import => return,
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

    fn report_error(&mut self, error: ParseError, span: Span) {
        let diagnostic = map_parse_error(error, self.source_id, span);
        self.diagnostics.push(diagnostic);
    }
}
