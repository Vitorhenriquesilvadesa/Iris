use iris_hir::Hir;
use iris_span::source_file::SourceFileId;

use crate::QueryResult;

pub trait HirQueries {
    fn hir_of(&self, file_id: SourceFileId) -> QueryResult<Hir>;
}
