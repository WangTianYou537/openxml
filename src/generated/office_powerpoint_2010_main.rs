//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2010_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2010/main` (prefix `p14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2010/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p14";

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

static CHILDREN_NON_VISUAL_CONTENT_PART_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/p14:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a14:CT_NonVisualInkContentPartProperties/p14:cNvContentPartPr", property_name: Some("NonVisualInkContentPartProperties") },
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingProps/p14:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
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
static ATTRS_EXTENSION_LIST_MODIFY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mod", property_name: Some("Modify"), type_name: "BooleanValue" },
];
static CHILDREN_EXTENSION_LIST_MODIFY: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static ATTRS_MEDIA: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
];
static CHILDREN_MEDIA: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_MediaTrim/p14:trim", property_name: Some("MediaTrim") },
    ChildInfo { name: "p14:CT_MediaFade/p14:fade", property_name: Some("MediaFade") },
    ChildInfo { name: "p14:CT_MediaBookmarkList/p14:bmkLst", property_name: Some("MediaBookmarkList") },
    ChildInfo { name: "p:CT_ExtensionList/p14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_VORTEX_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_PAN_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_SWITCH_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_FLIP_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_FERRIS_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_GALLERY_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_CONVEYOR_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_RIPPLE_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "StringValue" },
];
static ATTRS_PRISM_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
    AttributeInfo { qname: ":isContent", property_name: Some("IsContent"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":isInverted", property_name: Some("IsInverted"), type_name: "BooleanValue" },
];
static ATTRS_DOORS_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_WINDOW_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_GLITTER_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
    AttributeInfo { qname: ":pattern", property_name: Some("Pattern"), type_name: "EnumValue" },
];
static ATTRS_WARP_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_FLYTHROUGH_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hasBounce", property_name: Some("HasBounce"), type_name: "BooleanValue" },
];
static ATTRS_SHRED_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pattern", property_name: Some("Pattern"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_REVEAL_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":thruBlk", property_name: Some("ThroughBlack"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_WHEEL_REVERSE_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spokes", property_name: Some("Spokes"), type_name: "UInt32Value" },
];
static ATTRS_BOOKMARK_TARGET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":bmkName", property_name: Some("BookmarkName"), type_name: "StringValue" },
];
static CHILDREN_SECTION_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_SectionOld/p14:section", property_name: None },
];
static CHILDREN_SECTION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_Section/p14:section", property_name: None },
];
static ATTRS_BROWSE_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showStatus", property_name: Some("ShowStatus"), type_name: "BooleanValue" },
];
static CHILDREN_LASER_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_DEFAULT_IMAGE_DPI: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_DISCARD_IMAGE_EDIT_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_SHOW_MEDIA_CONTROLS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static CHILDREN_LASER_TRACE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_LaserTrace/p14:tracePtLst", property_name: None },
];
static ATTRS_CREATION_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_MODIFICATION_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static CHILDREN_SHOW_EVENT_RECORD_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_TriggerEventRecord/p14:triggerEvt", property_name: Some("TriggerEventRecord") },
    ChildInfo { name: "p14:CT_MediaPlaybackEventRecord/p14:playEvt", property_name: Some("PlayEventRecord") },
    ChildInfo { name: "p14:CT_MediaPlaybackEventRecord/p14:stopEvt", property_name: Some("StopEventRecord") },
    ChildInfo { name: "p14:CT_MediaPlaybackEventRecord/p14:pauseEvt", property_name: Some("PauseEventRecord") },
    ChildInfo { name: "p14:CT_MediaPlaybackEventRecord/p14:resumeEvt", property_name: Some("ResumeEventRecord") },
    ChildInfo { name: "p14:CT_MediaSeekEventRecord/p14:seekEvt", property_name: Some("SeekEventRecord") },
    ChildInfo { name: "p14:CT_NullEventRecord/p14:nullEvt", property_name: Some("NullEventRecord") },
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
static ATTRS_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isPhoto", property_name: Some("IsPhoto"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":userDrawn", property_name: Some("UserDrawn"), type_name: "BooleanValue" },
];
static CHILDREN_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Placeholder/p:ph", property_name: Some("PlaceholderShape") },
    ChildInfo { name: "a:CT_AudioCD/a:audioCd", property_name: None },
    ChildInfo { name: "a:CT_EmbeddedWAVAudioFile/a:wavAudioFile", property_name: None },
    ChildInfo { name: "a:CT_AudioFile/a:audioFile", property_name: None },
    ChildInfo { name: "a:CT_VideoFile/a:videoFile", property_name: None },
    ChildInfo { name: "a:CT_QuickTimeFile/a:quickTimeFile", property_name: None },
    ChildInfo { name: "p:CT_CustomerDataList/p:custDataLst", property_name: None },
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingPropsExtensionList/p:extLst", property_name: None },
];
static ATTRS_MEDIA_BOOKMARK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
];
static ATTRS_MEDIA_TRIM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":st", property_name: Some("Start"), type_name: "StringValue" },
    AttributeInfo { qname: ":end", property_name: Some("End"), type_name: "StringValue" },
];
static ATTRS_MEDIA_FADE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":in", property_name: Some("InDuration"), type_name: "StringValue" },
    AttributeInfo { qname: ":out", property_name: Some("OutDuration"), type_name: "StringValue" },
];
static CHILDREN_MEDIA_BOOKMARK_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_MediaBookmark/p14:bmk", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static ATTRS_SECTION_OLD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":slideIdLst", property_name: Some("SlideIdList"), type_name: "ListValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_SECTION_OLD: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SECTION_SLIDE_ID_LIST_ENTRY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
];
static CHILDREN_SECTION_SLIDE_ID_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_SectionSlideIdListEntry/p14:sldId", property_name: None },
];
static ATTRS_SECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_SECTION: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_SectionSlideIdList/p14:sldIdLst", property_name: Some("SectionSlideIdList") },
    ChildInfo { name: "p:CT_ExtensionList/p14:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TRACE_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":t", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":x", property_name: Some("XCoordinate"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("YCoordinate"), type_name: "Int64Value" },
];
static CHILDREN_TRACE_POINT_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_LaserTracePoint/p14:tracePt", property_name: None },
];
static ATTRS_TRIGGER_EVENT_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":objId", property_name: Some("ObjectId"), type_name: "UInt32Value" },
];
static ATTRS_PLAY_EVENT_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":objId", property_name: Some("ObjectId"), type_name: "UInt32Value" },
];
static ATTRS_STOP_EVENT_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":objId", property_name: Some("ObjectId"), type_name: "UInt32Value" },
];
static ATTRS_PAUSE_EVENT_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":objId", property_name: Some("ObjectId"), type_name: "UInt32Value" },
];
static ATTRS_RESUME_EVENT_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":objId", property_name: Some("ObjectId"), type_name: "UInt32Value" },
];
static ATTRS_SEEK_EVENT_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":objId", property_name: Some("ObjectId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":seek", property_name: Some("Seek"), type_name: "StringValue" },
];
static ATTRS_NULL_EVENT_RECORD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":objId", property_name: Some("ObjectId"), type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "NonVisualContentPartProperties", local_name: "nvContentPartPr", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONTENT_PART_PROPERTIES },
    ElementInfo { class_name: "Transform2D", local_name: "xfrm", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
    ElementInfo { class_name: "ExtensionListModify", local_name: "extLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTENSION_LIST_MODIFY, children: CHILDREN_EXTENSION_LIST_MODIFY },
    ElementInfo { class_name: "Media", local_name: "media", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MEDIA, children: CHILDREN_MEDIA },
    ElementInfo { class_name: "VortexTransition", local_name: "vortex", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VORTEX_TRANSITION, children: &[] },
    ElementInfo { class_name: "PanTransition", local_name: "pan", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PAN_TRANSITION, children: &[] },
    ElementInfo { class_name: "SwitchTransition", local_name: "switch", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SWITCH_TRANSITION, children: &[] },
    ElementInfo { class_name: "FlipTransition", local_name: "flip", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FLIP_TRANSITION, children: &[] },
    ElementInfo { class_name: "FerrisTransition", local_name: "ferris", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FERRIS_TRANSITION, children: &[] },
    ElementInfo { class_name: "GalleryTransition", local_name: "gallery", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GALLERY_TRANSITION, children: &[] },
    ElementInfo { class_name: "ConveyorTransition", local_name: "conveyor", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONVEYOR_TRANSITION, children: &[] },
    ElementInfo { class_name: "RippleTransition", local_name: "ripple", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RIPPLE_TRANSITION, children: &[] },
    ElementInfo { class_name: "HoneycombTransition", local_name: "honeycomb", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "FlashTransition", local_name: "flash", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "PrismTransition", local_name: "prism", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRISM_TRANSITION, children: &[] },
    ElementInfo { class_name: "DoorsTransition", local_name: "doors", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DOORS_TRANSITION, children: &[] },
    ElementInfo { class_name: "WindowTransition", local_name: "window", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WINDOW_TRANSITION, children: &[] },
    ElementInfo { class_name: "GlitterTransition", local_name: "glitter", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GLITTER_TRANSITION, children: &[] },
    ElementInfo { class_name: "WarpTransition", local_name: "warp", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WARP_TRANSITION, children: &[] },
    ElementInfo { class_name: "FlythroughTransition", local_name: "flythrough", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FLYTHROUGH_TRANSITION, children: &[] },
    ElementInfo { class_name: "ShredTransition", local_name: "shred", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHRED_TRANSITION, children: &[] },
    ElementInfo { class_name: "RevealTransition", local_name: "reveal", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REVEAL_TRANSITION, children: &[] },
    ElementInfo { class_name: "WheelReverseTransition", local_name: "wheelReverse", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WHEEL_REVERSE_TRANSITION, children: &[] },
    ElementInfo { class_name: "BookmarkTarget", local_name: "bmkTgt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOOKMARK_TARGET, children: &[] },
    ElementInfo { class_name: "SectionProperties", local_name: "sectionPr", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SECTION_PROPERTIES },
    ElementInfo { class_name: "SectionList", local_name: "sectionLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SECTION_LIST },
    ElementInfo { class_name: "BrowseMode", local_name: "browseMode", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BROWSE_MODE, children: &[] },
    ElementInfo { class_name: "LaserColor", local_name: "laserClr", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LASER_COLOR },
    ElementInfo { class_name: "DefaultImageDpi", local_name: "defaultImageDpi", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DEFAULT_IMAGE_DPI, children: &[] },
    ElementInfo { class_name: "DiscardImageEditData", local_name: "discardImageEditData", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DISCARD_IMAGE_EDIT_DATA, children: &[] },
    ElementInfo { class_name: "ShowMediaControls", local_name: "showMediaCtrls", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_MEDIA_CONTROLS, children: &[] },
    ElementInfo { class_name: "LaserTraceList", local_name: "laserTraceLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LASER_TRACE_LIST },
    ElementInfo { class_name: "CreationId", local_name: "creationId", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CREATION_ID, children: &[] },
    ElementInfo { class_name: "ModificationId", local_name: "modId", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFICATION_ID, children: &[] },
    ElementInfo { class_name: "ShowEventRecordList", local_name: "showEvtLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHOW_EVENT_RECORD_LIST },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualInkContentPartProperties", local_name: "cNvContentPartPr", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_INK_CONTENT_PART_PROPERTIES, children: CHILDREN_NON_VISUAL_INK_CONTENT_PART_PROPERTIES },
    ElementInfo { class_name: "ApplicationNonVisualDrawingProperties", local_name: "nvPr", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "MediaBookmark", local_name: "bmk", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MEDIA_BOOKMARK, children: &[] },
    ElementInfo { class_name: "MediaTrim", local_name: "trim", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MEDIA_TRIM, children: &[] },
    ElementInfo { class_name: "MediaFade", local_name: "fade", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MEDIA_FADE, children: &[] },
    ElementInfo { class_name: "MediaBookmarkList", local_name: "bmkLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MEDIA_BOOKMARK_LIST },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "SectionOld", local_name: "section", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SECTION_OLD, children: CHILDREN_SECTION_OLD },
    ElementInfo { class_name: "SectionSlideIdListEntry", local_name: "sldId", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SECTION_SLIDE_ID_LIST_ENTRY, children: &[] },
    ElementInfo { class_name: "SectionSlideIdList", local_name: "sldIdLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SECTION_SLIDE_ID_LIST },
    ElementInfo { class_name: "Section", local_name: "section", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SECTION, children: CHILDREN_SECTION },
    ElementInfo { class_name: "TracePoint", local_name: "tracePt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TRACE_POINT, children: &[] },
    ElementInfo { class_name: "TracePointList", local_name: "tracePtLst", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TRACE_POINT_LIST },
    ElementInfo { class_name: "TriggerEventRecord", local_name: "triggerEvt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TRIGGER_EVENT_RECORD, children: &[] },
    ElementInfo { class_name: "PlayEventRecord", local_name: "playEvt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PLAY_EVENT_RECORD, children: &[] },
    ElementInfo { class_name: "StopEventRecord", local_name: "stopEvt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STOP_EVENT_RECORD, children: &[] },
    ElementInfo { class_name: "PauseEventRecord", local_name: "pauseEvt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PAUSE_EVENT_RECORD, children: &[] },
    ElementInfo { class_name: "ResumeEventRecord", local_name: "resumeEvt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RESUME_EVENT_RECORD, children: &[] },
    ElementInfo { class_name: "SeekEventRecord", local_name: "seekEvt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SEEK_EVENT_RECORD, children: &[] },
    ElementInfo { class_name: "NullEventRecord", local_name: "nullEvt", prefix: "p14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NULL_EVENT_RECORD, children: &[] },
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

/// Create a `<p14:nvContentPartPr>` element (`NonVisualContentPartProperties`).
pub fn non_visual_content_part_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "nvContentPartPr").with_children(children)
}

/// Create a `<p14:xfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Create a `<p14:extLst>` element (`ExtensionListModify`).
pub fn extension_list_modify(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p14:media>` element (`Media`).
pub fn media(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "media").with_children(children)
}

/// Create a `<p14:vortex>` element (`VortexTransition`).
pub fn vortex_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "vortex")
}

/// Create a `<p14:pan>` element (`PanTransition`).
pub fn pan_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "pan")
}

/// Create a `<p14:switch>` element (`SwitchTransition`).
pub fn switch_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "switch")
}

/// Create a `<p14:flip>` element (`FlipTransition`).
pub fn flip_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "flip")
}

/// Create a `<p14:ferris>` element (`FerrisTransition`).
pub fn ferris_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "ferris")
}

/// Create a `<p14:gallery>` element (`GalleryTransition`).
pub fn gallery_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "gallery")
}

/// Create a `<p14:conveyor>` element (`ConveyorTransition`).
pub fn conveyor_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "conveyor")
}

/// Create a `<p14:ripple>` element (`RippleTransition`).
pub fn ripple_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "ripple")
}

/// Create a `<p14:honeycomb>` element (`HoneycombTransition`).
pub fn honeycomb_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "honeycomb")
}

/// Create a `<p14:flash>` element (`FlashTransition`).
pub fn flash_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "flash")
}

/// Create a `<p14:prism>` element (`PrismTransition`).
pub fn prism_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "prism")
}

/// Create a `<p14:doors>` element (`DoorsTransition`).
pub fn doors_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "doors")
}

/// Create a `<p14:window>` element (`WindowTransition`).
pub fn window_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "window")
}

/// Create a `<p14:glitter>` element (`GlitterTransition`).
pub fn glitter_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "glitter")
}

/// Create a `<p14:warp>` element (`WarpTransition`).
pub fn warp_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "warp")
}

/// Create a `<p14:flythrough>` element (`FlythroughTransition`).
pub fn flythrough_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "flythrough")
}

/// Create a `<p14:shred>` element (`ShredTransition`).
pub fn shred_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "shred")
}

/// Create a `<p14:reveal>` element (`RevealTransition`).
pub fn reveal_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "reveal")
}

/// Create a `<p14:wheelReverse>` element (`WheelReverseTransition`).
pub fn wheel_reverse_transition() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "wheelReverse")
}

/// Create a `<p14:bmkTgt>` element (`BookmarkTarget`).
pub fn bookmark_target() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "bmkTgt")
}

/// Create a `<p14:sectionPr>` element (`SectionProperties`).
pub fn section_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "sectionPr").with_children(children)
}

/// Create a `<p14:sectionLst>` element (`SectionList`).
pub fn section_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "sectionLst").with_children(children)
}

/// Create a `<p14:browseMode>` element (`BrowseMode`).
pub fn browse_mode() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "browseMode")
}

/// Create a `<p14:laserClr>` element (`LaserColor`).
pub fn laser_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "laserClr").with_children(children)
}

/// Create a `<p14:defaultImageDpi>` element (`DefaultImageDpi`).
pub fn default_image_dpi() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "defaultImageDpi")
}

/// Create a `<p14:discardImageEditData>` element (`DiscardImageEditData`).
pub fn discard_image_edit_data() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "discardImageEditData")
}

/// Create a `<p14:showMediaCtrls>` element (`ShowMediaControls`).
pub fn show_media_controls() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "showMediaCtrls")
}

/// Create a `<p14:laserTraceLst>` element (`LaserTraceList`).
pub fn laser_trace_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "laserTraceLst").with_children(children)
}

/// Create a `<p14:creationId>` element (`CreationId`).
pub fn creation_id() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "creationId")
}

/// Create a `<p14:modId>` element (`ModificationId`).
pub fn modification_id() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "modId")
}

/// Create a `<p14:showEvtLst>` element (`ShowEventRecordList`).
pub fn show_event_record_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "showEvtLst").with_children(children)
}

/// Create a `<p14:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<p14:cNvContentPartPr>` element (`NonVisualInkContentPartProperties`).
pub fn non_visual_ink_content_part_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "cNvContentPartPr").with_children(children)
}

/// Create a `<p14:nvPr>` element (`ApplicationNonVisualDrawingProperties`).
pub fn application_non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "nvPr").with_children(children)
}

/// Create a `<p14:bmk>` element (`MediaBookmark`).
pub fn media_bookmark() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "bmk")
}

/// Create a `<p14:trim>` element (`MediaTrim`).
pub fn media_trim() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "trim")
}

/// Create a `<p14:fade>` element (`MediaFade`).
pub fn media_fade() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "fade")
}

/// Create a `<p14:bmkLst>` element (`MediaBookmarkList`).
pub fn media_bookmark_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "bmkLst").with_children(children)
}

/// Create a `<p14:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p14:section>` element (`SectionOld`).
pub fn section_old(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "section").with_children(children)
}

/// Create a `<p14:sldId>` element (`SectionSlideIdListEntry`).
pub fn section_slide_id_list_entry() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "sldId")
}

/// Create a `<p14:sldIdLst>` element (`SectionSlideIdList`).
pub fn section_slide_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "sldIdLst").with_children(children)
}

/// Create a `<p14:section>` element (`Section`).
pub fn section(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "section").with_children(children)
}

/// Create a `<p14:tracePt>` element (`TracePoint`).
pub fn trace_point() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "tracePt")
}

/// Create a `<p14:tracePtLst>` element (`TracePointList`).
pub fn trace_point_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "tracePtLst").with_children(children)
}

/// Create a `<p14:triggerEvt>` element (`TriggerEventRecord`).
pub fn trigger_event_record() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "triggerEvt")
}

/// Create a `<p14:playEvt>` element (`PlayEventRecord`).
pub fn play_event_record() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "playEvt")
}

/// Create a `<p14:stopEvt>` element (`StopEventRecord`).
pub fn stop_event_record() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "stopEvt")
}

/// Create a `<p14:pauseEvt>` element (`PauseEventRecord`).
pub fn pause_event_record() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "pauseEvt")
}

/// Create a `<p14:resumeEvt>` element (`ResumeEventRecord`).
pub fn resume_event_record() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "resumeEvt")
}

/// Create a `<p14:seekEvt>` element (`SeekEventRecord`).
pub fn seek_event_record() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "seekEvt")
}

/// Create a `<p14:nullEvt>` element (`NullEventRecord`).
pub fn null_event_record() -> OpenXmlElement {
    OpenXmlElement::new("p14", NAMESPACE_URI, "nullEvt")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 62;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 56;
