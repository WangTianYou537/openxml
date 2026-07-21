//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_picture.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/picture` (prefix `pic`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/picture";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "pic";

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

static CHILDREN_PICTURE: &[ChildInfo] = &[
    ChildInfo { name: "pic:CT_PictureNonVisual/pic:nvPicPr", property_name: Some("NonVisualPictureProperties") },
    ChildInfo { name: "a:CT_BlipFillProperties/pic:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_ShapeProperties/pic:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/pic14:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/pic14:extLst", property_name: Some("OfficeArtExtensionList") },
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
static ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":preferRelativeResize", property_name: Some("PreferRelativeResize"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PictureLocking/a:picLocks", property_name: Some("PictureLocks") },
    ChildInfo { name: "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst", property_name: Some("NonVisualPicturePropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_PICTURE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/pic:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualPictureProperties/pic:cNvPicPr", property_name: Some("NonVisualPictureDrawingProperties") },
];
static ATTRS_BLIP_FILL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dpi", property_name: Some("Dpi"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rotWithShape", property_name: Some("RotateWithShape"), type_name: "BooleanValue" },
];
static CHILDREN_BLIP_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Blip/a:blip", property_name: Some("Blip") },
    ChildInfo { name: "a:CT_RelativeRect/a:srcRect", property_name: Some("SourceRectangle") },
    ChildInfo { name: "a:CT_TileInfoProperties/a:tile", property_name: None },
    ChildInfo { name: "a:CT_StretchInfoProperties/a:stretch", property_name: None },
];
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Picture", local_name: "pic", prefix: "pic", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PICTURE },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "pic", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureDrawingProperties", local_name: "cNvPicPr", prefix: "pic", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureProperties", local_name: "nvPicPr", prefix: "pic", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_PICTURE_PROPERTIES },
    ElementInfo { class_name: "BlipFill", local_name: "blipFill", prefix: "pic", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP_FILL, children: CHILDREN_BLIP_FILL },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "pic", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
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

/// Create a `<pic:pic>` element (`Picture`).
pub fn picture(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pic", NAMESPACE_URI, "pic").with_children(children)
}

/// Create a `<pic:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pic", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<pic:cNvPicPr>` element (`NonVisualPictureDrawingProperties`).
pub fn non_visual_picture_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pic", NAMESPACE_URI, "cNvPicPr").with_children(children)
}

/// Create a `<pic:nvPicPr>` element (`NonVisualPictureProperties`).
pub fn non_visual_picture_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pic", NAMESPACE_URI, "nvPicPr").with_children(children)
}

/// Create a `<pic:blipFill>` element (`BlipFill`).
pub fn blip_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pic", NAMESPACE_URI, "blipFill").with_children(children)
}

/// Create a `<pic:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pic", NAMESPACE_URI, "spPr").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 6;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 6;
