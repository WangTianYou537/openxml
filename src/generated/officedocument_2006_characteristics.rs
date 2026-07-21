//! Auto-generated from `schemas_openxmlformats_org_officeDocument_2006_characteristics.json`.
//! Target namespace: `http://schemas.openxmlformats.org/officeDocument/2006/characteristics` (prefix `ac`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/characteristics";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ac";

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

static CHILDREN_ADDITIONAL_CHARACTERISTICS_INFO: &[ChildInfo] = &[
    ChildInfo { name: "ac:CT_Characteristic/ac:characteristic", property_name: None },
];
static ATTRS_CHARACTERISTIC: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":relation", property_name: Some("Relation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
    AttributeInfo { qname: ":vocabulary", property_name: Some("Vocabulary"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "AdditionalCharacteristicsInfo", local_name: "additionalCharacteristics", prefix: "ac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ADDITIONAL_CHARACTERISTICS_INFO },
    ElementInfo { class_name: "Characteristic", local_name: "characteristic", prefix: "ac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHARACTERISTIC, children: &[] },
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

/// Create a `<ac:additionalCharacteristics>` element (`AdditionalCharacteristicsInfo`).
pub fn additional_characteristics_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ac", NAMESPACE_URI, "additionalCharacteristics").with_children(children)
}

/// Create a `<ac:characteristic>` element (`Characteristic`).
pub fn characteristic() -> OpenXmlElement {
    OpenXmlElement::new("ac", NAMESPACE_URI, "characteristic")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
