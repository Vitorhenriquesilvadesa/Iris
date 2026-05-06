use iris_ast::expression::ExprKind;
use iris_db::symbol::SymbolQueries;
use iris_hir::expression::ExprId;

use crate::{HirResult, hir_gen::HirGenerator};

impl<'a, Ctx> HirGenerator<'a, Ctx>
where
    Ctx: SymbolQueries,
{
    pub(crate) fn gen_expr_hir(&self, node: &ExprKind) -> HirResult<ExprId> {
        todo!()
    }
}
