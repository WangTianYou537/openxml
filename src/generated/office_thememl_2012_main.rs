//! Auto-generated from `schemas_microsoft_com_office_thememl_2012_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/thememl/2012/main` (prefix `thm15`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/thememl/2012/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "thm15";

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

static ATTRS_THEME_FAMILY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":vid", property_name: None, type_name: "StringValue" },
];
static CHILDREN_THEME_FAMILY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/thm15:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_THEME_VARIANT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":vid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":cx", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Y"), type_name: "Int64Value" },
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_THEME_VARIANT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/thm15:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_THEME_VARIANT_LIST: &[ChildInfo] = &[
    ChildInfo { name: "thm15:CT_ThemeVariant/thm15:themeVariant", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ThemeFamily", local_name: "themeFamily", prefix: "thm15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_THEME_FAMILY, children: CHILDREN_THEME_FAMILY },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "thm15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "ThemeVariant", local_name: "themeVariant", prefix: "thm15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_THEME_VARIANT, children: CHILDREN_THEME_VARIANT },
    ElementInfo { class_name: "ThemeVariantList", local_name: "themeVariantLst", prefix: "thm15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_THEME_VARIANT_LIST },
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

/// Create a `<thm15:themeFamily>` element (`ThemeFamily`).
pub fn theme_family(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("thm15", NAMESPACE_URI, "themeFamily").with_children(children)
}

/// Create a `<thm15:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("thm15", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<thm15:themeVariant>` element (`ThemeVariant`).
pub fn theme_variant(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("thm15", NAMESPACE_URI, "themeVariant").with_children(children)
}

/// Create a `<thm15:themeVariantLst>` element (`ThemeVariantList`).
pub fn theme_variant_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("thm15", NAMESPACE_URI, "themeVariantLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
