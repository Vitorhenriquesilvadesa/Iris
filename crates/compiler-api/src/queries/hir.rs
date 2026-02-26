use common::{
    hir::{Hir, module::HirModule},
    module::ModuleId,
};

use crate::queries::QueryResult;

pub trait HirQueries {
    fn hir_of(&self, module_id: ModuleId) -> QueryResult<Hir>;
    fn module_hir(&self, module_id: ModuleId) -> QueryResult<HirModule>;
}
