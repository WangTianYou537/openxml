//! Auto-generated from `schemas_openxmlformats_org_officeDocument_2006_customXml.json`.
//! Target namespace: `http://schemas.openxmlformats.org/officeDocument/2006/customXml` (prefix `ds`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/customXml";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ds";

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

static ATTRS_DATA_STORE_ITEM: &[AttributeInfo] = &[
    AttributeInfo { qname: "ds:itemID", property_name: Some("ItemId"), type_name: "StringValue" },
];
static CHILDREN_DATA_STORE_ITEM: &[ChildInfo] = &[
    ChildInfo { name: "ds:CT_DatastoreSchemaRefs/ds:schemaRefs", property_name: Some("SchemaReferences") },
];
static ATTRS_SCHEMA_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "ds:uri", property_name: Some("Uri"), type_name: "StringValue" },
];
static CHILDREN_SCHEMA_REFERENCES: &[ChildInfo] = &[
    ChildInfo { name: "ds:CT_DatastoreSchemaRef/ds:schemaRef", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "DataStoreItem", local_name: "datastoreItem", prefix: "ds", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_STORE_ITEM, children: CHILDREN_DATA_STORE_ITEM },
    ElementInfo { class_name: "SchemaReference", local_name: "schemaRef", prefix: "ds", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SCHEMA_REFERENCE, children: &[] },
    ElementInfo { class_name: "SchemaReferences", local_name: "schemaRefs", prefix: "ds", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCHEMA_REFERENCES },
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

/// Create a `<ds:datastoreItem>` element (`DataStoreItem`).
pub fn data_store_item(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ds", NAMESPACE_URI, "datastoreItem").with_children(children)
}

/// Create a `<ds:schemaRef>` element (`SchemaReference`).
pub fn schema_reference() -> OpenXmlElement {
    OpenXmlElement::new("ds", NAMESPACE_URI, "schemaRef")
}

/// Create a `<ds:schemaRefs>` element (`SchemaReferences`).
pub fn schema_references(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ds", NAMESPACE_URI, "schemaRefs").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 3;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 3;
