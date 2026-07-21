//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_chartDrawing.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/chartDrawing` (prefix `cdr`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "cdr";

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

static CHILDREN_RELATIVE_ANCHOR_SIZE: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_Marker/cdr:from", property_name: Some("FromAnchor") },
    ChildInfo { name: "cdr:CT_Marker/cdr:to", property_name: Some("ToAnchor") },
    ChildInfo { name: "cdr:CT_Shape/cdr:sp", property_name: None },
    ChildInfo { name: "cdr:CT_GroupShape/cdr:grpSp", property_name: None },
    ChildInfo { name: "cdr:CT_GraphicFrame/cdr:graphicFrame", property_name: None },
    ChildInfo { name: "cdr:CT_Connector/cdr:cxnSp", property_name: None },
    ChildInfo { name: "cdr:CT_Picture/cdr:pic", property_name: None },
    ChildInfo { name: "cdr14:CT_ContentPart/cdr14:contentPart", property_name: None },
];
static CHILDREN_ABSOLUTE_ANCHOR_SIZE: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_Marker/cdr:from", property_name: Some("FromAnchor") },
    ChildInfo { name: "a:CT_PositiveSize2D/cdr:ext", property_name: Some("Extent") },
    ChildInfo { name: "cdr:CT_Shape/cdr:sp", property_name: None },
    ChildInfo { name: "cdr:CT_GroupShape/cdr:grpSp", property_name: None },
    ChildInfo { name: "cdr:CT_GraphicFrame/cdr:graphicFrame", property_name: None },
    ChildInfo { name: "cdr:CT_Connector/cdr:cxnSp", property_name: None },
    ChildInfo { name: "cdr:CT_Picture/cdr:pic", property_name: None },
    ChildInfo { name: "cdr14:CT_ContentPart/cdr14:contentPart", property_name: None },
];
static ATTRS_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":textlink", property_name: Some("TextLink"), type_name: "StringValue" },
    AttributeInfo { qname: ":fLocksText", property_name: Some("LockText"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_ShapeNonVisual/cdr:nvSpPr", property_name: Some("NonVisualShapeProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/cdr:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/cdr:style", property_name: Some("Style") },
    ChildInfo { name: "a:CT_TextBody/cdr:txBody", property_name: Some("TextBody") },
];
static CHILDREN_GROUP_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_GroupShapeNonVisual/cdr:nvGrpSpPr", property_name: Some("NonVisualGroupShapeProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/cdr:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "cdr:CT_Shape/cdr:sp", property_name: None },
    ChildInfo { name: "cdr:CT_GroupShape/cdr:grpSp", property_name: None },
    ChildInfo { name: "cdr:CT_GraphicFrame/cdr:graphicFrame", property_name: None },
    ChildInfo { name: "cdr:CT_Connector/cdr:cxnSp", property_name: None },
    ChildInfo { name: "cdr:CT_Picture/cdr:pic", property_name: None },
    ChildInfo { name: "cdr14:CT_ContentPart/cdr14:contentPart", property_name: None },
];
static ATTRS_GRAPHIC_FRAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_GRAPHIC_FRAME: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_GraphicFrameNonVisual/cdr:nvGraphicFramePr", property_name: Some("NonVisualGraphicFrameProperties") },
    ChildInfo { name: "a:CT_Transform2D/cdr:xfrm", property_name: Some("Transform") },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
];
static ATTRS_CONNECTION_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_CONNECTION_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_ConnectorNonVisual/cdr:nvCxnSpPr", property_name: Some("NonVisualConnectorShapeDrawingProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/cdr:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/cdr:style", property_name: Some("Style") },
];
static ATTRS_PICTURE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_PICTURE: &[ChildInfo] = &[
    ChildInfo { name: "cdr:CT_PictureNonVisual/cdr:nvPicPr", property_name: Some("NonVisualPictureProperties") },
    ChildInfo { name: "a:CT_BlipFillProperties/cdr:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_ShapeProperties/cdr:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/cdr:style", property_name: Some("Style") },
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
static ATTRS_NON_VISUAL_SHAPE_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":txBox", property_name: Some("TextBox"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeLocking/a:spLocks", property_name: Some("ShapeLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/cdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualDrawingShapeProps/cdr:cNvSpPr", property_name: Some("NonVisualShapeDrawingProperties") },
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
static CHILDREN_STYLE: &[ChildInfo] = &[
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
static CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectorLocking/a:cxnSpLocks", property_name: Some("ConnectionShapeLocks") },
    ChildInfo { name: "a:CT_Connection/a:stCxn", property_name: Some("StartConnection") },
    ChildInfo { name: "a:CT_Connection/a:endCxn", property_name: Some("EndConnection") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/cdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualConnectorProperties/cdr:cNvCxnSpPr", property_name: Some("NonVisualConnectionShapeProperties") },
];
static ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":preferRelativeResize", property_name: Some("PreferRelativeResize"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PictureLocking/a:picLocks", property_name: Some("PictureLocks") },
    ChildInfo { name: "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst", property_name: Some("NonVisualPicturePropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_PICTURE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/cdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualPictureProperties/cdr:cNvPicPr", property_name: Some("NonVisualPictureDrawingProperties") },
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
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks", property_name: Some("GraphicFrameLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/cdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/cdr:cNvGraphicFramePr", property_name: Some("NonVisualGraphicFrameDrawingProperties") },
];
static ATTRS_TRANSFORM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":flipH", property_name: Some("HorizontalFlip"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":flipV", property_name: Some("VerticalFlip"), type_name: "BooleanValue" },
];
static CHILDREN_TRANSFORM: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/a:off", property_name: Some("Offset") },
    ChildInfo { name: "a:CT_PositiveSize2D/a:ext", property_name: Some("Extents") },
];
static CHILDREN_NON_VISUAL_GROUP_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GroupLocking/a:grpSpLocks", property_name: Some("GroupShapeLocks") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst", property_name: Some("NonVisualGroupDrawingShapePropsExtensionList") },
];
static CHILDREN_FROM_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "cdr:ST_MarkerCoordinate/cdr:x", property_name: Some("XPosition") },
    ChildInfo { name: "cdr:ST_MarkerCoordinate/cdr:y", property_name: Some("YPosition") },
];
static CHILDREN_TO_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "cdr:ST_MarkerCoordinate/cdr:x", property_name: Some("XPosition") },
    ChildInfo { name: "cdr:ST_MarkerCoordinate/cdr:y", property_name: Some("YPosition") },
];
static ATTRS_EXTENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int64Value" },
];
static CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/cdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/cdr:cNvGrpSpPr", property_name: Some("NonVisualGroupShapeDrawingProperties") },
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "RelativeAnchorSize", local_name: "relSizeAnchor", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RELATIVE_ANCHOR_SIZE },
    ElementInfo { class_name: "AbsoluteAnchorSize", local_name: "absSizeAnchor", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ABSOLUTE_ANCHOR_SIZE },
    ElementInfo { class_name: "Shape", local_name: "sp", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE, children: CHILDREN_SHAPE },
    ElementInfo { class_name: "GroupShape", local_name: "grpSp", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_SHAPE },
    ElementInfo { class_name: "GraphicFrame", local_name: "graphicFrame", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRAPHIC_FRAME, children: CHILDREN_GRAPHIC_FRAME },
    ElementInfo { class_name: "ConnectionShape", local_name: "cxnSp", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTION_SHAPE, children: CHILDREN_CONNECTION_SHAPE },
    ElementInfo { class_name: "Picture", local_name: "pic", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PICTURE, children: CHILDREN_PICTURE },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualShapeDrawingProperties", local_name: "cNvSpPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_SHAPE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualShapeProperties", local_name: "nvSpPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Style", local_name: "style", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STYLE },
    ElementInfo { class_name: "TextBody", local_name: "txBody", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_BODY },
    ElementInfo { class_name: "NonVisualConnectionShapeProperties", local_name: "cNvCxnSpPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES },
    ElementInfo { class_name: "NonVisualConnectorShapeDrawingProperties", local_name: "nvCxnSpPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureDrawingProperties", local_name: "cNvPicPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureProperties", local_name: "nvPicPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_PICTURE_PROPERTIES },
    ElementInfo { class_name: "BlipFill", local_name: "blipFill", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP_FILL, children: CHILDREN_BLIP_FILL },
    ElementInfo { class_name: "NonVisualGraphicFrameDrawingProperties", local_name: "cNvGraphicFramePr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualGraphicFrameProperties", local_name: "nvGraphicFramePr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES },
    ElementInfo { class_name: "Transform", local_name: "xfrm", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM, children: CHILDREN_TRANSFORM },
    ElementInfo { class_name: "NonVisualGroupShapeDrawingProperties", local_name: "cNvGrpSpPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "XPosition", local_name: "x", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "YPosition", local_name: "y", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FromAnchor", local_name: "from", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FROM_ANCHOR },
    ElementInfo { class_name: "ToAnchor", local_name: "to", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TO_ANCHOR },
    ElementInfo { class_name: "Extent", local_name: "ext", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXTENT, children: &[] },
    ElementInfo { class_name: "NonVisualGroupShapeProperties", local_name: "nvGrpSpPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "GroupShapeProperties", local_name: "grpSpPr", prefix: "cdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_PROPERTIES, children: CHILDREN_GROUP_SHAPE_PROPERTIES },
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

/// Create a `<cdr:relSizeAnchor>` element (`RelativeAnchorSize`).
pub fn relative_anchor_size(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "relSizeAnchor").with_children(children)
}

/// Create a `<cdr:absSizeAnchor>` element (`AbsoluteAnchorSize`).
pub fn absolute_anchor_size(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "absSizeAnchor").with_children(children)
}

/// Create a `<cdr:sp>` element (`Shape`).
pub fn shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "sp").with_children(children)
}

/// Create a `<cdr:grpSp>` element (`GroupShape`).
pub fn group_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "grpSp").with_children(children)
}

/// Create a `<cdr:graphicFrame>` element (`GraphicFrame`).
pub fn graphic_frame(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "graphicFrame").with_children(children)
}

/// Create a `<cdr:cxnSp>` element (`ConnectionShape`).
pub fn connection_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "cxnSp").with_children(children)
}

/// Create a `<cdr:pic>` element (`Picture`).
pub fn picture(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "pic").with_children(children)
}

/// Create a `<cdr:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<cdr:cNvSpPr>` element (`NonVisualShapeDrawingProperties`).
pub fn non_visual_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "cNvSpPr").with_children(children)
}

/// Create a `<cdr:nvSpPr>` element (`NonVisualShapeProperties`).
pub fn non_visual_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "nvSpPr").with_children(children)
}

/// Create a `<cdr:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<cdr:style>` element (`Style`).
pub fn style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<cdr:txBody>` element (`TextBody`).
pub fn text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "txBody").with_children(children)
}

/// Create a `<cdr:cNvCxnSpPr>` element (`NonVisualConnectionShapeProperties`).
pub fn non_visual_connection_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "cNvCxnSpPr").with_children(children)
}

/// Create a `<cdr:nvCxnSpPr>` element (`NonVisualConnectorShapeDrawingProperties`).
pub fn non_visual_connector_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "nvCxnSpPr").with_children(children)
}

/// Create a `<cdr:cNvPicPr>` element (`NonVisualPictureDrawingProperties`).
pub fn non_visual_picture_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "cNvPicPr").with_children(children)
}

/// Create a `<cdr:nvPicPr>` element (`NonVisualPictureProperties`).
pub fn non_visual_picture_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "nvPicPr").with_children(children)
}

/// Create a `<cdr:blipFill>` element (`BlipFill`).
pub fn blip_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "blipFill").with_children(children)
}

/// Create a `<cdr:cNvGraphicFramePr>` element (`NonVisualGraphicFrameDrawingProperties`).
pub fn non_visual_graphic_frame_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "cNvGraphicFramePr").with_children(children)
}

/// Create a `<cdr:nvGraphicFramePr>` element (`NonVisualGraphicFrameProperties`).
pub fn non_visual_graphic_frame_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "nvGraphicFramePr").with_children(children)
}

/// Create a `<cdr:xfrm>` element (`Transform`).
pub fn transform(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Create a `<cdr:cNvGrpSpPr>` element (`NonVisualGroupShapeDrawingProperties`).
pub fn non_visual_group_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "cNvGrpSpPr").with_children(children)
}

/// Create a `<cdr:x>` element (`XPosition`).
pub fn x_position(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "x").with_text(value)
}

/// Create a `<cdr:y>` element (`YPosition`).
pub fn y_position(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "y").with_text(value)
}

/// Create a `<cdr:from>` element (`FromAnchor`).
pub fn from_anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "from").with_children(children)
}

/// Create a `<cdr:to>` element (`ToAnchor`).
pub fn to_anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "to").with_children(children)
}

/// Create a `<cdr:ext>` element (`Extent`).
pub fn extent() -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "ext")
}

/// Create a `<cdr:nvGrpSpPr>` element (`NonVisualGroupShapeProperties`).
pub fn non_visual_group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "nvGrpSpPr").with_children(children)
}

/// Create a `<cdr:grpSpPr>` element (`GroupShapeProperties`).
pub fn group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 30;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 29;
