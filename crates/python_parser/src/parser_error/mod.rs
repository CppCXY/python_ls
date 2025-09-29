use rowan::TextRange;

use crate::text::SourceRange;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyParseErrorKind {
    SyntaxError,
    DocError,
    VersionWarning, // Warning for features that require newer Python versions
}

#[derive(Debug, Clone, PartialEq)]
pub struct PyParseError {
    pub kind: PyParseErrorKind,
    pub message: String,
    pub range: TextRange,
}

impl PyParseError {
    pub fn new(kind: PyParseErrorKind, message: &str, range: TextRange) -> Self {
        PyParseError {
            kind,
            message: message.to_string(),
            range,
        }
    }

    pub fn syntax_error_from(message: &str, range: SourceRange) -> Self {
        PyParseError {
            kind: PyParseErrorKind::SyntaxError,
            message: message.to_string(),
            range: range.into(),
        }
    }

    pub fn doc_error_from(message: &str, range: SourceRange) -> Self {
        PyParseError {
            kind: PyParseErrorKind::DocError,
            message: message.to_string(),
            range: range.into(),
        }
    }

    pub fn version_warning_from(message: &str, range: SourceRange) -> Self {
        PyParseError {
            kind: PyParseErrorKind::VersionWarning,
            message: message.to_string(),
            range: range.into(),
        }
    }
}
