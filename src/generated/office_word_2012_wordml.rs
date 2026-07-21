//! Auto-generated from `schemas_microsoft_com_office_word_2012_wordml.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2012/wordml` (prefix `w15`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2012/wordml";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "w15";

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

static ATTRS_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "StringValue" },
    AttributeInfo { qname: "w:themeColor", property_name: Some("ThemeColor"), type_name: "EnumValue" },
    AttributeInfo { qname: "w:themeTint", property_name: Some("ThemeTint"), type_name: "StringValue" },
    AttributeInfo { qname: "w:themeShade", property_name: Some("ThemeShade"), type_name: "StringValue" },
];
static ATTRS_DATA_BINDING: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:prefixMappings", property_name: Some("PrefixMappings"), type_name: "StringValue" },
    AttributeInfo { qname: "w:xpath", property_name: Some("XPath"), type_name: "StringValue" },
    AttributeInfo { qname: "w:storeItemID", property_name: Some("StoreItemId"), type_name: "StringValue" },
];
static ATTRS_APPEARANCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w15:val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_COMMENTS_EX: &[ChildInfo] = &[
    ChildInfo { name: "w15:CT_CommentEx/w15:commentEx", property_name: None },
];
static CHILDREN_PEOPLE: &[ChildInfo] = &[
    ChildInfo { name: "w15:CT_Person/w15:person", property_name: None },
];
static CHILDREN_SDT_REPEATED_SECTION: &[ChildInfo] = &[
    ChildInfo { name: "w:CT_String/w15:sectionTitle", property_name: Some("SectionTitle") },
    ChildInfo { name: "w:CT_OnOff/w15:doNotAllowInsertDeleteSection", property_name: Some("DoNotAllowInsertDeleteSection") },
];
static ATTRS_CHART_TRACKING_REF_BASED: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "OnOffValue" },
];
static ATTRS_DEFAULT_COLLAPSED: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "OnOffValue" },
];
static ATTRS_WEB_EXTENSION_LINKED: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "OnOffValue" },
];
static ATTRS_WEB_EXTENSION_CREATED: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "OnOffValue" },
];
static ATTRS_DO_NOT_ALLOW_INSERT_DELETE_SECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "OnOffValue" },
];
static ATTRS_PERSISTENT_DOCUMENT_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: "w15:val", property_name: None, type_name: "StringValue" },
];
static ATTRS_FOOTNOTE_COLUMNS: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_COMMENT_EX: &[AttributeInfo] = &[
    AttributeInfo { qname: "w15:paraId", property_name: Some("ParaId"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "w15:paraIdParent", property_name: Some("ParaIdParent"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "w15:done", property_name: Some("Done"), type_name: "OnOffValue" },
];
static ATTRS_PERSON: &[AttributeInfo] = &[
    AttributeInfo { qname: "w15:author", property_name: Some("Author"), type_name: "StringValue" },
];
static CHILDREN_PERSON: &[ChildInfo] = &[
    ChildInfo { name: "w15:CT_PresenceInfo/w15:presenceInfo", property_name: Some("PresenceInfo") },
];
static ATTRS_PRESENCE_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: "w15:providerId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: "w15:userId", property_name: None, type_name: "StringValue" },
];
static ATTRS_SECTION_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: "w:val", property_name: Some("Val"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Color", local_name: "color", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR, children: &[] },
    ElementInfo { class_name: "DataBinding", local_name: "dataBinding", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DATA_BINDING, children: &[] },
    ElementInfo { class_name: "Appearance", local_name: "appearance", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_APPEARANCE, children: &[] },
    ElementInfo { class_name: "CommentsEx", local_name: "commentsEx", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENTS_EX },
    ElementInfo { class_name: "People", local_name: "people", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PEOPLE },
    ElementInfo { class_name: "SdtRepeatedSection", local_name: "repeatingSection", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SDT_REPEATED_SECTION },
    ElementInfo { class_name: "SdtRepeatedSectionItem", local_name: "repeatingSectionItem", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ChartTrackingRefBased", local_name: "chartTrackingRefBased", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHART_TRACKING_REF_BASED, children: &[] },
    ElementInfo { class_name: "DefaultCollapsed", local_name: "collapsed", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DEFAULT_COLLAPSED, children: &[] },
    ElementInfo { class_name: "WebExtensionLinked", local_name: "webExtensionLinked", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_LINKED, children: &[] },
    ElementInfo { class_name: "WebExtensionCreated", local_name: "webExtensionCreated", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WEB_EXTENSION_CREATED, children: &[] },
    ElementInfo { class_name: "DoNotAllowInsertDeleteSection", local_name: "doNotAllowInsertDeleteSection", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DO_NOT_ALLOW_INSERT_DELETE_SECTION, children: &[] },
    ElementInfo { class_name: "PersistentDocumentId", local_name: "docId", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PERSISTENT_DOCUMENT_ID, children: &[] },
    ElementInfo { class_name: "FootnoteColumns", local_name: "footnoteColumns", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FOOTNOTE_COLUMNS, children: &[] },
    ElementInfo { class_name: "CommentEx", local_name: "commentEx", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMENT_EX, children: &[] },
    ElementInfo { class_name: "Person", local_name: "person", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PERSON, children: CHILDREN_PERSON },
    ElementInfo { class_name: "PresenceInfo", local_name: "presenceInfo", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRESENCE_INFO, children: &[] },
    ElementInfo { class_name: "SectionTitle", local_name: "sectionTitle", prefix: "w15", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SECTION_TITLE, children: &[] },
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

/// Create a `<w15:color>` element (`Color`).
pub fn color() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "color")
}

/// Create a `<w15:dataBinding>` element (`DataBinding`).
pub fn data_binding() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "dataBinding")
}

/// Create a `<w15:appearance>` element (`Appearance`).
pub fn appearance() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "appearance")
}

/// Create a `<w15:commentsEx>` element (`CommentsEx`).
pub fn comments_ex(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "commentsEx").with_children(children)
}

/// Create a `<w15:people>` element (`People`).
pub fn people(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "people").with_children(children)
}

/// Create a `<w15:repeatingSection>` element (`SdtRepeatedSection`).
pub fn sdt_repeated_section(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "repeatingSection").with_children(children)
}

/// Create a `<w15:repeatingSectionItem>` element (`SdtRepeatedSectionItem`).
pub fn sdt_repeated_section_item() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "repeatingSectionItem")
}

/// Create a `<w15:chartTrackingRefBased>` element (`ChartTrackingRefBased`).
pub fn chart_tracking_ref_based() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "chartTrackingRefBased")
}

/// Create a `<w15:collapsed>` element (`DefaultCollapsed`).
pub fn default_collapsed() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "collapsed")
}

/// Create a `<w15:webExtensionLinked>` element (`WebExtensionLinked`).
pub fn web_extension_linked() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "webExtensionLinked")
}

/// Create a `<w15:webExtensionCreated>` element (`WebExtensionCreated`).
pub fn web_extension_created() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "webExtensionCreated")
}

/// Create a `<w15:doNotAllowInsertDeleteSection>` element (`DoNotAllowInsertDeleteSection`).
pub fn do_not_allow_insert_delete_section() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "doNotAllowInsertDeleteSection")
}

/// Create a `<w15:docId>` element (`PersistentDocumentId`).
pub fn persistent_document_id() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "docId")
}

/// Create a `<w15:footnoteColumns>` element (`FootnoteColumns`).
pub fn footnote_columns() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "footnoteColumns")
}

/// Create a `<w15:commentEx>` element (`CommentEx`).
pub fn comment_ex() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "commentEx")
}

/// Create a `<w15:person>` element (`Person`).
pub fn person(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "person").with_children(children)
}

/// Create a `<w15:presenceInfo>` element (`PresenceInfo`).
pub fn presence_info() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "presenceInfo")
}

/// Create a `<w15:sectionTitle>` element (`SectionTitle`).
pub fn section_title() -> OpenXmlElement {
    OpenXmlElement::new("w15", NAMESPACE_URI, "sectionTitle")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 19;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 18;
