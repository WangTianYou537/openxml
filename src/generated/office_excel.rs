//! Auto-generated from `schemas-microsoft-com_office_excel.json`.
//! Target namespace: `urn:schemas-microsoft-com:office:excel` (prefix `xvml`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "urn:schemas-microsoft-com:office:excel";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xvml";

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

static ATTRS_CLIENT_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ObjectType", property_name: Some("ObjectType"), type_name: "EnumValue" },
];
static CHILDREN_CLIENT_DATA: &[ChildInfo] = &[
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:MoveWithCells", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:SizeWithCells", property_name: None },
    ChildInfo { name: "xsd:string/xvml:Anchor", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Locked", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:DefaultSize", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:PrintObject", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Disabled", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:AutoFill", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:AutoLine", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:AutoPict", property_name: None },
    ChildInfo { name: "xvml:ST_Macro/xvml:FmlaMacro", property_name: None },
    ChildInfo { name: "xsd:string/xvml:TextHAlign", property_name: None },
    ChildInfo { name: "xsd:string/xvml:TextVAlign", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:LockText", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:JustLastX", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:SecretEdit", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Default", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Help", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Cancel", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Dismiss", property_name: None },
    ChildInfo { name: "xsd:unsignedByte/xvml:Accel", property_name: None },
    ChildInfo { name: "xsd:unsignedByte/xvml:Accel2", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Row", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Column", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Visible", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:RowHidden", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:ColHidden", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:VTEdit", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:MultiLine", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:VScroll", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:ValidIds", property_name: None },
    ChildInfo { name: "xsd:string/xvml:FmlaRange", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:WidthMin", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Sel", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:NoThreeD2", property_name: None },
    ChildInfo { name: "xsd:string/xvml:SelType", property_name: None },
    ChildInfo { name: "xsd:string/xvml:MultiSel", property_name: None },
    ChildInfo { name: "xsd:string/xvml:LCT", property_name: None },
    ChildInfo { name: "xsd:string/xvml:ListItem", property_name: None },
    ChildInfo { name: "xsd:string/xvml:DropStyle", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Colored", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:DropLines", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Checked", property_name: None },
    ChildInfo { name: "xsd:string/xvml:FmlaLink", property_name: None },
    ChildInfo { name: "xsd:string/xvml:FmlaPict", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:NoThreeD", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:FirstButton", property_name: None },
    ChildInfo { name: "xsd:string/xvml:FmlaGroup", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Val", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Min", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Max", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Inc", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Page", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Horiz", property_name: None },
    ChildInfo { name: "xsd:integer/xvml:Dx", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:MapOCX", property_name: None },
    ChildInfo { name: "xvml:ST_CF/xvml:CF", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:Camera", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:RecalcAlways", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:AutoScale", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:DDE", property_name: None },
    ChildInfo { name: "xvml:ST_TrueFalseBlank/xvml:UIObj", property_name: None },
    ChildInfo { name: "xsd:string/xvml:ScriptText", property_name: None },
    ChildInfo { name: "xsd:string/xvml:ScriptExtended", property_name: None },
    ChildInfo { name: "xsd:nonNegativeInteger/xvml:ScriptLanguage", property_name: None },
    ChildInfo { name: "xsd:nonNegativeInteger/xvml:ScriptLocation", property_name: None },
    ChildInfo { name: "xsd:string/xvml:FmlaTxbx", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ClientData", local_name: "ClientData", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CLIENT_DATA, children: CHILDREN_CLIENT_DATA },
    ElementInfo { class_name: "MoveWithCells", local_name: "MoveWithCells", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ResizeWithCells", local_name: "SizeWithCells", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Locked", local_name: "Locked", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DefaultSize", local_name: "DefaultSize", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PrintObject", local_name: "PrintObject", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Disabled", local_name: "Disabled", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AutoFill", local_name: "AutoFill", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AutoLine", local_name: "AutoLine", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AutoSizePicture", local_name: "AutoPict", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "LockText", local_name: "LockText", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "JustifyLastLine", local_name: "JustLastX", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SecretEdit", local_name: "SecretEdit", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DefaultButton", local_name: "Default", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HelpButton", local_name: "Help", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CancelButton", local_name: "Cancel", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DismissButton", local_name: "Dismiss", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Visible", local_name: "Visible", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RowHidden", local_name: "RowHidden", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ColumnHidden", local_name: "ColHidden", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "MultiLine", local_name: "MultiLine", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VerticalScrollBar", local_name: "VScroll", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ValidIds", local_name: "ValidIds", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Disable3DForListBoxAndDropDown", local_name: "NoThreeD2", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Colored", local_name: "Colored", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Disable3D", local_name: "NoThreeD", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FirstButton", local_name: "FirstButton", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HorizontalScrollBar", local_name: "Horiz", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "MapOcxControl", local_name: "MapOCX", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CameraObject", local_name: "Camera", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RecalculateAlways", local_name: "RecalcAlways", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AutoScaleFont", local_name: "AutoScale", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DdeObject", local_name: "DDE", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "UIObject", local_name: "UIObj", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Anchor", local_name: "Anchor", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HorizontalTextAlignment", local_name: "TextHAlign", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VerticalTextAlignment", local_name: "TextVAlign", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FormulaRange", local_name: "FmlaRange", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SelectionType", local_name: "SelType", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "MultiSelections", local_name: "MultiSel", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ListBoxCallbackType", local_name: "LCT", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ListItem", local_name: "ListItem", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DropStyle", local_name: "DropStyle", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FormulaLink", local_name: "FmlaLink", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FormulaPicture", local_name: "FmlaPict", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FormulaGroup", local_name: "FmlaGroup", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScriptText", local_name: "ScriptText", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScriptExtended", local_name: "ScriptExtended", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FormulaTextBox", local_name: "FmlaTxbx", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FormulaMacro", local_name: "FmlaMacro", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AcceleratorPrimary", local_name: "Accel", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "AcceleratorSecondary", local_name: "Accel2", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CommentRowTarget", local_name: "Row", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CommentColumnTarget", local_name: "Column", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "InputValidationType", local_name: "VTEdit", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "MinDropDownWidth", local_name: "WidthMin", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SelectionEntry", local_name: "Sel", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DropLines", local_name: "DropLines", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Checked", local_name: "Checked", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScrollBarPosition", local_name: "Val", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScrollBarMin", local_name: "Min", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScrollBarMax", local_name: "Max", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScrollBarIncrement", local_name: "Inc", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScrollBarPageIncrement", local_name: "Page", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScrollBarWidth", local_name: "Dx", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ClipboardFormat", local_name: "CF", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScriptLanguage", local_name: "ScriptLanguage", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScriptLocation", local_name: "ScriptLocation", prefix: "xvml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
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

/// Create a `<xvml:ClientData>` element (`ClientData`).
pub fn client_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ClientData").with_children(children)
}

/// Create a `<xvml:MoveWithCells>` element (`MoveWithCells`).
pub fn move_with_cells(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "MoveWithCells").with_text(value)
}

/// Create a `<xvml:SizeWithCells>` element (`ResizeWithCells`).
pub fn resize_with_cells(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "SizeWithCells").with_text(value)
}

/// Create a `<xvml:Locked>` element (`Locked`).
pub fn locked(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Locked").with_text(value)
}

/// Create a `<xvml:DefaultSize>` element (`DefaultSize`).
pub fn default_size(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "DefaultSize").with_text(value)
}

/// Create a `<xvml:PrintObject>` element (`PrintObject`).
pub fn print_object(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "PrintObject").with_text(value)
}

/// Create a `<xvml:Disabled>` element (`Disabled`).
pub fn disabled(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Disabled").with_text(value)
}

/// Create a `<xvml:AutoFill>` element (`AutoFill`).
pub fn auto_fill(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "AutoFill").with_text(value)
}

/// Create a `<xvml:AutoLine>` element (`AutoLine`).
pub fn auto_line(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "AutoLine").with_text(value)
}

/// Create a `<xvml:AutoPict>` element (`AutoSizePicture`).
pub fn auto_size_picture(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "AutoPict").with_text(value)
}

/// Create a `<xvml:LockText>` element (`LockText`).
pub fn lock_text(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "LockText").with_text(value)
}

/// Create a `<xvml:JustLastX>` element (`JustifyLastLine`).
pub fn justify_last_line(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "JustLastX").with_text(value)
}

/// Create a `<xvml:SecretEdit>` element (`SecretEdit`).
pub fn secret_edit(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "SecretEdit").with_text(value)
}

/// Create a `<xvml:Default>` element (`DefaultButton`).
pub fn default_button(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Default").with_text(value)
}

/// Create a `<xvml:Help>` element (`HelpButton`).
pub fn help_button(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Help").with_text(value)
}

/// Create a `<xvml:Cancel>` element (`CancelButton`).
pub fn cancel_button(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Cancel").with_text(value)
}

/// Create a `<xvml:Dismiss>` element (`DismissButton`).
pub fn dismiss_button(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Dismiss").with_text(value)
}

/// Create a `<xvml:Visible>` element (`Visible`).
pub fn visible(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Visible").with_text(value)
}

/// Create a `<xvml:RowHidden>` element (`RowHidden`).
pub fn row_hidden(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "RowHidden").with_text(value)
}

/// Create a `<xvml:ColHidden>` element (`ColumnHidden`).
pub fn column_hidden(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ColHidden").with_text(value)
}

/// Create a `<xvml:MultiLine>` element (`MultiLine`).
pub fn multi_line(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "MultiLine").with_text(value)
}

/// Create a `<xvml:VScroll>` element (`VerticalScrollBar`).
pub fn vertical_scroll_bar(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "VScroll").with_text(value)
}

/// Create a `<xvml:ValidIds>` element (`ValidIds`).
pub fn valid_ids(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ValidIds").with_text(value)
}

/// Create a `<xvml:NoThreeD2>` element (`Disable3DForListBoxAndDropDown`).
pub fn disable3_d_for_list_box_and_drop_down(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "NoThreeD2").with_text(value)
}

/// Create a `<xvml:Colored>` element (`Colored`).
pub fn colored(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Colored").with_text(value)
}

/// Create a `<xvml:NoThreeD>` element (`Disable3D`).
pub fn disable3_d(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "NoThreeD").with_text(value)
}

/// Create a `<xvml:FirstButton>` element (`FirstButton`).
pub fn first_button(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "FirstButton").with_text(value)
}

/// Create a `<xvml:Horiz>` element (`HorizontalScrollBar`).
pub fn horizontal_scroll_bar(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Horiz").with_text(value)
}

/// Create a `<xvml:MapOCX>` element (`MapOcxControl`).
pub fn map_ocx_control(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "MapOCX").with_text(value)
}

/// Create a `<xvml:Camera>` element (`CameraObject`).
pub fn camera_object(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Camera").with_text(value)
}

/// Create a `<xvml:RecalcAlways>` element (`RecalculateAlways`).
pub fn recalculate_always(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "RecalcAlways").with_text(value)
}

/// Create a `<xvml:AutoScale>` element (`AutoScaleFont`).
pub fn auto_scale_font(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "AutoScale").with_text(value)
}

/// Create a `<xvml:DDE>` element (`DdeObject`).
pub fn dde_object(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "DDE").with_text(value)
}

/// Create a `<xvml:UIObj>` element (`UIObject`).
pub fn u_i_object(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "UIObj").with_text(value)
}

/// Create a `<xvml:Anchor>` element (`Anchor`).
pub fn anchor(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Anchor").with_text(value)
}

/// Create a `<xvml:TextHAlign>` element (`HorizontalTextAlignment`).
pub fn horizontal_text_alignment(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "TextHAlign").with_text(value)
}

/// Create a `<xvml:TextVAlign>` element (`VerticalTextAlignment`).
pub fn vertical_text_alignment(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "TextVAlign").with_text(value)
}

/// Create a `<xvml:FmlaRange>` element (`FormulaRange`).
pub fn formula_range(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "FmlaRange").with_text(value)
}

/// Create a `<xvml:SelType>` element (`SelectionType`).
pub fn selection_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "SelType").with_text(value)
}

/// Create a `<xvml:MultiSel>` element (`MultiSelections`).
pub fn multi_selections(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "MultiSel").with_text(value)
}

/// Create a `<xvml:LCT>` element (`ListBoxCallbackType`).
pub fn list_box_callback_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "LCT").with_text(value)
}

/// Create a `<xvml:ListItem>` element (`ListItem`).
pub fn list_item(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ListItem").with_text(value)
}

/// Create a `<xvml:DropStyle>` element (`DropStyle`).
pub fn drop_style(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "DropStyle").with_text(value)
}

/// Create a `<xvml:FmlaLink>` element (`FormulaLink`).
pub fn formula_link(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "FmlaLink").with_text(value)
}

/// Create a `<xvml:FmlaPict>` element (`FormulaPicture`).
pub fn formula_picture(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "FmlaPict").with_text(value)
}

/// Create a `<xvml:FmlaGroup>` element (`FormulaGroup`).
pub fn formula_group(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "FmlaGroup").with_text(value)
}

/// Create a `<xvml:ScriptText>` element (`ScriptText`).
pub fn script_text(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ScriptText").with_text(value)
}

/// Create a `<xvml:ScriptExtended>` element (`ScriptExtended`).
pub fn script_extended(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ScriptExtended").with_text(value)
}

/// Create a `<xvml:FmlaTxbx>` element (`FormulaTextBox`).
pub fn formula_text_box(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "FmlaTxbx").with_text(value)
}

/// Create a `<xvml:FmlaMacro>` element (`FormulaMacro`).
pub fn formula_macro(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "FmlaMacro").with_text(value)
}

/// Create a `<xvml:Accel>` element (`AcceleratorPrimary`).
pub fn accelerator_primary(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Accel").with_text(value)
}

/// Create a `<xvml:Accel2>` element (`AcceleratorSecondary`).
pub fn accelerator_secondary(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Accel2").with_text(value)
}

/// Create a `<xvml:Row>` element (`CommentRowTarget`).
pub fn comment_row_target(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Row").with_text(value)
}

/// Create a `<xvml:Column>` element (`CommentColumnTarget`).
pub fn comment_column_target(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Column").with_text(value)
}

/// Create a `<xvml:VTEdit>` element (`InputValidationType`).
pub fn input_validation_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "VTEdit").with_text(value)
}

/// Create a `<xvml:WidthMin>` element (`MinDropDownWidth`).
pub fn min_drop_down_width(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "WidthMin").with_text(value)
}

/// Create a `<xvml:Sel>` element (`SelectionEntry`).
pub fn selection_entry(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Sel").with_text(value)
}

/// Create a `<xvml:DropLines>` element (`DropLines`).
pub fn drop_lines(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "DropLines").with_text(value)
}

/// Create a `<xvml:Checked>` element (`Checked`).
pub fn checked(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Checked").with_text(value)
}

/// Create a `<xvml:Val>` element (`ScrollBarPosition`).
pub fn scroll_bar_position(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Val").with_text(value)
}

/// Create a `<xvml:Min>` element (`ScrollBarMin`).
pub fn scroll_bar_min(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Min").with_text(value)
}

/// Create a `<xvml:Max>` element (`ScrollBarMax`).
pub fn scroll_bar_max(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Max").with_text(value)
}

/// Create a `<xvml:Inc>` element (`ScrollBarIncrement`).
pub fn scroll_bar_increment(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Inc").with_text(value)
}

/// Create a `<xvml:Page>` element (`ScrollBarPageIncrement`).
pub fn scroll_bar_page_increment(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Page").with_text(value)
}

/// Create a `<xvml:Dx>` element (`ScrollBarWidth`).
pub fn scroll_bar_width(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "Dx").with_text(value)
}

/// Create a `<xvml:CF>` element (`ClipboardFormat`).
pub fn clipboard_format(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "CF").with_text(value)
}

/// Create a `<xvml:ScriptLanguage>` element (`ScriptLanguage`).
pub fn script_language(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ScriptLanguage").with_text(value)
}

/// Create a `<xvml:ScriptLocation>` element (`ScriptLocation`).
pub fn script_location(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xvml", NAMESPACE_URI, "ScriptLocation").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 68;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 68;
