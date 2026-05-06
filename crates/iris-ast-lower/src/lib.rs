use std::sync::Arc;

use iris_db::{AnalysisResult, QueryResult, parser::ParserQueries, symbol::SymbolQueries};
use iris_hir::Hir;
use iris_span::source_file::SourceFileId;

use crate::hir_gen::HirGenerator;

mod diagnostic;
mod error;
mod expression;
mod hir_gen;
mod items;
mod statement;

pub(crate) type HirResult<T> = Option<T>;

pub fn gen_hir_of<Ctx>(file: SourceFileId, ctx: &Ctx) -> QueryResult<Hir>
where
    Ctx: ParserQueries + SymbolQueries,
{
    let ast = ctx.ast_of(file)?.value;

    let mut generator = HirGenerator::new(file, &ast.items, ctx);

    let hir = generator.gen_hir();

    Ok(AnalysisResult {
        diagnostics: Arc::new(hir.diagnostics),
        value: Arc::new(hir.value),
    })
}
