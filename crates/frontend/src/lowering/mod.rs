// use std::sync::Arc;

// use common::{diagnostic::Diagnostics, hir::Hir, module::ModuleId, source::SourceFileId};
// use compiler_api::queries::{
//     AnalysisResult, QueryResult, parser::ParserQueries, scope::ScopeQueries, symbol::SymbolQueries,
// };

// use crate::lowering::hir_gen::HirGenerator;

// mod diagnostic;
// mod error;
// pub mod globals;
// mod hir_gen;
// pub mod module;
// mod resolver;

// pub fn generate_hir<Ctx: ParserQueries + ScopeQueries + SymbolQueries>(
//     ctx: &Ctx,
//     source_file_id: SourceFileId,
// ) -> QueryResult<Hir> {
//     let ast = ctx
//         .ast_of(source_file_id)
//         .map_err(|e| format!("{}", e))?
//         .value;
//     let items = &ast.items;

//     let globals = ctx
//         .globals_of(ModuleId::new(source_file_id.as_u32()))?
//         .value;

//     let ir_generator = HirGenerator::new(&items, source_file_id, ctx, &globals);
//     let output = ir_generator.generate();

//     Ok(AnalysisResult {
//         value: Arc::new(output.hir),
//         diagnostics: Arc::new(Diagnostics::new(output.diagnostics)),
//     })
// }
