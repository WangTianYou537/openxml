//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2016_pivotdefaultlayout.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2016/pivotdefaultlayout` (prefix `xpdl`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2016/pivotdefaultlayout";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xpdl";

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

static ATTRS_PIVOT_TABLE_DEFINITION16: &[AttributeInfo] = &[
    AttributeInfo { qname: ":EnabledSubtotalsDefault", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":SubtotalsOnTopDefault", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":InsertBlankRowDefault", property_name: None, type_name: "BooleanValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "PivotTableDefinition16", local_name: "pivotTableDefinition16", prefix: "xpdl", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_TABLE_DEFINITION16, children: &[] },
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

/// Create a `<xpdl:pivotTableDefinition16>` element (`PivotTableDefinition16`).
pub fn pivot_table_definition16() -> OpenXmlElement {
    OpenXmlElement::new("xpdl", NAMESPACE_URI, "pivotTableDefinition16")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 1;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 1;
