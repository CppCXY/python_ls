use crate::{kind::PyTokenKind, parser_error::PyParseError, text::Reader};

use super::{is_name_start, lex_name, lex_number, lex_operator};

/// F-string lexer for parsing f-string content with embedded expressions
#[allow(unused)]
pub struct FStringLexer<'a> {
    reader: Reader<'a>,
    quote_char: char,
    is_triple: bool,
    brace_depth: usize,
    errors: Vec<PyParseError>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FStringToken {
    Text(String),         // Plain text part
    ExprStart,            // {
    ExprEnd,              // }
    FormatSpec(String),   // :format_spec
    ConversionSpec(char), // !r, !s, !a
    Token(PyTokenKind),   // Python token inside expression
}

impl<'a> FStringLexer<'a> {
    pub fn new(text: &'a str, quote_char: char, is_triple: bool) -> Self {
        Self {
            reader: Reader::new(text),
            quote_char,
            is_triple,
            brace_depth: 0,
            errors: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<FStringToken> {
        let mut tokens = Vec::new();

        while !self.reader.is_eof() {
            if let Some(token) = self.lex_next() {
                tokens.push(token);
            }
        }

        tokens
    }

    pub fn get_errors(self) -> Vec<PyParseError> {
        self.errors
    }

    fn lex_next(&mut self) -> Option<FStringToken> {
        self.reader.reset_buff();

        if self.reader.is_eof() {
            return None;
        }

        match self.reader.current_char() {
            '{' => {
                if self.reader.next_char() == '{' {
                    // Escaped brace {{ -> {
                    self.reader.bump(); // consume first {
                    self.reader.bump(); // consume second {
                    Some(FStringToken::Text("{".to_string()))
                } else {
                    // Start of expression
                    self.reader.bump();
                    self.brace_depth += 1;
                    Some(FStringToken::ExprStart)
                }
            }
            '}' => {
                if self.brace_depth > 0 && self.reader.next_char() == '}' {
                    // Escaped brace }} -> }
                    self.reader.bump(); // consume first }
                    self.reader.bump(); // consume second }
                    Some(FStringToken::Text("}".to_string()))
                } else if self.brace_depth > 0 {
                    // End of expression
                    self.reader.bump();
                    self.brace_depth -= 1;
                    Some(FStringToken::ExprEnd)
                } else {
                    // Literal } outside expression
                    self.reader.bump();
                    Some(FStringToken::Text("}".to_string()))
                }
            }
            '!' if self.brace_depth > 0 => {
                // Conversion specifier (!r, !s, !a)
                self.reader.bump(); // consume !
                if matches!(self.reader.current_char(), 'r' | 's' | 'a') {
                    let conv = self.reader.current_char();
                    self.reader.bump();
                    Some(FStringToken::ConversionSpec(conv))
                } else {
                    // Invalid conversion specifier
                    self.push_error("Invalid conversion specifier");
                    None
                }
            }
            ':' if self.brace_depth > 0 => {
                // Format specification
                self.reader.bump(); // consume :
                let format_spec = self.lex_format_spec();
                Some(FStringToken::FormatSpec(format_spec))
            }
            // Inside expression, lex Python tokens
            _ if self.brace_depth > 0 => {
                // Skip whitespace
                if matches!(self.reader.current_char(), ' ' | '\t') {
                    self.reader.eat_while(|ch| ch == ' ' || ch == '\t');
                    return self.lex_next();
                }

                let token_kind = self.lex_expression_token();
                if token_kind != PyTokenKind::TkUnknown {
                    Some(FStringToken::Token(token_kind))
                } else {
                    None
                }
            }
            // Outside expression, lex text
            _ => {
                let text = self.lex_text();
                if !text.is_empty() {
                    Some(FStringToken::Text(text))
                } else {
                    None
                }
            }
        }
    }

    /// Lex a Python token inside the expression part of f-string
    fn lex_expression_token(&mut self) -> PyTokenKind {
        let ch = self.reader.current_char();

        match ch {
            // Numbers
            '0'..='9' => lex_number(&mut self.reader),
            // Names/identifiers
            _ if is_name_start(ch) => lex_name(&mut self.reader),
            // Operators and punctuation (but not { } which are handled separately)
            '(' | ')' | '[' | ']' | ',' | '.' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^'
            | '~' | '<' | '>' | '=' | '@' | ';' => lex_operator(&mut self.reader),
            // End of input or unknown
            _ if self.reader.is_eof() => PyTokenKind::TkEof,
            _ => {
                self.reader.bump();
                PyTokenKind::TkUnknown
            }
        }
    }

    fn lex_text(&mut self) -> String {
        let mut text = String::new();

        while !self.reader.is_eof() {
            let ch = self.reader.current_char();

            // Stop at special characters
            if matches!(ch, '{' | '}') {
                break;
            }

            // Stop at conversion/format specs inside braces
            if self.brace_depth > 0 && matches!(ch, '!' | ':') {
                break;
            }

            // Handle escape sequences
            if ch == '\\' && !self.reader.is_eof() {
                self.reader.bump(); // consume backslash
                if !self.reader.is_eof() {
                    let escaped = self.reader.current_char();
                    match escaped {
                        'n' => text.push('\n'),
                        't' => text.push('\t'),
                        'r' => text.push('\r'),
                        '\\' => text.push('\\'),
                        '\'' => text.push('\''),
                        '"' => text.push('"'),
                        _ => {
                            text.push('\\');
                            text.push(escaped);
                        }
                    }
                    self.reader.bump();
                }
            } else {
                text.push(ch);
                self.reader.bump();
            }
        }

        text
    }

    fn lex_format_spec(&mut self) -> String {
        let mut spec = String::new();

        while !self.reader.is_eof() {
            let ch = self.reader.current_char();

            // Stop at closing brace
            if ch == '}' {
                break;
            }

            spec.push(ch);
            self.reader.bump();
        }

        spec
    }

    fn push_error(&mut self, message: &str) {
        let error = PyParseError::syntax_error_from(message, self.reader.current_range());
        self.errors.push(error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_fstring() {
        let mut lexer = FStringLexer::new("hello {name}", '"', false);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], FStringToken::Text(ref s) if s == "hello "));
        assert!(matches!(tokens[1], FStringToken::ExprStart));
        assert!(matches!(tokens[2], FStringToken::Token(PyTokenKind::TkName)));
        assert!(matches!(tokens[3], FStringToken::ExprEnd));
    }

    #[test]
    fn test_fstring_with_format() {
        let mut lexer = FStringLexer::new("value: {x:.2f}", '"', false);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], FStringToken::Text(ref s) if s == "value: "));
        assert!(matches!(tokens[1], FStringToken::ExprStart));
        assert!(matches!(tokens[2], FStringToken::Token(PyTokenKind::TkName)));
        assert!(matches!(tokens[3], FStringToken::FormatSpec(ref s) if s == ".2f"));
        assert!(matches!(tokens[4], FStringToken::ExprEnd));
    }

    #[test]
    fn test_fstring_with_conversion() {
        let mut lexer = FStringLexer::new("repr: {obj!r}", '"', false);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], FStringToken::Text(ref s) if s == "repr: "));
        assert!(matches!(tokens[1], FStringToken::ExprStart));
        assert!(matches!(tokens[2], FStringToken::Token(PyTokenKind::TkName)));
        assert!(matches!(tokens[3], FStringToken::ConversionSpec('r')));
        assert!(matches!(tokens[4], FStringToken::ExprEnd));
    }

    #[test]
    fn test_escaped_braces() {
        let mut lexer = FStringLexer::new("{{escaped}} braces", '"', false);
        let tokens = lexer.tokenize();

        // {{ -> "{" (text)
        // "escaped}}" -> "escaped" (text) followed by "}}" which is parsed as "}}" (text)
        // " braces" -> " braces" (text)
        // So we expect: "{", "escaped", "}", "}", " braces"
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], FStringToken::Text(ref s) if s == "{"));
        assert!(matches!(tokens[1], FStringToken::Text(ref s) if s == "escaped"));
        assert!(matches!(tokens[2], FStringToken::Text(ref s) if s == "}"));
        assert!(matches!(tokens[3], FStringToken::Text(ref s) if s == "}"));
        assert!(matches!(tokens[4], FStringToken::Text(ref s) if s == " braces"));
    }

    #[test]
    fn test_fstring_with_expression() {
        let mut lexer = FStringLexer::new("result: {x + y * 2}", '"', false);
        let tokens = lexer.tokenize();

        // Should tokenize: "result: " { x + y * 2 }
        assert!(matches!(tokens[0], FStringToken::Text(ref s) if s == "result: "));
        assert!(matches!(tokens[1], FStringToken::ExprStart));
        assert!(matches!(tokens[2], FStringToken::Token(PyTokenKind::TkName))); // x
        assert!(matches!(tokens[3], FStringToken::Token(PyTokenKind::TkPlus))); // +
        assert!(matches!(tokens[4], FStringToken::Token(PyTokenKind::TkName))); // y
        assert!(matches!(tokens[5], FStringToken::Token(PyTokenKind::TkMul))); // *
        assert!(matches!(tokens[6], FStringToken::Token(PyTokenKind::TkInt))); // 2
        assert!(matches!(tokens[7], FStringToken::ExprEnd));
    }

    #[test]
    fn test_fstring_with_nested_brackets() {
        let mut lexer = FStringLexer::new("data: {arr[0]}", '"', false);
        let tokens = lexer.tokenize();

        assert!(matches!(tokens[0], FStringToken::Text(ref s) if s == "data: "));
        assert!(matches!(tokens[1], FStringToken::ExprStart));
        assert!(matches!(tokens[2], FStringToken::Token(PyTokenKind::TkName))); // arr
        assert!(matches!(tokens[3], FStringToken::Token(PyTokenKind::TkLeftBracket))); // [
        assert!(matches!(tokens[4], FStringToken::Token(PyTokenKind::TkInt))); // 0
        assert!(matches!(tokens[5], FStringToken::Token(PyTokenKind::TkRightBracket))); // ]
        assert!(matches!(tokens[6], FStringToken::ExprEnd));
    }

    #[test]
    fn test_fstring_with_function_call() {
        let mut lexer = FStringLexer::new("func: {foo(a, b)}", '"', false);
        let tokens = lexer.tokenize();

        assert!(matches!(tokens[0], FStringToken::Text(ref s) if s == "func: "));
        assert!(matches!(tokens[1], FStringToken::ExprStart));
        assert!(matches!(tokens[2], FStringToken::Token(PyTokenKind::TkName))); // foo
        assert!(matches!(tokens[3], FStringToken::Token(PyTokenKind::TkLeftParen))); // (
        assert!(matches!(tokens[4], FStringToken::Token(PyTokenKind::TkName))); // a
        assert!(matches!(tokens[5], FStringToken::Token(PyTokenKind::TkComma))); // ,
        assert!(matches!(tokens[6], FStringToken::Token(PyTokenKind::TkName))); // b
        assert!(matches!(tokens[7], FStringToken::Token(PyTokenKind::TkRightParen))); // )
        assert!(matches!(tokens[8], FStringToken::ExprEnd));
    }
}
