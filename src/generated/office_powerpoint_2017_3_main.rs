//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2017_3_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2017/3/main` (prefix `p173`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2017/3/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p173";

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

static ATTRS_TRACKS_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":displayLoc", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_TRACKS_INFO: &[ChildInfo] = &[
    ChildInfo { name: "p173:CT_TrackList/p173:trackLst", property_name: Some("TrackList") },
];
static ATTRS_TRACK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":lang", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
];
static CHILDREN_TRACK_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p173:CT_Track/p173:track", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "TracksInfo", local_name: "tracksInfo", prefix: "p173", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRACKS_INFO, children: CHILDREN_TRACKS_INFO },
    ElementInfo { class_name: "Track", local_name: "track", prefix: "p173", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TRACK, children: &[] },
    ElementInfo { class_name: "TrackList", local_name: "trackLst", prefix: "p173", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TRACK_LIST },
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

/// Create a `<p173:tracksInfo>` element (`TracksInfo`).
pub fn tracks_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p173", NAMESPACE_URI, "tracksInfo").with_children(children)
}

/// Create a `<p173:track>` element (`Track`).
pub fn track() -> OpenXmlElement {
    OpenXmlElement::new("p173", NAMESPACE_URI, "track")
}

/// Create a `<p173:trackLst>` element (`TrackList`).
pub fn track_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p173", NAMESPACE_URI, "trackLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
