use crate::{
    LuaAstChildren, LuaAstNode, LuaAstToken, LuaDocDescriptionOwner, LuaDocTypeBinaryToken,
    LuaDocTypeUnaryToken, LuaLiteralToken, LuaNameToken, PySyntaxKind, LuaSyntaxNode,
    PyTokenKind,
};

use super::{LuaDocObjectField, LuaDocTypeList};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaDocType {
    Name(LuaDocNameType),
    Array(LuaDocArrayType),
    Func(LuaDocFuncType),
    Object(LuaDocObjectType),
    Binary(LuaDocBinaryType),
    Unary(LuaDocUnaryType),
    Conditional(LuaDocConditionalType),
    Tuple(LuaDocTupleType),
    Literal(LuaDocLiteralType),
    Variadic(LuaDocVariadicType),
    Nullable(LuaDocNullableType),
    Generic(LuaDocGenericType),
    StrTpl(LuaDocStrTplType),
    MultiLineUnion(LuaDocMultiLineUnionType),
}

impl LuaAstNode for LuaDocType {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaDocType::Name(it) => it.syntax(),
            LuaDocType::Array(it) => it.syntax(),
            LuaDocType::Func(it) => it.syntax(),
            LuaDocType::Object(it) => it.syntax(),
            LuaDocType::Binary(it) => it.syntax(),
            LuaDocType::Unary(it) => it.syntax(),
            LuaDocType::Conditional(it) => it.syntax(),
            LuaDocType::Tuple(it) => it.syntax(),
            LuaDocType::Literal(it) => it.syntax(),
            LuaDocType::Variadic(it) => it.syntax(),
            LuaDocType::Nullable(it) => it.syntax(),
            LuaDocType::Generic(it) => it.syntax(),
            LuaDocType::StrTpl(it) => it.syntax(),
            LuaDocType::MultiLineUnion(it) => it.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::TypeName
                | PySyntaxKind::TypeArray
                | PySyntaxKind::TypeFun
                | PySyntaxKind::TypeObject
                | PySyntaxKind::TypeBinary
                | PySyntaxKind::TypeUnary
                | PySyntaxKind::TypeConditional
                | PySyntaxKind::TypeTuple
                | PySyntaxKind::TypeLiteral
                | PySyntaxKind::TypeVariadic
                | PySyntaxKind::TypeNullable
                | PySyntaxKind::TypeGeneric
                | PySyntaxKind::TypeStringTemplate
                | PySyntaxKind::TypeMultiLineUnion
        )
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            PySyntaxKind::TypeName => Some(LuaDocType::Name(LuaDocNameType::cast(syntax)?)),
            PySyntaxKind::TypeArray => Some(LuaDocType::Array(LuaDocArrayType::cast(syntax)?)),
            PySyntaxKind::TypeFun => Some(LuaDocType::Func(LuaDocFuncType::cast(syntax)?)),
            PySyntaxKind::TypeObject => Some(LuaDocType::Object(LuaDocObjectType::cast(syntax)?)),
            PySyntaxKind::TypeBinary => Some(LuaDocType::Binary(LuaDocBinaryType::cast(syntax)?)),
            PySyntaxKind::TypeUnary => Some(LuaDocType::Unary(LuaDocUnaryType::cast(syntax)?)),
            PySyntaxKind::TypeConditional => Some(LuaDocType::Conditional(
                LuaDocConditionalType::cast(syntax)?,
            )),
            PySyntaxKind::TypeTuple => Some(LuaDocType::Tuple(LuaDocTupleType::cast(syntax)?)),
            PySyntaxKind::TypeLiteral => {
                Some(LuaDocType::Literal(LuaDocLiteralType::cast(syntax)?))
            }
            PySyntaxKind::TypeVariadic => {
                Some(LuaDocType::Variadic(LuaDocVariadicType::cast(syntax)?))
            }
            PySyntaxKind::TypeNullable => {
                Some(LuaDocType::Nullable(LuaDocNullableType::cast(syntax)?))
            }
            PySyntaxKind::TypeGeneric => {
                Some(LuaDocType::Generic(LuaDocGenericType::cast(syntax)?))
            }
            PySyntaxKind::TypeStringTemplate => {
                Some(LuaDocType::StrTpl(LuaDocStrTplType::cast(syntax)?))
            }
            PySyntaxKind::TypeMultiLineUnion => Some(LuaDocType::MultiLineUnion(
                LuaDocMultiLineUnionType::cast(syntax)?,
            )),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocNameType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocNameType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeName
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

impl LuaDocNameType {
    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }

    pub fn get_name_text(&self) -> Option<String> {
        self.get_name_token()
            .map(|it| it.get_name_text().to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocArrayType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocArrayType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeArray
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

impl LuaDocArrayType {
    pub fn get_type(&self) -> Option<LuaDocType> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocFuncType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocFuncType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeFun
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

impl LuaDocFuncType {
    pub fn is_async(&self) -> bool {
        match self.token::<LuaNameToken>() {
            Some(it) => it.get_name_text() == "async",
            None => false,
        }
    }

    pub fn is_sync(&self) -> bool {
        match self.token::<LuaNameToken>() {
            Some(it) => it.get_name_text() == "sync",
            None => false,
        }
    }

    pub fn get_params(&self) -> LuaAstChildren<LuaDocTypeParam> {
        self.children()
    }

    pub fn get_return_type_list(&self) -> Option<LuaDocTypeList> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocTypeParam {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocTypeParam {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::DocTypedParameter
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

impl LuaDocTypeParam {
    pub fn is_dots(&self) -> bool {
        self.token_by_kind(PyTokenKind::TkDots).is_some()
    }

    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }

    pub fn get_type(&self) -> Option<LuaDocType> {
        self.child()
    }

    pub fn is_nullable(&self) -> bool {
        self.token_by_kind(PyTokenKind::TkDocQuestion).is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocObjectType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocObjectType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeObject
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

impl LuaDocObjectType {
    pub fn get_fields(&self) -> LuaAstChildren<LuaDocObjectField> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocBinaryType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocBinaryType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeBinary
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

impl LuaDocBinaryType {
    pub fn get_op_token(&self) -> Option<LuaDocTypeBinaryToken> {
        self.token()
    }

    pub fn get_types(&self) -> Option<(LuaDocType, LuaDocType)> {
        let mut children = self.children();
        let left = children.next()?;
        let right = children.next()?;
        Some((left, right))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocUnaryType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocUnaryType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeUnary
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

impl LuaDocUnaryType {
    pub fn get_op_token(&self) -> Option<LuaDocTypeUnaryToken> {
        self.token()
    }

    pub fn get_type(&self) -> Option<LuaDocType> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocConditionalType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocConditionalType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeConditional
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

impl LuaDocConditionalType {
    pub fn get_types(&self) -> Option<(LuaDocType, LuaDocType, LuaDocType)> {
        let mut children = self.children();
        let condition = children.next()?;
        let true_type = children.next()?;
        let false_type = children.next()?;
        Some((condition, true_type, false_type))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocTupleType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocTupleType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeTuple
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

impl LuaDocTupleType {
    pub fn get_types(&self) -> LuaAstChildren<LuaDocType> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocLiteralType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocLiteralType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeLiteral
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

impl LuaDocLiteralType {
    pub fn get_literal(&self) -> Option<LuaLiteralToken> {
        self.token()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocVariadicType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocVariadicType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeVariadic
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

impl LuaDocVariadicType {
    pub fn get_type(&self) -> Option<LuaDocType> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocNullableType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocNullableType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeNullable
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

impl LuaDocNullableType {
    pub fn get_type(&self) -> Option<LuaDocType> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocGenericType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocGenericType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeGeneric
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

impl LuaDocGenericType {
    pub fn get_name_type(&self) -> Option<LuaDocNameType> {
        self.child()
    }

    pub fn get_generic_types(&self) -> Option<LuaDocTypeList> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocStrTplType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocStrTplType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeStringTemplate
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

impl LuaDocStrTplType {
    /// `T` or  xxx.`T` or xxx.`T`.xxxx
    pub fn get_name(&self) -> (Option<String>, Option<String>, Option<String>) {
        let str_tpl = self.token_by_kind(PyTokenKind::TkStringTemplateType);
        if str_tpl.is_none() {
            return (None, None, None);
        }
        let str_tpl = str_tpl.unwrap();
        let text = str_tpl.get_text();
        let mut iter = text.split('`');
        let first = iter.next().map(|it| it.to_string());
        let second = iter.next().map(|it| it.to_string());
        let third = iter.next().map(|it| it.to_string());

        (first, second, third)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocMultiLineUnionType {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocMultiLineUnionType {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::TypeMultiLineUnion
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

impl LuaDocMultiLineUnionType {
    pub fn get_fields(&self) -> LuaAstChildren<LuaDocOneLineField> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocOneLineField {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocOneLineField {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == PySyntaxKind::DocOneLineField
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

impl LuaDocDescriptionOwner for LuaDocOneLineField {}

impl LuaDocOneLineField {
    pub fn get_type(&self) -> Option<LuaDocType> {
        self.child()
    }
}
