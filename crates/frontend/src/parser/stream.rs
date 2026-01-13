#![allow(dead_code)]

use std::sync::Arc;

use common::{
    diagnostic::Diagnostic,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct TokenStream {
    tokens: Arc<Vec<Token>>,
    pos: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Checkpoint(usize);

impl TokenStream {
    pub fn new(tokens: Arc<Vec<Token>>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn is_eof(&self) -> bool {
        if self.pos >= self.tokens.len() {
            return true;
        }

        return self.peek_kind().unwrap_or(TokenKind::Eof) == TokenKind::Eof;
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

    pub fn expect(&mut self, kind: TokenKind) -> Result<&Token, Diagnostic> {
        match self.peek() {
            Some(tok) if tok.kind == kind => Ok(self.bump().unwrap()),
            Some(tok) => Err(Diagnostic::error(format!(
                "expected token {:?}, found {:?}",
                kind, tok.kind
            ))
            .with_primary_span(tok.span)),
            None => Err(Diagnostic::error(format!(
                "expected token {:?}, found end of file",
                kind
            ))),
        }
    }
}
