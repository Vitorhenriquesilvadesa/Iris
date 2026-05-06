#![allow(dead_code)]

use std::{
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

use iris_ast::Ast;
use iris_db::{
    AnalysisResult, QueryResult, hir::HirQueries, lexer::LexerQueries, parser::ParserQueries,
    source::SourceQueries, symbol::SymbolQueries,
};
use iris_diagnostic::{Diagnostics, errors::FatalError};
use iris_hir::{
    Hir,
    globals::GlobalScope,
    module::{HirModule, ModuleId},
};
use iris_interner::{SymbolId, SymbolInterner};
use iris_span::{
    source_file::{SourceFile, SourceFileId},
    source_map::SourceMap,
};
use iris_syntax::Token;

use crate::{
    context::CompilationOutput,
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
                value: Arc::new(file.id()),
                diagnostics: Arc::new(Diagnostics::new(vec![])),
            }),
            Err(e) => self.fatal(FatalError::Io(Arc::new(e))),
        }
    }

    pub fn compile(&self, id: SourceFileId) -> QueryResult<CompilationOutput> {
        let file_res = self.source_by_id(id);

        println!("Compiling '{}'", file_res.text());

        let source_file = file_res;

        self.hir_of(source_file.id())
    }

    pub fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    pub fn fatal(&self, error: FatalError) -> ! {
        panic!("{:?}", error)
    }
}

impl SourceQueries for CompilerContext {
    fn source_by_path(&self, path: &str) -> Arc<SourceFile> {
        self.source_map
            .load_file(path)
            .unwrap_or_else(|e| self.fatal(FatalError::Io(Arc::new(e))))
    }

    fn source_by_id(&self, id: SourceFileId) -> Arc<SourceFile> {
        self.source_map
            .load_by_id(id)
            .unwrap_or_else(|| self.fatal(FatalError::FileNotFound(id)))
    }
}

impl LexerQueries for CompilerContext {
    fn tokens_of(&self, file: SourceFileId) -> QueryResult<Vec<Token>> {
        let query = QueryKey::tokens(file);

        let result = self
            .tokens_cache
            .get_or_compute(query, self, || iris_lexer::lex(self, file))?;

        Ok(result)
    }
}

impl ParserQueries for CompilerContext {
    fn ast_of(&self, file: SourceFileId) -> QueryResult<Ast> {
        let query = QueryKey::ast(ModuleId::new(file.as_u32()));

        let result = self
            .ast_cache
            .get_or_compute(query, self, || iris_parser::parse(self, file))?;

        Ok(result)
    }
}

impl HirQueries for CompilerContext {
    fn hir_of(&self, file_id: SourceFileId) -> QueryResult<Hir> {
        let query = QueryKey::hir(ModuleId::from_file(&file_id));

        let result = self
            .hir_cache
            .get_or_compute(query, self, || iris_ast_lower::gen_hir_of(file_id, self))?;

        Ok(result)
    }
}

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
