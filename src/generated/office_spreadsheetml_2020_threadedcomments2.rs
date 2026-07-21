//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2020_threadedcomments2.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2020/threadedcomments2` (prefix `xltc2`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2020/threadedcomments2";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xltc2";

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

static ATTRS_COMMENT_HYPERLINK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":startIndex", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":length", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":url", property_name: None, type_name: "StringValue" },
];
static CHILDREN_COMMENT_HYPERLINK: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/xltc2:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "XsdunsignedInt", local_name: "checksum", prefix: "xltc2", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CommentHyperlink", local_name: "hyperlink", prefix: "xltc2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT_HYPERLINK, children: CHILDREN_COMMENT_HYPERLINK },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xltc2", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
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

/// Create a `<xltc2:checksum>` element (`XsdunsignedInt`).
pub fn xsdunsigned_int(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xltc2", NAMESPACE_URI, "checksum").with_text(value)
}

/// Create a `<xltc2:hyperlink>` element (`CommentHyperlink`).
pub fn comment_hyperlink(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc2", NAMESPACE_URI, "hyperlink").with_children(children)
}

/// Create a `<xltc2:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xltc2", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
