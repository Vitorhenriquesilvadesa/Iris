use std::{hash::Hash, sync::Arc};

use dashmap::DashMap;
use iris_db::QueryResult;

use crate::query::{reporter::ErrorReporter, slot::QuerySlot};

#[derive(Debug)]
pub struct QueryCache<K: Eq + Hash, T: Clone> {
    slots: DashMap<K, Arc<QuerySlot<T>>>,
}

impl<K, T: Clone> QueryCache<K, T>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            slots: DashMap::new(),
        }
    }

    pub fn get_or_compute<F, Ctx>(&self, key: K, ctx: &Ctx, compute: F) -> QueryResult<T>
    where
        Ctx: ErrorReporter,
        F: FnOnce() -> QueryResult<T>,
    {
        use dashmap::mapref::entry::Entry;

        match self.slots.entry(key.clone()) {
            Entry::Occupied(entry) => entry.get().wait(),

            Entry::Vacant(entry) => {
                let slot = Arc::new(QuerySlot::new());
                entry.insert(slot.clone());

                let result = compute();

                match &result {
                    Ok(value) => {
                        ctx.emit_diagnostics(&value.diagnostics);
                        slot.complete(value.clone())
                    }
                    //FIXME: Change format macro usage to more apropriate type
                    Err(diags) => slot.fail(diags.clone()),
                }

                result
            }
        }
    }
}
