//! Auto-generated from `schemas_microsoft_com_office_word_2010_wordml.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2010/wordml` (prefix `w14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2010/wordml";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "w14";

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

static ATTRS_RUN_CONFLICT_INSERTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:author", property_name: Some("Author"), type_name: "StringValue" },
    AttributeInfo { qname: "w:date", property_name: Some("Date"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w16du:dateUtc", property_name: Some("DateUtc"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_RUN_CONFLICT_INSERTION: &[ChildInfo] = &[
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
    ChildInfo { name: "w:CT_R/w:r", property_name: None },
    ChildInfo { name: "w:CT_BdoContentRun/w:bdo", property_name: None },
    ChildInfo { name: "w:CT_DirContentRun/w:dir", property_name: None },
];
static ATTRS_RUN_CONFLICT_DELETION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:author", property_name: Some("Author"), type_name: "StringValue" },
    AttributeInfo { qname: "w:date", property_name: Some("Date"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w16du:dateUtc", property_name: Some("DateUtc"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_RUN_CONFLICT_DELETION: &[ChildInfo] = &[
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
    ChildInfo { name: "w:CT_R/w:r", property_name: None },
    ChildInfo { name: "w:CT_BdoContentRun/w:bdo", property_name: None },
    ChildInfo { name: "w:CT_DirContentRun/w:dir", property_name: None },
];
static ATTRS_CONFLICT_INSERTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:author", property_name: Some("Author"), type_name: "StringValue" },
    AttributeInfo { qname: "w:date", property_name: Some("Date"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w16du:dateUtc", property_name: Some("DateUtc"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_CONFLICT_DELETION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:author", property_name: Some("Author"), type_name: "StringValue" },
    AttributeInfo { qname: "w:date", property_name: Some("Date"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w16du:dateUtc", property_name: Some("DateUtc"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_CUSTOM_XML_CONFLICT_INSERTION_RANGE_START: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:author", property_name: Some("Author"), type_name: "StringValue" },
    AttributeInfo { qname: "w:date", property_name: Some("Date"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w16du:dateUtc", property_name: Some("DateUtc"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_CUSTOM_XML_CONFLICT_DELETION_RANGE_START: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:author", property_name: Some("Author"), type_name: "StringValue" },
    AttributeInfo { qname: "w:date", property_name: Some("Date"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w16du:dateUtc", property_name: Some("DateUtc"), type_name: "DateTimeValue" },
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_TINT: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SHADE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_ALPHA: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_HUE_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SATURATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SATURATION_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SATURATION_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_LUMINANCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_LUMINANCE_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_LUMINANCE_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_RGB_COLOR_MODEL_HEX: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "HexBinaryValue" },
];
static CHILDREN_RGB_COLOR_MODEL_HEX: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_PositiveFixedPercentage/w14:tint", property_name: None },
    ChildInfo { name: "w14:CT_PositiveFixedPercentage/w14:shade", property_name: None },
    ChildInfo { name: "w14:CT_PositiveFixedPercentage/w14:alpha", property_name: None },
    ChildInfo { name: "w14:CT_PositivePercentage/w14:hueMod", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:sat", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:satOff", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:satMod", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:lum", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:lumOff", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:lumMod", property_name: None },
];
static ATTRS_SCHEME_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_SCHEME_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_PositiveFixedPercentage/w14:tint", property_name: None },
    ChildInfo { name: "w14:CT_PositiveFixedPercentage/w14:shade", property_name: None },
    ChildInfo { name: "w14:CT_PositiveFixedPercentage/w14:alpha", property_name: None },
    ChildInfo { name: "w14:CT_PositivePercentage/w14:hueMod", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:sat", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:satOff", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:satMod", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:lum", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:lumOff", property_name: None },
    ChildInfo { name: "w14:CT_Percentage/w14:lumMod", property_name: None },
];
static ATTRS_LINEAR_SHADE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:ang", property_name: Some("Angle"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:scaled", property_name: Some("Scaled"), type_name: "EnumValue" },
];
static ATTRS_PATH_SHADE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:path", property_name: Some("Path"), type_name: "EnumValue" },
];
static CHILDREN_PATH_SHADE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_RelativeRect/w14:fillToRect", property_name: Some("FillToRectangle") },
];
static CHILDREN_SOLID_COLOR_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_SRgbColor/w14:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "w14:CT_SchemeColor/w14:schemeClr", property_name: Some("SchemeColor") },
];
static CHILDREN_GRADIENT_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_GradientStopList/w14:gsLst", property_name: Some("GradientStopList") },
    ChildInfo { name: "w14:CT_LinearShadeProperties/w14:lin", property_name: None },
    ChildInfo { name: "w14:CT_PathShadeProperties/w14:path", property_name: None },
];
static ATTRS_PRESET_LINE_DASH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_LINE_JOIN_MITER_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:lim", property_name: Some("Limit"), type_name: "Int32Value" },
];
static ATTRS_GLOW: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:rad", property_name: Some("GlowRadius"), type_name: "Int64Value" },
];
static CHILDREN_GLOW: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_SRgbColor/w14:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "w14:CT_SchemeColor/w14:schemeClr", property_name: Some("SchemeColor") },
];
static ATTRS_SHADOW: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:blurRad", property_name: Some("BlurRadius"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:dist", property_name: Some("DistanceFromText"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:dir", property_name: Some("DirectionAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:sx", property_name: Some("HorizontalScalingFactor"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:sy", property_name: Some("VerticalScalingFactor"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:kx", property_name: Some("HorizontalSkewAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:ky", property_name: Some("VerticalSkewAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_SHADOW: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_SRgbColor/w14:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "w14:CT_SchemeColor/w14:schemeClr", property_name: Some("SchemeColor") },
];
static ATTRS_REFLECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:blurRad", property_name: Some("BlurRadius"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:stA", property_name: Some("StartingOpacity"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:stPos", property_name: Some("StartPosition"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:endA", property_name: Some("EndingOpacity"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:endPos", property_name: Some("EndPosition"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:dist", property_name: Some("DistanceFromText"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:dir", property_name: Some("DirectionAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:fadeDir", property_name: Some("FadeDirection"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:sx", property_name: Some("HorizontalScalingFactor"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:sy", property_name: Some("VerticalScalingFactor"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:kx", property_name: Some("HorizontalSkewAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:ky", property_name: Some("VerticalSkewAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static ATTRS_TEXT_OUTLINE_EFFECT: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:w", property_name: Some("LineWidth"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: "w14:cmpd", property_name: Some("Compound"), type_name: "EnumValue" },
    AttributeInfo { qname: "w14:algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_TEXT_OUTLINE_EFFECT: &[ChildInfo] = &[
    ChildInfo { name: "w:CT_Empty/w14:noFill", property_name: None },
    ChildInfo { name: "w14:CT_SolidColorFillProperties/w14:solidFill", property_name: None },
    ChildInfo { name: "w14:CT_GradientFillProperties/w14:gradFill", property_name: None },
    ChildInfo { name: "w14:CT_PresetLineDashProperties/w14:prstDash", property_name: None },
    ChildInfo { name: "w:CT_Empty/w14:round", property_name: None },
    ChildInfo { name: "w:CT_Empty/w14:bevel", property_name: None },
    ChildInfo { name: "w14:CT_LineJoinMiterProperties/w14:miter", property_name: None },
];
static CHILDREN_FILL_TEXT_EFFECT: &[ChildInfo] = &[
    ChildInfo { name: "w:CT_Empty/w14:noFill", property_name: Some("NoFillEmpty") },
    ChildInfo { name: "w14:CT_SolidColorFillProperties/w14:solidFill", property_name: Some("SolidColorFillProperties") },
    ChildInfo { name: "w14:CT_GradientFillProperties/w14:gradFill", property_name: Some("GradientFillProperties") },
];
static CHILDREN_SCENE3_D: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_Camera/w14:camera", property_name: Some("Camera") },
    ChildInfo { name: "w14:CT_LightRig/w14:lightRig", property_name: Some("LightRig") },
];
static ATTRS_PROPERTIES3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:extrusionH", property_name: Some("ExtrusionHeight"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:contourW", property_name: Some("ContourWidth"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:prstMaterial", property_name: Some("PresetMaterialType"), type_name: "EnumValue" },
];
static CHILDREN_PROPERTIES3_D: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_Bevel/w14:bevelT", property_name: Some("BevelTop") },
    ChildInfo { name: "w14:CT_Bevel/w14:bevelB", property_name: Some("BevelBottom") },
    ChildInfo { name: "w14:CT_Color/w14:extrusionClr", property_name: Some("ExtrusionColor") },
    ChildInfo { name: "w14:CT_Color/w14:contourClr", property_name: Some("ContourColor") },
];
static ATTRS_LIGATURES: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_NUMBERING_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_NUMBER_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_STYLISTIC_SETS: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_StyleSet/w14:styleSet", property_name: None },
];
static ATTRS_CONTEXTUAL_ALTERNATIVES: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_CONFLICT_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_DISCARD_IMAGE_EDITING_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_CHECKED: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_CONTENT_PART: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
    AttributeInfo { qname: "r:id", property_name: Some("RelationshipId"), type_name: "StringValue" },
];
static CHILDREN_CONTENT_PART: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_WordContentPartNonVisual/w14:nvContentPartPr", property_name: Some("WordNonVisualContentPartShapeProperties") },
    ChildInfo { name: "a:CT_Transform2D/w14:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/w14:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DOCUMENT_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "HexBinaryValue" },
];
static ATTRS_CUSTOM_XML_CONFLICT_INSERTION_RANGE_END: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_CUSTOM_XML_CONFLICT_DELETION_RANGE_END: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_DEFAULT_IMAGE_DPI: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static CHILDREN_SDT_CONTENT_CHECK_BOX: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_OnOff/w14:checked", property_name: Some("Checked") },
    ChildInfo { name: "w14:CT_SdtCheckboxSymbol/w14:checkedState", property_name: Some("CheckedState") },
    ChildInfo { name: "w14:CT_SdtCheckboxSymbol/w14:uncheckedState", property_name: Some("UncheckedState") },
];
static ATTRS_GRADIENT_STOP: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:pos", property_name: Some("StopPosition"), type_name: "Int32Value" },
];
static CHILDREN_GRADIENT_STOP: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_SRgbColor/w14:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "w14:CT_SchemeColor/w14:schemeClr", property_name: Some("SchemeColor") },
];
static ATTRS_FILL_TO_RECTANGLE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:l", property_name: Some("Left"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:t", property_name: Some("Top"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:r", property_name: Some("Right"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:b", property_name: Some("Bottom"), type_name: "Int32Value" },
];
static CHILDREN_GRADIENT_STOP_LIST: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_GradientStop/w14:gs", property_name: None },
];
static ATTRS_SPHERE_COORDINATES: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:lat", property_name: Some("Lattitude"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:lon", property_name: Some("Longitude"), type_name: "Int32Value" },
    AttributeInfo { qname: "w14:rev", property_name: Some("Revolution"), type_name: "Int32Value" },
];
static ATTRS_CAMERA: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:prst", property_name: Some("PresetCameraType"), type_name: "EnumValue" },
];
static ATTRS_LIGHT_RIG: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:rig", property_name: Some("LightRigType"), type_name: "EnumValue" },
    AttributeInfo { qname: "w14:dir", property_name: Some("LightDirectionType"), type_name: "EnumValue" },
];
static CHILDREN_LIGHT_RIG: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_SphereCoords/w14:rot", property_name: Some("SphereCoordinates") },
];
static ATTRS_BEVEL_TOP: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:w", property_name: Some("Width"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:h", property_name: Some("Height"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:prst", property_name: Some("PresetProfileType"), type_name: "EnumValue" },
];
static ATTRS_BEVEL_BOTTOM: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:w", property_name: Some("Width"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:h", property_name: Some("Height"), type_name: "Int64Value" },
    AttributeInfo { qname: "w14:prst", property_name: Some("PresetProfileType"), type_name: "EnumValue" },
];
static CHILDREN_EXTRUSION_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_SRgbColor/w14:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "w14:CT_SchemeColor/w14:schemeClr", property_name: Some("SchemeColor") },
];
static CHILDREN_CONTOUR_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "w14:CT_SRgbColor/w14:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "w14:CT_SchemeColor/w14:schemeClr", property_name: Some("SchemeColor") },
];
static ATTRS_STYLE_SET: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_CHECKED_STATE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:font", property_name: Some("Font"), type_name: "StringValue" },
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "HexBinaryValue" },
];
static ATTRS_UNCHECKED_STATE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w14:font", property_name: Some("Font"), type_name: "StringValue" },
    AttributeInfo { qname: "w14:val", property_name: Some("Val"), type_name: "HexBinaryValue" },
];
static ATTRS_NON_VISUAL_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":descr", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: Some("HyperlinkOnClick") },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkHover", property_name: Some("HyperlinkOnHover") },
    ChildInfo { name: "a:CT_NonVisualDrawingPropsExtensionList/a:extLst", property_name: Some("NonVisualDrawingPropertiesExtensionList") },
];
static ATTRS_NON_VISUAL_INK_CONTENT_PART_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isComment", property_name: Some("IsComment"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_INK_CONTENT_PART_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_ContentPartLocking/a14:cpLocks", property_name: Some("ContentPartLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a14:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_WORD_NON_VISUAL_CONTENT_PART_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/w14:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a14:CT_NonVisualInkContentPartProperties/w14:cNvContentPartPr", property_name: Some("NonVisualInkContentPartProperties") },
];
static ATTRS_TRANSFORM2_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":flipH", property_name: Some("HorizontalFlip"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":flipV", property_name: Some("VerticalFlip"), type_name: "BooleanValue" },
];
static CHILDREN_TRANSFORM2_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/a:off", property_name: Some("Offset") },
    ChildInfo { name: "a:CT_PositiveSize2D/a:ext", property_name: Some("Extents") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "RunConflictInsertion", local_name: "conflictIns", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RUN_CONFLICT_INSERTION, children: CHILDREN_RUN_CONFLICT_INSERTION },
    ElementInfo { class_name: "RunConflictDeletion", local_name: "conflictDel", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RUN_CONFLICT_DELETION, children: CHILDREN_RUN_CONFLICT_DELETION },
    ElementInfo { class_name: "ConflictInsertion", local_name: "conflictIns", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONFLICT_INSERTION, children: &[] },
    ElementInfo { class_name: "ConflictDeletion", local_name: "conflictDel", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONFLICT_DELETION, children: &[] },
    ElementInfo { class_name: "CustomXmlConflictInsertionRangeStart", local_name: "customXmlConflictInsRangeStart", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_XML_CONFLICT_INSERTION_RANGE_START, children: &[] },
    ElementInfo { class_name: "CustomXmlConflictDeletionRangeStart", local_name: "customXmlConflictDelRangeStart", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_XML_CONFLICT_DELETION_RANGE_START, children: &[] },
    ElementInfo { class_name: "Tint", local_name: "tint", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TINT, children: &[] },
    ElementInfo { class_name: "Shade", local_name: "shade", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHADE, children: &[] },
    ElementInfo { class_name: "Alpha", local_name: "alpha", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA, children: &[] },
    ElementInfo { class_name: "HueModulation", local_name: "hueMod", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HUE_MODULATION, children: &[] },
    ElementInfo { class_name: "Saturation", local_name: "sat", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SATURATION, children: &[] },
    ElementInfo { class_name: "SaturationOffset", local_name: "satOff", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SATURATION_OFFSET, children: &[] },
    ElementInfo { class_name: "SaturationModulation", local_name: "satMod", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SATURATION_MODULATION, children: &[] },
    ElementInfo { class_name: "Luminance", local_name: "lum", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LUMINANCE, children: &[] },
    ElementInfo { class_name: "LuminanceOffset", local_name: "lumOff", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LUMINANCE_OFFSET, children: &[] },
    ElementInfo { class_name: "LuminanceModulation", local_name: "lumMod", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LUMINANCE_MODULATION, children: &[] },
    ElementInfo { class_name: "RgbColorModelHex", local_name: "srgbClr", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RGB_COLOR_MODEL_HEX, children: CHILDREN_RGB_COLOR_MODEL_HEX },
    ElementInfo { class_name: "SchemeColor", local_name: "schemeClr", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SCHEME_COLOR, children: CHILDREN_SCHEME_COLOR },
    ElementInfo { class_name: "LinearShadeProperties", local_name: "lin", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LINEAR_SHADE_PROPERTIES, children: &[] },
    ElementInfo { class_name: "PathShadeProperties", local_name: "path", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PATH_SHADE_PROPERTIES, children: CHILDREN_PATH_SHADE_PROPERTIES },
    ElementInfo { class_name: "NoFillEmpty", local_name: "noFill", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "RoundEmpty", local_name: "round", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BevelEmpty", local_name: "bevel", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "EntityPickerEmpty", local_name: "entityPicker", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SolidColorFillProperties", local_name: "solidFill", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SOLID_COLOR_FILL_PROPERTIES },
    ElementInfo { class_name: "GradientFillProperties", local_name: "gradFill", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRADIENT_FILL_PROPERTIES },
    ElementInfo { class_name: "PresetLineDashProperties", local_name: "prstDash", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRESET_LINE_DASH_PROPERTIES, children: &[] },
    ElementInfo { class_name: "LineJoinMiterProperties", local_name: "miter", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LINE_JOIN_MITER_PROPERTIES, children: &[] },
    ElementInfo { class_name: "Glow", local_name: "glow", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GLOW, children: CHILDREN_GLOW },
    ElementInfo { class_name: "Shadow", local_name: "shadow", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHADOW, children: CHILDREN_SHADOW },
    ElementInfo { class_name: "Reflection", local_name: "reflection", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REFLECTION, children: &[] },
    ElementInfo { class_name: "TextOutlineEffect", local_name: "textOutline", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_OUTLINE_EFFECT, children: CHILDREN_TEXT_OUTLINE_EFFECT },
    ElementInfo { class_name: "FillTextEffect", local_name: "textFill", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILL_TEXT_EFFECT },
    ElementInfo { class_name: "Scene3D", local_name: "scene3d", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCENE3_D },
    ElementInfo { class_name: "Properties3D", local_name: "props3d", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PROPERTIES3_D, children: CHILDREN_PROPERTIES3_D },
    ElementInfo { class_name: "Ligatures", local_name: "ligatures", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LIGATURES, children: &[] },
    ElementInfo { class_name: "NumberingFormat", local_name: "numForm", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NUMBERING_FORMAT, children: &[] },
    ElementInfo { class_name: "NumberSpacing", local_name: "numSpacing", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NUMBER_SPACING, children: &[] },
    ElementInfo { class_name: "StylisticSets", local_name: "stylisticSets", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STYLISTIC_SETS },
    ElementInfo { class_name: "ContextualAlternatives", local_name: "cntxtAlts", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONTEXTUAL_ALTERNATIVES, children: &[] },
    ElementInfo { class_name: "ConflictMode", local_name: "conflictMode", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONFLICT_MODE, children: &[] },
    ElementInfo { class_name: "DiscardImageEditingData", local_name: "discardImageEditingData", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DISCARD_IMAGE_EDITING_DATA, children: &[] },
    ElementInfo { class_name: "Checked", local_name: "checked", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHECKED, children: &[] },
    ElementInfo { class_name: "ContentPart", local_name: "contentPart", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTENT_PART, children: CHILDREN_CONTENT_PART },
    ElementInfo { class_name: "DocumentId", local_name: "docId", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DOCUMENT_ID, children: &[] },
    ElementInfo { class_name: "CustomXmlConflictInsertionRangeEnd", local_name: "customXmlConflictInsRangeEnd", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_XML_CONFLICT_INSERTION_RANGE_END, children: &[] },
    ElementInfo { class_name: "CustomXmlConflictDeletionRangeEnd", local_name: "customXmlConflictDelRangeEnd", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_XML_CONFLICT_DELETION_RANGE_END, children: &[] },
    ElementInfo { class_name: "DefaultImageDpi", local_name: "defaultImageDpi", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DEFAULT_IMAGE_DPI, children: &[] },
    ElementInfo { class_name: "SdtContentCheckBox", local_name: "checkbox", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SDT_CONTENT_CHECK_BOX },
    ElementInfo { class_name: "GradientStop", local_name: "gs", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRADIENT_STOP, children: CHILDREN_GRADIENT_STOP },
    ElementInfo { class_name: "FillToRectangle", local_name: "fillToRect", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FILL_TO_RECTANGLE, children: &[] },
    ElementInfo { class_name: "GradientStopList", local_name: "gsLst", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRADIENT_STOP_LIST },
    ElementInfo { class_name: "SphereCoordinates", local_name: "rot", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SPHERE_COORDINATES, children: &[] },
    ElementInfo { class_name: "Camera", local_name: "camera", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CAMERA, children: &[] },
    ElementInfo { class_name: "LightRig", local_name: "lightRig", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LIGHT_RIG, children: CHILDREN_LIGHT_RIG },
    ElementInfo { class_name: "BevelTop", local_name: "bevelT", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BEVEL_TOP, children: &[] },
    ElementInfo { class_name: "BevelBottom", local_name: "bevelB", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BEVEL_BOTTOM, children: &[] },
    ElementInfo { class_name: "ExtrusionColor", local_name: "extrusionClr", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTRUSION_COLOR },
    ElementInfo { class_name: "ContourColor", local_name: "contourClr", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONTOUR_COLOR },
    ElementInfo { class_name: "StyleSet", local_name: "styleSet", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STYLE_SET, children: &[] },
    ElementInfo { class_name: "CheckedState", local_name: "checkedState", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHECKED_STATE, children: &[] },
    ElementInfo { class_name: "UncheckedState", local_name: "uncheckedState", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNCHECKED_STATE, children: &[] },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualInkContentPartProperties", local_name: "cNvContentPartPr", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_INK_CONTENT_PART_PROPERTIES, children: CHILDREN_NON_VISUAL_INK_CONTENT_PART_PROPERTIES },
    ElementInfo { class_name: "WordNonVisualContentPartShapeProperties", local_name: "nvContentPartPr", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WORD_NON_VISUAL_CONTENT_PART_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Transform2D", local_name: "xfrm", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "w14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
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

/// Create a `<w14:conflictIns>` element (`RunConflictInsertion`).
pub fn run_conflict_insertion(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "conflictIns").with_children(children)
}

/// Create a `<w14:conflictDel>` element (`RunConflictDeletion`).
pub fn run_conflict_deletion(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "conflictDel").with_children(children)
}

/// Create a `<w14:conflictIns>` element (`ConflictInsertion`).
pub fn conflict_insertion() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "conflictIns")
}

/// Create a `<w14:conflictDel>` element (`ConflictDeletion`).
pub fn conflict_deletion() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "conflictDel")
}

/// Create a `<w14:customXmlConflictInsRangeStart>` element (`CustomXmlConflictInsertionRangeStart`).
pub fn custom_xml_conflict_insertion_range_start() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "customXmlConflictInsRangeStart")
}

/// Create a `<w14:customXmlConflictDelRangeStart>` element (`CustomXmlConflictDeletionRangeStart`).
pub fn custom_xml_conflict_deletion_range_start() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "customXmlConflictDelRangeStart")
}

/// Create a `<w14:tint>` element (`Tint`).
pub fn tint() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "tint")
}

/// Create a `<w14:shade>` element (`Shade`).
pub fn shade() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "shade")
}

/// Create a `<w14:alpha>` element (`Alpha`).
pub fn alpha() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "alpha")
}

/// Create a `<w14:hueMod>` element (`HueModulation`).
pub fn hue_modulation() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "hueMod")
}

/// Create a `<w14:sat>` element (`Saturation`).
pub fn saturation() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "sat")
}

/// Create a `<w14:satOff>` element (`SaturationOffset`).
pub fn saturation_offset() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "satOff")
}

/// Create a `<w14:satMod>` element (`SaturationModulation`).
pub fn saturation_modulation() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "satMod")
}

/// Create a `<w14:lum>` element (`Luminance`).
pub fn luminance() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "lum")
}

/// Create a `<w14:lumOff>` element (`LuminanceOffset`).
pub fn luminance_offset() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "lumOff")
}

/// Create a `<w14:lumMod>` element (`LuminanceModulation`).
pub fn luminance_modulation() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "lumMod")
}

/// Create a `<w14:srgbClr>` element (`RgbColorModelHex`).
pub fn rgb_color_model_hex(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "srgbClr").with_children(children)
}

/// Create a `<w14:schemeClr>` element (`SchemeColor`).
pub fn scheme_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "schemeClr").with_children(children)
}

/// Create a `<w14:lin>` element (`LinearShadeProperties`).
pub fn linear_shade_properties() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "lin")
}

/// Create a `<w14:path>` element (`PathShadeProperties`).
pub fn path_shade_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "path").with_children(children)
}

/// Create a `<w14:noFill>` element (`NoFillEmpty`).
pub fn no_fill_empty() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "noFill")
}

/// Create a `<w14:round>` element (`RoundEmpty`).
pub fn round_empty() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "round")
}

/// Create a `<w14:bevel>` element (`BevelEmpty`).
pub fn bevel_empty() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "bevel")
}

/// Create a `<w14:entityPicker>` element (`EntityPickerEmpty`).
pub fn entity_picker_empty() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "entityPicker")
}

/// Create a `<w14:solidFill>` element (`SolidColorFillProperties`).
pub fn solid_color_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "solidFill").with_children(children)
}

/// Create a `<w14:gradFill>` element (`GradientFillProperties`).
pub fn gradient_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "gradFill").with_children(children)
}

/// Create a `<w14:prstDash>` element (`PresetLineDashProperties`).
pub fn preset_line_dash_properties() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "prstDash")
}

/// Create a `<w14:miter>` element (`LineJoinMiterProperties`).
pub fn line_join_miter_properties() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "miter")
}

/// Create a `<w14:glow>` element (`Glow`).
pub fn glow(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "glow").with_children(children)
}

/// Create a `<w14:shadow>` element (`Shadow`).
pub fn shadow(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "shadow").with_children(children)
}

/// Create a `<w14:reflection>` element (`Reflection`).
pub fn reflection() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "reflection")
}

/// Create a `<w14:textOutline>` element (`TextOutlineEffect`).
pub fn text_outline_effect(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "textOutline").with_children(children)
}

/// Create a `<w14:textFill>` element (`FillTextEffect`).
pub fn fill_text_effect(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "textFill").with_children(children)
}

/// Create a `<w14:scene3d>` element (`Scene3D`).
pub fn scene3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "scene3d").with_children(children)
}

/// Create a `<w14:props3d>` element (`Properties3D`).
pub fn properties3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "props3d").with_children(children)
}

/// Create a `<w14:ligatures>` element (`Ligatures`).
pub fn ligatures() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "ligatures")
}

/// Create a `<w14:numForm>` element (`NumberingFormat`).
pub fn numbering_format() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "numForm")
}

/// Create a `<w14:numSpacing>` element (`NumberSpacing`).
pub fn number_spacing() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "numSpacing")
}

/// Create a `<w14:stylisticSets>` element (`StylisticSets`).
pub fn stylistic_sets(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "stylisticSets").with_children(children)
}

/// Create a `<w14:cntxtAlts>` element (`ContextualAlternatives`).
pub fn contextual_alternatives() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "cntxtAlts")
}

/// Create a `<w14:conflictMode>` element (`ConflictMode`).
pub fn conflict_mode() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "conflictMode")
}

/// Create a `<w14:discardImageEditingData>` element (`DiscardImageEditingData`).
pub fn discard_image_editing_data() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "discardImageEditingData")
}

/// Create a `<w14:checked>` element (`Checked`).
pub fn checked() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "checked")
}

/// Create a `<w14:contentPart>` element (`ContentPart`).
pub fn content_part(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "contentPart").with_children(children)
}

/// Create a `<w14:docId>` element (`DocumentId`).
pub fn document_id() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "docId")
}

/// Create a `<w14:customXmlConflictInsRangeEnd>` element (`CustomXmlConflictInsertionRangeEnd`).
pub fn custom_xml_conflict_insertion_range_end() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "customXmlConflictInsRangeEnd")
}

/// Create a `<w14:customXmlConflictDelRangeEnd>` element (`CustomXmlConflictDeletionRangeEnd`).
pub fn custom_xml_conflict_deletion_range_end() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "customXmlConflictDelRangeEnd")
}

/// Create a `<w14:defaultImageDpi>` element (`DefaultImageDpi`).
pub fn default_image_dpi() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "defaultImageDpi")
}

/// Create a `<w14:checkbox>` element (`SdtContentCheckBox`).
pub fn sdt_content_check_box(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "checkbox").with_children(children)
}

/// Create a `<w14:gs>` element (`GradientStop`).
pub fn gradient_stop(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "gs").with_children(children)
}

/// Create a `<w14:fillToRect>` element (`FillToRectangle`).
pub fn fill_to_rectangle() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "fillToRect")
}

/// Create a `<w14:gsLst>` element (`GradientStopList`).
pub fn gradient_stop_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "gsLst").with_children(children)
}

/// Create a `<w14:rot>` element (`SphereCoordinates`).
pub fn sphere_coordinates() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "rot")
}

/// Create a `<w14:camera>` element (`Camera`).
pub fn camera() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "camera")
}

/// Create a `<w14:lightRig>` element (`LightRig`).
pub fn light_rig(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "lightRig").with_children(children)
}

/// Create a `<w14:bevelT>` element (`BevelTop`).
pub fn bevel_top() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "bevelT")
}

/// Create a `<w14:bevelB>` element (`BevelBottom`).
pub fn bevel_bottom() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "bevelB")
}

/// Create a `<w14:extrusionClr>` element (`ExtrusionColor`).
pub fn extrusion_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "extrusionClr").with_children(children)
}

/// Create a `<w14:contourClr>` element (`ContourColor`).
pub fn contour_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "contourClr").with_children(children)
}

/// Create a `<w14:styleSet>` element (`StyleSet`).
pub fn style_set() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "styleSet")
}

/// Create a `<w14:checkedState>` element (`CheckedState`).
pub fn checked_state() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "checkedState")
}

/// Create a `<w14:uncheckedState>` element (`UncheckedState`).
pub fn unchecked_state() -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "uncheckedState")
}

/// Create a `<w14:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<w14:cNvContentPartPr>` element (`NonVisualInkContentPartProperties`).
pub fn non_visual_ink_content_part_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "cNvContentPartPr").with_children(children)
}

/// Create a `<w14:nvContentPartPr>` element (`WordNonVisualContentPartShapeProperties`).
pub fn word_non_visual_content_part_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "nvContentPartPr").with_children(children)
}

/// Create a `<w14:xfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Create a `<w14:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w14", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 77;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 67;
