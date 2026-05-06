use iris_diagnostic::{Diagnostic, codes::*};
use iris_span::{Span, source_file::SourceFileId};

use crate::error::ParseError;

pub fn map_parse_error(error: ParseError, file_id: SourceFileId, span: Span) -> Diagnostic {
    let (code, message, label) = match error {
        ParseError::Expected {
            expected,
            found,
            code,
        } => (
            code,
            format!("expected `{}`, found `{}`", expected, found),
            "expected this token".to_string(),
        ),
        ParseError::ExpectedExpression { found } => (
            EXPECTED_EXPRESSION,
            format!("expected expression, found `{}`", found),
            "expected start of expression here".to_string(),
        ),
        ParseError::UnexpectedToken { found } => (
            UNEXPECTED_TOKEN,
            format!("unexpected token `{}`", found),
            "unexpected".to_string(),
        ),
        ParseError::Message(msg) => (GENERIC_MESSAGE, msg, "syntax error".to_string()),
        ParseError::InvalidParam => (
            INVALID_PARAM,
            "invalid param".to_string(),
            "unexpected function param".to_string(),
        ),
    };

    Diagnostic::error(message)
        .with_label(span, label)
        .with_code(code)
        .with_primary_span(span)
        .with_file(file_id)
}
