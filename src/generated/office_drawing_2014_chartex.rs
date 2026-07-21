//! Auto-generated from `schemas_microsoft_com_office_drawing_2014_chartex.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2014/chartex` (prefix `cx`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2014/chartex";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "cx";

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

static ATTRS_CHART_SPACE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":version", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":featureList", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":fallbackImg", property_name: None, type_name: "StringValue" },
];
static CHILDREN_CHART_SPACE: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ChartData/cx:chartData", property_name: Some("ChartData") },
    ChildInfo { name: "cx:CT_Chart/cx:chart", property_name: Some("Chart") },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: Some("TxPrTextBody") },
    ChildInfo { name: "a:CT_ColorMapping/cx:clrMapOvr", property_name: Some("ColorMappingType") },
    ChildInfo { name: "cx:CT_FormatOverrides/cx:fmtOvrs", property_name: Some("FormatOverrides") },
    ChildInfo { name: "cx:CT_PrintSettings/cx:printSettings", property_name: Some("PrintSettings") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_REL_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_EXTENSION2: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: Some("Uri"), type_name: "StringValue" },
];
static CHILDREN_MIN_COLOR_SOLID_COLOR_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_MID_COLOR_SOLID_COLOR_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_MAX_COLOR_SOLID_COLOR_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_CHART_STRING_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static ATTRS_FORMULA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: None, type_name: "EnumValue" },
];
static ATTRS_NF_FORMULA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: None, type_name: "EnumValue" },
];
static ATTRS_STRING_LEVEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ptCount", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
];
static CHILDREN_STRING_LEVEL: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_StringValue/cx:pt", property_name: None },
];
static ATTRS_NUMERIC_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_NUMERIC_LEVEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ptCount", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":formatCode", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
];
static CHILDREN_NUMERIC_LEVEL: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_NumericValue/cx:pt", property_name: None },
];
static ATTRS_NUMERIC_DIMENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_NUMERIC_DIMENSION: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Formula/cx:f", property_name: None },
    ChildInfo { name: "cx:CT_Formula/cx:nf", property_name: None },
    ChildInfo { name: "cx:CT_NumericLevel/cx:lvl", property_name: None },
];
static ATTRS_STRING_DIMENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_STRING_DIMENSION: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Formula/cx:f", property_name: None },
    ChildInfo { name: "cx:CT_Formula/cx:nf", property_name: None },
    ChildInfo { name: "cx:CT_StringLevel/cx:lvl", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Extension/cx:ext", property_name: None },
];
static ATTRS_EXTERNAL_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: "cx:autoUpdate", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_DATA: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_NumericDimension/cx:numDim", property_name: None },
    ChildInfo { name: "cx:CT_StringDimension/cx:strDim", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static CHILDREN_TEXT_DATA: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Formula/cx:f", property_name: None },
    ChildInfo { name: "xsd:string/cx:v", property_name: None },
];
static CHILDREN_RICH_TEXT_BODY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
];
static CHILDREN_TX_PR_TEXT_BODY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
];
static CHILDREN_TEXT: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_TextData/cx:txData", property_name: Some("TextData") },
    ChildInfo { name: "a:CT_TextBody/cx:rich", property_name: Some("RichTextBody") },
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
static ATTRS_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":top", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":left", property_name: None, type_name: "DoubleValue" },
];
static CHILDREN_AXIS_UNITS_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Text/cx:tx", property_name: Some("Text") },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: Some("TxPrTextBody") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_CATEGORY_AXIS_SCALING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":gapWidth", property_name: None, type_name: "StringValue" },
];
static ATTRS_VALUE_AXIS_SCALING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":max", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":min", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":majorUnit", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":minorUnit", property_name: None, type_name: "StringValue" },
];
static CHILDREN_AXIS_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Text/cx:tx", property_name: Some("Text") },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: Some("TxPrTextBody") },
    ChildInfo { name: "cx:CT_Offset/cx:offset", property_name: Some("Offset") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_AXIS_UNITS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":unit", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_AXIS_UNITS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_AxisUnitsLabel/cx:unitsLabel", property_name: Some("AxisUnitsLabel") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_MAJOR_GRIDLINES_GRIDLINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_MINOR_GRIDLINES_GRIDLINES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_MAJOR_TICK_MARKS_TICK_MARKS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_MAJOR_TICK_MARKS_TICK_MARKS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_MINOR_TICK_MARKS_TICK_MARKS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_MINOR_TICK_MARKS_TICK_MARKS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_TICK_LABELS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_NUMBER_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":formatCode", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sourceLinked", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_ADDRESS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":address1", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":countryRegion", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":adminDistrict1", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":adminDistrict2", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":postalCode", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":locality", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":isoCountryCode", property_name: None, type_name: "StringValue" },
];
static ATTRS_GEO_LOCATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":latitude", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":longitude", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":entityName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":entityType", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_GEO_LOCATION: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Address/cx:address", property_name: Some("Address") },
];
static ATTRS_GEO_LOCATION_QUERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":countryRegion", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":adminDistrict1", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":adminDistrict2", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":postalCode", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":entityType", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_GEO_LOCATIONS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoLocation/cx:geoLocation", property_name: Some("GeoLocation") },
];
static CHILDREN_GEO_LOCATION_QUERY_RESULT: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoLocationQuery/cx:geoLocationQuery", property_name: Some("GeoLocationQuery") },
    ChildInfo { name: "cx:CT_GeoLocations/cx:geoLocations", property_name: Some("GeoLocations") },
];
static ATTRS_GEO_POLYGON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":polygonId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":numPoints", property_name: None, type_name: "IntegerValue" },
    AttributeInfo { qname: ":pcaRings", property_name: None, type_name: "StringValue" },
];
static CHILDREN_GEO_POLYGONS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoPolygon/cx:geoPolygon", property_name: None },
];
static CHILDREN_COPYRIGHTS: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/cx:copyright", property_name: None },
];
static ATTRS_GEO_DATA_ENTITY_QUERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":entityId", property_name: None, type_name: "StringValue" },
];
static ATTRS_GEO_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":entityId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":east", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":west", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":north", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":south", property_name: None, type_name: "DoubleValue" },
];
static CHILDREN_GEO_DATA: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoPolygons/cx:geoPolygons", property_name: Some("GeoPolygons") },
    ChildInfo { name: "cx:CT_Copyrights/cx:copyrights", property_name: Some("Copyrights") },
];
static CHILDREN_GEO_DATA_ENTITY_QUERY_RESULT: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoDataEntityQuery/cx:geoDataEntityQuery", property_name: Some("GeoDataEntityQuery") },
    ChildInfo { name: "cx:CT_GeoData/cx:geoData", property_name: Some("GeoData") },
];
static ATTRS_GEO_DATA_POINT_QUERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":latitude", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":longitude", property_name: None, type_name: "DoubleValue" },
];
static ATTRS_GEO_DATA_POINT_TO_ENTITY_QUERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":entityId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_GEO_DATA_POINT_TO_ENTITY_QUERY_RESULT: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoDataPointQuery/cx:geoDataPointQuery", property_name: Some("GeoDataPointQuery") },
    ChildInfo { name: "cx:CT_GeoDataPointToEntityQuery/cx:geoDataPointToEntityQuery", property_name: Some("GeoDataPointToEntityQuery") },
];
static CHILDREN_GEO_CHILD_TYPES: &[ChildInfo] = &[
    ChildInfo { name: "cx:ST_EntityType/cx:entityType", property_name: None },
];
static ATTRS_GEO_HIERARCHY_ENTITY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":entityId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":entityType", property_name: None, type_name: "EnumValue" },
];
static ATTRS_GEO_CHILD_ENTITIES_QUERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_GEO_CHILD_ENTITIES_QUERY: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoChildTypes/cx:geoChildTypes", property_name: Some("GeoChildTypes") },
];
static CHILDREN_GEO_CHILD_ENTITIES: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoHierarchyEntity/cx:geoHierarchyEntity", property_name: None },
];
static CHILDREN_GEO_CHILD_ENTITIES_QUERY_RESULT: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoChildEntitiesQuery/cx:geoChildEntitiesQuery", property_name: Some("GeoChildEntitiesQuery") },
    ChildInfo { name: "cx:CT_GeoChildEntities/cx:geoChildEntities", property_name: Some("GeoChildEntities") },
];
static ATTRS_GEO_PARENT_ENTITIES_QUERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityId", property_name: None, type_name: "StringValue" },
];
static ATTRS_GEO_ENTITY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":entityType", property_name: None, type_name: "EnumValue" },
];
static ATTRS_GEO_PARENT_ENTITY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":entityId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_GEO_PARENT_ENTITIES_QUERY_RESULT: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoParentEntitiesQuery/cx:geoParentEntitiesQuery", property_name: Some("GeoParentEntitiesQuery") },
    ChildInfo { name: "cx:CT_GeoEntity/cx:geoEntity", property_name: Some("GeoEntity") },
    ChildInfo { name: "cx:CT_GeoParentEntity/cx:geoParentEntity", property_name: Some("GeoParentEntity") },
];
static CHILDREN_GEO_LOCATION_QUERY_RESULTS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoLocationQueryResult/cx:geoLocationQueryResult", property_name: None },
];
static CHILDREN_GEO_DATA_ENTITY_QUERY_RESULTS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoDataEntityQueryResult/cx:geoDataEntityQueryResult", property_name: None },
];
static CHILDREN_GEO_DATA_POINT_TO_ENTITY_QUERY_RESULTS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoDataPointToEntityQueryResult/cx:geoDataPointToEntityQueryResult", property_name: None },
];
static CHILDREN_GEO_CHILD_ENTITIES_QUERY_RESULTS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoChildEntitiesQueryResult/cx:geoChildEntitiesQueryResult", property_name: None },
];
static CHILDREN_GEO_PARENT_ENTITIES_QUERY_RESULTS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoParentEntitiesQueryResult/cx:geoParentEntitiesQueryResult", property_name: None },
];
static CHILDREN_CLEAR: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoLocationQueryResults/cx:geoLocationQueryResults", property_name: Some("GeoLocationQueryResults") },
    ChildInfo { name: "cx:CT_GeoDataEntityQueryResults/cx:geoDataEntityQueryResults", property_name: Some("GeoDataEntityQueryResults") },
    ChildInfo { name: "cx:CT_GeoDataPointToEntityQueryResults/cx:geoDataPointToEntityQueryResults", property_name: Some("GeoDataPointToEntityQueryResults") },
    ChildInfo { name: "cx:CT_GeoChildEntitiesQueryResults/cx:geoChildEntitiesQueryResults", property_name: Some("GeoChildEntitiesQueryResults") },
    ChildInfo { name: "cx:CT_GeoParentEntitiesQueryResults/cx:geoParentEntitiesQueryResults", property_name: Some("GeoParentEntitiesQueryResults") },
];
static ATTRS_GEO_CACHE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":provider", property_name: None, type_name: "StringValue" },
];
static CHILDREN_GEO_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:base64Binary/cx:binary", property_name: None },
    ChildInfo { name: "cx:CT_Clear/cx:clear", property_name: None },
];
static ATTRS_PARENT_LABEL_LAYOUT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("ParentLabelLayoutVal"), type_name: "EnumValue" },
];
static ATTRS_REGION_LABEL_LAYOUT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "EnumValue" },
];
static ATTRS_SERIES_ELEMENT_VISIBILITIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":connectorLines", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":meanLine", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":meanMarker", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":nonoutliers", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":outliers", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_BINNING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":intervalClosed", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":underflow", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":overflow", property_name: None, type_name: "StringValue" },
];
static CHILDREN_BINNING: &[ChildInfo] = &[
    ChildInfo { name: "xsd:double/cx:binSize", property_name: Some("Xsddouble") },
    ChildInfo { name: "xsd:unsignedInt/cx:binCount", property_name: Some("BinCountXsdunsignedInt") },
];
static ATTRS_GEOGRAPHY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":projectionType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":viewedRegionType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":cultureLanguage", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":cultureRegion", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":attribution", property_name: None, type_name: "StringValue" },
];
static CHILDREN_GEOGRAPHY: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_GeoCache/cx:geoCache", property_name: Some("GeoCache") },
];
static ATTRS_STATISTICS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":quartileMethod", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_SUBTOTALS: &[ChildInfo] = &[
    ChildInfo { name: "c:CT_UnsignedInt/cx:idx", property_name: None },
];
static ATTRS_NUMBER_COLOR_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "DoubleValue" },
];
static ATTRS_PERCENTAGE_COLOR_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "DoubleValue" },
];
static CHILDREN_MIN_VALUE_COLOR_END_POSITION: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ExtremeValueColorPosition/cx:extremeValue", property_name: Some("ExtremeValueColorPosition") },
    ChildInfo { name: "cx:CT_NumberColorPosition/cx:number", property_name: Some("NumberColorPosition") },
    ChildInfo { name: "cx:CT_PercentageColorPosition/cx:percent", property_name: Some("PercentageColorPosition") },
];
static CHILDREN_MAX_VALUE_COLOR_END_POSITION: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ExtremeValueColorPosition/cx:extremeValue", property_name: Some("ExtremeValueColorPosition") },
    ChildInfo { name: "cx:CT_NumberColorPosition/cx:number", property_name: Some("NumberColorPosition") },
    ChildInfo { name: "cx:CT_PercentageColorPosition/cx:percent", property_name: Some("PercentageColorPosition") },
];
static CHILDREN_VALUE_COLOR_MIDDLE_POSITION: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_NumberColorPosition/cx:number", property_name: Some("NumberColorPosition") },
    ChildInfo { name: "cx:CT_PercentageColorPosition/cx:percent", property_name: Some("PercentageColorPosition") },
];
static ATTRS_DATA_LABEL_VISIBILITIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":seriesName", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":categoryName", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":value", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_DATA_LABEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":pos", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_DATA_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_NumberFormat/cx:numFmt", property_name: Some("NumberFormat") },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: Some("TxPrTextBody") },
    ChildInfo { name: "cx:CT_DataLabelVisibilities/cx:visibility", property_name: Some("DataLabelVisibilities") },
    ChildInfo { name: "xsd:string/cx:separator", property_name: Some("SeparatorXsdstring") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_DATA_LABEL_HIDDEN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_VALUE_COLORS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SolidColorFillProperties/cx:minColor", property_name: Some("MinColorSolidColorFillProperties") },
    ChildInfo { name: "a:CT_SolidColorFillProperties/cx:midColor", property_name: Some("MidColorSolidColorFillProperties") },
    ChildInfo { name: "a:CT_SolidColorFillProperties/cx:maxColor", property_name: Some("MaxColorSolidColorFillProperties") },
];
static ATTRS_VALUE_COLOR_POSITIONS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "Int32Value" },
];
static CHILDREN_VALUE_COLOR_POSITIONS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ValueColorEndPosition/cx:min", property_name: Some("MinValueColorEndPosition") },
    ChildInfo { name: "cx:CT_ValueColorMiddlePosition/cx:mid", property_name: Some("ValueColorMiddlePosition") },
    ChildInfo { name: "cx:CT_ValueColorEndPosition/cx:max", property_name: Some("MaxValueColorEndPosition") },
];
static ATTRS_DATA_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_DATA_POINT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_DATA_LABELS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pos", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_DATA_LABELS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_NumberFormat/cx:numFmt", property_name: Some("NumberFormat") },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: Some("TxPrTextBody") },
    ChildInfo { name: "cx:CT_DataLabelVisibilities/cx:visibility", property_name: Some("DataLabelVisibilities") },
    ChildInfo { name: "xsd:string/cx:separator", property_name: Some("SeparatorXsdstring") },
    ChildInfo { name: "cx:CT_DataLabel/cx:dataLabel", property_name: None },
    ChildInfo { name: "cx:CT_DataLabelHidden/cx:dataLabelHidden", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static ATTRS_DATA_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SERIES_LAYOUT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ParentLabelLayout/cx:parentLabelLayout", property_name: Some("ParentLabelLayout") },
    ChildInfo { name: "cx:CT_RegionLabelLayout/cx:regionLabelLayout", property_name: Some("RegionLabelLayout") },
    ChildInfo { name: "cx:CT_SeriesElementVisibilities/cx:visibility", property_name: Some("SeriesElementVisibilities") },
    ChildInfo { name: "cx:CT_Aggregation/cx:aggregation", property_name: None },
    ChildInfo { name: "cx:CT_Binning/cx:binning", property_name: None },
    ChildInfo { name: "cx:CT_Geography/cx:geography", property_name: None },
    ChildInfo { name: "cx:CT_Statistics/cx:statistics", property_name: None },
    ChildInfo { name: "cx:CT_Subtotals/cx:subtotals", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static CHILDREN_PLOT_SURFACE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SERIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":layoutId", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":hidden", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":ownerIdx", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":uniqueId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":formatIdx", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SERIES: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Text/cx:tx", property_name: Some("Text") },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "cx:CT_ValueColors/cx:valueColors", property_name: Some("ValueColors") },
    ChildInfo { name: "cx:CT_ValueColorPositions/cx:valueColorPositions", property_name: Some("ValueColorPositions") },
    ChildInfo { name: "cx:CT_DataPoint/cx:dataPt", property_name: None },
    ChildInfo { name: "cx:CT_DataLabels/cx:dataLabels", property_name: None },
    ChildInfo { name: "cx:CT_DataId/cx:dataId", property_name: None },
    ChildInfo { name: "cx:CT_SeriesLayoutProperties/cx:layoutPr", property_name: None },
    ChildInfo { name: "cx:ST_AxisId/cx:axisId", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static CHILDREN_PLOT_AREA_REGION: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_PlotSurface/cx:plotSurface", property_name: Some("PlotSurface") },
    ChildInfo { name: "cx:CT_Series/cx:series", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static ATTRS_AXIS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":hidden", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_AXIS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_CategoryAxisScaling/cx:catScaling", property_name: None },
    ChildInfo { name: "cx:CT_ValueAxisScaling/cx:valScaling", property_name: None },
    ChildInfo { name: "cx:CT_AxisTitle/cx:title", property_name: None },
    ChildInfo { name: "cx:CT_AxisUnits/cx:units", property_name: None },
    ChildInfo { name: "cx:CT_Gridlines/cx:majorGridlines", property_name: None },
    ChildInfo { name: "cx:CT_Gridlines/cx:minorGridlines", property_name: None },
    ChildInfo { name: "cx:CT_TickMarks/cx:majorTickMarks", property_name: None },
    ChildInfo { name: "cx:CT_TickMarks/cx:minorTickMarks", property_name: None },
    ChildInfo { name: "cx:CT_TickLabels/cx:tickLabels", property_name: None },
    ChildInfo { name: "cx:CT_NumberFormat/cx:numFmt", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: None },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static ATTRS_CHART_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pos", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":align", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":overlay", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_CHART_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_Text/cx:tx", property_name: Some("Text") },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: Some("TxPrTextBody") },
    ChildInfo { name: "cx:CT_Offset/cx:offset", property_name: Some("Offset") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_PLOT_AREA: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_PlotAreaRegion/cx:plotAreaRegion", property_name: Some("PlotAreaRegion") },
    ChildInfo { name: "cx:CT_Axis/cx:axis", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static ATTRS_LEGEND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pos", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":align", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":overlay", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_LEGEND: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/cx:txPr", property_name: Some("TxPrTextBody") },
    ChildInfo { name: "cx:CT_Offset/cx:offset", property_name: Some("Offset") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FORMAT_OVERRIDE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_FORMAT_OVERRIDE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/cx:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_HEADER_FOOTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":alignWithMargins", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":differentOddEven", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":differentFirst", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_HEADER_FOOTER: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/cx:oddHeader", property_name: Some("OddHeaderXsdstring") },
    ChildInfo { name: "xsd:string/cx:oddFooter", property_name: Some("OddFooterXsdstring") },
    ChildInfo { name: "xsd:string/cx:evenHeader", property_name: Some("EvenHeaderXsdstring") },
    ChildInfo { name: "xsd:string/cx:evenFooter", property_name: Some("EvenFooterXsdstring") },
    ChildInfo { name: "xsd:string/cx:firstHeader", property_name: Some("FirstHeaderXsdstring") },
    ChildInfo { name: "xsd:string/cx:firstFooter", property_name: Some("FirstFooterXsdstring") },
];
static ATTRS_PAGE_MARGINS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":r", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":b", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":header", property_name: None, type_name: "DoubleValue" },
    AttributeInfo { qname: ":footer", property_name: None, type_name: "DoubleValue" },
];
static ATTRS_PAGE_SETUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":paperSize", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":firstPageNumber", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":orientation", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":blackAndWhite", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":draft", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":useFirstPageNumber", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":horizontalDpi", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":verticalDpi", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":copies", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_CHART_DATA: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ExternalData/cx:externalData", property_name: Some("ExternalData") },
    ChildInfo { name: "cx:CT_Data/cx:data", property_name: None },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: None },
];
static CHILDREN_CHART: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_ChartTitle/cx:title", property_name: Some("ChartTitle") },
    ChildInfo { name: "cx:CT_PlotArea/cx:plotArea", property_name: Some("PlotArea") },
    ChildInfo { name: "cx:CT_Legend/cx:legend", property_name: Some("Legend") },
    ChildInfo { name: "cx:CT_ExtensionList/cx:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_COLOR_MAPPING_TYPE: &[AttributeInfo] = &[
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
static CHILDREN_COLOR_MAPPING_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_FORMAT_OVERRIDES: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_FormatOverride/cx:fmtOvr", property_name: None },
];
static CHILDREN_PRINT_SETTINGS: &[ChildInfo] = &[
    ChildInfo { name: "cx:CT_HeaderFooter/cx:headerFooter", property_name: Some("HeaderFooter") },
    ChildInfo { name: "cx:CT_PageMargins/cx:pageMargins", property_name: Some("PageMargins") },
    ChildInfo { name: "cx:CT_PageSetup/cx:pageSetup", property_name: Some("PageSetup") },
];
static ATTRS_UNSIGNED_INTEGER_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ChartSpace", local_name: "chartSpace", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHART_SPACE, children: CHILDREN_CHART_SPACE },
    ElementInfo { class_name: "RelId", local_name: "chart", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REL_ID, children: &[] },
    ElementInfo { class_name: "Openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3XsdunsignedInt", local_name: "openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "BinCountXsdunsignedInt", local_name: "binCount", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Extension2", local_name: "ext", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTENSION2, children: &[] },
    ElementInfo { class_name: "MinColorSolidColorFillProperties", local_name: "minColor", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MIN_COLOR_SOLID_COLOR_FILL_PROPERTIES },
    ElementInfo { class_name: "MidColorSolidColorFillProperties", local_name: "midColor", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MID_COLOR_SOLID_COLOR_FILL_PROPERTIES },
    ElementInfo { class_name: "MaxColorSolidColorFillProperties", local_name: "maxColor", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MAX_COLOR_SOLID_COLOR_FILL_PROPERTIES },
    ElementInfo { class_name: "ChartStringValue", local_name: "pt", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_CHART_STRING_VALUE, children: &[] },
    ElementInfo { class_name: "Formula", local_name: "f", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_FORMULA, children: &[] },
    ElementInfo { class_name: "NfFormula", local_name: "nf", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_NF_FORMULA, children: &[] },
    ElementInfo { class_name: "StringLevel", local_name: "lvl", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STRING_LEVEL, children: CHILDREN_STRING_LEVEL },
    ElementInfo { class_name: "NumericValue", local_name: "pt", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_NUMERIC_VALUE, children: &[] },
    ElementInfo { class_name: "NumericLevel", local_name: "lvl", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NUMERIC_LEVEL, children: CHILDREN_NUMERIC_LEVEL },
    ElementInfo { class_name: "NumericDimension", local_name: "numDim", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NUMERIC_DIMENSION, children: CHILDREN_NUMERIC_DIMENSION },
    ElementInfo { class_name: "StringDimension", local_name: "strDim", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STRING_DIMENSION, children: CHILDREN_STRING_DIMENSION },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "ExternalData", local_name: "externalData", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXTERNAL_DATA, children: &[] },
    ElementInfo { class_name: "Data", local_name: "data", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA, children: CHILDREN_DATA },
    ElementInfo { class_name: "VXsdstring", local_name: "v", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CopyrightXsdstring", local_name: "copyright", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SeparatorXsdstring", local_name: "separator", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "OddHeaderXsdstring", local_name: "oddHeader", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "OddFooterXsdstring", local_name: "oddFooter", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EvenHeaderXsdstring", local_name: "evenHeader", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EvenFooterXsdstring", local_name: "evenFooter", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FirstHeaderXsdstring", local_name: "firstHeader", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FirstFooterXsdstring", local_name: "firstFooter", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "TextData", local_name: "txData", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_DATA },
    ElementInfo { class_name: "RichTextBody", local_name: "rich", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_TEXT_BODY },
    ElementInfo { class_name: "TxPrTextBody", local_name: "txPr", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TX_PR_TEXT_BODY },
    ElementInfo { class_name: "Text", local_name: "tx", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Offset", local_name: "offset", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OFFSET, children: &[] },
    ElementInfo { class_name: "AxisUnitsLabel", local_name: "unitsLabel", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AXIS_UNITS_LABEL },
    ElementInfo { class_name: "CategoryAxisScaling", local_name: "catScaling", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CATEGORY_AXIS_SCALING, children: &[] },
    ElementInfo { class_name: "ValueAxisScaling", local_name: "valScaling", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VALUE_AXIS_SCALING, children: &[] },
    ElementInfo { class_name: "AxisTitle", local_name: "title", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AXIS_TITLE },
    ElementInfo { class_name: "AxisUnits", local_name: "units", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AXIS_UNITS, children: CHILDREN_AXIS_UNITS },
    ElementInfo { class_name: "MajorGridlinesGridlines", local_name: "majorGridlines", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MAJOR_GRIDLINES_GRIDLINES },
    ElementInfo { class_name: "MinorGridlinesGridlines", local_name: "minorGridlines", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MINOR_GRIDLINES_GRIDLINES },
    ElementInfo { class_name: "MajorTickMarksTickMarks", local_name: "majorTickMarks", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MAJOR_TICK_MARKS_TICK_MARKS, children: CHILDREN_MAJOR_TICK_MARKS_TICK_MARKS },
    ElementInfo { class_name: "MinorTickMarksTickMarks", local_name: "minorTickMarks", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MINOR_TICK_MARKS_TICK_MARKS, children: CHILDREN_MINOR_TICK_MARKS_TICK_MARKS },
    ElementInfo { class_name: "TickLabels", local_name: "tickLabels", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TICK_LABELS },
    ElementInfo { class_name: "NumberFormat", local_name: "numFmt", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NUMBER_FORMAT, children: &[] },
    ElementInfo { class_name: "Xsddouble", local_name: "binSize", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Address", local_name: "address", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ADDRESS, children: &[] },
    ElementInfo { class_name: "GeoLocation", local_name: "geoLocation", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GEO_LOCATION, children: CHILDREN_GEO_LOCATION },
    ElementInfo { class_name: "GeoLocationQuery", local_name: "geoLocationQuery", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_LOCATION_QUERY, children: &[] },
    ElementInfo { class_name: "GeoLocations", local_name: "geoLocations", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_LOCATIONS },
    ElementInfo { class_name: "GeoLocationQueryResult", local_name: "geoLocationQueryResult", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_LOCATION_QUERY_RESULT },
    ElementInfo { class_name: "GeoPolygon", local_name: "geoPolygon", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_POLYGON, children: &[] },
    ElementInfo { class_name: "GeoPolygons", local_name: "geoPolygons", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_POLYGONS },
    ElementInfo { class_name: "Copyrights", local_name: "copyrights", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COPYRIGHTS },
    ElementInfo { class_name: "GeoDataEntityQuery", local_name: "geoDataEntityQuery", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_DATA_ENTITY_QUERY, children: &[] },
    ElementInfo { class_name: "GeoData", local_name: "geoData", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GEO_DATA, children: CHILDREN_GEO_DATA },
    ElementInfo { class_name: "GeoDataEntityQueryResult", local_name: "geoDataEntityQueryResult", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_DATA_ENTITY_QUERY_RESULT },
    ElementInfo { class_name: "GeoDataPointQuery", local_name: "geoDataPointQuery", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_DATA_POINT_QUERY, children: &[] },
    ElementInfo { class_name: "GeoDataPointToEntityQuery", local_name: "geoDataPointToEntityQuery", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_DATA_POINT_TO_ENTITY_QUERY, children: &[] },
    ElementInfo { class_name: "GeoDataPointToEntityQueryResult", local_name: "geoDataPointToEntityQueryResult", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_DATA_POINT_TO_ENTITY_QUERY_RESULT },
    ElementInfo { class_name: "EntityType", local_name: "entityType", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "GeoChildTypes", local_name: "geoChildTypes", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_CHILD_TYPES },
    ElementInfo { class_name: "GeoHierarchyEntity", local_name: "geoHierarchyEntity", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_HIERARCHY_ENTITY, children: &[] },
    ElementInfo { class_name: "GeoChildEntitiesQuery", local_name: "geoChildEntitiesQuery", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GEO_CHILD_ENTITIES_QUERY, children: CHILDREN_GEO_CHILD_ENTITIES_QUERY },
    ElementInfo { class_name: "GeoChildEntities", local_name: "geoChildEntities", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_CHILD_ENTITIES },
    ElementInfo { class_name: "GeoChildEntitiesQueryResult", local_name: "geoChildEntitiesQueryResult", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_CHILD_ENTITIES_QUERY_RESULT },
    ElementInfo { class_name: "GeoParentEntitiesQuery", local_name: "geoParentEntitiesQuery", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_PARENT_ENTITIES_QUERY, children: &[] },
    ElementInfo { class_name: "GeoEntity", local_name: "geoEntity", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_ENTITY, children: &[] },
    ElementInfo { class_name: "GeoParentEntity", local_name: "geoParentEntity", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GEO_PARENT_ENTITY, children: &[] },
    ElementInfo { class_name: "GeoParentEntitiesQueryResult", local_name: "geoParentEntitiesQueryResult", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_PARENT_ENTITIES_QUERY_RESULT },
    ElementInfo { class_name: "GeoLocationQueryResults", local_name: "geoLocationQueryResults", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_LOCATION_QUERY_RESULTS },
    ElementInfo { class_name: "GeoDataEntityQueryResults", local_name: "geoDataEntityQueryResults", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_DATA_ENTITY_QUERY_RESULTS },
    ElementInfo { class_name: "GeoDataPointToEntityQueryResults", local_name: "geoDataPointToEntityQueryResults", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_DATA_POINT_TO_ENTITY_QUERY_RESULTS },
    ElementInfo { class_name: "GeoChildEntitiesQueryResults", local_name: "geoChildEntitiesQueryResults", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_CHILD_ENTITIES_QUERY_RESULTS },
    ElementInfo { class_name: "GeoParentEntitiesQueryResults", local_name: "geoParentEntitiesQueryResults", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GEO_PARENT_ENTITIES_QUERY_RESULTS },
    ElementInfo { class_name: "Xsdbase64Binary", local_name: "binary", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Clear", local_name: "clear", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CLEAR },
    ElementInfo { class_name: "GeoCache", local_name: "geoCache", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GEO_CACHE, children: CHILDREN_GEO_CACHE },
    ElementInfo { class_name: "ParentLabelLayout", local_name: "parentLabelLayout", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PARENT_LABEL_LAYOUT, children: &[] },
    ElementInfo { class_name: "RegionLabelLayout", local_name: "regionLabelLayout", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REGION_LABEL_LAYOUT, children: &[] },
    ElementInfo { class_name: "SeriesElementVisibilities", local_name: "visibility", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SERIES_ELEMENT_VISIBILITIES, children: &[] },
    ElementInfo { class_name: "Aggregation", local_name: "aggregation", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Binning", local_name: "binning", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BINNING, children: CHILDREN_BINNING },
    ElementInfo { class_name: "Geography", local_name: "geography", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GEOGRAPHY, children: CHILDREN_GEOGRAPHY },
    ElementInfo { class_name: "Statistics", local_name: "statistics", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STATISTICS, children: &[] },
    ElementInfo { class_name: "Subtotals", local_name: "subtotals", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUBTOTALS },
    ElementInfo { class_name: "ExtremeValueColorPosition", local_name: "extremeValue", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NumberColorPosition", local_name: "number", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NUMBER_COLOR_POSITION, children: &[] },
    ElementInfo { class_name: "PercentageColorPosition", local_name: "percent", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PERCENTAGE_COLOR_POSITION, children: &[] },
    ElementInfo { class_name: "MinValueColorEndPosition", local_name: "min", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MIN_VALUE_COLOR_END_POSITION },
    ElementInfo { class_name: "MaxValueColorEndPosition", local_name: "max", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MAX_VALUE_COLOR_END_POSITION },
    ElementInfo { class_name: "ValueColorMiddlePosition", local_name: "mid", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VALUE_COLOR_MIDDLE_POSITION },
    ElementInfo { class_name: "DataLabelVisibilities", local_name: "visibility", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_LABEL_VISIBILITIES, children: &[] },
    ElementInfo { class_name: "DataLabel", local_name: "dataLabel", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_LABEL, children: CHILDREN_DATA_LABEL },
    ElementInfo { class_name: "DataLabelHidden", local_name: "dataLabelHidden", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_LABEL_HIDDEN, children: &[] },
    ElementInfo { class_name: "ValueColors", local_name: "valueColors", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VALUE_COLORS },
    ElementInfo { class_name: "ValueColorPositions", local_name: "valueColorPositions", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VALUE_COLOR_POSITIONS, children: CHILDREN_VALUE_COLOR_POSITIONS },
    ElementInfo { class_name: "DataPoint", local_name: "dataPt", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_POINT, children: CHILDREN_DATA_POINT },
    ElementInfo { class_name: "DataLabels", local_name: "dataLabels", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_LABELS, children: CHILDREN_DATA_LABELS },
    ElementInfo { class_name: "DataId", local_name: "dataId", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_ID, children: &[] },
    ElementInfo { class_name: "SeriesLayoutProperties", local_name: "layoutPr", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SERIES_LAYOUT_PROPERTIES },
    ElementInfo { class_name: "AxisId", local_name: "axisId", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PlotSurface", local_name: "plotSurface", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PLOT_SURFACE },
    ElementInfo { class_name: "Series", local_name: "series", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SERIES, children: CHILDREN_SERIES },
    ElementInfo { class_name: "PlotAreaRegion", local_name: "plotAreaRegion", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PLOT_AREA_REGION },
    ElementInfo { class_name: "Axis", local_name: "axis", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AXIS, children: CHILDREN_AXIS },
    ElementInfo { class_name: "ChartTitle", local_name: "title", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHART_TITLE, children: CHILDREN_CHART_TITLE },
    ElementInfo { class_name: "PlotArea", local_name: "plotArea", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PLOT_AREA },
    ElementInfo { class_name: "Legend", local_name: "legend", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEGEND, children: CHILDREN_LEGEND },
    ElementInfo { class_name: "FormatOverride", local_name: "fmtOvr", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FORMAT_OVERRIDE, children: CHILDREN_FORMAT_OVERRIDE },
    ElementInfo { class_name: "HeaderFooter", local_name: "headerFooter", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HEADER_FOOTER, children: CHILDREN_HEADER_FOOTER },
    ElementInfo { class_name: "PageMargins", local_name: "pageMargins", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PAGE_MARGINS, children: &[] },
    ElementInfo { class_name: "PageSetup", local_name: "pageSetup", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PAGE_SETUP, children: &[] },
    ElementInfo { class_name: "ChartData", local_name: "chartData", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART_DATA },
    ElementInfo { class_name: "Chart", local_name: "chart", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHART },
    ElementInfo { class_name: "ColorMappingType", local_name: "clrMapOvr", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_MAPPING_TYPE, children: CHILDREN_COLOR_MAPPING_TYPE },
    ElementInfo { class_name: "FormatOverrides", local_name: "fmtOvrs", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FORMAT_OVERRIDES },
    ElementInfo { class_name: "PrintSettings", local_name: "printSettings", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRINT_SETTINGS },
    ElementInfo { class_name: "UnsignedIntegerType", local_name: "idx", prefix: "cx", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNSIGNED_INTEGER_TYPE, children: &[] },
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

/// Create a `<cx:chartSpace>` element (`ChartSpace`).
pub fn chart_space(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "chartSpace").with_children(children)
}

/// Create a `<cx:chart>` element (`RelId`).
pub fn rel_id() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "chart")
}

/// Create a `<cx:openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3>` element (`Openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3XsdunsignedInt`).
pub fn openxmlsdk_49_b_e_c_f_f_a_3_b03_4_d13_8272__d6_c_c_b22579_e3_xsdunsigned_int(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3").with_text(value)
}

/// Create a `<cx:binCount>` element (`BinCountXsdunsignedInt`).
pub fn bin_count_xsdunsigned_int(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "binCount").with_text(value)
}

/// Create a `<cx:ext>` element (`Extension2`).
pub fn extension2(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<cx:minColor>` element (`MinColorSolidColorFillProperties`).
pub fn min_color_solid_color_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "minColor").with_children(children)
}

/// Create a `<cx:midColor>` element (`MidColorSolidColorFillProperties`).
pub fn mid_color_solid_color_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "midColor").with_children(children)
}

/// Create a `<cx:maxColor>` element (`MaxColorSolidColorFillProperties`).
pub fn max_color_solid_color_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "maxColor").with_children(children)
}

/// Create a `<cx:pt>` element (`ChartStringValue`).
pub fn chart_string_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "pt").with_text(value)
}

/// Create a `<cx:f>` element (`Formula`).
pub fn formula(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "f").with_text(value)
}

/// Create a `<cx:nf>` element (`NfFormula`).
pub fn nf_formula(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "nf").with_text(value)
}

/// Create a `<cx:lvl>` element (`StringLevel`).
pub fn string_level(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "lvl").with_children(children)
}

/// Create a `<cx:pt>` element (`NumericValue`).
pub fn numeric_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "pt").with_text(value)
}

/// Create a `<cx:lvl>` element (`NumericLevel`).
pub fn numeric_level(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "lvl").with_children(children)
}

/// Create a `<cx:numDim>` element (`NumericDimension`).
pub fn numeric_dimension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "numDim").with_children(children)
}

/// Create a `<cx:strDim>` element (`StringDimension`).
pub fn string_dimension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "strDim").with_children(children)
}

/// Create a `<cx:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<cx:externalData>` element (`ExternalData`).
pub fn external_data() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "externalData")
}

/// Create a `<cx:data>` element (`Data`).
pub fn data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "data").with_children(children)
}

/// Create a `<cx:v>` element (`VXsdstring`).
pub fn v_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<cx:copyright>` element (`CopyrightXsdstring`).
pub fn copyright_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "copyright").with_text(value)
}

/// Create a `<cx:separator>` element (`SeparatorXsdstring`).
pub fn separator_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "separator").with_text(value)
}

/// Create a `<cx:oddHeader>` element (`OddHeaderXsdstring`).
pub fn odd_header_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "oddHeader").with_text(value)
}

/// Create a `<cx:oddFooter>` element (`OddFooterXsdstring`).
pub fn odd_footer_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "oddFooter").with_text(value)
}

/// Create a `<cx:evenHeader>` element (`EvenHeaderXsdstring`).
pub fn even_header_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "evenHeader").with_text(value)
}

/// Create a `<cx:evenFooter>` element (`EvenFooterXsdstring`).
pub fn even_footer_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "evenFooter").with_text(value)
}

/// Create a `<cx:firstHeader>` element (`FirstHeaderXsdstring`).
pub fn first_header_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "firstHeader").with_text(value)
}

/// Create a `<cx:firstFooter>` element (`FirstFooterXsdstring`).
pub fn first_footer_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "firstFooter").with_text(value)
}

/// Create a `<cx:txData>` element (`TextData`).
pub fn text_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "txData").with_children(children)
}

/// Create a `<cx:rich>` element (`RichTextBody`).
pub fn rich_text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "rich").with_children(children)
}

/// Create a `<cx:txPr>` element (`TxPrTextBody`).
pub fn tx_pr_text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "txPr").with_children(children)
}

/// Create a `<cx:tx>` element (`Text`).
pub fn text(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "tx").with_children(children)
}

/// Create a `<cx:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<cx:offset>` element (`Offset`).
pub fn offset() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "offset")
}

/// Create a `<cx:unitsLabel>` element (`AxisUnitsLabel`).
pub fn axis_units_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "unitsLabel").with_children(children)
}

/// Create a `<cx:catScaling>` element (`CategoryAxisScaling`).
pub fn category_axis_scaling() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "catScaling")
}

/// Create a `<cx:valScaling>` element (`ValueAxisScaling`).
pub fn value_axis_scaling() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "valScaling")
}

/// Create a `<cx:title>` element (`AxisTitle`).
pub fn axis_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "title").with_children(children)
}

/// Create a `<cx:units>` element (`AxisUnits`).
pub fn axis_units(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "units").with_children(children)
}

/// Create a `<cx:majorGridlines>` element (`MajorGridlinesGridlines`).
pub fn major_gridlines_gridlines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "majorGridlines").with_children(children)
}

/// Create a `<cx:minorGridlines>` element (`MinorGridlinesGridlines`).
pub fn minor_gridlines_gridlines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "minorGridlines").with_children(children)
}

/// Create a `<cx:majorTickMarks>` element (`MajorTickMarksTickMarks`).
pub fn major_tick_marks_tick_marks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "majorTickMarks").with_children(children)
}

/// Create a `<cx:minorTickMarks>` element (`MinorTickMarksTickMarks`).
pub fn minor_tick_marks_tick_marks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "minorTickMarks").with_children(children)
}

/// Create a `<cx:tickLabels>` element (`TickLabels`).
pub fn tick_labels(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "tickLabels").with_children(children)
}

/// Create a `<cx:numFmt>` element (`NumberFormat`).
pub fn number_format() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "numFmt")
}

/// Create a `<cx:binSize>` element (`Xsddouble`).
pub fn xsddouble(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "binSize").with_text(value)
}

/// Create a `<cx:address>` element (`Address`).
pub fn address() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "address")
}

/// Create a `<cx:geoLocation>` element (`GeoLocation`).
pub fn geo_location(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoLocation").with_children(children)
}

/// Create a `<cx:geoLocationQuery>` element (`GeoLocationQuery`).
pub fn geo_location_query() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoLocationQuery")
}

/// Create a `<cx:geoLocations>` element (`GeoLocations`).
pub fn geo_locations(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoLocations").with_children(children)
}

/// Create a `<cx:geoLocationQueryResult>` element (`GeoLocationQueryResult`).
pub fn geo_location_query_result(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoLocationQueryResult").with_children(children)
}

/// Create a `<cx:geoPolygon>` element (`GeoPolygon`).
pub fn geo_polygon() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoPolygon")
}

/// Create a `<cx:geoPolygons>` element (`GeoPolygons`).
pub fn geo_polygons(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoPolygons").with_children(children)
}

/// Create a `<cx:copyrights>` element (`Copyrights`).
pub fn copyrights(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "copyrights").with_children(children)
}

/// Create a `<cx:geoDataEntityQuery>` element (`GeoDataEntityQuery`).
pub fn geo_data_entity_query() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoDataEntityQuery")
}

/// Create a `<cx:geoData>` element (`GeoData`).
pub fn geo_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoData").with_children(children)
}

/// Create a `<cx:geoDataEntityQueryResult>` element (`GeoDataEntityQueryResult`).
pub fn geo_data_entity_query_result(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoDataEntityQueryResult").with_children(children)
}

/// Create a `<cx:geoDataPointQuery>` element (`GeoDataPointQuery`).
pub fn geo_data_point_query() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoDataPointQuery")
}

/// Create a `<cx:geoDataPointToEntityQuery>` element (`GeoDataPointToEntityQuery`).
pub fn geo_data_point_to_entity_query() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoDataPointToEntityQuery")
}

/// Create a `<cx:geoDataPointToEntityQueryResult>` element (`GeoDataPointToEntityQueryResult`).
pub fn geo_data_point_to_entity_query_result(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoDataPointToEntityQueryResult").with_children(children)
}

/// Create a `<cx:entityType>` element (`EntityType`).
pub fn entity_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "entityType").with_text(value)
}

/// Create a `<cx:geoChildTypes>` element (`GeoChildTypes`).
pub fn geo_child_types(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoChildTypes").with_children(children)
}

/// Create a `<cx:geoHierarchyEntity>` element (`GeoHierarchyEntity`).
pub fn geo_hierarchy_entity() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoHierarchyEntity")
}

/// Create a `<cx:geoChildEntitiesQuery>` element (`GeoChildEntitiesQuery`).
pub fn geo_child_entities_query(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoChildEntitiesQuery").with_children(children)
}

/// Create a `<cx:geoChildEntities>` element (`GeoChildEntities`).
pub fn geo_child_entities(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoChildEntities").with_children(children)
}

/// Create a `<cx:geoChildEntitiesQueryResult>` element (`GeoChildEntitiesQueryResult`).
pub fn geo_child_entities_query_result(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoChildEntitiesQueryResult").with_children(children)
}

/// Create a `<cx:geoParentEntitiesQuery>` element (`GeoParentEntitiesQuery`).
pub fn geo_parent_entities_query() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoParentEntitiesQuery")
}

/// Create a `<cx:geoEntity>` element (`GeoEntity`).
pub fn geo_entity() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoEntity")
}

/// Create a `<cx:geoParentEntity>` element (`GeoParentEntity`).
pub fn geo_parent_entity() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoParentEntity")
}

/// Create a `<cx:geoParentEntitiesQueryResult>` element (`GeoParentEntitiesQueryResult`).
pub fn geo_parent_entities_query_result(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoParentEntitiesQueryResult").with_children(children)
}

/// Create a `<cx:geoLocationQueryResults>` element (`GeoLocationQueryResults`).
pub fn geo_location_query_results(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoLocationQueryResults").with_children(children)
}

/// Create a `<cx:geoDataEntityQueryResults>` element (`GeoDataEntityQueryResults`).
pub fn geo_data_entity_query_results(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoDataEntityQueryResults").with_children(children)
}

/// Create a `<cx:geoDataPointToEntityQueryResults>` element (`GeoDataPointToEntityQueryResults`).
pub fn geo_data_point_to_entity_query_results(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoDataPointToEntityQueryResults").with_children(children)
}

/// Create a `<cx:geoChildEntitiesQueryResults>` element (`GeoChildEntitiesQueryResults`).
pub fn geo_child_entities_query_results(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoChildEntitiesQueryResults").with_children(children)
}

/// Create a `<cx:geoParentEntitiesQueryResults>` element (`GeoParentEntitiesQueryResults`).
pub fn geo_parent_entities_query_results(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoParentEntitiesQueryResults").with_children(children)
}

/// Create a `<cx:binary>` element (`Xsdbase64Binary`).
pub fn xsdbase64_binary(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "binary").with_text(value)
}

/// Create a `<cx:clear>` element (`Clear`).
pub fn clear(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "clear").with_children(children)
}

/// Create a `<cx:geoCache>` element (`GeoCache`).
pub fn geo_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geoCache").with_children(children)
}

/// Create a `<cx:parentLabelLayout>` element (`ParentLabelLayout`).
pub fn parent_label_layout() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "parentLabelLayout")
}

/// Create a `<cx:regionLabelLayout>` element (`RegionLabelLayout`).
pub fn region_label_layout() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "regionLabelLayout")
}

/// Create a `<cx:visibility>` element (`SeriesElementVisibilities`).
pub fn series_element_visibilities() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "visibility")
}

/// Create a `<cx:aggregation>` element (`Aggregation`).
pub fn aggregation() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "aggregation")
}

/// Create a `<cx:binning>` element (`Binning`).
pub fn binning(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "binning").with_children(children)
}

/// Create a `<cx:geography>` element (`Geography`).
pub fn geography(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "geography").with_children(children)
}

/// Create a `<cx:statistics>` element (`Statistics`).
pub fn statistics() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "statistics")
}

/// Create a `<cx:subtotals>` element (`Subtotals`).
pub fn subtotals(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "subtotals").with_children(children)
}

/// Create a `<cx:extremeValue>` element (`ExtremeValueColorPosition`).
pub fn extreme_value_color_position() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "extremeValue")
}

/// Create a `<cx:number>` element (`NumberColorPosition`).
pub fn number_color_position() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "number")
}

/// Create a `<cx:percent>` element (`PercentageColorPosition`).
pub fn percentage_color_position() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "percent")
}

/// Create a `<cx:min>` element (`MinValueColorEndPosition`).
pub fn min_value_color_end_position(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "min").with_children(children)
}

/// Create a `<cx:max>` element (`MaxValueColorEndPosition`).
pub fn max_value_color_end_position(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "max").with_children(children)
}

/// Create a `<cx:mid>` element (`ValueColorMiddlePosition`).
pub fn value_color_middle_position(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "mid").with_children(children)
}

/// Create a `<cx:visibility>` element (`DataLabelVisibilities`).
pub fn data_label_visibilities() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "visibility")
}

/// Create a `<cx:dataLabel>` element (`DataLabel`).
pub fn data_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "dataLabel").with_children(children)
}

/// Create a `<cx:dataLabelHidden>` element (`DataLabelHidden`).
pub fn data_label_hidden() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "dataLabelHidden")
}

/// Create a `<cx:valueColors>` element (`ValueColors`).
pub fn value_colors(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "valueColors").with_children(children)
}

/// Create a `<cx:valueColorPositions>` element (`ValueColorPositions`).
pub fn value_color_positions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "valueColorPositions").with_children(children)
}

/// Create a `<cx:dataPt>` element (`DataPoint`).
pub fn data_point(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "dataPt").with_children(children)
}

/// Create a `<cx:dataLabels>` element (`DataLabels`).
pub fn data_labels(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "dataLabels").with_children(children)
}

/// Create a `<cx:dataId>` element (`DataId`).
pub fn data_id() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "dataId")
}

/// Create a `<cx:layoutPr>` element (`SeriesLayoutProperties`).
pub fn series_layout_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "layoutPr").with_children(children)
}

/// Create a `<cx:axisId>` element (`AxisId`).
pub fn axis_id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "axisId").with_text(value)
}

/// Create a `<cx:plotSurface>` element (`PlotSurface`).
pub fn plot_surface(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "plotSurface").with_children(children)
}

/// Create a `<cx:series>` element (`Series`).
pub fn series(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "series").with_children(children)
}

/// Create a `<cx:plotAreaRegion>` element (`PlotAreaRegion`).
pub fn plot_area_region(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "plotAreaRegion").with_children(children)
}

/// Create a `<cx:axis>` element (`Axis`).
pub fn axis(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "axis").with_children(children)
}

/// Create a `<cx:title>` element (`ChartTitle`).
pub fn chart_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "title").with_children(children)
}

/// Create a `<cx:plotArea>` element (`PlotArea`).
pub fn plot_area(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "plotArea").with_children(children)
}

/// Create a `<cx:legend>` element (`Legend`).
pub fn legend(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "legend").with_children(children)
}

/// Create a `<cx:fmtOvr>` element (`FormatOverride`).
pub fn format_override(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "fmtOvr").with_children(children)
}

/// Create a `<cx:headerFooter>` element (`HeaderFooter`).
pub fn header_footer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "headerFooter").with_children(children)
}

/// Create a `<cx:pageMargins>` element (`PageMargins`).
pub fn page_margins() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "pageMargins")
}

/// Create a `<cx:pageSetup>` element (`PageSetup`).
pub fn page_setup() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "pageSetup")
}

/// Create a `<cx:chartData>` element (`ChartData`).
pub fn chart_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "chartData").with_children(children)
}

/// Create a `<cx:chart>` element (`Chart`).
pub fn chart(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "chart").with_children(children)
}

/// Create a `<cx:clrMapOvr>` element (`ColorMappingType`).
pub fn color_mapping_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "clrMapOvr").with_children(children)
}

/// Create a `<cx:fmtOvrs>` element (`FormatOverrides`).
pub fn format_overrides(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "fmtOvrs").with_children(children)
}

/// Create a `<cx:printSettings>` element (`PrintSettings`).
pub fn print_settings(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "printSettings").with_children(children)
}

/// Create a `<cx:idx>` element (`UnsignedIntegerType`).
pub fn unsigned_integer_type() -> OpenXmlElement {
    OpenXmlElement::new("cx", NAMESPACE_URI, "idx")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 125;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 119;
