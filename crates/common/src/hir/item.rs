use std::collections::HashMap;

use la_arena::Idx;

use crate::{
    hir::{
        expression::{ExprId, Resolution},
        statement::StmtId,
    },
    interner::SymbolId,
};

pub type ItemId = Idx<HirItem>;

#[derive(Debug, Clone)]
pub enum HirItem {
    Model(HirModel),
    Import(HirImport),
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
pub struct HirModel {
    pub name: SymbolId,
    pub fields: Vec<SymbolId>,
    pub methods: HashMap<SymbolId, HirMethod>,
}

#[derive(Debug, Clone)]
pub struct HirMethod {
    pub name: SymbolId,
    pub params: Vec<SymbolId>,
    pub body: Vec<StmtId>,
}
