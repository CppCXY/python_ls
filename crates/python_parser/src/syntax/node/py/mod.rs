pub mod common;
pub mod expr;
pub mod stat;
mod test;

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
    CaseClause(PyCaseClause),

    // Statement types
    ExprStmt(PyExprStmt),
    AssignStmt(PyAssignStmt),
    AnnAssignStmt(PyAnnAssignStmt),
    AugAssignStmt(PyAugAssignStmt),
    FuncDef(PyFuncDef),
    AsyncFuncDef(PyAsyncFuncDef),
    ClassDef(PyClassDef),
    IfStmt(PyIfStmt),
    WhileStmt(PyWhileStmt),
    ForStmt(PyForStmt),
    AsyncForStmt(PyAsyncForStmt),
    WithStmt(PyWithStmt),
    AsyncWithStmt(PyAsyncWithStmt),
    TryStmt(PyTryStmt),
    BreakStmt(PyBreakStmt),
    ContinueStmt(PyContinueStmt),
    ReturnStmt(PyReturnStmt),
    YieldStmt(PyYieldStmt),
    RaiseStmt(PyRaiseStmt),
    AssertStmt(PyAssertStmt),
    DeleteStmt(PyDeleteStmt),
    PassStmt(PyPassStmt),
    GlobalStmt(PyGlobalStmt),
    NonlocalStmt(PyNonlocalStmt),
    ImportStmt(PyImportStmt),
    ImportFromStmt(PyImportFromStmt),
    MatchStmt(PyMatchStmt),
    ElseStmt(PyElseStmt),
    ElifStmt(PyElifStmt),

    // Expression types
    NameExpr(PyNameExpr),
    LiteralExpr(PyLiteralExpr),
    ParenExpr(PyParenExpr),
    TupleExpr(PyTupleExpr),
    ListExpr(PyListExpr),
    DictExpr(PyDictExpr),
    SetExpr(PySetExpr),
    BinaryExpr(PyBinaryExpr),
    UnaryExpr(PyUnaryExpr),
    BoolOpExpr(PyBoolOpExpr),
    CompareExpr(PyCompareExpr),
    CallExpr(PyCallExpr),
    MethodCallExpr(PyMethodCallExpr),
    AttributeExpr(PyAttributeExpr),
    SubscriptExpr(PySubscriptExpr),
    SliceExpr(PySliceExpr),
    LambdaExpr(PyLambdaExpr),
    IfExpr(PyIfExpr),
    YieldExpr(PyYieldExpr),
    YieldFromExpr(PyYieldFromExpr),
    AwaitExpr(PyAwaitExpr),
    StarredExpr(PyStarredExpr),
    DoubleStarredExpr(PyDoubleStarredExpr),
    AssignExpr(PyAssignExpr),
    ConditionalExpr(PyConditionalExpr),
    ListCompExpr(PyListCompExpr),
    DictCompExpr(PyDictCompExpr),
    SetCompExpr(PySetCompExpr),
    GeneratorExpr(PyGeneratorExpr),

    // Union types for backward compatibility
    Stat(PyStat),
    Expr(PyExpr),
}

impl PyAstNode for PyAst {
    fn syntax(&self) -> &PySyntaxNode {
        match self {
            // Core structural nodes
            PyAst::Module(node) => node.syntax(),
            PyAst::Suite(node) => node.syntax(),
            PyAst::Parameter(node) => node.syntax(),
            PyAst::Arguments(node) => node.syntax(),
            PyAst::Decorator(node) => node.syntax(),
            PyAst::CaseClause(node) => node.syntax(),

            // Statement types
            PyAst::ExprStmt(node) => node.syntax(),
            PyAst::AssignStmt(node) => node.syntax(),
            PyAst::AnnAssignStmt(node) => node.syntax(),
            PyAst::AugAssignStmt(node) => node.syntax(),
            PyAst::FuncDef(node) => node.syntax(),
            PyAst::AsyncFuncDef(node) => node.syntax(),
            PyAst::ClassDef(node) => node.syntax(),
            PyAst::IfStmt(node) => node.syntax(),
            PyAst::WhileStmt(node) => node.syntax(),
            PyAst::ForStmt(node) => node.syntax(),
            PyAst::AsyncForStmt(node) => node.syntax(),
            PyAst::WithStmt(node) => node.syntax(),
            PyAst::AsyncWithStmt(node) => node.syntax(),
            PyAst::TryStmt(node) => node.syntax(),
            PyAst::BreakStmt(node) => node.syntax(),
            PyAst::ContinueStmt(node) => node.syntax(),
            PyAst::ReturnStmt(node) => node.syntax(),
            PyAst::YieldStmt(node) => node.syntax(),
            PyAst::RaiseStmt(node) => node.syntax(),
            PyAst::AssertStmt(node) => node.syntax(),
            PyAst::DeleteStmt(node) => node.syntax(),
            PyAst::PassStmt(node) => node.syntax(),
            PyAst::GlobalStmt(node) => node.syntax(),
            PyAst::NonlocalStmt(node) => node.syntax(),
            PyAst::ImportStmt(node) => node.syntax(),
            PyAst::ImportFromStmt(node) => node.syntax(),
            PyAst::MatchStmt(node) => node.syntax(),
            PyAst::ElseStmt(node) => node.syntax(),
            PyAst::ElifStmt(node) => node.syntax(),

            // Expression types
            PyAst::NameExpr(node) => node.syntax(),
            PyAst::LiteralExpr(node) => node.syntax(),
            PyAst::ParenExpr(node) => node.syntax(),
            PyAst::TupleExpr(node) => node.syntax(),
            PyAst::ListExpr(node) => node.syntax(),
            PyAst::DictExpr(node) => node.syntax(),
            PyAst::SetExpr(node) => node.syntax(),
            PyAst::BinaryExpr(node) => node.syntax(),
            PyAst::UnaryExpr(node) => node.syntax(),
            PyAst::BoolOpExpr(node) => node.syntax(),
            PyAst::CompareExpr(node) => node.syntax(),
            PyAst::CallExpr(node) => node.syntax(),
            PyAst::MethodCallExpr(node) => node.syntax(),
            PyAst::AttributeExpr(node) => node.syntax(),
            PyAst::SubscriptExpr(node) => node.syntax(),
            PyAst::SliceExpr(node) => node.syntax(),
            PyAst::LambdaExpr(node) => node.syntax(),
            PyAst::IfExpr(node) => node.syntax(),
            PyAst::YieldExpr(node) => node.syntax(),
            PyAst::YieldFromExpr(node) => node.syntax(),
            PyAst::AwaitExpr(node) => node.syntax(),
            PyAst::StarredExpr(node) => node.syntax(),
            PyAst::DoubleStarredExpr(node) => node.syntax(),
            PyAst::AssignExpr(node) => node.syntax(),
            PyAst::ConditionalExpr(node) => node.syntax(),
            PyAst::ListCompExpr(node) => node.syntax(),
            PyAst::DictCompExpr(node) => node.syntax(),
            PyAst::SetCompExpr(node) => node.syntax(),
            PyAst::GeneratorExpr(node) => node.syntax(),

            // Union types for backward compatibility
            PyAst::Stat(node) => node.syntax(),
            PyAst::Expr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        // Core structural nodes
        PyModule::can_cast(kind)
            || PySuite::can_cast(kind)
            || PyParameter::can_cast(kind)
            || PyArguments::can_cast(kind)
            || PyDecorator::can_cast(kind)
            || PyCaseClause::can_cast(kind)
            // Statement types
            || PyExprStmt::can_cast(kind)
            || PyAssignStmt::can_cast(kind)
            || PyAnnAssignStmt::can_cast(kind)
            || PyAugAssignStmt::can_cast(kind)
            || PyFuncDef::can_cast(kind)
            || PyAsyncFuncDef::can_cast(kind)
            || PyClassDef::can_cast(kind)
            || PyIfStmt::can_cast(kind)
            || PyWhileStmt::can_cast(kind)
            || PyForStmt::can_cast(kind)
            || PyAsyncForStmt::can_cast(kind)
            || PyWithStmt::can_cast(kind)
            || PyAsyncWithStmt::can_cast(kind)
            || PyTryStmt::can_cast(kind)
            || PyBreakStmt::can_cast(kind)
            || PyContinueStmt::can_cast(kind)
            || PyReturnStmt::can_cast(kind)
            || PyYieldStmt::can_cast(kind)
            || PyRaiseStmt::can_cast(kind)
            || PyAssertStmt::can_cast(kind)
            || PyDeleteStmt::can_cast(kind)
            || PyPassStmt::can_cast(kind)
            || PyGlobalStmt::can_cast(kind)
            || PyNonlocalStmt::can_cast(kind)
            || PyImportStmt::can_cast(kind)
            || PyImportFromStmt::can_cast(kind)
            || PyMatchStmt::can_cast(kind)
            || PyElseStmt::can_cast(kind)
            || PyElifStmt::can_cast(kind)
            // Expression types
            || PyNameExpr::can_cast(kind)
            || PyLiteralExpr::can_cast(kind)
            || PyParenExpr::can_cast(kind)
            || PyTupleExpr::can_cast(kind)
            || PyListExpr::can_cast(kind)
            || PyDictExpr::can_cast(kind)
            || PySetExpr::can_cast(kind)
            || PyBinaryExpr::can_cast(kind)
            || PyUnaryExpr::can_cast(kind)
            || PyBoolOpExpr::can_cast(kind)
            || PyCompareExpr::can_cast(kind)
            || PyCallExpr::can_cast(kind)
            || PyMethodCallExpr::can_cast(kind)
            || PyAttributeExpr::can_cast(kind)
            || PySubscriptExpr::can_cast(kind)
            || PySliceExpr::can_cast(kind)
            || PyLambdaExpr::can_cast(kind)
            || PyIfExpr::can_cast(kind)
            || PyYieldExpr::can_cast(kind)
            || PyYieldFromExpr::can_cast(kind)
            || PyAwaitExpr::can_cast(kind)
            || PyStarredExpr::can_cast(kind)
            || PyDoubleStarredExpr::can_cast(kind)
            || PyAssignExpr::can_cast(kind)
            || PyConditionalExpr::can_cast(kind)
            || PyListCompExpr::can_cast(kind)
            || PyDictCompExpr::can_cast(kind)
            || PySetCompExpr::can_cast(kind)
            || PyGeneratorExpr::can_cast(kind)
            // Union types for backward compatibility
            || PyStat::can_cast(kind)
            || PyExpr::can_cast(kind)
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        // Core structural nodes
        if let Some(node) = PyModule::cast(syntax.clone()) {
            Some(PyAst::Module(node))
        } else if let Some(node) = PySuite::cast(syntax.clone()) {
            Some(PyAst::Suite(node))
        } else if let Some(node) = PyParameter::cast(syntax.clone()) {
            Some(PyAst::Parameter(node))
        } else if let Some(node) = PyArguments::cast(syntax.clone()) {
            Some(PyAst::Arguments(node))
        } else if let Some(node) = PyDecorator::cast(syntax.clone()) {
            Some(PyAst::Decorator(node))
        } else if let Some(node) = PyCaseClause::cast(syntax.clone()) {
            Some(PyAst::CaseClause(node))
        // Statement types
        } else if let Some(node) = PyExprStmt::cast(syntax.clone()) {
            Some(PyAst::ExprStmt(node))
        } else if let Some(node) = PyAssignStmt::cast(syntax.clone()) {
            Some(PyAst::AssignStmt(node))
        } else if let Some(node) = PyAnnAssignStmt::cast(syntax.clone()) {
            Some(PyAst::AnnAssignStmt(node))
        } else if let Some(node) = PyAugAssignStmt::cast(syntax.clone()) {
            Some(PyAst::AugAssignStmt(node))
        } else if let Some(node) = PyFuncDef::cast(syntax.clone()) {
            Some(PyAst::FuncDef(node))
        } else if let Some(node) = PyAsyncFuncDef::cast(syntax.clone()) {
            Some(PyAst::AsyncFuncDef(node))
        } else if let Some(node) = PyClassDef::cast(syntax.clone()) {
            Some(PyAst::ClassDef(node))
        } else if let Some(node) = PyIfStmt::cast(syntax.clone()) {
            Some(PyAst::IfStmt(node))
        } else if let Some(node) = PyWhileStmt::cast(syntax.clone()) {
            Some(PyAst::WhileStmt(node))
        } else if let Some(node) = PyForStmt::cast(syntax.clone()) {
            Some(PyAst::ForStmt(node))
        } else if let Some(node) = PyAsyncForStmt::cast(syntax.clone()) {
            Some(PyAst::AsyncForStmt(node))
        } else if let Some(node) = PyWithStmt::cast(syntax.clone()) {
            Some(PyAst::WithStmt(node))
        } else if let Some(node) = PyAsyncWithStmt::cast(syntax.clone()) {
            Some(PyAst::AsyncWithStmt(node))
        } else if let Some(node) = PyTryStmt::cast(syntax.clone()) {
            Some(PyAst::TryStmt(node))
        } else if let Some(node) = PyBreakStmt::cast(syntax.clone()) {
            Some(PyAst::BreakStmt(node))
        } else if let Some(node) = PyContinueStmt::cast(syntax.clone()) {
            Some(PyAst::ContinueStmt(node))
        } else if let Some(node) = PyReturnStmt::cast(syntax.clone()) {
            Some(PyAst::ReturnStmt(node))
        } else if let Some(node) = PyYieldStmt::cast(syntax.clone()) {
            Some(PyAst::YieldStmt(node))
        } else if let Some(node) = PyRaiseStmt::cast(syntax.clone()) {
            Some(PyAst::RaiseStmt(node))
        } else if let Some(node) = PyAssertStmt::cast(syntax.clone()) {
            Some(PyAst::AssertStmt(node))
        } else if let Some(node) = PyDeleteStmt::cast(syntax.clone()) {
            Some(PyAst::DeleteStmt(node))
        } else if let Some(node) = PyPassStmt::cast(syntax.clone()) {
            Some(PyAst::PassStmt(node))
        } else if let Some(node) = PyGlobalStmt::cast(syntax.clone()) {
            Some(PyAst::GlobalStmt(node))
        } else if let Some(node) = PyNonlocalStmt::cast(syntax.clone()) {
            Some(PyAst::NonlocalStmt(node))
        } else if let Some(node) = PyImportStmt::cast(syntax.clone()) {
            Some(PyAst::ImportStmt(node))
        } else if let Some(node) = PyImportFromStmt::cast(syntax.clone()) {
            Some(PyAst::ImportFromStmt(node))
        } else if let Some(node) = PyMatchStmt::cast(syntax.clone()) {
            Some(PyAst::MatchStmt(node))
        } else if let Some(node) = PyElseStmt::cast(syntax.clone()) {
            Some(PyAst::ElseStmt(node))
        } else if let Some(node) = PyElifStmt::cast(syntax.clone()) {
            Some(PyAst::ElifStmt(node))
        // Expression types
        } else if let Some(node) = PyNameExpr::cast(syntax.clone()) {
            Some(PyAst::NameExpr(node))
        } else if let Some(node) = PyLiteralExpr::cast(syntax.clone()) {
            Some(PyAst::LiteralExpr(node))
        } else if let Some(node) = PyParenExpr::cast(syntax.clone()) {
            Some(PyAst::ParenExpr(node))
        } else if let Some(node) = PyTupleExpr::cast(syntax.clone()) {
            Some(PyAst::TupleExpr(node))
        } else if let Some(node) = PyListExpr::cast(syntax.clone()) {
            Some(PyAst::ListExpr(node))
        } else if let Some(node) = PyDictExpr::cast(syntax.clone()) {
            Some(PyAst::DictExpr(node))
        } else if let Some(node) = PySetExpr::cast(syntax.clone()) {
            Some(PyAst::SetExpr(node))
        } else if let Some(node) = PyBinaryExpr::cast(syntax.clone()) {
            Some(PyAst::BinaryExpr(node))
        } else if let Some(node) = PyUnaryExpr::cast(syntax.clone()) {
            Some(PyAst::UnaryExpr(node))
        } else if let Some(node) = PyBoolOpExpr::cast(syntax.clone()) {
            Some(PyAst::BoolOpExpr(node))
        } else if let Some(node) = PyCompareExpr::cast(syntax.clone()) {
            Some(PyAst::CompareExpr(node))
        } else if let Some(node) = PyCallExpr::cast(syntax.clone()) {
            Some(PyAst::CallExpr(node))
        } else if let Some(node) = PyMethodCallExpr::cast(syntax.clone()) {
            Some(PyAst::MethodCallExpr(node))
        } else if let Some(node) = PyAttributeExpr::cast(syntax.clone()) {
            Some(PyAst::AttributeExpr(node))
        } else if let Some(node) = PySubscriptExpr::cast(syntax.clone()) {
            Some(PyAst::SubscriptExpr(node))
        } else if let Some(node) = PySliceExpr::cast(syntax.clone()) {
            Some(PyAst::SliceExpr(node))
        } else if let Some(node) = PyLambdaExpr::cast(syntax.clone()) {
            Some(PyAst::LambdaExpr(node))
        } else if let Some(node) = PyIfExpr::cast(syntax.clone()) {
            Some(PyAst::IfExpr(node))
        } else if let Some(node) = PyYieldExpr::cast(syntax.clone()) {
            Some(PyAst::YieldExpr(node))
        } else if let Some(node) = PyYieldFromExpr::cast(syntax.clone()) {
            Some(PyAst::YieldFromExpr(node))
        } else if let Some(node) = PyAwaitExpr::cast(syntax.clone()) {
            Some(PyAst::AwaitExpr(node))
        } else if let Some(node) = PyStarredExpr::cast(syntax.clone()) {
            Some(PyAst::StarredExpr(node))
        } else if let Some(node) = PyDoubleStarredExpr::cast(syntax.clone()) {
            Some(PyAst::DoubleStarredExpr(node))
        } else if let Some(node) = PyAssignExpr::cast(syntax.clone()) {
            Some(PyAst::AssignExpr(node))
        } else if let Some(node) = PyConditionalExpr::cast(syntax.clone()) {
            Some(PyAst::ConditionalExpr(node))
        } else if let Some(node) = PyListCompExpr::cast(syntax.clone()) {
            Some(PyAst::ListCompExpr(node))
        } else if let Some(node) = PyDictCompExpr::cast(syntax.clone()) {
            Some(PyAst::DictCompExpr(node))
        } else if let Some(node) = PySetCompExpr::cast(syntax.clone()) {
            Some(PyAst::SetCompExpr(node))
        } else if let Some(node) = PyGeneratorExpr::cast(syntax.clone()) {
            Some(PyAst::GeneratorExpr(node))
        // Union types for backward compatibility
        } else if let Some(node) = PyStat::cast(syntax.clone()) {
            Some(PyAst::Stat(node))
        } else if let Some(node) = PyExpr::cast(syntax.clone()) {
            Some(PyAst::Expr(node))
        } else {
            None
        }
    }
}
