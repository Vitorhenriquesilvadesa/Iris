use std::sync::Arc;

use common::{diagnostic::Diagnostics, source::SourceFileId, token::Token};
use compiler_api::queries::{AnalysisResult, QueryResult, source::SourceQueries};

use crate::lexer::Lexer;

pub fn lex(ctx: &impl SourceQueries, source_id: SourceFileId) -> QueryResult<Vec<Token>> {
    let source_text = ctx
        .source_by_id(source_id)
        .map_err(|e| format!("{:?}", e))?
        .value;

    let lexer = Lexer::new(source_id, source_text.text());
    let output = lexer.run();

    Ok(AnalysisResult {
        value: Arc::new(output.tokens),
        diagnostics: Arc::new(Diagnostics::new(output.diagnostics)),
    })
}
