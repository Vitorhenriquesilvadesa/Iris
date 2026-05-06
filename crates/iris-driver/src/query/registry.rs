#![allow(dead_code)]
use std::path::PathBuf;

use iris_hir::module::ModuleId;
use iris_span::source_file::SourceFileId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QueryKind {
    SourceText,
    Tokens,
    Ast,
    Hir,
    SemanticModel,
    Globals,
    Module,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryInput {
    SourceByPath(PathBuf),
    SourceById(SourceFileId),
    Module(ModuleId),
    // Symbol(SymbolId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryKey {
    pub kind: QueryKind,
    pub input: QueryInput,
}

impl QueryKey {
    pub fn source_by_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            kind: QueryKind::SourceText,
            input: QueryInput::SourceByPath(path.into()),
        }
    }

    pub fn source_by_id(file: SourceFileId) -> Self {
        Self {
            kind: QueryKind::SourceText,
            input: QueryInput::SourceById(file),
        }
    }

    pub fn tokens(file: SourceFileId) -> Self {
        Self {
            kind: QueryKind::Tokens,
            input: QueryInput::SourceById(file),
        }
    }

    pub fn ast(module: ModuleId) -> Self {
        Self {
            kind: QueryKind::Ast,
            input: QueryInput::Module(module),
        }
    }

    pub fn hir(module: ModuleId) -> Self {
        Self {
            kind: QueryKind::Hir,
            input: QueryInput::Module(module),
        }
    }

    pub fn module(module: ModuleId) -> Self {
        Self {
            kind: QueryKind::Module,
            input: QueryInput::Module(module),
        }
    }

    pub fn globals(module: ModuleId) -> Self {
        Self {
            kind: QueryKind::Globals,
            input: QueryInput::Module(module),
        }
    }
}
