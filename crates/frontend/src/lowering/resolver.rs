use std::collections::HashMap;

use common::{
    hir::{
        expression::LocalId,
        globals::{DefId, GlobalScope},
    },
    interner::SymbolId,
    module::ModuleId,
};

#[derive(Debug, Clone)]
pub struct Resolver<'a> {
    next_id: u32,
    scopes: Vec<HashMap<SymbolId, LocalId>>,
    globals: &'a GlobalScope,
    pending_globals: HashMap<SymbolId, DefId>,
    module_id: ModuleId,
    global_index: u32,
}

impl<'a> Resolver<'a> {
    pub fn new(globals: &'a GlobalScope) -> Self {
        Self {
            next_id: 0,
            scopes: vec![],
            globals,
            pending_globals: HashMap::new(),
            module_id: ModuleId::new(0),
            global_index: 0,
        }
    }

    pub fn declare_global(&mut self, symbol: SymbolId) {
        let def_id = DefId::new(self.module_id, self.global_index);
        self.pending_globals.insert(symbol, def_id);
        self.global_index += 1;
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, symbol: SymbolId) -> LocalId {
        let id = LocalId(self.next_id);
        self.next_id += 1;

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(symbol, id);
        }

        id
    }

    pub fn resolve_global(&self, name: &SymbolId) -> Option<DefId> {
        if let Some(def_id) = self.pending_globals.get(name) {
            return Some(*def_id);
        }
        self.globals.resolve(name)
    }

    pub fn resolve_local(&self, name: &SymbolId) -> Option<LocalId> {
        for scope in self.scopes.iter().rev() {
            if let Some(id) = scope.get(name) {
                return Some(*id);
            }
        }
        None
    }
}
