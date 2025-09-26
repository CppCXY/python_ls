use crate::{
    PyKind,
    kind::PyTokenKind,
    parser_error::{PyParseError, PyParseErrorKind},
    syntax::PySyntaxToken,
};

pub fn string_token_value(token: &PySyntaxToken) -> Result<String, PyParseError> {
    match token.kind() {
        PyKind::Token(PyTokenKind::TkString) => normal_string_value(token),
        PyKind::Token(PyTokenKind::TkRawString) => raw_string_value(token),
        PyKind::Token(PyTokenKind::TkBytesString) => bytes_string_value(token),
        PyKind::Token(PyTokenKind::TkRawBytesString) => raw_bytes_string_value(token),
        PyKind::Token(PyTokenKind::TkFString) => f_string_value(token),
        _ => unreachable!(),
    }
}

fn raw_string_value(token: &PySyntaxToken) -> Result<String, PyParseError> {
    let text = token.text();

    // Raw strings: r"..." or R"..." or r'...' or R'...'
    // Find the quote character
    let quote_start = text.find(|c| c == '"' || c == '\'').unwrap_or(1);
    let quote_char = text.chars().nth(quote_start).unwrap();

    // Check for triple quotes
    let is_triple = text.len() > quote_start + 2
        && text.chars().nth(quote_start + 1) == Some(quote_char)
        && text.chars().nth(quote_start + 2) == Some(quote_char);

    if is_triple {
        let start = quote_start + 3;
        let end = text.len() - 3;
        Ok(text[start..end].to_string())
    } else {
        let start = quote_start + 1;
        let end = text.len() - 1;
        Ok(text[start..end].to_string())
    }
}

fn bytes_string_value(token: &PySyntaxToken) -> Result<String, PyParseError> {
    let text = token.text();
    
    // Skip the 'b' or 'B' prefix
    let quote_start = text.find(|c| c == '"' || c == '\'').unwrap_or(1);
    let quote_char = text.chars().nth(quote_start).unwrap();
    
    // Check for triple quotes
    let is_triple = text.len() > quote_start + 2
        && text.chars().nth(quote_start + 1) == Some(quote_char)
        && text.chars().nth(quote_start + 2) == Some(quote_char);
    
    if is_triple {
        let start = quote_start + 3;
        let end = text.len() - 3;
        Ok(text[start..end].to_string())
    } else {
        let start = quote_start + 1;
        let end = text.len() - 1;
        Ok(text[start..end].to_string())
    }
}

fn raw_bytes_string_value(token: &PySyntaxToken) -> Result<String, PyParseError> {
    let text = token.text();
    
    // Skip the 'rb' or 'br' or 'RB' or 'BR' prefix
    let quote_start = text.find(|c| c == '"' || c == '\'').unwrap_or(2);
    let quote_char = text.chars().nth(quote_start).unwrap();
    
    // Check for triple quotes
    let is_triple = text.len() > quote_start + 2
        && text.chars().nth(quote_start + 1) == Some(quote_char)
        && text.chars().nth(quote_start + 2) == Some(quote_char);
    
    if is_triple {
        let start = quote_start + 3;
        let end = text.len() - 3;
        Ok(text[start..end].to_string())
    } else {
        let start = quote_start + 1;
        let end = text.len() - 1;
        Ok(text[start..end].to_string())
    }
}

fn f_string_value(token: &PySyntaxToken) -> Result<String, PyParseError> {
    // F-strings need special handling for expressions inside {}
    // For now, return the raw content without processing expressions
    let text = token.text();
    let quote_start = text.find(|c| c == '"' || c == '\'').unwrap_or(1);
    let quote_char = text.chars().nth(quote_start).unwrap();

    let is_triple = text.len() > quote_start + 2
        && text.chars().nth(quote_start + 1) == Some(quote_char)
        && text.chars().nth(quote_start + 2) == Some(quote_char);

    if is_triple {
        let start = quote_start + 3;
        let end = text.len() - 3;
        Ok(text[start..end].to_string())
    } else {
        let start = quote_start + 1;
        let end = text.len() - 1;
        Ok(text[start..end].to_string())
    }
}

fn normal_string_value(token: &PySyntaxToken) -> Result<String, PyParseError> {
    let text = token.text();
    if text.len() < 2 {
        return Ok(String::new());
    }

    // Determine if this is a triple-quoted string
    let is_triple_single = text.starts_with("'''") && text.ends_with("'''");
    let is_triple_double = text.starts_with("\"\"\"") && text.ends_with("\"\"\"");
    let is_triple = is_triple_single || is_triple_double;

    let (start_offset, end_offset, _) = if is_triple {
        (3, 3, if is_triple_single { '\'' } else { '"' })
    } else {
        let delimiter = text.chars().next().unwrap();
        (1, 1, delimiter)
    };

    let content = &text[start_offset..text.len() - end_offset];
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next_char) = chars.next() {
                    match next_char {
                        'a' => result.push('\u{0007}'), // Bell
                        'b' => result.push('\u{0008}'), // Backspace
                        'f' => result.push('\u{000C}'), // Formfeed
                        'n' => result.push('\n'),       // Newline
                        'r' => result.push('\r'),       // Carriage return
                        't' => result.push('\t'),       // Horizontal tab
                        'v' => result.push('\u{000B}'), // Vertical tab
                        '\\' => result.push('\\'),      // Backslash
                        '\'' => result.push('\''),      // Single quote
                        '\"' => result.push('\"'),      // Double quote
                        '0' => result.push('\0'),       // Null character
                        'x' => {
                            // Hexadecimal escape sequence: \xhh
                            let hex = chars.by_ref().take(2).collect::<String>();
                            if hex.len() == 2 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                                if let Ok(value) = u8::from_str_radix(&hex, 16) {
                                    result.push(value as char);
                                } else {
                                    return Err(PyParseError::new(
                                        PyParseErrorKind::SyntaxError,
                                        &format!("Invalid hex escape sequence '\\x{}'", hex),
                                        token.text_range(),
                                    ));
                                }
                            } else {
                                return Err(PyParseError::new(
                                    PyParseErrorKind::SyntaxError,
                                    &format!("Invalid hex escape sequence '\\x{}'", hex),
                                    token.text_range(),
                                ));
                            }
                        }
                        'u' => {
                            // Unicode escape sequence: \uxxxx
                            let unicode_hex = chars.by_ref().take(4).collect::<String>();
                            if unicode_hex.len() == 4
                                && unicode_hex.chars().all(|c| c.is_ascii_hexdigit())
                            {
                                if let Ok(code_point) = u32::from_str_radix(&unicode_hex, 16) {
                                    if let Some(unicode_char) = std::char::from_u32(code_point) {
                                        result.push(unicode_char);
                                    } else {
                                        return Err(PyParseError::new(
                                            PyParseErrorKind::SyntaxError,
                                            &format!(
                                                "Invalid unicode escape sequence '\\u{}'",
                                                unicode_hex
                                            ),
                                            token.text_range(),
                                        ));
                                    }
                                } else {
                                    return Err(PyParseError::new(
                                        PyParseErrorKind::SyntaxError,
                                        &format!(
                                            "Invalid unicode escape sequence '\\u{}'",
                                            unicode_hex
                                        ),
                                        token.text_range(),
                                    ));
                                }
                            } else {
                                return Err(PyParseError::new(
                                    PyParseErrorKind::SyntaxError,
                                    &format!(
                                        "Invalid unicode escape sequence '\\u{}'",
                                        unicode_hex
                                    ),
                                    token.text_range(),
                                ));
                            }
                        }
                        'U' => {
                            // 32-bit Unicode escape sequence: \Uxxxxxxxx
                            let unicode_hex = chars.by_ref().take(8).collect::<String>();
                            if unicode_hex.len() == 8
                                && unicode_hex.chars().all(|c| c.is_ascii_hexdigit())
                            {
                                if let Ok(code_point) = u32::from_str_radix(&unicode_hex, 16) {
                                    if let Some(unicode_char) = std::char::from_u32(code_point) {
                                        result.push(unicode_char);
                                    } else {
                                        return Err(PyParseError::new(
                                            PyParseErrorKind::SyntaxError,
                                            &format!(
                                                "Invalid unicode escape sequence '\\U{}'",
                                                unicode_hex
                                            ),
                                            token.text_range(),
                                        ));
                                    }
                                } else {
                                    return Err(PyParseError::new(
                                        PyParseErrorKind::SyntaxError,
                                        &format!(
                                            "Invalid unicode escape sequence '\\U{}'",
                                            unicode_hex
                                        ),
                                        token.text_range(),
                                    ));
                                }
                            } else {
                                return Err(PyParseError::new(
                                    PyParseErrorKind::SyntaxError,
                                    &format!(
                                        "Invalid unicode escape sequence '\\U{}'",
                                        unicode_hex
                                    ),
                                    token.text_range(),
                                ));
                            }
                        }
                        'N' => {
                            // Named unicode escape: \N{name}
                            if let Some('{') = chars.next() {
                                let name =
                                    chars.by_ref().take_while(|c| *c != '}').collect::<String>();
                                // For simplicity, we'll just return the name as-is
                                // In a real implementation, you'd look up the Unicode name
                                result.push_str(&format!("\\N{{{}}}", name));
                            } else {
                                return Err(PyParseError::new(
                                    PyParseErrorKind::SyntaxError,
                                    "Invalid named unicode escape sequence",
                                    token.text_range(),
                                ));
                            }
                        }
                        '0'..='7' => {
                            // Octal escape sequence: \ooo
                            let mut octal = String::new();
                            octal.push(next_char);

                            // Read up to 2 more octal digits
                            for _ in 0..2 {
                                if let Some(&digit) = chars.peek() {
                                    if digit >= '0' && digit <= '7' {
                                        octal.push(digit);
                                        chars.next();
                                    } else {
                                        break;
                                    }
                                }
                            }

                            if let Ok(value) = u8::from_str_radix(&octal, 8) {
                                result.push(value as char);
                            } else {
                                return Err(PyParseError::new(
                                    PyParseErrorKind::SyntaxError,
                                    &format!("Invalid octal escape sequence '\\{}'", octal),
                                    token.text_range(),
                                ));
                            }
                        }
                        '\r' | '\n' => {
                            // Line continuation - skip the newline
                            if next_char == '\r' && chars.peek() == Some(&'\n') {
                                chars.next(); // Skip the \n after \r
                            }
                        }
                        _ => {
                            // Unknown escape sequence - in Python, this usually just includes both characters
                            result.push('\\');
                            result.push(next_char);
                        }
                    }
                } else {
                    // Backslash at end of string
                    result.push('\\');
                }
            }
            _ => {
                result.push(c);
            }
        }
    }

    Ok(result)
}
