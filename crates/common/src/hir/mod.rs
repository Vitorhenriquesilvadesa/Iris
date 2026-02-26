use std::collections::HashMap;

use la_arena::Arena;

use crate::{
    hir::{
        expression::{ExprId, Expression},
        item::{HirItem, HirModel, ItemId},
        statement::{Statement, StmtId},
    },
    interner::SymbolId,
};

pub mod expression;
pub mod globals;
pub mod item;
pub mod module;
pub mod statement;

#[derive(Debug, Clone, Default)]
pub struct Hir {
    pub expressions: Arena<Expression>,
    pub statements: Arena<Statement>,
    pub items: Arena<HirItem>,
    pub models: HashMap<SymbolId, HirModel>,
    pub globals: HashMap<SymbolId, ExprId>,
}

impl Hir {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn allocate_model(&mut self, id: SymbolId, model: HirModel) {
        self.models.insert(id, model);
    }

    pub fn allocate_expr(&mut self, expr: Expression) -> ExprId {
        self.expressions.alloc(expr)
    }

    pub fn allocate_stmt(&mut self, stmt: Statement) -> StmtId {
        self.statements.alloc(stmt)
    }

    pub fn allocate_item(&mut self, item: HirItem) -> ItemId {
        self.items.alloc(item)
    }
}
