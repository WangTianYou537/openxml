//! Auto-generated from `schemas_microsoft_com_office_drawing_2017_03_chart.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2017/03/chart` (prefix `c16r3`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2017/03/chart";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "c16r3";

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

static CHILDREN_DATA_DISPLAY_OPTIONS16: &[ChildInfo] = &[
    ChildInfo { name: "c16r3:CT_BooleanFalse/c16r3:dispNaAsBlank", property_name: Some("BooleanFalse") },
];
static ATTRS_BOOLEAN_FALSE: &[AttributeInfo] = &[
    AttributeInfo { qname: "c16r3:val", property_name: None, type_name: "BooleanValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "DataDisplayOptions16", local_name: "dataDisplayOptions16", prefix: "c16r3", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_DISPLAY_OPTIONS16 },
    ElementInfo { class_name: "BooleanFalse", local_name: "dispNaAsBlank", prefix: "c16r3", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOOLEAN_FALSE, children: &[] },
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

/// Create a `<c16r3:dataDisplayOptions16>` element (`DataDisplayOptions16`).
pub fn data_display_options16(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c16r3", NAMESPACE_URI, "dataDisplayOptions16").with_children(children)
}

/// Create a `<c16r3:dispNaAsBlank>` element (`BooleanFalse`).
pub fn boolean_false() -> OpenXmlElement {
    OpenXmlElement::new("c16r3", NAMESPACE_URI, "dispNaAsBlank")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
