//! Auto-generated from `schemas_microsoft_com_office_2020_mipLabelMetadata.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2020/mipLabelMetadata` (prefix `clbl`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2020/mipLabelMetadata";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "clbl";

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

static CHILDREN_CLASSIFICATION_LABEL_LIST: &[ChildInfo] = &[
    ChildInfo { name: "clbl:CT_ClassificationLabel/clbl:label", property_name: None },
    ChildInfo { name: "clbl:CT_ClassificationExtensionList/clbl:extLst", property_name: None },
];
static ATTRS_CLASSIFICATION_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static ATTRS_CLASSIFICATION_LABEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":setDate", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":method", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":siteId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":actionId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":contentBits", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":removed", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_CLASSIFICATION_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "clbl:CT_ClassificationExtension/clbl:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ClassificationLabelList", local_name: "labelList", prefix: "clbl", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CLASSIFICATION_LABEL_LIST },
    ElementInfo { class_name: "ClassificationExtension", local_name: "ext", prefix: "clbl", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CLASSIFICATION_EXTENSION, children: &[] },
    ElementInfo { class_name: "ClassificationLabel", local_name: "label", prefix: "clbl", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CLASSIFICATION_LABEL, children: &[] },
    ElementInfo { class_name: "ClassificationExtensionList", local_name: "extLst", prefix: "clbl", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CLASSIFICATION_EXTENSION_LIST },
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

/// Create a `<clbl:labelList>` element (`ClassificationLabelList`).
pub fn classification_label_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("clbl", NAMESPACE_URI, "labelList").with_children(children)
}

/// Create a `<clbl:ext>` element (`ClassificationExtension`).
pub fn classification_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("clbl", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<clbl:label>` element (`ClassificationLabel`).
pub fn classification_label() -> OpenXmlElement {
    OpenXmlElement::new("clbl", NAMESPACE_URI, "label")
}

/// Create a `<clbl:extLst>` element (`ClassificationExtensionList`).
pub fn classification_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("clbl", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
