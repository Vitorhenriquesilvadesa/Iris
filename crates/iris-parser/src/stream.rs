#![allow(dead_code)]

use std::sync::Arc;

use iris_diagnostic::{Diagnostic, DiagnosticResult};
use iris_span::source_file::SourceFileId;
use iris_syntax::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenStream {
    tokens: Arc<Vec<Token>>,
    pos: usize,
    source_id: SourceFileId,
}

#[derive(Debug, Copy, Clone)]
pub struct Checkpoint(usize);

impl TokenStream {
    pub fn new(tokens: Arc<Vec<Token>>, source: SourceFileId) -> Self {
        Self {
            tokens,
            pos: 0,
            source_id: source,
        }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn is_eof(&self) -> bool {
        if self.pos >= self.tokens.len() {
            return true;
        }

        self.peek_kind().unwrap_or(TokenKind::Eof) == TokenKind::Eof
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.pos + n)
    }

    pub fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(|t| t.kind)
    }

    pub fn bump(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.pos)
    }

    pub fn rewind(&mut self, checkpoint: Checkpoint) {
        self.pos = checkpoint.0;
    }

    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }

    pub fn expect(&mut self, kind: TokenKind) -> DiagnosticResult<&Token> {
        match self.peek() {
            Some(tok) if tok.kind == kind => Ok(self.bump().unwrap()),
            Some(tok) => Err(Box::new(
                Diagnostic::error(format!("expected token '{}', found '{}'", kind, tok.kind))
                    .with_primary_span(tok.span)
                    .with_label(tok.span, "unexpected token here")
                    .with_file(self.source_id),
            )),
            None => {
                let mut diag =
                    Diagnostic::error(format!("expected token '{}', found 'end of file'", kind))
                        .with_file(self.source_id);
                if let Some(tok) = self.last() {
                    diag = diag
                        .with_primary_span(tok.span)
                        .with_label(tok.span, "unexpected token here")
                }
                Err(Box::new(diag))
            }
        }
    }
}
