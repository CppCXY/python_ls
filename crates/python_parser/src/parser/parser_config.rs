use crate::{kind::PyLanguageLevel, lexer::LexerConfig};

#[derive(Debug, Clone, Copy)]
pub struct ParserConfig {
    pub level: PyLanguageLevel,
    lexer_config: LexerConfig,
}

impl ParserConfig {
    pub fn new(level: PyLanguageLevel) -> Self {
        Self {
            level,
            lexer_config: LexerConfig {
                language_level: level,
            },
        }
    }

    pub fn lexer_config(&self) -> LexerConfig {
        self.lexer_config
    }

    pub fn with_level(level: PyLanguageLevel) -> Self {
        Self {
            level,
            lexer_config: LexerConfig {
                language_level: level,
            },
        }
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            level: PyLanguageLevel::default(),
            lexer_config: LexerConfig {
                language_level: PyLanguageLevel::default(),
            },
        }
    }
}
