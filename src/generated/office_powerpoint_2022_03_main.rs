//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2022_03_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2022/03/main` (prefix `p223`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2022/03/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p223";

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

static CHILDREN_REACTIONS: &[ChildInfo] = &[
    ChildInfo { name: "p223:CT_Reaction/p223:rxn", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static ATTRS_REACTION_INSTANCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: None, type_name: "DateTimeValue" },
    AttributeInfo { qname: ":authorId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_REACTION_INSTANCE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p223:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_REACTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: None, type_name: "StringValue" },
];
static CHILDREN_REACTION: &[ChildInfo] = &[
    ChildInfo { name: "p223:CT_ReactionInstance/p223:instance", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Reactions", local_name: "reactions", prefix: "p223", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_REACTIONS },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "p223", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "ReactionInstance", local_name: "instance", prefix: "p223", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REACTION_INSTANCE, children: CHILDREN_REACTION_INSTANCE },
    ElementInfo { class_name: "Reaction", local_name: "rxn", prefix: "p223", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REACTION, children: CHILDREN_REACTION },
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

/// Create a `<p223:reactions>` element (`Reactions`).
pub fn reactions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p223", NAMESPACE_URI, "reactions").with_children(children)
}

/// Create a `<p223:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p223", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p223:instance>` element (`ReactionInstance`).
pub fn reaction_instance(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p223", NAMESPACE_URI, "instance").with_children(children)
}

/// Create a `<p223:rxn>` element (`Reaction`).
pub fn reaction(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p223", NAMESPACE_URI, "rxn").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
