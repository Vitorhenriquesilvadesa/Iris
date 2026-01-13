use common::source::{SourceFile, SourceFileId};

use crate::queries::QueryResult;

/// Queries related to source files.
pub trait SourceQueries {
    /// Returns the source file for the given path.
    fn source_by_path(&self, path: &str) -> QueryResult<SourceFile>;

    fn source_by_id(&self, id: SourceFileId) -> QueryResult<SourceFile>;
}
