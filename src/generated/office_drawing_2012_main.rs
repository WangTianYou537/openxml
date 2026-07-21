//! Auto-generated from `schemas_microsoft_com_office_drawing_2012_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2012/main` (prefix `a15`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2012/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "a15";

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

static ATTRS_BACKGROUND_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("Mode"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bwPure", property_name: Some("Pure"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bwNormal", property_name: Some("Normal"), type_name: "EnumValue" },
    AttributeInfo { qname: ":targetScreenSize", property_name: Some("TargetScreenSize"), type_name: "EnumValue" },
];
static ATTRS_NON_VISUAL_GROUP_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isLegacyGroup", property_name: Some("IsLegacyGroup"), type_name: "BooleanValue" },
];
static ATTRS_OBJECT_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":objectId", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":isActiveX", property_name: Some("IsActiveX"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":linkType", property_name: Some("LinkType"), type_name: "StringValue" },
];
static ATTRS_SIGNATURE_LINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isSignatureLine", property_name: Some("IsSignatureLine"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":provId", property_name: Some("ProviderId"), type_name: "StringValue" },
    AttributeInfo { qname: ":signingInstructionsSet", property_name: Some("SigningInstructionsSet"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":allowComments", property_name: Some("AllowComments"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showSignDate", property_name: Some("ShowSignDate"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":suggestedSigner", property_name: Some("SuggestedSigner"), type_name: "StringValue" },
    AttributeInfo { qname: ":suggestedSigner2", property_name: Some("SuggestedSigner2"), type_name: "StringValue" },
    AttributeInfo { qname: ":suggestedSignerEmail", property_name: Some("SuggestedSignerEmail"), type_name: "StringValue" },
    AttributeInfo { qname: ":signingInstructions", property_name: Some("SigningInstructions"), type_name: "StringValue" },
    AttributeInfo { qname: ":addlXml", property_name: Some("AdditionalXml"), type_name: "StringValue" },
    AttributeInfo { qname: ":sigProvUrl", property_name: Some("SignatureProviderUrl"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "BackgroundProperties", local_name: "backgroundPr", prefix: "a15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKGROUND_PROPERTIES, children: &[] },
    ElementInfo { class_name: "NonVisualGroupProperties", local_name: "nonVisualGroupProps", prefix: "a15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_GROUP_PROPERTIES, children: &[] },
    ElementInfo { class_name: "ObjectProperties", local_name: "objectPr", prefix: "a15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OBJECT_PROPERTIES, children: &[] },
    ElementInfo { class_name: "SignatureLine", local_name: "signatureLine", prefix: "a15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SIGNATURE_LINE, children: &[] },
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

/// Create a `<a15:backgroundPr>` element (`BackgroundProperties`).
pub fn background_properties() -> OpenXmlElement {
    OpenXmlElement::new("a15", NAMESPACE_URI, "backgroundPr")
}

/// Create a `<a15:nonVisualGroupProps>` element (`NonVisualGroupProperties`).
pub fn non_visual_group_properties() -> OpenXmlElement {
    OpenXmlElement::new("a15", NAMESPACE_URI, "nonVisualGroupProps")
}

/// Create a `<a15:objectPr>` element (`ObjectProperties`).
pub fn object_properties() -> OpenXmlElement {
    OpenXmlElement::new("a15", NAMESPACE_URI, "objectPr")
}

/// Create a `<a15:signatureLine>` element (`SignatureLine`).
pub fn signature_line() -> OpenXmlElement {
    OpenXmlElement::new("a15", NAMESPACE_URI, "signatureLine")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
