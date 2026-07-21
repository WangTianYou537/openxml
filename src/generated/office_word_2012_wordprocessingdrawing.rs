//! Auto-generated from `schemas_microsoft_com_office_word_2012_wordprocessingDrawing.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2012/wordprocessingDrawing` (prefix `wp15`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2012/wordprocessingDrawing";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wp15";

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

static ATTRS_WEB_VIDEO_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":embeddedHtml", property_name: Some("EmbeddedHtml"), type_name: "StringValue" },
    AttributeInfo { qname: ":h", property_name: Some("Height"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "WebVideoProperty", local_name: "webVideoPr", prefix: "wp15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WEB_VIDEO_PROPERTY, children: &[] },
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

/// Create a `<wp15:webVideoPr>` element (`WebVideoProperty`).
pub fn web_video_property() -> OpenXmlElement {
    OpenXmlElement::new("wp15", NAMESPACE_URI, "webVideoPr")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 1;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 1;
