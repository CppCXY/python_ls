use rowan::NodeCache;

use crate::{kind::PyLanguageLevel, lexer::LexerConfig};

pub struct ParserConfig<'cache> {
    pub level: PyLanguageLevel,
    lexer_config: LexerConfig,
    node_cache: Option<&'cache mut NodeCache>,
}

impl<'cache> ParserConfig<'cache> {
    pub fn new(level: PyLanguageLevel, node_cache: Option<&'cache mut NodeCache>) -> Self {
        Self {
            level,
            lexer_config: LexerConfig {
                language_level: level,
            },
            node_cache,
        }
    }

    pub fn lexer_config(&self) -> LexerConfig {
        self.lexer_config
    }

    pub fn node_cache(&mut self) -> Option<&mut NodeCache> {
        self.node_cache.as_deref_mut()
    }

    pub fn with_level(level: PyLanguageLevel) -> Self {
        Self {
            level,
            lexer_config: LexerConfig {
                language_level: level,
            },
            node_cache: None,
        }
    }
}

impl Default for ParserConfig<'_> {
    fn default() -> Self {
        Self {
            level: PyLanguageLevel::Py3,
            lexer_config: LexerConfig {
                language_level: PyLanguageLevel::Py3,
            },
            node_cache: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialFunction {
    None,
    Require,
    Error,
    Assert,
    Type,
    Setmetaatable,
}
