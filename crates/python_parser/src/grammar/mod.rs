mod fstring;
mod py;

use crate::parser::CompleteMarker;
pub use py::{parse_fstring_inner_expr, parse_module_suite};

type ParseResult = Result<CompleteMarker, ParseFailReason>;

pub enum ParseFailReason {
    /// Parsing was stopped due to reaching the end of the file.
    #[allow(unused)]
    Eof,
    /// Parsing was stopped due to encountering an unexpected token.
    UnexpectedToken,
}
