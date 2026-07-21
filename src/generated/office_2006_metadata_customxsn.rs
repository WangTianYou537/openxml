//! Auto-generated from `schemas_microsoft_com_office_2006_metadata_customXsn.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/metadata/customXsn` (prefix `ntns`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/metadata/customXsn";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ntns";

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

static CHILDREN_CUSTOM_XSN: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/ntns:xsnLocation", property_name: Some("XsnLocation") },
    ChildInfo { name: "xsd:string/ntns:cached", property_name: Some("CachedView") },
    ChildInfo { name: "xsd:string/ntns:openByDefault", property_name: Some("OpenByDefault") },
    ChildInfo { name: "xsd:string/ntns:xsnScope", property_name: Some("Scope") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CustomXsn", local_name: "customXsn", prefix: "ntns", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_XSN },
    ElementInfo { class_name: "XsnLocation", local_name: "xsnLocation", prefix: "ntns", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CachedView", local_name: "cached", prefix: "ntns", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "OpenByDefault", local_name: "openByDefault", prefix: "ntns", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Scope", local_name: "xsnScope", prefix: "ntns", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
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

/// Create a `<ntns:customXsn>` element (`CustomXsn`).
pub fn custom_xsn(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ntns", NAMESPACE_URI, "customXsn").with_children(children)
}

/// Create a `<ntns:xsnLocation>` element (`XsnLocation`).
pub fn xsn_location(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ntns", NAMESPACE_URI, "xsnLocation").with_text(value)
}

/// Create a `<ntns:cached>` element (`CachedView`).
pub fn cached_view(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ntns", NAMESPACE_URI, "cached").with_text(value)
}

/// Create a `<ntns:openByDefault>` element (`OpenByDefault`).
pub fn open_by_default(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ntns", NAMESPACE_URI, "openByDefault").with_text(value)
}

/// Create a `<ntns:xsnScope>` element (`Scope`).
pub fn scope(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ntns", NAMESPACE_URI, "xsnScope").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 5;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 5;
