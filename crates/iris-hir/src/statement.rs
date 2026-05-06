use iris_interner::SymbolId;
use la_arena::Idx;

use crate::expression::ExprId;

pub type StmtId = Idx<HirStatement>;

#[derive(Debug, Clone)]
pub enum HirStatement {
    Expression(ExprId),
    Let {
        symbol: SymbolId,
        initializer: Option<ExprId>,
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
