//! Auto-generated from `schemas_microsoft_com_office_2006_metadata_longProperties.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/metadata/longProperties` (prefix `lp`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/metadata/longProperties";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "lp";

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

static CHILDREN_LONG_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "lp:CT_LongProp/lp:LongProp", property_name: None },
];
static ATTRS_LONG_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "LongProperties", local_name: "LongProperties", prefix: "lp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LONG_PROPERTIES },
    ElementInfo { class_name: "LongProperty", local_name: "LongProp", prefix: "lp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_LONG_PROPERTY, children: &[] },
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

/// Create a `<lp:LongProperties>` element (`LongProperties`).
pub fn long_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("lp", NAMESPACE_URI, "LongProperties").with_children(children)
}

/// Create a `<lp:LongProp>` element (`LongProperty`).
pub fn long_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("lp", NAMESPACE_URI, "LongProp").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
