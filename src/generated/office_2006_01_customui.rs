//! Auto-generated from `schemas_microsoft_com_office_2006_01_customui.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2006/01/customui` (prefix `mso`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2006/01/customui";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "mso";

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

static ATTRS_UNSIZED_CONTROL_CLONE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_UNSIZED_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_CHECK_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":getPressed", property_name: Some("GetPressed"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static ATTRS_UNSIZED_GALLERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidateContentOnDrop", property_name: Some("InvalidateContentOnDrop"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":columns", property_name: Some("Columns"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":rows", property_name: Some("Rows"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":itemWidth", property_name: Some("ItemWidth"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":itemHeight", property_name: Some("ItemHeight"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":getItemWidth", property_name: Some("GetItemWidth"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemHeight", property_name: Some("GetItemHeight"), type_name: "StringValue" },
    AttributeInfo { qname: ":showItemLabel", property_name: Some("ShowItemLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":showItemImage", property_name: Some("ShowItemImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getItemCount", property_name: Some("GetItemCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemLabel", property_name: Some("GetItemLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemScreentip", property_name: Some("GetItemScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemSupertip", property_name: Some("GetItemSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemImage", property_name: Some("GetItemImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemID", property_name: Some("GetItemID"), type_name: "StringValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemID", property_name: Some("GetSelectedItemID"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemIndex", property_name: Some("GetSelectedItemIndex"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_UNSIZED_GALLERY: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Item/mso:item", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
];
static ATTRS_UNSIZED_TOGGLE_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":getPressed", property_name: Some("GetPressed"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_MENU_SEPARATOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: ":getTitle", property_name: Some("GetTitle"), type_name: "StringValue" },
];
static ATTRS_UNSIZED_SPLIT_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static CHILDREN_UNSIZED_SPLIT_BUTTON: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_VisibleButton/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_VisibleToggleButton/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuRegular/mso:menu", property_name: None },
];
static ATTRS_UNSIZED_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_UNSIZED_MENU: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneRegular/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_CheckBox/mso:checkBox", property_name: None },
    ChildInfo { name: "mso:CT_GalleryRegular/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButtonRegular/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuSeparator/mso:menuSeparator", property_name: None },
    ChildInfo { name: "mso:CT_SplitButtonRegular/mso:splitButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuRegular/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenuRegular/mso:dynamicMenu", property_name: None },
];
static ATTRS_UNSIZED_DYNAMIC_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":getContent", property_name: Some("GetContent"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidateContentOnDrop", property_name: Some("InvalidateContentOnDrop"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_SPLIT_BUTTON_WITH_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static CHILDREN_SPLIT_BUTTON_WITH_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_VisibleButton/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_VisibleToggleButton/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuWithTitle/mso:menu", property_name: None },
];
static ATTRS_MENU_WITH_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: ":getTitle", property_name: Some("GetTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_MENU_WITH_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneRegular/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_CheckBox/mso:checkBox", property_name: None },
    ChildInfo { name: "mso:CT_GalleryRegular/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButtonRegular/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuSeparator/mso:menuSeparator", property_name: None },
    ChildInfo { name: "mso:CT_SplitButtonWithTitle/mso:splitButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuWithTitle/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenuRegular/mso:dynamicMenu", property_name: None },
];
static ATTRS_CONTROL_CLONE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_TEXT_LABEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static ATTRS_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_TOGGLE_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":getPressed", property_name: Some("GetPressed"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_EDIT_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":maxLength", property_name: Some("MaxLength"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":getText", property_name: Some("GetText"), type_name: "StringValue" },
    AttributeInfo { qname: ":onChange", property_name: Some("OnChange"), type_name: "StringValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_COMBO_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showItemImage", property_name: Some("ShowItemImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getItemCount", property_name: Some("GetItemCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemLabel", property_name: Some("GetItemLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemScreentip", property_name: Some("GetItemScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemSupertip", property_name: Some("GetItemSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemImage", property_name: Some("GetItemImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemID", property_name: Some("GetItemID"), type_name: "StringValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidateContentOnDrop", property_name: Some("InvalidateContentOnDrop"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":maxLength", property_name: Some("MaxLength"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":getText", property_name: Some("GetText"), type_name: "StringValue" },
    AttributeInfo { qname: ":onChange", property_name: Some("OnChange"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_COMBO_BOX: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Item/mso:item", property_name: None },
];
static ATTRS_DROP_DOWN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":showItemImage", property_name: Some("ShowItemImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getItemCount", property_name: Some("GetItemCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemLabel", property_name: Some("GetItemLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemScreentip", property_name: Some("GetItemScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemSupertip", property_name: Some("GetItemSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemImage", property_name: Some("GetItemImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemID", property_name: Some("GetItemID"), type_name: "StringValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemID", property_name: Some("GetSelectedItemID"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemIndex", property_name: Some("GetSelectedItemIndex"), type_name: "StringValue" },
    AttributeInfo { qname: ":showItemLabel", property_name: Some("ShowItemLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_DROP_DOWN: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Item/mso:item", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
];
static ATTRS_GALLERY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidateContentOnDrop", property_name: Some("InvalidateContentOnDrop"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":columns", property_name: Some("Columns"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":rows", property_name: Some("Rows"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":itemWidth", property_name: Some("ItemWidth"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":itemHeight", property_name: Some("ItemHeight"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":getItemWidth", property_name: Some("GetItemWidth"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemHeight", property_name: Some("GetItemHeight"), type_name: "StringValue" },
    AttributeInfo { qname: ":showItemLabel", property_name: Some("ShowItemLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":showItemImage", property_name: Some("ShowItemImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getItemCount", property_name: Some("GetItemCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemLabel", property_name: Some("GetItemLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemScreentip", property_name: Some("GetItemScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemSupertip", property_name: Some("GetItemSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemImage", property_name: Some("GetItemImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemID", property_name: Some("GetItemID"), type_name: "StringValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemID", property_name: Some("GetSelectedItemID"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemIndex", property_name: Some("GetSelectedItemIndex"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_GALLERY: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Item/mso:item", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
];
static ATTRS_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_MENU: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneRegular/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_CheckBox/mso:checkBox", property_name: None },
    ChildInfo { name: "mso:CT_GalleryRegular/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButtonRegular/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuSeparator/mso:menuSeparator", property_name: None },
    ChildInfo { name: "mso:CT_SplitButtonRegular/mso:splitButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuRegular/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenuRegular/mso:dynamicMenu", property_name: None },
];
static ATTRS_DYNAMIC_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":getContent", property_name: Some("GetContent"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidateContentOnDrop", property_name: Some("InvalidateContentOnDrop"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_SPLIT_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static CHILDREN_SPLIT_BUTTON: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_VisibleButton/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_VisibleToggleButton/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuRegular/mso:menu", property_name: None },
];
static ATTRS_BOX_: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":boxStyle", property_name: Some("BoxStyle"), type_name: "EnumValue" },
];
static CHILDREN_BOX_: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlClone/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_LabelControl/mso:labelControl", property_name: None },
    ChildInfo { name: "mso:CT_Button/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButton/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_CheckBox/mso:checkBox", property_name: None },
    ChildInfo { name: "mso:CT_EditBox/mso:editBox", property_name: None },
    ChildInfo { name: "mso:CT_ComboBox/mso:comboBox", property_name: None },
    ChildInfo { name: "mso:CT_DropDownRegular/mso:dropDown", property_name: None },
    ChildInfo { name: "mso:CT_Gallery/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_Menu/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenu/mso:dynamicMenu", property_name: None },
    ChildInfo { name: "mso:CT_SplitButton/mso:splitButton", property_name: None },
    ChildInfo { name: "mso:CT_Box/mso:box", property_name: None },
    ChildInfo { name: "mso:CT_ButtonGroup/mso:buttonGroup", property_name: None },
];
static ATTRS_BUTTON_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
];
static CHILDREN_BUTTON_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneRegular/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButtonRegular/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_GalleryRegular/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_MenuRegular/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenuRegular/mso:dynamicMenu", property_name: None },
    ChildInfo { name: "mso:CT_SplitButtonRegular/mso:splitButton", property_name: None },
];
static ATTRS_MENU_ROOT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: ":getTitle", property_name: Some("GetTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
];
static CHILDREN_MENU_ROOT: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneRegular/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_CheckBox/mso:checkBox", property_name: None },
    ChildInfo { name: "mso:CT_GalleryRegular/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButtonRegular/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuSeparator/mso:menuSeparator", property_name: None },
    ChildInfo { name: "mso:CT_SplitButtonRegular/mso:splitButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuRegular/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenuRegular/mso:dynamicMenu", property_name: None },
];
static ATTRS_CUSTOM_U_I: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onLoad", property_name: Some("OnLoad"), type_name: "StringValue" },
    AttributeInfo { qname: ":loadImage", property_name: Some("LoadImage"), type_name: "StringValue" },
];
static CHILDREN_CUSTOM_U_I: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Commands/mso:commands", property_name: Some("RepurposedCommands") },
    ChildInfo { name: "mso:CT_Ribbon/mso:ribbon", property_name: Some("Ribbon") },
];
static ATTRS_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
];
static ATTRS_VISIBLE_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_VISIBLE_TOGGLE_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":getPressed", property_name: Some("GetPressed"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_VERTICAL_SEPARATOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
];
static CHILDREN_DIALOG_BOX_LAUNCHER: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: Some("UnsizedButton") },
];
static ATTRS_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static CHILDREN_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlClone/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_LabelControl/mso:labelControl", property_name: None },
    ChildInfo { name: "mso:CT_Button/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButton/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_CheckBox/mso:checkBox", property_name: None },
    ChildInfo { name: "mso:CT_EditBox/mso:editBox", property_name: None },
    ChildInfo { name: "mso:CT_ComboBox/mso:comboBox", property_name: None },
    ChildInfo { name: "mso:CT_DropDownRegular/mso:dropDown", property_name: None },
    ChildInfo { name: "mso:CT_Gallery/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_Menu/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenu/mso:dynamicMenu", property_name: None },
    ChildInfo { name: "mso:CT_SplitButton/mso:splitButton", property_name: None },
    ChildInfo { name: "mso:CT_Box/mso:box", property_name: None },
    ChildInfo { name: "mso:CT_ButtonGroup/mso:buttonGroup", property_name: None },
    ChildInfo { name: "mso:CT_Separator/mso:separator", property_name: None },
    ChildInfo { name: "mso:CT_DialogLauncher/mso:dialogBoxLauncher", property_name: None },
];
static ATTRS_QUICK_ACCESS_TOOLBAR_CONTROL_CLONE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_SHARED_QAT_CONTROLS: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneQat/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_Separator/mso:separator", property_name: None },
];
static CHILDREN_DOCUMENT_SPECIFIC_QUICK_ACCESS_TOOLBAR_CONTROLS: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneQat/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_Separator/mso:separator", property_name: None },
];
static ATTRS_TAB: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("IdQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQ"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static CHILDREN_TAB: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Group/mso:group", property_name: None },
];
static ATTRS_CONTEXTUAL_TAB_SET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
];
static CHILDREN_CONTEXTUAL_TAB_SET: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Tab/mso:tab", property_name: None },
];
static ATTRS_REPURPOSED_COMMAND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
];
static CHILDREN_OFFICE_MENU: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_ControlCloneRegular/mso:control", property_name: None },
    ChildInfo { name: "mso:CT_ButtonRegular/mso:button", property_name: None },
    ChildInfo { name: "mso:CT_CheckBox/mso:checkBox", property_name: None },
    ChildInfo { name: "mso:CT_GalleryRegular/mso:gallery", property_name: None },
    ChildInfo { name: "mso:CT_ToggleButtonRegular/mso:toggleButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuSeparator/mso:menuSeparator", property_name: None },
    ChildInfo { name: "mso:CT_SplitButtonWithTitle/mso:splitButton", property_name: None },
    ChildInfo { name: "mso:CT_MenuWithTitle/mso:menu", property_name: None },
    ChildInfo { name: "mso:CT_DynamicMenuRegular/mso:dynamicMenu", property_name: None },
];
static CHILDREN_QUICK_ACCESS_TOOLBAR: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_QatItems/mso:sharedControls", property_name: Some("SharedQatControls") },
    ChildInfo { name: "mso:CT_QatItems/mso:documentControls", property_name: Some("DocumentSpecificQuickAccessToolbarControls") },
];
static CHILDREN_TABS: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Tab/mso:tab", property_name: None },
];
static CHILDREN_CONTEXTUAL_TAB_SETS: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_TabSet/mso:tabSet", property_name: None },
];
static CHILDREN_REPURPOSED_COMMANDS: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_Command/mso:command", property_name: None },
];
static ATTRS_RIBBON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":startFromScratch", property_name: Some("StartFromScratch"), type_name: "BooleanValue" },
];
static CHILDREN_RIBBON: &[ChildInfo] = &[
    ChildInfo { name: "mso:CT_OfficeMenu/mso:officeMenu", property_name: Some("OfficeMenu") },
    ChildInfo { name: "mso:CT_Qat/mso:qat", property_name: Some("QuickAccessToolbar") },
    ChildInfo { name: "mso:CT_Tabs/mso:tabs", property_name: Some("Tabs") },
    ChildInfo { name: "mso:CT_ContextualTabs/mso:contextualTabs", property_name: Some("ContextualTabSets") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "UnsizedControlClone", local_name: "control", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNSIZED_CONTROL_CLONE, children: &[] },
    ElementInfo { class_name: "UnsizedButton", local_name: "button", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNSIZED_BUTTON, children: &[] },
    ElementInfo { class_name: "CheckBox", local_name: "checkBox", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHECK_BOX, children: &[] },
    ElementInfo { class_name: "UnsizedGallery", local_name: "gallery", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_UNSIZED_GALLERY, children: CHILDREN_UNSIZED_GALLERY },
    ElementInfo { class_name: "UnsizedToggleButton", local_name: "toggleButton", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNSIZED_TOGGLE_BUTTON, children: &[] },
    ElementInfo { class_name: "MenuSeparator", local_name: "menuSeparator", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MENU_SEPARATOR, children: &[] },
    ElementInfo { class_name: "UnsizedSplitButton", local_name: "splitButton", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_UNSIZED_SPLIT_BUTTON, children: CHILDREN_UNSIZED_SPLIT_BUTTON },
    ElementInfo { class_name: "UnsizedMenu", local_name: "menu", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_UNSIZED_MENU, children: CHILDREN_UNSIZED_MENU },
    ElementInfo { class_name: "UnsizedDynamicMenu", local_name: "dynamicMenu", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UNSIZED_DYNAMIC_MENU, children: &[] },
    ElementInfo { class_name: "SplitButtonWithTitle", local_name: "splitButton", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPLIT_BUTTON_WITH_TITLE, children: CHILDREN_SPLIT_BUTTON_WITH_TITLE },
    ElementInfo { class_name: "MenuWithTitle", local_name: "menu", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MENU_WITH_TITLE, children: CHILDREN_MENU_WITH_TITLE },
    ElementInfo { class_name: "ControlClone", local_name: "control", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONTROL_CLONE, children: &[] },
    ElementInfo { class_name: "TextLabel", local_name: "labelControl", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TEXT_LABEL, children: &[] },
    ElementInfo { class_name: "Button", local_name: "button", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUTTON, children: &[] },
    ElementInfo { class_name: "ToggleButton", local_name: "toggleButton", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOGGLE_BUTTON, children: &[] },
    ElementInfo { class_name: "EditBox", local_name: "editBox", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EDIT_BOX, children: &[] },
    ElementInfo { class_name: "ComboBox", local_name: "comboBox", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMBO_BOX, children: CHILDREN_COMBO_BOX },
    ElementInfo { class_name: "DropDown", local_name: "dropDown", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DROP_DOWN, children: CHILDREN_DROP_DOWN },
    ElementInfo { class_name: "Gallery", local_name: "gallery", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GALLERY, children: CHILDREN_GALLERY },
    ElementInfo { class_name: "Menu", local_name: "menu", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MENU, children: CHILDREN_MENU },
    ElementInfo { class_name: "DynamicMenu", local_name: "dynamicMenu", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DYNAMIC_MENU, children: &[] },
    ElementInfo { class_name: "SplitButton", local_name: "splitButton", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPLIT_BUTTON, children: CHILDREN_SPLIT_BUTTON },
    ElementInfo { class_name: "Box", local_name: "box", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BOX_, children: CHILDREN_BOX_ },
    ElementInfo { class_name: "ButtonGroup", local_name: "buttonGroup", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BUTTON_GROUP, children: CHILDREN_BUTTON_GROUP },
    ElementInfo { class_name: "MenuRoot", local_name: "menu", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MENU_ROOT, children: CHILDREN_MENU_ROOT },
    ElementInfo { class_name: "CustomUI", local_name: "customUI", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CUSTOM_U_I, children: CHILDREN_CUSTOM_U_I },
    ElementInfo { class_name: "Item", local_name: "item", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ITEM, children: &[] },
    ElementInfo { class_name: "VisibleButton", local_name: "button", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VISIBLE_BUTTON, children: &[] },
    ElementInfo { class_name: "VisibleToggleButton", local_name: "toggleButton", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VISIBLE_TOGGLE_BUTTON, children: &[] },
    ElementInfo { class_name: "VerticalSeparator", local_name: "separator", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VERTICAL_SEPARATOR, children: &[] },
    ElementInfo { class_name: "DialogBoxLauncher", local_name: "dialogBoxLauncher", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIALOG_BOX_LAUNCHER },
    ElementInfo { class_name: "Group", local_name: "group", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP, children: CHILDREN_GROUP },
    ElementInfo { class_name: "QuickAccessToolbarControlClone", local_name: "control", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_QUICK_ACCESS_TOOLBAR_CONTROL_CLONE, children: &[] },
    ElementInfo { class_name: "SharedQatControls", local_name: "sharedControls", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHARED_QAT_CONTROLS },
    ElementInfo { class_name: "DocumentSpecificQuickAccessToolbarControls", local_name: "documentControls", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DOCUMENT_SPECIFIC_QUICK_ACCESS_TOOLBAR_CONTROLS },
    ElementInfo { class_name: "Tab", local_name: "tab", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TAB, children: CHILDREN_TAB },
    ElementInfo { class_name: "ContextualTabSet", local_name: "tabSet", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTEXTUAL_TAB_SET, children: CHILDREN_CONTEXTUAL_TAB_SET },
    ElementInfo { class_name: "RepurposedCommand", local_name: "command", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REPURPOSED_COMMAND, children: &[] },
    ElementInfo { class_name: "OfficeMenu", local_name: "officeMenu", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_MENU },
    ElementInfo { class_name: "QuickAccessToolbar", local_name: "qat", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_QUICK_ACCESS_TOOLBAR },
    ElementInfo { class_name: "Tabs", local_name: "tabs", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABS },
    ElementInfo { class_name: "ContextualTabSets", local_name: "contextualTabs", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONTEXTUAL_TAB_SETS },
    ElementInfo { class_name: "RepurposedCommands", local_name: "commands", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_REPURPOSED_COMMANDS },
    ElementInfo { class_name: "Ribbon", local_name: "ribbon", prefix: "mso", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RIBBON, children: CHILDREN_RIBBON },
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

/// Create a `<mso:control>` element (`UnsizedControlClone`).
pub fn unsized_control_clone() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "control")
}

/// Create a `<mso:button>` element (`UnsizedButton`).
pub fn unsized_button() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "button")
}

/// Create a `<mso:checkBox>` element (`CheckBox`).
pub fn check_box() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "checkBox")
}

/// Create a `<mso:gallery>` element (`UnsizedGallery`).
pub fn unsized_gallery(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "gallery").with_children(children)
}

/// Create a `<mso:toggleButton>` element (`UnsizedToggleButton`).
pub fn unsized_toggle_button() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "toggleButton")
}

/// Create a `<mso:menuSeparator>` element (`MenuSeparator`).
pub fn menu_separator() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "menuSeparator")
}

/// Create a `<mso:splitButton>` element (`UnsizedSplitButton`).
pub fn unsized_split_button(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "splitButton").with_children(children)
}

/// Create a `<mso:menu>` element (`UnsizedMenu`).
pub fn unsized_menu(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso:dynamicMenu>` element (`UnsizedDynamicMenu`).
pub fn unsized_dynamic_menu() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "dynamicMenu")
}

/// Create a `<mso:splitButton>` element (`SplitButtonWithTitle`).
pub fn split_button_with_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "splitButton").with_children(children)
}

/// Create a `<mso:menu>` element (`MenuWithTitle`).
pub fn menu_with_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso:control>` element (`ControlClone`).
pub fn control_clone() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "control")
}

/// Create a `<mso:labelControl>` element (`TextLabel`).
pub fn text_label() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "labelControl")
}

/// Create a `<mso:button>` element (`Button`).
pub fn button() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "button")
}

/// Create a `<mso:toggleButton>` element (`ToggleButton`).
pub fn toggle_button() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "toggleButton")
}

/// Create a `<mso:editBox>` element (`EditBox`).
pub fn edit_box() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "editBox")
}

/// Create a `<mso:comboBox>` element (`ComboBox`).
pub fn combo_box(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "comboBox").with_children(children)
}

/// Create a `<mso:dropDown>` element (`DropDown`).
pub fn drop_down(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "dropDown").with_children(children)
}

/// Create a `<mso:gallery>` element (`Gallery`).
pub fn gallery(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "gallery").with_children(children)
}

/// Create a `<mso:menu>` element (`Menu`).
pub fn menu(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso:dynamicMenu>` element (`DynamicMenu`).
pub fn dynamic_menu() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "dynamicMenu")
}

/// Create a `<mso:splitButton>` element (`SplitButton`).
pub fn split_button(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "splitButton").with_children(children)
}

/// Create a `<mso:box>` element (`Box`).
pub fn box_(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "box").with_children(children)
}

/// Create a `<mso:buttonGroup>` element (`ButtonGroup`).
pub fn button_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "buttonGroup").with_children(children)
}

/// Create a `<mso:menu>` element (`MenuRoot`).
pub fn menu_root(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso:customUI>` element (`CustomUI`).
pub fn custom_u_i(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "customUI").with_children(children)
}

/// Create a `<mso:item>` element (`Item`).
pub fn item() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "item")
}

/// Create a `<mso:button>` element (`VisibleButton`).
pub fn visible_button() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "button")
}

/// Create a `<mso:toggleButton>` element (`VisibleToggleButton`).
pub fn visible_toggle_button() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "toggleButton")
}

/// Create a `<mso:separator>` element (`VerticalSeparator`).
pub fn vertical_separator() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "separator")
}

/// Create a `<mso:dialogBoxLauncher>` element (`DialogBoxLauncher`).
pub fn dialog_box_launcher(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "dialogBoxLauncher").with_children(children)
}

/// Create a `<mso:group>` element (`Group`).
pub fn group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "group").with_children(children)
}

/// Create a `<mso:control>` element (`QuickAccessToolbarControlClone`).
pub fn quick_access_toolbar_control_clone() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "control")
}

/// Create a `<mso:sharedControls>` element (`SharedQatControls`).
pub fn shared_qat_controls(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "sharedControls").with_children(children)
}

/// Create a `<mso:documentControls>` element (`DocumentSpecificQuickAccessToolbarControls`).
pub fn document_specific_quick_access_toolbar_controls(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "documentControls").with_children(children)
}

/// Create a `<mso:tab>` element (`Tab`).
pub fn tab(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "tab").with_children(children)
}

/// Create a `<mso:tabSet>` element (`ContextualTabSet`).
pub fn contextual_tab_set(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "tabSet").with_children(children)
}

/// Create a `<mso:command>` element (`RepurposedCommand`).
pub fn repurposed_command() -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "command")
}

/// Create a `<mso:officeMenu>` element (`OfficeMenu`).
pub fn office_menu(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "officeMenu").with_children(children)
}

/// Create a `<mso:qat>` element (`QuickAccessToolbar`).
pub fn quick_access_toolbar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "qat").with_children(children)
}

/// Create a `<mso:tabs>` element (`Tabs`).
pub fn tabs(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "tabs").with_children(children)
}

/// Create a `<mso:contextualTabs>` element (`ContextualTabSets`).
pub fn contextual_tab_sets(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "contextualTabs").with_children(children)
}

/// Create a `<mso:commands>` element (`RepurposedCommands`).
pub fn repurposed_commands(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "commands").with_children(children)
}

/// Create a `<mso:ribbon>` element (`Ribbon`).
pub fn ribbon(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso", NAMESPACE_URI, "ribbon").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 45;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 44;
