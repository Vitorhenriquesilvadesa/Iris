use common::{hir::globals::GlobalScope, module::ModuleId};

use crate::queries::QueryResult;

pub trait ScopeQueries {
    fn globals_of(&self, module: ModuleId) -> QueryResult<GlobalScope>;
}
