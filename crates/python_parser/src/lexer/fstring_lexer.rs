use crate::{SourceRange, parser_error::PyParseError, text::Reader};

/// F-string lexer for parsing f-string content with embedded expressions
pub struct FStringLexer<'a> {
    reader: Reader<'a>,
    brace_depth: usize,
    errors: Vec<PyParseError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FStringToken {
    Text(SourceRange),           // Plain text part
    ExprStart(SourceRange),      // {
    Expr(SourceRange),           // Expression content (not parsed)
    ExprEnd(SourceRange),        // }
    FormatSpec(SourceRange),     // :format_spec
    ConversionSpec(SourceRange), // !r, !s, !a
}

impl<'a> FStringLexer<'a> {
    pub fn new(text: &'a str, text_range: Option<SourceRange>) -> Self {
        if let Some(range) = text_range {
            Self {
                reader: Reader::new_with_range(text, range),
                brace_depth: 0,
                errors: Vec::new(),
            }
        } else {
            Self {
                reader: Reader::new(text),
                brace_depth: 0,
                errors: Vec::new(),
            }
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
                    let range = self.reader.current_range();
                    Some(FStringToken::Text(range))
                } else {
                    // Start of expression
                    self.reader.bump();
                    let range = self.reader.current_range();
                    self.brace_depth += 1;
                    Some(FStringToken::ExprStart(range))
                }
            }
            '}' => {
                if self.brace_depth > 0 && self.reader.next_char() == '}' {
                    // Escaped brace }} -> }
                    self.reader.bump(); // consume first }
                    self.reader.bump(); // consume second }
                    let range = self.reader.current_range();
                    Some(FStringToken::Text(range))
                } else if self.brace_depth > 0 {
                    // End of expression
                    self.reader.bump();
                    let range = self.reader.current_range();
                    self.brace_depth -= 1;
                    Some(FStringToken::ExprEnd(range))
                } else {
                    // Literal } outside expression
                    self.reader.bump();
                    let range = self.reader.current_range();
                    Some(FStringToken::Text(range))
                }
            }
            '!' if self.brace_depth > 0 => {
                // Conversion specifier (!r, !s, !a)
                self.reader.bump(); // consume !
                if matches!(self.reader.current_char(), 'r' | 's' | 'a') {
                    self.reader.bump();
                    let range = self.reader.current_range();
                    Some(FStringToken::ConversionSpec(range))
                } else {
                    // Invalid conversion specifier
                    self.push_error("Invalid conversion specifier");
                    None
                }
            }
            ':' if self.brace_depth > 0 => {
                // Format specification
                self.reader.bump(); // consume :
                self.lex_format_spec();
                let range = self.reader.current_range();
                Some(FStringToken::FormatSpec(range))
            }
            // Inside expression, collect expression content
            _ if self.brace_depth > 0 => {
                self.lex_expr();
                let range = self.reader.current_range();
                if !range.is_empty() {
                    Some(FStringToken::Expr(range))
                } else {
                    None
                }
            }
            // Outside expression, lex text
            _ => {
                self.lex_text();
                let range = self.reader.current_range();
                if !range.is_empty() {
                    Some(FStringToken::Text(range))
                } else {
                    None
                }
            }
        }
    }

    fn lex_expr(&mut self) {
        // Collect expression content until we hit !, :, or }
        while !self.reader.is_eof() {
            let ch = self.reader.current_char();

            // Stop at conversion/format specs or end of expression
            if matches!(ch, '!' | ':' | '}') {
                break;
            }

            // Handle nested braces in expressions
            if ch == '{' {
                self.reader.bump();
                self.brace_depth += 1;
                // Recursively handle nested content
                while self.brace_depth > 1 && !self.reader.is_eof() {
                    if self.reader.current_char() == '{' {
                        self.reader.bump();
                        self.brace_depth += 1;
                    } else if self.reader.current_char() == '}' {
                        self.reader.bump();
                        self.brace_depth -= 1;
                    } else {
                        self.reader.bump();
                    }
                }
            } else {
                self.reader.bump();
            }
        }
    }

    fn lex_text(&mut self) {
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
                    self.reader.bump(); // consume escaped character
                }
            } else {
                self.reader.bump();
            }
        }
    }

    fn lex_format_spec(&mut self) {
        while !self.reader.is_eof() {
            let ch = self.reader.current_char();

            // Stop at closing brace
            if ch == '}' {
                break;
            }

            self.reader.bump();
        }
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
        let mut lexer = FStringLexer::new("hello {name}", None);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], FStringToken::Text(_)));
        assert!(matches!(tokens[1], FStringToken::ExprStart(_)));
        assert!(matches!(tokens[2], FStringToken::Expr(_)));
        assert!(matches!(tokens[3], FStringToken::ExprEnd(_)));
    }

    #[test]
    fn test_fstring_with_format() {
        let mut lexer = FStringLexer::new("value: {x:.2f}", None);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], FStringToken::Text(_)));
        assert!(matches!(tokens[1], FStringToken::ExprStart(_)));
        assert!(matches!(tokens[2], FStringToken::Expr(_)));
        assert!(matches!(tokens[3], FStringToken::FormatSpec(_)));
        assert!(matches!(tokens[4], FStringToken::ExprEnd(_)));
    }

    #[test]
    fn test_fstring_with_conversion() {
        let mut lexer = FStringLexer::new("repr: {obj!r}", None);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], FStringToken::Text(_)));
        assert!(matches!(tokens[1], FStringToken::ExprStart(_)));
        assert!(matches!(tokens[2], FStringToken::Expr(_)));
        assert!(matches!(tokens[3], FStringToken::ConversionSpec(_)));
        assert!(matches!(tokens[4], FStringToken::ExprEnd(_)));
    }

    #[test]
    fn test_escaped_braces() {
        let mut lexer = FStringLexer::new("{{escaped}} braces", None);
        let tokens = lexer.tokenize();

        // {{ -> Text
        // "escaped}}" -> Text
        // Text (}) + Text (})
        // " braces" -> Text
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], FStringToken::Text(_)));
        assert!(matches!(tokens[1], FStringToken::Text(_)));
        assert!(matches!(tokens[2], FStringToken::Text(_)));
        assert!(matches!(tokens[3], FStringToken::Text(_)));
        assert!(matches!(tokens[4], FStringToken::Text(_)));
    }

    #[test]
    fn test_fstring_with_expression() {
        let mut lexer = FStringLexer::new("result: {x + y * 2}", None);
        let tokens = lexer.tokenize();

        // Should tokenize: "result: " { "x + y * 2" }
        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], FStringToken::Text(_))); // "result: "
        assert!(matches!(tokens[1], FStringToken::ExprStart(_))); // {
        assert!(matches!(tokens[2], FStringToken::Expr(_))); // x + y * 2
        assert!(matches!(tokens[3], FStringToken::ExprEnd(_))); // }
    }

    #[test]
    fn test_fstring_with_nested_brackets() {
        let mut lexer = FStringLexer::new("data: {arr[0]}", None);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], FStringToken::Text(_)));
        assert!(matches!(tokens[1], FStringToken::ExprStart(_)));
        assert!(matches!(tokens[2], FStringToken::Expr(_))); // arr[0]
        assert!(matches!(tokens[3], FStringToken::ExprEnd(_)));
    }

    #[test]
    fn test_fstring_with_function_call() {
        let mut lexer = FStringLexer::new("func: {foo(a, b)}", None);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], FStringToken::Text(_)));
        assert!(matches!(tokens[1], FStringToken::ExprStart(_)));
        assert!(matches!(tokens[2], FStringToken::Expr(_))); // foo(a, b)
        assert!(matches!(tokens[3], FStringToken::ExprEnd(_)));
    }
}
