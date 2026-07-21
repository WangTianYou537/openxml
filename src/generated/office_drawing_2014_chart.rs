//! Auto-generated from `schemas_microsoft_com_office_drawing_2014_chart.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2014/chart` (prefix `c16`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2014/chart";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "c16";

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
static ATTRS_UNSIGNED_INTEGER_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_INVERT_IF_NEGATIVE_BOOLEAN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_BUBBLE3_D_BOOLEAN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static CHILDREN_MARKER: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MarkerStyle/c:symbol", property_name: Some("Symbol") },
    ChildInfo { name: "c:CT_MarkerSize/c:size", property_name: Some("Size") },
    ChildInfo { name: "a:CT_ChartShapeProperties/c:spPr", property_name: Some("ChartShapeProperties") },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_D_LBL: &[ChildInfo] = &[
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
static CHILDREN_CATEGORY_FILTER_EXCEPTIONS: &[ChildInfo] = &[
    ChildInfo { name: "c16:CT_CategoryFilterException/c16:categoryFilterException", property_name: None },
];
static CHILDREN_PIVOT_OPTIONS16: &[ChildInfo] = &[
    ChildInfo { name: "c16:CT_BooleanFalse/c16:showExpandCollapseFieldButtons", property_name: Some("BooleanFalse") },
];
static CHILDREN_CHART_DATA_POINT_UNIQUE_I_D_MAP: &[ChildInfo] = &[
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMapEntry/c16:ptentry", property_name: None },
];
static ATTRS_UNIQUE_ID_CHART_UNIQUE_I_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_UNIQUE_I_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_CATEGORY_FILTER_EXCEPTION: &[ChildInfo] = &[
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: Some("UniqueIdChartUniqueID") },
    ChildInfo { name: "a:CT_ShapeProperties/c16:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "c:CT_UnsignedInt/c16:explosion", property_name: Some("UnsignedIntegerType") },
    ChildInfo { name: "c:CT_Boolean/c16:invertIfNegative", property_name: Some("InvertIfNegativeBoolean") },
    ChildInfo { name: "c:CT_Boolean/c16:bubble3D", property_name: Some("Bubble3DBoolean") },
    ChildInfo { name: "c:CT_Marker/c16:marker", property_name: Some("Marker") },
    ChildInfo { name: "c:CT_DLbl/c16:dLbl", property_name: Some("DLbl") },
];
static CHILDREN_NUMBER_DATA_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "c:ST_Xstring/c:formatCode", property_name: Some("FormatCode") },
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_NumVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_NUM_FILTERED_LITERAL_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_NumData/c16:numCache", property_name: Some("NumberDataType") },
];
static CHILDREN_STRING_DATA_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_StrVal/c:pt", property_name: None },
    ChildInfo { name: "c:CT_StrDataExtensionList/c:extLst", property_name: None },
];
static CHILDREN_STR_FILTERED_LITERAL_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_StrData/c16:strCache", property_name: Some("StringDataType") },
];
static CHILDREN_MULTI_LVL_STR_DATA: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/c:ptCount", property_name: Some("PointCount") },
    ChildInfo { name: "c:CT_Lvl/c:lvl", property_name: None },
    ChildInfo { name: "c:CT_ExtensionList/c:extLst", property_name: None },
];
static CHILDREN_MULTI_LVL_STR_FILTERED_LITERAL_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_MultiLvlStrData/c16:multiLvlStrCache", property_name: Some("MultiLvlStrData") },
];
static ATTRS_LITERAL_DATA_CHART: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_BOOLEAN_FALSE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_CHART_DATA_POINT_UNIQUE_I_D_MAP_ENTRY: &[ChildInfo] = &[
    ChildInfo { name: "xsd:unsignedInt/c16:ptidx", property_name: Some("XsdunsignedInt") },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueID", property_name: Some("UniqueID") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "UnsignedIntegerType", local_name: "explosion", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNSIGNED_INTEGER_TYPE, children: &[] },
    ElementInfo { class_name: "InvertIfNegativeBoolean", local_name: "invertIfNegative", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INVERT_IF_NEGATIVE_BOOLEAN, children: &[] },
    ElementInfo { class_name: "Bubble3DBoolean", local_name: "bubble3D", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUBBLE3_D_BOOLEAN, children: &[] },
    ElementInfo { class_name: "Marker", local_name: "marker", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MARKER },
    ElementInfo { class_name: "DLbl", local_name: "dLbl", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_D_LBL },
    ElementInfo { class_name: "CategoryFilterExceptions", local_name: "categoryFilterExceptions", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CATEGORY_FILTER_EXCEPTIONS },
    ElementInfo { class_name: "PivotOptions16", local_name: "pivotOptions16", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_OPTIONS16 },
    ElementInfo { class_name: "ChartDataPointUniqueIDMap", local_name: "datapointuniqueidmap", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART_DATA_POINT_UNIQUE_I_D_MAP },
    ElementInfo { class_name: "UniqueIdChartUniqueID", local_name: "uniqueId", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNIQUE_ID_CHART_UNIQUE_I_D, children: &[] },
    ElementInfo { class_name: "UniqueID", local_name: "uniqueID", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNIQUE_I_D, children: &[] },
    ElementInfo { class_name: "CategoryFilterException", local_name: "categoryFilterException", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CATEGORY_FILTER_EXCEPTION },
    ElementInfo { class_name: "NumberDataType", local_name: "numCache", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUMBER_DATA_TYPE },
    ElementInfo { class_name: "NumFilteredLiteralCache", local_name: "filteredLitCache", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUM_FILTERED_LITERAL_CACHE },
    ElementInfo { class_name: "StringDataType", local_name: "strCache", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STRING_DATA_TYPE },
    ElementInfo { class_name: "StrFilteredLiteralCache", local_name: "filteredLitCache", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STR_FILTERED_LITERAL_CACHE },
    ElementInfo { class_name: "MultiLvlStrData", local_name: "multiLvlStrCache", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MULTI_LVL_STR_DATA },
    ElementInfo { class_name: "MultiLvlStrFilteredLiteralCache", local_name: "filteredLitCache", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MULTI_LVL_STR_FILTERED_LITERAL_CACHE },
    ElementInfo { class_name: "LiteralDataChart", local_name: "literalDataChart", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LITERAL_DATA_CHART, children: &[] },
    ElementInfo { class_name: "BooleanFalse", local_name: "showExpandCollapseFieldButtons", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOOLEAN_FALSE, children: &[] },
    ElementInfo { class_name: "XsdunsignedInt", local_name: "ptidx", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ChartDataPointUniqueIDMapEntry", local_name: "ptentry", prefix: "c16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART_DATA_POINT_UNIQUE_I_D_MAP_ENTRY },
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

/// Create a `<c16:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<c16:explosion>` element (`UnsignedIntegerType`).
pub fn unsigned_integer_type() -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "explosion")
}

/// Create a `<c16:invertIfNegative>` element (`InvertIfNegativeBoolean`).
pub fn invert_if_negative_boolean() -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "invertIfNegative")
}

/// Create a `<c16:bubble3D>` element (`Bubble3DBoolean`).
pub fn bubble3_d_boolean() -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "bubble3D")
}

/// Create a `<c16:marker>` element (`Marker`).
pub fn marker(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "marker").with_children(children)
}

/// Create a `<c16:dLbl>` element (`DLbl`).
pub fn d_lbl(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "dLbl").with_children(children)
}

/// Create a `<c16:categoryFilterExceptions>` element (`CategoryFilterExceptions`).
pub fn category_filter_exceptions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "categoryFilterExceptions").with_children(children)
}

/// Create a `<c16:pivotOptions16>` element (`PivotOptions16`).
pub fn pivot_options16(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "pivotOptions16").with_children(children)
}

/// Create a `<c16:datapointuniqueidmap>` element (`ChartDataPointUniqueIDMap`).
pub fn chart_data_point_unique_i_d_map(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "datapointuniqueidmap").with_children(children)
}

/// Create a `<c16:uniqueId>` element (`UniqueIdChartUniqueID`).
pub fn unique_id_chart_unique_i_d() -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "uniqueId")
}

/// Create a `<c16:uniqueID>` element (`UniqueID`).
pub fn unique_i_d() -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "uniqueID")
}

/// Create a `<c16:categoryFilterException>` element (`CategoryFilterException`).
pub fn category_filter_exception(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "categoryFilterException").with_children(children)
}

/// Create a `<c16:numCache>` element (`NumberDataType`).
pub fn number_data_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "numCache").with_children(children)
}

/// Create a `<c16:filteredLitCache>` element (`NumFilteredLiteralCache`).
pub fn num_filtered_literal_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "filteredLitCache").with_children(children)
}

/// Create a `<c16:strCache>` element (`StringDataType`).
pub fn string_data_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "strCache").with_children(children)
}

/// Create a `<c16:filteredLitCache>` element (`StrFilteredLiteralCache`).
pub fn str_filtered_literal_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "filteredLitCache").with_children(children)
}

/// Create a `<c16:multiLvlStrCache>` element (`MultiLvlStrData`).
pub fn multi_lvl_str_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "multiLvlStrCache").with_children(children)
}

/// Create a `<c16:filteredLitCache>` element (`MultiLvlStrFilteredLiteralCache`).
pub fn multi_lvl_str_filtered_literal_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "filteredLitCache").with_children(children)
}

/// Create a `<c16:literalDataChart>` element (`LiteralDataChart`).
pub fn literal_data_chart() -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "literalDataChart")
}

/// Create a `<c16:showExpandCollapseFieldButtons>` element (`BooleanFalse`).
pub fn boolean_false() -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "showExpandCollapseFieldButtons")
}

/// Create a `<c16:ptidx>` element (`XsdunsignedInt`).
pub fn xsdunsigned_int(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "ptidx").with_text(value)
}

/// Create a `<c16:ptentry>` element (`ChartDataPointUniqueIDMapEntry`).
pub fn chart_data_point_unique_i_d_map_entry(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16", NAMESPACE_URI, "ptentry").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 24;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 22;
