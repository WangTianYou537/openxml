//! Auto-generated from `schemas_microsoft_com_office_drawing_2010_compatibility.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2010/compatibility` (prefix `com14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2010/compatibility";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "com14";

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

static ATTRS_COMPATIBILITY_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CompatibilityShape", local_name: "compatSp", prefix: "com14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMPATIBILITY_SHAPE, children: &[] },
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

/// Create a `<com14:compatSp>` element (`CompatibilityShape`).
pub fn compatibility_shape() -> OpenXmlElement {
    OpenXmlElement::new("com14", NAMESPACE_URI, "compatSp")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 1;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 1;
