use common::{ast::Ast, diagnostic::Diagnostic};

mod diagnostic;
mod error;
pub mod parse;
mod parser;
mod stream;

pub use parse::*;

struct ParserOutput {
    pub ast: Ast,
    pub diagnostics: Vec<Diagnostic>,
}
