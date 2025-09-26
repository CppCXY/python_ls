use crate::{
    grammar::{ParseFailReason, ParseResult, py::is_statement_start_token},
    kind::{PySyntaxKind, PyTokenKind},
    parser::{CompleteMarker, LuaParser, MarkerEventContainer},
    parser_error::LuaParseError,
};

use super::{
    expect_token,
    expr::{parse_expr},
    if_token_bump, parse_suite,
};

// Parse an indented block (suite in Python grammar)
fn parse_suite(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::Module); // Use Module as block equivalent
    
    // Expect INDENT
    if p.current_token() == PyTokenKind::TkIndent {
        p.bump();
    }
    
    // Parse statements until DEDENT
    while !matches!(p.current_token(), PyTokenKind::TkDedent | PyTokenKind::TkEof) {
        parse_stat(p)?;
    }
    
    // Expect DEDENT
    if p.current_token() == PyTokenKind::TkDedent {
        p.bump();
    }
    
    Ok(m.complete(p))
}

/// Push expression parsing error with lazy error message generation
fn push_expr_error_lazy<F>(p: &mut LuaParser, error_msg_fn: F)
where
    F: FnOnce() -> std::borrow::Cow<'static, str>,
{
    let error_msg = error_msg_fn();
    p.push_error(LuaParseError::syntax_error_from(
        &error_msg,
        p.current_token_range(),
    ));
}

/// Generic keyword expectation with error recovery and lazy error message generation
fn expect_keyword_with_recovery<F>(
    p: &mut LuaParser,
    expected: PyTokenKind,
    error_msg_fn: F,
) -> bool
where
    F: FnOnce() -> std::borrow::Cow<'static, str>,
{
    if p.current_token() == expected {
        p.bump();
        true
    } else {
        let error_msg = error_msg_fn();
        p.push_error(LuaParseError::syntax_error_from(
            &error_msg,
            p.current_token_range(),
        ));

        // Check if we can continue parsing (assume user forgot the keyword)
        is_statement_start_token(p.current_token())
    }
}

/// Expect 'end' keyword, report error at start keyword location if missing
fn expect_end_keyword<F>(p: &mut LuaParser, start_range: crate::text::SourceRange, error_msg_fn: F)
where
    F: FnOnce() -> std::borrow::Cow<'static, str>,
{
    if p.current_token() == PyTokenKind::TkEnd {
        p.bump();
    } else {
        let error_msg = error_msg_fn();
        // Report error at the start keyword location
        p.push_error(LuaParseError::syntax_error_from(&error_msg, start_range));

        // Try to recover: look for possible 'end' or other structure terminators
        recover_to_block_end(p);
    }
}

/// Error recovery: skip to block end markers
fn recover_to_block_end(p: &mut LuaParser) {
    let mut depth = 1;

    while p.current_token() != PyTokenKind::TkEof && depth > 0 {
        match p.current_token() {
            // Nested structure starts
            PyTokenKind::TkIf
            | PyTokenKind::TkWhile
            | PyTokenKind::TkFor
            | PyTokenKind::TkDo
            | PyTokenKind::TkFunction => {
                depth += 1;
                p.bump();
            }
            // Structure ends
            PyTokenKind::TkEnd => {
                depth -= 1;
                if depth == 0 {
                    p.bump(); // Consume the found 'end'
                }
            }
            // Other possible recovery points
            PyTokenKind::TkElseIf | PyTokenKind::TkElse => {
                if depth == 1 {
                    // Found same-level elseif/else, can recover
                    break;
                }
                p.bump();
            }
            // Other control flow end markers
            PyTokenKind::TkUntil => {
                depth -= 1;
                if depth == 0 {
                    // This might be the end of repeat-until
                    break;
                }
                p.bump();
            }
            _ => {
                p.bump();
            }
        }
    }
}

/// Error recovery: skip to specified keywords
fn recover_to_keywords(p: &mut LuaParser, keywords: &[PyTokenKind]) {
    while p.current_token() != PyTokenKind::TkEof {
        if keywords.contains(&p.current_token()) {
            break;
        }

        // Also stop recovery if we encounter statement start markers
        if is_statement_start_token(p.current_token()) {
            break;
        }

        p.bump();
    }
}

// Parse a comma-separated list of expressions, returning an error message only if there's an error.
fn parse_expr_list_impl(p: &mut LuaParser) -> Result<(), &'static str> {
    parse_expr(p).map_err(|_| "expected expression")?;

    while p.current_token() == PyTokenKind::TkComma {
        p.bump();
        parse_expr(p).map_err(|_| "expected expression after ','")?;
    }

    Ok(())
}

// Parse a comma-separated list of variable names.
fn parse_variable_name_list(p: &mut LuaParser, support_attrib: bool) -> ParseResult {
    parse_local_name(p, support_attrib)?;

    while p.current_token() == PyTokenKind::TkComma {
        p.bump();
        match parse_local_name(p, support_attrib) {
            Ok(_) => {}
            Err(_) => {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected variable name after ','"),
                    p.current_token_range(),
                ));
            }
        }
    }

    Ok(CompleteMarker::empty())
}

pub fn parse_stats(p: &mut LuaParser) {
    while !block_follow(p) {
        let level = p.get_mark_level();
        match parse_stat(p) {
            Ok(_) => {}
            Err(_) => {
                let current_level = p.get_mark_level();
                for _ in 0..(current_level - level) {
                    p.push_node_end();
                }

                let mut can_continue = false;
                // error recover
                while p.current_token() != PyTokenKind::TkEof {
                    if is_statement_start_token(p.current_token()) {
                        can_continue = true;
                        break;
                    }

                    p.bump();
                }

                if can_continue {
                    continue;
                }
                break;
            }
        }
    }
}

fn block_follow(p: &LuaParser) -> bool {
    matches!(
        p.current_token(),
        PyTokenKind::TkElse
            | PyTokenKind::TkElif
            | PyTokenKind::TkExcept
            | PyTokenKind::TkFinally
            | PyTokenKind::TkEof
            | PyTokenKind::TkDedent
    )
}

fn parse_stat(p: &mut LuaParser) -> ParseResult {
    let cm = match p.current_token() {
        // Python keywords
        PyTokenKind::TkIf => parse_if(p)?,
        PyTokenKind::TkWhile => parse_while(p)?,
        PyTokenKind::TkFor => parse_for(p)?,
        PyTokenKind::TkDef => parse_def(p)?,
        PyTokenKind::TkClass => parse_class(p)?,
        PyTokenKind::TkImport => parse_import(p)?,
        PyTokenKind::TkFrom => parse_from_import(p)?,
        PyTokenKind::TkReturn => parse_return(p)?,
        PyTokenKind::TkBreak => parse_break(p)?,
        PyTokenKind::TkContinue => parse_continue(p)?,
        PyTokenKind::TkPass => parse_pass(p)?,
        PyTokenKind::TkRaise => parse_raise(p)?,
        PyTokenKind::TkTry => parse_try(p)?,
        PyTokenKind::TkWith => parse_with(p)?,
        PyTokenKind::TkAssert => parse_assert(p)?,
        PyTokenKind::TkDel => parse_del(p)?,
        PyTokenKind::TkGlobal => parse_global(p)?,
        PyTokenKind::TkNonlocal => parse_nonlocal(p)?,
        PyTokenKind::TkYield => parse_yield_stmt(p)?,
        PyTokenKind::TkAsync => parse_async_stmt(p)?,
        PyTokenKind::TkNewline => parse_newline(p)?,
        _ => parse_assign_or_expr_stat(p)?,
    };

    Ok(cm)
}

fn parse_if(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::IfStmt);
    p.bump(); // consume 'if'

    // Parse condition expression
    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || "expected condition expression after 'if'");
        recover_to_keywords(p, &[PyTokenKind::TkColon, PyTokenKind::TkElif, PyTokenKind::TkElse]);
    }

    // Expect ':'
    if !expect_keyword_with_recovery(p, PyTokenKind::TkColon, || {
        "expected ':' after if condition"
    }) {
        recover_to_keywords(p, &[PyTokenKind::TkElif, PyTokenKind::TkElse, PyTokenKind::TkIndent]);
    }

    // Parse suite (indented block)
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            "expected indented block after ':'",
            p.current_token_range(),
        ));
    }

    // Parse elif clauses
    while p.current_token() == PyTokenKind::TkElif {
        parse_elif_clause(p)?;
    }

    // Parse else clause
    if p.current_token() == PyTokenKind::TkElse {
        parse_else_clause(p)?;
    }

    Ok(m.complete(p))
}

fn parse_elif_clause(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ElifClause);
    p.bump(); // consume 'elif'

    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || "expected condition expression after 'elif'");
    }

    expect_keyword_with_recovery(p, PyTokenKind::TkColon, || {
        "expected ':' after 'elif' condition"
    });

    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    Ok(m.complete(p))
}

fn parse_else_clause(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ElseClause);
    p.bump(); // consume 'else'
    
    expect_keyword_with_recovery(p, PyTokenKind::TkColon, || {
        "expected ':' after 'else'"
    });

    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    Ok(m.complete(p))
}

fn parse_while(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::WhileStat);
    let while_start_range = p.current_token_range();
    p.bump(); // consume 'while'

    // Parse condition expression
    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || t!("expected condition expression after 'while'"));
        recover_to_keywords(p, &[PyTokenKind::TkDo, PyTokenKind::TkEnd]);
    }

    // Expect 'do'
    if !expect_keyword_with_recovery(p, PyTokenKind::TkDo, || {
        t!("expected 'do' after while condition")
    }) {
        recover_to_keywords(p, &[PyTokenKind::TkEnd]);
    }

    // 只有在找到合适的恢复点时才解析块
    if p.current_token() != PyTokenKind::TkEnd && p.current_token() != PyTokenKind::TkEof {
        parse_suite(p)?;
    }

    // Use new end expectation function to associate error with 'while' keyword
    expect_end_keyword(p, while_start_range, || {
        t!("expected 'end' to close while statement")
    });

    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_do(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::DoStat);
    let do_start_range = p.current_token_range();
    p.bump();

    parse_suite(p)?;

    expect_end_keyword(p, do_start_range, || t!("expected 'end' after 'do' block"));

    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_for(p: &mut LuaParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::ForStat);
    let for_start_range = p.current_token_range();
    p.bump(); // consume 'for'

    // Expect variable name
    if p.current_token() == PyTokenKind::TkName {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            &t!("expected variable name after 'for'"),
            p.current_token_range(),
        ));
        // Try to recover: skip to '=' or 'in'
        recover_to_keywords(
            p,
            &[
                PyTokenKind::TkAssign,
                PyTokenKind::TkIn,
                PyTokenKind::TkComma,
                PyTokenKind::TkDo,
                PyTokenKind::TkEnd,
            ],
        );
    }

    match p.current_token() {
        PyTokenKind::TkAssign => {
            // Numeric for loop
            p.bump();
            // Start value
            if parse_expr(p).is_err() {
                push_expr_error_lazy(p, || {
                    t!("expected start value expression in numeric for loop")
                });
            }

            if p.current_token() == PyTokenKind::TkComma {
                p.bump();
            } else {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected ',' after start value in numeric for loop"),
                    p.current_token_range(),
                ));
            }

            // End value
            if parse_expr(p).is_err() {
                push_expr_error_lazy(p, || {
                    t!("expected end value expression in numeric for loop")
                });
            }

            // Optional step value
            if p.current_token() == PyTokenKind::TkComma {
                p.bump();
                if parse_expr(p).is_err() {
                    push_expr_error_lazy(p, || {
                        t!("expected step value expression in numeric for loop")
                    });
                }
            }
        }
        PyTokenKind::TkComma | PyTokenKind::TkIn => {
            // Generic for loop
            m.set_kind(p, PySyntaxKind::ForRangeStat);
            while p.current_token() == PyTokenKind::TkComma {
                p.bump();
                if p.current_token() == PyTokenKind::TkName {
                    p.bump();
                } else {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected variable name after ','"),
                        p.current_token_range(),
                    ));
                }
            }

            if p.current_token() == PyTokenKind::TkIn {
                p.bump();
            } else {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected 'in' after variable list in generic for loop"),
                    p.current_token_range(),
                ));
            }

            // Iterator expression list
            if parse_expr_list_impl(p).is_err() {
                push_expr_error_lazy(p, || t!("expected iterator expression after 'in'"));
            }
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected '=' for numeric for loop or ',' or 'in' for generic for loop"),
                p.current_token_range(),
            ));
        }
    }

    // Expect 'do'
    if !expect_keyword_with_recovery(p, PyTokenKind::TkDo, || {
        t!("expected 'do' in for statement")
    }) {
        recover_to_keywords(p, &[PyTokenKind::TkEnd]);
    }

    // 只有在找到合适的恢复点时才解析块
    if p.current_token() != PyTokenKind::TkEnd && p.current_token() != PyTokenKind::TkEof {
        parse_suite(p)?;
    }

    expect_end_keyword(p, for_start_range, || {
        t!("expected 'end' to close for statement")
    });

    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_function(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::FuncStat);
    p.bump();
    parse_func_name(p)?;
    parse_closure_expr(p)?;
    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_func_name(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::NameExpr);
    match expect_token(p, PyTokenKind::TkName) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected function name after 'function'"),
                p.current_token_range(),
            ));
            return Err(ParseFailReason::UnexpectedToken);
        }
    }

    let cm =
        if p.current_token() == PyTokenKind::TkDot || p.current_token() == PyTokenKind::TkColon {
            let mut cm = m.complete(p);
            while p.current_token() == PyTokenKind::TkDot {
                let m = cm.precede(p, PySyntaxKind::IndexExpr);
                p.bump();
                match expect_token(p, PyTokenKind::TkName) {
                    Ok(_) => {}
                    Err(_) => {
                        p.push_error(LuaParseError::syntax_error_from(
                            &t!("expected name after '.'"),
                            p.current_token_range(),
                        ));
                    }
                }
                cm = m.complete(p);
            }

            if p.current_token() == PyTokenKind::TkColon {
                let m = cm.precede(p, PySyntaxKind::IndexExpr);
                p.bump();
                match expect_token(p, PyTokenKind::TkName) {
                    Ok(_) => {}
                    Err(_) => {
                        p.push_error(LuaParseError::syntax_error_from(
                            &t!("expected name after ':'"),
                            p.current_token_range(),
                        ));
                    }
                }
                cm = m.complete(p);
            }

            cm
        } else {
            m.complete(p)
        };

    Ok(cm)
}

fn parse_local(p: &mut LuaParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::LocalStat);
    p.bump(); // consume 'local'

    match p.current_token() {
        PyTokenKind::TkFunction => {
            p.bump();
            m.set_kind(p, PySyntaxKind::LocalFuncStat);

            match parse_local_name(p, false) {
                Ok(_) => {}
                Err(_) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("expected function name after 'local function'"),
                        p.current_token_range(),
                    ));
                }
            }

            match parse_closure_expr(p) {
                Ok(_) => {}
                Err(_) => {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!("invalid function definition"),
                        p.current_token_range(),
                    ));
                }
            }
        }
        PyTokenKind::TkName => {
            parse_variable_name_list(p, true)?;

            // 可选的初始化表达式
            if p.current_token().is_assign_op() {
                p.bump();
                if parse_expr_list_impl(p).is_err() {
                    push_expr_error_lazy(p, || t!("expected initialization expression after '='"));
                }
            }
        }
        PyTokenKind::TkLt => {
            if p.parse_config.level >= LuaLanguageLevel::Lua55 {
                match parse_attrib(p) {
                    Ok(_) => {}
                    Err(_) => {
                        p.push_error(LuaParseError::syntax_error_from(
                            &t!("invalid attribute syntax"),
                            p.current_token_range(),
                        ));
                    }
                }

                parse_variable_name_list(p, true)?;

                if p.current_token().is_assign_op() {
                    p.bump();
                    if parse_expr_list_impl(p).is_err() {
                        push_expr_error_lazy(p, || {
                            t!("expected initialization expression after '='")
                        });
                    }
                }
            } else {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!(
                        "local attributes are not supported in Lua version %{level}",
                        level = p.parse_config.level
                    ),
                    p.current_token_range(),
                ));

                return Err(ParseFailReason::UnexpectedToken);
            }
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected 'function', variable name, or attribute after 'local'"),
                p.current_token_range(),
            ));

            return Err(ParseFailReason::UnexpectedToken);
        }
    }

    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_local_name(p: &mut LuaParser, support_attrib: bool) -> ParseResult {
    let m = p.mark(PySyntaxKind::LocalName);
    match expect_token(p, PyTokenKind::TkName) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected variable name after 'local'"),
                p.current_token_range(),
            ));
        }
    }
    if support_attrib && p.current_token() == PyTokenKind::TkLt {
        parse_attrib(p)?;
    }

    Ok(m.complete(p))
}

fn parse_attrib(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::Attribute);
    let range = p.current_token_range();
    p.bump();
    match expect_token(p, PyTokenKind::TkName) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected attribute name after '<'"),
                p.current_token_range(),
            ));
        }
    }
    match expect_token(p, PyTokenKind::TkGt) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected '>' after attribute name"),
                p.current_token_range(),
            ));
        }
    }
    if !p.parse_config.support_local_attrib() {
        p.errors.push(LuaParseError::syntax_error_from(
            &t!(
                "local attribute is not supported for current version: %{level}",
                level = p.parse_config.level
            ),
            range,
        ));
    }

    Ok(m.complete(p))
}

fn parse_return(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ReturnStat);
    p.bump();
    if !block_follow(p)
        && p.current_token() != PyTokenKind::TkSemicolon
        && parse_expr_list_impl(p).is_err()
    {
        push_expr_error_lazy(p, || t!("expected expression in return statement"));
    }

    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_break(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::BreakStat);
    p.bump();
    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_repeat(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::RepeatStat);
    p.bump();
    parse_suite(p)?;
    match expect_token(p, PyTokenKind::TkUntil) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected 'until' after repeat block"),
                p.current_token_range(),
            ));
        }
    }
    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || t!("expected condition expression after 'until'"));
    }
    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_goto(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::GotoStat);
    p.bump();
    match expect_token(p, PyTokenKind::TkName) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected label name after 'goto'"),
                p.current_token_range(),
            ));
        }
    }
    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_empty_stat(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::EmptyStat);
    p.bump();
    Ok(m.complete(p))
}

fn try_parse_global_stat(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::GlobalStat);
    match p.peek_next_token() {
        PyTokenKind::TkName => {
            p.set_current_token_kind(PyTokenKind::TkGlobal);
            p.bump();
            parse_variable_name_list(p, true)?;
        }
        PyTokenKind::TkLt => {
            p.set_current_token_kind(PyTokenKind::TkGlobal);
            p.bump();
            parse_attrib(p)?;
            parse_variable_name_list(p, true)?;
        }
        _ => {
            return Ok(m.undo(p));
        }
    }

    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_assign_or_expr_or_global_stat(p: &mut LuaParser) -> ParseResult {
    if p.parse_config.level >= LuaLanguageLevel::Lua55 && p.current_token() == PyTokenKind::TkName
    {
        let token_text = p.current_token_text();
        if token_text == "global" {
            let cm = try_parse_global_stat(p)?;
            if !cm.is_invalid() {
                return Ok(cm);
            }
        }
    }

    let mut m = p.mark(PySyntaxKind::AssignStat);
    let range = p.current_token_range();

    // 解析第一个表达式
    let cm = match parse_expr(p) {
        Ok(cm) => cm,
        Err(err) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected expression in assignment or statement"),
                range,
            ));
            return Err(err);
        }
    };

    // 检查是否是函数调用语句
    if matches!(
        cm.kind,
        PySyntaxKind::CallExpr
            | PySyntaxKind::AssertCallExpr
            | PySyntaxKind::ErrorCallExpr
            | PySyntaxKind::RequireCallExpr
            | PySyntaxKind::TypeCallExpr
            | PySyntaxKind::SetmetatableCallExpr
    ) {
        m.set_kind(p, PySyntaxKind::CallExprStat);
        if_token_bump(p, PyTokenKind::TkSemicolon);
        return Ok(m.complete(p));
    }

    // 验证左值
    if !matches!(cm.kind, PySyntaxKind::NameExpr | PySyntaxKind::IndexExpr) {
        p.push_error(LuaParseError::syntax_error_from(
            &t!("invalid left-hand side in assignment (expected variable or table index)"),
            range,
        ));

        return Err(ParseFailReason::UnexpectedToken);
    }

    // 解析更多左值（如果有逗号）
    while p.current_token() == PyTokenKind::TkComma {
        p.bump();
        match parse_expr(p) {
            Ok(expr_cm) => {
                if !matches!(
                    expr_cm.kind,
                    PySyntaxKind::NameExpr | PySyntaxKind::IndexExpr
                ) {
                    p.push_error(LuaParseError::syntax_error_from(
                        &t!(
                            "invalid left-hand side in assignment (expected variable or table index)"
                        ),
                        p.current_token_range(),
                    ));
                    return Err(ParseFailReason::UnexpectedToken);
                }
            }
            Err(_) => {
                p.push_error(LuaParseError::syntax_error_from(
                    &t!("expected variable after ',' in assignment"),
                    p.current_token_range(),
                ));
            }
        }
    }

    // 期望赋值操作符
    if p.current_token().is_assign_op() {
        p.bump();

        // 解析右值表达式列表
        if parse_expr_list_impl(p).is_err() {
            push_expr_error_lazy(p, || t!("expected expression after '=' in assignment"));
        }
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            &t!("expected '=' for assignment or this is an incomplete statement"),
            p.current_token_range(),
        ));

        return Err(ParseFailReason::UnexpectedToken);
    }

    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

fn parse_label_stat(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::LabelStat);
    p.bump();
    match expect_token(p, PyTokenKind::TkName) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected label name after 'goto'"),
                p.current_token_range(),
            ));
        }
    }
    match expect_token(p, PyTokenKind::TkDbColon) {
        Ok(_) => {}
        Err(_) => {
            p.push_error(LuaParseError::syntax_error_from(
                &t!("expected '::' after label name"),
                p.current_token_range(),
            ));
        }
    }
    Ok(m.complete(p))
}
