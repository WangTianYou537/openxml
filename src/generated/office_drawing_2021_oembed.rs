//! Auto-generated from `schemas_microsoft_com_office_drawing_2021_oembed.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2021/oembed` (prefix `aoe`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2021/oembed";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "aoe";

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

static ATTRS_O_EMBED_SHARED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":srcUrl", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: None, type_name: "StringValue" },
];
static CHILDREN_O_EMBED_SHARED: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/aoe:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "OEmbedShared", local_name: "oembedShared", prefix: "aoe", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_O_EMBED_SHARED, children: CHILDREN_O_EMBED_SHARED },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "aoe", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
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

/// Create a `<aoe:oembedShared>` element (`OEmbedShared`).
pub fn o_embed_shared(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("aoe", NAMESPACE_URI, "oembedShared").with_children(children)
}

/// Create a `<aoe:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("aoe", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
