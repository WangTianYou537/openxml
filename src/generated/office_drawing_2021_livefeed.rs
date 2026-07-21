//! Auto-generated from `schemas_microsoft_com_office_drawing_2021_livefeed.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2021/livefeed` (prefix `alf`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2021/livefeed";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "alf";

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

static CHILDREN_BACKGROUND_NORMAL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/alf:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_BACKGROUND_REMOVED_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/alf:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_BACKGROUND_BLUR_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/alf:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_BACKGROUND_CUSTOM_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/alf:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_LIVE_FEED_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "alf:CT_LiveFeedBackgroundProperties/alf:backgroundProps", property_name: Some("LiveFeedBackgroundProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/alf:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static CHILDREN_LIVE_FEED_BACKGROUND_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "alf:CT_BackgroundNormalProperties/alf:Normal", property_name: None },
    ChildInfo { name: "alf:CT_BackgroundRemovedProperties/alf:Removed", property_name: None },
    ChildInfo { name: "alf:CT_BackgroundBlurProperties/alf:Blur", property_name: None },
    ChildInfo { name: "alf:CT_BackgroundCustomProperties/alf:Custom", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/alf:extLst", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "BackgroundNormalProperties", local_name: "Normal", prefix: "alf", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND_NORMAL_PROPERTIES },
    ElementInfo { class_name: "BackgroundRemovedProperties", local_name: "Removed", prefix: "alf", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND_REMOVED_PROPERTIES },
    ElementInfo { class_name: "BackgroundBlurProperties", local_name: "Blur", prefix: "alf", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND_BLUR_PROPERTIES },
    ElementInfo { class_name: "BackgroundCustomProperties", local_name: "Custom", prefix: "alf", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND_CUSTOM_PROPERTIES },
    ElementInfo { class_name: "LiveFeedProperties", local_name: "liveFeedProps", prefix: "alf", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIVE_FEED_PROPERTIES },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "alf", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "LiveFeedBackgroundProperties", local_name: "backgroundProps", prefix: "alf", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIVE_FEED_BACKGROUND_PROPERTIES },
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

/// Create a `<alf:Normal>` element (`BackgroundNormalProperties`).
pub fn background_normal_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("alf", NAMESPACE_URI, "Normal").with_children(children)
}

/// Create a `<alf:Removed>` element (`BackgroundRemovedProperties`).
pub fn background_removed_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("alf", NAMESPACE_URI, "Removed").with_children(children)
}

/// Create a `<alf:Blur>` element (`BackgroundBlurProperties`).
pub fn background_blur_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("alf", NAMESPACE_URI, "Blur").with_children(children)
}

/// Create a `<alf:Custom>` element (`BackgroundCustomProperties`).
pub fn background_custom_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("alf", NAMESPACE_URI, "Custom").with_children(children)
}

/// Create a `<alf:liveFeedProps>` element (`LiveFeedProperties`).
pub fn live_feed_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("alf", NAMESPACE_URI, "liveFeedProps").with_children(children)
}

/// Create a `<alf:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("alf", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<alf:backgroundProps>` element (`LiveFeedBackgroundProperties`).
pub fn live_feed_background_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("alf", NAMESPACE_URI, "backgroundProps").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 7;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 7;
