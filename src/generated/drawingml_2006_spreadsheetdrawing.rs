//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_spreadsheetDrawing.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing` (prefix `xdr`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xdr";

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

static ATTRS_TWO_CELL_ANCHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":editAs", property_name: Some("EditAs"), type_name: "EnumValue" },
];
static CHILDREN_TWO_CELL_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_Marker/xdr:from", property_name: Some("FromMarker") },
    ChildInfo { name: "xdr:CT_Marker/xdr:to", property_name: Some("ToMarker") },
    ChildInfo { name: "xdr:CT_Shape/xdr:sp", property_name: None },
    ChildInfo { name: "xdr:CT_GroupShape/xdr:grpSp", property_name: None },
    ChildInfo { name: "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame", property_name: None },
    ChildInfo { name: "xdr:CT_Connector/xdr:cxnSp", property_name: None },
    ChildInfo { name: "xdr:CT_Picture/xdr:pic", property_name: None },
    ChildInfo { name: "xdr14:CT_ContentPart/xdr:contentPart", property_name: None },
    ChildInfo { name: "xdr:CT_AnchorClientData/xdr:clientData", property_name: None },
];
static CHILDREN_ONE_CELL_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_Marker/xdr:from", property_name: Some("FromMarker") },
    ChildInfo { name: "a:CT_PositiveSize2D/xdr:ext", property_name: Some("Extent") },
    ChildInfo { name: "xdr:CT_Shape/xdr:sp", property_name: None },
    ChildInfo { name: "xdr:CT_GroupShape/xdr:grpSp", property_name: None },
    ChildInfo { name: "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame", property_name: None },
    ChildInfo { name: "xdr:CT_Connector/xdr:cxnSp", property_name: None },
    ChildInfo { name: "xdr:CT_Picture/xdr:pic", property_name: None },
    ChildInfo { name: "xdr14:CT_ContentPart/xdr:contentPart", property_name: None },
    ChildInfo { name: "xdr:CT_AnchorClientData/xdr:clientData", property_name: None },
];
static CHILDREN_ABSOLUTE_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/xdr:pos", property_name: Some("Position") },
    ChildInfo { name: "a:CT_PositiveSize2D/xdr:ext", property_name: Some("Extent") },
    ChildInfo { name: "xdr:CT_Shape/xdr:sp", property_name: None },
    ChildInfo { name: "xdr:CT_GroupShape/xdr:grpSp", property_name: None },
    ChildInfo { name: "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame", property_name: None },
    ChildInfo { name: "xdr:CT_Connector/xdr:cxnSp", property_name: None },
    ChildInfo { name: "xdr:CT_Picture/xdr:pic", property_name: None },
    ChildInfo { name: "xdr14:CT_ContentPart/xdr:contentPart", property_name: None },
    ChildInfo { name: "xdr:CT_AnchorClientData/xdr:clientData", property_name: None },
];
static ATTRS_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":textlink", property_name: Some("TextLink"), type_name: "StringValue" },
    AttributeInfo { qname: ":fLocksText", property_name: Some("LockText"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_ShapeNonVisual/xdr:nvSpPr", property_name: Some("NonVisualShapeProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/xdr:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/xdr:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_TextBody/xdr:txBody", property_name: Some("TextBody") },
];
static CHILDREN_GROUP_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_GroupShapeNonVisual/xdr:nvGrpSpPr", property_name: Some("NonVisualGroupShapeProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/xdr:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "xdr:CT_Shape/xdr:sp", property_name: None },
    ChildInfo { name: "xdr:CT_GroupShape/xdr:grpSp", property_name: None },
    ChildInfo { name: "xdr:CT_GraphicalObjectFrame/xdr:graphicFrame", property_name: None },
    ChildInfo { name: "xdr:CT_Connector/xdr:cxnSp", property_name: None },
    ChildInfo { name: "xdr:CT_Picture/xdr:pic", property_name: None },
    ChildInfo { name: "xdr14:CT_ContentPart/xdr14:contentPart", property_name: None },
];
static ATTRS_GRAPHIC_FRAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_GRAPHIC_FRAME: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_GraphicalObjectFrameNonVisual/xdr:nvGraphicFramePr", property_name: Some("NonVisualGraphicFrameProperties") },
    ChildInfo { name: "a:CT_Transform2D/xdr:xfrm", property_name: Some("Transform") },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
];
static ATTRS_CONNECTION_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_CONNECTION_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_ConnectorNonVisual/xdr:nvCxnSpPr", property_name: Some("NonVisualConnectionShapeProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/xdr:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/xdr:style", property_name: Some("ShapeStyle") },
];
static ATTRS_PICTURE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
];
static CHILDREN_PICTURE: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_PictureNonVisual/xdr:nvPicPr", property_name: Some("NonVisualPictureProperties") },
    ChildInfo { name: "a:CT_BlipFillProperties2/xdr:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_ShapeProperties/xdr:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/xdr:style", property_name: Some("ShapeStyle") },
];
static ATTRS_CONTENT_PART: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("RelationshipId"), type_name: "StringValue" },
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_CONTENT_PART: &[ChildInfo] = &[
    ChildInfo { name: "xdr14:CT_ContentPartNonVisual/xdr14:nvContentPartPr", property_name: Some("ExcelNonVisualContentPartShapeProperties") },
    ChildInfo { name: "xdr14:CT_ApplicationNonVisualDrawingProps/xdr14:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_Transform2D/xdr14:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/xdr14:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_WORKSHEET_DRAWING: &[ChildInfo] = &[
    ChildInfo { name: "xdr:CT_TwoCellAnchor/xdr:twoCellAnchor", property_name: None },
    ChildInfo { name: "xdr:CT_OneCellAnchor/xdr:oneCellAnchor", property_name: None },
    ChildInfo { name: "xdr:CT_AbsoluteAnchor/xdr:absoluteAnchor", property_name: None },
];
static CHILDREN_NON_VISUAL_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualDrawingShapeProps/xdr:cNvSpPr", property_name: Some("NonVisualShapeDrawingProperties") },
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
static CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualConnectorProperties/xdr:cNvCxnSpPr", property_name: Some("NonVisualConnectorShapeDrawingProperties") },
];
static CHILDREN_NON_VISUAL_PICTURE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualPictureProperties/xdr:cNvPicPr", property_name: Some("NonVisualPictureDrawingProperties") },
];
static ATTRS_BLIP_FILL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rotWithShape", property_name: Some("RotateWithShape"), type_name: "BooleanValue" },
];
static CHILDREN_BLIP_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Blip/a:blip", property_name: Some("Blip") },
    ChildInfo { name: "a:CT_RelativeRect/a:srcRect", property_name: Some("SourceRectangle") },
    ChildInfo { name: "a:CT_TileInfoProperties/a:tile", property_name: None },
    ChildInfo { name: "a:CT_StretchInfoProperties/a:stretch", property_name: None },
];
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/xdr:cNvGraphicFramePr", property_name: Some("NonVisualGraphicFrameDrawingProperties") },
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
static CHILDREN_FROM_MARKER: &[ChildInfo] = &[
    ChildInfo { name: "xdr:ST_ColID/xdr:col", property_name: Some("ColumnId") },
    ChildInfo { name: "a:ST_Coordinate/xdr:colOff", property_name: Some("ColumnOffset") },
    ChildInfo { name: "xdr:ST_RowID/xdr:row", property_name: Some("RowId") },
    ChildInfo { name: "a:ST_Coordinate/xdr:rowOff", property_name: Some("RowOffset") },
];
static CHILDREN_TO_MARKER: &[ChildInfo] = &[
    ChildInfo { name: "xdr:ST_ColID/xdr:col", property_name: Some("ColumnId") },
    ChildInfo { name: "a:ST_Coordinate/xdr:colOff", property_name: Some("ColumnOffset") },
    ChildInfo { name: "xdr:ST_RowID/xdr:row", property_name: Some("RowId") },
    ChildInfo { name: "a:ST_Coordinate/xdr:rowOff", property_name: Some("RowOffset") },
];
static ATTRS_CLIENT_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fLocksWithSheet", property_name: Some("LockWithSheet"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fPrintsWithSheet", property_name: Some("PrintWithSheet"), type_name: "BooleanValue" },
];
static ATTRS_EXTENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int64Value" },
];
static ATTRS_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_NON_VISUAL_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":descr", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":title", property_name: None, type_name: "StringValue" },
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
static CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectorLocking/a:cxnSpLocks", property_name: Some("ConnectionShapeLocks") },
    ChildInfo { name: "a:CT_Connection/a:stCxn", property_name: Some("StartConnection") },
    ChildInfo { name: "a:CT_Connection/a:endCxn", property_name: Some("EndConnection") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":preferRelativeResize", property_name: Some("PreferRelativeResize"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PictureLocking/a:picLocks", property_name: Some("PictureLocks") },
    ChildInfo { name: "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst", property_name: Some("NonVisualPicturePropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks", property_name: Some("GraphicFrameLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_GROUP_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GroupLocking/a:grpSpLocks", property_name: Some("GroupShapeLocks") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst", property_name: Some("NonVisualGroupDrawingShapePropsExtensionList") },
];
static CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SpreadSheetNonVisualDrawingProps/xdr:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/xdr:cNvGrpSpPr", property_name: Some("NonVisualGroupShapeDrawingProperties") },
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
    ElementInfo { class_name: "TwoCellAnchor", local_name: "twoCellAnchor", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TWO_CELL_ANCHOR, children: CHILDREN_TWO_CELL_ANCHOR },
    ElementInfo { class_name: "OneCellAnchor", local_name: "oneCellAnchor", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ONE_CELL_ANCHOR },
    ElementInfo { class_name: "AbsoluteAnchor", local_name: "absoluteAnchor", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ABSOLUTE_ANCHOR },
    ElementInfo { class_name: "Shape", local_name: "sp", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE, children: CHILDREN_SHAPE },
    ElementInfo { class_name: "GroupShape", local_name: "grpSp", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_SHAPE },
    ElementInfo { class_name: "GraphicFrame", local_name: "graphicFrame", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRAPHIC_FRAME, children: CHILDREN_GRAPHIC_FRAME },
    ElementInfo { class_name: "ConnectionShape", local_name: "cxnSp", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTION_SHAPE, children: CHILDREN_CONNECTION_SHAPE },
    ElementInfo { class_name: "Picture", local_name: "pic", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PICTURE, children: CHILDREN_PICTURE },
    ElementInfo { class_name: "ContentPart", local_name: "contentPart", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTENT_PART, children: CHILDREN_CONTENT_PART },
    ElementInfo { class_name: "WorksheetDrawing", local_name: "wsDr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WORKSHEET_DRAWING },
    ElementInfo { class_name: "NonVisualShapeProperties", local_name: "nvSpPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeStyle", local_name: "style", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_STYLE },
    ElementInfo { class_name: "TextBody", local_name: "txBody", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_BODY },
    ElementInfo { class_name: "NonVisualConnectionShapeProperties", local_name: "nvCxnSpPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureProperties", local_name: "nvPicPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_PICTURE_PROPERTIES },
    ElementInfo { class_name: "BlipFill", local_name: "blipFill", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP_FILL, children: CHILDREN_BLIP_FILL },
    ElementInfo { class_name: "NonVisualGraphicFrameProperties", local_name: "nvGraphicFramePr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES },
    ElementInfo { class_name: "Transform", local_name: "xfrm", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM, children: CHILDREN_TRANSFORM },
    ElementInfo { class_name: "ColumnId", local_name: "col", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ColumnOffset", local_name: "colOff", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RowOffset", local_name: "rowOff", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RowId", local_name: "row", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FromMarker", local_name: "from", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FROM_MARKER },
    ElementInfo { class_name: "ToMarker", local_name: "to", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TO_MARKER },
    ElementInfo { class_name: "ClientData", local_name: "clientData", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CLIENT_DATA, children: &[] },
    ElementInfo { class_name: "Extent", local_name: "ext", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXTENT, children: &[] },
    ElementInfo { class_name: "Position", local_name: "pos", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POSITION, children: &[] },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualShapeDrawingProperties", local_name: "cNvSpPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_SHAPE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualConnectorShapeDrawingProperties", local_name: "cNvCxnSpPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureDrawingProperties", local_name: "cNvPicPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualGraphicFrameDrawingProperties", local_name: "cNvGraphicFramePr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualGroupShapeDrawingProperties", local_name: "cNvGrpSpPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualGroupShapeProperties", local_name: "nvGrpSpPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "GroupShapeProperties", local_name: "grpSpPr", prefix: "xdr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_PROPERTIES, children: CHILDREN_GROUP_SHAPE_PROPERTIES },
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

/// Create a `<xdr:twoCellAnchor>` element (`TwoCellAnchor`).
pub fn two_cell_anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "twoCellAnchor").with_children(children)
}

/// Create a `<xdr:oneCellAnchor>` element (`OneCellAnchor`).
pub fn one_cell_anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "oneCellAnchor").with_children(children)
}

/// Create a `<xdr:absoluteAnchor>` element (`AbsoluteAnchor`).
pub fn absolute_anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "absoluteAnchor").with_children(children)
}

/// Create a `<xdr:sp>` element (`Shape`).
pub fn shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "sp").with_children(children)
}

/// Create a `<xdr:grpSp>` element (`GroupShape`).
pub fn group_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "grpSp").with_children(children)
}

/// Create a `<xdr:graphicFrame>` element (`GraphicFrame`).
pub fn graphic_frame(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "graphicFrame").with_children(children)
}

/// Create a `<xdr:cxnSp>` element (`ConnectionShape`).
pub fn connection_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "cxnSp").with_children(children)
}

/// Create a `<xdr:pic>` element (`Picture`).
pub fn picture(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "pic").with_children(children)
}

/// Create a `<xdr:contentPart>` element (`ContentPart`).
pub fn content_part(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "contentPart").with_children(children)
}

/// Create a `<xdr:wsDr>` element (`WorksheetDrawing`).
pub fn worksheet_drawing(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "wsDr").with_children(children)
}

/// Create a `<xdr:nvSpPr>` element (`NonVisualShapeProperties`).
pub fn non_visual_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "nvSpPr").with_children(children)
}

/// Create a `<xdr:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<xdr:style>` element (`ShapeStyle`).
pub fn shape_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<xdr:txBody>` element (`TextBody`).
pub fn text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "txBody").with_children(children)
}

/// Create a `<xdr:nvCxnSpPr>` element (`NonVisualConnectionShapeProperties`).
pub fn non_visual_connection_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "nvCxnSpPr").with_children(children)
}

/// Create a `<xdr:nvPicPr>` element (`NonVisualPictureProperties`).
pub fn non_visual_picture_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "nvPicPr").with_children(children)
}

/// Create a `<xdr:blipFill>` element (`BlipFill`).
pub fn blip_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "blipFill").with_children(children)
}

/// Create a `<xdr:nvGraphicFramePr>` element (`NonVisualGraphicFrameProperties`).
pub fn non_visual_graphic_frame_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "nvGraphicFramePr").with_children(children)
}

/// Create a `<xdr:xfrm>` element (`Transform`).
pub fn transform(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Create a `<xdr:col>` element (`ColumnId`).
pub fn column_id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "col").with_text(value)
}

/// Create a `<xdr:colOff>` element (`ColumnOffset`).
pub fn column_offset(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "colOff").with_text(value)
}

/// Create a `<xdr:rowOff>` element (`RowOffset`).
pub fn row_offset(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "rowOff").with_text(value)
}

/// Create a `<xdr:row>` element (`RowId`).
pub fn row_id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "row").with_text(value)
}

/// Create a `<xdr:from>` element (`FromMarker`).
pub fn from_marker(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "from").with_children(children)
}

/// Create a `<xdr:to>` element (`ToMarker`).
pub fn to_marker(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "to").with_children(children)
}

/// Create a `<xdr:clientData>` element (`ClientData`).
pub fn client_data() -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "clientData")
}

/// Create a `<xdr:ext>` element (`Extent`).
pub fn extent() -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "ext")
}

/// Create a `<xdr:pos>` element (`Position`).
pub fn position() -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "pos")
}

/// Create a `<xdr:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<xdr:cNvSpPr>` element (`NonVisualShapeDrawingProperties`).
pub fn non_visual_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "cNvSpPr").with_children(children)
}

/// Create a `<xdr:cNvCxnSpPr>` element (`NonVisualConnectorShapeDrawingProperties`).
pub fn non_visual_connector_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "cNvCxnSpPr").with_children(children)
}

/// Create a `<xdr:cNvPicPr>` element (`NonVisualPictureDrawingProperties`).
pub fn non_visual_picture_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "cNvPicPr").with_children(children)
}

/// Create a `<xdr:cNvGraphicFramePr>` element (`NonVisualGraphicFrameDrawingProperties`).
pub fn non_visual_graphic_frame_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "cNvGraphicFramePr").with_children(children)
}

/// Create a `<xdr:cNvGrpSpPr>` element (`NonVisualGroupShapeDrawingProperties`).
pub fn non_visual_group_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "cNvGrpSpPr").with_children(children)
}

/// Create a `<xdr:nvGrpSpPr>` element (`NonVisualGroupShapeProperties`).
pub fn non_visual_group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "nvGrpSpPr").with_children(children)
}

/// Create a `<xdr:grpSpPr>` element (`GroupShapeProperties`).
pub fn group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xdr", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 37;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 36;
