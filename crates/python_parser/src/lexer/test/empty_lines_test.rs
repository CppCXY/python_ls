#[cfg(test)]
mod lexer_empty_lines_test {
    use crate::{LexerConfig, PyLexer, PyTokenKind, Reader};

    fn tokenize_code(code: &str) -> Vec<(PyTokenKind, String)> {
        let reader = Reader::new(code);
        let config = LexerConfig::default();
        let mut lexer = PyLexer::new(reader, config, None);

        lexer
            .tokenize()
            .into_iter()
            .map(|token| {
                let start = token.range.start_offset;
                let end = token.range.end_offset();
                let text = code[start..end].to_string();
                (token.kind, text)
            })
            .collect()
    }

    #[test]
    fn test_function_with_empty_lines() {
        let code = r#"def test():

    pass"#;

        let tokens = tokenize_code(code);

        println!("Tokens for function with empty lines:");
        for (i, (kind, text)) in tokens.iter().enumerate() {
            println!("  {}: {:?} = {:?}", i, kind, text);
        }

        // Find the pattern around colon
        let colon_idx = tokens
            .iter()
            .position(|(kind, _)| *kind == PyTokenKind::TkColon)
            .unwrap();
        println!("\nTokens around colon (index {}):", colon_idx);
        for i in (colon_idx.saturating_sub(2))..=(colon_idx + 5).min(tokens.len() - 1) {
            println!("  {}: {:?} = {:?}", i, tokens[i].0, tokens[i].1);
        }
    }

    #[test]
    fn test_function_without_empty_lines() {
        let code = r#"def test():
    pass"#;

        let tokens = tokenize_code(code);

        println!("Tokens for function without empty lines:");
        for (i, (kind, text)) in tokens.iter().enumerate() {
            println!("  {}: {:?} = {:?}", i, kind, text);
        }

        // Find the pattern around colon
        let colon_idx = tokens
            .iter()
            .position(|(kind, _)| *kind == PyTokenKind::TkColon)
            .unwrap();
        println!("\nTokens around colon (index {}):", colon_idx);
        for i in (colon_idx.saturating_sub(2))..=(colon_idx + 5).min(tokens.len() - 1) {
            println!("  {}: {:?} = {:?}", i, tokens[i].0, tokens[i].1);
        }
    }
}
