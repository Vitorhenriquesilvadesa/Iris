use common::{ast::Ast, source::SourceFileId};

use crate::queries::QueryResult;

/// Queries related to parsing.
pub trait ParserQueries {
    /// Returns the AST for the given source file.
    fn ast_of(&self, file: SourceFileId) -> QueryResult<Ast>;
}
