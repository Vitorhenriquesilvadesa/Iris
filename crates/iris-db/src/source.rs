use std::sync::Arc;

use iris_span::source_file::{SourceFile, SourceFileId};

/// Queries related to source files.
pub trait SourceQueries {
    /// Returns the source file for the given path.
    fn source_by_path(&self, path: &str) -> Arc<SourceFile>;

    fn source_by_id(&self, id: SourceFileId) -> Arc<SourceFile>;
}
