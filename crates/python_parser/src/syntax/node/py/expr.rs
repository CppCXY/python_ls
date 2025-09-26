use super::PyArguments;
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

// 为每个表达式节点实现子节点访问方法
impl PyNameExpr {
    pub fn get_id(&self) -> Option<String> {
        self.syntax()
            .first_token()
            .map(|token| token.text().to_string())
    }
}

impl PyLiteralExpr {
    pub fn get_value(&self) -> Option<String> {
        self.syntax()
            .first_token()
            .map(|token| token.text().to_string())
    }
}

impl PyListExpr {
    pub fn get_elements(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }
}

impl PyTupleExpr {
    pub fn get_elements(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }
}

impl PySetExpr {
    pub fn get_elements(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }
}

impl PyDictExpr {
    pub fn get_keys(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).step_by(2)
    }

    pub fn get_values(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax()
            .children()
            .filter_map(PyExpr::cast)
            .skip(1)
            .step_by(2)
    }
}

impl PyGeneratorExpr {
    pub fn get_element(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_generators(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(1)
    }
}

impl PyListCompExpr {
    pub fn get_element(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_generators(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(1)
    }
}

impl PyDictCompExpr {
    pub fn get_key(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_generators(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(2)
    }
}

impl PySetCompExpr {
    pub fn get_element(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_generators(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(1)
    }
}

impl PyAttributeExpr {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_attr(&self) -> Option<String> {
        self.syntax()
            .children()
            .filter_map(PyNameExpr::cast)
            .last()
            .and_then(|name| name.get_id())
    }
}

impl PySubscriptExpr {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_slice(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }
}

impl PySliceExpr {
    pub fn get_lower(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_upper(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_step(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(2)
    }
}

impl PyStarredExpr {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyDoubleStarredExpr {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyUnaryExpr {
    pub fn get_operand(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_op(&self) -> Option<String> {
        self.syntax()
            .first_token()
            .map(|token| token.text().to_string())
    }
}

impl PyBinaryExpr {
    pub fn get_left(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_right(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_op(&self) -> Option<String> {
        // 二元操作符通常在两个表达式之间
        self.syntax()
            .children_with_tokens()
            .find_map(|child| child.into_token())
            .map(|token| token.text().to_string())
    }
}

impl PyBoolOpExpr {
    pub fn get_op(&self) -> Option<String> {
        // and 或 or
        self.syntax()
            .children_with_tokens()
            .find_map(|child| child.into_token())
            .map(|token| token.text().to_string())
    }

    pub fn get_values(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }
}

impl PyCompareExpr {
    pub fn get_left(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_comparators(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(1)
    }
}

impl PyCallExpr {
    pub fn get_func(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_args(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(1)
    }
}

impl PyIfExpr {
    pub fn get_test(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_body(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_orelse(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(2)
    }
}

impl PyConditionalExpr {
    pub fn get_test(&self) -> Option<PyExpr> {
        // if 子句中的条件表达式
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_body(&self) -> Option<PyExpr> {
        // if 之前的表达式
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_orelse(&self) -> Option<PyExpr> {
        // else 之后的表达式
        self.syntax().children().filter_map(PyExpr::cast).nth(2)
    }
}

impl PyLambdaExpr {
    pub fn get_parameters(&self) -> Option<PyArguments> {
        self.syntax().children().find_map(PyArguments::cast)
    }

    pub fn get_body(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).last()
    }
}

impl PyYieldExpr {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyYieldFromExpr {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyAwaitExpr {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyAssignExpr {
    pub fn get_target(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }
}
