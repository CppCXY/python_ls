use super::{PyArguments, PyCaseClause, PyDecorator, PyExpr, PyNameExpr, PySuite};
use crate::{
    kind::PySyntaxKind,
    syntax::traits::{PyAstNode, PySyntaxNode},
};

macro_rules! py_stat_ast {
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

py_stat_ast!(
    PyExprStmt: ExprStmt,
    PyAssignStmt: AssignStmt,
    PyAnnAssignStmt: AnnAssignStmt,
    PyAugAssignStmt: AugAssignStmt,
    PyFuncDef: FuncDef,
    PyAsyncFuncDef: AsyncFuncDef,
    PyClassDef: ClassDef,
    PyIfStmt: IfStmt,
    PyWhileStmt: WhileStmt,
    PyForStmt: ForStmt,
    PyAsyncForStmt: AsyncForStmt,
    PyWithStmt: WithStmt,
    PyAsyncWithStmt: AsyncWithStmt,
    PyTryStmt: TryStmt,
    PyBreakStmt: BreakStmt,
    PyContinueStmt: ContinueStmt,
    PyReturnStmt: ReturnStmt,
    PyYieldStmt: YieldStmt,
    PyRaiseStmt: RaiseStmt,
    PyAssertStmt: AssertStmt,
    PyDeleteStmt: DeleteStmt,
    PyPassStmt: PassStmt,
    PyGlobalStmt: GlobalStmt,
    PyNonlocalStmt: NonlocalStmt,
    PyImportStmt: ImportStmt,
    PyImportFromStmt: ImportFromStmt,
    PyMatchStmt: MatchStmt,
    PyElseStmt: ElseClause,
    PyElifStmt: ElifClause,
    // Python 3.11+ Exception Groups
    PyTryStarStmt: TryStarStmt,
    PyExceptStarClause: ExceptStarClause,
    // Python 3.12+ Type Parameters and Type Statements
    PyTypeStatement: TypeStatement,
    PyTypeAliasStmt: TypeAliasStmt,
    PyGenericFuncDef: GenericFuncDef,
    PyGenericClassDef: GenericClassDef,
    // Python 3.14+ Experimental Features
    PyDecorated: Decorated,
    PyAsyncCompStmt: AsyncCompStmt
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyStat {
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
    // Python 3.11+ Exception Groups
    TryStarStmt(PyTryStarStmt),
    ExceptStarClause(PyExceptStarClause),
    // Python 3.12+ Type Parameters and Type Statements
    TypeStatement(PyTypeStatement),
    TypeAliasStmt(PyTypeAliasStmt),
    GenericFuncDef(PyGenericFuncDef),
    GenericClassDef(PyGenericClassDef),
    // Python 3.14+ Experimental Features
    Decorated(PyDecorated),
    AsyncCompStmt(PyAsyncCompStmt),
}

impl PyAstNode for PyStat {
    fn syntax(&self) -> &PySyntaxNode {
        match self {
            PyStat::ExprStmt(node) => node.syntax(),
            PyStat::AssignStmt(node) => node.syntax(),
            PyStat::AnnAssignStmt(node) => node.syntax(),
            PyStat::AugAssignStmt(node) => node.syntax(),
            PyStat::FuncDef(node) => node.syntax(),
            PyStat::AsyncFuncDef(node) => node.syntax(),
            PyStat::ClassDef(node) => node.syntax(),
            PyStat::IfStmt(node) => node.syntax(),
            PyStat::WhileStmt(node) => node.syntax(),
            PyStat::ForStmt(node) => node.syntax(),
            PyStat::AsyncForStmt(node) => node.syntax(),
            PyStat::WithStmt(node) => node.syntax(),
            PyStat::AsyncWithStmt(node) => node.syntax(),
            PyStat::TryStmt(node) => node.syntax(),
            PyStat::BreakStmt(node) => node.syntax(),
            PyStat::ContinueStmt(node) => node.syntax(),
            PyStat::ReturnStmt(node) => node.syntax(),
            PyStat::YieldStmt(node) => node.syntax(),
            PyStat::RaiseStmt(node) => node.syntax(),
            PyStat::AssertStmt(node) => node.syntax(),
            PyStat::DeleteStmt(node) => node.syntax(),
            PyStat::PassStmt(node) => node.syntax(),
            PyStat::GlobalStmt(node) => node.syntax(),
            PyStat::NonlocalStmt(node) => node.syntax(),
            PyStat::ImportStmt(node) => node.syntax(),
            PyStat::ImportFromStmt(node) => node.syntax(),
            PyStat::MatchStmt(node) => node.syntax(),
            PyStat::ElseStmt(node) => node.syntax(),
            PyStat::ElifStmt(node) => node.syntax(),
            // Python 3.11+ Exception Groups
            PyStat::TryStarStmt(node) => node.syntax(),
            PyStat::ExceptStarClause(node) => node.syntax(),
            // Python 3.12+ Type Parameters and Type Statements
            PyStat::TypeStatement(node) => node.syntax(),
            PyStat::TypeAliasStmt(node) => node.syntax(),
            PyStat::GenericFuncDef(node) => node.syntax(),
            PyStat::GenericClassDef(node) => node.syntax(),
            // Python 3.14+ Experimental Features
            PyStat::Decorated(node) => node.syntax(),
            PyStat::AsyncCompStmt(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::ExprStmt
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
                // Python 3.11+ Exception Groups
                | PySyntaxKind::TryStarStmt
                | PySyntaxKind::ExceptStarClause
                // Python 3.12+ Type Parameters and Type Statements
                | PySyntaxKind::TypeStatement
                | PySyntaxKind::TypeAliasStmt
                | PySyntaxKind::GenericFuncDef
                | PySyntaxKind::GenericClassDef
                // Python 3.14+ Experimental Features
                | PySyntaxKind::Decorated
                | PySyntaxKind::AsyncCompStmt
        )
    }

    fn cast(syntax: PySyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            PySyntaxKind::ExprStmt => PyExprStmt::cast(syntax).map(PyStat::ExprStmt),
            PySyntaxKind::AssignStmt => PyAssignStmt::cast(syntax).map(PyStat::AssignStmt),
            PySyntaxKind::AnnAssignStmt => PyAnnAssignStmt::cast(syntax).map(PyStat::AnnAssignStmt),
            PySyntaxKind::AugAssignStmt => PyAugAssignStmt::cast(syntax).map(PyStat::AugAssignStmt),
            PySyntaxKind::FuncDef => PyFuncDef::cast(syntax).map(PyStat::FuncDef),
            PySyntaxKind::AsyncFuncDef => PyAsyncFuncDef::cast(syntax).map(PyStat::AsyncFuncDef),
            PySyntaxKind::ClassDef => PyClassDef::cast(syntax).map(PyStat::ClassDef),
            PySyntaxKind::IfStmt => PyIfStmt::cast(syntax).map(PyStat::IfStmt),
            PySyntaxKind::WhileStmt => PyWhileStmt::cast(syntax).map(PyStat::WhileStmt),
            PySyntaxKind::ForStmt => PyForStmt::cast(syntax).map(PyStat::ForStmt),
            PySyntaxKind::AsyncForStmt => PyAsyncForStmt::cast(syntax).map(PyStat::AsyncForStmt),
            PySyntaxKind::WithStmt => PyWithStmt::cast(syntax).map(PyStat::WithStmt),
            PySyntaxKind::AsyncWithStmt => PyAsyncWithStmt::cast(syntax).map(PyStat::AsyncWithStmt),
            PySyntaxKind::TryStmt => PyTryStmt::cast(syntax).map(PyStat::TryStmt),
            PySyntaxKind::BreakStmt => PyBreakStmt::cast(syntax).map(PyStat::BreakStmt),
            PySyntaxKind::ContinueStmt => PyContinueStmt::cast(syntax).map(PyStat::ContinueStmt),
            PySyntaxKind::ReturnStmt => PyReturnStmt::cast(syntax).map(PyStat::ReturnStmt),
            PySyntaxKind::YieldStmt => PyYieldStmt::cast(syntax).map(PyStat::YieldStmt),
            PySyntaxKind::RaiseStmt => PyRaiseStmt::cast(syntax).map(PyStat::RaiseStmt),
            PySyntaxKind::AssertStmt => PyAssertStmt::cast(syntax).map(PyStat::AssertStmt),
            PySyntaxKind::DeleteStmt => PyDeleteStmt::cast(syntax).map(PyStat::DeleteStmt),
            PySyntaxKind::PassStmt => PyPassStmt::cast(syntax).map(PyStat::PassStmt),
            PySyntaxKind::GlobalStmt => PyGlobalStmt::cast(syntax).map(PyStat::GlobalStmt),
            PySyntaxKind::NonlocalStmt => PyNonlocalStmt::cast(syntax).map(PyStat::NonlocalStmt),
            PySyntaxKind::ImportStmt => PyImportStmt::cast(syntax).map(PyStat::ImportStmt),
            PySyntaxKind::ImportFromStmt => {
                PyImportFromStmt::cast(syntax).map(PyStat::ImportFromStmt)
            }
            PySyntaxKind::MatchStmt => PyMatchStmt::cast(syntax).map(PyStat::MatchStmt),
            PySyntaxKind::ElseClause => PyElseStmt::cast(syntax).map(PyStat::ElseStmt),
            PySyntaxKind::ElifClause => PyElifStmt::cast(syntax).map(PyStat::ElifStmt),
            // Python 3.11+ Exception Groups
            PySyntaxKind::TryStarStmt => PyTryStarStmt::cast(syntax).map(PyStat::TryStarStmt),
            PySyntaxKind::ExceptStarClause => {
                PyExceptStarClause::cast(syntax).map(PyStat::ExceptStarClause)
            }
            // Python 3.12+ Type Parameters and Type Statements
            PySyntaxKind::TypeStatement => PyTypeStatement::cast(syntax).map(PyStat::TypeStatement),
            PySyntaxKind::TypeAliasStmt => PyTypeAliasStmt::cast(syntax).map(PyStat::TypeAliasStmt),
            PySyntaxKind::GenericFuncDef => {
                PyGenericFuncDef::cast(syntax).map(PyStat::GenericFuncDef)
            }
            PySyntaxKind::GenericClassDef => {
                PyGenericClassDef::cast(syntax).map(PyStat::GenericClassDef)
            }
            // Python 3.14+ Experimental Features
            PySyntaxKind::Decorated => PyDecorated::cast(syntax).map(PyStat::Decorated),
            PySyntaxKind::AsyncCompStmt => PyAsyncCompStmt::cast(syntax).map(PyStat::AsyncCompStmt),
            _ => None,
        }
    }
}

// 为每个语句节点实现子节点访问方法
impl PyExprStmt {
    pub fn get_expr(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyAssignStmt {
    pub fn get_targets(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }

    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).last()
    }
}

impl PyAnnAssignStmt {
    pub fn get_target(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_annotation(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(2)
    }
}

impl PyAugAssignStmt {
    pub fn get_target(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }
}

impl PyFuncDef {
    pub fn get_name(&self) -> Option<PyNameExpr> {
        self.syntax().children().find_map(PyNameExpr::cast)
    }

    pub fn get_parameters(&self) -> Option<PyArguments> {
        self.syntax().children().find_map(PyArguments::cast)
    }

    pub fn get_return_annotation(&self) -> Option<PyExpr> {
        // 返回类型注解通常在参数列表之后
        self.syntax().children().filter_map(PyExpr::cast).last()
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_decorators(&self) -> impl Iterator<Item = PyDecorator> + '_ {
        self.syntax().children().filter_map(PyDecorator::cast)
    }
}

impl PyAsyncFuncDef {
    pub fn get_name(&self) -> Option<PyNameExpr> {
        self.syntax().children().find_map(PyNameExpr::cast)
    }

    pub fn get_parameters(&self) -> Option<PyArguments> {
        self.syntax().children().find_map(PyArguments::cast)
    }

    pub fn get_return_annotation(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).last()
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_decorators(&self) -> impl Iterator<Item = PyDecorator> + '_ {
        self.syntax().children().filter_map(PyDecorator::cast)
    }
}

impl PyClassDef {
    pub fn get_name(&self) -> Option<PyNameExpr> {
        self.syntax().children().find_map(PyNameExpr::cast)
    }

    pub fn get_bases(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_decorators(&self) -> impl Iterator<Item = PyDecorator> + '_ {
        self.syntax().children().filter_map(PyDecorator::cast)
    }
}

impl PyIfStmt {
    pub fn get_test(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_elif_clauses(&self) -> impl Iterator<Item = PyElifStmt> + '_ {
        self.syntax().children().filter_map(PyElifStmt::cast)
    }

    pub fn get_else_clause(&self) -> Option<PyElseStmt> {
        self.syntax().children().find_map(PyElseStmt::cast)
    }
}

impl PyWhileStmt {
    pub fn get_test(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_else_clause(&self) -> Option<PyElseStmt> {
        self.syntax().children().find_map(PyElseStmt::cast)
    }
}

impl PyForStmt {
    pub fn get_target(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_iter(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_else_clause(&self) -> Option<PyElseStmt> {
        self.syntax().children().find_map(PyElseStmt::cast)
    }
}

impl PyAsyncForStmt {
    pub fn get_target(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_iter(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_else_clause(&self) -> Option<PyElseStmt> {
        self.syntax().children().find_map(PyElseStmt::cast)
    }
}

impl PyWithStmt {
    pub fn get_items(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }
}

impl PyAsyncWithStmt {
    pub fn get_items(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }
}

impl PyTryStmt {
    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }

    pub fn get_handlers(&self) -> impl Iterator<Item = PyStat> + '_ {
        self.syntax().children().filter_map(PyStat::cast)
    }

    pub fn get_else_clause(&self) -> Option<PyElseStmt> {
        self.syntax().children().find_map(PyElseStmt::cast)
    }

    pub fn get_finally_clause(&self) -> Option<PySuite> {
        self.syntax().children().filter_map(PySuite::cast).nth(1)
    }
}

impl PyReturnStmt {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyYieldStmt {
    pub fn get_value(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }
}

impl PyRaiseStmt {
    pub fn get_exception(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_cause(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }
}

impl PyAssertStmt {
    pub fn get_test(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_msg(&self) -> Option<PyExpr> {
        self.syntax().children().filter_map(PyExpr::cast).nth(1)
    }
}

impl PyDeleteStmt {
    pub fn get_targets(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }
}

impl PyGlobalStmt {
    pub fn get_names(&self) -> impl Iterator<Item = PyNameExpr> + '_ {
        self.syntax().children().filter_map(PyNameExpr::cast)
    }
}

impl PyNonlocalStmt {
    pub fn get_names(&self) -> impl Iterator<Item = PyNameExpr> + '_ {
        self.syntax().children().filter_map(PyNameExpr::cast)
    }
}

impl PyImportStmt {
    pub fn get_names(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast)
    }
}

impl PyImportFromStmt {
    pub fn get_module(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_names(&self) -> impl Iterator<Item = PyExpr> + '_ {
        self.syntax().children().filter_map(PyExpr::cast).skip(1)
    }
}

impl PyMatchStmt {
    pub fn get_subject(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_cases(&self) -> impl Iterator<Item = PyCaseClause> + '_ {
        self.syntax().children().filter_map(PyCaseClause::cast)
    }
}

impl PyElifStmt {
    pub fn get_test(&self) -> Option<PyExpr> {
        self.syntax().children().find_map(PyExpr::cast)
    }

    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }
}

impl PyElseStmt {
    pub fn get_body(&self) -> Option<PySuite> {
        self.syntax().children().find_map(PySuite::cast)
    }
}
