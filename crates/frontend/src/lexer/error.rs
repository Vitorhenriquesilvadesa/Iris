#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub enum LexError {
    InvalidNumber,
    UnterminatedString,
    UnexpectedChar(char),
}
