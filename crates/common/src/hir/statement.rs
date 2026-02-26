use la_arena::Idx;

use crate::{
    hir::expression::{ExprId, Resolution},
    interner::SymbolId,
};

pub type StmtId = Idx<Statement>;

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(ExprId),
    Let {
        name: Resolution,
        symbol: SymbolId,
        initializer: ExprId,
    },
    If {
        condition: ExprId,
        if_branch: StmtId,
        else_branch: Option<StmtId>,
    },
    Error,
    Scope {
        statements: Vec<StmtId>,
    },
}
