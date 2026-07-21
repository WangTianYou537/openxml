//! Auto-generated from `schemas_microsoft_com_office_word_2016_wordml_cid.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2016/wordml/cid` (prefix `w16cid`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2016/wordml/cid";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "w16cid";

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

static CHILDREN_COMMENTS_IDS: &[ChildInfo] = &[
    ChildInfo { name: "w16cid:CT_CommentId/w16cid:commentId", property_name: None },
];
static ATTRS_COMMENT_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: "w16cid:paraId", property_name: None, type_name: "HexBinaryValue" },
    AttributeInfo { qname: "w16cid:durableId", property_name: None, type_name: "HexBinaryValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CommentsIds", local_name: "commentsIds", prefix: "w16cid", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENTS_IDS },
    ElementInfo { class_name: "CommentId", local_name: "commentId", prefix: "w16cid", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMENT_ID, children: &[] },
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

/// Create a `<w16cid:commentsIds>` element (`CommentsIds`).
pub fn comments_ids(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w16cid", NAMESPACE_URI, "commentsIds").with_children(children)
}

/// Create a `<w16cid:commentId>` element (`CommentId`).
pub fn comment_id() -> OpenXmlElement {
    OpenXmlElement::new("w16cid", NAMESPACE_URI, "commentId")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
