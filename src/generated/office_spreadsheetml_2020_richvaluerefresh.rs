//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2020_richvaluerefresh.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2020/richvaluerefresh` (prefix `xlrvr`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2020/richvaluerefresh";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xlrvr";

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

static CHILDREN_RICH_VALUE_REFRESH_INTERVALS: &[ChildInfo] = &[
    ChildInfo { name: "xlrvr:CT_RichValueRefreshInterval/xlrvr:refreshInterval", property_name: None },
];
static ATTRS_RICH_VALUE_REFRESH_INTERVAL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":resourceIdInt", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":resourceIdStr", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":interval", property_name: None, type_name: "Int32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "RichValueRefreshIntervals", local_name: "refreshIntervals", prefix: "xlrvr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_VALUE_REFRESH_INTERVALS },
    ElementInfo { class_name: "RichValueRefreshInterval", local_name: "refreshInterval", prefix: "xlrvr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_REFRESH_INTERVAL, children: &[] },
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

/// Create a `<xlrvr:refreshIntervals>` element (`RichValueRefreshIntervals`).
pub fn rich_value_refresh_intervals(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrvr", NAMESPACE_URI, "refreshIntervals").with_children(children)
}

/// Create a `<xlrvr:refreshInterval>` element (`RichValueRefreshInterval`).
pub fn rich_value_refresh_interval() -> OpenXmlElement {
    OpenXmlElement::new("xlrvr", NAMESPACE_URI, "refreshInterval")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
