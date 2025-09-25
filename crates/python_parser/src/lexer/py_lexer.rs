use crate::{
    LexerState, LuaNonStdSymbol, kind::PyTokenKind, parser_error::LuaParseError, text::Reader,
};

use super::{is_name_continue, is_name_start, lexer_config::LexerConfig, token_data::PyTokenData};

pub struct LuaLexer<'a> {
    reader: Reader<'a>,
    lexer_config: LexerConfig,
    errors: Option<&'a mut Vec<LuaParseError>>,
    state: LexerState,
}

impl<'a> LuaLexer<'a> {
    pub fn new(
        reader: Reader<'a>,
        lexer_config: LexerConfig,
        errors: Option<&'a mut Vec<LuaParseError>>,
    ) -> Self {
        Self::new_with_state(reader, LexerState::Normal, lexer_config, errors)
    }

    pub fn new_with_state(
        reader: Reader<'a>,
        state: LexerState,
        lexer_config: LexerConfig,
        errors: Option<&'a mut Vec<LuaParseError>>,
    ) -> Self {
        LuaLexer {
            reader,
            lexer_config,
            errors,
            state,
        }
    }

    pub fn tokenize(&mut self) -> Vec<PyTokenData> {
        let mut tokens = vec![];

        while !self.reader.is_eof() {
            let kind = match self.state {
                LexerState::Normal => self.lex(),
                LexerState::String(quote) => self.lex_string(quote),
                LexerState::LongString(sep) => self.lex_long_string(sep),
                LexerState::LongComment(sep) => {
                    self.lex_long_string(sep);
                    PyTokenKind::TkLongComment
                }
            };
            if kind == PyTokenKind::TkEof {
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

    fn support_non_std_symbol(&self, symbol: LuaNonStdSymbol) -> bool {
        self.lexer_config.non_std_symbols.support(symbol)
    }

    fn name_to_kind(&self, name: &str) -> PyTokenKind {
        match name {
            "and" => PyTokenKind::TkAnd,
            "break" => PyTokenKind::TkBreak,
            "do" => PyTokenKind::TkDo,
            "else" => PyTokenKind::TkElse,
            "elseif" => PyTokenKind::TkElseIf,
            "end" => PyTokenKind::TkEnd,
            "false" => PyTokenKind::TkFalse,
            "for" => PyTokenKind::TkFor,
            "function" => PyTokenKind::TkFunction,
            "goto" => {
                if self.lexer_config.support_goto() {
                    PyTokenKind::TkGoto
                } else {
                    PyTokenKind::TkName
                }
            }
            "if" => PyTokenKind::TkIf,
            "in" => PyTokenKind::TkIn,
            "local" => PyTokenKind::TkLocal,
            "nil" => PyTokenKind::TkNil,
            "not" => PyTokenKind::TkNot,
            "or" => PyTokenKind::TkOr,
            "repeat" => PyTokenKind::TkRepeat,
            "return" => PyTokenKind::TkReturn,
            "then" => PyTokenKind::TkThen,
            "true" => PyTokenKind::TkTrue,
            "until" => PyTokenKind::TkUntil,
            "while" => PyTokenKind::TkWhile,
            "continue" => {
                if self.support_non_std_symbol(LuaNonStdSymbol::Continue) {
                    PyTokenKind::TkBreak
                } else {
                    PyTokenKind::TkName
                }
            }
            _ => PyTokenKind::TkName,
        }
    }

    fn lex(&mut self) -> PyTokenKind {
        self.reader.reset_buff();

        match self.reader.current_char() {
            '\n' | '\r' => self.lex_new_line(),
            ' ' | '\t' => self.lex_white_space(),
            '-' => {
                self.reader.bump();
                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::MinusAssign)
                {
                    self.reader.bump();
                    return PyTokenKind::TkMinusAssign;
                }
                if self.reader.current_char() != '-' {
                    return PyTokenKind::TkMinus;
                }

                self.reader.bump();
                if self.reader.current_char() == '[' {
                    self.reader.bump();
                    let sep = self.skip_sep();
                    if self.reader.current_char() == '[' {
                        self.reader.bump();
                        self.state = LexerState::LongComment(sep);
                        self.lex_long_string(sep);
                        return PyTokenKind::TkLongComment;
                    }
                }

                self.reader.eat_while(|ch| ch != '\n' && ch != '\r');
                PyTokenKind::TkShortComment
            }
            '[' => {
                self.reader.bump();
                let sep = self.skip_sep();
                if sep == 0 && self.reader.current_char() != '[' {
                    return PyTokenKind::TkLeftBracket;
                }
                if self.reader.current_char() != '[' {
                    self.error(|| t!("invalid long string delimiter"));
                    return PyTokenKind::TkLongString;
                }

                self.reader.bump();
                self.state = LexerState::LongString(sep);
                self.lex_long_string(sep)
            }
            '=' => {
                self.reader.bump();
                if self.reader.current_char() != '=' {
                    return PyTokenKind::TkAssign;
                }
                self.reader.bump();
                PyTokenKind::TkEq
            }
            '<' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '=' => {
                        self.reader.bump();
                        PyTokenKind::TkLe
                    }
                    '<' => {
                        if !self.lexer_config.support_integer_operation() {
                            self.error(|| t!("bitwise operation is not supported"));
                        }

                        self.reader.bump();
                        if self.reader.current_char() == '='
                            && self.support_non_std_symbol(LuaNonStdSymbol::ShiftLeftAssign)
                        {
                            self.reader.bump();
                            return PyTokenKind::TkShiftLeftAssign;
                        }
                        PyTokenKind::TkShl
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
                        if !self.lexer_config.support_integer_operation() {
                            self.error(|| t!("bitwise operation is not supported"));
                        }

                        self.reader.bump();
                        if self.reader.current_char() == '='
                            && self.support_non_std_symbol(LuaNonStdSymbol::ShiftRightAssign)
                        {
                            self.reader.bump();
                            return PyTokenKind::TkShiftRightAssign;
                        }
                        PyTokenKind::TkShr
                    }
                    _ => PyTokenKind::TkGt,
                }
            }
            '~' => {
                self.reader.bump();
                if self.reader.current_char() != '=' {
                    if !self.lexer_config.support_integer_operation() {
                        self.error(|| t!("bitwise operation is not supported"));
                    }
                    return PyTokenKind::TkBitXor;
                }
                self.reader.bump();
                PyTokenKind::TkNe
            }
            ':' => {
                self.reader.bump();
                if self.reader.current_char() != ':' {
                    return PyTokenKind::TkColon;
                }
                self.reader.bump();
                PyTokenKind::TkDbColon
            }
            '"' | '\'' | '`' => {
                let quote = self.reader.current_char();
                if quote == '`' && !self.support_non_std_symbol(LuaNonStdSymbol::Backtick) {
                    self.reader.bump();
                    return PyTokenKind::TkUnknown;
                }

                self.reader.bump();
                self.state = LexerState::String(quote);
                self.lex_string(quote)
            }
            '.' => {
                if self.reader.next_char().is_ascii_digit() {
                    return self.lex_number();
                }

                self.reader.bump();
                if self.reader.current_char() != '.' {
                    return PyTokenKind::TkDot;
                }
                self.reader.bump();
                if self.reader.current_char() != '.' {
                    return PyTokenKind::TkConcat;
                }
                self.reader.bump();
                PyTokenKind::TkDots
            }
            '0'..='9' => self.lex_number(),
            '/' => {
                self.reader.bump();
                let current_char = self.reader.current_char();
                match current_char {
                    '*' if self.support_non_std_symbol(LuaNonStdSymbol::SlashStar) => {
                        // "/*" is a long comment
                        self.reader.bump();
                        loop {
                            let ch = self.reader.current_char();
                            match ch {
                                '*' => {
                                    self.reader.bump();
                                    if self.reader.current_char() == '/' {
                                        self.reader.bump();
                                        return PyTokenKind::TkLongComment;
                                    }
                                }
                                _ if self.reader.is_eof() => {
                                    self.error(|| t!("unfinished long comment"));
                                    return PyTokenKind::TkLongComment;
                                }
                                _ => {
                                    self.reader.bump();
                                }
                            }
                        }
                    }
                    '=' if self.support_non_std_symbol(LuaNonStdSymbol::SlashAssign) => {
                        self.reader.bump();
                        PyTokenKind::TkSlashAssign
                    }
                    _ if current_char != '/' => PyTokenKind::TkDiv,
                    _ if self.support_non_std_symbol(LuaNonStdSymbol::DoubleSlash) => {
                        // "//" is a short comment
                        self.reader.bump();
                        self.reader.eat_while(|ch| ch != '\n' && ch != '\r');
                        PyTokenKind::TkShortComment
                    }
                    _ => {
                        if !self.lexer_config.support_integer_operation() {
                            self.error(|| t!("integer division is not supported"));
                        }

                        self.reader.bump();
                        if self.reader.current_char() == '='
                            && self.support_non_std_symbol(LuaNonStdSymbol::DoubleSlashAssign)
                        {
                            self.reader.bump();
                            return PyTokenKind::TkDoubleSlashAssign;
                        }
                        PyTokenKind::TkIDiv
                    }
                }
            }
            '*' => {
                self.reader.bump();
                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::StarAssign)
                {
                    self.reader.bump();
                    return PyTokenKind::TkStarAssign;
                }
                PyTokenKind::TkMul
            }
            '+' => {
                self.reader.bump();
                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::PlusAssign)
                {
                    self.reader.bump();
                    return PyTokenKind::TkPlusAssign;
                }
                PyTokenKind::TkPlus
            }
            '%' => {
                self.reader.bump();
                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::PercentAssign)
                {
                    self.reader.bump();
                    return PyTokenKind::TkPercentAssign;
                }
                PyTokenKind::TkMod
            }
            '^' => {
                self.reader.bump();
                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::CaretAssign)
                {
                    self.reader.bump();
                    return PyTokenKind::TkCaretAssign;
                }
                PyTokenKind::TkPow
            }
            '#' => {
                self.reader.bump();
                if self.reader.current_char() != '!' {
                    return PyTokenKind::TkLen;
                }
                self.reader.eat_while(|ch| ch != '\n' && ch != '\r');
                PyTokenKind::TkShebang
            }
            '!' => {
                if !self.support_non_std_symbol(LuaNonStdSymbol::Exclamation) {
                    self.reader.bump();
                    return PyTokenKind::TkUnknown;
                }

                self.reader.bump();
                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::NotEqual)
                {
                    self.reader.bump();
                    return PyTokenKind::TkNe;
                }
                PyTokenKind::TkNot
            }
            '&' => {
                self.reader.bump();
                if self.reader.current_char() == '&'
                    && self.support_non_std_symbol(LuaNonStdSymbol::DoubleAmp)
                {
                    self.reader.bump();
                    return PyTokenKind::TkAnd;
                }
                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::AmpAssign)
                {
                    self.reader.bump();
                    return PyTokenKind::TkAmpAssign;
                }

                if !self.lexer_config.support_integer_operation() {
                    self.error(|| t!("bitwise operation is not supported"));
                }
                PyTokenKind::TkBitAnd
            }
            '|' => {
                self.reader.bump();
                if self.reader.current_char() == '|'
                    && self.support_non_std_symbol(LuaNonStdSymbol::DoublePipe)
                {
                    self.reader.bump();
                    return PyTokenKind::TkOr;
                }

                if self.reader.current_char() == '='
                    && self.support_non_std_symbol(LuaNonStdSymbol::PipeAssign)
                {
                    self.reader.bump();
                    return PyTokenKind::TkPipeAssign;
                }

                if !self.lexer_config.support_integer_operation() {
                    self.error(|| t!("bitwise operation is not supported"));
                }
                PyTokenKind::TkBitOr
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
                PyTokenKind::TkAt
            }
            _ if self.reader.is_eof() => PyTokenKind::TkEof,
            ch if is_name_start(ch) => {
                self.reader.bump();
                self.reader.eat_while(is_name_continue);
                let name = self.reader.current_text();
                self.name_to_kind(name)
            }
            _ => {
                self.reader.bump();
                PyTokenKind::TkUnknown
            }
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

        PyTokenKind::TkEndOfLine
    }

    fn lex_white_space(&mut self) -> PyTokenKind {
        self.reader.eat_while(|ch| ch == ' ' || ch == '\t');
        PyTokenKind::TkWhitespace
    }

    fn skip_sep(&mut self) -> usize {
        self.reader.eat_when('=')
    }

    fn lex_string(&mut self, quote: char) -> PyTokenKind {
        while !self.reader.is_eof() {
            let ch = self.reader.current_char();
            if ch == quote || ch == '\n' || ch == '\r' {
                break;
            }

            if ch != '\\' {
                self.reader.bump();
                continue;
            }

            self.reader.bump();
            match self.reader.current_char() {
                'z' => {
                    self.reader.bump();
                    self.reader
                        .eat_while(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n');
                }
                '\r' | '\n' => {
                    self.lex_new_line();
                }
                _ => {
                    self.reader.bump();
                }
            }
        }

        if self.reader.current_char() == quote || !self.reader.is_eof() {
            self.state = LexerState::Normal;
        }

        if self.reader.current_char() != quote {
            self.error(|| t!("unfinished string"));
            return PyTokenKind::TkString;
        }

        self.reader.bump();
        PyTokenKind::TkString
    }

    fn lex_long_string(&mut self, sep: usize) -> PyTokenKind {
        let mut end = false;
        while !self.reader.is_eof() {
            match self.reader.current_char() {
                ']' => {
                    self.reader.bump();
                    let count = self.reader.eat_when('=');
                    if count == sep && self.reader.current_char() == ']' {
                        self.reader.bump();
                        end = true;
                        break;
                    }
                }
                _ => {
                    self.reader.bump();
                }
            }
        }

        if end || !self.reader.is_eof() {
            self.state = LexerState::Normal;
        }

        if !end {
            self.error(|| t!("unfinished long string or comment"));
        }

        PyTokenKind::TkLongString
    }

    fn lex_number(&mut self) -> PyTokenKind {
        enum NumberState {
            Int,
            Float,
            Hex,
            HexFloat,
            WithExpo,
            Bin,
        }

        let mut state = NumberState::Int;
        let first = self.reader.current_char();
        self.reader.bump();
        match first {
            '0' if matches!(self.reader.current_char(), 'X' | 'x') => {
                self.reader.bump();
                state = NumberState::Hex;
            }
            '0' if matches!(self.reader.current_char(), 'B' | 'b')
                && self.lexer_config.support_binary_integer() =>
            {
                self.reader.bump();
                state = NumberState::Bin;
            }
            '.' => {
                state = NumberState::Float;
            }
            _ => {}
        }

        while !self.reader.is_eof() {
            let ch = self.reader.current_char();
            let continue_ = match state {
                NumberState::Int => match ch {
                    '0'..='9' => true,
                    '.' => {
                        state = NumberState::Float;
                        true
                    }
                    _ if matches!(self.reader.current_char(), 'e' | 'E') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::Float => match ch {
                    '0'..='9' => true,
                    _ if matches!(self.reader.current_char(), 'e' | 'E') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::Hex => match ch {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => true,
                    '.' => {
                        state = NumberState::HexFloat;
                        true
                    }
                    _ if matches!(self.reader.current_char(), 'P' | 'p') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::HexFloat => match ch {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => true,
                    _ if matches!(self.reader.current_char(), 'P' | 'p') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::WithExpo => ch.is_ascii_digit(),
                NumberState::Bin => matches!(ch, '0' | '1'),
            };

            if continue_ {
                self.reader.bump();
            } else {
                break;
            }
        }

        if self.lexer_config.support_complex_number() && self.reader.current_char() == 'i' {
            self.reader.bump();
            return PyTokenKind::TkComplex;
        }

        if self.lexer_config.support_ll_integer()
            && matches!(
                state,
                NumberState::Int | NumberState::Hex | NumberState::Bin
            )
        {
            self.reader
                .eat_while(|ch| matches!(ch, 'u' | 'U' | 'l' | 'L'));
            return PyTokenKind::TkInt;
        }

        if self.reader.current_char().is_alphabetic() {
            let ch = self.reader.current_char();
            self.error(|| t!("unexpected character '%{ch}' after number literal", ch = ch));
        }

        match state {
            NumberState::Int | NumberState::Hex => PyTokenKind::TkInt,
            _ => PyTokenKind::TkFloat,
        }
    }

    fn error<F, R>(&mut self, msg: F)
    where
        F: FnOnce() -> R,
        R: AsRef<str>,
    {
        if let Some(errors) = &mut self.errors {
            errors.push(LuaParseError::syntax_error_from(
                msg().as_ref(),
                self.reader.current_range(),
            ))
        }
    }
}
