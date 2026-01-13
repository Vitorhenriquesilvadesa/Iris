use crate::diagnostic::DiagnosticCode;

// Lexer error codes
pub const INVALID_NUMBER: DiagnosticCode = DiagnosticCode("L001");
pub const UNTERMINATED_STRING: DiagnosticCode = DiagnosticCode("L002");
pub const UNEXPECTED_CHAR: DiagnosticCode = DiagnosticCode("L003");

// Parser error codes
pub const UNEXPECTED_TOKEN: DiagnosticCode = DiagnosticCode("P001");
pub const EXPECTED_EXPRESSION: DiagnosticCode = DiagnosticCode("P002");
pub const GENERIC_MESSAGE: DiagnosticCode = DiagnosticCode("P404");
