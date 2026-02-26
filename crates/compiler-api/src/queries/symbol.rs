use std::sync::Arc;

use common::interner::SymbolId;

pub trait SymbolQueries {
    fn intern_symbol(&self, text: &str) -> SymbolId;

    fn symbol_text(&self, id: SymbolId) -> Arc<str>;
}
