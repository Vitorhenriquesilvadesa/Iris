#![allow(dead_code)]

use iris_diagnostic::DiagnosticCode;
use iris_syntax::TokenKind;

/// Represents the various errors that can occur during syntactic analysis (parsing).
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// The parser expected a specific token kind but found a different one.
    ///
    /// # Example
    /// Expecting a `)` after function arguments but finding a `;`.
    Expected {
        /// The token kind that was expected.
        expected: TokenKind,
        /// The token kind that was actually found in the input.
        found: TokenKind,
        /// The code that was mapped in the compiler spec.
        code: DiagnosticCode,
    },

    /// The parser expected the start of an expression but found a token
    /// that cannot begin an expression.
    ///
    /// # Example
    /// Encountering a `)` or a statement keyword where a value was expected.
    ExpectedExpression {
        /// The token kind that was found.
        found: TokenKind,
    },

    /// Encountered a token that is invalid in the current context,
    /// even if no specific other token was strictly expected.
    ///
    /// # Example
    /// Writing `let let = 1;` (the second `let` is unexpected).
    UnexpectedToken {
        /// The unexpected token kind found.
        found: TokenKind,
    },

    /// A generic error message for parsing scenarios not covered by specific variants.
    Message(String),
    InvalidParam,
}
