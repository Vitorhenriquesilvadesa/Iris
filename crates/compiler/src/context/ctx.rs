#![allow(dead_code)]

use std::{
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

use common::{
    ast::Ast,
    diagnostic::Diagnostics,
    hir::{Hir, globals::GlobalScope, module::HirModule},
    interner::{SymbolId, SymbolInterner},
    module::ModuleId,
    source::{SourceFile, SourceFileId},
    token::Token,
};
use compiler_api::queries::{
    AnalysisResult, QueryResult, lexer::LexerQueries, parser::ParserQueries, source::SourceQueries,
    symbol::SymbolQueries,
};

use crate::{
    context::CompilationOutput,
    map::source_map::SourceMap,
    query::{cache::QueryCache, registry::QueryKey, reporter::ErrorReporter},
};

#[derive(Debug)]
pub struct CompilerContext {
    source_map: SourceMap,
    tokens_cache: QueryCache<QueryKey, Vec<Token>>,
    ast_cache: QueryCache<QueryKey, Ast>,
    hir_cache: QueryCache<QueryKey, Hir>,
    module_cache: QueryCache<QueryKey, HirModule>,
    globals_cache: QueryCache<QueryKey, GlobalScope>,
    interner: Arc<RwLock<SymbolInterner>>,
    pub diagnostics: Mutex<Diagnostics>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            source_map: SourceMap::new(),
            tokens_cache: QueryCache::new(),
            ast_cache: QueryCache::new(),
            module_cache: QueryCache::new(),
            hir_cache: QueryCache::new(),
            globals_cache: QueryCache::new(),
            diagnostics: Mutex::new(Diagnostics::new(vec![])),
            interner: Arc::new(RwLock::new(SymbolInterner::new())),
        }
    }

    pub fn attach_root_file<P: Into<PathBuf>>(&self, path: P) -> QueryResult<SourceFileId> {
        match self.source_map.load_file(path) {
            Ok(file) => Ok(AnalysisResult {
                value: Arc::new(file.value.id()),
                diagnostics: Arc::new(Diagnostics::new(vec![])),
            }),
            Err(e) => Err(e.clone()),
        }
    }

    pub fn compile(&self, id: SourceFileId) -> QueryResult<CompilationOutput> {
        let file_res = self.source_by_id(id)?;

        println!("Compiling '{}'", file_res.value.text());

        let source_file = file_res.value;

        self.ast_of(source_file.id())
    }

    pub fn source_map(&self) -> &SourceMap {
        &self.source_map
    }
}

impl SourceQueries for CompilerContext {
    fn source_by_path(&self, path: &str) -> QueryResult<SourceFile> {
        let result = self.source_map.load_file(path)?;
        self.emit_diagnostics(&result.diagnostics);
        Ok(result)
    }

    fn source_by_id(&self, id: SourceFileId) -> QueryResult<SourceFile> {
        let result = self.source_map.load_by_id(id)?;
        self.emit_diagnostics(&result.diagnostics);
        Ok(result)
    }
}

impl LexerQueries for CompilerContext {
    fn tokens_of(&self, file: SourceFileId) -> QueryResult<Vec<Token>> {
        let query = QueryKey::tokens(file);

        let result = self
            .tokens_cache
            .get_or_compute(query, self, || frontend::lexer::lex(self, file))?;

        Ok(result)
    }
}

impl ParserQueries for CompilerContext {
    fn ast_of(&self, file: SourceFileId) -> QueryResult<Ast> {
        let query = QueryKey::ast(ModuleId::new(file.as_u32()));

        let result = self
            .ast_cache
            .get_or_compute(query, self, || frontend::parser::parse(self, file))?;

        Ok(result)
    }
}

// impl HirQueries for CompilerContext {
//     fn hir_of(&self, module: ModuleId) -> QueryResult<Hir> {
//         let query = QueryKey::hir(module);

//         let result = self.hir_cache.get_or_compute(query, self, || {
//             frontend::lowering::generate_hir(self, SourceFileId::from_module(&module))
//         })?;

//         Ok(result)
//     }

//     fn module_hir(&self, module: ModuleId) -> QueryResult<common::hir::module::HirModule> {
//         let query = QueryKey::module(module);

//         let result = self.module_cache.get_or_compute(query, self, || {
//             frontend::lowering::module::gen_hir_module(self, module)
//         })?;

//         Ok(result)
//     }
// }

impl ErrorReporter for CompilerContext {
    fn emit_diagnostics(&self, diags: &Diagnostics) {
        let mut lock = self.diagnostics.lock().unwrap();
        lock.extend(diags);
    }
}

// impl ScopeQueries for CompilerContext {
//     fn globals_of(&self, module: ModuleId) -> QueryResult<common::hir::globals::GlobalScope> {
//         let query = QueryKey::globals(module);

//         let result = self
//             .globals_cache
//             .get_or_compute(query, self, || globals_of(module, self))?;

//         Ok(result)
//     }
// }

impl SymbolQueries for CompilerContext {
    fn intern_symbol(&self, text: &str) -> SymbolId {
        self.interner.write().unwrap().intern(text)
    }

    fn symbol_text(&self, id: SymbolId) -> Arc<str> {
        self.interner.read().unwrap().resolve(id)
    }
}
