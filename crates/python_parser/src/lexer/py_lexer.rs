use crate::{kind::PyTokenKind, parser_error::PyParseError, text::Reader};

use super::{is_name_continue, is_name_start, lexer_config::LexerConfig, token_data::PyTokenData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LexerState {
    Normal,
    String(char, PyTokenKind),       // quote character and string type
    TripleString(char, PyTokenKind), // quote character and string type for triple-quoted strings
}

#[derive(Debug, Clone)]
struct IndentInfo {
    indent_stack: Vec<usize>,
    at_line_start: bool,
    pending_dedents: usize,
}

pub struct PyLexer<'a> {
    reader: Reader<'a>,
    #[allow(unused)]
    lexer_config: LexerConfig,
    errors: Option<&'a mut Vec<PyParseError>>,
    state: LexerState,
    indent_info: IndentInfo,
}

impl<'a> PyLexer<'a> {
    pub fn new(
        reader: Reader<'a>,
        lexer_config: LexerConfig,
        errors: Option<&'a mut Vec<PyParseError>>,
    ) -> Self {
        Self::new_with_state(reader, LexerState::Normal, lexer_config, errors)
    }

    pub fn new_with_state(
        reader: Reader<'a>,
        state: LexerState,
        lexer_config: LexerConfig,
        errors: Option<&'a mut Vec<PyParseError>>,
    ) -> Self {
        PyLexer {
            reader,
            lexer_config,
            errors,
            state,
            indent_info: IndentInfo {
                indent_stack: vec![0],
                at_line_start: true,
                pending_dedents: 0,
            },
        }
    }

    pub fn tokenize(&mut self) -> Vec<PyTokenData> {
        let mut tokens = vec![];

        while !self.reader.is_eof() {
            // Handle pending dedents
            if self.indent_info.pending_dedents > 0 {
                self.indent_info.pending_dedents -= 1;
                tokens.push(PyTokenData::new(
                    PyTokenKind::TkDedent,
                    self.reader.current_range(),
                ));
                continue;
            }

            let kind = match self.state {
                LexerState::Normal => self.lex(),
                LexerState::String(quote, string_type) => self.lex_string(quote, string_type),
                LexerState::TripleString(quote, string_type) => {
                    self.lex_triple_string(quote, string_type)
                }
            };

            if kind == PyTokenKind::TkEof {
                // Generate final dedents
                while self.indent_info.indent_stack.len() > 1 {
                    self.indent_info.indent_stack.pop();
                    tokens.push(PyTokenData::new(
                        PyTokenKind::TkDedent,
                        self.reader.current_range(),
                    ));
                }
                break;
            }

            tokens.push(PyTokenData::new(kind, self.reader.current_range()));
        }

        tokens
    }

    pub fn get_state(&self) -> LexerState {
        self.state
    }

    pub fn continue_with_new_reader(&mut self, reader: Reader<'a>) -> Vec<PyTokenData> {
        assert!(self.reader.is_eof(), "previous reader wasn't exhausted");
        self.reader = reader;
        self.tokenize()
    }

    fn name_to_kind(&self, name: &str) -> PyTokenKind {
        match name {
            // Python keywords
            "and" => PyTokenKind::TkAnd,
            "as" => PyTokenKind::TkAs,
            "assert" => PyTokenKind::TkAssert,
            "async" => PyTokenKind::TkAsync,
            "await" => PyTokenKind::TkAwait,
            "break" => PyTokenKind::TkBreak,
            "class" => PyTokenKind::TkClass,
            "continue" => PyTokenKind::TkContinue,
            "def" => PyTokenKind::TkDef,
            "del" => PyTokenKind::TkDel,
            "elif" => PyTokenKind::TkElif,
            "else" => PyTokenKind::TkElse,
            "except" => PyTokenKind::TkExcept,
            "False" => PyTokenKind::TkFalse,
            "finally" => PyTokenKind::TkFinally,
            "for" => PyTokenKind::TkFor,
            "from" => PyTokenKind::TkFrom,
            "global" => PyTokenKind::TkGlobal,
            "if" => PyTokenKind::TkIf,
            "import" => PyTokenKind::TkImport,
            "in" => PyTokenKind::TkIn,
            "is" => PyTokenKind::TkIs,
            "lambda" => PyTokenKind::TkLambda,
            "nonlocal" => PyTokenKind::TkNonlocal,
            "None" => PyTokenKind::TkNone,
            "not" => PyTokenKind::TkNot,
            "or" => PyTokenKind::TkOr,
            "pass" => PyTokenKind::TkPass,
            "raise" => PyTokenKind::TkRaise,
            "return" => PyTokenKind::TkReturn,
            "try" => PyTokenKind::TkTry,
            "True" => PyTokenKind::TkTrue,
            "while" => PyTokenKind::TkWhile,
            "with" => PyTokenKind::TkWith,
            "yield" => PyTokenKind::TkYield,
            _ => PyTokenKind::TkName,
        }
    }

    fn lex(&mut self) -> PyTokenKind {
        self.reader.reset_buff();

        // Handle indentation at the beginning of a line
        if self.indent_info.at_line_start {
            return self.handle_indentation();
        }

        match self.reader.current_char() {
            '\n' | '\r' => self.lex_new_line(),
            ' ' | '\t' => self.lex_white_space(),
            '-' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '=' => {
                        self.reader.bump();
                        PyTokenKind::TkMinusAssign
                    }
                    '>' => {
                        self.reader.bump();
                        PyTokenKind::TkArrow
                    }
                    _ => PyTokenKind::TkMinus,
                }
            }
            '[' => {
                self.reader.bump();
                PyTokenKind::TkLeftBracket
            }
            '=' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkEq
                } else {
                    PyTokenKind::TkAssign
                }
            }
            '<' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '=' => {
                        self.reader.bump();
                        PyTokenKind::TkLe
                    }
                    '<' => {
                        self.reader.bump();
                        if self.reader.current_char() == '=' {
                            self.reader.bump();
                            PyTokenKind::TkShlAssign
                        } else {
                            PyTokenKind::TkShl
                        }
                    }
                    _ => PyTokenKind::TkLt,
                }
            }
            '>' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '=' => {
                        self.reader.bump();
                        PyTokenKind::TkGe
                    }
                    '>' => {
                        self.reader.bump();
                        if self.reader.current_char() == '=' {
                            self.reader.bump();
                            PyTokenKind::TkShrAssign
                        } else {
                            PyTokenKind::TkShr
                        }
                    }
                    _ => PyTokenKind::TkGt,
                }
            }
            '~' => {
                self.reader.bump();
                PyTokenKind::TkBitNot
            }
            ':' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkColonAssign
                } else {
                    PyTokenKind::TkColon
                }
            }
            '"' | '\'' => {
                let quote = self.reader.current_char();
                self.reader.bump();

                // Check for triple-quoted strings
                if self.reader.current_char() == quote && self.reader.next_char() == quote {
                    self.reader.bump(); // second quote
                    self.reader.bump(); // third quote
                    self.state = LexerState::TripleString(quote, PyTokenKind::TkString);
                    self.lex_triple_string(quote, PyTokenKind::TkString)
                } else {
                    self.state = LexerState::String(quote, PyTokenKind::TkString);
                    self.lex_string(quote, PyTokenKind::TkString)
                }
            }
            '.' => {
                if self.reader.next_char().is_ascii_digit() {
                    return self.lex_number();
                }

                // Check for ellipsis (...)
                if self.reader.next_char() == '.' {
                    self.reader.bump(); // consume first '.'
                    if self.reader.current_char() == '.' && self.reader.next_char() == '.' {
                        self.reader.bump(); // consume second '.'
                        self.reader.bump(); // consume third '.'
                        PyTokenKind::TkEllipsis
                    } else {
                        // We already consumed one dot, return TkDot
                        PyTokenKind::TkDot
                    }
                } else {
                    self.reader.bump();
                    PyTokenKind::TkDot
                }
            }
            '0'..='9' => self.lex_number(),
            '/' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '/' => {
                        self.reader.bump();
                        if self.reader.current_char() == '=' {
                            self.reader.bump();
                            PyTokenKind::TkFloorDivAssign
                        } else {
                            PyTokenKind::TkFloorDiv
                        }
                    }
                    '=' => {
                        self.reader.bump();
                        PyTokenKind::TkDivAssign
                    }
                    _ => PyTokenKind::TkDiv,
                }
            }
            '*' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '*' => {
                        self.reader.bump();
                        if self.reader.current_char() == '=' {
                            self.reader.bump();
                            PyTokenKind::TkPowAssign
                        } else {
                            PyTokenKind::TkPow
                        }
                    }
                    '=' => {
                        self.reader.bump();
                        PyTokenKind::TkMulAssign
                    }
                    _ => PyTokenKind::TkMul,
                }
            }
            '+' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkPlusAssign
                } else {
                    PyTokenKind::TkPlus
                }
            }
            '%' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkModAssign
                } else {
                    PyTokenKind::TkMod
                }
            }
            '^' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkBitXorAssign
                } else {
                    PyTokenKind::TkBitXor
                }
            }
            '#' => {
                self.reader.bump();
                if self.reader.current_char() == '!' {
                    // Shebang
                    self.reader.eat_while(|ch| ch != '\n' && ch != '\r');
                    PyTokenKind::TkShebang
                } else {
                    // Regular comment
                    self.reader.eat_while(|ch| ch != '\n' && ch != '\r');
                    PyTokenKind::TkComment
                }
            }
            '!' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkNe
                } else {
                    PyTokenKind::TkUnknown // ! is not a valid operator in Python by itself
                }
            }
            '&' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkBitAndAssign
                } else {
                    PyTokenKind::TkBitAnd
                }
            }
            '|' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkBitOrAssign
                } else {
                    PyTokenKind::TkBitOr
                }
            }
            '(' => {
                self.reader.bump();
                PyTokenKind::TkLeftParen
            }
            ')' => {
                self.reader.bump();
                PyTokenKind::TkRightParen
            }
            '{' => {
                self.reader.bump();
                PyTokenKind::TkLeftBrace
            }
            '}' => {
                self.reader.bump();
                PyTokenKind::TkRightBrace
            }
            ']' => {
                self.reader.bump();
                PyTokenKind::TkRightBracket
            }
            ';' => {
                self.reader.bump();
                PyTokenKind::TkSemicolon
            }
            ',' => {
                self.reader.bump();
                PyTokenKind::TkComma
            }
            '@' => {
                self.reader.bump();
                if self.reader.current_char() == '=' {
                    self.reader.bump();
                    PyTokenKind::TkMatMulAssign
                } else {
                    PyTokenKind::TkMatMul
                }
            }
            _ if self.reader.is_eof() => PyTokenKind::TkEof,
            ch if is_name_start(ch) => {
                self.reader.bump();
                self.reader.eat_while(is_name_continue);
                let name = self.reader.current_text();

                // Check if this is a string prefix
                if let Some(string_token) = self.try_parse_prefixed_string(name) {
                    string_token
                } else {
                    self.name_to_kind(name)
                }
            }
            _ => {
                self.reader.bump();
                PyTokenKind::TkUnknown
            }
        }
    }

    fn handle_indentation(&mut self) -> PyTokenKind {
        let mut indent_level = 0;

        // Skip whitespace and count indentation
        while matches!(self.reader.current_char(), ' ' | '\t') {
            if self.reader.current_char() == ' ' {
                indent_level += 1;
            } else if self.reader.current_char() == '\t' {
                indent_level += 8; // Tab equals 8 spaces
            }
            self.reader.bump();
        }

        // If we hit a comment or newline, ignore this line
        if matches!(self.reader.current_char(), '#' | '\n' | '\r') || self.reader.is_eof() {
            self.indent_info.at_line_start = false;
            return self.lex();
        }

        self.indent_info.at_line_start = false;

        let current_indent = *self.indent_info.indent_stack.last().unwrap();

        if indent_level > current_indent {
            // Increased indentation
            self.indent_info.indent_stack.push(indent_level);
            PyTokenKind::TkIndent
        } else if indent_level < current_indent {
            // Decreased indentation - may need multiple DEDENTs
            let mut dedent_count = 0;
            while let Some(&stack_indent) = self.indent_info.indent_stack.last() {
                if stack_indent <= indent_level {
                    break;
                }
                self.indent_info.indent_stack.pop();
                dedent_count += 1;
            }

            if dedent_count > 0 {
                self.indent_info.pending_dedents = dedent_count - 1;
                PyTokenKind::TkDedent
            } else {
                // Continue with normal lexing
                self.lex()
            }
        } else {
            // Same indentation level
            self.lex()
        }
    }

    fn lex_new_line(&mut self) -> PyTokenKind {
        match self.reader.current_char() {
            // support \n or \n\r
            '\n' => {
                self.reader.bump();
                if self.reader.current_char() == '\r' {
                    self.reader.bump();
                }
            }
            // support \r or \r\n
            '\r' => {
                self.reader.bump();
                if self.reader.current_char() == '\n' {
                    self.reader.bump();
                }
            }
            _ => {}
        }

        self.indent_info.at_line_start = true;
        PyTokenKind::TkNewline
    }

    fn lex_white_space(&mut self) -> PyTokenKind {
        self.reader.eat_while(|ch| ch == ' ' || ch == '\t');
        PyTokenKind::TkWhitespace
    }

    fn lex_triple_string(&mut self, quote: char, string_type: PyTokenKind) -> PyTokenKind {
        while !self.reader.is_eof() {
            if self.reader.current_char() == quote {
                self.reader.bump(); // consume first quote
                if self.reader.current_char() == quote {
                    self.reader.bump(); // consume second quote
                    if self.reader.current_char() == quote {
                        self.reader.bump(); // consume third quote
                        self.state = LexerState::Normal;
                        return string_type;
                    }
                }
                // If we don't find three quotes, continue
                continue;
            }

            if self.reader.current_char() == '\\' {
                self.reader.bump(); // skip escape character
                if !self.reader.is_eof() {
                    self.reader.bump(); // skip escaped character
                }
            } else {
                self.reader.bump();
            }
        }

        self.error(|| t!("Unterminated triple-quoted string"));
        self.state = LexerState::Normal;
        string_type
    }

    fn lex_string(&mut self, quote: char, string_type: PyTokenKind) -> PyTokenKind {
        while !self.reader.is_eof() {
            let ch = self.reader.current_char();
            if ch == quote {
                break;
            }

            // Python strings cannot span multiple lines unless they're triple-quoted
            if ch == '\n' || ch == '\r' {
                self.error(|| "Unterminated string literal");
                self.state = LexerState::Normal;
                return string_type;
            }

            if ch == '\\' {
                self.reader.bump(); // consume backslash
                if !self.reader.is_eof() {
                    // Handle escape sequences
                    match self.reader.current_char() {
                        '\n' | '\r' => {
                            // Line continuation
                            self.lex_new_line();
                        }
                        _ => {
                            self.reader.bump(); // consume escaped character
                        }
                    }
                }
            } else {
                self.reader.bump();
            }
        }

        if self.reader.current_char() == quote {
            self.reader.bump(); // consume closing quote
            self.state = LexerState::Normal;
            string_type
        } else {
            self.error(|| "Unterminated string literal");
            self.state = LexerState::Normal;
            string_type
        }
    }

    /// Check if a name is a string prefix (r, u, b, f) and return the corresponding token type
    fn get_string_type_from_prefix(name: &str) -> Option<PyTokenKind> {
        match name.to_lowercase().as_str() {
            "r" => Some(PyTokenKind::TkRawString),
            "b" => Some(PyTokenKind::TkBytesString),
            "f" => Some(PyTokenKind::TkFString),
            "u" => Some(PyTokenKind::TkString), // Unicode strings are regular strings in Python 3
            "rb" | "br" => Some(PyTokenKind::TkRawBytesString), // Raw bytes
            "rf" | "fr" => Some(PyTokenKind::TkFString), // Raw f-string (if supported)
            "ur" | "ru" => Some(PyTokenKind::TkString), // Raw unicode (same as regular string)
            _ => None,
        }
    }

    /// Try to parse a string literal with possible prefix
    fn try_parse_prefixed_string(&mut self, name: &str) -> Option<PyTokenKind> {
        if let Some(string_type) = Self::get_string_type_from_prefix(name) {
            // Check if next character is a quote
            if matches!(self.reader.current_char(), '"' | '\'') {
                let quote = self.reader.current_char();
                self.reader.bump();

                // Check for triple-quoted strings
                if self.reader.current_char() == quote && self.reader.next_char() == quote {
                    self.reader.bump(); // second quote
                    self.reader.bump(); // third quote
                    self.state = LexerState::TripleString(quote, string_type);
                    return Some(self.lex_triple_string(quote, string_type));
                } else {
                    self.state = LexerState::String(quote, string_type);
                    return Some(self.lex_string(quote, string_type));
                }
            }
        }
        None
    }

    fn lex_number(&mut self) -> PyTokenKind {
        enum NumberState {
            Int,
            Float,
            Hex,
            Binary,
            Octal,
            WithExpo,
        }

        let mut state = NumberState::Int;
        let first = self.reader.current_char();

        if first == '.' {
            // Starting with dot means it's a float
            self.reader.bump();
            state = NumberState::Float;
        } else {
            self.reader.bump();

            if first == '0' {
                match self.reader.current_char() {
                    'x' | 'X' => {
                        self.reader.bump();
                        state = NumberState::Hex;
                    }
                    'b' | 'B' => {
                        self.reader.bump();
                        state = NumberState::Binary;
                    }
                    'o' | 'O' => {
                        self.reader.bump();
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

        while !self.reader.is_eof() {
            let ch = self.reader.current_char();
            let continue_ = match state {
                NumberState::Int => match ch {
                    '0'..='9' | '_' => true, // Python allows underscores in numbers
                    '.' => {
                        state = NumberState::Float;
                        true
                    }
                    'e' | 'E' => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::Float => match ch {
                    '0'..='9' | '_' => true,
                    'e' | 'E' => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
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
                self.reader.bump();
            } else {
                break;
            }
        }

        // Check for imaginary number suffix
        if self.reader.current_char() == 'j' || self.reader.current_char() == 'J' {
            self.reader.bump();
            return PyTokenKind::TkComplex;
        }

        // Check for invalid characters after number
        if self.reader.current_char().is_alphabetic() {
            let ch = self.reader.current_char();
            self.error(|| format!("Invalid character '{}' in number literal", ch));
        }

        match state {
            NumberState::Int | NumberState::Hex | NumberState::Binary | NumberState::Octal => {
                PyTokenKind::TkInt
            }
            _ => PyTokenKind::TkFloat,
        }
    }

    fn error<F, R>(&mut self, msg: F)
    where
        F: FnOnce() -> R,
        R: AsRef<str>,
    {
        if let Some(errors) = &mut self.errors {
            errors.push(PyParseError::syntax_error_from(
                msg().as_ref(),
                self.reader.current_range(),
            ))
        }
    }
}
