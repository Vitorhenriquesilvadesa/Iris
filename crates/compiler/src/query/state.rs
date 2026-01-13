use std::sync::Arc;

use common::diagnostic::Diagnostics;

#[derive(Debug, Clone)]
pub enum QueryState<T> {
    InProgress,
    Completed(Arc<T>),
    Failed(Arc<Diagnostics>),
}
