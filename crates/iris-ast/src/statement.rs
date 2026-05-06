use crate::{Expression, Spanned, Statement, expression::ExprKind};

/// Represents an executable statement used inside functions or scripts.
#[derive(Debug, Clone)]
pub enum StmtKind {
    /// Variable binding (e.g., `let filtered = raw_data |> ...`).
    Let(Box<LetStmt>),

    Block(Vec<Statement>),

    If {
        condition: Expression,
        if_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },

    /// An expression executed for its side effects (e.g., `print(age_mean)`).
    /// Since block returns are implicit in Iris, the last statement can be an Expr.
    Expr(Spanned<ExprKind>),
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    /// The name being bound (e.g., `mean`, `raw_data`).
    pub name: Spanned<String>,
    /// The value being assigned.
    pub initializer: Option<Spanned<ExprKind>>,
}

#[derive(Debug, Clone)]
pub struct TypeInfoAst {
    pub path: Vec<Spanned<String>>,
    pub flags: u32,
}

impl TypeInfoAst {
    // Flags (bitmask)
    pub const FLAG_ERROR: u32 = 1 << 0; // corresponds to !T (fallible)

    #[inline]
    pub fn new(path: Vec<Spanned<String>>) -> Self {
        Self { path, flags: 0 }
    }

    /// Constructs a fallible type `!T` (Zig-like) for the given path.
    #[inline]
    pub fn error(path: Vec<Spanned<String>>) -> Self {
        Self {
            path,
            flags: Self::FLAG_ERROR,
        }
    }

    #[inline]
    pub fn is_error(&self) -> bool {
        (self.flags & Self::FLAG_ERROR) != 0
    }

    #[inline]
    pub fn with_error(mut self) -> Self {
        self.flags |= Self::FLAG_ERROR;
        self
    }

    #[inline]
    pub fn with_flag(mut self, flag: u32) -> Self {
        self.flags |= flag;
        self
    }

    #[inline]
    pub fn without_flag(mut self, flag: u32) -> Self {
        self.flags &= !flag;
        self
    }
}
