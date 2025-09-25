use crate::{LuaAstNode, LuaDocDetailToken, PySyntaxKind, LuaSyntaxNode, PyTokenKind};

#[allow(unused)]
pub trait LuaDocDetailOwner: LuaAstNode {
    fn get_detail(&self) -> Option<LuaDocDetailToken> {
        self.token()
    }

    fn get_detail_text(&self) -> Option<String> {
        self.get_detail().map(|it| it.get_detail().to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocDescription {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocDescription {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::DocDescription
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::DocDescription.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaDocDetailOwner for LuaDocDescription {}

impl LuaDocDescription {
    pub fn get_description_text(&self) -> String {
        let mut text = String::new();
        for token in self
            .syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
        {
            match token.kind().into() {
                PyTokenKind::TkDocDetail => {
                    text.push_str(token.text());
                }
                PyTokenKind::TkEndOfLine => {
                    text.push('\n');
                }
                PyTokenKind::TkNormalStart | PyTokenKind::TkDocContinue => {
                    let mut white_space_count = 0;
                    let start_text_chars = token.text().chars();
                    for c in start_text_chars {
                        if c == ' ' {
                            white_space_count += 1;
                        } else if c == '\t' {
                            white_space_count += 4;
                        }
                    }

                    if white_space_count > 0 {
                        let white_space = " ".repeat(white_space_count);
                        text.push_str(&white_space);
                    }
                }
                _ => {}
            }
        }

        text
    }
}
