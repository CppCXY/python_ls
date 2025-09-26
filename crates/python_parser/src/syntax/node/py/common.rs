use crate::{
    kind::PySyntaxKind,
    syntax::traits::{PyAstNode, PySyntaxNode},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyModule {
    syntax: PySyntaxNode,
}

impl PyAstNode for PyModule {
    fn syntax(&self) -> &PySyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::Module
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySuite {
    syntax: PySyntaxNode,
}

impl PyAstNode for PySuite {
    fn syntax(&self) -> &PySyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::Suite
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyParameter {
    syntax: PySyntaxNode,
}

impl PyAstNode for PyParameter {
    fn syntax(&self) -> &PySyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::Parameter
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyArguments {
    syntax: PySyntaxNode,
}

impl PyAstNode for PyArguments {
    fn syntax(&self) -> &PySyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::Arguments
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyDecorator {
    syntax: PySyntaxNode,
}

impl PyAstNode for PyDecorator {
    fn syntax(&self) -> &PySyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::Decorator
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}
