//! Auto-generated from `schemas_microsoft_com_office_powerpoint_2013_main_command.json`.
//! Target namespace: `http://schemas.microsoft.com/office/powerpoint/2013/main/command` (prefix `pc`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/powerpoint/2013/main/command";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "pc";

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

static CHILDREN_SLIDE_MONIKER_LIST: &[ChildInfo] = &[
    ChildInfo { name: "pc:CT_DocumentMoniker/pc:docMk", property_name: Some("DocumentMoniker") },
    ChildInfo { name: "pc:CT_SlideMoniker/pc:sldMk", property_name: Some("SlideMoniker") },
];
static ATTRS_SLIDE_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cId", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":sldId", property_name: None, type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CommentAuthorMonikerList", local_name: "cmAuthorMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "CommentMonikerList", local_name: "cmMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "StringTagMonikerList", local_name: "tagMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "CustomShowMonikerList", local_name: "custShowMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DocumentMonikerList", local_name: "docMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SectionMonikerList", local_name: "sectionMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SlideBaseMonikerList", local_name: "sldBaseMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SlideLayoutMonikerList", local_name: "sldLayoutMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "MainMasterMonikerList", local_name: "sldMasterMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SlideMonikerList", local_name: "sldMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SLIDE_MONIKER_LIST },
    ElementInfo { class_name: "SlidePosMonikerList", local_name: "sldPosMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NotesMonikerList", local_name: "notesMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NotesTextMonikerList", local_name: "notesTxtMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NotesMasterMonikerList", local_name: "notesMasterMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "HandoutMonikerList", local_name: "handoutMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "AnimEffectMkLstAnimationEffectMonikerList", local_name: "animEffectMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "AnimEffectParentMkLstAnimationEffectMonikerList", local_name: "animEffectParentMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "OsfTaskPaneAppMonikerList", local_name: "tkAppMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SummaryZoomMonikerList", local_name: "tocMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SectionLinkObjMonikerList", local_name: "sectionLnkObjMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DesignerTagMonikerList", local_name: "designTagMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "CustomXmlPartMonikerList", local_name: "cXmlMkLst", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DocumentMoniker", local_name: "docMk", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SlideMoniker", local_name: "sldMk", prefix: "pc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SLIDE_MONIKER, children: &[] },
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

/// Create a `<pc:cmAuthorMkLst>` element (`CommentAuthorMonikerList`).
pub fn comment_author_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "cmAuthorMkLst").with_children(children)
}

/// Create a `<pc:cmMkLst>` element (`CommentMonikerList`).
pub fn comment_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "cmMkLst").with_children(children)
}

/// Create a `<pc:tagMkLst>` element (`StringTagMonikerList`).
pub fn string_tag_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "tagMkLst").with_children(children)
}

/// Create a `<pc:custShowMkLst>` element (`CustomShowMonikerList`).
pub fn custom_show_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "custShowMkLst").with_children(children)
}

/// Create a `<pc:docMkLst>` element (`DocumentMonikerList`).
pub fn document_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "docMkLst").with_children(children)
}

/// Create a `<pc:sectionMkLst>` element (`SectionMonikerList`).
pub fn section_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sectionMkLst").with_children(children)
}

/// Create a `<pc:sldBaseMkLst>` element (`SlideBaseMonikerList`).
pub fn slide_base_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sldBaseMkLst").with_children(children)
}

/// Create a `<pc:sldLayoutMkLst>` element (`SlideLayoutMonikerList`).
pub fn slide_layout_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sldLayoutMkLst").with_children(children)
}

/// Create a `<pc:sldMasterMkLst>` element (`MainMasterMonikerList`).
pub fn main_master_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sldMasterMkLst").with_children(children)
}

/// Create a `<pc:sldMkLst>` element (`SlideMonikerList`).
pub fn slide_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sldMkLst").with_children(children)
}

/// Create a `<pc:sldPosMkLst>` element (`SlidePosMonikerList`).
pub fn slide_pos_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sldPosMkLst").with_children(children)
}

/// Create a `<pc:notesMkLst>` element (`NotesMonikerList`).
pub fn notes_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "notesMkLst").with_children(children)
}

/// Create a `<pc:notesTxtMkLst>` element (`NotesTextMonikerList`).
pub fn notes_text_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "notesTxtMkLst").with_children(children)
}

/// Create a `<pc:notesMasterMkLst>` element (`NotesMasterMonikerList`).
pub fn notes_master_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "notesMasterMkLst").with_children(children)
}

/// Create a `<pc:handoutMkLst>` element (`HandoutMonikerList`).
pub fn handout_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "handoutMkLst").with_children(children)
}

/// Create a `<pc:animEffectMkLst>` element (`AnimEffectMkLstAnimationEffectMonikerList`).
pub fn anim_effect_mk_lst_animation_effect_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "animEffectMkLst").with_children(children)
}

/// Create a `<pc:animEffectParentMkLst>` element (`AnimEffectParentMkLstAnimationEffectMonikerList`).
pub fn anim_effect_parent_mk_lst_animation_effect_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "animEffectParentMkLst").with_children(children)
}

/// Create a `<pc:tkAppMkLst>` element (`OsfTaskPaneAppMonikerList`).
pub fn osf_task_pane_app_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "tkAppMkLst").with_children(children)
}

/// Create a `<pc:tocMkLst>` element (`SummaryZoomMonikerList`).
pub fn summary_zoom_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "tocMkLst").with_children(children)
}

/// Create a `<pc:sectionLnkObjMkLst>` element (`SectionLinkObjMonikerList`).
pub fn section_link_obj_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sectionLnkObjMkLst").with_children(children)
}

/// Create a `<pc:designTagMkLst>` element (`DesignerTagMonikerList`).
pub fn designer_tag_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "designTagMkLst").with_children(children)
}

/// Create a `<pc:cXmlMkLst>` element (`CustomXmlPartMonikerList`).
pub fn custom_xml_part_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "cXmlMkLst").with_children(children)
}

/// Create a `<pc:docMk>` element (`DocumentMoniker`).
pub fn document_moniker() -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "docMk")
}

/// Create a `<pc:sldMk>` element (`SlideMoniker`).
pub fn slide_moniker() -> OpenXmlElement {
    OpenXmlElement::new("pc", NAMESPACE_URI, "sldMk")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 25;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 24;
