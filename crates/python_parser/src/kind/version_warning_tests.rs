#[cfg(test)]
mod version_warning_tests {
    use crate::{
        kind::{PyLanguageLevel, PyVersionNumber},
        parser::{ParserConfig, PyParser},
        parser_error::PyParseErrorKind,
        syntax::PySyntaxTree,
    };

    fn parse_with_version(
        code: &str,
        minor_version: u32,
    ) -> (PySyntaxTree, Vec<crate::parser_error::PyParseError>) {
        let language_level = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: minor_version,
            patch: 0,
        });

        let config = ParserConfig::new(language_level, None);
        let tree = PyParser::parse(code, config);
        let errors = tree.get_errors().to_vec();
        (tree, errors)
    }

    #[test]
    fn test_type_parameters_version_warning() {
        let code = r#"
class Stack[T]:
    pass
"#;

        // Test with Python 3.11 (should warn)
        let (_, errors) = parse_with_version(code, 11);
        let warnings: Vec<_> = errors
            .iter()
            .filter(|e| e.kind == PyParseErrorKind::VersionWarning)
            .collect();
        assert!(!warnings.is_empty(), "There should be a version warning");

        // Debug: Print actual warning message
        for warning in &warnings {
            println!("Type parameter warning message: {}", warning.message);
        }

        assert!(
            warnings
                .iter()
                .any(|w| w.message.contains("type parameter"))
        );
        assert!(
            warnings
                .iter()
                .any(|w| w.message.contains("3.12") || w.message.contains("Python 3.12"))
        );

        // Test with Python 3.12 (should not warn)
        let (_, errors) = parse_with_version(code, 12);
        let warnings: Vec<_> = errors
            .iter()
            .filter(|e| e.kind == PyParseErrorKind::VersionWarning)
            .collect();
        assert!(
            warnings.is_empty(),
            "Python 3.12+ should not have version warnings"
        );
    }

    #[test]
    fn test_union_type_version_warning() {
        let code = "x: int | str = 5";

        // Test with Python 3.9 (should warn)
        let (_, errors) = parse_with_version(code, 9);
        let warnings: Vec<_> = errors
            .iter()
            .filter(|e| e.kind == PyParseErrorKind::VersionWarning)
            .collect();
        assert!(!warnings.is_empty(), "There should be a version warning");

        // Debug: Print actual warning message
        for warning in &warnings {
            println!("Union type warning message: {}", warning.message);
        }

        assert!(warnings.iter().any(|w| w.message.contains("union type")));
        assert!(
            warnings
                .iter()
                .any(|w| w.message.contains("3.10") || w.message.contains("Python 3.10"))
        );

        // Test with Python 3.10 (should not warn)
        let (_, errors) = parse_with_version(code, 10);
        let warnings: Vec<_> = errors
            .iter()
            .filter(|e| e.kind == PyParseErrorKind::VersionWarning)
            .collect();
        assert!(
            warnings.is_empty(),
            "Python 3.10+ should not have version warnings"
        );
    }
}
