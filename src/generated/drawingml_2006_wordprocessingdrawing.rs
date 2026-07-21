//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_wordprocessingDrawing.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing` (prefix `wp`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wp";

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

static ATTRS_WRAP_SQUARE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":wrapText", property_name: Some("WrapText"), type_name: "EnumValue" },
    AttributeInfo { qname: ":distT", property_name: Some("DistanceFromTop"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distB", property_name: Some("DistanceFromBottom"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distL", property_name: Some("DistanceFromLeft"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distR", property_name: Some("DistanceFromRight"), type_name: "UInt32Value" },
];
static CHILDREN_WRAP_SQUARE: &[ChildInfo] = &[
    ChildInfo { name: "wp:CT_EffectExtent/wp:effectExtent", property_name: Some("EffectExtent") },
];
static ATTRS_WRAP_TIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":wrapText", property_name: Some("WrapText"), type_name: "EnumValue" },
    AttributeInfo { qname: ":distL", property_name: Some("DistanceFromLeft"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distR", property_name: Some("DistanceFromRight"), type_name: "UInt32Value" },
];
static CHILDREN_WRAP_TIGHT: &[ChildInfo] = &[
    ChildInfo { name: "wp:CT_WrapPath/wp:wrapPolygon", property_name: Some("WrapPolygon") },
];
static ATTRS_WRAP_THROUGH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":wrapText", property_name: Some("WrapText"), type_name: "EnumValue" },
    AttributeInfo { qname: ":distL", property_name: Some("DistanceFromLeft"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distR", property_name: Some("DistanceFromRight"), type_name: "UInt32Value" },
];
static CHILDREN_WRAP_THROUGH: &[ChildInfo] = &[
    ChildInfo { name: "wp:CT_WrapPath/wp:wrapPolygon", property_name: Some("WrapPolygon") },
];
static ATTRS_WRAP_TOP_BOTTOM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":distT", property_name: Some("DistanceFromTop"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distB", property_name: Some("DistanceFromBottom"), type_name: "UInt32Value" },
];
static CHILDREN_WRAP_TOP_BOTTOM: &[ChildInfo] = &[
    ChildInfo { name: "wp:CT_EffectExtent/wp:effectExtent", property_name: Some("EffectExtent") },
];
static ATTRS_INLINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":distT", property_name: Some("DistanceFromTop"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distB", property_name: Some("DistanceFromBottom"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distL", property_name: Some("DistanceFromLeft"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distR", property_name: Some("DistanceFromRight"), type_name: "UInt32Value" },
    AttributeInfo { qname: "wp14:anchorId", property_name: None, type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wp14:editId", property_name: Some("EditId"), type_name: "HexBinaryValue" },
];
static CHILDREN_INLINE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveSize2D/wp:extent", property_name: Some("Extent") },
    ChildInfo { name: "wp:CT_EffectExtent/wp:effectExtent", property_name: Some("EffectExtent") },
    ChildInfo { name: "a:CT_NonVisualDrawingProps/wp:docPr", property_name: Some("DocProperties") },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/wp:cNvGraphicFramePr", property_name: Some("NonVisualGraphicFrameDrawingProperties") },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
];
static ATTRS_ANCHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":distT", property_name: Some("DistanceFromTop"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distB", property_name: Some("DistanceFromBottom"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distL", property_name: Some("DistanceFromLeft"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":distR", property_name: Some("DistanceFromRight"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":simplePos", property_name: Some("SimplePos"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":relativeHeight", property_name: Some("RelativeHeight"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":behindDoc", property_name: Some("BehindDoc"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":locked", property_name: Some("Locked"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":layoutInCell", property_name: Some("LayoutInCell"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":allowOverlap", property_name: Some("AllowOverlap"), type_name: "BooleanValue" },
    AttributeInfo { qname: "wp14:editId", property_name: Some("EditId"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wp14:anchorId", property_name: None, type_name: "HexBinaryValue" },
];
static CHILDREN_ANCHOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/wp:simplePos", property_name: Some("SimplePosition") },
    ChildInfo { name: "wp:CT_PosH/wp:positionH", property_name: Some("HorizontalPosition") },
    ChildInfo { name: "wp:CT_PosV/wp:positionV", property_name: Some("VerticalPosition") },
    ChildInfo { name: "a:CT_PositiveSize2D/wp:extent", property_name: Some("Extent") },
    ChildInfo { name: "wp:CT_EffectExtent/wp:effectExtent", property_name: Some("EffectExtent") },
    ChildInfo { name: "wp:CT_WrapNone/wp:wrapNone", property_name: None },
    ChildInfo { name: "wp:CT_WrapSquare/wp:wrapSquare", property_name: None },
    ChildInfo { name: "wp:CT_WrapTight/wp:wrapTight", property_name: None },
    ChildInfo { name: "wp:CT_WrapThrough/wp:wrapThrough", property_name: None },
    ChildInfo { name: "wp:CT_WrapTopBottom/wp:wrapTopAndBottom", property_name: None },
    ChildInfo { name: "a:CT_NonVisualDrawingProps/wp:docPr", property_name: None },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/wp:cNvGraphicFramePr", property_name: None },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: None },
    ChildInfo { name: "wp14:CT_SizeRelH/wp14:sizeRelH", property_name: None },
    ChildInfo { name: "wp14:CT_SizeRelV/wp14:sizeRelV", property_name: None },
];
static ATTRS_START_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_LINE_TO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_SIMPLE_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_EFFECT_EXTENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: Some("LeftEdge"), type_name: "Int64Value" },
    AttributeInfo { qname: ":t", property_name: Some("TopEdge"), type_name: "Int64Value" },
    AttributeInfo { qname: ":r", property_name: Some("RightEdge"), type_name: "Int64Value" },
    AttributeInfo { qname: ":b", property_name: Some("BottomEdge"), type_name: "Int64Value" },
];
static ATTRS_WRAP_POLYGON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":edited", property_name: Some("Edited"), type_name: "BooleanValue" },
];
static CHILDREN_WRAP_POLYGON: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/wp:start", property_name: Some("StartPoint") },
    ChildInfo { name: "a:CT_Point2D/wp:lineTo", property_name: None },
];
static ATTRS_HORIZONTAL_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":relativeFrom", property_name: Some("RelativeFrom"), type_name: "EnumValue" },
];
static CHILDREN_HORIZONTAL_POSITION: &[ChildInfo] = &[
    ChildInfo { name: "wp:ST_AlignH/wp:align", property_name: Some("HorizontalAlignment") },
    ChildInfo { name: "wp:ST_PositionOffset/wp:posOffset", property_name: Some("PositionOffset") },
    ChildInfo { name: "a:ST_Percentage/wp14:pctPosHOffset", property_name: Some("PercentagePositionHeightOffset") },
];
static ATTRS_VERTICAL_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":relativeFrom", property_name: Some("RelativeFrom"), type_name: "EnumValue" },
];
static CHILDREN_VERTICAL_POSITION: &[ChildInfo] = &[
    ChildInfo { name: "wp:ST_AlignV/wp:align", property_name: Some("VerticalAlignment") },
    ChildInfo { name: "wp:ST_PositionOffset/wp:posOffset", property_name: Some("PositionOffset") },
    ChildInfo { name: "a:ST_Percentage/wp14:pctPosVOffset", property_name: Some("PercentagePositionVerticalOffset") },
];
static ATTRS_EXTENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int64Value" },
];
static ATTRS_DOC_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":descr", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
];
static CHILDREN_DOC_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: Some("HyperlinkOnClick") },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkHover", property_name: Some("HyperlinkOnHover") },
    ChildInfo { name: "a:CT_NonVisualDrawingPropsExtensionList/a:extLst", property_name: Some("NonVisualDrawingPropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks", property_name: Some("GraphicFrameLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "WrapNone", local_name: "wrapNone", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "WrapSquare", local_name: "wrapSquare", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WRAP_SQUARE, children: CHILDREN_WRAP_SQUARE },
    ElementInfo { class_name: "WrapTight", local_name: "wrapTight", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WRAP_TIGHT, children: CHILDREN_WRAP_TIGHT },
    ElementInfo { class_name: "WrapThrough", local_name: "wrapThrough", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WRAP_THROUGH, children: CHILDREN_WRAP_THROUGH },
    ElementInfo { class_name: "WrapTopBottom", local_name: "wrapTopAndBottom", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WRAP_TOP_BOTTOM, children: CHILDREN_WRAP_TOP_BOTTOM },
    ElementInfo { class_name: "Inline", local_name: "inline", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_INLINE, children: CHILDREN_INLINE },
    ElementInfo { class_name: "Anchor", local_name: "anchor", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANCHOR, children: CHILDREN_ANCHOR },
    ElementInfo { class_name: "StartPoint", local_name: "start", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_START_POINT, children: &[] },
    ElementInfo { class_name: "LineTo", local_name: "lineTo", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LINE_TO, children: &[] },
    ElementInfo { class_name: "SimplePosition", local_name: "simplePos", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SIMPLE_POSITION, children: &[] },
    ElementInfo { class_name: "EffectExtent", local_name: "effectExtent", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EFFECT_EXTENT, children: &[] },
    ElementInfo { class_name: "WrapPolygon", local_name: "wrapPolygon", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WRAP_POLYGON, children: CHILDREN_WRAP_POLYGON },
    ElementInfo { class_name: "HorizontalPosition", local_name: "positionH", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HORIZONTAL_POSITION, children: CHILDREN_HORIZONTAL_POSITION },
    ElementInfo { class_name: "VerticalPosition", local_name: "positionV", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VERTICAL_POSITION, children: CHILDREN_VERTICAL_POSITION },
    ElementInfo { class_name: "Extent", local_name: "extent", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXTENT, children: &[] },
    ElementInfo { class_name: "DocProperties", local_name: "docPr", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DOC_PROPERTIES, children: CHILDREN_DOC_PROPERTIES },
    ElementInfo { class_name: "NonVisualGraphicFrameDrawingProperties", local_name: "cNvGraphicFramePr", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES },
    ElementInfo { class_name: "VerticalAlignment", local_name: "align", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PositionOffset", local_name: "posOffset", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HorizontalAlignment", local_name: "align", prefix: "wp", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
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

/// Create a `<wp:wrapNone>` element (`WrapNone`).
pub fn wrap_none() -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "wrapNone")
}

/// Create a `<wp:wrapSquare>` element (`WrapSquare`).
pub fn wrap_square(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "wrapSquare").with_children(children)
}

/// Create a `<wp:wrapTight>` element (`WrapTight`).
pub fn wrap_tight(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "wrapTight").with_children(children)
}

/// Create a `<wp:wrapThrough>` element (`WrapThrough`).
pub fn wrap_through(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "wrapThrough").with_children(children)
}

/// Create a `<wp:wrapTopAndBottom>` element (`WrapTopBottom`).
pub fn wrap_top_bottom(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "wrapTopAndBottom").with_children(children)
}

/// Create a `<wp:inline>` element (`Inline`).
pub fn inline(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "inline").with_children(children)
}

/// Create a `<wp:anchor>` element (`Anchor`).
pub fn anchor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "anchor").with_children(children)
}

/// Create a `<wp:start>` element (`StartPoint`).
pub fn start_point() -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "start")
}

/// Create a `<wp:lineTo>` element (`LineTo`).
pub fn line_to() -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "lineTo")
}

/// Create a `<wp:simplePos>` element (`SimplePosition`).
pub fn simple_position() -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "simplePos")
}

/// Create a `<wp:effectExtent>` element (`EffectExtent`).
pub fn effect_extent() -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "effectExtent")
}

/// Create a `<wp:wrapPolygon>` element (`WrapPolygon`).
pub fn wrap_polygon(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "wrapPolygon").with_children(children)
}

/// Create a `<wp:positionH>` element (`HorizontalPosition`).
pub fn horizontal_position(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "positionH").with_children(children)
}

/// Create a `<wp:positionV>` element (`VerticalPosition`).
pub fn vertical_position(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "positionV").with_children(children)
}

/// Create a `<wp:extent>` element (`Extent`).
pub fn extent() -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "extent")
}

/// Create a `<wp:docPr>` element (`DocProperties`).
pub fn doc_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "docPr").with_children(children)
}

/// Create a `<wp:cNvGraphicFramePr>` element (`NonVisualGraphicFrameDrawingProperties`).
pub fn non_visual_graphic_frame_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "cNvGraphicFramePr").with_children(children)
}

/// Create a `<wp:align>` element (`VerticalAlignment`).
pub fn vertical_alignment(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "align").with_text(value)
}

/// Create a `<wp:posOffset>` element (`PositionOffset`).
pub fn position_offset(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "posOffset").with_text(value)
}

/// Create a `<wp:align>` element (`HorizontalAlignment`).
pub fn horizontal_alignment(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wp", NAMESPACE_URI, "align").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 21;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 20;
