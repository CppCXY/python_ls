use std::borrow::Cow;

use crate::{
    grammar::{ParseFailReason, ParseResult, py::is_statement_start_token},
    kind::{PySyntaxKind, PyTokenKind},
    parser::{MarkerEventContainer, PyParser},
    parser_error::PyParseError,
};

use super::expr::parse_expr;

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
    p.push_error(PyParseError::syntax_error_from(
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
        p.push_error(PyParseError::syntax_error_from(
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
        PyTokenKind::TkMatMul => parse_decorated(p)?, // @ can be TkMatMul in statement context
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

    // Handle body: either indented block or simple statement
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
        // After newline, expect indented block
        if p.current_token() == PyTokenKind::TkIndent {
            parse_suite(p)?;
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected indented block after ':'"),
                p.current_token_range(),
            ));
        }
    } else if p.current_token() == PyTokenKind::TkIndent {
        // Direct indented block
        parse_suite(p)?;
    } else {
        // Simple statement on same line
        if parse_stat(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected statement after ':'"),
                p.current_token_range(),
            ));
        }
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

    // Consume optional newline before indented block
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
    }

    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    Ok(m.complete(p))
}

fn parse_else_clause(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ElseClause);
    p.bump(); // consume 'else'

    expect_keyword_with_recovery(p, PyTokenKind::TkColon, || t!("expected ':' after 'else'"));

    // Consume optional newline before indented block
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
    }

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
    if !expect_keyword_with_recovery(p, PyTokenKind::TkColon, || {
        t!("expected ':' after while condition")
    }) {
        recover_to_keywords(p, &[PyTokenKind::TkNewline, PyTokenKind::TkIndent]);
    }

    // Consume optional newline after colon
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
    }

    // Parse suite (indented block)
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    } else {
        p.push_error(PyParseError::syntax_error_from(
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
            p.push_error(PyParseError::syntax_error_from(
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
        p.push_error(PyParseError::syntax_error_from(
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
        p.push_error(PyParseError::syntax_error_from(
            "expected ':' after for clause",
            p.current_token_range(),
        ));
    }

    // Consume optional newline after colon
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
    }

    // Parse suite (indented block)
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    } else {
        p.push_error(PyParseError::syntax_error_from(
            "expected indented block after ':'",
            p.current_token_range(),
        ));
    }
    Ok(())
}

fn parse_return(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::ReturnStmt);
    p.bump(); // consume 'return'

    // Optional return value
    if !matches!(
        p.current_token(),
        PyTokenKind::TkNewline | PyTokenKind::TkEof | PyTokenKind::TkDedent
    ) {
        if parse_expr(p).is_err() {
            push_expr_error_lazy(p, || t!("expected expression in return statement"));
        }
    }

    consume_statement_terminator(p);
    Ok(m.complete(p))
}

fn parse_break(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::BreakStmt);
    p.bump(); // consume 'break'
    consume_statement_terminator(p);
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
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected function name after 'def'"),
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Parameters
    if p.current_token() == PyTokenKind::TkLeftParen {
        let param_m = p.mark(PySyntaxKind::Parameters);
        p.smart_bump(); // consume '(' and track paren context

        // Parse parameters with full Python 3.8+ syntax support
        if p.current_token() != PyTokenKind::TkRightParen {
            parse_function_parameters(p);
        }

        if p.current_token() == PyTokenKind::TkRightParen {
            p.smart_bump(); // consume ')' and update paren context
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected ')' to close parameter list"),
                p.current_token_range(),
            ));
        }
        param_m.complete(p);
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected '(' after function name"),
            p.current_token_range(),
        ));
    }

    // Optional return type annotation
    if p.current_token() == PyTokenKind::TkArrow {
        p.bump(); // consume '->'
        let _return_annotation_m = p.mark(PySyntaxKind::TypeAnnotation);
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected return type annotation after '->'"),
                p.current_token_range(),
            ));
        }
        _return_annotation_m.complete(p);
    }

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected ':' after function signature"),
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Body
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump(); // consume newline
        if p.current_token() == PyTokenKind::TkIndent {
            parse_suite_with_docstring(p, true)?;
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected indented block after ':'"),
                p.current_token_range(),
            ));
        }
    } else if p.current_token() == PyTokenKind::TkIndent {
        // Direct indentation without newline (shouldn't happen normally)
        parse_suite_with_docstring(p, true)?;
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected indented block after ':'"),
            p.current_token_range(),
        ));
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
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected class name after 'class'"),
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Optional inheritance
    if p.current_token() == PyTokenKind::TkLeftParen {
        p.smart_bump(); // consume '('

        // Parse base classes
        if p.current_token() != PyTokenKind::TkRightParen {
            loop {
                if parse_expr(p).is_err() {
                    p.push_error(PyParseError::syntax_error_from(
                        &t!("expected base class expression"),
                        p.current_token_range(),
                    ));
                    break;
                }

                if p.current_token() == PyTokenKind::TkComma {
                    p.bump();
                    // Allow trailing comma
                    if p.current_token() == PyTokenKind::TkRightParen {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if p.current_token() == PyTokenKind::TkRightParen {
            p.smart_bump(); // consume ')'
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected ')' to close base class list"),
                p.current_token_range(),
            ));
        }
    }

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected ':' after class header"),
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Body
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump(); // consume newline
        if p.current_token() == PyTokenKind::TkIndent {
            parse_suite_with_docstring(p, true)?;
        } else {
            p.push_error(PyParseError::syntax_error_from(
                &t!("expected indented block after ':'"),
                p.current_token_range(),
            ));
        }
    } else if p.current_token() == PyTokenKind::TkIndent {
        parse_suite_with_docstring(p, true)?;
    } else {
        p.push_error(PyParseError::syntax_error_from(
            &t!("expected indented block after ':'"),
            p.current_token_range(),
        ));
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

    // Handle relative imports (starting with dots)
    while p.current_token() == PyTokenKind::TkDot {
        p.bump(); // consume '.'
    }

    // Module name (optional for relative imports)
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
    if p.current_token() == PyTokenKind::TkName
        || p.current_token() == PyTokenKind::TkLeftParen
        || p.current_token() == PyTokenKind::TkMul
    {
        if p.current_token() == PyTokenKind::TkLeftParen {
            p.bump();
        }

        // Handle wildcard import
        if p.current_token() == PyTokenKind::TkMul {
            p.bump(); // consume '*'
        } else {
            // Handle regular import list
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
    consume_statement_terminator(p);
    Ok(m.complete(p))
}

fn parse_pass(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::PassStmt);
    p.bump(); // consume 'pass'
    consume_statement_terminator(p);
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
            p.push_error(PyParseError::syntax_error_from(
                "expected exception after 'raise'",
                p.current_token_range(),
            ));
        }
    }

    consume_statement_terminator(p);
    Ok(m.complete(p))
}

fn parse_try(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::TryStmt);
    p.bump(); // consume 'try'

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    }

    // Consume optional newline after colon
    if p.current_token() == PyTokenKind::TkNewline {
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

        // Consume optional newline after colon
        if p.current_token() == PyTokenKind::TkNewline {
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

        // Consume optional newline after colon
        if p.current_token() == PyTokenKind::TkNewline {
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

    // Parse multiple context managers separated by commas
    loop {
        // Context expression
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                "expected context expression after 'with'",
                p.current_token_range(),
            ));
            return Err(ParseFailReason::UnexpectedToken);
        }

        // Optional 'as' variable
        if p.current_token() == PyTokenKind::TkAs {
            p.bump();
            if p.current_token() == PyTokenKind::TkName {
                p.bump();
            } else {
                p.push_error(PyParseError::syntax_error_from(
                    "expected variable name after 'as'",
                    p.current_token_range(),
                ));
            }
        }

        // Check for comma to continue with more context managers
        if p.current_token() == PyTokenKind::TkComma {
            p.bump(); // consume ','
        } else {
            break;
        }
    }

    // Colon
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            "expected ':' after with statement",
            p.current_token_range(),
        ));
    }

    // Consume optional newline after colon
    if p.current_token() == PyTokenKind::TkNewline {
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
        p.push_error(PyParseError::syntax_error_from(
            "expected test expression after 'assert'",
            p.current_token_range(),
        ));
    }

    // Optional message
    if p.current_token() == PyTokenKind::TkComma {
        p.bump();
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
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
        p.push_error(PyParseError::syntax_error_from(
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
            p.push_error(PyParseError::syntax_error_from(
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
            p.push_error(PyParseError::syntax_error_from(
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

    // Use the expression-level yield parsing by calling parse_expr
    // which will handle 'yield', 'yield from', etc.
    if parse_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            "expected yield expression",
            p.current_token_range(),
        ));
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
            p.push_error(PyParseError::syntax_error_from(
                "expected 'def', 'with', or 'for' after 'async'",
                p.current_token_range(),
            ));
            Err(ParseFailReason::UnexpectedToken)
        }
    }
}

fn parse_newline(p: &mut PyParser) -> ParseResult {
    // In Python, a standalone newline represents an empty statement
    let m = p.mark(PySyntaxKind::Newline);
    p.bump(); // consume newline
    Ok(m.complete(p))
}

/// Consume optional newline at the end of simple statements
fn consume_statement_terminator(p: &mut PyParser) {
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
    }
}

fn parse_assign_or_expr_stat(p: &mut PyParser) -> ParseResult {
    // Check for type annotation pattern first: NAME ':' TYPE ['=' EXPR]
    if p.current_token() == PyTokenKind::TkName && p.peek_next_token() == PyTokenKind::TkColon {
        let m = p.mark(PySyntaxKind::AnnAssignStmt);

        // Parse name
        let name_marker = p.mark(PySyntaxKind::NameExpr);
        p.bump(); // consume name
        name_marker.complete(p);

        // Consume ':'
        p.bump(); // consume ':'

        // Parse type annotation
        if super::expr::parse_single_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                "expected type annotation after ':'",
                p.current_token_range(),
            ));
        }

        // Optional assignment value
        if p.current_token() == PyTokenKind::TkAssign {
            p.bump(); // consume '='

            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    "expected expression after '='",
                    p.current_token_range(),
                ));
            }
        }

        consume_statement_terminator(p);
        return Ok(m.complete(p));
    }

    let mut m = p.mark(PySyntaxKind::ExprStmt);

    // Parse the expression first (normal case)
    if parse_expr(p).is_err() {
        p.push_error(PyParseError::syntax_error_from(
            "expected expression",
            p.current_token_range(),
        ));
        return Err(ParseFailReason::UnexpectedToken);
    }

    // Check for assignment operators
    match p.current_token() {
        // Simple assignment
        PyTokenKind::TkAssign => {
            m.set_kind(p, PySyntaxKind::AssignStmt);
            p.bump(); // consume '='

            if parse_expr(p).is_err() {
                p.push_error(PyParseError::syntax_error_from(
                    "expected expression after '='",
                    p.current_token_range(),
                ));
            }
        }
        // Augmented assignments
        PyTokenKind::TkPlusAssign
        | PyTokenKind::TkMinusAssign
        | PyTokenKind::TkMulAssign
        | PyTokenKind::TkDivAssign
        | PyTokenKind::TkFloorDivAssign
        | PyTokenKind::TkModAssign
        | PyTokenKind::TkPowAssign
        | PyTokenKind::TkMatMulAssign
        | PyTokenKind::TkBitAndAssign
        | PyTokenKind::TkBitOrAssign
        | PyTokenKind::TkBitXorAssign
        | PyTokenKind::TkShlAssign
        | PyTokenKind::TkShrAssign => {
            m.set_kind(p, PySyntaxKind::AugAssignStmt);
            let op_token = p.current_token();
            p.bump(); // consume augmented assignment operator

            if parse_expr(p).is_err() {
                let op_str = format!("{:?}", op_token)
                    .replace("Tk", "")
                    .replace("Assign", "=");
                p.push_error(PyParseError::syntax_error_from(
                    &t!("expected expression after '{op}'", op = op_str),
                    p.current_token_range(),
                ));
            }
        }
        _ => {
            // Just an expression statement, keep ExprStmt kind
        }
    }

    consume_statement_terminator(p);
    Ok(m.complete(p))
}

// Parse decorators
fn parse_decorators(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::Decorators);

    while p.current_token() == PyTokenKind::TkAt || p.current_token() == PyTokenKind::TkMatMul {
        let decorator_m = p.mark(PySyntaxKind::Decorator);
        p.bump(); // consume '@' (TkAt or TkMatMul)

        // Parse the decorator expression
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
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

    // Skip any newlines between decorators and definition
    while p.current_token() == PyTokenKind::TkNewline {
        p.bump();
    }

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
                p.push_error(PyParseError::syntax_error_from(
                    "expected 'def' after 'async' in decorated function",
                    p.current_token_range(),
                ));
                Err(ParseFailReason::UnexpectedToken)
            }
        }
        _ => {
            p.push_error(PyParseError::syntax_error_from(
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

/// Parse function parameters with full Python 3.8+ syntax support:
/// def func(pos_only, /, regular, *args, kw_only, **kwargs):
fn parse_function_parameters(p: &mut PyParser) {
    let mut has_pos_separator = false;
    let mut has_star = false;
    let mut has_double_star = false;

    loop {
        match p.current_token() {
            // Position-only separator /
            PyTokenKind::TkDiv => {
                if has_pos_separator {
                    p.push_error(PyParseError::syntax_error_from(
                        "multiple '/' separators not allowed",
                        p.current_token_range(),
                    ));
                }
                has_pos_separator = true;
                p.bump(); // consume '/'
            }

            // *args or * (for keyword-only parameters)
            PyTokenKind::TkMul => {
                if has_star {
                    p.push_error(PyParseError::syntax_error_from(
                        "multiple '*' not allowed",
                        p.current_token_range(),
                    ));
                }
                has_star = true;

                let param_m = p.mark(PySyntaxKind::Parameter);
                p.bump(); // consume '*'

                // Check if this is *args (has a name) or just * (keyword-only separator)
                if p.current_token() == PyTokenKind::TkName {
                    p.bump(); // consume parameter name
                }

                param_m.complete(p);
            }

            // **kwargs
            PyTokenKind::TkPow => {
                if has_double_star {
                    p.push_error(PyParseError::syntax_error_from(
                        "multiple '**' not allowed",
                        p.current_token_range(),
                    ));
                }
                has_double_star = true;

                let param_m = p.mark(PySyntaxKind::Parameter);
                p.bump(); // consume '**'

                if p.current_token() == PyTokenKind::TkName {
                    p.bump(); // consume parameter name
                } else {
                    p.push_error(PyParseError::syntax_error_from(
                        "expected parameter name after '**'",
                        p.current_token_range(),
                    ));
                }

                param_m.complete(p);
            }

            // Regular parameter (positional, positional-only, or keyword-only)
            PyTokenKind::TkName => {
                let param_m = p.mark(PySyntaxKind::Parameter);
                p.bump(); // consume parameter name

                // Optional type annotation
                if p.current_token() == PyTokenKind::TkColon {
                    p.bump(); // consume ':'
                    let annotation_m = p.mark(PySyntaxKind::TypeAnnotation);
                    if super::expr::parse_single_expr(p).is_err() {
                        p.push_error(PyParseError::syntax_error_from(
                            "expected type annotation after ':'",
                            p.current_token_range(),
                        ));
                    }
                    annotation_m.complete(p);
                }

                // Optional default value
                if p.current_token() == PyTokenKind::TkAssign {
                    p.bump(); // consume '='
                    if super::expr::parse_single_expr(p).is_err() {
                        p.push_error(PyParseError::syntax_error_from(
                            "expected default value after '='",
                            p.current_token_range(),
                        ));
                    }
                }

                param_m.complete(p);
            }

            _ => break,
        }

        // Handle comma separation
        if p.current_token() == PyTokenKind::TkComma {
            p.bump();
            // Allow trailing comma
            if p.current_token() == PyTokenKind::TkRightParen {
                break;
            }
        } else {
            break;
        }
    }
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
        p.push_error(PyParseError::syntax_error_from(
            "expected expression after 'match'",
            p.current_token_range(),
        ));
    }

    // Expect ':'
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            "expected ':' after match expression",
            p.current_token_range(),
        ));
    }

    // Handle newline after ':'
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
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
        p.push_error(PyParseError::syntax_error_from(
            "expected pattern after 'case'",
            p.current_token_range(),
        ));
    }

    // Optional guard (if clause)
    if p.current_token() == PyTokenKind::TkIf {
        p.bump();
        if parse_expr(p).is_err() {
            p.push_error(PyParseError::syntax_error_from(
                "expected expression after 'if' in case guard",
                p.current_token_range(),
            ));
        }
    }

    // Expect ':'
    if p.current_token() == PyTokenKind::TkColon {
        p.bump();
    } else {
        p.push_error(PyParseError::syntax_error_from(
            "expected ':' after case pattern",
            p.current_token_range(),
        ));
    }

    // Handle newline after ':'
    if p.current_token() == PyTokenKind::TkNewline {
        p.bump();
    }

    // Parse suite
    if p.current_token() == PyTokenKind::TkIndent {
        parse_suite(p)?;
    }

    Ok(m.complete(p))
}
