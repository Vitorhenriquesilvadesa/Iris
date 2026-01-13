use common::ast::{Spanned, item::ItemKind};

use crate::parser::{ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(super) fn parse_item(&mut self) -> ParseResult<Spanned<ItemKind>> {
        let stmt = self.parse_stmt()?;
        Some(Spanned::new(ItemKind::Stmt(Box::new(stmt.node)), stmt.span))
    }
}
