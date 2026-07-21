//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2017_richdata2.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2` (prefix `xlrd2`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata2";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xlrd2";

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

static CHILDREN_RICH_FILTER_COLUMN: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichFilters/xlrd2:filters", property_name: Some("RichFilters") },
    ChildInfo { name: "xlrd2:CT_RichTop10/xlrd2:top10", property_name: Some("RichTop10") },
    ChildInfo { name: "xlrd2:CT_CustomRichFilters/xlrd2:customFilters", property_name: Some("CustomRichFilters") },
    ChildInfo { name: "xlrd2:CT_DynamicRichFilter/xlrd2:dynamicFilter", property_name: Some("DynamicRichFilter") },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_RICH_SORT_CONDITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":richSortKey", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":descending", property_name: Some("Descending"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":sortBy", property_name: Some("SortBy"), type_name: "EnumValue" },
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
    AttributeInfo { qname: ":customList", property_name: Some("CustomList"), type_name: "StringValue" },
    AttributeInfo { qname: ":dxfId", property_name: Some("FormatId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":iconSet", property_name: Some("IconSet"), type_name: "EnumValue" },
    AttributeInfo { qname: ":iconId", property_name: Some("IconId"), type_name: "UInt32Value" },
];
static CHILDREN_SUPPORTING_PROPERTY_BAGS: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBagArrayData/xlrd2:spbArrays", property_name: Some("SupportingPropertyBagArrayData") },
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBagData/xlrd2:spbData", property_name: Some("SupportingPropertyBagData") },
];
static ATTRS_SUPPORTING_PROPERTY_BAG_STRUCTURES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SUPPORTING_PROPERTY_BAG_STRUCTURES: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBagStructure/xlrd2:s", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: None },
];
static ATTRS_ARRAY_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_ARRAY_DATA: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_Array/xlrd2:a", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: None },
];
static CHILDREN_RICH_STYLESHEET: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Dxfs/xlrd2:dxfs", property_name: Some("Dxfs") },
    ChildInfo { name: "xlrd2:CT_RichFormatProperties/xlrd2:richProperties", property_name: Some("RichFormatProperties") },
    ChildInfo { name: "xlrd2:CT_RichStyles/xlrd2:richStyles", property_name: Some("RichStyles") },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_RICH_VALUE_TYPES_INFO: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichValueGlobalType/xlrd2:global", property_name: Some("RichValueGlobalType") },
    ChildInfo { name: "xlrd2:CT_RichValueTypes/xlrd2:types", property_name: Some("RichValueTypes") },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_RICH_FILTERS: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichFilter/xlrd2:filter", property_name: None },
    ChildInfo { name: "xlrd2:CT_RichDateGroupItem/xlrd2:dateGroupItem", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: None },
];
static ATTRS_RICH_TOP10: &[AttributeInfo] = &[
    AttributeInfo { qname: ":key", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":top", property_name: Some("Top"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":percent", property_name: Some("Percent"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":filterVal", property_name: Some("FilterValue"), type_name: "DoubleValue" },
];
static ATTRS_CUSTOM_RICH_FILTERS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":and", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_CUSTOM_RICH_FILTERS: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_CustomRichFilter/xlrd2:customFilter", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: None },
];
static ATTRS_DYNAMIC_RICH_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":key", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":maxVal", property_name: Some("MaxVal"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":valIso", property_name: Some("ValIso"), type_name: "DateTimeValue" },
    AttributeInfo { qname: ":maxValIso", property_name: Some("MaxValIso"), type_name: "DateTimeValue" },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_RICH_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":key", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":blank", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":nodata", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_RICH_DATE_GROUP_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":key", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":year", property_name: Some("Year"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":month", property_name: Some("Month"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":day", property_name: Some("Day"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":hour", property_name: Some("Hour"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":minute", property_name: Some("Minute"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":second", property_name: Some("Second"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":dateTimeGrouping", property_name: Some("DateTimeGrouping"), type_name: "EnumValue" },
];
static ATTRS_CUSTOM_RICH_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":key", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":operator", property_name: Some("Operator"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_SUPPORTING_PROPERTY_BAG_ARRAY_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SUPPORTING_PROPERTY_BAG_ARRAY_DATA: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBagArray/xlrd2:a", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: None },
];
static ATTRS_SUPPORTING_PROPERTY_BAG_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SUPPORTING_PROPERTY_BAG_DATA: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBag/xlrd2:spb", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: None },
];
static ATTRS_SUPPORTING_PROPERTY_BAG: &[AttributeInfo] = &[
    AttributeInfo { qname: ":s", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SUPPORTING_PROPERTY_BAG: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBagValue/xlrd2:v", property_name: None },
];
static CHILDREN_SUPPORTING_PROPERTY_BAG_STRUCTURE: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBagKey/xlrd2:k", property_name: None },
];
static ATTRS_SUPPORTING_PROPERTY_BAG_KEY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
];
static ATTRS_SUPPORTING_PROPERTY_BAG_ARRAY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_SUPPORTING_PROPERTY_BAG_ARRAY: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_SupportingPropertyBagArrayValue/xlrd2:v", property_name: None },
];
static ATTRS_SUPPORTING_PROPERTY_BAG_ARRAY_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
];
static ATTRS_ARRAY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":r", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":c", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_ARRAY: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_ArrayValue/xlrd2:v", property_name: None },
];
static ATTRS_ARRAY_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
];
static ATTRS_DXFS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_DXFS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Dxf/x:dxf", property_name: None },
];
static CHILDREN_RICH_FORMAT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichFormatProperty/xlrd2:rPr", property_name: None },
];
static CHILDREN_RICH_STYLES: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichStyle/xlrd2:rSty", property_name: None },
];
static ATTRS_RICH_FORMAT_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
];
static ATTRS_RICH_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dxfid", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_RICH_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichStylePropertyValue/xlrd2:rpv", property_name: None },
];
static ATTRS_RICH_STYLE_PROPERTY_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":i", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_RICH_VALUE_GLOBAL_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags", property_name: Some("RichValueTypeKeyFlags") },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_RICH_VALUE_TYPES: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichValueType/xlrd2:type", property_name: None },
];
static ATTRS_RICH_VALUE_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
];
static CHILDREN_RICH_VALUE_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags", property_name: Some("RichValueTypeKeyFlags") },
    ChildInfo { name: "x:CT_ExtensionList/xlrd2:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_RICH_VALUE_TYPE_KEY_FLAGS: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichValueTypeReservedKey/xlrd2:key", property_name: None },
];
static ATTRS_RICH_VALUE_TYPE_RESERVED_KEY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
];
static CHILDREN_RICH_VALUE_TYPE_RESERVED_KEY: &[ChildInfo] = &[
    ChildInfo { name: "xlrd2:CT_RichValueTypeReservedKeyFlag/xlrd2:flag", property_name: None },
];
static ATTRS_RICH_VALUE_TYPE_RESERVED_KEY_FLAG: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":value", property_name: None, type_name: "BooleanValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "RichFilterColumn", local_name: "filterColumn", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_FILTER_COLUMN },
    ElementInfo { class_name: "RichSortCondition", local_name: "richSortCondition", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_SORT_CONDITION, children: &[] },
    ElementInfo { class_name: "SupportingPropertyBags", local_name: "supportingPropertyBags", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUPPORTING_PROPERTY_BAGS },
    ElementInfo { class_name: "SupportingPropertyBagStructures", local_name: "spbStructures", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SUPPORTING_PROPERTY_BAG_STRUCTURES, children: CHILDREN_SUPPORTING_PROPERTY_BAG_STRUCTURES },
    ElementInfo { class_name: "ArrayData", local_name: "arrayData", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ARRAY_DATA, children: CHILDREN_ARRAY_DATA },
    ElementInfo { class_name: "RichStylesheet", local_name: "richStyleSheet", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_STYLESHEET },
    ElementInfo { class_name: "RichValueTypesInfo", local_name: "rvTypesInfo", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_VALUE_TYPES_INFO },
    ElementInfo { class_name: "RichFilters", local_name: "filters", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_FILTERS },
    ElementInfo { class_name: "RichTop10", local_name: "top10", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_TOP10, children: &[] },
    ElementInfo { class_name: "CustomRichFilters", local_name: "customFilters", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CUSTOM_RICH_FILTERS, children: CHILDREN_CUSTOM_RICH_FILTERS },
    ElementInfo { class_name: "DynamicRichFilter", local_name: "dynamicFilter", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DYNAMIC_RICH_FILTER, children: &[] },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "RichFilter", local_name: "filter", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_FILTER, children: &[] },
    ElementInfo { class_name: "RichDateGroupItem", local_name: "dateGroupItem", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_DATE_GROUP_ITEM, children: &[] },
    ElementInfo { class_name: "CustomRichFilter", local_name: "customFilter", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_RICH_FILTER, children: &[] },
    ElementInfo { class_name: "SupportingPropertyBagArrayData", local_name: "spbArrays", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SUPPORTING_PROPERTY_BAG_ARRAY_DATA, children: CHILDREN_SUPPORTING_PROPERTY_BAG_ARRAY_DATA },
    ElementInfo { class_name: "SupportingPropertyBagData", local_name: "spbData", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SUPPORTING_PROPERTY_BAG_DATA, children: CHILDREN_SUPPORTING_PROPERTY_BAG_DATA },
    ElementInfo { class_name: "SupportingPropertyBag", local_name: "spb", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SUPPORTING_PROPERTY_BAG, children: CHILDREN_SUPPORTING_PROPERTY_BAG },
    ElementInfo { class_name: "SupportingPropertyBagValue", local_name: "v", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SupportingPropertyBagStructure", local_name: "s", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUPPORTING_PROPERTY_BAG_STRUCTURE },
    ElementInfo { class_name: "SupportingPropertyBagKey", local_name: "k", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SUPPORTING_PROPERTY_BAG_KEY, children: &[] },
    ElementInfo { class_name: "SupportingPropertyBagArray", local_name: "a", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SUPPORTING_PROPERTY_BAG_ARRAY, children: CHILDREN_SUPPORTING_PROPERTY_BAG_ARRAY },
    ElementInfo { class_name: "SupportingPropertyBagArrayValue", local_name: "v", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_SUPPORTING_PROPERTY_BAG_ARRAY_VALUE, children: &[] },
    ElementInfo { class_name: "Array", local_name: "a", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ARRAY, children: CHILDREN_ARRAY },
    ElementInfo { class_name: "ArrayValue", local_name: "v", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_ARRAY_VALUE, children: &[] },
    ElementInfo { class_name: "Dxfs", local_name: "dxfs", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DXFS, children: CHILDREN_DXFS },
    ElementInfo { class_name: "RichFormatProperties", local_name: "richProperties", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_FORMAT_PROPERTIES },
    ElementInfo { class_name: "RichStyles", local_name: "richStyles", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_STYLES },
    ElementInfo { class_name: "RichFormatProperty", local_name: "rPr", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_FORMAT_PROPERTY, children: &[] },
    ElementInfo { class_name: "RichStyle", local_name: "rSty", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RICH_STYLE, children: CHILDREN_RICH_STYLE },
    ElementInfo { class_name: "RichStylePropertyValue", local_name: "rpv", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_RICH_STYLE_PROPERTY_VALUE, children: &[] },
    ElementInfo { class_name: "RichValueGlobalType", local_name: "global", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_VALUE_GLOBAL_TYPE },
    ElementInfo { class_name: "RichValueTypes", local_name: "types", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_VALUE_TYPES },
    ElementInfo { class_name: "RichValueType", local_name: "type", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_TYPE, children: CHILDREN_RICH_VALUE_TYPE },
    ElementInfo { class_name: "RichValueTypeKeyFlags", local_name: "keyFlags", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_VALUE_TYPE_KEY_FLAGS },
    ElementInfo { class_name: "RichValueTypeReservedKey", local_name: "key", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_TYPE_RESERVED_KEY, children: CHILDREN_RICH_VALUE_TYPE_RESERVED_KEY },
    ElementInfo { class_name: "RichValueTypeReservedKeyFlag", local_name: "flag", prefix: "xlrd2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_TYPE_RESERVED_KEY_FLAG, children: &[] },
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

/// Create a `<xlrd2:filterColumn>` element (`RichFilterColumn`).
pub fn rich_filter_column(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "filterColumn").with_children(children)
}

/// Create a `<xlrd2:richSortCondition>` element (`RichSortCondition`).
pub fn rich_sort_condition() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "richSortCondition")
}

/// Create a `<xlrd2:supportingPropertyBags>` element (`SupportingPropertyBags`).
pub fn supporting_property_bags(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "supportingPropertyBags").with_children(children)
}

/// Create a `<xlrd2:spbStructures>` element (`SupportingPropertyBagStructures`).
pub fn supporting_property_bag_structures(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "spbStructures").with_children(children)
}

/// Create a `<xlrd2:arrayData>` element (`ArrayData`).
pub fn array_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "arrayData").with_children(children)
}

/// Create a `<xlrd2:richStyleSheet>` element (`RichStylesheet`).
pub fn rich_stylesheet(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "richStyleSheet").with_children(children)
}

/// Create a `<xlrd2:rvTypesInfo>` element (`RichValueTypesInfo`).
pub fn rich_value_types_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "rvTypesInfo").with_children(children)
}

/// Create a `<xlrd2:filters>` element (`RichFilters`).
pub fn rich_filters(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "filters").with_children(children)
}

/// Create a `<xlrd2:top10>` element (`RichTop10`).
pub fn rich_top10() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "top10")
}

/// Create a `<xlrd2:customFilters>` element (`CustomRichFilters`).
pub fn custom_rich_filters(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "customFilters").with_children(children)
}

/// Create a `<xlrd2:dynamicFilter>` element (`DynamicRichFilter`).
pub fn dynamic_rich_filter() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "dynamicFilter")
}

/// Create a `<xlrd2:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<xlrd2:filter>` element (`RichFilter`).
pub fn rich_filter() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "filter")
}

/// Create a `<xlrd2:dateGroupItem>` element (`RichDateGroupItem`).
pub fn rich_date_group_item() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "dateGroupItem")
}

/// Create a `<xlrd2:customFilter>` element (`CustomRichFilter`).
pub fn custom_rich_filter() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "customFilter")
}

/// Create a `<xlrd2:spbArrays>` element (`SupportingPropertyBagArrayData`).
pub fn supporting_property_bag_array_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "spbArrays").with_children(children)
}

/// Create a `<xlrd2:spbData>` element (`SupportingPropertyBagData`).
pub fn supporting_property_bag_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "spbData").with_children(children)
}

/// Create a `<xlrd2:spb>` element (`SupportingPropertyBag`).
pub fn supporting_property_bag(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "spb").with_children(children)
}

/// Create a `<xlrd2:v>` element (`SupportingPropertyBagValue`).
pub fn supporting_property_bag_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<xlrd2:s>` element (`SupportingPropertyBagStructure`).
pub fn supporting_property_bag_structure(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "s").with_children(children)
}

/// Create a `<xlrd2:k>` element (`SupportingPropertyBagKey`).
pub fn supporting_property_bag_key() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "k")
}

/// Create a `<xlrd2:a>` element (`SupportingPropertyBagArray`).
pub fn supporting_property_bag_array(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "a").with_children(children)
}

/// Create a `<xlrd2:v>` element (`SupportingPropertyBagArrayValue`).
pub fn supporting_property_bag_array_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<xlrd2:a>` element (`Array`).
pub fn array(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "a").with_children(children)
}

/// Create a `<xlrd2:v>` element (`ArrayValue`).
pub fn array_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<xlrd2:dxfs>` element (`Dxfs`).
pub fn dxfs(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "dxfs").with_children(children)
}

/// Create a `<xlrd2:richProperties>` element (`RichFormatProperties`).
pub fn rich_format_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "richProperties").with_children(children)
}

/// Create a `<xlrd2:richStyles>` element (`RichStyles`).
pub fn rich_styles(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "richStyles").with_children(children)
}

/// Create a `<xlrd2:rPr>` element (`RichFormatProperty`).
pub fn rich_format_property() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "rPr")
}

/// Create a `<xlrd2:rSty>` element (`RichStyle`).
pub fn rich_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "rSty").with_children(children)
}

/// Create a `<xlrd2:rpv>` element (`RichStylePropertyValue`).
pub fn rich_style_property_value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "rpv").with_text(value)
}

/// Create a `<xlrd2:global>` element (`RichValueGlobalType`).
pub fn rich_value_global_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "global").with_children(children)
}

/// Create a `<xlrd2:types>` element (`RichValueTypes`).
pub fn rich_value_types(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "types").with_children(children)
}

/// Create a `<xlrd2:type>` element (`RichValueType`).
pub fn rich_value_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "type").with_children(children)
}

/// Create a `<xlrd2:keyFlags>` element (`RichValueTypeKeyFlags`).
pub fn rich_value_type_key_flags(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "keyFlags").with_children(children)
}

/// Create a `<xlrd2:key>` element (`RichValueTypeReservedKey`).
pub fn rich_value_type_reserved_key(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "key").with_children(children)
}

/// Create a `<xlrd2:flag>` element (`RichValueTypeReservedKeyFlag`).
pub fn rich_value_type_reserved_key_flag() -> OpenXmlElement {
    OpenXmlElement::new("xlrd2", NAMESPACE_URI, "flag")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 37;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 37;
