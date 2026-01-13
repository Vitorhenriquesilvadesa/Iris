use crate::ast::expression::Block;

/// Represents top-level declarations in a source file.
#[derive(Debug, Clone)]
pub enum ItemKind {
    /// Imports external modules (e.g., `import Iris.Data as df`).
    Import(Box<ImportDef>),

    /// Defines a new data model (e.g., `model LinearModel { ... }`).
    Model(Box<ModelDef>),

    /// Extends an existing type with new methods (e.g., `extend DataFrame { ... }`).
    Extend(Box<ExtendDef>),

    /// A standalone expression at the top level (e.g., `print(age_mean)`).
    Stmt(Box<super::statement::StmtKind>),
}

#[derive(Debug, Clone)]
pub struct ImportDef {
    /// The module path (e.g., `["Iris", "Data"]`).
    pub path: Vec<String>,
    /// Optional alias (e.g., `as df`).
    pub alias: Option<String>,
    /// List of symbols imported directly (e.g., `exposing (sum, count)`).
    pub exposing: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDef {
    pub name: String,
    /// Fields defined in the model (e.g., `coefficients`, `rows`).
    /// Types appear optional or inferred in your snippet.
    pub fields: Vec<String>,
    /// Methods defined inside the model block.
    pub methods: Vec<FunctionDef>,
}

#[derive(Debug, Clone)]
pub struct ExtendDef {
    /// The name of the type being extended (e.g., `DataFrame`).
    pub target: String,
    /// The new methods being added.
    pub methods: Vec<FunctionDef>,
}

/// Represents a function definition (used in methods or potentially top-level).
/// Note: Your snippet uses `let x = lambda` mostly, but `plot() {}` inside models is a function def.
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>, // Parameter names
    pub body: Block,
}
