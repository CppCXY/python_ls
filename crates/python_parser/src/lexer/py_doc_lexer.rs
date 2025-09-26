// use crate::{
//     kind::PyTokenKind,
//     text::{Reader, SourceRange},
// };

// use super::{is_name_continue, is_name_start};

// #[derive(Debug, Clone)]
// pub struct LuaDocLexer<'a> {
//     origin_text: &'a str,
//     origin_token_kind: PyTokenKind,
//     pub state: LuaDocLexerState,
//     pub reader: Option<Reader<'a>>,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum LuaDocLexerState {
//     Init,
//     Tag,
//     Normal,
//     FieldStart,
//     Description,
//     LongDescription,
//     Trivia,
//     See,
//     Version,
//     Source,
//     NormalDescription,
//     CastExpr,
// }

// impl LuaDocLexer<'_> {
//     pub fn new(origin_text: &str) -> LuaDocLexer<'_> {
//         LuaDocLexer {
//             origin_text,
//             reader: None,
//             origin_token_kind: PyTokenKind::None,
//             state: LuaDocLexerState::Init,
//         }
//     }

//     pub fn is_invalid(&self) -> bool {
//         match self.reader {
//             Some(ref reader) => reader.is_eof(),
//             None => true,
//         }
//     }

//     pub fn reset(&mut self, kind: PyTokenKind, range: SourceRange) {
//         let text = &self.origin_text[range.start_offset..range.end_offset()];
//         self.reader = Some(Reader::new_with_range(text, range));
//         self.origin_token_kind = kind;
//     }

//     pub fn lex(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         reader.reset_buff();

//         if reader.is_eof() {
//             return PyTokenKind::TkEof;
//         }

//         match self.state {
//             LuaDocLexerState::Init => self.lex_init(),
//             LuaDocLexerState::Tag => self.lex_tag(),
//             LuaDocLexerState::Normal => self.lex_normal(),
//             LuaDocLexerState::FieldStart => self.lex_field_start(),
//             LuaDocLexerState::Description => self.lex_description(),
//             LuaDocLexerState::LongDescription => self.lex_long_description(),
//             LuaDocLexerState::Trivia => self.lex_trivia(),
//             LuaDocLexerState::See => self.lex_see(),
//             LuaDocLexerState::Version => self.lex_version(),
//             LuaDocLexerState::Source => self.lex_source(),
//             LuaDocLexerState::NormalDescription => self.lex_normal_description(),
//             LuaDocLexerState::CastExpr => self.lex_cast_expr(),
//         }
//     }

//     pub fn current_token_range(&self) -> SourceRange {
//         self.reader.as_ref().unwrap().current_range()
//     }

//     fn lex_init(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             '-' if reader.is_start_of_line() => {
//                 let count = reader.consume_char_n_times('-', 3);
//                 match count {
//                     2 => {
//                         if self.origin_token_kind == PyTokenKind::TkLongComment {
//                             reader.bump();
//                             reader.eat_when('=');
//                             reader.bump();

//                             match reader.current_char() {
//                                 '@' => {
//                                     reader.bump();
//                                     PyTokenKind::TkDocLongStart
//                                 }
//                                 _ => PyTokenKind::TkLongCommentStart,
//                             }
//                         } else {
//                             PyTokenKind::TkNormalStart
//                         }
//                     }
//                     3 => {
//                         reader.eat_while(is_doc_whitespace);
//                         match reader.current_char() {
//                             '@' => {
//                                 reader.bump();
//                                 PyTokenKind::TkDocStart
//                             }
//                             _ => PyTokenKind::TkNormalStart,
//                         }
//                     }
//                     _ => {
//                         reader.eat_while(|_| true);
//                         PyTokenKind::TKDocTriviaStart
//                     }
//                 }
//             }
//             '/' if reader.is_start_of_line() => {
//                 let count = reader.consume_char_n_times('/', 3);
//                 if count >= 2 {
//                     // "//" is a non-standard lua comment
//                     return PyTokenKind::TkNormalStart;
//                 }

//                 PyTokenKind::TKNonStdComment
//             }
//             _ => {
//                 reader.eat_while(|_| true);
//                 PyTokenKind::TkDocTrivia
//             }
//         }
//     }

//     fn lex_tag(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ch if is_doc_whitespace(ch) => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             ch if is_name_start(ch) => {
//                 reader.bump();
//                 reader.eat_while(is_name_continue);
//                 let text = reader.current_text();
//                 to_tag(text)
//             }
//             _ => {
//                 reader.eat_while(|_| true);
//                 PyTokenKind::TkDocTrivia
//             }
//         }
//     }

//     fn lex_normal(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ch if is_doc_whitespace(ch) => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             ':' => {
//                 reader.bump();
//                 PyTokenKind::TkColon
//             }
//             '.' => {
//                 reader.bump();
//                 if reader.current_char() == '.' && reader.next_char() == '.' {
//                     reader.bump();
//                     reader.bump();
//                     PyTokenKind::TkDots
//                 } else {
//                     PyTokenKind::TkDot
//                 }
//             }
//             ',' => {
//                 reader.bump();
//                 PyTokenKind::TkComma
//             }
//             ';' => {
//                 reader.bump();
//                 PyTokenKind::TkSemicolon
//             }
//             '(' => {
//                 reader.bump();
//                 PyTokenKind::TkLeftParen
//             }
//             ')' => {
//                 reader.bump();
//                 PyTokenKind::TkRightParen
//             }
//             '[' => {
//                 reader.bump();
//                 PyTokenKind::TkLeftBracket
//             }
//             ']' => {
//                 reader.bump();
//                 if self.origin_token_kind == PyTokenKind::TkLongComment {
//                     match reader.current_char() {
//                         '=' => {
//                             reader.eat_when('=');
//                             reader.bump();
//                             return PyTokenKind::TkLongCommentEnd;
//                         }
//                         ']' => {
//                             reader.bump();
//                             return PyTokenKind::TkLongCommentEnd;
//                         }
//                         _ => (),
//                     }
//                 }

//                 PyTokenKind::TkRightBracket
//             }
//             '{' => {
//                 reader.bump();
//                 PyTokenKind::TkLeftBrace
//             }
//             '}' => {
//                 reader.bump();
//                 PyTokenKind::TkRightBrace
//             }
//             '<' => {
//                 reader.bump();
//                 PyTokenKind::TkLt
//             }
//             '>' => {
//                 reader.bump();
//                 PyTokenKind::TkGt
//             }
//             '|' => {
//                 reader.bump();
//                 PyTokenKind::TkDocOr
//             }
//             '&' => {
//                 reader.bump();
//                 PyTokenKind::TkDocAnd
//             }
//             '?' => {
//                 reader.bump();
//                 PyTokenKind::TkDocQuestion
//             }
//             '+' => {
//                 reader.bump();
//                 PyTokenKind::TkPlus
//             }
//             '-' => {
//                 let count = reader.eat_when('-');
//                 match count {
//                     1 => PyTokenKind::TkMinus,
//                     3 => {
//                         reader.eat_while(is_doc_whitespace);
//                         match reader.current_char() {
//                             '@' => {
//                                 reader.bump();
//                                 PyTokenKind::TkDocStart
//                             }
//                             '|' => {
//                                 reader.bump();
//                                 // compact luals
//                                 if matches!(reader.current_char(), '+' | '>') {
//                                     reader.bump();
//                                 }
//                                 PyTokenKind::TkDocContinueOr
//                             }
//                             _ => PyTokenKind::TkDocContinue,
//                         }
//                     }
//                     _ => PyTokenKind::TkDocTrivia,
//                 }
//             }
//             '#' | '@' => {
//                 reader.eat_while(|_| true);
//                 PyTokenKind::TkDocDetail
//             }
//             ch if ch.is_ascii_digit() => {
//                 reader.eat_while(|ch| ch.is_ascii_digit());
//                 PyTokenKind::TkInt
//             }
//             ch if ch == '"' || ch == '\'' => {
//                 reader.bump();
//                 reader.eat_while(|c| c != ch);
//                 if reader.current_char() == ch {
//                     reader.bump();
//                 }

//                 PyTokenKind::TkString
//             }
//             ch if is_name_start(ch) || ch == '`' => {
//                 let (text, str_tpl) = read_doc_name(reader);
//                 if str_tpl {
//                     return PyTokenKind::TkStringTemplateType;
//                 }
//                 to_token_or_name(text)
//             }
//             _ => {
//                 reader.eat_while(|_| true);
//                 PyTokenKind::TkDocTrivia
//             }
//         }
//     }

//     fn lex_field_start(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ch if is_name_start(ch) => {
//                 let (text, _) = read_doc_name(reader);
//                 to_modification_or_name(text)
//             }
//             _ => self.lex_normal(),
//         }
//     }

//     fn lex_description(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ch if is_doc_whitespace(ch) => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             '-' if reader.is_start_of_line() => {
//                 let count = reader.consume_char_n_times('-', 3);
//                 match count {
//                     2 => {
//                         if self.origin_token_kind == PyTokenKind::TkLongComment {
//                             reader.bump();
//                             reader.eat_when('=');
//                             reader.bump();

//                             match reader.current_char() {
//                                 '@' => {
//                                     reader.bump();
//                                     PyTokenKind::TkDocLongStart
//                                 }
//                                 _ => PyTokenKind::TkLongCommentStart,
//                             }
//                         } else {
//                             PyTokenKind::TkNormalStart
//                         }
//                     }
//                     3 => {
//                         reader.eat_while(is_doc_whitespace);
//                         match reader.current_char() {
//                             '@' => {
//                                 reader.bump();
//                                 PyTokenKind::TkDocStart
//                             }
//                             '|' => {
//                                 reader.bump();
//                                 // compact luals
//                                 if matches!(reader.current_char(), '+' | '>') {
//                                     reader.bump();
//                                 }

//                                 PyTokenKind::TkDocContinueOr
//                             }
//                             _ => PyTokenKind::TkNormalStart,
//                         }
//                     }
//                     _ => {
//                         reader.eat_while(|_| true);
//                         PyTokenKind::TKDocTriviaStart
//                     }
//                 }
//             }
//             _ => {
//                 reader.eat_while(|_| true);
//                 PyTokenKind::TkDocDetail
//             }
//         }
//     }

//     fn lex_long_description(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         let text = reader.get_source_text();
//         let mut chars = text.chars().rev().peekable();
//         let mut trivia_count = 0;
//         while let Some(&ch) = chars.peek() {
//             if ch != ']' && ch != '=' {
//                 break;
//             }
//             chars.next();
//             trivia_count += 1;
//         }
//         let end_pos = text.len() - trivia_count;

//         if reader.get_current_end_pos() < end_pos {
//             while reader.get_current_end_pos() < end_pos {
//                 reader.bump();
//             }
//             PyTokenKind::TkDocDetail
//         } else {
//             reader.eat_while(|_| true);
//             PyTokenKind::TkDocTrivia
//         }
//     }

//     fn lex_trivia(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         reader.eat_while(|_| true);
//         PyTokenKind::TkDocTrivia
//     }

//     fn lex_see(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ' ' | '\t' => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             _ => {
//                 reader.eat_while(|_| true);
//                 PyTokenKind::TkDocSeeContent
//             }
//         }
//     }

//     fn lex_version(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ',' => {
//                 reader.bump();
//                 PyTokenKind::TkComma
//             }
//             '>' => {
//                 reader.bump();
//                 if reader.current_char() == '=' {
//                     reader.bump();
//                     PyTokenKind::TkGe
//                 } else {
//                     PyTokenKind::TkGt
//                 }
//             }
//             '<' => {
//                 reader.bump();
//                 if reader.current_char() == '=' {
//                     reader.bump();
//                     PyTokenKind::TkLe
//                 } else {
//                     PyTokenKind::TkLt
//                 }
//             }
//             ch if is_doc_whitespace(ch) => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             ch if ch.is_ascii_digit() => {
//                 reader.eat_while(|ch| ch.is_ascii_digit() || ch == '.');
//                 PyTokenKind::TkDocVersionNumber
//             }
//             ch if is_name_start(ch) => {
//                 let (text, _) = read_doc_name(reader);
//                 match text {
//                     "JIT" => PyTokenKind::TkDocVersionNumber,
//                     _ => PyTokenKind::TkName,
//                 }
//             }
//             _ => self.lex_normal(),
//         }
//     }

//     fn lex_source(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ch if is_doc_whitespace(ch) => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             ch if is_name_start(ch) => {
//                 reader.bump();
//                 reader.eat_while(is_source_continue);
//                 PyTokenKind::TKDocPath
//             }
//             ch if ch == '"' || ch == '\'' => {
//                 reader.bump();
//                 reader.eat_while(|c| c != '\'' && c != '"');
//                 if reader.current_char() == '\'' || reader.current_char() == '"' {
//                     reader.bump();
//                 }

//                 PyTokenKind::TKDocPath
//             }
//             _ => self.lex_normal(),
//         }
//     }

//     fn lex_normal_description(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ch if is_doc_whitespace(ch) => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             ch if ch.is_ascii_alphabetic() || ch == '#' => {
//                 if reader.current_char() == '#' {
//                     reader.bump();
//                 }

//                 reader.eat_while(|c| c.is_ascii_alphabetic());
//                 let text = reader.current_text();
//                 match text {
//                     "region" | "#region" => PyTokenKind::TkDocRegion,
//                     "endregion" | "#endregion" => PyTokenKind::TkDocEndRegion,
//                     _ => {
//                         reader.eat_while(|_| true);
//                         PyTokenKind::TkDocDetail
//                     }
//                 }
//             }
//             '-' if reader.is_start_of_line() => {
//                 let count = reader.consume_char_n_times('-', 3);
//                 match count {
//                     2 => {
//                         if self.origin_token_kind == PyTokenKind::TkLongComment {
//                             reader.bump();
//                             reader.eat_when('=');
//                             reader.bump();

//                             match reader.current_char() {
//                                 '@' => {
//                                     reader.bump();
//                                     PyTokenKind::TkDocLongStart
//                                 }
//                                 _ => PyTokenKind::TkLongCommentStart,
//                             }
//                         } else {
//                             PyTokenKind::TkNormalStart
//                         }
//                     }
//                     3 => {
//                         reader.eat_while(is_doc_whitespace);
//                         match reader.current_char() {
//                             '@' => {
//                                 reader.bump();
//                                 PyTokenKind::TkDocStart
//                             }
//                             _ => PyTokenKind::TkNormalStart,
//                         }
//                     }
//                     _ => {
//                         reader.eat_while(|_| true);
//                         PyTokenKind::TKDocTriviaStart
//                     }
//                 }
//             }
//             '/' if reader.is_start_of_line() => {
//                 let count = reader.consume_char_n_times('/', 3);
//                 if count >= 2 {
//                     // "//" is a non-standard lua comment
//                     return PyTokenKind::TkNormalStart;
//                 }

//                 PyTokenKind::TKNonStdComment
//             }
//             _ => {
//                 reader.eat_while(|_| true);
//                 PyTokenKind::TkDocDetail
//             }
//         }
//     }

//     fn lex_cast_expr(&mut self) -> PyTokenKind {
//         let reader = self.reader.as_mut().unwrap();
//         match reader.current_char() {
//             ch if is_doc_whitespace(ch) => {
//                 reader.eat_while(is_doc_whitespace);
//                 PyTokenKind::TkWhitespace
//             }
//             '.' => {
//                 reader.bump();
//                 PyTokenKind::TkDot
//             }
//             ch if is_name_start(ch) => {
//                 reader.bump();
//                 reader.eat_while(is_name_continue);
//                 PyTokenKind::TkName
//             }
//             _ => self.lex_normal(),
//         }
//     }
// }

// fn to_tag(text: &str) -> PyTokenKind {
//     match text {
//         "class" => PyTokenKind::TkTagClass,
//         "enum" => PyTokenKind::TkTagEnum,
//         "interface" => PyTokenKind::TkTagInterface,
//         "alias" => PyTokenKind::TkTagAlias,
//         "module" => PyTokenKind::TkTagModule,
//         "field" => PyTokenKind::TkTagField,
//         "type" => PyTokenKind::TkTagType,
//         "param" => PyTokenKind::TkTagParam,
//         "return" => PyTokenKind::TkTagReturn,
//         "return_cast" => PyTokenKind::TkTagReturnCast,
//         "generic" => PyTokenKind::TkTagGeneric,
//         "see" => PyTokenKind::TkTagSee,
//         "overload" => PyTokenKind::TkTagOverload,
//         "async" => PyTokenKind::TkTagAsync,
//         "cast" => PyTokenKind::TkTagCast,
//         "deprecated" => PyTokenKind::TkTagDeprecated,
//         "private" | "protected" | "public" | "package" | "internal" => {
//             PyTokenKind::TkTagVisibility
//         }
//         "readonly" => PyTokenKind::TkTagReadonly,
//         "diagnostic" => PyTokenKind::TkTagDiagnostic,
//         "meta" => PyTokenKind::TkTagMeta,
//         "version" => PyTokenKind::TkTagVersion,
//         "as" => PyTokenKind::TkTagAs,
//         "nodiscard" => PyTokenKind::TkTagNodiscard,
//         "operator" => PyTokenKind::TkTagOperator,
//         "mapping" => PyTokenKind::TkTagMapping,
//         "namespace" => PyTokenKind::TkTagNamespace,
//         "using" => PyTokenKind::TkTagUsing,
//         "source" => PyTokenKind::TkTagSource,
//         "export" => PyTokenKind::TkTagExport,
//         "language" => PyTokenKind::TkLanguage,
//         _ => PyTokenKind::TkTagOther,
//     }
// }

// fn to_modification_or_name(text: &str) -> PyTokenKind {
//     match text {
//         "private" | "protected" | "public" | "package" => PyTokenKind::TkDocVisibility,
//         "readonly" => PyTokenKind::TkDocReadonly,
//         _ => PyTokenKind::TkName,
//     }
// }

// fn to_token_or_name(text: &str) -> PyTokenKind {
//     match text {
//         "true" => PyTokenKind::TkTrue,
//         "false" => PyTokenKind::TkFalse,
//         "keyof" => PyTokenKind::TkDocKeyOf,
//         "extends" => PyTokenKind::TkDocExtends,
//         "as" => PyTokenKind::TkDocAs,
//         "and" => PyTokenKind::TkAnd,
//         "or" => PyTokenKind::TkOr,
//         _ => PyTokenKind::TkName,
//     }
// }

// fn is_doc_whitespace(ch: char) -> bool {
//     ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
// }

// fn read_doc_name<'a>(reader: &'a mut Reader) -> (&'a str, bool /* str tpl */) {
//     reader.bump();
//     let mut str_tpl = false;
//     while !reader.is_eof() {
//         match reader.current_char() {
//             ch if is_name_continue(ch) => {
//                 reader.bump();
//             }
//             // donot continue if next char is '.' or '-' or '*' or '`'
//             '.' | '-' | '*' => {
//                 let next = reader.next_char();
//                 if next == '.' || next == '-' || next == '*' {
//                     break;
//                 }

//                 reader.bump();
//             }
//             '`' => {
//                 str_tpl = true;
//                 reader.bump();
//             }
//             _ => break,
//         }
//     }

//     (reader.current_text(), str_tpl)
// }

// fn is_source_continue(ch: char) -> bool {
//     is_name_continue(ch)
//         || ch == '.'
//         || ch == '-'
//         || ch == '/'
//         || ch == ' '
//         || ch == ':'
//         || ch == '#'
//         || ch == '\\'
// }

// #[cfg(test)]
// mod tests {
//     use crate::kind::PyTokenKind;
//     use crate::lexer::LuaDocLexer;
//     use crate::text::SourceRange;

//     #[test]
//     fn test_lex() {
//         let text = r#"-- comment"#;
//         let mut lexer = LuaDocLexer::new(text);
//         lexer.reset(PyTokenKind::TkShortComment, SourceRange::new(0, 10));
//         let k1 = lexer.lex();
//         assert_eq!(k1, PyTokenKind::TkNormalStart);
//         let k2 = lexer.lex();
//         let range = lexer.current_token_range();
//         let text = lexer.origin_text[range.start_offset..range.end_offset()].to_string();
//         assert_eq!(text, " comment");
//         assert_eq!(k2, PyTokenKind::TkDocTrivia);
//     }
// }
