use iris_span::source_file::SourceFileId;
use iris_syntax::Token;

use crate::QueryResult;

/// Queries related to lexical analysis.
pub trait LexerQueries {
    /// Returns the tokens for the given source file.
    fn tokens_of(&self, file: SourceFileId) -> QueryResult<Vec<Token>>;
}
