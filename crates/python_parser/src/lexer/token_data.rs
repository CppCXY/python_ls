use crate::{kind::PyTokenKind, text::SourceRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyTokenData {
    pub kind: PyTokenKind,
    pub range: SourceRange,
}

impl PyTokenData {
    pub fn new(kind: PyTokenKind, range: SourceRange) -> Self {
        PyTokenData { kind, range }
    }
}
