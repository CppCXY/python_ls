use crate::{
    parser_error::{PyParseError, PyParseErrorKind},
    syntax::PySyntaxToken,
};

pub fn float_token_value(token: &PySyntaxToken) -> Result<f64, PyParseError> {
    let text = token.text();

    // Python float formats: 3.14, .5, 5., 1e10, 2.5e-3, 0x1.Ap3 (hex float)
    let hex = text.starts_with("0x") || text.starts_with("0X");

    let value = if hex {
        // Hexadecimal floating-point numbers (Python 3.0+)
        let hex_float_text = &text[2..];
        let exponent_position = hex_float_text
            .find('p')
            .or_else(|| hex_float_text.find('P'));
        let (float_part, exponent_part) = if let Some(pos) = exponent_position {
            (&hex_float_text[..pos], &hex_float_text[(pos + 1)..])
        } else {
            (hex_float_text, "")
        };

        let (integer_part, fraction_value) = if let Some(dot_pos) = float_part.find('.') {
            let (int_part, frac_part) = float_part.split_at(dot_pos);
            let int_value = if !int_part.is_empty() {
                i64::from_str_radix(int_part, 16).unwrap_or(0)
            } else {
                0
            };
            let frac_part = &frac_part[1..];
            let frac_value = if !frac_part.is_empty() {
                // Parse hex fraction more accurately
                let mut frac_val = 0.0;
                for (i, c) in frac_part.chars().enumerate() {
                    if let Some(digit_val) = c.to_digit(16) {
                        frac_val += digit_val as f64 * 16f64.powi(-((i + 1) as i32));
                    }
                }
                frac_val
            } else {
                0.0
            };
            (int_value, frac_value)
        } else {
            (i64::from_str_radix(float_part, 16).unwrap_or(0), 0.0)
        };

        let mut value = integer_part as f64 + fraction_value;
        if !exponent_part.is_empty()
            && let Ok(exp) = exponent_part.parse::<i32>()
        {
            value *= 2f64.powi(exp);
        }
        value
    } else {
        // Standard decimal floating-point
        let (float_part, exponent_part) =
            if let Some(pos) = text.find('e').or_else(|| text.find('E')) {
                (&text[..pos], &text[(pos + 1)..])
            } else {
                (text, "")
            };

        let mut value = float_part.parse::<f64>().map_err(|e| {
            PyParseError::new(
                PyParseErrorKind::SyntaxError,
                &format!("The float literal '{}' is invalid: {}", text, e),
                token.text_range(),
            )
        })?;

        if !exponent_part.is_empty()
            && let Ok(exp) = exponent_part.parse::<i32>()
        {
            value *= 10f64.powi(exp);
        }
        value
    };

    Ok(value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntegerRepr {
    Normal,
    Hex,
    Bin,
    Oct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerOrLarge {
    Int(i64),
    Large,
}

impl IntegerOrLarge {
    pub fn is_signed(&self) -> bool {
        matches!(self, IntegerOrLarge::Int(_))
    }

    pub fn is_large(&self) -> bool {
        matches!(self, IntegerOrLarge::Large)
    }
}

pub fn int_token_value(token: &PySyntaxToken) -> Result<IntegerOrLarge, PyParseError> {
    let text = token.text();

    // Determine the representation
    let repr = if text.starts_with("0x") || text.starts_with("0X") {
        IntegerRepr::Hex
    } else if text.starts_with("0b") || text.starts_with("0B") {
        IntegerRepr::Bin
    } else if text.starts_with("0o") || text.starts_with("0O") {
        IntegerRepr::Oct
    } else if text.len() > 1
        && text.starts_with('0')
        && text.chars().nth(1).unwrap().is_ascii_digit()
    {
        // Legacy octal (Python 2 style: 0123)
        IntegerRepr::Oct
    } else {
        IntegerRepr::Normal
    };

    // Python integers don't have unsigned suffixes like C/C++
    let text = text;

    // Try to parse as signed integer first
    let signed_value = match repr {
        IntegerRepr::Hex => {
            let text = &text[2..];
            i64::from_str_radix(text, 16)
        }
        IntegerRepr::Bin => {
            let text = &text[2..];
            i64::from_str_radix(text, 2)
        }
        IntegerRepr::Oct => {
            let text = if text.starts_with("0o") || text.starts_with("0O") {
                &text[2..]
            } else {
                &text[1..] // Skip the leading '0' for legacy octal
            };
            i64::from_str_radix(text, 8)
        }
        IntegerRepr::Normal => text.parse::<i64>(),
    };

    match signed_value {
        Ok(value) => Ok(IntegerOrLarge::Int(value)),
        Err(e) => {
            let range = token.text_range();

            // For Python, integers have arbitrary precision in Python 3
            // But for our parser, we'll handle overflow by trying u64
            if *e.kind() == std::num::IntErrorKind::PosOverflow {
                Ok(IntegerOrLarge::Large)
            } else if matches!(
                *e.kind(),
                std::num::IntErrorKind::NegOverflow | std::num::IntErrorKind::PosOverflow
            ) {
                Err(PyParseError::new(
                    PyParseErrorKind::SyntaxError,
                    &format!(
                        "The integer literal '{}' is too large to be represented",
                        token.text()
                    ),
                    range,
                ))
            } else {
                Err(PyParseError::new(
                    PyParseErrorKind::SyntaxError,
                    &format!("The integer literal '{}' is invalid: {}", token.text(), e),
                    range,
                ))
            }
        }
    }
}
