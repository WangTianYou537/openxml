//! Auto-generated from `schemas_microsoft_com_office_drawing_2007_8_2_chart.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2007/8/2/chart` (prefix `c14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2007/8/2/chart";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "c14";

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

static CHILDREN_PIVOT_OPTIONS: &[ChildInfo] = &[
    ChildInfo { name: "c14:CT_BooleanFalse/c14:dropZoneFilter", property_name: Some("DropZoneFilter") },
    ChildInfo { name: "c14:CT_BooleanFalse/c14:dropZoneCategories", property_name: Some("DropZoneCategories") },
    ChildInfo { name: "c14:CT_BooleanFalse/c14:dropZoneData", property_name: Some("DropZoneData") },
    ChildInfo { name: "c14:CT_BooleanFalse/c14:dropZoneSeries", property_name: Some("DropZoneSeries") },
    ChildInfo { name: "c14:CT_BooleanFalse/c14:dropZonesVisible", property_name: Some("DropZonesVisible") },
];
static CHILDREN_SKETCH_OPTIONS: &[ChildInfo] = &[
    ChildInfo { name: "c14:CT_BooleanFalse/c14:inSketchMode", property_name: Some("InSketchMode") },
    ChildInfo { name: "c14:CT_BooleanTrue/c14:showSketchBtn", property_name: Some("ShowSketchButton") },
];
static CHILDREN_INVERT_SOLID_FILL_FORMAT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/c14:spPr", property_name: Some("ShapeProperties") },
];
static ATTRS_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "ByteValue" },
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
static ATTRS_DROP_ZONE_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DROP_ZONE_CATEGORIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DROP_ZONE_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DROP_ZONE_SERIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DROP_ZONES_VISIBLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_IN_SKETCH_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_SKETCH_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "PivotOptions", local_name: "pivotOptions", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_OPTIONS },
    ElementInfo { class_name: "SketchOptions", local_name: "sketchOptions", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SKETCH_OPTIONS },
    ElementInfo { class_name: "InvertSolidFillFormat", local_name: "invertSolidFillFmt", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_INVERT_SOLID_FILL_FORMAT },
    ElementInfo { class_name: "Style", local_name: "style", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STYLE, children: &[] },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "DropZoneFilter", local_name: "dropZoneFilter", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DROP_ZONE_FILTER, children: &[] },
    ElementInfo { class_name: "DropZoneCategories", local_name: "dropZoneCategories", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DROP_ZONE_CATEGORIES, children: &[] },
    ElementInfo { class_name: "DropZoneData", local_name: "dropZoneData", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DROP_ZONE_DATA, children: &[] },
    ElementInfo { class_name: "DropZoneSeries", local_name: "dropZoneSeries", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DROP_ZONE_SERIES, children: &[] },
    ElementInfo { class_name: "DropZonesVisible", local_name: "dropZonesVisible", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DROP_ZONES_VISIBLE, children: &[] },
    ElementInfo { class_name: "InSketchMode", local_name: "inSketchMode", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_IN_SKETCH_MODE, children: &[] },
    ElementInfo { class_name: "ShowSketchButton", local_name: "showSketchBtn", prefix: "c14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_SKETCH_BUTTON, children: &[] },
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

/// Create a `<c14:pivotOptions>` element (`PivotOptions`).
pub fn pivot_options(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "pivotOptions").with_children(children)
}

/// Create a `<c14:sketchOptions>` element (`SketchOptions`).
pub fn sketch_options(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "sketchOptions").with_children(children)
}

/// Create a `<c14:invertSolidFillFmt>` element (`InvertSolidFillFormat`).
pub fn invert_solid_fill_format(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "invertSolidFillFmt").with_children(children)
}

/// Create a `<c14:style>` element (`Style`).
pub fn style() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "style")
}

/// Create a `<c14:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<c14:dropZoneFilter>` element (`DropZoneFilter`).
pub fn drop_zone_filter() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "dropZoneFilter")
}

/// Create a `<c14:dropZoneCategories>` element (`DropZoneCategories`).
pub fn drop_zone_categories() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "dropZoneCategories")
}

/// Create a `<c14:dropZoneData>` element (`DropZoneData`).
pub fn drop_zone_data() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "dropZoneData")
}

/// Create a `<c14:dropZoneSeries>` element (`DropZoneSeries`).
pub fn drop_zone_series() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "dropZoneSeries")
}

/// Create a `<c14:dropZonesVisible>` element (`DropZonesVisible`).
pub fn drop_zones_visible() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "dropZonesVisible")
}

/// Create a `<c14:inSketchMode>` element (`InSketchMode`).
pub fn in_sketch_mode() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "inSketchMode")
}

/// Create a `<c14:showSketchBtn>` element (`ShowSketchButton`).
pub fn show_sketch_button() -> OpenXmlElement {
    OpenXmlElement::new("c14", NAMESPACE_URI, "showSketchBtn")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 13;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 12;
