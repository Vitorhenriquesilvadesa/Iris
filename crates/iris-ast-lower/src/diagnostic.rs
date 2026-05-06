use iris_diagnostic::{Diagnostic, codes::*};
use iris_span::{Span, source_file::SourceFileId};

use crate::error::HirError;

pub fn map_hir_error(error: HirError, file: SourceFileId, span: Span) -> Diagnostic {
    let (code, message, label, notes) = match error {
        HirError::TupleExpression => (
            TUPLE_EXPRESSION_CODE,
            format!("Iris does not support tuples"),
            format!("unexpected tuple expression"),
            vec![
                format!("comma is only allowed in function argument lists"),
                format!("consider using an array instead"),
            ],
        ),
        HirError::InvalidAssignTarget => (
            INVALID_ASSIGN_TARGET_CODE,
            format!("invalid assignment target"),
            format!("unexpected assign target"),
            vec![],
        ),
        HirError::SymbolNotFound(symbol) => (
            SYMBOL_NOT_FOUND_CODE,
            format!("symbol `{}` not found in this scope", symbol),
            format!("unexpected symbol"),
            vec![],
        ),
    };

    let mut diag = Diagnostic::error(message)
        .with_code(code)
        .with_label(span, label)
        .with_file(file)
        .with_primary_span(span);

    for note in notes {
        diag = diag.with_note(note);
    }

    diag
}
