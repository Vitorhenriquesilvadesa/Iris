use std::collections::HashMap;

use iris_interner::SymbolId;

use crate::module::ModuleId;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DefId {
    pub module: ModuleId,
    pub index: u32,
}

impl DefId {
    pub fn new(module: ModuleId, index: u32) -> Self {
        Self { module, index }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GlobalScope {
    pub definitions: HashMap<SymbolId, DefId>,
}

impl GlobalScope {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: SymbolId, id: DefId) {
        self.definitions.insert(name, id);
    }

    pub fn resolve(&self, name: &SymbolId) -> Option<DefId> {
        self.definitions.get(name).copied()
    }
}
