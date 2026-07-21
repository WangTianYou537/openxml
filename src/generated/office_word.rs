//! Auto-generated from `schemas-microsoft-com_office_word.json`.
//! Target namespace: `urn:schemas-microsoft-com:office:word` (prefix `w10`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "urn:schemas-microsoft-com:office:word";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "w10";

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

static ATTRS_TOP_BORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":width", property_name: Some("Width"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":shadow", property_name: Some("Shadow"), type_name: "TrueFalseValue" },
];
static ATTRS_LEFT_BORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":width", property_name: Some("Width"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":shadow", property_name: Some("Shadow"), type_name: "TrueFalseValue" },
];
static ATTRS_RIGHT_BORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":width", property_name: Some("Width"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":shadow", property_name: Some("Shadow"), type_name: "TrueFalseValue" },
];
static ATTRS_BOTTOM_BORDER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":width", property_name: Some("Width"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":shadow", property_name: Some("Shadow"), type_name: "TrueFalseValue" },
];
static ATTRS_TEXT_WRAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":side", property_name: Some("Side"), type_name: "EnumValue" },
    AttributeInfo { qname: ":anchorx", property_name: Some("AnchorX"), type_name: "EnumValue" },
    AttributeInfo { qname: ":anchory", property_name: Some("AnchorY"), type_name: "EnumValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "TopBorder", local_name: "bordertop", prefix: "w10", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOP_BORDER, children: &[] },
    ElementInfo { class_name: "LeftBorder", local_name: "borderleft", prefix: "w10", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEFT_BORDER, children: &[] },
    ElementInfo { class_name: "RightBorder", local_name: "borderright", prefix: "w10", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RIGHT_BORDER, children: &[] },
    ElementInfo { class_name: "BottomBorder", local_name: "borderbottom", prefix: "w10", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOTTOM_BORDER, children: &[] },
    ElementInfo { class_name: "TextWrap", local_name: "wrap", prefix: "w10", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TEXT_WRAP, children: &[] },
    ElementInfo { class_name: "AnchorLock", local_name: "anchorlock", prefix: "w10", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
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

/// Create a `<w10:bordertop>` element (`TopBorder`).
pub fn top_border() -> OpenXmlElement {
    OpenXmlElement::new("w10", NAMESPACE_URI, "bordertop")
}

/// Create a `<w10:borderleft>` element (`LeftBorder`).
pub fn left_border() -> OpenXmlElement {
    OpenXmlElement::new("w10", NAMESPACE_URI, "borderleft")
}

/// Create a `<w10:borderright>` element (`RightBorder`).
pub fn right_border() -> OpenXmlElement {
    OpenXmlElement::new("w10", NAMESPACE_URI, "borderright")
}

/// Create a `<w10:borderbottom>` element (`BottomBorder`).
pub fn bottom_border() -> OpenXmlElement {
    OpenXmlElement::new("w10", NAMESPACE_URI, "borderbottom")
}

/// Create a `<w10:wrap>` element (`TextWrap`).
pub fn text_wrap() -> OpenXmlElement {
    OpenXmlElement::new("w10", NAMESPACE_URI, "wrap")
}

/// Create a `<w10:anchorlock>` element (`AnchorLock`).
pub fn anchor_lock() -> OpenXmlElement {
    OpenXmlElement::new("w10", NAMESPACE_URI, "anchorlock")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 7;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 6;
