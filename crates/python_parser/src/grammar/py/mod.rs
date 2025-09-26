mod expr;
mod stat;
use stat::parse_stats;

use crate::{
    grammar::ParseFailReason,
    kind::{PySyntaxKind, PyTokenKind},
    parser::{LuaParser, MarkerEventContainer},
    parser_error::LuaParseError,
};

use super::ParseResult;

pub fn parse_module(p: &mut LuaParser) {
    let m = p.mark(PySyntaxKind::Module);

    p.init();
    while p.current_token() != PyTokenKind::TkEof {
        let consume_count = p.current_token_index();
        parse_stats(p);

        // Check if no token was consumed to prevent infinite loop
        if p.current_token_index() == consume_count {
            let error_range = p.current_token_range();
            let m = p.mark(PySyntaxKind::UnknownStat);

            // Provide more detailed error information
            let error_msg = match p.current_token() {
                PyTokenKind::TkRightBrace => {
                    t!("unexpected '}' - missing opening '{{' or extra closing brace")
                }
                PyTokenKind::TkRightParen => {
                    t!("unexpected ')' - missing opening '(' or extra closing parenthesis")
                }
                PyTokenKind::TkRightBracket => {
                    t!("unexpected ']' - missing opening '[' or extra closing bracket")
                }
                PyTokenKind::TkElse => {
                    t!("unexpected 'else' - missing corresponding 'if' statement")
                }
                _ => {
                    t!(
                        "unexpected token '%{token}' - expected statement",
                        token = p.current_token()
                    )
                }
            };

            p.push_error(LuaParseError::syntax_error_from(&error_msg, error_range));

            p.bump(); // Consume current token to avoid infinite loop
            m.complete(p);
        }
    }

    m.complete(p);
}

fn parse_suite(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::Suite);

    parse_stats(p);

    Ok(m.complete(p))
}

fn expect_token(p: &mut LuaParser, token: PyTokenKind) -> Result<(), ParseFailReason> {
    if p.current_token() == token {
        p.bump();
        Ok(())
    } else {
        if p.current_token() == PyTokenKind::TkEof {
            return Err(ParseFailReason::Eof);
        }

        Err(ParseFailReason::UnexpectedToken)
    }
}

fn if_token_bump(p: &mut LuaParser, token: PyTokenKind) -> bool {
    if p.current_token() == token {
        p.bump();
        true
    } else {
        false
    }
}

/// Check if a token is a statement start token
fn is_statement_start_token(token: PyTokenKind) -> bool {
    matches!(
        token,
        PyTokenKind::TkDef
            | PyTokenKind::TkClass
            | PyTokenKind::TkIf
            | PyTokenKind::TkFor
            | PyTokenKind::TkWhile
            | PyTokenKind::TkWith
            | PyTokenKind::TkTry
            | PyTokenKind::TkImport
            | PyTokenKind::TkFrom
            | PyTokenKind::TkName
            | PyTokenKind::TkReturn
            | PyTokenKind::TkBreak
            | PyTokenKind::TkContinue
            | PyTokenKind::TkPass
            | PyTokenKind::TkRaise
            | PyTokenKind::TkAssert
            | PyTokenKind::TkDel
            | PyTokenKind::TkGlobal
            | PyTokenKind::TkNonlocal
            | PyTokenKind::TkYield
            | PyTokenKind::TkAsync
    )
}
