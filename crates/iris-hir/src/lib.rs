use la_arena::Arena;

use crate::{
    expression::{ExprId, HirExpression},
    item::{HirItem, ItemId},
    statement::{HirStatement, StmtId},
};

pub mod expression;
pub mod globals;
pub mod item;
pub mod module;
pub mod statement;

#[derive(Debug, Clone, Default)]
pub struct Hir {
    pub expressions: Arena<HirExpression>,
    pub statements: Arena<HirStatement>,
    pub items: Arena<HirItem>,
}

impl Hir {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate_expr(&mut self, expr: HirExpression) -> ExprId {
        self.expressions.alloc(expr)
    }

    pub fn allocate_stmt(&mut self, stmt: HirStatement) -> StmtId {
        self.statements.alloc(stmt)
    }

    pub fn allocate_item(&mut self, item: HirItem) -> ItemId {
        self.items.alloc(item)
    }
}
