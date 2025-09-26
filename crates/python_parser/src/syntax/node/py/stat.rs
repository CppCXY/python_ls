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
    PyElifStmt: ElifClause
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
            _ => None,
        }
    }
}
