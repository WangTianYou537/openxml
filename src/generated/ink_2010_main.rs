//! Auto-generated from `schemas_microsoft_com_ink_2010_main.json`.
//! Target namespace: `http://schemas.microsoft.com/ink/2010/main` (prefix `msink`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/ink/2010/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "msink";

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

static ATTRS_CONTEXT_NODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":rotatedBoundingBox", property_name: Some("RotatedBoundingBox"), type_name: "ListValue" },
    AttributeInfo { qname: ":alignmentLevel", property_name: Some("AlignmentLevel"), type_name: "Int32Value" },
    AttributeInfo { qname: ":contentType", property_name: Some("ContentType"), type_name: "Int32Value" },
    AttributeInfo { qname: ":ascender", property_name: Some("Ascender"), type_name: "StringValue" },
    AttributeInfo { qname: ":descender", property_name: Some("Descender"), type_name: "StringValue" },
    AttributeInfo { qname: ":baseline", property_name: Some("Baseline"), type_name: "StringValue" },
    AttributeInfo { qname: ":midline", property_name: Some("Midline"), type_name: "StringValue" },
    AttributeInfo { qname: ":customRecognizerId", property_name: Some("CustomRecognizerId"), type_name: "StringValue" },
    AttributeInfo { qname: ":mathML", property_name: Some("MathML"), type_name: "StringValue" },
    AttributeInfo { qname: ":mathStruct", property_name: Some("MathStruct"), type_name: "StringValue" },
    AttributeInfo { qname: ":mathSymbol", property_name: Some("MathSymbol"), type_name: "StringValue" },
    AttributeInfo { qname: ":beginModifierType", property_name: Some("BeginModifierType"), type_name: "StringValue" },
    AttributeInfo { qname: ":endModifierType", property_name: Some("EndModifierType"), type_name: "StringValue" },
    AttributeInfo { qname: ":rotationAngle", property_name: Some("RotationAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: ":hotPoints", property_name: Some("HotPoints"), type_name: "ListValue" },
    AttributeInfo { qname: ":centroid", property_name: Some("Centroid"), type_name: "StringValue" },
    AttributeInfo { qname: ":semanticType", property_name: Some("SemanticType"), type_name: "StringValue" },
    AttributeInfo { qname: ":shapeName", property_name: Some("ShapeName"), type_name: "StringValue" },
    AttributeInfo { qname: ":shapeGeometry", property_name: Some("ShapeGeometry"), type_name: "ListValue" },
];
static CHILDREN_CONTEXT_NODE: &[ChildInfo] = &[
    ChildInfo { name: "msink:CT_Property/msink:property", property_name: None },
    ChildInfo { name: "msink:CT_CtxLink/msink:sourceLink", property_name: None },
    ChildInfo { name: "msink:CT_CtxLink/msink:destinationLink", property_name: None },
];
static ATTRS_CONTEXT_NODE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
];
static ATTRS_SOURCE_LINK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":direction", property_name: Some("Direction"), type_name: "EnumValue" },
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];
static ATTRS_DESTINATION_LINK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":direction", property_name: Some("Direction"), type_name: "EnumValue" },
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ContextNode", local_name: "context", prefix: "msink", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTEXT_NODE, children: CHILDREN_CONTEXT_NODE },
    ElementInfo { class_name: "ContextNodeProperty", local_name: "property", prefix: "msink", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_CONTEXT_NODE_PROPERTY, children: &[] },
    ElementInfo { class_name: "SourceLink", local_name: "sourceLink", prefix: "msink", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SOURCE_LINK, children: &[] },
    ElementInfo { class_name: "DestinationLink", local_name: "destinationLink", prefix: "msink", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DESTINATION_LINK, children: &[] },
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

/// Create a `<msink:context>` element (`ContextNode`).
pub fn context_node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("msink", NAMESPACE_URI, "context").with_children(children)
}

/// Create a `<msink:property>` element (`ContextNodeProperty`).
pub fn context_node_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("msink", NAMESPACE_URI, "property").with_text(value)
}

/// Create a `<msink:sourceLink>` element (`SourceLink`).
pub fn source_link() -> OpenXmlElement {
    OpenXmlElement::new("msink", NAMESPACE_URI, "sourceLink")
}

/// Create a `<msink:destinationLink>` element (`DestinationLink`).
pub fn destination_link() -> OpenXmlElement {
    OpenXmlElement::new("msink", NAMESPACE_URI, "destinationLink")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 5;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
