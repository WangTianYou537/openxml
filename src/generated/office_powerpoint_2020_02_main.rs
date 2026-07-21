//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2020_02_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2020/02/main` (prefix `p202`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2020/02/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p202";

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

static CHILDREN_DESIGNER_TAG_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p202:CT_DesignerTag/p202:designTag", property_name: None },
];
static ATTRS_DESIGNER_DRAWING_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":edtDesignElem", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_DESIGNER_DRAWING_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "p202:CT_DesignerTagList/p202:designTagLst", property_name: Some("DesignerTagList") },
    ChildInfo { name: "p:CT_ExtensionList/p202:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_DESIGNER_TAG: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: None, type_name: "StringValue" },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "DesignerTagList", local_name: "designTagLst", prefix: "p202", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DESIGNER_TAG_LIST },
    ElementInfo { class_name: "DesignerDrawingProps", local_name: "designPr", prefix: "p202", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DESIGNER_DRAWING_PROPS, children: CHILDREN_DESIGNER_DRAWING_PROPS },
    ElementInfo { class_name: "DesignerTag", local_name: "designTag", prefix: "p202", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DESIGNER_TAG, children: &[] },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "p202", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
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

/// Create a `<p202:designTagLst>` element (`DesignerTagList`).
pub fn designer_tag_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p202", NAMESPACE_URI, "designTagLst").with_children(children)
}

/// Create a `<p202:designPr>` element (`DesignerDrawingProps`).
pub fn designer_drawing_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p202", NAMESPACE_URI, "designPr").with_children(children)
}

/// Create a `<p202:designTag>` element (`DesignerTag`).
pub fn designer_tag() -> OpenXmlElement {
    OpenXmlElement::new("p202", NAMESPACE_URI, "designTag")
}

/// Create a `<p202:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p202", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 4;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 4;
