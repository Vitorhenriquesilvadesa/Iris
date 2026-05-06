use iris_syntax::TokenKind;

use crate::{Expression, Spanned, Statement, item::AstTypeInfo, statement::StmtKind};

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub assignee: Expression,
    pub op: AssignmentOp,
    pub value: Expression,
}

/// Represents the various kinds of expressions available in the language.
#[derive(Debug, Clone)]
pub enum ExprKind {
    /// Literal values like integers, floats, strings, or booleans.
    Literal(Literal),

    /// Variable identifiers or names.
    Ident(Spanned<String>),

    /// Binary operations (e.g., `a + b`, `a |> b`).
    Binary(Box<BinaryExpr>),

    /// Unary operations (e.g., `-x`, `!flag`).
    Unary(Box<UnaryExpr>),

    /// Function call expressions (e.g., `my_function(arg1, arg2)`).
    Call(Box<CallExpr>),

    /// A list literal (e.g., `[1, 2, 3]`).
    List(Vec<Spanned<ExprKind>>),

    /// Control flow expression for conditional execution.
    If(Box<IfExpr>),
    Grouping(Box<Expression>),
    Lambda(Box<LambdaExpr>),
    Assign(Box<AssignmentExpr>),
    Member(Box<MemberExpr>),
}

#[derive(Debug, Clone)]
pub struct MemberExpr {
    pub object: Expression,
    pub member: Spanned<String>,
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

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Spanned<String>,
    pub kind: Option<Spanned<AstTypeInfo>>,
}

#[derive(Debug, Clone)]
pub struct LambdaExpr {
    pub params: Vec<Param>,
    pub body: Statement,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOp {
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MulAssign,    // *=
    DivAssign,    // /=
    ModAssign,    // %=
    BitAndAssign, // &=
    BitOrAssign,  // |=
    LShiftAssign, // <<=
    RShitAssign,  // >>=
}

impl AssignmentOp {
    pub fn from_token(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Equal => Some(Self::Assign),
            TokenKind::PlusEqual => Some(Self::AddAssign),
            TokenKind::MinusEqual => Some(Self::SubAssign),
            TokenKind::StarEqual => Some(Self::MulAssign),
            TokenKind::SlashEqual => Some(Self::DivAssign),
            TokenKind::PercentEqual => Some(Self::ModAssign),
            TokenKind::BitwiseAndEqual => Some(Self::BitAndAssign),
            TokenKind::BitwiseOrEqual => Some(Self::BitOrAssign),
            TokenKind::LShiftEqual => Some(Self::LShiftAssign),
            TokenKind::RShiftEqual => Some(Self::RShitAssign),
            _ => None,
        }
    }
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
    Comma,
    Mod,
    And,
    Or,
    LShift,
    RShift,
    BitwiseAnd,
    BitwiseOr,
    Catch,
}

impl BinaryOp {
    pub fn from_token_kind(value: TokenKind) -> Option<Self> {
        match value {
            TokenKind::Plus => Some(Self::Add),
            TokenKind::Minus => Some(Self::Sub),
            TokenKind::Star => Some(Self::Mul),
            TokenKind::Slash => Some(Self::Div),
            TokenKind::Percent => Some(Self::Mod),
            TokenKind::Greater => Some(Self::Gt),
            TokenKind::GreaterEq => Some(Self::Geq),
            TokenKind::Less => Some(Self::Lt),
            TokenKind::LessEq => Some(Self::Leq),
            TokenKind::EqualEqual => Some(Self::Eq),
            TokenKind::BangEqual => Some(Self::Neq),
            TokenKind::Range => Some(Self::Range),
            TokenKind::PipeGt => Some(Self::Pipe),
            TokenKind::Comma => Some(Self::Comma),
            TokenKind::And => Some(Self::And),
            TokenKind::Or => Some(Self::Or),
            TokenKind::BitwiseAnd => Some(Self::BitwiseAnd),
            TokenKind::Pipe => Some(Self::BitwiseOr),
            TokenKind::LShift => Some(Self::LShift),
            TokenKind::RShift => Some(Self::RShift),
            TokenKind::Catch => Some(Self::Catch),

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

    Try,
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
    pub condition: Spanned<ExprKind>,
    pub then_branch: Spanned<ExprKind>,
    pub else_branch: Spanned<ExprKind>,
}
