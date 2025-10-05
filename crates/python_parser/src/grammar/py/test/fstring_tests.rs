#[cfg(test)]
mod fstring_expr_tests {
    use crate::parser::{ParserConfig, PyParser};

    fn parse_python_code(code: &str) -> bool {
        let tree = PyParser::parse(code, ParserConfig::default(), None);

        !tree.has_syntax_errors()
    }

    #[test]
    fn test_basic_fstring_parsing() {
        let code = r#"name = "world"
greeting = f"Hello {name}!"
"#;
        assert!(
            parse_python_code(code),
            "Basic f-string should parse successfully"
        );
    }

    #[test]
    fn test_fstring_with_format_spec() {
        let code = r#"value = 42
formatted = f"Value: {value:04d}"
"#;
        assert!(
            parse_python_code(code),
            "F-string with format spec should parse successfully"
        );
    }

    #[test]
    fn test_fstring_with_conversion() {
        let code = r#"obj = "test"
debug = f"Debug: {obj!r}"
"#;
        assert!(
            parse_python_code(code),
            "F-string with conversion should parse successfully"
        );
    }

    #[test]
    fn test_fstring_with_expression() {
        let code = r#"result = f"Result: {1 + 2 * 3}"
"#;
        assert!(
            parse_python_code(code),
            "F-string with expression should parse successfully"
        );
    }

    #[test]
    fn test_escaped_braces_in_fstring() {
        let code = r#"text = f"{{literal braces}} and {42}"
"#;
        assert!(
            parse_python_code(code),
            "F-string with escaped braces should parse successfully"
        );
    }
}
