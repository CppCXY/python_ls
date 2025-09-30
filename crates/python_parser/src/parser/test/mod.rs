mod multi_empty_lines_tests;
mod fstring_test;

#[cfg(test)]
use crate::{ParserConfig, PyParser};
#[cfg(test)]
pub fn print_ast_code(code: &str) {
    let config = ParserConfig::default();
    let tree = PyParser::parse(code, config);
    println!("{:#?}", tree.get_red_root());

    // Check for errors
    let errors = tree.get_errors();
    if !errors.is_empty() {
        println!("Errors found:");
        for error in errors {
            println!("  {:?}", error);
        }
    } else {
        println!("No errors found");
    }
}
