use crate::{
    ast::{
        Expression, Item, Spanned,
        expression::{Block, Param},
    },
    token::TokenKind,
};

/// Represents top-level declarations in a source file.
#[derive(Debug, Clone)]
pub enum ItemKind {
    /// Imports external modules.
    Import(Box<ImportDef>),

    /// Global variable declaration.
    GlobalLet(Box<super::statement::LetStmt>),

    /// Type declaration.
    Type(Box<TypeDef>),

    /// A standalone expression at the top level.
    Stmt(Box<super::statement::StmtKind>),

    /// Function declaration.
    Function(FunctionDef),

    Metadata(MetaDataUsage),

    Impl(ImplDef),
}

#[derive(Debug, Clone)]
pub struct MetaDataUsage {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<MetaArgument>>,
}

#[derive(Debug, Clone)]
pub struct ImplDef {
    pub target: Spanned<AstTypeInfo>,
    pub methods: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct MetaArgument {
    pub name: Spanned<String>,
    pub value: Expression,
}

/// Represents a user-defined type declaration.
///
/// A type is identified by a name and contains a list of fields.
/// Each field reuses [`Param`] to preserve parser and lowering consistency
/// with function parameters.
#[derive(Debug, Clone)]
pub struct TypeDef {
    /// The declared type name.
    pub name: Spanned<String>,

    /// The fields that compose this type.
    pub fields: Vec<Spanned<Param>>,

    pub generics: Vec<Spanned<GenericParam>>,
}

#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct AstTypeInfo {
    pub base: Spanned<AstTypeBase>,
    pub modifier: Spanned<AstTypeModifier>,
    pub generics: Vec<Spanned<AstTypeInfo>>,
}

#[derive(Debug, Clone)]
pub enum TypeFlags {
    Array,
}

#[derive(Debug, Clone)]
pub struct TypeName(pub String);

impl TypeName {
    pub fn inner(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum AstTypeBase {
    Named(TypeName),
    Array(Box<Spanned<AstTypeBase>>),
}

#[derive(Debug, Clone)]
pub enum AstTypeModifier {
    None,
    Optional,
    Fallible,
    FallibleOptional,
}

impl AstTypeModifier {
    pub fn from_token_kind(tk: TokenKind) -> Option<Self> {
        match tk {
            TokenKind::Not => Some(Self::Fallible),
            TokenKind::Optional => Some(Self::Optional),
            TokenKind::FallibleOptional => Some(Self::FallibleOptional),
            _ => None,
        }
    }
}

/// Represents an import declaration.
///
/// Imports can include:
/// - A module path (`path`)
/// - An optional alias (`alias`)
/// - An optional list of directly exposed symbols (`exposing`)
#[derive(Debug, Clone)]
pub struct ImportDef {
    /// The module path (e.g., `["Iris", "Data"]`).
    pub path: Vec<Spanned<String>>,

    /// Optional alias (e.g., `as df`).
    pub alias: Option<Spanned<String>>,

    /// List of symbols imported directly (e.g., `exposing (sum, count)`).
    pub exposing: Vec<Spanned<String>>,
}

/// Represents a function definition (used in methods or potentially top-level).
#[derive(Debug, Clone)]
pub struct FunctionDef {
    /// The function name.
    pub name: Spanned<String>,

    /// The return kind of the function.
    pub return_kind: Spanned<AstTypeInfo>,

    /// The declared function parameters.
    pub params: Vec<Spanned<Param>>,

    /// The function body block.
    pub body: Block,
}
