//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2023_msForms.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2023/msForms` (prefix `xlmsforms`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2023/msForms";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xlmsforms";

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

static ATTRS_QUESTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_QUESTION: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/xlmsforms:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_MS_FORM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":isFormConnected", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":maxResponseId", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":latestEventMarker", property_name: None, type_name: "StringValue" },
];
static CHILDREN_MS_FORM: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Xstring/xlmsforms:syncedQuestionId", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlmsforms:extLst", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Question", local_name: "question", prefix: "xlmsforms", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_QUESTION, children: CHILDREN_QUESTION },
    ElementInfo { class_name: "MsForm", local_name: "msForm", prefix: "xlmsforms", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MS_FORM, children: CHILDREN_MS_FORM },
    ElementInfo { class_name: "SyncedQuestionId", local_name: "syncedQuestionId", prefix: "xlmsforms", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xlmsforms", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
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

/// Create a `<xlmsforms:question>` element (`Question`).
pub fn question(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlmsforms", NAMESPACE_URI, "question").with_children(children)
}

/// Create a `<xlmsforms:msForm>` element (`MsForm`).
pub fn ms_form(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlmsforms", NAMESPACE_URI, "msForm").with_children(children)
}

/// Create a `<xlmsforms:syncedQuestionId>` element (`SyncedQuestionId`).
pub fn synced_question_id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xlmsforms", NAMESPACE_URI, "syncedQuestionId").with_text(value)
}

/// Create a `<xlmsforms:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlmsforms", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
