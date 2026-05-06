use iris_db::AnalysisResult;
use iris_diagnostic::errors::FatalError;

#[derive(Debug)]
pub enum QueryState<T: Clone> {
    InProgress,
    Completed(AnalysisResult<T>),
    Failed(FatalError),
}
