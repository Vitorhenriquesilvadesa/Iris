use std::sync::Arc;

use common::{ast::Ast, diagnostic::Diagnostics, source::SourceFileId, token::Token};
use compiler_api::queries::{
    AnalysisResult, QueryResult, lexer::LexerQueries, source::SourceQueries,
};

use crate::parser::{Parser, stream::TokenStream};

pub fn parse<Ctx>(ctx: &Ctx, source_id: SourceFileId) -> QueryResult<Ast>
where
    Ctx: LexerQueries + SourceQueries,
{
    let source_text = ctx
        .source_by_id(source_id)
        .map_err(|e| e.to_string())?
        .value;

    let tokens: Arc<Vec<Token>> = ctx.tokens_of(source_id).map_err(|e| e.to_string())?.value;

    let stream = TokenStream::new(tokens.clone(), source_id);
    let parser = Parser::new(stream, source_text.text(), source_id);

    let output = parser.run();

    Ok(AnalysisResult {
        value: Arc::new(output.ast),
        diagnostics: Arc::new(Diagnostics::new(output.diagnostics)),
    })
}
