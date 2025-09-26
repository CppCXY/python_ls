use super::{PyExpr, PyNameExpr, PyStat};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCaseClause {
    syntax: PySyntaxNode,
}

impl PyAstNode for PyCaseClause {
    fn syntax(&self) -> &PySyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::CaseClause
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

// 为通用节点实现子节点访问方法
impl PyModule {
    pub fn get_suite(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }
}

impl PySuite {
    pub fn get_statements(&self) -> impl Iterator<Item = PyStat> + '_ {
        self.syntax().children().filter_map(PyStat::cast)
    }
}

impl PyParameter {
    pub fn get_name(&self) -> Option<PyNameExpr> {
        self.syntax().children().find_map(PyNameExpr::cast)
    }

    pub fn get_annotation(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).last()
    }

    pub fn get_default_value(&self) -> Option<PyExpr> {
        // 默认值通常是最后一个表达式（如果存在注解的话）
        let exprs: Vec<_> = self.syntax().children().filter_map(PyExpr::cast).collect();
        let count = exprs.len();
        if count > 1 {
            exprs.into_iter().nth(count - 2)
        } else if count == 1 {
            // 如果只有一个表达式，可能是默认值或注解，需要根据语法判断
            None
        } else {
            None
        }
    }
}

impl PyArguments {
    pub fn get_args(&self) -> impl Iterator<Item = PyParameter> + '_ {
        self.syntax().children().filter_map(PyParameter::cast)
    }

    pub fn get_posonlyargs(&self) -> impl Iterator<Item = PyParameter> + '_ {
        // 位置参数在 / 符号之前
        self.syntax()
            .children()
            .filter_map(PyParameter::cast)
            .take_while(|_| {
                // 这里需要更精确的逻辑来识别 / 分隔符
                true
            })
    }

    pub fn get_kwonlyargs(&self) -> impl Iterator<Item = PyParameter> + '_ {
        // 关键字参数在 * 符号之后
        self.syntax()
            .children()
            .filter_map(PyParameter::cast)
            .skip_while(|_| {
                // 这里需要更精确的逻辑来识别 * 分隔符
                true
            })
    }
}

impl PyDecorator {
    pub fn get_name(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_args(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(1)
    }
}

impl PyCaseClause {
    pub fn get_pattern(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_guard(&self) -> Option<PyExpr> {
        // guard 是 if 后面的表达式
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }
}
