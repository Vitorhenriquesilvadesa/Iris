pub enum HirError {
    TupleExpression,
    InvalidAssignTarget,
    SymbolNotFound(String),
}
