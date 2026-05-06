use std::sync::Arc;

use iris_ast::Ast;
use iris_db::{AnalysisResult, QueryResult, lexer::LexerQueries, source::SourceQueries};
use iris_diagnostic::Diagnostics;
use iris_span::source_file::SourceFileId;

use crate::{Parser, stream::TokenStream};

pub fn parse<Ctx>(ctx: &Ctx, source_id: SourceFileId) -> QueryResult<Ast>
where
    Ctx: LexerQueries + SourceQueries,
{
    let source_file = ctx.source_by_id(source_id);
    let tokens = ctx.tokens_of(source_id)?.value;

    let stream = TokenStream::new(tokens, source_id);
    let parser = Parser::new(stream, source_file.text(), source_id);

    let output = parser.run();

    Ok(AnalysisResult {
        value: Arc::new(output.ast),
        diagnostics: Arc::new(Diagnostics::new(output.diagnostics)),
    })
}
