//! Auto-generated from `schemas_openxmlformats_org_presentationml_2006_main.json`.
//! Target namespace: `http://schemas.openxmlformats.org/presentationml/2006/main` (prefix `p`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "p";

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

static ATTRS_SLIDE_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":st", property_name: Some("Start"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":end", property_name: Some("End"), type_name: "UInt32Value" },
];
static ATTRS_CHAR_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":st", property_name: Some("Start"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":end", property_name: Some("End"), type_name: "UInt32Value" },
];
static ATTRS_PARAGRAPH_INDEX_RANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":st", property_name: Some("Start"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":end", property_name: Some("End"), type_name: "UInt32Value" },
];
static ATTRS_CUSTOM_SHOW_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
];
static ATTRS_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: Some("Uri"), type_name: "StringValue" },
];
static ATTRS_BROWSE_SLIDE_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showScrollbar", property_name: Some("ShowScrollbar"), type_name: "BooleanValue" },
];
static ATTRS_KIOSK_SLIDE_MODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":restart", property_name: Some("Restart"), type_name: "UInt32Value" },
];
static ATTRS_COLOR_MAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bg1", property_name: Some("Background1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx1", property_name: Some("Text1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bg2", property_name: Some("Background2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx2", property_name: Some("Text2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent1", property_name: Some("Accent1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent2", property_name: Some("Accent2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent3", property_name: Some("Accent3"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent4", property_name: Some("Accent4"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent5", property_name: Some("Accent5"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent6", property_name: Some("Accent6"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hlink", property_name: Some("Hyperlink"), type_name: "EnumValue" },
    AttributeInfo { qname: ":folHlink", property_name: Some("FollowedHyperlink"), type_name: "EnumValue" },
];
static CHILDREN_COLOR_MAP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_COLOR_MAP_OVERRIDE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EmptyElement/a:masterClrMapping", property_name: Some("MasterColorMapping") },
    ChildInfo { name: "a:CT_ColorMapping/a:overrideClrMapping", property_name: Some("OverrideColorMapping") },
];
static ATTRS_BACKGROUND_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":shadeToTitle", property_name: Some("ShadeToTitle"), type_name: "BooleanValue" },
];
static CHILDREN_BACKGROUND_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: None },
];
static ATTRS_BACKGROUND_STYLE_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_BACKGROUND_STYLE_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_COMMENT_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p228:CT_TaskDetails/p228:taskDetails", property_name: Some("TaskDetails") },
    ChildInfo { name: "p223:CT_Reactions/p223:reactions", property_name: Some("Reactions") },
];
static CHILDREN_COMMENT_AUTHOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommentAuthor/p:cmAuthor", property_name: None },
];
static CHILDREN_COMMENT_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Comment/p:cm", property_name: None },
];
static ATTRS_OLE_OBJECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":showAsIcon", property_name: Some("ShowAsIcon"), type_name: "BooleanValue" },
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":imgW", property_name: Some("ImageWidth"), type_name: "Int32Value" },
    AttributeInfo { qname: ":imgH", property_name: Some("ImageHeight"), type_name: "Int32Value" },
    AttributeInfo { qname: ":progId", property_name: Some("ProgId"), type_name: "StringValue" },
];
static CHILDREN_OLE_OBJECT: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_OleObjectEmbed/p:embed", property_name: None },
    ChildInfo { name: "p:CT_OleObjectLink/p:link", property_name: None },
    ChildInfo { name: "p:CT_Picture/p:pic", property_name: None },
];
static ATTRS_PRESENTATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":serverZoom", property_name: Some("ServerZoom"), type_name: "Int32Value" },
    AttributeInfo { qname: ":firstSlideNum", property_name: Some("FirstSlideNum"), type_name: "Int32Value" },
    AttributeInfo { qname: ":showSpecialPlsOnTitleSld", property_name: Some("ShowSpecialPlaceholderOnTitleSlide"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":removePersonalInfoOnSave", property_name: Some("RemovePersonalInfoOnSave"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":compatMode", property_name: Some("CompatibilityMode"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":strictFirstAndLastChars", property_name: Some("StrictFirstAndLastChars"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":embedTrueTypeFonts", property_name: Some("EmbedTrueTypeFonts"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":saveSubsetFonts", property_name: Some("SaveSubsetFonts"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":autoCompressPictures", property_name: Some("AutoCompressPictures"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":bookmarkIdSeed", property_name: Some("BookmarkIdSeed"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":conformance", property_name: Some("Conformance"), type_name: "EnumValue" },
];
static CHILDREN_PRESENTATION: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideMasterIdList/p:sldMasterIdLst", property_name: Some("SlideMasterIdList") },
    ChildInfo { name: "p:CT_NotesMasterIdList/p:notesMasterIdLst", property_name: Some("NotesMasterIdList") },
    ChildInfo { name: "p:CT_HandoutMasterIdList/p:handoutMasterIdLst", property_name: Some("HandoutMasterIdList") },
    ChildInfo { name: "p:CT_SlideIdList/p:sldIdLst", property_name: Some("SlideIdList") },
    ChildInfo { name: "p:CT_SlideSize/p:sldSz", property_name: Some("SlideSize") },
    ChildInfo { name: "a:CT_PositiveSize2D/p:notesSz", property_name: Some("NotesSize") },
    ChildInfo { name: "p:CT_EmbeddedFontList/p:embeddedFontLst", property_name: Some("EmbeddedFontList") },
    ChildInfo { name: "p:CT_CustomShowList/p:custShowLst", property_name: Some("CustomShowList") },
    ChildInfo { name: "p:CT_PhotoAlbum/p:photoAlbum", property_name: Some("PhotoAlbum") },
    ChildInfo { name: "p:CT_CustomerDataList/p:custDataLst", property_name: Some("CustomerDataList") },
    ChildInfo { name: "p:CT_Kinsoku/p:kinsoku", property_name: Some("Kinsoku") },
    ChildInfo { name: "a:CT_TextListStyle/p:defaultTextStyle", property_name: Some("DefaultTextStyle") },
    ChildInfo { name: "p:CT_ModifyVerifier/p:modifyVerifier", property_name: Some("ModificationVerifier") },
    ChildInfo { name: "p:CT_PresentationExtensionList/p:extLst", property_name: Some("PresentationExtensionList") },
];
static CHILDREN_PRESENTATION_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_HtmlPublishProperties/p:htmlPubPr", property_name: Some("HtmlPublishProperties") },
    ChildInfo { name: "p:CT_WebProperties/p:webPr", property_name: Some("WebProperties") },
    ChildInfo { name: "p:CT_PrintProperties/p:prnPr", property_name: Some("PrintingProperties") },
    ChildInfo { name: "p:CT_ShowProperties/p:showPr", property_name: Some("ShowProperties") },
    ChildInfo { name: "a:CT_ColorMRU/p:clrMru", property_name: Some("ColorMostRecentlyUsed") },
    ChildInfo { name: "p:CT_PresentationPropertiesExtensionList/p:extLst", property_name: Some("PresentationPropertiesExtensionList") },
];
static ATTRS_SLIDE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showMasterSp", property_name: Some("ShowMasterShapes"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showMasterPhAnim", property_name: Some("ShowMasterPlaceholderAnimations"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":show", property_name: Some("Show"), type_name: "BooleanValue" },
];
static CHILDREN_SLIDE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideData/p:cSld", property_name: Some("CommonSlideData") },
    ChildInfo { name: "a:CT_ColorMappingOverride/p:clrMapOvr", property_name: Some("ColorMapOverride") },
    ChildInfo { name: "p:CT_SlideTransition/p:transition", property_name: Some("Transition") },
    ChildInfo { name: "p:CT_SlideTiming/p:timing", property_name: Some("Timing") },
    ChildInfo { name: "p:CT_SlideExtensionList/p:extLst", property_name: Some("SlideExtensionList") },
];
static ATTRS_SLIDE_LAYOUT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showMasterSp", property_name: Some("ShowMasterShapes"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showMasterPhAnim", property_name: Some("ShowMasterPlaceholderAnimations"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":matchingName", property_name: Some("MatchingName"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":preserve", property_name: Some("Preserve"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":userDrawn", property_name: Some("UserDrawn"), type_name: "BooleanValue" },
];
static CHILDREN_SLIDE_LAYOUT: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideData/p:cSld", property_name: Some("CommonSlideData") },
    ChildInfo { name: "a:CT_ColorMappingOverride/p:clrMapOvr", property_name: Some("ColorMapOverride") },
    ChildInfo { name: "p:CT_SlideTransition/p:transition", property_name: Some("Transition") },
    ChildInfo { name: "p:CT_SlideTiming/p:timing", property_name: Some("Timing") },
    ChildInfo { name: "p:CT_HeaderFooter/p:hf", property_name: Some("HeaderFooter") },
    ChildInfo { name: "p:CT_SlideLayoutExtensionList/p:extLst", property_name: Some("SlideLayoutExtensionList") },
];
static ATTRS_SLIDE_MASTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":preserve", property_name: Some("Preserve"), type_name: "BooleanValue" },
];
static CHILDREN_SLIDE_MASTER: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideData/p:cSld", property_name: Some("CommonSlideData") },
    ChildInfo { name: "a:CT_ColorMapping/p:clrMap", property_name: Some("ColorMap") },
    ChildInfo { name: "p:CT_SlideLayoutIdList/p:sldLayoutIdLst", property_name: Some("SlideLayoutIdList") },
    ChildInfo { name: "p:CT_SlideTransition/p:transition", property_name: Some("Transition") },
    ChildInfo { name: "p:CT_SlideTiming/p:timing", property_name: Some("Timing") },
    ChildInfo { name: "p:CT_HeaderFooter/p:hf", property_name: Some("HeaderFooter") },
    ChildInfo { name: "p:CT_SlideMasterTextStyles/p:txStyles", property_name: Some("TextStyles") },
    ChildInfo { name: "p:CT_SlideMasterExtensionList/p:extLst", property_name: Some("SlideMasterExtensionList") },
];
static CHILDREN_HANDOUT_MASTER: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideData/p:cSld", property_name: Some("CommonSlideData") },
    ChildInfo { name: "a:CT_ColorMapping/p:clrMap", property_name: Some("ColorMap") },
    ChildInfo { name: "p:CT_HeaderFooter/p:hf", property_name: Some("HeaderFooter") },
    ChildInfo { name: "p:CT_HandoutMasterExtensionList/p:extLst", property_name: Some("HandoutMasterExtensionList") },
];
static CHILDREN_NOTES_MASTER: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideData/p:cSld", property_name: Some("CommonSlideData") },
    ChildInfo { name: "a:CT_ColorMapping/p:clrMap", property_name: Some("ColorMap") },
    ChildInfo { name: "p:CT_HeaderFooter/p:hf", property_name: Some("HeaderFooter") },
    ChildInfo { name: "a:CT_TextListStyle/p:notesStyle", property_name: Some("NotesStyle") },
    ChildInfo { name: "p:CT_NotesMasterExtensionList/p:extLst", property_name: Some("NotesMasterExtensionList") },
];
static ATTRS_NOTES_SLIDE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showMasterSp", property_name: Some("ShowMasterShapes"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showMasterPhAnim", property_name: Some("ShowMasterPlaceholderAnimations"), type_name: "BooleanValue" },
];
static CHILDREN_NOTES_SLIDE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideData/p:cSld", property_name: Some("CommonSlideData") },
    ChildInfo { name: "a:CT_ColorMappingOverride/p:clrMapOvr", property_name: Some("ColorMapOverride") },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static ATTRS_SLIDE_SYNC_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":serverSldId", property_name: Some("ServerSlideId"), type_name: "StringValue" },
    AttributeInfo { qname: ":serverSldModifiedTime", property_name: Some("ServerSlideModifiedTime"), type_name: "DateTimeValue" },
    AttributeInfo { qname: ":clientInsertedTime", property_name: Some("ClientInsertedTime"), type_name: "DateTimeValue" },
];
static CHILDREN_SLIDE_SYNC_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_TAG_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_StringTag/p:tag", property_name: None },
];
static ATTRS_VIEW_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lastView", property_name: Some("LastView"), type_name: "EnumValue" },
    AttributeInfo { qname: ":showComments", property_name: Some("ShowComments"), type_name: "BooleanValue" },
];
static CHILDREN_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_NormalViewProperties/p:normalViewPr", property_name: Some("NormalViewProperties") },
    ChildInfo { name: "p:CT_SlideViewProperties/p:slideViewPr", property_name: Some("SlideViewProperties") },
    ChildInfo { name: "p:CT_OutlineViewProperties/p:outlineViewPr", property_name: Some("OutlineViewProperties") },
    ChildInfo { name: "p:CT_NotesTextViewProperties/p:notesTextViewPr", property_name: Some("NotesTextViewProperties") },
    ChildInfo { name: "p:CT_SlideSorterViewProperties/p:sorterViewPr", property_name: Some("SorterViewProperties") },
    ChildInfo { name: "p:CT_NotesViewProperties/p:notesViewPr", property_name: Some("NotesViewProperties") },
    ChildInfo { name: "a:CT_PositiveSize2D/p:gridSpacing", property_name: Some("GridSpacing") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_CONTENT_PART: &[AttributeInfo] = &[
    AttributeInfo { qname: "p14:bwMode", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_CONTENT_PART: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_ContentPartNonVisual/p14:nvContentPartPr", property_name: Some("NonVisualContentPartProperties") },
    ChildInfo { name: "a:CT_Transform2D/p14:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "p:CT_ExtensionListModify/p14:extLst", property_name: Some("ExtensionListModify") },
];
static ATTRS_SOUND: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":builtIn", property_name: Some("BuiltIn"), type_name: "BooleanValue" },
];
static ATTRS_SOUND_TARGET: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":builtIn", property_name: Some("BuiltIn"), type_name: "BooleanValue" },
];
static ATTRS_START_SOUND_ACTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":loop", property_name: Some("Loop"), type_name: "BooleanValue" },
];
static CHILDREN_START_SOUND_ACTION: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EmbeddedWAVAudioFile/p:snd", property_name: Some("Sound") },
];
static ATTRS_TIME_ABSOLUTE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_TIME_PERCENTAGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static CHILDREN_TARGET_ELEMENT: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Empty/p:sldTgt", property_name: Some("SlideTarget") },
    ChildInfo { name: "a:CT_EmbeddedWAVAudioFile/p:sndTgt", property_name: Some("SoundTarget") },
    ChildInfo { name: "p:CT_TLShapeTargetElement/p:spTgt", property_name: Some("ShapeTarget") },
    ChildInfo { name: "p:CT_TLSubShapeId/p:inkTgt", property_name: Some("InkTarget") },
    ChildInfo { name: "p14:CT_MediaBookmarkTarget/p14:bmkTgt", property_name: Some("BookmarkTarget") },
];
static ATTRS_TIME_NODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "UInt32Value" },
];
static ATTRS_RUNTIME_NODE_TRIGGER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_CONDITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":evt", property_name: Some("Event"), type_name: "EnumValue" },
    AttributeInfo { qname: ":delay", property_name: Some("Delay"), type_name: "StringValue" },
];
static CHILDREN_CONDITION: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeTargetElement/p:tgtEl", property_name: Some("TargetElement") },
    ChildInfo { name: "p:CT_TLTriggerTimeNodeID/p:tn", property_name: Some("TimeNode") },
    ChildInfo { name: "p:CT_TLTriggerRuntimeNode/p:rtn", property_name: Some("RuntimeNodeTrigger") },
];
static ATTRS_END_SYNC: &[AttributeInfo] = &[
    AttributeInfo { qname: ":evt", property_name: Some("Event"), type_name: "EnumValue" },
    AttributeInfo { qname: ":delay", property_name: Some("Delay"), type_name: "StringValue" },
];
static CHILDREN_END_SYNC: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeTargetElement/p:tgtEl", property_name: Some("TargetElement") },
    ChildInfo { name: "p:CT_TLTriggerTimeNodeID/p:tn", property_name: Some("TimeNode") },
    ChildInfo { name: "p:CT_TLTriggerRuntimeNode/p:rtn", property_name: Some("RuntimeNodeTrigger") },
];
static CHILDREN_PARALLEL_TIME_NODE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonTimeNodeData/p:cTn", property_name: Some("CommonTimeNode") },
];
static ATTRS_SEQUENCE_TIME_NODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":concurrent", property_name: Some("Concurrent"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":prevAc", property_name: Some("PreviousAction"), type_name: "EnumValue" },
    AttributeInfo { qname: ":nextAc", property_name: Some("NextAction"), type_name: "EnumValue" },
];
static CHILDREN_SEQUENCE_TIME_NODE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonTimeNodeData/p:cTn", property_name: Some("CommonTimeNode") },
    ChildInfo { name: "p:CT_TLTimeConditionList/p:prevCondLst", property_name: Some("PreviousConditionList") },
    ChildInfo { name: "p:CT_TLTimeConditionList/p:nextCondLst", property_name: Some("NextConditionList") },
];
static CHILDREN_EXCLUSIVE_TIME_NODE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonTimeNodeData/p:cTn", property_name: Some("CommonTimeNode") },
];
static ATTRS_ANIMATE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":by", property_name: Some("By"), type_name: "StringValue" },
    AttributeInfo { qname: ":from", property_name: Some("From"), type_name: "StringValue" },
    AttributeInfo { qname: ":to", property_name: Some("To"), type_name: "StringValue" },
    AttributeInfo { qname: ":calcmode", property_name: Some("CalculationMode"), type_name: "EnumValue" },
    AttributeInfo { qname: ":valueType", property_name: Some("ValueType"), type_name: "EnumValue" },
    AttributeInfo { qname: "p14:bounceEnd", property_name: Some("BounceEnd"), type_name: "Int32Value" },
];
static CHILDREN_ANIMATE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
    ChildInfo { name: "p:CT_TLTimeAnimateValueList/p:tavLst", property_name: Some("TimeAnimateValueList") },
];
static ATTRS_ANIMATE_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":clrSpc", property_name: Some("ColorSpace"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static CHILDREN_ANIMATE_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
    ChildInfo { name: "p:CT_TLByAnimateColorTransform/p:by", property_name: Some("ByColor") },
    ChildInfo { name: "a:CT_Color3/p:from", property_name: Some("FromColor") },
    ChildInfo { name: "a:CT_Color3/p:to", property_name: Some("ToColor") },
];
static ATTRS_ANIMATE_EFFECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":transition", property_name: Some("Transition"), type_name: "EnumValue" },
    AttributeInfo { qname: ":filter", property_name: Some("Filter"), type_name: "StringValue" },
    AttributeInfo { qname: ":prLst", property_name: Some("PropertyList"), type_name: "StringValue" },
];
static CHILDREN_ANIMATE_EFFECT: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
    ChildInfo { name: "p:CT_TLAnimFloat/p:progress", property_name: Some("Progress") },
];
static ATTRS_ANIMATE_MOTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":origin", property_name: Some("Origin"), type_name: "EnumValue" },
    AttributeInfo { qname: ":path", property_name: Some("Path"), type_name: "StringValue" },
    AttributeInfo { qname: ":pathEditMode", property_name: Some("PathEditMode"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rAng", property_name: Some("RelativeAngle"), type_name: "Int32Value" },
    AttributeInfo { qname: ":ptsTypes", property_name: Some("PointTypes"), type_name: "StringValue" },
    AttributeInfo { qname: "p14:bounceEnd", property_name: Some("BounceEnd"), type_name: "Int32Value" },
];
static CHILDREN_ANIMATE_MOTION: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
    ChildInfo { name: "p:CT_TLPoint/p:by", property_name: Some("ByPosition") },
    ChildInfo { name: "p:CT_TLPoint/p:from", property_name: Some("FromPosition") },
    ChildInfo { name: "p:CT_TLPoint/p:to", property_name: Some("ToPosition") },
    ChildInfo { name: "p:CT_TLPoint/p:rCtr", property_name: Some("RotationCenter") },
];
static ATTRS_ANIMATE_ROTATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":by", property_name: Some("By"), type_name: "Int32Value" },
    AttributeInfo { qname: ":from", property_name: Some("From"), type_name: "Int32Value" },
    AttributeInfo { qname: ":to", property_name: Some("To"), type_name: "Int32Value" },
    AttributeInfo { qname: "p14:bounceEnd", property_name: Some("BounceEnd"), type_name: "Int32Value" },
];
static CHILDREN_ANIMATE_ROTATION: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
];
static ATTRS_ANIMATE_SCALE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":zoomContents", property_name: Some("ZoomContents"), type_name: "BooleanValue" },
    AttributeInfo { qname: "p14:bounceEnd", property_name: Some("BounceEnd"), type_name: "Int32Value" },
];
static CHILDREN_ANIMATE_SCALE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
    ChildInfo { name: "p:CT_TLPoint/p:by", property_name: Some("ByPosition") },
    ChildInfo { name: "p:CT_TLPoint/p:from", property_name: Some("FromPosition") },
    ChildInfo { name: "p:CT_TLPoint/p:to", property_name: Some("ToPosition") },
];
static ATTRS_COMMAND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmd", property_name: Some("CommandName"), type_name: "StringValue" },
];
static CHILDREN_COMMAND: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
];
static CHILDREN_SET_BEHAVIOR: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonBehaviorData/p:cBhvr", property_name: Some("CommonBehavior") },
    ChildInfo { name: "p:CT_TLAnimVariant/p:to", property_name: Some("ToVariantValue") },
];
static ATTRS_AUDIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isNarration", property_name: Some("IsNarration"), type_name: "BooleanValue" },
];
static CHILDREN_AUDIO: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonMediaNodeData/p:cMediaNode", property_name: Some("CommonMediaNode") },
];
static ATTRS_VIDEO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fullScrn", property_name: Some("FullScreen"), type_name: "BooleanValue" },
];
static CHILDREN_VIDEO: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonMediaNodeData/p:cMediaNode", property_name: Some("CommonMediaNode") },
];
static ATTRS_COMMON_TIME_NODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":presetID", property_name: Some("PresetId"), type_name: "Int32Value" },
    AttributeInfo { qname: ":presetClass", property_name: Some("PresetClass"), type_name: "EnumValue" },
    AttributeInfo { qname: ":presetSubtype", property_name: Some("PresetSubtype"), type_name: "Int32Value" },
    AttributeInfo { qname: ":dur", property_name: Some("Duration"), type_name: "StringValue" },
    AttributeInfo { qname: ":repeatCount", property_name: Some("RepeatCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":repeatDur", property_name: Some("RepeatDuration"), type_name: "StringValue" },
    AttributeInfo { qname: ":spd", property_name: Some("Speed"), type_name: "Int32Value" },
    AttributeInfo { qname: ":accel", property_name: Some("Acceleration"), type_name: "Int32Value" },
    AttributeInfo { qname: ":decel", property_name: Some("Deceleration"), type_name: "Int32Value" },
    AttributeInfo { qname: ":autoRev", property_name: Some("AutoReverse"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":restart", property_name: Some("Restart"), type_name: "EnumValue" },
    AttributeInfo { qname: ":fill", property_name: Some("Fill"), type_name: "EnumValue" },
    AttributeInfo { qname: ":syncBehavior", property_name: Some("SyncBehavior"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tmFilter", property_name: Some("TimeFilter"), type_name: "StringValue" },
    AttributeInfo { qname: ":evtFilter", property_name: Some("EventFilter"), type_name: "StringValue" },
    AttributeInfo { qname: ":display", property_name: Some("Display"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":masterRel", property_name: Some("MasterRelation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bldLvl", property_name: Some("BuildLevel"), type_name: "Int32Value" },
    AttributeInfo { qname: ":grpId", property_name: Some("GroupId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":afterEffect", property_name: Some("AfterEffect"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":nodeType", property_name: Some("NodeType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":nodePh", property_name: Some("NodePlaceholder"), type_name: "BooleanValue" },
    AttributeInfo { qname: "p14:presetBounceEnd", property_name: Some("PresetBounceEnd"), type_name: "Int32Value" },
];
static CHILDREN_COMMON_TIME_NODE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeConditionList/p:stCondLst", property_name: Some("StartConditionList") },
    ChildInfo { name: "p:CT_TLTimeConditionList/p:endCondLst", property_name: Some("EndConditionList") },
    ChildInfo { name: "p:CT_TLTimeCondition/p:endSync", property_name: Some("EndSync") },
    ChildInfo { name: "p:CT_TLIterateData/p:iterate", property_name: Some("Iterate") },
    ChildInfo { name: "p:CT_TimeNodeList/p:childTnLst", property_name: Some("ChildTimeNodeList") },
    ChildInfo { name: "p:CT_TimeNodeList/p:subTnLst", property_name: Some("SubTimeNodeList") },
];
static CHILDREN_PREVIOUS_CONDITION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeCondition/p:cond", property_name: None },
];
static CHILDREN_NEXT_CONDITION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeCondition/p:cond", property_name: None },
];
static CHILDREN_START_CONDITION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeCondition/p:cond", property_name: None },
];
static CHILDREN_END_CONDITION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeCondition/p:cond", property_name: None },
];
static CHILDREN_ATTRIBUTE_NAME_LIST: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/p:attrName", property_name: None },
];
static ATTRS_BOOLEAN_VARIANT_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_INTEGER_VARIANT_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_FLOAT_VARIANT_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "SingleValue" },
];
static ATTRS_STRING_VARIANT_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_COLOR_VALUE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_PEN_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_TIME_ANIMATE_VALUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tm", property_name: Some("Time"), type_name: "StringValue" },
    AttributeInfo { qname: ":fmla", property_name: Some("Fomula"), type_name: "StringValue" },
];
static CHILDREN_TIME_ANIMATE_VALUE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLAnimVariant/p:val", property_name: Some("VariantValue") },
];
static ATTRS_RGB_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":r", property_name: Some("Red"), type_name: "Int32Value" },
    AttributeInfo { qname: ":g", property_name: Some("Green"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Blue"), type_name: "Int32Value" },
];
static ATTRS_HSL_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":h", property_name: Some("Hue"), type_name: "Int32Value" },
    AttributeInfo { qname: ":s", property_name: Some("Saturation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":l", property_name: Some("Lightness"), type_name: "Int32Value" },
];
static ATTRS_COMMON_BEHAVIOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":additive", property_name: Some("Additive"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accumulate", property_name: Some("Accumulate"), type_name: "EnumValue" },
    AttributeInfo { qname: ":xfrmType", property_name: Some("TransformType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":from", property_name: Some("From"), type_name: "StringValue" },
    AttributeInfo { qname: ":to", property_name: Some("To"), type_name: "StringValue" },
    AttributeInfo { qname: ":by", property_name: Some("By"), type_name: "StringValue" },
    AttributeInfo { qname: ":rctx", property_name: Some("RuntimeContext"), type_name: "StringValue" },
    AttributeInfo { qname: ":override", property_name: Some("Override"), type_name: "EnumValue" },
];
static CHILDREN_COMMON_BEHAVIOR: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonTimeNodeData/p:cTn", property_name: Some("CommonTimeNode") },
    ChildInfo { name: "p:CT_TLTimeTargetElement/p:tgtEl", property_name: Some("TargetElement") },
    ChildInfo { name: "p:CT_TLBehaviorAttributeNameList/p:attrNameLst", property_name: Some("AttributeNameList") },
];
static CHILDREN_PROGRESS: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLAnimVariantFloatVal/p:fltVal", property_name: Some("FloatVariantValue") },
];
static CHILDREN_TO_VARIANT_VALUE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLAnimVariantBooleanVal/p:boolVal", property_name: Some("BooleanVariantValue") },
    ChildInfo { name: "p:CT_TLAnimVariantIntegerVal/p:intVal", property_name: Some("IntegerVariantValue") },
    ChildInfo { name: "p:CT_TLAnimVariantFloatVal/p:fltVal", property_name: Some("FloatVariantValue") },
    ChildInfo { name: "p:CT_TLAnimVariantStringVal/p:strVal", property_name: Some("StringVariantValue") },
    ChildInfo { name: "a:CT_Color/p:clrVal", property_name: Some("ColorValue") },
];
static CHILDREN_VARIANT_VALUE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLAnimVariantBooleanVal/p:boolVal", property_name: Some("BooleanVariantValue") },
    ChildInfo { name: "p:CT_TLAnimVariantIntegerVal/p:intVal", property_name: Some("IntegerVariantValue") },
    ChildInfo { name: "p:CT_TLAnimVariantFloatVal/p:fltVal", property_name: Some("FloatVariantValue") },
    ChildInfo { name: "p:CT_TLAnimVariantStringVal/p:strVal", property_name: Some("StringVariantValue") },
    ChildInfo { name: "a:CT_Color/p:clrVal", property_name: Some("ColorValue") },
];
static ATTRS_COMMON_MEDIA_NODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":vol", property_name: Some("Volume"), type_name: "Int32Value" },
    AttributeInfo { qname: ":mute", property_name: Some("Mute"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":numSld", property_name: Some("SlideCount"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":showWhenStopped", property_name: Some("ShowWhenStopped"), type_name: "BooleanValue" },
];
static CHILDREN_COMMON_MEDIA_NODE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLCommonTimeNodeData/p:cTn", property_name: Some("CommonTimeNode") },
    ChildInfo { name: "p:CT_TLTimeTargetElement/p:tgtEl", property_name: Some("TargetElement") },
];
static CHILDREN_TIME_NODE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeNodeParallel/p:par", property_name: Some("ParallelTimeNode") },
];
static ATTRS_TEMPLATE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "UInt32Value" },
];
static CHILDREN_TEMPLATE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_RootTimeNode/p:tnLst", property_name: Some("TimeNodeList") },
];
static CHILDREN_TEMPLATE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTemplate/p:tmpl", property_name: None },
];
static CHILDREN_BUILD_SUB_ELEMENT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AnimationDgmBuildProperties/a:bldDgm", property_name: Some("BuildDiagram") },
    ChildInfo { name: "a:CT_AnimationChartBuildProperties/a:bldChart", property_name: Some("BuildChart") },
];
static ATTRS_BUILD_PARAGRAPH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":grpId", property_name: Some("GroupId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":uiExpand", property_name: Some("UiExpand"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":build", property_name: Some("Build"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bldLvl", property_name: Some("BuildLevel"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":animBg", property_name: Some("AnimateBackground"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":autoUpdateAnimBg", property_name: Some("AutoAnimateBackground"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":rev", property_name: Some("Reverse"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":advAuto", property_name: Some("AutoAdvance"), type_name: "StringValue" },
];
static CHILDREN_BUILD_PARAGRAPH: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTemplateList/p:tmplLst", property_name: Some("TemplateList") },
];
static ATTRS_BUILD_DIAGRAM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":grpId", property_name: Some("GroupId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":uiExpand", property_name: Some("UiExpand"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":bld", property_name: Some("Build"), type_name: "EnumValue" },
];
static ATTRS_BUILD_OLE_CHART: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":grpId", property_name: Some("GroupId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":uiExpand", property_name: Some("UiExpand"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":bld", property_name: Some("Build"), type_name: "EnumValue" },
    AttributeInfo { qname: ":animBg", property_name: Some("AnimateBackground"), type_name: "BooleanValue" },
];
static ATTRS_BUILD_GRAPHICS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":grpId", property_name: Some("GroupId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":uiExpand", property_name: Some("UiExpand"), type_name: "BooleanValue" },
];
static CHILDREN_BUILD_GRAPHICS: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Empty/p:bldAsOne", property_name: Some("BuildAsOne") },
    ChildInfo { name: "a:CT_AnimationGraphicalObjectBuildProperties/p:bldSub", property_name: Some("BuildSubElement") },
];
static CHILDREN_BUILD_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLBuildParagraph/p:bldP", property_name: None },
    ChildInfo { name: "p:CT_TLBuildDiagram/p:bldDgm", property_name: None },
    ChildInfo { name: "p:CT_TLOleBuildChart/p:bldOleChart", property_name: None },
    ChildInfo { name: "p:CT_TLGraphicalObjectBuild/p:bldGraphic", property_name: None },
];
static ATTRS_EXTENSION_LIST_WITH_MODIFICATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":mod", property_name: Some("Modify"), type_name: "BooleanValue" },
];
static CHILDREN_EXTENSION_LIST_WITH_MODIFICATION: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static CHILDREN_BY_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLByRgbColorTransform/p:rgb", property_name: Some("RgbColor") },
    ChildInfo { name: "p:CT_TLByHslColorTransform/p:hsl", property_name: Some("HslColor") },
];
static CHILDREN_FROM_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_TO_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_SLIDE_LIST_ENTRY: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_CUSTOMER_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_CUSTOMER_DATA_TAGS: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_COMMENT_AUTHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":initials", property_name: Some("Initials"), type_name: "StringValue" },
    AttributeInfo { qname: ":lastIdx", property_name: Some("LastIndex"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":clrIdx", property_name: Some("ColorIndex"), type_name: "UInt32Value" },
];
static CHILDREN_COMMENT_AUTHOR: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommentAuthorExtensionList/p:extLst", property_name: Some("CommentAuthorExtensionList") },
];
static ATTRS_COMMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":authorId", property_name: Some("AuthorId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":dt", property_name: Some("DateTime"), type_name: "DateTimeValue" },
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_COMMENT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/p:pos", property_name: Some("Position") },
    ChildInfo { name: "xsd:string/p:text", property_name: Some("Text") },
    ChildInfo { name: "p:CT_CommentExtensionList/p:extLst", property_name: Some("CommentExtensionList") },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Extension/p:ext", property_name: None },
];
static ATTRS_CONTROL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":showAsIcon", property_name: Some("ShowAsIcon"), type_name: "BooleanValue" },
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":imgW", property_name: Some("ImageWidth"), type_name: "Int32Value" },
    AttributeInfo { qname: ":imgH", property_name: Some("ImageHeight"), type_name: "Int32Value" },
];
static CHILDREN_CONTROL: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
    ChildInfo { name: "p:CT_Picture/p:pic", property_name: Some("Picture") },
];
static ATTRS_SLIDE_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: "r:id", property_name: Some("RelationshipId"), type_name: "StringValue" },
];
static CHILDREN_SLIDE_ID: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SLIDE_MASTER_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: "r:id", property_name: Some("RelationshipId"), type_name: "StringValue" },
];
static CHILDREN_SLIDE_MASTER_ID: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_NOTES_MASTER_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_NOTES_MASTER_ID: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_HANDOUT_MASTER_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_HANDOUT_MASTER_ID: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":typeface", property_name: Some("Typeface"), type_name: "StringValue" },
    AttributeInfo { qname: ":panose", property_name: Some("Panose"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":pitchFamily", property_name: Some("PitchFamily"), type_name: "SByteValue" },
    AttributeInfo { qname: ":charset", property_name: Some("CharacterSet"), type_name: "SByteValue" },
];
static ATTRS_REGULAR_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_BOLD_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_ITALIC_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_BOLD_ITALIC_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_EMBEDDED_FONT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextFont/p:font", property_name: Some("Font") },
    ChildInfo { name: "p:CT_EmbeddedFontDataId/p:regular", property_name: Some("RegularFont") },
    ChildInfo { name: "p:CT_EmbeddedFontDataId/p:bold", property_name: Some("BoldFont") },
    ChildInfo { name: "p:CT_EmbeddedFontDataId/p:italic", property_name: Some("ItalicFont") },
    ChildInfo { name: "p:CT_EmbeddedFontDataId/p:boldItalic", property_name: Some("BoldItalicFont") },
];
static CHILDREN_SLIDE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideRelationshipListEntry/p:sld", property_name: None },
];
static ATTRS_CUSTOM_SHOW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
];
static CHILDREN_CUSTOM_SHOW: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideRelationshipList/p:sldLst", property_name: Some("SlideList") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
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
static CHILDREN_NON_VISUAL_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/p:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualDrawingShapeProps/p:cNvSpPr", property_name: Some("NonVisualShapeDrawingProperties") },
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingProps/p:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
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
static CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectorLocking/a:cxnSpLocks", property_name: Some("ConnectionShapeLocks") },
    ChildInfo { name: "a:CT_Connection/a:stCxn", property_name: Some("StartConnection") },
    ChildInfo { name: "a:CT_Connection/a:endCxn", property_name: Some("EndConnection") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/p:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualConnectorProperties/p:cNvCxnSpPr", property_name: Some("NonVisualConnectorShapeDrawingProperties") },
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingProps/p:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
];
static ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":preferRelativeResize", property_name: Some("PreferRelativeResize"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PictureLocking/a:picLocks", property_name: Some("PictureLocks") },
    ChildInfo { name: "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst", property_name: Some("NonVisualPicturePropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_PICTURE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/p:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualPictureProperties/p:cNvPicPr", property_name: Some("NonVisualPictureDrawingProperties") },
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingProps/p:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
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
    ChildInfo { name: "a:CT_NonVisualDrawingProps/p:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/p:cNvGraphicFramePr", property_name: Some("NonVisualGraphicFrameDrawingProperties") },
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingProps/p:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
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
static CHILDREN_TITLE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:defPPr", property_name: Some("DefaultParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl1pPr", property_name: Some("Level1ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl2pPr", property_name: Some("Level2ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl3pPr", property_name: Some("Level3ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl4pPr", property_name: Some("Level4ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl5pPr", property_name: Some("Level5ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl6pPr", property_name: Some("Level6ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl7pPr", property_name: Some("Level7ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl8pPr", property_name: Some("Level8ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl9pPr", property_name: Some("Level9ParagraphProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_BODY_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:defPPr", property_name: Some("DefaultParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl1pPr", property_name: Some("Level1ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl2pPr", property_name: Some("Level2ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl3pPr", property_name: Some("Level3ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl4pPr", property_name: Some("Level4ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl5pPr", property_name: Some("Level5ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl6pPr", property_name: Some("Level6ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl7pPr", property_name: Some("Level7ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl8pPr", property_name: Some("Level8ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl9pPr", property_name: Some("Level9ParagraphProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_OTHER_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:defPPr", property_name: Some("DefaultParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl1pPr", property_name: Some("Level1ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl2pPr", property_name: Some("Level2ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl3pPr", property_name: Some("Level3ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl4pPr", property_name: Some("Level4ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl5pPr", property_name: Some("Level5ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl6pPr", property_name: Some("Level6ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl7pPr", property_name: Some("Level7ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl8pPr", property_name: Some("Level8ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl9pPr", property_name: Some("Level9ParagraphProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_DEFAULT_TEXT_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:defPPr", property_name: Some("DefaultParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl1pPr", property_name: Some("Level1ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl2pPr", property_name: Some("Level2ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl3pPr", property_name: Some("Level3ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl4pPr", property_name: Some("Level4ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl5pPr", property_name: Some("Level5ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl6pPr", property_name: Some("Level6ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl7pPr", property_name: Some("Level7ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl8pPr", property_name: Some("Level8ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl9pPr", property_name: Some("Level9ParagraphProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NOTES_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:defPPr", property_name: Some("DefaultParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl1pPr", property_name: Some("Level1ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl2pPr", property_name: Some("Level2ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl3pPr", property_name: Some("Level3ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl4pPr", property_name: Some("Level4ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl5pPr", property_name: Some("Level5ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl6pPr", property_name: Some("Level6ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl7pPr", property_name: Some("Level7ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl8pPr", property_name: Some("Level8ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl9pPr", property_name: Some("Level9ParagraphProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SLIDE_LAYOUT_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: "r:id", property_name: Some("RelationshipId"), type_name: "StringValue" },
];
static CHILDREN_SLIDE_LAYOUT_ID: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_COMMON_SLIDE_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_COMMON_SLIDE_DATA: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Background/p:bg", property_name: Some("Background") },
    ChildInfo { name: "p:CT_GroupShape/p:spTree", property_name: Some("ShapeTree") },
    ChildInfo { name: "p:CT_CustomerDataList/p:custDataLst", property_name: Some("CustomerDataList") },
    ChildInfo { name: "p:CT_ControlList/p:controls", property_name: Some("ControlList") },
    ChildInfo { name: "p:CT_CommonSlideDataExtensionList/p:extLst", property_name: Some("CommonSlideDataExtensionList") },
];
static ATTRS_TAG: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_RESTORED_LEFT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sz", property_name: Some("Size"), type_name: "Int32Value" },
    AttributeInfo { qname: ":autoAdjust", property_name: Some("AutoAdjust"), type_name: "BooleanValue" },
];
static ATTRS_RESTORED_TOP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sz", property_name: Some("Size"), type_name: "Int32Value" },
    AttributeInfo { qname: ":autoAdjust", property_name: Some("AutoAdjust"), type_name: "BooleanValue" },
];
static CHILDREN_SCALE_FACTOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Ratio/a:sx", property_name: Some("ScaleX") },
    ChildInfo { name: "a:CT_Ratio/a:sy", property_name: Some("ScaleY") },
];
static ATTRS_ORIGIN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_COMMON_VIEW_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":varScale", property_name: Some("VariableScale"), type_name: "BooleanValue" },
];
static CHILDREN_COMMON_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Scale2D/p:scale", property_name: Some("ScaleFactor") },
    ChildInfo { name: "a:CT_Point2D/p:origin", property_name: Some("Origin") },
];
static ATTRS_OUTLINE_VIEW_SLIDE_LIST_ENTRY: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":collapse", property_name: Some("Collapse"), type_name: "BooleanValue" },
];
static CHILDREN_OUTLINE_VIEW_SLIDE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_OutlineViewSlideEntry/p:sld", property_name: None },
];
static ATTRS_GUIDE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":orient", property_name: Some("Orientation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":pos", property_name: Some("Position"), type_name: "Int32Value" },
];
static CHILDREN_GUIDE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Guide/p:guide", property_name: None },
];
static ATTRS_COMMON_SLIDE_VIEW_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":snapToGrid", property_name: Some("SnapToGrid"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":snapToObjects", property_name: Some("SnapToObjects"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showGuides", property_name: Some("ShowGuides"), type_name: "BooleanValue" },
];
static CHILDREN_COMMON_SLIDE_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonViewProperties/p:cViewPr", property_name: Some("CommonViewProperties") },
    ChildInfo { name: "p:CT_GuideList/p:guideLst", property_name: Some("GuideList") },
];
static ATTRS_NORMAL_VIEW_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showOutlineIcons", property_name: Some("ShowOutlineIcons"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":snapVertSplitter", property_name: Some("SnapVerticalSplitter"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":vertBarState", property_name: Some("VerticalBarState"), type_name: "EnumValue" },
    AttributeInfo { qname: ":horzBarState", property_name: Some("HorizontalBarState"), type_name: "EnumValue" },
    AttributeInfo { qname: ":preferSingleView", property_name: Some("PreferSingleView"), type_name: "BooleanValue" },
];
static CHILDREN_NORMAL_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_NormalViewPortion/p:restoredLeft", property_name: Some("RestoredLeft") },
    ChildInfo { name: "p:CT_NormalViewPortion/p:restoredTop", property_name: Some("RestoredTop") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_SLIDE_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideViewProperties/p:cSldViewPr", property_name: Some("CommonSlideViewProperties") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_OUTLINE_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonViewProperties/p:cViewPr", property_name: Some("CommonViewProperties") },
    ChildInfo { name: "p:CT_OutlineViewSlideList/p:sldLst", property_name: Some("OutlineViewSlideList") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NOTES_TEXT_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonViewProperties/p:cViewPr", property_name: Some("CommonViewProperties") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SORTER_VIEW_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showFormatting", property_name: Some("ShowFormatting"), type_name: "BooleanValue" },
];
static CHILDREN_SORTER_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonViewProperties/p:cViewPr", property_name: Some("CommonViewProperties") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NOTES_VIEW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideViewProperties/p:cSldViewPr", property_name: Some("CommonSlideViewProperties") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_GRID_SPACING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int64Value" },
];
static ATTRS_NOTES_SIZE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int64Value" },
];
static ATTRS_SLIDE_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SLIDE_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_LaserTraceList/p14:laserTraceLst", property_name: Some("LaserTraceList") },
    ChildInfo { name: "p14:CT_ShowEventRecordList/p14:showEvtLst", property_name: Some("ShowEventRecordList") },
    ChildInfo { name: "p188:CT_CommentRelationship/p188:commentRel", property_name: Some("CommentRelationship") },
];
static ATTRS_COMMON_SLIDE_DATA_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_COMMON_SLIDE_DATA_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_RandomId/p14:creationId", property_name: Some("CreationId") },
];
static ATTRS_SHOW_PROPERTIES_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SHOW_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_BrowseMode/p14:browseMode", property_name: Some("BrowseMode") },
    ChildInfo { name: "a:CT_Color/p14:laserClr", property_name: Some("LaserColor") },
    ChildInfo { name: "p14:CT_ShowMediaControls/p14:showMediaCtrls", property_name: Some("ShowMediaControls") },
];
static CHILDREN_PICTURE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_PictureNonVisual/p:nvPicPr", property_name: Some("NonVisualPictureProperties") },
    ChildInfo { name: "a:CT_BlipFillProperties/p:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_ShapeProperties/p:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/p:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static ATTRS_OLE_OBJECT_EMBED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":followColorScheme", property_name: Some("FollowColorScheme"), type_name: "EnumValue" },
];
static CHILDREN_OLE_OBJECT_EMBED: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_OLE_OBJECT_LINK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":updateAutomatic", property_name: Some("AutoUpdate"), type_name: "BooleanValue" },
];
static CHILDREN_OLE_OBJECT_LINK: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spd", property_name: Some("Speed"), type_name: "EnumValue" },
    AttributeInfo { qname: "p14:dur", property_name: Some("Duration"), type_name: "StringValue" },
    AttributeInfo { qname: ":advClick", property_name: Some("AdvanceOnClick"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":advTm", property_name: Some("AdvanceAfterTime"), type_name: "StringValue" },
];
static CHILDREN_TRANSITION: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_OrientationTransition/p:blinds", property_name: None },
    ChildInfo { name: "p:CT_OrientationTransition/p:checker", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:circle", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:dissolve", property_name: None },
    ChildInfo { name: "p:CT_OrientationTransition/p:comb", property_name: None },
    ChildInfo { name: "p:CT_EightDirectionTransition/p:cover", property_name: None },
    ChildInfo { name: "p:CT_OptionalBlackTransition/p:cut", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:diamond", property_name: None },
    ChildInfo { name: "p:CT_OptionalBlackTransition/p:fade", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:newsflash", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:plus", property_name: None },
    ChildInfo { name: "p:CT_EightDirectionTransition/p:pull", property_name: None },
    ChildInfo { name: "p:CT_SideDirectionTransition/p:push", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:random", property_name: None },
    ChildInfo { name: "p:CT_OrientationTransition/p:randomBar", property_name: None },
    ChildInfo { name: "p:CT_SplitTransition/p:split", property_name: None },
    ChildInfo { name: "p:CT_CornerDirectionTransition/p:strips", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:wedge", property_name: None },
    ChildInfo { name: "p:CT_WheelTransition/p:wheel", property_name: None },
    ChildInfo { name: "p:CT_SideDirectionTransition/p:wipe", property_name: None },
    ChildInfo { name: "p:CT_InOutTransition/p:zoom", property_name: None },
    ChildInfo { name: "p:CT_Empty/p14:flash", property_name: None },
    ChildInfo { name: "p:CT_SideDirectionTransition/p14:vortex", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:switch", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:flip", property_name: None },
    ChildInfo { name: "p14:CT_RippleTransition/p14:ripple", property_name: None },
    ChildInfo { name: "p14:CT_GlitterTransition/p14:glitter", property_name: None },
    ChildInfo { name: "p:CT_Empty/p14:honeycomb", property_name: None },
    ChildInfo { name: "p14:CT_PrismTransition/p14:prism", property_name: None },
    ChildInfo { name: "p:CT_OrientationTransition/p14:doors", property_name: None },
    ChildInfo { name: "p:CT_OrientationTransition/p14:window", property_name: None },
    ChildInfo { name: "p14:CT_ShredTransition/p14:shred", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:ferris", property_name: None },
    ChildInfo { name: "p14:CT_FlyThroughTransition/p14:flythrough", property_name: None },
    ChildInfo { name: "p:CT_InOutTransition/p14:warp", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:gallery", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:conveyor", property_name: None },
    ChildInfo { name: "p:CT_SideDirectionTransition/p14:pan", property_name: None },
    ChildInfo { name: "p14:CT_RevealTransition/p14:reveal", property_name: None },
    ChildInfo { name: "p:CT_WheelTransition/p14:wheelReverse", property_name: None },
    ChildInfo { name: "p15:CT_PresetTransition/p15:prstTrans", property_name: None },
    ChildInfo { name: "p:CT_TransitionSoundAction/p:sndAc", property_name: None },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: None },
];
static CHILDREN_TIMING: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_RootTimeNode/p:tnLst", property_name: Some("TimeNodeList") },
    ChildInfo { name: "p:CT_BuildList/p:bldLst", property_name: Some("BuildList") },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static CHILDREN_SLIDE_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideExtension/p:ext", property_name: None },
];
static ATTRS_BACKGROUND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_BACKGROUND: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_BackgroundProperties/p:bgPr", property_name: Some("BackgroundProperties") },
    ChildInfo { name: "a:CT_StyleMatrixReference/p:bgRef", property_name: Some("BackgroundStyleReference") },
];
static CHILDREN_SHAPE_TREE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_GroupShapeNonVisual/p:nvGrpSpPr", property_name: Some("NonVisualGroupShapeProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/p:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "p:CT_Shape/p:sp", property_name: None },
    ChildInfo { name: "p:CT_GroupShape/p:grpSp", property_name: None },
    ChildInfo { name: "p:CT_GraphicalObjectFrame/p:graphicFrame", property_name: None },
    ChildInfo { name: "p:CT_Connector/p:cxnSp", property_name: None },
    ChildInfo { name: "p:CT_Picture/p:pic", property_name: None },
    ChildInfo { name: "p:CT_ContentPart/p:contentPart", property_name: None },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: None },
];
static CHILDREN_GROUP_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_GroupShapeNonVisual/p:nvGrpSpPr", property_name: Some("NonVisualGroupShapeProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/p:grpSpPr", property_name: Some("GroupShapeProperties") },
    ChildInfo { name: "p:CT_Shape/p:sp", property_name: None },
    ChildInfo { name: "p:CT_GroupShape/p:grpSp", property_name: None },
    ChildInfo { name: "p:CT_GraphicalObjectFrame/p:graphicFrame", property_name: None },
    ChildInfo { name: "p:CT_Connector/p:cxnSp", property_name: None },
    ChildInfo { name: "p:CT_Picture/p:pic", property_name: None },
    ChildInfo { name: "p:CT_ContentPart/p:contentPart", property_name: None },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: None },
];
static CHILDREN_CUSTOMER_DATA_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CustomerData/p:custData", property_name: None },
    ChildInfo { name: "p:CT_TagsData/p:tags", property_name: None },
];
static CHILDREN_CONTROL_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Control/p:control", property_name: None },
];
static CHILDREN_COMMON_SLIDE_DATA_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommonSlideDataExtension/p:ext", property_name: None },
];
static CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/p:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/p:cNvGrpSpPr", property_name: Some("NonVisualGroupShapeDrawingProperties") },
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingProps/p:nvPr", property_name: Some("ApplicationNonVisualDrawingProperties") },
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
    AttributeInfo { qname: ":useBgFill", property_name: Some("UseBackgroundFill"), type_name: "BooleanValue" },
];
static CHILDREN_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ShapeNonVisual/p:nvSpPr", property_name: Some("NonVisualShapeProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/p:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/p:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_TextBody/p:txBody", property_name: Some("TextBody") },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static CHILDREN_GRAPHIC_FRAME: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_GraphicalObjectFrameNonVisual/p:nvGraphicFramePr", property_name: Some("NonVisualGraphicFrameProperties") },
    ChildInfo { name: "a:CT_Transform2D/p:xfrm", property_name: Some("Transform") },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static CHILDREN_CONNECTION_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ConnectorNonVisual/p:nvCxnSpPr", property_name: Some("NonVisualConnectionShapeProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/p:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/p:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static CHILDREN_SHOW_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ShowPropertiesExtension/p:ext", property_name: None },
];
static ATTRS_SHAPE_TARGET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
];
static CHILDREN_SHAPE_TARGET: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Empty/p:bg", property_name: Some("BackgroundAnimation") },
    ChildInfo { name: "p:CT_TLSubShapeId/p:subSp", property_name: Some("SubShape") },
    ChildInfo { name: "p:CT_TLOleChartTargetElement/p:oleChartEl", property_name: Some("OleChartElement") },
    ChildInfo { name: "p:CT_TLTextTargetElement/p:txEl", property_name: Some("TextElement") },
    ChildInfo { name: "a:CT_AnimationElementChoice/p:graphicEl", property_name: Some("GraphicElement") },
];
static ATTRS_INK_TARGET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
];
static ATTRS_SUB_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
];
static ATTRS_COMMENT_AUTHOR_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_COMMENT_AUTHOR_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_PresenceInfo/p15:presenceInfo", property_name: Some("PresenceInfo") },
];
static ATTRS_COMMENT_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_COMMENT_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_CommentThreading/p15:threadingInfo", property_name: Some("ThreadingInfo") },
];
static ATTRS_SLIDE_LAYOUT_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SLIDE_LAYOUT_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:sldGuideLst", property_name: Some("SlideGuideList") },
];
static ATTRS_SLIDE_MASTER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SLIDE_MASTER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:sldGuideLst", property_name: Some("SlideGuideList") },
];
static ATTRS_HANDOUT_MASTER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_HANDOUT_MASTER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:sldGuideLst", property_name: Some("SlideGuideList") },
];
static ATTRS_NOTES_MASTER_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_NOTES_MASTER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:sldGuideLst", property_name: Some("SlideGuideList") },
];
static ATTRS_PLACEHOLDER_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":orient", property_name: Some("Orientation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":sz", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":hasCustomPrompt", property_name: Some("HasCustomPrompt"), type_name: "BooleanValue" },
];
static CHILDREN_PLACEHOLDER_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static CHILDREN_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ApplicationNonVisualDrawingPropsExtension/p:ext", property_name: None },
];
static ATTRS_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_Media/p14:media", property_name: Some("Media") },
    ChildInfo { name: "p14:CT_RandomId/p14:modId", property_name: Some("ModificationId") },
];
static ATTRS_ITERATE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":backwards", property_name: Some("Backwards"), type_name: "BooleanValue" },
];
static CHILDREN_ITERATE: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLIterateIntervalTime/p:tmAbs", property_name: Some("TimeAbsolute") },
    ChildInfo { name: "p:CT_TLIterateIntervalPercentage/p:tmPct", property_name: Some("TimePercentage") },
];
static CHILDREN_CHILD_TIME_NODE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeNodeParallel/p:par", property_name: None },
    ChildInfo { name: "p:CT_TLTimeNodeSequence/p:seq", property_name: None },
    ChildInfo { name: "p:CT_TLTimeNodeExclusive/p:excl", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateBehavior/p:anim", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateColorBehavior/p:animClr", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateEffectBehavior/p:animEffect", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateMotionBehavior/p:animMotion", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateRotationBehavior/p:animRot", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateScaleBehavior/p:animScale", property_name: None },
    ChildInfo { name: "p:CT_TLCommandBehavior/p:cmd", property_name: None },
    ChildInfo { name: "p:CT_TLSetBehavior/p:set", property_name: None },
    ChildInfo { name: "p:CT_TLMediaNodeAudio/p:audio", property_name: None },
    ChildInfo { name: "p:CT_TLMediaNodeVideo/p:video", property_name: None },
];
static CHILDREN_SUB_TIME_NODE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeNodeParallel/p:par", property_name: None },
    ChildInfo { name: "p:CT_TLTimeNodeSequence/p:seq", property_name: None },
    ChildInfo { name: "p:CT_TLTimeNodeExclusive/p:excl", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateBehavior/p:anim", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateColorBehavior/p:animClr", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateEffectBehavior/p:animEffect", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateMotionBehavior/p:animMotion", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateRotationBehavior/p:animRot", property_name: None },
    ChildInfo { name: "p:CT_TLAnimateScaleBehavior/p:animScale", property_name: None },
    ChildInfo { name: "p:CT_TLCommandBehavior/p:cmd", property_name: None },
    ChildInfo { name: "p:CT_TLSetBehavior/p:set", property_name: None },
    ChildInfo { name: "p:CT_TLMediaNodeAudio/p:audio", property_name: None },
    ChildInfo { name: "p:CT_TLMediaNodeVideo/p:video", property_name: None },
];
static CHILDREN_TIME_ANIMATE_VALUE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TLTimeAnimateValue/p:tav", property_name: None },
];
static ATTRS_BY_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int32Value" },
];
static ATTRS_FROM_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int32Value" },
];
static ATTRS_TO_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int32Value" },
];
static ATTRS_ROTATION_CENTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int32Value" },
];
static CHILDREN_COMMENT_AUTHOR_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommentAuthorExtension/p:ext", property_name: None },
];
static CHILDREN_COMMENT_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CommentExtension/p:ext", property_name: None },
];
static CHILDREN_SLIDE_MASTER_ID_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideMasterIdListEntry/p:sldMasterId", property_name: None },
];
static CHILDREN_NOTES_MASTER_ID_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_NotesMasterIdListEntry/p:notesMasterId", property_name: Some("NotesMasterId") },
];
static CHILDREN_HANDOUT_MASTER_ID_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_HandoutMasterIdListEntry/p:handoutMasterId", property_name: Some("HandoutMasterId") },
];
static CHILDREN_SLIDE_ID_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideIdListEntry/p:sldId", property_name: None },
];
static ATTRS_SLIDE_SIZE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
];
static CHILDREN_EMBEDDED_FONT_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_EmbeddedFontListEntry/p:embeddedFont", property_name: None },
];
static CHILDREN_CUSTOM_SHOW_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_CustomShow/p:custShow", property_name: None },
];
static ATTRS_PHOTO_ALBUM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bw", property_name: Some("BlackWhite"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showCaptions", property_name: Some("ShowCaptions"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":layout", property_name: Some("Layout"), type_name: "EnumValue" },
    AttributeInfo { qname: ":frame", property_name: Some("Frame"), type_name: "EnumValue" },
];
static CHILDREN_PHOTO_ALBUM: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_KINSOKU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalStChars", property_name: Some("InvalidStartChars"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalEndChars", property_name: Some("InvalidEndChars"), type_name: "StringValue" },
];
static ATTRS_MODIFICATION_VERIFIER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cryptProviderType", property_name: Some("CryptographicProviderType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cryptAlgorithmClass", property_name: Some("CryptographicAlgorithmClass"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cryptAlgorithmType", property_name: Some("CryptographicAlgorithmType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cryptAlgorithmSid", property_name: Some("CryptographicAlgorithmSid"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":spinCount", property_name: Some("SpinCount"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":saltData", property_name: Some("SaltData"), type_name: "Base64BinaryValue" },
    AttributeInfo { qname: ":hashData", property_name: Some("HashData"), type_name: "StringValue" },
    AttributeInfo { qname: ":cryptProvider", property_name: Some("CryptographicProvider"), type_name: "StringValue" },
    AttributeInfo { qname: ":algIdExt", property_name: Some("ExtendedCryptographicAlgorithm"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":algIdExtSource", property_name: Some("ExtendedCryptographicAlgorithmSource"), type_name: "StringValue" },
    AttributeInfo { qname: ":cryptProviderTypeExt", property_name: Some("CryptographicProviderTypeExtensibility"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":cryptProviderTypeExtSource", property_name: Some("CryptographicProviderTypeExtensibilitySource"), type_name: "StringValue" },
    AttributeInfo { qname: ":algorithmName", property_name: Some("AlgorithmName"), type_name: "StringValue" },
    AttributeInfo { qname: ":hashValue", property_name: Some("HashValue"), type_name: "Base64BinaryValue" },
    AttributeInfo { qname: ":saltValue", property_name: Some("SaltValue"), type_name: "Base64BinaryValue" },
    AttributeInfo { qname: ":spinValue", property_name: Some("SpinValue"), type_name: "UInt32Value" },
];
static CHILDREN_PRESENTATION_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_PresentationExtension/p:ext", property_name: None },
];
static ATTRS_PRESENTATION_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PRESENTATION_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_SectionProperties/p14:sectionPr", property_name: Some("SectionProperties") },
    ChildInfo { name: "p14:CT_SectionList/p14:sectionLst", property_name: Some("SectionList") },
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:sldGuideLst", property_name: Some("SlideGuideList") },
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:notesGuideLst", property_name: Some("NotesGuideList") },
];
static ATTRS_HTML_PUBLISH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showSpeakerNotes", property_name: Some("ShowSpeakerNotes"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":pubBrowser", property_name: Some("TargetBrowser"), type_name: "EnumValue" },
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_HTML_PUBLISH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Empty/p:sldAll", property_name: None },
    ChildInfo { name: "p:CT_IndexRange/p:sldRg", property_name: None },
    ChildInfo { name: "p:CT_CustomShowId/p:custShow", property_name: None },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: None },
];
static ATTRS_WEB_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showAnimation", property_name: Some("ShowAnimation"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":resizeGraphics", property_name: Some("ResizeGraphics"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":allowPng", property_name: Some("AllowPng"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":relyOnVml", property_name: Some("RelyOnVml"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":organizeInFolders", property_name: Some("OrganizeInFolders"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":useLongFilenames", property_name: Some("UseLongFilenames"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":imgSz", property_name: Some("ImageSize"), type_name: "EnumValue" },
    AttributeInfo { qname: ":encoding", property_name: Some("Encoding"), type_name: "StringValue" },
    AttributeInfo { qname: ":clr", property_name: Some("Color"), type_name: "EnumValue" },
];
static CHILDREN_WEB_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PRINTING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prnWhat", property_name: Some("PrintWhat"), type_name: "EnumValue" },
    AttributeInfo { qname: ":clrMode", property_name: Some("ColorMode"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hiddenSlides", property_name: Some("HiddenSlides"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":scaleToFitPaper", property_name: Some("ScaleToFitPaper"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":frameSlides", property_name: Some("FrameSlides"), type_name: "BooleanValue" },
];
static CHILDREN_PRINTING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SHOW_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":loop", property_name: Some("Loop"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showNarration", property_name: Some("ShowNarration"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showAnimation", property_name: Some("ShowAnimation"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":useTimings", property_name: Some("UseTimings"), type_name: "BooleanValue" },
];
static CHILDREN_SHOW_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_Empty/p:present", property_name: None },
    ChildInfo { name: "p:CT_ShowInfoBrowse/p:browse", property_name: None },
    ChildInfo { name: "p:CT_ShowInfoKiosk/p:kiosk", property_name: None },
    ChildInfo { name: "p:CT_Empty/p:sldAll", property_name: None },
    ChildInfo { name: "p:CT_IndexRange/p:sldRg", property_name: None },
    ChildInfo { name: "p:CT_CustomShowId/p:custShow", property_name: None },
    ChildInfo { name: "a:CT_Color/p:penClr", property_name: None },
    ChildInfo { name: "p:CT_ShowPropertiesExtensionList/p:extLst", property_name: None },
];
static CHILDREN_COLOR_MOST_RECENTLY_USED: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static CHILDREN_PRESENTATION_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_PresentationPropertiesExtension/p:ext", property_name: None },
];
static ATTRS_PRESENTATION_PROPERTIES_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PRESENTATION_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p14:CT_DiscardImageEditData/p14:discardImageEditData", property_name: Some("DiscardImageEditData") },
    ChildInfo { name: "p14:CT_DefaultImageDpi/p14:defaultImageDpi", property_name: Some("DefaultImageDpi") },
    ChildInfo { name: "a14:CT_TextMath/a14:m", property_name: Some("TextMath") },
    ChildInfo { name: "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased", property_name: Some("ChartTrackingReferenceBased") },
];
static ATTRS_HEADER_FOOTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sldNum", property_name: Some("SlideNumber"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hdr", property_name: Some("Header"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":ftr", property_name: Some("Footer"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dt", property_name: Some("DateTime"), type_name: "BooleanValue" },
];
static CHILDREN_HEADER_FOOTER: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_ExtensionListModify/p:extLst", property_name: Some("ExtensionListWithModification") },
];
static CHILDREN_SLIDE_LAYOUT_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideLayoutExtension/p:ext", property_name: None },
];
static CHILDREN_SLIDE_LAYOUT_ID_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideLayoutIdListEntry/p:sldLayoutId", property_name: None },
];
static CHILDREN_TEXT_STYLES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextListStyle/p:titleStyle", property_name: Some("TitleStyle") },
    ChildInfo { name: "a:CT_TextListStyle/p:bodyStyle", property_name: Some("BodyStyle") },
    ChildInfo { name: "a:CT_TextListStyle/p:otherStyle", property_name: Some("OtherStyle") },
    ChildInfo { name: "p:CT_ExtensionList/p:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_SLIDE_MASTER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_SlideMasterExtension/p:ext", property_name: None },
];
static CHILDREN_HANDOUT_MASTER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_HandoutMasterExtension/p:ext", property_name: None },
];
static CHILDREN_NOTES_MASTER_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_NotesMasterExtension/p:ext", property_name: None },
];
static ATTRS_OLE_CHART_ELEMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "UInt32Value" },
];
static CHILDREN_TEXT_ELEMENT: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_IndexRange/p:charRg", property_name: Some("CharRange") },
    ChildInfo { name: "p:CT_IndexRange/p:pRg", property_name: Some("ParagraphIndexRange") },
];
static CHILDREN_GRAPHIC_ELEMENT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AnimationDgmElement/a:dgm", property_name: Some("Diagram") },
    ChildInfo { name: "a:CT_AnimationChartElement/a:chart", property_name: Some("Chart") },
];
static ATTRS_BLINDS_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_CHECKER_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_COMB_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_RANDOM_BAR_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_COVER_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "StringValue" },
];
static ATTRS_PULL_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "StringValue" },
];
static ATTRS_CUT_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":thruBlk", property_name: Some("ThroughBlack"), type_name: "BooleanValue" },
];
static ATTRS_FADE_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":thruBlk", property_name: Some("ThroughBlack"), type_name: "BooleanValue" },
];
static ATTRS_PUSH_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_WIPE_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_SPLIT_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":orient", property_name: Some("Orientation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_STRIPS_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static ATTRS_WHEEL_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spokes", property_name: Some("Spokes"), type_name: "UInt32Value" },
];
static ATTRS_ZOOM_TRANSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static CHILDREN_SOUND_ACTION: &[ChildInfo] = &[
    ChildInfo { name: "p:CT_TransitionStartSoundAction/p:stSnd", property_name: Some("StartSoundAction") },
    ChildInfo { name: "p:CT_Empty/p:endSnd", property_name: Some("EndSoundAction") },
];
static CHILDREN_PLACEHOLDER_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "p232:CT_PlaceholderTypeExtension/p232:phTypeExt", property_name: Some("PlaceholderTypeExtension") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "SlideAll", local_name: "sldAll", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "PresenterSlideMode", local_name: "present", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "EndSoundAction", local_name: "endSnd", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BuildAsOne", local_name: "bldAsOne", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SlideTarget", local_name: "sldTgt", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BackgroundAnimation", local_name: "bg", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "CircleTransition", local_name: "circle", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DissolveTransition", local_name: "dissolve", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DiamondTransition", local_name: "diamond", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NewsflashTransition", local_name: "newsflash", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "PlusTransition", local_name: "plus", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "RandomTransition", local_name: "random", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "WedgeTransition", local_name: "wedge", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SlideRange", local_name: "sldRg", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLIDE_RANGE, children: &[] },
    ElementInfo { class_name: "CharRange", local_name: "charRg", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHAR_RANGE, children: &[] },
    ElementInfo { class_name: "ParagraphIndexRange", local_name: "pRg", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PARAGRAPH_INDEX_RANGE, children: &[] },
    ElementInfo { class_name: "CustomShowReference", local_name: "custShow", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOM_SHOW_REFERENCE, children: &[] },
    ElementInfo { class_name: "Extension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTENSION, children: &[] },
    ElementInfo { class_name: "BrowseSlideMode", local_name: "browse", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BROWSE_SLIDE_MODE, children: &[] },
    ElementInfo { class_name: "KioskSlideMode", local_name: "kiosk", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_KIOSK_SLIDE_MODE, children: &[] },
    ElementInfo { class_name: "ColorMap", local_name: "clrMap", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_MAP, children: CHILDREN_COLOR_MAP },
    ElementInfo { class_name: "ColorMapOverride", local_name: "clrMapOvr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_MAP_OVERRIDE },
    ElementInfo { class_name: "BackgroundProperties", local_name: "bgPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKGROUND_PROPERTIES, children: CHILDREN_BACKGROUND_PROPERTIES },
    ElementInfo { class_name: "BackgroundStyleReference", local_name: "bgRef", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKGROUND_STYLE_REFERENCE, children: CHILDREN_BACKGROUND_STYLE_REFERENCE },
    ElementInfo { class_name: "CommentPropertiesExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "CommentAuthorList", local_name: "cmAuthorLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_AUTHOR_LIST },
    ElementInfo { class_name: "CommentList", local_name: "cmLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_LIST },
    ElementInfo { class_name: "OleObject", local_name: "oleObj", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLE_OBJECT, children: CHILDREN_OLE_OBJECT },
    ElementInfo { class_name: "Presentation", local_name: "presentation", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESENTATION, children: CHILDREN_PRESENTATION },
    ElementInfo { class_name: "PresentationProperties", local_name: "presentationPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRESENTATION_PROPERTIES },
    ElementInfo { class_name: "Slide", local_name: "sld", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE, children: CHILDREN_SLIDE },
    ElementInfo { class_name: "SlideLayout", local_name: "sldLayout", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_LAYOUT, children: CHILDREN_SLIDE_LAYOUT },
    ElementInfo { class_name: "SlideMaster", local_name: "sldMaster", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_MASTER, children: CHILDREN_SLIDE_MASTER },
    ElementInfo { class_name: "HandoutMaster", local_name: "handoutMaster", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HANDOUT_MASTER },
    ElementInfo { class_name: "NotesMaster", local_name: "notesMaster", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NOTES_MASTER },
    ElementInfo { class_name: "NotesSlide", local_name: "notes", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NOTES_SLIDE, children: CHILDREN_NOTES_SLIDE },
    ElementInfo { class_name: "SlideSyncProperties", local_name: "sldSyncPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_SYNC_PROPERTIES, children: CHILDREN_SLIDE_SYNC_PROPERTIES },
    ElementInfo { class_name: "TagList", local_name: "tagLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TAG_LIST },
    ElementInfo { class_name: "ViewProperties", local_name: "viewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VIEW_PROPERTIES, children: CHILDREN_VIEW_PROPERTIES },
    ElementInfo { class_name: "ContentPart", local_name: "contentPart", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTENT_PART, children: CHILDREN_CONTENT_PART },
    ElementInfo { class_name: "Sound", local_name: "snd", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SOUND, children: &[] },
    ElementInfo { class_name: "SoundTarget", local_name: "sndTgt", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SOUND_TARGET, children: &[] },
    ElementInfo { class_name: "StartSoundAction", local_name: "stSnd", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_START_SOUND_ACTION, children: CHILDREN_START_SOUND_ACTION },
    ElementInfo { class_name: "TimeAbsolute", local_name: "tmAbs", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIME_ABSOLUTE, children: &[] },
    ElementInfo { class_name: "TimePercentage", local_name: "tmPct", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIME_PERCENTAGE, children: &[] },
    ElementInfo { class_name: "TargetElement", local_name: "tgtEl", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TARGET_ELEMENT },
    ElementInfo { class_name: "TimeNode", local_name: "tn", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIME_NODE, children: &[] },
    ElementInfo { class_name: "RuntimeNodeTrigger", local_name: "rtn", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RUNTIME_NODE_TRIGGER, children: &[] },
    ElementInfo { class_name: "Condition", local_name: "cond", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONDITION, children: CHILDREN_CONDITION },
    ElementInfo { class_name: "EndSync", local_name: "endSync", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_END_SYNC, children: CHILDREN_END_SYNC },
    ElementInfo { class_name: "ParallelTimeNode", local_name: "par", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PARALLEL_TIME_NODE },
    ElementInfo { class_name: "SequenceTimeNode", local_name: "seq", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SEQUENCE_TIME_NODE, children: CHILDREN_SEQUENCE_TIME_NODE },
    ElementInfo { class_name: "ExclusiveTimeNode", local_name: "excl", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXCLUSIVE_TIME_NODE },
    ElementInfo { class_name: "Animate", local_name: "anim", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANIMATE, children: CHILDREN_ANIMATE },
    ElementInfo { class_name: "AnimateColor", local_name: "animClr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANIMATE_COLOR, children: CHILDREN_ANIMATE_COLOR },
    ElementInfo { class_name: "AnimateEffect", local_name: "animEffect", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANIMATE_EFFECT, children: CHILDREN_ANIMATE_EFFECT },
    ElementInfo { class_name: "AnimateMotion", local_name: "animMotion", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANIMATE_MOTION, children: CHILDREN_ANIMATE_MOTION },
    ElementInfo { class_name: "AnimateRotation", local_name: "animRot", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANIMATE_ROTATION, children: CHILDREN_ANIMATE_ROTATION },
    ElementInfo { class_name: "AnimateScale", local_name: "animScale", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANIMATE_SCALE, children: CHILDREN_ANIMATE_SCALE },
    ElementInfo { class_name: "Command", local_name: "cmd", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMAND, children: CHILDREN_COMMAND },
    ElementInfo { class_name: "SetBehavior", local_name: "set", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SET_BEHAVIOR },
    ElementInfo { class_name: "Audio", local_name: "audio", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AUDIO, children: CHILDREN_AUDIO },
    ElementInfo { class_name: "Video", local_name: "video", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VIDEO, children: CHILDREN_VIDEO },
    ElementInfo { class_name: "CommonTimeNode", local_name: "cTn", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMON_TIME_NODE, children: CHILDREN_COMMON_TIME_NODE },
    ElementInfo { class_name: "PreviousConditionList", local_name: "prevCondLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PREVIOUS_CONDITION_LIST },
    ElementInfo { class_name: "NextConditionList", local_name: "nextCondLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NEXT_CONDITION_LIST },
    ElementInfo { class_name: "StartConditionList", local_name: "stCondLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_START_CONDITION_LIST },
    ElementInfo { class_name: "EndConditionList", local_name: "endCondLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_END_CONDITION_LIST },
    ElementInfo { class_name: "AttributeName", local_name: "attrName", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Text", local_name: "text", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AttributeNameList", local_name: "attrNameLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ATTRIBUTE_NAME_LIST },
    ElementInfo { class_name: "BooleanVariantValue", local_name: "boolVal", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOOLEAN_VARIANT_VALUE, children: &[] },
    ElementInfo { class_name: "IntegerVariantValue", local_name: "intVal", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INTEGER_VARIANT_VALUE, children: &[] },
    ElementInfo { class_name: "FloatVariantValue", local_name: "fltVal", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FLOAT_VARIANT_VALUE, children: &[] },
    ElementInfo { class_name: "StringVariantValue", local_name: "strVal", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STRING_VARIANT_VALUE, children: &[] },
    ElementInfo { class_name: "ColorValue", local_name: "clrVal", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_VALUE },
    ElementInfo { class_name: "PenColor", local_name: "penClr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PEN_COLOR },
    ElementInfo { class_name: "TimeAnimateValue", local_name: "tav", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TIME_ANIMATE_VALUE, children: CHILDREN_TIME_ANIMATE_VALUE },
    ElementInfo { class_name: "RgbColor", local_name: "rgb", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RGB_COLOR, children: &[] },
    ElementInfo { class_name: "HslColor", local_name: "hsl", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HSL_COLOR, children: &[] },
    ElementInfo { class_name: "CommonBehavior", local_name: "cBhvr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMON_BEHAVIOR, children: CHILDREN_COMMON_BEHAVIOR },
    ElementInfo { class_name: "Progress", local_name: "progress", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PROGRESS },
    ElementInfo { class_name: "ToVariantValue", local_name: "to", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TO_VARIANT_VALUE },
    ElementInfo { class_name: "VariantValue", local_name: "val", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VARIANT_VALUE },
    ElementInfo { class_name: "CommonMediaNode", local_name: "cMediaNode", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMON_MEDIA_NODE, children: CHILDREN_COMMON_MEDIA_NODE },
    ElementInfo { class_name: "TimeNodeList", local_name: "tnLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIME_NODE_LIST },
    ElementInfo { class_name: "Template", local_name: "tmpl", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEMPLATE, children: CHILDREN_TEMPLATE },
    ElementInfo { class_name: "TemplateList", local_name: "tmplLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEMPLATE_LIST },
    ElementInfo { class_name: "BuildSubElement", local_name: "bldSub", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUILD_SUB_ELEMENT },
    ElementInfo { class_name: "BuildParagraph", local_name: "bldP", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BUILD_PARAGRAPH, children: CHILDREN_BUILD_PARAGRAPH },
    ElementInfo { class_name: "BuildDiagram", local_name: "bldDgm", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUILD_DIAGRAM, children: &[] },
    ElementInfo { class_name: "BuildOleChart", local_name: "bldOleChart", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUILD_OLE_CHART, children: &[] },
    ElementInfo { class_name: "BuildGraphics", local_name: "bldGraphic", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BUILD_GRAPHICS, children: CHILDREN_BUILD_GRAPHICS },
    ElementInfo { class_name: "BuildList", local_name: "bldLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BUILD_LIST },
    ElementInfo { class_name: "ExtensionListWithModification", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTENSION_LIST_WITH_MODIFICATION, children: CHILDREN_EXTENSION_LIST_WITH_MODIFICATION },
    ElementInfo { class_name: "ByColor", local_name: "by", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BY_COLOR },
    ElementInfo { class_name: "FromColor", local_name: "from", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FROM_COLOR },
    ElementInfo { class_name: "ToColor", local_name: "to", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TO_COLOR },
    ElementInfo { class_name: "SlideListEntry", local_name: "sld", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLIDE_LIST_ENTRY, children: &[] },
    ElementInfo { class_name: "CustomerData", local_name: "custData", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOMER_DATA, children: &[] },
    ElementInfo { class_name: "CustomerDataTags", local_name: "tags", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUSTOMER_DATA_TAGS, children: &[] },
    ElementInfo { class_name: "CommentAuthor", local_name: "cmAuthor", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT_AUTHOR, children: CHILDREN_COMMENT_AUTHOR },
    ElementInfo { class_name: "Comment", local_name: "cm", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT, children: CHILDREN_COMMENT },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "Control", local_name: "control", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTROL, children: CHILDREN_CONTROL },
    ElementInfo { class_name: "SlideId", local_name: "sldId", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_ID, children: CHILDREN_SLIDE_ID },
    ElementInfo { class_name: "SlideMasterId", local_name: "sldMasterId", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_MASTER_ID, children: CHILDREN_SLIDE_MASTER_ID },
    ElementInfo { class_name: "NotesMasterId", local_name: "notesMasterId", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NOTES_MASTER_ID, children: CHILDREN_NOTES_MASTER_ID },
    ElementInfo { class_name: "HandoutMasterId", local_name: "handoutMasterId", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HANDOUT_MASTER_ID, children: CHILDREN_HANDOUT_MASTER_ID },
    ElementInfo { class_name: "Font", local_name: "font", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FONT, children: &[] },
    ElementInfo { class_name: "RegularFont", local_name: "regular", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REGULAR_FONT, children: &[] },
    ElementInfo { class_name: "BoldFont", local_name: "bold", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOLD_FONT, children: &[] },
    ElementInfo { class_name: "ItalicFont", local_name: "italic", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ITALIC_FONT, children: &[] },
    ElementInfo { class_name: "BoldItalicFont", local_name: "boldItalic", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOLD_ITALIC_FONT, children: &[] },
    ElementInfo { class_name: "EmbeddedFont", local_name: "embeddedFont", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EMBEDDED_FONT },
    ElementInfo { class_name: "SlideList", local_name: "sldLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_LIST },
    ElementInfo { class_name: "CustomShow", local_name: "custShow", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CUSTOM_SHOW, children: CHILDREN_CUSTOM_SHOW },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualShapeDrawingProperties", local_name: "cNvSpPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_SHAPE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "ApplicationNonVisualDrawingProperties", local_name: "nvPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualShapeProperties", local_name: "nvSpPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeStyle", local_name: "style", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_STYLE },
    ElementInfo { class_name: "TextBody", local_name: "txBody", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_BODY },
    ElementInfo { class_name: "NonVisualConnectorShapeDrawingProperties", local_name: "cNvCxnSpPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualConnectionShapeProperties", local_name: "nvCxnSpPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureDrawingProperties", local_name: "cNvPicPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureProperties", local_name: "nvPicPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_PICTURE_PROPERTIES },
    ElementInfo { class_name: "BlipFill", local_name: "blipFill", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP_FILL, children: CHILDREN_BLIP_FILL },
    ElementInfo { class_name: "NonVisualGraphicFrameDrawingProperties", local_name: "cNvGraphicFramePr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualGraphicFrameProperties", local_name: "nvGraphicFramePr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES },
    ElementInfo { class_name: "Transform", local_name: "xfrm", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM, children: CHILDREN_TRANSFORM },
    ElementInfo { class_name: "NonVisualGroupShapeDrawingProperties", local_name: "cNvGrpSpPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "TitleStyle", local_name: "titleStyle", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TITLE_STYLE },
    ElementInfo { class_name: "BodyStyle", local_name: "bodyStyle", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BODY_STYLE },
    ElementInfo { class_name: "OtherStyle", local_name: "otherStyle", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OTHER_STYLE },
    ElementInfo { class_name: "DefaultTextStyle", local_name: "defaultTextStyle", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DEFAULT_TEXT_STYLE },
    ElementInfo { class_name: "NotesStyle", local_name: "notesStyle", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NOTES_STYLE },
    ElementInfo { class_name: "SlideLayoutId", local_name: "sldLayoutId", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_LAYOUT_ID, children: CHILDREN_SLIDE_LAYOUT_ID },
    ElementInfo { class_name: "CommonSlideData", local_name: "cSld", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMON_SLIDE_DATA, children: CHILDREN_COMMON_SLIDE_DATA },
    ElementInfo { class_name: "Tag", local_name: "tag", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TAG, children: &[] },
    ElementInfo { class_name: "RestoredLeft", local_name: "restoredLeft", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RESTORED_LEFT, children: &[] },
    ElementInfo { class_name: "RestoredTop", local_name: "restoredTop", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RESTORED_TOP, children: &[] },
    ElementInfo { class_name: "ScaleFactor", local_name: "scale", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCALE_FACTOR },
    ElementInfo { class_name: "Origin", local_name: "origin", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ORIGIN, children: &[] },
    ElementInfo { class_name: "Position", local_name: "pos", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POSITION, children: &[] },
    ElementInfo { class_name: "CommonViewProperties", local_name: "cViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMON_VIEW_PROPERTIES, children: CHILDREN_COMMON_VIEW_PROPERTIES },
    ElementInfo { class_name: "OutlineViewSlideListEntry", local_name: "sld", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OUTLINE_VIEW_SLIDE_LIST_ENTRY, children: &[] },
    ElementInfo { class_name: "OutlineViewSlideList", local_name: "sldLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OUTLINE_VIEW_SLIDE_LIST },
    ElementInfo { class_name: "Guide", local_name: "guide", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GUIDE, children: &[] },
    ElementInfo { class_name: "GuideList", local_name: "guideLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GUIDE_LIST },
    ElementInfo { class_name: "CommonSlideViewProperties", local_name: "cSldViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMON_SLIDE_VIEW_PROPERTIES, children: CHILDREN_COMMON_SLIDE_VIEW_PROPERTIES },
    ElementInfo { class_name: "NormalViewProperties", local_name: "normalViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NORMAL_VIEW_PROPERTIES, children: CHILDREN_NORMAL_VIEW_PROPERTIES },
    ElementInfo { class_name: "SlideViewProperties", local_name: "slideViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_VIEW_PROPERTIES },
    ElementInfo { class_name: "OutlineViewProperties", local_name: "outlineViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OUTLINE_VIEW_PROPERTIES },
    ElementInfo { class_name: "NotesTextViewProperties", local_name: "notesTextViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NOTES_TEXT_VIEW_PROPERTIES },
    ElementInfo { class_name: "SorterViewProperties", local_name: "sorterViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SORTER_VIEW_PROPERTIES, children: CHILDREN_SORTER_VIEW_PROPERTIES },
    ElementInfo { class_name: "NotesViewProperties", local_name: "notesViewPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NOTES_VIEW_PROPERTIES },
    ElementInfo { class_name: "GridSpacing", local_name: "gridSpacing", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GRID_SPACING, children: &[] },
    ElementInfo { class_name: "NotesSize", local_name: "notesSz", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NOTES_SIZE, children: &[] },
    ElementInfo { class_name: "SlideExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_EXTENSION, children: CHILDREN_SLIDE_EXTENSION },
    ElementInfo { class_name: "CommonSlideDataExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMON_SLIDE_DATA_EXTENSION, children: CHILDREN_COMMON_SLIDE_DATA_EXTENSION },
    ElementInfo { class_name: "ShowPropertiesExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHOW_PROPERTIES_EXTENSION, children: CHILDREN_SHOW_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "Picture", local_name: "pic", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PICTURE },
    ElementInfo { class_name: "OleObjectEmbed", local_name: "embed", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLE_OBJECT_EMBED, children: CHILDREN_OLE_OBJECT_EMBED },
    ElementInfo { class_name: "OleObjectLink", local_name: "link", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLE_OBJECT_LINK, children: CHILDREN_OLE_OBJECT_LINK },
    ElementInfo { class_name: "Transition", local_name: "transition", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSITION, children: CHILDREN_TRANSITION },
    ElementInfo { class_name: "Timing", local_name: "timing", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIMING },
    ElementInfo { class_name: "SlideExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_EXTENSION_LIST },
    ElementInfo { class_name: "Background", local_name: "bg", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKGROUND, children: CHILDREN_BACKGROUND },
    ElementInfo { class_name: "ShapeTree", local_name: "spTree", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_TREE },
    ElementInfo { class_name: "GroupShape", local_name: "grpSp", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_SHAPE },
    ElementInfo { class_name: "CustomerDataList", local_name: "custDataLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOMER_DATA_LIST },
    ElementInfo { class_name: "ControlList", local_name: "controls", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONTROL_LIST },
    ElementInfo { class_name: "CommonSlideDataExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMON_SLIDE_DATA_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualGroupShapeProperties", local_name: "nvGrpSpPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "GroupShapeProperties", local_name: "grpSpPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_PROPERTIES, children: CHILDREN_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Shape", local_name: "sp", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE, children: CHILDREN_SHAPE },
    ElementInfo { class_name: "GraphicFrame", local_name: "graphicFrame", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRAPHIC_FRAME },
    ElementInfo { class_name: "ConnectionShape", local_name: "cxnSp", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONNECTION_SHAPE },
    ElementInfo { class_name: "ShowPropertiesExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHOW_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "ShapeTarget", local_name: "spTgt", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_TARGET, children: CHILDREN_SHAPE_TARGET },
    ElementInfo { class_name: "InkTarget", local_name: "inkTgt", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INK_TARGET, children: &[] },
    ElementInfo { class_name: "SubShape", local_name: "subSp", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SUB_SHAPE, children: &[] },
    ElementInfo { class_name: "CommentAuthorExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT_AUTHOR_EXTENSION, children: CHILDREN_COMMENT_AUTHOR_EXTENSION },
    ElementInfo { class_name: "CommentExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMMENT_EXTENSION, children: CHILDREN_COMMENT_EXTENSION },
    ElementInfo { class_name: "SlideLayoutExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_LAYOUT_EXTENSION, children: CHILDREN_SLIDE_LAYOUT_EXTENSION },
    ElementInfo { class_name: "SlideMasterExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SLIDE_MASTER_EXTENSION, children: CHILDREN_SLIDE_MASTER_EXTENSION },
    ElementInfo { class_name: "HandoutMasterExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HANDOUT_MASTER_EXTENSION, children: CHILDREN_HANDOUT_MASTER_EXTENSION },
    ElementInfo { class_name: "NotesMasterExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NOTES_MASTER_EXTENSION, children: CHILDREN_NOTES_MASTER_EXTENSION },
    ElementInfo { class_name: "PlaceholderShape", local_name: "ph", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PLACEHOLDER_SHAPE, children: CHILDREN_PLACEHOLDER_SHAPE },
    ElementInfo { class_name: "ApplicationNonVisualDrawingPropertiesExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "ApplicationNonVisualDrawingPropertiesExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION, children: CHILDREN_APPLICATION_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "Iterate", local_name: "iterate", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ITERATE, children: CHILDREN_ITERATE },
    ElementInfo { class_name: "ChildTimeNodeList", local_name: "childTnLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHILD_TIME_NODE_LIST },
    ElementInfo { class_name: "SubTimeNodeList", local_name: "subTnLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SUB_TIME_NODE_LIST },
    ElementInfo { class_name: "TimeAnimateValueList", local_name: "tavLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TIME_ANIMATE_VALUE_LIST },
    ElementInfo { class_name: "ByPosition", local_name: "by", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BY_POSITION, children: &[] },
    ElementInfo { class_name: "FromPosition", local_name: "from", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FROM_POSITION, children: &[] },
    ElementInfo { class_name: "ToPosition", local_name: "to", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TO_POSITION, children: &[] },
    ElementInfo { class_name: "RotationCenter", local_name: "rCtr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROTATION_CENTER, children: &[] },
    ElementInfo { class_name: "CommentAuthorExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_AUTHOR_EXTENSION_LIST },
    ElementInfo { class_name: "CommentExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENT_EXTENSION_LIST },
    ElementInfo { class_name: "SlideMasterIdList", local_name: "sldMasterIdLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_MASTER_ID_LIST },
    ElementInfo { class_name: "NotesMasterIdList", local_name: "notesMasterIdLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NOTES_MASTER_ID_LIST },
    ElementInfo { class_name: "HandoutMasterIdList", local_name: "handoutMasterIdLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HANDOUT_MASTER_ID_LIST },
    ElementInfo { class_name: "SlideIdList", local_name: "sldIdLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_ID_LIST },
    ElementInfo { class_name: "SlideSize", local_name: "sldSz", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLIDE_SIZE, children: &[] },
    ElementInfo { class_name: "EmbeddedFontList", local_name: "embeddedFontLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EMBEDDED_FONT_LIST },
    ElementInfo { class_name: "CustomShowList", local_name: "custShowLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_SHOW_LIST },
    ElementInfo { class_name: "PhotoAlbum", local_name: "photoAlbum", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PHOTO_ALBUM, children: CHILDREN_PHOTO_ALBUM },
    ElementInfo { class_name: "Kinsoku", local_name: "kinsoku", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_KINSOKU, children: &[] },
    ElementInfo { class_name: "ModificationVerifier", local_name: "modifyVerifier", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFICATION_VERIFIER, children: &[] },
    ElementInfo { class_name: "PresentationExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRESENTATION_EXTENSION_LIST },
    ElementInfo { class_name: "PresentationExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESENTATION_EXTENSION, children: CHILDREN_PRESENTATION_EXTENSION },
    ElementInfo { class_name: "HtmlPublishProperties", local_name: "htmlPubPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HTML_PUBLISH_PROPERTIES, children: CHILDREN_HTML_PUBLISH_PROPERTIES },
    ElementInfo { class_name: "WebProperties", local_name: "webPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WEB_PROPERTIES, children: CHILDREN_WEB_PROPERTIES },
    ElementInfo { class_name: "PrintingProperties", local_name: "prnPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRINTING_PROPERTIES, children: CHILDREN_PRINTING_PROPERTIES },
    ElementInfo { class_name: "ShowProperties", local_name: "showPr", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHOW_PROPERTIES, children: CHILDREN_SHOW_PROPERTIES },
    ElementInfo { class_name: "ColorMostRecentlyUsed", local_name: "clrMru", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_MOST_RECENTLY_USED },
    ElementInfo { class_name: "PresentationPropertiesExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRESENTATION_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "PresentationPropertiesExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESENTATION_PROPERTIES_EXTENSION, children: CHILDREN_PRESENTATION_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "HeaderFooter", local_name: "hf", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HEADER_FOOTER, children: CHILDREN_HEADER_FOOTER },
    ElementInfo { class_name: "SlideLayoutExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_LAYOUT_EXTENSION_LIST },
    ElementInfo { class_name: "SlideLayoutIdList", local_name: "sldLayoutIdLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_LAYOUT_ID_LIST },
    ElementInfo { class_name: "TextStyles", local_name: "txStyles", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_STYLES },
    ElementInfo { class_name: "SlideMasterExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_MASTER_EXTENSION_LIST },
    ElementInfo { class_name: "HandoutMasterExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HANDOUT_MASTER_EXTENSION_LIST },
    ElementInfo { class_name: "NotesMasterExtensionList", local_name: "extLst", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NOTES_MASTER_EXTENSION_LIST },
    ElementInfo { class_name: "OleChartElement", local_name: "oleChartEl", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OLE_CHART_ELEMENT, children: &[] },
    ElementInfo { class_name: "TextElement", local_name: "txEl", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_ELEMENT },
    ElementInfo { class_name: "GraphicElement", local_name: "graphicEl", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRAPHIC_ELEMENT },
    ElementInfo { class_name: "BlindsTransition", local_name: "blinds", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BLINDS_TRANSITION, children: &[] },
    ElementInfo { class_name: "CheckerTransition", local_name: "checker", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHECKER_TRANSITION, children: &[] },
    ElementInfo { class_name: "CombTransition", local_name: "comb", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMB_TRANSITION, children: &[] },
    ElementInfo { class_name: "RandomBarTransition", local_name: "randomBar", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RANDOM_BAR_TRANSITION, children: &[] },
    ElementInfo { class_name: "CoverTransition", local_name: "cover", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COVER_TRANSITION, children: &[] },
    ElementInfo { class_name: "PullTransition", local_name: "pull", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PULL_TRANSITION, children: &[] },
    ElementInfo { class_name: "CutTransition", local_name: "cut", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CUT_TRANSITION, children: &[] },
    ElementInfo { class_name: "FadeTransition", local_name: "fade", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FADE_TRANSITION, children: &[] },
    ElementInfo { class_name: "PushTransition", local_name: "push", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PUSH_TRANSITION, children: &[] },
    ElementInfo { class_name: "WipeTransition", local_name: "wipe", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WIPE_TRANSITION, children: &[] },
    ElementInfo { class_name: "SplitTransition", local_name: "split", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SPLIT_TRANSITION, children: &[] },
    ElementInfo { class_name: "StripsTransition", local_name: "strips", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STRIPS_TRANSITION, children: &[] },
    ElementInfo { class_name: "WheelTransition", local_name: "wheel", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WHEEL_TRANSITION, children: &[] },
    ElementInfo { class_name: "ZoomTransition", local_name: "zoom", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ZOOM_TRANSITION, children: &[] },
    ElementInfo { class_name: "SoundAction", local_name: "sndAc", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SOUND_ACTION },
    ElementInfo { class_name: "PlaceholderExtension", local_name: "ext", prefix: "p", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PLACEHOLDER_EXTENSION },
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

/// Create a `<p:sldAll>` element (`SlideAll`).
pub fn slide_all() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldAll")
}

/// Create a `<p:present>` element (`PresenterSlideMode`).
pub fn presenter_slide_mode() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "present")
}

/// Create a `<p:endSnd>` element (`EndSoundAction`).
pub fn end_sound_action() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "endSnd")
}

/// Create a `<p:bldAsOne>` element (`BuildAsOne`).
pub fn build_as_one() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bldAsOne")
}

/// Create a `<p:sldTgt>` element (`SlideTarget`).
pub fn slide_target() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldTgt")
}

/// Create a `<p:bg>` element (`BackgroundAnimation`).
pub fn background_animation() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bg")
}

/// Create a `<p:circle>` element (`CircleTransition`).
pub fn circle_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "circle")
}

/// Create a `<p:dissolve>` element (`DissolveTransition`).
pub fn dissolve_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "dissolve")
}

/// Create a `<p:diamond>` element (`DiamondTransition`).
pub fn diamond_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "diamond")
}

/// Create a `<p:newsflash>` element (`NewsflashTransition`).
pub fn newsflash_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "newsflash")
}

/// Create a `<p:plus>` element (`PlusTransition`).
pub fn plus_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "plus")
}

/// Create a `<p:random>` element (`RandomTransition`).
pub fn random_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "random")
}

/// Create a `<p:wedge>` element (`WedgeTransition`).
pub fn wedge_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "wedge")
}

/// Create a `<p:sldRg>` element (`SlideRange`).
pub fn slide_range() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldRg")
}

/// Set `Start` (`:st`) on a `SlideRange` element.
pub fn slide_range_with_start(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("st", value);
    el
}

/// Set `End` (`:end`) on a `SlideRange` element.
pub fn slide_range_with_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("end", value);
    el
}

/// Create a `<p:charRg>` element (`CharRange`).
pub fn char_range() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "charRg")
}

/// Set `Start` (`:st`) on a `CharRange` element.
pub fn char_range_with_start(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("st", value);
    el
}

/// Set `End` (`:end`) on a `CharRange` element.
pub fn char_range_with_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("end", value);
    el
}

/// Create a `<p:pRg>` element (`ParagraphIndexRange`).
pub fn paragraph_index_range() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "pRg")
}

/// Set `Start` (`:st`) on a `ParagraphIndexRange` element.
pub fn paragraph_index_range_with_start(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("st", value);
    el
}

/// Set `End` (`:end`) on a `ParagraphIndexRange` element.
pub fn paragraph_index_range_with_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("end", value);
    el
}

/// Create a `<p:custShow>` element (`CustomShowReference`).
pub fn custom_show_reference() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "custShow")
}

/// Set `Id` (`:id`) on a `CustomShowReference` element.
pub fn custom_show_reference_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Create `<p:custShow>` with `Id` set.
pub fn custom_show_reference_id(value: impl Into<String>) -> OpenXmlElement {
    custom_show_reference_with_id(custom_show_reference(), value)
}

/// Create a `<p:ext>` element (`Extension`).
pub fn extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Set `Uri` (`:uri`) on a `Extension` element.
pub fn extension_with_uri(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("uri", value);
    el
}

/// Create a `<p:browse>` element (`BrowseSlideMode`).
pub fn browse_slide_mode() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "browse")
}

/// Set `ShowScrollbar` (`:showScrollbar`) on a `BrowseSlideMode` element.
pub fn browse_slide_mode_with_show_scrollbar(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showScrollbar", value);
    el
}

/// Create `<p:browse>` with `ShowScrollbar` set.
pub fn browse_slide_mode_show_scrollbar(value: impl Into<String>) -> OpenXmlElement {
    browse_slide_mode_with_show_scrollbar(browse_slide_mode(), value)
}

/// Create a `<p:kiosk>` element (`KioskSlideMode`).
pub fn kiosk_slide_mode() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "kiosk")
}

/// Set `Restart` (`:restart`) on a `KioskSlideMode` element.
pub fn kiosk_slide_mode_with_restart(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("restart", value);
    el
}

/// Create `<p:kiosk>` with `Restart` set.
pub fn kiosk_slide_mode_restart(value: impl Into<String>) -> OpenXmlElement {
    kiosk_slide_mode_with_restart(kiosk_slide_mode(), value)
}

/// Create a `<p:clrMap>` element (`ColorMap`).
pub fn color_map(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "clrMap").with_children(children)
}

/// Set `Background1` (`:bg1`) on a `ColorMap` element.
pub fn color_map_with_background1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bg1", value);
    el
}

/// Set `Text1` (`:tx1`) on a `ColorMap` element.
pub fn color_map_with_text1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx1", value);
    el
}

/// Set `Background2` (`:bg2`) on a `ColorMap` element.
pub fn color_map_with_background2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bg2", value);
    el
}

/// Set `Text2` (`:tx2`) on a `ColorMap` element.
pub fn color_map_with_text2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx2", value);
    el
}

/// Set `Accent1` (`:accent1`) on a `ColorMap` element.
pub fn color_map_with_accent1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent1", value);
    el
}

/// Set `Accent2` (`:accent2`) on a `ColorMap` element.
pub fn color_map_with_accent2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent2", value);
    el
}

/// Set `Accent3` (`:accent3`) on a `ColorMap` element.
pub fn color_map_with_accent3(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent3", value);
    el
}

/// Set `Accent4` (`:accent4`) on a `ColorMap` element.
pub fn color_map_with_accent4(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent4", value);
    el
}

/// Set `Accent5` (`:accent5`) on a `ColorMap` element.
pub fn color_map_with_accent5(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent5", value);
    el
}

/// Set `Accent6` (`:accent6`) on a `ColorMap` element.
pub fn color_map_with_accent6(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent6", value);
    el
}

/// Set `Hyperlink` (`:hlink`) on a `ColorMap` element.
pub fn color_map_with_hyperlink(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hlink", value);
    el
}

/// Set `FollowedHyperlink` (`:folHlink`) on a `ColorMap` element.
pub fn color_map_with_followed_hyperlink(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("folHlink", value);
    el
}

/// Create a `<p:clrMapOvr>` element (`ColorMapOverride`).
pub fn color_map_override(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "clrMapOvr").with_children(children)
}

/// Create a `<p:bgPr>` element (`BackgroundProperties`).
pub fn background_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bgPr").with_children(children)
}

/// Set `ShadeToTitle` (`:shadeToTitle`) on a `BackgroundProperties` element.
pub fn background_properties_with_shade_to_title(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("shadeToTitle", value);
    el
}

/// Create a `<p:bgRef>` element (`BackgroundStyleReference`).
pub fn background_style_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bgRef").with_children(children)
}

/// Set `Index` (`:idx`) on a `BackgroundStyleReference` element.
pub fn background_style_reference_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Create a `<p:ext>` element (`CommentPropertiesExtension`).
pub fn comment_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:cmAuthorLst>` element (`CommentAuthorList`).
pub fn comment_author_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cmAuthorLst").with_children(children)
}

/// Create a `<p:cmLst>` element (`CommentList`).
pub fn comment_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cmLst").with_children(children)
}

/// Create a `<p:oleObj>` element (`OleObject`).
pub fn ole_object(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "oleObj").with_children(children)
}

/// Set `ShapeId` (`:spid`) on a `OleObject` element.
pub fn ole_object_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Set `Name` (`:name`) on a `OleObject` element.
pub fn ole_object_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `ShowAsIcon` (`:showAsIcon`) on a `OleObject` element.
pub fn ole_object_with_show_as_icon(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showAsIcon", value);
    el
}

/// Set `Id` (`r:id`) on a `OleObject` element.
pub fn ole_object_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Set `ImageWidth` (`:imgW`) on a `OleObject` element.
pub fn ole_object_with_image_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("imgW", value);
    el
}

/// Set `ImageHeight` (`:imgH`) on a `OleObject` element.
pub fn ole_object_with_image_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("imgH", value);
    el
}

/// Set `ProgId` (`:progId`) on a `OleObject` element.
pub fn ole_object_with_prog_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("progId", value);
    el
}

/// Create a `<p:presentation>` element (`Presentation`).
pub fn presentation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "presentation").with_children(children)
}

/// Set `ServerZoom` (`:serverZoom`) on a `Presentation` element.
pub fn presentation_with_server_zoom(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("serverZoom", value);
    el
}

/// Set `FirstSlideNum` (`:firstSlideNum`) on a `Presentation` element.
pub fn presentation_with_first_slide_num(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("firstSlideNum", value);
    el
}

/// Set `ShowSpecialPlaceholderOnTitleSlide` (`:showSpecialPlsOnTitleSld`) on a `Presentation` element.
pub fn presentation_with_show_special_placeholder_on_title_slide(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showSpecialPlsOnTitleSld", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Presentation` element.
pub fn presentation_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `RemovePersonalInfoOnSave` (`:removePersonalInfoOnSave`) on a `Presentation` element.
pub fn presentation_with_remove_personal_info_on_save(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("removePersonalInfoOnSave", value);
    el
}

/// Set `CompatibilityMode` (`:compatMode`) on a `Presentation` element.
pub fn presentation_with_compatibility_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("compatMode", value);
    el
}

/// Set `StrictFirstAndLastChars` (`:strictFirstAndLastChars`) on a `Presentation` element.
pub fn presentation_with_strict_first_and_last_chars(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("strictFirstAndLastChars", value);
    el
}

/// Set `EmbedTrueTypeFonts` (`:embedTrueTypeFonts`) on a `Presentation` element.
pub fn presentation_with_embed_true_type_fonts(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("embedTrueTypeFonts", value);
    el
}

/// Set `SaveSubsetFonts` (`:saveSubsetFonts`) on a `Presentation` element.
pub fn presentation_with_save_subset_fonts(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("saveSubsetFonts", value);
    el
}

/// Set `AutoCompressPictures` (`:autoCompressPictures`) on a `Presentation` element.
pub fn presentation_with_auto_compress_pictures(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("autoCompressPictures", value);
    el
}

/// Set `BookmarkIdSeed` (`:bookmarkIdSeed`) on a `Presentation` element.
pub fn presentation_with_bookmark_id_seed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bookmarkIdSeed", value);
    el
}

/// Set `Conformance` (`:conformance`) on a `Presentation` element.
pub fn presentation_with_conformance(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("conformance", value);
    el
}

/// Create a `<p:presentationPr>` element (`PresentationProperties`).
pub fn presentation_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "presentationPr").with_children(children)
}

/// Create a `<p:sld>` element (`Slide`).
pub fn slide(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sld").with_children(children)
}

/// Set `ShowMasterShapes` (`:showMasterSp`) on a `Slide` element.
pub fn slide_with_show_master_shapes(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showMasterSp", value);
    el
}

/// Set `ShowMasterPlaceholderAnimations` (`:showMasterPhAnim`) on a `Slide` element.
pub fn slide_with_show_master_placeholder_animations(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showMasterPhAnim", value);
    el
}

/// Set `Show` (`:show`) on a `Slide` element.
pub fn slide_with_show(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("show", value);
    el
}

/// Create a `<p:sldLayout>` element (`SlideLayout`).
pub fn slide_layout(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldLayout").with_children(children)
}

/// Set `ShowMasterShapes` (`:showMasterSp`) on a `SlideLayout` element.
pub fn slide_layout_with_show_master_shapes(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showMasterSp", value);
    el
}

/// Set `ShowMasterPlaceholderAnimations` (`:showMasterPhAnim`) on a `SlideLayout` element.
pub fn slide_layout_with_show_master_placeholder_animations(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showMasterPhAnim", value);
    el
}

/// Set `MatchingName` (`:matchingName`) on a `SlideLayout` element.
pub fn slide_layout_with_matching_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("matchingName", value);
    el
}

/// Set `Type` (`:type`) on a `SlideLayout` element.
pub fn slide_layout_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Preserve` (`:preserve`) on a `SlideLayout` element.
pub fn slide_layout_with_preserve(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("preserve", value);
    el
}

/// Set `UserDrawn` (`:userDrawn`) on a `SlideLayout` element.
pub fn slide_layout_with_user_drawn(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("userDrawn", value);
    el
}

/// Create a `<p:sldMaster>` element (`SlideMaster`).
pub fn slide_master(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldMaster").with_children(children)
}

/// Set `Preserve` (`:preserve`) on a `SlideMaster` element.
pub fn slide_master_with_preserve(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("preserve", value);
    el
}

/// Create a `<p:handoutMaster>` element (`HandoutMaster`).
pub fn handout_master(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "handoutMaster").with_children(children)
}

/// Create a `<p:notesMaster>` element (`NotesMaster`).
pub fn notes_master(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notesMaster").with_children(children)
}

/// Create a `<p:notes>` element (`NotesSlide`).
pub fn notes_slide(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notes").with_children(children)
}

/// Set `ShowMasterShapes` (`:showMasterSp`) on a `NotesSlide` element.
pub fn notes_slide_with_show_master_shapes(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showMasterSp", value);
    el
}

/// Set `ShowMasterPlaceholderAnimations` (`:showMasterPhAnim`) on a `NotesSlide` element.
pub fn notes_slide_with_show_master_placeholder_animations(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showMasterPhAnim", value);
    el
}

/// Create a `<p:sldSyncPr>` element (`SlideSyncProperties`).
pub fn slide_sync_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldSyncPr").with_children(children)
}

/// Set `ServerSlideId` (`:serverSldId`) on a `SlideSyncProperties` element.
pub fn slide_sync_properties_with_server_slide_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("serverSldId", value);
    el
}

/// Set `ServerSlideModifiedTime` (`:serverSldModifiedTime`) on a `SlideSyncProperties` element.
pub fn slide_sync_properties_with_server_slide_modified_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("serverSldModifiedTime", value);
    el
}

/// Set `ClientInsertedTime` (`:clientInsertedTime`) on a `SlideSyncProperties` element.
pub fn slide_sync_properties_with_client_inserted_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("clientInsertedTime", value);
    el
}

/// Create a `<p:tagLst>` element (`TagList`).
pub fn tag_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tagLst").with_children(children)
}

/// Create a `<p:viewPr>` element (`ViewProperties`).
pub fn view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "viewPr").with_children(children)
}

/// Set `LastView` (`:lastView`) on a `ViewProperties` element.
pub fn view_properties_with_last_view(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lastView", value);
    el
}

/// Set `ShowComments` (`:showComments`) on a `ViewProperties` element.
pub fn view_properties_with_show_comments(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showComments", value);
    el
}

/// Create a `<p:contentPart>` element (`ContentPart`).
pub fn content_part(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "contentPart").with_children(children)
}

/// Create a `<p:snd>` element (`Sound`).
pub fn sound() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "snd")
}

/// Set `Embed` (`r:embed`) on a `Sound` element.
pub fn sound_with_embed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:embed", value);
    el
}

/// Set `Name` (`:name`) on a `Sound` element.
pub fn sound_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `BuiltIn` (`:builtIn`) on a `Sound` element.
pub fn sound_with_built_in(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("builtIn", value);
    el
}

/// Create a `<p:sndTgt>` element (`SoundTarget`).
pub fn sound_target() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sndTgt")
}

/// Set `Embed` (`r:embed`) on a `SoundTarget` element.
pub fn sound_target_with_embed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:embed", value);
    el
}

/// Set `Name` (`:name`) on a `SoundTarget` element.
pub fn sound_target_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `BuiltIn` (`:builtIn`) on a `SoundTarget` element.
pub fn sound_target_with_built_in(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("builtIn", value);
    el
}

/// Create a `<p:stSnd>` element (`StartSoundAction`).
pub fn start_sound_action(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "stSnd").with_children(children)
}

/// Set `Loop` (`:loop`) on a `StartSoundAction` element.
pub fn start_sound_action_with_loop_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("loop", value);
    el
}

/// Create a `<p:tmAbs>` element (`TimeAbsolute`).
pub fn time_absolute() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tmAbs")
}

/// Set `Val` (`:val`) on a `TimeAbsolute` element.
pub fn time_absolute_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:tmAbs>` with `Val` set.
pub fn time_absolute_val(value: impl Into<String>) -> OpenXmlElement {
    time_absolute_with_val(time_absolute(), value)
}

/// Create a `<p:tmPct>` element (`TimePercentage`).
pub fn time_percentage() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tmPct")
}

/// Set `Val` (`:val`) on a `TimePercentage` element.
pub fn time_percentage_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:tmPct>` with `Val` set.
pub fn time_percentage_val(value: impl Into<String>) -> OpenXmlElement {
    time_percentage_with_val(time_percentage(), value)
}

/// Create a `<p:tgtEl>` element (`TargetElement`).
pub fn target_element(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tgtEl").with_children(children)
}

/// Create a `<p:tn>` element (`TimeNode`).
pub fn time_node() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tn")
}

/// Set `Val` (`:val`) on a `TimeNode` element.
pub fn time_node_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:tn>` with `Val` set.
pub fn time_node_val(value: impl Into<String>) -> OpenXmlElement {
    time_node_with_val(time_node(), value)
}

/// Create a `<p:rtn>` element (`RuntimeNodeTrigger`).
pub fn runtime_node_trigger() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "rtn")
}

/// Set `Val` (`:val`) on a `RuntimeNodeTrigger` element.
pub fn runtime_node_trigger_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:rtn>` with `Val` set.
pub fn runtime_node_trigger_val(value: impl Into<String>) -> OpenXmlElement {
    runtime_node_trigger_with_val(runtime_node_trigger(), value)
}

/// Create a `<p:cond>` element (`Condition`).
pub fn condition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cond").with_children(children)
}

/// Set `Event` (`:evt`) on a `Condition` element.
pub fn condition_with_event(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("evt", value);
    el
}

/// Set `Delay` (`:delay`) on a `Condition` element.
pub fn condition_with_delay(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("delay", value);
    el
}

/// Create a `<p:endSync>` element (`EndSync`).
pub fn end_sync(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "endSync").with_children(children)
}

/// Set `Event` (`:evt`) on a `EndSync` element.
pub fn end_sync_with_event(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("evt", value);
    el
}

/// Set `Delay` (`:delay`) on a `EndSync` element.
pub fn end_sync_with_delay(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("delay", value);
    el
}

/// Create a `<p:par>` element (`ParallelTimeNode`).
pub fn parallel_time_node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "par").with_children(children)
}

/// Create a `<p:seq>` element (`SequenceTimeNode`).
pub fn sequence_time_node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "seq").with_children(children)
}

/// Set `Concurrent` (`:concurrent`) on a `SequenceTimeNode` element.
pub fn sequence_time_node_with_concurrent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("concurrent", value);
    el
}

/// Set `PreviousAction` (`:prevAc`) on a `SequenceTimeNode` element.
pub fn sequence_time_node_with_previous_action(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prevAc", value);
    el
}

/// Set `NextAction` (`:nextAc`) on a `SequenceTimeNode` element.
pub fn sequence_time_node_with_next_action(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("nextAc", value);
    el
}

/// Create a `<p:excl>` element (`ExclusiveTimeNode`).
pub fn exclusive_time_node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "excl").with_children(children)
}

/// Create a `<p:anim>` element (`Animate`).
pub fn animate(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "anim").with_children(children)
}

/// Set `By` (`:by`) on a `Animate` element.
pub fn animate_with_by(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("by", value);
    el
}

/// Set `From` (`:from`) on a `Animate` element.
pub fn animate_with_from(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("from", value);
    el
}

/// Set `To` (`:to`) on a `Animate` element.
pub fn animate_with_to(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("to", value);
    el
}

/// Set `CalculationMode` (`:calcmode`) on a `Animate` element.
pub fn animate_with_calculation_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("calcmode", value);
    el
}

/// Set `ValueType` (`:valueType`) on a `Animate` element.
pub fn animate_with_value_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("valueType", value);
    el
}

/// Set `BounceEnd` (`p14:bounceEnd`) on a `Animate` element.
pub fn animate_with_bounce_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("p14:bounceEnd", value);
    el
}

/// Create a `<p:animClr>` element (`AnimateColor`).
pub fn animate_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "animClr").with_children(children)
}

/// Set `ColorSpace` (`:clrSpc`) on a `AnimateColor` element.
pub fn animate_color_with_color_space(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("clrSpc", value);
    el
}

/// Set `Direction` (`:dir`) on a `AnimateColor` element.
pub fn animate_color_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create a `<p:animEffect>` element (`AnimateEffect`).
pub fn animate_effect(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "animEffect").with_children(children)
}

/// Set `Transition` (`:transition`) on a `AnimateEffect` element.
pub fn animate_effect_with_transition(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("transition", value);
    el
}

/// Set `Filter` (`:filter`) on a `AnimateEffect` element.
pub fn animate_effect_with_filter(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("filter", value);
    el
}

/// Set `PropertyList` (`:prLst`) on a `AnimateEffect` element.
pub fn animate_effect_with_property_list(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prLst", value);
    el
}

/// Create a `<p:animMotion>` element (`AnimateMotion`).
pub fn animate_motion(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "animMotion").with_children(children)
}

/// Set `Origin` (`:origin`) on a `AnimateMotion` element.
pub fn animate_motion_with_origin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("origin", value);
    el
}

/// Set `Path` (`:path`) on a `AnimateMotion` element.
pub fn animate_motion_with_path(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("path", value);
    el
}

/// Set `PathEditMode` (`:pathEditMode`) on a `AnimateMotion` element.
pub fn animate_motion_with_path_edit_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pathEditMode", value);
    el
}

/// Set `RelativeAngle` (`:rAng`) on a `AnimateMotion` element.
pub fn animate_motion_with_relative_angle(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rAng", value);
    el
}

/// Set `PointTypes` (`:ptsTypes`) on a `AnimateMotion` element.
pub fn animate_motion_with_point_types(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ptsTypes", value);
    el
}

/// Set `BounceEnd` (`p14:bounceEnd`) on a `AnimateMotion` element.
pub fn animate_motion_with_bounce_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("p14:bounceEnd", value);
    el
}

/// Create a `<p:animRot>` element (`AnimateRotation`).
pub fn animate_rotation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "animRot").with_children(children)
}

/// Set `By` (`:by`) on a `AnimateRotation` element.
pub fn animate_rotation_with_by(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("by", value);
    el
}

/// Set `From` (`:from`) on a `AnimateRotation` element.
pub fn animate_rotation_with_from(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("from", value);
    el
}

/// Set `To` (`:to`) on a `AnimateRotation` element.
pub fn animate_rotation_with_to(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("to", value);
    el
}

/// Set `BounceEnd` (`p14:bounceEnd`) on a `AnimateRotation` element.
pub fn animate_rotation_with_bounce_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("p14:bounceEnd", value);
    el
}

/// Create a `<p:animScale>` element (`AnimateScale`).
pub fn animate_scale(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "animScale").with_children(children)
}

/// Set `ZoomContents` (`:zoomContents`) on a `AnimateScale` element.
pub fn animate_scale_with_zoom_contents(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("zoomContents", value);
    el
}

/// Set `BounceEnd` (`p14:bounceEnd`) on a `AnimateScale` element.
pub fn animate_scale_with_bounce_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("p14:bounceEnd", value);
    el
}

/// Create a `<p:cmd>` element (`Command`).
pub fn command(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cmd").with_children(children)
}

/// Set `Type` (`:type`) on a `Command` element.
pub fn command_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `CommandName` (`:cmd`) on a `Command` element.
pub fn command_with_command_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmd", value);
    el
}

/// Create a `<p:set>` element (`SetBehavior`).
pub fn set_behavior(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "set").with_children(children)
}

/// Create a `<p:audio>` element (`Audio`).
pub fn audio(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "audio").with_children(children)
}

/// Set `IsNarration` (`:isNarration`) on a `Audio` element.
pub fn audio_with_is_narration(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("isNarration", value);
    el
}

/// Create a `<p:video>` element (`Video`).
pub fn video(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "video").with_children(children)
}

/// Set `FullScreen` (`:fullScrn`) on a `Video` element.
pub fn video_with_full_screen(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fullScrn", value);
    el
}

/// Create a `<p:cTn>` element (`CommonTimeNode`).
pub fn common_time_node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cTn").with_children(children)
}

/// Set `Id` (`:id`) on a `CommonTimeNode` element.
pub fn common_time_node_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `PresetId` (`:presetID`) on a `CommonTimeNode` element.
pub fn common_time_node_with_preset_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("presetID", value);
    el
}

/// Set `PresetClass` (`:presetClass`) on a `CommonTimeNode` element.
pub fn common_time_node_with_preset_class(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("presetClass", value);
    el
}

/// Set `PresetSubtype` (`:presetSubtype`) on a `CommonTimeNode` element.
pub fn common_time_node_with_preset_subtype(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("presetSubtype", value);
    el
}

/// Set `Duration` (`:dur`) on a `CommonTimeNode` element.
pub fn common_time_node_with_duration(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dur", value);
    el
}

/// Set `RepeatCount` (`:repeatCount`) on a `CommonTimeNode` element.
pub fn common_time_node_with_repeat_count(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("repeatCount", value);
    el
}

/// Set `RepeatDuration` (`:repeatDur`) on a `CommonTimeNode` element.
pub fn common_time_node_with_repeat_duration(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("repeatDur", value);
    el
}

/// Set `Speed` (`:spd`) on a `CommonTimeNode` element.
pub fn common_time_node_with_speed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spd", value);
    el
}

/// Set `Acceleration` (`:accel`) on a `CommonTimeNode` element.
pub fn common_time_node_with_acceleration(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accel", value);
    el
}

/// Set `Deceleration` (`:decel`) on a `CommonTimeNode` element.
pub fn common_time_node_with_deceleration(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("decel", value);
    el
}

/// Set `AutoReverse` (`:autoRev`) on a `CommonTimeNode` element.
pub fn common_time_node_with_auto_reverse(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("autoRev", value);
    el
}

/// Set `Restart` (`:restart`) on a `CommonTimeNode` element.
pub fn common_time_node_with_restart(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("restart", value);
    el
}

/// Set `Fill` (`:fill`) on a `CommonTimeNode` element.
pub fn common_time_node_with_fill(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fill", value);
    el
}

/// Set `SyncBehavior` (`:syncBehavior`) on a `CommonTimeNode` element.
pub fn common_time_node_with_sync_behavior(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("syncBehavior", value);
    el
}

/// Set `TimeFilter` (`:tmFilter`) on a `CommonTimeNode` element.
pub fn common_time_node_with_time_filter(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tmFilter", value);
    el
}

/// Set `EventFilter` (`:evtFilter`) on a `CommonTimeNode` element.
pub fn common_time_node_with_event_filter(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("evtFilter", value);
    el
}

/// Set `Display` (`:display`) on a `CommonTimeNode` element.
pub fn common_time_node_with_display(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("display", value);
    el
}

/// Set `MasterRelation` (`:masterRel`) on a `CommonTimeNode` element.
pub fn common_time_node_with_master_relation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("masterRel", value);
    el
}

/// Set `BuildLevel` (`:bldLvl`) on a `CommonTimeNode` element.
pub fn common_time_node_with_build_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bldLvl", value);
    el
}

/// Set `GroupId` (`:grpId`) on a `CommonTimeNode` element.
pub fn common_time_node_with_group_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("grpId", value);
    el
}

/// Set `AfterEffect` (`:afterEffect`) on a `CommonTimeNode` element.
pub fn common_time_node_with_after_effect(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("afterEffect", value);
    el
}

/// Set `NodeType` (`:nodeType`) on a `CommonTimeNode` element.
pub fn common_time_node_with_node_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("nodeType", value);
    el
}

/// Set `NodePlaceholder` (`:nodePh`) on a `CommonTimeNode` element.
pub fn common_time_node_with_node_placeholder(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("nodePh", value);
    el
}

/// Set `PresetBounceEnd` (`p14:presetBounceEnd`) on a `CommonTimeNode` element.
pub fn common_time_node_with_preset_bounce_end(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("p14:presetBounceEnd", value);
    el
}

/// Create a `<p:prevCondLst>` element (`PreviousConditionList`).
pub fn previous_condition_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "prevCondLst").with_children(children)
}

/// Create a `<p:nextCondLst>` element (`NextConditionList`).
pub fn next_condition_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "nextCondLst").with_children(children)
}

/// Create a `<p:stCondLst>` element (`StartConditionList`).
pub fn start_condition_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "stCondLst").with_children(children)
}

/// Create a `<p:endCondLst>` element (`EndConditionList`).
pub fn end_condition_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "endCondLst").with_children(children)
}

/// Create a `<p:attrName>` element (`AttributeName`).
pub fn attribute_name(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "attrName").with_text(value)
}

/// Create a `<p:text>` element (`Text`).
pub fn text(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "text").with_text(value)
}

/// Create a `<p:attrNameLst>` element (`AttributeNameList`).
pub fn attribute_name_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "attrNameLst").with_children(children)
}

/// Create a `<p:boolVal>` element (`BooleanVariantValue`).
pub fn boolean_variant_value() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "boolVal")
}

/// Set `Val` (`:val`) on a `BooleanVariantValue` element.
pub fn boolean_variant_value_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:boolVal>` with `Val` set.
pub fn boolean_variant_value_val(value: impl Into<String>) -> OpenXmlElement {
    boolean_variant_value_with_val(boolean_variant_value(), value)
}

/// Create a `<p:intVal>` element (`IntegerVariantValue`).
pub fn integer_variant_value() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "intVal")
}

/// Set `Val` (`:val`) on a `IntegerVariantValue` element.
pub fn integer_variant_value_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:intVal>` with `Val` set.
pub fn integer_variant_value_val(value: impl Into<String>) -> OpenXmlElement {
    integer_variant_value_with_val(integer_variant_value(), value)
}

/// Create a `<p:fltVal>` element (`FloatVariantValue`).
pub fn float_variant_value() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "fltVal")
}

/// Set `Val` (`:val`) on a `FloatVariantValue` element.
pub fn float_variant_value_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:fltVal>` with `Val` set.
pub fn float_variant_value_val(value: impl Into<String>) -> OpenXmlElement {
    float_variant_value_with_val(float_variant_value(), value)
}

/// Create a `<p:strVal>` element (`StringVariantValue`).
pub fn string_variant_value() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "strVal")
}

/// Set `Val` (`:val`) on a `StringVariantValue` element.
pub fn string_variant_value_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<p:strVal>` with `Val` set.
pub fn string_variant_value_val(value: impl Into<String>) -> OpenXmlElement {
    string_variant_value_with_val(string_variant_value(), value)
}

/// Create a `<p:clrVal>` element (`ColorValue`).
pub fn color_value(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "clrVal").with_children(children)
}

/// Create a `<p:penClr>` element (`PenColor`).
pub fn pen_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "penClr").with_children(children)
}

/// Create a `<p:tav>` element (`TimeAnimateValue`).
pub fn time_animate_value(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tav").with_children(children)
}

/// Set `Time` (`:tm`) on a `TimeAnimateValue` element.
pub fn time_animate_value_with_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tm", value);
    el
}

/// Set `Fomula` (`:fmla`) on a `TimeAnimateValue` element.
pub fn time_animate_value_with_fomula(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fmla", value);
    el
}

/// Create a `<p:rgb>` element (`RgbColor`).
pub fn rgb_color() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "rgb")
}

/// Set `Red` (`:r`) on a `RgbColor` element.
pub fn rgb_color_with_red(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r", value);
    el
}

/// Set `Green` (`:g`) on a `RgbColor` element.
pub fn rgb_color_with_green(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("g", value);
    el
}

/// Set `Blue` (`:b`) on a `RgbColor` element.
pub fn rgb_color_with_blue(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Create a `<p:hsl>` element (`HslColor`).
pub fn hsl_color() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "hsl")
}

/// Set `Hue` (`:h`) on a `HslColor` element.
pub fn hsl_color_with_hue(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("h", value);
    el
}

/// Set `Saturation` (`:s`) on a `HslColor` element.
pub fn hsl_color_with_saturation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("s", value);
    el
}

/// Set `Lightness` (`:l`) on a `HslColor` element.
pub fn hsl_color_with_lightness(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("l", value);
    el
}

/// Create a `<p:cBhvr>` element (`CommonBehavior`).
pub fn common_behavior(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cBhvr").with_children(children)
}

/// Set `Additive` (`:additive`) on a `CommonBehavior` element.
pub fn common_behavior_with_additive(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("additive", value);
    el
}

/// Set `Accumulate` (`:accumulate`) on a `CommonBehavior` element.
pub fn common_behavior_with_accumulate(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accumulate", value);
    el
}

/// Set `TransformType` (`:xfrmType`) on a `CommonBehavior` element.
pub fn common_behavior_with_transform_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("xfrmType", value);
    el
}

/// Set `From` (`:from`) on a `CommonBehavior` element.
pub fn common_behavior_with_from(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("from", value);
    el
}

/// Set `To` (`:to`) on a `CommonBehavior` element.
pub fn common_behavior_with_to(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("to", value);
    el
}

/// Set `By` (`:by`) on a `CommonBehavior` element.
pub fn common_behavior_with_by(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("by", value);
    el
}

/// Set `RuntimeContext` (`:rctx`) on a `CommonBehavior` element.
pub fn common_behavior_with_runtime_context(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rctx", value);
    el
}

/// Set `Override` (`:override`) on a `CommonBehavior` element.
pub fn common_behavior_with_override(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("override", value);
    el
}

/// Create a `<p:progress>` element (`Progress`).
pub fn progress(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "progress").with_children(children)
}

/// Create a `<p:to>` element (`ToVariantValue`).
pub fn to_variant_value(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "to").with_children(children)
}

/// Create a `<p:val>` element (`VariantValue`).
pub fn variant_value(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "val").with_children(children)
}

/// Create a `<p:cMediaNode>` element (`CommonMediaNode`).
pub fn common_media_node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cMediaNode").with_children(children)
}

/// Set `Volume` (`:vol`) on a `CommonMediaNode` element.
pub fn common_media_node_with_volume(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("vol", value);
    el
}

/// Set `Mute` (`:mute`) on a `CommonMediaNode` element.
pub fn common_media_node_with_mute(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("mute", value);
    el
}

/// Set `SlideCount` (`:numSld`) on a `CommonMediaNode` element.
pub fn common_media_node_with_slide_count(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("numSld", value);
    el
}

/// Set `ShowWhenStopped` (`:showWhenStopped`) on a `CommonMediaNode` element.
pub fn common_media_node_with_show_when_stopped(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showWhenStopped", value);
    el
}

/// Create a `<p:tnLst>` element (`TimeNodeList`).
pub fn time_node_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tnLst").with_children(children)
}

/// Create a `<p:tmpl>` element (`Template`).
pub fn template(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tmpl").with_children(children)
}

/// Set `Level` (`:lvl`) on a `Template` element.
pub fn template_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Create a `<p:tmplLst>` element (`TemplateList`).
pub fn template_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tmplLst").with_children(children)
}

/// Create a `<p:bldSub>` element (`BuildSubElement`).
pub fn build_sub_element(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bldSub").with_children(children)
}

/// Create a `<p:bldP>` element (`BuildParagraph`).
pub fn build_paragraph(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bldP").with_children(children)
}

/// Set `ShapeId` (`:spid`) on a `BuildParagraph` element.
pub fn build_paragraph_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Set `GroupId` (`:grpId`) on a `BuildParagraph` element.
pub fn build_paragraph_with_group_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("grpId", value);
    el
}

/// Set `UiExpand` (`:uiExpand`) on a `BuildParagraph` element.
pub fn build_paragraph_with_ui_expand(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("uiExpand", value);
    el
}

/// Set `Build` (`:build`) on a `BuildParagraph` element.
pub fn build_paragraph_with_build(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("build", value);
    el
}

/// Set `BuildLevel` (`:bldLvl`) on a `BuildParagraph` element.
pub fn build_paragraph_with_build_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bldLvl", value);
    el
}

/// Set `AnimateBackground` (`:animBg`) on a `BuildParagraph` element.
pub fn build_paragraph_with_animate_background(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("animBg", value);
    el
}

/// Set `AutoAnimateBackground` (`:autoUpdateAnimBg`) on a `BuildParagraph` element.
pub fn build_paragraph_with_auto_animate_background(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("autoUpdateAnimBg", value);
    el
}

/// Set `Reverse` (`:rev`) on a `BuildParagraph` element.
pub fn build_paragraph_with_reverse(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rev", value);
    el
}

/// Set `AutoAdvance` (`:advAuto`) on a `BuildParagraph` element.
pub fn build_paragraph_with_auto_advance(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("advAuto", value);
    el
}

/// Create a `<p:bldDgm>` element (`BuildDiagram`).
pub fn build_diagram() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bldDgm")
}

/// Set `ShapeId` (`:spid`) on a `BuildDiagram` element.
pub fn build_diagram_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Set `GroupId` (`:grpId`) on a `BuildDiagram` element.
pub fn build_diagram_with_group_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("grpId", value);
    el
}

/// Set `UiExpand` (`:uiExpand`) on a `BuildDiagram` element.
pub fn build_diagram_with_ui_expand(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("uiExpand", value);
    el
}

/// Set `Build` (`:bld`) on a `BuildDiagram` element.
pub fn build_diagram_with_build(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bld", value);
    el
}

/// Create a `<p:bldOleChart>` element (`BuildOleChart`).
pub fn build_ole_chart() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bldOleChart")
}

/// Set `ShapeId` (`:spid`) on a `BuildOleChart` element.
pub fn build_ole_chart_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Set `GroupId` (`:grpId`) on a `BuildOleChart` element.
pub fn build_ole_chart_with_group_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("grpId", value);
    el
}

/// Set `UiExpand` (`:uiExpand`) on a `BuildOleChart` element.
pub fn build_ole_chart_with_ui_expand(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("uiExpand", value);
    el
}

/// Set `Build` (`:bld`) on a `BuildOleChart` element.
pub fn build_ole_chart_with_build(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bld", value);
    el
}

/// Set `AnimateBackground` (`:animBg`) on a `BuildOleChart` element.
pub fn build_ole_chart_with_animate_background(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("animBg", value);
    el
}

/// Create a `<p:bldGraphic>` element (`BuildGraphics`).
pub fn build_graphics(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bldGraphic").with_children(children)
}

/// Set `ShapeId` (`:spid`) on a `BuildGraphics` element.
pub fn build_graphics_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Set `GroupId` (`:grpId`) on a `BuildGraphics` element.
pub fn build_graphics_with_group_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("grpId", value);
    el
}

/// Set `UiExpand` (`:uiExpand`) on a `BuildGraphics` element.
pub fn build_graphics_with_ui_expand(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("uiExpand", value);
    el
}

/// Create a `<p:bldLst>` element (`BuildList`).
pub fn build_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bldLst").with_children(children)
}

/// Create a `<p:extLst>` element (`ExtensionListWithModification`).
pub fn extension_list_with_modification(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Set `Modify` (`:mod`) on a `ExtensionListWithModification` element.
pub fn extension_list_with_modification_with_modify(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("mod", value);
    el
}

/// Create a `<p:by>` element (`ByColor`).
pub fn by_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "by").with_children(children)
}

/// Create a `<p:from>` element (`FromColor`).
pub fn from_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "from").with_children(children)
}

/// Create a `<p:to>` element (`ToColor`).
pub fn to_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "to").with_children(children)
}

/// Create a `<p:sld>` element (`SlideListEntry`).
pub fn slide_list_entry() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sld")
}

/// Set `Id` (`r:id`) on a `SlideListEntry` element.
pub fn slide_list_entry_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create `<p:sld>` with `Id` set.
pub fn slide_list_entry_id(value: impl Into<String>) -> OpenXmlElement {
    slide_list_entry_with_id(slide_list_entry(), value)
}

/// Create a `<p:custData>` element (`CustomerData`).
pub fn customer_data() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "custData")
}

/// Set `Id` (`r:id`) on a `CustomerData` element.
pub fn customer_data_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create `<p:custData>` with `Id` set.
pub fn customer_data_id(value: impl Into<String>) -> OpenXmlElement {
    customer_data_with_id(customer_data(), value)
}

/// Create a `<p:tags>` element (`CustomerDataTags`).
pub fn customer_data_tags() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tags")
}

/// Set `Id` (`r:id`) on a `CustomerDataTags` element.
pub fn customer_data_tags_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create `<p:tags>` with `Id` set.
pub fn customer_data_tags_id(value: impl Into<String>) -> OpenXmlElement {
    customer_data_tags_with_id(customer_data_tags(), value)
}

/// Create a `<p:cmAuthor>` element (`CommentAuthor`).
pub fn comment_author(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cmAuthor").with_children(children)
}

/// Set `Id` (`:id`) on a `CommentAuthor` element.
pub fn comment_author_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `Name` (`:name`) on a `CommentAuthor` element.
pub fn comment_author_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `Initials` (`:initials`) on a `CommentAuthor` element.
pub fn comment_author_with_initials(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("initials", value);
    el
}

/// Set `LastIndex` (`:lastIdx`) on a `CommentAuthor` element.
pub fn comment_author_with_last_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lastIdx", value);
    el
}

/// Set `ColorIndex` (`:clrIdx`) on a `CommentAuthor` element.
pub fn comment_author_with_color_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("clrIdx", value);
    el
}

/// Create a `<p:cm>` element (`Comment`).
pub fn comment(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cm").with_children(children)
}

/// Set `AuthorId` (`:authorId`) on a `Comment` element.
pub fn comment_with_author_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("authorId", value);
    el
}

/// Set `DateTime` (`:dt`) on a `Comment` element.
pub fn comment_with_date_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dt", value);
    el
}

/// Set `Index` (`:idx`) on a `Comment` element.
pub fn comment_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Create a `<p:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:control>` element (`Control`).
pub fn control(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "control").with_children(children)
}

/// Set `ShapeId` (`:spid`) on a `Control` element.
pub fn control_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Set `Name` (`:name`) on a `Control` element.
pub fn control_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `ShowAsIcon` (`:showAsIcon`) on a `Control` element.
pub fn control_with_show_as_icon(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showAsIcon", value);
    el
}

/// Set `Id` (`r:id`) on a `Control` element.
pub fn control_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Set `ImageWidth` (`:imgW`) on a `Control` element.
pub fn control_with_image_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("imgW", value);
    el
}

/// Set `ImageHeight` (`:imgH`) on a `Control` element.
pub fn control_with_image_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("imgH", value);
    el
}

/// Create a `<p:sldId>` element (`SlideId`).
pub fn slide_id(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldId").with_children(children)
}

/// Set `Id` (`:id`) on a `SlideId` element.
pub fn slide_id_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `RelationshipId` (`r:id`) on a `SlideId` element.
pub fn slide_id_with_relationship_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create a `<p:sldMasterId>` element (`SlideMasterId`).
pub fn slide_master_id(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldMasterId").with_children(children)
}

/// Set `Id` (`:id`) on a `SlideMasterId` element.
pub fn slide_master_id_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `RelationshipId` (`r:id`) on a `SlideMasterId` element.
pub fn slide_master_id_with_relationship_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create a `<p:notesMasterId>` element (`NotesMasterId`).
pub fn notes_master_id(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notesMasterId").with_children(children)
}

/// Set `Id` (`r:id`) on a `NotesMasterId` element.
pub fn notes_master_id_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create a `<p:handoutMasterId>` element (`HandoutMasterId`).
pub fn handout_master_id(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "handoutMasterId").with_children(children)
}

/// Set `Id` (`r:id`) on a `HandoutMasterId` element.
pub fn handout_master_id_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create a `<p:font>` element (`Font`).
pub fn font() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "font")
}

/// Set `Typeface` (`:typeface`) on a `Font` element.
pub fn font_with_typeface(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("typeface", value);
    el
}

/// Set `Panose` (`:panose`) on a `Font` element.
pub fn font_with_panose(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("panose", value);
    el
}

/// Set `PitchFamily` (`:pitchFamily`) on a `Font` element.
pub fn font_with_pitch_family(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pitchFamily", value);
    el
}

/// Set `CharacterSet` (`:charset`) on a `Font` element.
pub fn font_with_character_set(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("charset", value);
    el
}

/// Create a `<p:regular>` element (`RegularFont`).
pub fn regular_font() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "regular")
}

/// Set `Id` (`r:id`) on a `RegularFont` element.
pub fn regular_font_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create `<p:regular>` with `Id` set.
pub fn regular_font_id(value: impl Into<String>) -> OpenXmlElement {
    regular_font_with_id(regular_font(), value)
}

/// Create a `<p:bold>` element (`BoldFont`).
pub fn bold_font() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bold")
}

/// Set `Id` (`r:id`) on a `BoldFont` element.
pub fn bold_font_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create `<p:bold>` with `Id` set.
pub fn bold_font_id(value: impl Into<String>) -> OpenXmlElement {
    bold_font_with_id(bold_font(), value)
}

/// Create a `<p:italic>` element (`ItalicFont`).
pub fn italic_font() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "italic")
}

/// Set `Id` (`r:id`) on a `ItalicFont` element.
pub fn italic_font_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create `<p:italic>` with `Id` set.
pub fn italic_font_id(value: impl Into<String>) -> OpenXmlElement {
    italic_font_with_id(italic_font(), value)
}

/// Create a `<p:boldItalic>` element (`BoldItalicFont`).
pub fn bold_italic_font() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "boldItalic")
}

/// Set `Id` (`r:id`) on a `BoldItalicFont` element.
pub fn bold_italic_font_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create `<p:boldItalic>` with `Id` set.
pub fn bold_italic_font_id(value: impl Into<String>) -> OpenXmlElement {
    bold_italic_font_with_id(bold_italic_font(), value)
}

/// Create a `<p:embeddedFont>` element (`EmbeddedFont`).
pub fn embedded_font(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "embeddedFont").with_children(children)
}

/// Create a `<p:sldLst>` element (`SlideList`).
pub fn slide_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldLst").with_children(children)
}

/// Create a `<p:custShow>` element (`CustomShow`).
pub fn custom_show(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "custShow").with_children(children)
}

/// Set `Name` (`:name`) on a `CustomShow` element.
pub fn custom_show_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `Id` (`:id`) on a `CustomShow` element.
pub fn custom_show_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Create a `<p:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Set `Id` (`:id`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `Name` (`:name`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `Description` (`:descr`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_description(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("descr", value);
    el
}

/// Set `Hidden` (`:hidden`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_hidden(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hidden", value);
    el
}

/// Set `Title` (`:title`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_title(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("title", value);
    el
}

/// Create a `<p:cNvSpPr>` element (`NonVisualShapeDrawingProperties`).
pub fn non_visual_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cNvSpPr").with_children(children)
}

/// Set `TextBox` (`:txBox`) on a `NonVisualShapeDrawingProperties` element.
pub fn non_visual_shape_drawing_properties_with_text_box(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("txBox", value);
    el
}

/// Create a `<p:nvPr>` element (`ApplicationNonVisualDrawingProperties`).
pub fn application_non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "nvPr").with_children(children)
}

/// Set `IsPhoto` (`:isPhoto`) on a `ApplicationNonVisualDrawingProperties` element.
pub fn application_non_visual_drawing_properties_with_is_photo(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("isPhoto", value);
    el
}

/// Set `UserDrawn` (`:userDrawn`) on a `ApplicationNonVisualDrawingProperties` element.
pub fn application_non_visual_drawing_properties_with_user_drawn(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("userDrawn", value);
    el
}

/// Create a `<p:nvSpPr>` element (`NonVisualShapeProperties`).
pub fn non_visual_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "nvSpPr").with_children(children)
}

/// Create a `<p:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "spPr").with_children(children)
}

/// Set `BlackWhiteMode` (`:bwMode`) on a `ShapeProperties` element.
pub fn shape_properties_with_black_white_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bwMode", value);
    el
}

/// Create a `<p:style>` element (`ShapeStyle`).
pub fn shape_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<p:txBody>` element (`TextBody`).
pub fn text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "txBody").with_children(children)
}

/// Create a `<p:cNvCxnSpPr>` element (`NonVisualConnectorShapeDrawingProperties`).
pub fn non_visual_connector_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cNvCxnSpPr").with_children(children)
}

/// Create a `<p:nvCxnSpPr>` element (`NonVisualConnectionShapeProperties`).
pub fn non_visual_connection_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "nvCxnSpPr").with_children(children)
}

/// Create a `<p:cNvPicPr>` element (`NonVisualPictureDrawingProperties`).
pub fn non_visual_picture_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cNvPicPr").with_children(children)
}

/// Set `PreferRelativeResize` (`:preferRelativeResize`) on a `NonVisualPictureDrawingProperties` element.
pub fn non_visual_picture_drawing_properties_with_prefer_relative_resize(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("preferRelativeResize", value);
    el
}

/// Create a `<p:nvPicPr>` element (`NonVisualPictureProperties`).
pub fn non_visual_picture_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "nvPicPr").with_children(children)
}

/// Create a `<p:blipFill>` element (`BlipFill`).
pub fn blip_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "blipFill").with_children(children)
}

/// Set `Dpi` (`:dpi`) on a `BlipFill` element.
pub fn blip_fill_with_dpi(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dpi", value);
    el
}

/// Set `RotateWithShape` (`:rotWithShape`) on a `BlipFill` element.
pub fn blip_fill_with_rotate_with_shape(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rotWithShape", value);
    el
}

/// Create a `<p:cNvGraphicFramePr>` element (`NonVisualGraphicFrameDrawingProperties`).
pub fn non_visual_graphic_frame_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cNvGraphicFramePr").with_children(children)
}

/// Create a `<p:nvGraphicFramePr>` element (`NonVisualGraphicFrameProperties`).
pub fn non_visual_graphic_frame_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "nvGraphicFramePr").with_children(children)
}

/// Create a `<p:xfrm>` element (`Transform`).
pub fn transform(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Set `Rotation` (`:rot`) on a `Transform` element.
pub fn transform_with_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rot", value);
    el
}

/// Set `HorizontalFlip` (`:flipH`) on a `Transform` element.
pub fn transform_with_horizontal_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flipH", value);
    el
}

/// Set `VerticalFlip` (`:flipV`) on a `Transform` element.
pub fn transform_with_vertical_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flipV", value);
    el
}

/// Create a `<p:cNvGrpSpPr>` element (`NonVisualGroupShapeDrawingProperties`).
pub fn non_visual_group_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cNvGrpSpPr").with_children(children)
}

/// Create a `<p:titleStyle>` element (`TitleStyle`).
pub fn title_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "titleStyle").with_children(children)
}

/// Create a `<p:bodyStyle>` element (`BodyStyle`).
pub fn body_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bodyStyle").with_children(children)
}

/// Create a `<p:otherStyle>` element (`OtherStyle`).
pub fn other_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "otherStyle").with_children(children)
}

/// Create a `<p:defaultTextStyle>` element (`DefaultTextStyle`).
pub fn default_text_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "defaultTextStyle").with_children(children)
}

/// Create a `<p:notesStyle>` element (`NotesStyle`).
pub fn notes_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notesStyle").with_children(children)
}

/// Create a `<p:sldLayoutId>` element (`SlideLayoutId`).
pub fn slide_layout_id(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldLayoutId").with_children(children)
}

/// Set `Id` (`:id`) on a `SlideLayoutId` element.
pub fn slide_layout_id_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `RelationshipId` (`r:id`) on a `SlideLayoutId` element.
pub fn slide_layout_id_with_relationship_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create a `<p:cSld>` element (`CommonSlideData`).
pub fn common_slide_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cSld").with_children(children)
}

/// Set `Name` (`:name`) on a `CommonSlideData` element.
pub fn common_slide_data_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Create a `<p:tag>` element (`Tag`).
pub fn tag() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tag")
}

/// Set `Name` (`:name`) on a `Tag` element.
pub fn tag_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `Val` (`:val`) on a `Tag` element.
pub fn tag_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create a `<p:restoredLeft>` element (`RestoredLeft`).
pub fn restored_left() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "restoredLeft")
}

/// Set `Size` (`:sz`) on a `RestoredLeft` element.
pub fn restored_left_with_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sz", value);
    el
}

/// Set `AutoAdjust` (`:autoAdjust`) on a `RestoredLeft` element.
pub fn restored_left_with_auto_adjust(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("autoAdjust", value);
    el
}

/// Create a `<p:restoredTop>` element (`RestoredTop`).
pub fn restored_top() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "restoredTop")
}

/// Set `Size` (`:sz`) on a `RestoredTop` element.
pub fn restored_top_with_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sz", value);
    el
}

/// Set `AutoAdjust` (`:autoAdjust`) on a `RestoredTop` element.
pub fn restored_top_with_auto_adjust(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("autoAdjust", value);
    el
}

/// Create a `<p:scale>` element (`ScaleFactor`).
pub fn scale_factor(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "scale").with_children(children)
}

/// Create a `<p:origin>` element (`Origin`).
pub fn origin() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "origin")
}

/// Set `X` (`:x`) on a `Origin` element.
pub fn origin_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `Origin` element.
pub fn origin_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<p:pos>` element (`Position`).
pub fn position() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "pos")
}

/// Set `X` (`:x`) on a `Position` element.
pub fn position_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `Position` element.
pub fn position_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<p:cViewPr>` element (`CommonViewProperties`).
pub fn common_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cViewPr").with_children(children)
}

/// Set `VariableScale` (`:varScale`) on a `CommonViewProperties` element.
pub fn common_view_properties_with_variable_scale(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("varScale", value);
    el
}

/// Create a `<p:sld>` element (`OutlineViewSlideListEntry`).
pub fn outline_view_slide_list_entry() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sld")
}

/// Set `Id` (`r:id`) on a `OutlineViewSlideListEntry` element.
pub fn outline_view_slide_list_entry_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Set `Collapse` (`:collapse`) on a `OutlineViewSlideListEntry` element.
pub fn outline_view_slide_list_entry_with_collapse(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("collapse", value);
    el
}

/// Create a `<p:sldLst>` element (`OutlineViewSlideList`).
pub fn outline_view_slide_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldLst").with_children(children)
}

/// Create a `<p:guide>` element (`Guide`).
pub fn guide() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "guide")
}

/// Set `Orientation` (`:orient`) on a `Guide` element.
pub fn guide_with_orientation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("orient", value);
    el
}

/// Set `Position` (`:pos`) on a `Guide` element.
pub fn guide_with_position(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pos", value);
    el
}

/// Create a `<p:guideLst>` element (`GuideList`).
pub fn guide_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "guideLst").with_children(children)
}

/// Create a `<p:cSldViewPr>` element (`CommonSlideViewProperties`).
pub fn common_slide_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cSldViewPr").with_children(children)
}

/// Set `SnapToGrid` (`:snapToGrid`) on a `CommonSlideViewProperties` element.
pub fn common_slide_view_properties_with_snap_to_grid(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("snapToGrid", value);
    el
}

/// Set `SnapToObjects` (`:snapToObjects`) on a `CommonSlideViewProperties` element.
pub fn common_slide_view_properties_with_snap_to_objects(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("snapToObjects", value);
    el
}

/// Set `ShowGuides` (`:showGuides`) on a `CommonSlideViewProperties` element.
pub fn common_slide_view_properties_with_show_guides(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showGuides", value);
    el
}

/// Create a `<p:normalViewPr>` element (`NormalViewProperties`).
pub fn normal_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "normalViewPr").with_children(children)
}

/// Set `ShowOutlineIcons` (`:showOutlineIcons`) on a `NormalViewProperties` element.
pub fn normal_view_properties_with_show_outline_icons(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showOutlineIcons", value);
    el
}

/// Set `SnapVerticalSplitter` (`:snapVertSplitter`) on a `NormalViewProperties` element.
pub fn normal_view_properties_with_snap_vertical_splitter(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("snapVertSplitter", value);
    el
}

/// Set `VerticalBarState` (`:vertBarState`) on a `NormalViewProperties` element.
pub fn normal_view_properties_with_vertical_bar_state(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("vertBarState", value);
    el
}

/// Set `HorizontalBarState` (`:horzBarState`) on a `NormalViewProperties` element.
pub fn normal_view_properties_with_horizontal_bar_state(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("horzBarState", value);
    el
}

/// Set `PreferSingleView` (`:preferSingleView`) on a `NormalViewProperties` element.
pub fn normal_view_properties_with_prefer_single_view(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("preferSingleView", value);
    el
}

/// Create a `<p:slideViewPr>` element (`SlideViewProperties`).
pub fn slide_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "slideViewPr").with_children(children)
}

/// Create a `<p:outlineViewPr>` element (`OutlineViewProperties`).
pub fn outline_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "outlineViewPr").with_children(children)
}

/// Create a `<p:notesTextViewPr>` element (`NotesTextViewProperties`).
pub fn notes_text_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notesTextViewPr").with_children(children)
}

/// Create a `<p:sorterViewPr>` element (`SorterViewProperties`).
pub fn sorter_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sorterViewPr").with_children(children)
}

/// Set `ShowFormatting` (`:showFormatting`) on a `SorterViewProperties` element.
pub fn sorter_view_properties_with_show_formatting(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showFormatting", value);
    el
}

/// Create a `<p:notesViewPr>` element (`NotesViewProperties`).
pub fn notes_view_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notesViewPr").with_children(children)
}

/// Create a `<p:gridSpacing>` element (`GridSpacing`).
pub fn grid_spacing() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "gridSpacing")
}

/// Set `Cx` (`:cx`) on a `GridSpacing` element.
pub fn grid_spacing_with_cx(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cx", value);
    el
}

/// Set `Cy` (`:cy`) on a `GridSpacing` element.
pub fn grid_spacing_with_cy(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cy", value);
    el
}

/// Create a `<p:notesSz>` element (`NotesSize`).
pub fn notes_size() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notesSz")
}

/// Set `Cx` (`:cx`) on a `NotesSize` element.
pub fn notes_size_with_cx(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cx", value);
    el
}

/// Set `Cy` (`:cy`) on a `NotesSize` element.
pub fn notes_size_with_cy(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cy", value);
    el
}

/// Create a `<p:ext>` element (`SlideExtension`).
pub fn slide_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ext>` element (`CommonSlideDataExtension`).
pub fn common_slide_data_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ext>` element (`ShowPropertiesExtension`).
pub fn show_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:pic>` element (`Picture`).
pub fn picture(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "pic").with_children(children)
}

/// Create a `<p:embed>` element (`OleObjectEmbed`).
pub fn ole_object_embed(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "embed").with_children(children)
}

/// Set `FollowColorScheme` (`:followColorScheme`) on a `OleObjectEmbed` element.
pub fn ole_object_embed_with_follow_color_scheme(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("followColorScheme", value);
    el
}

/// Create a `<p:link>` element (`OleObjectLink`).
pub fn ole_object_link(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "link").with_children(children)
}

/// Set `AutoUpdate` (`:updateAutomatic`) on a `OleObjectLink` element.
pub fn ole_object_link_with_auto_update(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("updateAutomatic", value);
    el
}

/// Create a `<p:transition>` element (`Transition`).
pub fn transition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "transition").with_children(children)
}

/// Set `Speed` (`:spd`) on a `Transition` element.
pub fn transition_with_speed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spd", value);
    el
}

/// Set `Duration` (`p14:dur`) on a `Transition` element.
pub fn transition_with_duration(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("p14:dur", value);
    el
}

/// Set `AdvanceOnClick` (`:advClick`) on a `Transition` element.
pub fn transition_with_advance_on_click(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("advClick", value);
    el
}

/// Set `AdvanceAfterTime` (`:advTm`) on a `Transition` element.
pub fn transition_with_advance_after_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("advTm", value);
    el
}

/// Create a `<p:timing>` element (`Timing`).
pub fn timing(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "timing").with_children(children)
}

/// Create a `<p:extLst>` element (`SlideExtensionList`).
pub fn slide_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:bg>` element (`Background`).
pub fn background(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "bg").with_children(children)
}

/// Set `BlackWhiteMode` (`:bwMode`) on a `Background` element.
pub fn background_with_black_white_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bwMode", value);
    el
}

/// Create a `<p:spTree>` element (`ShapeTree`).
pub fn shape_tree(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "spTree").with_children(children)
}

/// Create a `<p:grpSp>` element (`GroupShape`).
pub fn group_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "grpSp").with_children(children)
}

/// Create a `<p:custDataLst>` element (`CustomerDataList`).
pub fn customer_data_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "custDataLst").with_children(children)
}

/// Create a `<p:controls>` element (`ControlList`).
pub fn control_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "controls").with_children(children)
}

/// Create a `<p:extLst>` element (`CommonSlideDataExtensionList`).
pub fn common_slide_data_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:nvGrpSpPr>` element (`NonVisualGroupShapeProperties`).
pub fn non_visual_group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "nvGrpSpPr").with_children(children)
}

/// Create a `<p:grpSpPr>` element (`GroupShapeProperties`).
pub fn group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Set `BlackWhiteMode` (`:bwMode`) on a `GroupShapeProperties` element.
pub fn group_shape_properties_with_black_white_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bwMode", value);
    el
}

/// Create a `<p:sp>` element (`Shape`).
pub fn shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sp").with_children(children)
}

/// Set `UseBackgroundFill` (`:useBgFill`) on a `Shape` element.
pub fn shape_with_use_background_fill(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("useBgFill", value);
    el
}

/// Create a `<p:graphicFrame>` element (`GraphicFrame`).
pub fn graphic_frame(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "graphicFrame").with_children(children)
}

/// Create a `<p:cxnSp>` element (`ConnectionShape`).
pub fn connection_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cxnSp").with_children(children)
}

/// Create a `<p:extLst>` element (`ShowPropertiesExtensionList`).
pub fn show_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:spTgt>` element (`ShapeTarget`).
pub fn shape_target(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "spTgt").with_children(children)
}

/// Set `ShapeId` (`:spid`) on a `ShapeTarget` element.
pub fn shape_target_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Create a `<p:inkTgt>` element (`InkTarget`).
pub fn ink_target() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "inkTgt")
}

/// Set `ShapeId` (`:spid`) on a `InkTarget` element.
pub fn ink_target_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Create `<p:inkTgt>` with `ShapeId` set.
pub fn ink_target_shape_id(value: impl Into<String>) -> OpenXmlElement {
    ink_target_with_shape_id(ink_target(), value)
}

/// Create a `<p:subSp>` element (`SubShape`).
pub fn sub_shape() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "subSp")
}

/// Set `ShapeId` (`:spid`) on a `SubShape` element.
pub fn sub_shape_with_shape_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spid", value);
    el
}

/// Create `<p:subSp>` with `ShapeId` set.
pub fn sub_shape_shape_id(value: impl Into<String>) -> OpenXmlElement {
    sub_shape_with_shape_id(sub_shape(), value)
}

/// Create a `<p:ext>` element (`CommentAuthorExtension`).
pub fn comment_author_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ext>` element (`CommentExtension`).
pub fn comment_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ext>` element (`SlideLayoutExtension`).
pub fn slide_layout_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ext>` element (`SlideMasterExtension`).
pub fn slide_master_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ext>` element (`HandoutMasterExtension`).
pub fn handout_master_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ext>` element (`NotesMasterExtension`).
pub fn notes_master_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:ph>` element (`PlaceholderShape`).
pub fn placeholder_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ph").with_children(children)
}

/// Set `Type` (`:type`) on a `PlaceholderShape` element.
pub fn placeholder_shape_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Orientation` (`:orient`) on a `PlaceholderShape` element.
pub fn placeholder_shape_with_orientation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("orient", value);
    el
}

/// Set `Size` (`:sz`) on a `PlaceholderShape` element.
pub fn placeholder_shape_with_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sz", value);
    el
}

/// Set `Index` (`:idx`) on a `PlaceholderShape` element.
pub fn placeholder_shape_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Set `HasCustomPrompt` (`:hasCustomPrompt`) on a `PlaceholderShape` element.
pub fn placeholder_shape_with_has_custom_prompt(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hasCustomPrompt", value);
    el
}

/// Create a `<p:extLst>` element (`ApplicationNonVisualDrawingPropertiesExtensionList`).
pub fn application_non_visual_drawing_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:ext>` element (`ApplicationNonVisualDrawingPropertiesExtension`).
pub fn application_non_visual_drawing_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:iterate>` element (`Iterate`).
pub fn iterate(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "iterate").with_children(children)
}

/// Set `Type` (`:type`) on a `Iterate` element.
pub fn iterate_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Backwards` (`:backwards`) on a `Iterate` element.
pub fn iterate_with_backwards(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("backwards", value);
    el
}

/// Create a `<p:childTnLst>` element (`ChildTimeNodeList`).
pub fn child_time_node_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "childTnLst").with_children(children)
}

/// Create a `<p:subTnLst>` element (`SubTimeNodeList`).
pub fn sub_time_node_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "subTnLst").with_children(children)
}

/// Create a `<p:tavLst>` element (`TimeAnimateValueList`).
pub fn time_animate_value_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "tavLst").with_children(children)
}

/// Create a `<p:by>` element (`ByPosition`).
pub fn by_position() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "by")
}

/// Set `X` (`:x`) on a `ByPosition` element.
pub fn by_position_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `ByPosition` element.
pub fn by_position_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<p:from>` element (`FromPosition`).
pub fn from_position() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "from")
}

/// Set `X` (`:x`) on a `FromPosition` element.
pub fn from_position_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `FromPosition` element.
pub fn from_position_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<p:to>` element (`ToPosition`).
pub fn to_position() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "to")
}

/// Set `X` (`:x`) on a `ToPosition` element.
pub fn to_position_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `ToPosition` element.
pub fn to_position_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<p:rCtr>` element (`RotationCenter`).
pub fn rotation_center() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "rCtr")
}

/// Set `X` (`:x`) on a `RotationCenter` element.
pub fn rotation_center_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `RotationCenter` element.
pub fn rotation_center_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<p:extLst>` element (`CommentAuthorExtensionList`).
pub fn comment_author_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:extLst>` element (`CommentExtensionList`).
pub fn comment_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:sldMasterIdLst>` element (`SlideMasterIdList`).
pub fn slide_master_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldMasterIdLst").with_children(children)
}

/// Create a `<p:notesMasterIdLst>` element (`NotesMasterIdList`).
pub fn notes_master_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "notesMasterIdLst").with_children(children)
}

/// Create a `<p:handoutMasterIdLst>` element (`HandoutMasterIdList`).
pub fn handout_master_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "handoutMasterIdLst").with_children(children)
}

/// Create a `<p:sldIdLst>` element (`SlideIdList`).
pub fn slide_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldIdLst").with_children(children)
}

/// Create a `<p:sldSz>` element (`SlideSize`).
pub fn slide_size() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldSz")
}

/// Set `Cx` (`:cx`) on a `SlideSize` element.
pub fn slide_size_with_cx(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cx", value);
    el
}

/// Set `Cy` (`:cy`) on a `SlideSize` element.
pub fn slide_size_with_cy(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cy", value);
    el
}

/// Set `Type` (`:type`) on a `SlideSize` element.
pub fn slide_size_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Create a `<p:embeddedFontLst>` element (`EmbeddedFontList`).
pub fn embedded_font_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "embeddedFontLst").with_children(children)
}

/// Create a `<p:custShowLst>` element (`CustomShowList`).
pub fn custom_show_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "custShowLst").with_children(children)
}

/// Create a `<p:photoAlbum>` element (`PhotoAlbum`).
pub fn photo_album(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "photoAlbum").with_children(children)
}

/// Set `BlackWhite` (`:bw`) on a `PhotoAlbum` element.
pub fn photo_album_with_black_white(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bw", value);
    el
}

/// Set `ShowCaptions` (`:showCaptions`) on a `PhotoAlbum` element.
pub fn photo_album_with_show_captions(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showCaptions", value);
    el
}

/// Set `Layout` (`:layout`) on a `PhotoAlbum` element.
pub fn photo_album_with_layout(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("layout", value);
    el
}

/// Set `Frame` (`:frame`) on a `PhotoAlbum` element.
pub fn photo_album_with_frame(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("frame", value);
    el
}

/// Create a `<p:kinsoku>` element (`Kinsoku`).
pub fn kinsoku() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "kinsoku")
}

/// Set `Language` (`:lang`) on a `Kinsoku` element.
pub fn kinsoku_with_language(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lang", value);
    el
}

/// Set `InvalidStartChars` (`:invalStChars`) on a `Kinsoku` element.
pub fn kinsoku_with_invalid_start_chars(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("invalStChars", value);
    el
}

/// Set `InvalidEndChars` (`:invalEndChars`) on a `Kinsoku` element.
pub fn kinsoku_with_invalid_end_chars(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("invalEndChars", value);
    el
}

/// Create a `<p:modifyVerifier>` element (`ModificationVerifier`).
pub fn modification_verifier() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "modifyVerifier")
}

/// Set `CryptographicProviderType` (`:cryptProviderType`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_cryptographic_provider_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cryptProviderType", value);
    el
}

/// Set `CryptographicAlgorithmClass` (`:cryptAlgorithmClass`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_cryptographic_algorithm_class(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cryptAlgorithmClass", value);
    el
}

/// Set `CryptographicAlgorithmType` (`:cryptAlgorithmType`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_cryptographic_algorithm_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cryptAlgorithmType", value);
    el
}

/// Set `CryptographicAlgorithmSid` (`:cryptAlgorithmSid`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_cryptographic_algorithm_sid(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cryptAlgorithmSid", value);
    el
}

/// Set `SpinCount` (`:spinCount`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_spin_count(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spinCount", value);
    el
}

/// Set `SaltData` (`:saltData`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_salt_data(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("saltData", value);
    el
}

/// Set `HashData` (`:hashData`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_hash_data(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hashData", value);
    el
}

/// Set `CryptographicProvider` (`:cryptProvider`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_cryptographic_provider(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cryptProvider", value);
    el
}

/// Set `ExtendedCryptographicAlgorithm` (`:algIdExt`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_extended_cryptographic_algorithm(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algIdExt", value);
    el
}

/// Set `ExtendedCryptographicAlgorithmSource` (`:algIdExtSource`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_extended_cryptographic_algorithm_source(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algIdExtSource", value);
    el
}

/// Set `CryptographicProviderTypeExtensibility` (`:cryptProviderTypeExt`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_cryptographic_provider_type_extensibility(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cryptProviderTypeExt", value);
    el
}

/// Set `CryptographicProviderTypeExtensibilitySource` (`:cryptProviderTypeExtSource`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_cryptographic_provider_type_extensibility_source(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cryptProviderTypeExtSource", value);
    el
}

/// Set `AlgorithmName` (`:algorithmName`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_algorithm_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algorithmName", value);
    el
}

/// Set `HashValue` (`:hashValue`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_hash_value(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hashValue", value);
    el
}

/// Set `SaltValue` (`:saltValue`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_salt_value(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("saltValue", value);
    el
}

/// Set `SpinValue` (`:spinValue`) on a `ModificationVerifier` element.
pub fn modification_verifier_with_spin_value(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spinValue", value);
    el
}

/// Create a `<p:extLst>` element (`PresentationExtensionList`).
pub fn presentation_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:ext>` element (`PresentationExtension`).
pub fn presentation_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:htmlPubPr>` element (`HtmlPublishProperties`).
pub fn html_publish_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "htmlPubPr").with_children(children)
}

/// Set `ShowSpeakerNotes` (`:showSpeakerNotes`) on a `HtmlPublishProperties` element.
pub fn html_publish_properties_with_show_speaker_notes(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showSpeakerNotes", value);
    el
}

/// Set `TargetBrowser` (`:pubBrowser`) on a `HtmlPublishProperties` element.
pub fn html_publish_properties_with_target_browser(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pubBrowser", value);
    el
}

/// Set `Id` (`r:id`) on a `HtmlPublishProperties` element.
pub fn html_publish_properties_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Create a `<p:webPr>` element (`WebProperties`).
pub fn web_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "webPr").with_children(children)
}

/// Set `ShowAnimation` (`:showAnimation`) on a `WebProperties` element.
pub fn web_properties_with_show_animation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showAnimation", value);
    el
}

/// Set `ResizeGraphics` (`:resizeGraphics`) on a `WebProperties` element.
pub fn web_properties_with_resize_graphics(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("resizeGraphics", value);
    el
}

/// Set `AllowPng` (`:allowPng`) on a `WebProperties` element.
pub fn web_properties_with_allow_png(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("allowPng", value);
    el
}

/// Set `RelyOnVml` (`:relyOnVml`) on a `WebProperties` element.
pub fn web_properties_with_rely_on_vml(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("relyOnVml", value);
    el
}

/// Set `OrganizeInFolders` (`:organizeInFolders`) on a `WebProperties` element.
pub fn web_properties_with_organize_in_folders(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("organizeInFolders", value);
    el
}

/// Set `UseLongFilenames` (`:useLongFilenames`) on a `WebProperties` element.
pub fn web_properties_with_use_long_filenames(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("useLongFilenames", value);
    el
}

/// Set `ImageSize` (`:imgSz`) on a `WebProperties` element.
pub fn web_properties_with_image_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("imgSz", value);
    el
}

/// Set `Encoding` (`:encoding`) on a `WebProperties` element.
pub fn web_properties_with_encoding(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("encoding", value);
    el
}

/// Set `Color` (`:clr`) on a `WebProperties` element.
pub fn web_properties_with_color(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("clr", value);
    el
}

/// Create a `<p:prnPr>` element (`PrintingProperties`).
pub fn printing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "prnPr").with_children(children)
}

/// Set `PrintWhat` (`:prnWhat`) on a `PrintingProperties` element.
pub fn printing_properties_with_print_what(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prnWhat", value);
    el
}

/// Set `ColorMode` (`:clrMode`) on a `PrintingProperties` element.
pub fn printing_properties_with_color_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("clrMode", value);
    el
}

/// Set `HiddenSlides` (`:hiddenSlides`) on a `PrintingProperties` element.
pub fn printing_properties_with_hidden_slides(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hiddenSlides", value);
    el
}

/// Set `ScaleToFitPaper` (`:scaleToFitPaper`) on a `PrintingProperties` element.
pub fn printing_properties_with_scale_to_fit_paper(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("scaleToFitPaper", value);
    el
}

/// Set `FrameSlides` (`:frameSlides`) on a `PrintingProperties` element.
pub fn printing_properties_with_frame_slides(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("frameSlides", value);
    el
}

/// Create a `<p:showPr>` element (`ShowProperties`).
pub fn show_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "showPr").with_children(children)
}

/// Set `Loop` (`:loop`) on a `ShowProperties` element.
pub fn show_properties_with_loop_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("loop", value);
    el
}

/// Set `ShowNarration` (`:showNarration`) on a `ShowProperties` element.
pub fn show_properties_with_show_narration(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showNarration", value);
    el
}

/// Set `ShowAnimation` (`:showAnimation`) on a `ShowProperties` element.
pub fn show_properties_with_show_animation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("showAnimation", value);
    el
}

/// Set `UseTimings` (`:useTimings`) on a `ShowProperties` element.
pub fn show_properties_with_use_timings(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("useTimings", value);
    el
}

/// Create a `<p:clrMru>` element (`ColorMostRecentlyUsed`).
pub fn color_most_recently_used(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "clrMru").with_children(children)
}

/// Create a `<p:extLst>` element (`PresentationPropertiesExtensionList`).
pub fn presentation_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:ext>` element (`PresentationPropertiesExtension`).
pub fn presentation_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<p:hf>` element (`HeaderFooter`).
pub fn header_footer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "hf").with_children(children)
}

/// Set `SlideNumber` (`:sldNum`) on a `HeaderFooter` element.
pub fn header_footer_with_slide_number(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sldNum", value);
    el
}

/// Set `Header` (`:hdr`) on a `HeaderFooter` element.
pub fn header_footer_with_header(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hdr", value);
    el
}

/// Set `Footer` (`:ftr`) on a `HeaderFooter` element.
pub fn header_footer_with_footer(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ftr", value);
    el
}

/// Set `DateTime` (`:dt`) on a `HeaderFooter` element.
pub fn header_footer_with_date_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dt", value);
    el
}

/// Create a `<p:extLst>` element (`SlideLayoutExtensionList`).
pub fn slide_layout_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:sldLayoutIdLst>` element (`SlideLayoutIdList`).
pub fn slide_layout_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sldLayoutIdLst").with_children(children)
}

/// Create a `<p:txStyles>` element (`TextStyles`).
pub fn text_styles(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "txStyles").with_children(children)
}

/// Create a `<p:extLst>` element (`SlideMasterExtensionList`).
pub fn slide_master_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:extLst>` element (`HandoutMasterExtensionList`).
pub fn handout_master_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:extLst>` element (`NotesMasterExtensionList`).
pub fn notes_master_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<p:oleChartEl>` element (`OleChartElement`).
pub fn ole_chart_element() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "oleChartEl")
}

/// Set `Type` (`:type`) on a `OleChartElement` element.
pub fn ole_chart_element_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Level` (`:lvl`) on a `OleChartElement` element.
pub fn ole_chart_element_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Create a `<p:txEl>` element (`TextElement`).
pub fn text_element(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "txEl").with_children(children)
}

/// Create a `<p:graphicEl>` element (`GraphicElement`).
pub fn graphic_element(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "graphicEl").with_children(children)
}

/// Create a `<p:blinds>` element (`BlindsTransition`).
pub fn blinds_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "blinds")
}

/// Set `Direction` (`:dir`) on a `BlindsTransition` element.
pub fn blinds_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:blinds>` with `Direction` set.
pub fn blinds_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    blinds_transition_with_direction(blinds_transition(), value)
}

/// Create a `<p:checker>` element (`CheckerTransition`).
pub fn checker_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "checker")
}

/// Set `Direction` (`:dir`) on a `CheckerTransition` element.
pub fn checker_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:checker>` with `Direction` set.
pub fn checker_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    checker_transition_with_direction(checker_transition(), value)
}

/// Create a `<p:comb>` element (`CombTransition`).
pub fn comb_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "comb")
}

/// Set `Direction` (`:dir`) on a `CombTransition` element.
pub fn comb_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:comb>` with `Direction` set.
pub fn comb_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    comb_transition_with_direction(comb_transition(), value)
}

/// Create a `<p:randomBar>` element (`RandomBarTransition`).
pub fn random_bar_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "randomBar")
}

/// Set `Direction` (`:dir`) on a `RandomBarTransition` element.
pub fn random_bar_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:randomBar>` with `Direction` set.
pub fn random_bar_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    random_bar_transition_with_direction(random_bar_transition(), value)
}

/// Create a `<p:cover>` element (`CoverTransition`).
pub fn cover_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cover")
}

/// Set `Direction` (`:dir`) on a `CoverTransition` element.
pub fn cover_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:cover>` with `Direction` set.
pub fn cover_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    cover_transition_with_direction(cover_transition(), value)
}

/// Create a `<p:pull>` element (`PullTransition`).
pub fn pull_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "pull")
}

/// Set `Direction` (`:dir`) on a `PullTransition` element.
pub fn pull_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:pull>` with `Direction` set.
pub fn pull_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    pull_transition_with_direction(pull_transition(), value)
}

/// Create a `<p:cut>` element (`CutTransition`).
pub fn cut_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "cut")
}

/// Set `ThroughBlack` (`:thruBlk`) on a `CutTransition` element.
pub fn cut_transition_with_through_black(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("thruBlk", value);
    el
}

/// Create `<p:cut>` with `ThroughBlack` set.
pub fn cut_transition_through_black(value: impl Into<String>) -> OpenXmlElement {
    cut_transition_with_through_black(cut_transition(), value)
}

/// Create a `<p:fade>` element (`FadeTransition`).
pub fn fade_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "fade")
}

/// Set `ThroughBlack` (`:thruBlk`) on a `FadeTransition` element.
pub fn fade_transition_with_through_black(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("thruBlk", value);
    el
}

/// Create `<p:fade>` with `ThroughBlack` set.
pub fn fade_transition_through_black(value: impl Into<String>) -> OpenXmlElement {
    fade_transition_with_through_black(fade_transition(), value)
}

/// Create a `<p:push>` element (`PushTransition`).
pub fn push_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "push")
}

/// Set `Direction` (`:dir`) on a `PushTransition` element.
pub fn push_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:push>` with `Direction` set.
pub fn push_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    push_transition_with_direction(push_transition(), value)
}

/// Create a `<p:wipe>` element (`WipeTransition`).
pub fn wipe_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "wipe")
}

/// Set `Direction` (`:dir`) on a `WipeTransition` element.
pub fn wipe_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:wipe>` with `Direction` set.
pub fn wipe_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    wipe_transition_with_direction(wipe_transition(), value)
}

/// Create a `<p:split>` element (`SplitTransition`).
pub fn split_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "split")
}

/// Set `Orientation` (`:orient`) on a `SplitTransition` element.
pub fn split_transition_with_orientation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("orient", value);
    el
}

/// Set `Direction` (`:dir`) on a `SplitTransition` element.
pub fn split_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create a `<p:strips>` element (`StripsTransition`).
pub fn strips_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "strips")
}

/// Set `Direction` (`:dir`) on a `StripsTransition` element.
pub fn strips_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:strips>` with `Direction` set.
pub fn strips_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    strips_transition_with_direction(strips_transition(), value)
}

/// Create a `<p:wheel>` element (`WheelTransition`).
pub fn wheel_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "wheel")
}

/// Set `Spokes` (`:spokes`) on a `WheelTransition` element.
pub fn wheel_transition_with_spokes(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spokes", value);
    el
}

/// Create `<p:wheel>` with `Spokes` set.
pub fn wheel_transition_spokes(value: impl Into<String>) -> OpenXmlElement {
    wheel_transition_with_spokes(wheel_transition(), value)
}

/// Create a `<p:zoom>` element (`ZoomTransition`).
pub fn zoom_transition() -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "zoom")
}

/// Set `Direction` (`:dir`) on a `ZoomTransition` element.
pub fn zoom_transition_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create `<p:zoom>` with `Direction` set.
pub fn zoom_transition_direction(value: impl Into<String>) -> OpenXmlElement {
    zoom_transition_with_direction(zoom_transition(), value)
}

/// Create a `<p:sndAc>` element (`SoundAction`).
pub fn sound_action(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "sndAc").with_children(children)
}

/// Create a `<p:ext>` element (`PlaceholderExtension`).
pub fn placeholder_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", NAMESPACE_URI, "ext").with_children(children)
}

// ---------------------------------------------------------------------------
// Schema particles (content models)
// ---------------------------------------------------------------------------

use crate::validation::{Occurs, Particle};

/// Content model particle for `Extension`.
pub fn particle_extension() -> Particle {
    Particle::sequence(vec![
      Particle::any(Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ColorMap`.
pub fn particle_color_map() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ColorMapOverride`.
pub fn particle_color_map_override() -> Particle {
    Particle::sequence(vec![
      Particle::choice(vec![
          Particle::element("masterClrMapping", Occurs::STAR),
          Particle::element("overrideClrMapping", Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `BackgroundProperties`.
pub fn particle_background_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BackgroundStyleReference`.
pub fn particle_background_style_reference() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CommentPropertiesExtension`.
pub fn particle_comment_properties_extension() -> Particle {
    Particle::sequence(vec![
      Particle::element("taskDetails", Occurs::OPTIONAL),
      Particle::element("reactions", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CommentAuthorList`.
pub fn particle_comment_author_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("cmAuthor", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommentList`.
pub fn particle_comment_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("cm", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `OleObject`.
pub fn particle_ole_object() -> Particle {
    Particle::sequence(vec![
      Particle::choice(vec![
          Particle::element("embed", Occurs::STAR),
          Particle::element("link", Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("pic", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Presentation`.
pub fn particle_presentation() -> Particle {
    Particle::sequence(vec![
      Particle::element("sldMasterIdLst", Occurs::OPTIONAL),
      Particle::element("notesMasterIdLst", Occurs::OPTIONAL),
      Particle::element("handoutMasterIdLst", Occurs::OPTIONAL),
      Particle::element("sldIdLst", Occurs::OPTIONAL),
      Particle::element("sldSz", Occurs::OPTIONAL),
      Particle::element("notesSz", Occurs::STAR),
      Particle::element("embeddedFontLst", Occurs::OPTIONAL),
      Particle::element("custShowLst", Occurs::OPTIONAL),
      Particle::element("photoAlbum", Occurs::OPTIONAL),
      Particle::element("custDataLst", Occurs::OPTIONAL),
      Particle::element("kinsoku", Occurs::OPTIONAL),
      Particle::element("defaultTextStyle", Occurs::OPTIONAL),
      Particle::element("modifyVerifier", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PresentationProperties`.
pub fn particle_presentation_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("htmlPubPr", Occurs::OPTIONAL),
      Particle::element("webPr", Occurs::OPTIONAL),
      Particle::element("prnPr", Occurs::OPTIONAL),
      Particle::element("showPr", Occurs::OPTIONAL),
      Particle::element("clrMru", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Slide`.
pub fn particle_slide() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSld", Occurs::STAR),
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("clrMapOvr", Occurs::OPTIONAL),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("transition", Occurs::OPTIONAL),
      Particle::element("timing", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideLayout`.
pub fn particle_slide_layout() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSld", Occurs::STAR),
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("clrMapOvr", Occurs::OPTIONAL),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("transition", Occurs::OPTIONAL),
      Particle::element("timing", Occurs::OPTIONAL),
      Particle::element("hf", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideMaster`.
pub fn particle_slide_master() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSld", Occurs::STAR),
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("clrMap", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("sldLayoutIdLst", Occurs::OPTIONAL),
      Particle::element("transition", Occurs::OPTIONAL),
      Particle::element("timing", Occurs::OPTIONAL),
      Particle::element("hf", Occurs::OPTIONAL),
      Particle::element("txStyles", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HandoutMaster`.
pub fn particle_handout_master() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSld", Occurs::STAR),
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("clrMap", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("hf", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NotesMaster`.
pub fn particle_notes_master() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSld", Occurs::STAR),
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("clrMap", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("hf", Occurs::OPTIONAL),
      Particle::element("notesStyle", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NotesSlide`.
pub fn particle_notes_slide() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSld", Occurs::STAR),
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("clrMapOvr", Occurs::OPTIONAL),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideSyncProperties`.
pub fn particle_slide_sync_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TagList`.
pub fn particle_tag_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("tag", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ViewProperties`.
pub fn particle_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("normalViewPr", Occurs::OPTIONAL),
      Particle::element("slideViewPr", Occurs::OPTIONAL),
      Particle::element("outlineViewPr", Occurs::OPTIONAL),
      Particle::element("notesTextViewPr", Occurs::OPTIONAL),
      Particle::element("sorterViewPr", Occurs::OPTIONAL),
      Particle::element("notesViewPr", Occurs::OPTIONAL),
      Particle::element("gridSpacing", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `ContentPart`.
pub fn particle_content_part() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvContentPartPr", Occurs::OPTIONAL),
      Particle::element("xfrm", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `StartSoundAction`.
pub fn particle_start_sound_action() -> Particle {
    Particle::sequence(vec![
      Particle::element("snd", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TargetElement`.
pub fn particle_target_element() -> Particle {
    Particle::choice(vec![
      Particle::element("sldTgt", Occurs::STAR),
      Particle::element("sndTgt", Occurs::STAR),
      Particle::element("spTgt", Occurs::STAR),
      Particle::element("inkTgt", Occurs::STAR),
      Particle::element("bmkTgt", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Condition`.
pub fn particle_condition() -> Particle {
    Particle::choice(vec![
      Particle::element("tgtEl", Occurs::STAR),
      Particle::element("tn", Occurs::STAR),
      Particle::element("rtn", Occurs::STAR),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `EndSync`.
pub fn particle_end_sync() -> Particle {
    Particle::choice(vec![
      Particle::element("tgtEl", Occurs::STAR),
      Particle::element("tn", Occurs::STAR),
      Particle::element("rtn", Occurs::STAR),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `ParallelTimeNode`.
pub fn particle_parallel_time_node() -> Particle {
    Particle::sequence(vec![
      Particle::element("cTn", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SequenceTimeNode`.
pub fn particle_sequence_time_node() -> Particle {
    Particle::sequence(vec![
      Particle::element("cTn", Occurs::STAR),
      Particle::element("prevCondLst", Occurs::OPTIONAL),
      Particle::element("nextCondLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ExclusiveTimeNode`.
pub fn particle_exclusive_time_node() -> Particle {
    Particle::sequence(vec![
      Particle::element("cTn", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Animate`.
pub fn particle_animate() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
      Particle::element("tavLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `AnimateColor`.
pub fn particle_animate_color() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
      Particle::element("by", Occurs::OPTIONAL),
      Particle::element("from", Occurs::OPTIONAL),
      Particle::element("to", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `AnimateEffect`.
pub fn particle_animate_effect() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
      Particle::element("progress", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `AnimateMotion`.
pub fn particle_animate_motion() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
      Particle::element("by", Occurs::OPTIONAL),
      Particle::element("from", Occurs::OPTIONAL),
      Particle::element("to", Occurs::OPTIONAL),
      Particle::element("rCtr", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `AnimateRotation`.
pub fn particle_animate_rotation() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `AnimateScale`.
pub fn particle_animate_scale() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
      Particle::element("by", Occurs::OPTIONAL),
      Particle::element("from", Occurs::OPTIONAL),
      Particle::element("to", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Command`.
pub fn particle_command() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SetBehavior`.
pub fn particle_set_behavior() -> Particle {
    Particle::sequence(vec![
      Particle::element("cBhvr", Occurs::STAR),
      Particle::element("to", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Audio`.
pub fn particle_audio() -> Particle {
    Particle::sequence(vec![
      Particle::element("cMediaNode", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Video`.
pub fn particle_video() -> Particle {
    Particle::sequence(vec![
      Particle::element("cMediaNode", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommonTimeNode`.
pub fn particle_common_time_node() -> Particle {
    Particle::sequence(vec![
      Particle::element("stCondLst", Occurs::OPTIONAL),
      Particle::element("endCondLst", Occurs::OPTIONAL),
      Particle::element("endSync", Occurs::OPTIONAL),
      Particle::element("iterate", Occurs::OPTIONAL),
      Particle::element("childTnLst", Occurs::OPTIONAL),
      Particle::element("subTnLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PreviousConditionList`.
pub fn particle_previous_condition_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("cond", Occurs::PLUS),
  ], Occurs::STAR)
}

/// Content model particle for `NextConditionList`.
pub fn particle_next_condition_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("cond", Occurs::PLUS),
  ], Occurs::STAR)
}

/// Content model particle for `StartConditionList`.
pub fn particle_start_condition_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("cond", Occurs::PLUS),
  ], Occurs::STAR)
}

/// Content model particle for `EndConditionList`.
pub fn particle_end_condition_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("cond", Occurs::PLUS),
  ], Occurs::STAR)
}

/// Content model particle for `AttributeNameList`.
pub fn particle_attribute_name_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("attrName", Occurs::PLUS),
  ], Occurs::STAR)
}

/// Content model particle for `ColorValue`.
pub fn particle_color_value() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PenColor`.
pub fn particle_pen_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TimeAnimateValue`.
pub fn particle_time_animate_value() -> Particle {
    Particle::sequence(vec![
      Particle::element("val", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CommonBehavior`.
pub fn particle_common_behavior() -> Particle {
    Particle::sequence(vec![
      Particle::element("cTn", Occurs::STAR),
      Particle::element("tgtEl", Occurs::STAR),
      Particle::element("attrNameLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Progress`.
pub fn particle_progress() -> Particle {
    Particle::choice(vec![
      Particle::element("fltVal", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ToVariantValue`.
pub fn particle_to_variant_value() -> Particle {
    Particle::choice(vec![
      Particle::element("boolVal", Occurs::STAR),
      Particle::element("intVal", Occurs::STAR),
      Particle::element("fltVal", Occurs::STAR),
      Particle::element("strVal", Occurs::STAR),
      Particle::element("clrVal", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `VariantValue`.
pub fn particle_variant_value() -> Particle {
    Particle::choice(vec![
      Particle::element("boolVal", Occurs::STAR),
      Particle::element("intVal", Occurs::STAR),
      Particle::element("fltVal", Occurs::STAR),
      Particle::element("strVal", Occurs::STAR),
      Particle::element("clrVal", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommonMediaNode`.
pub fn particle_common_media_node() -> Particle {
    Particle::sequence(vec![
      Particle::element("cTn", Occurs::STAR),
      Particle::element("tgtEl", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TimeNodeList`.
pub fn particle_time_node_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("par", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Template`.
pub fn particle_template() -> Particle {
    Particle::sequence(vec![
      Particle::element("tnLst", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TemplateList`.
pub fn particle_template_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("tmpl", Occurs::new(0, Some(9))),
  ], Occurs::STAR)
}

/// Content model particle for `BuildSubElement`.
pub fn particle_build_sub_element() -> Particle {
    Particle::choice(vec![
      Particle::element("bldDgm", Occurs::STAR),
      Particle::element("bldChart", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `BuildParagraph`.
pub fn particle_build_paragraph() -> Particle {
    Particle::sequence(vec![
      Particle::element("tmplLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BuildGraphics`.
pub fn particle_build_graphics() -> Particle {
    Particle::choice(vec![
      Particle::element("bldAsOne", Occurs::STAR),
      Particle::element("bldSub", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `BuildList`.
pub fn particle_build_list() -> Particle {
    Particle::choice(vec![
      Particle::element("bldP", Occurs::STAR),
      Particle::element("bldDgm", Occurs::STAR),
      Particle::element("bldOleChart", Occurs::STAR),
      Particle::element("bldGraphic", Occurs::STAR),
  ], Occurs::PLUS)
}

/// Content model particle for `ExtensionListWithModification`.
pub fn particle_extension_list_with_modification() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("ext", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ByColor`.
pub fn particle_by_color() -> Particle {
    Particle::choice(vec![
      Particle::element("rgb", Occurs::STAR),
      Particle::element("hsl", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `FromColor`.
pub fn particle_from_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("schemeClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ToColor`.
pub fn particle_to_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("schemeClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommentAuthor`.
pub fn particle_comment_author() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Comment`.
pub fn particle_comment() -> Particle {
    Particle::sequence(vec![
      Particle::element("pos", Occurs::STAR),
      Particle::element("text", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ExtensionList`.
pub fn particle_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("ext", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Control`.
pub fn particle_control() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
      Particle::element("pic", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideId`.
pub fn particle_slide_id() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideMasterId`.
pub fn particle_slide_master_id() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NotesMasterId`.
pub fn particle_notes_master_id() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HandoutMasterId`.
pub fn particle_handout_master_id() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `EmbeddedFont`.
pub fn particle_embedded_font() -> Particle {
    Particle::sequence(vec![
      Particle::element("font", Occurs::STAR),
      Particle::element("regular", Occurs::OPTIONAL),
      Particle::element("bold", Occurs::OPTIONAL),
      Particle::element("italic", Occurs::OPTIONAL),
      Particle::element("boldItalic", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideList`.
pub fn particle_slide_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("sld", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CustomShow`.
pub fn particle_custom_show() -> Particle {
    Particle::sequence(vec![
      Particle::element("sldLst", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualDrawingProperties`.
pub fn particle_non_visual_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("hlinkClick", Occurs::OPTIONAL),
      Particle::element("hlinkHover", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualShapeDrawingProperties`.
pub fn particle_non_visual_shape_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("spLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ApplicationNonVisualDrawingProperties`.
pub fn particle_application_non_visual_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("ph", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("audioCd", Occurs::STAR),
              Particle::element("wavAudioFile", Occurs::STAR),
              Particle::element("audioFile", Occurs::STAR),
              Particle::element("videoFile", Occurs::STAR),
              Particle::element("quickTimeFile", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("custDataLst", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualShapeProperties`.
pub fn particle_non_visual_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvSpPr", Occurs::STAR),
      Particle::element("nvPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeProperties`.
pub fn particle_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("xfrm", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("custGeom", Occurs::STAR),
              Particle::element("prstGeom", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("ln", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("scene3d", Occurs::OPTIONAL),
      Particle::element("sp3d", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeStyle`.
pub fn particle_shape_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnRef", Occurs::STAR),
      Particle::element("fillRef", Occurs::STAR),
      Particle::element("effectRef", Occurs::STAR),
      Particle::element("fontRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TextBody`.
pub fn particle_text_body() -> Particle {
    Particle::sequence(vec![
      Particle::element("bodyPr", Occurs::STAR),
      Particle::element("lstStyle", Occurs::OPTIONAL),
      Particle::element("p", Occurs::PLUS),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualConnectorShapeDrawingProperties`.
pub fn particle_non_visual_connector_shape_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cxnSpLocks", Occurs::OPTIONAL),
      Particle::element("stCxn", Occurs::OPTIONAL),
      Particle::element("endCxn", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualConnectionShapeProperties`.
pub fn particle_non_visual_connection_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvCxnSpPr", Occurs::STAR),
      Particle::element("nvPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualPictureDrawingProperties`.
pub fn particle_non_visual_picture_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("picLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualPictureProperties`.
pub fn particle_non_visual_picture_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvPicPr", Occurs::STAR),
      Particle::element("nvPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `BlipFill`.
pub fn particle_blip_fill() -> Particle {
    Particle::sequence(vec![
      Particle::element("blip", Occurs::OPTIONAL),
      Particle::element("srcRect", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tile", Occurs::STAR),
              Particle::element("stretch", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGraphicFrameDrawingProperties`.
pub fn particle_non_visual_graphic_frame_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("graphicFrameLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGraphicFrameProperties`.
pub fn particle_non_visual_graphic_frame_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvGraphicFramePr", Occurs::STAR),
      Particle::element("nvPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Transform`.
pub fn particle_transform() -> Particle {
    Particle::sequence(vec![
      Particle::element("off", Occurs::OPTIONAL),
      Particle::element("ext", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGroupShapeDrawingProperties`.
pub fn particle_non_visual_group_shape_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("grpSpLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TitleStyle`.
pub fn particle_title_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("defPPr", Occurs::OPTIONAL),
      Particle::element("lvl1pPr", Occurs::OPTIONAL),
      Particle::element("lvl2pPr", Occurs::OPTIONAL),
      Particle::element("lvl3pPr", Occurs::OPTIONAL),
      Particle::element("lvl4pPr", Occurs::OPTIONAL),
      Particle::element("lvl5pPr", Occurs::OPTIONAL),
      Particle::element("lvl6pPr", Occurs::OPTIONAL),
      Particle::element("lvl7pPr", Occurs::OPTIONAL),
      Particle::element("lvl8pPr", Occurs::OPTIONAL),
      Particle::element("lvl9pPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BodyStyle`.
pub fn particle_body_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("defPPr", Occurs::OPTIONAL),
      Particle::element("lvl1pPr", Occurs::OPTIONAL),
      Particle::element("lvl2pPr", Occurs::OPTIONAL),
      Particle::element("lvl3pPr", Occurs::OPTIONAL),
      Particle::element("lvl4pPr", Occurs::OPTIONAL),
      Particle::element("lvl5pPr", Occurs::OPTIONAL),
      Particle::element("lvl6pPr", Occurs::OPTIONAL),
      Particle::element("lvl7pPr", Occurs::OPTIONAL),
      Particle::element("lvl8pPr", Occurs::OPTIONAL),
      Particle::element("lvl9pPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `OtherStyle`.
pub fn particle_other_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("defPPr", Occurs::OPTIONAL),
      Particle::element("lvl1pPr", Occurs::OPTIONAL),
      Particle::element("lvl2pPr", Occurs::OPTIONAL),
      Particle::element("lvl3pPr", Occurs::OPTIONAL),
      Particle::element("lvl4pPr", Occurs::OPTIONAL),
      Particle::element("lvl5pPr", Occurs::OPTIONAL),
      Particle::element("lvl6pPr", Occurs::OPTIONAL),
      Particle::element("lvl7pPr", Occurs::OPTIONAL),
      Particle::element("lvl8pPr", Occurs::OPTIONAL),
      Particle::element("lvl9pPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `DefaultTextStyle`.
pub fn particle_default_text_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("defPPr", Occurs::OPTIONAL),
      Particle::element("lvl1pPr", Occurs::OPTIONAL),
      Particle::element("lvl2pPr", Occurs::OPTIONAL),
      Particle::element("lvl3pPr", Occurs::OPTIONAL),
      Particle::element("lvl4pPr", Occurs::OPTIONAL),
      Particle::element("lvl5pPr", Occurs::OPTIONAL),
      Particle::element("lvl6pPr", Occurs::OPTIONAL),
      Particle::element("lvl7pPr", Occurs::OPTIONAL),
      Particle::element("lvl8pPr", Occurs::OPTIONAL),
      Particle::element("lvl9pPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NotesStyle`.
pub fn particle_notes_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("defPPr", Occurs::OPTIONAL),
      Particle::element("lvl1pPr", Occurs::OPTIONAL),
      Particle::element("lvl2pPr", Occurs::OPTIONAL),
      Particle::element("lvl3pPr", Occurs::OPTIONAL),
      Particle::element("lvl4pPr", Occurs::OPTIONAL),
      Particle::element("lvl5pPr", Occurs::OPTIONAL),
      Particle::element("lvl6pPr", Occurs::OPTIONAL),
      Particle::element("lvl7pPr", Occurs::OPTIONAL),
      Particle::element("lvl8pPr", Occurs::OPTIONAL),
      Particle::element("lvl9pPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideLayoutId`.
pub fn particle_slide_layout_id() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CommonSlideData`.
pub fn particle_common_slide_data() -> Particle {
    Particle::sequence(vec![
      Particle::element("bg", Occurs::OPTIONAL),
      Particle::element("spTree", Occurs::STAR),
      Particle::element("custDataLst", Occurs::OPTIONAL),
      Particle::element("controls", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ScaleFactor`.
pub fn particle_scale_factor() -> Particle {
    Particle::sequence(vec![
      Particle::element("sx", Occurs::STAR),
      Particle::element("sy", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommonViewProperties`.
pub fn particle_common_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("scale", Occurs::STAR),
      Particle::element("origin", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `OutlineViewSlideList`.
pub fn particle_outline_view_slide_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("sld", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `GuideList`.
pub fn particle_guide_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("guide", Occurs::STAR),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `CommonSlideViewProperties`.
pub fn particle_common_slide_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cViewPr", Occurs::STAR),
      Particle::element("guideLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NormalViewProperties`.
pub fn particle_normal_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("restoredLeft", Occurs::STAR),
      Particle::element("restoredTop", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideViewProperties`.
pub fn particle_slide_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSldViewPr", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `OutlineViewProperties`.
pub fn particle_outline_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cViewPr", Occurs::STAR),
      Particle::element("sldLst", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NotesTextViewProperties`.
pub fn particle_notes_text_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cViewPr", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SorterViewProperties`.
pub fn particle_sorter_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cViewPr", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NotesViewProperties`.
pub fn particle_notes_view_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cSldViewPr", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideExtension`.
pub fn particle_slide_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("laserTraceLst", Occurs::STAR),
      Particle::element("showEvtLst", Occurs::STAR),
      Particle::element("commentRel", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CommonSlideDataExtension`.
pub fn particle_common_slide_data_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("creationId", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ShowPropertiesExtension`.
pub fn particle_show_properties_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("browseMode", Occurs::STAR),
      Particle::element("laserClr", Occurs::STAR),
      Particle::element("showMediaCtrls", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Picture`.
pub fn particle_picture() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvPicPr", Occurs::STAR),
      Particle::element("blipFill", Occurs::STAR),
      Particle::element("spPr", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `OleObjectEmbed`.
pub fn particle_ole_object_embed() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `OleObjectLink`.
pub fn particle_ole_object_link() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Transition`.
pub fn particle_transition() -> Particle {
    Particle::sequence(vec![
      Particle::choice(vec![
          Particle::element("blinds", Occurs::STAR),
          Particle::element("checker", Occurs::STAR),
          Particle::element("circle", Occurs::STAR),
          Particle::element("dissolve", Occurs::STAR),
          Particle::element("comb", Occurs::STAR),
          Particle::element("cover", Occurs::STAR),
          Particle::element("cut", Occurs::STAR),
          Particle::element("diamond", Occurs::STAR),
          Particle::element("fade", Occurs::STAR),
          Particle::element("newsflash", Occurs::STAR),
          Particle::element("plus", Occurs::STAR),
          Particle::element("pull", Occurs::STAR),
          Particle::element("push", Occurs::STAR),
          Particle::element("random", Occurs::STAR),
          Particle::element("randomBar", Occurs::STAR),
          Particle::element("split", Occurs::STAR),
          Particle::element("strips", Occurs::STAR),
          Particle::element("wedge", Occurs::STAR),
          Particle::element("wheel", Occurs::STAR),
          Particle::element("wipe", Occurs::STAR),
          Particle::element("zoom", Occurs::STAR),
          Particle::element("flash", Occurs::STAR),
          Particle::element("vortex", Occurs::STAR),
          Particle::element("switch", Occurs::STAR),
          Particle::element("flip", Occurs::STAR),
          Particle::element("ripple", Occurs::STAR),
          Particle::element("glitter", Occurs::STAR),
          Particle::element("honeycomb", Occurs::STAR),
          Particle::element("prism", Occurs::STAR),
          Particle::element("doors", Occurs::STAR),
          Particle::element("window", Occurs::STAR),
          Particle::element("shred", Occurs::STAR),
          Particle::element("ferris", Occurs::STAR),
          Particle::element("flythrough", Occurs::STAR),
          Particle::element("warp", Occurs::STAR),
          Particle::element("gallery", Occurs::STAR),
          Particle::element("conveyor", Occurs::STAR),
          Particle::element("pan", Occurs::STAR),
          Particle::element("reveal", Occurs::STAR),
          Particle::element("wheelReverse", Occurs::STAR),
          Particle::element("prstTrans", Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("sndAc", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Timing`.
pub fn particle_timing() -> Particle {
    Particle::sequence(vec![
      Particle::element("tnLst", Occurs::OPTIONAL),
      Particle::element("bldLst", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideExtensionList`.
pub fn particle_slide_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Background`.
pub fn particle_background() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("bgPr", Occurs::STAR),
              Particle::element("bgRef", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeTree`.
pub fn particle_shape_tree() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvGrpSpPr", Occurs::STAR),
      Particle::element("grpSpPr", Occurs::STAR),
      Particle::choice(vec![
          Particle::element("sp", Occurs::STAR),
          Particle::element("grpSp", Occurs::STAR),
          Particle::element("graphicFrame", Occurs::STAR),
          Particle::element("cxnSp", Occurs::STAR),
          Particle::element("pic", Occurs::STAR),
          Particle::element("contentPart", Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GroupShape`.
pub fn particle_group_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvGrpSpPr", Occurs::STAR),
      Particle::element("grpSpPr", Occurs::STAR),
      Particle::choice(vec![
          Particle::element("sp", Occurs::STAR),
          Particle::element("grpSp", Occurs::STAR),
          Particle::element("graphicFrame", Occurs::STAR),
          Particle::element("cxnSp", Occurs::STAR),
          Particle::element("pic", Occurs::STAR),
          Particle::element("contentPart", Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CustomerDataList`.
pub fn particle_customer_data_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("custData", Occurs::STAR),
      Particle::element("tags", Occurs::OPTIONAL),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `ControlList`.
pub fn particle_control_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("control", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommonSlideDataExtensionList`.
pub fn particle_common_slide_data_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGroupShapeProperties`.
pub fn particle_non_visual_group_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvGrpSpPr", Occurs::STAR),
      Particle::element("nvPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `GroupShapeProperties`.
pub fn particle_group_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("xfrm", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("scene3d", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Shape`.
pub fn particle_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvSpPr", Occurs::STAR),
      Particle::element("spPr", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("txBody", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GraphicFrame`.
pub fn particle_graphic_frame() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvGraphicFramePr", Occurs::STAR),
      Particle::element("xfrm", Occurs::STAR),
      Particle::element("graphic", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ConnectionShape`.
pub fn particle_connection_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvCxnSpPr", Occurs::STAR),
      Particle::element("spPr", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ShowPropertiesExtensionList`.
pub fn particle_show_properties_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeTarget`.
pub fn particle_shape_target() -> Particle {
    Particle::choice(vec![
      Particle::element("bg", Occurs::STAR),
      Particle::element("subSp", Occurs::STAR),
      Particle::element("oleChartEl", Occurs::STAR),
      Particle::element("txEl", Occurs::STAR),
      Particle::element("graphicEl", Occurs::STAR),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `CommentAuthorExtension`.
pub fn particle_comment_author_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("presenceInfo", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CommentExtension`.
pub fn particle_comment_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("threadingInfo", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideLayoutExtension`.
pub fn particle_slide_layout_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("sldGuideLst", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideMasterExtension`.
pub fn particle_slide_master_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("sldGuideLst", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HandoutMasterExtension`.
pub fn particle_handout_master_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("sldGuideLst", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NotesMasterExtension`.
pub fn particle_notes_master_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("sldGuideLst", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PlaceholderShape`.
pub fn particle_placeholder_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ApplicationNonVisualDrawingPropertiesExtensionList`.
pub fn particle_application_non_visual_drawing_properties_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ApplicationNonVisualDrawingPropertiesExtension`.
pub fn particle_application_non_visual_drawing_properties_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("media", Occurs::STAR),
      Particle::element("modId", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Iterate`.
pub fn particle_iterate() -> Particle {
    Particle::choice(vec![
      Particle::element("tmAbs", Occurs::STAR),
      Particle::element("tmPct", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ChildTimeNodeList`.
pub fn particle_child_time_node_list() -> Particle {
    Particle::choice(vec![
      Particle::element("par", Occurs::STAR),
      Particle::element("seq", Occurs::STAR),
      Particle::element("excl", Occurs::STAR),
      Particle::element("anim", Occurs::STAR),
      Particle::element("animClr", Occurs::STAR),
      Particle::element("animEffect", Occurs::STAR),
      Particle::element("animMotion", Occurs::STAR),
      Particle::element("animRot", Occurs::STAR),
      Particle::element("animScale", Occurs::STAR),
      Particle::element("cmd", Occurs::STAR),
      Particle::element("set", Occurs::STAR),
      Particle::element("audio", Occurs::STAR),
      Particle::element("video", Occurs::STAR),
  ], Occurs::PLUS)
}

/// Content model particle for `SubTimeNodeList`.
pub fn particle_sub_time_node_list() -> Particle {
    Particle::choice(vec![
      Particle::element("par", Occurs::STAR),
      Particle::element("seq", Occurs::STAR),
      Particle::element("excl", Occurs::STAR),
      Particle::element("anim", Occurs::STAR),
      Particle::element("animClr", Occurs::STAR),
      Particle::element("animEffect", Occurs::STAR),
      Particle::element("animMotion", Occurs::STAR),
      Particle::element("animRot", Occurs::STAR),
      Particle::element("animScale", Occurs::STAR),
      Particle::element("cmd", Occurs::STAR),
      Particle::element("set", Occurs::STAR),
      Particle::element("audio", Occurs::STAR),
      Particle::element("video", Occurs::STAR),
  ], Occurs::PLUS)
}

/// Content model particle for `TimeAnimateValueList`.
pub fn particle_time_animate_value_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("tav", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommentAuthorExtensionList`.
pub fn particle_comment_author_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CommentExtensionList`.
pub fn particle_comment_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SlideMasterIdList`.
pub fn particle_slide_master_id_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("sldMasterId", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NotesMasterIdList`.
pub fn particle_notes_master_id_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("notesMasterId", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HandoutMasterIdList`.
pub fn particle_handout_master_id_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("handoutMasterId", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideIdList`.
pub fn particle_slide_id_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("sldId", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `EmbeddedFontList`.
pub fn particle_embedded_font_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("embeddedFont", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CustomShowList`.
pub fn particle_custom_show_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("custShow", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PhotoAlbum`.
pub fn particle_photo_album() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PresentationExtensionList`.
pub fn particle_presentation_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PresentationExtension`.
pub fn particle_presentation_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("sectionPr", Occurs::STAR),
      Particle::element("sectionLst", Occurs::STAR),
      Particle::element("sldGuideLst", Occurs::STAR),
      Particle::element("notesGuideLst", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HtmlPublishProperties`.
pub fn particle_html_publish_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("sldAll", Occurs::STAR),
              Particle::element("sldRg", Occurs::STAR),
              Particle::element("custShow", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `WebProperties`.
pub fn particle_web_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PrintingProperties`.
pub fn particle_printing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ShowProperties`.
pub fn particle_show_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("present", Occurs::STAR),
              Particle::element("browse", Occurs::STAR),
              Particle::element("kiosk", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("sldAll", Occurs::STAR),
              Particle::element("sldRg", Occurs::STAR),
              Particle::element("custShow", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("penClr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `ColorMostRecentlyUsed`.
pub fn particle_color_most_recently_used() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::new(0, Some(10))),
  ], Occurs::STAR)
}

/// Content model particle for `PresentationPropertiesExtensionList`.
pub fn particle_presentation_properties_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PresentationPropertiesExtension`.
pub fn particle_presentation_properties_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("discardImageEditData", Occurs::STAR),
      Particle::element("defaultImageDpi", Occurs::STAR),
      Particle::element("m", Occurs::STAR),
      Particle::element("chartTrackingRefBased", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HeaderFooter`.
pub fn particle_header_footer() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideLayoutExtensionList`.
pub fn particle_slide_layout_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SlideLayoutIdList`.
pub fn particle_slide_layout_id_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("sldLayoutId", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TextStyles`.
pub fn particle_text_styles() -> Particle {
    Particle::sequence(vec![
      Particle::element("titleStyle", Occurs::OPTIONAL),
      Particle::element("bodyStyle", Occurs::OPTIONAL),
      Particle::element("otherStyle", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SlideMasterExtensionList`.
pub fn particle_slide_master_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `HandoutMasterExtensionList`.
pub fn particle_handout_master_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NotesMasterExtensionList`.
pub fn particle_notes_master_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TextElement`.
pub fn particle_text_element() -> Particle {
    Particle::choice(vec![
      Particle::element("charRg", Occurs::STAR),
      Particle::element("pRg", Occurs::STAR),
  ], Occurs::OPTIONAL)
}

/// Content model particle for `GraphicElement`.
pub fn particle_graphic_element() -> Particle {
    Particle::choice(vec![
      Particle::element("dgm", Occurs::STAR),
      Particle::element("chart", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SoundAction`.
pub fn particle_sound_action() -> Particle {
    Particle::choice(vec![
      Particle::element("stSnd", Occurs::STAR),
      Particle::element("endSnd", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PlaceholderExtension`.
pub fn particle_placeholder_extension() -> Particle {
    Particle::sequence(vec![
      Particle::element("phTypeExt", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Look up a content-model particle by schema class name.
pub fn particle_for_class(class_name: &str) -> Option<Particle> {
    match class_name {
        "Extension" => Some(particle_extension()),
        "ColorMap" => Some(particle_color_map()),
        "ColorMapOverride" => Some(particle_color_map_override()),
        "BackgroundProperties" => Some(particle_background_properties()),
        "BackgroundStyleReference" => Some(particle_background_style_reference()),
        "CommentPropertiesExtension" => Some(particle_comment_properties_extension()),
        "CommentAuthorList" => Some(particle_comment_author_list()),
        "CommentList" => Some(particle_comment_list()),
        "OleObject" => Some(particle_ole_object()),
        "Presentation" => Some(particle_presentation()),
        "PresentationProperties" => Some(particle_presentation_properties()),
        "Slide" => Some(particle_slide()),
        "SlideLayout" => Some(particle_slide_layout()),
        "SlideMaster" => Some(particle_slide_master()),
        "HandoutMaster" => Some(particle_handout_master()),
        "NotesMaster" => Some(particle_notes_master()),
        "NotesSlide" => Some(particle_notes_slide()),
        "SlideSyncProperties" => Some(particle_slide_sync_properties()),
        "TagList" => Some(particle_tag_list()),
        "ViewProperties" => Some(particle_view_properties()),
        "ContentPart" => Some(particle_content_part()),
        "StartSoundAction" => Some(particle_start_sound_action()),
        "TargetElement" => Some(particle_target_element()),
        "Condition" => Some(particle_condition()),
        "EndSync" => Some(particle_end_sync()),
        "ParallelTimeNode" => Some(particle_parallel_time_node()),
        "SequenceTimeNode" => Some(particle_sequence_time_node()),
        "ExclusiveTimeNode" => Some(particle_exclusive_time_node()),
        "Animate" => Some(particle_animate()),
        "AnimateColor" => Some(particle_animate_color()),
        "AnimateEffect" => Some(particle_animate_effect()),
        "AnimateMotion" => Some(particle_animate_motion()),
        "AnimateRotation" => Some(particle_animate_rotation()),
        "AnimateScale" => Some(particle_animate_scale()),
        "Command" => Some(particle_command()),
        "SetBehavior" => Some(particle_set_behavior()),
        "Audio" => Some(particle_audio()),
        "Video" => Some(particle_video()),
        "CommonTimeNode" => Some(particle_common_time_node()),
        "PreviousConditionList" => Some(particle_previous_condition_list()),
        "NextConditionList" => Some(particle_next_condition_list()),
        "StartConditionList" => Some(particle_start_condition_list()),
        "EndConditionList" => Some(particle_end_condition_list()),
        "AttributeNameList" => Some(particle_attribute_name_list()),
        "ColorValue" => Some(particle_color_value()),
        "PenColor" => Some(particle_pen_color()),
        "TimeAnimateValue" => Some(particle_time_animate_value()),
        "CommonBehavior" => Some(particle_common_behavior()),
        "Progress" => Some(particle_progress()),
        "ToVariantValue" => Some(particle_to_variant_value()),
        "VariantValue" => Some(particle_variant_value()),
        "CommonMediaNode" => Some(particle_common_media_node()),
        "TimeNodeList" => Some(particle_time_node_list()),
        "Template" => Some(particle_template()),
        "TemplateList" => Some(particle_template_list()),
        "BuildSubElement" => Some(particle_build_sub_element()),
        "BuildParagraph" => Some(particle_build_paragraph()),
        "BuildGraphics" => Some(particle_build_graphics()),
        "BuildList" => Some(particle_build_list()),
        "ExtensionListWithModification" => Some(particle_extension_list_with_modification()),
        "ByColor" => Some(particle_by_color()),
        "FromColor" => Some(particle_from_color()),
        "ToColor" => Some(particle_to_color()),
        "CommentAuthor" => Some(particle_comment_author()),
        "Comment" => Some(particle_comment()),
        "ExtensionList" => Some(particle_extension_list()),
        "Control" => Some(particle_control()),
        "SlideId" => Some(particle_slide_id()),
        "SlideMasterId" => Some(particle_slide_master_id()),
        "NotesMasterId" => Some(particle_notes_master_id()),
        "HandoutMasterId" => Some(particle_handout_master_id()),
        "EmbeddedFont" => Some(particle_embedded_font()),
        "SlideList" => Some(particle_slide_list()),
        "CustomShow" => Some(particle_custom_show()),
        "NonVisualDrawingProperties" => Some(particle_non_visual_drawing_properties()),
        "NonVisualShapeDrawingProperties" => Some(particle_non_visual_shape_drawing_properties()),
        "ApplicationNonVisualDrawingProperties" => Some(particle_application_non_visual_drawing_properties()),
        "NonVisualShapeProperties" => Some(particle_non_visual_shape_properties()),
        "ShapeProperties" => Some(particle_shape_properties()),
        "ShapeStyle" => Some(particle_shape_style()),
        "TextBody" => Some(particle_text_body()),
        "NonVisualConnectorShapeDrawingProperties" => Some(particle_non_visual_connector_shape_drawing_properties()),
        "NonVisualConnectionShapeProperties" => Some(particle_non_visual_connection_shape_properties()),
        "NonVisualPictureDrawingProperties" => Some(particle_non_visual_picture_drawing_properties()),
        "NonVisualPictureProperties" => Some(particle_non_visual_picture_properties()),
        "BlipFill" => Some(particle_blip_fill()),
        "NonVisualGraphicFrameDrawingProperties" => Some(particle_non_visual_graphic_frame_drawing_properties()),
        "NonVisualGraphicFrameProperties" => Some(particle_non_visual_graphic_frame_properties()),
        "Transform" => Some(particle_transform()),
        "NonVisualGroupShapeDrawingProperties" => Some(particle_non_visual_group_shape_drawing_properties()),
        "TitleStyle" => Some(particle_title_style()),
        "BodyStyle" => Some(particle_body_style()),
        "OtherStyle" => Some(particle_other_style()),
        "DefaultTextStyle" => Some(particle_default_text_style()),
        "NotesStyle" => Some(particle_notes_style()),
        "SlideLayoutId" => Some(particle_slide_layout_id()),
        "CommonSlideData" => Some(particle_common_slide_data()),
        "ScaleFactor" => Some(particle_scale_factor()),
        "CommonViewProperties" => Some(particle_common_view_properties()),
        "OutlineViewSlideList" => Some(particle_outline_view_slide_list()),
        "GuideList" => Some(particle_guide_list()),
        "CommonSlideViewProperties" => Some(particle_common_slide_view_properties()),
        "NormalViewProperties" => Some(particle_normal_view_properties()),
        "SlideViewProperties" => Some(particle_slide_view_properties()),
        "OutlineViewProperties" => Some(particle_outline_view_properties()),
        "NotesTextViewProperties" => Some(particle_notes_text_view_properties()),
        "SorterViewProperties" => Some(particle_sorter_view_properties()),
        "NotesViewProperties" => Some(particle_notes_view_properties()),
        "SlideExtension" => Some(particle_slide_extension()),
        "CommonSlideDataExtension" => Some(particle_common_slide_data_extension()),
        "ShowPropertiesExtension" => Some(particle_show_properties_extension()),
        "Picture" => Some(particle_picture()),
        "OleObjectEmbed" => Some(particle_ole_object_embed()),
        "OleObjectLink" => Some(particle_ole_object_link()),
        "Transition" => Some(particle_transition()),
        "Timing" => Some(particle_timing()),
        "SlideExtensionList" => Some(particle_slide_extension_list()),
        "Background" => Some(particle_background()),
        "ShapeTree" => Some(particle_shape_tree()),
        "GroupShape" => Some(particle_group_shape()),
        "CustomerDataList" => Some(particle_customer_data_list()),
        "ControlList" => Some(particle_control_list()),
        "CommonSlideDataExtensionList" => Some(particle_common_slide_data_extension_list()),
        "NonVisualGroupShapeProperties" => Some(particle_non_visual_group_shape_properties()),
        "GroupShapeProperties" => Some(particle_group_shape_properties()),
        "Shape" => Some(particle_shape()),
        "GraphicFrame" => Some(particle_graphic_frame()),
        "ConnectionShape" => Some(particle_connection_shape()),
        "ShowPropertiesExtensionList" => Some(particle_show_properties_extension_list()),
        "ShapeTarget" => Some(particle_shape_target()),
        "CommentAuthorExtension" => Some(particle_comment_author_extension()),
        "CommentExtension" => Some(particle_comment_extension()),
        "SlideLayoutExtension" => Some(particle_slide_layout_extension()),
        "SlideMasterExtension" => Some(particle_slide_master_extension()),
        "HandoutMasterExtension" => Some(particle_handout_master_extension()),
        "NotesMasterExtension" => Some(particle_notes_master_extension()),
        "PlaceholderShape" => Some(particle_placeholder_shape()),
        "ApplicationNonVisualDrawingPropertiesExtensionList" => Some(particle_application_non_visual_drawing_properties_extension_list()),
        "ApplicationNonVisualDrawingPropertiesExtension" => Some(particle_application_non_visual_drawing_properties_extension()),
        "Iterate" => Some(particle_iterate()),
        "ChildTimeNodeList" => Some(particle_child_time_node_list()),
        "SubTimeNodeList" => Some(particle_sub_time_node_list()),
        "TimeAnimateValueList" => Some(particle_time_animate_value_list()),
        "CommentAuthorExtensionList" => Some(particle_comment_author_extension_list()),
        "CommentExtensionList" => Some(particle_comment_extension_list()),
        "SlideMasterIdList" => Some(particle_slide_master_id_list()),
        "NotesMasterIdList" => Some(particle_notes_master_id_list()),
        "HandoutMasterIdList" => Some(particle_handout_master_id_list()),
        "SlideIdList" => Some(particle_slide_id_list()),
        "EmbeddedFontList" => Some(particle_embedded_font_list()),
        "CustomShowList" => Some(particle_custom_show_list()),
        "PhotoAlbum" => Some(particle_photo_album()),
        "PresentationExtensionList" => Some(particle_presentation_extension_list()),
        "PresentationExtension" => Some(particle_presentation_extension()),
        "HtmlPublishProperties" => Some(particle_html_publish_properties()),
        "WebProperties" => Some(particle_web_properties()),
        "PrintingProperties" => Some(particle_printing_properties()),
        "ShowProperties" => Some(particle_show_properties()),
        "ColorMostRecentlyUsed" => Some(particle_color_most_recently_used()),
        "PresentationPropertiesExtensionList" => Some(particle_presentation_properties_extension_list()),
        "PresentationPropertiesExtension" => Some(particle_presentation_properties_extension()),
        "HeaderFooter" => Some(particle_header_footer()),
        "SlideLayoutExtensionList" => Some(particle_slide_layout_extension_list()),
        "SlideLayoutIdList" => Some(particle_slide_layout_id_list()),
        "TextStyles" => Some(particle_text_styles()),
        "SlideMasterExtensionList" => Some(particle_slide_master_extension_list()),
        "HandoutMasterExtensionList" => Some(particle_handout_master_extension_list()),
        "NotesMasterExtensionList" => Some(particle_notes_master_extension_list()),
        "TextElement" => Some(particle_text_element()),
        "GraphicElement" => Some(particle_graphic_element()),
        "SoundAction" => Some(particle_sound_action()),
        "PlaceholderExtension" => Some(particle_placeholder_extension()),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Schema enums
// ---------------------------------------------------------------------------

/// Transition Slide Direction Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionSlideDirectionValues {
    Left,
    Up,
    Right,
    Down,
}

impl TransitionSlideDirectionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "l",
            Self::Up => "u",
            Self::Right => "r",
            Self::Down => "d",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "l" => Some(Self::Left),
            "u" => Some(Self::Up),
            "r" => Some(Self::Right),
            "d" => Some(Self::Down),
            _ => None,
        }
    }
}

impl core::fmt::Display for TransitionSlideDirectionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TransitionSlideDirectionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Transition Corner Direction Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionCornerDirectionValues {
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl TransitionCornerDirectionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LeftUp => "lu",
            Self::RightUp => "ru",
            Self::LeftDown => "ld",
            Self::RightDown => "rd",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "lu" => Some(Self::LeftUp),
            "ru" => Some(Self::RightUp),
            "ld" => Some(Self::LeftDown),
            "rd" => Some(Self::RightDown),
            _ => None,
        }
    }
}

impl core::fmt::Display for TransitionCornerDirectionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TransitionCornerDirectionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Transition In/Out Direction Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionInOutDirectionValues {
    Out,
    In_,
}

impl TransitionInOutDirectionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Out => "out",
            Self::In_ => "in",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "out" => Some(Self::Out),
            "in" => Some(Self::In_),
            _ => None,
        }
    }
}

impl core::fmt::Display for TransitionInOutDirectionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TransitionInOutDirectionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Transition Speed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionSpeedValues {
    Slow,
    Medium,
    Fast,
}

impl TransitionSpeedValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Slow => "slow",
            Self::Medium => "med",
            Self::Fast => "fast",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "slow" => Some(Self::Slow),
            "med" => Some(Self::Medium),
            "fast" => Some(Self::Fast),
            _ => None,
        }
    }
}

impl core::fmt::Display for TransitionSpeedValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TransitionSpeedValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Indefinite Time Declaration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndefiniteTimeDeclarationValues {
    Indefinite,
}

impl IndefiniteTimeDeclarationValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Indefinite => "indefinite",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "indefinite" => Some(Self::Indefinite),
            _ => None,
        }
    }
}

impl core::fmt::Display for IndefiniteTimeDeclarationValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for IndefiniteTimeDeclarationValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Iterate Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IterateValues {
    Element,
    Word,
    Letter,
}

impl IterateValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Element => "el",
            Self::Word => "wd",
            Self::Letter => "lt",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "el" => Some(Self::Element),
            "wd" => Some(Self::Word),
            "lt" => Some(Self::Letter),
            _ => None,
        }
    }
}

impl core::fmt::Display for IterateValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for IterateValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Chart Subelement Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChartSubElementValues {
    GridLegend,
    Series,
    Category,
    PointInSeries,
    PointInCategory,
}

impl ChartSubElementValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GridLegend => "gridLegend",
            Self::Series => "series",
            Self::Category => "category",
            Self::PointInSeries => "ptInSeries",
            Self::PointInCategory => "ptInCategory",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "gridLegend" => Some(Self::GridLegend),
            "series" => Some(Self::Series),
            "category" => Some(Self::Category),
            "ptInSeries" => Some(Self::PointInSeries),
            "ptInCategory" => Some(Self::PointInCategory),
            _ => None,
        }
    }
}

impl core::fmt::Display for ChartSubElementValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for ChartSubElementValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Trigger RunTime Node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerRuntimeNodeValues {
    First,
    Last,
    All,
}

impl TriggerRuntimeNodeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::First => "first",
            Self::Last => "last",
            Self::All => "all",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "first" => Some(Self::First),
            "last" => Some(Self::Last),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

impl core::fmt::Display for TriggerRuntimeNodeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TriggerRuntimeNodeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time Node Preset Class Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeNodePresetClassValues {
    Entrance,
    Exit,
    Emphasis,
    Path,
    Verb,
    MediaCall,
}

impl TimeNodePresetClassValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Entrance => "entr",
            Self::Exit => "exit",
            Self::Emphasis => "emph",
            Self::Path => "path",
            Self::Verb => "verb",
            Self::MediaCall => "mediacall",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "entr" => Some(Self::Entrance),
            "exit" => Some(Self::Exit),
            "emph" => Some(Self::Emphasis),
            "path" => Some(Self::Path),
            "verb" => Some(Self::Verb),
            "mediacall" => Some(Self::MediaCall),
            _ => None,
        }
    }
}

impl core::fmt::Display for TimeNodePresetClassValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TimeNodePresetClassValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time Node Restart Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeNodeRestartValues {
    Always,
    WhenNotActive,
    Never,
}

impl TimeNodeRestartValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::WhenNotActive => "whenNotActive",
            Self::Never => "never",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "always" => Some(Self::Always),
            "whenNotActive" => Some(Self::WhenNotActive),
            "never" => Some(Self::Never),
            _ => None,
        }
    }
}

impl core::fmt::Display for TimeNodeRestartValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TimeNodeRestartValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time Node Fill Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeNodeFillValues {
    Remove,
    Freeze,
    Hold,
    Transition,
}

impl TimeNodeFillValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Remove => "remove",
            Self::Freeze => "freeze",
            Self::Hold => "hold",
            Self::Transition => "transition",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "remove" => Some(Self::Remove),
            "freeze" => Some(Self::Freeze),
            "hold" => Some(Self::Hold),
            "transition" => Some(Self::Transition),
            _ => None,
        }
    }
}

impl core::fmt::Display for TimeNodeFillValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TimeNodeFillValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time Node Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeNodeValues {
    ClickEffect,
    WithEffect,
    AfterEffect,
    MainSequence,
    InteractiveSequence,
    ClickParagraph,
    WithGroup,
    AfterGroup,
    TmingRoot,
}

impl TimeNodeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClickEffect => "clickEffect",
            Self::WithEffect => "withEffect",
            Self::AfterEffect => "afterEffect",
            Self::MainSequence => "mainSeq",
            Self::InteractiveSequence => "interactiveSeq",
            Self::ClickParagraph => "clickPar",
            Self::WithGroup => "withGroup",
            Self::AfterGroup => "afterGroup",
            Self::TmingRoot => "tmRoot",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "clickEffect" => Some(Self::ClickEffect),
            "withEffect" => Some(Self::WithEffect),
            "afterEffect" => Some(Self::AfterEffect),
            "mainSeq" => Some(Self::MainSequence),
            "interactiveSeq" => Some(Self::InteractiveSequence),
            "clickPar" => Some(Self::ClickParagraph),
            "withGroup" => Some(Self::WithGroup),
            "afterGroup" => Some(Self::AfterGroup),
            "tmRoot" => Some(Self::TmingRoot),
            _ => None,
        }
    }
}

impl core::fmt::Display for TimeNodeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TimeNodeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Next Action Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NextActionValues {
    None_,
    Seek,
}

impl NextActionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Seek => "seek",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "seek" => Some(Self::Seek),
            _ => None,
        }
    }
}

impl core::fmt::Display for NextActionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for NextActionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Previous Action Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreviousActionValues {
    None_,
    SkipTimed,
}

impl PreviousActionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::SkipTimed => "skipTimed",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "skipTimed" => Some(Self::SkipTimed),
            _ => None,
        }
    }
}

impl core::fmt::Display for PreviousActionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PreviousActionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Behavior Additive Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BehaviorAdditiveValues {
    Base,
    Sum,
    Replace,
    Multiply,
    None_,
}

impl BehaviorAdditiveValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Sum => "sum",
            Self::Replace => "repl",
            Self::Multiply => "mult",
            Self::None_ => "none",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "base" => Some(Self::Base),
            "sum" => Some(Self::Sum),
            "repl" => Some(Self::Replace),
            "mult" => Some(Self::Multiply),
            "none" => Some(Self::None_),
            _ => None,
        }
    }
}

impl core::fmt::Display for BehaviorAdditiveValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BehaviorAdditiveValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Behavior Accumulate Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BehaviorAccumulateValues {
    None_,
    Always,
}

impl BehaviorAccumulateValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Always => "always",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "always" => Some(Self::Always),
            _ => None,
        }
    }
}

impl core::fmt::Display for BehaviorAccumulateValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BehaviorAccumulateValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Behavior Transform Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BehaviorTransformValues {
    Point,
    Image,
}

impl BehaviorTransformValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Point => "pt",
            Self::Image => "img",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pt" => Some(Self::Point),
            "img" => Some(Self::Image),
            _ => None,
        }
    }
}

impl core::fmt::Display for BehaviorTransformValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BehaviorTransformValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Behavior Override Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BehaviorOverrideValues {
    Normal,
    ChildStyle,
}

impl BehaviorOverrideValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::ChildStyle => "childStyle",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(Self::Normal),
            "childStyle" => Some(Self::ChildStyle),
            _ => None,
        }
    }
}

impl core::fmt::Display for BehaviorOverrideValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BehaviorOverrideValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time List Animate Behavior Calculate Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimateBehaviorCalculateModeValues {
    Discrete,
    Linear,
    Formula,
}

impl AnimateBehaviorCalculateModeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discrete => "discrete",
            Self::Linear => "lin",
            Self::Formula => "fmla",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "discrete" => Some(Self::Discrete),
            "lin" => Some(Self::Linear),
            "fmla" => Some(Self::Formula),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimateBehaviorCalculateModeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimateBehaviorCalculateModeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time List Animate Behavior Value Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimateBehaviorValues {
    String,
    Number,
    Color,
}

impl AnimateBehaviorValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::String => "str",
            Self::Number => "num",
            Self::Color => "clr",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "str" => Some(Self::String),
            "num" => Some(Self::Number),
            "clr" => Some(Self::Color),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimateBehaviorValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimateBehaviorValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time List Animate Color Space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimateColorSpaceValues {
    Rgb,
    Hsl,
}

impl AnimateColorSpaceValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rgb => "rgb",
            Self::Hsl => "hsl",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rgb" => Some(Self::Rgb),
            "hsl" => Some(Self::Hsl),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimateColorSpaceValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimateColorSpaceValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time List Animate Color Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimateColorDirectionValues {
    Clockwise,
    CounterClockwise,
}

impl AnimateColorDirectionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clockwise => "cw",
            Self::CounterClockwise => "ccw",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cw" => Some(Self::Clockwise),
            "ccw" => Some(Self::CounterClockwise),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimateColorDirectionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimateColorDirectionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time List Animate Effect Transition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimateEffectTransitionValues {
    In_,
    Out,
    None_,
}

impl AnimateEffectTransitionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::In_ => "in",
            Self::Out => "out",
            Self::None_ => "none",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "in" => Some(Self::In_),
            "out" => Some(Self::Out),
            "none" => Some(Self::None_),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimateEffectTransitionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimateEffectTransitionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time List Animate Motion Behavior Origin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimateMotionBehaviorOriginValues {
    Parent,
    Layout,
}

impl AnimateMotionBehaviorOriginValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parent => "parent",
            Self::Layout => "layout",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "parent" => Some(Self::Parent),
            "layout" => Some(Self::Layout),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimateMotionBehaviorOriginValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimateMotionBehaviorOriginValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time List Animate Motion Path Edit Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimateMotionPathEditModeValues {
    Relative,
    Fixed,
}

impl AnimateMotionPathEditModeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Relative => "relative",
            Self::Fixed => "fixed",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "relative" => Some(Self::Relative),
            "fixed" => Some(Self::Fixed),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimateMotionPathEditModeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimateMotionPathEditModeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Command Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandValues {
    Event,
    Call,
    Verb,
}

impl CommandValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Event => "evt",
            Self::Call => "call",
            Self::Verb => "verb",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "evt" => Some(Self::Event),
            "call" => Some(Self::Call),
            "verb" => Some(Self::Verb),
            _ => None,
        }
    }
}

impl core::fmt::Display for CommandValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for CommandValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Paragraph Build Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParagraphBuildValues {
    AllAtOnce,
    Paragraph,
    Custom,
    Whole,
}

impl ParagraphBuildValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllAtOnce => "allAtOnce",
            Self::Paragraph => "p",
            Self::Custom => "cust",
            Self::Whole => "whole",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allAtOnce" => Some(Self::AllAtOnce),
            "p" => Some(Self::Paragraph),
            "cust" => Some(Self::Custom),
            "whole" => Some(Self::Whole),
            _ => None,
        }
    }
}

impl core::fmt::Display for ParagraphBuildValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for ParagraphBuildValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Diagram Build Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagramBuildValues {
    Whole,
    DepthByNode,
    DepthByBranch,
    BreadthByNode,
    BreadthByLevel,
    Clockwise,
    ClockwiseIn,
    ClockwiseOut,
    CounterClockwise,
    CounterClockwiseIn,
    CounterClockwiseOut,
    InByRing,
    OutByRing,
    Up,
    Down,
    AllAtOnce,
    Custom,
}

impl DiagramBuildValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Whole => "whole",
            Self::DepthByNode => "depthByNode",
            Self::DepthByBranch => "depthByBranch",
            Self::BreadthByNode => "breadthByNode",
            Self::BreadthByLevel => "breadthByLvl",
            Self::Clockwise => "cw",
            Self::ClockwiseIn => "cwIn",
            Self::ClockwiseOut => "cwOut",
            Self::CounterClockwise => "ccw",
            Self::CounterClockwiseIn => "ccwIn",
            Self::CounterClockwiseOut => "ccwOut",
            Self::InByRing => "inByRing",
            Self::OutByRing => "outByRing",
            Self::Up => "up",
            Self::Down => "down",
            Self::AllAtOnce => "allAtOnce",
            Self::Custom => "cust",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "whole" => Some(Self::Whole),
            "depthByNode" => Some(Self::DepthByNode),
            "depthByBranch" => Some(Self::DepthByBranch),
            "breadthByNode" => Some(Self::BreadthByNode),
            "breadthByLvl" => Some(Self::BreadthByLevel),
            "cw" => Some(Self::Clockwise),
            "cwIn" => Some(Self::ClockwiseIn),
            "cwOut" => Some(Self::ClockwiseOut),
            "ccw" => Some(Self::CounterClockwise),
            "ccwIn" => Some(Self::CounterClockwiseIn),
            "ccwOut" => Some(Self::CounterClockwiseOut),
            "inByRing" => Some(Self::InByRing),
            "outByRing" => Some(Self::OutByRing),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            "allAtOnce" => Some(Self::AllAtOnce),
            "cust" => Some(Self::Custom),
            _ => None,
        }
    }
}

impl core::fmt::Display for DiagramBuildValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for DiagramBuildValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// OLE Chart Build Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OleChartBuildValues {
    AllAtOnce,
    Series,
    Category,
    SeriesElement,
    CategoryElement,
}

impl OleChartBuildValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllAtOnce => "allAtOnce",
            Self::Series => "series",
            Self::Category => "category",
            Self::SeriesElement => "seriesEl",
            Self::CategoryElement => "categoryEl",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allAtOnce" => Some(Self::AllAtOnce),
            "series" => Some(Self::Series),
            "category" => Some(Self::Category),
            "seriesEl" => Some(Self::SeriesElement),
            "categoryEl" => Some(Self::CategoryElement),
            _ => None,
        }
    }
}

impl core::fmt::Display for OleChartBuildValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for OleChartBuildValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time Node Master Relation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeNodeMasterRelationValues {
    SameClick,
    NextClick,
}

impl TimeNodeMasterRelationValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SameClick => "sameClick",
            Self::NextClick => "nextClick",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sameClick" => Some(Self::SameClick),
            "nextClick" => Some(Self::NextClick),
            _ => None,
        }
    }
}

impl core::fmt::Display for TimeNodeMasterRelationValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TimeNodeMasterRelationValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Time Node Sync Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeNodeSyncValues {
    None_,
    CanSlip,
    Locked,
}

impl TimeNodeSyncValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::CanSlip => "canSlip",
            Self::Locked => "locked",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "canSlip" => Some(Self::CanSlip),
            "locked" => Some(Self::Locked),
            _ => None,
        }
    }
}

impl core::fmt::Display for TimeNodeSyncValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TimeNodeSyncValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionValues {
    Horizontal,
    Vertical,
}

impl DirectionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horz",
            Self::Vertical => "vert",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "horz" => Some(Self::Horizontal),
            "vert" => Some(Self::Vertical),
            _ => None,
        }
    }
}

impl core::fmt::Display for DirectionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for DirectionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// OLE Object to Follow Color Scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OleObjectFollowColorSchemeValues {
    None_,
    Full,
    TextAndBackground,
}

impl OleObjectFollowColorSchemeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Full => "full",
            Self::TextAndBackground => "textAndBackground",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "full" => Some(Self::Full),
            "textAndBackground" => Some(Self::TextAndBackground),
            _ => None,
        }
    }
}

impl core::fmt::Display for OleObjectFollowColorSchemeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for OleObjectFollowColorSchemeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Photo Album Layout Definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhotoAlbumLayoutValues {
    FitToSlide,
    OnePic,
    TwoPic,
    FourPic,
    OnePicWithTitle,
    TwoPicWithTitle,
    FourPicWithTitle,
}

impl PhotoAlbumLayoutValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FitToSlide => "fitToSlide",
            Self::OnePic => "1pic",
            Self::TwoPic => "2pic",
            Self::FourPic => "4pic",
            Self::OnePicWithTitle => "1picTitle",
            Self::TwoPicWithTitle => "2picTitle",
            Self::FourPicWithTitle => "4picTitle",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "fitToSlide" => Some(Self::FitToSlide),
            "1pic" => Some(Self::OnePic),
            "2pic" => Some(Self::TwoPic),
            "4pic" => Some(Self::FourPic),
            "1picTitle" => Some(Self::OnePicWithTitle),
            "2picTitle" => Some(Self::TwoPicWithTitle),
            "4picTitle" => Some(Self::FourPicWithTitle),
            _ => None,
        }
    }
}

impl core::fmt::Display for PhotoAlbumLayoutValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PhotoAlbumLayoutValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Photo Album Shape for Photo Mask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhotoAlbumFrameShapeValues {
    FrameStyle1,
    FrameStyle2,
    FrameStyle3,
    FrameStyle4,
    FrameStyle5,
    FrameStyle6,
    FrameStyle7,
}

impl PhotoAlbumFrameShapeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FrameStyle1 => "frameStyle1",
            Self::FrameStyle2 => "frameStyle2",
            Self::FrameStyle3 => "frameStyle3",
            Self::FrameStyle4 => "frameStyle4",
            Self::FrameStyle5 => "frameStyle5",
            Self::FrameStyle6 => "frameStyle6",
            Self::FrameStyle7 => "frameStyle7",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "frameStyle1" => Some(Self::FrameStyle1),
            "frameStyle2" => Some(Self::FrameStyle2),
            "frameStyle3" => Some(Self::FrameStyle3),
            "frameStyle4" => Some(Self::FrameStyle4),
            "frameStyle5" => Some(Self::FrameStyle5),
            "frameStyle6" => Some(Self::FrameStyle6),
            "frameStyle7" => Some(Self::FrameStyle7),
            _ => None,
        }
    }
}

impl core::fmt::Display for PhotoAlbumFrameShapeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PhotoAlbumFrameShapeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Slide Size Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlideSizeValues {
    Screen4x3,
    Letter,
    A4,
    Film35mm,
    Overhead,
    Banner,
    Custom,
    Ledger,
    A3,
    B4ISO,
    B5ISO,
    B4JIS,
    B5JIS,
    HagakiCard,
    Screen16x9,
    Screen16x10,
}

impl SlideSizeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Screen4x3 => "screen4x3",
            Self::Letter => "letter",
            Self::A4 => "A4",
            Self::Film35mm => "35mm",
            Self::Overhead => "overhead",
            Self::Banner => "banner",
            Self::Custom => "custom",
            Self::Ledger => "ledger",
            Self::A3 => "A3",
            Self::B4ISO => "B4ISO",
            Self::B5ISO => "B5ISO",
            Self::B4JIS => "B4JIS",
            Self::B5JIS => "B5JIS",
            Self::HagakiCard => "hagakiCard",
            Self::Screen16x9 => "screen16x9",
            Self::Screen16x10 => "screen16x10",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "screen4x3" => Some(Self::Screen4x3),
            "letter" => Some(Self::Letter),
            "A4" => Some(Self::A4),
            "35mm" => Some(Self::Film35mm),
            "overhead" => Some(Self::Overhead),
            "banner" => Some(Self::Banner),
            "custom" => Some(Self::Custom),
            "ledger" => Some(Self::Ledger),
            "A3" => Some(Self::A3),
            "B4ISO" => Some(Self::B4ISO),
            "B5ISO" => Some(Self::B5ISO),
            "B4JIS" => Some(Self::B4JIS),
            "B5JIS" => Some(Self::B5JIS),
            "hagakiCard" => Some(Self::HagakiCard),
            "screen16x9" => Some(Self::Screen16x9),
            "screen16x10" => Some(Self::Screen16x10),
            _ => None,
        }
    }
}

impl core::fmt::Display for SlideSizeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for SlideSizeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Cryptographic Provider Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CryptProviderValues {
    RsaAES,
    RsaFull,
    Invalid,
}

impl CryptProviderValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RsaAES => "rsaAES",
            Self::RsaFull => "rsaFull",
            Self::Invalid => "invalid",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rsaAES" => Some(Self::RsaAES),
            "rsaFull" => Some(Self::RsaFull),
            "invalid" => Some(Self::Invalid),
            _ => None,
        }
    }
}

impl core::fmt::Display for CryptProviderValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for CryptProviderValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Cryptographic Algorithm Classes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CryptAlgorithmClassValues {
    Hash,
    Invalid,
}

impl CryptAlgorithmClassValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hash => "hash",
            Self::Invalid => "invalid",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "hash" => Some(Self::Hash),
            "invalid" => Some(Self::Invalid),
            _ => None,
        }
    }
}

impl core::fmt::Display for CryptAlgorithmClassValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for CryptAlgorithmClassValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Cryptographic Algorithm Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CryptAlgorithmValues {
    TypeAny,
    Invalid,
}

impl CryptAlgorithmValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TypeAny => "typeAny",
            Self::Invalid => "invalid",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "typeAny" => Some(Self::TypeAny),
            "invalid" => Some(Self::Invalid),
            _ => None,
        }
    }
}

impl core::fmt::Display for CryptAlgorithmValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for CryptAlgorithmValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Web browsers supported for HTML output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HtmlPublishWebBrowserSupportValues {
    V4,
    V3,
    V3v4,
}

impl HtmlPublishWebBrowserSupportValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V4 => "v4",
            Self::V3 => "v3",
            Self::V3v4 => "v3v4",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "v4" => Some(Self::V4),
            "v3" => Some(Self::V3),
            "v3v4" => Some(Self::V3v4),
            _ => None,
        }
    }
}

impl core::fmt::Display for HtmlPublishWebBrowserSupportValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for HtmlPublishWebBrowserSupportValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// HTML Slide Navigation Control Colors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WebColorValues {
    None_,
    Browser,
    PresentationText,
    PresentationAccent,
    WhiteTextOnBlack,
    BlackTextOnWhite,
}

impl WebColorValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Browser => "browser",
            Self::PresentationText => "presentationText",
            Self::PresentationAccent => "presentationAccent",
            Self::WhiteTextOnBlack => "whiteTextOnBlack",
            Self::BlackTextOnWhite => "blackTextOnWhite",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "browser" => Some(Self::Browser),
            "presentationText" => Some(Self::PresentationText),
            "presentationAccent" => Some(Self::PresentationAccent),
            "whiteTextOnBlack" => Some(Self::WhiteTextOnBlack),
            "blackTextOnWhite" => Some(Self::BlackTextOnWhite),
            _ => None,
        }
    }
}

impl core::fmt::Display for WebColorValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for WebColorValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// HTML/Web Screen Size Target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WebScreenSizeValues {
    Sz544x376,
    Sz640x480,
    Sz720x512,
    Sz800x600,
    Sz1024x768,
    Sz1152x882,
    Sz1152x900,
    Sz1280x1024,
    Sz1600x1200,
    Sz1800x1400,
    Sz1920x1200,
}

impl WebScreenSizeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sz544x376 => "544x376",
            Self::Sz640x480 => "640x480",
            Self::Sz720x512 => "720x512",
            Self::Sz800x600 => "800x600",
            Self::Sz1024x768 => "1024x768",
            Self::Sz1152x882 => "1152x882",
            Self::Sz1152x900 => "1152x900",
            Self::Sz1280x1024 => "1280x1024",
            Self::Sz1600x1200 => "1600x1200",
            Self::Sz1800x1400 => "1800x1400",
            Self::Sz1920x1200 => "1920x1200",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "544x376" => Some(Self::Sz544x376),
            "640x480" => Some(Self::Sz640x480),
            "720x512" => Some(Self::Sz720x512),
            "800x600" => Some(Self::Sz800x600),
            "1024x768" => Some(Self::Sz1024x768),
            "1152x882" => Some(Self::Sz1152x882),
            "1152x900" => Some(Self::Sz1152x900),
            "1280x1024" => Some(Self::Sz1280x1024),
            "1600x1200" => Some(Self::Sz1600x1200),
            "1800x1400" => Some(Self::Sz1800x1400),
            "1920x1200" => Some(Self::Sz1920x1200),
            _ => None,
        }
    }
}

impl core::fmt::Display for WebScreenSizeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for WebScreenSizeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Default print output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrintOutputValues {
    Slides,
    Handouts1,
    Handouts2,
    Handouts3,
    Handouts4,
    Handouts6,
    Handouts9,
    Notes,
    Outline,
}

impl PrintOutputValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Slides => "slides",
            Self::Handouts1 => "handouts1",
            Self::Handouts2 => "handouts2",
            Self::Handouts3 => "handouts3",
            Self::Handouts4 => "handouts4",
            Self::Handouts6 => "handouts6",
            Self::Handouts9 => "handouts9",
            Self::Notes => "notes",
            Self::Outline => "outline",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "slides" => Some(Self::Slides),
            "handouts1" => Some(Self::Handouts1),
            "handouts2" => Some(Self::Handouts2),
            "handouts3" => Some(Self::Handouts3),
            "handouts4" => Some(Self::Handouts4),
            "handouts6" => Some(Self::Handouts6),
            "handouts9" => Some(Self::Handouts9),
            "notes" => Some(Self::Notes),
            "outline" => Some(Self::Outline),
            _ => None,
        }
    }
}

impl core::fmt::Display for PrintOutputValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PrintOutputValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Print Color Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrintColorModeValues {
    BlackWhite,
    Gray,
    Color,
}

impl PrintColorModeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlackWhite => "bw",
            Self::Gray => "gray",
            Self::Color => "clr",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "bw" => Some(Self::BlackWhite),
            "gray" => Some(Self::Gray),
            "clr" => Some(Self::Color),
            _ => None,
        }
    }
}

impl core::fmt::Display for PrintColorModeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PrintColorModeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Placeholder IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceholderValues {
    Title,
    Body,
    CenteredTitle,
    SubTitle,
    DateAndTime,
    SlideNumber,
    Footer,
    Header,
    Object,
    Chart,
    Table,
    ClipArt,
    Diagram,
    Media,
    SlideImage,
    Picture,
}

impl PlaceholderValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Body => "body",
            Self::CenteredTitle => "ctrTitle",
            Self::SubTitle => "subTitle",
            Self::DateAndTime => "dt",
            Self::SlideNumber => "sldNum",
            Self::Footer => "ftr",
            Self::Header => "hdr",
            Self::Object => "obj",
            Self::Chart => "chart",
            Self::Table => "tbl",
            Self::ClipArt => "clipArt",
            Self::Diagram => "dgm",
            Self::Media => "media",
            Self::SlideImage => "sldImg",
            Self::Picture => "pic",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "title" => Some(Self::Title),
            "body" => Some(Self::Body),
            "ctrTitle" => Some(Self::CenteredTitle),
            "subTitle" => Some(Self::SubTitle),
            "dt" => Some(Self::DateAndTime),
            "sldNum" => Some(Self::SlideNumber),
            "ftr" => Some(Self::Footer),
            "hdr" => Some(Self::Header),
            "obj" => Some(Self::Object),
            "chart" => Some(Self::Chart),
            "tbl" => Some(Self::Table),
            "clipArt" => Some(Self::ClipArt),
            "dgm" => Some(Self::Diagram),
            "media" => Some(Self::Media),
            "sldImg" => Some(Self::SlideImage),
            "pic" => Some(Self::Picture),
            _ => None,
        }
    }
}

impl core::fmt::Display for PlaceholderValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PlaceholderValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Placeholder Size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceholderSizeValues {
    Full,
    Half,
    Quarter,
}

impl PlaceholderSizeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Half => "half",
            Self::Quarter => "quarter",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "full" => Some(Self::Full),
            "half" => Some(Self::Half),
            "quarter" => Some(Self::Quarter),
            _ => None,
        }
    }
}

impl core::fmt::Display for PlaceholderSizeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PlaceholderSizeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Slide Layout Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlideLayoutValues {
    Title,
    Text,
    TwoColumnText,
    Table,
    TextAndChart,
    ChartAndText,
    Diagram,
    Chart,
    TextAndClipArt,
    ClipArtAndText,
    TitleOnly,
    Blank,
    TextAndObject,
    ObjectAndText,
    ObjectOnly,
    Object,
    TextAndMedia,
    MidiaAndText,
    ObjectOverText,
    TextOverObject,
    TextAndTwoObjects,
    TwoObjectsAndText,
    TwoObjectsOverText,
    FourObjects,
    VerticalText,
    ClipArtAndVerticalText,
    VerticalTitleAndText,
    VerticalTitleAndTextOverChart,
    TwoObjects,
    ObjectAndTwoObjects,
    TwoObjectsAndObject,
    Custom,
    SectionHeader,
    TwoTextAndTwoObjects,
    ObjectText,
    PictureText,
}

impl SlideLayoutValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Text => "tx",
            Self::TwoColumnText => "twoColTx",
            Self::Table => "tbl",
            Self::TextAndChart => "txAndChart",
            Self::ChartAndText => "chartAndTx",
            Self::Diagram => "dgm",
            Self::Chart => "chart",
            Self::TextAndClipArt => "txAndClipArt",
            Self::ClipArtAndText => "clipArtAndTx",
            Self::TitleOnly => "titleOnly",
            Self::Blank => "blank",
            Self::TextAndObject => "txAndObj",
            Self::ObjectAndText => "objAndTx",
            Self::ObjectOnly => "objOnly",
            Self::Object => "obj",
            Self::TextAndMedia => "txAndMedia",
            Self::MidiaAndText => "mediaAndTx",
            Self::ObjectOverText => "objOverTx",
            Self::TextOverObject => "txOverObj",
            Self::TextAndTwoObjects => "txAndTwoObj",
            Self::TwoObjectsAndText => "twoObjAndTx",
            Self::TwoObjectsOverText => "twoObjOverTx",
            Self::FourObjects => "fourObj",
            Self::VerticalText => "vertTx",
            Self::ClipArtAndVerticalText => "clipArtAndVertTx",
            Self::VerticalTitleAndText => "vertTitleAndTx",
            Self::VerticalTitleAndTextOverChart => "vertTitleAndTxOverChart",
            Self::TwoObjects => "twoObj",
            Self::ObjectAndTwoObjects => "objAndTwoObj",
            Self::TwoObjectsAndObject => "twoObjAndObj",
            Self::Custom => "cust",
            Self::SectionHeader => "secHead",
            Self::TwoTextAndTwoObjects => "twoTxTwoObj",
            Self::ObjectText => "objTx",
            Self::PictureText => "picTx",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "title" => Some(Self::Title),
            "tx" => Some(Self::Text),
            "twoColTx" => Some(Self::TwoColumnText),
            "tbl" => Some(Self::Table),
            "txAndChart" => Some(Self::TextAndChart),
            "chartAndTx" => Some(Self::ChartAndText),
            "dgm" => Some(Self::Diagram),
            "chart" => Some(Self::Chart),
            "txAndClipArt" => Some(Self::TextAndClipArt),
            "clipArtAndTx" => Some(Self::ClipArtAndText),
            "titleOnly" => Some(Self::TitleOnly),
            "blank" => Some(Self::Blank),
            "txAndObj" => Some(Self::TextAndObject),
            "objAndTx" => Some(Self::ObjectAndText),
            "objOnly" => Some(Self::ObjectOnly),
            "obj" => Some(Self::Object),
            "txAndMedia" => Some(Self::TextAndMedia),
            "mediaAndTx" => Some(Self::MidiaAndText),
            "objOverTx" => Some(Self::ObjectOverText),
            "txOverObj" => Some(Self::TextOverObject),
            "txAndTwoObj" => Some(Self::TextAndTwoObjects),
            "twoObjAndTx" => Some(Self::TwoObjectsAndText),
            "twoObjOverTx" => Some(Self::TwoObjectsOverText),
            "fourObj" => Some(Self::FourObjects),
            "vertTx" => Some(Self::VerticalText),
            "clipArtAndVertTx" => Some(Self::ClipArtAndVerticalText),
            "vertTitleAndTx" => Some(Self::VerticalTitleAndText),
            "vertTitleAndTxOverChart" => Some(Self::VerticalTitleAndTextOverChart),
            "twoObj" => Some(Self::TwoObjects),
            "objAndTwoObj" => Some(Self::ObjectAndTwoObjects),
            "twoObjAndObj" => Some(Self::TwoObjectsAndObject),
            "cust" => Some(Self::Custom),
            "secHead" => Some(Self::SectionHeader),
            "twoTxTwoObj" => Some(Self::TwoTextAndTwoObjects),
            "objTx" => Some(Self::ObjectText),
            "picTx" => Some(Self::PictureText),
            _ => None,
        }
    }
}

impl core::fmt::Display for SlideLayoutValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for SlideLayoutValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Splitter Bar State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SplitterBarStateValues {
    Minimized,
    Restored,
    Maximized,
}

impl SplitterBarStateValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Minimized => "minimized",
            Self::Restored => "restored",
            Self::Maximized => "maximized",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "minimized" => Some(Self::Minimized),
            "restored" => Some(Self::Restored),
            "maximized" => Some(Self::Maximized),
            _ => None,
        }
    }
}

impl core::fmt::Display for SplitterBarStateValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for SplitterBarStateValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// List of View Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewValues {
    SlideView,
    SlideMasterView,
    NotesView,
    HandoutView,
    NotesMasterView,
    OutlineView,
    SlideSorterView,
    SlideThumbnailView,
}

impl ViewValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SlideView => "sldView",
            Self::SlideMasterView => "sldMasterView",
            Self::NotesView => "notesView",
            Self::HandoutView => "handoutView",
            Self::NotesMasterView => "notesMasterView",
            Self::OutlineView => "outlineView",
            Self::SlideSorterView => "sldSorterView",
            Self::SlideThumbnailView => "sldThumbnailView",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sldView" => Some(Self::SlideView),
            "sldMasterView" => Some(Self::SlideMasterView),
            "notesView" => Some(Self::NotesView),
            "handoutView" => Some(Self::HandoutView),
            "notesMasterView" => Some(Self::NotesMasterView),
            "outlineView" => Some(Self::OutlineView),
            "sldSorterView" => Some(Self::SlideSorterView),
            "sldThumbnailView" => Some(Self::SlideThumbnailView),
            _ => None,
        }
    }
}

impl core::fmt::Display for ViewValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for ViewValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Trigger Event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerEventValues {
    None_,
    OnBegin,
    OnEnd,
    Begin,
    End,
    OnClick,
    OnDoubleClick,
    OnMouseOver,
    OnMouseOut,
    OnNext,
    OnPrevious,
    OnStopAudio,
    OnMediaBookmark,
}

impl TriggerEventValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::OnBegin => "onBegin",
            Self::OnEnd => "onEnd",
            Self::Begin => "begin",
            Self::End => "end",
            Self::OnClick => "onClick",
            Self::OnDoubleClick => "onDblClick",
            Self::OnMouseOver => "onMouseOver",
            Self::OnMouseOut => "onMouseOut",
            Self::OnNext => "onNext",
            Self::OnPrevious => "onPrev",
            Self::OnStopAudio => "onStopAudio",
            Self::OnMediaBookmark => "onMediaBookmark",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "onBegin" => Some(Self::OnBegin),
            "onEnd" => Some(Self::OnEnd),
            "begin" => Some(Self::Begin),
            "end" => Some(Self::End),
            "onClick" => Some(Self::OnClick),
            "onDblClick" => Some(Self::OnDoubleClick),
            "onMouseOver" => Some(Self::OnMouseOver),
            "onMouseOut" => Some(Self::OnMouseOut),
            "onNext" => Some(Self::OnNext),
            "onPrev" => Some(Self::OnPrevious),
            "onStopAudio" => Some(Self::OnStopAudio),
            "onMediaBookmark" => Some(Self::OnMediaBookmark),
            _ => None,
        }
    }
}

impl core::fmt::Display for TriggerEventValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TriggerEventValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Schema enum `ConformanceClassValues`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConformanceClassValues {
    Strict,
    Transitional,
}

impl ConformanceClassValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Transitional => "transitional",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "strict" => Some(Self::Strict),
            "transitional" => Some(Self::Transitional),
            _ => None,
        }
    }
}

impl core::fmt::Display for ConformanceClassValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for ConformanceClassValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 269;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 248;
/// Number of generated enums.
pub const ENUM_COUNT: usize = 51;
/// Number of generated content-model particles.
pub const PARTICLE_COUNT: usize = 172;
