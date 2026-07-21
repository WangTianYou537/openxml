//! Auto-generated from `schemas_microsoft_com_office_drawing_2018_sketchyshapes.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2018/sketchyshapes` (prefix `ask`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2018/sketchyshapes";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ask";

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

static ATTRS_LINE_SKETCH_STYLE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sd", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_LINE_SKETCH_STYLE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_CustomGeometry2D/a:custGeom", property_name: None },
    ChildInfo { name: "a:CT_PresetGeometry2D/a:prstGeom", property_name: None },
    ChildInfo { name: "ask:CT_LineSketchTypeProperties/ask:type", property_name: None },
    ChildInfo { name: "ask:ST_LineSketchSeed/ask:seed", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/ask:extLst", property_name: None },
];
static CHILDREN_LINE_SKETCH_TYPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "ask:CT_Empty/ask:lineSketchNone", property_name: Some("LineSketchNoneEmpty") },
    ChildInfo { name: "ask:CT_Empty/ask:lineSketchCurved", property_name: Some("LineSketchCurvedEmpty") },
    ChildInfo { name: "ask:CT_Empty/ask:lineSketchFreehand", property_name: Some("LineSketchFreehandEmpty") },
    ChildInfo { name: "ask:CT_Empty/ask:lineSketchScribble", property_name: Some("LineSketchScribbleEmpty") },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "LineSketchNoneEmpty", local_name: "lineSketchNone", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LineSketchCurvedEmpty", local_name: "lineSketchCurved", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LineSketchFreehandEmpty", local_name: "lineSketchFreehand", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LineSketchScribbleEmpty", local_name: "lineSketchScribble", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LineSketchStyleProperties", local_name: "lineSketchStyleProps", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_SKETCH_STYLE_PROPERTIES, children: CHILDREN_LINE_SKETCH_STYLE_PROPERTIES },
    ElementInfo { class_name: "LineSketchTypeProperties", local_name: "type", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_SKETCH_TYPE_PROPERTIES },
    ElementInfo { class_name: "LineSketchSeed", local_name: "seed", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "ask", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
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

/// Create a `<ask:lineSketchNone>` element (`LineSketchNoneEmpty`).
pub fn line_sketch_none_empty() -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "lineSketchNone")
}

/// Create a `<ask:lineSketchCurved>` element (`LineSketchCurvedEmpty`).
pub fn line_sketch_curved_empty() -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "lineSketchCurved")
}

/// Create a `<ask:lineSketchFreehand>` element (`LineSketchFreehandEmpty`).
pub fn line_sketch_freehand_empty() -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "lineSketchFreehand")
}

/// Create a `<ask:lineSketchScribble>` element (`LineSketchScribbleEmpty`).
pub fn line_sketch_scribble_empty() -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "lineSketchScribble")
}

/// Create a `<ask:lineSketchStyleProps>` element (`LineSketchStyleProperties`).
pub fn line_sketch_style_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "lineSketchStyleProps").with_children(children)
}

/// Create a `<ask:type>` element (`LineSketchTypeProperties`).
pub fn line_sketch_type_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "type").with_children(children)
}

/// Create a `<ask:seed>` element (`LineSketchSeed`).
pub fn line_sketch_seed(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "seed").with_text(value)
}

/// Create a `<ask:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ask", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 9;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 8;
