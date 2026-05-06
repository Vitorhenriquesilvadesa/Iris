use iris_hir::{globals::GlobalScope, module::ModuleId};

use crate::QueryResult;

pub trait ScopeQueries {
    fn globals_of(&self, module: ModuleId) -> QueryResult<GlobalScope>;
}
