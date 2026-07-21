//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2022/featurepropertybag` (prefix `xfpb`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2022/featurepropertybag";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xfpb";

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

static ATTRS_FEATURE_PROPERTY_BAGS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_FEATURE_PROPERTY_BAGS: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_BagExtensions/xfpb:bagExt", property_name: None },
    ChildInfo { name: "xfpb:CT_FeaturePropertyBag/xfpb:bag", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xfpb:extLst", property_name: None },
];
static ATTRS_FPBS_FEATURE_PROPERTY_BAGS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":count", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_FPBS_FEATURE_PROPERTY_BAGS: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_BagExtensions/xfpb:bagExt", property_name: None },
    ChildInfo { name: "xfpb:CT_FeaturePropertyBag/xfpb:bag", property_name: None },
    ChildInfo { name: "x:CT_ExtensionList/xfpb:extLst", property_name: None },
];
static ATTRS_XF_COMPLEMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":i", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_D_X_F_COMPLEMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":i", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_HEADER_ROW_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_DATA_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_TOTALS_ROW_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_HEADER_ROW_BORDER_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_TABLE_BORDER_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_TOTALS_ROW_BORDER_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_COLUMN_HEADER_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_COLUMN_BODY_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_COLUMN_TOTALS_REV_DXF_TABLE_REV_DXF: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_FeaturePropertyBags/xfpb:fpbs", property_name: Some("FpbsFeaturePropertyBags") },
    ChildInfo { name: "x:CT_Dxf/xfpb:dxf", property_name: Some("DifferentialFormatType") },
];
static CHILDREN_BAG_EXTENSIONS: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_ExtensionList/xfpb:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FEATURE_PROPERTY_BAG: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":extRef", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":bagExtId", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":att", property_name: None, type_name: "StringValue" },
];
static CHILDREN_FEATURE_PROPERTY_BAG: &[ChildInfo] = &[
    ChildInfo { name: "xfpb:CT_ArrayFeatureProperty/xfpb:a", property_name: None },
    ChildInfo { name: "xfpb:CT_BagFeatureProperty/xfpb:bagId", property_name: None },
    ChildInfo { name: "xfpb:CT_IntFeatureProperty/xfpb:i", property_name: None },
    ChildInfo { name: "xfpb:CT_StringFeatureProperty/xfpb:s", property_name: None },
    ChildInfo { name: "xfpb:CT_BoolFeatureProperty/xfpb:b", property_name: None },
    ChildInfo { name: "xfpb:CT_DecimalFeatureProperty/xfpb:d", property_name: None },
    ChildInfo { name: "xfpb:CT_RelFeatureProperty/xfpb:rel", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "x:CT_Extension/x:ext", property_name: None },
];
static ATTRS_ARRAY_FEATURE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":k", property_name: None, type_name: "StringValue" },
];
static CHILDREN_ARRAY_FEATURE_PROPERTY: &[ChildInfo] = &[
    ChildInfo { name: "xsd:unsignedInt/xfpb:bagId", property_name: None },
    ChildInfo { name: "xsd:integer/xfpb:i", property_name: None },
    ChildInfo { name: "xsd:string/xfpb:s", property_name: None },
    ChildInfo { name: "xsd:boolean/xfpb:b", property_name: None },
    ChildInfo { name: "xsd:double/xfpb:d", property_name: None },
    ChildInfo { name: "xsd:string/xfpb:rel", property_name: None },
];
static ATTRS_BAG_FEATURE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":k", property_name: None, type_name: "StringValue" },
];
static ATTRS_INT_FEATURE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":k", property_name: None, type_name: "StringValue" },
];
static ATTRS_STRING_FEATURE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":k", property_name: None, type_name: "StringValue" },
];
static ATTRS_BOOL_FEATURE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":k", property_name: None, type_name: "StringValue" },
];
static ATTRS_DECIMAL_FEATURE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":k", property_name: None, type_name: "StringValue" },
];
static ATTRS_REL_FEATURE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":k", property_name: None, type_name: "StringValue" },
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "FeaturePropertyBags", local_name: "FeaturePropertyBags", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FEATURE_PROPERTY_BAGS, children: CHILDREN_FEATURE_PROPERTY_BAGS },
    ElementInfo { class_name: "FpbsFeaturePropertyBags", local_name: "fpbs", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FPBS_FEATURE_PROPERTY_BAGS, children: CHILDREN_FPBS_FEATURE_PROPERTY_BAGS },
    ElementInfo { class_name: "XfComplement", local_name: "xfComplement", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_XF_COMPLEMENT, children: &[] },
    ElementInfo { class_name: "DXFComplement", local_name: "DXFComplement", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_D_X_F_COMPLEMENT, children: &[] },
    ElementInfo { class_name: "RevDxf", local_name: "revdxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_REV_DXF },
    ElementInfo { class_name: "HeaderRowRevDxfTableRevDxf", local_name: "headerRowRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HEADER_ROW_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "DataRevDxfTableRevDxf", local_name: "dataRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "TotalsRowRevDxfTableRevDxf", local_name: "totalsRowRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TOTALS_ROW_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "HeaderRowBorderRevDxfTableRevDxf", local_name: "headerRowBorderRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HEADER_ROW_BORDER_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "TableBorderRevDxfTableRevDxf", local_name: "tableBorderRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABLE_BORDER_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "TotalsRowBorderRevDxfTableRevDxf", local_name: "totalsRowBorderRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TOTALS_ROW_BORDER_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "ColumnHeaderRevDxfTableRevDxf", local_name: "columnHeaderRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLUMN_HEADER_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "ColumnBodyRevDxfTableRevDxf", local_name: "columnBodyRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLUMN_BODY_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "ColumnTotalsRevDxfTableRevDxf", local_name: "columnTotalsRevDxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLUMN_TOTALS_REV_DXF_TABLE_REV_DXF },
    ElementInfo { class_name: "BagExtensions", local_name: "bagExt", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAG_EXTENSIONS },
    ElementInfo { class_name: "FeaturePropertyBag", local_name: "bag", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FEATURE_PROPERTY_BAG, children: CHILDREN_FEATURE_PROPERTY_BAG },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "ArrayFeatureProperty", local_name: "a", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ARRAY_FEATURE_PROPERTY, children: CHILDREN_ARRAY_FEATURE_PROPERTY },
    ElementInfo { class_name: "BagFeatureProperty", local_name: "bagId", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_BAG_FEATURE_PROPERTY, children: &[] },
    ElementInfo { class_name: "IntFeatureProperty", local_name: "i", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_INT_FEATURE_PROPERTY, children: &[] },
    ElementInfo { class_name: "StringFeatureProperty", local_name: "s", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_STRING_FEATURE_PROPERTY, children: &[] },
    ElementInfo { class_name: "BoolFeatureProperty", local_name: "b", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_BOOL_FEATURE_PROPERTY, children: &[] },
    ElementInfo { class_name: "DecimalFeatureProperty", local_name: "d", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_DECIMAL_FEATURE_PROPERTY, children: &[] },
    ElementInfo { class_name: "RelFeatureProperty", local_name: "rel", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_REL_FEATURE_PROPERTY, children: &[] },
    ElementInfo { class_name: "DifferentialFormatType", local_name: "dxf", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIFFERENTIAL_FORMAT_TYPE },
    ElementInfo { class_name: "XsdunsignedInt", local_name: "bagId", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Xsdinteger", local_name: "i", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SXsdstring", local_name: "s", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "RelXsdstring", local_name: "rel", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Xsdboolean", local_name: "b", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Xsddouble", local_name: "d", prefix: "xfpb", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
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

/// Create a `<xfpb:FeaturePropertyBags>` element (`FeaturePropertyBags`).
pub fn feature_property_bags(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "FeaturePropertyBags").with_children(children)
}

/// Create a `<xfpb:fpbs>` element (`FpbsFeaturePropertyBags`).
pub fn fpbs_feature_property_bags(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "fpbs").with_children(children)
}

/// Create a `<xfpb:xfComplement>` element (`XfComplement`).
pub fn xf_complement() -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "xfComplement")
}

/// Create a `<xfpb:DXFComplement>` element (`DXFComplement`).
pub fn d_x_f_complement() -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "DXFComplement")
}

/// Create a `<xfpb:revdxf>` element (`RevDxf`).
pub fn rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "revdxf").with_children(children)
}

/// Create a `<xfpb:headerRowRevDxf>` element (`HeaderRowRevDxfTableRevDxf`).
pub fn header_row_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "headerRowRevDxf").with_children(children)
}

/// Create a `<xfpb:dataRevDxf>` element (`DataRevDxfTableRevDxf`).
pub fn data_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "dataRevDxf").with_children(children)
}

/// Create a `<xfpb:totalsRowRevDxf>` element (`TotalsRowRevDxfTableRevDxf`).
pub fn totals_row_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "totalsRowRevDxf").with_children(children)
}

/// Create a `<xfpb:headerRowBorderRevDxf>` element (`HeaderRowBorderRevDxfTableRevDxf`).
pub fn header_row_border_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "headerRowBorderRevDxf").with_children(children)
}

/// Create a `<xfpb:tableBorderRevDxf>` element (`TableBorderRevDxfTableRevDxf`).
pub fn table_border_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "tableBorderRevDxf").with_children(children)
}

/// Create a `<xfpb:totalsRowBorderRevDxf>` element (`TotalsRowBorderRevDxfTableRevDxf`).
pub fn totals_row_border_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "totalsRowBorderRevDxf").with_children(children)
}

/// Create a `<xfpb:columnHeaderRevDxf>` element (`ColumnHeaderRevDxfTableRevDxf`).
pub fn column_header_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "columnHeaderRevDxf").with_children(children)
}

/// Create a `<xfpb:columnBodyRevDxf>` element (`ColumnBodyRevDxfTableRevDxf`).
pub fn column_body_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "columnBodyRevDxf").with_children(children)
}

/// Create a `<xfpb:columnTotalsRevDxf>` element (`ColumnTotalsRevDxfTableRevDxf`).
pub fn column_totals_rev_dxf_table_rev_dxf(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "columnTotalsRevDxf").with_children(children)
}

/// Create a `<xfpb:bagExt>` element (`BagExtensions`).
pub fn bag_extensions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "bagExt").with_children(children)
}

/// Create a `<xfpb:bag>` element (`FeaturePropertyBag`).
pub fn feature_property_bag(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "bag").with_children(children)
}

/// Create a `<xfpb:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<xfpb:a>` element (`ArrayFeatureProperty`).
pub fn array_feature_property(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "a").with_children(children)
}

/// Create a `<xfpb:bagId>` element (`BagFeatureProperty`).
pub fn bag_feature_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "bagId").with_text(value)
}

/// Create a `<xfpb:i>` element (`IntFeatureProperty`).
pub fn int_feature_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "i").with_text(value)
}

/// Create a `<xfpb:s>` element (`StringFeatureProperty`).
pub fn string_feature_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "s").with_text(value)
}

/// Create a `<xfpb:b>` element (`BoolFeatureProperty`).
pub fn bool_feature_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "b").with_text(value)
}

/// Create a `<xfpb:d>` element (`DecimalFeatureProperty`).
pub fn decimal_feature_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "d").with_text(value)
}

/// Create a `<xfpb:rel>` element (`RelFeatureProperty`).
pub fn rel_feature_property(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "rel").with_text(value)
}

/// Create a `<xfpb:dxf>` element (`DifferentialFormatType`).
pub fn differential_format_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "dxf").with_children(children)
}

/// Create a `<xfpb:bagId>` element (`XsdunsignedInt`).
pub fn xsdunsigned_int(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "bagId").with_text(value)
}

/// Create a `<xfpb:i>` element (`Xsdinteger`).
pub fn xsdinteger(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "i").with_text(value)
}

/// Create a `<xfpb:s>` element (`SXsdstring`).
pub fn s_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "s").with_text(value)
}

/// Create a `<xfpb:rel>` element (`RelXsdstring`).
pub fn rel_xsdstring(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "rel").with_text(value)
}

/// Create a `<xfpb:b>` element (`Xsdboolean`).
pub fn xsdboolean(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "b").with_text(value)
}

/// Create a `<xfpb:d>` element (`Xsddouble`).
pub fn xsddouble(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("xfpb", NAMESPACE_URI, "d").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 33;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 31;
