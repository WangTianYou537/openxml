//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2014_revision.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2014/revision` (prefix `xr`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2014/revision";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xr";

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

static ATTRS_REV_EX_HEADERS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":minRev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":maxRev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":docId", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":endpointId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_REV_EX_HEADERS: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RevExHeader/xr:hdr", property_name: None },
];
static CHILDREN_REV_EX_STREAM: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RevExFuture/xr:xrrftr", property_name: None },
    ChildInfo { name: "xr:CT_RevExUnsupported/xr:xrrUspt", property_name: None },
    ChildInfo { name: "xr:CT_RevExTrimmed/xr:xrrTrim", property_name: None },
    ChildInfo { name: "xr:CT_RevExRowColumn/xr:xrrrc", property_name: None },
    ChildInfo { name: "xr:CT_RevExMove/xr:xrrm", property_name: None },
    ChildInfo { name: "xr:CT_RevExChangeCell/xr:xrrc", property_name: None },
    ChildInfo { name: "xr:CT_RevExFormatting/xr:xrrf", property_name: None },
    ChildInfo { name: "xr:CT_RevExDefinedName/xr:xrrDefName", property_name: None },
    ChildInfo { name: "xr:CT_RevExDelObj/xr:xrrdo", property_name: None },
    ChildInfo { name: "xr:CT_RevExChgObj/xr:xrrco", property_name: None },
    ChildInfo { name: "xr:CT_RevExSheetOp/xr:xrrSheet", property_name: None },
    ChildInfo { name: "xr:CT_RevisionList/xr:xrrList", property_name: None },
    ChildInfo { name: "xr:CT_RevListAutoExpandRw/xr:xrrListExpR", property_name: None },
    ChildInfo { name: "xr:CT_RevGroup/xr:xrrg", property_name: None },
];
static CHILDREN_DIFFERENTIAL_FORMAT_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Font/x:font", property_name: Some("Font") },
    ChildInfo { name: "x:CT_NumFmt/x:numFmt", property_name: Some("NumberingFormat") },
    ChildInfo { name: "x:CT_Fill/x:fill", property_name: Some("Fill") },
    ChildInfo { name: "x:CT_CellAlignment/x:alignment", property_name: Some("Alignment") },
    ChildInfo { name: "x:CT_Border/x:border", property_name: Some("Border") },
    ChildInfo { name: "x:CT_CellProtection/x:protection", property_name: Some("Protection") },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_REVISION_PTR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":revIDLastSave", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":documentId", property_name: None, type_name: "StringValue" },
];
static CHILDREN_STATE_BASED_OBJECT: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_DataValidation/xr:dataValidation", property_name: Some("DataValidation") },
    ChildInfo { name: "x:CT_Hyperlink/xr:hyperlink", property_name: Some("Hyperlink") },
    ChildInfo { name: "x14:CT_SparklineGroup/xr:sparklineGroup", property_name: Some("SparklineGroup") },
    ChildInfo { name: "x:CT_Comments/xr:comments", property_name: Some("Comments") },
    ChildInfo { name: "x:CT_AutoFilter/xr:autoFilter", property_name: Some("AutoFilter") },
    ChildInfo { name: "x:CT_pivotTableDefinition/xr:pivotTableDefinition", property_name: Some("pivotTableDefinition") },
];
static ATTRS_REV_EX_HEADER: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":minRev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":maxRev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":time", property_name: None, type_name: "DateTimeValue" },
];
static ATTRS_REV_EX_FUTURE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":sti", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_REV_EX_FUTURE: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RevExTest/xr:xrrtest", property_name: None },
];
static ATTRS_REV_EX_UNSUPPORTED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
];
static ATTRS_REV_EX_TRIMMED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
];
static ATTRS_REV_EX_ROW_COLUMN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":eol", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":ref", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":action", property_name: None, type_name: "EnumValue" },
];
static ATTRS_REV_EX_MOVE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":src", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":dst", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":srcSh", property_name: None, type_name: "StringValue" },
];
static ATTRS_REV_EX_CHANGE_CELL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":listUid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":r", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":x", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":w", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_REV_EX_CHANGE_CELL: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RevCell/xr:c", property_name: None },
    ChildInfo { name: "xr:CT_ChangeCellSubEdit/xr:ccse", property_name: None },
];
static ATTRS_REV_EX_FORMATTING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":numFmtId", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":xfDxf", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":style", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":sqref", property_name: None, type_name: "ListValue" },
    AttributeInfo { qname: ":start", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":length", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":styleUid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":fBlankCell", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyNumberFormat", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyFont", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyFill", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyBorder", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyAlignment", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyProtection", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_REV_EX_FORMATTING: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Dxf/xr:dxf", property_name: Some("DifferentialFormatType") },
    ChildInfo { name: "x:CT_ExtensionList/xr:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_REV_EX_DEFINED_NAME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":customView", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":function", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":functionGroupId", property_name: None, type_name: "ByteValue" },
    AttributeInfo { qname: ":shortcutKey", property_name: None, type_name: "ByteValue" },
    AttributeInfo { qname: ":hidden", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":customMenu", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":help", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":statusBar", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":comment", property_name: None, type_name: "StringValue" },
];
static CHILDREN_REV_EX_DEFINED_NAME: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xr:formula", property_name: Some("FormulaFormula") },
    ChildInfo { name: "x:CT_ExtensionList/xr:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_REV_EX_DEL_OBJ: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_REV_EX_DEL_OBJ: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_StateBasedHeader/xr:hdr", property_name: Some("StateBasedHeader") },
];
static ATTRS_REV_EX_CHG_OBJ: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_REV_EX_CHG_OBJ: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_StateBasedHeader/xr:hdr", property_name: Some("StateBasedHeader") },
    ChildInfo { name: "xr:CT_RevisionStateLink/xr:link", property_name: None },
    ChildInfo { name: "xr:CT_RevisionState/xr:body", property_name: None },
];
static ATTRS_REV_EX_SHEET_OP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":op", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":idOrig", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":idNew", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_REVISION_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":Data", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":Formatting", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":RangeBased", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":Fake", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":ref", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":Headers", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":InsDelHeaders", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":rId", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_REV_LIST_AUTO_EXPAND_RW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":refAdded", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":listGuid", property_name: None, type_name: "StringValue" },
];
static ATTRS_REV_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rev", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":sh", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidp", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ctx", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_REV_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RevExFuture/xr:xrrftr", property_name: None },
    ChildInfo { name: "xr:CT_RevExUnsupported/xr:xrrUspt", property_name: None },
    ChildInfo { name: "xr:CT_RevExTrimmed/xr:xrrTrim", property_name: None },
    ChildInfo { name: "xr:CT_RevExRowColumn/xr:xrrrc", property_name: None },
    ChildInfo { name: "xr:CT_RevExMove/xr:xrrm", property_name: None },
    ChildInfo { name: "xr:CT_RevExChangeCell/xr:xrrc", property_name: None },
    ChildInfo { name: "xr:CT_RevExFormatting/xr:xrrf", property_name: None },
    ChildInfo { name: "xr:CT_RevExDefinedName/xr:xrrDefName", property_name: None },
    ChildInfo { name: "xr:CT_RevExDelObj/xr:xrrdo", property_name: None },
    ChildInfo { name: "xr:CT_RevExChgObj/xr:xrrco", property_name: None },
    ChildInfo { name: "xr:CT_RevExSheetOp/xr:xrrSheet", property_name: None },
    ChildInfo { name: "xr:CT_RevisionList/xr:xrrList", property_name: None },
    ChildInfo { name: "xr:CT_RevListAutoExpandRw/xr:xrrListExpR", property_name: None },
];
static ATTRS_REV_CELL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":nop", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":tick", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":rep", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_REV_CELL: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Formula/xr:f", property_name: Some("FFormula") },
    ChildInfo { name: "x:ST_Xstring/xr:v", property_name: Some("Xstring") },
    ChildInfo { name: "x:CT_Rst/xr:is", property_name: Some("RstType") },
];
static ATTRS_CHANGE_CELL_SUB_EDIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":r", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":x", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":w", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_CHANGE_CELL_SUB_EDIT: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RevCell/xr:c", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_STATE_BASED_HEADER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":eft", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":eftx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":seft", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":seftx", property_name: None, type_name: "EnumValue" },
];
static CHILDREN_STATE_BASED_HEADER: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RefMap/xr:refmap", property_name: Some("RefMap") },
];
static ATTRS_REVISION_STATE_LINK: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static CHILDREN_REVISION_STATE: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RowColVisualOps/xr:rowColVisualOps", property_name: Some("RowColVisualOps") },
    ChildInfo { name: "xr:CT_HideUnhideSheet/xr:hideUnhideSheet", property_name: Some("HideUnhideSheet") },
    ChildInfo { name: "xr:CT_ShowGridlinesHeadings/xr:showGridlinesHeadings", property_name: Some("ShowGridlinesHeadings") },
    ChildInfo { name: "xr:CT_FreezePanes/xr:freezePanes", property_name: Some("FreezePanes") },
    ChildInfo { name: "xr:CT_Outlines/xr:outlines", property_name: Some("Outlines") },
];
static CHILDREN_REF_MAP: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_RefCell/xr:ref", property_name: None },
    ChildInfo { name: "xr:CT_SheetXluid/xr:sheetUid", property_name: None },
    ChildInfo { name: "xr:CT_RefOartAnchor/xr:oartAnchor", property_name: None },
    ChildInfo { name: "xr:CT_RefFuture/xr:future", property_name: None },
    ChildInfo { name: "xr:CT_RefTest/xr:test", property_name: None },
];
static ATTRS_ROW_COL_VISUAL_OPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":action", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":isRow", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":size", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":userSized", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_HIDE_UNHIDE_SHEET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":hide", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_SHOW_GRIDLINES_HEADINGS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":showGridLines", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":showRowCol", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_FREEZE_PANES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sheetViewUid", property_name: None, type_name: "StringValue" },
];
static ATTRS_OUTLINES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isRow", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_OUTLINES: &[ChildInfo] = &[
    ChildInfo { name: "xr:CT_Outline/xr:outline", property_name: None },
];
static ATTRS_OUTLINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isCollapsed", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":level", property_name: None, type_name: "ByteValue" },
];
static CHILDREN_RST_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Xstring/x:t", property_name: Some("Text") },
    ChildInfo { name: "x:CT_RElt/x:r", property_name: None },
    ChildInfo { name: "x:CT_PhoneticRun/x:rPh", property_name: None },
    ChildInfo { name: "x:CT_PhoneticPr/x:phoneticPr", property_name: None },
];
static ATTRS_REF_CELL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ajt", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":ajtx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":homeRef", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":r", property_name: None, type_name: "ListValue" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":uidLast", property_name: None, type_name: "StringValue" },
];
static ATTRS_SHEET_XLUID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ajt", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":ajtx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":homeRef", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":uid", property_name: None, type_name: "StringValue" },
];
static ATTRS_REF_OART_ANCHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ajt", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":ajtx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":homeRef", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":r", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":fromRowOff", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":fromColOff", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":toRowOff", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":toColOff", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":cx", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":x", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":oat", property_name: None, type_name: "EnumValue" },
];
static ATTRS_REF_TEST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":ajt", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":ajtx", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":homeRef", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_DATA_VALIDATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":errorStyle", property_name: Some("ErrorStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":imeMode", property_name: Some("ImeMode"), type_name: "EnumValue" },
    AttributeInfo { qname: ":operator", property_name: Some("Operator"), type_name: "EnumValue" },
    AttributeInfo { qname: ":allowBlank", property_name: Some("AllowBlank"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showDropDown", property_name: Some("ShowDropDown"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showInputMessage", property_name: Some("ShowInputMessage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showErrorMessage", property_name: Some("ShowErrorMessage"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":errorTitle", property_name: Some("ErrorTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":error", property_name: Some("Error"), type_name: "StringValue" },
    AttributeInfo { qname: ":promptTitle", property_name: Some("PromptTitle"), type_name: "StringValue" },
    AttributeInfo { qname: ":prompt", property_name: Some("Prompt"), type_name: "StringValue" },
    AttributeInfo { qname: ":sqref", property_name: Some("SequenceOfReferences"), type_name: "ListValue" },
];
static CHILDREN_DATA_VALIDATION: &[ChildInfo] = &[
    ChildInfo { name: "x:ST_Xstring/x12ac:list", property_name: Some("List") },
    ChildInfo { name: "x:CT_Xstring/x:formula1", property_name: Some("Formula1") },
    ChildInfo { name: "x:CT_Xstring/x:formula2", property_name: Some("Formula2") },
];
static ATTRS_HYPERLINK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":location", property_name: Some("Location"), type_name: "StringValue" },
    AttributeInfo { qname: ":tooltip", property_name: Some("Tooltip"), type_name: "StringValue" },
    AttributeInfo { qname: ":display", property_name: Some("Display"), type_name: "StringValue" },
];
static ATTRS_SPARKLINE_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":manualMax", property_name: Some("ManualMax"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":manualMin", property_name: Some("ManualMin"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":lineWeight", property_name: Some("LineWeight"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dateAxis", property_name: Some("DateAxis"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":displayEmptyCellsAs", property_name: Some("DisplayEmptyCellsAs"), type_name: "EnumValue" },
    AttributeInfo { qname: ":markers", property_name: Some("Markers"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":high", property_name: Some("High"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":low", property_name: Some("Low"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":first", property_name: Some("First"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":last", property_name: Some("Last"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":negative", property_name: Some("Negative"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":displayXAxis", property_name: Some("DisplayXAxis"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":displayHidden", property_name: Some("DisplayHidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":minAxisType", property_name: Some("MinAxisType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":maxAxisType", property_name: Some("MaxAxisType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rightToLeft", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
];
static CHILDREN_SPARKLINE_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Color/x14:colorSeries", property_name: Some("SeriesColor") },
    ChildInfo { name: "x:CT_Color/x14:colorNegative", property_name: Some("NegativeColor") },
    ChildInfo { name: "x:CT_Color/x14:colorAxis", property_name: Some("AxisColor") },
    ChildInfo { name: "x:CT_Color/x14:colorMarkers", property_name: Some("MarkersColor") },
    ChildInfo { name: "x:CT_Color/x14:colorFirst", property_name: Some("FirstMarkerColor") },
    ChildInfo { name: "x:CT_Color/x14:colorLast", property_name: Some("LastMarkerColor") },
    ChildInfo { name: "x:CT_Color/x14:colorHigh", property_name: Some("HighMarkerColor") },
    ChildInfo { name: "x:CT_Color/x14:colorLow", property_name: Some("LowMarkerColor") },
    ChildInfo { name: "x:ST_Formula/xne:f", property_name: Some("Formula") },
    ChildInfo { name: "x14:CT_Sparklines/x14:sparklines", property_name: Some("Sparklines") },
];
static CHILDREN_COMMENTS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Authors/x:authors", property_name: Some("Authors") },
    ChildInfo { name: "x:CT_CommentList/x:commentList", property_name: Some("CommentList") },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_AUTO_FILTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];
static CHILDREN_AUTO_FILTER: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_FilterColumn/x:filterColumn", property_name: None },
    ChildInfo { name: "x:CT_SortState/x:sortState", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/x:extLst", property_name: None },
];
static ATTRS_PIVOT_TABLE_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":cacheId", property_name: Some("CacheId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":dataOnRows", property_name: Some("DataOnRows"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dataPosition", property_name: Some("DataPosition"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":autoFormatId", property_name: Some("AutoFormatId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":applyNumberFormats", property_name: Some("ApplyNumberFormats"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyBorderFormats", property_name: Some("ApplyBorderFormats"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyFontFormats", property_name: Some("ApplyFontFormats"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyPatternFormats", property_name: Some("ApplyPatternFormats"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyAlignmentFormats", property_name: Some("ApplyAlignmentFormats"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":applyWidthHeightFormats", property_name: Some("ApplyWidthHeightFormats"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dataCaption", property_name: Some("DataCaption"), type_name: "StringValue" },
    AttributeInfo { qname: ":grandTotalCaption", property_name: Some("GrandTotalCaption"), type_name: "StringValue" },
    AttributeInfo { qname: ":errorCaption", property_name: Some("ErrorCaption"), type_name: "StringValue" },
    AttributeInfo { qname: ":showError", property_name: Some("ShowError"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":missingCaption", property_name: Some("MissingCaption"), type_name: "StringValue" },
    AttributeInfo { qname: ":showMissing", property_name: Some("ShowMissing"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":pageStyle", property_name: Some("PageStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":pivotTableStyle", property_name: Some("PivotTableStyleName"), type_name: "StringValue" },
    AttributeInfo { qname: ":vacatedStyle", property_name: Some("VacatedStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":tag", property_name: Some("Tag"), type_name: "StringValue" },
    AttributeInfo { qname: ":updatedVersion", property_name: Some("UpdatedVersion"), type_name: "ByteValue" },
    AttributeInfo { qname: ":minRefreshableVersion", property_name: Some("MinRefreshableVersion"), type_name: "ByteValue" },
    AttributeInfo { qname: ":asteriskTotals", property_name: Some("AsteriskTotals"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showItems", property_name: Some("ShowItems"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":editData", property_name: Some("EditData"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":disableFieldList", property_name: Some("DisableFieldList"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showCalcMbrs", property_name: Some("ShowCalculatedMembers"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":visualTotals", property_name: Some("VisualTotals"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showMultipleLabel", property_name: Some("ShowMultipleLabel"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showDataDropDown", property_name: Some("ShowDataDropDown"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showDrill", property_name: Some("ShowDrill"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":printDrill", property_name: Some("PrintDrill"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showMemberPropertyTips", property_name: Some("ShowMemberPropertyTips"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showDataTips", property_name: Some("ShowDataTips"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enableWizard", property_name: Some("EnableWizard"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enableDrill", property_name: Some("EnableDrill"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":enableFieldProperties", property_name: Some("EnableFieldProperties"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":preserveFormatting", property_name: Some("PreserveFormatting"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":useAutoFormatting", property_name: Some("UseAutoFormatting"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":pageWrap", property_name: Some("PageWrap"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":pageOverThenDown", property_name: Some("PageOverThenDown"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":subtotalHiddenItems", property_name: Some("SubtotalHiddenItems"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":rowGrandTotals", property_name: Some("RowGrandTotals"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":colGrandTotals", property_name: Some("ColumnGrandTotals"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fieldPrintTitles", property_name: Some("FieldPrintTitles"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":itemPrintTitles", property_name: Some("ItemPrintTitles"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":mergeItem", property_name: Some("MergeItem"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showDropZones", property_name: Some("ShowDropZones"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":createdVersion", property_name: Some("CreatedVersion"), type_name: "ByteValue" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":showEmptyRow", property_name: Some("ShowEmptyRow"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showEmptyCol", property_name: Some("ShowEmptyColumn"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":showHeaders", property_name: Some("ShowHeaders"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":compact", property_name: Some("Compact"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":outline", property_name: Some("Outline"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":outlineData", property_name: Some("OutlineData"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":compactData", property_name: Some("CompactData"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":published", property_name: Some("Published"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":gridDropZones", property_name: Some("GridDropZones"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":immersive", property_name: Some("StopImmersiveUi"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":multipleFieldFilters", property_name: Some("MultipleFieldFilters"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":chartFormat", property_name: Some("ChartFormat"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rowHeaderCaption", property_name: Some("RowHeaderCaption"), type_name: "StringValue" },
    AttributeInfo { qname: ":colHeaderCaption", property_name: Some("ColumnHeaderCaption"), type_name: "StringValue" },
    AttributeInfo { qname: ":fieldListSortAscending", property_name: Some("FieldListSortAscending"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":mdxSubqueries", property_name: Some("MdxSubqueries"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":customListSort", property_name: Some("CustomListSort"), type_name: "BooleanValue" },
];
static CHILDREN_PIVOT_TABLE_DEFINITION: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Location/x:location", property_name: Some("Location") },
    ChildInfo { name: "x:CT_PivotFields/x:pivotFields", property_name: Some("PivotFields") },
    ChildInfo { name: "x:CT_RowFields/x:rowFields", property_name: Some("RowFields") },
    ChildInfo { name: "x:CT_rowItems/x:rowItems", property_name: Some("RowItems") },
    ChildInfo { name: "x:CT_ColFields/x:colFields", property_name: Some("ColumnFields") },
    ChildInfo { name: "x:CT_colItems/x:colItems", property_name: Some("ColumnItems") },
    ChildInfo { name: "x:CT_PageFields/x:pageFields", property_name: Some("PageFields") },
    ChildInfo { name: "x:CT_DataFields/x:dataFields", property_name: Some("DataFields") },
    ChildInfo { name: "x:CT_Formats/x:formats", property_name: Some("Formats") },
    ChildInfo { name: "x:CT_ConditionalFormats/x:conditionalFormats", property_name: Some("ConditionalFormats") },
    ChildInfo { name: "x:CT_ChartFormats/x:chartFormats", property_name: Some("ChartFormats") },
    ChildInfo { name: "x:CT_PivotHierarchies/x:pivotHierarchies", property_name: Some("PivotHierarchies") },
    ChildInfo { name: "x:CT_PivotTableStyle/x:pivotTableStyleInfo", property_name: Some("PivotTableStyle") },
    ChildInfo { name: "x:CT_PivotFilters/x:filters", property_name: Some("PivotFilters") },
    ChildInfo { name: "x:CT_RowHierarchiesUsage/x:rowHierarchiesUsage", property_name: Some("RowHierarchiesUsage") },
    ChildInfo { name: "x:CT_ColHierarchiesUsage/x:colHierarchiesUsage", property_name: Some("ColumnHierarchiesUsage") },
    ChildInfo { name: "x:CT_pivotTableDefinitionExtensionList/x:extLst", property_name: Some("PivotTableDefinitionExtensionList") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "RevExHeaders", local_name: "revHdrs", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_EX_HEADERS, children: CHILDREN_REV_EX_HEADERS },
    ElementInfo { class_name: "RevExStream", local_name: "revStream", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_REV_EX_STREAM },
    ElementInfo { class_name: "DifferentialFormatType", local_name: "dxf", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIFFERENTIAL_FORMAT_TYPE },
    ElementInfo { class_name: "RevisionPtr", local_name: "revisionPtr", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REVISION_PTR, children: &[] },
    ElementInfo { class_name: "StateBasedObject", local_name: "objectState", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STATE_BASED_OBJECT },
    ElementInfo { class_name: "RevExHeader", local_name: "hdr", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REV_EX_HEADER, children: &[] },
    ElementInfo { class_name: "RevExFuture", local_name: "xrrftr", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_EX_FUTURE, children: CHILDREN_REV_EX_FUTURE },
    ElementInfo { class_name: "RevExUnsupported", local_name: "xrrUspt", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REV_EX_UNSUPPORTED, children: &[] },
    ElementInfo { class_name: "RevExTrimmed", local_name: "xrrTrim", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REV_EX_TRIMMED, children: &[] },
    ElementInfo { class_name: "RevExRowColumn", local_name: "xrrrc", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REV_EX_ROW_COLUMN, children: &[] },
    ElementInfo { class_name: "RevExMove", local_name: "xrrm", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REV_EX_MOVE, children: &[] },
    ElementInfo { class_name: "RevExChangeCell", local_name: "xrrc", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_EX_CHANGE_CELL, children: CHILDREN_REV_EX_CHANGE_CELL },
    ElementInfo { class_name: "RevExFormatting", local_name: "xrrf", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_EX_FORMATTING, children: CHILDREN_REV_EX_FORMATTING },
    ElementInfo { class_name: "RevExDefinedName", local_name: "xrrDefName", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_EX_DEFINED_NAME, children: CHILDREN_REV_EX_DEFINED_NAME },
    ElementInfo { class_name: "RevExDelObj", local_name: "xrrdo", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_EX_DEL_OBJ, children: CHILDREN_REV_EX_DEL_OBJ },
    ElementInfo { class_name: "RevExChgObj", local_name: "xrrco", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_EX_CHG_OBJ, children: CHILDREN_REV_EX_CHG_OBJ },
    ElementInfo { class_name: "RevExSheetOp", local_name: "xrrSheet", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REV_EX_SHEET_OP, children: &[] },
    ElementInfo { class_name: "RevisionList", local_name: "xrrList", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REVISION_LIST, children: &[] },
    ElementInfo { class_name: "RevListAutoExpandRw", local_name: "xrrListExpR", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REV_LIST_AUTO_EXPAND_RW, children: &[] },
    ElementInfo { class_name: "RevGroup", local_name: "xrrg", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_GROUP, children: CHILDREN_REV_GROUP },
    ElementInfo { class_name: "RevExTest", local_name: "xrrtest", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "RevCell", local_name: "c", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REV_CELL, children: CHILDREN_REV_CELL },
    ElementInfo { class_name: "ChangeCellSubEdit", local_name: "ccse", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHANGE_CELL_SUB_EDIT, children: CHILDREN_CHANGE_CELL_SUB_EDIT },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "FormulaFormula", local_name: "formula", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FFormula", local_name: "f", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "StateBasedHeader", local_name: "hdr", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STATE_BASED_HEADER, children: CHILDREN_STATE_BASED_HEADER },
    ElementInfo { class_name: "RevisionStateLink", local_name: "link", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REVISION_STATE_LINK, children: &[] },
    ElementInfo { class_name: "RevisionState", local_name: "body", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_REVISION_STATE },
    ElementInfo { class_name: "RefMap", local_name: "refmap", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_REF_MAP },
    ElementInfo { class_name: "RowColVisualOps", local_name: "rowColVisualOps", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROW_COL_VISUAL_OPS, children: &[] },
    ElementInfo { class_name: "HideUnhideSheet", local_name: "hideUnhideSheet", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIDE_UNHIDE_SHEET, children: &[] },
    ElementInfo { class_name: "ShowGridlinesHeadings", local_name: "showGridlinesHeadings", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHOW_GRIDLINES_HEADINGS, children: &[] },
    ElementInfo { class_name: "FreezePanes", local_name: "freezePanes", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FREEZE_PANES, children: &[] },
    ElementInfo { class_name: "Outlines", local_name: "outlines", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OUTLINES, children: CHILDREN_OUTLINES },
    ElementInfo { class_name: "Outline", local_name: "outline", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OUTLINE, children: &[] },
    ElementInfo { class_name: "Xstring", local_name: "v", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RstType", local_name: "is", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RST_TYPE },
    ElementInfo { class_name: "RefCell", local_name: "ref", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REF_CELL, children: &[] },
    ElementInfo { class_name: "SheetXluid", local_name: "sheetUid", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHEET_XLUID, children: &[] },
    ElementInfo { class_name: "RefOartAnchor", local_name: "oartAnchor", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REF_OART_ANCHOR, children: &[] },
    ElementInfo { class_name: "RefFuture", local_name: "future", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "RefTest", local_name: "test", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REF_TEST, children: &[] },
    ElementInfo { class_name: "DataValidation", local_name: "dataValidation", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_VALIDATION, children: CHILDREN_DATA_VALIDATION },
    ElementInfo { class_name: "Hyperlink", local_name: "hyperlink", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HYPERLINK, children: &[] },
    ElementInfo { class_name: "SparklineGroup", local_name: "sparklineGroup", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPARKLINE_GROUP, children: CHILDREN_SPARKLINE_GROUP },
    ElementInfo { class_name: "Comments", local_name: "comments", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COMMENTS },
    ElementInfo { class_name: "AutoFilter", local_name: "autoFilter", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AUTO_FILTER, children: CHILDREN_AUTO_FILTER },
    ElementInfo { class_name: "pivotTableDefinition", local_name: "pivotTableDefinition", prefix: "xr", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PIVOT_TABLE_DEFINITION, children: CHILDREN_PIVOT_TABLE_DEFINITION },
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

/// Create a `<xr:revHdrs>` element (`RevExHeaders`).
pub fn rev_ex_headers(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "revHdrs").with_children(children)
}

/// Create a `<xr:revStream>` element (`RevExStream`).
pub fn rev_ex_stream(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "revStream").with_children(children)
}

/// Create a `<xr:dxf>` element (`DifferentialFormatType`).
pub fn differential_format_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "dxf").with_children(children)
}

/// Create a `<xr:revisionPtr>` element (`RevisionPtr`).
pub fn revision_ptr() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "revisionPtr")
}

/// Create a `<xr:objectState>` element (`StateBasedObject`).
pub fn state_based_object(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "objectState").with_children(children)
}

/// Create a `<xr:hdr>` element (`RevExHeader`).
pub fn rev_ex_header() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "hdr")
}

/// Create a `<xr:xrrftr>` element (`RevExFuture`).
pub fn rev_ex_future(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrftr").with_children(children)
}

/// Create a `<xr:xrrUspt>` element (`RevExUnsupported`).
pub fn rev_ex_unsupported() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrUspt")
}

/// Create a `<xr:xrrTrim>` element (`RevExTrimmed`).
pub fn rev_ex_trimmed() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrTrim")
}

/// Create a `<xr:xrrrc>` element (`RevExRowColumn`).
pub fn rev_ex_row_column() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrrc")
}

/// Create a `<xr:xrrm>` element (`RevExMove`).
pub fn rev_ex_move() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrm")
}

/// Create a `<xr:xrrc>` element (`RevExChangeCell`).
pub fn rev_ex_change_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrc").with_children(children)
}

/// Create a `<xr:xrrf>` element (`RevExFormatting`).
pub fn rev_ex_formatting(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrf").with_children(children)
}

/// Create a `<xr:xrrDefName>` element (`RevExDefinedName`).
pub fn rev_ex_defined_name(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrDefName").with_children(children)
}

/// Create a `<xr:xrrdo>` element (`RevExDelObj`).
pub fn rev_ex_del_obj(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrdo").with_children(children)
}

/// Create a `<xr:xrrco>` element (`RevExChgObj`).
pub fn rev_ex_chg_obj(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrco").with_children(children)
}

/// Create a `<xr:xrrSheet>` element (`RevExSheetOp`).
pub fn rev_ex_sheet_op() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrSheet")
}

/// Create a `<xr:xrrList>` element (`RevisionList`).
pub fn revision_list() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrList")
}

/// Create a `<xr:xrrListExpR>` element (`RevListAutoExpandRw`).
pub fn rev_list_auto_expand_rw() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrListExpR")
}

/// Create a `<xr:xrrg>` element (`RevGroup`).
pub fn rev_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrg").with_children(children)
}

/// Create a `<xr:xrrtest>` element (`RevExTest`).
pub fn rev_ex_test() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "xrrtest")
}

/// Create a `<xr:c>` element (`RevCell`).
pub fn rev_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "c").with_children(children)
}

/// Create a `<xr:ccse>` element (`ChangeCellSubEdit`).
pub fn change_cell_sub_edit(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "ccse").with_children(children)
}

/// Create a `<xr:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<xr:formula>` element (`FormulaFormula`).
pub fn formula_formula(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "formula").with_text(value)
}

/// Create a `<xr:f>` element (`FFormula`).
pub fn f_formula(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "f").with_text(value)
}

/// Create a `<xr:hdr>` element (`StateBasedHeader`).
pub fn state_based_header(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "hdr").with_children(children)
}

/// Create a `<xr:link>` element (`RevisionStateLink`).
pub fn revision_state_link() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "link")
}

/// Create a `<xr:body>` element (`RevisionState`).
pub fn revision_state(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "body").with_children(children)
}

/// Create a `<xr:refmap>` element (`RefMap`).
pub fn ref_map(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "refmap").with_children(children)
}

/// Create a `<xr:rowColVisualOps>` element (`RowColVisualOps`).
pub fn row_col_visual_ops() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "rowColVisualOps")
}

/// Create a `<xr:hideUnhideSheet>` element (`HideUnhideSheet`).
pub fn hide_unhide_sheet() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "hideUnhideSheet")
}

/// Create a `<xr:showGridlinesHeadings>` element (`ShowGridlinesHeadings`).
pub fn show_gridlines_headings() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "showGridlinesHeadings")
}

/// Create a `<xr:freezePanes>` element (`FreezePanes`).
pub fn freeze_panes() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "freezePanes")
}

/// Create a `<xr:outlines>` element (`Outlines`).
pub fn outlines(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "outlines").with_children(children)
}

/// Create a `<xr:outline>` element (`Outline`).
pub fn outline() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "outline")
}

/// Create a `<xr:v>` element (`Xstring`).
pub fn xstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "v").with_text(value)
}

/// Create a `<xr:is>` element (`RstType`).
pub fn rst_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "is").with_children(children)
}

/// Create a `<xr:ref>` element (`RefCell`).
pub fn ref_cell() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "ref")
}

/// Create a `<xr:sheetUid>` element (`SheetXluid`).
pub fn sheet_xluid() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "sheetUid")
}

/// Create a `<xr:oartAnchor>` element (`RefOartAnchor`).
pub fn ref_oart_anchor() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "oartAnchor")
}

/// Create a `<xr:future>` element (`RefFuture`).
pub fn ref_future() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "future")
}

/// Create a `<xr:test>` element (`RefTest`).
pub fn ref_test() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "test")
}

/// Create a `<xr:dataValidation>` element (`DataValidation`).
pub fn data_validation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "dataValidation").with_children(children)
}

/// Create a `<xr:hyperlink>` element (`Hyperlink`).
pub fn hyperlink() -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "hyperlink")
}

/// Create a `<xr:sparklineGroup>` element (`SparklineGroup`).
pub fn sparkline_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "sparklineGroup").with_children(children)
}

/// Create a `<xr:comments>` element (`Comments`).
pub fn comments(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "comments").with_children(children)
}

/// Create a `<xr:autoFilter>` element (`AutoFilter`).
pub fn auto_filter(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "autoFilter").with_children(children)
}

/// Create a `<xr:pivotTableDefinition>` element (`pivotTableDefinition`).
pub fn pivot_table_definition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xr", NAMESPACE_URI, "pivotTableDefinition").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 49;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 49;
