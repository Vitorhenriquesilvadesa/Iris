#[warn(dead_code)]
use iris_ast::Item;
use iris_db::symbol::SymbolQueries;
use iris_diagnostic::{Diagnostic, Diagnostics};
use iris_hir::{
    Hir,
    expression::{ExprId, HirExpression},
    item::{HirItem, ItemId},
    statement::{HirStatement, StmtId},
};
use iris_span::{Span, source_file::SourceFileId};

use crate::{diagnostic::map_hir_error, error::HirError};

#[derive(Debug, Clone)]
pub struct HirOutput {
    pub value: Hir,
    pub diagnostics: Diagnostics,
}

#[derive(Debug, Clone)]
pub struct HirGenerator<'a, Ctx>
where
    Ctx: SymbolQueries,
{
    pub(crate) hir: Hir,
    pub(crate) items: &'a [Item],
    pub(crate) ctx: &'a Ctx,
    pub(crate) source_file_id: SourceFileId,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

impl<'a, Ctx> HirGenerator<'a, Ctx>
where
    Ctx: SymbolQueries,
{
    pub fn new(id: SourceFileId, items: &'a [Item], ctx: &'a Ctx) -> Self {
        Self {
            hir: Hir::new(),
            items,
            ctx,
            source_file_id: id,
            diagnostics: vec![],
        }
    }

    pub(crate) fn gen_hir(&mut self) -> HirOutput {
        for i in self.items {
            self.gen_hir_for(i);
        }

        HirOutput {
            value: self.hir.clone(),
            diagnostics: Diagnostics::new(self.diagnostics.clone()),
        }
    }

    pub(crate) fn report_error(&mut self, error: HirError, span: Span) {
        let diagnostic = map_hir_error(error, self.source_file_id, span);
        self.diagnostics.push(diagnostic);
    }

    pub(crate) fn allocate_item(&mut self, item: HirItem) -> ItemId {
        self.hir.allocate_item(item)
    }

    pub(crate) fn allocate_stmt(&mut self, stmt: HirStatement) -> StmtId {
        self.hir.allocate_stmt(stmt)
    }

    pub(crate) fn allocate_expr(&mut self, expr: HirExpression) -> ExprId {
        self.hir.allocate_expr(expr)
    }
}
