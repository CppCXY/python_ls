pub mod common;
pub mod expr;
pub mod stat;

pub use common::*;
pub use expr::*;
pub use stat::*;

use crate::{
    kind::PySyntaxKind,
    syntax::node::{PyAstNode, PySyntaxNode},
};

// Simplified PyAst enum for core Python node types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyAst {
    // Core structural nodes
    Module(PyModule),
    Suite(PySuite),
    Parameter(PyParameter),
    Arguments(PyArguments),
    Decorator(PyDecorator),

    // Statement union
    Stat(PyStat),

    // Expression union
    Expr(PyExpr),
}

impl PyAstNode for PyAst {
    fn syntax(&self) -> &PySyntaxNode {
        match self {
            PyAst::Module(node) => node.syntax(),
            PyAst::Suite(node) => node.syntax(),
            PyAst::Parameter(node) => node.syntax(),
            PyAst::Arguments(node) => node.syntax(),
            PyAst::Decorator(node) => node.syntax(),
            PyAst::Stat(node) => node.syntax(),
            PyAst::Expr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        PyModule::can_cast(kind)
            || PySuite::can_cast(kind)
            || PyParameter::can_cast(kind)
            || PyArguments::can_cast(kind)
            || PyDecorator::can_cast(kind)
            || PyStat::can_cast(kind)
            || PyExpr::can_cast(kind)
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(module) = PyModule::cast(syntax.clone()) {
            Some(PyAst::Module(module))
        } else if let Some(suite) = PySuite::cast(syntax.clone()) {
            Some(PyAst::Suite(suite))
        } else if let Some(param) = PyParameter::cast(syntax.clone()) {
            Some(PyAst::Parameter(param))
        } else if let Some(args) = PyArguments::cast(syntax.clone()) {
            Some(PyAst::Arguments(args))
        } else if let Some(decorator) = PyDecorator::cast(syntax.clone()) {
            Some(PyAst::Decorator(decorator))
        } else if let Some(stat) = PyStat::cast(syntax.clone()) {
            Some(PyAst::Stat(stat))
        } else if let Some(expr) = PyExpr::cast(syntax.clone()) {
            Some(PyAst::Expr(expr))
        } else {
            None
        }
    }
}
