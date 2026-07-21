//! Auto-generated from `schemas_openxmlformats_org_officeDocument_2006_math.json`.
//! Target namespace: `http://schemas.openxmlformats.org/officeDocument/2006/math` (prefix `m`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "m";

/// Metadata for a schema element.
#[derive(Debug, Clone, Copy)]
pub struct ElementInfo {
    pub class_name: &'static str,
    pub local_name: &'static str,
    pub prefix: &'static str,
    pub namespace_uri: &'static str,
    pub is_leaf: bool,
    pub is_leaf_text: bool,
    pub attributes: &'static [AttributeInfo],
    pub children: &'static [ChildInfo],
}

/// Schema attribute metadata.
#[derive(Debug, Clone, Copy)]
pub struct AttributeInfo {
    pub qname: &'static str,
    pub property_name: Option<&'static str>,
    pub type_name: &'static str,
}

/// Schema child-element metadata.
#[derive(Debug, Clone, Copy)]
pub struct ChildInfo {
    pub name: &'static str,
    pub property_name: Option<&'static str>,
}

static ATTRS_SCRIPT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_RUN: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_RPR/m:rPr", property_name: Some("MathRunProperties") },
    ChildInfo { name: "w:CT_RPr/w:rPr", property_name: Some("RunProperties") },
    ChildInfo { name: "w:CT_Br/w:br", property_name: None },
    ChildInfo { name: "w:CT_Text/w:t", property_name: None },
    ChildInfo { name: "w:CT_Text/w:delText", property_name: None },
    ChildInfo { name: "w:CT_Text/w:instrText", property_name: None },
    ChildInfo { name: "w:CT_Text/w:delInstrText", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:noBreakHyphen", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:softHyphen", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:dayShort", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:monthShort", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:yearShort", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:dayLong", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:monthLong", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:yearLong", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:annotationRef", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:footnoteRef", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:endnoteRef", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:separator", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:continuationSeparator", property_name: None },
    ChildInfo { name: "w:CT_Sym/w:sym", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:pgNum", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:cr", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:tab", property_name: None },
    ChildInfo { name: "w:CT_Object/w:object", property_name: None },
    ChildInfo { name: "w:CT_Picture/w:pict", property_name: None },
    ChildInfo { name: "w:CT_FldChar/w:fldChar", property_name: None },
    ChildInfo { name: "w:CT_Ruby/w:ruby", property_name: None },
    ChildInfo { name: "w:CT_FtnEdnRef/w:footnoteReference", property_name: None },
    ChildInfo { name: "w:CT_FtnEdnRef/w:endnoteReference", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:commentReference", property_name: None },
    ChildInfo { name: "w:CT_Drawing/w:drawing", property_name: None },
    ChildInfo { name: "w:CT_PTab/w:ptab", property_name: None },
    ChildInfo { name: "w:CT_Empty/w:lastRenderedPageBreak", property_name: None },
    ChildInfo { name: "m:CT_Text/m:t", property_name: None },
];
static CHILDREN_ACCENT: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_AccPr/m:accPr", property_name: Some("AccentProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_BAR: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_BarPr/m:barPr", property_name: Some("BarProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_BOX_: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_BoxPr/m:boxPr", property_name: Some("BoxProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_BORDER_BOX: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_BorderBoxPr/m:borderBoxPr", property_name: Some("BorderBoxProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_DELIMITER: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_DPr/m:dPr", property_name: Some("DelimiterProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: None },
];
static CHILDREN_EQUATION_ARRAY: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_EqArrPr/m:eqArrPr", property_name: Some("EquationArrayProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: None },
];
static CHILDREN_FRACTION: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_FPr/m:fPr", property_name: Some("FractionProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:num", property_name: Some("Numerator") },
    ChildInfo { name: "m:CT_OMathArg/m:den", property_name: Some("Denominator") },
];
static CHILDREN_MATH_FUNCTION: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_FuncPr/m:funcPr", property_name: Some("FunctionProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:fName", property_name: Some("FunctionName") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_GROUP_CHAR: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_GroupChrPr/m:groupChrPr", property_name: Some("GroupCharProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_LIMIT_LOWER: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_LimLowPr/m:limLowPr", property_name: Some("LimitLowerProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
    ChildInfo { name: "m:CT_OMathArg/m:lim", property_name: Some("Limit") },
];
static CHILDREN_LIMIT_UPPER: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_LimUppPr/m:limUppPr", property_name: Some("LimitUpperProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
    ChildInfo { name: "m:CT_OMathArg/m:lim", property_name: Some("Limit") },
];
static CHILDREN_MATRIX: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_MPr/m:mPr", property_name: Some("MatrixProperties") },
    ChildInfo { name: "m:CT_MR/m:mr", property_name: None },
];
static CHILDREN_NARY: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_NaryPr/m:naryPr", property_name: Some("NaryProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:sub", property_name: Some("SubArgument") },
    ChildInfo { name: "m:CT_OMathArg/m:sup", property_name: Some("SuperArgument") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_PHANTOM: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_PhantPr/m:phantPr", property_name: Some("PhantomProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_RADICAL: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_RadPr/m:radPr", property_name: Some("RadicalProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:deg", property_name: Some("Degree") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_PRE_SUB_SUPER: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_SPrePr/m:sPrePr", property_name: Some("PreSubSuperProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:sub", property_name: Some("SubArgument") },
    ChildInfo { name: "m:CT_OMathArg/m:sup", property_name: Some("SuperArgument") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
];
static CHILDREN_SUBSCRIPT: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_SSubPr/m:sSubPr", property_name: Some("SubscriptProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
    ChildInfo { name: "m:CT_OMathArg/m:sub", property_name: Some("SubArgument") },
];
static CHILDREN_SUB_SUPERSCRIPT: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_SSubSupPr/m:sSubSupPr", property_name: Some("SubSuperscriptProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
    ChildInfo { name: "m:CT_OMathArg/m:sub", property_name: Some("SubArgument") },
    ChildInfo { name: "m:CT_OMathArg/m:sup", property_name: Some("SuperArgument") },
];
static CHILDREN_SUPERSCRIPT: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_SSupPr/m:sSupPr", property_name: Some("SuperscriptProperties") },
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: Some("Base") },
    ChildInfo { name: "m:CT_OMathArg/m:sup", property_name: Some("SuperArgument") },
];
static CHILDREN_PARAGRAPH: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathParaPr/m:oMathParaPr", property_name: Some("ParagraphProperties") },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "w:CT_R/w:r", property_name: None },
];
static CHILDREN_OFFICE_MATH: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
];
static CHILDREN_MATH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_FontFace/m:mathFont", property_name: Some("MathFont") },
    ChildInfo { name: "m:CT_BreakBin/m:brkBin", property_name: Some("BreakBinary") },
    ChildInfo { name: "m:CT_BreakBinSub/m:brkBinSub", property_name: Some("BreakBinarySubtraction") },
    ChildInfo { name: "m:CT_OnOff/m:smallFrac", property_name: Some("SmallFraction") },
    ChildInfo { name: "m:CT_OnOff/m:dispDef", property_name: Some("DisplayDefaults") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:lMargin", property_name: Some("LeftMargin") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:rMargin", property_name: Some("RightMargin") },
    ChildInfo { name: "m:CT_OMathJc/m:defJc", property_name: Some("DefaultJustification") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:preSp", property_name: Some("PreSpacing") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:postSp", property_name: Some("PostSpacing") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:interSp", property_name: Some("InterSpacing") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:intraSp", property_name: Some("IntraSpacing") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:wrapIndent", property_name: None },
    ChildInfo { name: "m:CT_OnOff/m:wrapRight", property_name: None },
    ChildInfo { name: "m:CT_LimLoc/m:intLim", property_name: None },
    ChildInfo { name: "m:CT_LimLoc/m:naryLim", property_name: None },
];
static ATTRS_LITERAL: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_NORMAL_TEXT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ALIGNMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_OPERATOR_EMULATOR: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_NO_BREAK: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_DIFFERENTIAL: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_TOP: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_BOTTOM: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_LEFT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_RIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_STRIKE_HORIZONTAL: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_STRIKE_VERTICAL: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_STRIKE_BOTTOM_LEFT_TO_TOP_RIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_STRIKE_TOP_LEFT_TO_BOTTOM_RIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_GROW_OPERATORS: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_MAX_DISTRIBUTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_OBJECT_DISTRIBUTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_PLACEHOLDER: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_SUB_ARGUMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_SUPER_ARGUMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_SHOW_PHANTOM: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ZERO_WIDTH: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ZERO_ASCENT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ZERO_DESCENT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_TRANSPARENT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIDE_DEGREE: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ALIGN_SCRIPTS: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_SMALL_FRACTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_DISPLAY_DEFAULTS: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_WRAP_RIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_BREAK_: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:alnAt", property_name: Some("AlignAt"), type_name: "IntegerValue" },
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "IntegerValue" },
];
static CHILDREN_RUN_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OnOff/m:lit", property_name: Some("Literal") },
    ChildInfo { name: "m:CT_OnOff/m:nor", property_name: None },
    ChildInfo { name: "m:CT_Script/m:scr", property_name: None },
    ChildInfo { name: "m:CT_Style/m:sty", property_name: None },
    ChildInfo { name: "m:CT_ManualBreak/m:brk", property_name: None },
    ChildInfo { name: "m:CT_OnOff/m:aln", property_name: None },
];
static ATTRS_TEXT: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:space", property_name: Some("Space"), type_name: "EnumValue" },
];
static ATTRS_ACCENT_CHAR: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_BEGIN_CHAR: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_SEPARATOR_CHAR: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_END_CHAR: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_CONTROL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "w:CT_RPr/w:rPr", property_name: None },
    ChildInfo { name: "w:CT_MathCtrlIns/w:ins", property_name: None },
    ChildInfo { name: "w:CT_MathCtrlDel/w:del", property_name: None },
    ChildInfo { name: "w:CT_MathCtrlMove/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_MathCtrlMove/w:moveTo", property_name: None },
];
static CHILDREN_ACCENT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_Char/m:chr", property_name: Some("AccentChar") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_BASE: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static CHILDREN_NUMERATOR: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static CHILDREN_DENOMINATOR: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static CHILDREN_FUNCTION_NAME: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static CHILDREN_LIMIT: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static CHILDREN_SUB_ARGUMENT: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static CHILDREN_SUPER_ARGUMENT: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static CHILDREN_DEGREE: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArgPr/m:argPr", property_name: Some("ArgumentProperties") },
    ChildInfo { name: "m:CT_Acc/m:acc", property_name: None },
    ChildInfo { name: "m:CT_Bar/m:bar", property_name: None },
    ChildInfo { name: "m:CT_Box/m:box", property_name: None },
    ChildInfo { name: "m:CT_BorderBox/m:borderBox", property_name: None },
    ChildInfo { name: "m:CT_D/m:d", property_name: None },
    ChildInfo { name: "m:CT_EqArr/m:eqArr", property_name: None },
    ChildInfo { name: "m:CT_F/m:f", property_name: None },
    ChildInfo { name: "m:CT_Func/m:func", property_name: None },
    ChildInfo { name: "m:CT_GroupChr/m:groupChr", property_name: None },
    ChildInfo { name: "m:CT_LimLow/m:limLow", property_name: None },
    ChildInfo { name: "m:CT_LimUpp/m:limUpp", property_name: None },
    ChildInfo { name: "m:CT_M/m:m", property_name: None },
    ChildInfo { name: "m:CT_Nary/m:nary", property_name: None },
    ChildInfo { name: "m:CT_Phant/m:phant", property_name: None },
    ChildInfo { name: "m:CT_Rad/m:rad", property_name: None },
    ChildInfo { name: "m:CT_SPre/m:sPre", property_name: None },
    ChildInfo { name: "m:CT_SSub/m:sSub", property_name: None },
    ChildInfo { name: "m:CT_SSubSup/m:sSubSup", property_name: None },
    ChildInfo { name: "m:CT_SSup/m:sSup", property_name: None },
    ChildInfo { name: "m:CT_R/m:r", property_name: None },
    ChildInfo { name: "w:CT_CustomXmlRun/w:customXml", property_name: None },
    ChildInfo { name: "w:CT_SimpleField/w:fldSimple", property_name: None },
    ChildInfo { name: "w:CT_Hyperlink/w:hyperlink", property_name: None },
    ChildInfo { name: "w:CT_SdtRun/w:sdt", property_name: None },
    ChildInfo { name: "w:CT_ProofErr/w:proofErr", property_name: None },
    ChildInfo { name: "w:CT_PermStart/w:permStart", property_name: None },
    ChildInfo { name: "w:CT_Perm/w:permEnd", property_name: None },
    ChildInfo { name: "w:CT_Bookmark/w:bookmarkStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:bookmarkEnd", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:commentRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_MoveBookmark/w:moveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_MarkupRange/w:moveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveFromRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveFromRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w:customXmlMoveToRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w:customXmlMoveToRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:ins", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:del", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveFrom", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w:moveTo", property_name: None },
    ChildInfo { name: "w:CT_ContentPart/w:contentPart", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictIns", property_name: None },
    ChildInfo { name: "w:CT_RunTrackChange/w14:conflictDel", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: None },
];
static ATTRS_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_VERTICAL_JUSTIFICATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_BAR_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_TopBot/m:pos", property_name: Some("Position") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_BOX_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OnOff/m:opEmu", property_name: Some("OperatorEmulator") },
    ChildInfo { name: "m:CT_OnOff/m:noBreak", property_name: Some("NoBreak") },
    ChildInfo { name: "m:CT_OnOff/m:diff", property_name: Some("Differential") },
    ChildInfo { name: "m:CT_ManualBreak/m:brk", property_name: Some("Break") },
    ChildInfo { name: "m:CT_OnOff/m:aln", property_name: Some("Alignment") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_BORDER_BOX_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OnOff/m:hideTop", property_name: Some("HideTop") },
    ChildInfo { name: "m:CT_OnOff/m:hideBot", property_name: Some("HideBottom") },
    ChildInfo { name: "m:CT_OnOff/m:hideLeft", property_name: Some("HideLeft") },
    ChildInfo { name: "m:CT_OnOff/m:hideRight", property_name: Some("HideRight") },
    ChildInfo { name: "m:CT_OnOff/m:strikeH", property_name: Some("StrikeHorizontal") },
    ChildInfo { name: "m:CT_OnOff/m:strikeV", property_name: Some("StrikeVertical") },
    ChildInfo { name: "m:CT_OnOff/m:strikeBLTR", property_name: Some("StrikeBottomLeftToTopRight") },
    ChildInfo { name: "m:CT_OnOff/m:strikeTLBR", property_name: Some("StrikeTopLeftToBottomRight") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static ATTRS_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_DELIMITER_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_Char/m:begChr", property_name: Some("BeginChar") },
    ChildInfo { name: "m:CT_Char/m:sepChr", property_name: Some("SeparatorChar") },
    ChildInfo { name: "m:CT_Char/m:endChr", property_name: Some("EndChar") },
    ChildInfo { name: "m:CT_OnOff/m:grow", property_name: Some("GrowOperators") },
    ChildInfo { name: "m:CT_Shp/m:shp", property_name: Some("Shape") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static ATTRS_BASE_JUSTIFICATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ROW_SPACING_RULE: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "IntegerValue" },
];
static ATTRS_COLUMN_GAP_RULE: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "IntegerValue" },
];
static ATTRS_ROW_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static ATTRS_COLUMN_GAP: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static CHILDREN_EQUATION_ARRAY_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_YAlign/m:baseJc", property_name: Some("BaseJustification") },
    ChildInfo { name: "m:CT_OnOff/m:maxDist", property_name: Some("MaxDistribution") },
    ChildInfo { name: "m:CT_OnOff/m:objDist", property_name: Some("ObjectDistribution") },
    ChildInfo { name: "m:CT_SpacingRule/m:rSpRule", property_name: Some("RowSpacingRule") },
    ChildInfo { name: "m:CT_UnSignedShort/m:rSp", property_name: Some("RowSpacing") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static ATTRS_FRACTION_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_FRACTION_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_FType/m:type", property_name: Some("FractionType") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_FUNCTION_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_GROUP_CHAR_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_Char/m:chr", property_name: Some("AccentChar") },
    ChildInfo { name: "m:CT_TopBot/m:pos", property_name: Some("Position") },
    ChildInfo { name: "m:CT_TopBot/m:vertJc", property_name: Some("VerticalJustification") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_LIMIT_LOWER_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_LIMIT_UPPER_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static ATTRS_MATRIX_COLUMN_COUNT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "IntegerValue" },
];
static ATTRS_MATRIX_COLUMN_JUSTIFICATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_MATRIX_COLUMN_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_Integer64/m:count", property_name: Some("MatrixColumnCount") },
    ChildInfo { name: "m:CT_XAlign/m:mcJc", property_name: Some("MatrixColumnJustification") },
];
static CHILDREN_MATRIX_COLUMN: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_MCPr/m:mcPr", property_name: Some("MatrixColumnProperties") },
];
static ATTRS_COLUMN_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_LEFT_MARGIN: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_RIGHT_MARGIN: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_PRE_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_POST_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_INTER_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_INTRA_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_WRAP_INDENT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static CHILDREN_MATRIX_COLUMNS: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_MC/m:mc", property_name: None },
];
static CHILDREN_MATRIX_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_YAlign/m:baseJc", property_name: Some("BaseJustification") },
    ChildInfo { name: "m:CT_OnOff/m:plcHide", property_name: Some("HidePlaceholder") },
    ChildInfo { name: "m:CT_SpacingRule/m:rSpRule", property_name: Some("RowSpacingRule") },
    ChildInfo { name: "m:CT_SpacingRule/m:cGpRule", property_name: Some("ColumnGapRule") },
    ChildInfo { name: "m:CT_UnSignedShort/m:rSp", property_name: Some("RowSpacing") },
    ChildInfo { name: "m:CT_TwipsMeasure/m:cSp", property_name: Some("ColumnSpacing") },
    ChildInfo { name: "m:CT_UnSignedShort/m:cGp", property_name: Some("ColumnGap") },
    ChildInfo { name: "m:CT_MCS/m:mcs", property_name: Some("MatrixColumns") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_MATRIX_ROW: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathArg/m:e", property_name: None },
];
static ATTRS_LIMIT_LOCATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_INTEGRAL_LIMIT_LOCATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_NARY_LIMIT_LOCATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_NARY_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_Char/m:chr", property_name: Some("AccentChar") },
    ChildInfo { name: "m:CT_LimLoc/m:limLoc", property_name: Some("LimitLocation") },
    ChildInfo { name: "m:CT_OnOff/m:grow", property_name: Some("GrowOperators") },
    ChildInfo { name: "m:CT_OnOff/m:subHide", property_name: Some("HideSubArgument") },
    ChildInfo { name: "m:CT_OnOff/m:supHide", property_name: Some("HideSuperArgument") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_PHANTOM_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OnOff/m:show", property_name: Some("ShowPhantom") },
    ChildInfo { name: "m:CT_OnOff/m:zeroWid", property_name: Some("ZeroWidth") },
    ChildInfo { name: "m:CT_OnOff/m:zeroAsc", property_name: Some("ZeroAscent") },
    ChildInfo { name: "m:CT_OnOff/m:zeroDesc", property_name: Some("ZeroDescent") },
    ChildInfo { name: "m:CT_OnOff/m:transp", property_name: Some("Transparent") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_RADICAL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OnOff/m:degHide", property_name: Some("HideDegree") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_PRE_SUB_SUPER_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_SUBSCRIPT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_SUB_SUPERSCRIPT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OnOff/m:alnScr", property_name: Some("AlignScripts") },
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static CHILDREN_SUPERSCRIPT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_CtrlPr/m:ctrlPr", property_name: Some("ControlProperties") },
];
static ATTRS_ARGUMENT_SIZE: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "IntegerValue" },
];
static CHILDREN_ARGUMENT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_Integer2/m:argSz", property_name: Some("ArgumentSize") },
];
static ATTRS_JUSTIFICATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_DEFAULT_JUSTIFICATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_MATH_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_BREAK_BINARY: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_BREAK_BINARY_SUBTRACTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "m:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "m:CT_OMathJc/m:jc", property_name: Some("Justification") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Script", local_name: "scr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SCRIPT, children: &[] },
    ElementInfo { class_name: "Style", local_name: "sty", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STYLE, children: &[] },
    ElementInfo { class_name: "Run", local_name: "r", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RUN },
    ElementInfo { class_name: "Accent", local_name: "acc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT },
    ElementInfo { class_name: "Bar", local_name: "bar", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR },
    ElementInfo { class_name: "Box", local_name: "box", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BOX_ },
    ElementInfo { class_name: "BorderBox", local_name: "borderBox", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BORDER_BOX },
    ElementInfo { class_name: "Delimiter", local_name: "d", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DELIMITER },
    ElementInfo { class_name: "EquationArray", local_name: "eqArr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EQUATION_ARRAY },
    ElementInfo { class_name: "Fraction", local_name: "f", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FRACTION },
    ElementInfo { class_name: "MathFunction", local_name: "func", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATH_FUNCTION },
    ElementInfo { class_name: "GroupChar", local_name: "groupChr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_CHAR },
    ElementInfo { class_name: "LimitLower", local_name: "limLow", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIMIT_LOWER },
    ElementInfo { class_name: "LimitUpper", local_name: "limUpp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIMIT_UPPER },
    ElementInfo { class_name: "Matrix", local_name: "m", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATRIX },
    ElementInfo { class_name: "Nary", local_name: "nary", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NARY },
    ElementInfo { class_name: "Phantom", local_name: "phant", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PHANTOM },
    ElementInfo { class_name: "Radical", local_name: "rad", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RADICAL },
    ElementInfo { class_name: "PreSubSuper", local_name: "sPre", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRE_SUB_SUPER },
    ElementInfo { class_name: "Subscript", local_name: "sSub", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUBSCRIPT },
    ElementInfo { class_name: "SubSuperscript", local_name: "sSubSup", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUB_SUPERSCRIPT },
    ElementInfo { class_name: "Superscript", local_name: "sSup", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUPERSCRIPT },
    ElementInfo { class_name: "Paragraph", local_name: "oMathPara", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PARAGRAPH },
    ElementInfo { class_name: "OfficeMath", local_name: "oMath", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_MATH },
    ElementInfo { class_name: "MathProperties", local_name: "mathPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATH_PROPERTIES },
    ElementInfo { class_name: "Literal", local_name: "lit", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LITERAL, children: &[] },
    ElementInfo { class_name: "NormalText", local_name: "nor", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NORMAL_TEXT, children: &[] },
    ElementInfo { class_name: "Alignment", local_name: "aln", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALIGNMENT, children: &[] },
    ElementInfo { class_name: "OperatorEmulator", local_name: "opEmu", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OPERATOR_EMULATOR, children: &[] },
    ElementInfo { class_name: "NoBreak", local_name: "noBreak", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NO_BREAK, children: &[] },
    ElementInfo { class_name: "Differential", local_name: "diff", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DIFFERENTIAL, children: &[] },
    ElementInfo { class_name: "HideTop", local_name: "hideTop", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_TOP, children: &[] },
    ElementInfo { class_name: "HideBottom", local_name: "hideBot", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_BOTTOM, children: &[] },
    ElementInfo { class_name: "HideLeft", local_name: "hideLeft", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_LEFT, children: &[] },
    ElementInfo { class_name: "HideRight", local_name: "hideRight", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_RIGHT, children: &[] },
    ElementInfo { class_name: "StrikeHorizontal", local_name: "strikeH", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STRIKE_HORIZONTAL, children: &[] },
    ElementInfo { class_name: "StrikeVertical", local_name: "strikeV", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STRIKE_VERTICAL, children: &[] },
    ElementInfo { class_name: "StrikeBottomLeftToTopRight", local_name: "strikeBLTR", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STRIKE_BOTTOM_LEFT_TO_TOP_RIGHT, children: &[] },
    ElementInfo { class_name: "StrikeTopLeftToBottomRight", local_name: "strikeTLBR", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STRIKE_TOP_LEFT_TO_BOTTOM_RIGHT, children: &[] },
    ElementInfo { class_name: "GrowOperators", local_name: "grow", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GROW_OPERATORS, children: &[] },
    ElementInfo { class_name: "MaxDistribution", local_name: "maxDist", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MAX_DISTRIBUTION, children: &[] },
    ElementInfo { class_name: "ObjectDistribution", local_name: "objDist", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OBJECT_DISTRIBUTION, children: &[] },
    ElementInfo { class_name: "HidePlaceholder", local_name: "plcHide", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_PLACEHOLDER, children: &[] },
    ElementInfo { class_name: "HideSubArgument", local_name: "subHide", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_SUB_ARGUMENT, children: &[] },
    ElementInfo { class_name: "HideSuperArgument", local_name: "supHide", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_SUPER_ARGUMENT, children: &[] },
    ElementInfo { class_name: "ShowPhantom", local_name: "show", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_PHANTOM, children: &[] },
    ElementInfo { class_name: "ZeroWidth", local_name: "zeroWid", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ZERO_WIDTH, children: &[] },
    ElementInfo { class_name: "ZeroAscent", local_name: "zeroAsc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ZERO_ASCENT, children: &[] },
    ElementInfo { class_name: "ZeroDescent", local_name: "zeroDesc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ZERO_DESCENT, children: &[] },
    ElementInfo { class_name: "Transparent", local_name: "transp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TRANSPARENT, children: &[] },
    ElementInfo { class_name: "HideDegree", local_name: "degHide", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_DEGREE, children: &[] },
    ElementInfo { class_name: "AlignScripts", local_name: "alnScr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALIGN_SCRIPTS, children: &[] },
    ElementInfo { class_name: "SmallFraction", local_name: "smallFrac", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SMALL_FRACTION, children: &[] },
    ElementInfo { class_name: "DisplayDefaults", local_name: "dispDef", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DISPLAY_DEFAULTS, children: &[] },
    ElementInfo { class_name: "WrapRight", local_name: "wrapRight", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WRAP_RIGHT, children: &[] },
    ElementInfo { class_name: "Break", local_name: "brk", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BREAK_, children: &[] },
    ElementInfo { class_name: "RunProperties", local_name: "rPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RUN_PROPERTIES },
    ElementInfo { class_name: "Text", local_name: "t", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_TEXT, children: &[] },
    ElementInfo { class_name: "AccentChar", local_name: "chr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ACCENT_CHAR, children: &[] },
    ElementInfo { class_name: "BeginChar", local_name: "begChr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BEGIN_CHAR, children: &[] },
    ElementInfo { class_name: "SeparatorChar", local_name: "sepChr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SEPARATOR_CHAR, children: &[] },
    ElementInfo { class_name: "EndChar", local_name: "endChr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_END_CHAR, children: &[] },
    ElementInfo { class_name: "ControlProperties", local_name: "ctrlPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONTROL_PROPERTIES },
    ElementInfo { class_name: "AccentProperties", local_name: "accPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT_PROPERTIES },
    ElementInfo { class_name: "Base", local_name: "e", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BASE },
    ElementInfo { class_name: "Numerator", local_name: "num", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUMERATOR },
    ElementInfo { class_name: "Denominator", local_name: "den", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DENOMINATOR },
    ElementInfo { class_name: "FunctionName", local_name: "fName", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FUNCTION_NAME },
    ElementInfo { class_name: "Limit", local_name: "lim", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIMIT },
    ElementInfo { class_name: "SubArgument", local_name: "sub", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUB_ARGUMENT },
    ElementInfo { class_name: "SuperArgument", local_name: "sup", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUPER_ARGUMENT },
    ElementInfo { class_name: "Degree", local_name: "deg", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DEGREE },
    ElementInfo { class_name: "Position", local_name: "pos", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POSITION, children: &[] },
    ElementInfo { class_name: "VerticalJustification", local_name: "vertJc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VERTICAL_JUSTIFICATION, children: &[] },
    ElementInfo { class_name: "BarProperties", local_name: "barPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR_PROPERTIES },
    ElementInfo { class_name: "BoxProperties", local_name: "boxPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BOX_PROPERTIES },
    ElementInfo { class_name: "BorderBoxProperties", local_name: "borderBoxPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BORDER_BOX_PROPERTIES },
    ElementInfo { class_name: "Shape", local_name: "shp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHAPE, children: &[] },
    ElementInfo { class_name: "DelimiterProperties", local_name: "dPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DELIMITER_PROPERTIES },
    ElementInfo { class_name: "BaseJustification", local_name: "baseJc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BASE_JUSTIFICATION, children: &[] },
    ElementInfo { class_name: "RowSpacingRule", local_name: "rSpRule", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROW_SPACING_RULE, children: &[] },
    ElementInfo { class_name: "ColumnGapRule", local_name: "cGpRule", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLUMN_GAP_RULE, children: &[] },
    ElementInfo { class_name: "RowSpacing", local_name: "rSp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROW_SPACING, children: &[] },
    ElementInfo { class_name: "ColumnGap", local_name: "cGp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLUMN_GAP, children: &[] },
    ElementInfo { class_name: "EquationArrayProperties", local_name: "eqArrPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EQUATION_ARRAY_PROPERTIES },
    ElementInfo { class_name: "FractionType", local_name: "type", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FRACTION_TYPE, children: &[] },
    ElementInfo { class_name: "FractionProperties", local_name: "fPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FRACTION_PROPERTIES },
    ElementInfo { class_name: "FunctionProperties", local_name: "funcPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FUNCTION_PROPERTIES },
    ElementInfo { class_name: "GroupCharProperties", local_name: "groupChrPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_CHAR_PROPERTIES },
    ElementInfo { class_name: "LimitLowerProperties", local_name: "limLowPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIMIT_LOWER_PROPERTIES },
    ElementInfo { class_name: "LimitUpperProperties", local_name: "limUppPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIMIT_UPPER_PROPERTIES },
    ElementInfo { class_name: "MatrixColumnCount", local_name: "count", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MATRIX_COLUMN_COUNT, children: &[] },
    ElementInfo { class_name: "MatrixColumnJustification", local_name: "mcJc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MATRIX_COLUMN_JUSTIFICATION, children: &[] },
    ElementInfo { class_name: "MatrixColumnProperties", local_name: "mcPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATRIX_COLUMN_PROPERTIES },
    ElementInfo { class_name: "MatrixColumn", local_name: "mc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATRIX_COLUMN },
    ElementInfo { class_name: "ColumnSpacing", local_name: "cSp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLUMN_SPACING, children: &[] },
    ElementInfo { class_name: "LeftMargin", local_name: "lMargin", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEFT_MARGIN, children: &[] },
    ElementInfo { class_name: "RightMargin", local_name: "rMargin", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RIGHT_MARGIN, children: &[] },
    ElementInfo { class_name: "PreSpacing", local_name: "preSp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRE_SPACING, children: &[] },
    ElementInfo { class_name: "PostSpacing", local_name: "postSp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POST_SPACING, children: &[] },
    ElementInfo { class_name: "InterSpacing", local_name: "interSp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INTER_SPACING, children: &[] },
    ElementInfo { class_name: "IntraSpacing", local_name: "intraSp", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INTRA_SPACING, children: &[] },
    ElementInfo { class_name: "WrapIndent", local_name: "wrapIndent", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WRAP_INDENT, children: &[] },
    ElementInfo { class_name: "MatrixColumns", local_name: "mcs", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATRIX_COLUMNS },
    ElementInfo { class_name: "MatrixProperties", local_name: "mPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATRIX_PROPERTIES },
    ElementInfo { class_name: "MatrixRow", local_name: "mr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MATRIX_ROW },
    ElementInfo { class_name: "LimitLocation", local_name: "limLoc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LIMIT_LOCATION, children: &[] },
    ElementInfo { class_name: "IntegralLimitLocation", local_name: "intLim", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INTEGRAL_LIMIT_LOCATION, children: &[] },
    ElementInfo { class_name: "NaryLimitLocation", local_name: "naryLim", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NARY_LIMIT_LOCATION, children: &[] },
    ElementInfo { class_name: "NaryProperties", local_name: "naryPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NARY_PROPERTIES },
    ElementInfo { class_name: "PhantomProperties", local_name: "phantPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PHANTOM_PROPERTIES },
    ElementInfo { class_name: "RadicalProperties", local_name: "radPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RADICAL_PROPERTIES },
    ElementInfo { class_name: "PreSubSuperProperties", local_name: "sPrePr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRE_SUB_SUPER_PROPERTIES },
    ElementInfo { class_name: "SubscriptProperties", local_name: "sSubPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUBSCRIPT_PROPERTIES },
    ElementInfo { class_name: "SubSuperscriptProperties", local_name: "sSubSupPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUB_SUPERSCRIPT_PROPERTIES },
    ElementInfo { class_name: "SuperscriptProperties", local_name: "sSupPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUPERSCRIPT_PROPERTIES },
    ElementInfo { class_name: "ArgumentSize", local_name: "argSz", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARGUMENT_SIZE, children: &[] },
    ElementInfo { class_name: "ArgumentProperties", local_name: "argPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ARGUMENT_PROPERTIES },
    ElementInfo { class_name: "Justification", local_name: "jc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_JUSTIFICATION, children: &[] },
    ElementInfo { class_name: "DefaultJustification", local_name: "defJc", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DEFAULT_JUSTIFICATION, children: &[] },
    ElementInfo { class_name: "MathFont", local_name: "mathFont", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MATH_FONT, children: &[] },
    ElementInfo { class_name: "BreakBinary", local_name: "brkBin", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BREAK_BINARY, children: &[] },
    ElementInfo { class_name: "BreakBinarySubtraction", local_name: "brkBinSub", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BREAK_BINARY_SUBTRACTION, children: &[] },
    ElementInfo { class_name: "ParagraphProperties", local_name: "oMathParaPr", prefix: "m", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PARAGRAPH_PROPERTIES },
];

/// Look up element metadata by class name.
pub fn info_by_class(class_name: &str) -> Option<&'static ElementInfo> {
    ELEMENTS.iter().find(|e| e.class_name == class_name)
}

/// Look up element metadata by local name (first match).
pub fn info_by_local_name(local_name: &str) -> Option<&'static ElementInfo> {
    ELEMENTS.iter().find(|e| e.local_name == local_name)
}

/// Create an empty element by its schema class name (e.g. `"Paragraph"`).
pub fn create(class_name: &str) -> Option<OpenXmlElement> {
    let info = info_by_class(class_name)?;
    Some(OpenXmlElement::new(info.prefix, info.namespace_uri, info.local_name))
}

// ---------------------------------------------------------------------------
// Typed constructors
// ---------------------------------------------------------------------------

/// Create a `<m:scr>` element (`Script`).
pub fn script() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "scr")
}

/// Create a `<m:sty>` element (`Style`).
pub fn style() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sty")
}

/// Create a `<m:r>` element (`Run`).
pub fn run(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "r").with_children(children)
}

/// Create a `<m:acc>` element (`Accent`).
pub fn accent(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "acc").with_children(children)
}

/// Create a `<m:bar>` element (`Bar`).
pub fn bar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "bar").with_children(children)
}

/// Create a `<m:box>` element (`Box`).
pub fn box_(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "box").with_children(children)
}

/// Create a `<m:borderBox>` element (`BorderBox`).
pub fn border_box(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "borderBox").with_children(children)
}

/// Create a `<m:d>` element (`Delimiter`).
pub fn delimiter(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "d").with_children(children)
}

/// Create a `<m:eqArr>` element (`EquationArray`).
pub fn equation_array(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "eqArr").with_children(children)
}

/// Create a `<m:f>` element (`Fraction`).
pub fn fraction(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "f").with_children(children)
}

/// Create a `<m:func>` element (`MathFunction`).
pub fn math_function(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "func").with_children(children)
}

/// Create a `<m:groupChr>` element (`GroupChar`).
pub fn group_char(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "groupChr").with_children(children)
}

/// Create a `<m:limLow>` element (`LimitLower`).
pub fn limit_lower(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "limLow").with_children(children)
}

/// Create a `<m:limUpp>` element (`LimitUpper`).
pub fn limit_upper(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "limUpp").with_children(children)
}

/// Create a `<m:m>` element (`Matrix`).
pub fn matrix(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "m").with_children(children)
}

/// Create a `<m:nary>` element (`Nary`).
pub fn nary(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "nary").with_children(children)
}

/// Create a `<m:phant>` element (`Phantom`).
pub fn phantom(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "phant").with_children(children)
}

/// Create a `<m:rad>` element (`Radical`).
pub fn radical(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "rad").with_children(children)
}

/// Create a `<m:sPre>` element (`PreSubSuper`).
pub fn pre_sub_super(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sPre").with_children(children)
}

/// Create a `<m:sSub>` element (`Subscript`).
pub fn subscript(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sSub").with_children(children)
}

/// Create a `<m:sSubSup>` element (`SubSuperscript`).
pub fn sub_superscript(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sSubSup").with_children(children)
}

/// Create a `<m:sSup>` element (`Superscript`).
pub fn superscript(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sSup").with_children(children)
}

/// Create a `<m:oMathPara>` element (`Paragraph`).
pub fn paragraph(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "oMathPara").with_children(children)
}

/// Create a `<m:oMath>` element (`OfficeMath`).
pub fn office_math(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "oMath").with_children(children)
}

/// Create a `<m:mathPr>` element (`MathProperties`).
pub fn math_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mathPr").with_children(children)
}

/// Create a `<m:lit>` element (`Literal`).
pub fn literal() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "lit")
}

/// Create a `<m:nor>` element (`NormalText`).
pub fn normal_text() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "nor")
}

/// Create a `<m:aln>` element (`Alignment`).
pub fn alignment() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "aln")
}

/// Create a `<m:opEmu>` element (`OperatorEmulator`).
pub fn operator_emulator() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "opEmu")
}

/// Create a `<m:noBreak>` element (`NoBreak`).
pub fn no_break() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "noBreak")
}

/// Create a `<m:diff>` element (`Differential`).
pub fn differential() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "diff")
}

/// Create a `<m:hideTop>` element (`HideTop`).
pub fn hide_top() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "hideTop")
}

/// Create a `<m:hideBot>` element (`HideBottom`).
pub fn hide_bottom() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "hideBot")
}

/// Create a `<m:hideLeft>` element (`HideLeft`).
pub fn hide_left() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "hideLeft")
}

/// Create a `<m:hideRight>` element (`HideRight`).
pub fn hide_right() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "hideRight")
}

/// Create a `<m:strikeH>` element (`StrikeHorizontal`).
pub fn strike_horizontal() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "strikeH")
}

/// Create a `<m:strikeV>` element (`StrikeVertical`).
pub fn strike_vertical() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "strikeV")
}

/// Create a `<m:strikeBLTR>` element (`StrikeBottomLeftToTopRight`).
pub fn strike_bottom_left_to_top_right() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "strikeBLTR")
}

/// Create a `<m:strikeTLBR>` element (`StrikeTopLeftToBottomRight`).
pub fn strike_top_left_to_bottom_right() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "strikeTLBR")
}

/// Create a `<m:grow>` element (`GrowOperators`).
pub fn grow_operators() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "grow")
}

/// Create a `<m:maxDist>` element (`MaxDistribution`).
pub fn max_distribution() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "maxDist")
}

/// Create a `<m:objDist>` element (`ObjectDistribution`).
pub fn object_distribution() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "objDist")
}

/// Create a `<m:plcHide>` element (`HidePlaceholder`).
pub fn hide_placeholder() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "plcHide")
}

/// Create a `<m:subHide>` element (`HideSubArgument`).
pub fn hide_sub_argument() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "subHide")
}

/// Create a `<m:supHide>` element (`HideSuperArgument`).
pub fn hide_super_argument() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "supHide")
}

/// Create a `<m:show>` element (`ShowPhantom`).
pub fn show_phantom() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "show")
}

/// Create a `<m:zeroWid>` element (`ZeroWidth`).
pub fn zero_width() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "zeroWid")
}

/// Create a `<m:zeroAsc>` element (`ZeroAscent`).
pub fn zero_ascent() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "zeroAsc")
}

/// Create a `<m:zeroDesc>` element (`ZeroDescent`).
pub fn zero_descent() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "zeroDesc")
}

/// Create a `<m:transp>` element (`Transparent`).
pub fn transparent() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "transp")
}

/// Create a `<m:degHide>` element (`HideDegree`).
pub fn hide_degree() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "degHide")
}

/// Create a `<m:alnScr>` element (`AlignScripts`).
pub fn align_scripts() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "alnScr")
}

/// Create a `<m:smallFrac>` element (`SmallFraction`).
pub fn small_fraction() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "smallFrac")
}

/// Create a `<m:dispDef>` element (`DisplayDefaults`).
pub fn display_defaults() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "dispDef")
}

/// Create a `<m:wrapRight>` element (`WrapRight`).
pub fn wrap_right() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "wrapRight")
}

/// Create a `<m:brk>` element (`Break`).
pub fn break_() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "brk")
}

/// Create a `<m:rPr>` element (`RunProperties`).
pub fn run_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "rPr").with_children(children)
}

/// Create a `<m:t>` element (`Text`).
pub fn text(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "t").with_text(value)
}

/// Create a `<m:chr>` element (`AccentChar`).
pub fn accent_char() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "chr")
}

/// Create a `<m:begChr>` element (`BeginChar`).
pub fn begin_char() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "begChr")
}

/// Create a `<m:sepChr>` element (`SeparatorChar`).
pub fn separator_char() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sepChr")
}

/// Create a `<m:endChr>` element (`EndChar`).
pub fn end_char() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "endChr")
}

/// Create a `<m:ctrlPr>` element (`ControlProperties`).
pub fn control_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "ctrlPr").with_children(children)
}

/// Create a `<m:accPr>` element (`AccentProperties`).
pub fn accent_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "accPr").with_children(children)
}

/// Create a `<m:e>` element (`Base`).
pub fn base(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "e").with_children(children)
}

/// Create a `<m:num>` element (`Numerator`).
pub fn numerator(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "num").with_children(children)
}

/// Create a `<m:den>` element (`Denominator`).
pub fn denominator(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "den").with_children(children)
}

/// Create a `<m:fName>` element (`FunctionName`).
pub fn function_name(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "fName").with_children(children)
}

/// Create a `<m:lim>` element (`Limit`).
pub fn limit(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "lim").with_children(children)
}

/// Create a `<m:sub>` element (`SubArgument`).
pub fn sub_argument(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sub").with_children(children)
}

/// Create a `<m:sup>` element (`SuperArgument`).
pub fn super_argument(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sup").with_children(children)
}

/// Create a `<m:deg>` element (`Degree`).
pub fn degree(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "deg").with_children(children)
}

/// Create a `<m:pos>` element (`Position`).
pub fn position() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "pos")
}

/// Create a `<m:vertJc>` element (`VerticalJustification`).
pub fn vertical_justification() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "vertJc")
}

/// Create a `<m:barPr>` element (`BarProperties`).
pub fn bar_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "barPr").with_children(children)
}

/// Create a `<m:boxPr>` element (`BoxProperties`).
pub fn box_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "boxPr").with_children(children)
}

/// Create a `<m:borderBoxPr>` element (`BorderBoxProperties`).
pub fn border_box_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "borderBoxPr").with_children(children)
}

/// Create a `<m:shp>` element (`Shape`).
pub fn shape() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "shp")
}

/// Create a `<m:dPr>` element (`DelimiterProperties`).
pub fn delimiter_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "dPr").with_children(children)
}

/// Create a `<m:baseJc>` element (`BaseJustification`).
pub fn base_justification() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "baseJc")
}

/// Create a `<m:rSpRule>` element (`RowSpacingRule`).
pub fn row_spacing_rule() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "rSpRule")
}

/// Create a `<m:cGpRule>` element (`ColumnGapRule`).
pub fn column_gap_rule() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "cGpRule")
}

/// Create a `<m:rSp>` element (`RowSpacing`).
pub fn row_spacing() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "rSp")
}

/// Create a `<m:cGp>` element (`ColumnGap`).
pub fn column_gap() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "cGp")
}

/// Create a `<m:eqArrPr>` element (`EquationArrayProperties`).
pub fn equation_array_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "eqArrPr").with_children(children)
}

/// Create a `<m:type>` element (`FractionType`).
pub fn fraction_type() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "type")
}

/// Create a `<m:fPr>` element (`FractionProperties`).
pub fn fraction_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "fPr").with_children(children)
}

/// Create a `<m:funcPr>` element (`FunctionProperties`).
pub fn function_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "funcPr").with_children(children)
}

/// Create a `<m:groupChrPr>` element (`GroupCharProperties`).
pub fn group_char_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "groupChrPr").with_children(children)
}

/// Create a `<m:limLowPr>` element (`LimitLowerProperties`).
pub fn limit_lower_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "limLowPr").with_children(children)
}

/// Create a `<m:limUppPr>` element (`LimitUpperProperties`).
pub fn limit_upper_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "limUppPr").with_children(children)
}

/// Create a `<m:count>` element (`MatrixColumnCount`).
pub fn matrix_column_count() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "count")
}

/// Create a `<m:mcJc>` element (`MatrixColumnJustification`).
pub fn matrix_column_justification() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mcJc")
}

/// Create a `<m:mcPr>` element (`MatrixColumnProperties`).
pub fn matrix_column_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mcPr").with_children(children)
}

/// Create a `<m:mc>` element (`MatrixColumn`).
pub fn matrix_column(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mc").with_children(children)
}

/// Create a `<m:cSp>` element (`ColumnSpacing`).
pub fn column_spacing() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "cSp")
}

/// Create a `<m:lMargin>` element (`LeftMargin`).
pub fn left_margin() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "lMargin")
}

/// Create a `<m:rMargin>` element (`RightMargin`).
pub fn right_margin() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "rMargin")
}

/// Create a `<m:preSp>` element (`PreSpacing`).
pub fn pre_spacing() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "preSp")
}

/// Create a `<m:postSp>` element (`PostSpacing`).
pub fn post_spacing() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "postSp")
}

/// Create a `<m:interSp>` element (`InterSpacing`).
pub fn inter_spacing() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "interSp")
}

/// Create a `<m:intraSp>` element (`IntraSpacing`).
pub fn intra_spacing() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "intraSp")
}

/// Create a `<m:wrapIndent>` element (`WrapIndent`).
pub fn wrap_indent() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "wrapIndent")
}

/// Create a `<m:mcs>` element (`MatrixColumns`).
pub fn matrix_columns(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mcs").with_children(children)
}

/// Create a `<m:mPr>` element (`MatrixProperties`).
pub fn matrix_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mPr").with_children(children)
}

/// Create a `<m:mr>` element (`MatrixRow`).
pub fn matrix_row(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mr").with_children(children)
}

/// Create a `<m:limLoc>` element (`LimitLocation`).
pub fn limit_location() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "limLoc")
}

/// Create a `<m:intLim>` element (`IntegralLimitLocation`).
pub fn integral_limit_location() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "intLim")
}

/// Create a `<m:naryLim>` element (`NaryLimitLocation`).
pub fn nary_limit_location() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "naryLim")
}

/// Create a `<m:naryPr>` element (`NaryProperties`).
pub fn nary_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "naryPr").with_children(children)
}

/// Create a `<m:phantPr>` element (`PhantomProperties`).
pub fn phantom_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "phantPr").with_children(children)
}

/// Create a `<m:radPr>` element (`RadicalProperties`).
pub fn radical_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "radPr").with_children(children)
}

/// Create a `<m:sPrePr>` element (`PreSubSuperProperties`).
pub fn pre_sub_super_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sPrePr").with_children(children)
}

/// Create a `<m:sSubPr>` element (`SubscriptProperties`).
pub fn subscript_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sSubPr").with_children(children)
}

/// Create a `<m:sSubSupPr>` element (`SubSuperscriptProperties`).
pub fn sub_superscript_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sSubSupPr").with_children(children)
}

/// Create a `<m:sSupPr>` element (`SuperscriptProperties`).
pub fn superscript_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "sSupPr").with_children(children)
}

/// Create a `<m:argSz>` element (`ArgumentSize`).
pub fn argument_size() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "argSz")
}

/// Create a `<m:argPr>` element (`ArgumentProperties`).
pub fn argument_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "argPr").with_children(children)
}

/// Create a `<m:jc>` element (`Justification`).
pub fn justification() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "jc")
}

/// Create a `<m:defJc>` element (`DefaultJustification`).
pub fn default_justification() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "defJc")
}

/// Create a `<m:mathFont>` element (`MathFont`).
pub fn math_font() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "mathFont")
}

/// Create a `<m:brkBin>` element (`BreakBinary`).
pub fn break_binary() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "brkBin")
}

/// Create a `<m:brkBinSub>` element (`BreakBinarySubtraction`).
pub fn break_binary_subtraction() -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "brkBinSub")
}

/// Create a `<m:oMathParaPr>` element (`ParagraphProperties`).
pub fn paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", NAMESPACE_URI, "oMathParaPr").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 133;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 124;
