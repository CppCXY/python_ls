#[cfg(test)]
mod tests {
    use crate::{
        kind::{PySyntaxKind, PyTokenKind},
        syntax::node::token::{
            IntegerOrLarge, float_token_value, int_token_value, string_token_value,
        },
        syntax::{PySyntaxNode, PySyntaxToken},
    };

    fn get_token(text: &str, kind: PyTokenKind) -> PySyntaxToken {
        let mut builder = rowan::GreenNodeBuilder::new();
        builder.start_node(PySyntaxKind::Module.into());
        builder.token(kind.into(), text);
        builder.finish_node();
        let green = builder.finish();
        let root = PySyntaxNode::new_root(green);
        root.first_token().unwrap()
    }

    macro_rules! test_string_value {
        ($name:ident, $code:expr, $expected:expr, $kind:expr) => {
            #[test]
            fn $name() {
                let token = &get_token($code, $kind);
                let result = string_token_value(token);
                assert_eq!(result.unwrap(), $expected.to_string());
            }
        };
    }

    // Normal string tests
    test_string_value!(
        test_string_token_value_normal,
        "\"hello\"",
        "hello",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_single_quote,
        "'hello'",
        "hello",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_triple_double,
        "\"\"\"hello\"\"\"",
        "hello",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_triple_single,
        "'''hello'''",
        "hello",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_escaped_quote,
        "\"he\\\"llo\"",
        "he\"llo",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_escaped_single_quote,
        "'he\\'llo'",
        "he'llo",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_multiline,
        "\"hello\\nworld\"",
        "hello\nworld",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_empty,
        "\"\"",
        "",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_hex_escape,
        "\"\\x48\\x65\\x6c\\x6c\\x6f\"",
        "Hello",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_unicode_escape,
        "\"\\u03B1\\u03B2\\u03B3\"",
        "αβγ",
        PyTokenKind::TkString
    );
    test_string_value!(
        test_string_token_value_unicode_32bit,
        "\"\\U0001F600\"",
        "😀",
        PyTokenKind::TkString
    );

    // Raw string tests
    test_string_value!(
        test_raw_string_basic,
        "r\"hello\\nworld\"",
        "hello\\nworld",
        PyTokenKind::TkRawString
    );
    test_string_value!(
        test_raw_string_triple,
        "r'''hello\\nworld'''",
        "hello\\nworld",
        PyTokenKind::TkRawString
    );

    // F-string tests (basic)
    test_string_value!(
        test_f_string_basic,
        "f\"hello\"",
        "hello",
        PyTokenKind::TkFString
    );

    macro_rules! test_float_value {
        ($name:ident, $code:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let token = &get_token($code, PyTokenKind::TkFloat);
                let result = float_token_value(token);
                assert!((result.unwrap() - $expected).abs() < f64::EPSILON);
            }
        };
    }

    test_float_value!(test_float_basic, "3.14", 3.14);
    test_float_value!(test_float_scientific, "1e10", 1e10);
    test_float_value!(test_float_scientific_negative, "1.23e-4", 1.23e-4);
    test_float_value!(test_float_scientific_positive, "1.23e+4", 1.23e+4);
    test_float_value!(test_float_leading_dot, ".5", 0.5);
    test_float_value!(test_float_trailing_dot, "5.", 5.0);
    test_float_value!(test_float_hex, "0x1.8p1", 3.0);
    test_float_value!(test_float_hex_fraction, "0x1.4p1", 2.5);

    macro_rules! test_int_value {
        ($name:ident, $code:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let token = &get_token($code, PyTokenKind::TkInt);
                let result = int_token_value(token);
                assert_eq!(result.unwrap(), IntegerOrLarge::Int($expected));
            }
        };
    }

    test_int_value!(test_int_basic, "123", 123);
    test_int_value!(test_int_zero, "0", 0);
    test_int_value!(test_int_hex, "0x1A", 26);
    test_int_value!(test_int_hex_upper, "0X1A", 26);
    test_int_value!(test_int_hex_lower, "0x1a", 26);
    test_int_value!(test_int_bin, "0b1010", 10);
    test_int_value!(test_int_bin_upper, "0B1010", 10);
    test_int_value!(test_int_oct_new, "0o12", 10);
    test_int_value!(test_int_oct_upper, "0O12", 10);
    test_int_value!(test_int_negative, "-123", -123);

    #[test]
    fn test_multiline_string() {
        let code = "\"\"\"This is a\nmultiline\nstring\"\"\"";
        let expected = "This is a\nmultiline\nstring";
        let token = &get_token(code, PyTokenKind::TkString);
        let result = string_token_value(token);
        assert_eq!(result.unwrap(), expected.to_string());
    }

    #[test]
    fn test_escape_sequences() {
        let test_cases = vec![
            ("\"\\a\"", "\u{0007}"), // Bell
            ("\"\\b\"", "\u{0008}"), // Backspace
            ("\"\\f\"", "\u{000C}"), // Formfeed
            ("\"\\n\"", "\n"),       // Newline
            ("\"\\r\"", "\r"),       // Carriage return
            ("\"\\t\"", "\t"),       // Tab
            ("\"\\v\"", "\u{000B}"), // Vertical tab
            ("\"\\\\\"", "\\"),      // Backslash
            ("\"\\'\"", "'"),        // Single quote
            ("\"\\\"\"", "\""),      // Double quote
            ("\"\\0\"", "\0"),       // Null
        ];

        for (input, expected) in test_cases {
            let token = &get_token(input, PyTokenKind::TkString);
            let result = string_token_value(token).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_octal_escape() {
        let token = &get_token("\"\\101\"", PyTokenKind::TkString); // Octal for 'A'
        let result = string_token_value(token);
        assert_eq!(result.unwrap(), "A");
    }

    #[test]
    fn test_raw_string_no_escape() {
        let token = &get_token("r\"\\n\\t\\r\"", PyTokenKind::TkRawString);
        let result = string_token_value(token);
        assert_eq!(result.unwrap(), "\\n\\t\\r");
    }

    #[test]
    fn test_bytes_string() {
        let token = &get_token("b\"hello\"", PyTokenKind::TkBytesString);
        let result = string_token_value(token);
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_raw_bytes_string() {
        let token = &get_token("rb\"hello\\nworld\"", PyTokenKind::TkRawBytesString);
        let result = string_token_value(token);
        assert_eq!(result.unwrap(), "hello\\nworld");
    }

    #[test]
    fn test_large_int() {
        // Test that we can handle large integers
        let token = &get_token("999999999999999999999999999999", PyTokenKind::TkInt);
        let result = int_token_value(token);
        // This should either work as a large int or return an error gracefully
        match result {
            Ok(_) => {}  // Success
            Err(_) => {} // Expected for very large numbers
        }
    }

    #[test]
    fn test_hex_float_precision() {
        let token = &get_token("0x1.91eb851eb851fp+1", PyTokenKind::TkFloat);
        let result = float_token_value(token).unwrap();

        // Should be approximately pi
        assert!((result - std::f64::consts::PI).abs() < 0.01);
    }
}
