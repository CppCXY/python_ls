#[cfg(test)]
mod fstring_expr_tests {
    use crate::parser::test::print_ast_code;

    #[test]
    fn test_basic_fstring_parsing() {
        let code = r#"
name.upper()
"#;
        print_ast_code(code);
    }

    #[test]
    fn test_fstring_with_expression() {
        let code = r#"
f"Hello {name.upper()}!"
"#;
        print_ast_code(code);
    }
}
