//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2012_roamingSettings.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2012/roamingSettings` (prefix `pRoam`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2012/roamingSettings";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "pRoam";

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

static CHILDREN_ROAMING_PROPERTY: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/pRoam:key", property_name: Some("Key") },
    ChildInfo { name: "xsd:string/pRoam:value", property_name: Some("Value") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Key", local_name: "key", prefix: "pRoam", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Value", local_name: "value", prefix: "pRoam", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RoamingProperty", local_name: "props", prefix: "pRoam", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ROAMING_PROPERTY },
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

/// Create a `<pRoam:key>` element (`Key`).
pub fn key(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("pRoam", NAMESPACE_URI, "key").with_text(value)
}

/// Create a `<pRoam:value>` element (`Value`).
pub fn value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("pRoam", NAMESPACE_URI, "value").with_text(value)
}

/// Create a `<pRoam:props>` element (`RoamingProperty`).
pub fn roaming_property(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pRoam", NAMESPACE_URI, "props").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
