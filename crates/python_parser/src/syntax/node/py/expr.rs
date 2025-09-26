use crate::{
    LuaAstToken, LuaIndexToken, LuaLiteralToken, LuaSyntaxNode, LuaSyntaxToken, PyTokenKind,
    kind::PySyntaxKind,
    syntax::{
        node::{LuaBinaryOpToken, LuaNameToken, LuaUnaryOpToken},
        traits::{LuaAstChildren, PyAstNode, LuaCommentOwner},
    },
};

use super::{
    LuaBlock, LuaCallArgList, LuaIndexKey, LuaParamList, LuaTableField, path_trait::PathTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaExpr {
    CallExpr(LuaCallExpr),
    TableExpr(LuaTableExpr),
    LiteralExpr(LuaLiteralExpr),
    BinaryExpr(LuaBinaryExpr),
    UnaryExpr(LuaUnaryExpr),
    ClosureExpr(LuaClosureExpr),
    ParenExpr(LuaParenExpr),
    NameExpr(LuaNameExpr),
    IndexExpr(LuaIndexExpr),
}

impl PyAstNode for LuaExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaExpr::CallExpr(node) => node.syntax(),
            LuaExpr::TableExpr(node) => node.syntax(),
            LuaExpr::LiteralExpr(node) => node.syntax(),
            LuaExpr::BinaryExpr(node) => node.syntax(),
            LuaExpr::UnaryExpr(node) => node.syntax(),
            LuaExpr::ClosureExpr(node) => node.syntax(),
            LuaExpr::ParenExpr(node) => node.syntax(),
            LuaExpr::NameExpr(node) => node.syntax(),
            LuaExpr::IndexExpr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::CallExpr
                | PySyntaxKind::AssertCallExpr
                | PySyntaxKind::ErrorCallExpr
                | PySyntaxKind::RequireCallExpr
                | PySyntaxKind::TypeCallExpr
                | PySyntaxKind::SetmetatableCallExpr
                | PySyntaxKind::TableArrayExpr
                | PySyntaxKind::TableObjectExpr
                | PySyntaxKind::TableEmptyExpr
                | PySyntaxKind::LiteralExpr
                | PySyntaxKind::BinaryExpr
                | PySyntaxKind::UnaryExpr
                | PySyntaxKind::ClosureExpr
                | PySyntaxKind::ParenExpr
                | PySyntaxKind::NameExpr
                | PySyntaxKind::IndexExpr
        )
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            PySyntaxKind::CallExpr
            | PySyntaxKind::AssertCallExpr
            | PySyntaxKind::ErrorCallExpr
            | PySyntaxKind::RequireCallExpr
            | PySyntaxKind::TypeCallExpr
            | PySyntaxKind::SetmetatableCallExpr => {
                LuaCallExpr::cast(syntax).map(LuaExpr::CallExpr)
            }
            PySyntaxKind::TableArrayExpr
            | PySyntaxKind::TableObjectExpr
            | PySyntaxKind::TableEmptyExpr => LuaTableExpr::cast(syntax).map(LuaExpr::TableExpr),
            PySyntaxKind::LiteralExpr => LuaLiteralExpr::cast(syntax).map(LuaExpr::LiteralExpr),
            PySyntaxKind::BinaryExpr => LuaBinaryExpr::cast(syntax).map(LuaExpr::BinaryExpr),
            PySyntaxKind::UnaryExpr => LuaUnaryExpr::cast(syntax).map(LuaExpr::UnaryExpr),
            PySyntaxKind::ClosureExpr => LuaClosureExpr::cast(syntax).map(LuaExpr::ClosureExpr),
            PySyntaxKind::ParenExpr => LuaParenExpr::cast(syntax).map(LuaExpr::ParenExpr),
            PySyntaxKind::NameExpr => LuaNameExpr::cast(syntax).map(LuaExpr::NameExpr),
            PySyntaxKind::IndexExpr => LuaIndexExpr::cast(syntax).map(LuaExpr::IndexExpr),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaVarExpr {
    NameExpr(LuaNameExpr),
    IndexExpr(LuaIndexExpr),
}

impl PyAstNode for LuaVarExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaVarExpr::NameExpr(node) => node.syntax(),
            LuaVarExpr::IndexExpr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, PySyntaxKind::NameExpr | PySyntaxKind::IndexExpr)
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            PySyntaxKind::NameExpr => LuaNameExpr::cast(syntax).map(LuaVarExpr::NameExpr),
            PySyntaxKind::IndexExpr => LuaIndexExpr::cast(syntax).map(LuaVarExpr::IndexExpr),
            _ => None,
        }
    }
}

impl LuaVarExpr {
    pub fn to_expr(&self) -> LuaExpr {
        match self {
            LuaVarExpr::NameExpr(node) => LuaExpr::NameExpr(node.clone()),
            LuaVarExpr::IndexExpr(node) => LuaExpr::IndexExpr(node.clone()),
        }
    }
}

impl From<LuaVarExpr> for LuaExpr {
    fn from(expr: LuaVarExpr) -> Self {
        match expr {
            LuaVarExpr::NameExpr(node) => LuaExpr::NameExpr(node),
            LuaVarExpr::IndexExpr(node) => LuaExpr::IndexExpr(node),
        }
    }
}

impl PathTrait for LuaVarExpr {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaSingleArgExpr {
    TableExpr(LuaTableExpr),
    LiteralExpr(LuaLiteralExpr),
}

impl PyAstNode for LuaSingleArgExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaSingleArgExpr::TableExpr(node) => node.syntax(),
            LuaSingleArgExpr::LiteralExpr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::TableArrayExpr
                | PySyntaxKind::TableObjectExpr
                | PySyntaxKind::TableEmptyExpr
                | PySyntaxKind::LiteralExpr
        )
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            PySyntaxKind::TableArrayExpr
            | PySyntaxKind::TableObjectExpr
            | PySyntaxKind::TableEmptyExpr => {
                LuaTableExpr::cast(syntax).map(LuaSingleArgExpr::TableExpr)
            }
            PySyntaxKind::LiteralExpr => {
                LuaLiteralExpr::cast(syntax).map(LuaSingleArgExpr::LiteralExpr)
            }
            _ => None,
        }
    }
}

impl From<LuaSingleArgExpr> for LuaExpr {
    fn from(expr: LuaSingleArgExpr) -> Self {
        match expr {
            LuaSingleArgExpr::TableExpr(node) => LuaExpr::TableExpr(node),
            LuaSingleArgExpr::LiteralExpr(node) => LuaExpr::LiteralExpr(node),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaNameExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaNameExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::NameExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaCommentOwner for LuaNameExpr {}

impl LuaNameExpr {
    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }

    pub fn get_name_text(&self) -> Option<String> {
        self.get_name_token()
            .map(|it| it.get_name_text().to_string())
    }
}

impl PathTrait for LuaNameExpr {}

impl From<LuaNameExpr> for LuaVarExpr {
    fn from(expr: LuaNameExpr) -> Self {
        LuaVarExpr::NameExpr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaIndexExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaIndexExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::IndexExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaIndexExpr {
    pub fn get_prefix_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_index_token(&self) -> Option<LuaIndexToken> {
        self.token()
    }

    pub fn get_index_key(&self) -> Option<LuaIndexKey> {
        let mut meet_left_bracket = false;
        for child in self.syntax.children_with_tokens() {
            if meet_left_bracket {
                match child {
                    rowan::NodeOrToken::Node(node) => {
                        if LuaLiteralExpr::can_cast(node.kind().into()) {
                            let literal_expr = LuaLiteralExpr::cast(node.clone()).unwrap();
                            if let Some(literal_token) = literal_expr.get_literal() {
                                match literal_token {
                                    LuaLiteralToken::String(token) => {
                                        return Some(LuaIndexKey::String(token.clone()));
                                    }
                                    LuaLiteralToken::Number(token) => {
                                        return Some(LuaIndexKey::Integer(token.clone()));
                                    }
                                    _ => {}
                                }
                            }
                        }

                        return Some(LuaIndexKey::Expr(LuaExpr::cast(node).unwrap()));
                    }
                    _ => return None,
                }
            } else if let Some(token) = child.as_token() {
                if token.kind() == PyTokenKind::TkLeftBracket.into() {
                    meet_left_bracket = true;
                } else if token.kind() == PyTokenKind::TkName.into() {
                    return Some(LuaIndexKey::Name(
                        LuaNameToken::cast(token.clone()).unwrap(),
                    ));
                }
            }
        }

        None
    }

    pub fn get_index_name_token(&self) -> Option<LuaSyntaxToken> {
        let index_token = self.get_index_token()?;
        index_token.syntax().next_token()
    }

    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }
}

impl PathTrait for LuaIndexExpr {}

impl From<LuaIndexExpr> for LuaVarExpr {
    fn from(expr: LuaIndexExpr) -> Self {
        LuaVarExpr::IndexExpr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaCallExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaCallExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::CallExpr
            || kind == PySyntaxKind::AssertCallExpr
            || kind == PySyntaxKind::ErrorCallExpr
            || kind == PySyntaxKind::RequireCallExpr
            || kind == PySyntaxKind::TypeCallExpr
            || kind == PySyntaxKind::SetmetatableCallExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaCallExpr {
    pub fn get_prefix_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_args_list(&self) -> Option<LuaCallArgList> {
        self.child()
    }

    pub fn get_args_count(&self) -> Option<usize> {
        self.get_args_list().map(|it| it.get_args().count())
    }

    pub fn is_colon_call(&self) -> bool {
        if let Some(index_token) = self.get_colon_token() {
            return index_token.is_colon();
        }
        false
    }

    pub fn get_colon_token(&self) -> Option<LuaIndexToken> {
        self.get_prefix_expr().and_then(|prefix| match prefix {
            LuaExpr::IndexExpr(index_expr) => index_expr.get_index_token(),
            _ => None,
        })
    }

    pub fn is_require(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::RequireCallExpr.into()
    }

    pub fn is_error(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::ErrorCallExpr.into()
    }

    pub fn is_assert(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::AssertCallExpr.into()
    }

    pub fn is_type(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::TypeCallExpr.into()
    }

    pub fn is_setmetatable(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::SetmetatableCallExpr.into()
    }
}

impl PathTrait for LuaCallExpr {}

impl From<LuaCallExpr> for LuaExpr {
    fn from(expr: LuaCallExpr) -> Self {
        LuaExpr::CallExpr(expr)
    }
}

/// In Lua, tables are a fundamental data structure that can be used to represent arrays, objects,
/// and more. To facilitate parsing and handling of different table structures, we categorize tables
/// into three types: `TableArrayExpr`, `TableObjectExpr`, and `TableEmptyExpr`.
///
/// - `TableArrayExpr`: Represents a table used as an array, where elements are indexed by integers.
/// - `TableObjectExpr`: Represents a table used as an object, where elements are indexed by strings or other keys.
/// - `TableEmptyExpr`: Represents an empty table with no elements.
///
/// This categorization helps in accurately parsing and processing Lua code by distinguishing between
/// different uses of tables, thereby enabling more precise syntax analysis and manipulation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaTableExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaTableExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TableArrayExpr
            || kind == PySyntaxKind::TableObjectExpr
            || kind == PySyntaxKind::TableEmptyExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaCommentOwner for LuaTableExpr {}

impl LuaTableExpr {
    pub fn is_empty(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::TableEmptyExpr.into()
    }

    pub fn is_array(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::TableArrayExpr.into()
    }

    pub fn is_object(&self) -> bool {
        self.syntax().kind() == PySyntaxKind::TableObjectExpr.into()
    }

    pub fn get_fields(&self) -> LuaAstChildren<LuaTableField> {
        self.children()
    }
}

impl From<LuaTableExpr> for LuaSingleArgExpr {
    fn from(expr: LuaTableExpr) -> Self {
        LuaSingleArgExpr::TableExpr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLiteralExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaLiteralExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::LiteralExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaLiteralExpr {
    pub fn get_literal(&self) -> Option<LuaLiteralToken> {
        self.token()
    }
}

impl From<LuaLiteralExpr> for LuaSingleArgExpr {
    fn from(expr: LuaLiteralExpr) -> Self {
        LuaSingleArgExpr::LiteralExpr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBinaryExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaBinaryExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::BinaryExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaBinaryExpr {
    pub fn get_exprs(&self) -> Option<(LuaExpr, LuaExpr)> {
        let exprs = self.children::<LuaExpr>().collect::<Vec<_>>();
        if exprs.len() == 2 {
            Some((exprs[0].clone(), exprs[1].clone()))
        } else {
            None
        }
    }

    pub fn get_op_token(&self) -> Option<LuaBinaryOpToken> {
        self.token()
    }

    pub fn get_left_expr(&self) -> Option<LuaExpr> {
        self.child()
    }
}

impl From<LuaBinaryExpr> for LuaExpr {
    fn from(expr: LuaBinaryExpr) -> Self {
        LuaExpr::BinaryExpr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaUnaryExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaUnaryExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::UnaryExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaUnaryExpr {
    pub fn get_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_op_token(&self) -> Option<LuaUnaryOpToken> {
        self.token()
    }
}

impl From<LuaUnaryExpr> for LuaExpr {
    fn from(expr: LuaUnaryExpr) -> Self {
        LuaExpr::UnaryExpr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaClosureExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaClosureExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::ClosureExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaClosureExpr {
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }

    pub fn get_params_list(&self) -> Option<LuaParamList> {
        self.child()
    }
}

impl From<LuaClosureExpr> for LuaExpr {
    fn from(expr: LuaClosureExpr) -> Self {
        LuaExpr::ClosureExpr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaParenExpr {
    syntax: LuaSyntaxNode,
}

impl PyAstNode for LuaParenExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::ParenExpr
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
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

impl LuaParenExpr {
    pub fn get_expr(&self) -> Option<LuaExpr> {
        self.child()
    }
}

impl From<LuaParenExpr> for LuaExpr {
    fn from(expr: LuaParenExpr) -> Self {
        LuaExpr::ParenExpr(expr)
    }
}
