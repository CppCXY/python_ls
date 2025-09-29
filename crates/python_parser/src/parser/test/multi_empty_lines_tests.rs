#[cfg(test)]
mod multi_empty_lines_tests {
    use crate::{ParserConfig, PyParser};

    fn print_ast_code(code: &str) {
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

    fn check_ast_code(code: &str) {
        let config = ParserConfig::default();
        let tree = PyParser::parse(code, config);

        // Check for errors
        let errors = tree.get_errors();
        assert!(
            errors.is_empty(),
            "Expected no errors, but found: {:?}",
            errors
        );
    }

    #[test]
    fn test_multiple_empty_lines_in_function() {
        let code = r#"
def test_function():

    print("first statement")


    print("second statement")



    print("third statement")

    return True
"#;
        print_ast_code(code);
        check_ast_code(code);
    }

    #[test]
    fn test_multiple_empty_lines_at_module_level() {
        let code = r#"

print("first")



print("second")


def foo():
    pass


print("third")

"#;
        print_ast_code(code);
    }

    #[test]
    fn test_multiple_empty_lines_in_class() {
        let code = r#"
class TestClass:

    def method1(self):

        print("method1")


    def method2(self):



        print("method2")

        return 42

"#;
        print_ast_code(code);
    }

    #[test]
    fn test_complex_empty_lines_scenario() {
        let code = r#"


def complex_function():

    # First section
    x = 1


    # Second section
    if x > 0:

        print("positive")


        for i in range(3):

            print(i)



    # Final section

    return x


class MyClass:


    def __init__(self):

        self.value = 10



    def process(self):



        return self.value * 2


"#;
        print_ast_code(code);
    }
}
