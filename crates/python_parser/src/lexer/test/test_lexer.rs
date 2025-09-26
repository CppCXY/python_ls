#[cfg(test)]
mod tests {
    use crate::{LexerConfig, PyLexer, PyTokenKind, Reader};

    fn test_tokenize(input: &str) -> Vec<(PyTokenKind, String)> {
        let reader = Reader::new(input);
        let config = LexerConfig::default();
        let mut lexer = PyLexer::new(reader, config, None);
        let tokens = lexer.tokenize();

        tokens
            .into_iter()
            .filter(|token| token.kind != PyTokenKind::TkEof)
            .map(|token| {
                let start = token.range.start_offset;
                let end = start + token.range.length;
                let text = input[start..end].to_string();
                (token.kind, text)
            })
            .collect()
    }

    #[test]
    fn test_regular_string() {
        let tokens = test_tokenize(r#""hello world""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, PyTokenKind::TkString);
        assert_eq!(tokens[0].1, r#""hello world""#);
    }

    #[test]
    fn test_raw_string() {
        let tokens = test_tokenize(r#"r"hello\nworld""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, PyTokenKind::TkRawString);
        assert_eq!(tokens[0].1, r#"r"hello\nworld""#);
    }

    #[test]
    fn test_bytes_string() {
        let tokens = test_tokenize(r#"b"hello""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, PyTokenKind::TkBytesString);
        assert_eq!(tokens[0].1, r#"b"hello""#);
    }

    #[test]
    fn test_raw_bytes_string() {
        let tokens = test_tokenize(r#"rb"hello\x00""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, PyTokenKind::TkRawBytesString);
        assert_eq!(tokens[0].1, r#"rb"hello\x00""#);
    }

    #[test]
    fn test_f_string() {
        let tokens = test_tokenize(r#"f"hello {name}""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, PyTokenKind::TkFString);
        assert_eq!(tokens[0].1, r#"f"hello {name}""#);
    }

    #[test]
    fn test_ellipsis() {
        let tokens = test_tokenize("...");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, PyTokenKind::TkEllipsis);
        assert_eq!(tokens[0].1, "...");
    }

    #[test]
    fn test_ellipsis_with_spaces() {
        let tokens = test_tokenize("x = ...");
        // Should get: identifier, whitespace, equals, whitespace, ellipsis
        let non_whitespace: Vec<_> = tokens
            .into_iter()
            .filter(|(kind, _)| *kind != PyTokenKind::TkWhitespace)
            .collect();

        assert_eq!(non_whitespace.len(), 3);
        assert_eq!(non_whitespace[0].0, PyTokenKind::TkName);
        assert_eq!(non_whitespace[1].0, PyTokenKind::TkAssign);
        assert_eq!(non_whitespace[2].0, PyTokenKind::TkEllipsis);
    }

    #[test]
    fn test_three_dots_separate() {
        let tokens = test_tokenize(". . .");
        // Should get separate dots, not ellipsis
        let non_whitespace: Vec<_> = tokens
            .into_iter()
            .filter(|(kind, _)| *kind != PyTokenKind::TkWhitespace)
            .collect();

        assert_eq!(non_whitespace.len(), 3);
        for (kind, _) in non_whitespace {
            assert_eq!(kind, PyTokenKind::TkDot);
        }
    }

    #[test]
    fn test_mixed_string_types() {
        let input = r#"
"normal"
r"raw"
b"bytes"
f"format"
rb"raw_bytes"
"#;
        let tokens = test_tokenize(input);
        let string_tokens: Vec<_> = tokens
            .into_iter()
            .filter(|(kind, _)| {
                matches!(
                    kind,
                    PyTokenKind::TkString
                        | PyTokenKind::TkRawString
                        | PyTokenKind::TkBytesString
                        | PyTokenKind::TkFString
                        | PyTokenKind::TkRawBytesString
                )
            })
            .collect();

        assert_eq!(string_tokens.len(), 5);
        assert_eq!(string_tokens[0].0, PyTokenKind::TkString);
        assert_eq!(string_tokens[1].0, PyTokenKind::TkRawString);
        assert_eq!(string_tokens[2].0, PyTokenKind::TkBytesString);
        assert_eq!(string_tokens[3].0, PyTokenKind::TkFString);
        assert_eq!(string_tokens[4].0, PyTokenKind::TkRawBytesString);
    }
}
