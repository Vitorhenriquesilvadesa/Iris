use common::ast::{Spanned, statement::StmtKind};

use crate::parser::{ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(super) fn parse_stmt(&mut self) -> ParseResult<Spanned<StmtKind>> {
        let expr = self.parse_expr()?;
        let span = expr.span;
        Some(Spanned::new(StmtKind::Expr(expr), span))
    }
}
