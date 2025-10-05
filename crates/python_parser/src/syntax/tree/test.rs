#[cfg(test)]
mod test {
    use crate::{ParserConfig, PyParser};

    #[allow(unused)]
    fn print_ast_code(code: &str) {
        let config = ParserConfig::default();
        let tree = PyParser::parse(code, config, None);
        println!("{:#?}", tree.get_red_root());
    }

    macro_rules! assert_ast {
        ($code:expr, $expected:expr) => {
            let config = ParserConfig::default();
            let tree = PyParser::parse($code, config, None);
            let actual = format!("{:#?}", tree.get_red_root());
            assert_eq!(actual.trim(), $expected.trim());
        };
    }

    #[test]
    fn test_print_ast() {
        let code = r#"
def foo(x: int) -> int:
    return x + 1
"#;

        let expected = r#"
Syntax(Module)@0..42
  Syntax(Suite)@0..42
    Token(TkNewline)@0..1 "\n"
    Syntax(FuncDef)@1..42
      Token(TkDef)@1..4 "def"
      Token(TkWhitespace)@4..5 " "
      Token(TkName)@5..8 "foo"
      Syntax(Parameters)@8..16
        Token(TkLeftParen)@8..9 "("
        Syntax(Parameter)@9..15
          Token(TkName)@9..10 "x"
          Token(TkColon)@10..11 ":"
          Token(TkWhitespace)@11..12 " "
          Syntax(TypeAnnotation)@12..15
            Syntax(NameExpr)@12..15
              Token(TkName)@12..15 "int"
        Token(TkRightParen)@15..16 ")"
      Token(TkWhitespace)@16..17 " "
      Token(TkArrow)@17..19 "->"
      Token(TkWhitespace)@19..20 " "
      Syntax(TypeAnnotation)@20..23
        Syntax(NameExpr)@20..23
          Token(TkName)@20..23 "int"
      Token(TkColon)@23..24 ":"
      Syntax(Suite)@24..42
        Token(TkNewline)@24..25 "\n"
        Token(TkIndent)@25..29 "    "
        Syntax(ReturnStmt)@29..41
          Token(TkReturn)@29..35 "return"
          Token(TkWhitespace)@35..36 " "
          Syntax(BinaryExpr)@36..41
            Syntax(NameExpr)@36..37
              Token(TkName)@36..37 "x"
            Token(TkWhitespace)@37..38 " "
            Token(TkPlus)@38..39 "+"
            Token(TkWhitespace)@39..40 " "
            Syntax(LiteralExpr)@40..41
              Token(TkInt)@40..41 "1"
        Token(TkNewline)@41..42 "\n"
        "#;

        assert_ast!(code, expected);
    }
}
