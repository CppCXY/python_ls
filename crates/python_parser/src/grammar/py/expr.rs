use crate::{
    grammar::{ParseFailReason, ParseResult, py::is_statement_start_token},
    kind::{BinaryOperator, PyOpKind, PySyntaxKind, PyTokenKind, UNARY_PRIORITY, UnaryOperator},
    parser::{MarkerEventContainer, PyParser},
    parser_error::PyParseError,
};

use super::if_token_bump;

pub fn parse_expr(p: &mut PyParser) -> ParseResult {
    parse_tuple_or_expr(p)
}

// Parse tuple or single expression (handles comma-separated expressions)
fn parse_tuple_or_expr(p: &mut PyParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::TupleExpr);
    
    // Parse first expression
    let first_expr = parse_sub_expr(p, 0)?;
    
    // Check for comma (indicates tuple)
    if p.current_token() == PyTokenKind::TkComma {
        // This is a tuple - parse remaining elements
        while p.current_token() == PyTokenKind::TkComma {
            p.bump(); // consume comma
            
            // Optional trailing comma (especially for single-element tuples)
            if matches!(p.current_token(), 
                PyTokenKind::TkNewline | PyTokenKind::TkRightParen | PyTokenKind::TkRightBrace |
                PyTokenKind::TkRightBracket | PyTokenKind::TkEof | PyTokenKind::TkDedent
            ) {
                break;
            }
            
            // Parse next expression
            if parse_sub_expr(p, 0).is_err() {
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
        PyTokenKind::TkYield => parse_yield_expr(p),
        PyTokenKind::TkAwait => parse_await_expr(p),
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
    } else if !matches!(p.current_token(), 
        PyTokenKind::TkNewline | PyTokenKind::TkRightParen | PyTokenKind::TkComma | 
        PyTokenKind::TkRightBracket | PyTokenKind::TkRightBrace | PyTokenKind::TkEof |
        PyTokenKind::TkDedent | PyTokenKind::TkColon
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

    // Set the correct node type
    if is_tuple {
        m.set_kind(p, PySyntaxKind::TupleExpr);
    }

    Ok(m.complete(p))
}

// Parse a single argument (either positional, keyword, *args, or **kwargs)
fn parse_argument(p: &mut PyParser) {
    // Check for **kwargs
    if p.current_token() == PyTokenKind::TkPow {
        let marker = p.mark(PySyntaxKind::StarredExpr);
        p.bump(); // consume '**'
        
        if parse_expr(p).is_err() {
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
        
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected expression after '*'"),
                p.current_token_range(),
            ));
        }
        
        marker.complete(p);
        return;
    }
    
    // Check for keyword argument pattern: NAME '=' 
    // We'll use the fact that in Python, keyword args have NAME = EXPR pattern
    if p.current_token() == PyTokenKind::TkName {
        let marker = p.mark(PySyntaxKind::Keyword);
        p.bump(); // consume name
        
        if p.current_token() == PyTokenKind::TkAssign {
            // This is indeed a keyword argument
            p.bump(); // consume '='
            
            // Parse value expression
            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected value expression after '='"),
                    p.current_token_range(),
                ));
            }
            
            marker.complete(p);
            return;
        } else {
            // Not a keyword argument, rollback and parse as expression
            marker.undo(p);
            // We need to restore the parser position, but since we can't, 
            // let's create a name expression manually
            let name_marker = p.mark(PySyntaxKind::NameExpr);
            // The name is already consumed, so we just complete the name expression
            name_marker.complete(p);
            return;
        }
    }
    
    // Parse regular argument expression
    if parse_expr(p).is_err() {
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
    if p.current_token() != PyTokenKind::TkColon && p.current_token() != PyTokenKind::TkRightBracket {
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
        if p.current_token() != PyTokenKind::TkColon && p.current_token() != PyTokenKind::TkRightBracket {
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
