//! Auto-generated from `schemas_microsoft_com_office_2009_07_customui.json`.
//! Target namespace: `http://schemas.microsoft.com/office/2009/07/customui` (prefix `mso14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/2009/07/customui";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "mso14";

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

static ATTRS_CONTROL_CLONE_REGULAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_BUTTON_REGULAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static ATTRS_GALLERY_REGULAR: &[AttributeInfo] = &[
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
    AttributeInfo { qname: ":showInRibbon", property_name: Some("ShowInRibbon"), type_name: "EnumValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_GALLERY_REGULAR: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Item/mso14:item", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
];
static ATTRS_TOGGLE_BUTTON_REGULAR: &[AttributeInfo] = &[
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: ":getTitle", property_name: Some("GetTitle"), type_name: "StringValue" },
];
static ATTRS_SPLIT_BUTTON_REGULAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static CHILDREN_SPLIT_BUTTON_REGULAR: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_VisibleButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_VisibleToggleButton/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuRegular/mso14:menu", property_name: None },
];
static ATTRS_MENU_REGULAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_MENU_REGULAR: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlCloneRegular/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_CheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_GalleryRegular/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButtonRegular/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuSeparator/mso14:menuSeparator", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButtonRegular/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuRegular/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu", property_name: None },
];
static ATTRS_DYNAMIC_MENU_REGULAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static CHILDREN_SPLIT_BUTTON_WITH_TITLE: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_VisibleButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_VisibleToggleButton/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuWithTitle/mso14:menu", property_name: None },
];
static ATTRS_MENU_WITH_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    ChildInfo { name: "mso14:CT_ControlCloneRegular/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_CheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_GalleryRegular/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButtonRegular/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuSeparator/mso14:menuSeparator", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButtonWithTitle/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuWithTitle/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu", property_name: None },
];
static ATTRS_MENU_SEPARATOR_NO_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_LABEL_CONTROL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    ChildInfo { name: "mso14:CT_Item/mso14:item", property_name: None },
];
static ATTRS_DROP_DOWN_REGULAR: &[AttributeInfo] = &[
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_DROP_DOWN_REGULAR: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Item/mso14:item", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
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
    AttributeInfo { qname: ":showInRibbon", property_name: Some("ShowInRibbon"), type_name: "EnumValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    ChildInfo { name: "mso14:CT_Item/mso14:item", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
];
static ATTRS_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    ChildInfo { name: "mso14:CT_ControlCloneRegular/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_CheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_GalleryRegular/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButtonRegular/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuSeparator/mso14:menuSeparator", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButtonRegular/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuRegular/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu", property_name: None },
];
static ATTRS_DYNAMIC_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getSize", property_name: Some("GetSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static CHILDREN_SPLIT_BUTTON: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_VisibleButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_VisibleToggleButton/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuRegular/mso14:menu", property_name: None },
];
static ATTRS_BOX_: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":boxStyle", property_name: Some("BoxStyle"), type_name: "EnumValue" },
];
static CHILDREN_BOX_: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlClone/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_LabelControl/mso14:labelControl", property_name: None },
    ChildInfo { name: "mso14:CT_Button/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButton/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_CheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_EditBox/mso14:editBox", property_name: None },
    ChildInfo { name: "mso14:CT_ComboBox/mso14:comboBox", property_name: None },
    ChildInfo { name: "mso14:CT_DropDownRegular/mso14:dropDown", property_name: None },
    ChildInfo { name: "mso14:CT_Gallery/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_Menu/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenu/mso14:dynamicMenu", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButton/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_Box/mso14:box", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonGroup/mso14:buttonGroup", property_name: None },
];
static ATTRS_BUTTON_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
];
static CHILDREN_BUTTON_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlCloneRegular/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButtonRegular/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_GalleryRegular/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_MenuRegular/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButtonRegular/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_Separator/mso14:separator", property_name: None },
];
static ATTRS_BACKSTAGE_MENU_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":isDefinitive", property_name: Some("IsDefinitive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_MENU_CHECK_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":getPressed", property_name: Some("GetPressed"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_SUB_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static CHILDREN_BACKSTAGE_SUB_MENU: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageMenuGroup/mso14:menuGroup", property_name: None },
];
static ATTRS_BACKSTAGE_MENU_TOGGLE_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":getPressed", property_name: Some("GetPressed"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_GROUP_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":style", property_name: Some("Style"), type_name: "EnumValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":isDefinitive", property_name: Some("IsDefinitive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_CHECK_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":getPressed", property_name: Some("GetPressed"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_EDIT_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":alignLabel", property_name: Some("AlignLabel"), type_name: "EnumValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getText", property_name: Some("GetText"), type_name: "StringValue" },
    AttributeInfo { qname: ":onChange", property_name: Some("OnChange"), type_name: "StringValue" },
    AttributeInfo { qname: ":maxLength", property_name: Some("MaxLength"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_DROP_DOWN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":alignLabel", property_name: Some("AlignLabel"), type_name: "EnumValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemIndex", property_name: Some("GetSelectedItemIndex"), type_name: "StringValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemCount", property_name: Some("GetItemCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemLabel", property_name: Some("GetItemLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemID", property_name: Some("GetItemID"), type_name: "StringValue" },
];
static CHILDREN_BACKSTAGE_DROP_DOWN: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageItem/mso14:item", property_name: None },
];
static ATTRS_RADIO_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":alignLabel", property_name: Some("AlignLabel"), type_name: "EnumValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSelectedItemIndex", property_name: Some("GetSelectedItemIndex"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemCount", property_name: Some("GetItemCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemLabel", property_name: Some("GetItemLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemID", property_name: Some("GetItemID"), type_name: "StringValue" },
];
static CHILDREN_RADIO_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageItem/mso14:radioButton", property_name: None },
];
static ATTRS_BACKSTAGE_COMBO_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":alignLabel", property_name: Some("AlignLabel"), type_name: "EnumValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getText", property_name: Some("GetText"), type_name: "StringValue" },
    AttributeInfo { qname: ":onChange", property_name: Some("OnChange"), type_name: "StringValue" },
    AttributeInfo { qname: ":sizeString", property_name: Some("SizeString"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemCount", property_name: Some("GetItemCount"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemLabel", property_name: Some("GetItemLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":getItemID", property_name: Some("GetItemID"), type_name: "StringValue" },
];
static CHILDREN_BACKSTAGE_COMBO_BOX: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageItem/mso14:item", property_name: None },
];
static ATTRS_HYPERLINK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":alignLabel", property_name: Some("AlignLabel"), type_name: "EnumValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":target", property_name: Some("Target"), type_name: "StringValue" },
    AttributeInfo { qname: ":getTarget", property_name: Some("GetTarget"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_LABEL_CONTROL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":alignLabel", property_name: Some("AlignLabel"), type_name: "EnumValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":noWrap", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_GROUP_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
];
static CHILDREN_GROUP_BOX: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageGroupButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageCheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageEditBox/mso14:editBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageDropDown/mso14:dropDown", property_name: None },
    ChildInfo { name: "mso14:CT_RadioGroup/mso14:radioGroup", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageComboBox/mso14:comboBox", property_name: None },
    ChildInfo { name: "mso14:CT_Hyperlink/mso14:hyperlink", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageLabelControl/mso14:labelControl", property_name: None },
    ChildInfo { name: "mso14:CT_GroupBox/mso14:groupBox", property_name: None },
    ChildInfo { name: "mso14:CT_LayoutContainer/mso14:layoutContainer", property_name: None },
    ChildInfo { name: "mso14:CT_ImageControl/mso14:imageControl", property_name: None },
];
static ATTRS_LAYOUT_CONTAINER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":align", property_name: Some("Align"), type_name: "EnumValue" },
    AttributeInfo { qname: ":expand", property_name: Some("Expand"), type_name: "EnumValue" },
    AttributeInfo { qname: ":layoutChildren", property_name: Some("LayoutChildren"), type_name: "EnumValue" },
];
static CHILDREN_LAYOUT_CONTAINER: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageGroupButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageCheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageEditBox/mso14:editBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageDropDown/mso14:dropDown", property_name: None },
    ChildInfo { name: "mso14:CT_RadioGroup/mso14:radioGroup", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageComboBox/mso14:comboBox", property_name: None },
    ChildInfo { name: "mso14:CT_Hyperlink/mso14:hyperlink", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageLabelControl/mso14:labelControl", property_name: None },
    ChildInfo { name: "mso14:CT_GroupBox/mso14:groupBox", property_name: None },
    ChildInfo { name: "mso14:CT_LayoutContainer/mso14:layoutContainer", property_name: None },
    ChildInfo { name: "mso14:CT_ImageControl/mso14:imageControl", property_name: None },
];
static ATTRS_IMAGE_CONTROL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":altText", property_name: Some("AltText"), type_name: "StringValue" },
    AttributeInfo { qname: ":getAltText", property_name: Some("GetAltText"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":style", property_name: Some("Style"), type_name: "EnumValue" },
    AttributeInfo { qname: ":getStyle", property_name: Some("GetStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":helperText", property_name: Some("HelperText"), type_name: "StringValue" },
    AttributeInfo { qname: ":getHelperText", property_name: Some("GetHelperText"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
];
static CHILDREN_BACKSTAGE_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_PrimaryItem/mso14:primaryItem", property_name: None },
    ChildInfo { name: "mso14:CT_GroupControls/mso14:topItems", property_name: None },
    ChildInfo { name: "mso14:CT_GroupControls/mso14:bottomItems", property_name: None },
];
static ATTRS_TASK_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":helperText", property_name: Some("HelperText"), type_name: "StringValue" },
    AttributeInfo { qname: ":getHelperText", property_name: Some("GetHelperText"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":allowedTaskSizes", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_TASK_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_TaskGroupCategory/mso14:category", property_name: None },
];
static ATTRS_MENU_ROOT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: ":getTitle", property_name: Some("GetTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
];
static CHILDREN_MENU_ROOT: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlCloneRegular/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_CheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_GalleryRegular/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButtonRegular/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuSeparator/mso14:menuSeparator", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButtonRegular/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuRegular/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu", property_name: None },
];
static ATTRS_CUSTOM_U_I: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onLoad", property_name: Some("OnLoad"), type_name: "StringValue" },
    AttributeInfo { qname: ":loadImage", property_name: Some("LoadImage"), type_name: "StringValue" },
];
static CHILDREN_CUSTOM_U_I: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Commands/mso14:commands", property_name: Some("Commands") },
    ChildInfo { name: "mso14:CT_Ribbon/mso14:ribbon", property_name: Some("Ribbon") },
    ChildInfo { name: "mso14:CT_Backstage/mso14:backstage", property_name: Some("Backstage") },
    ChildInfo { name: "mso14:CT_ContextMenus/mso14:contextMenus", property_name: Some("ContextMenus") },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
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
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static ATTRS_SEPARATOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
];
static CHILDREN_DIALOG_BOX_LAUNCHER: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: Some("ButtonRegular") },
];
static ATTRS_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":autoScale", property_name: Some("AutoScale"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":centerVertically", property_name: Some("CenterVertically"), type_name: "BooleanValue" },
];
static CHILDREN_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlClone/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_LabelControl/mso14:labelControl", property_name: None },
    ChildInfo { name: "mso14:CT_Button/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButton/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_CheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_EditBox/mso14:editBox", property_name: None },
    ChildInfo { name: "mso14:CT_ComboBox/mso14:comboBox", property_name: None },
    ChildInfo { name: "mso14:CT_DropDownRegular/mso14:dropDown", property_name: None },
    ChildInfo { name: "mso14:CT_Gallery/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_Menu/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenu/mso14:dynamicMenu", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButton/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_Box/mso14:box", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonGroup/mso14:buttonGroup", property_name: None },
    ChildInfo { name: "mso14:CT_Separator/mso14:separator", property_name: None },
    ChildInfo { name: "mso14:CT_DialogLauncher/mso14:dialogBoxLauncher", property_name: None },
];
static ATTRS_CONTROL_CLONE_QAT: &[AttributeInfo] = &[
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
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":showImage", property_name: Some("ShowImage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowImage", property_name: Some("GetShowImage"), type_name: "StringValue" },
];
static CHILDREN_SHARED_CONTROLS_QAT_ITEMS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlCloneQat/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_Separator/mso14:separator", property_name: None },
];
static CHILDREN_DOCUMENT_CONTROLS_QAT_ITEMS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlCloneQat/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_Separator/mso14:separator", property_name: None },
];
static ATTRS_TAB: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static CHILDREN_TAB: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Group/mso14:group", property_name: None },
];
static ATTRS_TAB_SET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
];
static CHILDREN_TAB_SET: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Tab/mso14:tab", property_name: None },
];
static ATTRS_COMMAND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
];
static CHILDREN_QUICK_ACCESS_TOOLBAR: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_QatItems/mso14:sharedControls", property_name: Some("SharedControlsQatItems") },
    ChildInfo { name: "mso14:CT_QatItems/mso14:documentControls", property_name: Some("DocumentControlsQatItems") },
];
static CHILDREN_TABS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Tab/mso14:tab", property_name: None },
];
static CHILDREN_CONTEXTUAL_TABS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_TabSet/mso14:tabSet", property_name: None },
];
static ATTRS_CONTEXT_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
];
static CHILDREN_CONTEXT_MENU: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ControlCloneRegular/mso14:control", property_name: None },
    ChildInfo { name: "mso14:CT_ButtonRegular/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_CheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_GalleryRegular/mso14:gallery", property_name: None },
    ChildInfo { name: "mso14:CT_ToggleButtonRegular/mso14:toggleButton", property_name: None },
    ChildInfo { name: "mso14:CT_SplitButtonRegular/mso14:splitButton", property_name: None },
    ChildInfo { name: "mso14:CT_MenuRegular/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_DynamicMenuRegular/mso14:dynamicMenu", property_name: None },
    ChildInfo { name: "mso14:CT_MenuSeparatorNoTitle/mso14:menuSeparator", property_name: None },
];
static ATTRS_ITEM_BACKSTAGE_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
];
static ATTRS_RADIO_BUTTON_BACKSTAGE_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_REGULAR_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":isDefinitive", property_name: Some("IsDefinitive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
];
static ATTRS_BACKSTAGE_PRIMARY_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: ":screentip", property_name: Some("Screentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getScreentip", property_name: Some("GetScreentip"), type_name: "StringValue" },
    AttributeInfo { qname: ":supertip", property_name: Some("Supertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getSupertip", property_name: Some("GetSupertip"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static CHILDREN_BACKSTAGE_PRIMARY_MENU: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageMenuGroup/mso14:menuGroup", property_name: None },
];
static ATTRS_BACKSTAGE_MENU_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":itemSize", property_name: Some("ItemSize"), type_name: "EnumValue" },
];
static CHILDREN_BACKSTAGE_MENU_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageMenuButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageMenuCheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageSubMenu/mso14:menu", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageMenuToggleButton/mso14:toggleButton", property_name: None },
];
static CHILDREN_PRIMARY_ITEM: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageRegularButton/mso14:button", property_name: Some("BackstageRegularButton") },
    ChildInfo { name: "mso14:CT_BackstagePrimaryMenu/mso14:menu", property_name: Some("BackstagePrimaryMenu") },
];
static CHILDREN_TOP_ITEMS_GROUP_CONTROLS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageGroupButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageCheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageEditBox/mso14:editBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageDropDown/mso14:dropDown", property_name: None },
    ChildInfo { name: "mso14:CT_RadioGroup/mso14:radioGroup", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageComboBox/mso14:comboBox", property_name: None },
    ChildInfo { name: "mso14:CT_Hyperlink/mso14:hyperlink", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageLabelControl/mso14:labelControl", property_name: None },
    ChildInfo { name: "mso14:CT_GroupBox/mso14:groupBox", property_name: None },
    ChildInfo { name: "mso14:CT_LayoutContainer/mso14:layoutContainer", property_name: None },
    ChildInfo { name: "mso14:CT_ImageControl/mso14:imageControl", property_name: None },
];
static CHILDREN_BOTTOM_ITEMS_GROUP_CONTROLS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageGroupButton/mso14:button", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageCheckBox/mso14:checkBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageEditBox/mso14:editBox", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageDropDown/mso14:dropDown", property_name: None },
    ChildInfo { name: "mso14:CT_RadioGroup/mso14:radioGroup", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageComboBox/mso14:comboBox", property_name: None },
    ChildInfo { name: "mso14:CT_Hyperlink/mso14:hyperlink", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageLabelControl/mso14:labelControl", property_name: None },
    ChildInfo { name: "mso14:CT_GroupBox/mso14:groupBox", property_name: None },
    ChildInfo { name: "mso14:CT_LayoutContainer/mso14:layoutContainer", property_name: None },
    ChildInfo { name: "mso14:CT_ImageControl/mso14:imageControl", property_name: None },
];
static ATTRS_TASK_GROUP_CATEGORY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
];
static CHILDREN_TASK_GROUP_CATEGORY: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_TaskGroupTask/mso14:task", property_name: None },
];
static ATTRS_TASK_GROUP_TASK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":isDefinitive", property_name: Some("IsDefinitive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static ATTRS_TASK_FORM_GROUP_CATEGORY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
];
static CHILDREN_TASK_FORM_GROUP_CATEGORY: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_TaskFormGroupTask/mso14:task", property_name: None },
];
static ATTRS_TASK_FORM_GROUP_TASK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":getDescription", property_name: Some("GetDescription"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
];
static CHILDREN_TASK_FORM_GROUP_TASK: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageGroup/mso14:group", property_name: None },
];
static ATTRS_TASK_FORM_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":helperText", property_name: Some("HelperText"), type_name: "StringValue" },
    AttributeInfo { qname: ":getHelperText", property_name: Some("GetHelperText"), type_name: "StringValue" },
    AttributeInfo { qname: ":showLabel", property_name: Some("ShowLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getShowLabel", property_name: Some("GetShowLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":allowedTaskSizes", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_TASK_FORM_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_TaskFormGroupCategory/mso14:category", property_name: None },
];
static CHILDREN_BACKSTAGE_GROUPS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_TaskFormGroup/mso14:taskFormGroup", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageGroup/mso14:group", property_name: None },
    ChildInfo { name: "mso14:CT_TaskGroup/mso14:taskGroup", property_name: None },
];
static CHILDREN_SIMPLE_GROUPS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageGroup/mso14:group", property_name: None },
    ChildInfo { name: "mso14:CT_TaskGroup/mso14:taskGroup", property_name: None },
];
static ATTRS_BACKSTAGE_TAB: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: ":getTitle", property_name: Some("GetTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":columnWidthPercent", property_name: Some("ColumnWidthPercent"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":firstColumnMinWidth", property_name: Some("FirstColumnMinWidth"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":firstColumnMaxWidth", property_name: Some("FirstColumnMaxWidth"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":secondColumnMinWidth", property_name: Some("SecondColumnMinWidth"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":secondColumnMaxWidth", property_name: Some("SecondColumnMaxWidth"), type_name: "IntegerValue" },
];
static CHILDREN_BACKSTAGE_TAB: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageGroups/mso14:firstColumn", property_name: Some("BackstageGroups") },
    ChildInfo { name: "mso14:CT_SimpleGroups/mso14:secondColumn", property_name: Some("SimpleGroups") },
];
static ATTRS_BACKSTAGE_FAST_COMMAND_BUTTON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idMso", property_name: Some("IdMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterMso", property_name: Some("InsertAfterMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeMso", property_name: Some("InsertBeforeMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertAfterQ", property_name: Some("InsertAfterQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":insertBeforeQ", property_name: Some("InsertBeforeQulifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":idQ", property_name: Some("QualifiedId"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":onAction", property_name: Some("OnAction"), type_name: "StringValue" },
    AttributeInfo { qname: ":isDefinitive", property_name: Some("IsDefinitive"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enabled", property_name: Some("Enabled"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getEnabled", property_name: Some("GetEnabled"), type_name: "StringValue" },
    AttributeInfo { qname: ":label", property_name: Some("Label"), type_name: "StringValue" },
    AttributeInfo { qname: ":getLabel", property_name: Some("GetLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":getVisible", property_name: Some("GetVisible"), type_name: "StringValue" },
    AttributeInfo { qname: ":keytip", property_name: Some("Keytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":getKeytip", property_name: Some("GetKeytip"), type_name: "StringValue" },
    AttributeInfo { qname: ":image", property_name: Some("Image"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageMso", property_name: Some("ImageMso"), type_name: "StringValue" },
    AttributeInfo { qname: ":getImage", property_name: Some("GetImage"), type_name: "StringValue" },
];
static CHILDREN_COMMANDS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Command/mso14:command", property_name: None },
];
static ATTRS_RIBBON: &[AttributeInfo] = &[
    AttributeInfo { qname: ":startFromScratch", property_name: Some("StartFromScratch"), type_name: "BooleanValue" },
];
static CHILDREN_RIBBON: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_Qat/mso14:qat", property_name: Some("QuickAccessToolbar") },
    ChildInfo { name: "mso14:CT_Tabs/mso14:tabs", property_name: Some("Tabs") },
    ChildInfo { name: "mso14:CT_ContextualTabs/mso14:contextualTabs", property_name: Some("ContextualTabs") },
];
static ATTRS_BACKSTAGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":onShow", property_name: Some("OnShow"), type_name: "StringValue" },
    AttributeInfo { qname: ":onHide", property_name: Some("OnHide"), type_name: "StringValue" },
];
static CHILDREN_BACKSTAGE: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_BackstageTab/mso14:tab", property_name: None },
    ChildInfo { name: "mso14:CT_BackstageFastCommandButton/mso14:button", property_name: None },
];
static CHILDREN_CONTEXT_MENUS: &[ChildInfo] = &[
    ChildInfo { name: "mso14:CT_ContextMenu/mso14:contextMenu", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ControlCloneRegular", local_name: "control", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONTROL_CLONE_REGULAR, children: &[] },
    ElementInfo { class_name: "ButtonRegular", local_name: "button", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUTTON_REGULAR, children: &[] },
    ElementInfo { class_name: "CheckBox", local_name: "checkBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHECK_BOX, children: &[] },
    ElementInfo { class_name: "GalleryRegular", local_name: "gallery", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GALLERY_REGULAR, children: CHILDREN_GALLERY_REGULAR },
    ElementInfo { class_name: "ToggleButtonRegular", local_name: "toggleButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOGGLE_BUTTON_REGULAR, children: &[] },
    ElementInfo { class_name: "MenuSeparator", local_name: "menuSeparator", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MENU_SEPARATOR, children: &[] },
    ElementInfo { class_name: "SplitButtonRegular", local_name: "splitButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPLIT_BUTTON_REGULAR, children: CHILDREN_SPLIT_BUTTON_REGULAR },
    ElementInfo { class_name: "MenuRegular", local_name: "menu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MENU_REGULAR, children: CHILDREN_MENU_REGULAR },
    ElementInfo { class_name: "DynamicMenuRegular", local_name: "dynamicMenu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DYNAMIC_MENU_REGULAR, children: &[] },
    ElementInfo { class_name: "SplitButtonWithTitle", local_name: "splitButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPLIT_BUTTON_WITH_TITLE, children: CHILDREN_SPLIT_BUTTON_WITH_TITLE },
    ElementInfo { class_name: "MenuWithTitle", local_name: "menu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MENU_WITH_TITLE, children: CHILDREN_MENU_WITH_TITLE },
    ElementInfo { class_name: "MenuSeparatorNoTitle", local_name: "menuSeparator", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MENU_SEPARATOR_NO_TITLE, children: &[] },
    ElementInfo { class_name: "ControlClone", local_name: "control", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONTROL_CLONE, children: &[] },
    ElementInfo { class_name: "LabelControl", local_name: "labelControl", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LABEL_CONTROL, children: &[] },
    ElementInfo { class_name: "Button", local_name: "button", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUTTON, children: &[] },
    ElementInfo { class_name: "ToggleButton", local_name: "toggleButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOGGLE_BUTTON, children: &[] },
    ElementInfo { class_name: "EditBox", local_name: "editBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EDIT_BOX, children: &[] },
    ElementInfo { class_name: "ComboBox", local_name: "comboBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COMBO_BOX, children: CHILDREN_COMBO_BOX },
    ElementInfo { class_name: "DropDownRegular", local_name: "dropDown", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DROP_DOWN_REGULAR, children: CHILDREN_DROP_DOWN_REGULAR },
    ElementInfo { class_name: "Gallery", local_name: "gallery", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GALLERY, children: CHILDREN_GALLERY },
    ElementInfo { class_name: "Menu", local_name: "menu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MENU, children: CHILDREN_MENU },
    ElementInfo { class_name: "DynamicMenu", local_name: "dynamicMenu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DYNAMIC_MENU, children: &[] },
    ElementInfo { class_name: "SplitButton", local_name: "splitButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPLIT_BUTTON, children: CHILDREN_SPLIT_BUTTON },
    ElementInfo { class_name: "Box", local_name: "box", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BOX_, children: CHILDREN_BOX_ },
    ElementInfo { class_name: "ButtonGroup", local_name: "buttonGroup", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BUTTON_GROUP, children: CHILDREN_BUTTON_GROUP },
    ElementInfo { class_name: "BackstageMenuButton", local_name: "button", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_MENU_BUTTON, children: &[] },
    ElementInfo { class_name: "BackstageMenuCheckBox", local_name: "checkBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_MENU_CHECK_BOX, children: &[] },
    ElementInfo { class_name: "BackstageSubMenu", local_name: "menu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_SUB_MENU, children: CHILDREN_BACKSTAGE_SUB_MENU },
    ElementInfo { class_name: "BackstageMenuToggleButton", local_name: "toggleButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_MENU_TOGGLE_BUTTON, children: &[] },
    ElementInfo { class_name: "BackstageGroupButton", local_name: "button", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_GROUP_BUTTON, children: &[] },
    ElementInfo { class_name: "BackstageCheckBox", local_name: "checkBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_CHECK_BOX, children: &[] },
    ElementInfo { class_name: "BackstageEditBox", local_name: "editBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_EDIT_BOX, children: &[] },
    ElementInfo { class_name: "BackstageDropDown", local_name: "dropDown", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_DROP_DOWN, children: CHILDREN_BACKSTAGE_DROP_DOWN },
    ElementInfo { class_name: "RadioGroup", local_name: "radioGroup", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RADIO_GROUP, children: CHILDREN_RADIO_GROUP },
    ElementInfo { class_name: "BackstageComboBox", local_name: "comboBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_COMBO_BOX, children: CHILDREN_BACKSTAGE_COMBO_BOX },
    ElementInfo { class_name: "Hyperlink", local_name: "hyperlink", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HYPERLINK, children: &[] },
    ElementInfo { class_name: "BackstageLabelControl", local_name: "labelControl", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_LABEL_CONTROL, children: &[] },
    ElementInfo { class_name: "GroupBox", local_name: "groupBox", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_BOX, children: CHILDREN_GROUP_BOX },
    ElementInfo { class_name: "LayoutContainer", local_name: "layoutContainer", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LAYOUT_CONTAINER, children: CHILDREN_LAYOUT_CONTAINER },
    ElementInfo { class_name: "ImageControl", local_name: "imageControl", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_IMAGE_CONTROL, children: &[] },
    ElementInfo { class_name: "BackstageGroup", local_name: "group", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_GROUP, children: CHILDREN_BACKSTAGE_GROUP },
    ElementInfo { class_name: "TaskGroup", local_name: "taskGroup", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_GROUP, children: CHILDREN_TASK_GROUP },
    ElementInfo { class_name: "MenuRoot", local_name: "menu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MENU_ROOT, children: CHILDREN_MENU_ROOT },
    ElementInfo { class_name: "CustomUI", local_name: "customUI", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CUSTOM_U_I, children: CHILDREN_CUSTOM_U_I },
    ElementInfo { class_name: "Item", local_name: "item", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ITEM, children: &[] },
    ElementInfo { class_name: "VisibleButton", local_name: "button", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VISIBLE_BUTTON, children: &[] },
    ElementInfo { class_name: "VisibleToggleButton", local_name: "toggleButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_VISIBLE_TOGGLE_BUTTON, children: &[] },
    ElementInfo { class_name: "Separator", local_name: "separator", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SEPARATOR, children: &[] },
    ElementInfo { class_name: "DialogBoxLauncher", local_name: "dialogBoxLauncher", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIALOG_BOX_LAUNCHER },
    ElementInfo { class_name: "Group", local_name: "group", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP, children: CHILDREN_GROUP },
    ElementInfo { class_name: "ControlCloneQat", local_name: "control", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONTROL_CLONE_QAT, children: &[] },
    ElementInfo { class_name: "SharedControlsQatItems", local_name: "sharedControls", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHARED_CONTROLS_QAT_ITEMS },
    ElementInfo { class_name: "DocumentControlsQatItems", local_name: "documentControls", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DOCUMENT_CONTROLS_QAT_ITEMS },
    ElementInfo { class_name: "Tab", local_name: "tab", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TAB, children: CHILDREN_TAB },
    ElementInfo { class_name: "TabSet", local_name: "tabSet", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TAB_SET, children: CHILDREN_TAB_SET },
    ElementInfo { class_name: "Command", local_name: "command", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMMAND, children: &[] },
    ElementInfo { class_name: "QuickAccessToolbar", local_name: "qat", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_QUICK_ACCESS_TOOLBAR },
    ElementInfo { class_name: "Tabs", local_name: "tabs", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABS },
    ElementInfo { class_name: "ContextualTabs", local_name: "contextualTabs", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONTEXTUAL_TABS },
    ElementInfo { class_name: "ContextMenu", local_name: "contextMenu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTEXT_MENU, children: CHILDREN_CONTEXT_MENU },
    ElementInfo { class_name: "ItemBackstageItem", local_name: "item", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ITEM_BACKSTAGE_ITEM, children: &[] },
    ElementInfo { class_name: "RadioButtonBackstageItem", local_name: "radioButton", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RADIO_BUTTON_BACKSTAGE_ITEM, children: &[] },
    ElementInfo { class_name: "BackstageRegularButton", local_name: "button", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_REGULAR_BUTTON, children: &[] },
    ElementInfo { class_name: "BackstagePrimaryMenu", local_name: "menu", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_PRIMARY_MENU, children: CHILDREN_BACKSTAGE_PRIMARY_MENU },
    ElementInfo { class_name: "BackstageMenuGroup", local_name: "menuGroup", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_MENU_GROUP, children: CHILDREN_BACKSTAGE_MENU_GROUP },
    ElementInfo { class_name: "PrimaryItem", local_name: "primaryItem", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRIMARY_ITEM },
    ElementInfo { class_name: "TopItemsGroupControls", local_name: "topItems", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TOP_ITEMS_GROUP_CONTROLS },
    ElementInfo { class_name: "BottomItemsGroupControls", local_name: "bottomItems", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BOTTOM_ITEMS_GROUP_CONTROLS },
    ElementInfo { class_name: "TaskGroupCategory", local_name: "category", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_GROUP_CATEGORY, children: CHILDREN_TASK_GROUP_CATEGORY },
    ElementInfo { class_name: "TaskGroupTask", local_name: "task", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TASK_GROUP_TASK, children: &[] },
    ElementInfo { class_name: "TaskFormGroupCategory", local_name: "category", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_FORM_GROUP_CATEGORY, children: CHILDREN_TASK_FORM_GROUP_CATEGORY },
    ElementInfo { class_name: "TaskFormGroupTask", local_name: "task", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_FORM_GROUP_TASK, children: CHILDREN_TASK_FORM_GROUP_TASK },
    ElementInfo { class_name: "TaskFormGroup", local_name: "taskFormGroup", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TASK_FORM_GROUP, children: CHILDREN_TASK_FORM_GROUP },
    ElementInfo { class_name: "BackstageGroups", local_name: "firstColumn", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKSTAGE_GROUPS },
    ElementInfo { class_name: "SimpleGroups", local_name: "secondColumn", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SIMPLE_GROUPS },
    ElementInfo { class_name: "BackstageTab", local_name: "tab", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_TAB, children: CHILDREN_BACKSTAGE_TAB },
    ElementInfo { class_name: "BackstageFastCommandButton", local_name: "button", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKSTAGE_FAST_COMMAND_BUTTON, children: &[] },
    ElementInfo { class_name: "Commands", local_name: "commands", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMANDS },
    ElementInfo { class_name: "Ribbon", local_name: "ribbon", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RIBBON, children: CHILDREN_RIBBON },
    ElementInfo { class_name: "Backstage", local_name: "backstage", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKSTAGE, children: CHILDREN_BACKSTAGE },
    ElementInfo { class_name: "ContextMenus", local_name: "contextMenus", prefix: "mso14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONTEXT_MENUS },
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

/// Create a `<mso14:control>` element (`ControlCloneRegular`).
pub fn control_clone_regular() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "control")
}

/// Create a `<mso14:button>` element (`ButtonRegular`).
pub fn button_regular() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "button")
}

/// Create a `<mso14:checkBox>` element (`CheckBox`).
pub fn check_box() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "checkBox")
}

/// Create a `<mso14:gallery>` element (`GalleryRegular`).
pub fn gallery_regular(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "gallery").with_children(children)
}

/// Create a `<mso14:toggleButton>` element (`ToggleButtonRegular`).
pub fn toggle_button_regular() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "toggleButton")
}

/// Create a `<mso14:menuSeparator>` element (`MenuSeparator`).
pub fn menu_separator() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menuSeparator")
}

/// Create a `<mso14:splitButton>` element (`SplitButtonRegular`).
pub fn split_button_regular(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "splitButton").with_children(children)
}

/// Create a `<mso14:menu>` element (`MenuRegular`).
pub fn menu_regular(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso14:dynamicMenu>` element (`DynamicMenuRegular`).
pub fn dynamic_menu_regular() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "dynamicMenu")
}

/// Create a `<mso14:splitButton>` element (`SplitButtonWithTitle`).
pub fn split_button_with_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "splitButton").with_children(children)
}

/// Create a `<mso14:menu>` element (`MenuWithTitle`).
pub fn menu_with_title(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso14:menuSeparator>` element (`MenuSeparatorNoTitle`).
pub fn menu_separator_no_title() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menuSeparator")
}

/// Create a `<mso14:control>` element (`ControlClone`).
pub fn control_clone() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "control")
}

/// Create a `<mso14:labelControl>` element (`LabelControl`).
pub fn label_control() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "labelControl")
}

/// Create a `<mso14:button>` element (`Button`).
pub fn button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "button")
}

/// Create a `<mso14:toggleButton>` element (`ToggleButton`).
pub fn toggle_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "toggleButton")
}

/// Create a `<mso14:editBox>` element (`EditBox`).
pub fn edit_box() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "editBox")
}

/// Create a `<mso14:comboBox>` element (`ComboBox`).
pub fn combo_box(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "comboBox").with_children(children)
}

/// Create a `<mso14:dropDown>` element (`DropDownRegular`).
pub fn drop_down_regular(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "dropDown").with_children(children)
}

/// Create a `<mso14:gallery>` element (`Gallery`).
pub fn gallery(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "gallery").with_children(children)
}

/// Create a `<mso14:menu>` element (`Menu`).
pub fn menu(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso14:dynamicMenu>` element (`DynamicMenu`).
pub fn dynamic_menu() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "dynamicMenu")
}

/// Create a `<mso14:splitButton>` element (`SplitButton`).
pub fn split_button(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "splitButton").with_children(children)
}

/// Create a `<mso14:box>` element (`Box`).
pub fn box_(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "box").with_children(children)
}

/// Create a `<mso14:buttonGroup>` element (`ButtonGroup`).
pub fn button_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "buttonGroup").with_children(children)
}

/// Create a `<mso14:button>` element (`BackstageMenuButton`).
pub fn backstage_menu_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "button")
}

/// Create a `<mso14:checkBox>` element (`BackstageMenuCheckBox`).
pub fn backstage_menu_check_box() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "checkBox")
}

/// Create a `<mso14:menu>` element (`BackstageSubMenu`).
pub fn backstage_sub_menu(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso14:toggleButton>` element (`BackstageMenuToggleButton`).
pub fn backstage_menu_toggle_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "toggleButton")
}

/// Create a `<mso14:button>` element (`BackstageGroupButton`).
pub fn backstage_group_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "button")
}

/// Create a `<mso14:checkBox>` element (`BackstageCheckBox`).
pub fn backstage_check_box() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "checkBox")
}

/// Create a `<mso14:editBox>` element (`BackstageEditBox`).
pub fn backstage_edit_box() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "editBox")
}

/// Create a `<mso14:dropDown>` element (`BackstageDropDown`).
pub fn backstage_drop_down(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "dropDown").with_children(children)
}

/// Create a `<mso14:radioGroup>` element (`RadioGroup`).
pub fn radio_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "radioGroup").with_children(children)
}

/// Create a `<mso14:comboBox>` element (`BackstageComboBox`).
pub fn backstage_combo_box(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "comboBox").with_children(children)
}

/// Create a `<mso14:hyperlink>` element (`Hyperlink`).
pub fn hyperlink() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "hyperlink")
}

/// Create a `<mso14:labelControl>` element (`BackstageLabelControl`).
pub fn backstage_label_control() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "labelControl")
}

/// Create a `<mso14:groupBox>` element (`GroupBox`).
pub fn group_box(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "groupBox").with_children(children)
}

/// Create a `<mso14:layoutContainer>` element (`LayoutContainer`).
pub fn layout_container(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "layoutContainer").with_children(children)
}

/// Create a `<mso14:imageControl>` element (`ImageControl`).
pub fn image_control() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "imageControl")
}

/// Create a `<mso14:group>` element (`BackstageGroup`).
pub fn backstage_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "group").with_children(children)
}

/// Create a `<mso14:taskGroup>` element (`TaskGroup`).
pub fn task_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "taskGroup").with_children(children)
}

/// Create a `<mso14:menu>` element (`MenuRoot`).
pub fn menu_root(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso14:customUI>` element (`CustomUI`).
pub fn custom_u_i(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "customUI").with_children(children)
}

/// Create a `<mso14:item>` element (`Item`).
pub fn item() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "item")
}

/// Create a `<mso14:button>` element (`VisibleButton`).
pub fn visible_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "button")
}

/// Create a `<mso14:toggleButton>` element (`VisibleToggleButton`).
pub fn visible_toggle_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "toggleButton")
}

/// Create a `<mso14:separator>` element (`Separator`).
pub fn separator() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "separator")
}

/// Create a `<mso14:dialogBoxLauncher>` element (`DialogBoxLauncher`).
pub fn dialog_box_launcher(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "dialogBoxLauncher").with_children(children)
}

/// Create a `<mso14:group>` element (`Group`).
pub fn group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "group").with_children(children)
}

/// Create a `<mso14:control>` element (`ControlCloneQat`).
pub fn control_clone_qat() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "control")
}

/// Create a `<mso14:sharedControls>` element (`SharedControlsQatItems`).
pub fn shared_controls_qat_items(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "sharedControls").with_children(children)
}

/// Create a `<mso14:documentControls>` element (`DocumentControlsQatItems`).
pub fn document_controls_qat_items(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "documentControls").with_children(children)
}

/// Create a `<mso14:tab>` element (`Tab`).
pub fn tab(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "tab").with_children(children)
}

/// Create a `<mso14:tabSet>` element (`TabSet`).
pub fn tab_set(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "tabSet").with_children(children)
}

/// Create a `<mso14:command>` element (`Command`).
pub fn command() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "command")
}

/// Create a `<mso14:qat>` element (`QuickAccessToolbar`).
pub fn quick_access_toolbar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "qat").with_children(children)
}

/// Create a `<mso14:tabs>` element (`Tabs`).
pub fn tabs(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "tabs").with_children(children)
}

/// Create a `<mso14:contextualTabs>` element (`ContextualTabs`).
pub fn contextual_tabs(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "contextualTabs").with_children(children)
}

/// Create a `<mso14:contextMenu>` element (`ContextMenu`).
pub fn context_menu(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "contextMenu").with_children(children)
}

/// Create a `<mso14:item>` element (`ItemBackstageItem`).
pub fn item_backstage_item() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "item")
}

/// Create a `<mso14:radioButton>` element (`RadioButtonBackstageItem`).
pub fn radio_button_backstage_item() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "radioButton")
}

/// Create a `<mso14:button>` element (`BackstageRegularButton`).
pub fn backstage_regular_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "button")
}

/// Create a `<mso14:menu>` element (`BackstagePrimaryMenu`).
pub fn backstage_primary_menu(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menu").with_children(children)
}

/// Create a `<mso14:menuGroup>` element (`BackstageMenuGroup`).
pub fn backstage_menu_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "menuGroup").with_children(children)
}

/// Create a `<mso14:primaryItem>` element (`PrimaryItem`).
pub fn primary_item(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "primaryItem").with_children(children)
}

/// Create a `<mso14:topItems>` element (`TopItemsGroupControls`).
pub fn top_items_group_controls(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "topItems").with_children(children)
}

/// Create a `<mso14:bottomItems>` element (`BottomItemsGroupControls`).
pub fn bottom_items_group_controls(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "bottomItems").with_children(children)
}

/// Create a `<mso14:category>` element (`TaskGroupCategory`).
pub fn task_group_category(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "category").with_children(children)
}

/// Create a `<mso14:task>` element (`TaskGroupTask`).
pub fn task_group_task() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "task")
}

/// Create a `<mso14:category>` element (`TaskFormGroupCategory`).
pub fn task_form_group_category(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "category").with_children(children)
}

/// Create a `<mso14:task>` element (`TaskFormGroupTask`).
pub fn task_form_group_task(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "task").with_children(children)
}

/// Create a `<mso14:taskFormGroup>` element (`TaskFormGroup`).
pub fn task_form_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "taskFormGroup").with_children(children)
}

/// Create a `<mso14:firstColumn>` element (`BackstageGroups`).
pub fn backstage_groups(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "firstColumn").with_children(children)
}

/// Create a `<mso14:secondColumn>` element (`SimpleGroups`).
pub fn simple_groups(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "secondColumn").with_children(children)
}

/// Create a `<mso14:tab>` element (`BackstageTab`).
pub fn backstage_tab(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "tab").with_children(children)
}

/// Create a `<mso14:button>` element (`BackstageFastCommandButton`).
pub fn backstage_fast_command_button() -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "button")
}

/// Create a `<mso14:commands>` element (`Commands`).
pub fn commands(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "commands").with_children(children)
}

/// Create a `<mso14:ribbon>` element (`Ribbon`).
pub fn ribbon(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "ribbon").with_children(children)
}

/// Create a `<mso14:backstage>` element (`Backstage`).
pub fn backstage(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "backstage").with_children(children)
}

/// Create a `<mso14:contextMenus>` element (`ContextMenus`).
pub fn context_menus(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mso14", NAMESPACE_URI, "contextMenus").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 84;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 81;
