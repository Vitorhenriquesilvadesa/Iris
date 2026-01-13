use std::sync::Arc;

use common::{ast::Ast, diagnostic::Diagnostics, source::SourceFileId, token::Token};
use compiler_api::queries::{QueryResult, lexer::LexerQueries, source::SourceQueries};

use crate::parser::{parser::Parser, stream::TokenStream};

pub fn parse<Ctx>(ctx: &Ctx, source_id: SourceFileId) -> QueryResult<Ast>
where
    Ctx: LexerQueries + SourceQueries,
{
    let source_text = ctx.source_by_id(source_id)?;
    let tokens: Arc<Vec<Token>> = ctx.tokens_of(source_id)?;

    let stream = TokenStream::new(tokens.clone());
    let parser = Parser::new(stream, source_text.text(), source_id);

    let output = parser.run();

    if output.diagnostics.is_empty() {
        Ok(Arc::new(output.ast))
    } else {
        Err(Arc::new(Diagnostics::new(output.diagnostics)))
    }
}
