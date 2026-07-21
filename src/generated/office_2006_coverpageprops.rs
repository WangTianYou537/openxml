//! Auto-generated from `schemas_microsoft_com_office_2006_coverPageProps.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/coverPageProps` (prefix `cppr`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/coverPageProps";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "cppr";

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

static CHILDREN_COVER_PAGE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "cppr:ST_PublishDate/cppr:PublishDate", property_name: Some("PublishDate") },
    ChildInfo { name: "xsd:string/cppr:Abstract", property_name: Some("DocumentAbstract") },
    ChildInfo { name: "xsd:string/cppr:CompanyAddress", property_name: Some("CompanyAddress") },
    ChildInfo { name: "xsd:string/cppr:CompanyPhone", property_name: Some("CompanyPhoneNumber") },
    ChildInfo { name: "xsd:string/cppr:CompanyFax", property_name: Some("CompanyFaxNumber") },
    ChildInfo { name: "xsd:string/cppr:CompanyEmail", property_name: Some("CompanyEmailAddress") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CoverPageProperties", local_name: "CoverPageProperties", prefix: "cppr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COVER_PAGE_PROPERTIES },
    ElementInfo { class_name: "PublishDate", local_name: "PublishDate", prefix: "cppr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DocumentAbstract", local_name: "Abstract", prefix: "cppr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CompanyAddress", local_name: "CompanyAddress", prefix: "cppr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CompanyPhoneNumber", local_name: "CompanyPhone", prefix: "cppr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CompanyFaxNumber", local_name: "CompanyFax", prefix: "cppr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CompanyEmailAddress", local_name: "CompanyEmail", prefix: "cppr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
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

/// Create a `<cppr:CoverPageProperties>` element (`CoverPageProperties`).
pub fn cover_page_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cppr", NAMESPACE_URI, "CoverPageProperties").with_children(children)
}

/// Create a `<cppr:PublishDate>` element (`PublishDate`).
pub fn publish_date(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cppr", NAMESPACE_URI, "PublishDate").with_text(value)
}

/// Create a `<cppr:Abstract>` element (`DocumentAbstract`).
pub fn document_abstract(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cppr", NAMESPACE_URI, "Abstract").with_text(value)
}

/// Create a `<cppr:CompanyAddress>` element (`CompanyAddress`).
pub fn company_address(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cppr", NAMESPACE_URI, "CompanyAddress").with_text(value)
}

/// Create a `<cppr:CompanyPhone>` element (`CompanyPhoneNumber`).
pub fn company_phone_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cppr", NAMESPACE_URI, "CompanyPhone").with_text(value)
}

/// Create a `<cppr:CompanyFax>` element (`CompanyFaxNumber`).
pub fn company_fax_number(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cppr", NAMESPACE_URI, "CompanyFax").with_text(value)
}

/// Create a `<cppr:CompanyEmail>` element (`CompanyEmailAddress`).
pub fn company_email_address(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cppr", NAMESPACE_URI, "CompanyEmail").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 7;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 7;
