//! Auto-generated from `schemas_microsoft_com_office_word_2006_wordml.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2006/wordml` (prefix `wne`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2006/wordml";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wne";

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

static CHILDREN_TEMPLATE_COMMAND_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_Keymaps/wne:keymaps", property_name: None },
    ChildInfo { name: "wne:CT_Keymaps/wne:keymapsBad", property_name: None },
    ChildInfo { name: "wne:CT_Toolbars/wne:toolbars", property_name: None },
    ChildInfo { name: "wne:CT_Acds/wne:acds", property_name: None },
];
static CHILDREN_MCDS: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_Mcd/wne:mcd", property_name: None },
];
static CHILDREN_VBA_SUPP_DATA: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_DocEvents/wne:docEvents", property_name: Some("DocEvents") },
    ChildInfo { name: "wne:CT_Mcds/wne:mcds", property_name: Some("Mcds") },
];
static CHILDREN_MAIL_MERGE_RECIPIENTS: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_HashedRecipientData/wne:recipientData", property_name: None },
];
static ATTRS_FIXED_COMMAND_KEYBOARD_CUSTOMIZATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:fciName", property_name: Some("CommandName"), type_name: "StringValue" },
    AttributeInfo { qname: "wne:fciIndex", property_name: Some("CommandIndex"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wne:swArg", property_name: Some("Argument"), type_name: "HexBinaryValue" },
];
static ATTRS_MACRO_KEYBOARD_CUSTOMIZATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:macroName", property_name: Some("MacroName"), type_name: "StringValue" },
];
static ATTRS_WLL_MACRO_KEYBOARD_CUSTOMIZATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:macroName", property_name: Some("MacroName"), type_name: "StringValue" },
];
static ATTRS_ALLOCATED_COMMAND_KEYBOARD_CUSTOMIZATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:acdName", property_name: Some("AcceleratorName"), type_name: "StringValue" },
];
static ATTRS_ALLOCATED_COMMAND_MANIFEST_ENTRY: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:acdName", property_name: Some("AcceleratorName"), type_name: "StringValue" },
];
static ATTRS_CHARACTER_INSERTION: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:val", property_name: Some("Val"), type_name: "HexBinaryValue" },
];
static ATTRS_KEY_MAP_ENTRY: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:chmPrimary", property_name: Some("CharacterMapPrimary"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wne:chmSecondary", property_name: Some("CharacterMapSecondary"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wne:kcmPrimary", property_name: Some("KeyCodePrimary"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wne:kcmSecondary", property_name: Some("KeyCodeSecondary"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wne:mask", property_name: Some("Mask"), type_name: "OnOffValue" },
];
static CHILDREN_KEY_MAP_ENTRY: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_Fci/wne:fci", property_name: Some("FixedCommandKeyboardCustomization") },
    ChildInfo { name: "wne:CT_MacroWll/wne:macro", property_name: Some("MacroKeyboardCustomization") },
    ChildInfo { name: "wne:CT_AcdKeymap/wne:acd", property_name: Some("AllocatedCommandKeyboardCustomization") },
    ChildInfo { name: "wne:CT_MacroWll/wne:wll", property_name: Some("WllMacroKeyboardCustomization") },
    ChildInfo { name: "wne:CT_LongHexNumber/wne:wch", property_name: Some("CharacterInsertion") },
];
static ATTRS_ALLOCATED_COMMAND: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:argValue", property_name: Some("ArgumentValue"), type_name: "StringValue" },
    AttributeInfo { qname: "wne:fciBasedOn", property_name: Some("CommandBasedOn"), type_name: "StringValue" },
    AttributeInfo { qname: "wne:fciIndexBasedOn", property_name: Some("CommandIndexBasedOn"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wne:acdName", property_name: Some("AcceleratorName"), type_name: "StringValue" },
];
static ATTRS_MCD: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:macroName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: "wne:name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: "wne:menuHelp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: "wne:bEncrypt", property_name: None, type_name: "HexBinaryValue" },
    AttributeInfo { qname: "wne:cmg", property_name: None, type_name: "HexBinaryValue" },
];
static CHILDREN_DOC_EVENTS: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/wne:eventDocNew", property_name: Some("EventDocNewXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocOpen", property_name: Some("EventDocOpenXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocClose", property_name: Some("EventDocCloseXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocSync", property_name: Some("EventDocSyncXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocXmlAfterInsert", property_name: Some("EventDocXmlAfterInsertXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocXmlBeforeDelete", property_name: Some("EventDocXmlBeforeDeleteXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocContentControlAfterInsert", property_name: Some("EventDocContentControlAfterInsertXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocContentControlBeforeDelete", property_name: Some("EventDocContentControlBeforeDeleteXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocContentControlOnExit", property_name: Some("EventDocContentControlOnExistXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocContentControlOnEnter", property_name: Some("EventDocContentControlOnEnterXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocStoreUpdate", property_name: Some("EventDocStoreUpdateXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocContentControlContentUpdate", property_name: Some("EventDocContentControlUpdateXsdString") },
    ChildInfo { name: "xsd:string/wne:eventDocBuildingBlockAfterInsert", property_name: Some("EventDocBuildingBlockAfterInsertXsdString") },
];
static CHILDREN_ALLOCATED_COMMAND_MANIFEST: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_AcdKeymap/wne:acdEntry", property_name: None },
];
static ATTRS_TOOLBAR_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_KEY_MAP_CUSTOMIZATIONS: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_Keymap/wne:keymap", property_name: None },
];
static CHILDREN_MISMATCHED_KEY_MAP_CUSTOMIZATION: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_Keymap/wne:keymap", property_name: None },
];
static CHILDREN_TOOLBARS: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_AcdManifest/wne:acdManifest", property_name: None },
    ChildInfo { name: "wne:CT_Rel/wne:toolbarData", property_name: None },
];
static CHILDREN_ALLOCATED_COMMANDS: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_Acd/wne:acd", property_name: None },
];
static ATTRS_RECORD_INCLUDED: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:val", property_name: Some("Val"), type_name: "OnOffValue" },
];
static ATTRS_RECORD_HASH_CODE: &[AttributeInfo] = &[
    AttributeInfo { qname: "wne:val", property_name: Some("Val"), type_name: "IntegerValue" },
];
static CHILDREN_SINGLE_DATA_SOURCE_RECORD: &[ChildInfo] = &[
    ChildInfo { name: "wne:CT_OnOff/wne:active", property_name: Some("RecordIncluded") },
    ChildInfo { name: "wne:CT_DecimalNumber/wne:hash", property_name: Some("RecordHashCode") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "TemplateCommandGroup", local_name: "tcg", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEMPLATE_COMMAND_GROUP },
    ElementInfo { class_name: "Mcds", local_name: "mcds", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MCDS },
    ElementInfo { class_name: "VbaSuppData", local_name: "vbaSuppData", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VBA_SUPP_DATA },
    ElementInfo { class_name: "MailMergeRecipients", local_name: "recipients", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MAIL_MERGE_RECIPIENTS },
    ElementInfo { class_name: "FixedCommandKeyboardCustomization", local_name: "fci", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FIXED_COMMAND_KEYBOARD_CUSTOMIZATION, children: &[] },
    ElementInfo { class_name: "MacroKeyboardCustomization", local_name: "macro", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MACRO_KEYBOARD_CUSTOMIZATION, children: &[] },
    ElementInfo { class_name: "WllMacroKeyboardCustomization", local_name: "wll", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WLL_MACRO_KEYBOARD_CUSTOMIZATION, children: &[] },
    ElementInfo { class_name: "AllocatedCommandKeyboardCustomization", local_name: "acd", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALLOCATED_COMMAND_KEYBOARD_CUSTOMIZATION, children: &[] },
    ElementInfo { class_name: "AllocatedCommandManifestEntry", local_name: "acdEntry", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALLOCATED_COMMAND_MANIFEST_ENTRY, children: &[] },
    ElementInfo { class_name: "CharacterInsertion", local_name: "wch", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHARACTER_INSERTION, children: &[] },
    ElementInfo { class_name: "KeyMapEntry", local_name: "keymap", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_KEY_MAP_ENTRY, children: CHILDREN_KEY_MAP_ENTRY },
    ElementInfo { class_name: "AllocatedCommand", local_name: "acd", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALLOCATED_COMMAND, children: &[] },
    ElementInfo { class_name: "Mcd", local_name: "mcd", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MCD, children: &[] },
    ElementInfo { class_name: "EventDocNewXsdString", local_name: "eventDocNew", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocOpenXsdString", local_name: "eventDocOpen", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocCloseXsdString", local_name: "eventDocClose", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocSyncXsdString", local_name: "eventDocSync", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocXmlAfterInsertXsdString", local_name: "eventDocXmlAfterInsert", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocXmlBeforeDeleteXsdString", local_name: "eventDocXmlBeforeDelete", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocContentControlAfterInsertXsdString", local_name: "eventDocContentControlAfterInsert", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocContentControlBeforeDeleteXsdString", local_name: "eventDocContentControlBeforeDelete", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocContentControlOnExistXsdString", local_name: "eventDocContentControlOnExit", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocContentControlOnEnterXsdString", local_name: "eventDocContentControlOnEnter", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocStoreUpdateXsdString", local_name: "eventDocStoreUpdate", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocContentControlUpdateXsdString", local_name: "eventDocContentControlContentUpdate", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "EventDocBuildingBlockAfterInsertXsdString", local_name: "eventDocBuildingBlockAfterInsert", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DocEvents", local_name: "docEvents", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DOC_EVENTS },
    ElementInfo { class_name: "AllocatedCommandManifest", local_name: "acdManifest", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ALLOCATED_COMMAND_MANIFEST },
    ElementInfo { class_name: "ToolbarData", local_name: "toolbarData", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOOLBAR_DATA, children: &[] },
    ElementInfo { class_name: "KeyMapCustomizations", local_name: "keymaps", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_KEY_MAP_CUSTOMIZATIONS },
    ElementInfo { class_name: "MismatchedKeyMapCustomization", local_name: "keymapsBad", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MISMATCHED_KEY_MAP_CUSTOMIZATION },
    ElementInfo { class_name: "Toolbars", local_name: "toolbars", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TOOLBARS },
    ElementInfo { class_name: "AllocatedCommands", local_name: "acds", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ALLOCATED_COMMANDS },
    ElementInfo { class_name: "RecordIncluded", local_name: "active", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RECORD_INCLUDED, children: &[] },
    ElementInfo { class_name: "RecordHashCode", local_name: "hash", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RECORD_HASH_CODE, children: &[] },
    ElementInfo { class_name: "SingleDataSourceRecord", local_name: "recipientData", prefix: "wne", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SINGLE_DATA_SOURCE_RECORD },
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

/// Create a `<wne:tcg>` element (`TemplateCommandGroup`).
pub fn template_command_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "tcg").with_children(children)
}

/// Create a `<wne:mcds>` element (`Mcds`).
pub fn mcds(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "mcds").with_children(children)
}

/// Create a `<wne:vbaSuppData>` element (`VbaSuppData`).
pub fn vba_supp_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "vbaSuppData").with_children(children)
}

/// Create a `<wne:recipients>` element (`MailMergeRecipients`).
pub fn mail_merge_recipients(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "recipients").with_children(children)
}

/// Create a `<wne:fci>` element (`FixedCommandKeyboardCustomization`).
pub fn fixed_command_keyboard_customization() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "fci")
}

/// Create a `<wne:macro>` element (`MacroKeyboardCustomization`).
pub fn macro_keyboard_customization() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "macro")
}

/// Create a `<wne:wll>` element (`WllMacroKeyboardCustomization`).
pub fn wll_macro_keyboard_customization() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "wll")
}

/// Create a `<wne:acd>` element (`AllocatedCommandKeyboardCustomization`).
pub fn allocated_command_keyboard_customization() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "acd")
}

/// Create a `<wne:acdEntry>` element (`AllocatedCommandManifestEntry`).
pub fn allocated_command_manifest_entry() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "acdEntry")
}

/// Create a `<wne:wch>` element (`CharacterInsertion`).
pub fn character_insertion() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "wch")
}

/// Create a `<wne:keymap>` element (`KeyMapEntry`).
pub fn key_map_entry(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "keymap").with_children(children)
}

/// Create a `<wne:acd>` element (`AllocatedCommand`).
pub fn allocated_command() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "acd")
}

/// Create a `<wne:mcd>` element (`Mcd`).
pub fn mcd() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "mcd")
}

/// Create a `<wne:eventDocNew>` element (`EventDocNewXsdString`).
pub fn event_doc_new_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocNew").with_text(value)
}

/// Create a `<wne:eventDocOpen>` element (`EventDocOpenXsdString`).
pub fn event_doc_open_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocOpen").with_text(value)
}

/// Create a `<wne:eventDocClose>` element (`EventDocCloseXsdString`).
pub fn event_doc_close_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocClose").with_text(value)
}

/// Create a `<wne:eventDocSync>` element (`EventDocSyncXsdString`).
pub fn event_doc_sync_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocSync").with_text(value)
}

/// Create a `<wne:eventDocXmlAfterInsert>` element (`EventDocXmlAfterInsertXsdString`).
pub fn event_doc_xml_after_insert_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocXmlAfterInsert").with_text(value)
}

/// Create a `<wne:eventDocXmlBeforeDelete>` element (`EventDocXmlBeforeDeleteXsdString`).
pub fn event_doc_xml_before_delete_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocXmlBeforeDelete").with_text(value)
}

/// Create a `<wne:eventDocContentControlAfterInsert>` element (`EventDocContentControlAfterInsertXsdString`).
pub fn event_doc_content_control_after_insert_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocContentControlAfterInsert").with_text(value)
}

/// Create a `<wne:eventDocContentControlBeforeDelete>` element (`EventDocContentControlBeforeDeleteXsdString`).
pub fn event_doc_content_control_before_delete_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocContentControlBeforeDelete").with_text(value)
}

/// Create a `<wne:eventDocContentControlOnExit>` element (`EventDocContentControlOnExistXsdString`).
pub fn event_doc_content_control_on_exist_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocContentControlOnExit").with_text(value)
}

/// Create a `<wne:eventDocContentControlOnEnter>` element (`EventDocContentControlOnEnterXsdString`).
pub fn event_doc_content_control_on_enter_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocContentControlOnEnter").with_text(value)
}

/// Create a `<wne:eventDocStoreUpdate>` element (`EventDocStoreUpdateXsdString`).
pub fn event_doc_store_update_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocStoreUpdate").with_text(value)
}

/// Create a `<wne:eventDocContentControlContentUpdate>` element (`EventDocContentControlUpdateXsdString`).
pub fn event_doc_content_control_update_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocContentControlContentUpdate").with_text(value)
}

/// Create a `<wne:eventDocBuildingBlockAfterInsert>` element (`EventDocBuildingBlockAfterInsertXsdString`).
pub fn event_doc_building_block_after_insert_xsd_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "eventDocBuildingBlockAfterInsert").with_text(value)
}

/// Create a `<wne:docEvents>` element (`DocEvents`).
pub fn doc_events(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "docEvents").with_children(children)
}

/// Create a `<wne:acdManifest>` element (`AllocatedCommandManifest`).
pub fn allocated_command_manifest(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "acdManifest").with_children(children)
}

/// Create a `<wne:toolbarData>` element (`ToolbarData`).
pub fn toolbar_data() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "toolbarData")
}

/// Create a `<wne:keymaps>` element (`KeyMapCustomizations`).
pub fn key_map_customizations(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "keymaps").with_children(children)
}

/// Create a `<wne:keymapsBad>` element (`MismatchedKeyMapCustomization`).
pub fn mismatched_key_map_customization(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "keymapsBad").with_children(children)
}

/// Create a `<wne:toolbars>` element (`Toolbars`).
pub fn toolbars(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "toolbars").with_children(children)
}

/// Create a `<wne:acds>` element (`AllocatedCommands`).
pub fn allocated_commands(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "acds").with_children(children)
}

/// Create a `<wne:active>` element (`RecordIncluded`).
pub fn record_included() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "active")
}

/// Create a `<wne:hash>` element (`RecordHashCode`).
pub fn record_hash_code() -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "hash")
}

/// Create a `<wne:recipientData>` element (`SingleDataSourceRecord`).
pub fn single_data_source_record(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wne", NAMESPACE_URI, "recipientData").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 39;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 36;
