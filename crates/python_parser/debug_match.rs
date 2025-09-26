fn main() {
    use crate::parser::PyParser;
    let code = "match value:\n    case 1:\n        pass";
    let tree = PyParser::parse(code, crate::ParserConfig::default());
    println!("{:#?}", tree.get_errors());
    println!("{:#?}", tree.get_red_root());
}
