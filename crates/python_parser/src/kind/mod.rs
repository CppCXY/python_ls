mod py_language_level;
mod py_operator_kind;
mod py_syntax_kind;
mod py_token_kind;
mod py_version;
mod py_visibility_kind;

pub use py_language_level::PyLanguageLevel;
pub use py_operator_kind::{BinaryOperator, UNARY_PRIORITY, UnaryOperator};
pub use py_syntax_kind::PySyntaxKind;
pub use py_token_kind::PyTokenKind;
pub use py_version::{LuaVersionCondition, PyVersionNumber};
pub use py_visibility_kind::VisibilityKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum LuaKind {
    Syntax(PySyntaxKind),
    Token(PyTokenKind),
}

impl From<PySyntaxKind> for LuaKind {
    fn from(kind: PySyntaxKind) -> Self {
        LuaKind::Syntax(kind)
    }
}

impl From<PyTokenKind> for LuaKind {
    fn from(kind: PyTokenKind) -> Self {
        LuaKind::Token(kind)
    }
}

impl From<LuaKind> for PySyntaxKind {
    fn from(val: LuaKind) -> Self {
        match val {
            LuaKind::Syntax(kind) => kind,
            _ => PySyntaxKind::None,
        }
    }
}

impl From<LuaKind> for PyTokenKind {
    fn from(val: LuaKind) -> Self {
        match val {
            LuaKind::Token(kind) => kind,
            _ => PyTokenKind::None,
        }
    }
}

impl LuaKind {
    pub fn is_syntax(self) -> bool {
        matches!(self, LuaKind::Syntax(_))
    }

    pub fn is_token(self) -> bool {
        matches!(self, LuaKind::Token(_))
    }

    pub fn to_syntax(self) -> PySyntaxKind {
        match self {
            LuaKind::Syntax(kind) => kind,
            LuaKind::Token(_) => PySyntaxKind::None,
        }
    }

    pub fn to_token(self) -> PyTokenKind {
        match self {
            LuaKind::Token(kind) => kind,
            LuaKind::Syntax(_) => PyTokenKind::None,
        }
    }

    pub fn get_raw(self) -> u16 {
        match self {
            LuaKind::Syntax(kind) => kind as u16 | 0x8000,
            LuaKind::Token(kind) => kind as u16,
        }
    }

    pub fn from_raw(raw: u16) -> LuaKind {
        if raw & 0x8000 != 0 {
            LuaKind::Syntax(unsafe { std::mem::transmute::<u16, PySyntaxKind>(raw & 0x7FFF) })
        } else {
            LuaKind::Token(unsafe { std::mem::transmute::<u16, PyTokenKind>(raw) })
        }
    }
}

#[derive(Debug)]
pub struct PriorityTable {
    pub left: i32,
    pub right: i32,
}

#[derive(Debug, PartialEq)]
pub enum LuaOpKind {
    None,
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

impl From<UnaryOperator> for LuaOpKind {
    fn from(op: UnaryOperator) -> Self {
        LuaOpKind::Unary(op)
    }
}

impl From<BinaryOperator> for LuaOpKind {
    fn from(op: BinaryOperator) -> Self {
        LuaOpKind::Binary(op)
    }
}

impl LuaOpKind {
    pub fn to_unary_operator(kind: PyTokenKind) -> UnaryOperator {
        match kind {
            PyTokenKind::TkNot => UnaryOperator::OpNot,
            PyTokenKind::TkLen => UnaryOperator::OpLen,
            PyTokenKind::TkMinus => UnaryOperator::OpUnm,
            PyTokenKind::TkBitXor => UnaryOperator::OpBNot,
            _ => UnaryOperator::OpNop,
        }
    }

    pub fn to_binary_operator(kind: PyTokenKind) -> BinaryOperator {
        match kind {
            PyTokenKind::TkPlus => BinaryOperator::OpAdd,
            PyTokenKind::TkMinus => BinaryOperator::OpSub,
            PyTokenKind::TkMul => BinaryOperator::OpMul,
            PyTokenKind::TkMod => BinaryOperator::OpMod,
            PyTokenKind::TkPow => BinaryOperator::OpPow,
            PyTokenKind::TkDiv => BinaryOperator::OpDiv,
            PyTokenKind::TkIDiv => BinaryOperator::OpIDiv,
            PyTokenKind::TkBitAnd => BinaryOperator::OpBAnd,
            PyTokenKind::TkBitOr => BinaryOperator::OpBOr,
            PyTokenKind::TkBitXor => BinaryOperator::OpBXor,
            PyTokenKind::TkShl => BinaryOperator::OpShl,
            PyTokenKind::TkShr => BinaryOperator::OpShr,
            PyTokenKind::TkConcat => BinaryOperator::OpConcat,
            PyTokenKind::TkLt => BinaryOperator::OpLt,
            PyTokenKind::TkLe => BinaryOperator::OpLe,
            PyTokenKind::TkGt => BinaryOperator::OpGt,
            PyTokenKind::TkGe => BinaryOperator::OpGe,
            PyTokenKind::TkEq => BinaryOperator::OpEq,
            PyTokenKind::TkNe => BinaryOperator::OpNe,
            PyTokenKind::TkAnd => BinaryOperator::OpAnd,
            PyTokenKind::TkOr => BinaryOperator::OpOr,
            _ => BinaryOperator::OpNop,
        }
    }
}
