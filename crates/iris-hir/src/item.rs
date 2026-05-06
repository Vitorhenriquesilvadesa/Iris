use iris_interner::SymbolId;
use la_arena::Idx;

use crate::{
    expression::{ExprId, Resolution},
    statement::StmtId,
};

pub type ItemId = Idx<HirItem>;

#[derive(Debug, Clone)]
pub enum HirItem {
    Type(HirType),
    Import(HirImport),
    Function(HirFunction),
    Stmt(StmtId),
    Let {
        name: SymbolId,
        resolution: Resolution,
        initializer: ExprId,
    },
    Error,
}

#[derive(Debug, Clone)]
pub struct HirImport {
    pub module_path: Vec<SymbolId>,
    pub alias: Option<SymbolId>,
    pub exposed: Vec<SymbolId>,
}

#[derive(Debug, Clone)]
pub struct HirType {
    pub name: SymbolId,
    pub fields: Vec<(SymbolId, Option<HirTypeInfo>)>,
}

#[derive(Debug, Clone)]
pub enum HirTypeInfo {
    /// Interned named type
    Named(SymbolId),

    /// Array desugared: []T -> Array(T)
    Array(Box<HirTypeInfo>),

    /// Optional desugared: T? -> Optional(T)
    Optional(Box<HirTypeInfo>),

    /// Fallible desugared: T! -> Fallible(T)
    Fallible(Box<HirTypeInfo>),
}

#[derive(Debug, Clone)]
pub struct HirParam {
    pub name: SymbolId,
    pub kind: Option<HirTypeInfo>,
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: SymbolId,
    pub params: Vec<HirParam>,
    pub return_type: Option<HirTypeInfo>,
    pub body: Vec<StmtId>,
}
