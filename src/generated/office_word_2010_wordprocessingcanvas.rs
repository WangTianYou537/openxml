//! Auto-generated from `schemas_microsoft_com_office_word_2010_wordprocessingCanvas.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas` (prefix `wpc`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wpc";

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

static CHILDREN_WORDPROCESSING_CANVAS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_BackgroundFormatting/wpc:bg", property_name: Some("BackgroundFormatting") },
    ChildInfo { name: "a:CT_WholeE2oFormatting/wpc:whole", property_name: Some("WholeFormatting") },
    ChildInfo { name: "wps:CT_WordprocessingShape/wps:wsp", property_name: None },
    ChildInfo { name: "pic:CT_Picture/pic:pic", property_name: None },
    ChildInfo { name: "w14:CT_WordContentPart/w14:contentPart", property_name: None },
    ChildInfo { name: "wpg:CT_WordprocessingGroup/wpg:wgp", property_name: None },
    ChildInfo { name: "wpg:CT_GraphicFrame/wpc:graphicFrame", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wpc:extLst", property_name: None },
];
static CHILDREN_BACKGROUND_FORMATTING: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
];
static CHILDREN_WHOLE_FORMATTING: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
];
static CHILDREN_GRAPHIC_FRAME_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/wpg:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr", property_name: Some("NonVisualGraphicFrameProperties") },
    ChildInfo { name: "a:CT_Transform2D/wpg:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wpg:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "WordprocessingCanvas", local_name: "wpc", prefix: "wpc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WORDPROCESSING_CANVAS },
    ElementInfo { class_name: "BackgroundFormatting", local_name: "bg", prefix: "wpc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND_FORMATTING },
    ElementInfo { class_name: "WholeFormatting", local_name: "whole", prefix: "wpc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WHOLE_FORMATTING },
    ElementInfo { class_name: "GraphicFrameType", local_name: "graphicFrame", prefix: "wpc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRAPHIC_FRAME_TYPE },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "wpc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
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

/// Create a `<wpc:wpc>` element (`WordprocessingCanvas`).
pub fn wordprocessing_canvas(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpc", NAMESPACE_URI, "wpc").with_children(children)
}

/// Create a `<wpc:bg>` element (`BackgroundFormatting`).
pub fn background_formatting(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpc", NAMESPACE_URI, "bg").with_children(children)
}

/// Create a `<wpc:whole>` element (`WholeFormatting`).
pub fn whole_formatting(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpc", NAMESPACE_URI, "whole").with_children(children)
}

/// Create a `<wpc:graphicFrame>` element (`GraphicFrameType`).
pub fn graphic_frame_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpc", NAMESPACE_URI, "graphicFrame").with_children(children)
}

/// Create a `<wpc:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpc", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 5;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 5;
