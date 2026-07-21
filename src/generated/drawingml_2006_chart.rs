//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_chart.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/chart` (prefix `c`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "c";

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

static ATTRS_NUMBERING_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":formatCode", property_name: Some("FormatCode"), type_name: "StringValue" },
    AttributeInfo { qname: ":sourceLinked", property_name: Some("SourceLinked"), type_name: "BooleanValue" },
];
static ATTRS_CHART_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_CHART_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Transform2D/a:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_CustomGeometry2D/a:custGeom", property_name: None },
    ChildInfo { name: "a:CT_PresetGeometry2D/a:prstGeom", property_name: None },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_TEXT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
];
static CHILDREN_RICH_TEXT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
];
static ATTRS_DATA_LABEL_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_SHOW_LEGEND_KEY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_CATEGORY_NAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_SERIES_NAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_PERCENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_BUBBLE_SIZE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_LEADER_LINES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_VARY_COLORS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_WIREFRAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DELETE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_OVERLAY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_RIGHT_ANGLE_AXES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_HORIZONTAL_BORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_VERTICAL_BORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_OUTLINE_BORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_KEYS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_INVERT_IF_NEGATIVE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_BUBBLE3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DISPLAY_R_SQUARED_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DISPLAY_EQUATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_NO_END_CAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_APPLY_TO_FRONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_APPLY_TO_SIDES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_APPLY_TO_END: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_CHART_OBJECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_FORMATTING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SELECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_USER_INTERFACE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_AUTO_UPDATE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_MARKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SMOOTH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_NEGATIVE_BUBBLES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_AUTO_LABELED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_NO_MULTI_LEVEL_LABELS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_AUTO_TITLE_DELETED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_PLOT_VISIBLE_ONLY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_DATA_LABELS_OVER_MAXIMUM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DATE1904: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_ROUNDED_CORNERS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static CHILDREN_LAYOUT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ManualLayout/c:manualLayout", property_name: Some("ManualLayout") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_CHART_TEXT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StrRef/c:strRef", property_name: Some("StringReference") },
    ChildInfo { name: "a:CT_TextBody/c:rich", property_name: Some("RichText") },
    ChildInfo { name: "c:CT_StrData/c:strLit", property_name: Some("StringLiteral") },
];
static CHILDREN_LEADER_LINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static CHILDREN_DROP_LINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static CHILDREN_MAJOR_GRIDLINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static CHILDREN_MINOR_GRIDLINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static CHILDREN_SERIES_LINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static CHILDREN_HIGH_LOW_LINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static ATTRS_INDEX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_ORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_AXIS_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_CROSSING_AXIS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_POINT_COUNT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_SECOND_PIE_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_EXPLOSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_FORMAT_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static CHILDREN_SERIES_TEXT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StrRef/c:strRef", property_name: Some("StringReference") },
    ChildInfo { name: "c:ST_Xstring/c:v", property_name: Some("NumericValue") },
];
static ATTRS_GROUPING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_LINE_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_Marker/c:marker", property_name: Some("Marker") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_DPt/c:dPt", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_Trendline/c:trendline", property_name: None },
    ChildInfo { name: "c:CT_ErrBars/c:errBars", property_name: None },
    ChildInfo { name: "c:CT_AxDataSource/c:cat", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:val", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:smooth", property_name: None },
    ChildInfo { name: "c:CT_LineSerExtensionList/c:extLst", property_name: None },
];
static CHILDREN_DATA_LABELS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_DLbl/c:dLbl", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:delete", property_name: None },
    ChildInfo { name: "c:CT_NumFmt/c:numFmt", property_name: None },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: None },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: None },
    ChildInfo { name: "c:CT_DLblPos/c:dLblPos", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showLegendKey", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showVal", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showCatName", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showSerName", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showPercent", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showBubbleSize", property_name: None },
    ChildInfo { name: "xsd:string/c:separator", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showLeaderLines", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:leaderLines", property_name: None },
    ChildInfo { name: "c:CT_DLblsExtensionList/c:extLst", property_name: None },
];
static ATTRS_BAR_DIRECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_BAR_GROUPING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_BAR_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_Boolean/c:invertIfNegative", property_name: Some("InvertIfNegative") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_DPt/c:dPt", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_Trendline/c:trendline", property_name: None },
    ChildInfo { name: "c:CT_ErrBars/c:errBars", property_name: None },
    ChildInfo { name: "c:CT_AxDataSource/c:cat", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:val", property_name: None },
    ChildInfo { name: "c:CT_Shape/c:shape", property_name: None },
    ChildInfo { name: "c:CT_BarSerExtensionList/c:extLst", property_name: None },
];
static CHILDREN_AREA_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_DPt/c:dPt", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_Trendline/c:trendline", property_name: None },
    ChildInfo { name: "c:CT_ErrBars/c:errBars", property_name: None },
    ChildInfo { name: "c:CT_AxDataSource/c:cat", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:val", property_name: None },
    ChildInfo { name: "c:CT_AreaSerExtensionList/c:extLst", property_name: None },
];
static CHILDREN_PIE_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_UnsignedInt/c:explosion", property_name: Some("Explosion") },
    ChildInfo { name: "c:CT_DPt/c:dPt", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_AxDataSource/c:cat", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:val", property_name: None },
    ChildInfo { name: "c:CT_PieSerExtensionList/c:extLst", property_name: None },
];
static CHILDREN_SURFACE_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_AxDataSource/c:cat", property_name: Some("CategoryAxisData") },
    ChildInfo { name: "c:CT_NumDataSource/c:val", property_name: Some("Values") },
    ChildInfo { name: "c:CT_Boolean/c:bubble3D", property_name: Some("Bubble3D") },
    ChildInfo { name: "c:CT_SurfaceSerExtensionList/c:extLst", property_name: Some("SurfaceSerExtensionList") },
];
static CHILDREN_BAND_FORMATS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BandFmt/c:bandFmt", property_name: None },
];
static CHILDREN_SCALING: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_LogBase/c:logBase", property_name: Some("LogBase") },
    ChildInfo { name: "c:CT_Orientation/c:orientation", property_name: Some("Orientation") },
    ChildInfo { name: "c:CT_Double/c:max", property_name: Some("MaxAxisValue") },
    ChildInfo { name: "c:CT_Double/c:min", property_name: Some("MinAxisValue") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_AXIS_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Tx/c:tx", property_name: Some("ChartText") },
    ChildInfo { name: "c:CT_Layout/c:layout", property_name: Some("Layout") },
    ChildInfo { name: "c:CT_Boolean/c:overlay", property_name: Some("Overlay") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_MAJOR_TICK_MARK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_MINOR_TICK_MARK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_TICK_LABEL_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_CROSSES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_CROSSES_AT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_LEFT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_TOP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_WIDTH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_HEIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_FORWARD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_BACKWARD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_INTERCEPT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_ERROR_BAR_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_SPLIT_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_CUSTOM_DISPLAY_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_MAX_AXIS_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_MIN_AXIS_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_CHART_SPACE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":version", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":featureList", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":fallbackImg", property_name: None, type_name: "StringValue" },
];
static CHILDREN_CHART_SPACE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:date1904", property_name: Some("Date1904") },
    ChildInfo { name: "c:CT_TextLanguageID/c:lang", property_name: Some("EditingLanguage") },
    ChildInfo { name: "c:CT_Boolean/c:roundedCorners", property_name: Some("RoundedCorners") },
    ChildInfo { name: "c14:CT_Style/c14:style", property_name: None },
    ChildInfo { name: "c:CT_Style/c:style", property_name: None },
    ChildInfo { name: "a:CT_ColorMapping/c:clrMapOvr", property_name: None },
    ChildInfo { name: "c:CT_PivotSource/c:pivotSource", property_name: None },
    ChildInfo { name: "c:CT_Protection/c:protection", property_name: None },
    ChildInfo { name: "c:CT_Chart/c:chart", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/c:spPr", property_name: None },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: None },
    ChildInfo { name: "c:CT_ExternalData/c:externalData", property_name: None },
    ChildInfo { name: "c:CT_PrintSettings/c:printSettings", property_name: None },
    ChildInfo { name: "c:CT_RelId/c:userShapes", property_name: None },
    ChildInfo { name: "c:CT_ChartSpaceExtensionList/c:extLst", property_name: None },
];
static CHILDREN_USER_SHAPES: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_RelSizeAnchor/cdr:relSizeAnchor", property_name: None },
    ChildInfo { name: "cdr:CT_AbsSizeAnchor/cdr:absSizeAnchor", property_name: None },
];
static ATTRS_CHART_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_LEGACY_DRAWING_HEADER_FOOTER: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_USER_SHAPES_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: Some("Uri"), type_name: "StringValue" },
];
static ATTRS_NUMERIC_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":formatCode", property_name: Some("FormatCode"), type_name: "StringValue" },
];
static CHILDREN_NUMERIC_POINT: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:v", property_name: Some("NumericValue") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Extension/c:ext", property_name: None },
];
static CHILDREN_NUMBER_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c:f", property_name: Some("Formula") },
    ChildInfo { name: "c:CT_NumData/c:numCache", property_name: Some("NumberingCache") },
    ChildInfo { name: "c:CT_NumRefExtensionList/c:extLst", property_name: Some("NumRefExtensionList") },
];
static CHILDREN_NUMBER_LITERAL: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:formatCode", property_name: Some("FormatCode") },
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_NumVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_NUMBERING_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:formatCode", property_name: Some("FormatCode") },
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_NumVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_LEVEL: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StrVal/c:pt", property_name: None },
];
static CHILDREN_MULTI_LEVEL_STRING_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c:f", property_name: Some("Formula") },
    ChildInfo { name: "c:CT_MultiLvlStrData/c:multiLvlStrCache", property_name: Some("MultiLevelStringCache") },
    ChildInfo { name: "c:CT_MultiLvlStrRefExtensionList/c:extLst", property_name: Some("MultiLvlStrRefExtensionList") },
];
static CHILDREN_STRING_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c:f", property_name: Some("Formula") },
    ChildInfo { name: "c:CT_StrData/c:strCache", property_name: Some("StringCache") },
    ChildInfo { name: "c:CT_StrRefExtensionList/c:extLst", property_name: Some("StrRefExtensionList") },
];
static CHILDREN_STRING_LITERAL: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_StrVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_StrDataExtensionList/c:extLst", property_name: None },
];
static CHILDREN_STRING_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_StrVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_StrDataExtensionList/c:extLst", property_name: None },
];
static ATTRS_LAYOUT_TARGET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_LEFT_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_TOP_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_WIDTH_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HEIGHT_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_MANUAL_LAYOUT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_LayoutTarget/c:layoutTarget", property_name: Some("LayoutTarget") },
    ChildInfo { name: "c:CT_LayoutMode/c:xMode", property_name: Some("LeftMode") },
    ChildInfo { name: "c:CT_LayoutMode/c:yMode", property_name: Some("TopMode") },
    ChildInfo { name: "c:CT_LayoutMode/c:wMode", property_name: Some("WidthMode") },
    ChildInfo { name: "c:CT_LayoutMode/c:hMode", property_name: Some("HeightMode") },
    ChildInfo { name: "c:CT_Double/c:x", property_name: Some("Left") },
    ChildInfo { name: "c:CT_Double/c:y", property_name: Some("Top") },
    ChildInfo { name: "c:CT_Double/c:w", property_name: Some("Width") },
    ChildInfo { name: "c:CT_Double/c:h", property_name: Some("Height") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_ROTATE_X: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "SByteValue" },
];
static ATTRS_HEIGHT_PERCENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static ATTRS_ROTATE_Y: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static ATTRS_DEPTH_PERCENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static ATTRS_PERSPECTIVE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "ByteValue" },
];
static ATTRS_SYMBOL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_SIZE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "ByteValue" },
];
static CHILDREN_MARKER: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MarkerStyle/c:symbol", property_name: Some("Symbol") },
    ChildInfo { name: "c:CT_MarkerSize/c:size", property_name: Some("Size") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_PICTURE_OPTIONS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:applyToFront", property_name: Some("ApplyToFront") },
    ChildInfo { name: "c:CT_Boolean/c:applyToSides", property_name: Some("ApplyToSides") },
    ChildInfo { name: "c:CT_Boolean/c:applyToEnd", property_name: Some("ApplyToEnd") },
    ChildInfo { name: "c:CT_PictureFormat/c:pictureFormat", property_name: Some("PictureFormat") },
    ChildInfo { name: "c:CT_PictureStackUnit/c:pictureStackUnit", property_name: Some("PictureStackUnit") },
];
static ATTRS_TRENDLINE_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_POLYNOMIAL_ORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "ByteValue" },
];
static ATTRS_PERIOD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static CHILDREN_TRENDLINE_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Layout/c:layout", property_name: Some("Layout") },
    ChildInfo { name: "c:CT_Tx/c:tx", property_name: Some("ChartText") },
    ChildInfo { name: "c:CT_NumFmt/c:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_ERROR_DIRECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ERROR_BAR_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ERROR_BAR_VALUE_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_PLUS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
];
static CHILDREN_MINUS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
];
static CHILDREN_VALUES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
];
static CHILDREN_Y_VALUES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
];
static CHILDREN_BUBBLE_SIZE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
];
static ATTRS_GAP_WIDTH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static ATTRS_GAP_DEPTH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static CHILDREN_UP_BARS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static CHILDREN_DOWN_BARS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static ATTRS_OF_PIE_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_SPLIT_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_CUSTOM_SPLIT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:secondPiePt", property_name: None },
];
static ATTRS_SECOND_PIE_SIZE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static CHILDREN_BAND_FORMAT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static ATTRS_PICTURE_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_PICTURE_STACK_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_BUILT_IN_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_DISPLAY_UNITS_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Layout/c:layout", property_name: Some("Layout") },
    ChildInfo { name: "c:CT_Tx/c:tx", property_name: Some("ChartText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
];
static ATTRS_LOG_BASE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_ORIENTATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_PIVOT_FORMAT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "a:CT_ShapeProperties/c:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_Marker/c:marker", property_name: Some("Marker") },
    ChildInfo { name: "c:CT_DLbl/c:dLbl", property_name: Some("DataLabel") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_LEGEND_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_LEGEND_ENTRY: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_Boolean/c:delete", property_name: None },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static ATTRS_HEADER_FOOTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":alignWithMargins", property_name: Some("AlignWithMargins"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":differentOddEven", property_name: Some("DifferentOddEven"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":differentFirst", property_name: Some("DifferentFirst"), type_name: "BooleanValue" },
];
static CHILDREN_HEADER_FOOTER: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:oddHeader", property_name: Some("OddHeader") },
    ChildInfo { name: "c:ST_Xstring/c:oddFooter", property_name: Some("OddFooter") },
    ChildInfo { name: "c:ST_Xstring/c:evenHeader", property_name: Some("EvenHeader") },
    ChildInfo { name: "c:ST_Xstring/c:evenFooter", property_name: Some("EvenFooter") },
    ChildInfo { name: "c:ST_Xstring/c:firstHeader", property_name: Some("FirstHeader") },
    ChildInfo { name: "c:ST_Xstring/c:firstFooter", property_name: Some("FirstFooter") },
];
static ATTRS_PAGE_MARGINS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: Some("Left"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":r", property_name: Some("Right"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":t", property_name: Some("Top"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":b", property_name: Some("Bottom"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":header", property_name: Some("Header"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":footer", property_name: Some("Footer"), type_name: "DoubleValue" },
];
static ATTRS_PAGE_SETUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":paperSize", property_name: Some("PaperSize"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":firstPageNumber", property_name: Some("FirstPageNumber"), type_name: "Int32Value" },
    AttributeInfo { qname: ":orientation", property_name: Some("Orientation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":blackAndWhite", property_name: Some("BlackAndWhite"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":draft", property_name: Some("Draft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":useFirstPageNumber", property_name: Some("UseFirstPageNumber"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":horizontalDpi", property_name: Some("HorizontalDpi"), type_name: "Int32Value" },
    AttributeInfo { qname: ":verticalDpi", property_name: Some("VerticalDpi"), type_name: "Int32Value" },
    AttributeInfo { qname: ":copies", property_name: Some("Copies"), type_name: "UInt32Value" },
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
static CHILDREN_DATA_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_Boolean/c:delete", property_name: None },
    ChildInfo { name: "c:CT_Layout/c:layout", property_name: None },
    ChildInfo { name: "c:CT_Tx/c:tx", property_name: None },
    ChildInfo { name: "c:CT_NumFmt/c:numFmt", property_name: None },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: None },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: None },
    ChildInfo { name: "c:CT_DLblPos/c:dLblPos", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showLegendKey", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showVal", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showCatName", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showSerName", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showPercent", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showBubbleSize", property_name: None },
    ChildInfo { name: "xsd:string/c:separator", property_name: None },
    ChildInfo { name: "c:CT_DLblExtensionList/c:extLst", property_name: None },
];
static CHILDREN_AREA_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Grouping/c:grouping", property_name: Some("Grouping") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_AreaSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:dropLines", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_AreaChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_AREA3_D_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Grouping/c:grouping", property_name: Some("Grouping") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_AreaSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:dropLines", property_name: None },
    ChildInfo { name: "c:CT_GapAmount/c:gapDepth", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_Area3DChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_LINE_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Grouping/c:grouping", property_name: Some("Grouping") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_LineSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:dropLines", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:hiLowLines", property_name: None },
    ChildInfo { name: "c:CT_UpDownBars/c:upDownBars", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:marker", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:smooth", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_LineChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_LINE3_D_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Grouping/c:grouping", property_name: Some("Grouping") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_LineSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:dropLines", property_name: None },
    ChildInfo { name: "c:CT_GapAmount/c:gapDepth", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_Line3DChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_STOCK_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_LineSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:dropLines", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:hiLowLines", property_name: None },
    ChildInfo { name: "c:CT_UpDownBars/c:upDownBars", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_StockChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_RADAR_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_RadarStyle/c:radarStyle", property_name: Some("RadarStyle") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_RadarSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_RadarChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_SCATTER_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ScatterStyle/c:scatterStyle", property_name: Some("ScatterStyle") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_ScatterSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_ScatterChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_PIE_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_PieSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_FirstSliceAng/c:firstSliceAng", property_name: None },
    ChildInfo { name: "c:CT_PieChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_PIE3_D_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_PieSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_Pie3DChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_DOUGHNUT_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_PieSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_FirstSliceAng/c:firstSliceAng", property_name: None },
    ChildInfo { name: "c:CT_HoleSize/c:holeSize", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_BAR_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BarDir/c:barDir", property_name: Some("BarDirection") },
    ChildInfo { name: "c:CT_BarGrouping/c:grouping", property_name: Some("BarGrouping") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_BarSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_GapAmount/c:gapWidth", property_name: None },
    ChildInfo { name: "c:CT_Overlap/c:overlap", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:serLines", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_BarChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_BAR3_D_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BarDir/c:barDir", property_name: Some("BarDirection") },
    ChildInfo { name: "c:CT_BarGrouping/c:grouping", property_name: Some("BarGrouping") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_BarSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_GapAmount/c:gapWidth", property_name: None },
    ChildInfo { name: "c:CT_GapAmount/c:gapDepth", property_name: None },
    ChildInfo { name: "c:CT_Shape/c:shape", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_Bar3DChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_OF_PIE_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_OfPieType/c:ofPieType", property_name: Some("OfPieType") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_PieSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_GapAmount/c:gapWidth", property_name: None },
    ChildInfo { name: "c:CT_SplitType/c:splitType", property_name: None },
    ChildInfo { name: "c:CT_Double/c:splitPos", property_name: None },
    ChildInfo { name: "c:CT_CustSplit/c:custSplit", property_name: None },
    ChildInfo { name: "c:CT_SecondPieSize/c:secondPieSize", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c:serLines", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_SURFACE_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:wireframe", property_name: Some("Wireframe") },
    ChildInfo { name: "c:CT_SurfaceSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_BandFmts/c:bandFmts", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_SurfaceChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_SURFACE3_D_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:wireframe", property_name: Some("Wireframe") },
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_SurfaceSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_BandFmts/c:bandFmts", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_Surface3DChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_BUBBLE_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:varyColors", property_name: Some("VaryColors") },
    ChildInfo { name: "c:CT_BubbleSer/c:ser", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:bubble3D", property_name: None },
    ChildInfo { name: "c:CT_BubbleScale/c:bubbleScale", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:showNegBubbles", property_name: None },
    ChildInfo { name: "c:CT_SizeRepresents/c:sizeRepresents", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: None },
    ChildInfo { name: "c:CT_BubbleChartExtensionList/c:extLst", property_name: None },
];
static CHILDREN_VALUE_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: Some("AxisId") },
    ChildInfo { name: "c:CT_Scaling/c:scaling", property_name: Some("Scaling") },
    ChildInfo { name: "c:CT_Boolean/c:delete", property_name: Some("Delete") },
    ChildInfo { name: "c:CT_AxPos/c:axPos", property_name: Some("AxisPosition") },
    ChildInfo { name: "c:CT_ChartLines/c:majorGridlines", property_name: Some("MajorGridlines") },
    ChildInfo { name: "c:CT_ChartLines/c:minorGridlines", property_name: Some("MinorGridlines") },
    ChildInfo { name: "c:CT_Title/c:title", property_name: Some("Title") },
    ChildInfo { name: "c:CT_NumFmt/c:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "c:CT_TickMark/c:majorTickMark", property_name: Some("MajorTickMark") },
    ChildInfo { name: "c:CT_TickMark/c:minorTickMark", property_name: Some("MinorTickMark") },
    ChildInfo { name: "c:CT_TickLblPos/c:tickLblPos", property_name: Some("TickLabelPosition") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "c:CT_UnsignedInt/c:crossAx", property_name: Some("CrossingAxis") },
    ChildInfo { name: "c:CT_Crosses/c:crosses", property_name: None },
    ChildInfo { name: "c:CT_Double/c:crossesAt", property_name: None },
    ChildInfo { name: "c:CT_CrossBetween/c:crossBetween", property_name: None },
    ChildInfo { name: "c:CT_AxisUnit/c:majorUnit", property_name: None },
    ChildInfo { name: "c:CT_AxisUnit/c:minorUnit", property_name: None },
    ChildInfo { name: "c:CT_DispUnits/c:dispUnits", property_name: None },
    ChildInfo { name: "c:CT_ValAxExtensionList/c:extLst", property_name: None },
];
static CHILDREN_CATEGORY_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: Some("AxisId") },
    ChildInfo { name: "c:CT_Scaling/c:scaling", property_name: Some("Scaling") },
    ChildInfo { name: "c:CT_Boolean/c:delete", property_name: Some("Delete") },
    ChildInfo { name: "c:CT_AxPos/c:axPos", property_name: Some("AxisPosition") },
    ChildInfo { name: "c:CT_ChartLines/c:majorGridlines", property_name: Some("MajorGridlines") },
    ChildInfo { name: "c:CT_ChartLines/c:minorGridlines", property_name: Some("MinorGridlines") },
    ChildInfo { name: "c:CT_Title/c:title", property_name: Some("Title") },
    ChildInfo { name: "c:CT_NumFmt/c:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "c:CT_TickMark/c:majorTickMark", property_name: Some("MajorTickMark") },
    ChildInfo { name: "c:CT_TickMark/c:minorTickMark", property_name: Some("MinorTickMark") },
    ChildInfo { name: "c:CT_TickLblPos/c:tickLblPos", property_name: Some("TickLabelPosition") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "c:CT_UnsignedInt/c:crossAx", property_name: Some("CrossingAxis") },
    ChildInfo { name: "c:CT_Crosses/c:crosses", property_name: None },
    ChildInfo { name: "c:CT_Double/c:crossesAt", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:auto", property_name: None },
    ChildInfo { name: "c:CT_LblAlgn/c:lblAlgn", property_name: None },
    ChildInfo { name: "c:CT_LblOffset/c:lblOffset", property_name: None },
    ChildInfo { name: "c:CT_Skip/c:tickLblSkip", property_name: None },
    ChildInfo { name: "c:CT_Skip/c:tickMarkSkip", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:noMultiLvlLbl", property_name: None },
    ChildInfo { name: "c:CT_CatAxExtensionList/c:extLst", property_name: None },
];
static CHILDREN_DATE_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: Some("AxisId") },
    ChildInfo { name: "c:CT_Scaling/c:scaling", property_name: Some("Scaling") },
    ChildInfo { name: "c:CT_Boolean/c:delete", property_name: Some("Delete") },
    ChildInfo { name: "c:CT_AxPos/c:axPos", property_name: Some("AxisPosition") },
    ChildInfo { name: "c:CT_ChartLines/c:majorGridlines", property_name: Some("MajorGridlines") },
    ChildInfo { name: "c:CT_ChartLines/c:minorGridlines", property_name: Some("MinorGridlines") },
    ChildInfo { name: "c:CT_Title/c:title", property_name: Some("Title") },
    ChildInfo { name: "c:CT_NumFmt/c:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "c:CT_TickMark/c:majorTickMark", property_name: Some("MajorTickMark") },
    ChildInfo { name: "c:CT_TickMark/c:minorTickMark", property_name: Some("MinorTickMark") },
    ChildInfo { name: "c:CT_TickLblPos/c:tickLblPos", property_name: Some("TickLabelPosition") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "c:CT_UnsignedInt/c:crossAx", property_name: Some("CrossingAxis") },
    ChildInfo { name: "c:CT_Crosses/c:crosses", property_name: None },
    ChildInfo { name: "c:CT_Double/c:crossesAt", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:auto", property_name: None },
    ChildInfo { name: "c:CT_LblOffset/c:lblOffset", property_name: None },
    ChildInfo { name: "c:CT_TimeUnit/c:baseTimeUnit", property_name: None },
    ChildInfo { name: "c:CT_AxisUnit/c:majorUnit", property_name: None },
    ChildInfo { name: "c:CT_TimeUnit/c:majorTimeUnit", property_name: None },
    ChildInfo { name: "c:CT_AxisUnit/c:minorUnit", property_name: None },
    ChildInfo { name: "c:CT_TimeUnit/c:minorTimeUnit", property_name: None },
    ChildInfo { name: "c:CT_DateAxExtensionList/c:extLst", property_name: None },
];
static CHILDREN_SERIES_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:axId", property_name: Some("AxisId") },
    ChildInfo { name: "c:CT_Scaling/c:scaling", property_name: Some("Scaling") },
    ChildInfo { name: "c:CT_Boolean/c:delete", property_name: Some("Delete") },
    ChildInfo { name: "c:CT_AxPos/c:axPos", property_name: Some("AxisPosition") },
    ChildInfo { name: "c:CT_ChartLines/c:majorGridlines", property_name: Some("MajorGridlines") },
    ChildInfo { name: "c:CT_ChartLines/c:minorGridlines", property_name: Some("MinorGridlines") },
    ChildInfo { name: "c:CT_Title/c:title", property_name: Some("Title") },
    ChildInfo { name: "c:CT_NumFmt/c:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "c:CT_TickMark/c:majorTickMark", property_name: Some("MajorTickMark") },
    ChildInfo { name: "c:CT_TickMark/c:minorTickMark", property_name: Some("MinorTickMark") },
    ChildInfo { name: "c:CT_TickLblPos/c:tickLblPos", property_name: Some("TickLabelPosition") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "c:CT_UnsignedInt/c:crossAx", property_name: Some("CrossingAxis") },
    ChildInfo { name: "c:CT_Crosses/c:crosses", property_name: None },
    ChildInfo { name: "c:CT_Double/c:crossesAt", property_name: None },
    ChildInfo { name: "c:CT_Skip/c:tickLblSkip", property_name: None },
    ChildInfo { name: "c:CT_Skip/c:tickMarkSkip", property_name: None },
    ChildInfo { name: "c:CT_SerAxExtensionList/c:extLst", property_name: None },
];
static CHILDREN_DATA_TABLE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:showHorzBorder", property_name: Some("ShowHorizontalBorder") },
    ChildInfo { name: "c:CT_Boolean/c:showVertBorder", property_name: Some("ShowVerticalBorder") },
    ChildInfo { name: "c:CT_Boolean/c:showOutline", property_name: Some("ShowOutlineBorder") },
    ChildInfo { name: "c:CT_Boolean/c:showKeys", property_name: Some("ShowKeys") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FIRST_SLICE_ANGLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static ATTRS_HOLE_SIZE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "ByteValue" },
];
static ATTRS_STRING_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_STRING_POINT: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:v", property_name: Some("NumericValue") },
];
static ATTRS_THICKNESS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "ByteValue" },
];
static ATTRS_STOCK_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_STOCK_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredLineSer/c15:filteredLineSeries", property_name: Some("FilteredLineSeriesExtension") },
];
static ATTRS_PIE_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PIE_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredPieSer/c15:filteredPieSeries", property_name: Some("FilteredPieSeries") },
];
static ATTRS_PIE3_D_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PIE3_D_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredPieSer/c15:filteredPieSeries", property_name: Some("FilteredPieSeries") },
];
static ATTRS_NUM_REF_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_NUM_REF_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FullRef/c15:fullRef", property_name: Some("FullReference") },
    ChildInfo { name: "c15:CT_LevelRef/c15:levelRef", property_name: Some("LevelReference") },
    ChildInfo { name: "c15:CT_FormulaRef/c15:formulaRef", property_name: Some("FormulaReference") },
];
static ATTRS_STR_DATA_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_STR_DATA_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c15:autoCat", property_name: Some("AutoGeneneratedCategories") },
];
static ATTRS_STR_REF_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_STR_REF_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FullRef/c15:fullRef", property_name: Some("FullReference") },
    ChildInfo { name: "c15:CT_LevelRef/c15:levelRef", property_name: Some("LevelReference") },
    ChildInfo { name: "c15:CT_FormulaRef/c15:formulaRef", property_name: Some("FormulaReference") },
];
static ATTRS_MULTI_LVL_STR_REF_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_MULTI_LVL_STR_REF_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FullRef/c15:fullRef", property_name: Some("FullReference") },
    ChildInfo { name: "c15:CT_LevelRef/c15:levelRef", property_name: Some("LevelReference") },
    ChildInfo { name: "c15:CT_FormulaRef/c15:formulaRef", property_name: Some("FormulaReference") },
];
static ATTRS_D_LBLS_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_D_LBLS_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Tx/c15:tx", property_name: Some("ChartText") },
    ChildInfo { name: "c15:CT_DataLabelFieldTable/c15:dlblFieldTable", property_name: Some("DataLabelFieldTable") },
    ChildInfo { name: "c:CT_Boolean/c15:showDataLabelsRange", property_name: Some("ShowDataLabelsRange") },
    ChildInfo { name: "a:CT_ShapeProperties/c15:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_Layout/c15:layout", property_name: Some("Layout") },
    ChildInfo { name: "c:CT_Boolean/c15:showLeaderLines", property_name: Some("ShowLeaderLines") },
    ChildInfo { name: "c:CT_ChartLines/c15:leaderLines", property_name: Some("LeaderLines") },
];
static ATTRS_LINE_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_LINE_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredLineSer/c15:filteredLineSeries", property_name: Some("FilteredLineSeriesExtension") },
];
static ATTRS_LINE3_D_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_LINE3_D_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredLineSer/c15:filteredLineSeries", property_name: Some("FilteredLineSeriesExtension") },
];
static ATTRS_SCATTER_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SCATTER_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredScatterSer/c15:filteredScatterSeries", property_name: Some("FilteredScatterSeries") },
];
static ATTRS_RADAR_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_RADAR_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredRadarSer/c15:filteredRadarSeries", property_name: Some("FilteredRadarSeries") },
];
static ATTRS_BAR_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_BAR_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredBarSer/c15:filteredBarSeries", property_name: Some("FilteredBarSeries") },
];
static ATTRS_BAR3_D_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_BAR3_D_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredBarSer/c15:filteredBarSeries", property_name: Some("FilteredBarSeries") },
];
static ATTRS_AREA_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_AREA_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredAreaSer/c15:filteredAreaSeries", property_name: Some("FilteredAreaSeries") },
];
static ATTRS_AREA3_D_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_AREA3_D_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredAreaSer/c15:filteredAreaSeries", property_name: Some("FilteredAreaSeries") },
];
static ATTRS_BUBBLE_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_BUBBLE_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredBubbleSer/c15:filteredBubbleSeries", property_name: Some("FilteredBubbleSeries") },
];
static ATTRS_SURFACE_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SURFACE_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries", property_name: Some("FilteredSurfaceSeries") },
];
static ATTRS_SURFACE3_D_CHART_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SURFACE3_D_CHART_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries", property_name: Some("FilteredSurfaceSeries") },
];
static ATTRS_CAT_AX_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_CAT_AX_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumFmt/c15:numFmt", property_name: Some("NumberingFormat") },
];
static ATTRS_DATE_AX_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_DATE_AX_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumFmt/c15:numFmt", property_name: Some("NumberingFormat") },
];
static ATTRS_SER_AX_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SER_AX_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumFmt/c15:numFmt", property_name: Some("NumberingFormat") },
];
static ATTRS_VAL_AX_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_VAL_AX_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumFmt/c15:numFmt", property_name: Some("NumberingFormat") },
];
static CHILDREN_UP_DOWN_BARS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_GapAmount/c:gapWidth", property_name: Some("GapWidth") },
    ChildInfo { name: "c:CT_UpDownBar/c:upBars", property_name: Some("UpBars") },
    ChildInfo { name: "c:CT_UpDownBar/c:downBars", property_name: Some("DownBars") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_STOCK_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StockChartExtension/c:ext", property_name: None },
];
static CHILDREN_PIE_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_PieChartExtension/c:ext", property_name: None },
];
static CHILDREN_PIE3_D_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Pie3DChartExtension/c:ext", property_name: None },
];
static CHILDREN_NUM_REF_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumRefExtension/c:ext", property_name: None },
];
static CHILDREN_STR_DATA_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StrDataExtension/c:ext", property_name: None },
];
static CHILDREN_STR_REF_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StrRefExtension/c:ext", property_name: None },
];
static CHILDREN_MULTI_LEVEL_STRING_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_Lvl/c:lvl", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_MULTI_LVL_STR_REF_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MultiLvlStrRefExtension/c:ext", property_name: None },
];
static CHILDREN_D_LBLS_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_DLblsExtension/c:ext", property_name: None },
];
static CHILDREN_LINE_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_LineChartExtension/c:ext", property_name: None },
];
static CHILDREN_LINE3_D_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Line3DChartExtension/c:ext", property_name: None },
];
static ATTRS_SCATTER_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_SCATTER_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_Marker/c:marker", property_name: Some("Marker") },
    ChildInfo { name: "c:CT_DPt/c:dPt", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_Trendline/c:trendline", property_name: None },
    ChildInfo { name: "c:CT_ErrBars/c:errBars", property_name: None },
    ChildInfo { name: "c:CT_AxDataSource/c:xVal", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:yVal", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:smooth", property_name: None },
    ChildInfo { name: "c:CT_ScatterSerExtensionList/c:extLst", property_name: None },
];
static CHILDREN_SCATTER_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ScatterChartExtension/c:ext", property_name: None },
];
static ATTRS_RADAR_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_RADAR_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_Marker/c:marker", property_name: Some("Marker") },
    ChildInfo { name: "c:CT_DPt/c:dPt", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_AxDataSource/c:cat", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:val", property_name: None },
    ChildInfo { name: "c:CT_RadarSerExtensionList/c:extLst", property_name: None },
];
static CHILDREN_RADAR_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_RadarChartExtension/c:ext", property_name: None },
];
static ATTRS_OVERLAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "SByteValue" },
];
static CHILDREN_BAR_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BarChartExtension/c:ext", property_name: None },
];
static ATTRS_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_BAR3_D_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Bar3DChartExtension/c:ext", property_name: None },
];
static CHILDREN_AREA_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_AreaChartExtension/c:ext", property_name: None },
];
static CHILDREN_AREA3_D_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Area3DChartExtension/c:ext", property_name: None },
];
static CHILDREN_BUBBLE_CHART_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_UnsignedInt/c:order", property_name: Some("Order") },
    ChildInfo { name: "c:CT_SerTx/c:tx", property_name: Some("SeriesText") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_Boolean/c:invertIfNegative", property_name: Some("InvertIfNegative") },
    ChildInfo { name: "c:CT_DPt/c:dPt", property_name: None },
    ChildInfo { name: "c:CT_DLbls/c:dLbls", property_name: None },
    ChildInfo { name: "c:CT_Trendline/c:trendline", property_name: None },
    ChildInfo { name: "c:CT_ErrBars/c:errBars", property_name: None },
    ChildInfo { name: "c:CT_AxDataSource/c:xVal", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:yVal", property_name: None },
    ChildInfo { name: "c:CT_NumDataSource/c:bubbleSize", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:bubble3D", property_name: None },
    ChildInfo { name: "c:CT_BubbleSerExtensionList/c:extLst", property_name: None },
];
static ATTRS_BUBBLE_SCALE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_SIZE_REPRESENTS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_BUBBLE_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BubbleChartExtension/c:ext", property_name: None },
];
static CHILDREN_SURFACE_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_SurfaceChartExtension/c:ext", property_name: None },
];
static CHILDREN_SURFACE3_D_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Surface3DChartExtension/c:ext", property_name: None },
];
static ATTRS_LABEL_ALIGNMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_LABEL_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt16Value" },
];
static ATTRS_TICK_LABEL_SKIP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_TICK_MARK_SKIP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static CHILDREN_CAT_AX_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_CatAxExtension/c:ext", property_name: None },
];
static ATTRS_BASE_TIME_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_MAJOR_TIME_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_MINOR_TIME_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_MAJOR_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static ATTRS_MINOR_UNIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static CHILDREN_DATE_AX_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_DateAxExtension/c:ext", property_name: None },
];
static CHILDREN_SER_AX_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_SerAxExtension/c:ext", property_name: None },
];
static ATTRS_CROSS_BETWEEN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_DISPLAY_UNITS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Double/c:custUnit", property_name: None },
    ChildInfo { name: "c:CT_BuiltInUnit/c:builtInUnit", property_name: None },
    ChildInfo { name: "c:CT_DispUnitsLbl/c:dispUnitsLbl", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_VAL_AX_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ValAxExtension/c:ext", property_name: None },
];
static CHILDREN_D_LBL_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_DLblExtension/c:ext", property_name: None },
];
static ATTRS_D_LBL_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_D_LBL_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_DataLabelFieldTable/c15:dlblFieldTable", property_name: Some("DataLabelFieldTable") },
    ChildInfo { name: "c:CT_Boolean/c15:xForSave", property_name: Some("ExceptionForSave") },
    ChildInfo { name: "c:CT_Boolean/c15:showDataLabelsRange", property_name: Some("ShowDataLabelsRange") },
    ChildInfo { name: "a:CT_ShapeProperties/c15:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_Layout/c15:layout", property_name: Some("Layout") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_DATA_POINT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:idx", property_name: Some("Index") },
    ChildInfo { name: "c:CT_Boolean/c:invertIfNegative", property_name: Some("InvertIfNegative") },
    ChildInfo { name: "c:CT_Marker/c:marker", property_name: Some("Marker") },
    ChildInfo { name: "c:CT_Boolean/c:bubble3D", property_name: Some("Bubble3D") },
    ChildInfo { name: "c:CT_UnsignedInt/c:explosion", property_name: Some("Explosion") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_TRENDLINE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c:name", property_name: Some("TrendlineName") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_TrendlineType/c:trendlineType", property_name: Some("TrendlineType") },
    ChildInfo { name: "c:CT_Order/c:order", property_name: Some("PolynomialOrder") },
    ChildInfo { name: "c:CT_Period/c:period", property_name: Some("Period") },
    ChildInfo { name: "c:CT_Double/c:forward", property_name: Some("Forward") },
    ChildInfo { name: "c:CT_Double/c:backward", property_name: Some("Backward") },
    ChildInfo { name: "c:CT_Double/c:intercept", property_name: Some("Intercept") },
    ChildInfo { name: "c:CT_Boolean/c:dispRSqr", property_name: Some("DisplayRSquaredValue") },
    ChildInfo { name: "c:CT_Boolean/c:dispEq", property_name: Some("DisplayEquation") },
    ChildInfo { name: "c:CT_TrendlineLbl/c:trendlineLbl", property_name: Some("TrendlineLabel") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_ERROR_BARS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ErrDir/c:errDir", property_name: Some("ErrorDirection") },
    ChildInfo { name: "c:CT_ErrBarType/c:errBarType", property_name: Some("ErrorBarType") },
    ChildInfo { name: "c:CT_ErrValType/c:errValType", property_name: Some("ErrorBarValueType") },
    ChildInfo { name: "c:CT_Boolean/c:noEndCap", property_name: Some("NoEndCap") },
    ChildInfo { name: "c:CT_NumDataSource/c:plus", property_name: Some("Plus") },
    ChildInfo { name: "c:CT_NumDataSource/c:minus", property_name: Some("Minus") },
    ChildInfo { name: "c:CT_Double/c:val", property_name: Some("ErrorBarValue") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_CATEGORY_AXIS_DATA: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MultiLvlStrRef/c:multiLvlStrRef", property_name: Some("MultiLevelStringReference") },
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
    ChildInfo { name: "c:CT_StrRef/c:strRef", property_name: Some("StringReference") },
    ChildInfo { name: "c:CT_StrData/c:strLit", property_name: Some("StringLiteral") },
];
static CHILDREN_X_VALUES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MultiLvlStrRef/c:multiLvlStrRef", property_name: Some("MultiLevelStringReference") },
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
    ChildInfo { name: "c:CT_StrRef/c:strRef", property_name: Some("StringReference") },
    ChildInfo { name: "c:CT_StrData/c:strLit", property_name: Some("StringLiteral") },
];
static CHILDREN_LINE_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_LineSerExtension/c:ext", property_name: None },
];
static ATTRS_LINE_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_LINE_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: Some("FilteredSeriesTitle") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: Some("DataLabelsRange") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_SCATTER_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ScatterSerExtension/c:ext", property_name: None },
];
static ATTRS_SCATTER_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SCATTER_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: Some("FilteredSeriesTitle") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: Some("DataLabelsRange") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_RADAR_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_RadarSerExtension/c:ext", property_name: None },
];
static ATTRS_RADAR_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_RADAR_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: Some("FilteredSeriesTitle") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: Some("DataLabelsRange") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_BAR_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BarSerExtension/c:ext", property_name: None },
];
static ATTRS_BAR_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_BAR_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt", property_name: Some("InvertSolidFillFormat") },
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: Some("FilteredSeriesTitle") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: Some("DataLabelsRange") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_AREA_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_AreaSerExtension/c:ext", property_name: None },
];
static ATTRS_AREA_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_AREA_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: Some("FilteredSeriesTitle") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: Some("DataLabelsRange") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_PIE_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_PieSerExtension/c:ext", property_name: None },
];
static ATTRS_PIE_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PIE_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: Some("FilteredSeriesTitle") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: Some("DataLabelsRange") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_BUBBLE_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BubbleSerExtension/c:ext", property_name: None },
];
static ATTRS_BUBBLE_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_BUBBLE_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt", property_name: Some("InvertSolidFillFormat") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: Some("DataLabelsRange") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_SURFACE_SER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_SurfaceSerExtension/c:ext", property_name: None },
];
static ATTRS_SURFACE_SER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SURFACE_SER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: Some("FilteredSeriesTitle") },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: Some("FilteredCategoryTitle") },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: Some("CategoryFilterExceptions") },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: Some("ChartDataPointUniqueIDMap") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
];
static CHILDREN_DATA_DISPLAY_OPTIONS16: &[ChildInfo] = &[
    ChildInfo { name: "c16r3:CT_BooleanFalse/c16r3:dispNaAsBlank", property_name: Some("BooleanFalse") },
];
static CHILDREN_PIVOT_FORMATS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_PivotFmt/c:pivotFmt", property_name: None },
];
static CHILDREN_VIEW3_D: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_RotX/c:rotX", property_name: Some("RotateX") },
    ChildInfo { name: "c:CT_HPercent/c:hPercent", property_name: Some("HeightPercent") },
    ChildInfo { name: "c:CT_RotY/c:rotY", property_name: Some("RotateY") },
    ChildInfo { name: "c:CT_DepthPercent/c:depthPercent", property_name: Some("DepthPercent") },
    ChildInfo { name: "c:CT_Boolean/c:rAngAx", property_name: Some("RightAngleAxes") },
    ChildInfo { name: "c:CT_Perspective/c:perspective", property_name: Some("Perspective") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_FLOOR: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_WallThickness/c:thickness", property_name: Some("Thickness") },
    ChildInfo { name: "a:CT_ShapeProperties/c:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_SIDE_WALL: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_WallThickness/c:thickness", property_name: Some("Thickness") },
    ChildInfo { name: "a:CT_ShapeProperties/c:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_BACK_WALL: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_WallThickness/c:thickness", property_name: Some("Thickness") },
    ChildInfo { name: "a:CT_ShapeProperties/c:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_PictureOptions/c:pictureOptions", property_name: Some("PictureOptions") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_PLOT_AREA: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Layout/c:layout", property_name: Some("Layout") },
    ChildInfo { name: "c:CT_AreaChart/c:areaChart", property_name: None },
    ChildInfo { name: "c:CT_Area3DChart/c:area3DChart", property_name: None },
    ChildInfo { name: "c:CT_LineChart/c:lineChart", property_name: None },
    ChildInfo { name: "c:CT_Line3DChart/c:line3DChart", property_name: None },
    ChildInfo { name: "c:CT_StockChart/c:stockChart", property_name: None },
    ChildInfo { name: "c:CT_RadarChart/c:radarChart", property_name: None },
    ChildInfo { name: "c:CT_ScatterChart/c:scatterChart", property_name: None },
    ChildInfo { name: "c:CT_PieChart/c:pieChart", property_name: None },
    ChildInfo { name: "c:CT_Pie3DChart/c:pie3DChart", property_name: None },
    ChildInfo { name: "c:CT_DoughnutChart/c:doughnutChart", property_name: None },
    ChildInfo { name: "c:CT_BarChart/c:barChart", property_name: None },
    ChildInfo { name: "c:CT_Bar3DChart/c:bar3DChart", property_name: None },
    ChildInfo { name: "c:CT_OfPieChart/c:ofPieChart", property_name: None },
    ChildInfo { name: "c:CT_SurfaceChart/c:surfaceChart", property_name: None },
    ChildInfo { name: "c:CT_Surface3DChart/c:surface3DChart", property_name: None },
    ChildInfo { name: "c:CT_BubbleChart/c:bubbleChart", property_name: None },
    ChildInfo { name: "c:CT_ValAx/c:valAx", property_name: None },
    ChildInfo { name: "c:CT_CatAx/c:catAx", property_name: None },
    ChildInfo { name: "c:CT_DateAx/c:dateAx", property_name: None },
    ChildInfo { name: "c:CT_SerAx/c:serAx", property_name: None },
    ChildInfo { name: "c:CT_DTable/c:dTable", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/c:spPr", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_LEGEND: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_LegendPos/c:legendPos", property_name: Some("LegendPosition") },
    ChildInfo { name: "c:CT_LegendEntry/c:legendEntry", property_name: None },
    ChildInfo { name: "c:CT_Layout/c:layout", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c:overlay", property_name: None },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: None },
    ChildInfo { name: "a:CT_TextBody/c:txPr", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static ATTRS_DISPLAY_BLANKS_AS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_CHART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c16r3:CT_DataDisplayOptions16/c:ext", property_name: None },
];
static ATTRS_EDITING_LANGUAGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "ByteValue" },
];
static ATTRS_COLOR_MAP_OVERRIDE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bg1", property_name: Some("Background1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx1", property_name: Some("Text1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bg2", property_name: Some("Background2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx2", property_name: Some("Text2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent1", property_name: Some("Accent1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent2", property_name: Some("Accent2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent3", property_name: Some("Accent3"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent4", property_name: Some("Accent4"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent5", property_name: Some("Accent5"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent6", property_name: Some("Accent6"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hlink", property_name: Some("Hyperlink"), type_name: "EnumValue" },
    AttributeInfo { qname: ":folHlink", property_name: Some("FollowedHyperlink"), type_name: "EnumValue" },
];
static CHILDREN_COLOR_MAP_OVERRIDE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_PIVOT_SOURCE: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:name", property_name: Some("PivotTableName") },
    ChildInfo { name: "c:CT_UnsignedInt/c:fmtId", property_name: Some("FormatId") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_PROTECTION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:chartObject", property_name: Some("ChartObject") },
    ChildInfo { name: "c:CT_Boolean/c:data", property_name: Some("Data") },
    ChildInfo { name: "c:CT_Boolean/c:formatting", property_name: Some("Formatting") },
    ChildInfo { name: "c:CT_Boolean/c:selection", property_name: Some("Selection") },
    ChildInfo { name: "c:CT_Boolean/c:userInterface", property_name: Some("UserInterface") },
];
static CHILDREN_CHART: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Title/c:title", property_name: Some("Title") },
    ChildInfo { name: "c:CT_Boolean/c:autoTitleDeleted", property_name: Some("AutoTitleDeleted") },
    ChildInfo { name: "c:CT_PivotFmts/c:pivotFmts", property_name: Some("PivotFormats") },
    ChildInfo { name: "c:CT_View3D/c:view3D", property_name: Some("View3D") },
    ChildInfo { name: "c:CT_Surface/c:floor", property_name: Some("Floor") },
    ChildInfo { name: "c:CT_Surface/c:sideWall", property_name: Some("SideWall") },
    ChildInfo { name: "c:CT_Surface/c:backWall", property_name: Some("BackWall") },
    ChildInfo { name: "c:CT_PlotArea/c:plotArea", property_name: Some("PlotArea") },
    ChildInfo { name: "c:CT_Legend/c:legend", property_name: Some("Legend") },
    ChildInfo { name: "c:CT_Boolean/c:plotVisOnly", property_name: Some("PlotVisibleOnly") },
    ChildInfo { name: "c:CT_DispBlanksAs/c:dispBlanksAs", property_name: Some("DisplayBlanksAs") },
    ChildInfo { name: "c:CT_Boolean/c:showDLblsOverMax", property_name: Some("ShowDataLabelsOverMaximum") },
    ChildInfo { name: "c:CT_ChartExtensionList/c:extLst", property_name: Some("ChartExtensionList") },
];
static ATTRS_EXTERNAL_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_EXTERNAL_DATA: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Boolean/c:autoUpdate", property_name: Some("AutoUpdate") },
];
static CHILDREN_PRINT_SETTINGS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_HeaderFooter/c:headerFooter", property_name: Some("HeaderFooter") },
    ChildInfo { name: "c:CT_PageMargins/c:pageMargins", property_name: Some("PageMargins") },
    ChildInfo { name: "c:CT_PageSetup/c:pageSetup", property_name: Some("PageSetup") },
    ChildInfo { name: "c:CT_RelId/c:legacyDrawingHF", property_name: Some("LegacyDrawingHeaderFooter") },
];
static CHILDREN_CHART_SPACE_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ChartSpaceExtension/c:ext", property_name: None },
];
static ATTRS_CHART_SPACE_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_CHART_SPACE_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c14:CT_PivotOptions/c14:pivotOptions", property_name: Some("PivotOptions") },
    ChildInfo { name: "c14:CT_SketchOptions/c14:sketchOptions", property_name: Some("SketchOptions") },
    ChildInfo { name: "c:CT_PivotSource/c15:pivotSource", property_name: Some("PivotSource") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "NumberingFormat", local_name: "numFmt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NUMBERING_FORMAT, children: &[] },
    ElementInfo { class_name: "ChartShapeProperties", local_name: "spPr", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHART_SHAPE_PROPERTIES, children: CHILDREN_CHART_SHAPE_PROPERTIES },
    ElementInfo { class_name: "TextProperties", local_name: "txPr", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_PROPERTIES },
    ElementInfo { class_name: "RichText", local_name: "rich", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_TEXT },
    ElementInfo { class_name: "DataLabelPosition", local_name: "dLblPos", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_LABEL_POSITION, children: &[] },
    ElementInfo { class_name: "ShowLegendKey", local_name: "showLegendKey", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_LEGEND_KEY, children: &[] },
    ElementInfo { class_name: "ShowValue", local_name: "showVal", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_VALUE, children: &[] },
    ElementInfo { class_name: "ShowCategoryName", local_name: "showCatName", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_CATEGORY_NAME, children: &[] },
    ElementInfo { class_name: "ShowSeriesName", local_name: "showSerName", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_SERIES_NAME, children: &[] },
    ElementInfo { class_name: "ShowPercent", local_name: "showPercent", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_PERCENT, children: &[] },
    ElementInfo { class_name: "ShowBubbleSize", local_name: "showBubbleSize", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_BUBBLE_SIZE, children: &[] },
    ElementInfo { class_name: "ShowLeaderLines", local_name: "showLeaderLines", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_LEADER_LINES, children: &[] },
    ElementInfo { class_name: "VaryColors", local_name: "varyColors", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VARY_COLORS, children: &[] },
    ElementInfo { class_name: "Wireframe", local_name: "wireframe", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WIREFRAME, children: &[] },
    ElementInfo { class_name: "Delete", local_name: "delete", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DELETE, children: &[] },
    ElementInfo { class_name: "Overlay", local_name: "overlay", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OVERLAY, children: &[] },
    ElementInfo { class_name: "RightAngleAxes", local_name: "rAngAx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RIGHT_ANGLE_AXES, children: &[] },
    ElementInfo { class_name: "ShowHorizontalBorder", local_name: "showHorzBorder", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_HORIZONTAL_BORDER, children: &[] },
    ElementInfo { class_name: "ShowVerticalBorder", local_name: "showVertBorder", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_VERTICAL_BORDER, children: &[] },
    ElementInfo { class_name: "ShowOutlineBorder", local_name: "showOutline", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_OUTLINE_BORDER, children: &[] },
    ElementInfo { class_name: "ShowKeys", local_name: "showKeys", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_KEYS, children: &[] },
    ElementInfo { class_name: "InvertIfNegative", local_name: "invertIfNegative", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INVERT_IF_NEGATIVE, children: &[] },
    ElementInfo { class_name: "Bubble3D", local_name: "bubble3D", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUBBLE3_D, children: &[] },
    ElementInfo { class_name: "DisplayRSquaredValue", local_name: "dispRSqr", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DISPLAY_R_SQUARED_VALUE, children: &[] },
    ElementInfo { class_name: "DisplayEquation", local_name: "dispEq", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DISPLAY_EQUATION, children: &[] },
    ElementInfo { class_name: "NoEndCap", local_name: "noEndCap", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NO_END_CAP, children: &[] },
    ElementInfo { class_name: "ApplyToFront", local_name: "applyToFront", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_APPLY_TO_FRONT, children: &[] },
    ElementInfo { class_name: "ApplyToSides", local_name: "applyToSides", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_APPLY_TO_SIDES, children: &[] },
    ElementInfo { class_name: "ApplyToEnd", local_name: "applyToEnd", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_APPLY_TO_END, children: &[] },
    ElementInfo { class_name: "ChartObject", local_name: "chartObject", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHART_OBJECT, children: &[] },
    ElementInfo { class_name: "Data", local_name: "data", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA, children: &[] },
    ElementInfo { class_name: "Formatting", local_name: "formatting", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FORMATTING, children: &[] },
    ElementInfo { class_name: "Selection", local_name: "selection", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SELECTION, children: &[] },
    ElementInfo { class_name: "UserInterface", local_name: "userInterface", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_USER_INTERFACE, children: &[] },
    ElementInfo { class_name: "AutoUpdate", local_name: "autoUpdate", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AUTO_UPDATE, children: &[] },
    ElementInfo { class_name: "ShowMarker", local_name: "marker", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_MARKER, children: &[] },
    ElementInfo { class_name: "Smooth", local_name: "smooth", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SMOOTH, children: &[] },
    ElementInfo { class_name: "ShowNegativeBubbles", local_name: "showNegBubbles", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_NEGATIVE_BUBBLES, children: &[] },
    ElementInfo { class_name: "AutoLabeled", local_name: "auto", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AUTO_LABELED, children: &[] },
    ElementInfo { class_name: "NoMultiLevelLabels", local_name: "noMultiLvlLbl", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NO_MULTI_LEVEL_LABELS, children: &[] },
    ElementInfo { class_name: "AutoTitleDeleted", local_name: "autoTitleDeleted", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AUTO_TITLE_DELETED, children: &[] },
    ElementInfo { class_name: "PlotVisibleOnly", local_name: "plotVisOnly", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PLOT_VISIBLE_ONLY, children: &[] },
    ElementInfo { class_name: "ShowDataLabelsOverMaximum", local_name: "showDLblsOverMax", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_DATA_LABELS_OVER_MAXIMUM, children: &[] },
    ElementInfo { class_name: "Date1904", local_name: "date1904", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATE1904, children: &[] },
    ElementInfo { class_name: "RoundedCorners", local_name: "roundedCorners", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROUNDED_CORNERS, children: &[] },
    ElementInfo { class_name: "Separator", local_name: "separator", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "TrendlineName", local_name: "name", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Formula", local_name: "f", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Layout", local_name: "layout", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LAYOUT },
    ElementInfo { class_name: "ChartText", local_name: "tx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART_TEXT },
    ElementInfo { class_name: "LeaderLines", local_name: "leaderLines", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LEADER_LINES },
    ElementInfo { class_name: "DropLines", local_name: "dropLines", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DROP_LINES },
    ElementInfo { class_name: "MajorGridlines", local_name: "majorGridlines", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MAJOR_GRIDLINES },
    ElementInfo { class_name: "MinorGridlines", local_name: "minorGridlines", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MINOR_GRIDLINES },
    ElementInfo { class_name: "SeriesLines", local_name: "serLines", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SERIES_LINES },
    ElementInfo { class_name: "HighLowLines", local_name: "hiLowLines", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HIGH_LOW_LINES },
    ElementInfo { class_name: "Index", local_name: "idx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INDEX, children: &[] },
    ElementInfo { class_name: "Order", local_name: "order", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ORDER, children: &[] },
    ElementInfo { class_name: "AxisId", local_name: "axId", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AXIS_ID, children: &[] },
    ElementInfo { class_name: "CrossingAxis", local_name: "crossAx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CROSSING_AXIS, children: &[] },
    ElementInfo { class_name: "PointCount", local_name: "ptCount", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POINT_COUNT, children: &[] },
    ElementInfo { class_name: "SecondPiePoint", local_name: "secondPiePt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SECOND_PIE_POINT, children: &[] },
    ElementInfo { class_name: "Explosion", local_name: "explosion", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXPLOSION, children: &[] },
    ElementInfo { class_name: "FormatId", local_name: "fmtId", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FORMAT_ID, children: &[] },
    ElementInfo { class_name: "SeriesText", local_name: "tx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SERIES_TEXT },
    ElementInfo { class_name: "Grouping", local_name: "grouping", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GROUPING, children: &[] },
    ElementInfo { class_name: "LineChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_CHART_SERIES },
    ElementInfo { class_name: "DataLabels", local_name: "dLbls", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABELS },
    ElementInfo { class_name: "BarDirection", local_name: "barDir", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BAR_DIRECTION, children: &[] },
    ElementInfo { class_name: "BarGrouping", local_name: "grouping", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BAR_GROUPING, children: &[] },
    ElementInfo { class_name: "BarChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR_CHART_SERIES },
    ElementInfo { class_name: "AreaChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AREA_CHART_SERIES },
    ElementInfo { class_name: "PieChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIE_CHART_SERIES },
    ElementInfo { class_name: "SurfaceChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURFACE_CHART_SERIES },
    ElementInfo { class_name: "BandFormats", local_name: "bandFmts", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAND_FORMATS },
    ElementInfo { class_name: "Scaling", local_name: "scaling", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCALING },
    ElementInfo { class_name: "AxisPosition", local_name: "axPos", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AXIS_POSITION, children: &[] },
    ElementInfo { class_name: "Title", local_name: "title", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TITLE },
    ElementInfo { class_name: "MajorTickMark", local_name: "majorTickMark", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MAJOR_TICK_MARK, children: &[] },
    ElementInfo { class_name: "MinorTickMark", local_name: "minorTickMark", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MINOR_TICK_MARK, children: &[] },
    ElementInfo { class_name: "TickLabelPosition", local_name: "tickLblPos", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TICK_LABEL_POSITION, children: &[] },
    ElementInfo { class_name: "Crosses", local_name: "crosses", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CROSSES, children: &[] },
    ElementInfo { class_name: "CrossesAt", local_name: "crossesAt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CROSSES_AT, children: &[] },
    ElementInfo { class_name: "Left", local_name: "x", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEFT, children: &[] },
    ElementInfo { class_name: "Top", local_name: "y", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOP, children: &[] },
    ElementInfo { class_name: "Width", local_name: "w", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WIDTH, children: &[] },
    ElementInfo { class_name: "Height", local_name: "h", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HEIGHT, children: &[] },
    ElementInfo { class_name: "Forward", local_name: "forward", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FORWARD, children: &[] },
    ElementInfo { class_name: "Backward", local_name: "backward", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKWARD, children: &[] },
    ElementInfo { class_name: "Intercept", local_name: "intercept", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INTERCEPT, children: &[] },
    ElementInfo { class_name: "ErrorBarValue", local_name: "val", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ERROR_BAR_VALUE, children: &[] },
    ElementInfo { class_name: "SplitPosition", local_name: "splitPos", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SPLIT_POSITION, children: &[] },
    ElementInfo { class_name: "CustomDisplayUnit", local_name: "custUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_DISPLAY_UNIT, children: &[] },
    ElementInfo { class_name: "MaxAxisValue", local_name: "max", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MAX_AXIS_VALUE, children: &[] },
    ElementInfo { class_name: "MinAxisValue", local_name: "min", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MIN_AXIS_VALUE, children: &[] },
    ElementInfo { class_name: "ChartSpace", local_name: "chartSpace", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHART_SPACE, children: CHILDREN_CHART_SPACE },
    ElementInfo { class_name: "UserShapes", local_name: "userShapes", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_USER_SHAPES },
    ElementInfo { class_name: "ChartReference", local_name: "chart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHART_REFERENCE, children: &[] },
    ElementInfo { class_name: "LegacyDrawingHeaderFooter", local_name: "legacyDrawingHF", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEGACY_DRAWING_HEADER_FOOTER, children: &[] },
    ElementInfo { class_name: "UserShapesReference", local_name: "userShapes", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_USER_SHAPES_REFERENCE, children: &[] },
    ElementInfo { class_name: "Extension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTENSION, children: &[] },
    ElementInfo { class_name: "NumericValue", local_name: "v", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FormatCode", local_name: "formatCode", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "OddHeader", local_name: "oddHeader", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "OddFooter", local_name: "oddFooter", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EvenHeader", local_name: "evenHeader", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EvenFooter", local_name: "evenFooter", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FirstHeader", local_name: "firstHeader", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FirstFooter", local_name: "firstFooter", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PivotTableName", local_name: "name", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "NumericPoint", local_name: "pt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NUMERIC_POINT, children: CHILDREN_NUMERIC_POINT },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "NumberReference", local_name: "numRef", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUMBER_REFERENCE },
    ElementInfo { class_name: "NumberLiteral", local_name: "numLit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUMBER_LITERAL },
    ElementInfo { class_name: "NumberingCache", local_name: "numCache", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUMBERING_CACHE },
    ElementInfo { class_name: "Level", local_name: "lvl", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LEVEL },
    ElementInfo { class_name: "MultiLevelStringReference", local_name: "multiLvlStrRef", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MULTI_LEVEL_STRING_REFERENCE },
    ElementInfo { class_name: "StringReference", local_name: "strRef", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STRING_REFERENCE },
    ElementInfo { class_name: "StringLiteral", local_name: "strLit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STRING_LITERAL },
    ElementInfo { class_name: "StringCache", local_name: "strCache", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STRING_CACHE },
    ElementInfo { class_name: "LayoutTarget", local_name: "layoutTarget", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LAYOUT_TARGET, children: &[] },
    ElementInfo { class_name: "LeftMode", local_name: "xMode", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEFT_MODE, children: &[] },
    ElementInfo { class_name: "TopMode", local_name: "yMode", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOP_MODE, children: &[] },
    ElementInfo { class_name: "WidthMode", local_name: "wMode", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WIDTH_MODE, children: &[] },
    ElementInfo { class_name: "HeightMode", local_name: "hMode", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HEIGHT_MODE, children: &[] },
    ElementInfo { class_name: "ManualLayout", local_name: "manualLayout", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MANUAL_LAYOUT },
    ElementInfo { class_name: "RotateX", local_name: "rotX", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROTATE_X, children: &[] },
    ElementInfo { class_name: "HeightPercent", local_name: "hPercent", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HEIGHT_PERCENT, children: &[] },
    ElementInfo { class_name: "RotateY", local_name: "rotY", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROTATE_Y, children: &[] },
    ElementInfo { class_name: "DepthPercent", local_name: "depthPercent", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DEPTH_PERCENT, children: &[] },
    ElementInfo { class_name: "Perspective", local_name: "perspective", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PERSPECTIVE, children: &[] },
    ElementInfo { class_name: "Symbol", local_name: "symbol", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SYMBOL, children: &[] },
    ElementInfo { class_name: "Size", local_name: "size", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SIZE, children: &[] },
    ElementInfo { class_name: "Marker", local_name: "marker", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MARKER },
    ElementInfo { class_name: "PictureOptions", local_name: "pictureOptions", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PICTURE_OPTIONS },
    ElementInfo { class_name: "TrendlineType", local_name: "trendlineType", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TRENDLINE_TYPE, children: &[] },
    ElementInfo { class_name: "PolynomialOrder", local_name: "order", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POLYNOMIAL_ORDER, children: &[] },
    ElementInfo { class_name: "Period", local_name: "period", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PERIOD, children: &[] },
    ElementInfo { class_name: "TrendlineLabel", local_name: "trendlineLbl", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TRENDLINE_LABEL },
    ElementInfo { class_name: "ErrorDirection", local_name: "errDir", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ERROR_DIRECTION, children: &[] },
    ElementInfo { class_name: "ErrorBarType", local_name: "errBarType", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ERROR_BAR_TYPE, children: &[] },
    ElementInfo { class_name: "ErrorBarValueType", local_name: "errValType", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ERROR_BAR_VALUE_TYPE, children: &[] },
    ElementInfo { class_name: "Plus", local_name: "plus", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PLUS },
    ElementInfo { class_name: "Minus", local_name: "minus", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MINUS },
    ElementInfo { class_name: "Values", local_name: "val", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VALUES },
    ElementInfo { class_name: "YValues", local_name: "yVal", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_Y_VALUES },
    ElementInfo { class_name: "BubbleSize", local_name: "bubbleSize", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUBBLE_SIZE },
    ElementInfo { class_name: "GapWidth", local_name: "gapWidth", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GAP_WIDTH, children: &[] },
    ElementInfo { class_name: "GapDepth", local_name: "gapDepth", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GAP_DEPTH, children: &[] },
    ElementInfo { class_name: "UpBars", local_name: "upBars", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_UP_BARS },
    ElementInfo { class_name: "DownBars", local_name: "downBars", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DOWN_BARS },
    ElementInfo { class_name: "OfPieType", local_name: "ofPieType", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OF_PIE_TYPE, children: &[] },
    ElementInfo { class_name: "SplitType", local_name: "splitType", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SPLIT_TYPE, children: &[] },
    ElementInfo { class_name: "CustomSplit", local_name: "custSplit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_SPLIT },
    ElementInfo { class_name: "SecondPieSize", local_name: "secondPieSize", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SECOND_PIE_SIZE, children: &[] },
    ElementInfo { class_name: "BandFormat", local_name: "bandFmt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAND_FORMAT },
    ElementInfo { class_name: "PictureFormat", local_name: "pictureFormat", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PICTURE_FORMAT, children: &[] },
    ElementInfo { class_name: "PictureStackUnit", local_name: "pictureStackUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PICTURE_STACK_UNIT, children: &[] },
    ElementInfo { class_name: "BuiltInUnit", local_name: "builtInUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUILT_IN_UNIT, children: &[] },
    ElementInfo { class_name: "DisplayUnitsLabel", local_name: "dispUnitsLbl", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DISPLAY_UNITS_LABEL },
    ElementInfo { class_name: "LogBase", local_name: "logBase", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LOG_BASE, children: &[] },
    ElementInfo { class_name: "Orientation", local_name: "orientation", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ORIENTATION, children: &[] },
    ElementInfo { class_name: "PivotFormat", local_name: "pivotFmt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_FORMAT },
    ElementInfo { class_name: "LegendPosition", local_name: "legendPos", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEGEND_POSITION, children: &[] },
    ElementInfo { class_name: "LegendEntry", local_name: "legendEntry", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LEGEND_ENTRY },
    ElementInfo { class_name: "HeaderFooter", local_name: "headerFooter", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HEADER_FOOTER, children: CHILDREN_HEADER_FOOTER },
    ElementInfo { class_name: "PageMargins", local_name: "pageMargins", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PAGE_MARGINS, children: &[] },
    ElementInfo { class_name: "PageSetup", local_name: "pageSetup", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PAGE_SETUP, children: &[] },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "DataLabel", local_name: "dLbl", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABEL },
    ElementInfo { class_name: "AreaChart", local_name: "areaChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AREA_CHART },
    ElementInfo { class_name: "Area3DChart", local_name: "area3DChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AREA3_D_CHART },
    ElementInfo { class_name: "LineChart", local_name: "lineChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_CHART },
    ElementInfo { class_name: "Line3DChart", local_name: "line3DChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE3_D_CHART },
    ElementInfo { class_name: "StockChart", local_name: "stockChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STOCK_CHART },
    ElementInfo { class_name: "RadarChart", local_name: "radarChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RADAR_CHART },
    ElementInfo { class_name: "ScatterChart", local_name: "scatterChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCATTER_CHART },
    ElementInfo { class_name: "PieChart", local_name: "pieChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIE_CHART },
    ElementInfo { class_name: "Pie3DChart", local_name: "pie3DChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIE3_D_CHART },
    ElementInfo { class_name: "DoughnutChart", local_name: "doughnutChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DOUGHNUT_CHART },
    ElementInfo { class_name: "BarChart", local_name: "barChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR_CHART },
    ElementInfo { class_name: "Bar3DChart", local_name: "bar3DChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR3_D_CHART },
    ElementInfo { class_name: "OfPieChart", local_name: "ofPieChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OF_PIE_CHART },
    ElementInfo { class_name: "SurfaceChart", local_name: "surfaceChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURFACE_CHART },
    ElementInfo { class_name: "Surface3DChart", local_name: "surface3DChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURFACE3_D_CHART },
    ElementInfo { class_name: "BubbleChart", local_name: "bubbleChart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUBBLE_CHART },
    ElementInfo { class_name: "ValueAxis", local_name: "valAx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VALUE_AXIS },
    ElementInfo { class_name: "CategoryAxis", local_name: "catAx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CATEGORY_AXIS },
    ElementInfo { class_name: "DateAxis", local_name: "dateAx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATE_AXIS },
    ElementInfo { class_name: "SeriesAxis", local_name: "serAx", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SERIES_AXIS },
    ElementInfo { class_name: "DataTable", local_name: "dTable", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_TABLE },
    ElementInfo { class_name: "FirstSliceAngle", local_name: "firstSliceAng", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FIRST_SLICE_ANGLE, children: &[] },
    ElementInfo { class_name: "HoleSize", local_name: "holeSize", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HOLE_SIZE, children: &[] },
    ElementInfo { class_name: "StringPoint", local_name: "pt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STRING_POINT, children: CHILDREN_STRING_POINT },
    ElementInfo { class_name: "Thickness", local_name: "thickness", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_THICKNESS, children: &[] },
    ElementInfo { class_name: "StockChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STOCK_CHART_EXTENSION, children: CHILDREN_STOCK_CHART_EXTENSION },
    ElementInfo { class_name: "PieChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIE_CHART_EXTENSION, children: CHILDREN_PIE_CHART_EXTENSION },
    ElementInfo { class_name: "Pie3DChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIE3_D_CHART_EXTENSION, children: CHILDREN_PIE3_D_CHART_EXTENSION },
    ElementInfo { class_name: "NumRefExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NUM_REF_EXTENSION, children: CHILDREN_NUM_REF_EXTENSION },
    ElementInfo { class_name: "StrDataExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STR_DATA_EXTENSION, children: CHILDREN_STR_DATA_EXTENSION },
    ElementInfo { class_name: "StrRefExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STR_REF_EXTENSION, children: CHILDREN_STR_REF_EXTENSION },
    ElementInfo { class_name: "MultiLvlStrRefExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MULTI_LVL_STR_REF_EXTENSION, children: CHILDREN_MULTI_LVL_STR_REF_EXTENSION },
    ElementInfo { class_name: "DLblsExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_D_LBLS_EXTENSION, children: CHILDREN_D_LBLS_EXTENSION },
    ElementInfo { class_name: "LineChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_CHART_EXTENSION, children: CHILDREN_LINE_CHART_EXTENSION },
    ElementInfo { class_name: "Line3DChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE3_D_CHART_EXTENSION, children: CHILDREN_LINE3_D_CHART_EXTENSION },
    ElementInfo { class_name: "ScatterChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SCATTER_CHART_EXTENSION, children: CHILDREN_SCATTER_CHART_EXTENSION },
    ElementInfo { class_name: "RadarChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RADAR_CHART_EXTENSION, children: CHILDREN_RADAR_CHART_EXTENSION },
    ElementInfo { class_name: "BarChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BAR_CHART_EXTENSION, children: CHILDREN_BAR_CHART_EXTENSION },
    ElementInfo { class_name: "Bar3DChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BAR3_D_CHART_EXTENSION, children: CHILDREN_BAR3_D_CHART_EXTENSION },
    ElementInfo { class_name: "AreaChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AREA_CHART_EXTENSION, children: CHILDREN_AREA_CHART_EXTENSION },
    ElementInfo { class_name: "Area3DChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AREA3_D_CHART_EXTENSION, children: CHILDREN_AREA3_D_CHART_EXTENSION },
    ElementInfo { class_name: "BubbleChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BUBBLE_CHART_EXTENSION, children: CHILDREN_BUBBLE_CHART_EXTENSION },
    ElementInfo { class_name: "SurfaceChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SURFACE_CHART_EXTENSION, children: CHILDREN_SURFACE_CHART_EXTENSION },
    ElementInfo { class_name: "Surface3DChartExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SURFACE3_D_CHART_EXTENSION, children: CHILDREN_SURFACE3_D_CHART_EXTENSION },
    ElementInfo { class_name: "CatAxExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CAT_AX_EXTENSION, children: CHILDREN_CAT_AX_EXTENSION },
    ElementInfo { class_name: "DateAxExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATE_AX_EXTENSION, children: CHILDREN_DATE_AX_EXTENSION },
    ElementInfo { class_name: "SerAxExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SER_AX_EXTENSION, children: CHILDREN_SER_AX_EXTENSION },
    ElementInfo { class_name: "ValAxExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VAL_AX_EXTENSION, children: CHILDREN_VAL_AX_EXTENSION },
    ElementInfo { class_name: "UpDownBars", local_name: "upDownBars", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_UP_DOWN_BARS },
    ElementInfo { class_name: "StockChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STOCK_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "PieChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIE_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "Pie3DChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIE3_D_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "NumRefExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUM_REF_EXTENSION_LIST },
    ElementInfo { class_name: "StrDataExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STR_DATA_EXTENSION_LIST },
    ElementInfo { class_name: "StrRefExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STR_REF_EXTENSION_LIST },
    ElementInfo { class_name: "MultiLevelStringCache", local_name: "multiLvlStrCache", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MULTI_LEVEL_STRING_CACHE },
    ElementInfo { class_name: "MultiLvlStrRefExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MULTI_LVL_STR_REF_EXTENSION_LIST },
    ElementInfo { class_name: "DLblsExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_D_LBLS_EXTENSION_LIST },
    ElementInfo { class_name: "LineChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "Line3DChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE3_D_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "ScatterStyle", local_name: "scatterStyle", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SCATTER_STYLE, children: &[] },
    ElementInfo { class_name: "ScatterChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCATTER_CHART_SERIES },
    ElementInfo { class_name: "ScatterChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCATTER_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "RadarStyle", local_name: "radarStyle", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RADAR_STYLE, children: &[] },
    ElementInfo { class_name: "RadarChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RADAR_CHART_SERIES },
    ElementInfo { class_name: "RadarChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RADAR_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "Overlap", local_name: "overlap", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OVERLAP, children: &[] },
    ElementInfo { class_name: "BarChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "Shape", local_name: "shape", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHAPE, children: &[] },
    ElementInfo { class_name: "Bar3DChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR3_D_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "AreaChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AREA_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "Area3DChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AREA3_D_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "BubbleChartSeries", local_name: "ser", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUBBLE_CHART_SERIES },
    ElementInfo { class_name: "BubbleScale", local_name: "bubbleScale", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUBBLE_SCALE, children: &[] },
    ElementInfo { class_name: "SizeRepresents", local_name: "sizeRepresents", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SIZE_REPRESENTS, children: &[] },
    ElementInfo { class_name: "BubbleChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUBBLE_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "SurfaceChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURFACE_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "Surface3DChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURFACE3_D_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "LabelAlignment", local_name: "lblAlgn", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LABEL_ALIGNMENT, children: &[] },
    ElementInfo { class_name: "LabelOffset", local_name: "lblOffset", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LABEL_OFFSET, children: &[] },
    ElementInfo { class_name: "TickLabelSkip", local_name: "tickLblSkip", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TICK_LABEL_SKIP, children: &[] },
    ElementInfo { class_name: "TickMarkSkip", local_name: "tickMarkSkip", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TICK_MARK_SKIP, children: &[] },
    ElementInfo { class_name: "CatAxExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CAT_AX_EXTENSION_LIST },
    ElementInfo { class_name: "BaseTimeUnit", local_name: "baseTimeUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BASE_TIME_UNIT, children: &[] },
    ElementInfo { class_name: "MajorTimeUnit", local_name: "majorTimeUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MAJOR_TIME_UNIT, children: &[] },
    ElementInfo { class_name: "MinorTimeUnit", local_name: "minorTimeUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MINOR_TIME_UNIT, children: &[] },
    ElementInfo { class_name: "MajorUnit", local_name: "majorUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MAJOR_UNIT, children: &[] },
    ElementInfo { class_name: "MinorUnit", local_name: "minorUnit", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MINOR_UNIT, children: &[] },
    ElementInfo { class_name: "DateAxExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATE_AX_EXTENSION_LIST },
    ElementInfo { class_name: "SerAxExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SER_AX_EXTENSION_LIST },
    ElementInfo { class_name: "CrossBetween", local_name: "crossBetween", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CROSS_BETWEEN, children: &[] },
    ElementInfo { class_name: "DisplayUnits", local_name: "dispUnits", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DISPLAY_UNITS },
    ElementInfo { class_name: "ValAxExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VAL_AX_EXTENSION_LIST },
    ElementInfo { class_name: "DLblExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_D_LBL_EXTENSION_LIST },
    ElementInfo { class_name: "DLblExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_D_LBL_EXTENSION, children: CHILDREN_D_LBL_EXTENSION },
    ElementInfo { class_name: "DataPoint", local_name: "dPt", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_POINT },
    ElementInfo { class_name: "Trendline", local_name: "trendline", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TRENDLINE },
    ElementInfo { class_name: "ErrorBars", local_name: "errBars", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ERROR_BARS },
    ElementInfo { class_name: "CategoryAxisData", local_name: "cat", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CATEGORY_AXIS_DATA },
    ElementInfo { class_name: "XValues", local_name: "xVal", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_X_VALUES },
    ElementInfo { class_name: "LineSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_SER_EXTENSION_LIST },
    ElementInfo { class_name: "LineSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_SER_EXTENSION, children: CHILDREN_LINE_SER_EXTENSION },
    ElementInfo { class_name: "ScatterSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCATTER_SER_EXTENSION_LIST },
    ElementInfo { class_name: "ScatterSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SCATTER_SER_EXTENSION, children: CHILDREN_SCATTER_SER_EXTENSION },
    ElementInfo { class_name: "RadarSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RADAR_SER_EXTENSION_LIST },
    ElementInfo { class_name: "RadarSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RADAR_SER_EXTENSION, children: CHILDREN_RADAR_SER_EXTENSION },
    ElementInfo { class_name: "BarSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR_SER_EXTENSION_LIST },
    ElementInfo { class_name: "BarSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BAR_SER_EXTENSION, children: CHILDREN_BAR_SER_EXTENSION },
    ElementInfo { class_name: "AreaSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AREA_SER_EXTENSION_LIST },
    ElementInfo { class_name: "AreaSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AREA_SER_EXTENSION, children: CHILDREN_AREA_SER_EXTENSION },
    ElementInfo { class_name: "PieSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIE_SER_EXTENSION_LIST },
    ElementInfo { class_name: "PieSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIE_SER_EXTENSION, children: CHILDREN_PIE_SER_EXTENSION },
    ElementInfo { class_name: "BubbleSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUBBLE_SER_EXTENSION_LIST },
    ElementInfo { class_name: "BubbleSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BUBBLE_SER_EXTENSION, children: CHILDREN_BUBBLE_SER_EXTENSION },
    ElementInfo { class_name: "SurfaceSerExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURFACE_SER_EXTENSION_LIST },
    ElementInfo { class_name: "SurfaceSerExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SURFACE_SER_EXTENSION, children: CHILDREN_SURFACE_SER_EXTENSION },
    ElementInfo { class_name: "DataDisplayOptions16", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_DISPLAY_OPTIONS16 },
    ElementInfo { class_name: "PivotFormats", local_name: "pivotFmts", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_FORMATS },
    ElementInfo { class_name: "View3D", local_name: "view3D", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VIEW3_D },
    ElementInfo { class_name: "Floor", local_name: "floor", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FLOOR },
    ElementInfo { class_name: "SideWall", local_name: "sideWall", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SIDE_WALL },
    ElementInfo { class_name: "BackWall", local_name: "backWall", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACK_WALL },
    ElementInfo { class_name: "PlotArea", local_name: "plotArea", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PLOT_AREA },
    ElementInfo { class_name: "Legend", local_name: "legend", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LEGEND },
    ElementInfo { class_name: "DisplayBlanksAs", local_name: "dispBlanksAs", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DISPLAY_BLANKS_AS, children: &[] },
    ElementInfo { class_name: "ChartExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART_EXTENSION_LIST },
    ElementInfo { class_name: "EditingLanguage", local_name: "lang", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EDITING_LANGUAGE, children: &[] },
    ElementInfo { class_name: "Style", local_name: "style", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STYLE, children: &[] },
    ElementInfo { class_name: "ColorMapOverride", local_name: "clrMapOvr", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_MAP_OVERRIDE, children: CHILDREN_COLOR_MAP_OVERRIDE },
    ElementInfo { class_name: "PivotSource", local_name: "pivotSource", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_SOURCE },
    ElementInfo { class_name: "Protection", local_name: "protection", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PROTECTION },
    ElementInfo { class_name: "Chart", local_name: "chart", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART },
    ElementInfo { class_name: "ExternalData", local_name: "externalData", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTERNAL_DATA, children: CHILDREN_EXTERNAL_DATA },
    ElementInfo { class_name: "PrintSettings", local_name: "printSettings", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRINT_SETTINGS },
    ElementInfo { class_name: "ChartSpaceExtensionList", local_name: "extLst", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART_SPACE_EXTENSION_LIST },
    ElementInfo { class_name: "ChartSpaceExtension", local_name: "ext", prefix: "c", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHART_SPACE_EXTENSION, children: CHILDREN_CHART_SPACE_EXTENSION },
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

/// Create a `<c:numFmt>` element (`NumberingFormat`).
pub fn numbering_format() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "numFmt")
}

/// Create a `<c:spPr>` element (`ChartShapeProperties`).
pub fn chart_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<c:txPr>` element (`TextProperties`).
pub fn text_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "txPr").with_children(children)
}

/// Create a `<c:rich>` element (`RichText`).
pub fn rich_text(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "rich").with_children(children)
}

/// Create a `<c:dLblPos>` element (`DataLabelPosition`).
pub fn data_label_position() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dLblPos")
}

/// Create a `<c:showLegendKey>` element (`ShowLegendKey`).
pub fn show_legend_key() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showLegendKey")
}

/// Create a `<c:showVal>` element (`ShowValue`).
pub fn show_value() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showVal")
}

/// Create a `<c:showCatName>` element (`ShowCategoryName`).
pub fn show_category_name() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showCatName")
}

/// Create a `<c:showSerName>` element (`ShowSeriesName`).
pub fn show_series_name() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showSerName")
}

/// Create a `<c:showPercent>` element (`ShowPercent`).
pub fn show_percent() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showPercent")
}

/// Create a `<c:showBubbleSize>` element (`ShowBubbleSize`).
pub fn show_bubble_size() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showBubbleSize")
}

/// Create a `<c:showLeaderLines>` element (`ShowLeaderLines`).
pub fn show_leader_lines() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showLeaderLines")
}

/// Create a `<c:varyColors>` element (`VaryColors`).
pub fn vary_colors() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "varyColors")
}

/// Create a `<c:wireframe>` element (`Wireframe`).
pub fn wireframe() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "wireframe")
}

/// Create a `<c:delete>` element (`Delete`).
pub fn delete() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "delete")
}

/// Create a `<c:overlay>` element (`Overlay`).
pub fn overlay() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "overlay")
}

/// Create a `<c:rAngAx>` element (`RightAngleAxes`).
pub fn right_angle_axes() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "rAngAx")
}

/// Create a `<c:showHorzBorder>` element (`ShowHorizontalBorder`).
pub fn show_horizontal_border() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showHorzBorder")
}

/// Create a `<c:showVertBorder>` element (`ShowVerticalBorder`).
pub fn show_vertical_border() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showVertBorder")
}

/// Create a `<c:showOutline>` element (`ShowOutlineBorder`).
pub fn show_outline_border() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showOutline")
}

/// Create a `<c:showKeys>` element (`ShowKeys`).
pub fn show_keys() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showKeys")
}

/// Create a `<c:invertIfNegative>` element (`InvertIfNegative`).
pub fn invert_if_negative() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "invertIfNegative")
}

/// Create a `<c:bubble3D>` element (`Bubble3D`).
pub fn bubble3_d() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "bubble3D")
}

/// Create a `<c:dispRSqr>` element (`DisplayRSquaredValue`).
pub fn display_r_squared_value() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dispRSqr")
}

/// Create a `<c:dispEq>` element (`DisplayEquation`).
pub fn display_equation() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dispEq")
}

/// Create a `<c:noEndCap>` element (`NoEndCap`).
pub fn no_end_cap() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "noEndCap")
}

/// Create a `<c:applyToFront>` element (`ApplyToFront`).
pub fn apply_to_front() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "applyToFront")
}

/// Create a `<c:applyToSides>` element (`ApplyToSides`).
pub fn apply_to_sides() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "applyToSides")
}

/// Create a `<c:applyToEnd>` element (`ApplyToEnd`).
pub fn apply_to_end() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "applyToEnd")
}

/// Create a `<c:chartObject>` element (`ChartObject`).
pub fn chart_object() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "chartObject")
}

/// Create a `<c:data>` element (`Data`).
pub fn data() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "data")
}

/// Create a `<c:formatting>` element (`Formatting`).
pub fn formatting() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "formatting")
}

/// Create a `<c:selection>` element (`Selection`).
pub fn selection() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "selection")
}

/// Create a `<c:userInterface>` element (`UserInterface`).
pub fn user_interface() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "userInterface")
}

/// Create a `<c:autoUpdate>` element (`AutoUpdate`).
pub fn auto_update() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "autoUpdate")
}

/// Create a `<c:marker>` element (`ShowMarker`).
pub fn show_marker() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "marker")
}

/// Create a `<c:smooth>` element (`Smooth`).
pub fn smooth() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "smooth")
}

/// Create a `<c:showNegBubbles>` element (`ShowNegativeBubbles`).
pub fn show_negative_bubbles() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showNegBubbles")
}

/// Create a `<c:auto>` element (`AutoLabeled`).
pub fn auto_labeled() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "auto")
}

/// Create a `<c:noMultiLvlLbl>` element (`NoMultiLevelLabels`).
pub fn no_multi_level_labels() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "noMultiLvlLbl")
}

/// Create a `<c:autoTitleDeleted>` element (`AutoTitleDeleted`).
pub fn auto_title_deleted() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "autoTitleDeleted")
}

/// Create a `<c:plotVisOnly>` element (`PlotVisibleOnly`).
pub fn plot_visible_only() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "plotVisOnly")
}

/// Create a `<c:showDLblsOverMax>` element (`ShowDataLabelsOverMaximum`).
pub fn show_data_labels_over_maximum() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "showDLblsOverMax")
}

/// Create a `<c:date1904>` element (`Date1904`).
pub fn date1904() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "date1904")
}

/// Create a `<c:roundedCorners>` element (`RoundedCorners`).
pub fn rounded_corners() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "roundedCorners")
}

/// Create a `<c:separator>` element (`Separator`).
pub fn separator(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "separator").with_text(value)
}

/// Create a `<c:name>` element (`TrendlineName`).
pub fn trendline_name(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "name").with_text(value)
}

/// Create a `<c:f>` element (`Formula`).
pub fn formula(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "f").with_text(value)
}

/// Create a `<c:layout>` element (`Layout`).
pub fn layout(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "layout").with_children(children)
}

/// Create a `<c:tx>` element (`ChartText`).
pub fn chart_text(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "tx").with_children(children)
}

/// Create a `<c:leaderLines>` element (`LeaderLines`).
pub fn leader_lines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "leaderLines").with_children(children)
}

/// Create a `<c:dropLines>` element (`DropLines`).
pub fn drop_lines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dropLines").with_children(children)
}

/// Create a `<c:majorGridlines>` element (`MajorGridlines`).
pub fn major_gridlines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "majorGridlines").with_children(children)
}

/// Create a `<c:minorGridlines>` element (`MinorGridlines`).
pub fn minor_gridlines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "minorGridlines").with_children(children)
}

/// Create a `<c:serLines>` element (`SeriesLines`).
pub fn series_lines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "serLines").with_children(children)
}

/// Create a `<c:hiLowLines>` element (`HighLowLines`).
pub fn high_low_lines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "hiLowLines").with_children(children)
}

/// Create a `<c:idx>` element (`Index`).
pub fn index() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "idx")
}

/// Create a `<c:order>` element (`Order`).
pub fn order() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "order")
}

/// Create a `<c:axId>` element (`AxisId`).
pub fn axis_id() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "axId")
}

/// Create a `<c:crossAx>` element (`CrossingAxis`).
pub fn crossing_axis() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "crossAx")
}

/// Create a `<c:ptCount>` element (`PointCount`).
pub fn point_count() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ptCount")
}

/// Create a `<c:secondPiePt>` element (`SecondPiePoint`).
pub fn second_pie_point() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "secondPiePt")
}

/// Create a `<c:explosion>` element (`Explosion`).
pub fn explosion() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "explosion")
}

/// Create a `<c:fmtId>` element (`FormatId`).
pub fn format_id() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "fmtId")
}

/// Create a `<c:tx>` element (`SeriesText`).
pub fn series_text(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "tx").with_children(children)
}

/// Create a `<c:grouping>` element (`Grouping`).
pub fn grouping() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "grouping")
}

/// Create a `<c:ser>` element (`LineChartSeries`).
pub fn line_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:dLbls>` element (`DataLabels`).
pub fn data_labels(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dLbls").with_children(children)
}

/// Create a `<c:barDir>` element (`BarDirection`).
pub fn bar_direction() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "barDir")
}

/// Create a `<c:grouping>` element (`BarGrouping`).
pub fn bar_grouping() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "grouping")
}

/// Create a `<c:ser>` element (`BarChartSeries`).
pub fn bar_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:ser>` element (`AreaChartSeries`).
pub fn area_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:ser>` element (`PieChartSeries`).
pub fn pie_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:ser>` element (`SurfaceChartSeries`).
pub fn surface_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:bandFmts>` element (`BandFormats`).
pub fn band_formats(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "bandFmts").with_children(children)
}

/// Create a `<c:scaling>` element (`Scaling`).
pub fn scaling(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "scaling").with_children(children)
}

/// Create a `<c:axPos>` element (`AxisPosition`).
pub fn axis_position() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "axPos")
}

/// Create a `<c:title>` element (`Title`).
pub fn title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "title").with_children(children)
}

/// Create a `<c:majorTickMark>` element (`MajorTickMark`).
pub fn major_tick_mark() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "majorTickMark")
}

/// Create a `<c:minorTickMark>` element (`MinorTickMark`).
pub fn minor_tick_mark() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "minorTickMark")
}

/// Create a `<c:tickLblPos>` element (`TickLabelPosition`).
pub fn tick_label_position() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "tickLblPos")
}

/// Create a `<c:crosses>` element (`Crosses`).
pub fn crosses() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "crosses")
}

/// Create a `<c:crossesAt>` element (`CrossesAt`).
pub fn crosses_at() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "crossesAt")
}

/// Create a `<c:x>` element (`Left`).
pub fn left() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "x")
}

/// Create a `<c:y>` element (`Top`).
pub fn top() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "y")
}

/// Create a `<c:w>` element (`Width`).
pub fn width() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "w")
}

/// Create a `<c:h>` element (`Height`).
pub fn height() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "h")
}

/// Create a `<c:forward>` element (`Forward`).
pub fn forward() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "forward")
}

/// Create a `<c:backward>` element (`Backward`).
pub fn backward() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "backward")
}

/// Create a `<c:intercept>` element (`Intercept`).
pub fn intercept() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "intercept")
}

/// Create a `<c:val>` element (`ErrorBarValue`).
pub fn error_bar_value() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "val")
}

/// Create a `<c:splitPos>` element (`SplitPosition`).
pub fn split_position() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "splitPos")
}

/// Create a `<c:custUnit>` element (`CustomDisplayUnit`).
pub fn custom_display_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "custUnit")
}

/// Create a `<c:max>` element (`MaxAxisValue`).
pub fn max_axis_value() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "max")
}

/// Create a `<c:min>` element (`MinAxisValue`).
pub fn min_axis_value() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "min")
}

/// Create a `<c:chartSpace>` element (`ChartSpace`).
pub fn chart_space(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "chartSpace").with_children(children)
}

/// Create a `<c:userShapes>` element (`UserShapes`).
pub fn user_shapes(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "userShapes").with_children(children)
}

/// Create a `<c:chart>` element (`ChartReference`).
pub fn chart_reference() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "chart")
}

/// Create a `<c:legacyDrawingHF>` element (`LegacyDrawingHeaderFooter`).
pub fn legacy_drawing_header_footer() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "legacyDrawingHF")
}

/// Create a `<c:userShapes>` element (`UserShapesReference`).
pub fn user_shapes_reference() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "userShapes")
}

/// Create a `<c:ext>` element (`Extension`).
pub fn extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:v>` element (`NumericValue`).
pub fn numeric_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<c:formatCode>` element (`FormatCode`).
pub fn format_code(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "formatCode").with_text(value)
}

/// Create a `<c:oddHeader>` element (`OddHeader`).
pub fn odd_header(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "oddHeader").with_text(value)
}

/// Create a `<c:oddFooter>` element (`OddFooter`).
pub fn odd_footer(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "oddFooter").with_text(value)
}

/// Create a `<c:evenHeader>` element (`EvenHeader`).
pub fn even_header(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "evenHeader").with_text(value)
}

/// Create a `<c:evenFooter>` element (`EvenFooter`).
pub fn even_footer(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "evenFooter").with_text(value)
}

/// Create a `<c:firstHeader>` element (`FirstHeader`).
pub fn first_header(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "firstHeader").with_text(value)
}

/// Create a `<c:firstFooter>` element (`FirstFooter`).
pub fn first_footer(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "firstFooter").with_text(value)
}

/// Create a `<c:name>` element (`PivotTableName`).
pub fn pivot_table_name(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "name").with_text(value)
}

/// Create a `<c:pt>` element (`NumericPoint`).
pub fn numeric_point(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pt").with_children(children)
}

/// Create a `<c:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:numRef>` element (`NumberReference`).
pub fn number_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "numRef").with_children(children)
}

/// Create a `<c:numLit>` element (`NumberLiteral`).
pub fn number_literal(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "numLit").with_children(children)
}

/// Create a `<c:numCache>` element (`NumberingCache`).
pub fn numbering_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "numCache").with_children(children)
}

/// Create a `<c:lvl>` element (`Level`).
pub fn level(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "lvl").with_children(children)
}

/// Create a `<c:multiLvlStrRef>` element (`MultiLevelStringReference`).
pub fn multi_level_string_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "multiLvlStrRef").with_children(children)
}

/// Create a `<c:strRef>` element (`StringReference`).
pub fn string_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "strRef").with_children(children)
}

/// Create a `<c:strLit>` element (`StringLiteral`).
pub fn string_literal(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "strLit").with_children(children)
}

/// Create a `<c:strCache>` element (`StringCache`).
pub fn string_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "strCache").with_children(children)
}

/// Create a `<c:layoutTarget>` element (`LayoutTarget`).
pub fn layout_target() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "layoutTarget")
}

/// Create a `<c:xMode>` element (`LeftMode`).
pub fn left_mode() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "xMode")
}

/// Create a `<c:yMode>` element (`TopMode`).
pub fn top_mode() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "yMode")
}

/// Create a `<c:wMode>` element (`WidthMode`).
pub fn width_mode() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "wMode")
}

/// Create a `<c:hMode>` element (`HeightMode`).
pub fn height_mode() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "hMode")
}

/// Create a `<c:manualLayout>` element (`ManualLayout`).
pub fn manual_layout(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "manualLayout").with_children(children)
}

/// Create a `<c:rotX>` element (`RotateX`).
pub fn rotate_x() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "rotX")
}

/// Create a `<c:hPercent>` element (`HeightPercent`).
pub fn height_percent() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "hPercent")
}

/// Create a `<c:rotY>` element (`RotateY`).
pub fn rotate_y() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "rotY")
}

/// Create a `<c:depthPercent>` element (`DepthPercent`).
pub fn depth_percent() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "depthPercent")
}

/// Create a `<c:perspective>` element (`Perspective`).
pub fn perspective() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "perspective")
}

/// Create a `<c:symbol>` element (`Symbol`).
pub fn symbol() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "symbol")
}

/// Create a `<c:size>` element (`Size`).
pub fn size() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "size")
}

/// Create a `<c:marker>` element (`Marker`).
pub fn marker(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "marker").with_children(children)
}

/// Create a `<c:pictureOptions>` element (`PictureOptions`).
pub fn picture_options(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pictureOptions").with_children(children)
}

/// Create a `<c:trendlineType>` element (`TrendlineType`).
pub fn trendline_type() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "trendlineType")
}

/// Create a `<c:order>` element (`PolynomialOrder`).
pub fn polynomial_order() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "order")
}

/// Create a `<c:period>` element (`Period`).
pub fn period() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "period")
}

/// Create a `<c:trendlineLbl>` element (`TrendlineLabel`).
pub fn trendline_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "trendlineLbl").with_children(children)
}

/// Create a `<c:errDir>` element (`ErrorDirection`).
pub fn error_direction() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "errDir")
}

/// Create a `<c:errBarType>` element (`ErrorBarType`).
pub fn error_bar_type() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "errBarType")
}

/// Create a `<c:errValType>` element (`ErrorBarValueType`).
pub fn error_bar_value_type() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "errValType")
}

/// Create a `<c:plus>` element (`Plus`).
pub fn plus(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "plus").with_children(children)
}

/// Create a `<c:minus>` element (`Minus`).
pub fn minus(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "minus").with_children(children)
}

/// Create a `<c:val>` element (`Values`).
pub fn values(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "val").with_children(children)
}

/// Create a `<c:yVal>` element (`YValues`).
pub fn y_values(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "yVal").with_children(children)
}

/// Create a `<c:bubbleSize>` element (`BubbleSize`).
pub fn bubble_size(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "bubbleSize").with_children(children)
}

/// Create a `<c:gapWidth>` element (`GapWidth`).
pub fn gap_width() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "gapWidth")
}

/// Create a `<c:gapDepth>` element (`GapDepth`).
pub fn gap_depth() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "gapDepth")
}

/// Create a `<c:upBars>` element (`UpBars`).
pub fn up_bars(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "upBars").with_children(children)
}

/// Create a `<c:downBars>` element (`DownBars`).
pub fn down_bars(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "downBars").with_children(children)
}

/// Create a `<c:ofPieType>` element (`OfPieType`).
pub fn of_pie_type() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ofPieType")
}

/// Create a `<c:splitType>` element (`SplitType`).
pub fn split_type() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "splitType")
}

/// Create a `<c:custSplit>` element (`CustomSplit`).
pub fn custom_split(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "custSplit").with_children(children)
}

/// Create a `<c:secondPieSize>` element (`SecondPieSize`).
pub fn second_pie_size() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "secondPieSize")
}

/// Create a `<c:bandFmt>` element (`BandFormat`).
pub fn band_format(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "bandFmt").with_children(children)
}

/// Create a `<c:pictureFormat>` element (`PictureFormat`).
pub fn picture_format() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pictureFormat")
}

/// Create a `<c:pictureStackUnit>` element (`PictureStackUnit`).
pub fn picture_stack_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pictureStackUnit")
}

/// Create a `<c:builtInUnit>` element (`BuiltInUnit`).
pub fn built_in_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "builtInUnit")
}

/// Create a `<c:dispUnitsLbl>` element (`DisplayUnitsLabel`).
pub fn display_units_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dispUnitsLbl").with_children(children)
}

/// Create a `<c:logBase>` element (`LogBase`).
pub fn log_base() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "logBase")
}

/// Create a `<c:orientation>` element (`Orientation`).
pub fn orientation() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "orientation")
}

/// Create a `<c:pivotFmt>` element (`PivotFormat`).
pub fn pivot_format(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pivotFmt").with_children(children)
}

/// Create a `<c:legendPos>` element (`LegendPosition`).
pub fn legend_position() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "legendPos")
}

/// Create a `<c:legendEntry>` element (`LegendEntry`).
pub fn legend_entry(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "legendEntry").with_children(children)
}

/// Create a `<c:headerFooter>` element (`HeaderFooter`).
pub fn header_footer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "headerFooter").with_children(children)
}

/// Create a `<c:pageMargins>` element (`PageMargins`).
pub fn page_margins() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pageMargins")
}

/// Create a `<c:pageSetup>` element (`PageSetup`).
pub fn page_setup() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pageSetup")
}

/// Create a `<c:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<c:dLbl>` element (`DataLabel`).
pub fn data_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dLbl").with_children(children)
}

/// Create a `<c:areaChart>` element (`AreaChart`).
pub fn area_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "areaChart").with_children(children)
}

/// Create a `<c:area3DChart>` element (`Area3DChart`).
pub fn area3_d_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "area3DChart").with_children(children)
}

/// Create a `<c:lineChart>` element (`LineChart`).
pub fn line_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "lineChart").with_children(children)
}

/// Create a `<c:line3DChart>` element (`Line3DChart`).
pub fn line3_d_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "line3DChart").with_children(children)
}

/// Create a `<c:stockChart>` element (`StockChart`).
pub fn stock_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "stockChart").with_children(children)
}

/// Create a `<c:radarChart>` element (`RadarChart`).
pub fn radar_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "radarChart").with_children(children)
}

/// Create a `<c:scatterChart>` element (`ScatterChart`).
pub fn scatter_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "scatterChart").with_children(children)
}

/// Create a `<c:pieChart>` element (`PieChart`).
pub fn pie_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pieChart").with_children(children)
}

/// Create a `<c:pie3DChart>` element (`Pie3DChart`).
pub fn pie3_d_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pie3DChart").with_children(children)
}

/// Create a `<c:doughnutChart>` element (`DoughnutChart`).
pub fn doughnut_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "doughnutChart").with_children(children)
}

/// Create a `<c:barChart>` element (`BarChart`).
pub fn bar_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "barChart").with_children(children)
}

/// Create a `<c:bar3DChart>` element (`Bar3DChart`).
pub fn bar3_d_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "bar3DChart").with_children(children)
}

/// Create a `<c:ofPieChart>` element (`OfPieChart`).
pub fn of_pie_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ofPieChart").with_children(children)
}

/// Create a `<c:surfaceChart>` element (`SurfaceChart`).
pub fn surface_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "surfaceChart").with_children(children)
}

/// Create a `<c:surface3DChart>` element (`Surface3DChart`).
pub fn surface3_d_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "surface3DChart").with_children(children)
}

/// Create a `<c:bubbleChart>` element (`BubbleChart`).
pub fn bubble_chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "bubbleChart").with_children(children)
}

/// Create a `<c:valAx>` element (`ValueAxis`).
pub fn value_axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "valAx").with_children(children)
}

/// Create a `<c:catAx>` element (`CategoryAxis`).
pub fn category_axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "catAx").with_children(children)
}

/// Create a `<c:dateAx>` element (`DateAxis`).
pub fn date_axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dateAx").with_children(children)
}

/// Create a `<c:serAx>` element (`SeriesAxis`).
pub fn series_axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "serAx").with_children(children)
}

/// Create a `<c:dTable>` element (`DataTable`).
pub fn data_table(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dTable").with_children(children)
}

/// Create a `<c:firstSliceAng>` element (`FirstSliceAngle`).
pub fn first_slice_angle() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "firstSliceAng")
}

/// Create a `<c:holeSize>` element (`HoleSize`).
pub fn hole_size() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "holeSize")
}

/// Create a `<c:pt>` element (`StringPoint`).
pub fn string_point(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pt").with_children(children)
}

/// Create a `<c:thickness>` element (`Thickness`).
pub fn thickness() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "thickness")
}

/// Create a `<c:ext>` element (`StockChartExtension`).
pub fn stock_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`PieChartExtension`).
pub fn pie_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`Pie3DChartExtension`).
pub fn pie3_d_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`NumRefExtension`).
pub fn num_ref_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`StrDataExtension`).
pub fn str_data_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`StrRefExtension`).
pub fn str_ref_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`MultiLvlStrRefExtension`).
pub fn multi_lvl_str_ref_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`DLblsExtension`).
pub fn d_lbls_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`LineChartExtension`).
pub fn line_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`Line3DChartExtension`).
pub fn line3_d_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`ScatterChartExtension`).
pub fn scatter_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`RadarChartExtension`).
pub fn radar_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`BarChartExtension`).
pub fn bar_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`Bar3DChartExtension`).
pub fn bar3_d_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`AreaChartExtension`).
pub fn area_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`Area3DChartExtension`).
pub fn area3_d_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`BubbleChartExtension`).
pub fn bubble_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`SurfaceChartExtension`).
pub fn surface_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`Surface3DChartExtension`).
pub fn surface3_d_chart_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`CatAxExtension`).
pub fn cat_ax_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`DateAxExtension`).
pub fn date_ax_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`SerAxExtension`).
pub fn ser_ax_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`ValAxExtension`).
pub fn val_ax_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:upDownBars>` element (`UpDownBars`).
pub fn up_down_bars(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "upDownBars").with_children(children)
}

/// Create a `<c:extLst>` element (`StockChartExtensionList`).
pub fn stock_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`PieChartExtensionList`).
pub fn pie_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`Pie3DChartExtensionList`).
pub fn pie3_d_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`NumRefExtensionList`).
pub fn num_ref_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`StrDataExtensionList`).
pub fn str_data_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`StrRefExtensionList`).
pub fn str_ref_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:multiLvlStrCache>` element (`MultiLevelStringCache`).
pub fn multi_level_string_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "multiLvlStrCache").with_children(children)
}

/// Create a `<c:extLst>` element (`MultiLvlStrRefExtensionList`).
pub fn multi_lvl_str_ref_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`DLblsExtensionList`).
pub fn d_lbls_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`LineChartExtensionList`).
pub fn line_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`Line3DChartExtensionList`).
pub fn line3_d_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:scatterStyle>` element (`ScatterStyle`).
pub fn scatter_style() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "scatterStyle")
}

/// Create a `<c:ser>` element (`ScatterChartSeries`).
pub fn scatter_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:extLst>` element (`ScatterChartExtensionList`).
pub fn scatter_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:radarStyle>` element (`RadarStyle`).
pub fn radar_style() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "radarStyle")
}

/// Create a `<c:ser>` element (`RadarChartSeries`).
pub fn radar_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:extLst>` element (`RadarChartExtensionList`).
pub fn radar_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:overlap>` element (`Overlap`).
pub fn overlap() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "overlap")
}

/// Create a `<c:extLst>` element (`BarChartExtensionList`).
pub fn bar_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:shape>` element (`Shape`).
pub fn shape() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "shape")
}

/// Create a `<c:extLst>` element (`Bar3DChartExtensionList`).
pub fn bar3_d_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`AreaChartExtensionList`).
pub fn area_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`Area3DChartExtensionList`).
pub fn area3_d_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ser>` element (`BubbleChartSeries`).
pub fn bubble_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c:bubbleScale>` element (`BubbleScale`).
pub fn bubble_scale() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "bubbleScale")
}

/// Create a `<c:sizeRepresents>` element (`SizeRepresents`).
pub fn size_represents() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "sizeRepresents")
}

/// Create a `<c:extLst>` element (`BubbleChartExtensionList`).
pub fn bubble_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`SurfaceChartExtensionList`).
pub fn surface_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`Surface3DChartExtensionList`).
pub fn surface3_d_chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:lblAlgn>` element (`LabelAlignment`).
pub fn label_alignment() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "lblAlgn")
}

/// Create a `<c:lblOffset>` element (`LabelOffset`).
pub fn label_offset() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "lblOffset")
}

/// Create a `<c:tickLblSkip>` element (`TickLabelSkip`).
pub fn tick_label_skip() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "tickLblSkip")
}

/// Create a `<c:tickMarkSkip>` element (`TickMarkSkip`).
pub fn tick_mark_skip() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "tickMarkSkip")
}

/// Create a `<c:extLst>` element (`CatAxExtensionList`).
pub fn cat_ax_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:baseTimeUnit>` element (`BaseTimeUnit`).
pub fn base_time_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "baseTimeUnit")
}

/// Create a `<c:majorTimeUnit>` element (`MajorTimeUnit`).
pub fn major_time_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "majorTimeUnit")
}

/// Create a `<c:minorTimeUnit>` element (`MinorTimeUnit`).
pub fn minor_time_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "minorTimeUnit")
}

/// Create a `<c:majorUnit>` element (`MajorUnit`).
pub fn major_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "majorUnit")
}

/// Create a `<c:minorUnit>` element (`MinorUnit`).
pub fn minor_unit() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "minorUnit")
}

/// Create a `<c:extLst>` element (`DateAxExtensionList`).
pub fn date_ax_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`SerAxExtensionList`).
pub fn ser_ax_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:crossBetween>` element (`CrossBetween`).
pub fn cross_between() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "crossBetween")
}

/// Create a `<c:dispUnits>` element (`DisplayUnits`).
pub fn display_units(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dispUnits").with_children(children)
}

/// Create a `<c:extLst>` element (`ValAxExtensionList`).
pub fn val_ax_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:extLst>` element (`DLblExtensionList`).
pub fn d_lbl_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`DLblExtension`).
pub fn d_lbl_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:dPt>` element (`DataPoint`).
pub fn data_point(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dPt").with_children(children)
}

/// Create a `<c:trendline>` element (`Trendline`).
pub fn trendline(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "trendline").with_children(children)
}

/// Create a `<c:errBars>` element (`ErrorBars`).
pub fn error_bars(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "errBars").with_children(children)
}

/// Create a `<c:cat>` element (`CategoryAxisData`).
pub fn category_axis_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "cat").with_children(children)
}

/// Create a `<c:xVal>` element (`XValues`).
pub fn x_values(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "xVal").with_children(children)
}

/// Create a `<c:extLst>` element (`LineSerExtensionList`).
pub fn line_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`LineSerExtension`).
pub fn line_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:extLst>` element (`ScatterSerExtensionList`).
pub fn scatter_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`ScatterSerExtension`).
pub fn scatter_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:extLst>` element (`RadarSerExtensionList`).
pub fn radar_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`RadarSerExtension`).
pub fn radar_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:extLst>` element (`BarSerExtensionList`).
pub fn bar_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`BarSerExtension`).
pub fn bar_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:extLst>` element (`AreaSerExtensionList`).
pub fn area_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`AreaSerExtension`).
pub fn area_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:extLst>` element (`PieSerExtensionList`).
pub fn pie_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`PieSerExtension`).
pub fn pie_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:extLst>` element (`BubbleSerExtensionList`).
pub fn bubble_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`BubbleSerExtension`).
pub fn bubble_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:extLst>` element (`SurfaceSerExtensionList`).
pub fn surface_ser_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`SurfaceSerExtension`).
pub fn surface_ser_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:ext>` element (`DataDisplayOptions16`).
pub fn data_display_options16(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<c:pivotFmts>` element (`PivotFormats`).
pub fn pivot_formats(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pivotFmts").with_children(children)
}

/// Create a `<c:view3D>` element (`View3D`).
pub fn view3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "view3D").with_children(children)
}

/// Create a `<c:floor>` element (`Floor`).
pub fn floor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "floor").with_children(children)
}

/// Create a `<c:sideWall>` element (`SideWall`).
pub fn side_wall(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "sideWall").with_children(children)
}

/// Create a `<c:backWall>` element (`BackWall`).
pub fn back_wall(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "backWall").with_children(children)
}

/// Create a `<c:plotArea>` element (`PlotArea`).
pub fn plot_area(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "plotArea").with_children(children)
}

/// Create a `<c:legend>` element (`Legend`).
pub fn legend(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "legend").with_children(children)
}

/// Create a `<c:dispBlanksAs>` element (`DisplayBlanksAs`).
pub fn display_blanks_as() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "dispBlanksAs")
}

/// Create a `<c:extLst>` element (`ChartExtensionList`).
pub fn chart_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:lang>` element (`EditingLanguage`).
pub fn editing_language() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "lang")
}

/// Create a `<c:style>` element (`Style`).
pub fn style() -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "style")
}

/// Create a `<c:clrMapOvr>` element (`ColorMapOverride`).
pub fn color_map_override(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "clrMapOvr").with_children(children)
}

/// Create a `<c:pivotSource>` element (`PivotSource`).
pub fn pivot_source(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "pivotSource").with_children(children)
}

/// Create a `<c:protection>` element (`Protection`).
pub fn protection(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "protection").with_children(children)
}

/// Create a `<c:chart>` element (`Chart`).
pub fn chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "chart").with_children(children)
}

/// Create a `<c:externalData>` element (`ExternalData`).
pub fn external_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "externalData").with_children(children)
}

/// Create a `<c:printSettings>` element (`PrintSettings`).
pub fn print_settings(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "printSettings").with_children(children)
}

/// Create a `<c:extLst>` element (`ChartSpaceExtensionList`).
pub fn chart_space_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<c:ext>` element (`ChartSpaceExtension`).
pub fn chart_space_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c", NAMESPACE_URI, "ext").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 324;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 306;
