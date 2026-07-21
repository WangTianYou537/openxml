//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments` (prefix `xltc`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xltc";

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

static CHILDREN_PERSON_LIST: &[ChildInfo] = &[
    ChildInfo { name: "xltc:CT_Person/xltc:person", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xltc:extLst", property_name: None },
];
static CHILDREN_THREADED_COMMENTS: &[ChildInfo] = &[
    ChildInfo { name: "xltc:CT_ThreadedComment/xltc:threadedComment", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xltc:extLst", property_name: None },
];
static ATTRS_PERSON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":displayName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":providerId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PERSON: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/xltc:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_THREADED_COMMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":dT", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":personId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":parentId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":done", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_THREADED_COMMENT: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Xstring/xltc:text", property_name: Some("ThreadedCommentText") },
    ChildInfo { name: "xltc:CT_ThreadedCommentMentions/xltc:mentions", property_name: Some("ThreadedCommentMentions") },
    ChildInfo { name: "x:CT_ExtensionList/xltc:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_THREADED_COMMENT_MENTIONS: &[ChildInfo] = &[
    ChildInfo { name: "xltc:CT_Mention/xltc:mention", property_name: None },
];
static ATTRS_MENTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mentionpersonId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":mentionId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":startIndex", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":length", property_name: None, type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "PersonList", local_name: "personList", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PERSON_LIST },
    ElementInfo { class_name: "ThreadedComments", local_name: "ThreadedComments", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_THREADED_COMMENTS },
    ElementInfo { class_name: "Person", local_name: "person", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PERSON, children: CHILDREN_PERSON },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "ThreadedComment", local_name: "threadedComment", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_THREADED_COMMENT, children: CHILDREN_THREADED_COMMENT },
    ElementInfo { class_name: "ThreadedCommentText", local_name: "text", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ThreadedCommentMentions", local_name: "mentions", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_THREADED_COMMENT_MENTIONS },
    ElementInfo { class_name: "Mention", local_name: "mention", prefix: "xltc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MENTION, children: &[] },
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

/// Create a `<xltc:personList>` element (`PersonList`).
pub fn person_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "personList").with_children(children)
}

/// Create a `<xltc:ThreadedComments>` element (`ThreadedComments`).
pub fn threaded_comments(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "ThreadedComments").with_children(children)
}

/// Create a `<xltc:person>` element (`Person`).
pub fn person(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "person").with_children(children)
}

/// Create a `<xltc:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<xltc:threadedComment>` element (`ThreadedComment`).
pub fn threaded_comment(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "threadedComment").with_children(children)
}

/// Create a `<xltc:text>` element (`ThreadedCommentText`).
pub fn threaded_comment_text(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "text").with_text(value)
}

/// Create a `<xltc:mentions>` element (`ThreadedCommentMentions`).
pub fn threaded_comment_mentions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "mentions").with_children(children)
}

/// Create a `<xltc:mention>` element (`Mention`).
pub fn mention() -> OpenXmlElement {
    OpenXmlElement::new("xltc", NAMESPACE_URI, "mention")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 8;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 8;
