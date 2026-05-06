use std::collections::HashMap;

use iris_interner::SymbolId;
use iris_span::source_file::SourceFileId;

use crate::{
    expression::ExprId,
    globals::DefId,
    item::{HirImport, HirItem, HirType, ItemId},
    statement::StmtId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefVisibility {
    Public,
    Private,
}
impl DefVisibility {
    fn is_public(&self) -> bool {
        self == &DefVisibility::Public
    }
}

#[derive(Debug, Clone)]
pub enum HirModuleItem {
    Model(HirType),
    Let(ExprId),
    Import(HirImport),
}

#[derive(Debug, Clone)]
pub struct HirModule {
    pub id: ModuleId,
    pub name: SymbolId,
    pub scope: HashMap<SymbolId, DefId>,
    pub items: Vec<HirItem>,
    pub body: Vec<StmtId>,
    pub exports: Vec<DefId>,
}

impl HirModule {
    pub fn new(id: ModuleId, name: SymbolId) -> Self {
        Self {
            id,
            name,
            scope: HashMap::new(),
            items: vec![],
            exports: vec![],
            body: vec![],
        }
    }

    pub fn define_global(
        &mut self,
        name: SymbolId,
        item: HirItem,
        visibility: DefVisibility,
    ) -> DefId {
        let index = self.items.len() as u32;

        let def_id = DefId::new(self.id, index);

        self.items.push(item);

        self.scope.insert(name, def_id);

        if visibility.is_public() {
            self.exports.push(def_id);
        }

        def_id
    }

    pub fn define_import(&mut self, import: HirImport) {
        let index = self.items.len() as u32;
        let def_id = DefId::new(self.id, index);

        self.items.push(HirItem::Import(import.clone()));

        if let Some(alias) = import.alias {
            self.scope.insert(alias, def_id);
        } else if let Some(last_segment) = import.module_path.last() {
            self.scope.insert(*last_segment, def_id);
        }

        for exposed_name in import.exposed {
            self.scope.insert(exposed_name, def_id);
        }
    }

    pub fn push_body_statement(&mut self, stmt: StmtId) {
        self.body.push(stmt);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId(u32);

impl ModuleId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn from_file(file: &SourceFileId) -> Self {
        Self(file.as_u32())
    }

    pub fn invalid() -> Self {
        Self(u32::MAX)
    }

    pub fn is_invalid(&self) -> bool {
        self.0 == u32::MAX
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub id: ModuleId,
    definitions: Vec<ItemId>,
    imports: Vec<HirImport>,
}

impl Module {
    pub fn new(id: SourceFileId) -> Self {
        Self {
            definitions: vec![],
            id: ModuleId::from_file(&id),
            imports: vec![],
        }
    }

    pub fn invalid() -> Self {
        Self::new(SourceFileId::invalid())
    }

    pub fn is_invalid(&self) -> bool {
        self.id.0 == u32::MAX
    }

    pub fn push_import(&mut self, import: HirImport) {
        self.imports.push(import);
    }

    pub fn push_definition(&mut self, def: ItemId) {
        self.definitions.push(def)
    }
}
