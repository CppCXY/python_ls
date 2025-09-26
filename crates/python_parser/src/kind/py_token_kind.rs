use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum PyTokenKind {
    None,
    // Python Keywords
    TkAnd,       // and
    TkAs,        // as
    TkAssert,    // assert
    TkAsync,     // async
    TkAwait,     // await
    TkBreak,     // break
    TkClass,     // class
    TkContinue,  // continue
    TkDef,       // def
    TkDel,       // del
    TkElif,      // elif
    TkElse,      // else
    TkExcept,    // except
    TkFalse,     // False
    TkFinally,   // finally
    TkFor,       // for
    TkFrom,      // from
    TkGlobal,    // global
    TkIf,        // if
    TkImport,    // import
    TkIn,        // in
    TkIs,        // is
    TkLambda,    // lambda
    TkMatch,     // match
    TkCase,      // case
    TkNonlocal,  // nonlocal
    TkNone,      // None
    TkNot,       // not
    TkOr,        // or
    TkPass,      // pass
    TkRaise,     // raise
    TkReturn,    // return
    TkTry,       // try
    TkTrue,      // True
    TkWhile,     // while
    TkWith,      // with
    TkYield,     // yield

    TkWhitespace, // whitespace
    TkNewline,    // \n
    
    // Operators
    TkPlus,       // +
    TkMinus,      // -
    TkMul,        // *
    TkDiv,        // /
    TkFloorDiv,   // //
    TkMod,        // %
    TkPow,        // **
    TkMatMul,     // @ (matrix multiplication)
    
    // Bitwise operators
    TkBitAnd,     // &
    TkBitOr,      // |
    TkBitXor,     // ^
    TkBitNot,     // ~
    TkShl,        // <<
    TkShr,        // >>
    
    // Comparison operators
    TkEq,         // ==
    TkNe,         // !=
    TkLt,         // <
    TkLe,         // <=
    TkGt,         // >
    TkGe,         // >=
    
    // Assignment operators
    TkAssign,     // =
    TkPlusAssign,        // +=
    TkMinusAssign,       // -=
    TkMulAssign,         // *=
    TkDivAssign,         // /=
    TkFloorDivAssign,    // //=
    TkModAssign,         // %=
    TkPowAssign,         // **=
    TkMatMulAssign,      // @=
    TkBitAndAssign,      // &=
    TkBitOrAssign,       // |=
    TkBitXorAssign,      // ^=
    TkShlAssign,         // <<=
    TkShrAssign,         // >>=
    
    // Delimiters
    TkDot,        // .
    TkComma,      // ,
    TkColon,      // :
    TkSemicolon,  // ;
    TkArrow,      // ->
    TkAt,         // @


    // Brackets
    TkLeftBracket,  // [
    TkRightBracket, // ]
    TkLeftParen,    // (
    TkRightParen,   // )
    TkLeftBrace,    // {
    TkRightBrace,   // }
    
    // Literals
    TkInt,          // integer literal
    TkFloat,        // float literal
    TkComplex,      // complex literal
    TkString,       // string literal
    TkBytes,        // bytes literal
    TkFString,      // f-string literal
    TkRawString,    // raw string literal
    
    // Identifiers and Names
    TkName,         // identifier/name
    TkIndent,       // indentation
    TkDedent,       // dedentation
    
    // Comments
    TkComment,      // # comment
    TkShebang,      // shebang
    
    // Special
    TkEof,          // end of file
    TkUnknown,      // unknown token
    TkErrorToken,   // error token


}

impl fmt::Display for PyTokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PyTokenKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
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
                | PyTokenKind::TkNonlocal
                | PyTokenKind::TkNone
                | PyTokenKind::TkNot
                | PyTokenKind::TkOr
                | PyTokenKind::TkPass
                | PyTokenKind::TkRaise
                | PyTokenKind::TkReturn
                | PyTokenKind::TkTry
                | PyTokenKind::TkTrue
                | PyTokenKind::TkWhile
                | PyTokenKind::TkWith
                | PyTokenKind::TkYield
        )
    }

    pub fn is_assign_op(self) -> bool {
        matches!(
            self,
            PyTokenKind::TkAssign
                | PyTokenKind::TkPlusAssign
                | PyTokenKind::TkMinusAssign
                | PyTokenKind::TkMulAssign
                | PyTokenKind::TkDivAssign
                | PyTokenKind::TkFloorDivAssign
                | PyTokenKind::TkModAssign
                | PyTokenKind::TkPowAssign
                | PyTokenKind::TkMatMulAssign
                | PyTokenKind::TkBitAndAssign
                | PyTokenKind::TkBitOrAssign
                | PyTokenKind::TkBitXorAssign
                | PyTokenKind::TkShlAssign
                | PyTokenKind::TkShrAssign
        )
    }
}
