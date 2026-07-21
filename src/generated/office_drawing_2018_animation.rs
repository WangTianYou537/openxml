//! Auto-generated from `schemas_microsoft_com_office_drawing_2018_animation.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2018/animation` (prefix `aanim`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2018/animation";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "aanim";

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

static ATTRS_ANIMATION_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":length", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":count", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":auto", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":offset", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":st", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":end", property_name: None, type_name: "StringValue" },
];
static CHILDREN_ANIMATION_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/aanim:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "AnimationProperties", local_name: "animPr", prefix: "aanim", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANIMATION_PROPERTIES, children: CHILDREN_ANIMATION_PROPERTIES },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "aanim", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
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

/// Create a `<aanim:animPr>` element (`AnimationProperties`).
pub fn animation_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("aanim", NAMESPACE_URI, "animPr").with_children(children)
}

/// Create a `<aanim:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("aanim", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
