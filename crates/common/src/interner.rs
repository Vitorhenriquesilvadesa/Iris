use std::{collections::HashMap, sync::Arc};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SymbolId(u32);

#[derive(Debug, Clone, Default)]
pub struct SymbolInterner {
    map: HashMap<Arc<str>, SymbolId>,
    strings: Vec<Arc<str>>,
}

impl SymbolInterner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, text: &str) -> SymbolId {
        if let Some(&id) = self.map.get(text) {
            return id;
        }

        let arc: Arc<str> = Arc::from(text);
        let id = SymbolId(self.strings.len() as u32);

        self.strings.push(arc.clone());
        self.map.insert(arc, id);

        id
    }

    pub fn resolve(&self, id: SymbolId) -> Arc<str> {
        self.strings[id.0 as usize].clone()
    }
}
