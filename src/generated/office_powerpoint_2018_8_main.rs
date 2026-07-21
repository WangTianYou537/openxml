//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2018_8_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2018/8/main` (prefix `p188`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2018/8/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p188";

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

static CHILDREN_TEXT_BODY_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
];
static CHILDREN_COMMENT_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p188:CT_CommentPropertiesExtension/p:ext", property_name: None },
];
static CHILDREN_AUTHOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p188:CT_Author/p188:author", property_name: None },
];
static CHILDREN_COMMENT_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p188:CT_Comment/p188:cm", property_name: None },
];
static ATTRS_COMMENT_RELATIONSHIP: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static ATTRS_AUTHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":initials", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":userId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":providerId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_AUTHOR: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p188:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_COMMENT_REPLY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":authorId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":status", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":created", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":tags", property_name: None, type_name: "ListValue" },
    AttributeInfo { qname: ":likes", property_name: None, type_name: "ListValue" },
];
static CHILDREN_COMMENT_REPLY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBody/p188:txBody", property_name: Some("TextBodyType") },
    ChildInfo { name: "p188:CT_CommentPropertiesExtensionList/p188:extLst", property_name: Some("CommentPropertiesExtensionList") },
];
static ATTRS_POINT2_D_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static CHILDREN_COMMENT_REPLY_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p188:CT_CommentReply/p188:reply", property_name: None },
];
static ATTRS_COMMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":authorId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":status", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":created", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":tags", property_name: None, type_name: "ListValue" },
    AttributeInfo { qname: ":likes", property_name: None, type_name: "ListValue" },
    AttributeInfo { qname: ":startDate", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":dueDate", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":assignedTo", property_name: None, type_name: "ListValue" },
    AttributeInfo { qname: ":complete", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":priority", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":title", property_name: None, type_name: "StringValue" },
];
static CHILDREN_COMMENT: &[ChildInfo] = &[
    ChildInfo { name: "pc:CT_SlideMonikerList/pc:sldMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst", property_name: None },
    ChildInfo { name: "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst", property_name: None },
    ChildInfo { name: "oac:CT_DrawingElementMonikerList/oac:deMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TextBodyMonikerList/oac:txBodyMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TextCharRangeMonikerList/oac:txMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TableCellMonikerList/oac:tcMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TableRowMonikerList/oac:trMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TableColumnMonikerList/oac:gridColMkLst", property_name: None },
    ChildInfo { name: "p188:CT_CommentUnknownAnchor/p188:unknownAnchor", property_name: None },
    ChildInfo { name: "a:CT_Point2D/p188:pos", property_name: None },
    ChildInfo { name: "p188:CT_CommentReplyList/p188:replyLst", property_name: None },
    ChildInfo { name: "a:CT_TextBody/p188:txBody", property_name: None },
    ChildInfo { name: "p188:CT_CommentPropertiesExtensionList/p188:extLst", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CommentUnknownAnchor", local_name: "unknownAnchor", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TextBodyType", local_name: "txBody", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_BODY_TYPE },
    ElementInfo { class_name: "CommentPropertiesExtensionList", local_name: "extLst", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "AuthorList", local_name: "authorLst", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AUTHOR_LIST },
    ElementInfo { class_name: "CommentList", local_name: "cmLst", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_LIST },
    ElementInfo { class_name: "CommentRelationship", local_name: "commentRel", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMENT_RELATIONSHIP, children: &[] },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "Author", local_name: "author", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AUTHOR, children: CHILDREN_AUTHOR },
    ElementInfo { class_name: "CommentReply", local_name: "reply", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT_REPLY, children: CHILDREN_COMMENT_REPLY },
    ElementInfo { class_name: "Point2DType", local_name: "pos", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POINT2_D_TYPE, children: &[] },
    ElementInfo { class_name: "CommentReplyList", local_name: "replyLst", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_REPLY_LIST },
    ElementInfo { class_name: "Comment", local_name: "cm", prefix: "p188", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT, children: CHILDREN_COMMENT },
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

/// Create a `<p188:unknownAnchor>` element (`CommentUnknownAnchor`).
pub fn comment_unknown_anchor() -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "unknownAnchor")
}

/// Create a `<p188:txBody>` element (`TextBodyType`).
pub fn text_body_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "txBody").with_children(children)
}

/// Create a `<p188:extLst>` element (`CommentPropertiesExtensionList`).
pub fn comment_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p188:authorLst>` element (`AuthorList`).
pub fn author_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "authorLst").with_children(children)
}

/// Create a `<p188:cmLst>` element (`CommentList`).
pub fn comment_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "cmLst").with_children(children)
}

/// Create a `<p188:commentRel>` element (`CommentRelationship`).
pub fn comment_relationship() -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "commentRel")
}

/// Create a `<p188:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p188:author>` element (`Author`).
pub fn author(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "author").with_children(children)
}

/// Create a `<p188:reply>` element (`CommentReply`).
pub fn comment_reply(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "reply").with_children(children)
}

/// Create a `<p188:pos>` element (`Point2DType`).
pub fn point2_d_type() -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "pos")
}

/// Create a `<p188:replyLst>` element (`CommentReplyList`).
pub fn comment_reply_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "replyLst").with_children(children)
}

/// Create a `<p188:cm>` element (`Comment`).
pub fn comment(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p188", NAMESPACE_URI, "cm").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 12;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 12;
