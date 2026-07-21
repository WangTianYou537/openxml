//! Auto-generated from `schemas_microsoft_com_office_drawing_2008_diagram.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2008/diagram` (prefix `dsp`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2008/diagram";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "dsp";

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

static CHILDREN_DRAWING: &[ChildInfo] = &[
    ChildInfo { name: "dsp:CT_GroupShape/dsp:spTree", property_name: Some("ShapeTree") },
];
static ATTRS_DATA_MODEL_EXTENSION_BLOCK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":relId", property_name: Some("RelId"), type_name: "StringValue" },
    AttributeInfo { qname: ":minVer", property_name: Some("MinVer"), type_name: "StringValue" },
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
static ATTRS_NON_VISUAL_DRAWING_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":txBox", property_name: Some("TextBox"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeLocking/a:spLocks", property_name: Some("ShapeLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_SHAPE_NON_VISUAL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/dsp:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualDrawingShapeProps/dsp:cNvSpPr", property_name: Some("NonVisualDrawingShapeProperties") },
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
static CHILDREN_SHAPE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "a:CT_FontReference/a:fontRef", property_name: Some("FontReference") },
];
static CHILDREN_TEXT_BODY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
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
static CHILDREN_GROUP_SHAPE_NON_VISUAL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/dsp:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/dsp:cNvGrpSpPr", property_name: Some("NonVisualGroupDrawingShapeProperties") },
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
static ATTRS_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":modelId", property_name: Some("ModelId"), type_name: "StringValue" },
];
static CHILDREN_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "dsp:CT_ShapeNonVisual/dsp:nvSpPr", property_name: Some("ShapeNonVisualProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/dsp:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/dsp:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_TextBody/dsp:txBody", property_name: Some("TextBody") },
    ChildInfo { name: "a:CT_Transform2D/dsp:txXfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dsp:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_GROUP_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "dsp:CT_GroupShapeNonVisual/dsp:nvGrpSpPr", property_name: Some("GroupShapeNonVisualProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/dsp:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "dsp:CT_Shape/dsp:sp", property_name: None },
    ChildInfo { name: "dsp:CT_GroupShape/dsp:grpSp", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dsp:extLst", property_name: None },
];
static CHILDREN_SHAPE_TREE: &[ChildInfo] = &[
    ChildInfo { name: "dsp:CT_GroupShapeNonVisual/dsp:nvGrpSpPr", property_name: Some("GroupShapeNonVisualProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/dsp:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "dsp:CT_Shape/dsp:sp", property_name: None },
    ChildInfo { name: "dsp:CT_GroupShape/dsp:grpSp", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dsp:extLst", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Drawing", local_name: "drawing", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DRAWING },
    ElementInfo { class_name: "DataModelExtensionBlock", local_name: "dataModelExt", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_MODEL_EXTENSION_BLOCK, children: &[] },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualDrawingShapeProperties", local_name: "cNvSpPr", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_SHAPE_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeNonVisualProperties", local_name: "nvSpPr", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_NON_VISUAL_PROPERTIES },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeStyle", local_name: "style", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_STYLE },
    ElementInfo { class_name: "TextBody", local_name: "txBody", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_BODY },
    ElementInfo { class_name: "Transform2D", local_name: "txXfrm", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualGroupDrawingShapeProperties", local_name: "cNvGrpSpPr", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPERTIES },
    ElementInfo { class_name: "GroupShapeNonVisualProperties", local_name: "nvGrpSpPr", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_SHAPE_NON_VISUAL_PROPERTIES },
    ElementInfo { class_name: "GroupShapeProperties", local_name: "grpSpPr", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_PROPERTIES, children: CHILDREN_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Shape", local_name: "sp", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE, children: CHILDREN_SHAPE },
    ElementInfo { class_name: "GroupShape", local_name: "grpSp", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_SHAPE },
    ElementInfo { class_name: "ShapeTree", local_name: "spTree", prefix: "dsp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_TREE },
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

/// Create a `<dsp:drawing>` element (`Drawing`).
pub fn drawing(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "drawing").with_children(children)
}

/// Create a `<dsp:dataModelExt>` element (`DataModelExtensionBlock`).
pub fn data_model_extension_block() -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "dataModelExt")
}

/// Create a `<dsp:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<dsp:cNvSpPr>` element (`NonVisualDrawingShapeProperties`).
pub fn non_visual_drawing_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "cNvSpPr").with_children(children)
}

/// Create a `<dsp:nvSpPr>` element (`ShapeNonVisualProperties`).
pub fn shape_non_visual_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "nvSpPr").with_children(children)
}

/// Create a `<dsp:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<dsp:style>` element (`ShapeStyle`).
pub fn shape_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<dsp:txBody>` element (`TextBody`).
pub fn text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "txBody").with_children(children)
}

/// Create a `<dsp:txXfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "txXfrm").with_children(children)
}

/// Create a `<dsp:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<dsp:cNvGrpSpPr>` element (`NonVisualGroupDrawingShapeProperties`).
pub fn non_visual_group_drawing_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "cNvGrpSpPr").with_children(children)
}

/// Create a `<dsp:nvGrpSpPr>` element (`GroupShapeNonVisualProperties`).
pub fn group_shape_non_visual_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "nvGrpSpPr").with_children(children)
}

/// Create a `<dsp:grpSpPr>` element (`GroupShapeProperties`).
pub fn group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Create a `<dsp:sp>` element (`Shape`).
pub fn shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "sp").with_children(children)
}

/// Create a `<dsp:grpSp>` element (`GroupShape`).
pub fn group_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "grpSp").with_children(children)
}

/// Create a `<dsp:spTree>` element (`ShapeTree`).
pub fn shape_tree(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dsp", NAMESPACE_URI, "spTree").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 17;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 16;
