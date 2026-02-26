use compiler_api::queries::AnalysisResult;

#[derive(Debug, Clone)]
pub enum QueryState<T: Clone> {
    InProgress,
    Completed(AnalysisResult<T>),
    Failed(String),
}
