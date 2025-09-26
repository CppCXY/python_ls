use crate::PyTokenKind;

use super::PriorityTable;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnaryOperator {
    OpNot,    // not
    OpUPlus,  // +
    OpUMinus, // -
    OpInvert, // ~ (bitwise not)
    OpNop,    // (empty)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOperator {
    // Arithmetic operators
    OpAdd,      // +
    OpSub,      // -
    OpMul,      // *
    OpDiv,      // /
    OpFloorDiv, // //
    OpMod,      // %
    OpPow,      // **
    OpMatMul,   // @ (matrix multiplication)

    // Bitwise operators
    OpBitAnd, // &
    OpBitOr,  // |
    OpBitXor, // ^
    OpLShift, // <<
    OpRShift, // >>

    // Comparison operators
    OpEq,    // ==
    OpNotEq, // !=
    OpLt,    // <
    OpLtE,   // <=
    OpGt,    // >
    OpGtE,   // >=
    OpIs,    // is
    OpIsNot, // is not
    OpIn,    // in
    OpNotIn, // not in

    // Logical operators
    OpAnd, // and
    OpOr,  // or

    OpNop, // (empty)
}

// Python operator precedence (higher number = higher precedence)
pub const PRIORITY: [PriorityTable; 25] = [
    // Arithmetic operators
    PriorityTable {
        left: 10,
        right: 10,
    }, // OpAdd
    PriorityTable {
        left: 10,
        right: 10,
    }, // OpSub
    PriorityTable {
        left: 11,
        right: 11,
    }, // OpMul
    PriorityTable {
        left: 11,
        right: 11,
    }, // OpDiv
    PriorityTable {
        left: 11,
        right: 11,
    }, // OpFloorDiv
    PriorityTable {
        left: 11,
        right: 11,
    }, // OpMod
    PriorityTable {
        left: 14,
        right: 13,
    }, // OpPow (right associative)
    PriorityTable {
        left: 11,
        right: 11,
    }, // OpMatMul
    // Bitwise operators
    PriorityTable { left: 8, right: 8 }, // OpBitAnd
    PriorityTable { left: 6, right: 6 }, // OpBitOr
    PriorityTable { left: 7, right: 7 }, // OpBitXor
    PriorityTable { left: 9, right: 9 }, // OpLShift
    PriorityTable { left: 9, right: 9 }, // OpRShift
    // Comparison operators (all same precedence in Python)
    PriorityTable { left: 5, right: 5 }, // OpEq
    PriorityTable { left: 5, right: 5 }, // OpNotEq
    PriorityTable { left: 5, right: 5 }, // OpLt
    PriorityTable { left: 5, right: 5 }, // OpLtE
    PriorityTable { left: 5, right: 5 }, // OpGt
    PriorityTable { left: 5, right: 5 }, // OpGtE
    PriorityTable { left: 5, right: 5 }, // OpIs
    PriorityTable { left: 5, right: 5 }, // OpIsNot
    PriorityTable { left: 5, right: 5 }, // OpIn
    PriorityTable { left: 5, right: 5 }, // OpNotIn
    // Logical operators
    PriorityTable { left: 4, right: 4 }, // OpAnd
    PriorityTable { left: 3, right: 3 }, // OpOr
];

impl BinaryOperator {
    pub fn get_priority(&self) -> &PriorityTable {
        &PRIORITY[*self as usize]
    }

    pub fn from_token_kind(kind: super::PyTokenKind) -> Option<Self> {
        match kind {
            PyTokenKind::TkPlus => Some(BinaryOperator::OpAdd),
            PyTokenKind::TkMinus => Some(BinaryOperator::OpSub),
            PyTokenKind::TkMul => Some(BinaryOperator::OpMul),
            PyTokenKind::TkDiv => Some(BinaryOperator::OpDiv),
            PyTokenKind::TkFloorDiv => Some(BinaryOperator::OpFloorDiv),
            PyTokenKind::TkMod => Some(BinaryOperator::OpMod),
            PyTokenKind::TkPow => Some(BinaryOperator::OpPow),
            PyTokenKind::TkAt => Some(BinaryOperator::OpMatMul),
            PyTokenKind::TkBitAnd => Some(BinaryOperator::OpBitAnd),
            PyTokenKind::TkBitOr => Some(BinaryOperator::OpBitOr),
            PyTokenKind::TkBitXor => Some(BinaryOperator::OpBitXor),
            PyTokenKind::TkShl => Some(BinaryOperator::OpLShift),
            PyTokenKind::TkShr => Some(BinaryOperator::OpRShift),
            PyTokenKind::TkEq => Some(BinaryOperator::OpEq),
            PyTokenKind::TkNe => Some(BinaryOperator::OpNotEq),
            PyTokenKind::TkLt => Some(BinaryOperator::OpLt),
            PyTokenKind::TkLe => Some(BinaryOperator::OpLtE),
            PyTokenKind::TkGt => Some(BinaryOperator::OpGt),
            PyTokenKind::TkGe => Some(BinaryOperator::OpGtE),
            PyTokenKind::TkIs => Some(BinaryOperator::OpIs),
            PyTokenKind::TkIn => Some(BinaryOperator::OpIn),
            PyTokenKind::TkAnd => Some(BinaryOperator::OpAnd),
            PyTokenKind::TkOr => Some(BinaryOperator::OpOr),
            _ => None,
        }
    }
}

impl UnaryOperator {
    pub fn from_token_kind(kind: super::PyTokenKind) -> Option<Self> {
        match kind {
            PyTokenKind::TkNot => Some(UnaryOperator::OpNot),
            PyTokenKind::TkPlus => Some(UnaryOperator::OpUPlus),
            PyTokenKind::TkMinus => Some(UnaryOperator::OpUMinus),
            PyTokenKind::TkBitNot => Some(UnaryOperator::OpInvert),
            _ => None,
        }
    }
}

pub const UNARY_PRIORITY: i32 = 12; // priority for unary operators
