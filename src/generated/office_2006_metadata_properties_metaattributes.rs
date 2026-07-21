//! Auto-generated from `schemas_microsoft_com_office_2006_metadata_properties_metaAttributes.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/metadata/properties/metaAttributes` (prefix `ma`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/metadata/properties/metaAttributes";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ma";

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

static ATTRS_DUMMY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":decimals", property_name: Some("Decimals"), type_name: "StringValue" },
    AttributeInfo { qname: ":default", property_name: Some("Default"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":displayName", property_name: Some("DisplayName"), type_name: "StringValue" },
    AttributeInfo { qname: ":fieldsID", property_name: Some("FieldsID"), type_name: "StringValue" },
    AttributeInfo { qname: ":format", property_name: Some("Format"), type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "StringValue" },
    AttributeInfo { qname: ":index", property_name: Some("Index"), type_name: "Int32Value" },
    AttributeInfo { qname: ":internalName", property_name: Some("InternalName"), type_name: "StringValue" },
    AttributeInfo { qname: ":LCID", property_name: Some("LCID"), type_name: "Int32Value" },
    AttributeInfo { qname: ":list", property_name: Some("List"), type_name: "StringValue" },
    AttributeInfo { qname: ":percentage", property_name: Some("Percentage"), type_name: "StringValue" },
    AttributeInfo { qname: ":readOnly", property_name: Some("ReadOnly"), type_name: "StringValue" },
    AttributeInfo { qname: ":requiredMultiChoice", property_name: Some("RequiredMultiChoice"), type_name: "StringValue" },
    AttributeInfo { qname: ":root", property_name: Some("Root"), type_name: "EnumValue" },
    AttributeInfo { qname: ":showField", property_name: Some("ShowField"), type_name: "StringValue" },
    AttributeInfo { qname: ":web", property_name: Some("Web"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Dummy", local_name: "DummyContentTypeElement", prefix: "ma", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DUMMY, children: &[] },
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

/// Create a `<ma:DummyContentTypeElement>` element (`Dummy`).
pub fn dummy() -> OpenXmlElement {
    OpenXmlElement::new("ma", NAMESPACE_URI, "DummyContentTypeElement")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 1;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 1;
