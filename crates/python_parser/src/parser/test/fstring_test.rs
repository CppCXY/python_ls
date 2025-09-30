#[cfg(test)]
mod fstring_expr_tests {
    use crate::parser::test::print_ast_code;


    #[test]
    fn test_basic_fstring_parsing() {
        let code = r#"
name = "world"
greeting = f"Hello {name}!"
"#;
        print_ast_code(code);
    }
}