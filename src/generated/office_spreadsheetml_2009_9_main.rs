//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2009_9_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2009/9/main` (prefix `x14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "x14";

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

static CHILDREN_CONDITIONAL_FORMATTINGS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_ConditionalFormatting/x14:conditionalFormatting", property_name: None },
];
static ATTRS_DATA_VALIDATIONS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":disablePrompts", property_name: Some("DisablePrompts"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":xWindow", property_name: Some("XWindow"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":yWindow", property_name: Some("YWindow"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_DATA_VALIDATIONS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_DataValidation/x14:dataValidation", property_name: None },
];
static CHILDREN_SPARKLINE_GROUPS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SparklineGroup/x14:sparklineGroup", property_name: None },
];
static CHILDREN_SLICER_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerRef/x14:slicer", property_name: None },
];
static CHILDREN_PROTECTED_RANGES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_ProtectedRange/x14:protectedRange", property_name: None },
];
static CHILDREN_IGNORED_ERRORS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_IgnoredError/x14:ignoredError", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: None },
];
static CHILDREN_DEFINED_NAMES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_DefinedName/x14:definedName", property_name: None },
];
static CHILDREN_PIVOT_CACHES: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_PivotCache/x:pivotCache", property_name: None },
];
static CHILDREN_SLICER_CACHES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerCache/x14:slicerCache", property_name: None },
];
static ATTRS_WORKBOOK_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":defaultImageDpi", property_name: Some("DefaultImageDpi"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":discardImageEditData", property_name: Some("DiscardImageEditData"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":accuracyVersion", property_name: Some("AccuracyVersion"), type_name: "UInt32Value" },
];
static ATTRS_CALCULATED_MEMBER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":displayFolder", property_name: Some("DisplayFolder"), type_name: "StringValue" },
    AttributeInfo { qname: ":flattenHierarchies", property_name: Some("FlattenHierarchies"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dynamicSet", property_name: Some("DynamicSet"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hierarchizeDistinct", property_name: Some("HierarchizeDistinct"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":mdxLong", property_name: Some("MdxLong"), type_name: "StringValue" },
];
static CHILDREN_CALCULATED_MEMBER: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_TupleSet/x14:tupleSet", property_name: Some("TupleSet") },
];
static ATTRS_CACHE_HIERARCHY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":flattenHierarchies", property_name: Some("FlattenHierarchies"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":measuresSet", property_name: Some("MeasuresSet"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hierarchizeDistinct", property_name: Some("HierarchizeDistinct"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":ignore", property_name: Some("Ignore"), type_name: "BooleanValue" },
];
static CHILDREN_CACHE_HIERARCHY: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SetLevels/x14:setLevels", property_name: Some("SetLevels") },
];
static ATTRS_DATA_FIELD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pivotShowAs", property_name: Some("PivotShowAs"), type_name: "EnumValue" },
    AttributeInfo { qname: ":sourceField", property_name: Some("SourceField"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":uniqueName", property_name: Some("UniqueName"), type_name: "StringValue" },
];
static ATTRS_PIVOT_FIELD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fillDownLabels", property_name: Some("FillDownLabels"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":ignore", property_name: Some("Ignore"), type_name: "BooleanValue" },
];
static ATTRS_PIVOT_TABLE_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fillDownLabelsDefault", property_name: Some("FillDownLabelsDefault"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":visualTotalsForSets", property_name: Some("VisualTotalsForSets"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":calculatedMembersInFilters", property_name: Some("CalculatedMembersInFilters"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":altText", property_name: Some("AltText"), type_name: "StringValue" },
    AttributeInfo { qname: ":altTextSummary", property_name: Some("AltTextSummary"), type_name: "StringValue" },
    AttributeInfo { qname: ":enableEdit", property_name: Some("EnableEdit"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":autoApply", property_name: Some("AutoApply"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":allocationMethod", property_name: Some("AllocationMethod"), type_name: "EnumValue" },
    AttributeInfo { qname: ":weightExpression", property_name: Some("WeightExpression"), type_name: "StringValue" },
    AttributeInfo { qname: ":hideValuesRow", property_name: Some("HideValuesRow"), type_name: "BooleanValue" },
];
static CHILDREN_PIVOT_TABLE_DEFINITION: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_PivotEdits/x14:pivotEdits", property_name: Some("PivotEdits") },
    ChildInfo { name: "x14:CT_PivotChanges/x14:pivotChanges", property_name: Some("PivotChanges") },
    ChildInfo { name: "x14:CT_ConditionalFormats/x14:conditionalFormats", property_name: Some("ConditionalFormats") },
];
static ATTRS_PIVOT_CACHE_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":slicerData", property_name: Some("SlicerData"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":pivotCacheId", property_name: Some("PivotCacheId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":supportSubqueryNonVisual", property_name: Some("SupportSubqueryNonVisual"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":supportSubqueryCalcMem", property_name: Some("SupportSubqueryCalcMem"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":supportAddCalcMems", property_name: Some("SupportAddCalcMems"), type_name: "BooleanValue" },
];
static ATTRS_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":culture", property_name: Some("Culture"), type_name: "StringValue" },
    AttributeInfo { qname: ":embeddedDataId", property_name: Some("EmbeddedDataId"), type_name: "StringValue" },
];
static CHILDREN_CONNECTION: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_CalculatedMembers/x14:calculatedMembers", property_name: Some("CalculatedMembers") },
];
static ATTRS_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":altText", property_name: Some("AltText"), type_name: "StringValue" },
    AttributeInfo { qname: ":altTextSummary", property_name: Some("AltTextSummary"), type_name: "StringValue" },
];
static ATTRS_SLICER_STYLES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":defaultSlicerStyle", property_name: Some("DefaultSlicerStyle"), type_name: "StringValue" },
];
static CHILDREN_SLICER_STYLES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerStyle/x14:slicerStyle", property_name: None },
];
static ATTRS_DIFFERENTIAL_FORMATS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_DIFFERENTIAL_FORMATS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Dxf/x:dxf", property_name: None },
];
static ATTRS_OLE_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":icon", property_name: Some("Icon"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":advise", property_name: Some("Advise"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":preferPic", property_name: Some("PreferPicture"), type_name: "BooleanValue" },
];
static CHILDREN_OLE_ITEM: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_DdeValues/x14:values", property_name: Some("DdeValues") },
];
static ATTRS_PIVOT_HIERARCHY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ignore", property_name: Some("Ignore"), type_name: "BooleanValue" },
];
static ATTRS_CACHE_FIELD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ignore", property_name: Some("Ignore"), type_name: "BooleanValue" },
];
static ATTRS_ICON_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":iconSet", property_name: Some("IconSet"), type_name: "EnumValue" },
    AttributeInfo { qname: ":iconId", property_name: Some("IconId"), type_name: "UInt32Value" },
];
static ATTRS_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_CUSTOM_FILTERS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":and", property_name: Some("And"), type_name: "BooleanValue" },
];
static CHILDREN_CUSTOM_FILTERS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_CustomFilter/x14:customFilter", property_name: None },
];
static ATTRS_SORT_CONDITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":descending", property_name: Some("Descending"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":sortBy", property_name: Some("SortBy"), type_name: "EnumValue" },
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
    AttributeInfo { qname: ":customList", property_name: Some("CustomList"), type_name: "StringValue" },
    AttributeInfo { qname: ":dxfId", property_name: Some("FormatId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":iconSet", property_name: Some("IconSet"), type_name: "EnumValue" },
    AttributeInfo { qname: ":iconId", property_name: Some("IconId"), type_name: "UInt32Value" },
];
static ATTRS_SOURCE_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
];
static ATTRS_DATASTORE_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_DATASTORE_ITEM: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FORM_CONTROL_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":objectType", property_name: Some("ObjectType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":checked", property_name: Some("Checked"), type_name: "EnumValue" },
    AttributeInfo { qname: ":colored", property_name: Some("Colored"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dropLines", property_name: Some("DropLines"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":dropStyle", property_name: Some("DropStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dx", property_name: Some("ScrollBarWidth"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":firstButton", property_name: Some("FirstButton"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fmlaGroup", property_name: Some("FmlaGroup"), type_name: "StringValue" },
    AttributeInfo { qname: ":fmlaLink", property_name: Some("FmlaLink"), type_name: "StringValue" },
    AttributeInfo { qname: ":fmlaRange", property_name: Some("FmlaRange"), type_name: "StringValue" },
    AttributeInfo { qname: ":fmlaTxbx", property_name: Some("FmlaTextbox"), type_name: "StringValue" },
    AttributeInfo { qname: ":horiz", property_name: Some("Horizontal"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":inc", property_name: Some("Incremental"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":justLastX", property_name: Some("JustLastX"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lockText", property_name: Some("LockText"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":max", property_name: Some("Max"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":min", property_name: Some("Min"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":multiSel", property_name: Some("MultipleSelection"), type_name: "StringValue" },
    AttributeInfo { qname: ":noThreeD", property_name: Some("NoThreeD"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noThreeD2", property_name: Some("NoThreeD2"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":page", property_name: Some("Page"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":sel", property_name: Some("Selected"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":seltype", property_name: Some("SelectionType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":textHAlign", property_name: Some("TextHorizontalAlign"), type_name: "EnumValue" },
    AttributeInfo { qname: ":textVAlign", property_name: Some("TextVerticalAlign"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":widthMin", property_name: Some("MinimumWidth"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":editVal", property_name: Some("EditVal"), type_name: "EnumValue" },
    AttributeInfo { qname: ":multiLine", property_name: Some("MultipleLines"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":verticalBar", property_name: Some("VerticalBar"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":passwordEdit", property_name: Some("PasswordEdit"), type_name: "BooleanValue" },
];
static CHILDREN_FORM_CONTROL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_ListItems/x14:itemLst", property_name: Some("ListItems") },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_SLICERS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_Slicer/x14:slicer", property_name: None },
];
static ATTRS_SLICER_CACHE_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":sourceName", property_name: Some("SourceName"), type_name: "StringValue" },
];
static CHILDREN_SLICER_CACHE_DEFINITION: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerCachePivotTables/x14:pivotTables", property_name: Some("SlicerCachePivotTables") },
    ChildInfo { name: "x14:CT_SlicerCacheData/x14:data", property_name: Some("SlicerCacheData") },
    ChildInfo { name: "x:CT_SlicerCacheDefinitionExtensionList/x14:extLst", property_name: Some("SlicerCacheDefinitionExtensionList") },
];
static ATTRS_CONDITIONAL_FORMATTING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pivot", property_name: Some("Pivot"), type_name: "BooleanValue" },
];
static CHILDREN_CONDITIONAL_FORMATTING: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_CfRule/x14:cfRule", property_name: None },
    ChildInfo { name: "xne:ST_Sqref/xne:sqref", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: None },
];
static ATTRS_CONDITIONAL_FORMATTING_RULE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":priority", property_name: Some("Priority"), type_name: "Int32Value" },
    AttributeInfo { qname: ":stopIfTrue", property_name: Some("StopIfTrue"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":aboveAverage", property_name: Some("AboveAverage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":percent", property_name: Some("Percent"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":bottom", property_name: Some("Bottom"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":operator", property_name: Some("Operator"), type_name: "EnumValue" },
    AttributeInfo { qname: ":text", property_name: Some("Text"), type_name: "StringValue" },
    AttributeInfo { qname: ":timePeriod", property_name: Some("TimePeriod"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rank", property_name: Some("Rank"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":stdDev", property_name: Some("StandardDeviation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":equalAverage", property_name: Some("EqualAverage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":activePresent", property_name: Some("ActivePresent"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_CONDITIONAL_FORMATTING_RULE: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: None },
    ChildInfo { name: "x14:CT_ColorScale/x14:colorScale", property_name: None },
    ChildInfo { name: "x14:CT_DataBar/x14:dataBar", property_name: None },
    ChildInfo { name: "x14:CT_IconSet/x14:iconSet", property_name: None },
    ChildInfo { name: "x:CT_Dxf/x14:dxf", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_DATA_VALIDATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":errorStyle", property_name: Some("ErrorStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":imeMode", property_name: Some("ImeMode"), type_name: "EnumValue" },
    AttributeInfo { qname: ":operator", property_name: Some("Operator"), type_name: "EnumValue" },
    AttributeInfo { qname: ":allowBlank", property_name: Some("AllowBlank"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showDropDown", property_name: Some("ShowDropDown"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showInputMessage", property_name: Some("ShowInputMessage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showErrorMessage", property_name: Some("ShowErrorMessage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":errorTitle", property_name: Some("ErrorTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":error", property_name: Some("Error"), type_name: "StringValue" },
    AttributeInfo { qname: ":promptTitle", property_name: Some("PromptTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":prompt", property_name: Some("Prompt"), type_name: "StringValue" },
];
static CHILDREN_DATA_VALIDATION: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_DataValidationFormula/x14:formula1", property_name: Some("DataValidationForumla1") },
    ChildInfo { name: "x14:CT_DataValidationFormula/x14:formula2", property_name: Some("DataValidationForumla2") },
    ChildInfo { name: "xne:ST_Sqref/xne:sqref", property_name: Some("ReferenceSequence") },
];
static CHILDREN_DATA_VALIDATION_FORUMLA1: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
];
static CHILDREN_DATA_VALIDATION_FORUMLA2: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
];
static ATTRS_SPARKLINE_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":manualMax", property_name: Some("ManualMax"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":manualMin", property_name: Some("ManualMin"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":lineWeight", property_name: Some("LineWeight"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dateAxis", property_name: Some("DateAxis"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":displayEmptyCellsAs", property_name: Some("DisplayEmptyCellsAs"), type_name: "EnumValue" },
    AttributeInfo { qname: ":markers", property_name: Some("Markers"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":high", property_name: Some("High"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":low", property_name: Some("Low"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":first", property_name: Some("First"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":last", property_name: Some("Last"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":negative", property_name: Some("Negative"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":displayXAxis", property_name: Some("DisplayXAxis"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":displayHidden", property_name: Some("DisplayHidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":minAxisType", property_name: Some("MinAxisType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":maxAxisType", property_name: Some("MaxAxisType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rightToLeft", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
];
static CHILDREN_SPARKLINE_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Color/x14:colorSeries", property_name: Some("SeriesColor") },
    ChildInfo { name: "x:CT_Color/x14:colorNegative", property_name: Some("NegativeColor") },
    ChildInfo { name: "x:CT_Color/x14:colorAxis", property_name: Some("AxisColor") },
    ChildInfo { name: "x:CT_Color/x14:colorMarkers", property_name: Some("MarkersColor") },
    ChildInfo { name: "x:CT_Color/x14:colorFirst", property_name: Some("FirstMarkerColor") },
    ChildInfo { name: "x:CT_Color/x14:colorLast", property_name: Some("LastMarkerColor") },
    ChildInfo { name: "x:CT_Color/x14:colorHigh", property_name: Some("HighMarkerColor") },
    ChildInfo { name: "x:CT_Color/x14:colorLow", property_name: Some("LowMarkerColor") },
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
    ChildInfo { name: "x14:CT_Sparklines/x14:sparklines", property_name: Some("Sparklines") },
];
static ATTRS_SERIES_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_NEGATIVE_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_AXIS_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_MARKERS_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_FIRST_MARKER_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_LAST_MARKER_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_HIGH_MARKER_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_LOW_MARKER_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_FILL_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_BORDER_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_NEGATIVE_FILL_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_NEGATIVE_BORDER_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static ATTRS_BAR_AXIS_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":auto", property_name: Some("Auto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":indexed", property_name: Some("Indexed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rgb", property_name: Some("Rgb"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":theme", property_name: Some("Theme"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":tint", property_name: Some("Tint"), type_name: "DoubleValue" },
];
static CHILDREN_SPARKLINES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_Sparkline/x14:sparkline", property_name: None },
];
static CHILDREN_SPARKLINE: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
    ChildInfo { name: "xne:ST_Sqref/xne:sqref", property_name: Some("ReferenceSequence") },
];
static ATTRS_SLICER_REF: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_SLICER_CACHE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_DEFINED_NAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_DEFINED_NAME: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_DefinedNameArgumentDescriptions/x14:argumentDescriptions", property_name: Some("ArgumentDescriptions") },
];
static ATTRS_ARGUMENT_DESCRIPTIONS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_ARGUMENT_DESCRIPTIONS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_DefinedNameArgumentDescription/x14:argumentDescription", property_name: None },
];
static ATTRS_ARGUMENT_DESCRIPTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":index", property_name: Some("Index"), type_name: "UInt32Value" },
];
static ATTRS_TUPLE_SET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rowCount", property_name: Some("RowCount"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":columnCount", property_name: Some("ColumnCount"), type_name: "UInt32Value" },
];
static CHILDREN_TUPLE_SET: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_TupleSetHeaders/x14:headers", property_name: Some("TupleSetHeaders") },
    ChildInfo { name: "x14:CT_TupleSetRows/x14:rows", property_name: Some("TupleSetRows") },
];
static CHILDREN_TUPLE_SET_HEADERS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_TupleSetHeader/x14:header", property_name: None },
];
static CHILDREN_TUPLE_SET_ROWS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_TupleSetRow/x14:row", property_name: None },
];
static ATTRS_TUPLE_SET_HEADER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueName", property_name: Some("UniqueName"), type_name: "StringValue" },
    AttributeInfo { qname: ":hierarchyName", property_name: Some("HierarchyName"), type_name: "StringValue" },
];
static CHILDREN_TUPLE_SET_ROW: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_TupleSetRowItem/x14:rowItem", property_name: None },
];
static ATTRS_TUPLE_SET_ROW_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":u", property_name: Some("UniqueName"), type_name: "StringValue" },
    AttributeInfo { qname: ":d", property_name: Some("DisplayName"), type_name: "StringValue" },
];
static ATTRS_SET_LEVEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":hierarchy", property_name: Some("Hierarchy"), type_name: "Int32Value" },
];
static ATTRS_SET_LEVELS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_SET_LEVELS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SetLevel/x14:setLevel", property_name: None },
];
static CHILDREN_COLOR_SCALE: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_Cfvo/x14:cfvo", property_name: None },
    ChildInfo { name: "x:CT_Color/x14:color", property_name: None },
];
static ATTRS_DATA_BAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":minLength", property_name: Some("MinLength"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":maxLength", property_name: Some("MaxLength"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":showValue", property_name: Some("ShowValue"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":border", property_name: Some("Border"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":gradient", property_name: Some("Gradient"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":direction", property_name: Some("Direction"), type_name: "EnumValue" },
    AttributeInfo { qname: ":negativeBarColorSameAsPositive", property_name: Some("NegativeBarColorSameAsPositive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":negativeBarBorderColorSameAsPositive", property_name: Some("NegativeBarBorderColorSameAsPositive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":axisPosition", property_name: Some("AxisPosition"), type_name: "EnumValue" },
];
static CHILDREN_DATA_BAR: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_Cfvo/x14:cfvo", property_name: None },
    ChildInfo { name: "x:CT_Color/x14:fillColor", property_name: None },
    ChildInfo { name: "x:CT_Color/x14:borderColor", property_name: None },
    ChildInfo { name: "x:CT_Color/x14:negativeFillColor", property_name: None },
    ChildInfo { name: "x:CT_Color/x14:negativeBorderColor", property_name: None },
    ChildInfo { name: "x:CT_Color/x14:axisColor", property_name: None },
];
static ATTRS_ICON_SET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":iconSet", property_name: Some("IconSetTypes"), type_name: "EnumValue" },
    AttributeInfo { qname: ":showValue", property_name: Some("ShowValue"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":percent", property_name: Some("Percent"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":reverse", property_name: Some("Reverse"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":custom", property_name: Some("Custom"), type_name: "BooleanValue" },
];
static CHILDREN_ICON_SET: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_Cfvo/x14:cfvo", property_name: None },
    ChildInfo { name: "x14:CT_CfIcon/x14:cfIcon", property_name: None },
];
static CHILDREN_DIFFERENTIAL_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Font/x:font", property_name: Some("Font") },
    ChildInfo { name: "x:CT_NumFmt/x:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "x:CT_Fill/x:fill", property_name: Some("Fill") },
    ChildInfo { name: "x:CT_CellAlignment/x:alignment", property_name: Some("Alignment") },
    ChildInfo { name: "x:CT_Border/x:border", property_name: Some("Border") },
    ChildInfo { name: "x:CT_CellProtection/x:protection", property_name: Some("Protection") },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_CONDITIONAL_FORMATTING_VALUE_OBJECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":gte", property_name: Some("GreaterThanOrEqual"), type_name: "BooleanValue" },
];
static CHILDREN_CONDITIONAL_FORMATTING_VALUE_OBJECT: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_CONDITIONAL_FORMATTING_ICON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":iconSet", property_name: Some("IconSet"), type_name: "EnumValue" },
    AttributeInfo { qname: ":iconId", property_name: Some("IconId"), type_name: "UInt32Value" },
];
static CHILDREN_PIVOT_EDITS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_PivotEdit/x14:pivotEdit", property_name: None },
];
static CHILDREN_PIVOT_CHANGES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_PivotChange/x14:pivotChange", property_name: None },
];
static ATTRS_CONDITIONAL_FORMATS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_CONDITIONAL_FORMATS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_ConditionalFormat/x14:conditionalFormat", property_name: None },
];
static ATTRS_CALCULATED_MEMBERS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_CALCULATED_MEMBERS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_CalculatedMember/x:calculatedMember", property_name: None },
];
static CHILDREN_PIVOT_EDIT: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_PivotUserEdit/x14:userEdit", property_name: Some("PivotUserEdit") },
    ChildInfo { name: "x14:CT_TupleItems/x14:tupleItems", property_name: Some("TupleItems") },
    ChildInfo { name: "x:CT_PivotArea/x14:pivotArea", property_name: Some("PivotArea") },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_PIVOT_USER_EDIT: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
    ChildInfo { name: "x14:CT_PivotEditValue/x14:editValue", property_name: Some("PivotEditValue") },
];
static CHILDREN_TUPLE_ITEMS: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Xstring/x14:tupleItem", property_name: None },
];
static ATTRS_PIVOT_AREA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":field", property_name: Some("Field"), type_name: "Int32Value" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dataOnly", property_name: Some("DataOnly"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":labelOnly", property_name: Some("LabelOnly"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":grandRow", property_name: Some("GrandRow"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":grandCol", property_name: Some("GrandColumn"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":cacheIndex", property_name: Some("CacheIndex"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":outline", property_name: Some("Outline"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":offset", property_name: Some("Offset"), type_name: "StringValue" },
    AttributeInfo { qname: ":collapsedLevelsAreSubtotals", property_name: Some("CollapsedLevelsAreSubtotals"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":axis", property_name: Some("Axis"), type_name: "EnumValue" },
    AttributeInfo { qname: ":fieldPosition", property_name: Some("FieldPosition"), type_name: "UInt32Value" },
];
static CHILDREN_PIVOT_AREA: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_PivotAreaReferences/x:references", property_name: Some("PivotAreaReferences") },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PIVOT_CHANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":allocationMethod", property_name: Some("AllocationMethod"), type_name: "EnumValue" },
    AttributeInfo { qname: ":weightExpression", property_name: Some("WeightExpression"), type_name: "StringValue" },
];
static CHILDREN_PIVOT_CHANGE: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_PivotEditValue/x14:editValue", property_name: Some("PivotEditValue") },
    ChildInfo { name: "x14:CT_TupleItems/x14:tupleItems", property_name: Some("TupleItems") },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PIVOT_EDIT_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":valueType", property_name: Some("ValueType"), type_name: "EnumValue" },
];
static CHILDREN_SLICER_STYLE_ELEMENTS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerStyleElement/x14:slicerStyleElement", property_name: None },
];
static ATTRS_DDE_VALUES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rows", property_name: Some("Rows"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":cols", property_name: Some("Columns"), type_name: "UInt32Value" },
];
static CHILDREN_DDE_VALUES: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_DdeValue/x:value", property_name: None },
];
static ATTRS_CONDITIONAL_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":scope", property_name: Some("Scope"), type_name: "EnumValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":priority", property_name: Some("Priority"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_CONDITIONAL_FORMAT: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_PivotAreas/x14:pivotAreas", property_name: Some("PivotAreas") },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PIVOT_AREAS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_PIVOT_AREAS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_PivotArea/x:pivotArea", property_name: None },
];
static ATTRS_SLICER_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_SLICER_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerStyleElements/x14:slicerStyleElements", property_name: Some("SlicerStyleElements") },
];
static ATTRS_SLICER_STYLE_ELEMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dxfId", property_name: Some("FormatId"), type_name: "UInt32Value" },
];
static ATTRS_IGNORED_ERROR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":evalError", property_name: Some("EvalError"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":twoDigitTextYear", property_name: Some("TwoDigitTextYear"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":numberStoredAsText", property_name: Some("NumberStoredAsText"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":formula", property_name: Some("Formula"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":formulaRange", property_name: Some("FormulaRange"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":unlockedFormula", property_name: Some("UnlockedFormula"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":emptyCellReference", property_name: Some("EmptyCellReference"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":listDataValidation", property_name: Some("ListDataValidation"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":calculatedColumn", property_name: Some("CalculatedColumn"), type_name: "BooleanValue" },
];
static CHILDREN_IGNORED_ERROR: &[ChildInfo] = &[
    ChildInfo { name: "xne:ST_Sqref/xne:sqref", property_name: Some("ReferenceSequence") },
];
static ATTRS_PROTECTED_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":password", property_name: Some("Password"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":algorithmName", property_name: Some("AlgorithmName"), type_name: "StringValue" },
    AttributeInfo { qname: ":hashValue", property_name: Some("HashValue"), type_name: "Base64BinaryValue" },
    AttributeInfo { qname: ":saltValue", property_name: Some("SaltValue"), type_name: "Base64BinaryValue" },
    AttributeInfo { qname: ":spinCount", property_name: Some("SpinCount"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":securityDescriptor", property_name: Some("SecurityDescriptor"), type_name: "StringValue" },
];
static CHILDREN_PROTECTED_RANGE: &[ChildInfo] = &[
    ChildInfo { name: "xne:ST_Sqref/xne:sqref", property_name: Some("ReferenceSequence") },
];
static ATTRS_CUSTOM_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":operator", property_name: Some("Operator"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_LIST_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "StringValue" },
];
static CHILDREN_LIST_ITEMS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_ListItem/x14:item", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: None },
];
static ATTRS_SLICER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":cache", property_name: Some("Cache"), type_name: "StringValue" },
    AttributeInfo { qname: ":caption", property_name: Some("Caption"), type_name: "StringValue" },
    AttributeInfo { qname: ":startItem", property_name: Some("StartItem"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":columnCount", property_name: Some("ColumnCount"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":showCaption", property_name: Some("ShowCaption"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":level", property_name: Some("Level"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":style", property_name: Some("Style"), type_name: "StringValue" },
    AttributeInfo { qname: ":lockedPosition", property_name: Some("LockedPosition"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":rowHeight", property_name: Some("RowHeight"), type_name: "UInt32Value" },
];
static CHILDREN_SLICER: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_OLAP_SLICER_CACHE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pivotCacheId", property_name: Some("PivotCacheId"), type_name: "UInt32Value" },
];
static CHILDREN_OLAP_SLICER_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheLevelsData/x14:levels", property_name: Some("OlapSlicerCacheLevelsData") },
    ChildInfo { name: "x14:CT_OlapSlicerCacheSelections/x14:selections", property_name: Some("OlapSlicerCacheSelections") },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TABULAR_SLICER_CACHE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pivotCacheId", property_name: Some("PivotCacheId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":sortOrder", property_name: Some("SortOrder"), type_name: "EnumValue" },
    AttributeInfo { qname: ":customListSort", property_name: Some("CustomListSort"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showMissing", property_name: Some("ShowMissing"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":crossFilter", property_name: Some("CrossFilter"), type_name: "EnumValue" },
];
static CHILDREN_TABULAR_SLICER_CACHE: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_TabularSlicerCacheItems/x14:items", property_name: Some("TabularSlicerCacheItems") },
    ChildInfo { name: "x:CT_ExtensionList/x14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SLICER_CACHE_PIVOT_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tabId", property_name: Some("TabId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static ATTRS_OLAP_SLICER_CACHE_ITEM_PARENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Name"), type_name: "StringValue" },
];
static ATTRS_OLAP_SLICER_CACHE_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":c", property_name: Some("DisplayName"), type_name: "StringValue" },
    AttributeInfo { qname: ":nd", property_name: Some("NonDisplay"), type_name: "BooleanValue" },
];
static CHILDREN_OLAP_SLICER_CACHE_ITEM: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheItemParent/x14:p", property_name: None },
];
static ATTRS_OLAP_SLICER_CACHE_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":startItem", property_name: Some("StartItem"), type_name: "UInt32Value" },
];
static CHILDREN_OLAP_SLICER_CACHE_RANGE: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheItem/x14:i", property_name: None },
];
static CHILDREN_OLAP_SLICER_CACHE_RANGES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheRange/x14:range", property_name: None },
];
static ATTRS_OLAP_SLICER_CACHE_LEVEL_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueName", property_name: Some("UniqueName"), type_name: "StringValue" },
    AttributeInfo { qname: ":sourceCaption", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":sortOrder", property_name: Some("SortOrder"), type_name: "EnumValue" },
    AttributeInfo { qname: ":crossFilter", property_name: Some("CrossFilter"), type_name: "EnumValue" },
];
static CHILDREN_OLAP_SLICER_CACHE_LEVEL_DATA: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheRanges/x14:ranges", property_name: Some("OlapSlicerCacheRanges") },
];
static ATTRS_OLAP_SLICER_CACHE_LEVELS_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_OLAP_SLICER_CACHE_LEVELS_DATA: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheLevelData/x14:level", property_name: None },
];
static ATTRS_OLAP_SLICER_CACHE_SELECTIONS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_OLAP_SLICER_CACHE_SELECTIONS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheSelection/x14:selection", property_name: None },
];
static ATTRS_OLAP_SLICER_CACHE_SELECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_OLAP_SLICER_CACHE_SELECTION: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCacheItemParent/x14:p", property_name: None },
];
static ATTRS_TABULAR_SLICER_CACHE_ITEMS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_TABULAR_SLICER_CACHE_ITEMS: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_TabularSlicerCacheItem/x14:i", property_name: None },
];
static ATTRS_TABULAR_SLICER_CACHE_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("Atom"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":s", property_name: Some("IsSelected"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":nd", property_name: Some("NonDisplay"), type_name: "BooleanValue" },
];
static CHILDREN_SLICER_CACHE_PIVOT_TABLES: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_SlicerCachePivotTable/x14:pivotTable", property_name: None },
];
static CHILDREN_SLICER_CACHE_DATA: &[ChildInfo] = &[
    ChildInfo { name: "x14:CT_OlapSlicerCache/x14:olap", property_name: Some("OlapSlicerCache") },
    ChildInfo { name: "x14:CT_TabularSlicerCache/x14:tabular", property_name: Some("TabularSlicerCache") },
];
static CHILDREN_SLICER_CACHE_DEFINITION_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_SlicerCacheDefinitionExtension/x:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ConditionalFormattings", local_name: "conditionalFormattings", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONDITIONAL_FORMATTINGS },
    ElementInfo { class_name: "DataValidations", local_name: "dataValidations", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_VALIDATIONS, children: CHILDREN_DATA_VALIDATIONS },
    ElementInfo { class_name: "SparklineGroups", local_name: "sparklineGroups", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SPARKLINE_GROUPS },
    ElementInfo { class_name: "SlicerList", local_name: "slicerList", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_LIST },
    ElementInfo { class_name: "ProtectedRanges", local_name: "protectedRanges", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PROTECTED_RANGES },
    ElementInfo { class_name: "IgnoredErrors", local_name: "ignoredErrors", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_IGNORED_ERRORS },
    ElementInfo { class_name: "DefinedNames", local_name: "definedNames", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DEFINED_NAMES },
    ElementInfo { class_name: "PivotCaches", local_name: "pivotCaches", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_CACHES },
    ElementInfo { class_name: "SlicerCaches", local_name: "slicerCaches", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_CACHES },
    ElementInfo { class_name: "WorkbookProperties", local_name: "workbookPr", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WORKBOOK_PROPERTIES, children: &[] },
    ElementInfo { class_name: "CalculatedMember", local_name: "calculatedMember", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CALCULATED_MEMBER, children: CHILDREN_CALCULATED_MEMBER },
    ElementInfo { class_name: "CacheHierarchy", local_name: "cacheHierarchy", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CACHE_HIERARCHY, children: CHILDREN_CACHE_HIERARCHY },
    ElementInfo { class_name: "DataField", local_name: "dataField", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_FIELD, children: &[] },
    ElementInfo { class_name: "PivotField", local_name: "pivotField", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_FIELD, children: &[] },
    ElementInfo { class_name: "PivotTableDefinition", local_name: "pivotTableDefinition", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_TABLE_DEFINITION, children: CHILDREN_PIVOT_TABLE_DEFINITION },
    ElementInfo { class_name: "PivotCacheDefinition", local_name: "pivotCacheDefinition", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_CACHE_DEFINITION, children: &[] },
    ElementInfo { class_name: "Connection", local_name: "connection", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTION, children: CHILDREN_CONNECTION },
    ElementInfo { class_name: "Table", local_name: "table", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TABLE, children: &[] },
    ElementInfo { class_name: "SlicerStyles", local_name: "slicerStyles", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLICER_STYLES, children: CHILDREN_SLICER_STYLES },
    ElementInfo { class_name: "DifferentialFormats", local_name: "dxfs", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIFFERENTIAL_FORMATS, children: CHILDREN_DIFFERENTIAL_FORMATS },
    ElementInfo { class_name: "OleItem", local_name: "oleItem", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLE_ITEM, children: CHILDREN_OLE_ITEM },
    ElementInfo { class_name: "PivotHierarchy", local_name: "pivotHierarchy", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_HIERARCHY, children: &[] },
    ElementInfo { class_name: "CacheField", local_name: "cacheField", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CACHE_FIELD, children: &[] },
    ElementInfo { class_name: "Id", local_name: "id", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "IconFilter", local_name: "iconFilter", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ICON_FILTER, children: &[] },
    ElementInfo { class_name: "Filter", local_name: "filter", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FILTER, children: &[] },
    ElementInfo { class_name: "CustomFilters", local_name: "customFilters", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CUSTOM_FILTERS, children: CHILDREN_CUSTOM_FILTERS },
    ElementInfo { class_name: "SortCondition", local_name: "sortCondition", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SORT_CONDITION, children: &[] },
    ElementInfo { class_name: "SourceConnection", local_name: "sourceConnection", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SOURCE_CONNECTION, children: &[] },
    ElementInfo { class_name: "DatastoreItem", local_name: "datastoreItem", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATASTORE_ITEM, children: CHILDREN_DATASTORE_ITEM },
    ElementInfo { class_name: "FormControlProperties", local_name: "formControlPr", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FORM_CONTROL_PROPERTIES, children: CHILDREN_FORM_CONTROL_PROPERTIES },
    ElementInfo { class_name: "Slicers", local_name: "slicers", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICERS },
    ElementInfo { class_name: "SlicerCacheDefinition", local_name: "slicerCacheDefinition", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLICER_CACHE_DEFINITION, children: CHILDREN_SLICER_CACHE_DEFINITION },
    ElementInfo { class_name: "ConditionalFormatting", local_name: "conditionalFormatting", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONDITIONAL_FORMATTING, children: CHILDREN_CONDITIONAL_FORMATTING },
    ElementInfo { class_name: "ConditionalFormattingRule", local_name: "cfRule", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONDITIONAL_FORMATTING_RULE, children: CHILDREN_CONDITIONAL_FORMATTING_RULE },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "DataValidation", local_name: "dataValidation", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_VALIDATION, children: CHILDREN_DATA_VALIDATION },
    ElementInfo { class_name: "DataValidationForumla1", local_name: "formula1", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_VALIDATION_FORUMLA1 },
    ElementInfo { class_name: "DataValidationForumla2", local_name: "formula2", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_VALIDATION_FORUMLA2 },
    ElementInfo { class_name: "SparklineGroup", local_name: "sparklineGroup", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPARKLINE_GROUP, children: CHILDREN_SPARKLINE_GROUP },
    ElementInfo { class_name: "SeriesColor", local_name: "colorSeries", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SERIES_COLOR, children: &[] },
    ElementInfo { class_name: "NegativeColor", local_name: "colorNegative", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NEGATIVE_COLOR, children: &[] },
    ElementInfo { class_name: "AxisColor", local_name: "colorAxis", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AXIS_COLOR, children: &[] },
    ElementInfo { class_name: "MarkersColor", local_name: "colorMarkers", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MARKERS_COLOR, children: &[] },
    ElementInfo { class_name: "FirstMarkerColor", local_name: "colorFirst", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FIRST_MARKER_COLOR, children: &[] },
    ElementInfo { class_name: "LastMarkerColor", local_name: "colorLast", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LAST_MARKER_COLOR, children: &[] },
    ElementInfo { class_name: "HighMarkerColor", local_name: "colorHigh", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIGH_MARKER_COLOR, children: &[] },
    ElementInfo { class_name: "LowMarkerColor", local_name: "colorLow", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LOW_MARKER_COLOR, children: &[] },
    ElementInfo { class_name: "Color", local_name: "color", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR, children: &[] },
    ElementInfo { class_name: "FillColor", local_name: "fillColor", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FILL_COLOR, children: &[] },
    ElementInfo { class_name: "BorderColor", local_name: "borderColor", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BORDER_COLOR, children: &[] },
    ElementInfo { class_name: "NegativeFillColor", local_name: "negativeFillColor", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NEGATIVE_FILL_COLOR, children: &[] },
    ElementInfo { class_name: "NegativeBorderColor", local_name: "negativeBorderColor", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NEGATIVE_BORDER_COLOR, children: &[] },
    ElementInfo { class_name: "BarAxisColor", local_name: "axisColor", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BAR_AXIS_COLOR, children: &[] },
    ElementInfo { class_name: "Sparklines", local_name: "sparklines", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SPARKLINES },
    ElementInfo { class_name: "Sparkline", local_name: "sparkline", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SPARKLINE },
    ElementInfo { class_name: "SlicerRef", local_name: "slicer", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLICER_REF, children: &[] },
    ElementInfo { class_name: "SlicerCache", local_name: "slicerCache", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLICER_CACHE, children: &[] },
    ElementInfo { class_name: "DefinedName", local_name: "definedName", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DEFINED_NAME, children: CHILDREN_DEFINED_NAME },
    ElementInfo { class_name: "ArgumentDescriptions", local_name: "argumentDescriptions", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ARGUMENT_DESCRIPTIONS, children: CHILDREN_ARGUMENT_DESCRIPTIONS },
    ElementInfo { class_name: "ArgumentDescription", local_name: "argumentDescription", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_ARGUMENT_DESCRIPTION, children: &[] },
    ElementInfo { class_name: "TupleSet", local_name: "tupleSet", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TUPLE_SET, children: CHILDREN_TUPLE_SET },
    ElementInfo { class_name: "TupleSetHeaders", local_name: "headers", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TUPLE_SET_HEADERS },
    ElementInfo { class_name: "TupleSetRows", local_name: "rows", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TUPLE_SET_ROWS },
    ElementInfo { class_name: "TupleSetHeader", local_name: "header", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TUPLE_SET_HEADER, children: &[] },
    ElementInfo { class_name: "TupleSetRow", local_name: "row", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TUPLE_SET_ROW },
    ElementInfo { class_name: "TupleSetRowItem", local_name: "rowItem", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TUPLE_SET_ROW_ITEM, children: &[] },
    ElementInfo { class_name: "SetLevel", local_name: "setLevel", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SET_LEVEL, children: &[] },
    ElementInfo { class_name: "SetLevels", local_name: "setLevels", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SET_LEVELS, children: CHILDREN_SET_LEVELS },
    ElementInfo { class_name: "ColorScale", local_name: "colorScale", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_SCALE },
    ElementInfo { class_name: "DataBar", local_name: "dataBar", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_BAR, children: CHILDREN_DATA_BAR },
    ElementInfo { class_name: "IconSet", local_name: "iconSet", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ICON_SET, children: CHILDREN_ICON_SET },
    ElementInfo { class_name: "DifferentialType", local_name: "dxf", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIFFERENTIAL_TYPE },
    ElementInfo { class_name: "ConditionalFormattingValueObject", local_name: "cfvo", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONDITIONAL_FORMATTING_VALUE_OBJECT, children: CHILDREN_CONDITIONAL_FORMATTING_VALUE_OBJECT },
    ElementInfo { class_name: "ConditionalFormattingIcon", local_name: "cfIcon", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONDITIONAL_FORMATTING_ICON, children: &[] },
    ElementInfo { class_name: "PivotEdits", local_name: "pivotEdits", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_EDITS },
    ElementInfo { class_name: "PivotChanges", local_name: "pivotChanges", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_CHANGES },
    ElementInfo { class_name: "ConditionalFormats", local_name: "conditionalFormats", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONDITIONAL_FORMATS, children: CHILDREN_CONDITIONAL_FORMATS },
    ElementInfo { class_name: "CalculatedMembers", local_name: "calculatedMembers", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CALCULATED_MEMBERS, children: CHILDREN_CALCULATED_MEMBERS },
    ElementInfo { class_name: "PivotEdit", local_name: "pivotEdit", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_EDIT },
    ElementInfo { class_name: "PivotUserEdit", local_name: "userEdit", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_USER_EDIT },
    ElementInfo { class_name: "TupleItems", local_name: "tupleItems", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TUPLE_ITEMS },
    ElementInfo { class_name: "PivotArea", local_name: "pivotArea", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_AREA, children: CHILDREN_PIVOT_AREA },
    ElementInfo { class_name: "PivotChange", local_name: "pivotChange", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_CHANGE, children: CHILDREN_PIVOT_CHANGE },
    ElementInfo { class_name: "PivotEditValue", local_name: "editValue", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_PIVOT_EDIT_VALUE, children: &[] },
    ElementInfo { class_name: "Xstring", local_name: "tupleItem", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SlicerStyleElements", local_name: "slicerStyleElements", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_STYLE_ELEMENTS },
    ElementInfo { class_name: "DdeValues", local_name: "values", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DDE_VALUES, children: CHILDREN_DDE_VALUES },
    ElementInfo { class_name: "ConditionalFormat", local_name: "conditionalFormat", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONDITIONAL_FORMAT, children: CHILDREN_CONDITIONAL_FORMAT },
    ElementInfo { class_name: "PivotAreas", local_name: "pivotAreas", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_AREAS, children: CHILDREN_PIVOT_AREAS },
    ElementInfo { class_name: "SlicerStyle", local_name: "slicerStyle", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLICER_STYLE, children: CHILDREN_SLICER_STYLE },
    ElementInfo { class_name: "SlicerStyleElement", local_name: "slicerStyleElement", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLICER_STYLE_ELEMENT, children: &[] },
    ElementInfo { class_name: "IgnoredError", local_name: "ignoredError", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_IGNORED_ERROR, children: CHILDREN_IGNORED_ERROR },
    ElementInfo { class_name: "ProtectedRange", local_name: "protectedRange", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PROTECTED_RANGE, children: CHILDREN_PROTECTED_RANGE },
    ElementInfo { class_name: "CustomFilter", local_name: "customFilter", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_FILTER, children: &[] },
    ElementInfo { class_name: "ListItem", local_name: "item", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LIST_ITEM, children: &[] },
    ElementInfo { class_name: "ListItems", local_name: "itemLst", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIST_ITEMS },
    ElementInfo { class_name: "Slicer", local_name: "slicer", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLICER, children: CHILDREN_SLICER },
    ElementInfo { class_name: "OlapSlicerCache", local_name: "olap", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE, children: CHILDREN_OLAP_SLICER_CACHE },
    ElementInfo { class_name: "TabularSlicerCache", local_name: "tabular", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABULAR_SLICER_CACHE, children: CHILDREN_TABULAR_SLICER_CACHE },
    ElementInfo { class_name: "SlicerCachePivotTable", local_name: "pivotTable", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLICER_CACHE_PIVOT_TABLE, children: &[] },
    ElementInfo { class_name: "OlapSlicerCacheItemParent", local_name: "p", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE_ITEM_PARENT, children: &[] },
    ElementInfo { class_name: "OlapSlicerCacheItem", local_name: "i", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE_ITEM, children: CHILDREN_OLAP_SLICER_CACHE_ITEM },
    ElementInfo { class_name: "OlapSlicerCacheRange", local_name: "range", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE_RANGE, children: CHILDREN_OLAP_SLICER_CACHE_RANGE },
    ElementInfo { class_name: "OlapSlicerCacheRanges", local_name: "ranges", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OLAP_SLICER_CACHE_RANGES },
    ElementInfo { class_name: "OlapSlicerCacheLevelData", local_name: "level", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE_LEVEL_DATA, children: CHILDREN_OLAP_SLICER_CACHE_LEVEL_DATA },
    ElementInfo { class_name: "OlapSlicerCacheLevelsData", local_name: "levels", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE_LEVELS_DATA, children: CHILDREN_OLAP_SLICER_CACHE_LEVELS_DATA },
    ElementInfo { class_name: "OlapSlicerCacheSelections", local_name: "selections", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE_SELECTIONS, children: CHILDREN_OLAP_SLICER_CACHE_SELECTIONS },
    ElementInfo { class_name: "OlapSlicerCacheSelection", local_name: "selection", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLAP_SLICER_CACHE_SELECTION, children: CHILDREN_OLAP_SLICER_CACHE_SELECTION },
    ElementInfo { class_name: "TabularSlicerCacheItems", local_name: "items", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABULAR_SLICER_CACHE_ITEMS, children: CHILDREN_TABULAR_SLICER_CACHE_ITEMS },
    ElementInfo { class_name: "TabularSlicerCacheItem", local_name: "i", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TABULAR_SLICER_CACHE_ITEM, children: &[] },
    ElementInfo { class_name: "SlicerCachePivotTables", local_name: "pivotTables", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_CACHE_PIVOT_TABLES },
    ElementInfo { class_name: "SlicerCacheData", local_name: "data", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_CACHE_DATA },
    ElementInfo { class_name: "SlicerCacheDefinitionExtensionList", local_name: "extLst", prefix: "x14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLICER_CACHE_DEFINITION_EXTENSION_LIST },
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

/// Create a `<x14:conditionalFormattings>` element (`ConditionalFormattings`).
pub fn conditional_formattings(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "conditionalFormattings").with_children(children)
}

/// Create a `<x14:dataValidations>` element (`DataValidations`).
pub fn data_validations(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "dataValidations").with_children(children)
}

/// Create a `<x14:sparklineGroups>` element (`SparklineGroups`).
pub fn sparkline_groups(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "sparklineGroups").with_children(children)
}

/// Create a `<x14:slicerList>` element (`SlicerList`).
pub fn slicer_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerList").with_children(children)
}

/// Create a `<x14:protectedRanges>` element (`ProtectedRanges`).
pub fn protected_ranges(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "protectedRanges").with_children(children)
}

/// Create a `<x14:ignoredErrors>` element (`IgnoredErrors`).
pub fn ignored_errors(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "ignoredErrors").with_children(children)
}

/// Create a `<x14:definedNames>` element (`DefinedNames`).
pub fn defined_names(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "definedNames").with_children(children)
}

/// Create a `<x14:pivotCaches>` element (`PivotCaches`).
pub fn pivot_caches(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotCaches").with_children(children)
}

/// Create a `<x14:slicerCaches>` element (`SlicerCaches`).
pub fn slicer_caches(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerCaches").with_children(children)
}

/// Create a `<x14:workbookPr>` element (`WorkbookProperties`).
pub fn workbook_properties() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "workbookPr")
}

/// Create a `<x14:calculatedMember>` element (`CalculatedMember`).
pub fn calculated_member(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "calculatedMember").with_children(children)
}

/// Create a `<x14:cacheHierarchy>` element (`CacheHierarchy`).
pub fn cache_hierarchy(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "cacheHierarchy").with_children(children)
}

/// Create a `<x14:dataField>` element (`DataField`).
pub fn data_field() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "dataField")
}

/// Create a `<x14:pivotField>` element (`PivotField`).
pub fn pivot_field() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotField")
}

/// Create a `<x14:pivotTableDefinition>` element (`PivotTableDefinition`).
pub fn pivot_table_definition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotTableDefinition").with_children(children)
}

/// Create a `<x14:pivotCacheDefinition>` element (`PivotCacheDefinition`).
pub fn pivot_cache_definition() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotCacheDefinition")
}

/// Create a `<x14:connection>` element (`Connection`).
pub fn connection(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "connection").with_children(children)
}

/// Create a `<x14:table>` element (`Table`).
pub fn table() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "table")
}

/// Create a `<x14:slicerStyles>` element (`SlicerStyles`).
pub fn slicer_styles(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerStyles").with_children(children)
}

/// Create a `<x14:dxfs>` element (`DifferentialFormats`).
pub fn differential_formats(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "dxfs").with_children(children)
}

/// Create a `<x14:oleItem>` element (`OleItem`).
pub fn ole_item(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "oleItem").with_children(children)
}

/// Create a `<x14:pivotHierarchy>` element (`PivotHierarchy`).
pub fn pivot_hierarchy() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotHierarchy")
}

/// Create a `<x14:cacheField>` element (`CacheField`).
pub fn cache_field() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "cacheField")
}

/// Create a `<x14:id>` element (`Id`).
pub fn id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "id").with_text(value)
}

/// Create a `<x14:iconFilter>` element (`IconFilter`).
pub fn icon_filter() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "iconFilter")
}

/// Create a `<x14:filter>` element (`Filter`).
pub fn filter() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "filter")
}

/// Create a `<x14:customFilters>` element (`CustomFilters`).
pub fn custom_filters(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "customFilters").with_children(children)
}

/// Create a `<x14:sortCondition>` element (`SortCondition`).
pub fn sort_condition() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "sortCondition")
}

/// Create a `<x14:sourceConnection>` element (`SourceConnection`).
pub fn source_connection() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "sourceConnection")
}

/// Create a `<x14:datastoreItem>` element (`DatastoreItem`).
pub fn datastore_item(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "datastoreItem").with_children(children)
}

/// Create a `<x14:formControlPr>` element (`FormControlProperties`).
pub fn form_control_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "formControlPr").with_children(children)
}

/// Create a `<x14:slicers>` element (`Slicers`).
pub fn slicers(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicers").with_children(children)
}

/// Create a `<x14:slicerCacheDefinition>` element (`SlicerCacheDefinition`).
pub fn slicer_cache_definition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerCacheDefinition").with_children(children)
}

/// Create a `<x14:conditionalFormatting>` element (`ConditionalFormatting`).
pub fn conditional_formatting(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "conditionalFormatting").with_children(children)
}

/// Create a `<x14:cfRule>` element (`ConditionalFormattingRule`).
pub fn conditional_formatting_rule(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "cfRule").with_children(children)
}

/// Create a `<x14:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<x14:dataValidation>` element (`DataValidation`).
pub fn data_validation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "dataValidation").with_children(children)
}

/// Create a `<x14:formula1>` element (`DataValidationForumla1`).
pub fn data_validation_forumla1(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "formula1").with_children(children)
}

/// Create a `<x14:formula2>` element (`DataValidationForumla2`).
pub fn data_validation_forumla2(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "formula2").with_children(children)
}

/// Create a `<x14:sparklineGroup>` element (`SparklineGroup`).
pub fn sparkline_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "sparklineGroup").with_children(children)
}

/// Create a `<x14:colorSeries>` element (`SeriesColor`).
pub fn series_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorSeries")
}

/// Create a `<x14:colorNegative>` element (`NegativeColor`).
pub fn negative_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorNegative")
}

/// Create a `<x14:colorAxis>` element (`AxisColor`).
pub fn axis_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorAxis")
}

/// Create a `<x14:colorMarkers>` element (`MarkersColor`).
pub fn markers_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorMarkers")
}

/// Create a `<x14:colorFirst>` element (`FirstMarkerColor`).
pub fn first_marker_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorFirst")
}

/// Create a `<x14:colorLast>` element (`LastMarkerColor`).
pub fn last_marker_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorLast")
}

/// Create a `<x14:colorHigh>` element (`HighMarkerColor`).
pub fn high_marker_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorHigh")
}

/// Create a `<x14:colorLow>` element (`LowMarkerColor`).
pub fn low_marker_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorLow")
}

/// Create a `<x14:color>` element (`Color`).
pub fn color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "color")
}

/// Create a `<x14:fillColor>` element (`FillColor`).
pub fn fill_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "fillColor")
}

/// Create a `<x14:borderColor>` element (`BorderColor`).
pub fn border_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "borderColor")
}

/// Create a `<x14:negativeFillColor>` element (`NegativeFillColor`).
pub fn negative_fill_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "negativeFillColor")
}

/// Create a `<x14:negativeBorderColor>` element (`NegativeBorderColor`).
pub fn negative_border_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "negativeBorderColor")
}

/// Create a `<x14:axisColor>` element (`BarAxisColor`).
pub fn bar_axis_color() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "axisColor")
}

/// Create a `<x14:sparklines>` element (`Sparklines`).
pub fn sparklines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "sparklines").with_children(children)
}

/// Create a `<x14:sparkline>` element (`Sparkline`).
pub fn sparkline(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "sparkline").with_children(children)
}

/// Create a `<x14:slicer>` element (`SlicerRef`).
pub fn slicer_ref() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicer")
}

/// Create a `<x14:slicerCache>` element (`SlicerCache`).
pub fn slicer_cache() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerCache")
}

/// Create a `<x14:definedName>` element (`DefinedName`).
pub fn defined_name(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "definedName").with_children(children)
}

/// Create a `<x14:argumentDescriptions>` element (`ArgumentDescriptions`).
pub fn argument_descriptions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "argumentDescriptions").with_children(children)
}

/// Create a `<x14:argumentDescription>` element (`ArgumentDescription`).
pub fn argument_description(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "argumentDescription").with_text(value)
}

/// Create a `<x14:tupleSet>` element (`TupleSet`).
pub fn tuple_set(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "tupleSet").with_children(children)
}

/// Create a `<x14:headers>` element (`TupleSetHeaders`).
pub fn tuple_set_headers(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "headers").with_children(children)
}

/// Create a `<x14:rows>` element (`TupleSetRows`).
pub fn tuple_set_rows(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "rows").with_children(children)
}

/// Create a `<x14:header>` element (`TupleSetHeader`).
pub fn tuple_set_header() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "header")
}

/// Create a `<x14:row>` element (`TupleSetRow`).
pub fn tuple_set_row(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "row").with_children(children)
}

/// Create a `<x14:rowItem>` element (`TupleSetRowItem`).
pub fn tuple_set_row_item() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "rowItem")
}

/// Create a `<x14:setLevel>` element (`SetLevel`).
pub fn set_level() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "setLevel")
}

/// Create a `<x14:setLevels>` element (`SetLevels`).
pub fn set_levels(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "setLevels").with_children(children)
}

/// Create a `<x14:colorScale>` element (`ColorScale`).
pub fn color_scale(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "colorScale").with_children(children)
}

/// Create a `<x14:dataBar>` element (`DataBar`).
pub fn data_bar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "dataBar").with_children(children)
}

/// Create a `<x14:iconSet>` element (`IconSet`).
pub fn icon_set(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "iconSet").with_children(children)
}

/// Create a `<x14:dxf>` element (`DifferentialType`).
pub fn differential_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "dxf").with_children(children)
}

/// Create a `<x14:cfvo>` element (`ConditionalFormattingValueObject`).
pub fn conditional_formatting_value_object(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "cfvo").with_children(children)
}

/// Create a `<x14:cfIcon>` element (`ConditionalFormattingIcon`).
pub fn conditional_formatting_icon() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "cfIcon")
}

/// Create a `<x14:pivotEdits>` element (`PivotEdits`).
pub fn pivot_edits(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotEdits").with_children(children)
}

/// Create a `<x14:pivotChanges>` element (`PivotChanges`).
pub fn pivot_changes(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotChanges").with_children(children)
}

/// Create a `<x14:conditionalFormats>` element (`ConditionalFormats`).
pub fn conditional_formats(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "conditionalFormats").with_children(children)
}

/// Create a `<x14:calculatedMembers>` element (`CalculatedMembers`).
pub fn calculated_members(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "calculatedMembers").with_children(children)
}

/// Create a `<x14:pivotEdit>` element (`PivotEdit`).
pub fn pivot_edit(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotEdit").with_children(children)
}

/// Create a `<x14:userEdit>` element (`PivotUserEdit`).
pub fn pivot_user_edit(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "userEdit").with_children(children)
}

/// Create a `<x14:tupleItems>` element (`TupleItems`).
pub fn tuple_items(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "tupleItems").with_children(children)
}

/// Create a `<x14:pivotArea>` element (`PivotArea`).
pub fn pivot_area(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotArea").with_children(children)
}

/// Create a `<x14:pivotChange>` element (`PivotChange`).
pub fn pivot_change(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotChange").with_children(children)
}

/// Create a `<x14:editValue>` element (`PivotEditValue`).
pub fn pivot_edit_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "editValue").with_text(value)
}

/// Create a `<x14:tupleItem>` element (`Xstring`).
pub fn xstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "tupleItem").with_text(value)
}

/// Create a `<x14:slicerStyleElements>` element (`SlicerStyleElements`).
pub fn slicer_style_elements(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerStyleElements").with_children(children)
}

/// Create a `<x14:values>` element (`DdeValues`).
pub fn dde_values(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "values").with_children(children)
}

/// Create a `<x14:conditionalFormat>` element (`ConditionalFormat`).
pub fn conditional_format(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "conditionalFormat").with_children(children)
}

/// Create a `<x14:pivotAreas>` element (`PivotAreas`).
pub fn pivot_areas(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotAreas").with_children(children)
}

/// Create a `<x14:slicerStyle>` element (`SlicerStyle`).
pub fn slicer_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerStyle").with_children(children)
}

/// Create a `<x14:slicerStyleElement>` element (`SlicerStyleElement`).
pub fn slicer_style_element() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicerStyleElement")
}

/// Create a `<x14:ignoredError>` element (`IgnoredError`).
pub fn ignored_error(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "ignoredError").with_children(children)
}

/// Create a `<x14:protectedRange>` element (`ProtectedRange`).
pub fn protected_range(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "protectedRange").with_children(children)
}

/// Create a `<x14:customFilter>` element (`CustomFilter`).
pub fn custom_filter() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "customFilter")
}

/// Create a `<x14:item>` element (`ListItem`).
pub fn list_item() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "item")
}

/// Create a `<x14:itemLst>` element (`ListItems`).
pub fn list_items(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "itemLst").with_children(children)
}

/// Create a `<x14:slicer>` element (`Slicer`).
pub fn slicer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "slicer").with_children(children)
}

/// Create a `<x14:olap>` element (`OlapSlicerCache`).
pub fn olap_slicer_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "olap").with_children(children)
}

/// Create a `<x14:tabular>` element (`TabularSlicerCache`).
pub fn tabular_slicer_cache(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "tabular").with_children(children)
}

/// Create a `<x14:pivotTable>` element (`SlicerCachePivotTable`).
pub fn slicer_cache_pivot_table() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotTable")
}

/// Create a `<x14:p>` element (`OlapSlicerCacheItemParent`).
pub fn olap_slicer_cache_item_parent() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "p")
}

/// Create a `<x14:i>` element (`OlapSlicerCacheItem`).
pub fn olap_slicer_cache_item(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "i").with_children(children)
}

/// Create a `<x14:range>` element (`OlapSlicerCacheRange`).
pub fn olap_slicer_cache_range(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "range").with_children(children)
}

/// Create a `<x14:ranges>` element (`OlapSlicerCacheRanges`).
pub fn olap_slicer_cache_ranges(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "ranges").with_children(children)
}

/// Create a `<x14:level>` element (`OlapSlicerCacheLevelData`).
pub fn olap_slicer_cache_level_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "level").with_children(children)
}

/// Create a `<x14:levels>` element (`OlapSlicerCacheLevelsData`).
pub fn olap_slicer_cache_levels_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "levels").with_children(children)
}

/// Create a `<x14:selections>` element (`OlapSlicerCacheSelections`).
pub fn olap_slicer_cache_selections(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "selections").with_children(children)
}

/// Create a `<x14:selection>` element (`OlapSlicerCacheSelection`).
pub fn olap_slicer_cache_selection(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "selection").with_children(children)
}

/// Create a `<x14:items>` element (`TabularSlicerCacheItems`).
pub fn tabular_slicer_cache_items(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "items").with_children(children)
}

/// Create a `<x14:i>` element (`TabularSlicerCacheItem`).
pub fn tabular_slicer_cache_item() -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "i")
}

/// Create a `<x14:pivotTables>` element (`SlicerCachePivotTables`).
pub fn slicer_cache_pivot_tables(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "pivotTables").with_children(children)
}

/// Create a `<x14:data>` element (`SlicerCacheData`).
pub fn slicer_cache_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "data").with_children(children)
}

/// Create a `<x14:extLst>` element (`SlicerCacheDefinitionExtensionList`).
pub fn slicer_cache_definition_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x14", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 116;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 114;
