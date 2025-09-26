use crate::{
    grammar::{ParseFailReason, ParseResult, py::is_statement_start_token},
    kind::{BinaryOperator, PyOpKind, PySyntaxKind, PyTokenKind, UNARY_PRIORITY, UnaryOperator},
    parser::{MarkerEventContainer, PyParser},
    parser_error::PyParseError,
};

use super::if_token_bump;

pub fn parse_expr(p: &mut PyParser) -> ParseResult {
    parse_sub_expr(p, 0)
}

fn parse_sub_expr(p: &mut PyParser, limit: i32) -> ParseResult {
    let uop = PyOpKind::to_unary_operator(p.current_token());
    let mut cm = if uop != UnaryOperator::OpNop {
        let m = p.mark(PySyntaxKind::UnaryExpr);
        let op_range = p.current_token_range();
        let op_token = p.current_token();
        p.bump();
        match parse_sub_expr(p, UNARY_PRIORITY) {
            Ok(_) => {}
            Err(err) => {
                p.push_error(PyParseError::syntax_error_from(
                    &t!(
                        "unary operator '%{op}' is not followed by an expression",
                        op = op_token
                    ),
                    op_range,
                ));
                return Err(err);
            }
        }
        m.complete(p)
    } else {
        parse_simple_expr(p)?
    };

    let mut bop = PyOpKind::to_binary_operator(p.current_token());
    while bop != BinaryOperator::OpNop && bop.get_priority().left > limit {
        let op_range = p.current_token_range();
        let op_token = p.current_token();
        let m = cm.precede(p, PySyntaxKind::BinaryExpr);
        p.bump();
        match parse_sub_expr(p, bop.get_priority().right) {
            Ok(_) => {}
            Err(err) => {
                p.push_error(PyParseError::syntax_error_from(
                    &t!(
                        "binary operator '%{op}' is not followed by an expression",
                        op = op_token
                    ),
                    op_range,
                ));
                return Err(err);
            }
        }

        cm = m.complete(p);
        bop = PyOpKind::to_binary_operator(p.current_token());
    }

    Ok(cm)
}

fn parse_simple_expr(p: &mut PyParser) -> ParseResult {
    match p.current_token() {
        PyTokenKind::TkInt
        | PyTokenKind::TkFloat
        | PyTokenKind::TkComplex
        | PyTokenKind::TkNone
        | PyTokenKind::TkTrue
        | PyTokenKind::TkFalse
        | PyTokenKind::TkString
        | PyTokenKind::TkBytesString
        | PyTokenKind::TkRawBytesString
        | PyTokenKind::TkFString
        | PyTokenKind::TkRawString => {
            let m = p.mark(PySyntaxKind::LiteralExpr);
            p.bump();
            Ok(m.complete(p))
        }
        PyTokenKind::TkLeftBracket => parse_list_expr(p),
        PyTokenKind::TkLeftBrace => parse_dict_expr(p),
        PyTokenKind::TkLambda => parse_lambda_expr(p),
        PyTokenKind::TkName | PyTokenKind::TkLeftParen => parse_suffixed_expr(p),
        _ => {
            // Provide more specific error information
            let error_msg = match p.current_token() {
                PyTokenKind::TkEof => t!("unexpected end of file, expected expression"),
                PyTokenKind::TkRightParen => t!("unexpected ')', expected expression"),
                PyTokenKind::TkRightBrace => t!("unexpected '}', expected expression"),
                PyTokenKind::TkRightBracket => t!("unexpected ']', expected expression"),
                PyTokenKind::TkComma => t!("unexpected ',', expected expression"),
                PyTokenKind::TkColon => t!("unexpected ':', expected expression"),
                PyTokenKind::TkElse => t!("unexpected 'else', expected expression"),
                PyTokenKind::TkElif => t!("unexpected 'elif', expected expression"),
                PyTokenKind::TkExcept => t!("unexpected 'except', expected expression"),
                PyTokenKind::TkFinally => t!("unexpected 'finally', expected expression"),
                PyTokenKind::TkDedent => t!("unexpected dedent, expected expression"),
                _ => t!(
                    "unexpected token '%{token}', expected expression",
                    token = p.current_token()
                ),
            };

            p.push_error(PyParseError::syntax_error_from(
                &error_msg,
                p.current_token_range(),
            ));
            Err(ParseFailReason::UnexpectedToken)
        }
    }
}

pub fn parse_lambda_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::LambdaExpr);

    if_token_bump(p, PyTokenKind::TkLambda);

    // Parse parameters (optional)
    if p.current_token() == PyTokenKind::TkName {
        parse_lambda_params(p)?;
    }

    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected ':' after lambda parameters"),
            p.current_token_range(),
        ));
    }

    // Parse lambda body expression
    if parse_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected expression after ':' in lambda"),
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_list_expr(p: &mut PyParser) -> ParseResult {
    parse_list_or_comprehension(p)
}

fn parse_list_or_comprehension(p: &mut PyParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::ListExpr);
    p.smart_bump(); // consume '[' and track bracket context

    // Empty list
    if p.current_token() == PyTokenKind::TkRightBracket {
        p.smart_bump(); // consume ']'
        return Ok(m.complete(p));
    }

    // Parse first expression
    if parse_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected expression in list"),
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Check if this is a comprehension (look for 'for' keyword)
    if p.current_token() == PyTokenKind::TkFor {
        // This is a list comprehension
        m.set_kind(p, PySyntaxKind::ListCompExpr);
        
        // Parse 'for' clause
        p.bump(); // consume 'for'
        
        // Parse target variable(s) - use simple name parsing to avoid 'in' conflict
        if p.current_token() == PyTokenKind::TkName {
            p.bump(); // consume target name
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected target variable after 'for'"),
                p.current_token_range(),
            ));
        }
        
        // Parse 'in' keyword
        if p.current_token() == PyTokenKind::TkIn {
            p.bump(); // consume 'in'
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected 'in' after for target"),
                p.current_token_range(),
            ));
        }
        
        // Parse iterator expression
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected iterator expression after 'in'"),
                p.current_token_range(),
            ));
        }
        
        // Optional 'if' condition
        if p.current_token() == PyTokenKind::TkIf {
            p.bump(); // consume 'if'
            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected condition after 'if'"),
                    p.current_token_range(),
                ));
            }
        }
        
    } else {
        // Regular list - parse remaining elements
        while p.current_token() == PyTokenKind::TkComma {
            p.bump(); // consume comma
            
            if p.current_token() == PyTokenKind::TkRightBracket {
                // Trailing comma is allowed
                break;
            }
            
            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected expression in list"),
                    p.current_token_range(),
                ));
                break;
            }
        }
    }

    // Expect closing bracket
    if p.current_token() == PyTokenKind::TkRightBracket {
        p.smart_bump(); // consume ']'
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected ']' to close list"),
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_dict_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::DictExpr);
    p.bump(); // consume '{'

    if p.current_token() != PyTokenKind::TkRightBrace {
        loop {
            // Parse key
            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected key expression in dictionary"),
                    p.current_token_range(),
                ));
                break;
            }

            if p.current_token() == PyTokenKind::TkColon {
                p.bump();
            } else {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected ':' after dictionary key"),
                    p.current_token_range(),
                ));
                break;
            }

            // Parse value
            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected value expression in dictionary"),
                    p.current_token_range(),
                ));
                break;
            }

            if p.current_token() == PyTokenKind::TkComma {
                p.bump();
                if p.current_token() == PyTokenKind::TkRightBrace {
                    // trailing comma
                    break;
                }
            } else {
                break;
            }
        }
    }

    if p.current_token() == PyTokenKind::TkRightBrace {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected '}' to close dictionary"),
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_lambda_params(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::Parameters);

    loop {
        if p.current_token() == PyTokenKind::TkName {
            let param_m = p.mark(PySyntaxKind::Parameter);
            p.bump(); // parameter name
            param_m.complete(p);
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected parameter name"),
                p.current_token_range(),
            ));
            break;
        }

        if p.current_token() == PyTokenKind::TkComma {
            p.bump();
            if p.current_token() == PyTokenKind::TkColon {
                // trailing comma before colon
                break;
            }
        } else {
            break;
        }
    }

    Ok(m.complete(p))
}

fn parse_suffixed_expr(p: &mut PyParser) -> ParseResult {
    let mut cm = match p.current_token() {
        PyTokenKind::TkName => parse_name_expr(p)?,
        PyTokenKind::TkLeftParen => {
            parse_parenthesized_expr_or_tuple(p)?
        }
        _ => {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expect primary expression (identifier or parenthesized expression)"),
                p.current_token_range(),
            ));
            return Err(ParseFailReason::UnexpectedToken);
        }
    };

    loop {
        match p.current_token() {
            PyTokenKind::TkDot => {
                let m = cm.precede(p, PySyntaxKind::AttributeExpr);
                p.bump(); // consume '.'
                if p.current_token() == PyTokenKind::TkName {
                    p.bump(); // consume attribute name
                } else {
                    p.push_error(PyParseError::syntax_error_from(
                        &t!("expected attribute name after '.'"),
                        p.current_token_range(),
                    ));
                }
                cm = m.complete(p);
            }
            PyTokenKind::TkLeftBracket => {
                let m = cm.precede(p, PySyntaxKind::SubscriptExpr);
                p.bump(); // consume '['
                if parse_expr(p).is_err() {
                    p.push_error(PyParseError::syntax_error_from(
                        &t!("expected index expression"),
                        p.current_token_range(),
                    ));
                }
                if p.current_token() == PyTokenKind::TkRightBracket {
                    p.bump();
                } else {
                    p.push_error(PyParseError::syntax_error_from(
                        &t!("expected ']' to close subscript"),
                        p.current_token_range(),
                    ));
                }
                cm = m.complete(p);
            }
            PyTokenKind::TkLeftParen => {
                let m = cm.precede(p, PySyntaxKind::CallExpr);
                parse_args(p)?;
                cm = m.complete(p);
            }
            _ => {
                return Ok(cm);
            }
        }
    }
}

fn parse_name_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::NameExpr);
    p.bump(); // consume name
    Ok(m.complete(p))
}

fn parse_args(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::Arguments);

    if p.current_token() == PyTokenKind::TkLeftParen {
        p.bump(); // consume '('

        if p.current_token() != PyTokenKind::TkRightParen {
            loop {
                // Parse argument expression
                match parse_expr(p) {
                    Ok(_) => {}
                    Err(_) => {
                        p.push_error(PyParseError::syntax_error_from(
                            &t!("expected argument expression"),
                            p.current_token_range(),
                        ));
                        // Skip to next comma or right parenthesis
                        while !matches!(
                            p.current_token(),
                            PyTokenKind::TkComma | PyTokenKind::TkRightParen | PyTokenKind::TkEof
                        ) && !is_statement_start_token(p.current_token())
                        {
                            p.bump();
                        }

                        if p.current_token() == PyTokenKind::TkComma {
                            p.bump();
                            continue;
                        }
                        break;
                    }
                }

                if p.current_token() == PyTokenKind::TkComma {
                    p.bump();
                    if p.current_token() == PyTokenKind::TkRightParen {
                        // trailing comma
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if p.current_token() == PyTokenKind::TkRightParen {
            p.bump();
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected ')' to close argument list"),
                p.current_token_range(),
            ));
        }
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected '(' for function call"),
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    Ok(m.complete(p))
}

/// Parse parenthesized expression or tuple
/// Handles: (), (expr), (expr,), (expr1, expr2, ...)
fn parse_parenthesized_expr_or_tuple(p: &mut PyParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::ParenExpr); // Start as paren expr, might change to tuple
    let paren_range = p.current_token_range();
    p.smart_bump(); // consume '('

    // Empty parentheses - empty tuple  
    if p.current_token() == PyTokenKind::TkRightParen {
        m.set_kind(p, PySyntaxKind::TupleExpr);
        p.smart_bump(); // consume ')'
        return Ok(m.complete(p));
    }

    // Parse first expression
    if parse_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected expression inside parentheses"),
            paren_range,
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Check for comma (indicates tuple)
    let mut is_tuple = false;
    if p.current_token() == PyTokenKind::TkComma {
        is_tuple = true;
        p.bump(); // consume comma

        // Parse remaining elements (could be empty for single-element tuple like (x,))
        while p.current_token() != PyTokenKind::TkRightParen && p.current_token() != PyTokenKind::TkEof {
            if parse_expr(p).is_err() {
                break;
            }
            
            if p.current_token() == PyTokenKind::TkComma {
                p.bump(); // consume comma
                // Allow trailing comma
            } else {
                break;
            }
        }
    }

    // Expect closing parenthesis
    if p.current_token() == PyTokenKind::TkRightParen {
        p.smart_bump(); // consume ')'
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected ')' to close parentheses"),
            p.current_token_range(),
        ));
    }

    // Set the correct node type
    if is_tuple {
        m.set_kind(p, PySyntaxKind::TupleExpr);
    }

    Ok(m.complete(p))
}
