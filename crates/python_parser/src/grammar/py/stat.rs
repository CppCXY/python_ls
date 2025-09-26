use std::borrow::Cow;

use crate::{
    grammar::{ParseFailReason, ParseResult, py::is_statement_start_token},
    kind::{PySyntaxKind, PyTokenKind},
    parser::{MarkerEventContainer, PyParser},
    parser_error::LuaParseError,
};

use super::{expr::parse_expr, if_token_bump};

// Parse an indented block (suite in Python grammar)
fn parse_suite(p: &mut PyParser) -> ParseResult {
    parse_suite_with_docstring(p, false)
}

// Parse an indented block with optional docstring detection
fn parse_suite_with_docstring(p: &mut PyParser, expect_docstring: bool) -> ParseResult {
    let m = p.mark(PySyntaxKind::Suite);

    // Expect INDENT
    if p.current_token() == PyTokenKind::TkIndent {
        p.bump();
    }

    // Check for docstring if expected (first string literal)
    if expect_docstring && p.current_token() == PyTokenKind::TkString {
        let docstring_m = p.mark(PySyntaxKind::Docstring);
        p.bump(); // consume string literal
        docstring_m.complete(p);
        
        // Consume newline after docstring
        if p.current_token() == PyTokenKind::TkNewline {
            p.bump();
        }
    }

    // Parse statements until DEDENT
    while !matches!(
        p.current_token(),
        PyTokenKind::TkDedent | PyTokenKind::TkEof
    ) {
        parse_stat(p)?;
    }

    // Expect DEDENT
    if p.current_token() == PyTokenKind::TkDedent {
        p.bump();
    }

    Ok(m.complete(p))
}

/// Push expression parsing error with lazy error message generation
fn push_expr_error_lazy<F>(p: &mut PyParser, error_msg_fn: F)
where
    F: FnOnce() -> Cow<'static, str>,
{
    let error_msg = error_msg_fn();
    p.push_error(LuaParseError::syntax_error_from(
        &error_msg,
        p.current_token_range(),
    ));
}

/// Generic keyword expectation with error recovery and lazy error message generation
fn expect_keyword_with_recovery<F>(p: &mut PyParser, expected: PyTokenKind, error_msg_fn: F) -> bool
where
    F: FnOnce() -> Cow<'static, str>,
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

/// Error recovery: skip to specified keywords
fn recover_to_keywords(p: &mut PyParser, keywords: &[PyTokenKind]) {
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
fn parse_expr_list_impl(p: &mut PyParser) -> Result<(), &'static str> {
    parse_expr(p).map_err(|_| "expected expression")?;

    while p.current_token() == PyTokenKind::TkComma {
        p.bump();
        parse_expr(p).map_err(|_| "expected expression after ','")?;
    }

    Ok(())
}

pub fn parse_stats(p: &mut PyParser) {
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

fn block_follow(p: &PyParser) -> bool {
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

fn parse_stat(p: &mut PyParser) -> ParseResult {
    let cm = match p.current_token() {
        // Python keywords
        PyTokenKind::TkIf => parse_if(p)?,
        PyTokenKind::TkWhile => parse_while(p)?,
        PyTokenKind::TkFor => parse_for(p)?,
        PyTokenKind::TkDef => parse_def_with_decorators(p)?,
        PyTokenKind::TkClass => parse_class_with_decorators(p)?,
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
        PyTokenKind::TkMatch => parse_match(p)?,
        PyTokenKind::TkAt => parse_decorated(p)?,
        PyTokenKind::TkNewline => parse_newline(p)?,
        _ => parse_assign_or_expr_stat(p)?,
    };

    Ok(cm)
}

fn parse_if(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::IfStmt);
    p.bump(); // consume 'if'

    // Parse condition expression
    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || t!("expected condition expression after 'if'"));
        recover_to_keywords(
            p,
            &[
                PyTokenKind::TkColon,
                PyTokenKind::TkElif,
                PyTokenKind::TkElse,
            ],
        );
    }

    // Expect ':'
    if !expect_keyword_with_recovery(p, PyTokenKind::TkColon, || {
        t!("expected ':' after if condition")
    }) {
        recover_to_keywords(
            p,
            &[
                PyTokenKind::TkElif,
                PyTokenKind::TkElse,
                PyTokenKind::TkIndent,
            ],
        );
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

fn parse_elif_clause(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ElifClause);
    p.bump(); // consume 'elif'

    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || t!("expected condition expression after 'elif'"));
    }

    expect_keyword_with_recovery(p, PyTokenKind::TkColon, || {
        t!("expected ':' after 'elif' condition")
    });

    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    Ok(m.complete(p))
}

fn parse_else_clause(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ElseClause);
    p.bump(); // consume 'else'

    expect_keyword_with_recovery(p, PyTokenKind::TkColon, || t!("expected ':' after 'else'"));

    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    Ok(m.complete(p))
}

fn parse_while(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::WhileStmt);
    p.bump(); // consume 'while'

    // Parse condition expression
    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || t!("expected condition expression after 'while'"));
        recover_to_keywords(p, &[PyTokenKind::TkColon]);
    }

    // Expect ':'
    if !expect_keyword_with_recovery(
        p,
        PyTokenKind::TkColon,
        || t!("expected ':' after while condition"),
    ) {
        recover_to_keywords(p, &[PyTokenKind::TkIndent]);
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
    Ok(m.complete(p))
}

fn parse_for(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ForStmt);
    parse_for_body(p)?;
    Ok(m.complete(p))
}

fn parse_for_body(p: &mut PyParser) -> Result<(), ParseFailReason> {
    p.bump(); // consume 'for'

    // Parse target list (can be multiple variables)
    loop {
        if p.current_token() == PyTokenKind::TkName {
            p.bump();
        } else {
            p.push_error(LuaParseError::syntax_error_from(
                "expected variable name after 'for'",
                p.current_token_range(),
            ));
            break;
        }

        if p.current_token() == PyTokenKind::TkComma {
            p.bump();
        } else {
            break;
        }
    }

    // Expect 'in'
    if p.current_token() == PyTokenKind::TkIn {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            "expected 'in' after variable list in for loop",
            p.current_token_range(),
        ));
    }

    // Parse iterable expression
    if parse_expr(p).is_err() {
        push_expr_error_lazy(p, || t!("expected iterable expression after 'in'"));
    }

    // Expect ':'
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            "expected ':' after for clause",
            p.current_token_range(),
        ));
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
    Ok(())
}

fn parse_return(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ReturnStmt);
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

fn parse_break(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::BreakStmt);
    p.bump();
    if_token_bump(p, PyTokenKind::TkSemicolon);
    Ok(m.complete(p))
}

// Python-specific statement parsing functions
fn parse_def(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::FuncDef);
    p.bump(); // consume 'def'

    // Function name
    if p.current_token() == PyTokenKind::TkName {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            "expected function name after 'def'",
            p.current_token_range(),
        ));
    }

    // Parameters
    if p.current_token() == PyTokenKind::TkLeftParen {
        let param_m = p.mark(PySyntaxKind::Parameters);
        p.bump(); // consume '('

        // Parse parameters if any
        if p.current_token() != PyTokenKind::TkRightParen {
            loop {
                if p.current_token() == PyTokenKind::TkName {
                    let single_param_m = p.mark(PySyntaxKind::Parameter);
                    p.bump();
                    single_param_m.complete(p);
                } else {
                    break;
                }

                if p.current_token() == PyTokenKind::TkComma {
                    p.bump();
                } else {
                    break;
                }
            }
        }

        if p.current_token() == PyTokenKind::TkRightParen {
            p.bump();
        }
        param_m.complete(p);
    }

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    }

    // Body
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite_with_docstring(p, true)?;
    }

    Ok(m.complete(p))
}

fn parse_class(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ClassDef);
    p.bump(); // consume 'class'

    // Class name
    if p.current_token() == PyTokenKind::TkName {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            "expected class name after 'class'",
            p.current_token_range(),
        ));
    }

    // Optional inheritance
    if p.current_token() == PyTokenKind::TkLeftParen {
        p.bump();
        // Parse base classes
        if p.current_token() != PyTokenKind::TkRightParen {
            loop {
                if parse_expr(p).is_err() {
                    break;
                }
                if p.current_token() == PyTokenKind::TkComma {
                    p.bump();
                } else {
                    break;
                }
            }
        }
        if p.current_token() == PyTokenKind::TkRightParen {
            p.bump();
        }
    }

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    }

    // Body
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite_with_docstring(p, true)?;
    }

    Ok(m.complete(p))
}

fn parse_import(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ImportStmt);
    p.bump(); // consume 'import'

    // Parse module names
    loop {
        if p.current_token() == PyTokenKind::TkName {
            p.bump();

            // Handle dotted names
            while p.current_token() == PyTokenKind::TkDot {
                p.bump();
                if p.current_token() == PyTokenKind::TkName {
                    p.bump();
                } else {
                    break;
                }
            }

            // Handle 'as' alias
            if p.current_token() == PyTokenKind::TkAs {
                p.bump();
                if p.current_token() == PyTokenKind::TkName {
                    p.bump();
                }
            }
        } else {
            break;
        }

        if p.current_token() == PyTokenKind::TkComma {
            p.bump();
        } else {
            break;
        }
    }

    Ok(m.complete(p))
}

fn parse_from_import(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ImportFromStmt);
    p.bump(); // consume 'from'

    // Module name
    if p.current_token() == PyTokenKind::TkName {
        p.bump();

        // Handle dotted names
        while p.current_token() == PyTokenKind::TkDot {
            p.bump();
            if p.current_token() == PyTokenKind::TkName {
                p.bump();
            } else {
                break;
            }
        }
    }

    // 'import'
    if p.current_token() == PyTokenKind::TkImport {
        p.bump();
    }

    // Import list
    if p.current_token() == PyTokenKind::TkName || p.current_token() == PyTokenKind::TkLeftParen {
        if p.current_token() == PyTokenKind::TkLeftParen {
            p.bump();
        }

        loop {
            if p.current_token() == PyTokenKind::TkName {
                p.bump();

                // Handle 'as' alias
                if p.current_token() == PyTokenKind::TkAs {
                    p.bump();
                    if p.current_token() == PyTokenKind::TkName {
                        p.bump();
                    }
                }
            } else {
                break;
            }

            if p.current_token() == PyTokenKind::TkComma {
                p.bump();
            } else {
                break;
            }
        }

        if p.current_token() == PyTokenKind::TkRightParen {
            p.bump();
        }
    }

    Ok(m.complete(p))
}

fn parse_continue(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ContinueStmt);
    p.bump(); // consume 'continue'
    Ok(m.complete(p))
}

fn parse_pass(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::PassStmt);
    p.bump(); // consume 'pass'
    Ok(m.complete(p))
}

fn parse_raise(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::RaiseStmt);
    p.bump(); // consume 'raise'

    // Optional exception
    if !matches!(
        p.current_token(),
        PyTokenKind::TkNewline | PyTokenKind::TkEof
    ) {
        if parse_expr(p).is_err() {
            p.push_error(LuaParseError::syntax_error_from(
                "expected exception after 'raise'",
                p.current_token_range(),
            ));
        }
    }

    Ok(m.complete(p))
}

fn parse_try(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::TryStmt);
    p.bump(); // consume 'try'

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    }

    // Body
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    // Except clauses
    while p.current_token() == PyTokenKind::TkExcept {
        let except_m = p.mark(PySyntaxKind::ExceptClause);
        p.bump(); // consume 'except'

        // Optional exception type
        if p.current_token() != PyTokenKind::TkColon {
            if parse_expr(p).is_ok() {
                // Optional 'as' name
                if p.current_token() == PyTokenKind::TkAs {
                    p.bump();
                    if p.current_token() == PyTokenKind::TkName {
                        p.bump();
                    }
                }
            }
        }

        if p.current_token() == PyTokenKind::TkColon {
            p.bump();
        }

        if p.current_token() == PyTokenKind::TkIndent {
            parse_suite(p)?;
        }

        except_m.complete(p);
    }

    // Optional finally
    if p.current_token() == PyTokenKind::TkFinally {
        let finally_m = p.mark(PySyntaxKind::FinallyClause);
        p.bump(); // consume 'finally'

        if p.current_token() == PyTokenKind::TkColon {
            p.bump();
        }

        if p.current_token() == PyTokenKind::TkIndent {
            parse_suite(p)?;
        }

        finally_m.complete(p);
    }

    Ok(m.complete(p))
}

fn parse_with(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::WithStmt);
    parse_with_body(p)?;
    Ok(m.complete(p))
}

fn parse_with_body(p: &mut PyParser) -> Result<(), ParseFailReason> {
    p.bump(); // consume 'with'

    // Context expression
    if parse_expr(p).is_err() {
        p.push_error(LuaParseError::syntax_error_from(
            "expected context expression after 'with'",
            p.current_token_range(),
        ));
    }

    // Optional 'as' variable
    if p.current_token() == PyTokenKind::TkAs {
        p.bump();
        if p.current_token() == PyTokenKind::TkName {
            p.bump();
        }
    }

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    }

    // Body
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    Ok(())
}

fn parse_assert(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::AssertStmt);
    p.bump(); // consume 'assert'

    // Test expression
    if parse_expr(p).is_err() {
        p.push_error(LuaParseError::syntax_error_from(
            "expected test expression after 'assert'",
            p.current_token_range(),
        ));
    }

    // Optional message
    if p.current_token() == PyTokenKind::TkComma {
        p.bump();
        if parse_expr(p).is_err() {
            p.push_error(LuaParseError::syntax_error_from(
                "expected message expression after ','",
                p.current_token_range(),
            ));
        }
    }

    Ok(m.complete(p))
}

fn parse_del(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::DeleteStmt);
    p.bump(); // consume 'del'

    // Target list
    if parse_expr(p).is_err() {
        p.push_error(LuaParseError::syntax_error_from(
            "expected target after 'del'",
            p.current_token_range(),
        ));
    }

    Ok(m.complete(p))
}

fn parse_global(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::GlobalStmt);
    p.bump(); // consume 'global'

    // Variable names
    loop {
        if p.current_token() == PyTokenKind::TkName {
            p.bump();
        } else {
            p.push_error(LuaParseError::syntax_error_from(
                "expected variable name",
                p.current_token_range(),
            ));
            break;
        }

        if p.current_token() == PyTokenKind::TkComma {
            p.bump();
        } else {
            break;
        }
    }

    Ok(m.complete(p))
}

fn parse_nonlocal(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::NonlocalStmt);
    p.bump(); // consume 'nonlocal'

    // Variable names
    loop {
        if p.current_token() == PyTokenKind::TkName {
            p.bump();
        } else {
            p.push_error(LuaParseError::syntax_error_from(
                "expected variable name",
                p.current_token_range(),
            ));
            break;
        }

        if p.current_token() == PyTokenKind::TkComma {
            p.bump();
        } else {
            break;
        }
    }

    Ok(m.complete(p))
}

fn parse_yield_stmt(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::YieldStmt);
    p.bump(); // consume 'yield'

    // Optional expression
    if !matches!(
        p.current_token(),
        PyTokenKind::TkNewline | PyTokenKind::TkEof
    ) {
        if parse_expr(p).is_err() {
            p.push_error(LuaParseError::syntax_error_from(
                "expected expression after 'yield'",
                p.current_token_range(),
            ));
        }
    }

    Ok(m.complete(p))
}

fn parse_async_stmt(p: &mut PyParser) -> ParseResult {
    p.bump(); // consume 'async'

    match p.current_token() {
        PyTokenKind::TkDef => {
            let m = p.mark(PySyntaxKind::AsyncFuncDef);
            parse_def(p)?;
            Ok(m.complete(p))
        }
        PyTokenKind::TkWith => {
            let m = p.mark(PySyntaxKind::AsyncWithStmt);
            parse_with_body(p)?;
            Ok(m.complete(p))
        }
        PyTokenKind::TkFor => {
            let m = p.mark(PySyntaxKind::AsyncForStmt);
            parse_for_body(p)?;
            Ok(m.complete(p))
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                "expected 'def', 'with', or 'for' after 'async'",
                p.current_token_range(),
            ));
            Err(ParseFailReason::UnexpectedToken)
        }
    }
}

fn parse_newline(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ExprStmt);
    p.bump(); // consume newline
    Ok(m.complete(p))
}

fn parse_assign_or_expr_stat(p: &mut PyParser) -> ParseResult {
    let mut m = p.mark(PySyntaxKind::AssignStmt);

    // Parse expression
    if parse_expr(p).is_err() {
        p.push_error(LuaParseError::syntax_error_from(
            "expected expression",
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Check for assignment
    if matches!(p.current_token(), PyTokenKind::TkAssign) {
        p.bump(); // consume assignment operator

        if parse_expr(p).is_err() {
            p.push_error(LuaParseError::syntax_error_from(
                "expected expression after assignment",
                p.current_token_range(),
            ));
        }

        Ok(m.complete(p))
    } else {
        // Just an expression statement
        m.set_kind(p, PySyntaxKind::ExprStmt);
        Ok(m.complete(p))
    }
}

// Parse decorators
fn parse_decorators(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::Decorators);
    
    while p.current_token() == PyTokenKind::TkAt {
        let decorator_m = p.mark(PySyntaxKind::Decorator);
        p.bump(); // consume '@'
        
        // Parse the decorator expression
        if parse_expr(p).is_err() {
            p.push_error(LuaParseError::syntax_error_from(
                "expected expression after '@'",
                p.current_token_range(),
            ));
        }
        
        // Expect newline
        if p.current_token() == PyTokenKind::TkNewline {
            p.bump();
        }
        
        decorator_m.complete(p);
    }
    
    Ok(m.complete(p))
}

// Parse decorated function or class
fn parse_decorated(p: &mut PyParser) -> ParseResult {
    parse_decorators(p)?;
    
    match p.current_token() {
        PyTokenKind::TkDef => parse_def(p),
        PyTokenKind::TkClass => parse_class(p),
        PyTokenKind::TkAsync => {
            p.bump(); // consume 'async'
            if p.current_token() == PyTokenKind::TkDef {
                let m = p.mark(PySyntaxKind::AsyncFuncDef);
                parse_def(p)?;
                Ok(m.complete(p))
            } else {
                p.push_error(LuaParseError::syntax_error_from(
                    "expected 'def' after 'async' in decorated function",
                    p.current_token_range(),
                ));
                Err(ParseFailReason::UnexpectedToken)
            }
        }
        _ => {
            p.push_error(LuaParseError::syntax_error_from(
                "expected function or class definition after decorator(s)",
                p.current_token_range(),
            ));
            Err(ParseFailReason::UnexpectedToken)
        }
    }
}

// Parse function definition with optional decorators
fn parse_def_with_decorators(p: &mut PyParser) -> ParseResult {
    parse_def(p)
}

// Parse class definition with optional decorators  
fn parse_class_with_decorators(p: &mut PyParser) -> ParseResult {
    parse_class(p)
}

// Parse match statement (Python 3.10+)
fn parse_match(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::MatchStmt);
    p.bump(); // consume 'match'
    
    // Parse the subject expression
    if parse_expr(p).is_err() {
        p.push_error(LuaParseError::syntax_error_from(
            "expected expression after 'match'",
            p.current_token_range(),
        ));
    }
    
    // Expect ':'
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            "expected ':' after match expression",
            p.current_token_range(),
        ));
    }
    
    // Parse case clauses
    if p.current_token() == PyTokenKind::TkIndent {
        p.bump();
        
        while p.current_token() == PyTokenKind::TkCase {
            parse_case_clause(p)?;
        }
        
        if p.current_token() == PyTokenKind::TkDedent {
            p.bump();
        }
    }
    
    Ok(m.complete(p))
}

// Parse case clause
fn parse_case_clause(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::CaseClause);
    p.bump(); // consume 'case'
    
    // Parse pattern
    if parse_expr(p).is_err() {
        p.push_error(LuaParseError::syntax_error_from(
            "expected pattern after 'case'",
            p.current_token_range(),
        ));
    }
    
    // Optional guard (if clause)
    if p.current_token() == PyTokenKind::TkIf {
        p.bump();
        if parse_expr(p).is_err() {
            p.push_error(LuaParseError::syntax_error_from(
                "expected expression after 'if' in case guard",
                p.current_token_range(),
            ));
        }
    }
    
    // Expect ':'
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(LuaParseError::syntax_error_from(
            "expected ':' after case pattern",
            p.current_token_range(),
        ));
    }
    
    // Parse suite
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }
    
    Ok(m.complete(p))
}
