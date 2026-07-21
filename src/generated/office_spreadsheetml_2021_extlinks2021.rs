//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2021_extlinks2021.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2021/extlinks2021` (prefix `xxl21`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2021/extlinks2021";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xxl21";

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

static ATTRS_EXTERNAL_BOOK_ALTERNATE_URLS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":driveId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":itemId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_EXTERNAL_BOOK_ALTERNATE_URLS: &[ChildInfo] = &[
    ChildInfo { name: "xxl21:CT_AlternateUrl/xxl21:absoluteUrl", property_name: Some("AbsoluteUrlAlternateUrl") },
    ChildInfo { name: "xxl21:CT_AlternateUrl/xxl21:relativeUrl", property_name: Some("RelativeUrlAlternateUrl") },
];
static ATTRS_ABSOLUTE_URL_ALTERNATE_URL: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_RELATIVE_URL_ALTERNATE_URL: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ExternalBookAlternateUrls", local_name: "alternateUrls", prefix: "xxl21", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTERNAL_BOOK_ALTERNATE_URLS, children: CHILDREN_EXTERNAL_BOOK_ALTERNATE_URLS },
    ElementInfo { class_name: "AbsoluteUrlAlternateUrl", local_name: "absoluteUrl", prefix: "xxl21", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ABSOLUTE_URL_ALTERNATE_URL, children: &[] },
    ElementInfo { class_name: "RelativeUrlAlternateUrl", local_name: "relativeUrl", prefix: "xxl21", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RELATIVE_URL_ALTERNATE_URL, children: &[] },
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

/// Create a `<xxl21:alternateUrls>` element (`ExternalBookAlternateUrls`).
pub fn external_book_alternate_urls(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xxl21", NAMESPACE_URI, "alternateUrls").with_children(children)
}

/// Create a `<xxl21:absoluteUrl>` element (`AbsoluteUrlAlternateUrl`).
pub fn absolute_url_alternate_url() -> OpenXmlElement {
    OpenXmlElement::new("xxl21", NAMESPACE_URI, "absoluteUrl")
}

/// Create a `<xxl21:relativeUrl>` element (`RelativeUrlAlternateUrl`).
pub fn relative_url_alternate_url() -> OpenXmlElement {
    OpenXmlElement::new("xxl21", NAMESPACE_URI, "relativeUrl")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
