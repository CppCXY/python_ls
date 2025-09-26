use crate::{
    kind::{BinaryOperator, PyTokenKind, UnaryOperator},
    syntax::{PySyntaxToken, traits::PyAstToken},
};

use super::{float_token_value, int_token_value, string_token_value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyGeneralToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyGeneralToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(_: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        true
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        Some(PyGeneralToken { token: syntax })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyNameToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyNameToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        kind == PyTokenKind::TkName
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyNameToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyNameToken {
    pub fn get_name_text(&self) -> &str {
        self.token.text()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyStringToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyStringToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PyTokenKind::TkString
                | PyTokenKind::TkRawString
                | PyTokenKind::TkFString
                | PyTokenKind::TkBytesString
                | PyTokenKind::TkRawBytesString
        )
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyStringToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyStringToken {
    pub fn get_value(&self) -> String {
        string_token_value(&self.token).unwrap_or_default()
    }

    pub fn is_raw(&self) -> bool {
        matches!(
            self.token.kind().into(),
            PyTokenKind::TkRawString | PyTokenKind::TkRawBytesString
        )
    }

    pub fn is_bytes(&self) -> bool {
        matches!(
            self.token.kind().into(),
            PyTokenKind::TkBytesString | PyTokenKind::TkRawBytesString
        )
    }

    pub fn is_f_string(&self) -> bool {
        self.token.kind() == PyTokenKind::TkFString.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyNumberToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyNumberToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PyTokenKind::TkInt | PyTokenKind::TkFloat | PyTokenKind::TkComplex
        )
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyNumberToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyNumberToken {
    pub fn is_float(&self) -> bool {
        self.token.kind() == PyTokenKind::TkFloat.into()
    }

    pub fn is_int(&self) -> bool {
        self.token.kind() == PyTokenKind::TkInt.into()
    }

    pub fn is_complex(&self) -> bool {
        self.token.kind() == PyTokenKind::TkComplex.into()
    }

    pub fn get_float_value(&self) -> f64 {
        if self.is_float() {
            float_token_value(&self.token).unwrap_or(0.0)
        } else {
            0.0
        }
    }

    pub fn get_int_value(&self) -> i64 {
        if self.is_int() {
            if let Ok(value) = int_token_value(&self.token) {
                value.as_integer().unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyBinaryOpToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyBinaryOpToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        BinaryOperator::from_token_kind(kind).is_some()
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyBinaryOpToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyBinaryOpToken {
    pub fn get_op(&self) -> BinaryOperator {
        BinaryOperator::from_token_kind(self.token.kind().into()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyUnaryOpToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyUnaryOpToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        UnaryOperator::from_token_kind(kind).is_some()
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyUnaryOpToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyUnaryOpToken {
    pub fn get_op(&self) -> UnaryOperator {
        UnaryOperator::from_token_kind(self.token.kind().into()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyKeywordToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyKeywordToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PyTokenKind::TkAnd
                | PyTokenKind::TkAs
                | PyTokenKind::TkAssert
                | PyTokenKind::TkAsync
                | PyTokenKind::TkAwait
                | PyTokenKind::TkBreak
                | PyTokenKind::TkClass
                | PyTokenKind::TkContinue
                | PyTokenKind::TkDef
                | PyTokenKind::TkDel
                | PyTokenKind::TkElif
                | PyTokenKind::TkElse
                | PyTokenKind::TkExcept
                | PyTokenKind::TkFalse
                | PyTokenKind::TkFinally
                | PyTokenKind::TkFor
                | PyTokenKind::TkFrom
                | PyTokenKind::TkGlobal
                | PyTokenKind::TkIf
                | PyTokenKind::TkImport
                | PyTokenKind::TkIn
                | PyTokenKind::TkIs
                | PyTokenKind::TkLambda
                | PyTokenKind::TkMatch
                | PyTokenKind::TkNone
                | PyTokenKind::TkNonlocal
                | PyTokenKind::TkNot
                | PyTokenKind::TkOr
                | PyTokenKind::TkPass
                | PyTokenKind::TkRaise
                | PyTokenKind::TkReturn
                | PyTokenKind::TkTrue
                | PyTokenKind::TkTry
                | PyTokenKind::TkWhile
                | PyTokenKind::TkWith
                | PyTokenKind::TkYield
        )
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyKeywordToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyKeywordToken {
    pub fn get_keyword(&self) -> PyTokenKind {
        self.token.kind().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyBoolToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyBoolToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, PyTokenKind::TkTrue | PyTokenKind::TkFalse)
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyBoolToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyBoolToken {
    pub fn is_true(&self) -> bool {
        self.token.kind() == PyTokenKind::TkTrue.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyNoneToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyNoneToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        kind == PyTokenKind::TkNone
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyNoneToken { token: syntax })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyLiteralToken {
    String(PyStringToken),
    Number(PyNumberToken),
    Bool(PyBoolToken),
    None(PyNoneToken),
    Ellipsis(PyGeneralToken),
}

impl PyAstToken for PyLiteralToken {
    fn syntax(&self) -> &PySyntaxToken {
        match self {
            PyLiteralToken::String(token) => token.syntax(),
            PyLiteralToken::Number(token) => token.syntax(),
            PyLiteralToken::Bool(token) => token.syntax(),
            PyLiteralToken::None(token) => token.syntax(),
            PyLiteralToken::Ellipsis(token) => token.syntax(),
        }
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        PyStringToken::can_cast(kind)
            || PyNumberToken::can_cast(kind)
            || PyBoolToken::can_cast(kind)
            || PyNoneToken::can_cast(kind)
            || kind == PyTokenKind::TkEllipsis
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        let kind: PyTokenKind = syntax.kind().into();
        if let Some(string) = PyStringToken::cast(syntax.clone()) {
            Some(PyLiteralToken::String(string))
        } else if let Some(number) = PyNumberToken::cast(syntax.clone()) {
            Some(PyLiteralToken::Number(number))
        } else if let Some(bool_token) = PyBoolToken::cast(syntax.clone()) {
            Some(PyLiteralToken::Bool(bool_token))
        } else if let Some(none) = PyNoneToken::cast(syntax.clone()) {
            Some(PyLiteralToken::None(none))
        } else if kind == PyTokenKind::TkEllipsis {
            Some(PyLiteralToken::Ellipsis(PyGeneralToken::cast(syntax)?))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySpaceToken {
    token: PySyntaxToken,
}

impl PyAstToken for PySpaceToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PyTokenKind::TkWhitespace
                | PyTokenKind::TkNewline
                | PyTokenKind::TkIndent
                | PyTokenKind::TkDedent
        )
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PySpaceToken { token: syntax })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCommentToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyCommentToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        kind == PyTokenKind::TkComment
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyCommentToken { token: syntax })
        } else {
            None
        }
    }
}

impl PyCommentToken {
    pub fn get_comment_text(&self) -> &str {
        let text = self.token.text();
        if text.starts_with('#') {
            &text[1..]
        } else {
            text
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyDecoratorToken {
    token: PySyntaxToken,
}

impl PyAstToken for PyDecoratorToken {
    fn syntax(&self) -> &PySyntaxToken {
        &self.token
    }

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized,
    {
        kind == PyTokenKind::TkAt
    }

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(PyDecoratorToken { token: syntax })
        } else {
            None
        }
    }
}
