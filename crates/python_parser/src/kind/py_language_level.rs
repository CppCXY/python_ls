use core::fmt;

use crate::PyVersionNumber;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
pub enum PyLanguageLevel {
    Py3(PyVersionNumber),
}

impl PyLanguageLevel {
    pub fn support_f_string(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 6,
        }
    }

    /// Python 3.10+ Match statements (structural pattern matching)
    pub fn support_match_statement(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 10,
        }
    }

    /// Python 3.10+ Union types with | operator (PEP 604)
    pub fn support_union_types(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 10,
        }
    }

    /// Python 3.10+ Parenthesized context managers (PEP 617)
    pub fn support_parenthesized_context_managers(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 10,
        }
    }

    /// Python 3.11+ Exception groups and except* (PEP 654)
    pub fn support_exception_groups(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 11,
        }
    }

    /// Python 3.11+ Task groups in asyncio
    pub fn support_task_groups(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 11,
        }
    }

    /// Python 3.12+ Type parameter syntax (PEP 695)
    pub fn support_type_parameters(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 12,
        }
    }

    /// Python 3.12+ Type statements (PEP 695)
    pub fn support_type_statements(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 12,
        }
    }

    /// Python 3.12+ Generic classes with type parameters
    pub fn support_generic_type_syntax(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 12,
        }
    }

    /// Python 3.13+ @override decorator
    pub fn support_override_decorator(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 13,
        }
    }

    /// Python 3.13+ Enhanced async features
    pub fn support_enhanced_async(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 13,
        }
    }

    /// Python 3.14+ Experimental features
    pub fn support_experimental_features(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 14,
        }
    }

    /// Check if at least Python 3.10
    pub fn is_python_310_or_later(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 10,
        }
    }

    /// Check if at least Python 3.11
    pub fn is_python_311_or_later(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 11,
        }
    }

    /// Check if at least Python 3.12
    pub fn is_python_312_or_later(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 12,
        }
    }

    /// Check if at least Python 3.13
    pub fn is_python_313_or_later(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 13,
        }
    }

    /// Check if at least Python 3.14
    pub fn is_python_314_or_later(&self) -> bool {
        match self {
            PyLanguageLevel::Py3(num) => num.minor >= 14,
        }
    }
}

impl fmt::Display for PyLanguageLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyLanguageLevel::Py3(num) => write!(f, "Python 3.{}.{:02}", num.minor, num.patch),
        }
    }
}

impl Default for PyLanguageLevel {
    fn default() -> Self {
        PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 14,
            patch: 0,
        })
    }
}

impl PartialOrd for PyLanguageLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PyLanguageLevel::Py3(a), PyLanguageLevel::Py3(b)) => a.partial_cmp(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PyVersionNumber;

    #[test]
    fn test_python_310_features() {
        let py310 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 10,
            patch: 0,
        });
        let py39 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 9,
            patch: 0,
        });

        // Python 3.10+ features
        assert!(py310.support_match_statement());
        assert!(py310.support_union_types());
        assert!(py310.support_parenthesized_context_managers());
        assert!(py310.is_python_310_or_later());

        // Python 3.9 should not support these
        assert!(!py39.support_match_statement());
        assert!(!py39.support_union_types());
        assert!(!py39.support_parenthesized_context_managers());
        assert!(!py39.is_python_310_or_later());
    }

    #[test]
    fn test_python_311_features() {
        let py311 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 11,
            patch: 0,
        });
        let py310 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 10,
            patch: 0,
        });

        // Python 3.11+ features
        assert!(py311.support_exception_groups());
        assert!(py311.support_task_groups());
        assert!(py311.is_python_311_or_later());

        // Python 3.10 should not support these
        assert!(!py310.support_exception_groups());
        assert!(!py310.support_task_groups());
        assert!(!py310.is_python_311_or_later());

        // But should still support 3.10 features
        assert!(py311.support_match_statement());
        assert!(py311.support_union_types());
    }

    #[test]
    fn test_python_312_features() {
        let py312 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 12,
            patch: 0,
        });
        let py311 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 11,
            patch: 0,
        });

        // Python 3.12+ features
        assert!(py312.support_type_parameters());
        assert!(py312.support_type_statements());
        assert!(py312.support_generic_type_syntax());
        assert!(py312.is_python_312_or_later());

        // Python 3.11 should not support these
        assert!(!py311.support_type_parameters());
        assert!(!py311.support_type_statements());
        assert!(!py311.support_generic_type_syntax());
        assert!(!py311.is_python_312_or_later());

        // But should still support earlier features
        assert!(py312.support_exception_groups());
        assert!(py312.support_match_statement());
    }

    #[test]
    fn test_python_313_features() {
        let py313 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 13,
            patch: 0,
        });
        let py312 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 12,
            patch: 0,
        });

        // Python 3.13+ features
        assert!(py313.support_override_decorator());
        assert!(py313.support_enhanced_async());
        assert!(py313.is_python_313_or_later());

        // Python 3.12 should not support these
        assert!(!py312.support_override_decorator());
        assert!(!py312.support_enhanced_async());
        assert!(!py312.is_python_313_or_later());

        // But should still support earlier features
        assert!(py313.support_type_parameters());
        assert!(py313.support_exception_groups());
        assert!(py313.support_match_statement());
    }

    #[test]
    fn test_python_314_features() {
        let py314 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 14,
            patch: 0,
        });
        let py313 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 13,
            patch: 0,
        });

        // Python 3.14+ features
        assert!(py314.support_experimental_features());
        assert!(py314.is_python_314_or_later());

        // Python 3.13 should not support these
        assert!(!py313.support_experimental_features());
        assert!(!py313.is_python_314_or_later());

        // But should still support all earlier features
        assert!(py314.support_override_decorator());
        assert!(py314.support_type_parameters());
        assert!(py314.support_exception_groups());
        assert!(py314.support_match_statement());
    }

    #[test]
    fn test_feature_compatibility() {
        let py314 = PyLanguageLevel::Py3(PyVersionNumber {
            major: 3,
            minor: 14,
            patch: 0,
        });

        // Ensure backward compatibility - Python 3.14 should support all features
        assert!(py314.support_f_string()); // Python 3.6+
        assert!(py314.support_match_statement()); // Python 3.10+
        assert!(py314.support_union_types()); // Python 3.10+
        assert!(py314.support_exception_groups()); // Python 3.11+
        assert!(py314.support_type_parameters()); // Python 3.12+
        assert!(py314.support_override_decorator()); // Python 3.13+
        assert!(py314.support_experimental_features()); // Python 3.14+
    }

    #[test]
    fn test_default_language_level() {
        let default_level = PyLanguageLevel::default();

        // Default should be Python 3.14
        assert!(default_level.is_python_314_or_later());
        assert!(default_level.support_experimental_features());

        // Should support all features
        assert!(default_level.support_f_string());
        assert!(default_level.support_match_statement());
        assert!(default_level.support_exception_groups());
        assert!(default_level.support_type_parameters());
        assert!(default_level.support_override_decorator());
    }
}
