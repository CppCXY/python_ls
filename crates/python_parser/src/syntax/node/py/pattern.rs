use crate::{
    kind::PySyntaxKind,
    syntax::node::{PyAstNode, PySyntaxNode},
};

macro_rules! py_pattern_ast {
    ($(
        $ast_name:ident: $kind:ident,
    )*) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct $ast_name {
                syntax: PySyntaxNode,
            }

            impl PyAstNode for $ast_name {
                fn syntax(&self) -> &PySyntaxNode {
                    &self.syntax
                }

                fn can_cast(kind: PySyntaxKind) -> bool
                where
                    Self: Sized,
                {
                    matches!(kind, PySyntaxKind::$kind)
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
        )*
    };
}

py_pattern_ast!(
    // Python 3.10+ Pattern Matching
    PyWildcardPattern: WildcardPattern,
    PyValuePattern: ValuePattern,
    PyBindPattern: BindPattern,
    PyClassPattern: ClassPattern,
    PySequencePattern: SequencePattern,
    PyMappingPattern: MappingPattern,
    PyOrPattern: OrPattern,
    PyGuardClause: GuardClause,
    // Python 3.14+ Enhanced Patterns
    PyEnhancedPattern: EnhancedPattern,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyPattern {
    WildcardPattern(PyWildcardPattern),
    ValuePattern(PyValuePattern),
    BindPattern(PyBindPattern),
    ClassPattern(PyClassPattern),
    SequencePattern(PySequencePattern),
    MappingPattern(PyMappingPattern),
    OrPattern(PyOrPattern),
    GuardClause(PyGuardClause),
    EnhancedPattern(PyEnhancedPattern),
}

impl PyAstNode for PyPattern {
    fn syntax(&self) -> &PySyntaxNode {
        match self {
            PyPattern::WildcardPattern(node) => node.syntax(),
            PyPattern::ValuePattern(node) => node.syntax(),
            PyPattern::BindPattern(node) => node.syntax(),
            PyPattern::ClassPattern(node) => node.syntax(),
            PyPattern::SequencePattern(node) => node.syntax(),
            PyPattern::MappingPattern(node) => node.syntax(),
            PyPattern::OrPattern(node) => node.syntax(),
            PyPattern::GuardClause(node) => node.syntax(),
            PyPattern::EnhancedPattern(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::WildcardPattern
                | PySyntaxKind::ValuePattern
                | PySyntaxKind::BindPattern
                | PySyntaxKind::ClassPattern
                | PySyntaxKind::SequencePattern
                | PySyntaxKind::MappingPattern
                | PySyntaxKind::OrPattern
                | PySyntaxKind::GuardClause
                | PySyntaxKind::EnhancedPattern
        )
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            PySyntaxKind::WildcardPattern => {
                PyWildcardPattern::cast(syntax).map(PyPattern::WildcardPattern)
            }
            PySyntaxKind::ValuePattern => {
                PyValuePattern::cast(syntax).map(PyPattern::ValuePattern)
            }
            PySyntaxKind::BindPattern => PyBindPattern::cast(syntax).map(PyPattern::BindPattern),
            PySyntaxKind::ClassPattern => {
                PyClassPattern::cast(syntax).map(PyPattern::ClassPattern)
            }
            PySyntaxKind::SequencePattern => {
                PySequencePattern::cast(syntax).map(PyPattern::SequencePattern)
            }
            PySyntaxKind::MappingPattern => {
                PyMappingPattern::cast(syntax).map(PyPattern::MappingPattern)
            }
            PySyntaxKind::OrPattern => PyOrPattern::cast(syntax).map(PyPattern::OrPattern),
            PySyntaxKind::GuardClause => PyGuardClause::cast(syntax).map(PyPattern::GuardClause),
            PySyntaxKind::EnhancedPattern => {
                PyEnhancedPattern::cast(syntax).map(PyPattern::EnhancedPattern)
            }
            _ => None,
        }
    }
}

// Pattern child accessor methods
impl PyWildcardPattern {
    pub fn text(&self) -> String {
        self.syntax().text().to_string()
    }
}

impl PyValuePattern {
    pub fn value(&self) -> Option<crate::syntax::node::py::expr::PyExpr> {
        use crate::syntax::node::py::expr::PyExpr;
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyBindPattern {
    pub fn name(&self) -> String {
        self.syntax().text().to_string()
    }

    pub fn pattern(&self) -> Option<PyPattern> {
        self.syntax().children().find_map(PyPattern::cast)
    }
}

impl PyClassPattern {
    pub fn class_name(&self) -> Option<crate::syntax::node::py::expr::PyExpr> {
        use crate::syntax::node::py::expr::PyExpr;
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn patterns(&self) -> impl Iterator<Item = PyPattern> {
        self.syntax().children().filter_map(PyPattern::cast)
    }
}

impl PySequencePattern {
    pub fn patterns(&self) -> impl Iterator<Item = PyPattern> {
        self.syntax().children().filter_map(PyPattern::cast)
    }
}

impl PyMappingPattern {
    pub fn key_patterns(&self) -> impl Iterator<Item = crate::syntax::node::py::expr::PyExpr> {
        use crate::syntax::node::py::expr::PyExpr;
        self.syntax().children().filter_map(PyExpr::cast)
    }

    pub fn value_patterns(&self) -> impl Iterator<Item = PyPattern> {
        self.syntax().children().filter_map(PyPattern::cast)
    }
}

impl PyOrPattern {
    pub fn patterns(&self) -> impl Iterator<Item = PyPattern> {
        self.syntax().children().filter_map(PyPattern::cast)
    }
}

impl PyGuardClause {
    pub fn condition(&self) -> Option<crate::syntax::node::py::expr::PyExpr> {
        use crate::syntax::node::py::expr::PyExpr;
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyEnhancedPattern {
    pub fn base_pattern(&self) -> Option<PyPattern> {
        self.syntax().children().find_map(PyPattern::cast)
    }

    pub fn enhancement(&self) -> Option<crate::syntax::node::py::expr::PyExpr> {
        use crate::syntax::node::py::expr::PyExpr;
        self.syntax().children().find_map(PyExpr::cast)
    }
}