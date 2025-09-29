mod py_language_level;
mod py_operator_kind;
mod py_syntax_kind;
mod py_token_kind;
mod py_version;
mod py_visibility_kind;
mod version_warning_tests;

pub use py_language_level::PyLanguageLevel;
pub use py_operator_kind::{BinaryOperator, UNARY_PRIORITY, UnaryOperator};
pub use py_syntax_kind::PySyntaxKind;
pub use py_token_kind::PyTokenKind;
pub use py_version::{PyVersionCondition, PyVersionNumber};
pub use py_visibility_kind::VisibilityKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum PyKind {
    Syntax(PySyntaxKind),
    Token(PyTokenKind),
}

impl From<PySyntaxKind> for PyKind {
    fn from(kind: PySyntaxKind) -> Self {
        PyKind::Syntax(kind)
    }
}

impl From<PyTokenKind> for PyKind {
    fn from(kind: PyTokenKind) -> Self {
        PyKind::Token(kind)
    }
}

impl From<PyKind> for PySyntaxKind {
    fn from(val: PyKind) -> Self {
        match val {
            PyKind::Syntax(kind) => kind,
            _ => PySyntaxKind::None,
        }
    }
}

impl From<PyKind> for PyTokenKind {
    fn from(val: PyKind) -> Self {
        match val {
            PyKind::Token(kind) => kind,
            _ => PyTokenKind::None,
        }
    }
}

impl PyKind {
    pub fn is_syntax(self) -> bool {
        matches!(self, PyKind::Syntax(_))
    }

    pub fn is_token(self) -> bool {
        matches!(self, PyKind::Token(_))
    }

    pub fn to_syntax(self) -> PySyntaxKind {
        match self {
            PyKind::Syntax(kind) => kind,
            PyKind::Token(_) => PySyntaxKind::None,
        }
    }

    pub fn to_token(self) -> PyTokenKind {
        match self {
            PyKind::Token(kind) => kind,
            PyKind::Syntax(_) => PyTokenKind::None,
        }
    }

    pub fn get_raw(self) -> u16 {
        match self {
            PyKind::Syntax(kind) => kind as u16 | 0x8000,
            PyKind::Token(kind) => kind as u16,
        }
    }

    pub fn from_raw(raw: u16) -> PyKind {
        if raw & 0x8000 != 0 {
            PyKind::Syntax(unsafe { std::mem::transmute::<u16, PySyntaxKind>(raw & 0x7FFF) })
        } else {
            PyKind::Token(unsafe { std::mem::transmute::<u16, PyTokenKind>(raw) })
        }
    }
}

#[derive(Debug)]
pub struct PriorityTable {
    pub left: i32,
    pub right: i32,
}

#[derive(Debug, PartialEq)]
pub enum PyOpKind {
    None,
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

impl From<UnaryOperator> for PyOpKind {
    fn from(op: UnaryOperator) -> Self {
        PyOpKind::Unary(op)
    }
}

impl From<BinaryOperator> for PyOpKind {
    fn from(op: BinaryOperator) -> Self {
        PyOpKind::Binary(op)
    }
}

impl PyOpKind {
    pub fn to_unary_operator(kind: PyTokenKind) -> UnaryOperator {
        match kind {
            PyTokenKind::TkNot => UnaryOperator::OpNot,
            PyTokenKind::TkPlus => UnaryOperator::OpUPlus,
            PyTokenKind::TkMinus => UnaryOperator::OpUMinus,
            PyTokenKind::TkBitNot => UnaryOperator::OpInvert,
            _ => UnaryOperator::OpNop,
        }
    }

    pub fn to_binary_operator(kind: PyTokenKind) -> BinaryOperator {
        match kind {
            PyTokenKind::TkPlus => BinaryOperator::OpAdd,
            PyTokenKind::TkMinus => BinaryOperator::OpSub,
            PyTokenKind::TkMul => BinaryOperator::OpMul,
            PyTokenKind::TkDiv => BinaryOperator::OpDiv,
            PyTokenKind::TkFloorDiv => BinaryOperator::OpFloorDiv,
            PyTokenKind::TkMod => BinaryOperator::OpMod,
            PyTokenKind::TkPow => BinaryOperator::OpPow,
            PyTokenKind::TkMatMul => BinaryOperator::OpMatMul,
            PyTokenKind::TkBitAnd => BinaryOperator::OpBitAnd,
            PyTokenKind::TkBitOr => BinaryOperator::OpBitOr,
            PyTokenKind::TkBitXor => BinaryOperator::OpBitXor,
            PyTokenKind::TkShl => BinaryOperator::OpLShift,
            PyTokenKind::TkShr => BinaryOperator::OpRShift,
            PyTokenKind::TkEq => BinaryOperator::OpEq,
            PyTokenKind::TkNe => BinaryOperator::OpNotEq,
            PyTokenKind::TkLt => BinaryOperator::OpLt,
            PyTokenKind::TkLe => BinaryOperator::OpLtE,
            PyTokenKind::TkGt => BinaryOperator::OpGt,
            PyTokenKind::TkGe => BinaryOperator::OpGtE,
            PyTokenKind::TkIs => BinaryOperator::OpIs,
            PyTokenKind::TkIn => BinaryOperator::OpIn,
            PyTokenKind::TkAnd => BinaryOperator::OpAnd,
            PyTokenKind::TkOr => BinaryOperator::OpOr,
            PyTokenKind::TkColonAssign => BinaryOperator::OpAssignExpr,
            _ => BinaryOperator::OpNop,
        }
    }
}
