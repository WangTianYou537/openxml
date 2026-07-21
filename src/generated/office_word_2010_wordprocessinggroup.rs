//! Auto-generated from `schemas_microsoft_com_office_word_2010_wordprocessingGroup.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2010/wordprocessingGroup` (prefix `wpg`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wpg";

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

static CHILDREN_WORDPROCESSING_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/wpg:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr", property_name: Some("NonVisualGroupDrawingShapeProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/wpg:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "wps:CT_WordprocessingShape/wps:wsp", property_name: None },
    ChildInfo { name: "wpg:CT_WordprocessingGroup/wpg:grpSp", property_name: None },
    ChildInfo { name: "wpg:CT_GraphicFrame/wpg:graphicFrame", property_name: None },
    ChildInfo { name: "pic:CT_Picture/pic:pic", property_name: None },
    ChildInfo { name: "w14:CT_WordContentPart/w14:contentPart", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wpg:extLst", property_name: None },
];
static CHILDREN_GROUP_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/wpg:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/wpg:cNvGrpSpPr", property_name: Some("NonVisualGroupDrawingShapeProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/wpg:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "wps:CT_WordprocessingShape/wps:wsp", property_name: None },
    ChildInfo { name: "wpg:CT_WordprocessingGroup/wpg:grpSp", property_name: None },
    ChildInfo { name: "wpg:CT_GraphicFrame/wpg:graphicFrame", property_name: None },
    ChildInfo { name: "pic:CT_Picture/pic:pic", property_name: None },
    ChildInfo { name: "w14:CT_WordContentPart/w14:contentPart", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wpg:extLst", property_name: None },
];
static ATTRS_NON_VISUAL_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":descr", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: Some("HyperlinkOnClick") },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkHover", property_name: Some("HyperlinkOnHover") },
    ChildInfo { name: "a:CT_NonVisualDrawingPropsExtensionList/a:extLst", property_name: Some("NonVisualDrawingPropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks", property_name: Some("GraphicFrameLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TRANSFORM2_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":flipH", property_name: Some("HorizontalFlip"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":flipV", property_name: Some("VerticalFlip"), type_name: "BooleanValue" },
];
static CHILDREN_TRANSFORM2_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/a:off", property_name: Some("Offset") },
    ChildInfo { name: "a:CT_PositiveSize2D/a:ext", property_name: Some("Extents") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GroupLocking/a:grpSpLocks", property_name: Some("GroupShapeLocks") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst", property_name: Some("NonVisualGroupDrawingShapePropsExtensionList") },
];
static ATTRS_GROUP_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GroupTransform2D/a:xfrm", property_name: Some("TransformGroup") },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_GRAPHIC_FRAME: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/wpg:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/wpg:cNvFrPr", property_name: Some("NonVisualGraphicFrameProperties") },
    ChildInfo { name: "a:CT_Transform2D/wpg:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wpg:extLst", property_name: Some("OfficeArtExtensionList") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "WordprocessingGroup", local_name: "wgp", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WORDPROCESSING_GROUP },
    ElementInfo { class_name: "GroupShape", local_name: "grpSp", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_SHAPE },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualGraphicFrameProperties", local_name: "cNvFrPr", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES },
    ElementInfo { class_name: "Transform2D", local_name: "xfrm", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualGroupDrawingShapeProperties", local_name: "cNvGrpSpPr", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPERTIES },
    ElementInfo { class_name: "GroupShapeProperties", local_name: "grpSpPr", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_PROPERTIES, children: CHILDREN_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "GraphicFrame", local_name: "graphicFrame", prefix: "wpg", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRAPHIC_FRAME },
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

/// Create a `<wpg:wgp>` element (`WordprocessingGroup`).
pub fn wordprocessing_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "wgp").with_children(children)
}

/// Create a `<wpg:grpSp>` element (`GroupShape`).
pub fn group_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "grpSp").with_children(children)
}

/// Create a `<wpg:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<wpg:cNvFrPr>` element (`NonVisualGraphicFrameProperties`).
pub fn non_visual_graphic_frame_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "cNvFrPr").with_children(children)
}

/// Create a `<wpg:xfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Create a `<wpg:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<wpg:cNvGrpSpPr>` element (`NonVisualGroupDrawingShapeProperties`).
pub fn non_visual_group_drawing_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "cNvGrpSpPr").with_children(children)
}

/// Create a `<wpg:grpSpPr>` element (`GroupShapeProperties`).
pub fn group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Create a `<wpg:graphicFrame>` element (`GraphicFrame`).
pub fn graphic_frame(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wpg", NAMESPACE_URI, "graphicFrame").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 10;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 9;
