use std::sync::Arc;

use common::diagnostic::Diagnostics;

pub mod lexer;
pub mod parser;
pub mod source;

/// Result type returned by compiler queries.
///
/// Diagnostics are emitted via the diagnostic sink owned by the context.
pub type QueryResult<T> = Result<Arc<T>, Arc<Diagnostics>>;
