use std::sync::Arc;

use common::{
    diagnostic::Diagnostics,
    hir::{
        expression::Resolution,
        globals::DefId,
        item::HirItem,
        module::{DefVisibility, HirModule},
    },
    module::ModuleId,
    source::SourceFileId,
};
use compiler_api::queries::{
    AnalysisResult, QueryResult, hir::HirQueries, source::SourceQueries, symbol::SymbolQueries,
};

pub fn gen_hir_module<Ctx: HirQueries + SymbolQueries + SourceQueries>(
    ctx: &Ctx,
    module_id: ModuleId,
) -> QueryResult<HirModule> {
    let file = ctx.source_by_id(SourceFileId::from_module(&module_id))?;
    let hir = ctx.hir_of(module_id)?;
    let name = ctx.intern_symbol(file.value.file_name()?);

    let mut module = HirModule::new(module_id, name);

    for (symbol, expr_id) in &hir.value.globals {
        let def_id = DefId::new(module_id, module.items.len() as u32);
        let item = HirItem::Let {
            name: *symbol,
            resolution: Resolution::Global(def_id),
            initializer: *expr_id,
        };
        module.define_global(*symbol, item, DefVisibility::Public);
    }

    for (id, model) in &hir.value.models {
        let item = HirItem::Model(model.clone());
        module.define_global(*id, item, DefVisibility::Public);
    }

    for (_, item) in hir.value.items.iter() {
        match item {
            HirItem::Stmt(stmt) => {
                module.push_body_statement(*stmt);
            }
            _ => {}
        };
    }

    Ok(AnalysisResult {
        diagnostics: Arc::new(Diagnostics::new(vec![])),
        value: Arc::new(module),
    })
}
