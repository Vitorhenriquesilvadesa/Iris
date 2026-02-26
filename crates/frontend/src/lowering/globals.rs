use std::sync::Arc;

use common::{
    ast::{item::ItemKind, statement::StmtKind},
    diagnostic::Diagnostics,
    hir::globals::{DefId, GlobalScope},
    module::ModuleId,
    source::SourceFileId,
};
use compiler_api::queries::{
    AnalysisResult, QueryResult, parser::ParserQueries, symbol::SymbolQueries,
};

pub fn globals_of<Ctx: ParserQueries + SymbolQueries>(
    module: ModuleId,
    ctx: &Ctx,
) -> QueryResult<GlobalScope> {
    let ast = ctx.ast_of(SourceFileId::new(module.as_u32()))?.value;

    let mut globals = GlobalScope::new();
    let mut next_index = 0;

    for item in &ast.items {
        match &item.node {
            ItemKind::Model(model) => {
                let id = ctx.intern_symbol(&model.name.node);
                globals.insert(id, DefId::new(module, next_index));
                next_index += 1;
            }

            ItemKind::Import(import) => {
                if let Some(alias) = &import.alias {
                    let id = ctx.intern_symbol(&alias.node);
                    globals.insert(id, DefId::new(module, next_index));
                    next_index += 1;
                }
                for exposed in &import.exposing {
                    let id = ctx.intern_symbol(&exposed.node);
                    globals.insert(id, DefId::new(module, next_index));
                    next_index += 1;
                }
            }

            _ => {}
        }
    }

    return Ok(AnalysisResult {
        value: Arc::new(globals),
        diagnostics: Arc::new(Diagnostics::new(vec![])),
    });
}
