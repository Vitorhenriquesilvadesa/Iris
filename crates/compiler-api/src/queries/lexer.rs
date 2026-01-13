use common::{source::SourceFileId, token::Token};

use crate::queries::QueryResult;

/// Queries related to lexical analysis.
pub trait LexerQueries {
    /// Returns the tokens for the given source file.
    fn tokens_of(&self, file: SourceFileId) -> QueryResult<Vec<Token>>;
}
