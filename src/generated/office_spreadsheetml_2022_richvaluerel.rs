//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2022_richvaluerel.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2022/richvaluerel` (prefix `xlrvrel`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2022/richvaluerel";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xlrvrel";

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

static CHILDREN_RICH_VALUE_RELS: &[ChildInfo] = &[
    ChildInfo { name: "xlrvrel:CT_RichValueRelRelationship/xlrvrel:rel", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrvrel:extLst", property_name: None },
];
static ATTRS_RICH_VALUE_REL_RELATIONSHIP: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "RichValueRels", local_name: "richValueRels", prefix: "xlrvrel", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RICH_VALUE_RELS },
    ElementInfo { class_name: "RichValueRelRelationship", local_name: "rel", prefix: "xlrvrel", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_REL_RELATIONSHIP, children: &[] },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xlrvrel", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
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

/// Create a `<xlrvrel:richValueRels>` element (`RichValueRels`).
pub fn rich_value_rels(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrvrel", NAMESPACE_URI, "richValueRels").with_children(children)
}

/// Create a `<xlrvrel:rel>` element (`RichValueRelRelationship`).
pub fn rich_value_rel_relationship() -> OpenXmlElement {
    OpenXmlElement::new("xlrvrel", NAMESPACE_URI, "rel")
}

/// Create a `<xlrvrel:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrvrel", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
