#[warn(dead_code)]
use iris_diagnostic::{Diagnostic, codes::*};
use iris_span::{Span, source_file::SourceFileId};

use crate::error::HirError;

pub fn map_hir_error(error: HirError, file: SourceFileId, span: Span) -> Diagnostic {
    let (code, message, label, notes) = match error {
        HirError::TupleExpression => (
            TUPLE_EXPRESSION_CODE,
            "Iris does not support tuples".to_string(),
            "unexpected tuple expression".to_string(),
            vec![
                "comma is only allowed in function argument lists".to_string(),
                "consider using an array instead".to_string(),
            ],
        ),
        HirError::InvalidAssignTarget => (
            INVALID_ASSIGN_TARGET_CODE,
            "invalid assignment target".to_string(),
            "unexpected assign target".to_string(),
            vec![],
        ),
        HirError::SymbolNotFound(symbol) => (
            SYMBOL_NOT_FOUND_CODE,
            format!("symbol `{}` not found in this scope", symbol),
            "unexpected symbol".to_string(),
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
