use crate::kind::PyLanguageLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexerConfig {
    pub language_level: PyLanguageLevel,
}

impl LexerConfig {}

impl Default for LexerConfig {
    fn default() -> Self {
        LexerConfig {
            language_level: PyLanguageLevel::default(),
        }
    }
}
