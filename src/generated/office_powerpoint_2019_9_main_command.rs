//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2019_9_main_command.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2019/9/main/command` (prefix `pc2`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2019/9/main/command";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "pc2";

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

static CHILDREN_COMMENT_V2_MONIKER_LIST: &[ChildInfo] = &[
    ChildInfo { name: "pc:CT_SlideMonikerList/pc:sldMkLst", property_name: Some("SlideMonikerList") },
    ChildInfo { name: "pc2:CT_CommentV2Moniker/pc2:cmMK", property_name: Some("CommentV2Moniker") },
];
static CHILDREN_COMMENT_REPLY_V2_MONIKER_LIST: &[ChildInfo] = &[
    ChildInfo { name: "pc2:CT_CommentV2MonikerList/pc2:cmMkLst", property_name: Some("CommentV2MonikerList") },
    ChildInfo { name: "pc2:CT_CommentReplyV2Moniker/pc2:cmRplyMk", property_name: Some("CommentReplyV2Moniker") },
];
static ATTRS_COMMENT_V2_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static ATTRS_COMMENT_REPLY_V2_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CommentV2MonikerList", local_name: "cmMkLst", prefix: "pc2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_V2_MONIKER_LIST },
    ElementInfo { class_name: "CommentReplyV2MonikerList", local_name: "cmRplyMkLst", prefix: "pc2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_REPLY_V2_MONIKER_LIST },
    ElementInfo { class_name: "CommentV2Moniker", local_name: "cmMK", prefix: "pc2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMENT_V2_MONIKER, children: &[] },
    ElementInfo { class_name: "CommentReplyV2Moniker", local_name: "cmRplyMk", prefix: "pc2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMENT_REPLY_V2_MONIKER, children: &[] },
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

/// Create a `<pc2:cmMkLst>` element (`CommentV2MonikerList`).
pub fn comment_v2_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc2", NAMESPACE_URI, "cmMkLst").with_children(children)
}

/// Create a `<pc2:cmRplyMkLst>` element (`CommentReplyV2MonikerList`).
pub fn comment_reply_v2_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc2", NAMESPACE_URI, "cmRplyMkLst").with_children(children)
}

/// Create a `<pc2:cmMK>` element (`CommentV2Moniker`).
pub fn comment_v2_moniker() -> OpenXmlElement {
    OpenXmlElement::new("pc2", NAMESPACE_URI, "cmMK")
}

/// Create a `<pc2:cmRplyMk>` element (`CommentReplyV2Moniker`).
pub fn comment_reply_v2_moniker() -> OpenXmlElement {
    OpenXmlElement::new("pc2", NAMESPACE_URI, "cmRplyMk")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
