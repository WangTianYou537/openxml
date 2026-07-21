//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2010_11_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2010/11/main` (prefix `x15`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2010/11/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "x15";

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

static CHILDREN_PIVOT_CACHES: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_PivotCache/x:pivotCache", property_name: None },
];
static CHILDREN_TIMELINE_CACHE_PIVOT_CACHES: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_PivotCache/x:pivotCache", property_name: None },
];
static CHILDREN_PIVOT_TABLE_REFERENCES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_PivotTableReference/x15:pivotTableReference", property_name: None },
];
static ATTRS_QUERY_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":clipped", property_name: Some("Clipped"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":sourceDataName", property_name: Some("SourceDataName"), type_name: "StringValue" },
    AttributeInfo { qname: ":drillThrough", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_WEB_EXTENSIONS: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_WebExtension/x15:webExtension", property_name: None },
];
static CHILDREN_TIMELINE_CACHE_REFERENCES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineCacheRef/x15:timelineCacheRef", property_name: None },
];
static CHILDREN_TIMELINE_REFERENCES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineRef/x15:timelineRef", property_name: None },
];
static ATTRS_WORKBOOK_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":chartTrackingRefBase", property_name: Some("ChartTrackingReferenceBase"), type_name: "BooleanValue" },
];
static ATTRS_TIMELINE_STYLES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":defaultTimelineStyle", property_name: Some("DefaultTimelineStyle"), type_name: "StringValue" },
];
static CHILDREN_TIMELINE_STYLES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineStyle/x15:timelineStyle", property_name: None },
];
static ATTRS_DIFFERENTIAL_FORMATS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_DIFFERENTIAL_FORMATS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Dxf/x:dxf", property_name: None },
];
static ATTRS_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":model", property_name: Some("Model"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":excludeFromRefreshAll", property_name: Some("ExcludeFromRefreshAll"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":autoDelete", property_name: Some("AutoDelete"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":usedByAddin", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_CONNECTION: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_TextPr/x15:textPr", property_name: Some("TextProperties") },
    ChildInfo { name: "x15:CT_ModelTextPr/x15:modelTextPr", property_name: Some("ModelTextProperties") },
    ChildInfo { name: "x15:CT_RangePr/x15:rangePr", property_name: Some("RangeProperties") },
    ChildInfo { name: "x15:CT_OledbPr/x15:oledbPr", property_name: Some("OleDbPrpoperties") },
    ChildInfo { name: "x15:CT_DataFeedPr/x15:dataFeedPr", property_name: Some("DataFeedProperties") },
];
static ATTRS_CALCULATED_MEMBER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":measureGroup", property_name: Some("MeasureGroup"), type_name: "StringValue" },
    AttributeInfo { qname: ":numberFormat", property_name: Some("NumberFormat"), type_name: "EnumValue" },
    AttributeInfo { qname: ":measure", property_name: Some("Measure"), type_name: "BooleanValue" },
];
static ATTRS_PIVOT_TABLE_U_I_SETTINGS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sourceDataName", property_name: Some("SourceDataName"), type_name: "StringValue" },
    AttributeInfo { qname: ":relNeededHidden", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_PIVOT_TABLE_U_I_SETTINGS: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_FieldListActiveTabTopLevelEntity/x15:activeTabTopLevelEntity", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: None },
];
static ATTRS_PIVOT_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":useWholeDay", property_name: Some("UseWholeDay"), type_name: "BooleanValue" },
];
static CHILDREN_CACHED_UNIQUE_NAMES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_CachedUniqueName/x15:cachedUniqueName", property_name: None },
];
static ATTRS_CACHE_HIERARCHY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":aggregatedColumn", property_name: Some("AggregatedColumn"), type_name: "Int32Value" },
];
static ATTRS_TIMELINE_PIVOT_CACHE_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":timelineData", property_name: Some("TimelineData"), type_name: "BooleanValue" },
];
static ATTRS_PIVOT_CACHE_ID_VERSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cacheIdSupportedVersion", property_name: Some("CacheIdSupportedVersion"), type_name: "ByteValue" },
    AttributeInfo { qname: ":cacheIdCreatedVersion", property_name: Some("CacheIdCreatedVersion"), type_name: "ByteValue" },
];
static ATTRS_DATA_MODEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":minVersionLoad", property_name: Some("MinVersionLoad"), type_name: "ByteValue" },
];
static CHILDREN_DATA_MODEL: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_ModelTables/x15:modelTables", property_name: Some("ModelTables") },
    ChildInfo { name: "x15:CT_ModelRelationships/x15:modelRelationships", property_name: Some("ModelRelationships") },
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PIVOT_TABLE_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rowCount", property_name: Some("RowCount"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":columnCount", property_name: Some("ColumnCount"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":cacheId", property_name: Some("CacheId"), type_name: "UInt32Value" },
];
static CHILDREN_PIVOT_TABLE_DATA: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_PivotRow/x15:pivotRow", property_name: None },
];
static ATTRS_PIVOT_CACHE_DECOUPLED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":decoupled", property_name: Some("Decoupled"), type_name: "BooleanValue" },
];
static ATTRS_DATA_FIELD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isCountDistinct", property_name: Some("IsCountDistinct"), type_name: "BooleanValue" },
];
static ATTRS_MOVING_PERIOD_STATE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":referenceDateBegin", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":referencePeriod", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":referenceMultiple", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":movingPeriod", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":movingMultiple", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SLICER_CACHES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerCache/x14:slicerCache", property_name: None },
];
static ATTRS_TABLE_SLICER_CACHE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tableId", property_name: Some("TableId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":column", property_name: Some("Column"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":sortOrder", property_name: Some("SortOrder"), type_name: "EnumValue" },
    AttributeInfo { qname: ":customListSort", property_name: Some("CustomListSort"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":crossFilter", property_name: Some("CrossFilter"), type_name: "EnumValue" },
];
static CHILDREN_TABLE_SLICER_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SLICER_CACHE_HIDE_ITEMS_WITH_NO_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_SLICER_CACHE_HIDE_ITEMS_WITH_NO_DATA: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_SlicerCacheOlapLevelName/x15:slicerCacheOlapLevelName", property_name: None },
];
static CHILDREN_SLICER_CACHE_PIVOT_TABLES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerCachePivotTable/x14:pivotTable", property_name: None },
];
static ATTRS_SURVEY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":guid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":title", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SURVEY: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_SurveyElementPr/x15:surveyPr", property_name: Some("SurveyPrSurveyElementPr") },
    ChildInfo { name: "x15:CT_SurveyElementPr/x15:titlePr", property_name: Some("TitlePrSurveyElementPr") },
    ChildInfo { name: "x15:CT_SurveyElementPr/x15:descriptionPr", property_name: Some("DescriptionPrSurveyElementPr") },
    ChildInfo { name: "x15:CT_SurveyQuestions/x15:questions", property_name: Some("SurveyQuestions") },
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_TIMELINES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_Timeline/x15:timeline", property_name: None },
];
static ATTRS_TIMELINE_CACHE_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":sourceName", property_name: Some("SourceName"), type_name: "StringValue" },
];
static CHILDREN_TIMELINE_CACHE_DEFINITION: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineCachePivotTables/x15:pivotTables", property_name: Some("TimelineCachePivotTables") },
    ChildInfo { name: "x15:CT_TimelineState/x15:state", property_name: Some("TimelineState") },
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PIVOT_TABLE_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_WEB_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":appRef", property_name: Some("ApplicationReference"), type_name: "StringValue" },
];
static CHILDREN_WEB_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
];
static ATTRS_TIMELINE_CACHE_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_TIMELINE_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_TIMELINE_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_TIMELINE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineStyleElements/x15:timelineStyleElements", property_name: Some("TimelineStyleElements") },
];
static ATTRS_TIMELINE_STYLE_ELEMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dxfId", property_name: Some("FormatId"), type_name: "UInt32Value" },
];
static CHILDREN_TIMELINE_STYLE_ELEMENTS: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineStyleElement/x15:timelineStyleElement", property_name: None },
];
static ATTRS_DB_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_DB_TABLES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_DbTable/x15:dbTable", property_name: None },
];
static ATTRS_DB_COMMAND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":text", property_name: Some("Text"), type_name: "StringValue" },
];
static ATTRS_TEXT_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prompt", property_name: Some("Prompt"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fileType", property_name: Some("FileType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":codePage", property_name: Some("CodePage"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":characterSet", property_name: Some("TextCharacterSet"), type_name: "StringValue" },
    AttributeInfo { qname: ":firstRow", property_name: Some("FirstRow"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":sourceFile", property_name: Some("SourceFile"), type_name: "StringValue" },
    AttributeInfo { qname: ":delimited", property_name: Some("Delimited"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":decimal", property_name: Some("Decimal"), type_name: "StringValue" },
    AttributeInfo { qname: ":thousands", property_name: Some("Thousands"), type_name: "StringValue" },
    AttributeInfo { qname: ":tab", property_name: Some("TabAsDelimiter"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":space", property_name: Some("Space"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":comma", property_name: Some("Comma"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":semicolon", property_name: Some("Semicolon"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":consecutive", property_name: Some("Consecutive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":qualifier", property_name: Some("Qualifier"), type_name: "EnumValue" },
    AttributeInfo { qname: ":delimiter", property_name: Some("Delimiter"), type_name: "StringValue" },
];
static CHILDREN_TEXT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_TextFields/x:textFields", property_name: Some("TextFields") },
];
static ATTRS_MODEL_TEXT_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":headers", property_name: Some("Headers"), type_name: "BooleanValue" },
];
static ATTRS_RANGE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sourceName", property_name: Some("SourceName"), type_name: "StringValue" },
];
static ATTRS_OLE_DB_PRPOPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":connection", property_name: Some("Connection"), type_name: "StringValue" },
];
static CHILDREN_OLE_DB_PRPOPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_DbTables/x15:dbTables", property_name: Some("DbTables") },
    ChildInfo { name: "x15:CT_DbCommand/x15:dbCommand", property_name: Some("DbCommand") },
];
static ATTRS_DATA_FEED_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":connection", property_name: Some("Connection"), type_name: "StringValue" },
];
static CHILDREN_DATA_FEED_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_DbTables/x15:dbTables", property_name: Some("DbTables") },
];
static ATTRS_FIELD_LIST_ACTIVE_TAB_TOP_LEVEL_ENTITY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "UInt32Value" },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_CACHED_UNIQUE_NAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":index", property_name: Some("Index"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static ATTRS_MODEL_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":connection", property_name: Some("Connection"), type_name: "StringValue" },
];
static ATTRS_MODEL_RELATIONSHIP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fromTable", property_name: Some("FromTable"), type_name: "StringValue" },
    AttributeInfo { qname: ":fromColumn", property_name: Some("FromColumn"), type_name: "StringValue" },
    AttributeInfo { qname: ":toTable", property_name: Some("ToTable"), type_name: "StringValue" },
    AttributeInfo { qname: ":toColumn", property_name: Some("ToColumn"), type_name: "StringValue" },
];
static CHILDREN_MODEL_TABLES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_ModelTable/x15:modelTable", property_name: None },
];
static CHILDREN_MODEL_RELATIONSHIPS: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_ModelRelationship/x15:modelRelationship", property_name: None },
];
static ATTRS_PIVOT_VALUE_CELL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":i", property_name: Some("Item"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":t", property_name: Some("Text"), type_name: "EnumValue" },
];
static CHILDREN_PIVOT_VALUE_CELL: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Xstring/x15:v", property_name: Some("Xstring") },
    ChildInfo { name: "x15:CT_PivotValueCellExtra/x15:x", property_name: Some("PivotValueCellExtra") },
];
static ATTRS_PIVOT_VALUE_CELL_EXTRA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":in", property_name: Some("FormatIndex"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":bc", property_name: Some("BackgroundColor"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":fc", property_name: Some("ForegroundColor"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":i", property_name: Some("Italic"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":un", property_name: Some("Underline"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":st", property_name: Some("Strikethrough"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":b", property_name: Some("Bold"), type_name: "BooleanValue" },
];
static ATTRS_PIVOT_TABLE_SERVER_FORMATS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_PIVOT_TABLE_SERVER_FORMATS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ServerFormat/x15:serverFormat", property_name: None },
];
static ATTRS_SERVER_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":culture", property_name: Some("Culture"), type_name: "StringValue" },
    AttributeInfo { qname: ":format", property_name: Some("Format"), type_name: "StringValue" },
];
static ATTRS_SLICER_CACHE_OLAP_LEVEL_NAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueName", property_name: Some("UniqueName"), type_name: "StringValue" },
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static ATTRS_SURVEY_PR_SURVEY_ELEMENT_PR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cssClass", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":bottom", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":top", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":left", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":right", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":width", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":height", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":position", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_SURVEY_PR_SURVEY_ELEMENT_PR: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TITLE_PR_SURVEY_ELEMENT_PR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cssClass", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":bottom", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":top", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":left", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":right", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":width", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":height", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":position", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_TITLE_PR_SURVEY_ELEMENT_PR: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_DESCRIPTION_PR_SURVEY_ELEMENT_PR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cssClass", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":bottom", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":top", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":left", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":right", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":width", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":height", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":position", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_DESCRIPTION_PR_SURVEY_ELEMENT_PR: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_QUESTIONS_PR_SURVEY_ELEMENT_PR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cssClass", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":bottom", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":top", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":left", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":right", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":width", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":height", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":position", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_QUESTIONS_PR_SURVEY_ELEMENT_PR: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_QUESTION_PR_SURVEY_ELEMENT_PR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cssClass", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":bottom", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":top", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":left", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":right", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":width", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":height", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":position", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_QUESTION_PR_SURVEY_ELEMENT_PR: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_SURVEY_QUESTIONS: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_SurveyElementPr/x15:questionsPr", property_name: Some("QuestionsPrSurveyElementPr") },
    ChildInfo { name: "x15:CT_SurveyQuestion/x15:question", property_name: None },
];
static ATTRS_SURVEY_QUESTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":binding", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":text", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":format", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":helpText", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":required", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":defaultValue", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":decimalPlaces", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":rowSource", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SURVEY_QUESTION: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_SurveyElementPr/x15:questionPr", property_name: Some("QuestionPrSurveyElementPr") },
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TIMELINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":cache", property_name: Some("Cache"), type_name: "StringValue" },
    AttributeInfo { qname: ":caption", property_name: Some("Caption"), type_name: "StringValue" },
    AttributeInfo { qname: ":showHeader", property_name: Some("ShowHeader"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showSelectionLabel", property_name: Some("ShowSelectionLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showTimeLevel", property_name: Some("ShowTimeLevel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showHorizontalScrollbar", property_name: Some("ShowHorizontalScrollbar"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":level", property_name: Some("Level"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":selectionLevel", property_name: Some("SelectionLevel"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":scrollPosition", property_name: Some("ScrollPosition"), type_name: "DateTimeValue" },
    AttributeInfo { qname: ":style", property_name: Some("Style"), type_name: "StringValue" },
];
static CHILDREN_TIMELINE: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TIMELINE_CACHE_PIVOT_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tabId", property_name: Some("TabId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static ATTRS_SELECTION_TIMELINE_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":startDate", property_name: Some("StartDate"), type_name: "DateTimeValue" },
    AttributeInfo { qname: ":endDate", property_name: Some("EndDate"), type_name: "DateTimeValue" },
];
static ATTRS_BOUNDS_TIMELINE_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":startDate", property_name: Some("StartDate"), type_name: "DateTimeValue" },
    AttributeInfo { qname: ":endDate", property_name: Some("EndDate"), type_name: "DateTimeValue" },
];
static ATTRS_AUTO_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];
static CHILDREN_AUTO_FILTER: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_FilterColumn/x:filterColumn", property_name: None },
    ChildInfo { name: "x:CT_SortState/x:sortState", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: None },
];
static CHILDREN_TIMELINE_CACHE_PIVOT_TABLES: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineCachePivotTable/x15:pivotTable", property_name: None },
];
static ATTRS_TIMELINE_STATE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":singleRangeFilterState", property_name: Some("SingleRangeFilterState"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":minimalRefreshVersion", property_name: Some("MinimalRefreshVersion"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":lastRefreshVersion", property_name: Some("LastRefreshVersion"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":pivotCacheId", property_name: Some("PivotCacheId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":filterType", property_name: Some("FilterType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":filterId", property_name: Some("FilterId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":filterTabId", property_name: Some("FilterTabId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":filterPivotName", property_name: Some("FilterPivotName"), type_name: "StringValue" },
];
static CHILDREN_TIMELINE_STATE: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_TimelineRange/x15:selection", property_name: Some("SelectionTimelineRange") },
    ChildInfo { name: "x15:CT_TimelineRange/x15:bounds", property_name: Some("BoundsTimelineRange") },
    ChildInfo { name: "x15:CT_MovingPeriodState/x15:movingPeriodState", property_name: Some("MovingPeriodState") },
    ChildInfo { name: "x:CT_ExtensionList/x15:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PIVOT_ROW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":r", property_name: Some("Reference"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_PIVOT_ROW: &[ChildInfo] = &[
    ChildInfo { name: "x15:CT_PivotValueCell/x15:c", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "PivotCaches", local_name: "pivotCaches", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_CACHES },
    ElementInfo { class_name: "TimelineCachePivotCaches", local_name: "timelineCachePivotCaches", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIMELINE_CACHE_PIVOT_CACHES },
    ElementInfo { class_name: "PivotTableReferences", local_name: "pivotTableReferences", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_TABLE_REFERENCES },
    ElementInfo { class_name: "QueryTable", local_name: "queryTable", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_QUERY_TABLE, children: &[] },
    ElementInfo { class_name: "WebExtensions", local_name: "webExtensions", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WEB_EXTENSIONS },
    ElementInfo { class_name: "TimelineCacheReferences", local_name: "timelineCacheRefs", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIMELINE_CACHE_REFERENCES },
    ElementInfo { class_name: "TimelineReferences", local_name: "timelineRefs", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIMELINE_REFERENCES },
    ElementInfo { class_name: "WorkbookProperties", local_name: "workbookPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WORKBOOK_PROPERTIES, children: &[] },
    ElementInfo { class_name: "TimelineStyles", local_name: "timelineStyles", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TIMELINE_STYLES, children: CHILDREN_TIMELINE_STYLES },
    ElementInfo { class_name: "DifferentialFormats", local_name: "dxfs", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIFFERENTIAL_FORMATS, children: CHILDREN_DIFFERENTIAL_FORMATS },
    ElementInfo { class_name: "Connection", local_name: "connection", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTION, children: CHILDREN_CONNECTION },
    ElementInfo { class_name: "CalculatedMember", local_name: "calculatedMember", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CALCULATED_MEMBER, children: &[] },
    ElementInfo { class_name: "PivotTableUISettings", local_name: "pivotTableUISettings", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_TABLE_U_I_SETTINGS, children: CHILDREN_PIVOT_TABLE_U_I_SETTINGS },
    ElementInfo { class_name: "PivotFilter", local_name: "pivotFilter", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_FILTER, children: &[] },
    ElementInfo { class_name: "CachedUniqueNames", local_name: "cachedUniqueNames", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CACHED_UNIQUE_NAMES },
    ElementInfo { class_name: "CacheHierarchy", local_name: "cacheHierarchy", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CACHE_HIERARCHY, children: &[] },
    ElementInfo { class_name: "TimelinePivotCacheDefinition", local_name: "timelinePivotCacheDefinition", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIMELINE_PIVOT_CACHE_DEFINITION, children: &[] },
    ElementInfo { class_name: "PivotCacheIdVersion", local_name: "pivotCacheIdVersion", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_CACHE_ID_VERSION, children: &[] },
    ElementInfo { class_name: "DataModel", local_name: "dataModel", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_MODEL, children: CHILDREN_DATA_MODEL },
    ElementInfo { class_name: "PivotTableData", local_name: "pivotTableData", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_TABLE_DATA, children: CHILDREN_PIVOT_TABLE_DATA },
    ElementInfo { class_name: "PivotCacheDecoupled", local_name: "pivotCacheDecoupled", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_CACHE_DECOUPLED, children: &[] },
    ElementInfo { class_name: "DataField", local_name: "dataField", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_FIELD, children: &[] },
    ElementInfo { class_name: "MovingPeriodState", local_name: "movingPeriodState", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MOVING_PERIOD_STATE, children: &[] },
    ElementInfo { class_name: "SlicerCaches", local_name: "slicerCaches", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_CACHES },
    ElementInfo { class_name: "TableSlicerCache", local_name: "tableSlicerCache", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_SLICER_CACHE, children: CHILDREN_TABLE_SLICER_CACHE },
    ElementInfo { class_name: "SlicerCacheHideItemsWithNoData", local_name: "slicerCacheHideItemsWithNoData", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLICER_CACHE_HIDE_ITEMS_WITH_NO_DATA, children: CHILDREN_SLICER_CACHE_HIDE_ITEMS_WITH_NO_DATA },
    ElementInfo { class_name: "SlicerCachePivotTables", local_name: "slicerCachePivotTables", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_CACHE_PIVOT_TABLES },
    ElementInfo { class_name: "Survey", local_name: "survey", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SURVEY, children: CHILDREN_SURVEY },
    ElementInfo { class_name: "Timelines", local_name: "timelines", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIMELINES },
    ElementInfo { class_name: "TimelineCacheDefinition", local_name: "timelineCacheDefinition", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TIMELINE_CACHE_DEFINITION, children: CHILDREN_TIMELINE_CACHE_DEFINITION },
    ElementInfo { class_name: "PivotTableReference", local_name: "pivotTableReference", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_TABLE_REFERENCE, children: &[] },
    ElementInfo { class_name: "WebExtension", local_name: "webExtension", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION, children: CHILDREN_WEB_EXTENSION },
    ElementInfo { class_name: "TimelineCacheReference", local_name: "timelineCacheRef", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIMELINE_CACHE_REFERENCE, children: &[] },
    ElementInfo { class_name: "TimelineReference", local_name: "timelineRef", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIMELINE_REFERENCE, children: &[] },
    ElementInfo { class_name: "TimelineStyle", local_name: "timelineStyle", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TIMELINE_STYLE, children: CHILDREN_TIMELINE_STYLE },
    ElementInfo { class_name: "TimelineStyleElement", local_name: "timelineStyleElement", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIMELINE_STYLE_ELEMENT, children: &[] },
    ElementInfo { class_name: "TimelineStyleElements", local_name: "timelineStyleElements", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIMELINE_STYLE_ELEMENTS },
    ElementInfo { class_name: "DbTable", local_name: "dbTable", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DB_TABLE, children: &[] },
    ElementInfo { class_name: "DbTables", local_name: "dbTables", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DB_TABLES },
    ElementInfo { class_name: "DbCommand", local_name: "dbCommand", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DB_COMMAND, children: &[] },
    ElementInfo { class_name: "TextProperties", local_name: "textPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_PROPERTIES, children: CHILDREN_TEXT_PROPERTIES },
    ElementInfo { class_name: "ModelTextProperties", local_name: "modelTextPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODEL_TEXT_PROPERTIES, children: &[] },
    ElementInfo { class_name: "RangeProperties", local_name: "rangePr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RANGE_PROPERTIES, children: &[] },
    ElementInfo { class_name: "OleDbPrpoperties", local_name: "oledbPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLE_DB_PRPOPERTIES, children: CHILDREN_OLE_DB_PRPOPERTIES },
    ElementInfo { class_name: "DataFeedProperties", local_name: "dataFeedPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_FEED_PROPERTIES, children: CHILDREN_DATA_FEED_PROPERTIES },
    ElementInfo { class_name: "FieldListActiveTabTopLevelEntity", local_name: "activeTabTopLevelEntity", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FIELD_LIST_ACTIVE_TAB_TOP_LEVEL_ENTITY, children: &[] },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "CachedUniqueName", local_name: "cachedUniqueName", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CACHED_UNIQUE_NAME, children: &[] },
    ElementInfo { class_name: "ModelTable", local_name: "modelTable", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODEL_TABLE, children: &[] },
    ElementInfo { class_name: "ModelRelationship", local_name: "modelRelationship", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODEL_RELATIONSHIP, children: &[] },
    ElementInfo { class_name: "ModelTables", local_name: "modelTables", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODEL_TABLES },
    ElementInfo { class_name: "ModelRelationships", local_name: "modelRelationships", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODEL_RELATIONSHIPS },
    ElementInfo { class_name: "PivotValueCell", local_name: "c", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_VALUE_CELL, children: CHILDREN_PIVOT_VALUE_CELL },
    ElementInfo { class_name: "Xstring", local_name: "v", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PivotValueCellExtra", local_name: "x", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_VALUE_CELL_EXTRA, children: &[] },
    ElementInfo { class_name: "PivotTableServerFormats", local_name: "pivotTableServerFormats", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_TABLE_SERVER_FORMATS, children: CHILDREN_PIVOT_TABLE_SERVER_FORMATS },
    ElementInfo { class_name: "ServerFormat", local_name: "serverFormat", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SERVER_FORMAT, children: &[] },
    ElementInfo { class_name: "SlicerCacheOlapLevelName", local_name: "slicerCacheOlapLevelName", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLICER_CACHE_OLAP_LEVEL_NAME, children: &[] },
    ElementInfo { class_name: "SurveyPrSurveyElementPr", local_name: "surveyPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SURVEY_PR_SURVEY_ELEMENT_PR, children: CHILDREN_SURVEY_PR_SURVEY_ELEMENT_PR },
    ElementInfo { class_name: "TitlePrSurveyElementPr", local_name: "titlePr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TITLE_PR_SURVEY_ELEMENT_PR, children: CHILDREN_TITLE_PR_SURVEY_ELEMENT_PR },
    ElementInfo { class_name: "DescriptionPrSurveyElementPr", local_name: "descriptionPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DESCRIPTION_PR_SURVEY_ELEMENT_PR, children: CHILDREN_DESCRIPTION_PR_SURVEY_ELEMENT_PR },
    ElementInfo { class_name: "QuestionsPrSurveyElementPr", local_name: "questionsPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_QUESTIONS_PR_SURVEY_ELEMENT_PR, children: CHILDREN_QUESTIONS_PR_SURVEY_ELEMENT_PR },
    ElementInfo { class_name: "QuestionPrSurveyElementPr", local_name: "questionPr", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_QUESTION_PR_SURVEY_ELEMENT_PR, children: CHILDREN_QUESTION_PR_SURVEY_ELEMENT_PR },
    ElementInfo { class_name: "SurveyQuestions", local_name: "questions", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SURVEY_QUESTIONS },
    ElementInfo { class_name: "SurveyQuestion", local_name: "question", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SURVEY_QUESTION, children: CHILDREN_SURVEY_QUESTION },
    ElementInfo { class_name: "Timeline", local_name: "timeline", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TIMELINE, children: CHILDREN_TIMELINE },
    ElementInfo { class_name: "TimelineCachePivotTable", local_name: "pivotTable", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIMELINE_CACHE_PIVOT_TABLE, children: &[] },
    ElementInfo { class_name: "SelectionTimelineRange", local_name: "selection", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SELECTION_TIMELINE_RANGE, children: &[] },
    ElementInfo { class_name: "BoundsTimelineRange", local_name: "bounds", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOUNDS_TIMELINE_RANGE, children: &[] },
    ElementInfo { class_name: "AutoFilter", local_name: "autoFilter", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AUTO_FILTER, children: CHILDREN_AUTO_FILTER },
    ElementInfo { class_name: "TimelineCachePivotTables", local_name: "pivotTables", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIMELINE_CACHE_PIVOT_TABLES },
    ElementInfo { class_name: "TimelineState", local_name: "state", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TIMELINE_STATE, children: CHILDREN_TIMELINE_STATE },
    ElementInfo { class_name: "PivotRow", local_name: "pivotRow", prefix: "x15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_ROW, children: CHILDREN_PIVOT_ROW },
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

/// Create a `<x15:pivotCaches>` element (`PivotCaches`).
pub fn pivot_caches(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotCaches").with_children(children)
}

/// Create a `<x15:timelineCachePivotCaches>` element (`TimelineCachePivotCaches`).
pub fn timeline_cache_pivot_caches(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineCachePivotCaches").with_children(children)
}

/// Create a `<x15:pivotTableReferences>` element (`PivotTableReferences`).
pub fn pivot_table_references(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotTableReferences").with_children(children)
}

/// Create a `<x15:queryTable>` element (`QueryTable`).
pub fn query_table() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "queryTable")
}

/// Create a `<x15:webExtensions>` element (`WebExtensions`).
pub fn web_extensions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "webExtensions").with_children(children)
}

/// Create a `<x15:timelineCacheRefs>` element (`TimelineCacheReferences`).
pub fn timeline_cache_references(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineCacheRefs").with_children(children)
}

/// Create a `<x15:timelineRefs>` element (`TimelineReferences`).
pub fn timeline_references(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineRefs").with_children(children)
}

/// Create a `<x15:workbookPr>` element (`WorkbookProperties`).
pub fn workbook_properties() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "workbookPr")
}

/// Create a `<x15:timelineStyles>` element (`TimelineStyles`).
pub fn timeline_styles(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineStyles").with_children(children)
}

/// Create a `<x15:dxfs>` element (`DifferentialFormats`).
pub fn differential_formats(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "dxfs").with_children(children)
}

/// Create a `<x15:connection>` element (`Connection`).
pub fn connection(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "connection").with_children(children)
}

/// Create a `<x15:calculatedMember>` element (`CalculatedMember`).
pub fn calculated_member() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "calculatedMember")
}

/// Create a `<x15:pivotTableUISettings>` element (`PivotTableUISettings`).
pub fn pivot_table_u_i_settings(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotTableUISettings").with_children(children)
}

/// Create a `<x15:pivotFilter>` element (`PivotFilter`).
pub fn pivot_filter() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotFilter")
}

/// Create a `<x15:cachedUniqueNames>` element (`CachedUniqueNames`).
pub fn cached_unique_names(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "cachedUniqueNames").with_children(children)
}

/// Create a `<x15:cacheHierarchy>` element (`CacheHierarchy`).
pub fn cache_hierarchy() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "cacheHierarchy")
}

/// Create a `<x15:timelinePivotCacheDefinition>` element (`TimelinePivotCacheDefinition`).
pub fn timeline_pivot_cache_definition() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelinePivotCacheDefinition")
}

/// Create a `<x15:pivotCacheIdVersion>` element (`PivotCacheIdVersion`).
pub fn pivot_cache_id_version() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotCacheIdVersion")
}

/// Create a `<x15:dataModel>` element (`DataModel`).
pub fn data_model(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "dataModel").with_children(children)
}

/// Create a `<x15:pivotTableData>` element (`PivotTableData`).
pub fn pivot_table_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotTableData").with_children(children)
}

/// Create a `<x15:pivotCacheDecoupled>` element (`PivotCacheDecoupled`).
pub fn pivot_cache_decoupled() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotCacheDecoupled")
}

/// Create a `<x15:dataField>` element (`DataField`).
pub fn data_field() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "dataField")
}

/// Create a `<x15:movingPeriodState>` element (`MovingPeriodState`).
pub fn moving_period_state() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "movingPeriodState")
}

/// Create a `<x15:slicerCaches>` element (`SlicerCaches`).
pub fn slicer_caches(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "slicerCaches").with_children(children)
}

/// Create a `<x15:tableSlicerCache>` element (`TableSlicerCache`).
pub fn table_slicer_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "tableSlicerCache").with_children(children)
}

/// Create a `<x15:slicerCacheHideItemsWithNoData>` element (`SlicerCacheHideItemsWithNoData`).
pub fn slicer_cache_hide_items_with_no_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "slicerCacheHideItemsWithNoData").with_children(children)
}

/// Create a `<x15:slicerCachePivotTables>` element (`SlicerCachePivotTables`).
pub fn slicer_cache_pivot_tables(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "slicerCachePivotTables").with_children(children)
}

/// Create a `<x15:survey>` element (`Survey`).
pub fn survey(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "survey").with_children(children)
}

/// Create a `<x15:timelines>` element (`Timelines`).
pub fn timelines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelines").with_children(children)
}

/// Create a `<x15:timelineCacheDefinition>` element (`TimelineCacheDefinition`).
pub fn timeline_cache_definition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineCacheDefinition").with_children(children)
}

/// Create a `<x15:pivotTableReference>` element (`PivotTableReference`).
pub fn pivot_table_reference() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotTableReference")
}

/// Create a `<x15:webExtension>` element (`WebExtension`).
pub fn web_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "webExtension").with_children(children)
}

/// Create a `<x15:timelineCacheRef>` element (`TimelineCacheReference`).
pub fn timeline_cache_reference() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineCacheRef")
}

/// Create a `<x15:timelineRef>` element (`TimelineReference`).
pub fn timeline_reference() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineRef")
}

/// Create a `<x15:timelineStyle>` element (`TimelineStyle`).
pub fn timeline_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineStyle").with_children(children)
}

/// Create a `<x15:timelineStyleElement>` element (`TimelineStyleElement`).
pub fn timeline_style_element() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineStyleElement")
}

/// Create a `<x15:timelineStyleElements>` element (`TimelineStyleElements`).
pub fn timeline_style_elements(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timelineStyleElements").with_children(children)
}

/// Create a `<x15:dbTable>` element (`DbTable`).
pub fn db_table() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "dbTable")
}

/// Create a `<x15:dbTables>` element (`DbTables`).
pub fn db_tables(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "dbTables").with_children(children)
}

/// Create a `<x15:dbCommand>` element (`DbCommand`).
pub fn db_command() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "dbCommand")
}

/// Create a `<x15:textPr>` element (`TextProperties`).
pub fn text_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "textPr").with_children(children)
}

/// Create a `<x15:modelTextPr>` element (`ModelTextProperties`).
pub fn model_text_properties() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "modelTextPr")
}

/// Create a `<x15:rangePr>` element (`RangeProperties`).
pub fn range_properties() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "rangePr")
}

/// Create a `<x15:oledbPr>` element (`OleDbPrpoperties`).
pub fn ole_db_prpoperties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "oledbPr").with_children(children)
}

/// Create a `<x15:dataFeedPr>` element (`DataFeedProperties`).
pub fn data_feed_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "dataFeedPr").with_children(children)
}

/// Create a `<x15:activeTabTopLevelEntity>` element (`FieldListActiveTabTopLevelEntity`).
pub fn field_list_active_tab_top_level_entity() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "activeTabTopLevelEntity")
}

/// Create a `<x15:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<x15:cachedUniqueName>` element (`CachedUniqueName`).
pub fn cached_unique_name() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "cachedUniqueName")
}

/// Create a `<x15:modelTable>` element (`ModelTable`).
pub fn model_table() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "modelTable")
}

/// Create a `<x15:modelRelationship>` element (`ModelRelationship`).
pub fn model_relationship() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "modelRelationship")
}

/// Create a `<x15:modelTables>` element (`ModelTables`).
pub fn model_tables(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "modelTables").with_children(children)
}

/// Create a `<x15:modelRelationships>` element (`ModelRelationships`).
pub fn model_relationships(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "modelRelationships").with_children(children)
}

/// Create a `<x15:c>` element (`PivotValueCell`).
pub fn pivot_value_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "c").with_children(children)
}

/// Create a `<x15:v>` element (`Xstring`).
pub fn xstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<x15:x>` element (`PivotValueCellExtra`).
pub fn pivot_value_cell_extra() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "x")
}

/// Create a `<x15:pivotTableServerFormats>` element (`PivotTableServerFormats`).
pub fn pivot_table_server_formats(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotTableServerFormats").with_children(children)
}

/// Create a `<x15:serverFormat>` element (`ServerFormat`).
pub fn server_format() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "serverFormat")
}

/// Create a `<x15:slicerCacheOlapLevelName>` element (`SlicerCacheOlapLevelName`).
pub fn slicer_cache_olap_level_name() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "slicerCacheOlapLevelName")
}

/// Create a `<x15:surveyPr>` element (`SurveyPrSurveyElementPr`).
pub fn survey_pr_survey_element_pr(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "surveyPr").with_children(children)
}

/// Create a `<x15:titlePr>` element (`TitlePrSurveyElementPr`).
pub fn title_pr_survey_element_pr(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "titlePr").with_children(children)
}

/// Create a `<x15:descriptionPr>` element (`DescriptionPrSurveyElementPr`).
pub fn description_pr_survey_element_pr(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "descriptionPr").with_children(children)
}

/// Create a `<x15:questionsPr>` element (`QuestionsPrSurveyElementPr`).
pub fn questions_pr_survey_element_pr(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "questionsPr").with_children(children)
}

/// Create a `<x15:questionPr>` element (`QuestionPrSurveyElementPr`).
pub fn question_pr_survey_element_pr(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "questionPr").with_children(children)
}

/// Create a `<x15:questions>` element (`SurveyQuestions`).
pub fn survey_questions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "questions").with_children(children)
}

/// Create a `<x15:question>` element (`SurveyQuestion`).
pub fn survey_question(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "question").with_children(children)
}

/// Create a `<x15:timeline>` element (`Timeline`).
pub fn timeline(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "timeline").with_children(children)
}

/// Create a `<x15:pivotTable>` element (`TimelineCachePivotTable`).
pub fn timeline_cache_pivot_table() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotTable")
}

/// Create a `<x15:selection>` element (`SelectionTimelineRange`).
pub fn selection_timeline_range() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "selection")
}

/// Create a `<x15:bounds>` element (`BoundsTimelineRange`).
pub fn bounds_timeline_range() -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "bounds")
}

/// Create a `<x15:autoFilter>` element (`AutoFilter`).
pub fn auto_filter(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "autoFilter").with_children(children)
}

/// Create a `<x15:pivotTables>` element (`TimelineCachePivotTables`).
pub fn timeline_cache_pivot_tables(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotTables").with_children(children)
}

/// Create a `<x15:state>` element (`TimelineState`).
pub fn timeline_state(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "state").with_children(children)
}

/// Create a `<x15:pivotRow>` element (`PivotRow`).
pub fn pivot_row(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x15", NAMESPACE_URI, "pivotRow").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 76;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 73;
