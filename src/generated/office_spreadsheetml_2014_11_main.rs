//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2014_11_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2014/11/main` (prefix `x16`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2014/11/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "x16";

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

static CHILDREN_MODEL_TIME_GROUPINGS: &[ChildInfo] = &[
    ChildInfo { name: "x16:CT_ModelTimeGrouping/x16:modelTimeGrouping", property_name: None },
];
static ATTRS_MODEL_TIME_GROUPING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tableName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":columnName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":columnId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_MODEL_TIME_GROUPING: &[ChildInfo] = &[
    ChildInfo { name: "x16:CT_CalculatedTimeColumn/x16:calculatedTimeColumn", property_name: None },
];
static ATTRS_CALCULATED_TIME_COLUMN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":columnName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":columnId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":contentType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":isSelected", property_name: None, type_name: "BooleanValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ModelTimeGroupings", local_name: "modelTimeGroupings", prefix: "x16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODEL_TIME_GROUPINGS },
    ElementInfo { class_name: "ModelTimeGrouping", local_name: "modelTimeGrouping", prefix: "x16", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MODEL_TIME_GROUPING, children: CHILDREN_MODEL_TIME_GROUPING },
    ElementInfo { class_name: "CalculatedTimeColumn", local_name: "calculatedTimeColumn", prefix: "x16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CALCULATED_TIME_COLUMN, children: &[] },
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

/// Create a `<x16:modelTimeGroupings>` element (`ModelTimeGroupings`).
pub fn model_time_groupings(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x16", NAMESPACE_URI, "modelTimeGroupings").with_children(children)
}

/// Create a `<x16:modelTimeGrouping>` element (`ModelTimeGrouping`).
pub fn model_time_grouping(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x16", NAMESPACE_URI, "modelTimeGrouping").with_children(children)
}

/// Create a `<x16:calculatedTimeColumn>` element (`CalculatedTimeColumn`).
pub fn calculated_time_column() -> OpenXmlElement {
    OpenXmlElement::new("x16", NAMESPACE_URI, "calculatedTimeColumn")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
