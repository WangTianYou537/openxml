//! Auto-generated from `schemas_microsoft_com_office_drawing_2012_chart.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2012/chart` (prefix `c15`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2012/chart";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "c15";

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

static CHILDREN_PIVOT_SOURCE: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:name", property_name: Some("PivotTableName") },
    ChildInfo { name: "c:CT_UnsignedInt/c:fmtId", property_name: Some("FormatId") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_NUMBERING_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":formatCode", property_name: Some("FormatCode"), type_name: "StringValue" },
    AttributeInfo { qname: ":sourceLinked", property_name: Some("SourceLinked"), type_name: "BooleanValue" },
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
static CHILDREN_LAYOUT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ManualLayout/c:manualLayout", property_name: Some("ManualLayout") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_FULL_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c15:sqref", property_name: Some("SequenceOfReferences") },
];
static CHILDREN_LEVEL_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c15:sqref", property_name: Some("SequenceOfReferences") },
];
static CHILDREN_FORMULA_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c15:sqref", property_name: Some("SequenceOfReferences") },
];
static CHILDREN_FILTERED_SERIES_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_Tx/c15:tx", property_name: Some("ChartText") },
];
static CHILDREN_FILTERED_CATEGORY_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_AxDataSource/c15:cat", property_name: Some("AxisDataSourceType") },
];
static CHILDREN_FILTERED_AREA_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_AreaSer/c15:ser", property_name: Some("AreaChartSeries") },
];
static CHILDREN_FILTERED_BAR_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BarSer/c15:ser", property_name: Some("BarChartSeries") },
];
static CHILDREN_FILTERED_BUBBLE_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_BubbleSer/c15:ser", property_name: Some("BubbleChartSeries") },
];
static CHILDREN_FILTERED_LINE_SERIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_LineSer/c15:ser", property_name: Some("LineChartSeries") },
];
static CHILDREN_FILTERED_PIE_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_PieSer/c15:ser", property_name: Some("PieChartSeries") },
];
static CHILDREN_FILTERED_RADAR_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_RadarSer/c15:ser", property_name: Some("RadarChartSeries") },
];
static CHILDREN_FILTERED_SCATTER_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_ScatterSer/c15:ser", property_name: Some("ScatterChartSeries") },
];
static CHILDREN_FILTERED_SURFACE_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_SurfaceSer/c15:ser", property_name: Some("SurfaceChartSeries") },
];
static CHILDREN_DATA_LABELS_RANGE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c15:f", property_name: Some("Formula") },
    ChildInfo { name: "c:CT_StrData/c15:dlblRangeCache", property_name: Some("DataLabelsRangeChache") },
];
static CHILDREN_CATEGORY_FILTER_EXCEPTIONS: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_CategoryFilterException/c15:categoryFilterException", property_name: None },
];
static CHILDREN_DATA_LABEL_FIELD_TABLE: &[ChildInfo] = &[
    ChildInfo { name: "c15:CT_DataLabelFieldTableEntry/c15:dlblFTEntry", property_name: None },
];
static ATTRS_EXCEPTION_FOR_SAVE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_DATA_LABELS_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_LEADER_LINES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_AUTO_GENENERATED_CATEGORIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_INVERT_IF_NEGATIVE_BOOLEAN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_BUBBLE3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static CHILDREN_CHART_TEXT: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StrRef/c:strRef", property_name: Some("StringReference") },
    ChildInfo { name: "a:CT_TextBody/c:rich", property_name: Some("RichText") },
    ChildInfo { name: "c:CT_StrData/c:strLit", property_name: Some("StringLiteral") },
];
static CHILDREN_LEADER_LINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
];
static CHILDREN_AXIS_DATA_SOURCE_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MultiLvlStrRef/c:multiLvlStrRef", property_name: Some("MultiLevelStringReference") },
    ChildInfo { name: "c:CT_NumRef/c:numRef", property_name: Some("NumberReference") },
    ChildInfo { name: "c:CT_NumData/c:numLit", property_name: Some("NumberLiteral") },
    ChildInfo { name: "c:CT_StrRef/c:strRef", property_name: Some("StringReference") },
    ChildInfo { name: "c:CT_StrData/c:strLit", property_name: Some("StringLiteral") },
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
static CHILDREN_DATA_LABELS_RANGE_CHACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_StrVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_StrDataExtensionList/c:extLst", property_name: None },
];
static CHILDREN_DATA_LABEL_FIELD_TABLE_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_StrVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_StrDataExtensionList/c:extLst", property_name: None },
];
static ATTRS_EXPLOSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static CHILDREN_MARKER: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MarkerStyle/c:symbol", property_name: Some("Symbol") },
    ChildInfo { name: "c:CT_MarkerSize/c:size", property_name: Some("Size") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
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
static CHILDREN_CATEGORY_FILTER_EXCEPTION: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c15:sqref", property_name: Some("SequenceOfReferences") },
    ChildInfo { name: "a:CT_ShapeProperties/c15:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_UnsignedInt/c15:explosion", property_name: Some("Explosion") },
    ChildInfo { name: "c:CT_Boolean/c15:invertIfNegative", property_name: Some("InvertIfNegativeBoolean") },
    ChildInfo { name: "c:CT_Boolean/c15:bubble3D", property_name: Some("Bubble3D") },
    ChildInfo { name: "c:CT_Marker/c15:marker", property_name: Some("Marker") },
    ChildInfo { name: "c:CT_DLbl/c15:dLbl", property_name: Some("DataLabel") },
];
static CHILDREN_DATA_LABEL_FIELD_TABLE_ENTRY: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/c15:txfldGUID", property_name: Some("TextFieldGuid") },
    ChildInfo { name: "xsd:string/c15:f", property_name: Some("Formula") },
    ChildInfo { name: "c:CT_StrData/c15:dlblFieldTableCache", property_name: Some("DataLabelFieldTableCache") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "PivotSource", local_name: "pivotSource", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_SOURCE },
    ElementInfo { class_name: "NumberingFormat", local_name: "numFmt", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NUMBERING_FORMAT, children: &[] },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Layout", local_name: "layout", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LAYOUT },
    ElementInfo { class_name: "FullReference", local_name: "fullRef", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FULL_REFERENCE },
    ElementInfo { class_name: "LevelReference", local_name: "levelRef", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LEVEL_REFERENCE },
    ElementInfo { class_name: "FormulaReference", local_name: "formulaRef", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FORMULA_REFERENCE },
    ElementInfo { class_name: "FilteredSeriesTitle", local_name: "filteredSeriesTitle", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_SERIES_TITLE },
    ElementInfo { class_name: "FilteredCategoryTitle", local_name: "filteredCategoryTitle", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_CATEGORY_TITLE },
    ElementInfo { class_name: "FilteredAreaSeries", local_name: "filteredAreaSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_AREA_SERIES },
    ElementInfo { class_name: "FilteredBarSeries", local_name: "filteredBarSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_BAR_SERIES },
    ElementInfo { class_name: "FilteredBubbleSeries", local_name: "filteredBubbleSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_BUBBLE_SERIES },
    ElementInfo { class_name: "FilteredLineSeriesExtension", local_name: "filteredLineSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_LINE_SERIES_EXTENSION },
    ElementInfo { class_name: "FilteredPieSeries", local_name: "filteredPieSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_PIE_SERIES },
    ElementInfo { class_name: "FilteredRadarSeries", local_name: "filteredRadarSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_RADAR_SERIES },
    ElementInfo { class_name: "FilteredScatterSeries", local_name: "filteredScatterSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_SCATTER_SERIES },
    ElementInfo { class_name: "FilteredSurfaceSeries", local_name: "filteredSurfaceSeries", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILTERED_SURFACE_SERIES },
    ElementInfo { class_name: "DataLabelsRange", local_name: "datalabelsRange", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABELS_RANGE },
    ElementInfo { class_name: "CategoryFilterExceptions", local_name: "categoryFilterExceptions", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CATEGORY_FILTER_EXCEPTIONS },
    ElementInfo { class_name: "DataLabelFieldTable", local_name: "dlblFieldTable", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABEL_FIELD_TABLE },
    ElementInfo { class_name: "ExceptionForSave", local_name: "xForSave", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXCEPTION_FOR_SAVE, children: &[] },
    ElementInfo { class_name: "ShowDataLabelsRange", local_name: "showDataLabelsRange", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_DATA_LABELS_RANGE, children: &[] },
    ElementInfo { class_name: "ShowLeaderLines", local_name: "showLeaderLines", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_LEADER_LINES, children: &[] },
    ElementInfo { class_name: "AutoGeneneratedCategories", local_name: "autoCat", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AUTO_GENENERATED_CATEGORIES, children: &[] },
    ElementInfo { class_name: "InvertIfNegativeBoolean", local_name: "invertIfNegative", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INVERT_IF_NEGATIVE_BOOLEAN, children: &[] },
    ElementInfo { class_name: "Bubble3D", local_name: "bubble3D", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUBBLE3_D, children: &[] },
    ElementInfo { class_name: "ChartText", local_name: "tx", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART_TEXT },
    ElementInfo { class_name: "LeaderLines", local_name: "leaderLines", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LEADER_LINES },
    ElementInfo { class_name: "SequenceOfReferences", local_name: "sqref", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Formula", local_name: "f", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "TextFieldGuid", local_name: "txfldGUID", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AxisDataSourceType", local_name: "cat", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AXIS_DATA_SOURCE_TYPE },
    ElementInfo { class_name: "BarChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAR_CHART_SERIES },
    ElementInfo { class_name: "LineChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_CHART_SERIES },
    ElementInfo { class_name: "ScatterChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCATTER_CHART_SERIES },
    ElementInfo { class_name: "AreaChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AREA_CHART_SERIES },
    ElementInfo { class_name: "PieChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIE_CHART_SERIES },
    ElementInfo { class_name: "BubbleChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUBBLE_CHART_SERIES },
    ElementInfo { class_name: "RadarChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RADAR_CHART_SERIES },
    ElementInfo { class_name: "SurfaceChartSeries", local_name: "ser", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURFACE_CHART_SERIES },
    ElementInfo { class_name: "DataLabelsRangeChache", local_name: "dlblRangeCache", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABELS_RANGE_CHACHE },
    ElementInfo { class_name: "DataLabelFieldTableCache", local_name: "dlblFieldTableCache", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABEL_FIELD_TABLE_CACHE },
    ElementInfo { class_name: "Explosion", local_name: "explosion", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXPLOSION, children: &[] },
    ElementInfo { class_name: "Marker", local_name: "marker", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MARKER },
    ElementInfo { class_name: "DataLabel", local_name: "dLbl", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABEL },
    ElementInfo { class_name: "CategoryFilterException", local_name: "categoryFilterException", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CATEGORY_FILTER_EXCEPTION },
    ElementInfo { class_name: "DataLabelFieldTableEntry", local_name: "dlblFTEntry", prefix: "c15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_LABEL_FIELD_TABLE_ENTRY },
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

/// Create a `<c15:pivotSource>` element (`PivotSource`).
pub fn pivot_source(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "pivotSource").with_children(children)
}

/// Create a `<c15:numFmt>` element (`NumberingFormat`).
pub fn numbering_format() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "numFmt")
}

/// Create a `<c15:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<c15:layout>` element (`Layout`).
pub fn layout(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "layout").with_children(children)
}

/// Create a `<c15:fullRef>` element (`FullReference`).
pub fn full_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "fullRef").with_children(children)
}

/// Create a `<c15:levelRef>` element (`LevelReference`).
pub fn level_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "levelRef").with_children(children)
}

/// Create a `<c15:formulaRef>` element (`FormulaReference`).
pub fn formula_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "formulaRef").with_children(children)
}

/// Create a `<c15:filteredSeriesTitle>` element (`FilteredSeriesTitle`).
pub fn filtered_series_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredSeriesTitle").with_children(children)
}

/// Create a `<c15:filteredCategoryTitle>` element (`FilteredCategoryTitle`).
pub fn filtered_category_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredCategoryTitle").with_children(children)
}

/// Create a `<c15:filteredAreaSeries>` element (`FilteredAreaSeries`).
pub fn filtered_area_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredAreaSeries").with_children(children)
}

/// Create a `<c15:filteredBarSeries>` element (`FilteredBarSeries`).
pub fn filtered_bar_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredBarSeries").with_children(children)
}

/// Create a `<c15:filteredBubbleSeries>` element (`FilteredBubbleSeries`).
pub fn filtered_bubble_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredBubbleSeries").with_children(children)
}

/// Create a `<c15:filteredLineSeries>` element (`FilteredLineSeriesExtension`).
pub fn filtered_line_series_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredLineSeries").with_children(children)
}

/// Create a `<c15:filteredPieSeries>` element (`FilteredPieSeries`).
pub fn filtered_pie_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredPieSeries").with_children(children)
}

/// Create a `<c15:filteredRadarSeries>` element (`FilteredRadarSeries`).
pub fn filtered_radar_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredRadarSeries").with_children(children)
}

/// Create a `<c15:filteredScatterSeries>` element (`FilteredScatterSeries`).
pub fn filtered_scatter_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredScatterSeries").with_children(children)
}

/// Create a `<c15:filteredSurfaceSeries>` element (`FilteredSurfaceSeries`).
pub fn filtered_surface_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "filteredSurfaceSeries").with_children(children)
}

/// Create a `<c15:datalabelsRange>` element (`DataLabelsRange`).
pub fn data_labels_range(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "datalabelsRange").with_children(children)
}

/// Create a `<c15:categoryFilterExceptions>` element (`CategoryFilterExceptions`).
pub fn category_filter_exceptions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "categoryFilterExceptions").with_children(children)
}

/// Create a `<c15:dlblFieldTable>` element (`DataLabelFieldTable`).
pub fn data_label_field_table(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "dlblFieldTable").with_children(children)
}

/// Create a `<c15:xForSave>` element (`ExceptionForSave`).
pub fn exception_for_save() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "xForSave")
}

/// Create a `<c15:showDataLabelsRange>` element (`ShowDataLabelsRange`).
pub fn show_data_labels_range() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "showDataLabelsRange")
}

/// Create a `<c15:showLeaderLines>` element (`ShowLeaderLines`).
pub fn show_leader_lines() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "showLeaderLines")
}

/// Create a `<c15:autoCat>` element (`AutoGeneneratedCategories`).
pub fn auto_genenerated_categories() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "autoCat")
}

/// Create a `<c15:invertIfNegative>` element (`InvertIfNegativeBoolean`).
pub fn invert_if_negative_boolean() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "invertIfNegative")
}

/// Create a `<c15:bubble3D>` element (`Bubble3D`).
pub fn bubble3_d() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "bubble3D")
}

/// Create a `<c15:tx>` element (`ChartText`).
pub fn chart_text(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "tx").with_children(children)
}

/// Create a `<c15:leaderLines>` element (`LeaderLines`).
pub fn leader_lines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "leaderLines").with_children(children)
}

/// Create a `<c15:sqref>` element (`SequenceOfReferences`).
pub fn sequence_of_references(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "sqref").with_text(value)
}

/// Create a `<c15:f>` element (`Formula`).
pub fn formula(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "f").with_text(value)
}

/// Create a `<c15:txfldGUID>` element (`TextFieldGuid`).
pub fn text_field_guid(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "txfldGUID").with_text(value)
}

/// Create a `<c15:cat>` element (`AxisDataSourceType`).
pub fn axis_data_source_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "cat").with_children(children)
}

/// Create a `<c15:ser>` element (`BarChartSeries`).
pub fn bar_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:ser>` element (`LineChartSeries`).
pub fn line_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:ser>` element (`ScatterChartSeries`).
pub fn scatter_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:ser>` element (`AreaChartSeries`).
pub fn area_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:ser>` element (`PieChartSeries`).
pub fn pie_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:ser>` element (`BubbleChartSeries`).
pub fn bubble_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:ser>` element (`RadarChartSeries`).
pub fn radar_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:ser>` element (`SurfaceChartSeries`).
pub fn surface_chart_series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "ser").with_children(children)
}

/// Create a `<c15:dlblRangeCache>` element (`DataLabelsRangeChache`).
pub fn data_labels_range_chache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "dlblRangeCache").with_children(children)
}

/// Create a `<c15:dlblFieldTableCache>` element (`DataLabelFieldTableCache`).
pub fn data_label_field_table_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "dlblFieldTableCache").with_children(children)
}

/// Create a `<c15:explosion>` element (`Explosion`).
pub fn explosion() -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "explosion")
}

/// Create a `<c15:marker>` element (`Marker`).
pub fn marker(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "marker").with_children(children)
}

/// Create a `<c15:dLbl>` element (`DataLabel`).
pub fn data_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "dLbl").with_children(children)
}

/// Create a `<c15:categoryFilterException>` element (`CategoryFilterException`).
pub fn category_filter_exception(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "categoryFilterException").with_children(children)
}

/// Create a `<c15:dlblFTEntry>` element (`DataLabelFieldTableEntry`).
pub fn data_label_field_table_entry(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c15", NAMESPACE_URI, "dlblFTEntry").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 49;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 47;
