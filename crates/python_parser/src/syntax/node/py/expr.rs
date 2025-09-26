use crate::{
    kind::PySyntaxKind,
    syntax::traits::{PyAstNode, PySyntaxNode},
};

macro_rules! py_expr_ast {
    ($($ast_name:ident : $syntax_kind:ident),* $(,)?) => {
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
                    kind == PySyntaxKind::$syntax_kind
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

py_expr_ast!(
    PyNameExpr: NameExpr,
    PyLiteralExpr: LiteralExpr,
    PyParenExpr: ParenExpr,
    PyTupleExpr: TupleExpr,
    PyListExpr: ListExpr,
    PyDictExpr: DictExpr,
    PySetExpr: SetExpr,
    PyBinaryExpr: BinaryExpr,
    PyUnaryExpr: UnaryExpr,
    PyBoolOpExpr: BoolOpExpr,
    PyCompareExpr: CompareExpr,
    PyCallExpr: CallExpr,
    PyMethodCallExpr: MethodCallExpr,
    PyAttributeExpr: AttributeExpr,
    PySubscriptExpr: SubscriptExpr,
    PySliceExpr: SliceExpr,
    PyLambdaExpr: LambdaExpr,
    PyIfExpr: IfExpr,
    PyYieldExpr: YieldExpr,
    PyYieldFromExpr: YieldFromExpr,
    PyAwaitExpr: AwaitExpr,
    PyStarredExpr: StarredExpr,
    PyDoubleStarredExpr: DoubleStarredExpr,
    PyAssignExpr: AssignExpr,
    PyConditionalExpr: ConditionalExpr,
    PyListCompExpr: ListCompExpr,
    PyDictCompExpr: DictCompExpr,
    PySetCompExpr: SetCompExpr,
    PyGeneratorExpr: GeneratorExpr
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyExpr {
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

impl PyAstNode for PyExpr {
    fn syntax(&self) -> &PySyntaxNode {
        match self {
            PyExpr::NameExpr(node) => node.syntax(),
            PyExpr::LiteralExpr(node) => node.syntax(),
            PyExpr::ParenExpr(node) => node.syntax(),
            PyExpr::TupleExpr(node) => node.syntax(),
            PyExpr::ListExpr(node) => node.syntax(),
            PyExpr::DictExpr(node) => node.syntax(),
            PyExpr::SetExpr(node) => node.syntax(),
            PyExpr::BinaryExpr(node) => node.syntax(),
            PyExpr::UnaryExpr(node) => node.syntax(),
            PyExpr::BoolOpExpr(node) => node.syntax(),
            PyExpr::CompareExpr(node) => node.syntax(),
            PyExpr::CallExpr(node) => node.syntax(),
            PyExpr::MethodCallExpr(node) => node.syntax(),
            PyExpr::AttributeExpr(node) => node.syntax(),
            PyExpr::SubscriptExpr(node) => node.syntax(),
            PyExpr::SliceExpr(node) => node.syntax(),
            PyExpr::LambdaExpr(node) => node.syntax(),
            PyExpr::IfExpr(node) => node.syntax(),
            PyExpr::YieldExpr(node) => node.syntax(),
            PyExpr::YieldFromExpr(node) => node.syntax(),
            PyExpr::AwaitExpr(node) => node.syntax(),
            PyExpr::StarredExpr(node) => node.syntax(),
            PyExpr::DoubleStarredExpr(node) => node.syntax(),
            PyExpr::AssignExpr(node) => node.syntax(),
            PyExpr::ConditionalExpr(node) => node.syntax(),
            PyExpr::ListCompExpr(node) => node.syntax(),
            PyExpr::DictCompExpr(node) => node.syntax(),
            PyExpr::SetCompExpr(node) => node.syntax(),
            PyExpr::GeneratorExpr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::NameExpr
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
                | PySyntaxKind::IfExpr
                | PySyntaxKind::YieldExpr
                | PySyntaxKind::YieldFromExpr
                | PySyntaxKind::AwaitExpr
                | PySyntaxKind::StarredExpr
                | PySyntaxKind::DoubleStarredExpr
                | PySyntaxKind::AssignExpr
                | PySyntaxKind::ConditionalExpr
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
        match syntax.kind().into() {
            PySyntaxKind::NameExpr => PyNameExpr::cast(syntax).map(PyExpr::NameExpr),
            PySyntaxKind::LiteralExpr => PyLiteralExpr::cast(syntax).map(PyExpr::LiteralExpr),
            PySyntaxKind::ParenExpr => PyParenExpr::cast(syntax).map(PyExpr::ParenExpr),
            PySyntaxKind::TupleExpr => PyTupleExpr::cast(syntax).map(PyExpr::TupleExpr),
            PySyntaxKind::ListExpr => PyListExpr::cast(syntax).map(PyExpr::ListExpr),
            PySyntaxKind::DictExpr => PyDictExpr::cast(syntax).map(PyExpr::DictExpr),
            PySyntaxKind::SetExpr => PySetExpr::cast(syntax).map(PyExpr::SetExpr),
            PySyntaxKind::BinaryExpr => PyBinaryExpr::cast(syntax).map(PyExpr::BinaryExpr),
            PySyntaxKind::UnaryExpr => PyUnaryExpr::cast(syntax).map(PyExpr::UnaryExpr),
            PySyntaxKind::BoolOpExpr => PyBoolOpExpr::cast(syntax).map(PyExpr::BoolOpExpr),
            PySyntaxKind::CompareExpr => PyCompareExpr::cast(syntax).map(PyExpr::CompareExpr),
            PySyntaxKind::CallExpr => PyCallExpr::cast(syntax).map(PyExpr::CallExpr),
            PySyntaxKind::MethodCallExpr => {
                PyMethodCallExpr::cast(syntax).map(PyExpr::MethodCallExpr)
            }
            PySyntaxKind::AttributeExpr => PyAttributeExpr::cast(syntax).map(PyExpr::AttributeExpr),
            PySyntaxKind::SubscriptExpr => PySubscriptExpr::cast(syntax).map(PyExpr::SubscriptExpr),
            PySyntaxKind::SliceExpr => PySliceExpr::cast(syntax).map(PyExpr::SliceExpr),
            PySyntaxKind::LambdaExpr => PyLambdaExpr::cast(syntax).map(PyExpr::LambdaExpr),
            PySyntaxKind::IfExpr => PyIfExpr::cast(syntax).map(PyExpr::IfExpr),
            PySyntaxKind::YieldExpr => PyYieldExpr::cast(syntax).map(PyExpr::YieldExpr),
            PySyntaxKind::YieldFromExpr => PyYieldFromExpr::cast(syntax).map(PyExpr::YieldFromExpr),
            PySyntaxKind::AwaitExpr => PyAwaitExpr::cast(syntax).map(PyExpr::AwaitExpr),
            PySyntaxKind::StarredExpr => PyStarredExpr::cast(syntax).map(PyExpr::StarredExpr),
            PySyntaxKind::DoubleStarredExpr => {
                PyDoubleStarredExpr::cast(syntax).map(PyExpr::DoubleStarredExpr)
            }
            PySyntaxKind::AssignExpr => PyAssignExpr::cast(syntax).map(PyExpr::AssignExpr),
            PySyntaxKind::ConditionalExpr => {
                PyConditionalExpr::cast(syntax).map(PyExpr::ConditionalExpr)
            }
            PySyntaxKind::ListCompExpr => PyListCompExpr::cast(syntax).map(PyExpr::ListCompExpr),
            PySyntaxKind::DictCompExpr => PyDictCompExpr::cast(syntax).map(PyExpr::DictCompExpr),
            PySyntaxKind::SetCompExpr => PySetCompExpr::cast(syntax).map(PyExpr::SetCompExpr),
            PySyntaxKind::GeneratorExpr => PyGeneratorExpr::cast(syntax).map(PyExpr::GeneratorExpr),
            _ => None,
        }
    }
}
