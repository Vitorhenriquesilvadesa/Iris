use common::{
    diagnostic::{Diagnostic, codes},
    source::SourceFileId,
    span::Span,
};

use crate::lexer::error::LexError;

pub fn map_lex_error(error: LexError, file: SourceFileId, span: Span) -> Diagnostic {
    match error {
        LexError::InvalidNumber => Diagnostic::error("invalid number literal")
            .with_code(codes::INVALID_NUMBER)
            .with_file(file)
            .with_primary_span(span),

        LexError::UnterminatedString => Diagnostic::error("unterminated string literal")
            .with_code(codes::UNTERMINATED_STRING)
            .with_file(file)
            .with_primary_span(span),

        LexError::UnexpectedChar(c) => Diagnostic::error(format!("unexpected character `{c}`"))
            .with_code(codes::UNEXPECTED_CHAR)
            .with_file(file)
            .with_primary_span(span),
    }
}
