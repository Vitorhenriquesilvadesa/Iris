pub mod ctx;
use common::{ast::Ast, token::Token};
pub use ctx::*;

pub type CompilationOutput = Ast;
