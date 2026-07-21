//! Auto-generated from `schemas_microsoft_com_office_word_2010_wordprocessingDrawing.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing` (prefix `wp14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wp14";

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

static ATTRS_RELATIVE_WIDTH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":relativeFrom", property_name: Some("ObjectId"), type_name: "EnumValue" },
];
static CHILDREN_RELATIVE_WIDTH: &[ChildInfo] = &[
    ChildInfo { name: "a:ST_PositivePercentage/wp14:pctWidth", property_name: Some("PercentageWidth") },
];
static ATTRS_RELATIVE_HEIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":relativeFrom", property_name: Some("RelativeFrom"), type_name: "EnumValue" },
];
static CHILDREN_RELATIVE_HEIGHT: &[ChildInfo] = &[
    ChildInfo { name: "a:ST_PositivePercentage/wp14:pctHeight", property_name: Some("PercentageHeight") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "PercentagePositionHeightOffset", local_name: "pctPosHOffset", prefix: "wp14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PercentagePositionVerticalOffset", local_name: "pctPosVOffset", prefix: "wp14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RelativeWidth", local_name: "sizeRelH", prefix: "wp14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RELATIVE_WIDTH, children: CHILDREN_RELATIVE_WIDTH },
    ElementInfo { class_name: "RelativeHeight", local_name: "sizeRelV", prefix: "wp14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RELATIVE_HEIGHT, children: CHILDREN_RELATIVE_HEIGHT },
    ElementInfo { class_name: "PercentageWidth", local_name: "pctWidth", prefix: "wp14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PercentageHeight", local_name: "pctHeight", prefix: "wp14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
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

/// Create a `<wp14:pctPosHOffset>` element (`PercentagePositionHeightOffset`).
pub fn percentage_position_height_offset(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wp14", NAMESPACE_URI, "pctPosHOffset").with_text(value)
}

/// Create a `<wp14:pctPosVOffset>` element (`PercentagePositionVerticalOffset`).
pub fn percentage_position_vertical_offset(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wp14", NAMESPACE_URI, "pctPosVOffset").with_text(value)
}

/// Create a `<wp14:sizeRelH>` element (`RelativeWidth`).
pub fn relative_width(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp14", NAMESPACE_URI, "sizeRelH").with_children(children)
}

/// Create a `<wp14:sizeRelV>` element (`RelativeHeight`).
pub fn relative_height(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp14", NAMESPACE_URI, "sizeRelV").with_children(children)
}

/// Create a `<wp14:pctWidth>` element (`PercentageWidth`).
pub fn percentage_width(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wp14", NAMESPACE_URI, "pctWidth").with_text(value)
}

/// Create a `<wp14:pctHeight>` element (`PercentageHeight`).
pub fn percentage_height(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wp14", NAMESPACE_URI, "pctHeight").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 6;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 6;
