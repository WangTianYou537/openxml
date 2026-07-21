//! Auto-generated from `schemas_microsoft_com_office_excel_2006_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/excel/2006/main` (prefix `xne`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/excel/2006/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xne";

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

static CHILDREN_MACROSHEET: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_SheetPr/x:sheetPr", property_name: Some("SheetProperties") },
    ChildInfo { name: "x:CT_SheetDimension/x:dimension", property_name: Some("SheetDimension") },
    ChildInfo { name: "x:CT_SheetViews/x:sheetViews", property_name: Some("SheetViews") },
    ChildInfo { name: "x:CT_SheetFormatPr/x:sheetFormatPr", property_name: Some("SheetFormatProperties") },
    ChildInfo { name: "x:CT_Cols/x:cols", property_name: None },
    ChildInfo { name: "x:CT_SheetData/x:sheetData", property_name: None },
    ChildInfo { name: "x:CT_SheetProtection/x:sheetProtection", property_name: None },
    ChildInfo { name: "x:CT_AutoFilter/x:autoFilter", property_name: None },
    ChildInfo { name: "x:CT_SortState/x:sortState", property_name: None },
    ChildInfo { name: "x:CT_DataConsolidate/x:dataConsolidate", property_name: None },
    ChildInfo { name: "x:CT_CustomSheetViews/x:customSheetViews", property_name: None },
    ChildInfo { name: "x:CT_PhoneticPr/x:phoneticPr", property_name: None },
    ChildInfo { name: "x:CT_ConditionalFormatting/x:conditionalFormatting", property_name: None },
    ChildInfo { name: "x:CT_PrintOptions/x:printOptions", property_name: None },
    ChildInfo { name: "x:CT_PageMargins/x:pageMargins", property_name: None },
    ChildInfo { name: "x:CT_PageSetup/x:pageSetup", property_name: None },
    ChildInfo { name: "x:CT_HeaderFooter/x:headerFooter", property_name: None },
    ChildInfo { name: "x:CT_PageBreak/x:rowBreaks", property_name: None },
    ChildInfo { name: "x:CT_PageBreak/x:colBreaks", property_name: None },
    ChildInfo { name: "x:CT_CustomProperties/x:customProperties", property_name: None },
    ChildInfo { name: "x:CT_Drawing/x:drawing", property_name: None },
    ChildInfo { name: "x:CT_LegacyDrawing/x:legacyDrawing", property_name: None },
    ChildInfo { name: "x:CT_LegacyDrawing/x:legacyDrawingHF", property_name: None },
    ChildInfo { name: "x:CT_SheetBackgroundPicture/x:picture", property_name: None },
    ChildInfo { name: "x:CT_OleObjects/x:oleObjects", property_name: None },
    ChildInfo { name: "x:CT_DrawingHF/x:drawingHF", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: None },
];
static CHILDREN_WORKSHEET_SORT_MAP: &[ChildInfo] = &[
    ChildInfo { name: "xne:CT_RowSortMap/xne:rowSortMap", property_name: Some("RowSortMap") },
    ChildInfo { name: "xne:CT_ColSortMap/xne:colSortMap", property_name: Some("ColumnSortMap") },
];
static ATTRS_ROW_SORT_MAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: Some("Ref"), type_name: "StringValue" },
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_ROW_SORT_MAP: &[ChildInfo] = &[
    ChildInfo { name: "xne:CT_SortMapItem/xne:row", property_name: None },
];
static ATTRS_COLUMN_SORT_MAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: Some("Ref"), type_name: "StringValue" },
    AttributeInfo { qname: ":count", property_name: Some("Count"), type_name: "UInt32Value" },
];
static CHILDREN_COLUMN_SORT_MAP: &[ChildInfo] = &[
    ChildInfo { name: "xne:CT_SortMapItem/xne:col", property_name: None },
];
static ATTRS_ROW_SORT_MAP_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":newVal", property_name: Some("NewVal"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":oldVal", property_name: Some("OldVal"), type_name: "UInt32Value" },
];
static ATTRS_COLUMN_SORT_MAP_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":newVal", property_name: Some("NewVal"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":oldVal", property_name: Some("OldVal"), type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Macrosheet", local_name: "macrosheet", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MACROSHEET },
    ElementInfo { class_name: "WorksheetSortMap", local_name: "worksheetSortMap", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WORKSHEET_SORT_MAP },
    ElementInfo { class_name: "ReferenceSequence", local_name: "sqref", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Formula", local_name: "f", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RowSortMap", local_name: "rowSortMap", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ROW_SORT_MAP, children: CHILDREN_ROW_SORT_MAP },
    ElementInfo { class_name: "ColumnSortMap", local_name: "colSortMap", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLUMN_SORT_MAP, children: CHILDREN_COLUMN_SORT_MAP },
    ElementInfo { class_name: "RowSortMapItem", local_name: "row", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROW_SORT_MAP_ITEM, children: &[] },
    ElementInfo { class_name: "ColumnSortMapItem", local_name: "col", prefix: "xne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLUMN_SORT_MAP_ITEM, children: &[] },
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

/// Create a `<xne:macrosheet>` element (`Macrosheet`).
pub fn macrosheet(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "macrosheet").with_children(children)
}

/// Create a `<xne:worksheetSortMap>` element (`WorksheetSortMap`).
pub fn worksheet_sort_map(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "worksheetSortMap").with_children(children)
}

/// Create a `<xne:sqref>` element (`ReferenceSequence`).
pub fn reference_sequence(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "sqref").with_text(value)
}

/// Create a `<xne:f>` element (`Formula`).
pub fn formula(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "f").with_text(value)
}

/// Create a `<xne:rowSortMap>` element (`RowSortMap`).
pub fn row_sort_map(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "rowSortMap").with_children(children)
}

/// Create a `<xne:colSortMap>` element (`ColumnSortMap`).
pub fn column_sort_map(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "colSortMap").with_children(children)
}

/// Create a `<xne:row>` element (`RowSortMapItem`).
pub fn row_sort_map_item() -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "row")
}

/// Create a `<xne:col>` element (`ColumnSortMapItem`).
pub fn column_sort_map_item() -> OpenXmlElement {
    OpenXmlElement::new("xne", NAMESPACE_URI, "col")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 9;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 8;
