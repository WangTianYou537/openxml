//! Auto-generated from `schemas_openxmlformats_org_schemaLibrary_2006_main.json`.
//! Target namespace: `http://schemas.openxmlformats.org/schemaLibrary/2006/main` (prefix `sl`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/schemaLibrary/2006/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "sl";

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

static CHILDREN_SCHEMA_LIBRARY: &[ChildInfo] = &[
    ChildInfo { name: "sl:CT_Schema/sl:schema", property_name: None },
];
static ATTRS_SCHEMA: &[AttributeInfo] = &[
    AttributeInfo { qname: "sl:uri", property_name: Some("Uri"), type_name: "StringValue" },
    AttributeInfo { qname: "sl:manifestLocation", property_name: Some("ManifestLocation"), type_name: "StringValue" },
    AttributeInfo { qname: "sl:schemaLocation", property_name: Some("SchemaLocation"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "SchemaLibrary", local_name: "schemaLibrary", prefix: "sl", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCHEMA_LIBRARY },
    ElementInfo { class_name: "Schema", local_name: "schema", prefix: "sl", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SCHEMA, children: &[] },
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

/// Create a `<sl:schemaLibrary>` element (`SchemaLibrary`).
pub fn schema_library(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("sl", NAMESPACE_URI, "schemaLibrary").with_children(children)
}

/// Create a `<sl:schema>` element (`Schema`).
pub fn schema() -> OpenXmlElement {
    OpenXmlElement::new("sl", NAMESPACE_URI, "schema")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
