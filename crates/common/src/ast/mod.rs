pub mod expression;
pub mod item;
pub mod statement;

use crate::ast::expression::ExprKind;
use crate::ast::statement::StmtKind;
use crate::span::Span;

use crate::ast::item::ItemKind;

pub type Expression = Spanned<ExprKind>;
pub type Statement = Spanned<StmtKind>;
pub type Item = Spanned<ItemKind>;

/// Represents the root of a parsed Iris source file.
///
/// A program in Iris is a sequence of top-level items, which can be
/// declarations (like imports or models) or executable statements (script mode).
#[derive(Debug, Clone)]
pub struct Ast {
    /// The collection of top-level items found in the file.
    pub items: Vec<Spanned<ItemKind>>,
}

impl Ast {
    /// Creates a new Program AST node.
    pub fn new(items: Vec<Spanned<ItemKind>>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}
