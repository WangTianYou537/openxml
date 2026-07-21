//! Auto-generated from `schemas_microsoft_com_office_drawing_2010_chartDrawing.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2010/chartDrawing` (prefix `cdr14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2010/chartDrawing";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "cdr14";

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

static ATTRS_CONTENT_PART: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("RelationshipId"), type_name: "StringValue" },
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_CONTENT_PART: &[ChildInfo] = &[
    ChildInfo { name: "cdr14:CT_ContentPartNonVisual/cdr14:nvContentPartPr", property_name: Some("NonVisualContentPartProperties") },
    ChildInfo { name: "cdr14:CT_ApplicationNonVisualDrawingProps/cdr14:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_Transform2D/cdr14:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/cdr14:extLst", property_name: Some("OfficeArtExtensionList") },
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
static ATTRS_NON_VISUAL_INK_CONTENT_PART_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isComment", property_name: Some("IsComment"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_INK_CONTENT_PART_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_ContentPartLocking/a14:cpLocks", property_name: Some("ContentPartLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a14:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_NON_VISUAL_CONTENT_PART_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/cdr14:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a14:CT_NonVisualInkContentPartProperties/cdr14:cNvContentPartPr", property_name: Some("NonVisualInkContentPartProperties") },
];
static ATTRS_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":macro", property_name: Some("Macro"), type_name: "StringValue" },
    AttributeInfo { qname: ":fPublished", property_name: Some("Published"), type_name: "BooleanValue" },
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ContentPart", local_name: "contentPart", prefix: "cdr14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTENT_PART, children: CHILDREN_CONTENT_PART },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "cdr14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualInkContentPartProperties", local_name: "cNvContentPartPr", prefix: "cdr14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_INK_CONTENT_PART_PROPERTIES, children: CHILDREN_NON_VISUAL_INK_CONTENT_PART_PROPERTIES },
    ElementInfo { class_name: "NonVisualContentPartProperties", local_name: "nvContentPartPr", prefix: "cdr14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONTENT_PART_PROPERTIES },
    ElementInfo { class_name: "ApplicationNonVisualDrawingProperties", local_name: "nvPr", prefix: "cdr14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES, children: &[] },
    ElementInfo { class_name: "Transform2D", local_name: "xfrm", prefix: "cdr14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "cdr14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
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

/// Create a `<cdr14:contentPart>` element (`ContentPart`).
pub fn content_part(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr14", NAMESPACE_URI, "contentPart").with_children(children)
}

/// Create a `<cdr14:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr14", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<cdr14:cNvContentPartPr>` element (`NonVisualInkContentPartProperties`).
pub fn non_visual_ink_content_part_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr14", NAMESPACE_URI, "cNvContentPartPr").with_children(children)
}

/// Create a `<cdr14:nvContentPartPr>` element (`NonVisualContentPartProperties`).
pub fn non_visual_content_part_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr14", NAMESPACE_URI, "nvContentPartPr").with_children(children)
}

/// Create a `<cdr14:nvPr>` element (`ApplicationNonVisualDrawingProperties`).
pub fn application_non_visual_drawing_properties() -> OpenXmlElement {
    OpenXmlElement::new("cdr14", NAMESPACE_URI, "nvPr")
}

/// Create a `<cdr14:xfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr14", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Create a `<cdr14:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("cdr14", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 7;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 7;
