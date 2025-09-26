pub mod common;
pub mod expr;
pub mod stat;
mod test;

pub use common::*;
pub use expr::*;
pub use stat::*;

use crate::{
    kind::{PyKind, PySyntaxKind},
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
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            // Core structural nodes
            PySyntaxKind::Module
            | PySyntaxKind::Suite
            | PySyntaxKind::Parameter
            | PySyntaxKind::Arguments
            | PySyntaxKind::Decorator
            | PySyntaxKind::CaseClause
            // Statement types
            | PySyntaxKind::ExprStmt
            | PySyntaxKind::AssignStmt
            | PySyntaxKind::AnnAssignStmt
            | PySyntaxKind::AugAssignStmt
            | PySyntaxKind::FuncDef
            | PySyntaxKind::AsyncFuncDef
            | PySyntaxKind::ClassDef
            | PySyntaxKind::IfStmt
            | PySyntaxKind::WhileStmt
            | PySyntaxKind::ForStmt
            | PySyntaxKind::AsyncForStmt
            | PySyntaxKind::WithStmt
            | PySyntaxKind::AsyncWithStmt
            | PySyntaxKind::TryStmt
            | PySyntaxKind::BreakStmt
            | PySyntaxKind::ContinueStmt
            | PySyntaxKind::ReturnStmt
            | PySyntaxKind::YieldStmt
            | PySyntaxKind::RaiseStmt
            | PySyntaxKind::AssertStmt
            | PySyntaxKind::DeleteStmt
            | PySyntaxKind::PassStmt
            | PySyntaxKind::GlobalStmt
            | PySyntaxKind::NonlocalStmt
            | PySyntaxKind::ImportStmt
            | PySyntaxKind::ImportFromStmt
            | PySyntaxKind::MatchStmt
            | PySyntaxKind::ElseClause
            | PySyntaxKind::ElifClause
            // Expression types
            | PySyntaxKind::NameExpr
            | PySyntaxKind::LiteralExpr
            | PySyntaxKind::ParenExpr
            | PySyntaxKind::TupleExpr
            | PySyntaxKind::ListExpr
            | PySyntaxKind::DictExpr
            | PySyntaxKind::SetExpr
            | PySyntaxKind::BinaryExpr
            | PySyntaxKind::UnaryExpr
            | PySyntaxKind::BoolOpExpr
            | PySyntaxKind::CompareExpr
            | PySyntaxKind::CallExpr
            | PySyntaxKind::MethodCallExpr
            | PySyntaxKind::AttributeExpr
            | PySyntaxKind::SubscriptExpr
            | PySyntaxKind::SliceExpr
            | PySyntaxKind::LambdaExpr
            | PySyntaxKind::ConditionalExpr
            | PySyntaxKind::YieldExpr
            | PySyntaxKind::YieldFromExpr
            | PySyntaxKind::AwaitExpr
            | PySyntaxKind::StarredExpr
            | PySyntaxKind::DoubleStarredExpr
            | PySyntaxKind::AssignExpr
            | PySyntaxKind::ListCompExpr
            | PySyntaxKind::DictCompExpr
            | PySyntaxKind::SetCompExpr
            | PySyntaxKind::GeneratorExpr
        )
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            // Core structural nodes
            PyKind::Syntax(PySyntaxKind::Module) => PyModule::cast(syntax).map(PyAst::Module),
            PyKind::Syntax(PySyntaxKind::Suite) => PySuite::cast(syntax).map(PyAst::Suite),
            PyKind::Syntax(PySyntaxKind::Parameter) => {
                PyParameter::cast(syntax).map(PyAst::Parameter)
            }
            PyKind::Syntax(PySyntaxKind::Arguments) => {
                PyArguments::cast(syntax).map(PyAst::Arguments)
            }
            PyKind::Syntax(PySyntaxKind::Decorator) => {
                PyDecorator::cast(syntax).map(PyAst::Decorator)
            }
            PyKind::Syntax(PySyntaxKind::CaseClause) => {
                PyCaseClause::cast(syntax).map(PyAst::CaseClause)
            }

            // Statement types
            PyKind::Syntax(PySyntaxKind::ExprStmt) => PyExprStmt::cast(syntax).map(PyAst::ExprStmt),
            PyKind::Syntax(PySyntaxKind::AssignStmt) => {
                PyAssignStmt::cast(syntax).map(PyAst::AssignStmt)
            }
            PyKind::Syntax(PySyntaxKind::AnnAssignStmt) => {
                PyAnnAssignStmt::cast(syntax).map(PyAst::AnnAssignStmt)
            }
            PyKind::Syntax(PySyntaxKind::AugAssignStmt) => {
                PyAugAssignStmt::cast(syntax).map(PyAst::AugAssignStmt)
            }
            PyKind::Syntax(PySyntaxKind::FuncDef) => PyFuncDef::cast(syntax).map(PyAst::FuncDef),
            PyKind::Syntax(PySyntaxKind::AsyncFuncDef) => {
                PyAsyncFuncDef::cast(syntax).map(PyAst::AsyncFuncDef)
            }
            PyKind::Syntax(PySyntaxKind::ClassDef) => PyClassDef::cast(syntax).map(PyAst::ClassDef),
            PyKind::Syntax(PySyntaxKind::IfStmt) => PyIfStmt::cast(syntax).map(PyAst::IfStmt),
            PyKind::Syntax(PySyntaxKind::WhileStmt) => {
                PyWhileStmt::cast(syntax).map(PyAst::WhileStmt)
            }
            PyKind::Syntax(PySyntaxKind::ForStmt) => PyForStmt::cast(syntax).map(PyAst::ForStmt),
            PyKind::Syntax(PySyntaxKind::AsyncForStmt) => {
                PyAsyncForStmt::cast(syntax).map(PyAst::AsyncForStmt)
            }
            PyKind::Syntax(PySyntaxKind::WithStmt) => PyWithStmt::cast(syntax).map(PyAst::WithStmt),
            PyKind::Syntax(PySyntaxKind::AsyncWithStmt) => {
                PyAsyncWithStmt::cast(syntax).map(PyAst::AsyncWithStmt)
            }
            PyKind::Syntax(PySyntaxKind::TryStmt) => PyTryStmt::cast(syntax).map(PyAst::TryStmt),
            PyKind::Syntax(PySyntaxKind::BreakStmt) => {
                PyBreakStmt::cast(syntax).map(PyAst::BreakStmt)
            }
            PyKind::Syntax(PySyntaxKind::ContinueStmt) => {
                PyContinueStmt::cast(syntax).map(PyAst::ContinueStmt)
            }
            PyKind::Syntax(PySyntaxKind::ReturnStmt) => {
                PyReturnStmt::cast(syntax).map(PyAst::ReturnStmt)
            }
            PyKind::Syntax(PySyntaxKind::YieldStmt) => {
                PyYieldStmt::cast(syntax).map(PyAst::YieldStmt)
            }
            PyKind::Syntax(PySyntaxKind::RaiseStmt) => {
                PyRaiseStmt::cast(syntax).map(PyAst::RaiseStmt)
            }
            PyKind::Syntax(PySyntaxKind::AssertStmt) => {
                PyAssertStmt::cast(syntax).map(PyAst::AssertStmt)
            }
            PyKind::Syntax(PySyntaxKind::DeleteStmt) => {
                PyDeleteStmt::cast(syntax).map(PyAst::DeleteStmt)
            }
            PyKind::Syntax(PySyntaxKind::PassStmt) => PyPassStmt::cast(syntax).map(PyAst::PassStmt),
            PyKind::Syntax(PySyntaxKind::GlobalStmt) => {
                PyGlobalStmt::cast(syntax).map(PyAst::GlobalStmt)
            }
            PyKind::Syntax(PySyntaxKind::NonlocalStmt) => {
                PyNonlocalStmt::cast(syntax).map(PyAst::NonlocalStmt)
            }
            PyKind::Syntax(PySyntaxKind::ImportStmt) => {
                PyImportStmt::cast(syntax).map(PyAst::ImportStmt)
            }
            PyKind::Syntax(PySyntaxKind::ImportFromStmt) => {
                PyImportFromStmt::cast(syntax).map(PyAst::ImportFromStmt)
            }
            PyKind::Syntax(PySyntaxKind::MatchStmt) => {
                PyMatchStmt::cast(syntax).map(PyAst::MatchStmt)
            }
            PyKind::Syntax(PySyntaxKind::ElseClause) => {
                PyElseStmt::cast(syntax).map(PyAst::ElseStmt)
            }
            PyKind::Syntax(PySyntaxKind::ElifClause) => {
                PyElifStmt::cast(syntax).map(PyAst::ElifStmt)
            }

            // Expression types
            PyKind::Syntax(PySyntaxKind::NameExpr) => PyNameExpr::cast(syntax).map(PyAst::NameExpr),
            PyKind::Syntax(PySyntaxKind::LiteralExpr) => {
                PyLiteralExpr::cast(syntax).map(PyAst::LiteralExpr)
            }
            PyKind::Syntax(PySyntaxKind::ParenExpr) => {
                PyParenExpr::cast(syntax).map(PyAst::ParenExpr)
            }
            PyKind::Syntax(PySyntaxKind::TupleExpr) => {
                PyTupleExpr::cast(syntax).map(PyAst::TupleExpr)
            }
            PyKind::Syntax(PySyntaxKind::ListExpr) => PyListExpr::cast(syntax).map(PyAst::ListExpr),
            PyKind::Syntax(PySyntaxKind::DictExpr) => PyDictExpr::cast(syntax).map(PyAst::DictExpr),
            PyKind::Syntax(PySyntaxKind::SetExpr) => PySetExpr::cast(syntax).map(PyAst::SetExpr),
            PyKind::Syntax(PySyntaxKind::BinaryExpr) => {
                PyBinaryExpr::cast(syntax).map(PyAst::BinaryExpr)
            }
            PyKind::Syntax(PySyntaxKind::UnaryExpr) => {
                PyUnaryExpr::cast(syntax).map(PyAst::UnaryExpr)
            }
            PyKind::Syntax(PySyntaxKind::BoolOpExpr) => {
                PyBoolOpExpr::cast(syntax).map(PyAst::BoolOpExpr)
            }
            PyKind::Syntax(PySyntaxKind::CompareExpr) => {
                PyCompareExpr::cast(syntax).map(PyAst::CompareExpr)
            }
            PyKind::Syntax(PySyntaxKind::CallExpr) => PyCallExpr::cast(syntax).map(PyAst::CallExpr),
            PyKind::Syntax(PySyntaxKind::MethodCallExpr) => {
                PyMethodCallExpr::cast(syntax).map(PyAst::MethodCallExpr)
            }
            PyKind::Syntax(PySyntaxKind::AttributeExpr) => {
                PyAttributeExpr::cast(syntax).map(PyAst::AttributeExpr)
            }
            PyKind::Syntax(PySyntaxKind::SubscriptExpr) => {
                PySubscriptExpr::cast(syntax).map(PyAst::SubscriptExpr)
            }
            PyKind::Syntax(PySyntaxKind::SliceExpr) => {
                PySliceExpr::cast(syntax).map(PyAst::SliceExpr)
            }
            PyKind::Syntax(PySyntaxKind::LambdaExpr) => {
                PyLambdaExpr::cast(syntax).map(PyAst::LambdaExpr)
            }
            PyKind::Syntax(PySyntaxKind::ConditionalExpr) => {
                PyIfExpr::cast(syntax).map(PyAst::IfExpr)
            }
            PyKind::Syntax(PySyntaxKind::YieldExpr) => {
                PyYieldExpr::cast(syntax).map(PyAst::YieldExpr)
            }
            PyKind::Syntax(PySyntaxKind::YieldFromExpr) => {
                PyYieldFromExpr::cast(syntax).map(PyAst::YieldFromExpr)
            }
            PyKind::Syntax(PySyntaxKind::AwaitExpr) => {
                PyAwaitExpr::cast(syntax).map(PyAst::AwaitExpr)
            }
            PyKind::Syntax(PySyntaxKind::StarredExpr) => {
                PyStarredExpr::cast(syntax).map(PyAst::StarredExpr)
            }
            PyKind::Syntax(PySyntaxKind::DoubleStarredExpr) => {
                PyDoubleStarredExpr::cast(syntax).map(PyAst::DoubleStarredExpr)
            }
            PyKind::Syntax(PySyntaxKind::AssignExpr) => {
                PyAssignExpr::cast(syntax).map(PyAst::AssignExpr)
            }
            PyKind::Syntax(PySyntaxKind::ListCompExpr) => {
                PyListCompExpr::cast(syntax).map(PyAst::ListCompExpr)
            }
            PyKind::Syntax(PySyntaxKind::DictCompExpr) => {
                PyDictCompExpr::cast(syntax).map(PyAst::DictCompExpr)
            }
            PyKind::Syntax(PySyntaxKind::SetCompExpr) => {
                PySetCompExpr::cast(syntax).map(PyAst::SetCompExpr)
            }
            PyKind::Syntax(PySyntaxKind::GeneratorExpr) => {
                PyGeneratorExpr::cast(syntax).map(PyAst::GeneratorExpr)
            }

            _ => None,
        }
    }
}
