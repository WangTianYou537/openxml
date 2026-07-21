//! Auto-generated from `schemas_microsoft_com_office_webextensions_taskpanes_2010_11.json`.
//! Target namespace: `http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11` (prefix `wetp`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wetp";

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

static CHILDREN_TASKPANES: &[ChildInfo] = &[
    ChildInfo { name: "wetp:CT_OsfTaskpane/wetp:taskpane", property_name: None },
];
static ATTRS_WEB_EXTENSION_PART_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_WEB_EXTENSION_TASKPANE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dockstate", property_name: Some("DockState"), type_name: "StringValue" },
    AttributeInfo { qname: ":visibility", property_name: Some("Visibility"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":width", property_name: Some("Width"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":row", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":locked", property_name: Some("Locked"), type_name: "BooleanValue" },
];
static CHILDREN_WEB_EXTENSION_TASKPANE: &[ChildInfo] = &[
    ChildInfo { name: "we:CT_WebExtensionPartRef/wetp:webextensionref", property_name: Some("WebExtensionPartReference") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wetp:extLst", property_name: Some("OfficeArtExtensionList") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Taskpanes", local_name: "taskpanes", prefix: "wetp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TASKPANES },
    ElementInfo { class_name: "WebExtensionPartReference", local_name: "webextensionref", prefix: "wetp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_PART_REFERENCE, children: &[] },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "wetp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "WebExtensionTaskpane", local_name: "taskpane", prefix: "wetp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_TASKPANE, children: CHILDREN_WEB_EXTENSION_TASKPANE },
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

/// Create a `<wetp:taskpanes>` element (`Taskpanes`).
pub fn taskpanes(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wetp", NAMESPACE_URI, "taskpanes").with_children(children)
}

/// Create a `<wetp:webextensionref>` element (`WebExtensionPartReference`).
pub fn web_extension_part_reference() -> OpenXmlElement {
    OpenXmlElement::new("wetp", NAMESPACE_URI, "webextensionref")
}

/// Create a `<wetp:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wetp", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<wetp:taskpane>` element (`WebExtensionTaskpane`).
pub fn web_extension_taskpane(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wetp", NAMESPACE_URI, "taskpane").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
