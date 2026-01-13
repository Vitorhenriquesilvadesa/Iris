#![allow(dead_code)]

use std::{path::PathBuf, sync::Arc};

use common::{
    ast::Ast,
    module::{Module, ModuleId},
    source::{SourceFile, SourceFileId},
    token::Token,
};
use compiler_api::queries::{
    QueryResult, lexer::LexerQueries, parser::ParserQueries, source::SourceQueries,
};

use frontend;

use crate::{
    context::CompilationOutput,
    map::source_map::SourceMap,
    query::{cache::QueryCache, registry::QueryKey},
};

#[derive(Debug)]
pub struct CompilerContext {
    source_map: SourceMap,
    tokens_cache: QueryCache<QueryKey, Vec<Token>>,
    ast_cache: QueryCache<QueryKey, Ast>,
    module_cache: QueryCache<QueryKey, Module>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            source_map: SourceMap::new(),
            tokens_cache: QueryCache::new(),
            ast_cache: QueryCache::new(),
            module_cache: QueryCache::new(),
        }
    }

    pub fn attach_root_file<P: Into<PathBuf>>(&self, path: P) -> QueryResult<()> {
        match self.source_map.load_file(path) {
            Ok(_) => Ok(Arc::new(())),
            Err(e) => Err(e.clone()),
        }
    }

    pub fn compile(&self) -> QueryResult<CompilationOutput> {
        let file_result = self.source_by_id(SourceFileId::new(1));

        match file_result {
            Ok(file) => self.ast_of(file.id()),
            Err(e) => Err(e),
        }
    }

    pub fn source_map(&self) -> &SourceMap {
        &self.source_map
    }
}

impl SourceQueries for CompilerContext {
    fn source_by_path(&self, path: &str) -> QueryResult<SourceFile> {
        self.source_map.load_file(path)
    }

    fn source_by_id(&self, id: SourceFileId) -> QueryResult<SourceFile> {
        self.source_map.load_by_id(id)
    }
}

impl LexerQueries for CompilerContext {
    fn tokens_of(&self, file: SourceFileId) -> QueryResult<Vec<Token>> {
        let query = QueryKey::tokens(file);

        self.tokens_cache
            .get_or_compute(query, || frontend::lexer::lex(self, file))
    }
}

impl ParserQueries for CompilerContext {
    fn ast_of(&self, file: SourceFileId) -> QueryResult<Ast> {
        let query = QueryKey::ast(ModuleId::new(file.as_u32()));

        self.ast_cache
            .get_or_compute(query, || frontend::parser::parse(self, file))
    }
}
