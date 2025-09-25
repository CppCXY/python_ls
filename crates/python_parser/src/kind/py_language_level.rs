use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PyLanguageLevel {
    Py3,
}

impl fmt::Display for PyLanguageLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyLanguageLevel::Py3 => write!(f, "Python 3"),
        }
    }
}

impl Default for PyLanguageLevel {
    fn default() -> Self {
        PyLanguageLevel::Py3
    }
}
