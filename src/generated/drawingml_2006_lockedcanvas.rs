//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_lockedCanvas.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas` (prefix `lc`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "lc";

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

static CHILDREN_LOCKED_CANVAS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GvmlGroupShapeNonVisual/a:nvGrpSpPr", property_name: Some("NonVisualGroupShapeProperties") },
    ChildInfo { name: "a:CT_GroupShapeProperties/a:grpSpPr", property_name: Some("VisualGroupShapeProperties") },
    ChildInfo { name: "a:CT_GvmlTextShape/a:txSp", property_name: None },
    ChildInfo { name: "a:CT_GvmlShape/a:sp", property_name: None },
    ChildInfo { name: "a:CT_GvmlConnector/a:cxnSp", property_name: None },
    ChildInfo { name: "a:CT_GvmlPicture/a:pic", property_name: None },
    ChildInfo { name: "a14:CT_GvmlContentPart/a14:contentPart", property_name: None },
    ChildInfo { name: "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame", property_name: None },
    ChildInfo { name: "a:CT_GvmlGroupShape/a:grpSp", property_name: None },
    ChildInfo { name: "a:CT_GvmlGroupShapeExtensionList/a:extLst", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "LockedCanvas", local_name: "lockedCanvas", prefix: "lc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LOCKED_CANVAS },
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

/// Create a `<lc:lockedCanvas>` element (`LockedCanvas`).
pub fn locked_canvas(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("lc", NAMESPACE_URI, "lockedCanvas").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 1;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 1;
