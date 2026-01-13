use std::sync::Arc;

use common::{diagnostic::Diagnostics, source::SourceFileId, token::Token};
use compiler_api::queries::{QueryResult, source::SourceQueries};

use crate::lexer::lexer::Lexer;

pub fn lex(ctx: &impl SourceQueries, source_id: SourceFileId) -> QueryResult<Vec<Token>> {
    let source_text = ctx.source_by_id(source_id)?;
    let lexer = Lexer::new(source_id, source_text.text());

    let output = lexer.run();

    if output.diagnostics.is_empty() {
        Ok(Arc::new(output.tokens))
    } else {
        Err(Arc::new(Diagnostics::new(output.diagnostics)))
    }
}
