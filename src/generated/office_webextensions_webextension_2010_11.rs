//! Auto-generated from `schemas_microsoft_com_office_webextensions_webextension_2010_11.json`.
//! Target namespace: `http://schemas.microsoft.com/office/webextensions/webextension/2010/11` (prefix `we`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/webextensions/webextension/2010/11";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "we";

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

static ATTRS_WEB_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":frozen", property_name: Some("Frozen"), type_name: "BooleanValue" },
];
static CHILDREN_WEB_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "we:CT_OsfWebExtensionReference/we:reference", property_name: Some("WebExtensionStoreReference") },
    ChildInfo { name: "we:CT_OsfWebExtensionReferenceList/we:alternateReferences", property_name: Some("WebExtensionReferenceList") },
    ChildInfo { name: "we:CT_OsfWebExtensionPropertyBag/we:properties", property_name: Some("WebExtensionPropertyBag") },
    ChildInfo { name: "we:CT_OsfWebExtensionBindingList/we:bindings", property_name: Some("WebExtensionBindingList") },
    ChildInfo { name: "a:CT_Blip/we:snapshot", property_name: Some("Snapshot") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/we:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_WEB_EXTENSION_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_WEB_EXTENSION_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":value", property_name: Some("Value"), type_name: "StringValue" },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_WEB_EXTENSION_BINDING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":appref", property_name: Some("AppReference"), type_name: "StringValue" },
];
static CHILDREN_WEB_EXTENSION_BINDING: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/we:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_WEB_EXTENSION_STORE_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":version", property_name: Some("Version"), type_name: "StringValue" },
    AttributeInfo { qname: ":store", property_name: Some("Store"), type_name: "StringValue" },
    AttributeInfo { qname: ":storeType", property_name: Some("StoreType"), type_name: "StringValue" },
];
static CHILDREN_WEB_EXTENSION_STORE_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/we:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_WEB_EXTENSION_REFERENCE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "we:CT_OsfWebExtensionReference/we:reference", property_name: None },
];
static CHILDREN_WEB_EXTENSION_PROPERTY_BAG: &[ChildInfo] = &[
    ChildInfo { name: "we:CT_OsfWebExtensionProperty/we:property", property_name: None },
];
static CHILDREN_WEB_EXTENSION_BINDING_LIST: &[ChildInfo] = &[
    ChildInfo { name: "we:CT_OsfWebExtensionBinding/we:binding", property_name: None },
];
static ATTRS_SNAPSHOT: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
    AttributeInfo { qname: ":cstate", property_name: Some("CompressionState"), type_name: "EnumValue" },
];
static CHILDREN_SNAPSHOT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AlphaBiLevelEffect/a:alphaBiLevel", property_name: None },
    ChildInfo { name: "a:CT_AlphaCeilingEffect/a:alphaCeiling", property_name: None },
    ChildInfo { name: "a:CT_AlphaFloorEffect/a:alphaFloor", property_name: None },
    ChildInfo { name: "a:CT_AlphaInverseEffect/a:alphaInv", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateEffect/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateFixedEffect/a:alphaModFix", property_name: None },
    ChildInfo { name: "a:CT_AlphaReplaceEffect/a:alphaRepl", property_name: None },
    ChildInfo { name: "a:CT_BiLevelEffect/a:biLevel", property_name: None },
    ChildInfo { name: "a:CT_BlurEffect/a:blur", property_name: None },
    ChildInfo { name: "a:CT_ColorChangeEffect/a:clrChange", property_name: None },
    ChildInfo { name: "a:CT_ColorReplaceEffect/a:clrRepl", property_name: None },
    ChildInfo { name: "a:CT_DuotoneEffect/a:duotone", property_name: None },
    ChildInfo { name: "a:CT_FillOverlayEffect/a:fillOverlay", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleEffect/a:grayscl", property_name: None },
    ChildInfo { name: "a:CT_HSLEffect/a:hsl", property_name: None },
    ChildInfo { name: "a:CT_LuminanceEffect/a:lum", property_name: None },
    ChildInfo { name: "a:CT_TintEffect/a:tint", property_name: None },
    ChildInfo { name: "a:CT_BlipExtensionList/a:extLst", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "WebExtension", local_name: "webextension", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION, children: CHILDREN_WEB_EXTENSION },
    ElementInfo { class_name: "WebExtensionReference", local_name: "webextensionref", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_REFERENCE, children: &[] },
    ElementInfo { class_name: "WebExtensionProperty", local_name: "property", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_PROPERTY, children: &[] },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "WebExtensionBinding", local_name: "binding", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_BINDING, children: CHILDREN_WEB_EXTENSION_BINDING },
    ElementInfo { class_name: "WebExtensionStoreReference", local_name: "reference", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_STORE_REFERENCE, children: CHILDREN_WEB_EXTENSION_STORE_REFERENCE },
    ElementInfo { class_name: "WebExtensionReferenceList", local_name: "alternateReferences", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WEB_EXTENSION_REFERENCE_LIST },
    ElementInfo { class_name: "WebExtensionPropertyBag", local_name: "properties", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WEB_EXTENSION_PROPERTY_BAG },
    ElementInfo { class_name: "WebExtensionBindingList", local_name: "bindings", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WEB_EXTENSION_BINDING_LIST },
    ElementInfo { class_name: "Snapshot", local_name: "snapshot", prefix: "we", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SNAPSHOT, children: CHILDREN_SNAPSHOT },
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

/// Create a `<we:webextension>` element (`WebExtension`).
pub fn web_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "webextension").with_children(children)
}

/// Create a `<we:webextensionref>` element (`WebExtensionReference`).
pub fn web_extension_reference() -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "webextensionref")
}

/// Create a `<we:property>` element (`WebExtensionProperty`).
pub fn web_extension_property() -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "property")
}

/// Create a `<we:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<we:binding>` element (`WebExtensionBinding`).
pub fn web_extension_binding(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "binding").with_children(children)
}

/// Create a `<we:reference>` element (`WebExtensionStoreReference`).
pub fn web_extension_store_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "reference").with_children(children)
}

/// Create a `<we:alternateReferences>` element (`WebExtensionReferenceList`).
pub fn web_extension_reference_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "alternateReferences").with_children(children)
}

/// Create a `<we:properties>` element (`WebExtensionPropertyBag`).
pub fn web_extension_property_bag(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "properties").with_children(children)
}

/// Create a `<we:bindings>` element (`WebExtensionBindingList`).
pub fn web_extension_binding_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "bindings").with_children(children)
}

/// Create a `<we:snapshot>` element (`Snapshot`).
pub fn snapshot(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("we", NAMESPACE_URI, "snapshot").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 10;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 10;
