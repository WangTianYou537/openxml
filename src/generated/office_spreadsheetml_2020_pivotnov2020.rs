//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2020_pivotNov2020.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2020/pivotNov2020` (prefix `xxpim`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2020/pivotNov2020";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xxpim";

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

static ATTRS_IGNORABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":version", property_name: None, type_name: "ByteValue" },
];
static ATTRS_DATA_FIELD_FUTURE_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":version", property_name: None, type_name: "ByteValue" },
    AttributeInfo { qname: ":sourceField", property_name: None, type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Xsdboolean", local_name: "implicitMeasureSupport", prefix: "xxpim", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Ignorable", local_name: "ignorableAfterVersion", prefix: "xxpim", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_IGNORABLE, children: &[] },
    ElementInfo { class_name: "DataFieldFutureData", local_name: "dataFieldFutureData", prefix: "xxpim", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_FIELD_FUTURE_DATA, children: &[] },
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

/// Create a `<xxpim:implicitMeasureSupport>` element (`Xsdboolean`).
pub fn xsdboolean(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xxpim", NAMESPACE_URI, "implicitMeasureSupport").with_text(value)
}

/// Create a `<xxpim:ignorableAfterVersion>` element (`Ignorable`).
pub fn ignorable() -> OpenXmlElement {
    OpenXmlElement::new("xxpim", NAMESPACE_URI, "ignorableAfterVersion")
}

/// Create a `<xxpim:dataFieldFutureData>` element (`DataFieldFutureData`).
pub fn data_field_future_data() -> OpenXmlElement {
    OpenXmlElement::new("xxpim", NAMESPACE_URI, "dataFieldFutureData")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
