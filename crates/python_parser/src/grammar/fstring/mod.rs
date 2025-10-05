use crate::{
    FStringLexer, FStringToken, PyParser, PySyntaxKind, PyTokenKind, grammar::ParseResult,
    parser::MarkerEventContainer,
};

/// Parse f-string expression with embedded expressions
pub fn parse_fstring_expr(p: &mut PyParser) -> ParseResult {
    let m = p.mark(PySyntaxKind::FStringExpr);

    // Get the f-string token text to parse its content
    let fstring_text = p.current_token_text().to_string();
    let current_range = p.current_token_range();

    // Use FStringLexer to parse the f-string content
    let mut fstring_lexer = FStringLexer::new(&fstring_text, Some(current_range));
    let tokens = fstring_lexer.tokenize();
    let mut errors = fstring_lexer.get_errors();

    // Process the tokens and create appropriate syntax nodes
    for token in tokens {
        match token {
            FStringToken::Text(range) => {
                // Create a text part node
                p.eat_token(PyTokenKind::TkFStringText, range);
            }
            FStringToken::ExprStart(range) => {
                p.eat_token(PyTokenKind::TkFStringExprStart, range);
            }
            FStringToken::Expr(range) => {
                // Handle Python expression inside f-string
                let text = &p.source_text()[range.start_offset..range.end_offset()];
                let events = PyParser::parse_sub_expression(text, range, p.parse_config(), &mut errors);
                p.add_events(events);
            }
            FStringToken::FormatSpec(range) => {
                p.eat_token(PyTokenKind::TkFStringFormatSpec, range);
            }
            FStringToken::ConversionSpec(range) => {
                // Handle conversion specifiers (!s, !r, !a)
                p.eat_token(PyTokenKind::TkFStringConversion, range);
            }
            FStringToken::ExprEnd(range) => {
                p.eat_token(PyTokenKind::TkFStringExprEnd, range);
            }
        }
    }

    // Add any lexer errors to the parser
    for error in errors {
        p.push_error(error);
    }

    p.skip_bump();

    Ok(m.complete(p))
}
