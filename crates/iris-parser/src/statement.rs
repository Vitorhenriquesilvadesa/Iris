use iris_ast::{
    Spanned, Statement,
    expression::ExprKind,
    statement::{LetStmt, StmtKind},
};
use iris_syntax::TokenKind;

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(super) fn parse_stmt(&mut self) -> ParseResult<Statement> {
        if let Some(tok) = self.stream().peek_kind() {
            match tok {
                TokenKind::Let => self.parse_let(),
                TokenKind::LBrace => self.parse_block(),
                TokenKind::If => self.parse_if(),
                _ => {
                    let expr = self.parse_expr()?;
                    self.expect_bump(TokenKind::Semicolon)?;
                    let span = expr.span;
                    Some(Spanned::new(StmtKind::Expr(expr), span))
                }
            }
        } else {
            None
        }
    }

    pub(super) fn parse_block(&mut self) -> ParseResult<Statement> {
        let start_token = self.expect_bump(TokenKind::LBrace)?;
        let start_span = start_token.span;

        let mut statements = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.stream.is_eof() {
            let stmt = self.parse_stmt()?;
            statements.push(stmt);
        }

        let end_token = self.expect_bump(TokenKind::RBrace)?;

        let full_span = start_span.merge(&end_token.span);

        Some(Spanned::new(StmtKind::Block(statements), full_span))
    }

    fn parse_let(&mut self) -> ParseResult<Statement> {
        let start = self.expect_bump(TokenKind::Let)?.span;
        let span = self.expect_bump(TokenKind::Ident)?.span;

        let name = span.slice(self.source_text).to_string();

        let initializer: Option<Spanned<ExprKind>> = if self.check(TokenKind::Equal) {
            self.expect_bump(TokenKind::Equal)?;
            Some(self.parse_expr()?)
        } else {
            None
        };

        let end = self.expect_bump(TokenKind::Semicolon)?.span;

        Some(Spanned::new(
            StmtKind::Let(Box::new(LetStmt {
                name: Spanned::new(name, span),
                initializer,
            })),
            start.merge(&end),
        ))
    }

    fn parse_if(&mut self) -> ParseResult<Statement> {
        let mut complete_span = self.expect_bump(TokenKind::If)?.span;
        let condition = self.parse_expr()?;
        let if_branch = self.parse_stmt()?;

        complete_span = complete_span.merge(&if_branch.span);

        let else_branch: Option<Box<Statement>> = self.parse_else()?;

        if let Some(branch) = &else_branch {
            complete_span = complete_span.merge(&branch.span);
        }

        Some(Spanned::new(
            StmtKind::If {
                condition,
                if_branch: Box::new(if_branch),
                else_branch,
            },
            complete_span,
        ))
    }

    fn parse_else(&mut self) -> ParseResult<Option<Box<Statement>>> {
        if self.check(TokenKind::Else) {
            self.expect_bump(TokenKind::Else)?;

            // `else if` desugars into a nested if-statement.
            if self.check(TokenKind::If) {
                let if_stmt = self.parse_if()?;
                return Some(Some(Box::new(if_stmt)));
            }

            let body = self.parse_stmt()?;
            return Some(Some(Box::new(body)));
        }

        Some(None)
    }
}
