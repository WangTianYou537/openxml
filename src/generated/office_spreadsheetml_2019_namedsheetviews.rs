//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2019/namedsheetviews` (prefix `xnsv`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2019/namedsheetviews";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xnsv";

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

static CHILDREN_NAMED_SHEET_VIEWS: &[ChildInfo] = &[
    ChildInfo { name: "xnsv:CT_NamedSheetView/xnsv:namedSheetView", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xnsv:extLst", property_name: None },
];
static ATTRS_NAMED_SHEET_VIEW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_NAMED_SHEET_VIEW: &[ChildInfo] = &[
    ChildInfo { name: "xnsv:CT_NsvFilter/xnsv:nsvFilter", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xnsv:extLst", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_NSV_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":filterId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ref", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":tableId", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_NSV_FILTER: &[ChildInfo] = &[
    ChildInfo { name: "xnsv:CT_ColumnFilter/xnsv:columnFilter", property_name: None },
    ChildInfo { name: "xnsv:CT_SortRules/xnsv:sortRules", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xnsv:extLst", property_name: None },
];
static ATTRS_COLUMN_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":colId", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_COLUMN_FILTER: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Dxf/xnsv:dxf", property_name: Some("DifferentialFormatType") },
    ChildInfo { name: "x:CT_FilterColumn/xnsv:filter", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xnsv:extLst", property_name: None },
];
static ATTRS_SORT_RULES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sortMethod", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":caseSensitive", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_SORT_RULES: &[ChildInfo] = &[
    ChildInfo { name: "xnsv:CT_SortRule/xnsv:sortRule", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xnsv:extLst", property_name: None },
];
static CHILDREN_DIFFERENTIAL_FORMAT_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Font/x:font", property_name: Some("Font") },
    ChildInfo { name: "x:CT_NumFmt/x:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "x:CT_Fill/x:fill", property_name: Some("Fill") },
    ChildInfo { name: "x:CT_CellAlignment/x:alignment", property_name: Some("Alignment") },
    ChildInfo { name: "x:CT_Border/x:border", property_name: Some("Border") },
    ChildInfo { name: "x:CT_CellProtection/x:protection", property_name: Some("Protection") },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FILTER_COLUMN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":colId", property_name: Some("ColumnId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":hiddenButton", property_name: Some("HiddenButton"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showButton", property_name: Some("ShowButton"), type_name: "BooleanValue" },
];
static CHILDREN_FILTER_COLUMN: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Filters/x:filters", property_name: Some("Filters") },
    ChildInfo { name: "x:CT_Top10/x:top10", property_name: Some("Top10") },
    ChildInfo { name: "x14:CT_CustomFilters/x14:customFilters", property_name: Some("CustomFilters14") },
    ChildInfo { name: "x:CT_CustomFilters/x:customFilters", property_name: Some("CustomFilters") },
    ChildInfo { name: "x:CT_DynamicFilter/x:dynamicFilter", property_name: Some("DynamicFilter") },
    ChildInfo { name: "x:CT_ColorFilter/x:colorFilter", property_name: Some("ColorFilter") },
    ChildInfo { name: "x14:CT_IconFilter/x14:iconFilter", property_name: Some("IconFilter14") },
    ChildInfo { name: "x:CT_IconFilter/x:iconFilter", property_name: Some("IconFilter") },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SORT_RULE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":colId", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SORT_RULE: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Dxf/xnsv:dxf", property_name: Some("DifferentialFormatType") },
    ChildInfo { name: "x14:CT_SortCondition/xnsv:sortCondition", property_name: None },
    ChildInfo { name: "xlrd2:CT_RichSortCondition/xnsv:richSortCondition", property_name: None },
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "NamedSheetViews", local_name: "namedSheetViews", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NAMED_SHEET_VIEWS },
    ElementInfo { class_name: "NamedSheetView", local_name: "namedSheetView", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NAMED_SHEET_VIEW, children: CHILDREN_NAMED_SHEET_VIEW },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "NsvFilter", local_name: "nsvFilter", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NSV_FILTER, children: CHILDREN_NSV_FILTER },
    ElementInfo { class_name: "ColumnFilter", local_name: "columnFilter", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLUMN_FILTER, children: CHILDREN_COLUMN_FILTER },
    ElementInfo { class_name: "SortRules", local_name: "sortRules", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SORT_RULES, children: CHILDREN_SORT_RULES },
    ElementInfo { class_name: "DifferentialFormatType", local_name: "dxf", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIFFERENTIAL_FORMAT_TYPE },
    ElementInfo { class_name: "FilterColumn", local_name: "filter", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FILTER_COLUMN, children: CHILDREN_FILTER_COLUMN },
    ElementInfo { class_name: "SortRule", local_name: "sortRule", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SORT_RULE, children: CHILDREN_SORT_RULE },
    ElementInfo { class_name: "SortCondition", local_name: "sortCondition", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SORT_CONDITION, children: &[] },
    ElementInfo { class_name: "RichSortCondition", local_name: "richSortCondition", prefix: "xnsv", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_SORT_CONDITION, children: &[] },
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

/// Create a `<xnsv:namedSheetViews>` element (`NamedSheetViews`).
pub fn named_sheet_views(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "namedSheetViews").with_children(children)
}

/// Create a `<xnsv:namedSheetView>` element (`NamedSheetView`).
pub fn named_sheet_view(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "namedSheetView").with_children(children)
}

/// Create a `<xnsv:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<xnsv:nsvFilter>` element (`NsvFilter`).
pub fn nsv_filter(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "nsvFilter").with_children(children)
}

/// Create a `<xnsv:columnFilter>` element (`ColumnFilter`).
pub fn column_filter(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "columnFilter").with_children(children)
}

/// Create a `<xnsv:sortRules>` element (`SortRules`).
pub fn sort_rules(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "sortRules").with_children(children)
}

/// Create a `<xnsv:dxf>` element (`DifferentialFormatType`).
pub fn differential_format_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "dxf").with_children(children)
}

/// Create a `<xnsv:filter>` element (`FilterColumn`).
pub fn filter_column(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "filter").with_children(children)
}

/// Create a `<xnsv:sortRule>` element (`SortRule`).
pub fn sort_rule(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "sortRule").with_children(children)
}

/// Create a `<xnsv:sortCondition>` element (`SortCondition`).
pub fn sort_condition() -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "sortCondition")
}

/// Create a `<xnsv:richSortCondition>` element (`RichSortCondition`).
pub fn rich_sort_condition() -> OpenXmlElement {
    OpenXmlElement::new("xnsv", NAMESPACE_URI, "richSortCondition")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 11;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 11;
