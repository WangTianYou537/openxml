//! Auto-generated from `schemas_microsoft_com_office_2006_metadata_contentType.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/metadata/contentType` (prefix `ct`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/metadata/contentType";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ct";

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

static ATTRS_CONTENT_TYPE_SCHEMA: &[AttributeInfo] = &[
    AttributeInfo { qname: "ct:_", property_name: Some("UnderScore"), type_name: "StringValue" },
    AttributeInfo { qname: "ma:_", property_name: Some("ReservedAttributeString"), type_name: "StringValue" },
    AttributeInfo { qname: "ma:contentTypeName", property_name: Some("ContentTypeName"), type_name: "StringValue" },
    AttributeInfo { qname: "ma:contentTypeID", property_name: Some("ContentTypeID"), type_name: "StringValue" },
    AttributeInfo { qname: "ma:contentTypeVersion", property_name: Some("ContentTypeVersion"), type_name: "Int32Value" },
    AttributeInfo { qname: "ma:contentTypeDescription", property_name: Some("ContentTypeDescription"), type_name: "StringValue" },
    AttributeInfo { qname: "ma:contentTypeScope", property_name: Some("ContentTypeScope"), type_name: "StringValue" },
    AttributeInfo { qname: "ma:versionID", property_name: Some("VersionID"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ContentTypeSchema", local_name: "contentTypeSchema", prefix: "ct", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTENT_TYPE_SCHEMA, children: &[] },
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

/// Create a `<ct:contentTypeSchema>` element (`ContentTypeSchema`).
pub fn content_type_schema(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ct", NAMESPACE_URI, "contentTypeSchema").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 1;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 1;
