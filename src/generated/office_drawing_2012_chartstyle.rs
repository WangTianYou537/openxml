//! Auto-generated from `schemas_microsoft_com_office_drawing_2012_chartStyle.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2012/chartStyle` (prefix `cs`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2012/chartStyle";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "cs";

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

static ATTRS_COLOR_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":meth", property_name: Some("Method"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
];
static CHILDREN_COLOR_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
    ChildInfo { name: "cs:CT_ColorStyleVariation/cs:variation", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: None },
];
static ATTRS_CHART_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
];
static CHILDREN_CHART_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleEntry/cs:axisTitle", property_name: Some("AxisTitle") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:categoryAxis", property_name: Some("CategoryAxis") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:chartArea", property_name: Some("ChartArea") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataLabel", property_name: Some("DataLabel") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataLabelCallout", property_name: Some("DataLabelCallout") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataPoint", property_name: Some("DataPoint") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataPoint3D", property_name: Some("DataPoint3D") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataPointLine", property_name: Some("DataPointLine") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataPointMarker", property_name: Some("DataPointMarker") },
    ChildInfo { name: "cs:CT_MarkerLayout/cs:dataPointMarkerLayout", property_name: Some("MarkerLayoutProperties") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataPointWireframe", property_name: Some("DataPointWireframe") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dataTable", property_name: Some("DataTableStyle") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:downBar", property_name: Some("DownBar") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:dropLine", property_name: Some("DropLine") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:errorBar", property_name: Some("ErrorBar") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:floor", property_name: Some("Floor") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:gridlineMajor", property_name: Some("GridlineMajor") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:gridlineMinor", property_name: Some("GridlineMinor") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:hiLoLine", property_name: Some("HiLoLine") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:leaderLine", property_name: Some("LeaderLine") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:legend", property_name: Some("LegendStyle") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:plotArea", property_name: Some("PlotArea") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:plotArea3D", property_name: Some("PlotArea3D") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:seriesAxis", property_name: Some("SeriesAxis") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:seriesLine", property_name: Some("SeriesLine") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:title", property_name: Some("TitleStyle") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:trendline", property_name: Some("TrendlineStyle") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:trendlineLabel", property_name: Some("TrendlineLabel") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:upBar", property_name: Some("UpBar") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:valueAxis", property_name: Some("ValueAxis") },
    ChildInfo { name: "cs:CT_StyleEntry/cs:wall", property_name: Some("Wall") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_COLOR_STYLE_VARIATION: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_STYLE_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_STYLE_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static ATTRS_LINE_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_LINE_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
    ChildInfo { name: "cs:CT_StyleColor/cs:styleClr", property_name: None },
];
static ATTRS_FILL_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_FILL_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
    ChildInfo { name: "cs:CT_StyleColor/cs:styleClr", property_name: None },
];
static ATTRS_EFFECT_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_EFFECT_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
    ChildInfo { name: "cs:CT_StyleColor/cs:styleClr", property_name: None },
];
static ATTRS_FONT_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "EnumValue" },
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_FONT_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
    ChildInfo { name: "cs:CT_StyleColor/cs:styleClr", property_name: None },
];
static ATTRS_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Transform2D/a:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_CustomGeometry2D/a:custGeom", property_name: None },
    ChildInfo { name: "a:CT_PresetGeometry2D/a:prstGeom", property_name: None },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
    ChildInfo { name: "a:CT_ShapePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_TEXT_CHARACTER_PROPERTIES_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":kumimoji", property_name: Some("Kumimoji"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":altLang", property_name: Some("AlternativeLanguage"), type_name: "StringValue" },
    AttributeInfo { qname: ":sz", property_name: Some("FontSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bold"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":i", property_name: Some("Italic"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":u", property_name: Some("Underline"), type_name: "EnumValue" },
    AttributeInfo { qname: ":strike", property_name: Some("Strike"), type_name: "EnumValue" },
    AttributeInfo { qname: ":kern", property_name: Some("Kerning"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("Capital"), type_name: "EnumValue" },
    AttributeInfo { qname: ":spc", property_name: Some("Spacing"), type_name: "Int32Value" },
    AttributeInfo { qname: ":normalizeH", property_name: Some("NormalizeHeight"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":baseline", property_name: Some("Baseline"), type_name: "Int32Value" },
    AttributeInfo { qname: ":noProof", property_name: Some("NoProof"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dirty", property_name: Some("Dirty"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":err", property_name: Some("SpellingError"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtClean", property_name: Some("SmartTagClean"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtId", property_name: Some("SmartTagId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":bmk", property_name: Some("Bookmark"), type_name: "StringValue" },
];
static CHILDREN_TEXT_CHARACTER_PROPERTIES_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Color/a:highlight", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineLineFollowText/a:uLnTx", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:uLn", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillFollowText/a:uFillTx", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillGroupWrapper/a:uFill", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:latin", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:ea", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:cs", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:sym", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkMouseOver", property_name: None },
    ChildInfo { name: "a:CT_Bool/a:rtl", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_TEXT_BODY_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":spcFirstLastPara", property_name: Some("UseParagraphSpacing"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":vertOverflow", property_name: Some("VerticalOverflow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":horzOverflow", property_name: Some("HorizontalOverflow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":vert", property_name: Some("Vertical"), type_name: "EnumValue" },
    AttributeInfo { qname: ":wrap", property_name: Some("Wrap"), type_name: "EnumValue" },
    AttributeInfo { qname: ":lIns", property_name: Some("LeftInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":tIns", property_name: Some("TopInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rIns", property_name: Some("RightInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":bIns", property_name: Some("BottomInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":numCol", property_name: Some("ColumnCount"), type_name: "Int32Value" },
    AttributeInfo { qname: ":spcCol", property_name: Some("ColumnSpacing"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtlCol", property_name: Some("RightToLeftColumns"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fromWordArt", property_name: Some("FromWordArt"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":anchor", property_name: Some("Anchor"), type_name: "EnumValue" },
    AttributeInfo { qname: ":anchorCtr", property_name: Some("AnchorCenter"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":forceAA", property_name: Some("ForceAntiAlias"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":upright", property_name: Some("UpRight"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":compatLnSpc", property_name: Some("CompatibleLineSpacing"), type_name: "BooleanValue" },
];
static CHILDREN_TEXT_BODY_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PresetTextShape/a:prstTxWarp", property_name: Some("PresetTextWarp") },
    ChildInfo { name: "a:CT_TextNoAutofit/a:noAutofit", property_name: None },
    ChildInfo { name: "a:CT_TextNormalAutofit/a:normAutofit", property_name: None },
    ChildInfo { name: "a:CT_TextShapeAutofit/a:spAutoFit", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
    ChildInfo { name: "a:CT_FlatText/a:flatTx", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_CATEGORY_AXIS_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "EnumValue" },
    AttributeInfo { qname: ":majorTick", property_name: Some("MajorTick"), type_name: "EnumValue" },
    AttributeInfo { qname: ":minorTick", property_name: Some("MinorTickProp"), type_name: "EnumValue" },
    AttributeInfo { qname: ":labelPosition", property_name: Some("LabelPosition"), type_name: "EnumValue" },
    AttributeInfo { qname: ":majorGridlines", property_name: Some("MajorGridlines"), type_name: "EnumValue" },
    AttributeInfo { qname: ":minorGridlines", property_name: Some("MinorGridlinesProp"), type_name: "EnumValue" },
    AttributeInfo { qname: ":title", property_name: Some("TitleProp"), type_name: "EnumValue" },
];
static ATTRS_SERIES_AXIS_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "EnumValue" },
    AttributeInfo { qname: ":majorTick", property_name: Some("MajorTick"), type_name: "EnumValue" },
    AttributeInfo { qname: ":minorTick", property_name: Some("MinorTickProp"), type_name: "EnumValue" },
    AttributeInfo { qname: ":labelPosition", property_name: Some("LabelPosition"), type_name: "EnumValue" },
    AttributeInfo { qname: ":majorGridlines", property_name: Some("MajorGridlines"), type_name: "EnumValue" },
    AttributeInfo { qname: ":minorGridlines", property_name: Some("MinorGridlinesProp"), type_name: "EnumValue" },
    AttributeInfo { qname: ":title", property_name: Some("TitleProp"), type_name: "EnumValue" },
];
static ATTRS_VALUE_AXIS_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "EnumValue" },
    AttributeInfo { qname: ":majorTick", property_name: Some("MajorTick"), type_name: "EnumValue" },
    AttributeInfo { qname: ":minorTick", property_name: Some("MinorTickProp"), type_name: "EnumValue" },
    AttributeInfo { qname: ":labelPosition", property_name: Some("LabelPosition"), type_name: "EnumValue" },
    AttributeInfo { qname: ":majorGridlines", property_name: Some("MajorGridlines"), type_name: "EnumValue" },
    AttributeInfo { qname: ":minorGridlines", property_name: Some("MinorGridlinesProp"), type_name: "EnumValue" },
    AttributeInfo { qname: ":title", property_name: Some("TitleProp"), type_name: "EnumValue" },
];
static ATTRS_DATA_SERIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":overlap", property_name: Some("Overlap"), type_name: "SByteValue" },
    AttributeInfo { qname: ":gapWidth", property_name: Some("GapWidth"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":gapDepth", property_name: Some("GapDepth"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":doughnutHoleSize", property_name: Some("DoughnutHoleSize"), type_name: "ByteValue" },
    AttributeInfo { qname: ":markerVisible", property_name: Some("MarkerVisible"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hiloLines", property_name: Some("HiloLines"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dropLines", property_name: Some("DropLines"), type_name: "EnumValue" },
    AttributeInfo { qname: ":seriesLines", property_name: Some("SeriesLines"), type_name: "EnumValue" },
];
static ATTRS_DATA_LABELS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":position", property_name: Some("Position"), type_name: "EnumValue" },
    AttributeInfo { qname: ":value", property_name: Some("Value"), type_name: "EnumValue" },
    AttributeInfo { qname: ":seriesName", property_name: Some("SeriesName"), type_name: "EnumValue" },
    AttributeInfo { qname: ":categoryName", property_name: Some("CategoryName"), type_name: "EnumValue" },
    AttributeInfo { qname: ":legendKey", property_name: Some("LegendKey"), type_name: "EnumValue" },
    AttributeInfo { qname: ":percentage", property_name: Some("Percentage"), type_name: "EnumValue" },
];
static ATTRS_DATA_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":legendKeys", property_name: Some("LegendKeys"), type_name: "EnumValue" },
    AttributeInfo { qname: ":horizontalBorder", property_name: Some("HorizontalBorder"), type_name: "EnumValue" },
    AttributeInfo { qname: ":verticalBorder", property_name: Some("VerticalBorder"), type_name: "EnumValue" },
    AttributeInfo { qname: ":outlineBorder", property_name: Some("OutlineBorder"), type_name: "EnumValue" },
];
static ATTRS_LEGEND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "EnumValue" },
    AttributeInfo { qname: ":includeInLayout", property_name: Some("IncludeInLayout"), type_name: "EnumValue" },
    AttributeInfo { qname: ":position", property_name: Some("Position"), type_name: "EnumValue" },
];
static ATTRS_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":position", property_name: Some("Position"), type_name: "EnumValue" },
];
static ATTRS_TRENDLINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":add", property_name: Some("Add"), type_name: "EnumValue" },
    AttributeInfo { qname: ":equation", property_name: Some("Equation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rsquared", property_name: Some("RSquared"), type_name: "EnumValue" },
];
static ATTRS_VIEW3_D_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rotX", property_name: Some("RotX"), type_name: "SByteValue" },
    AttributeInfo { qname: ":rotY", property_name: Some("RotY"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":rAngAx", property_name: Some("RightAngleAxes"), type_name: "EnumValue" },
    AttributeInfo { qname: ":perspective", property_name: Some("Perspective"), type_name: "ByteValue" },
    AttributeInfo { qname: ":heightPercent", property_name: Some("HeightPercent"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":depthPercent", property_name: Some("DepthPercent"), type_name: "UInt16Value" },
];
static ATTRS_AXIS_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_AXIS_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_CATEGORY_AXIS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_CATEGORY_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_CHART_AREA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_CHART_AREA: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_LABEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_LABEL_CALLOUT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_LABEL_CALLOUT: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_POINT: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_POINT3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_POINT3_D: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_POINT_LINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_POINT_LINE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_POINT_MARKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_POINT_MARKER: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_POINT_WIREFRAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_POINT_WIREFRAME: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DATA_TABLE_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DATA_TABLE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DOWN_BAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DOWN_BAR: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DROP_LINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_DROP_LINE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_ERROR_BAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_ERROR_BAR: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_FLOOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_FLOOR: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_GRIDLINE_MAJOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_GRIDLINE_MAJOR: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_GRIDLINE_MINOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_GRIDLINE_MINOR: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_HI_LO_LINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_HI_LO_LINE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_LEADER_LINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_LEADER_LINE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_LEGEND_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_LEGEND_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_PLOT_AREA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_PLOT_AREA: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_PLOT_AREA3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_PLOT_AREA3_D: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_SERIES_AXIS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_SERIES_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_SERIES_LINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_SERIES_LINE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_TITLE_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_TITLE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_TRENDLINE_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_TRENDLINE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_TRENDLINE_LABEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_TRENDLINE_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_UP_BAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_UP_BAR: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_VALUE_AXIS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_VALUE_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_WALL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mods", property_name: Some("Modifiers"), type_name: "ListValue" },
];
static CHILDREN_WALL: &[ChildInfo] = &[
    ChildInfo { name: "cs:CT_StyleReference/cs:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "xsd:double/cs:lineWidthScale", property_name: Some("LineWidthScale") },
    ChildInfo { name: "cs:CT_StyleReference/cs:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "cs:CT_StyleReference/cs:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "cs:CT_FontReference/cs:fontRef", property_name: Some("FontReference") },
    ChildInfo { name: "a:CT_ShapeProperties/cs:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextCharacterProperties/cs:defRPr", property_name: Some("TextCharacterPropertiesType") },
    ChildInfo { name: "a:CT_TextBodyProperties/cs:bodyPr", property_name: Some("TextBodyProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cs:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_MARKER_LAYOUT_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":symbol", property_name: Some("Symbol"), type_name: "EnumValue" },
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "ByteValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ColorStyle", local_name: "colorStyle", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_STYLE, children: CHILDREN_COLOR_STYLE },
    ElementInfo { class_name: "ChartStyle", local_name: "chartStyle", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHART_STYLE, children: CHILDREN_CHART_STYLE },
    ElementInfo { class_name: "ColorStyleVariation", local_name: "variation", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_STYLE_VARIATION },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "StyleColor", local_name: "styleClr", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STYLE_COLOR, children: CHILDREN_STYLE_COLOR },
    ElementInfo { class_name: "LineReference", local_name: "lnRef", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_REFERENCE, children: CHILDREN_LINE_REFERENCE },
    ElementInfo { class_name: "FillReference", local_name: "fillRef", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FILL_REFERENCE, children: CHILDREN_FILL_REFERENCE },
    ElementInfo { class_name: "EffectReference", local_name: "effectRef", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EFFECT_REFERENCE, children: CHILDREN_EFFECT_REFERENCE },
    ElementInfo { class_name: "LineWidthScale", local_name: "lineWidthScale", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FontReference", local_name: "fontRef", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FONT_REFERENCE, children: CHILDREN_FONT_REFERENCE },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "TextCharacterPropertiesType", local_name: "defRPr", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_CHARACTER_PROPERTIES_TYPE, children: CHILDREN_TEXT_CHARACTER_PROPERTIES_TYPE },
    ElementInfo { class_name: "TextBodyProperties", local_name: "bodyPr", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_BODY_PROPERTIES, children: CHILDREN_TEXT_BODY_PROPERTIES },
    ElementInfo { class_name: "CategoryAxisProperties", local_name: "categoryAxis", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CATEGORY_AXIS_PROPERTIES, children: &[] },
    ElementInfo { class_name: "SeriesAxisProperties", local_name: "seriesAxis", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SERIES_AXIS_PROPERTIES, children: &[] },
    ElementInfo { class_name: "ValueAxisProperties", local_name: "valueAxis", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VALUE_AXIS_PROPERTIES, children: &[] },
    ElementInfo { class_name: "DataSeries", local_name: "dataSeries", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_SERIES, children: &[] },
    ElementInfo { class_name: "DataLabels", local_name: "dataLabels", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_LABELS, children: &[] },
    ElementInfo { class_name: "DataTable", local_name: "dataTable", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_TABLE, children: &[] },
    ElementInfo { class_name: "Legend", local_name: "legend", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEGEND, children: &[] },
    ElementInfo { class_name: "Title", local_name: "title", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TITLE, children: &[] },
    ElementInfo { class_name: "Trendline", local_name: "trendline", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TRENDLINE, children: &[] },
    ElementInfo { class_name: "View3DProperties", local_name: "view3D", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VIEW3_D_PROPERTIES, children: &[] },
    ElementInfo { class_name: "AxisTitle", local_name: "axisTitle", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AXIS_TITLE, children: CHILDREN_AXIS_TITLE },
    ElementInfo { class_name: "CategoryAxis", local_name: "categoryAxis", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CATEGORY_AXIS, children: CHILDREN_CATEGORY_AXIS },
    ElementInfo { class_name: "ChartArea", local_name: "chartArea", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHART_AREA, children: CHILDREN_CHART_AREA },
    ElementInfo { class_name: "DataLabel", local_name: "dataLabel", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_LABEL, children: CHILDREN_DATA_LABEL },
    ElementInfo { class_name: "DataLabelCallout", local_name: "dataLabelCallout", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_LABEL_CALLOUT, children: CHILDREN_DATA_LABEL_CALLOUT },
    ElementInfo { class_name: "DataPoint", local_name: "dataPoint", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_POINT, children: CHILDREN_DATA_POINT },
    ElementInfo { class_name: "DataPoint3D", local_name: "dataPoint3D", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_POINT3_D, children: CHILDREN_DATA_POINT3_D },
    ElementInfo { class_name: "DataPointLine", local_name: "dataPointLine", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_POINT_LINE, children: CHILDREN_DATA_POINT_LINE },
    ElementInfo { class_name: "DataPointMarker", local_name: "dataPointMarker", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_POINT_MARKER, children: CHILDREN_DATA_POINT_MARKER },
    ElementInfo { class_name: "DataPointWireframe", local_name: "dataPointWireframe", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_POINT_WIREFRAME, children: CHILDREN_DATA_POINT_WIREFRAME },
    ElementInfo { class_name: "DataTableStyle", local_name: "dataTable", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_TABLE_STYLE, children: CHILDREN_DATA_TABLE_STYLE },
    ElementInfo { class_name: "DownBar", local_name: "downBar", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DOWN_BAR, children: CHILDREN_DOWN_BAR },
    ElementInfo { class_name: "DropLine", local_name: "dropLine", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DROP_LINE, children: CHILDREN_DROP_LINE },
    ElementInfo { class_name: "ErrorBar", local_name: "errorBar", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ERROR_BAR, children: CHILDREN_ERROR_BAR },
    ElementInfo { class_name: "Floor", local_name: "floor", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FLOOR, children: CHILDREN_FLOOR },
    ElementInfo { class_name: "GridlineMajor", local_name: "gridlineMajor", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRIDLINE_MAJOR, children: CHILDREN_GRIDLINE_MAJOR },
    ElementInfo { class_name: "GridlineMinor", local_name: "gridlineMinor", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRIDLINE_MINOR, children: CHILDREN_GRIDLINE_MINOR },
    ElementInfo { class_name: "HiLoLine", local_name: "hiLoLine", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HI_LO_LINE, children: CHILDREN_HI_LO_LINE },
    ElementInfo { class_name: "LeaderLine", local_name: "leaderLine", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEADER_LINE, children: CHILDREN_LEADER_LINE },
    ElementInfo { class_name: "LegendStyle", local_name: "legend", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEGEND_STYLE, children: CHILDREN_LEGEND_STYLE },
    ElementInfo { class_name: "PlotArea", local_name: "plotArea", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PLOT_AREA, children: CHILDREN_PLOT_AREA },
    ElementInfo { class_name: "PlotArea3D", local_name: "plotArea3D", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PLOT_AREA3_D, children: CHILDREN_PLOT_AREA3_D },
    ElementInfo { class_name: "SeriesAxis", local_name: "seriesAxis", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SERIES_AXIS, children: CHILDREN_SERIES_AXIS },
    ElementInfo { class_name: "SeriesLine", local_name: "seriesLine", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SERIES_LINE, children: CHILDREN_SERIES_LINE },
    ElementInfo { class_name: "TitleStyle", local_name: "title", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TITLE_STYLE, children: CHILDREN_TITLE_STYLE },
    ElementInfo { class_name: "TrendlineStyle", local_name: "trendline", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRENDLINE_STYLE, children: CHILDREN_TRENDLINE_STYLE },
    ElementInfo { class_name: "TrendlineLabel", local_name: "trendlineLabel", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRENDLINE_LABEL, children: CHILDREN_TRENDLINE_LABEL },
    ElementInfo { class_name: "UpBar", local_name: "upBar", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_UP_BAR, children: CHILDREN_UP_BAR },
    ElementInfo { class_name: "ValueAxis", local_name: "valueAxis", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VALUE_AXIS, children: CHILDREN_VALUE_AXIS },
    ElementInfo { class_name: "Wall", local_name: "wall", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WALL, children: CHILDREN_WALL },
    ElementInfo { class_name: "MarkerLayoutProperties", local_name: "dataPointMarkerLayout", prefix: "cs", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MARKER_LAYOUT_PROPERTIES, children: &[] },
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

/// Create a `<cs:colorStyle>` element (`ColorStyle`).
pub fn color_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "colorStyle").with_children(children)
}

/// Create a `<cs:chartStyle>` element (`ChartStyle`).
pub fn chart_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "chartStyle").with_children(children)
}

/// Create a `<cs:variation>` element (`ColorStyleVariation`).
pub fn color_style_variation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "variation").with_children(children)
}

/// Create a `<cs:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<cs:styleClr>` element (`StyleColor`).
pub fn style_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "styleClr").with_children(children)
}

/// Create a `<cs:lnRef>` element (`LineReference`).
pub fn line_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "lnRef").with_children(children)
}

/// Create a `<cs:fillRef>` element (`FillReference`).
pub fn fill_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "fillRef").with_children(children)
}

/// Create a `<cs:effectRef>` element (`EffectReference`).
pub fn effect_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "effectRef").with_children(children)
}

/// Create a `<cs:lineWidthScale>` element (`LineWidthScale`).
pub fn line_width_scale(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "lineWidthScale").with_text(value)
}

/// Create a `<cs:fontRef>` element (`FontReference`).
pub fn font_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "fontRef").with_children(children)
}

/// Create a `<cs:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<cs:defRPr>` element (`TextCharacterPropertiesType`).
pub fn text_character_properties_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "defRPr").with_children(children)
}

/// Create a `<cs:bodyPr>` element (`TextBodyProperties`).
pub fn text_body_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "bodyPr").with_children(children)
}

/// Create a `<cs:categoryAxis>` element (`CategoryAxisProperties`).
pub fn category_axis_properties() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "categoryAxis")
}

/// Create a `<cs:seriesAxis>` element (`SeriesAxisProperties`).
pub fn series_axis_properties() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "seriesAxis")
}

/// Create a `<cs:valueAxis>` element (`ValueAxisProperties`).
pub fn value_axis_properties() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "valueAxis")
}

/// Create a `<cs:dataSeries>` element (`DataSeries`).
pub fn data_series() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataSeries")
}

/// Create a `<cs:dataLabels>` element (`DataLabels`).
pub fn data_labels() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataLabels")
}

/// Create a `<cs:dataTable>` element (`DataTable`).
pub fn data_table() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataTable")
}

/// Create a `<cs:legend>` element (`Legend`).
pub fn legend() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "legend")
}

/// Create a `<cs:title>` element (`Title`).
pub fn title() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "title")
}

/// Create a `<cs:trendline>` element (`Trendline`).
pub fn trendline() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "trendline")
}

/// Create a `<cs:view3D>` element (`View3DProperties`).
pub fn view3_d_properties() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "view3D")
}

/// Create a `<cs:axisTitle>` element (`AxisTitle`).
pub fn axis_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "axisTitle").with_children(children)
}

/// Create a `<cs:categoryAxis>` element (`CategoryAxis`).
pub fn category_axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "categoryAxis").with_children(children)
}

/// Create a `<cs:chartArea>` element (`ChartArea`).
pub fn chart_area(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "chartArea").with_children(children)
}

/// Create a `<cs:dataLabel>` element (`DataLabel`).
pub fn data_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataLabel").with_children(children)
}

/// Create a `<cs:dataLabelCallout>` element (`DataLabelCallout`).
pub fn data_label_callout(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataLabelCallout").with_children(children)
}

/// Create a `<cs:dataPoint>` element (`DataPoint`).
pub fn data_point(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataPoint").with_children(children)
}

/// Create a `<cs:dataPoint3D>` element (`DataPoint3D`).
pub fn data_point3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataPoint3D").with_children(children)
}

/// Create a `<cs:dataPointLine>` element (`DataPointLine`).
pub fn data_point_line(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataPointLine").with_children(children)
}

/// Create a `<cs:dataPointMarker>` element (`DataPointMarker`).
pub fn data_point_marker(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataPointMarker").with_children(children)
}

/// Create a `<cs:dataPointWireframe>` element (`DataPointWireframe`).
pub fn data_point_wireframe(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataPointWireframe").with_children(children)
}

/// Create a `<cs:dataTable>` element (`DataTableStyle`).
pub fn data_table_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataTable").with_children(children)
}

/// Create a `<cs:downBar>` element (`DownBar`).
pub fn down_bar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "downBar").with_children(children)
}

/// Create a `<cs:dropLine>` element (`DropLine`).
pub fn drop_line(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dropLine").with_children(children)
}

/// Create a `<cs:errorBar>` element (`ErrorBar`).
pub fn error_bar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "errorBar").with_children(children)
}

/// Create a `<cs:floor>` element (`Floor`).
pub fn floor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "floor").with_children(children)
}

/// Create a `<cs:gridlineMajor>` element (`GridlineMajor`).
pub fn gridline_major(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "gridlineMajor").with_children(children)
}

/// Create a `<cs:gridlineMinor>` element (`GridlineMinor`).
pub fn gridline_minor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "gridlineMinor").with_children(children)
}

/// Create a `<cs:hiLoLine>` element (`HiLoLine`).
pub fn hi_lo_line(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "hiLoLine").with_children(children)
}

/// Create a `<cs:leaderLine>` element (`LeaderLine`).
pub fn leader_line(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "leaderLine").with_children(children)
}

/// Create a `<cs:legend>` element (`LegendStyle`).
pub fn legend_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "legend").with_children(children)
}

/// Create a `<cs:plotArea>` element (`PlotArea`).
pub fn plot_area(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "plotArea").with_children(children)
}

/// Create a `<cs:plotArea3D>` element (`PlotArea3D`).
pub fn plot_area3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "plotArea3D").with_children(children)
}

/// Create a `<cs:seriesAxis>` element (`SeriesAxis`).
pub fn series_axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "seriesAxis").with_children(children)
}

/// Create a `<cs:seriesLine>` element (`SeriesLine`).
pub fn series_line(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "seriesLine").with_children(children)
}

/// Create a `<cs:title>` element (`TitleStyle`).
pub fn title_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "title").with_children(children)
}

/// Create a `<cs:trendline>` element (`TrendlineStyle`).
pub fn trendline_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "trendline").with_children(children)
}

/// Create a `<cs:trendlineLabel>` element (`TrendlineLabel`).
pub fn trendline_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "trendlineLabel").with_children(children)
}

/// Create a `<cs:upBar>` element (`UpBar`).
pub fn up_bar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "upBar").with_children(children)
}

/// Create a `<cs:valueAxis>` element (`ValueAxis`).
pub fn value_axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "valueAxis").with_children(children)
}

/// Create a `<cs:wall>` element (`Wall`).
pub fn wall(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "wall").with_children(children)
}

/// Create a `<cs:dataPointMarkerLayout>` element (`MarkerLayoutProperties`).
pub fn marker_layout_properties() -> OpenXmlElement {
    OpenXmlElement::new("cs", NAMESPACE_URI, "dataPointMarkerLayout")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 57;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 54;
