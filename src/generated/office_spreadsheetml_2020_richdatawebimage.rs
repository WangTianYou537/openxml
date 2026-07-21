//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2020/richdatawebimage` (prefix `xlrdwi`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2020/richdatawebimage";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xlrdwi";

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

static CHILDREN_WEB_IMAGES_SUPPORTING_RICH_DATA: &[ChildInfo] = &[
    ChildInfo { name: "xlrdwi:CT_WebImageSupportingRichData/xlrdwi:webImageSrd", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xlrdwi:extLst", property_name: None },
];
static CHILDREN_WEB_IMAGE_SUPPORTING_RICH_DATA: &[ChildInfo] = &[
    ChildInfo { name: "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:address", property_name: Some("AddressWebImageSupportingRichDataRelationship") },
    ChildInfo { name: "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:moreImagesAddress", property_name: Some("MoreImagesAddressWebImageSupportingRichDataRelationship") },
    ChildInfo { name: "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:blip", property_name: Some("BlipWebImageSupportingRichDataRelationship") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_ADDRESS_WEB_IMAGE_SUPPORTING_RICH_DATA_RELATIONSHIP: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_MORE_IMAGES_ADDRESS_WEB_IMAGE_SUPPORTING_RICH_DATA_RELATIONSHIP: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_BLIP_WEB_IMAGE_SUPPORTING_RICH_DATA_RELATIONSHIP: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "WebImagesSupportingRichData", local_name: "webImagesSrd", prefix: "xlrdwi", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WEB_IMAGES_SUPPORTING_RICH_DATA },
    ElementInfo { class_name: "WebImageSupportingRichData", local_name: "webImageSrd", prefix: "xlrdwi", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WEB_IMAGE_SUPPORTING_RICH_DATA },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xlrdwi", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "AddressWebImageSupportingRichDataRelationship", local_name: "address", prefix: "xlrdwi", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ADDRESS_WEB_IMAGE_SUPPORTING_RICH_DATA_RELATIONSHIP, children: &[] },
    ElementInfo { class_name: "MoreImagesAddressWebImageSupportingRichDataRelationship", local_name: "moreImagesAddress", prefix: "xlrdwi", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MORE_IMAGES_ADDRESS_WEB_IMAGE_SUPPORTING_RICH_DATA_RELATIONSHIP, children: &[] },
    ElementInfo { class_name: "BlipWebImageSupportingRichDataRelationship", local_name: "blip", prefix: "xlrdwi", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BLIP_WEB_IMAGE_SUPPORTING_RICH_DATA_RELATIONSHIP, children: &[] },
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

/// Create a `<xlrdwi:webImagesSrd>` element (`WebImagesSupportingRichData`).
pub fn web_images_supporting_rich_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrdwi", NAMESPACE_URI, "webImagesSrd").with_children(children)
}

/// Create a `<xlrdwi:webImageSrd>` element (`WebImageSupportingRichData`).
pub fn web_image_supporting_rich_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrdwi", NAMESPACE_URI, "webImageSrd").with_children(children)
}

/// Create a `<xlrdwi:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlrdwi", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<xlrdwi:address>` element (`AddressWebImageSupportingRichDataRelationship`).
pub fn address_web_image_supporting_rich_data_relationship() -> OpenXmlElement {
    OpenXmlElement::new("xlrdwi", NAMESPACE_URI, "address")
}

/// Create a `<xlrdwi:moreImagesAddress>` element (`MoreImagesAddressWebImageSupportingRichDataRelationship`).
pub fn more_images_address_web_image_supporting_rich_data_relationship() -> OpenXmlElement {
    OpenXmlElement::new("xlrdwi", NAMESPACE_URI, "moreImagesAddress")
}

/// Create a `<xlrdwi:blip>` element (`BlipWebImageSupportingRichDataRelationship`).
pub fn blip_web_image_supporting_rich_data_relationship() -> OpenXmlElement {
    OpenXmlElement::new("xlrdwi", NAMESPACE_URI, "blip")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 7;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 6;
