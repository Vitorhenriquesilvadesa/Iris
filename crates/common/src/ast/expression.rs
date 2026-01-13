use crate::{
    ast::{Spanned, statement::StmtKind},
    token::TokenKind,
};

/// Represents the various kinds of expressions available in the language.
#[derive(Debug, Clone)]
pub enum ExprKind {
    /// Literal values like integers, floats, strings, or booleans.
    Literal(Literal),

    /// Variable identifiers or names.
    Ident(String),

    /// Binary operations (e.g., `a + b`, `a |> b`).
    Binary(Box<BinaryExpr>),

    /// Unary operations (e.g., `-x`, `!flag`).
    Unary(Box<UnaryExpr>),

    /// Function call expressions (e.g., `my_function(arg1, arg2)`).
    Call(Box<CallExpr>),

    /// A list literal (e.g., `[1, 2, 3]`).
    List(Vec<Spanned<ExprKind>>),

    /// A range expression (e.g., `1..10`).
    Range(Box<RangeExpr>),

    /// A block of code typically containing statements (e.g., `{ let x = 1; x }`).
    Block(Block),

    /// Control flow expression for conditional execution.
    If(Box<IfExpr>),
}

/// Represents literal values in the AST.
#[derive(Debug, Clone)]
pub enum Literal {
    /// 64-bit signed integer literal.
    Int(i64),
    /// 64-bit floating point literal.
    Float(f64),
    /// UTF-8 string literal.
    String(String),
    /// Boolean literal (`true` or `false`).
    Bool(bool),
}

/// Represents a binary operation with two operands.
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    /// The left-hand side operand.
    pub left: Spanned<ExprKind>,
    /// The operator to apply.
    pub op: BinaryOp,
    /// The right-hand side operand.
    pub right: Spanned<ExprKind>,
}

/// Available binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition (`+`).
    Add,
    /// Subtraction (`-`).
    Sub,
    /// Multiplication (`*`).
    Mul,
    /// Division (`/`).
    Div,
    /// Pipe operator (`|>`) for function chaining.
    Pipe,
    /// Range operator (`..`) used in binary context (if applicable).
    Range,
    /// Equality check (`==`).
    Eq,
    /// Inequality check (`!=`).
    Neq,
    /// Less than (`<`).
    Lt,
    /// Greater than (`>`).
    Gt,
    /// Less than or equal to (`<=`).
    Leq,
    /// Greater than or equal to (`>=`).
    Geq,
}

impl BinaryOp {
    pub fn from_token_kind(value: TokenKind) -> Option<Self> {
        match value {
            TokenKind::Plus => Some(Self::Add),
            TokenKind::Minus => Some(Self::Sub),
            TokenKind::Star => Some(Self::Mul),
            TokenKind::Slash => Some(Self::Div),
            TokenKind::Greater => Some(Self::Gt),
            TokenKind::GreaterEq => Some(Self::Geq),
            TokenKind::Less => Some(Self::Lt),
            TokenKind::LessEq => Some(Self::Leq),
            TokenKind::EqualEqual => Some(Self::Eq),
            TokenKind::BangEqual => Some(Self::Neq),
            TokenKind::Range => Some(Self::Range),
            TokenKind::Pipe => Some(Self::Pipe),

            _ => None,
        }
    }
}

/// Represents a unary operation with a single operand.
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    /// The operator to apply.
    pub op: UnaryOp,
    /// The operand expression.
    pub expr: Spanned<ExprKind>,
}

/// Available unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Negation (`-`), flips the sign of a number.
    Neg,
    /// Logical NOT (`!`), flips a boolean value.
    Not,
    /// Only to follow math concepts.
    Plus,
}

impl UnaryOp {
    pub fn from_token_kind(value: TokenKind) -> Option<Self> {
        match value {
            TokenKind::Plus => Some(Self::Plus),
            TokenKind::Minus => Some(Self::Neg),
            TokenKind::Not => Some(Self::Not),
            _ => None,
        }
    }
}

/// Represents a function call.
#[derive(Debug, Clone)]
pub struct CallExpr {
    /// The expression being called (usually an identifier).
    pub callee: Spanned<ExprKind>,
    /// The list of arguments passed to the function.
    pub args: Vec<Spanned<ExprKind>>,
}

/// Represents a block of code enclosed in braces `{ ... }`.
#[derive(Debug, Clone)]
pub struct Block {
    /// The list of statements contained within the block.
    pub stmts: Vec<Spanned<StmtKind>>,
}

/// Represents a range expression, typically used for slicing or iteration.
/// Both start and end are optional to support open ranges like `1..` or `..10`.
#[derive(Debug, Clone)]
pub struct RangeExpr {
    /// The inclusive start of the range.
    pub start: Option<Spanned<ExprKind>>,
    /// The exclusive end of the range.
    pub end: Option<Spanned<ExprKind>>,
}

/// Represents an `if` expression (and optional `else`).
#[derive(Debug, Clone)]
pub struct IfExpr {
    /// The condition to evaluate.
    pub condition: Spanned<ExprKind>,
    /// The block to execute if the condition is true.
    pub then_branch: Block,
    /// The optional expression to execute if the condition is false.
    /// This is an `ExprKind` to allow chaining `else if` structures smoothly,
    /// as well as `else { block }`.
    pub else_branch: Option<Spanned<ExprKind>>,
}
