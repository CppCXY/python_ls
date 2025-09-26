mod doc;
mod lua;
mod test;
mod token;

#[allow(unused)]
pub use doc::*;
#[allow(unused)]
pub use lua::*;
#[allow(unused)]
pub use token::*;

use crate::kind::PySyntaxKind;

use super::{LuaSyntaxNode, traits::LuaAstNode};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaAst {
    LuaChunk(LuaChunk),
    LuaBlock(LuaBlock),
    // stats
    LuaAssignStat(LuaAssignStat),
    LuaLocalStat(LuaLocalStat),
    LuaCallExprStat(LuaCallExprStat),
    LuaLabelStat(LuaLabelStat),
    LuaBreakStat(LuaBreakStat),
    LuaGotoStat(LuaGotoStat),
    LuaDoStat(LuaDoStat),
    LuaWhileStat(LuaWhileStat),
    LuaRepeatStat(LuaRepeatStat),
    LuaIfStat(LuaIfStat),
    LuaForStat(LuaForStat),
    LuaForRangeStat(LuaForRangeStat),
    LuaFuncStat(LuaFuncStat),
    LuaLocalFuncStat(LuaLocalFuncStat),
    LuaReturnStat(LuaReturnStat),
    LuaGlobalStat(LuaGlobalStat),

    // exprs
    LuaNameExpr(LuaNameExpr),
    LuaIndexExpr(LuaIndexExpr),
    LuaTableExpr(LuaTableExpr),
    LuaBinaryExpr(LuaBinaryExpr),
    LuaUnaryExpr(LuaUnaryExpr),
    LuaParenExpr(LuaParenExpr),
    LuaCallExpr(LuaCallExpr),
    LuaLiteralExpr(LuaLiteralExpr),
    LuaClosureExpr(LuaClosureExpr),

    // other lua struct
    LuaTableField(LuaTableField),
    LuaParamList(LuaParamList),
    LuaParamName(LuaParamName),
    LuaCallArgList(LuaCallArgList),
    LuaLocalName(LuaLocalName),
    LuaLocalAttribute(LuaLocalAttribute),
    LuaElseIfClauseStat(LuaElseIfClauseStat),
    LuaElseClauseStat(LuaElseClauseStat),

    // comment
    LuaComment(LuaComment),
    // doc tag
    LuaDocTagClass(LuaDocTagClass),
    LuaDocTagEnum(LuaDocTagEnum),
    LuaDocTagAlias(LuaDocTagAlias),
    LuaDocTagType(LuaDocTagType),
    LuaDocTagParam(LuaDocTagParam),
    LuaDocTagReturn(LuaDocTagReturn),
    LuaDocTagOverload(LuaDocTagOverload),
    LuaDocTagField(LuaDocTagField),
    LuaDocTagModule(LuaDocTagModule),
    LuaDocTagSee(LuaDocTagSee),
    LuaDocTagDiagnostic(LuaDocTagDiagnostic),
    LuaDocTagDeprecated(LuaDocTagDeprecated),
    LuaDocTagVersion(LuaDocTagVersion),
    LuaDocTagCast(LuaDocTagCast),
    LuaDocTagSource(LuaDocTagSource),
    LuaDocTagOther(LuaDocTagOther),
    LuaDocTagNamespace(LuaDocTagNamespace),
    LuaDocTagUsing(LuaDocTagUsing),
    LuaDocTagMeta(LuaDocTagMeta),
    LuaDocTagNodiscard(LuaDocTagNodiscard),
    LuaDocTagReadonly(LuaDocTagReadonly),
    LuaDocTagOperator(LuaDocTagOperator),
    LuaDocTagGeneric(LuaDocTagGeneric),
    LuaDocTagAsync(LuaDocTagAsync),
    LuaDocTagAs(LuaDocTagAs),
    LuaDocTagReturnCast(LuaDocTagReturnCast),
    LuaDocTagExport(LuaDocTagExport),
    LuaDocTagLanguage(LuaDocTagLanguage),
    // doc description
    LuaDocDescription(LuaDocDescription),

    // doc type
    LuaDocNameType(LuaDocNameType),
    LuaDocArrayType(LuaDocArrayType),
    LuaDocFuncType(LuaDocFuncType),
    LuaDocObjectType(LuaDocObjectType),
    LuaDocBinaryType(LuaDocBinaryType),
    LuaDocUnaryType(LuaDocUnaryType),
    LuaDocConditionalType(LuaDocConditionalType),
    LuaDocTupleType(LuaDocTupleType),
    LuaDocLiteralType(LuaDocLiteralType),
    LuaDocVariadicType(LuaDocVariadicType),
    LuaDocNullableType(LuaDocNullableType),
    LuaDocGenericType(LuaDocGenericType),
    LuaDocStrTplType(LuaDocStrTplType),
    LuaDocMultiLineUnionType(LuaDocMultiLineUnionType),
    // other structure do not need enum here
}

impl LuaAstNode for LuaAst {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaAst::LuaChunk(node) => node.syntax(),
            LuaAst::LuaBlock(node) => node.syntax(),
            LuaAst::LuaAssignStat(node) => node.syntax(),
            LuaAst::LuaLocalStat(node) => node.syntax(),
            LuaAst::LuaCallExprStat(node) => node.syntax(),
            LuaAst::LuaLabelStat(node) => node.syntax(),
            LuaAst::LuaBreakStat(node) => node.syntax(),
            LuaAst::LuaGotoStat(node) => node.syntax(),
            LuaAst::LuaDoStat(node) => node.syntax(),
            LuaAst::LuaWhileStat(node) => node.syntax(),
            LuaAst::LuaRepeatStat(node) => node.syntax(),
            LuaAst::LuaIfStat(node) => node.syntax(),
            LuaAst::LuaForStat(node) => node.syntax(),
            LuaAst::LuaForRangeStat(node) => node.syntax(),
            LuaAst::LuaFuncStat(node) => node.syntax(),
            LuaAst::LuaLocalFuncStat(node) => node.syntax(),
            LuaAst::LuaReturnStat(node) => node.syntax(),
            LuaAst::LuaGlobalStat(node) => node.syntax(),
            LuaAst::LuaNameExpr(node) => node.syntax(),
            LuaAst::LuaIndexExpr(node) => node.syntax(),
            LuaAst::LuaTableExpr(node) => node.syntax(),
            LuaAst::LuaBinaryExpr(node) => node.syntax(),
            LuaAst::LuaUnaryExpr(node) => node.syntax(),
            LuaAst::LuaParenExpr(node) => node.syntax(),
            LuaAst::LuaCallExpr(node) => node.syntax(),
            LuaAst::LuaLiteralExpr(node) => node.syntax(),
            LuaAst::LuaClosureExpr(node) => node.syntax(),
            LuaAst::LuaComment(node) => node.syntax(),
            LuaAst::LuaTableField(node) => node.syntax(),
            LuaAst::LuaParamList(node) => node.syntax(),
            LuaAst::LuaParamName(node) => node.syntax(),
            LuaAst::LuaCallArgList(node) => node.syntax(),
            LuaAst::LuaLocalName(node) => node.syntax(),
            LuaAst::LuaLocalAttribute(node) => node.syntax(),
            LuaAst::LuaElseIfClauseStat(node) => node.syntax(),
            LuaAst::LuaElseClauseStat(node) => node.syntax(),
            LuaAst::LuaDocTagClass(node) => node.syntax(),
            LuaAst::LuaDocTagEnum(node) => node.syntax(),
            LuaAst::LuaDocTagAlias(node) => node.syntax(),
            LuaAst::LuaDocTagType(node) => node.syntax(),
            LuaAst::LuaDocTagParam(node) => node.syntax(),
            LuaAst::LuaDocTagReturn(node) => node.syntax(),
            LuaAst::LuaDocTagOverload(node) => node.syntax(),
            LuaAst::LuaDocTagField(node) => node.syntax(),
            LuaAst::LuaDocTagModule(node) => node.syntax(),
            LuaAst::LuaDocTagSee(node) => node.syntax(),
            LuaAst::LuaDocTagDiagnostic(node) => node.syntax(),
            LuaAst::LuaDocTagDeprecated(node) => node.syntax(),
            LuaAst::LuaDocTagVersion(node) => node.syntax(),
            LuaAst::LuaDocTagCast(node) => node.syntax(),
            LuaAst::LuaDocTagSource(node) => node.syntax(),
            LuaAst::LuaDocTagOther(node) => node.syntax(),
            LuaAst::LuaDocTagNamespace(node) => node.syntax(),
            LuaAst::LuaDocTagUsing(node) => node.syntax(),
            LuaAst::LuaDocTagMeta(node) => node.syntax(),
            LuaAst::LuaDocTagNodiscard(node) => node.syntax(),
            LuaAst::LuaDocTagReadonly(node) => node.syntax(),
            LuaAst::LuaDocTagOperator(node) => node.syntax(),
            LuaAst::LuaDocTagGeneric(node) => node.syntax(),
            LuaAst::LuaDocTagAsync(node) => node.syntax(),
            LuaAst::LuaDocTagAs(node) => node.syntax(),
            LuaAst::LuaDocTagReturnCast(node) => node.syntax(),
            LuaAst::LuaDocTagExport(node) => node.syntax(),
            LuaAst::LuaDocTagLanguage(node) => node.syntax(),
            LuaAst::LuaDocDescription(node) => node.syntax(),
            LuaAst::LuaDocNameType(node) => node.syntax(),
            LuaAst::LuaDocArrayType(node) => node.syntax(),
            LuaAst::LuaDocFuncType(node) => node.syntax(),
            LuaAst::LuaDocObjectType(node) => node.syntax(),
            LuaAst::LuaDocBinaryType(node) => node.syntax(),
            LuaAst::LuaDocUnaryType(node) => node.syntax(),
            LuaAst::LuaDocConditionalType(node) => node.syntax(),
            LuaAst::LuaDocTupleType(node) => node.syntax(),
            LuaAst::LuaDocLiteralType(node) => node.syntax(),
            LuaAst::LuaDocVariadicType(node) => node.syntax(),
            LuaAst::LuaDocNullableType(node) => node.syntax(),
            LuaAst::LuaDocGenericType(node) => node.syntax(),
            LuaAst::LuaDocStrTplType(node) => node.syntax(),
            LuaAst::LuaDocMultiLineUnionType(node) => node.syntax(),
        }
    }

    fn can_cast(kind: PySyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            PySyntaxKind::Chunk
                | PySyntaxKind::Suite
                | PySyntaxKind::AssignStat
                | PySyntaxKind::LocalStat
                | PySyntaxKind::CallExprStat
                | PySyntaxKind::LabelStat
                | PySyntaxKind::BreakStat
                | PySyntaxKind::GotoStat
                | PySyntaxKind::DoStat
                | PySyntaxKind::WhileStat
                | PySyntaxKind::RepeatStat
                | PySyntaxKind::IfStat
                | PySyntaxKind::ForStat
                | PySyntaxKind::ForRangeStat
                | PySyntaxKind::FuncStat
                | PySyntaxKind::LocalFuncStat
                | PySyntaxKind::ReturnStat
                | PySyntaxKind::GlobalStat
                | PySyntaxKind::NameExpr
                | PySyntaxKind::IndexExpr
                | PySyntaxKind::TableEmptyExpr
                | PySyntaxKind::TableArrayExpr
                | PySyntaxKind::TableObjectExpr
                | PySyntaxKind::BinaryExpr
                | PySyntaxKind::UnaryExpr
                | PySyntaxKind::ParenExpr
                | PySyntaxKind::CallExpr
                | PySyntaxKind::AssertCallExpr
                | PySyntaxKind::ErrorCallExpr
                | PySyntaxKind::RequireCallExpr
                | PySyntaxKind::TypeCallExpr
                | PySyntaxKind::SetmetatableCallExpr
                | PySyntaxKind::LiteralExpr
                | PySyntaxKind::ClosureExpr
                | PySyntaxKind::ParamList
                | PySyntaxKind::CallArgList
                | PySyntaxKind::LocalName
                | PySyntaxKind::TableFieldAssign
                | PySyntaxKind::TableFieldValue
                | PySyntaxKind::ParamName
                | PySyntaxKind::Attribute
                | PySyntaxKind::ElseIfClauseStat
                | PySyntaxKind::ElseClauseStat
                | PySyntaxKind::Comment
                | PySyntaxKind::DocTagClass
                | PySyntaxKind::DocTagEnum
                | PySyntaxKind::DocTagAlias
                | PySyntaxKind::DocTagType
                | PySyntaxKind::DocTagParam
                | PySyntaxKind::DocTagReturn
                | PySyntaxKind::DocTagOverload
                | PySyntaxKind::DocTagField
                | PySyntaxKind::DocTagModule
                | PySyntaxKind::DocTagSee
                | PySyntaxKind::DocTagDiagnostic
                | PySyntaxKind::DocTagDeprecated
                | PySyntaxKind::DocTagVersion
                | PySyntaxKind::DocTagCast
                | PySyntaxKind::DocTagSource
                | PySyntaxKind::DocTagOther
                | PySyntaxKind::DocTagNamespace
                | PySyntaxKind::DocTagUsing
                | PySyntaxKind::DocTagMeta
                | PySyntaxKind::DocTagNodiscard
                | PySyntaxKind::DocTagReadonly
                | PySyntaxKind::DocTagOperator
                | PySyntaxKind::DocTagGeneric
                | PySyntaxKind::DocTagAsync
                | PySyntaxKind::DocTagAs
                | PySyntaxKind::DocTagReturnCast
                | PySyntaxKind::DocTagExport
                | PySyntaxKind::DocTagLanguage
                | PySyntaxKind::TypeName
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
            PySyntaxKind::Chunk => LuaChunk::cast(syntax).map(LuaAst::LuaChunk),
            PySyntaxKind::Suite => LuaBlock::cast(syntax).map(LuaAst::LuaBlock),
            PySyntaxKind::AssignStat => LuaAssignStat::cast(syntax).map(LuaAst::LuaAssignStat),
            PySyntaxKind::LocalStat => LuaLocalStat::cast(syntax).map(LuaAst::LuaLocalStat),
            PySyntaxKind::CallExprStat => {
                LuaCallExprStat::cast(syntax).map(LuaAst::LuaCallExprStat)
            }
            PySyntaxKind::LabelStat => LuaLabelStat::cast(syntax).map(LuaAst::LuaLabelStat),
            PySyntaxKind::BreakStat => LuaBreakStat::cast(syntax).map(LuaAst::LuaBreakStat),
            PySyntaxKind::GotoStat => LuaGotoStat::cast(syntax).map(LuaAst::LuaGotoStat),
            PySyntaxKind::DoStat => LuaDoStat::cast(syntax).map(LuaAst::LuaDoStat),
            PySyntaxKind::WhileStat => LuaWhileStat::cast(syntax).map(LuaAst::LuaWhileStat),
            PySyntaxKind::RepeatStat => LuaRepeatStat::cast(syntax).map(LuaAst::LuaRepeatStat),
            PySyntaxKind::IfStat => LuaIfStat::cast(syntax).map(LuaAst::LuaIfStat),
            PySyntaxKind::ForStat => LuaForStat::cast(syntax).map(LuaAst::LuaForStat),
            PySyntaxKind::ForRangeStat => {
                LuaForRangeStat::cast(syntax).map(LuaAst::LuaForRangeStat)
            }
            PySyntaxKind::FuncStat => LuaFuncStat::cast(syntax).map(LuaAst::LuaFuncStat),
            PySyntaxKind::LocalFuncStat => {
                LuaLocalFuncStat::cast(syntax).map(LuaAst::LuaLocalFuncStat)
            }
            PySyntaxKind::ReturnStat => LuaReturnStat::cast(syntax).map(LuaAst::LuaReturnStat),
            PySyntaxKind::GlobalStat => LuaGlobalStat::cast(syntax).map(LuaAst::LuaGlobalStat),
            PySyntaxKind::NameExpr => LuaNameExpr::cast(syntax).map(LuaAst::LuaNameExpr),
            PySyntaxKind::IndexExpr => LuaIndexExpr::cast(syntax).map(LuaAst::LuaIndexExpr),
            PySyntaxKind::TableEmptyExpr
            | PySyntaxKind::TableArrayExpr
            | PySyntaxKind::TableObjectExpr => {
                LuaTableExpr::cast(syntax).map(LuaAst::LuaTableExpr)
            }
            PySyntaxKind::BinaryExpr => LuaBinaryExpr::cast(syntax).map(LuaAst::LuaBinaryExpr),
            PySyntaxKind::UnaryExpr => LuaUnaryExpr::cast(syntax).map(LuaAst::LuaUnaryExpr),
            PySyntaxKind::ParenExpr => LuaParenExpr::cast(syntax).map(LuaAst::LuaParenExpr),
            PySyntaxKind::CallExpr
            | PySyntaxKind::AssertCallExpr
            | PySyntaxKind::ErrorCallExpr
            | PySyntaxKind::RequireCallExpr
            | PySyntaxKind::TypeCallExpr
            | PySyntaxKind::SetmetatableCallExpr => {
                LuaCallExpr::cast(syntax).map(LuaAst::LuaCallExpr)
            }
            PySyntaxKind::LiteralExpr => LuaLiteralExpr::cast(syntax).map(LuaAst::LuaLiteralExpr),
            PySyntaxKind::ClosureExpr => LuaClosureExpr::cast(syntax).map(LuaAst::LuaClosureExpr),
            PySyntaxKind::Comment => LuaComment::cast(syntax).map(LuaAst::LuaComment),
            PySyntaxKind::TableFieldAssign | PySyntaxKind::TableFieldValue => {
                LuaTableField::cast(syntax).map(LuaAst::LuaTableField)
            }
            PySyntaxKind::ParamList => LuaParamList::cast(syntax).map(LuaAst::LuaParamList),
            PySyntaxKind::ParamName => LuaParamName::cast(syntax).map(LuaAst::LuaParamName),
            PySyntaxKind::CallArgList => LuaCallArgList::cast(syntax).map(LuaAst::LuaCallArgList),
            PySyntaxKind::LocalName => LuaLocalName::cast(syntax).map(LuaAst::LuaLocalName),
            PySyntaxKind::Attribute => {
                LuaLocalAttribute::cast(syntax).map(LuaAst::LuaLocalAttribute)
            }
            PySyntaxKind::ElseIfClauseStat => {
                LuaElseIfClauseStat::cast(syntax).map(LuaAst::LuaElseIfClauseStat)
            }
            PySyntaxKind::ElseClauseStat => {
                LuaElseClauseStat::cast(syntax).map(LuaAst::LuaElseClauseStat)
            }
            PySyntaxKind::DocTagClass => LuaDocTagClass::cast(syntax).map(LuaAst::LuaDocTagClass),
            PySyntaxKind::DocTagEnum => LuaDocTagEnum::cast(syntax).map(LuaAst::LuaDocTagEnum),
            PySyntaxKind::DocTagAlias => LuaDocTagAlias::cast(syntax).map(LuaAst::LuaDocTagAlias),
            PySyntaxKind::DocTagType => LuaDocTagType::cast(syntax).map(LuaAst::LuaDocTagType),
            PySyntaxKind::DocTagParam => LuaDocTagParam::cast(syntax).map(LuaAst::LuaDocTagParam),
            PySyntaxKind::DocTagReturn => {
                LuaDocTagReturn::cast(syntax).map(LuaAst::LuaDocTagReturn)
            }
            PySyntaxKind::DocTagOverload => {
                LuaDocTagOverload::cast(syntax).map(LuaAst::LuaDocTagOverload)
            }
            PySyntaxKind::DocTagField => LuaDocTagField::cast(syntax).map(LuaAst::LuaDocTagField),
            PySyntaxKind::DocTagModule => {
                LuaDocTagModule::cast(syntax).map(LuaAst::LuaDocTagModule)
            }
            PySyntaxKind::DocTagSee => LuaDocTagSee::cast(syntax).map(LuaAst::LuaDocTagSee),
            PySyntaxKind::DocTagDiagnostic => {
                LuaDocTagDiagnostic::cast(syntax).map(LuaAst::LuaDocTagDiagnostic)
            }
            PySyntaxKind::DocTagDeprecated => {
                LuaDocTagDeprecated::cast(syntax).map(LuaAst::LuaDocTagDeprecated)
            }
            PySyntaxKind::DocTagVersion => {
                LuaDocTagVersion::cast(syntax).map(LuaAst::LuaDocTagVersion)
            }
            PySyntaxKind::DocTagCast => LuaDocTagCast::cast(syntax).map(LuaAst::LuaDocTagCast),
            PySyntaxKind::DocTagSource => {
                LuaDocTagSource::cast(syntax).map(LuaAst::LuaDocTagSource)
            }
            PySyntaxKind::DocTagOther => LuaDocTagOther::cast(syntax).map(LuaAst::LuaDocTagOther),
            PySyntaxKind::DocTagNamespace => {
                LuaDocTagNamespace::cast(syntax).map(LuaAst::LuaDocTagNamespace)
            }
            PySyntaxKind::DocTagUsing => LuaDocTagUsing::cast(syntax).map(LuaAst::LuaDocTagUsing),
            PySyntaxKind::DocTagMeta => LuaDocTagMeta::cast(syntax).map(LuaAst::LuaDocTagMeta),
            PySyntaxKind::DocTagNodiscard => {
                LuaDocTagNodiscard::cast(syntax).map(LuaAst::LuaDocTagNodiscard)
            }
            PySyntaxKind::DocTagReadonly => {
                LuaDocTagReadonly::cast(syntax).map(LuaAst::LuaDocTagReadonly)
            }
            PySyntaxKind::DocTagOperator => {
                LuaDocTagOperator::cast(syntax).map(LuaAst::LuaDocTagOperator)
            }
            PySyntaxKind::DocTagGeneric => {
                LuaDocTagGeneric::cast(syntax).map(LuaAst::LuaDocTagGeneric)
            }
            PySyntaxKind::DocTagAsync => LuaDocTagAsync::cast(syntax).map(LuaAst::LuaDocTagAsync),
            PySyntaxKind::DocTagAs => LuaDocTagAs::cast(syntax).map(LuaAst::LuaDocTagAs),
            PySyntaxKind::DocTagReturnCast => {
                LuaDocTagReturnCast::cast(syntax).map(LuaAst::LuaDocTagReturnCast)
            }
            PySyntaxKind::DocTagExport => {
                LuaDocTagExport::cast(syntax).map(LuaAst::LuaDocTagExport)
            }
            PySyntaxKind::DocTagLanguage => {
                LuaDocTagLanguage::cast(syntax).map(LuaAst::LuaDocTagLanguage)
            }
            PySyntaxKind::DocDescription => {
                LuaDocDescription::cast(syntax).map(LuaAst::LuaDocDescription)
            }
            PySyntaxKind::TypeName => LuaDocNameType::cast(syntax).map(LuaAst::LuaDocNameType),
            PySyntaxKind::TypeArray => LuaDocArrayType::cast(syntax).map(LuaAst::LuaDocArrayType),
            PySyntaxKind::TypeFun => LuaDocFuncType::cast(syntax).map(LuaAst::LuaDocFuncType),
            PySyntaxKind::TypeObject => {
                LuaDocObjectType::cast(syntax).map(LuaAst::LuaDocObjectType)
            }
            PySyntaxKind::TypeBinary => {
                LuaDocBinaryType::cast(syntax).map(LuaAst::LuaDocBinaryType)
            }
            PySyntaxKind::TypeUnary => LuaDocUnaryType::cast(syntax).map(LuaAst::LuaDocUnaryType),
            PySyntaxKind::TypeConditional => {
                LuaDocConditionalType::cast(syntax).map(LuaAst::LuaDocConditionalType)
            }
            PySyntaxKind::TypeTuple => LuaDocTupleType::cast(syntax).map(LuaAst::LuaDocTupleType),
            PySyntaxKind::TypeLiteral => {
                LuaDocLiteralType::cast(syntax).map(LuaAst::LuaDocLiteralType)
            }
            PySyntaxKind::TypeVariadic => {
                LuaDocVariadicType::cast(syntax).map(LuaAst::LuaDocVariadicType)
            }
            PySyntaxKind::TypeNullable => {
                LuaDocNullableType::cast(syntax).map(LuaAst::LuaDocNullableType)
            }
            PySyntaxKind::TypeGeneric => {
                LuaDocGenericType::cast(syntax).map(LuaAst::LuaDocGenericType)
            }
            PySyntaxKind::TypeStringTemplate => {
                LuaDocStrTplType::cast(syntax).map(LuaAst::LuaDocStrTplType)
            }
            PySyntaxKind::TypeMultiLineUnion => {
                LuaDocMultiLineUnionType::cast(syntax).map(LuaAst::LuaDocMultiLineUnionType)
            }
            _ => None,
        }
    }
}
