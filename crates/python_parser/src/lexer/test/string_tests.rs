#[cfg(test)]
mod tests {
    use crate::{
        kind::PyTokenKind,
        lexer::{lexer_config::LexerConfig, py_lexer::PyLexer},
        text::Reader,
    };

    fn test_tokenize(input: &str) -> Vec<PyTokenKind> {
        let reader = Reader::new(input);
        let config = LexerConfig::default();
        let mut errors = Vec::new();
        let mut lexer = PyLexer::new(reader, config, Some(&mut errors));

        lexer
            .tokenize()
            .into_iter()
            .map(|token| token.kind)
            .collect()
    }

    #[test]
    fn test_regular_strings() {
        let tokens = test_tokenize(r#""hello""#);
        assert!(tokens.contains(&PyTokenKind::TkString));

        let tokens = test_tokenize(r#"'hello'"#);
        assert!(tokens.contains(&PyTokenKind::TkString));
    }

    #[test]
    fn test_raw_strings() {
        let tokens = test_tokenize(r#"r"hello""#);
        assert!(tokens.contains(&PyTokenKind::TkRawString));

        let tokens = test_tokenize(r#"R'hello'"#);
        assert!(tokens.contains(&PyTokenKind::TkRawString));
    }

    #[test]
    fn test_byte_strings() {
        let tokens = test_tokenize(r#"b"hello""#);
        assert!(tokens.contains(&PyTokenKind::TkBytesString));

        let tokens = test_tokenize(r#"B'hello'"#);
        assert!(tokens.contains(&PyTokenKind::TkBytesString));
    }

    #[test]
    fn test_f_strings() {
        let tokens = test_tokenize(r#"f"hello""#);
        assert!(tokens.contains(&PyTokenKind::TkFString));

        let tokens = test_tokenize(r#"F'hello'"#);
        assert!(tokens.contains(&PyTokenKind::TkFString));
    }

    #[test]
    fn test_unicode_strings() {
        let tokens = test_tokenize(r#"u"hello""#);
        assert!(tokens.contains(&PyTokenKind::TkString));

        let tokens = test_tokenize(r#"U'hello'"#);
        assert!(tokens.contains(&PyTokenKind::TkString));
    }

    #[test]
    fn test_combined_prefixes() {
        let tokens = test_tokenize(r#"rb"hello""#);
        assert!(tokens.contains(&PyTokenKind::TkRawBytesString));

        let tokens = test_tokenize(r#"br'hello'"#);
        assert!(tokens.contains(&PyTokenKind::TkRawBytesString));

        let tokens = test_tokenize(r#"rf"hello""#);
        assert!(tokens.contains(&PyTokenKind::TkFString));

        let tokens = test_tokenize(r#"fr'hello'"#);
        assert!(tokens.contains(&PyTokenKind::TkFString));
    }

    #[test]
    fn test_ellipsis() {
        let tokens = test_tokenize("...");
        assert!(tokens.contains(&PyTokenKind::TkEllipsis));

        // Test that single dots are still handled correctly
        let tokens = test_tokenize(".");
        assert!(tokens.contains(&PyTokenKind::TkDot));

        // Test that two dots are handled as separate tokens
        let tokens = test_tokenize("..");
        assert_eq!(
            tokens.iter().filter(|&&t| t == PyTokenKind::TkDot).count(),
            2
        );

        // Test ellipsis in context
        let tokens = test_tokenize("x[...]");
        assert!(tokens.contains(&PyTokenKind::TkEllipsis));
        assert!(tokens.contains(&PyTokenKind::TkName));
        assert!(tokens.contains(&PyTokenKind::TkLeftBracket));
        assert!(tokens.contains(&PyTokenKind::TkRightBracket));
    }

    #[test]
    fn test_triple_quoted_strings() {
        let tokens = test_tokenize(r#"r"""hello""""#);
        assert!(tokens.contains(&PyTokenKind::TkRawString));

        let tokens = test_tokenize(r#"b'''hello'''"#);
        assert!(tokens.contains(&PyTokenKind::TkBytesString));

        let tokens = test_tokenize(r#"f"""hello""""#);
        assert!(tokens.contains(&PyTokenKind::TkFString));
    }

    #[test]
    fn test_raw_bytes_strings() {
        let tokens = test_tokenize(r#"rb"raw bytes""#);
        assert!(tokens.contains(&PyTokenKind::TkRawBytesString));

        let tokens = test_tokenize(r#"BR'RAW BYTES'"#);
        assert!(tokens.contains(&PyTokenKind::TkRawBytesString));

        let tokens = test_tokenize(r#"br"""raw bytes triple""""#);
        assert!(tokens.contains(&PyTokenKind::TkRawBytesString));
    }
}
