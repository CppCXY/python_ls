use super::{
    // py_doc_parser::LuaDocParser,
    marker::{MarkEvent, MarkerEventContainer},
    parser_config::ParserConfig,
};
use crate::text::Reader;
use crate::{
    // LuaSyntaxTree, LuaTreeBuilder,
    grammar::parse_module,
    kind::PyTokenKind,
    lexer::{PyLexer, PyTokenData},
    parser_error::LuaParseError,
    text::SourceRange,
};

#[allow(unused)]
pub struct PyParser<'a> {
    text: &'a str,
    events: Vec<MarkEvent>,
    tokens: Vec<PyTokenData>,
    token_index: usize,
    current_token: PyTokenKind,
    mark_level: usize,
    pub parse_config: ParserConfig<'a>,
    pub(crate) errors: &'a mut Vec<LuaParseError>,
    // 括号嵌套级别跟踪
    paren_level: usize,    // ()
    bracket_level: usize,  // []
    brace_level: usize,    // {}
}

impl MarkerEventContainer for PyParser<'_> {
    fn get_mark_level(&self) -> usize {
        self.mark_level
    }

    fn incr_mark_level(&mut self) {
        self.mark_level += 1;
    }

    fn decr_mark_level(&mut self) {
        self.mark_level -= 1;
    }

    fn get_events(&mut self) -> &mut Vec<MarkEvent> {
        &mut self.events
    }
}

impl<'a> PyParser<'a> {
    #[allow(unused)]
    pub fn parse(text: &'a str, config: ParserConfig) -> Option<()> {
        let mut errors: Vec<LuaParseError> = Vec::new();
        let tokens = {
            let mut lexer =
                PyLexer::new(Reader::new(text), config.lexer_config(), Some(&mut errors));
            lexer.tokenize()
        };

        let mut parser = PyParser {
            text,
            events: Vec::new(),
            tokens,
            token_index: 0,
            current_token: PyTokenKind::None,
            parse_config: config,
            mark_level: 0,
            errors: &mut errors,
            paren_level: 0,
            bracket_level: 0,
            brace_level: 0,
        };

        parse_module(&mut parser);
        let errors = parser.get_errors();
        // let root = {
        //     let mut builder = LuaTreeBuilder::new(
        //         parser.origin_text(),
        //         parser.events,
        //         parser.parse_config.node_cache(),
        //     );
        //     builder.build();
        //     builder.finish()
        // };
        // LuaSyntaxTree::new(root, errors)
        todo!()
    }

    pub fn init(&mut self) {
        if self.tokens.is_empty() {
            self.current_token = PyTokenKind::TkEof;
        } else {
            self.current_token = self.tokens[0].kind;
        }

        if is_trivia_kind(self.current_token) {
            self.bump();
        }
    }

    pub fn origin_text(&self) -> &'a str {
        self.text
    }

    pub fn current_token(&self) -> PyTokenKind {
        self.current_token
    }

    pub fn current_token_index(&self) -> usize {
        self.token_index
    }

    pub fn current_token_range(&self) -> SourceRange {
        if self.token_index >= self.tokens.len() {
            if self.tokens.is_empty() {
                return SourceRange::EMPTY;
            } else {
                return self.tokens[self.tokens.len() - 1].range;
            }
        }

        self.tokens[self.token_index].range
    }

    pub fn previous_token_range(&self) -> SourceRange {
        if self.token_index == 0 || self.tokens.is_empty() {
            return SourceRange::EMPTY;
        }

        // Find the previous non-trivia token
        let mut prev_index = self.token_index - 1;
        while prev_index > 0 && is_trivia_kind(self.tokens[prev_index].kind) {
            prev_index -= 1;
        }

        // If we found a non-trivia token or reached the first token
        if prev_index < self.tokens.len() && !is_trivia_kind(self.tokens[prev_index].kind) {
            self.tokens[prev_index].range
        } else if prev_index == 0 {
            // If the first token is also trivia, return its range anyway
            self.tokens[0].range
        } else {
            SourceRange::EMPTY
        }
    }

    pub fn current_token_text(&self) -> &str {
        let range = &self.tokens[self.token_index].range;
        &self.text[range.start_offset..range.end_offset()]
    }

    pub fn set_current_token_kind(&mut self, kind: PyTokenKind) {
        if self.token_index < self.tokens.len() {
            self.tokens[self.token_index].kind = kind;
            self.current_token = kind;
        }
    }

    pub fn bump(&mut self) {
        if !is_invalid_kind(self.current_token) && self.token_index < self.tokens.len() {
            let token = &self.tokens[self.token_index];
            self.events.push(MarkEvent::EatToken {
                kind: token.kind,
                range: token.range,
            });
        }

        let start_trivia = self.token_index + 1;
        let mut next_index = start_trivia;
        self.skip_trivia(&mut next_index);
        self.parse_trivia_tokens(start_trivia, next_index);
        self.token_index = next_index;

        if self.token_index >= self.tokens.len() {
            self.current_token = PyTokenKind::TkEof;
            return;
        }

        self.current_token = self.tokens[self.token_index].kind;
    }

    pub fn peek_next_token(&self) -> PyTokenKind {
        let mut next_index = self.token_index + 1;
        self.skip_trivia(&mut next_index);

        if next_index >= self.tokens.len() {
            PyTokenKind::None
        } else {
            self.tokens[next_index].kind
        }
    }

    fn skip_trivia(&self, index: &mut usize) {
        if index >= &mut self.tokens.len() {
            return;
        }

        let mut kind = self.tokens[*index].kind;
        while is_trivia_kind(kind) {
            *index += 1;
            if *index >= self.tokens.len() {
                break;
            }
            kind = self.tokens[*index].kind;
        }
    }

    // Parse trivia tokens (comments, whitespace, shebang) 
    // Note: TkNewline is no longer trivia in Python - it has syntactic meaning
    fn parse_trivia_tokens(&mut self, start: usize, end: usize) {
        // Simply consume trivia tokens and add them to events
        for i in start..end {
            let token = &self.tokens[i];
            match token.kind {
                PyTokenKind::TkComment => {
                    // For Python, comments are simple - just consume them
                    self.events.push(MarkEvent::EatToken {
                        kind: token.kind,
                        range: token.range,
                    });
                }
                PyTokenKind::TkWhitespace | PyTokenKind::TkShebang => {
                    // Simple trivia - just consume
                    self.events.push(MarkEvent::EatToken {
                        kind: token.kind,
                        range: token.range,
                    });
                }
                _ => {
                    // Non-trivia token should not be here, but handle gracefully
                    // This should not happen if is_trivia_kind is correct
                    break;
                }
            }
        }
    }

    pub fn push_error(&mut self, err: LuaParseError) {
        self.errors.push(err);
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_errors(&self) -> Vec<LuaParseError> {
        self.errors.clone()
    }

    /// Check if we're inside parentheses, brackets, or braces where newlines can be ignored
    pub fn in_parentheses_context(&self) -> bool {
        self.paren_level > 0 || self.bracket_level > 0 || self.brace_level > 0
    }

    /// Skip whitespace and optionally newlines (when inside parentheses)
    pub fn skip_whitespace_and_optional_newlines(&mut self) {
        while matches!(self.current_token(), PyTokenKind::TkWhitespace) 
            || (self.in_parentheses_context() && matches!(self.current_token(), PyTokenKind::TkNewline)) 
        {
            self.bump();
        }
    }

    /// 期待特定token，如果是括号类token则使用smart_bump
    pub fn expect_token(&mut self, expected: PyTokenKind) -> bool {
        if self.current_token() == expected {
            match expected {
                PyTokenKind::TkLeftParen | PyTokenKind::TkLeftBracket | PyTokenKind::TkLeftBrace |
                PyTokenKind::TkRightParen | PyTokenKind::TkRightBracket | PyTokenKind::TkRightBrace => {
                    self.smart_bump();
                }
                _ => {
                    self.bump();
                }
            }
            true
        } else {
            false
        }
    }

    /// Check if current position is at statement boundary (newline or dedent)
    pub fn at_statement_boundary(&self) -> bool {
        matches!(
            self.current_token(),
            PyTokenKind::TkNewline | PyTokenKind::TkDedent | PyTokenKind::TkEof
        )
    }

    /// 进入括号上下文（增加嵌套级别）
    pub fn enter_paren_context(&mut self, token: PyTokenKind) {
        match token {
            PyTokenKind::TkLeftParen => self.paren_level += 1,
            PyTokenKind::TkLeftBracket => self.bracket_level += 1,
            PyTokenKind::TkLeftBrace => self.brace_level += 1,
            _ => {}
        }
    }

    /// 退出括号上下文（减少嵌套级别）
    pub fn exit_paren_context(&mut self, token: PyTokenKind) {
        match token {
            PyTokenKind::TkRightParen if self.paren_level > 0 => self.paren_level -= 1,
            PyTokenKind::TkRightBracket if self.bracket_level > 0 => self.bracket_level -= 1,
            PyTokenKind::TkRightBrace if self.brace_level > 0 => self.brace_level -= 1,
            _ => {}
        }
    }

    /// 智能bump：自动处理括号跟踪和上下文相关的换行符跳过
    pub fn smart_bump(&mut self) {
        let current = self.current_token();
        
        // 跟踪括号嵌套
        match current {
            PyTokenKind::TkLeftParen | PyTokenKind::TkLeftBracket | PyTokenKind::TkLeftBrace => {
                self.enter_paren_context(current);
            }
            PyTokenKind::TkRightParen | PyTokenKind::TkRightBracket | PyTokenKind::TkRightBrace => {
                self.exit_paren_context(current);
            }
            _ => {}
        }
        
        self.bump();
        
        // 在括号内自动跳过换行符
        if self.in_parentheses_context() && self.current_token() == PyTokenKind::TkNewline {
            self.bump();
        }
    }
}

fn is_trivia_kind(kind: PyTokenKind) -> bool {
    matches!(
        kind,
        PyTokenKind::TkComment
            | PyTokenKind::TkWhitespace
            | PyTokenKind::TkShebang
    )
}

fn is_invalid_kind(kind: PyTokenKind) -> bool {
    matches!(
        kind,
        PyTokenKind::None
            | PyTokenKind::TkEof
            | PyTokenKind::TkWhitespace
            | PyTokenKind::TkShebang
            | PyTokenKind::TkComment
    )
}

// #[cfg(test)]
// mod tests {
//     use crate::text::Reader;
//     use crate::{
//         PyParser, kind::PyTokenKind, lexer::PyLexer, parser::ParserConfig,
//         parser_error::LuaParseError,
//     };

//     #[allow(unused)]
//     fn new_parser<'a>(
//         text: &'a str,
//         config: ParserConfig<'a>,
//         errors: &'a mut Vec<LuaParseError>,
//         show_tokens: bool,
//     ) -> PyParser<'a> {
//         let tokens = {
//             let mut lexer = PyLexer::new(Reader::new(text), config.lexer_config(), Some(errors));
//             lexer.tokenize()
//         };

//         if show_tokens {
//             println!("tokens: ");
//             for t in &tokens {
//                 println!("{:?}", t);
//             }
//         }

//         let mut parser = PyParser {
//             text,
//             events: Vec::new(),
//             tokens,
//             token_index: 0,
//             current_token: PyTokenKind::None,
//             parse_config: config,
//             mark_level: 0,
//             errors,
//         };
//         parser.init();

//         parser
//     }

//     #[test]
//     fn test_parse_and_ast() {
//         let lua_code = r#"
//             function foo(a, b)
//                 return a + b
//             end
//         "#;

//         let tree = PyParser::parse(lua_code, ParserConfig::default());
//         println!("{:#?}", tree.get_red_root());
//     }

//     #[test]
//     fn test_parse_and_ast_with_error() {
//         let lua_code = r#"
//             function foo(a, b)
//                 return a + b
//         "#;

//         let tree = PyParser::parse(lua_code, ParserConfig::default());
//         println!("{:#?}", tree.get_red_root());
//     }

//     #[test]
//     fn test_parse_comment() {
//         let lua_code = r#"
//             -- comment
//             local t
//             -- inline comment
//         "#;

//         let tree = PyParser::parse(lua_code, ParserConfig::default());
//         println!("{:#?}", tree.get_red_root());
//     }

//     #[test]
//     fn test_parse_empty_file() {
//         let lua_code = r#""#;

//         let tree = PyParser::parse(lua_code, ParserConfig::default());
//         println!("{:#?}", tree.get_red_root());
//     }
// }
