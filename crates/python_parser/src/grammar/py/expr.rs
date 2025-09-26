use crate::{
    grammar::{ParseFailReason, ParseResult, py::is_statement_start_token},
    kind::{BinaryOperator, PyOpKind, PySyntaxKind, PyTokenKind, UNARY_PRIORITY, UnaryOperator},
    parser::{LuaParser, MarkerEventContainer},
    parser_error::LuaParseError,
};

use super::{expect_token, if_token_bump, parse_suite};

pub fn parse_expr(p: &mut LuaParser) -> ParseResult {
    parse_sub_expr(p, 0)
}

fn parse_sub_expr(p: &mut LuaParser, limit: i32) -> ParseResult {
    let uop = PyOpKind::to_unary_operator(p.current_token());
    let mut cm = if uop != UnaryOperator::OpNop {
        let m = p.mark(PySyntaxKind::UnaryExpr);
        let op_range = p.current_token_range();
        let op_token = p.current_token();
        p.bump();
        match parse_sub_expr(p, UNARY_PRIORITY) {
            Ok(_) => {}
            Err(err) => {
                p.push_error(LuaParseError::syntax_error_from(
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
                p.push_error(LuaParseError::syntax_error_from(
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

fn parse_simple_expr(p: &mut LuaParser) -> ParseResult {
    match p.current_token() {
        PyTokenKind::TkInt
        | PyTokenKind::TkFloat
        | PyTokenKind::TkComplex
        | PyTokenKind::TkNil
        | PyTokenKind::TkTrue
        | PyTokenKind::TkFalse
        | PyTokenKind::TkDots
        | PyTokenKind::TkString
        | PyTokenKind::TkLongString => {
            let m = p.mark(PySyntaxKind::LiteralExpr);
            p.bump();
            Ok(m.complete(p))
        }
        PyTokenKind::TkLeftBrace => parse_table_expr(p),
        PyTokenKind::TkFunction => parse_closure_expr(p),
        PyTokenKind::TkName | PyTokenKind::TkLeftParen => parse_suffixed_expr(p),
        _ => {
            // Provide more specific error information
            let error_msg = match p.current_token() {
                PyTokenKind::TkEof => t!("unexpected end of file, expected expression"),
                PyTokenKind::TkRightParen => t!("unexpected ')', expected expression"),
                PyTokenKind::TkRightBrace => t!("unexpected '}', expected expression"),
                PyTokenKind::TkRightBracket => t!("unexpected ']', expected expression"),
                PyTokenKind::TkComma => t!("unexpected ',', expected expression"),
                PyTokenKind::TkSemicolon => t!("unexpected ';', expected expression"),
                PyTokenKind::TkEnd => t!("unexpected 'end', expected expression"),
                PyTokenKind::TkElse => t!("unexpected 'else', expected expression"),
                PyTokenKind::TkElseIf => t!("unexpected 'elseif', expected expression"),
                PyTokenKind::TkThen => t!("unexpected 'then', expected expression"),
                PyTokenKind::TkDo => t!("unexpected 'do', expected expression"),
                PyTokenKind::TkUntil => t!("unexpected 'until', expected expression"),
                _ => t!(
                    "unexpected token '%{token}', expected expression",
                    token = p.current_token()
                ),
            };

            p.push_error(LuaParseError::syntax_error_from(
                &error_msg,
                p.current_token_range(),
            ));
            Err(ParseFailReason::UnexpectedToken)
        }
    }
}

pub fn parse_closure_expr(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ClosureExpr);

    if_token_bump(p, PyTokenKind::TkFunction);

    parse_param_list(p)?;

    if p.current_token() != PyTokenKind::TkEnd {
        parse_suite(p)?;
    }

    if p.current_token() == PyTokenKind::TkEnd {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            &t!("expected 'end' to close function definition"),
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_param_list(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ParamList);

    if p.current_token() == PyTokenKind::TkLeftParen {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            &t!("expected '(' to start parameter list"),
            p.current_token_range(),
        ));
    }

    if p.current_token() != PyTokenKind::TkRightParen {
        loop {
            match parse_param_name(p) {
                Ok(_) => {}
                Err(_) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected parameter name"),
                        p.current_token_range(),
                    ));
                    // Try to recover to next comma or right parenthesis
                    while !matches!(
                        p.current_token(),
                        PyTokenKind::TkComma
                            | PyTokenKind::TkRightParen
                            | PyTokenKind::TkEof
                            | PyTokenKind::TkEnd
                    ) && !is_statement_start_token(p.current_token())
                    {
                        p.bump();
                    }
                }
            }

            if p.current_token() == PyTokenKind::TkComma {
                p.bump();
                // Check if there is a parameter after comma
                if p.current_token() == PyTokenKind::TkRightParen {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected parameter name after ','"),
                        p.current_token_range(),
                    ));
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
        p.push_error(LuaParseError::syntax_error_from(
            &t!("expected ')' to close parameter list"),
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_param_name(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ParamName);

    match p.current_token() {
        PyTokenKind::TkName | PyTokenKind::TkDots => {
            p.bump();
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected parameter name or '...' (vararg)"),
                p.current_token_range(),
            ));
            return Err(ParseFailReason::UnexpectedToken);
        }
    }

    Ok(m.complete(p))
}

fn parse_table_expr(p: &mut LuaParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::TableEmptyExpr);
    p.bump(); // consume '{'

    if p.current_token() == PyTokenKind::TkRightBrace {
        p.bump();
        return Ok(m.complete(p));
    }

    // Parse first field
    match parse_field_with_recovery(p) {
        Ok(cm) => match cm.kind {
            PySyntaxKind::TableFieldAssign => {
                m.set_kind(p, PySyntaxKind::TableObjectExpr);
            }
            PySyntaxKind::TableFieldValue => {
                m.set_kind(p, PySyntaxKind::TableArrayExpr);
            }
            _ => {}
        },
        Err(_) => {
            // If first field parsing failed, continue trying to recover
            recover_to_table_boundary(p);
        }
    }

    // Parse remaining fields
    while matches!(
        p.current_token(),
        PyTokenKind::TkComma | PyTokenKind::TkSemicolon
    ) {
        let separator_token = p.current_token();
        p.bump(); // consume separator

        if p.current_token() == PyTokenKind::TkRightBrace {
            // Allow trailing separator
            break;
        }

        match parse_field_with_recovery(p) {
            Ok(cm) => {
                if cm.kind == PySyntaxKind::TableFieldAssign {
                    m.set_kind(p, PySyntaxKind::TableObjectExpr);
                }
            }
            Err(_) => {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("invalid table field after '%{sep}'", sep = separator_token),
                    p.current_token_range(),
                ));
                // Recover to next field boundary
                recover_to_table_boundary(p);
                if p.current_token() == PyTokenKind::TkRightBrace {
                    break;
                }
            }
        }
    }

    // Handle closing brace
    if p.current_token() == PyTokenKind::TkRightBrace {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            &t!("expected '}' to close table constructor"),
            p.current_token_range(),
        ));

        // Try to recover: look for possible closing brace
        let mut found_brace = false;
        let mut brace_count = 1; // 我们已经在表中
        let mut lookahead_count = 0;
        const MAX_LOOKAHEAD: usize = 50; // 限制向前查看的token数量

        while p.current_token() != PyTokenKind::TkEof && lookahead_count < MAX_LOOKAHEAD {
            match p.current_token() {
                PyTokenKind::TkRightBrace => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        p.bump(); // 消费闭合括号
                        found_brace = true;
                        break;
                    }
                    p.bump();
                }
                PyTokenKind::TkLeftBrace => {
                    brace_count += 1;
                    p.bump();
                }
                // 如果遇到看起来像是表外部的token，停止寻找
                PyTokenKind::TkEnd
                | PyTokenKind::TkElse
                | PyTokenKind::TkElseIf
                | PyTokenKind::TkUntil
                | PyTokenKind::TkThen
                | PyTokenKind::TkDo => {
                    break;
                }
                _ => {
                    p.bump();
                }
            }
            lookahead_count += 1;
        }

        if !found_brace {
            // 如果没有找到闭合括号，在当前位置创建一个错误标记
            p.push_error(LuaParseError::syntax_error_from(
                &t!("table constructor was not properly closed"),
                p.current_token_range(),
            ));
        }
    }

    Ok(m.complete(p))
}

fn parse_field_with_recovery(p: &mut LuaParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::TableFieldValue);

    match p.current_token() {
        PyTokenKind::TkLeftBracket => {
            // [expr] = expr 形式
            m.set_kind(p, PySyntaxKind::TableFieldAssign);
            p.bump(); // consume '['

            match parse_expr(p) {
                Ok(_) => {}
                Err(_) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected expression inside table index brackets"),
                        p.current_token_range(),
                    ));
                    // 恢复到边界
                    while !matches!(
                        p.current_token(),
                        PyTokenKind::TkRightBracket
                            | PyTokenKind::TkAssign
                            | PyTokenKind::TkComma
                            | PyTokenKind::TkSemicolon
                            | PyTokenKind::TkRightBrace
                            | PyTokenKind::TkEof
                    ) {
                        p.bump();
                    }
                }
            }

            if p.current_token() == PyTokenKind::TkRightBracket {
                p.bump();
            } else {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected ']' to close table index"),
                    p.current_token_range(),
                ));
            }

            if p.current_token() == PyTokenKind::TkAssign {
                p.bump();
            } else {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected '=' after table index"),
                    p.current_token_range(),
                ));
            }

            match parse_expr(p) {
                Ok(_) => {}
                Err(_) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected value expression after '='"),
                        p.current_token_range(),
                    ));
                }
            }
        }
        PyTokenKind::TkName => {
            // 可能是 name = expr 或者只是 expr
            if p.peek_next_token() == PyTokenKind::TkAssign {
                m.set_kind(p, PySyntaxKind::TableFieldAssign);
                p.bump(); // consume name
                p.bump(); // consume '='
                match parse_expr(p) {
                    Ok(_) => {}
                    Err(_) => {
                        p.push_error(LuaParseError::syntax_error_from(
                            &t!("expected value expression after field name"),
                            p.current_token_range(),
                        ));
                    }
                }
            } else {
                // 作为表达式解析
                match parse_expr(p) {
                    Ok(_) => {}
                    Err(_) => {
                        p.push_error(LuaParseError::syntax_error_from(
                            &t!("invalid table field expression"),
                            p.current_token_range(),
                        ));
                    }
                }
            }
        }
        // 表示表实际上已经结束的token
        PyTokenKind::TkEof | PyTokenKind::TkLocal => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("unexpected end of table field"),
                p.current_token_range(),
            ));
        }
        _ => {
            // 尝试解析为普通表达式
            match parse_expr(p) {
                Ok(_) => {}
                Err(_) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("invalid table field, expected expression, field assignment, or table end"),
                        p.current_token_range(),
                    ));
                }
            }
        }
    }

    Ok(m.complete(p))
}

fn recover_to_table_boundary(p: &mut LuaParser) {
    // 跳过直到找到表边界或字段分隔符
    while !matches!(
        p.current_token(),
        PyTokenKind::TkComma
            | PyTokenKind::TkSemicolon
            | PyTokenKind::TkRightBrace
            | PyTokenKind::TkEof
    ) {
        p.bump();
    }
}

fn parse_suffixed_expr(p: &mut LuaParser) -> ParseResult {
    let mut cm = match p.current_token() {
        PyTokenKind::TkName => parse_name_or_special_function(p)?,
        PyTokenKind::TkLeftParen => {
            let m = p.mark(PySyntaxKind::ParenExpr);
            let paren_range = p.current_token_range();
            p.bump();
            match parse_expr(p) {
                Ok(_) => {}
                Err(err) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected expression inside parentheses"),
                        paren_range,
                    ));
                    return Err(err);
                }
            }
            if p.current_token() == PyTokenKind::TkRightParen {
                p.bump();
            } else {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected ')' to close parentheses"),
                    paren_range,
                ));
            }
            m.complete(p)
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expect primary expression (identifier or parenthesized expression)"),
                p.current_token_range(),
            ));
            return Err(ParseFailReason::UnexpectedToken);
        }
    };

    loop {
        match p.current_token() {
            PyTokenKind::TkDot | PyTokenKind::TkColon | PyTokenKind::TkLeftBracket => {
                let m = cm.precede(p, PySyntaxKind::IndexExpr);
                parse_index_struct(p)?;
                cm = m.complete(p);
            }
            PyTokenKind::TkLeftParen
            | PyTokenKind::TkLongString
            | PyTokenKind::TkString
            | PyTokenKind::TkLeftBrace => {
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

fn parse_name_or_special_function(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::NameExpr);
    let special_kind = match p.parse_config.get_special_function(p.current_token_text()) {
        SpecialFunction::Require => PySyntaxKind::RequireCallExpr,
        SpecialFunction::Assert => PySyntaxKind::AssertCallExpr,
        SpecialFunction::Error => PySyntaxKind::ErrorCallExpr,
        SpecialFunction::Type => PySyntaxKind::TypeCallExpr,
        SpecialFunction::Setmetaatable => PySyntaxKind::SetmetatableCallExpr,
        _ => PySyntaxKind::None,
    };
    p.bump();
    let mut cm = m.complete(p);
    if special_kind == PySyntaxKind::None {
        return Ok(cm);
    }

    if matches!(
        p.current_token(),
        PyTokenKind::TkLeftParen
            | PyTokenKind::TkLongString
            | PyTokenKind::TkString
            | PyTokenKind::TkLeftBrace
    ) {
        let m1 = cm.precede(p, special_kind);
        parse_args(p)?;
        cm = m1.complete(p);
    }

    Ok(cm)
}

fn parse_index_struct(p: &mut LuaParser) -> Result<(), ParseFailReason> {
    let index_op_range = p.current_token_range();
    match p.current_token() {
        PyTokenKind::TkLeftBracket => {
            p.bump();
            match parse_expr(p) {
                Ok(_) => {}
                Err(err) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected expression inside table index brackets"),
                        index_op_range,
                    ));
                    return Err(err);
                }
            }
            match expect_token(p, PyTokenKind::TkRightBracket) {
                Ok(_) => {}
                Err(err) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected ']' to close table index"),
                        index_op_range,
                    ));
                    return Err(err);
                }
            }
        }
        PyTokenKind::TkDot => {
            p.bump();
            match expect_token(p, PyTokenKind::TkName) {
                Ok(_) => {}
                Err(err) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected field name after '.'"),
                        index_op_range,
                    ));
                    return Err(err);
                }
            }
        }
        PyTokenKind::TkColon => {
            p.bump();
            let name_token_range = p.current_token_range();
            match expect_token(p, PyTokenKind::TkName) {
                Ok(_) => {}
                Err(err) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected method name after ':'"),
                        index_op_range,
                    ));
                    return Err(err);
                }
            }
            if !matches!(
                p.current_token(),
                PyTokenKind::TkLeftParen
                    | PyTokenKind::TkLeftBrace
                    | PyTokenKind::TkString
                    | PyTokenKind::TkLongString
            ) {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!(
                        "colon accessor must be followed by a function call or table constructor or string literal"
                    ),
                    name_token_range,
                ));

                return Err(ParseFailReason::UnexpectedToken);
            }
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expect index struct"),
                p.current_token_range(),
            ));

            return Err(ParseFailReason::UnexpectedToken);
        }
    }

    Ok(())
}

fn parse_args(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::CallArgList);
    match p.current_token() {
        PyTokenKind::TkLeftParen => {
            p.bump();
            if p.current_token() != PyTokenKind::TkRightParen {
                loop {
                    match parse_expr(p) {
                        Ok(_) => {}
                        Err(_) => {
                            p.push_error(LuaParseError::syntax_error_from(
                                &t!("expected argument expression"),
                                p.current_token_range(),
                            ));
                            // 跳过到下一个逗号或右括号
                            while !matches!(
                                p.current_token(),
                                PyTokenKind::TkComma
                                    | PyTokenKind::TkRightParen
                                    | PyTokenKind::TkEof
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
                            p.push_error(LuaParseError::syntax_error_from(
                                &t!("expected expression after ','"),
                                p.current_token_range(),
                            ));
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
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected ')' to close argument list"),
                    p.current_token_range(),
                ));
            }
        }
        PyTokenKind::TkLeftBrace => match parse_table_expr(p) {
            Ok(_) => {}
            Err(err) => {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("invalid table constructor in function call"),
                    p.current_token_range(),
                ));
                return Err(err);
            }
        },
        PyTokenKind::TkString | PyTokenKind::TkLongString => {
            let m1 = p.mark(PySyntaxKind::LiteralExpr);
            p.bump();
            m1.complete(p);
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected '(', string, or table constructor for function call"),
                p.current_token_range(),
            ));

            return Err(ParseFailReason::UnexpectedToken);
        }
    }

    Ok(m.complete(p))
}
