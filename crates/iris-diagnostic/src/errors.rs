use std::{io, sync::Arc};

use iris_span::source_file::SourceFileId;

#[derive(Debug, Clone)]
pub enum FatalError {
    Io(Arc<io::Error>),
    FileNotFound(SourceFileId),
}
