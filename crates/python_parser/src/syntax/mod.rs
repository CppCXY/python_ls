mod node;
mod traits;
mod tree;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::iter::successors;
use std::marker::PhantomData;

use rowan::{Language, TextRange, TextSize};

use crate::kind::{PyKind, PySyntaxKind, PyTokenKind};
pub use node::*;
pub use traits::*;
pub use tree::{PySyntaxTree, PyTreeBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyLanguage;

impl Language for PyLanguage {
    type Kind = PyKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        PyKind::from_raw(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.get_raw())
    }
}

pub type PySyntaxNode = rowan::SyntaxNode<PyLanguage>;
pub type PySyntaxToken = rowan::SyntaxToken<PyLanguage>;
pub type PySyntaxElement = rowan::NodeOrToken<PySyntaxNode, PySyntaxToken>;
pub type PySyntaxElementChildren = rowan::SyntaxElementChildren<PyLanguage>;
pub type PySyntaxNodeChildren = rowan::SyntaxNodeChildren<PyLanguage>;
pub type PySyntaxNodePtr = rowan::ast::SyntaxNodePtr<PyLanguage>;

impl From<PySyntaxKind> for rowan::SyntaxKind {
    fn from(kind: PySyntaxKind) -> Self {
        let lua_kind = PyKind::from(kind);
        rowan::SyntaxKind(lua_kind.get_raw())
    }
}

impl From<rowan::SyntaxKind> for PySyntaxKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        PyKind::from_raw(kind.0).into()
    }
}

impl From<PyTokenKind> for rowan::SyntaxKind {
    fn from(kind: PyTokenKind) -> Self {
        let lua_kind = PyKind::from(kind);
        rowan::SyntaxKind(lua_kind.get_raw())
    }
}

impl From<rowan::SyntaxKind> for PyTokenKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        PyKind::from_raw(kind.0).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PySyntaxId {
    kind: PyKind,
    range: TextRange,
}

impl PySyntaxId {
    pub fn new(kind: PyKind, range: TextRange) -> Self {
        PySyntaxId { kind, range }
    }

    pub fn from_ptr(ptr: PySyntaxNodePtr) -> Self {
        PySyntaxId {
            kind: ptr.kind(),
            range: ptr.text_range(),
        }
    }

    pub fn from_node(node: &PySyntaxNode) -> Self {
        PySyntaxId {
            kind: node.kind(),
            range: node.text_range(),
        }
    }

    pub fn from_token(token: &PySyntaxToken) -> Self {
        PySyntaxId {
            kind: token.kind(),
            range: token.text_range(),
        }
    }

    pub fn get_kind(&self) -> PySyntaxKind {
        self.kind.into()
    }

    pub fn get_token_kind(&self) -> PyTokenKind {
        self.kind.into()
    }

    pub fn is_token(&self) -> bool {
        self.kind.is_token()
    }

    pub fn is_node(&self) -> bool {
        self.kind.is_syntax()
    }

    pub fn get_range(&self) -> TextRange {
        self.range
    }

    pub fn to_node(&self, tree: &PySyntaxTree) -> Option<PySyntaxNode> {
        let root = tree.get_red_root();
        if root.parent().is_some() {
            return None;
        }
        self.to_node_from_root(&root)
    }

    pub fn to_node_from_root(&self, root: &PySyntaxNode) -> Option<PySyntaxNode> {
        successors(Some(root.clone()), |node| {
            node.child_or_token_at_range(self.range)?.into_node()
        })
        .find(|it| it.text_range() == self.range && it.kind() == self.kind)
    }

    pub fn to_token(&self, tree: &PySyntaxTree) -> Option<PySyntaxToken> {
        let root = tree.get_red_root();
        if root.parent().is_some() {
            return None;
        }
        self.to_token_from_root(&root)
    }

    pub fn to_token_from_root(&self, root: &PySyntaxNode) -> Option<PySyntaxToken> {
        let mut current_node = Some(root.clone());
        while let Some(node) = current_node {
            let node_or_token = node.child_or_token_at_range(self.range)?;
            match node_or_token {
                rowan::NodeOrToken::Node(node) => {
                    current_node = Some(node);
                }
                rowan::NodeOrToken::Token(token) => {
                    if token.text_range() == self.range && token.kind() == self.kind {
                        return Some(token);
                    }
                    return None;
                }
            }
        }
        None
    }

    pub fn to_node_at_range(root: &PySyntaxNode, range: TextRange) -> Option<PySyntaxNode> {
        successors(Some(root.clone()), |node| {
            node.child_or_token_at_range(range)?.into_node()
        })
        .find(|it| it.text_range() == range)
    }
}

impl Serialize for PySyntaxId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let kind_raw = self.kind.get_raw();
        let start = u32::from(self.range.start());
        let end = u32::from(self.range.end());
        let range_combined = ((start as u64) << 32) | (end as u64);
        let value = format!("{:x}:{:x}", kind_raw, range_combined);
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for PySyntaxId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LuaSyntaxIdVisitor;

        impl Visitor<'_> for LuaSyntaxIdVisitor {
            type Value = PySyntaxId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with format 'kind:range'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.split(':').collect();
                if parts.len() != 2 {
                    return Err(E::custom("expected format 'kind:range'"));
                }

                let kind_raw = u16::from_str_radix(parts[0], 16)
                    .map_err(|e| E::custom(format!("invalid kind: {}", e)))?;
                let range_combined = u64::from_str_radix(parts[1], 16)
                    .map_err(|e| E::custom(format!("invalid range: {}", e)))?;

                let start = TextSize::new(((range_combined >> 32) & 0xFFFFFFFF) as u32);
                let end = TextSize::new((range_combined & 0xFFFFFFFF) as u32);

                Ok(PySyntaxId {
                    kind: PyKind::from_raw(kind_raw),
                    range: TextRange::new(start, end),
                })
            }
        }

        deserializer.deserialize_str(LuaSyntaxIdVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyAstPtr<T: PyAstNode> {
    pub syntax_id: PySyntaxId,
    _phantom: PhantomData<T>,
}

impl<T: PyAstNode> PyAstPtr<T> {
    pub fn new(node: &T) -> Self {
        PyAstPtr {
            syntax_id: node.get_syntax_id(),
            _phantom: PhantomData,
        }
    }

    pub fn get_syntax_id(&self) -> PySyntaxId {
        self.syntax_id
    }

    pub fn to_node(&self, root: &PyModule) -> Option<T> {
        let syntax_node = self.syntax_id.to_node_from_root(root.syntax());
        if let Some(node) = syntax_node {
            T::cast(node)
        } else {
            None
        }
    }
}

unsafe impl<T: PyAstNode> Send for PyAstPtr<T> {}
unsafe impl<T: PyAstNode> Sync for PyAstPtr<T> {}
