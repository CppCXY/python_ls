use rowan::TextRange;

use crate::text::SourceRange;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyParseErrorKind {
    SyntaxError,
    DocError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LuaParseError {
    pub kind: PyParseErrorKind,
    pub message: String,
    pub range: TextRange,
}

impl LuaParseError {
    pub fn new(kind: PyParseErrorKind, message: &str, range: TextRange) -> Self {
        LuaParseError {
            kind,
            message: message.to_string(),
            range,
        }
    }

    pub fn syntax_error_from(message: &str, range: SourceRange) -> Self {
        LuaParseError {
            kind: PyParseErrorKind::SyntaxError,
            message: message.to_string(),
            range: range.into(),
        }
    }

    pub fn doc_error_from(message: &str, range: SourceRange) -> Self {
        LuaParseError {
            kind: PyParseErrorKind::DocError,
            message: message.to_string(),
            range: range.into(),
        }
    }
}
