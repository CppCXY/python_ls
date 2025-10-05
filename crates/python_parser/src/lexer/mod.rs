mod fstring_lexer;
mod lexer_config;
mod py_lexer;
mod test;
mod token_data;

pub use fstring_lexer::{FStringLexer, FStringToken};
pub use lexer_config::LexerConfig;
pub use py_lexer::PyLexer;
pub use token_data::PyTokenData;

pub(crate) fn is_name_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

pub(crate) fn is_name_continue(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// This enum allows preserving lexer state between reader resets. This is used
/// when lexer doesn't see the whole input source, and only sees a reader
/// for each individual line. It happens when we're lexing
/// code blocks in comments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerState {
    Normal,
    String(char),
    LongString(usize),
    LongComment(usize),
}
