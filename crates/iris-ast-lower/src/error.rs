#![allow(dead_code)]

pub enum HirError {
    TupleExpression,
    InvalidAssignTarget,
    SymbolNotFound(String),
}
