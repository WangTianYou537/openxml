//! Auto-generated from `schemas_openxmlformats_org_officeDocument_2006_custom-properties.json`.
//! Target namespace: `http://schemas.openxmlformats.org/officeDocument/2006/custom-properties` (prefix `op`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "op";

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

static CHILDREN_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "op:CT_Property/op:property", property_name: None },
];
static ATTRS_CUSTOM_DOCUMENT_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fmtid", property_name: Some("FormatId"), type_name: "StringValue" },
    AttributeInfo { qname: ":pid", property_name: Some("PropertyId"), type_name: "Int32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":linkTarget", property_name: Some("LinkTarget"), type_name: "StringValue" },
];
static CHILDREN_CUSTOM_DOCUMENT_PROPERTY: &[ChildInfo] = &[
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Properties", local_name: "Properties", prefix: "op", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PROPERTIES },
    ElementInfo { class_name: "CustomDocumentProperty", local_name: "property", prefix: "op", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CUSTOM_DOCUMENT_PROPERTY, children: CHILDREN_CUSTOM_DOCUMENT_PROPERTY },
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

/// Create a `<op:Properties>` element (`Properties`).
pub fn properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("op", NAMESPACE_URI, "Properties").with_children(children)
}

/// Create a `<op:property>` element (`CustomDocumentProperty`).
pub fn custom_document_property(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("op", NAMESPACE_URI, "property").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 2;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 2;
