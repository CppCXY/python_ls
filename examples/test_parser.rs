use python_parser::{
    grammar::py::parse_module,
    kind::PySyntaxKind,
    lexer::{PyLexer, LexerConfig},
    parser::{PyParser, ParserConfig},
    text::Reader,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <python_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    println!("Parsing file: {}", filename);
    println!("Content:\n{}", content);
    println!("\n" + &"=".repeat(50));

    // Tokenize
    let reader = Reader::new(&content);
    let config = LexerConfig::default();
    let mut errors = Vec::new();
    let mut lexer = PyLexer::new(reader, config, Some(&mut errors));
    let tokens = lexer.tokenize();
    
    println!("Tokens found: {}", tokens.len());
    for (i, token) in tokens.iter().take(20).enumerate() {
        let start = token.range.start_offset;
        let end = start + token.range.length;
        let text = &content[start..end];
        println!("  {}: {:?} = {:?}", i, token.kind, text);
    }
    
    if !errors.is_empty() {
        println!("\nLexer errors:");
        for error in &errors {
            println!("  {:?}", error);
        }
    }

    // Parse
    let mut parse_errors = Vec::new();
    let mut parser = PyParser::new(&content, tokens, ParserConfig::default(), &mut parse_errors);
    
    let syntax_tree = parse_module(&mut parser);
    
    println!("\nSyntax tree:");
    println!("{:#?}", syntax_tree);
    
    if !parse_errors.is_empty() {
        println!("\nParse errors:");
        for error in &parse_errors {
            println!("  {:?}", error);
        }
    }
}