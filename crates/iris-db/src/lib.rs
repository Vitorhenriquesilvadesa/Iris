use std::sync::Arc;

use iris_diagnostic::{Diagnostics, errors::FatalError};

pub mod hir;
pub mod lexer;
pub mod parser;
pub mod scope;
pub mod source;
pub mod symbol;

#[derive(Clone, Debug)]
pub struct AnalysisResult<T: Clone> {
    pub value: Arc<T>,
    pub diagnostics: Arc<Diagnostics>,
}

/// Result type returned by compiler queries.
///
/// Diagnostics are emitted via the diagnostic sink owned by the context.
pub type QueryResult<T> = Result<AnalysisResult<T>, FatalError>;
