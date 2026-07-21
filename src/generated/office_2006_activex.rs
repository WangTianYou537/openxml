//! Auto-generated from `schemas_microsoft_com_office_2006_activeX.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/activeX` (prefix `ax`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/activeX";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ax";

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

static ATTRS_ACTIVE_X_CONTROL_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: "ax:classid", property_name: Some("ActiveXControlClassId"), type_name: "StringValue" },
    AttributeInfo { qname: "ax:license", property_name: Some("License"), type_name: "StringValue" },
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: "ax:persistence", property_name: Some("Persistence"), type_name: "EnumValue" },
];
static CHILDREN_ACTIVE_X_CONTROL_DATA: &[ChildInfo] = &[
    ChildInfo { name: "ax:CT_OcxPr/ax:ocxPr", property_name: None },
];
static ATTRS_ACTIVE_X_OBJECT_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: "ax:name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: "ax:value", property_name: Some("Value"), type_name: "StringValue" },
];
static CHILDREN_ACTIVE_X_OBJECT_PROPERTY: &[ChildInfo] = &[
    ChildInfo { name: "ax:CT_Font/ax:font", property_name: Some("SharedComFont") },
    ChildInfo { name: "ax:CT_Picture/ax:picture", property_name: Some("SharedComPicture") },
];
static ATTRS_SHARED_COM_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: "ax:persistence", property_name: Some("Persistence"), type_name: "EnumValue" },
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_SHARED_COM_FONT: &[ChildInfo] = &[
    ChildInfo { name: "ax:CT_OcxPr/ax:ocxPr", property_name: None },
];
static ATTRS_SHARED_COM_PICTURE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ActiveXControlData", local_name: "ocx", prefix: "ax", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ACTIVE_X_CONTROL_DATA, children: CHILDREN_ACTIVE_X_CONTROL_DATA },
    ElementInfo { class_name: "ActiveXObjectProperty", local_name: "ocxPr", prefix: "ax", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ACTIVE_X_OBJECT_PROPERTY, children: CHILDREN_ACTIVE_X_OBJECT_PROPERTY },
    ElementInfo { class_name: "SharedComFont", local_name: "font", prefix: "ax", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHARED_COM_FONT, children: CHILDREN_SHARED_COM_FONT },
    ElementInfo { class_name: "SharedComPicture", local_name: "picture", prefix: "ax", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHARED_COM_PICTURE, children: &[] },
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

/// Create a `<ax:ocx>` element (`ActiveXControlData`).
pub fn active_x_control_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ax", NAMESPACE_URI, "ocx").with_children(children)
}

/// Create a `<ax:ocxPr>` element (`ActiveXObjectProperty`).
pub fn active_x_object_property(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ax", NAMESPACE_URI, "ocxPr").with_children(children)
}

/// Create a `<ax:font>` element (`SharedComFont`).
pub fn shared_com_font(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ax", NAMESPACE_URI, "font").with_children(children)
}

/// Create a `<ax:picture>` element (`SharedComPicture`).
pub fn shared_com_picture() -> OpenXmlElement {
    OpenXmlElement::new("ax", NAMESPACE_URI, "picture")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
