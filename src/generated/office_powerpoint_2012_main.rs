//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2012_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2012/main` (prefix `p15`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2012/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p15";

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

static ATTRS_PRESET_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "StringValue" },
    AttributeInfo { qname: ":invX", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":invY", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_PRESENCE_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":userId", property_name: Some("UserId"), type_name: "StringValue" },
    AttributeInfo { qname: ":providerId", property_name: Some("ProviderId"), type_name: "StringValue" },
];
static ATTRS_THREADING_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":timeZoneBias", property_name: Some("TimeZoneBias"), type_name: "Int32Value" },
];
static CHILDREN_THREADING_INFO: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_ParentCommentIdentifier/p15:parentCm", property_name: Some("ParentCommentIdentifier") },
];
static CHILDREN_SLIDE_GUIDE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_ExtendedGuide/p15:guide", property_name: None },
    ChildInfo { name: "p:CT_ExtensionList/p15:extLst", property_name: None },
];
static CHILDREN_NOTES_GUIDE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_ExtendedGuide/p15:guide", property_name: None },
    ChildInfo { name: "p:CT_ExtensionList/p15:extLst", property_name: None },
];
static ATTRS_CHART_TRACKING_REFERENCE_BASED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_PARENT_COMMENT_IDENTIFIER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":authorId", property_name: Some("AuthorId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_COLOR_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static ATTRS_EXTENDED_GUIDE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":orient", property_name: Some("Orientation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":pos", property_name: Some("Position"), type_name: "Int32Value" },
    AttributeInfo { qname: ":userDrawn", property_name: Some("IsUserDrawn"), type_name: "BooleanValue" },
];
static CHILDREN_EXTENDED_GUIDE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color/p15:clr", property_name: Some("ColorType") },
    ChildInfo { name: "p:CT_ExtensionList/p15:extLst", property_name: Some("ExtensionList") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "PresetTransition", local_name: "prstTrans", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRESET_TRANSITION, children: &[] },
    ElementInfo { class_name: "PresenceInfo", local_name: "presenceInfo", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRESENCE_INFO, children: &[] },
    ElementInfo { class_name: "ThreadingInfo", local_name: "threadingInfo", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_THREADING_INFO, children: CHILDREN_THREADING_INFO },
    ElementInfo { class_name: "SlideGuideList", local_name: "sldGuideLst", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_GUIDE_LIST },
    ElementInfo { class_name: "NotesGuideList", local_name: "notesGuideLst", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NOTES_GUIDE_LIST },
    ElementInfo { class_name: "ChartTrackingReferenceBased", local_name: "chartTrackingRefBased", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHART_TRACKING_REFERENCE_BASED, children: &[] },
    ElementInfo { class_name: "ParentCommentIdentifier", local_name: "parentCm", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PARENT_COMMENT_IDENTIFIER, children: &[] },
    ElementInfo { class_name: "ColorType", local_name: "clr", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_TYPE },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "ExtendedGuide", local_name: "guide", prefix: "p15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTENDED_GUIDE, children: CHILDREN_EXTENDED_GUIDE },
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

/// Create a `<p15:prstTrans>` element (`PresetTransition`).
pub fn preset_transition() -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "prstTrans")
}

/// Create a `<p15:presenceInfo>` element (`PresenceInfo`).
pub fn presence_info() -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "presenceInfo")
}

/// Create a `<p15:threadingInfo>` element (`ThreadingInfo`).
pub fn threading_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "threadingInfo").with_children(children)
}

/// Create a `<p15:sldGuideLst>` element (`SlideGuideList`).
pub fn slide_guide_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "sldGuideLst").with_children(children)
}

/// Create a `<p15:notesGuideLst>` element (`NotesGuideList`).
pub fn notes_guide_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "notesGuideLst").with_children(children)
}

/// Create a `<p15:chartTrackingRefBased>` element (`ChartTrackingReferenceBased`).
pub fn chart_tracking_reference_based() -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "chartTrackingRefBased")
}

/// Create a `<p15:parentCm>` element (`ParentCommentIdentifier`).
pub fn parent_comment_identifier() -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "parentCm")
}

/// Create a `<p15:clr>` element (`ColorType`).
pub fn color_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "clr").with_children(children)
}

/// Create a `<p15:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p15:guide>` element (`ExtendedGuide`).
pub fn extended_guide(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p15", NAMESPACE_URI, "guide").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 11;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 10;
