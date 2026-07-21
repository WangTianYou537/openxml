//! Auto-generated from `schemas_openxmlformats_org_officeDocument_2006_docPropsVTypes.json`.
//! Target namespace: `http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes` (prefix `vt`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "vt";

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

static CHILDREN_VARIANT: &[ChildInfo] = &[
    ChildInfo { name: "vt:CT_Variant/vt:variant", property_name: Some("InnerVariant") },
    ChildInfo { name: "vt:CT_Vector/vt:vector", property_name: Some("VTVector") },
    ChildInfo { name: "vt:CT_Array/vt:array", property_name: Some("VTArray") },
    ChildInfo { name: "xsd:base64Binary/vt:blob", property_name: Some("VTBlob") },
    ChildInfo { name: "xsd:base64Binary/vt:oblob", property_name: Some("VTOBlob") },
    ChildInfo { name: "vt:CT_Empty/vt:empty", property_name: Some("VTEmpty") },
    ChildInfo { name: "vt:CT_Null/vt:null", property_name: Some("VTNull") },
    ChildInfo { name: "xsd:byte/vt:i1", property_name: Some("VTByte") },
    ChildInfo { name: "xsd:short/vt:i2", property_name: Some("VTShort") },
    ChildInfo { name: "xsd:int/vt:i4", property_name: Some("VTInt32") },
    ChildInfo { name: "xsd:long/vt:i8", property_name: Some("VTInt64") },
    ChildInfo { name: "xsd:int/vt:int", property_name: Some("VTInteger") },
    ChildInfo { name: "xsd:unsignedByte/vt:ui1", property_name: Some("VTUnsignedByte") },
    ChildInfo { name: "xsd:unsignedShort/vt:ui2", property_name: Some("VTUnsignedShort") },
    ChildInfo { name: "xsd:unsignedInt/vt:ui4", property_name: Some("VTUnsignedInt32") },
    ChildInfo { name: "xsd:unsignedLong/vt:ui8", property_name: Some("VTUnsignedInt64") },
    ChildInfo { name: "xsd:unsignedInt/vt:uint", property_name: Some("VTUnsignedInteger") },
    ChildInfo { name: "xsd:float/vt:r4", property_name: Some("VTFloat") },
    ChildInfo { name: "xsd:double/vt:r8", property_name: Some("VTDouble") },
    ChildInfo { name: "xsd:decimal/vt:decimal", property_name: Some("VTDecimal") },
    ChildInfo { name: "xsd:string/vt:lpstr", property_name: Some("VTLPSTR") },
    ChildInfo { name: "xsd:string/vt:lpwstr", property_name: Some("VTLPWSTR") },
    ChildInfo { name: "xsd:string/vt:bstr", property_name: Some("VTBString") },
    ChildInfo { name: "xsd:dateTime/vt:date", property_name: Some("VTDate") },
    ChildInfo { name: "xsd:dateTime/vt:filetime", property_name: Some("VTFileTime") },
    ChildInfo { name: "xsd:boolean/vt:bool", property_name: Some("VTBool") },
    ChildInfo { name: "vt:ST_Cy/vt:cy", property_name: Some("VTCurrency") },
    ChildInfo { name: "vt:ST_Error/vt:error", property_name: Some("VTError") },
    ChildInfo { name: "xsd:base64Binary/vt:stream", property_name: Some("VTStreamData") },
    ChildInfo { name: "xsd:base64Binary/vt:ostream", property_name: Some("VTOStreamData") },
    ChildInfo { name: "xsd:base64Binary/vt:storage", property_name: Some("VTStorage") },
    ChildInfo { name: "xsd:base64Binary/vt:ostorage", property_name: Some("VTOStorage") },
    ChildInfo { name: "vt:CT_Vstream/vt:vstream", property_name: Some("VTVStreamData") },
    ChildInfo { name: "vt:ST_Clsid/vt:clsid", property_name: Some("VTClassId") },
    ChildInfo { name: "vt:CT_Cf/vt:cf", property_name: Some("VTClipboardData") },
];
static ATTRS_V_T_VECTOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":baseType", property_name: Some("BaseType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "UInt32Value" },
];
static CHILDREN_V_T_VECTOR: &[ChildInfo] = &[
    ChildInfo { name: "vt:CT_Variant/vt:variant", property_name: None },
    ChildInfo { name: "xsd:byte/vt:i1", property_name: None },
    ChildInfo { name: "xsd:short/vt:i2", property_name: None },
    ChildInfo { name: "xsd:int/vt:i4", property_name: None },
    ChildInfo { name: "xsd:long/vt:i8", property_name: None },
    ChildInfo { name: "xsd:unsignedByte/vt:ui1", property_name: None },
    ChildInfo { name: "xsd:unsignedShort/vt:ui2", property_name: None },
    ChildInfo { name: "xsd:unsignedInt/vt:ui4", property_name: None },
    ChildInfo { name: "xsd:unsignedLong/vt:ui8", property_name: None },
    ChildInfo { name: "xsd:float/vt:r4", property_name: None },
    ChildInfo { name: "xsd:double/vt:r8", property_name: None },
    ChildInfo { name: "xsd:string/vt:lpstr", property_name: None },
    ChildInfo { name: "xsd:string/vt:lpwstr", property_name: None },
    ChildInfo { name: "xsd:string/vt:bstr", property_name: None },
    ChildInfo { name: "xsd:dateTime/vt:date", property_name: None },
    ChildInfo { name: "xsd:dateTime/vt:filetime", property_name: None },
    ChildInfo { name: "xsd:boolean/vt:bool", property_name: None },
    ChildInfo { name: "vt:ST_Cy/vt:cy", property_name: None },
    ChildInfo { name: "vt:ST_Error/vt:error", property_name: None },
    ChildInfo { name: "vt:ST_Clsid/vt:clsid", property_name: None },
    ChildInfo { name: "vt:CT_Cf/vt:cf", property_name: None },
];
static ATTRS_V_T_ARRAY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lBound", property_name: Some("LowerBounds"), type_name: "Int32Value" },
    AttributeInfo { qname: ":uBound", property_name: Some("UpperBounds"), type_name: "Int32Value" },
    AttributeInfo { qname: ":baseType", property_name: Some("BaseType"), type_name: "EnumValue" },
];
static CHILDREN_V_T_ARRAY: &[ChildInfo] = &[
    ChildInfo { name: "vt:CT_Variant/vt:variant", property_name: None },
    ChildInfo { name: "xsd:byte/vt:i1", property_name: None },
    ChildInfo { name: "xsd:short/vt:i2", property_name: None },
    ChildInfo { name: "xsd:int/vt:i4", property_name: None },
    ChildInfo { name: "xsd:int/vt:int", property_name: None },
    ChildInfo { name: "xsd:unsignedByte/vt:ui1", property_name: None },
    ChildInfo { name: "xsd:unsignedShort/vt:ui2", property_name: None },
    ChildInfo { name: "xsd:unsignedInt/vt:ui4", property_name: None },
    ChildInfo { name: "xsd:unsignedInt/vt:uint", property_name: None },
    ChildInfo { name: "xsd:float/vt:r4", property_name: None },
    ChildInfo { name: "xsd:double/vt:r8", property_name: None },
    ChildInfo { name: "xsd:decimal/vt:decimal", property_name: None },
    ChildInfo { name: "xsd:string/vt:bstr", property_name: None },
    ChildInfo { name: "xsd:dateTime/vt:date", property_name: None },
    ChildInfo { name: "xsd:boolean/vt:bool", property_name: None },
    ChildInfo { name: "vt:ST_Error/vt:error", property_name: None },
    ChildInfo { name: "vt:ST_Cy/vt:cy", property_name: None },
];
static ATTRS_V_T_V_STREAM_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":version", property_name: Some("Version"), type_name: "StringValue" },
];
static ATTRS_V_T_CLIPBOARD_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":format", property_name: Some("Format"), type_name: "Int32Value" },
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Variant", local_name: "variant", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VARIANT },
    ElementInfo { class_name: "VTVector", local_name: "vector", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_V_T_VECTOR, children: CHILDREN_V_T_VECTOR },
    ElementInfo { class_name: "VTArray", local_name: "array", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_V_T_ARRAY, children: CHILDREN_V_T_ARRAY },
    ElementInfo { class_name: "VTBlob", local_name: "blob", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTOBlob", local_name: "oblob", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTStreamData", local_name: "stream", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTOStreamData", local_name: "ostream", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTStorage", local_name: "storage", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTOStorage", local_name: "ostorage", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTEmpty", local_name: "empty", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTNull", local_name: "null", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTByte", local_name: "i1", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTShort", local_name: "i2", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTInt32", local_name: "i4", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTInteger", local_name: "int", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTInt64", local_name: "i8", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTUnsignedByte", local_name: "ui1", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTUnsignedShort", local_name: "ui2", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTUnsignedInt32", local_name: "ui4", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTUnsignedInteger", local_name: "uint", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTUnsignedInt64", local_name: "ui8", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTFloat", local_name: "r4", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTDouble", local_name: "r8", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTDecimal", local_name: "decimal", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTLPSTR", local_name: "lpstr", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTLPWSTR", local_name: "lpwstr", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTBString", local_name: "bstr", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTDate", local_name: "date", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTFileTime", local_name: "filetime", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTBool", local_name: "bool", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTCurrency", local_name: "cy", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTError", local_name: "error", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTVStreamData", local_name: "vstream", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_V_T_V_STREAM_DATA, children: &[] },
    ElementInfo { class_name: "VTClassId", local_name: "clsid", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "VTClipboardData", local_name: "cf", prefix: "vt", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_V_T_CLIPBOARD_DATA, children: &[] },
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

/// Create a `<vt:variant>` element (`Variant`).
pub fn variant(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "variant").with_children(children)
}

/// Create a `<vt:vector>` element (`VTVector`).
pub fn v_t_vector(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "vector").with_children(children)
}

/// Create a `<vt:array>` element (`VTArray`).
pub fn v_t_array(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "array").with_children(children)
}

/// Create a `<vt:blob>` element (`VTBlob`).
pub fn v_t_blob(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "blob").with_text(value)
}

/// Create a `<vt:oblob>` element (`VTOBlob`).
pub fn v_t_o_blob(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "oblob").with_text(value)
}

/// Create a `<vt:stream>` element (`VTStreamData`).
pub fn v_t_stream_data(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "stream").with_text(value)
}

/// Create a `<vt:ostream>` element (`VTOStreamData`).
pub fn v_t_o_stream_data(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "ostream").with_text(value)
}

/// Create a `<vt:storage>` element (`VTStorage`).
pub fn v_t_storage(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "storage").with_text(value)
}

/// Create a `<vt:ostorage>` element (`VTOStorage`).
pub fn v_t_o_storage(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "ostorage").with_text(value)
}

/// Create a `<vt:empty>` element (`VTEmpty`).
pub fn v_t_empty() -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "empty")
}

/// Create a `<vt:null>` element (`VTNull`).
pub fn v_t_null() -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "null")
}

/// Create a `<vt:i1>` element (`VTByte`).
pub fn v_t_byte(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "i1").with_text(value)
}

/// Create a `<vt:i2>` element (`VTShort`).
pub fn v_t_short(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "i2").with_text(value)
}

/// Create a `<vt:i4>` element (`VTInt32`).
pub fn v_t_int32(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "i4").with_text(value)
}

/// Create a `<vt:int>` element (`VTInteger`).
pub fn v_t_integer(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "int").with_text(value)
}

/// Create a `<vt:i8>` element (`VTInt64`).
pub fn v_t_int64(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "i8").with_text(value)
}

/// Create a `<vt:ui1>` element (`VTUnsignedByte`).
pub fn v_t_unsigned_byte(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "ui1").with_text(value)
}

/// Create a `<vt:ui2>` element (`VTUnsignedShort`).
pub fn v_t_unsigned_short(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "ui2").with_text(value)
}

/// Create a `<vt:ui4>` element (`VTUnsignedInt32`).
pub fn v_t_unsigned_int32(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "ui4").with_text(value)
}

/// Create a `<vt:uint>` element (`VTUnsignedInteger`).
pub fn v_t_unsigned_integer(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "uint").with_text(value)
}

/// Create a `<vt:ui8>` element (`VTUnsignedInt64`).
pub fn v_t_unsigned_int64(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "ui8").with_text(value)
}

/// Create a `<vt:r4>` element (`VTFloat`).
pub fn v_t_float(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "r4").with_text(value)
}

/// Create a `<vt:r8>` element (`VTDouble`).
pub fn v_t_double(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "r8").with_text(value)
}

/// Create a `<vt:decimal>` element (`VTDecimal`).
pub fn v_t_decimal(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "decimal").with_text(value)
}

/// Create a `<vt:lpstr>` element (`VTLPSTR`).
pub fn v_t_l_p_s_t_r(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "lpstr").with_text(value)
}

/// Create a `<vt:lpwstr>` element (`VTLPWSTR`).
pub fn v_t_l_p_w_s_t_r(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "lpwstr").with_text(value)
}

/// Create a `<vt:bstr>` element (`VTBString`).
pub fn v_t_b_string(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "bstr").with_text(value)
}

/// Create a `<vt:date>` element (`VTDate`).
pub fn v_t_date(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "date").with_text(value)
}

/// Create a `<vt:filetime>` element (`VTFileTime`).
pub fn v_t_file_time(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "filetime").with_text(value)
}

/// Create a `<vt:bool>` element (`VTBool`).
pub fn v_t_bool(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "bool").with_text(value)
}

/// Create a `<vt:cy>` element (`VTCurrency`).
pub fn v_t_currency(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "cy").with_text(value)
}

/// Create a `<vt:error>` element (`VTError`).
pub fn v_t_error(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "error").with_text(value)
}

/// Create a `<vt:vstream>` element (`VTVStreamData`).
pub fn v_t_v_stream_data(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "vstream").with_text(value)
}

/// Create a `<vt:clsid>` element (`VTClassId`).
pub fn v_t_class_id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "clsid").with_text(value)
}

/// Create a `<vt:cf>` element (`VTClipboardData`).
pub fn v_t_clipboard_data(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("vt", NAMESPACE_URI, "cf").with_text(value)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 35;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 35;
