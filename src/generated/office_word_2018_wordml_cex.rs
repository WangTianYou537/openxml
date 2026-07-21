//! Auto-generated from `schemas_microsoft_com_office_word_2018_wordml_cex.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2018/wordml/cex` (prefix `w16cex`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2018/wordml/cex";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "w16cex";

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

static CHILDREN_COMMENTS_EXTENSIBLE: &[ChildInfo] = &[
    ChildInfo { name: "w16cex:CT_CommentExtensible/w16cex:commentExtensible", property_name: None },
    ChildInfo { name: "w16cur:CT_ExtensionList/w16cex:extLst", property_name: None },
];
static ATTRS_COMMENT_EXTENSIBLE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w16cex:durableId", property_name: None, type_name: "HexBinaryValue" },
    AttributeInfo { qname: "w16cex:dateUtc", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: "w16cex:intelligentPlaceholder", property_name: None, type_name: "OnOffValue" },
];
static CHILDREN_COMMENT_EXTENSIBLE: &[ChildInfo] = &[
    ChildInfo { name: "w16cur:CT_ExtensionList/w16cex:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "w16cur:CT_Extension/w16cur:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CommentsExtensible", local_name: "commentsExtensible", prefix: "w16cex", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENTS_EXTENSIBLE },
    ElementInfo { class_name: "CommentExtensible", local_name: "commentExtensible", prefix: "w16cex", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT_EXTENSIBLE, children: CHILDREN_COMMENT_EXTENSIBLE },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "w16cex", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
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

/// Create a `<w16cex:commentsExtensible>` element (`CommentsExtensible`).
pub fn comments_extensible(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w16cex", NAMESPACE_URI, "commentsExtensible").with_children(children)
}

/// Create a `<w16cex:commentExtensible>` element (`CommentExtensible`).
pub fn comment_extensible(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w16cex", NAMESPACE_URI, "commentExtensible").with_children(children)
}

/// Create a `<w16cex:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w16cex", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
