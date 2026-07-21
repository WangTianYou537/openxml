//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2023_dataSourceVersioning.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2023/dataSourceVersioning` (prefix `xxdsv`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2023/dataSourceVersioning";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xxdsv";

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

static CHILDREN_VERSION_INFO: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/xxdsv:requiredFeature", property_name: None },
    ChildInfo { name: "xsd:string/xxdsv:lastRefreshFeature", property_name: None },
    ChildInfo { name: "xsd:string/xxdsv:lastEditFeature", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "VersionInfo", local_name: "versionInfo", prefix: "xxdsv", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VERSION_INFO },
    ElementInfo { class_name: "RequiredFeatureXsdstring", local_name: "requiredFeature", prefix: "xxdsv", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "LastRefreshFeatureXsdstring", local_name: "lastRefreshFeature", prefix: "xxdsv", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "LastEditFeatureXsdstring", local_name: "lastEditFeature", prefix: "xxdsv", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
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

/// Create a `<xxdsv:versionInfo>` element (`VersionInfo`).
pub fn version_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xxdsv", NAMESPACE_URI, "versionInfo").with_children(children)
}

/// Create a `<xxdsv:requiredFeature>` element (`RequiredFeatureXsdstring`).
pub fn required_feature_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xxdsv", NAMESPACE_URI, "requiredFeature").with_text(value)
}

/// Create a `<xxdsv:lastRefreshFeature>` element (`LastRefreshFeatureXsdstring`).
pub fn last_refresh_feature_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xxdsv", NAMESPACE_URI, "lastRefreshFeature").with_text(value)
}

/// Create a `<xxdsv:lastEditFeature>` element (`LastEditFeatureXsdstring`).
pub fn last_edit_feature_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xxdsv", NAMESPACE_URI, "lastEditFeature").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
