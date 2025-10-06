use crate::{
    grammar::{ParseFailReason, ParseResult, fstring::parse_fstring_expr},
    kind::{BinaryOperator, PyOpKind, PySyntaxKind, PyTokenKind, UNARY_PRIORITY, UnaryOperator},
    parser::{MarkerEventContainer, PyParser},
    parser_error::PyParseError,
};

use super::if_token_bump;

pub fn parse_fstring_inner_expr(p: &mut PyParser) {
    p.init();

    let _ = parse_single_expr(p);
    while p.current_token() != PyTokenKind::TkEof {
        p.bump();
    }
}

pub fn parse_expr(p: &mut PyParser) -> ParseResult {
    parse_tuple_or_expr(p)
}

// Parse single expression without tuple handling (for contexts like dict values, function args)
pub fn parse_single_expr(p: &mut PyParser) -> ParseResult {
    parse_conditional_or_expr(p)
}

// Parse conditional expression or regular expression
fn parse_conditional_or_expr(p: &mut PyParser) -> ParseResult {
    let expr = parse_sub_expr(p, 0)?;

    // Check for conditional expression pattern: expr if condition else expr
    if p.current_token() == PyTokenKind::TkIf {
        let m = expr.precede(p, PySyntaxKind::ConditionalExpr);
        p.bump(); // consume 'if'

        // Parse condition
        if parse_sub_expr(p, 0).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                "expected condition after 'if' in conditional expression",
                p.current_token_range(),
            ));
            return Ok(m.complete(p));
        }

        // Expect 'else'
        if p.current_token() == PyTokenKind::TkElse {
            p.bump(); // consume 'else'
        } else {
            p.push_error(PyParseError::syntax_error_from(
                "expected 'else' in conditional expression",
                p.current_token_range(),
            ));
            return Ok(m.complete(p));
        }

        // Parse else expression
        if parse_conditional_or_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                "expected expression after 'else' in conditional expression",
                p.current_token_range(),
            ));
        }

        Ok(m.complete(p))
    } else {
        Ok(expr)
    }
}

// Parse tuple or single expression (handles comma-separated expressions)
fn parse_tuple_or_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::TupleExpr);

    // Parse first expression
    let first_expr = parse_conditional_or_expr(p)?;

    // Check for comma (indicates tuple)
    if p.current_token() == PyTokenKind::TkComma {
        // This is a tuple - parse remaining elements
        while p.current_token() == PyTokenKind::TkComma {
            p.bump(); // consume comma

            // Optional trailing comma (especially for single-element tuples)
            if matches!(
                p.current_token(),
                PyTokenKind::TkNewline
                    | PyTokenKind::TkRightParen
                    | PyTokenKind::TkRightBrace
                    | PyTokenKind::TkRightBracket
                    | PyTokenKind::TkEof
                    | PyTokenKind::TkDedent
            ) {
                break;
            }

            // Parse next expression
            if parse_conditional_or_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected expression after ','"),
                    p.current_token_range(),
                ));
                break;
            }
        }

        Ok(m.complete(p))
    } else {
        // Single expression, not a tuple
        m.undo(p);
        Ok(first_expr)
    }
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

        // Special handling for assignment expressions (walrus operator :=)
        if bop == BinaryOperator::OpAssignExpr {
            let m = cm.precede(p, PySyntaxKind::AssignExpr);
            p.bump(); // consume ':='

            match parse_sub_expr(p, bop.get_priority().right) {
                Ok(_) => {}
                Err(err) => {
                    p.push_error(PyParseError::syntax_error_from(
                        &t!("assignment expression is not followed by an expression"),
                        op_range,
                    ));
                    return Err(err);
                }
            }

            cm = m.complete(p);
        } else {
            // Regular binary operators
            let m = cm.precede(p, PySyntaxKind::BinaryExpr);

            // Check for union types (Python 3.10+)
            if bop == BinaryOperator::OpBitOr {
                // This could be a union type if used in type annotation context
                // For now, we'll emit a general warning for | operator usage
                p.check_version_warning(
                    &t!("syntax.union_type_operator"),
                    p.parse_config.level.support_union_types(),
                    "Python 3.10+",
                );
            }

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
        }

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
        | PyTokenKind::TkRawString
        | PyTokenKind::TkEllipsis => {
            let m = p.mark(PySyntaxKind::LiteralExpr);
            p.bump();
            Ok(m.complete(p))
        }
        PyTokenKind::TkFString => parse_fstring_expr(p),
        PyTokenKind::TkTString => parse_tstring_expr(p),
        PyTokenKind::TkLeftBracket => parse_list_expr(p),
        PyTokenKind::TkLeftBrace => parse_dict_or_set_expr(p),
        PyTokenKind::TkLambda => parse_lambda_expr(p),
        PyTokenKind::TkYield => parse_yield_expr(p),
        PyTokenKind::TkAwait => parse_await_expr(p),
        PyTokenKind::TkMul => parse_starred_expr(p),
        PyTokenKind::TkPow => parse_double_starred_expr(p),
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

fn parse_yield_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::YieldExpr);
    p.bump(); // consume 'yield'

    // Parse optional value expression
    // yield can be used without a value (yield), or with a value (yield expr)
    // or with yield from (yield from expr)
    if p.current_token() == PyTokenKind::TkFrom {
        p.bump(); // consume 'from'

        // Parse expression after 'yield from'
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected expression after 'yield from'"),
                p.current_token_range(),
            ));
        }
    } else if !matches!(
        p.current_token(),
        PyTokenKind::TkNewline
            | PyTokenKind::TkRightParen
            | PyTokenKind::TkComma
            | PyTokenKind::TkRightBracket
            | PyTokenKind::TkRightBrace
            | PyTokenKind::TkEof
            | PyTokenKind::TkDedent
            | PyTokenKind::TkColon
    ) {
        // Parse optional yield value (if present)
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected expression after 'yield'"),
                p.current_token_range(),
            ));
        }
    }

    Ok(m.complete(p))
}

fn parse_await_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::AwaitExpr);
    p.bump(); // consume 'await'

    // Parse awaitable expression
    if parse_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected expression after 'await'"),
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_tstring_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::TStringExpr);
    
    // T-strings are supported in Python 3.14+
    // Similar to f-strings but return Template objects instead of strings
    
    p.bump(); // consume t-string token
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

    // Parse first expression (avoid conditional expression parsing in comprehension context)
    if parse_sub_expr(p, 0).is_err() {
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

        // Parse iterator expression (avoid conditional expression parsing)
        if parse_sub_expr(p, 0).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected iterator expression after 'in'"),
                p.current_token_range(),
            ));
        }

        // Optional 'if' condition (avoid conditional expression parsing)
        if p.current_token() == PyTokenKind::TkIf {
            p.bump(); // consume 'if'
            if parse_sub_expr(p, 0).is_err() {
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

            if parse_conditional_or_expr(p).is_err() {
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
            if parse_single_expr(p).is_err() {
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
            if parse_single_expr(p).is_err() {
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

fn parse_dict_or_set_expr(p: &mut PyParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::DictExpr);
    p.bump(); // consume '{'

    // Empty dict/set
    if p.current_token() == PyTokenKind::TkRightBrace {
        p.bump(); // consume '}'
        return Ok(m.complete(p));
    }

    // Parse first expression
    if parse_sub_expr(p, 0).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected expression in dict or set"),
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Determine if it's a dict, set, or comprehension
    match p.current_token() {
        PyTokenKind::TkColon => {
            // This is a dict: {key: value, ...}
            p.bump(); // consume ':'
            
            // Parse value
            if parse_single_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected value expression after ':'"),
                    p.current_token_range(),
                ));
            }
            
            // Check for dict comprehension
            if p.current_token() == PyTokenKind::TkFor {
                m.set_kind(p, PySyntaxKind::DictCompExpr);
                parse_comprehension_clauses(p);
            } else {
                // Regular dict - parse remaining key-value pairs
                while p.current_token() == PyTokenKind::TkComma {
                    p.bump();
                    if p.current_token() == PyTokenKind::TkRightBrace {
                        break;
                    }
                    
                    if parse_single_expr(p).is_err() {
                        p.push_error(PyParseError::syntax_error_from(
                            &t!("expected key expression"),
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
                    
                    if parse_single_expr(p).is_err() {
                        p.push_error(PyParseError::syntax_error_from(
                            &t!("expected value expression"),
                            p.current_token_range(),
                        ));
                        break;
                    }
                }
            }
        }
        PyTokenKind::TkFor => {
            // Set comprehension: {expr for ...}
            m.set_kind(p, PySyntaxKind::SetCompExpr);
            parse_comprehension_clauses(p);
        }
        PyTokenKind::TkComma => {
            // Set: {expr, expr, ...}
            m.set_kind(p, PySyntaxKind::SetExpr);
            
            while p.current_token() == PyTokenKind::TkComma {
                p.bump();
                if p.current_token() == PyTokenKind::TkRightBrace {
                    break;
                }
                
                if parse_conditional_or_expr(p).is_err() {
                    p.push_error(PyParseError::syntax_error_from(
                        &t!("expected expression in set"),
                        p.current_token_range(),
                    ));
                    break;
                }
            }
        }
        PyTokenKind::TkRightBrace => {
            // Single element set: {expr}
            m.set_kind(p, PySyntaxKind::SetExpr);
        }
        _ => {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected ':', ',', 'for', or '}'"),
                p.current_token_range(),
            ));
        }
    }

    if p.current_token() == PyTokenKind::TkRightBrace {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected '}' to close dict or set"),
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_comprehension_clauses(p: &mut PyParser) {
    // Parse 'for' clause
    p.bump(); // consume 'for'
    
    // Parse target variable(s)
    if p.current_token() == PyTokenKind::TkName {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected target variable after 'for'"),
            p.current_token_range(),
        ));
        return;
    }
    
    // Parse 'in' keyword
    if p.current_token() == PyTokenKind::TkIn {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected 'in' after for target"),
            p.current_token_range(),
        ));
        return;
    }
    
    // Parse iterator expression
    if parse_sub_expr(p, 0).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected iterator expression after 'in'"),
            p.current_token_range(),
        ));
        return;
    }
    
    // Optional 'if' conditions
    while p.current_token() == PyTokenKind::TkIf {
        p.bump();
        if parse_sub_expr(p, 0).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected condition after 'if'"),
                p.current_token_range(),
            ));
            return;
        }
    }
    
    // Additional 'for' clauses
    if p.current_token() == PyTokenKind::TkFor {
        parse_comprehension_clauses(p);
    }
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
        PyTokenKind::TkLeftParen => parse_parenthesized_expr_or_tuple(p)?,
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

                // Parse index or slice expression
                parse_subscript_inner(p);

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
                // Try to parse keyword argument or regular expression
                parse_argument(p);

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
/// Handles: (), (expr), (expr,), (expr1, expr2, ...), (expr for ...)
fn parse_parenthesized_expr_or_tuple(p: &mut PyParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::ParenExpr); // Start as paren expr, might change to tuple or generator
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

    // Check for generator expression or tuple
    if p.current_token() == PyTokenKind::TkFor {
        // This is a generator expression: (expr for ...)
        m.set_kind(p, PySyntaxKind::GeneratorExpr);
        parse_comprehension_clauses(p);
    } else if p.current_token() == PyTokenKind::TkComma {
        // This is a tuple
        m.set_kind(p, PySyntaxKind::TupleExpr);
        p.bump(); // consume comma

        // Parse remaining elements (could be empty for single-element tuple like (x,))
        while p.current_token() != PyTokenKind::TkRightParen
            && p.current_token() != PyTokenKind::TkEof
        {
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

    Ok(m.complete(p))
}

// Parse a single argument (either positional, keyword, *args, or **kwargs)
fn parse_argument(p: &mut PyParser) {
    // Check for **kwargs
    if p.current_token() == PyTokenKind::TkPow {
        let marker = p.mark(PySyntaxKind::StarredExpr);
        p.bump(); // consume '**'

        if parse_single_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected expression after '**'"),
                p.current_token_range(),
            ));
        }

        marker.complete(p);
        return;
    }

    // Check for *args
    if p.current_token() == PyTokenKind::TkMul {
        let marker = p.mark(PySyntaxKind::StarredExpr);
        p.bump(); // consume '*'

        if parse_single_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected expression after '*'"),
                p.current_token_range(),
            ));
        }

        marker.complete(p);
        return;
    }

    // Special handling for keyword arguments in call context
    // Check if it's a simple name followed by '='
    if p.current_token() == PyTokenKind::TkName {
        // Check if next token is '=' without consuming the name
        let next_token = p.peek_next_token(); // Look ahead one token

        if next_token == PyTokenKind::TkAssign {
            // This is a keyword argument: name = value
            let keyword_marker = p.mark(PySyntaxKind::Keyword);

            // Parse name
            let name_marker = p.mark(PySyntaxKind::NameExpr);
            p.bump(); // consume name
            name_marker.complete(p);

            // consume '='
            p.bump();

            // Parse value expression
            if parse_single_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected value expression after '='"),
                    p.current_token_range(),
                ));
            }

            keyword_marker.complete(p);
            return;
        }
    }

    // Parse regular expression for positional arguments
    if parse_single_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected argument expression"),
            p.current_token_range(),
        ));
    }
}

// Parse subscript/slice expression inside brackets
fn parse_subscript_inner(p: &mut PyParser) {
    // Handle slice syntax: [start:end:step] or simple index [expr]

    // Parse optional first expression (start or index)
    if p.current_token() != PyTokenKind::TkColon && p.current_token() != PyTokenKind::TkRightBracket
    {
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected index or slice expression"),
                p.current_token_range(),
            ));
            return;
        }
    }

    // Check if this is a slice (contains colon)
    if p.current_token() == PyTokenKind::TkColon {
        let slice_m = p.mark(PySyntaxKind::SliceExpr);
        p.bump(); // consume ':'

        // Parse optional end expression
        if p.current_token() != PyTokenKind::TkColon
            && p.current_token() != PyTokenKind::TkRightBracket
        {
            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected end expression in slice"),
                    p.current_token_range(),
                ));
            }
        }

        // Parse optional step if there's another colon
        if p.current_token() == PyTokenKind::TkColon {
            p.bump(); // consume second ':'

            // Parse optional step expression
            if p.current_token() != PyTokenKind::TkRightBracket {
                if parse_expr(p).is_err() {
                    p.push_error(PyParseError::syntax_error_from(
                        &t!("expected step expression in slice"),
                        p.current_token_range(),
                    ));
                }
            }
        }

        slice_m.complete(p);
    }
}

fn parse_starred_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::StarredExpr);
    p.bump(); // consume '*'

    if parse_single_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            "expected expression after '*'",
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_double_starred_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::DoubleStarredExpr);
    p.bump(); // consume '**'

    if parse_single_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            "expected expression after '**'",
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}
