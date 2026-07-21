//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2017_richdata.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2017/richdata` (prefix `xlrd`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xlrd";

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

static ATTRS_RICH_VALUE_BLOCK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":i", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_RICH_VALUE_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_RICH_VALUE_DATA: &[ChildInfo] = &[
    ChildInfo { name: "xlrd:CT_RichValue/xlrd:rv", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd:extLst", property_name: None },
];
static ATTRS_RICH_VALUE_STRUCTURES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_RICH_VALUE_STRUCTURES: &[ChildInfo] = &[
    ChildInfo { name: "xlrd:CT_RichValueStructure/xlrd:s", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrd:extLst", property_name: None },
];
static ATTRS_RICH_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":s", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_RICH_VALUE: &[ChildInfo] = &[
    ChildInfo { name: "xlrd:CT_RichValueFallback/xlrd:fb", property_name: Some("RichValueFallback") },
    ChildInfo { name: "xlrd:CT_Value/xlrd:v", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_RICH_VALUE_FALLBACK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
];
static ATTRS_RICH_VALUE_STRUCTURE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":t", property_name: None, type_name: "StringValue" },
];
static CHILDREN_RICH_VALUE_STRUCTURE: &[ChildInfo] = &[
    ChildInfo { name: "xlrd:CT_Key/xlrd:k", property_name: None },
];
static ATTRS_KEY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "RichValueBlock", local_name: "rvb", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_BLOCK, children: &[] },
    ElementInfo { class_name: "RichValueData", local_name: "rvData", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_DATA, children: CHILDREN_RICH_VALUE_DATA },
    ElementInfo { class_name: "RichValueStructures", local_name: "rvStructures", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_STRUCTURES, children: CHILDREN_RICH_VALUE_STRUCTURES },
    ElementInfo { class_name: "RichValue", local_name: "rv", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RICH_VALUE, children: CHILDREN_RICH_VALUE },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "RichValueFallback", local_name: "fb", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_RICH_VALUE_FALLBACK, children: &[] },
    ElementInfo { class_name: "Value", local_name: "v", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RichValueStructure", local_name: "s", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RICH_VALUE_STRUCTURE, children: CHILDREN_RICH_VALUE_STRUCTURE },
    ElementInfo { class_name: "Key", local_name: "k", prefix: "xlrd", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_KEY, children: &[] },
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

/// Create a `<xlrd:rvb>` element (`RichValueBlock`).
pub fn rich_value_block() -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "rvb")
}

/// Create a `<xlrd:rvData>` element (`RichValueData`).
pub fn rich_value_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "rvData").with_children(children)
}

/// Create a `<xlrd:rvStructures>` element (`RichValueStructures`).
pub fn rich_value_structures(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "rvStructures").with_children(children)
}

/// Create a `<xlrd:rv>` element (`RichValue`).
pub fn rich_value(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "rv").with_children(children)
}

/// Create a `<xlrd:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<xlrd:fb>` element (`RichValueFallback`).
pub fn rich_value_fallback(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "fb").with_text(value)
}

/// Create a `<xlrd:v>` element (`Value`).
pub fn value(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<xlrd:s>` element (`RichValueStructure`).
pub fn rich_value_structure(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "s").with_children(children)
}

/// Create a `<xlrd:k>` element (`Key`).
pub fn key() -> OpenXmlElement {
    OpenXmlElement::new("xlrd", NAMESPACE_URI, "k")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 9;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 9;
