use crate::{
    LuaAstToken, LuaGeneralToken, LuaLocalAttribute, LuaSyntaxNode,
    kind::PySyntaxKind,
    syntax::{
        LuaCommentOwner,
        node::LuaNameToken,
        traits::{LuaAstChildren, LuaAstNode, LuaAstTokenChildren},
    },
};

use super::{
    LuaBlock, LuaLocalName,
    expr::{LuaCallExpr, LuaClosureExpr, LuaExpr, LuaVarExpr},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaStat {
    LocalStat(LuaLocalStat),
    AssignStat(LuaAssignStat),
    CallExprStat(LuaCallExprStat),
    FuncStat(LuaFuncStat),
    LocalFuncStat(LuaLocalFuncStat),
    IfStat(LuaIfStat),
    WhileStat(LuaWhileStat),
    DoStat(LuaDoStat),
    ForStat(LuaForStat),
    ForRangeStat(LuaForRangeStat),
    RepeatStat(LuaRepeatStat),
    BreakStat(LuaBreakStat),
    ReturnStat(LuaReturnStat),
    GotoStat(LuaGotoStat),
    LabelStat(LuaLabelStat),
    EmptyStat(LuaEmptyStat),
    GlobalStat(LuaGlobalStat),
}

impl LuaAstNode for LuaStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaStat::LocalStat(node) => node.syntax(),
            LuaStat::AssignStat(node) => node.syntax(),
            LuaStat::CallExprStat(node) => node.syntax(),
            LuaStat::FuncStat(node) => node.syntax(),
            LuaStat::LocalFuncStat(node) => node.syntax(),
            LuaStat::IfStat(node) => node.syntax(),
            LuaStat::WhileStat(node) => node.syntax(),
            LuaStat::DoStat(node) => node.syntax(),
            LuaStat::ForStat(node) => node.syntax(),
            LuaStat::ForRangeStat(node) => node.syntax(),
            LuaStat::RepeatStat(node) => node.syntax(),
            LuaStat::BreakStat(node) => node.syntax(),
            LuaStat::ReturnStat(node) => node.syntax(),
            LuaStat::GotoStat(node) => node.syntax(),
            LuaStat::LabelStat(node) => node.syntax(),
            LuaStat::EmptyStat(node) => node.syntax(),
            LuaStat::GlobalStat(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::LocalStat
                | PySyntaxKind::AssignStat
                | PySyntaxKind::CallExprStat
                | PySyntaxKind::FuncStat
                | PySyntaxKind::LocalFuncStat
                | PySyntaxKind::IfStat
                | PySyntaxKind::WhileStat
                | PySyntaxKind::DoStat
                | PySyntaxKind::ForStat
                | PySyntaxKind::ForRangeStat
                | PySyntaxKind::RepeatStat
                | PySyntaxKind::BreakStat
                | PySyntaxKind::ReturnStat
                | PySyntaxKind::GotoStat
                | PySyntaxKind::LabelStat
                | PySyntaxKind::EmptyStat
                | PySyntaxKind::GlobalStat
        )
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            PySyntaxKind::LocalStat => Some(LuaStat::LocalStat(LuaLocalStat::cast(syntax)?)),
            PySyntaxKind::AssignStat => Some(LuaStat::AssignStat(LuaAssignStat::cast(syntax)?)),
            PySyntaxKind::CallExprStat => {
                Some(LuaStat::CallExprStat(LuaCallExprStat::cast(syntax)?))
            }
            PySyntaxKind::FuncStat => Some(LuaStat::FuncStat(LuaFuncStat::cast(syntax)?)),
            PySyntaxKind::LocalFuncStat => {
                Some(LuaStat::LocalFuncStat(LuaLocalFuncStat::cast(syntax)?))
            }
            PySyntaxKind::IfStat => Some(LuaStat::IfStat(LuaIfStat::cast(syntax)?)),
            PySyntaxKind::WhileStat => Some(LuaStat::WhileStat(LuaWhileStat::cast(syntax)?)),
            PySyntaxKind::DoStat => Some(LuaStat::DoStat(LuaDoStat::cast(syntax)?)),
            PySyntaxKind::ForStat => Some(LuaStat::ForStat(LuaForStat::cast(syntax)?)),
            PySyntaxKind::ForRangeStat => {
                Some(LuaStat::ForRangeStat(LuaForRangeStat::cast(syntax)?))
            }
            PySyntaxKind::RepeatStat => Some(LuaStat::RepeatStat(LuaRepeatStat::cast(syntax)?)),
            PySyntaxKind::BreakStat => Some(LuaStat::BreakStat(LuaBreakStat::cast(syntax)?)),
            PySyntaxKind::ReturnStat => Some(LuaStat::ReturnStat(LuaReturnStat::cast(syntax)?)),
            PySyntaxKind::GotoStat => Some(LuaStat::GotoStat(LuaGotoStat::cast(syntax)?)),
            PySyntaxKind::LabelStat => Some(LuaStat::LabelStat(LuaLabelStat::cast(syntax)?)),
            PySyntaxKind::EmptyStat => Some(LuaStat::EmptyStat(LuaEmptyStat::cast(syntax)?)),
            PySyntaxKind::GlobalStat => Some(LuaStat::GlobalStat(LuaGlobalStat::cast(syntax)?)),
            _ => None,
        }
    }
}

impl LuaCommentOwner for LuaStat {}

impl LuaStat {
    pub fn get_parent_block(&self) -> Option<LuaBlock> {
        LuaBlock::cast(self.syntax().parent()?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaLoopStat {
    WhileStat(LuaWhileStat),
    RepeatStat(LuaRepeatStat),
    ForStat(LuaForStat),
    ForRangeStat(LuaForRangeStat),
}

impl LuaAstNode for LuaLoopStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaLoopStat::WhileStat(node) => node.syntax(),
            LuaLoopStat::RepeatStat(node) => node.syntax(),
            LuaLoopStat::ForStat(node) => node.syntax(),
            LuaLoopStat::ForRangeStat(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::WhileStat
                | PySyntaxKind::RepeatStat
                | PySyntaxKind::ForStat
                | PySyntaxKind::ForRangeStat
        )
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(node) = LuaWhileStat::cast(syntax.clone()) {
            Some(LuaLoopStat::WhileStat(node))
        } else if let Some(node) = LuaRepeatStat::cast(syntax.clone()) {
            Some(LuaLoopStat::RepeatStat(node))
        } else if let Some(node) = LuaForStat::cast(syntax.clone()) {
            Some(LuaLoopStat::ForStat(node))
        } else {
            LuaForRangeStat::cast(syntax.clone()).map(LuaLoopStat::ForRangeStat)
        }
    }
}

impl LuaCommentOwner for LuaLoopStat {}

impl LuaLoopStat {
    pub fn get_parent_block(&self) -> Option<LuaBlock> {
        LuaBlock::cast(self.syntax().parent()?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLocalStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLocalStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::LocalStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::LocalStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaLocalStat {}

impl LuaLocalStat {
    pub fn get_local_name_list(&self) -> LuaAstChildren<LuaLocalName> {
        self.children()
    }

    pub fn get_value_exprs(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }

    pub fn get_local_name_by_value(&self, value: LuaExpr) -> Option<LuaLocalName> {
        let local_names = self.get_local_name_list();
        let value_exprs = self.get_value_exprs().collect::<Vec<_>>();

        for (i, local_name) in local_names.enumerate() {
            if let Some(value_expr) = value_exprs.get(i)
                && value_expr.syntax() == value.syntax()
            {
                return Some(local_name);
            }
        }
        None
    }

    pub fn get_attrib(&self) -> Option<LuaLocalAttribute> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaAssignStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaAssignStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::AssignStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::AssignStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaAssignStat {}

impl LuaAssignStat {
    pub fn get_var_and_expr_list(&self) -> (Vec<LuaVarExpr>, Vec<LuaExpr>) {
        let mut vars = Vec::new();
        let mut exprs = Vec::new();
        let mut meet_assign = false;
        for child in self.syntax.children_with_tokens() {
            if child.kind().to_token().is_assign_op() {
                meet_assign = true;
            }

            if let Some(node) = child.into_node() {
                if meet_assign {
                    if let Some(var) = LuaExpr::cast(node) {
                        exprs.push(var);
                    }
                } else if let Some(var) = LuaVarExpr::cast(node) {
                    vars.push(var);
                }
            }
        }

        (vars, exprs)
    }

    pub fn get_assign_op(&self) -> Option<LuaGeneralToken> {
        for child in self.syntax.children_with_tokens() {
            if let Some(token) = child.into_token()
                && token.kind().to_token().is_assign_op()
            {
                return LuaGeneralToken::cast(token);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaCallExprStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaCallExprStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::CallExprStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::CallExprStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaCallExprStat {}

impl LuaCallExprStat {
    pub fn get_call_expr(&self) -> Option<LuaCallExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaFuncStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaFuncStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::FuncStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::FuncStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaFuncStat {}

impl LuaFuncStat {
    pub fn get_func_name(&self) -> Option<LuaVarExpr> {
        self.child()
    }

    pub fn get_closure(&self) -> Option<LuaClosureExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLocalFuncStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLocalFuncStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::LocalFuncStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::LocalFuncStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaLocalFuncStat {}

impl LuaLocalFuncStat {
    pub fn get_local_name(&self) -> Option<LuaLocalName> {
        self.child()
    }

    pub fn get_closure(&self) -> Option<LuaClosureExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaIfStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaIfStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::IfStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::IfStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaIfStat {}

impl LuaIfStat {
    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }

    pub fn get_else_if_clause_list(&self) -> LuaAstChildren<LuaElseIfClauseStat> {
        self.children()
    }

    pub fn get_else_clause(&self) -> Option<LuaElseClauseStat> {
        self.child()
    }

    pub fn get_all_clause(&self) -> LuaAstChildren<LuaIfClauseStat> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaElseIfClauseStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaElseIfClauseStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::ElseIfClauseStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::ElseIfClauseStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaElseIfClauseStat {}

impl LuaElseIfClauseStat {
    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaElseClauseStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaElseClauseStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::ElseClauseStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::ElseClauseStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaElseClauseStat {}

impl LuaElseClauseStat {
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaIfClauseStat {
    ElseIf(LuaElseIfClauseStat),
    Else(LuaElseClauseStat),
}

impl LuaAstNode for LuaIfClauseStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaIfClauseStat::ElseIf(node) => node.syntax(),
            LuaIfClauseStat::Else(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        LuaElseIfClauseStat::can_cast(kind) || LuaElseClauseStat::can_cast(kind)
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if LuaElseIfClauseStat::can_cast(syntax.kind().into()) {
            Some(LuaIfClauseStat::ElseIf(LuaElseIfClauseStat::cast(syntax)?))
        } else if LuaElseClauseStat::can_cast(syntax.kind().into()) {
            Some(LuaIfClauseStat::Else(LuaElseClauseStat::cast(syntax)?))
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaIfClauseStat {}

impl LuaIfClauseStat {
    pub fn get_parent_if_stat(&self) -> Option<LuaIfStat> {
        LuaIfStat::cast(self.syntax().parent()?)
    }

    pub fn get_block(&self) -> Option<LuaBlock> {
        match self {
            LuaIfClauseStat::ElseIf(node) => node.get_block(),
            LuaIfClauseStat::Else(node) => node.get_block(),
        }
    }

    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        match self {
            LuaIfClauseStat::ElseIf(node) => node.get_condition_expr(),
            LuaIfClauseStat::Else(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaWhileStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaWhileStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::WhileStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::WhileStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaWhileStat {}

impl LuaWhileStat {
    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDoStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDoStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::DoStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::DoStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaDoStat {}

impl LuaDoStat {
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaForStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaForStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::ForStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::ForStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaForStat {}

impl LuaForStat {
    pub fn get_var_name(&self) -> Option<LuaNameToken> {
        self.token()
    }

    pub fn get_iter_expr(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }

    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaForRangeStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaForRangeStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::ForRangeStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::ForRangeStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaForRangeStat {}

impl LuaForRangeStat {
    pub fn get_var_name_list(&self) -> LuaAstTokenChildren<LuaNameToken> {
        self.tokens()
    }

    pub fn get_expr_list(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }

    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaRepeatStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaRepeatStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::RepeatStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::RepeatStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaRepeatStat {}

impl LuaRepeatStat {
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }

    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBreakStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaBreakStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::BreakStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::BreakStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaBreakStat {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaReturnStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaReturnStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::ReturnStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::ReturnStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaReturnStat {}

impl LuaReturnStat {
    pub fn get_expr_list(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaGotoStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaGotoStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::GotoStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::GotoStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaGotoStat {}

impl LuaGotoStat {
    pub fn get_label_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLabelStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLabelStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::LabelStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::LabelStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaLabelStat {}

impl LuaLabelStat {
    pub fn get_label_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaEmptyStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaEmptyStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::EmptyStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::EmptyStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaEmptyStat {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaGlobalStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaGlobalStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::GlobalStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == PySyntaxKind::GlobalStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaGlobalStat {}

impl LuaGlobalStat {
    pub fn get_local_name_list(&self) -> LuaAstChildren<LuaLocalName> {
        self.children()
    }

    pub fn get_attrib(&self) -> Option<LuaLocalAttribute> {
        self.child()
    }
}
