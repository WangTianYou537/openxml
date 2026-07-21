//! Auto-generated from `schemas_microsoft_com_office_2006_customDocumentInformationPanel.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/customDocumentInformationPanel` (prefix `cdip`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/customDocumentInformationPanel";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "cdip";

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

static CHILDREN_CUSTOM_PROPERTY_EDITORS: &[ChildInfo] = &[
    ChildInfo { name: "xsd:boolean/cdip:showOnOpen", property_name: Some("ShowOnOpen") },
    ChildInfo { name: "xsd:anyURI/cdip:defaultPropertyEditorNamespace", property_name: Some("DefaultPropertyEditorNamespace") },
    ChildInfo { name: "cdip:CT_CustomPropertyEditor/cdip:customPropertyEditor", property_name: None },
];
static CHILDREN_CUSTOM_PROPERTY_EDITOR: &[ChildInfo] = &[
    ChildInfo { name: "xsd:anyURI/cdip:XMLNamespace", property_name: Some("PropertyEditorNamespace") },
    ChildInfo { name: "xsd:string/cdip:XSNLocation", property_name: Some("XsnFileLocation") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CustomPropertyEditors", local_name: "customPropertyEditors", prefix: "cdip", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_PROPERTY_EDITORS },
    ElementInfo { class_name: "PropertyEditorNamespace", local_name: "XMLNamespace", prefix: "cdip", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DefaultPropertyEditorNamespace", local_name: "defaultPropertyEditorNamespace", prefix: "cdip", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "XsnFileLocation", local_name: "XSNLocation", prefix: "cdip", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ShowOnOpen", local_name: "showOnOpen", prefix: "cdip", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CustomPropertyEditor", local_name: "customPropertyEditor", prefix: "cdip", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_PROPERTY_EDITOR },
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

/// Create a `<cdip:customPropertyEditors>` element (`CustomPropertyEditors`).
pub fn custom_property_editors(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdip", NAMESPACE_URI, "customPropertyEditors").with_children(children)
}

/// Create a `<cdip:XMLNamespace>` element (`PropertyEditorNamespace`).
pub fn property_editor_namespace(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cdip", NAMESPACE_URI, "XMLNamespace").with_text(value)
}

/// Create a `<cdip:defaultPropertyEditorNamespace>` element (`DefaultPropertyEditorNamespace`).
pub fn default_property_editor_namespace(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cdip", NAMESPACE_URI, "defaultPropertyEditorNamespace").with_text(value)
}

/// Create a `<cdip:XSNLocation>` element (`XsnFileLocation`).
pub fn xsn_file_location(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cdip", NAMESPACE_URI, "XSNLocation").with_text(value)
}

/// Create a `<cdip:showOnOpen>` element (`ShowOnOpen`).
pub fn show_on_open(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cdip", NAMESPACE_URI, "showOnOpen").with_text(value)
}

/// Create a `<cdip:customPropertyEditor>` element (`CustomPropertyEditor`).
pub fn custom_property_editor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdip", NAMESPACE_URI, "customPropertyEditor").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 6;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 6;
