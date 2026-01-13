use crate::ast::Spanned;
use crate::ast::expression::ExprKind;

/// Represents an executable statement used inside functions or scripts.
#[derive(Debug, Clone)]
pub enum StmtKind {
    /// Variable binding (e.g., `let filtered = raw_data |> ...`).
    Let(Box<LetStmt>),

    /// An expression executed for its side effects (e.g., `print(age_mean)`).
    /// Since block returns are implicit in Iris, the last statement can be an Expr.
    Expr(Spanned<ExprKind>),
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    /// The name being bound (e.g., `mean`, `raw_data`).
    pub name: String,
    /// The value being assigned.
    pub initializer: Spanned<ExprKind>,
}
