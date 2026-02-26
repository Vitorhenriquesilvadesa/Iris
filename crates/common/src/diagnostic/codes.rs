use crate::diagnostic::DiagnosticCode;

// Lexer error codes
pub const INVALID_NUMBER: DiagnosticCode = DiagnosticCode("L001");
pub const UNTERMINATED_STRING: DiagnosticCode = DiagnosticCode("L002");
pub const UNEXPECTED_CHAR: DiagnosticCode = DiagnosticCode("L003");

// Parser error codes
pub const UNEXPECTED_TOKEN: DiagnosticCode = DiagnosticCode("P001");
pub const EXPECTED_EXPRESSION: DiagnosticCode = DiagnosticCode("P002");
pub const INVALID_PARAM: DiagnosticCode = DiagnosticCode("P003");
pub const GENERIC_MESSAGE: DiagnosticCode = DiagnosticCode("P404");

// HIR error cores
pub const TUPLE_EXPRESSION_CODE: DiagnosticCode = DiagnosticCode("H001");
pub const INVALID_ASSIGN_TARGET_CODE: DiagnosticCode = DiagnosticCode("H002");
pub const SYMBOL_NOT_FOUND_CODE: DiagnosticCode = DiagnosticCode("H003");
