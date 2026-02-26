#[derive(Debug, Clone)]
pub enum HirError {
    TupleExpression,
    InvalidAssignTarget,
    SymbolNotFound(String),
}
