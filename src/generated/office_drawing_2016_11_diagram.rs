//! Auto-generated from `schemas_microsoft_com_office_drawing_2016_11_diagram.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2016/11/diagram` (prefix `dgm1611`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2016/11/diagram";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "dgm1611";

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

static CHILDREN_NUMBER_DIAGRAM_INFO_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm1611:CT_NumberDiagramInfo/dgm1611:autoBuNodeInfo", property_name: None },
];
static ATTRS_DIAGRAM_AUTO_BULLET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prefix", property_name: Some("AutoBulletPrefix"), type_name: "StringValue" },
    AttributeInfo { qname: ":leadZeros", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_DIAGRAM_AUTO_BULLET: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: Some("NoBullet") },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: Some("AutoNumberedBullet") },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: Some("CharacterBullet") },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: Some("PictureBullet") },
];
static ATTRS_NUMBER_DIAGRAM_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lvl", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":ptType", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_NUMBER_DIAGRAM_INFO: &[ChildInfo] = &[
    ChildInfo { name: "dgm1611:CT_DiagramAutoBullet/dgm1611:buPr", property_name: Some("DiagramAutoBullet") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "NumberDiagramInfoList", local_name: "autoBuNodeInfoLst", prefix: "dgm1611", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NUMBER_DIAGRAM_INFO_LIST },
    ElementInfo { class_name: "DiagramAutoBullet", local_name: "buPr", prefix: "dgm1611", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIAGRAM_AUTO_BULLET, children: CHILDREN_DIAGRAM_AUTO_BULLET },
    ElementInfo { class_name: "NumberDiagramInfo", local_name: "autoBuNodeInfo", prefix: "dgm1611", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NUMBER_DIAGRAM_INFO, children: CHILDREN_NUMBER_DIAGRAM_INFO },
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

/// Create a `<dgm1611:autoBuNodeInfoLst>` element (`NumberDiagramInfoList`).
pub fn number_diagram_info_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm1611", NAMESPACE_URI, "autoBuNodeInfoLst").with_children(children)
}

/// Create a `<dgm1611:buPr>` element (`DiagramAutoBullet`).
pub fn diagram_auto_bullet(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm1611", NAMESPACE_URI, "buPr").with_children(children)
}

/// Create a `<dgm1611:autoBuNodeInfo>` element (`NumberDiagramInfo`).
pub fn number_diagram_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm1611", NAMESPACE_URI, "autoBuNodeInfo").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
