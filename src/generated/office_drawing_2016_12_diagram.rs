//! Auto-generated from `schemas_microsoft_com_office_drawing_2016_12_diagram.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2016/12/diagram` (prefix `dgm1612`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2016/12/diagram";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "dgm1612";

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

static ATTRS_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Transform2D/a:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_CustomGeometry2D/a:custGeom", property_name: None },
    ChildInfo { name: "a:CT_PresetGeometry2D/a:prstGeom", property_name: None },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
    ChildInfo { name: "a:CT_ShapePropertiesExtensionList/a:extLst", property_name: None },
];
static CHILDREN_TEXT_LIST_STYLE_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:defPPr", property_name: Some("DefaultParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl1pPr", property_name: Some("Level1ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl2pPr", property_name: Some("Level2ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl3pPr", property_name: Some("Level3ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl4pPr", property_name: Some("Level4ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl5pPr", property_name: Some("Level5ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl6pPr", property_name: Some("Level6ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl7pPr", property_name: Some("Level7ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl8pPr", property_name: Some("Level8ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl9pPr", property_name: Some("Level9ParagraphProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "dgm1612", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "TextListStyleType", local_name: "lstStyle", prefix: "dgm1612", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_LIST_STYLE_TYPE },
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

/// Create a `<dgm1612:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm1612", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<dgm1612:lstStyle>` element (`TextListStyleType`).
pub fn text_list_style_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm1612", NAMESPACE_URI, "lstStyle").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
