mod expr;
mod stat;
mod test;
use stat::parse_stats;

use crate::{
    kind::{PySyntaxKind, PyTokenKind},
    parser::{MarkerEventContainer, PyParser},
    parser_error::PyParseError,
};

pub fn parse_module_suite(p: &mut PyParser) {
    let m = p.mark(PySyntaxKind::Suite);

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

            p.push_error(PyParseError::syntax_error_from(&error_msg, error_range));

            p.bump(); // Consume current token to avoid infinite loop
            m.complete(p);
        }
    }

    m.complete(p);
}

fn if_token_bump(p: &mut PyParser, token: PyTokenKind) -> bool {
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
            | PyTokenKind::TkMatch
            | PyTokenKind::TkType
            | PyTokenKind::TkAt // decorators
    )
}
