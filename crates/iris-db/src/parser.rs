use iris_ast::Ast;
use iris_span::source_file::SourceFileId;

use crate::QueryResult;

/// Queries related to parsing.
pub trait ParserQueries {
    /// Returns the AST for the given source file.
    fn ast_of(&self, file: SourceFileId) -> QueryResult<Ast>;
}
