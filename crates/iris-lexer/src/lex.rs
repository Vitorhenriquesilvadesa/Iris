use std::sync::Arc;

use iris_db::{AnalysisResult, QueryResult, source::SourceQueries};
use iris_diagnostic::Diagnostics;
use iris_span::source_file::SourceFileId;
use iris_syntax::Token;

use crate::Lexer;

pub fn lex(ctx: &impl SourceQueries, source_id: SourceFileId) -> QueryResult<Vec<Token>> {
    let source_file = ctx.source_by_id(source_id);

    let lexer = Lexer::new(source_id, source_file.text());
    let output = lexer.run();

    Ok(AnalysisResult {
        value: Arc::new(output.tokens),
        diagnostics: Arc::new(Diagnostics::new(output.diagnostics)),
    })
}
