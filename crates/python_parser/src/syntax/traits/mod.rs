// mod comment_trait;
// mod description_trait;

use std::marker::PhantomData;

use rowan::{TextRange, TextSize, WalkEvent};

use crate::{
    kind::{PySyntaxKind, PyTokenKind},
    syntax::PyAstPtr,
};

use super::PySyntaxId;
pub use super::{
    PySyntaxElementChildren, PySyntaxNode, PySyntaxNodeChildren, PySyntaxToken, node::*,
};
// pub use comment_trait::*;
// pub use description_trait::*;

pub trait PyAstNode {
    fn syntax(&self) -> &PySyntaxNode;

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn child<N: PyAstNode>(&self) -> Option<N> {
        self.syntax().children().find_map(N::cast)
    }

    fn token<N: PyAstToken>(&self) -> Option<N> {
        self.syntax()
            .children_with_tokens()
            .find_map(|it| it.into_token().and_then(N::cast))
    }

    fn token_by_kind(&self, kind: PyTokenKind) -> Option<PyGeneralToken> {
        let token = self
            .syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == kind.into())?;

        PyGeneralToken::cast(token)
    }

    fn tokens<N: PyAstToken>(&self) -> LuaAstTokenChildren<N> {
        LuaAstTokenChildren::new(self.syntax())
    }

    fn children<N: PyAstNode>(&self) -> PyAstChildren<N> {
        PyAstChildren::new(self.syntax())
    }

    fn descendants<N: PyAstNode>(&self) -> impl Iterator<Item = N> {
        self.syntax().descendants().filter_map(N::cast)
    }

    fn walk_descendants<N: PyAstNode>(&self) -> impl Iterator<Item = WalkEvent<N>> {
        self.syntax().preorder().filter_map(|event| match event {
            WalkEvent::Enter(node) => N::cast(node).map(WalkEvent::Enter),
            WalkEvent::Leave(node) => N::cast(node).map(WalkEvent::Leave),
        })
    }

    fn ancestors<N: PyAstNode>(&self) -> impl Iterator<Item = N> {
        self.syntax().ancestors().filter_map(N::cast)
    }

    fn get_root(&self) -> PySyntaxNode {
        let syntax = self.syntax();
        if syntax.kind() == PySyntaxKind::Module.into() {
            syntax.clone()
        } else {
            syntax.ancestors().last().unwrap()
        }
    }

    fn get_parent<N: PyAstNode>(&self) -> Option<N> {
        self.syntax().parent().and_then(N::cast)
    }

    fn get_position(&self) -> TextSize {
        let range = self.syntax().text_range();
        range.start()
    }

    fn get_range(&self) -> TextRange {
        self.syntax().text_range()
    }

    fn get_syntax_id(&self) -> PySyntaxId {
        PySyntaxId::from_node(self.syntax())
    }

    fn get_text(&self) -> String {
        format!("{}", self.syntax().text())
    }

    fn dump(&self) -> String {
        format!("{:#?}", self.syntax())
    }

    fn to_ptr(&self) -> PyAstPtr<Self>
    where
        Self: Sized,
    {
        PyAstPtr::new(self)
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct PyAstChildren<N> {
    inner: PySyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> PyAstChildren<N> {
    pub fn new(parent: &PySyntaxNode) -> PyAstChildren<N> {
        PyAstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: PyAstNode> Iterator for PyAstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

pub trait PyAstToken {
    fn syntax(&self) -> &PySyntaxToken;

    fn can_cast(kind: PyTokenKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: PySyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn get_token_kind(&self) -> PyTokenKind {
        self.syntax().kind().into()
    }

    fn get_position(&self) -> TextSize {
        let range = self.syntax().text_range();
        range.start()
    }

    fn get_range(&self) -> TextRange {
        self.syntax().text_range()
    }

    fn get_syntax_id(&self) -> PySyntaxId {
        PySyntaxId::from_token(self.syntax())
    }

    fn get_text(&self) -> &str {
        self.syntax().text()
    }

    fn slice(&self, range: TextRange) -> Option<&str> {
        let text = self.get_text();
        let self_range = self.get_range();
        if range.start() >= self_range.start() && range.end() <= self_range.end() {
            let start = (range.start() - self_range.start()).into();
            let end = (range.end() - self_range.start()).into();
            text.get(start..end)
        } else {
            None
        }
    }

    fn get_parent<N: PyAstNode>(&self) -> Option<N> {
        self.syntax().parent().and_then(N::cast)
    }

    fn ancestors<N: PyAstNode>(&self) -> impl Iterator<Item = N> {
        self.syntax().parent_ancestors().filter_map(N::cast)
    }

    fn dump(&self) -> String {
        format!("{:#?}", self.syntax())
    }
}

#[derive(Debug, Clone)]
pub struct LuaAstTokenChildren<N> {
    inner: PySyntaxElementChildren,
    ph: PhantomData<N>,
}

impl<N> LuaAstTokenChildren<N> {
    pub fn new(parent: &PySyntaxNode) -> LuaAstTokenChildren<N> {
        LuaAstTokenChildren {
            inner: parent.children_with_tokens(),
            ph: PhantomData,
        }
    }
}

impl<N: PyAstToken> Iterator for LuaAstTokenChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        self.inner.find_map(|it| it.into_token().and_then(N::cast))
    }
}
