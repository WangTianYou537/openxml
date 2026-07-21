//! Auto-generated from `schemas_microsoft_com_office_drawing_2014_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2014/main` (prefix `a16`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2014/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "a16";

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

static ATTRS_CREATION_ID: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "StringValue" },
];
static ATTRS_PREDECESSOR_DRAWING_ELEMENT_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pred", property_name: None, type_name: "StringValue" },
];
static ATTRS_CONNECTABLE_REFERENCES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":st", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":end", property_name: None, type_name: "StringValue" },
];
static ATTRS_ROW_ID_IDENTIFIER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_COL_ID_IDENTIFIER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CreationId", local_name: "creationId", prefix: "a16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CREATION_ID, children: &[] },
    ElementInfo { class_name: "PredecessorDrawingElementReference", local_name: "predDERef", prefix: "a16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PREDECESSOR_DRAWING_ELEMENT_REFERENCE, children: &[] },
    ElementInfo { class_name: "ConnectableReferences", local_name: "cxnDERefs", prefix: "a16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONNECTABLE_REFERENCES, children: &[] },
    ElementInfo { class_name: "RowIdIdentifier", local_name: "rowId", prefix: "a16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROW_ID_IDENTIFIER, children: &[] },
    ElementInfo { class_name: "ColIdIdentifier", local_name: "colId", prefix: "a16", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COL_ID_IDENTIFIER, children: &[] },
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

/// Create a `<a16:creationId>` element (`CreationId`).
pub fn creation_id() -> OpenXmlElement {
    OpenXmlElement::new("a16", NAMESPACE_URI, "creationId")
}

/// Create a `<a16:predDERef>` element (`PredecessorDrawingElementReference`).
pub fn predecessor_drawing_element_reference() -> OpenXmlElement {
    OpenXmlElement::new("a16", NAMESPACE_URI, "predDERef")
}

/// Create a `<a16:cxnDERefs>` element (`ConnectableReferences`).
pub fn connectable_references() -> OpenXmlElement {
    OpenXmlElement::new("a16", NAMESPACE_URI, "cxnDERefs")
}

/// Create a `<a16:rowId>` element (`RowIdIdentifier`).
pub fn row_id_identifier() -> OpenXmlElement {
    OpenXmlElement::new("a16", NAMESPACE_URI, "rowId")
}

/// Create a `<a16:colId>` element (`ColIdIdentifier`).
pub fn col_id_identifier() -> OpenXmlElement {
    OpenXmlElement::new("a16", NAMESPACE_URI, "colId")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 6;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 5;
