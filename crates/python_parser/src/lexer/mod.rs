mod lexer_config;
mod fstring_lexer;
mod py_lexer;
mod test;
mod token_data;

pub use lexer_config::LexerConfig;
pub use fstring_lexer::{FStringLexer, FStringToken};
pub use py_lexer::PyLexer;
pub use token_data::PyTokenData;

use crate::{kind::PyTokenKind, text::Reader};

pub(crate) fn is_name_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

pub(crate) fn is_name_continue(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// Lex an identifier/name from the reader
pub(crate) fn lex_name(reader: &mut Reader) -> PyTokenKind {
    reader.bump();
    reader.eat_while(is_name_continue);
    PyTokenKind::TkName
}

/// Lex a number literal (int, float, complex) from the reader
pub(crate) fn lex_number(reader: &mut Reader) -> PyTokenKind {
    enum NumberState {
        Int,
        Float,
        Hex,
        Binary,
        Octal,
        WithExpo,
    }

    let mut state = NumberState::Int;
    let first = reader.current_char();

    if first == '.' {
        // Starting with dot means it's a float
        reader.bump();
        state = NumberState::Float;
    } else {
        reader.bump();

        if first == '0' {
            match reader.current_char() {
                'x' | 'X' => {
                    reader.bump();
                    state = NumberState::Hex;
                }
                'b' | 'B' => {
                    reader.bump();
                    state = NumberState::Binary;
                }
                'o' | 'O' => {
                    reader.bump();
                    state = NumberState::Octal;
                }
                '0'..='7' => {
                    // Legacy octal (Python 2 style), treat as regular int
                    state = NumberState::Int;
                }
                _ => {
                    // Just a zero
                    state = NumberState::Int;
                }
            }
        }
    }

    while !reader.is_eof() {
        let ch = reader.current_char();
        let continue_ = match state {
            NumberState::Int => match ch {
                '0'..='9' | '_' => true, // Python allows underscores in numbers
                '.' => {
                    state = NumberState::Float;
                    true
                }
                'e' | 'E' => {
                    if matches!(reader.next_char(), '+' | '-') {
                        reader.bump();
                    }
                    state = NumberState::WithExpo;
                    true
                }
                _ => false,
            },
            NumberState::Float => match ch {
                '0'..='9' | '_' => true,
                'e' | 'E' => {
                    if matches!(reader.next_char(), '+' | '-') {
                        reader.bump();
                    }
                    state = NumberState::WithExpo;
                    true
                }
                _ => false,
            },
            NumberState::Hex => matches!(ch, '0'..='9' | 'a'..='f' | 'A'..='F' | '_'),
            NumberState::Binary => matches!(ch, '0' | '1' | '_'),
            NumberState::Octal => matches!(ch, '0'..='7' | '_'),
            NumberState::WithExpo => ch.is_ascii_digit() || ch == '_',
        };

        if continue_ {
            reader.bump();
        } else {
            break;
        }
    }

    // Check for imaginary number suffix
    if reader.current_char() == 'j' || reader.current_char() == 'J' {
        reader.bump();
        return PyTokenKind::TkComplex;
    }

    match state {
        NumberState::Int | NumberState::Hex | NumberState::Binary | NumberState::Octal => {
            PyTokenKind::TkInt
        }
        _ => PyTokenKind::TkFloat,
    }
}

/// Lex operators and punctuation from the reader
pub(crate) fn lex_operator(reader: &mut Reader) -> PyTokenKind {
    let ch = reader.current_char();
    reader.bump();

    match ch {
        '-' => match reader.current_char() {
            '=' => {
                reader.bump();
                PyTokenKind::TkMinusAssign
            }
            '>' => {
                reader.bump();
                PyTokenKind::TkArrow
            }
            _ => PyTokenKind::TkMinus,
        },
        '[' => PyTokenKind::TkLeftBracket,
        '=' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkEq
            } else {
                PyTokenKind::TkAssign
            }
        }
        '<' => match reader.current_char() {
            '=' => {
                reader.bump();
                PyTokenKind::TkLe
            }
            '<' => {
                reader.bump();
                if reader.current_char() == '=' {
                    reader.bump();
                    PyTokenKind::TkShlAssign
                } else {
                    PyTokenKind::TkShl
                }
            }
            _ => PyTokenKind::TkLt,
        },
        '>' => match reader.current_char() {
            '=' => {
                reader.bump();
                PyTokenKind::TkGe
            }
            '>' => {
                reader.bump();
                if reader.current_char() == '=' {
                    reader.bump();
                    PyTokenKind::TkShrAssign
                } else {
                    PyTokenKind::TkShr
                }
            }
            _ => PyTokenKind::TkGt,
        },
        '~' => PyTokenKind::TkBitNot,
        ':' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkColonAssign
            } else {
                PyTokenKind::TkColon
            }
        }
        '.' => {
            // Check for ellipsis (...)
            if reader.current_char() == '.' && reader.next_char() == '.' {
                reader.bump(); // consume second '.'
                reader.bump(); // consume third '.'
                PyTokenKind::TkEllipsis
            } else {
                PyTokenKind::TkDot
            }
        }
        '/' => match reader.current_char() {
            '/' => {
                reader.bump();
                if reader.current_char() == '=' {
                    reader.bump();
                    PyTokenKind::TkFloorDivAssign
                } else {
                    PyTokenKind::TkFloorDiv
                }
            }
            '=' => {
                reader.bump();
                PyTokenKind::TkDivAssign
            }
            _ => PyTokenKind::TkDiv,
        },
        '*' => match reader.current_char() {
            '*' => {
                reader.bump();
                if reader.current_char() == '=' {
                    reader.bump();
                    PyTokenKind::TkPowAssign
                } else {
                    PyTokenKind::TkPow
                }
            }
            '=' => {
                reader.bump();
                PyTokenKind::TkMulAssign
            }
            _ => PyTokenKind::TkMul,
        },
        '+' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkPlusAssign
            } else {
                PyTokenKind::TkPlus
            }
        }
        '%' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkModAssign
            } else {
                PyTokenKind::TkMod
            }
        }
        '^' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkBitXorAssign
            } else {
                PyTokenKind::TkBitXor
            }
        }
        '!' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkNe
            } else {
                PyTokenKind::TkUnknown
            }
        }
        '&' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkBitAndAssign
            } else {
                PyTokenKind::TkBitAnd
            }
        }
        '|' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkBitOrAssign
            } else {
                PyTokenKind::TkBitOr
            }
        }
        '(' => PyTokenKind::TkLeftParen,
        ')' => PyTokenKind::TkRightParen,
        '{' => PyTokenKind::TkLeftBrace,
        '}' => PyTokenKind::TkRightBrace,
        ']' => PyTokenKind::TkRightBracket,
        ';' => PyTokenKind::TkSemicolon,
        ',' => PyTokenKind::TkComma,
        '@' => {
            if reader.current_char() == '=' {
                reader.bump();
                PyTokenKind::TkMatMulAssign
            } else {
                PyTokenKind::TkMatMul
            }
        }
        _ => PyTokenKind::TkUnknown,
    }
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
