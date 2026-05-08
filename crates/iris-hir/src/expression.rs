use iris_ast::expression::{AssignmentOp, BinaryOp, UnaryOp};
use iris_interner::SymbolId;
use la_arena::Idx;

use crate::{globals::DefId, statement::StmtId};

pub type ExprId = Idx<HirExpression>;

/// Represents literal values in the AST.
#[derive(Debug, Clone)]
pub enum HirLiteral {
    /// 64-bit signed integer literal.
    Int(i64),
    /// 64-bit floating point literal.
    Float(f64),
    /// UTF-8 string literal.
    String(String),
    /// Boolean literal (`true` or `false`).
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub u32);

#[derive(Debug, Clone, Copy)]
pub enum Resolution {
    Local(LocalId),
    Global(DefId),
    Unresolved,
}

#[derive(Debug, Clone)]
pub struct HirLambdaParam {
    pub symbol: SymbolId,
    pub local: LocalId,
}

#[derive(Debug, Clone)]
pub enum HirExpression {
    Literal(HirLiteral),

    Ident(SymbolId),

    Variable(Resolution),

    Unary {
        op: HirUnaryOp,
        expr: ExprId,
    },

    Binary {
        op: HirBinaryOp,
        lhs: ExprId,
        rhs: ExprId,
    },

    Call {
        callee: ExprId,
        args: Vec<ExprId>,
    },

    Lambda {
        params: Vec<HirLambdaParam>,
        body: StmtId,
    },

    List {
        elements: Vec<ExprId>,
    },

    Range {
        start: ExprId,
        end: ExprId,
    },

    Assign {
        assignee: ExprId,
        op: HirAssignOp,
        value: ExprId,
    },

    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirAssignOp {
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

impl HirAssignOp {
    pub fn from_ast_assign_op(kind: AssignmentOp) -> Self {
        match kind {
            AssignmentOp::Assign => Self::Assign,
            AssignmentOp::AddAssign => Self::AddAssign,
            AssignmentOp::SubAssign => Self::SubAssign,
            AssignmentOp::MulAssign => Self::MulAssign,
            AssignmentOp::DivAssign => Self::DivAssign,
            AssignmentOp::ModAssign => Self::ModAssign,
            AssignmentOp::BitAndAssign => Self::BitAndAssign,
            AssignmentOp::BitOrAssign => Self::BitOrAssign,
            AssignmentOp::LShiftAssign => Self::LShiftAssign,
            AssignmentOp::RShitAssign => Self::RShitAssign,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirUnaryOp {
    Neg,
    Not,
    Plus,
    Try,
}

impl HirUnaryOp {
    pub fn from_ast_unary_op(op: UnaryOp) -> Self {
        match op {
            UnaryOp::Neg => HirUnaryOp::Neg,
            UnaryOp::Not => HirUnaryOp::Not,
            UnaryOp::Plus => HirUnaryOp::Plus,
            UnaryOp::Try => HirUnaryOp::Try,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    LShift,
    RShift,
    BitOr,
    BitAnd,
    Error,
}

impl HirBinaryOp {
    pub fn from_ast_op(op: &BinaryOp) -> Self {
        match op {
            BinaryOp::Add => Self::Add,
            BinaryOp::Sub => Self::Sub,
            BinaryOp::Mul => Self::Mul,
            BinaryOp::Div => Self::Div,
            BinaryOp::Mod => Self::Mod,
            BinaryOp::Eq => Self::Eq,
            BinaryOp::Neq => Self::Neq,
            BinaryOp::Lt => Self::Lt,
            BinaryOp::Gt => Self::Gt,
            BinaryOp::Leq => Self::Le,
            BinaryOp::Geq => Self::Ge,
            BinaryOp::And => Self::And,
            BinaryOp::Or => Self::Or,
            BinaryOp::LShift => Self::LShift,
            BinaryOp::RShift => Self::RShift,
            BinaryOp::BitwiseAnd => Self::BitAnd,
            BinaryOp::BitwiseOr => Self::BitOr,
            BinaryOp::Pipe | BinaryOp::Range | BinaryOp::Comma | BinaryOp::Catch => Self::Error,
        }
    }
}
