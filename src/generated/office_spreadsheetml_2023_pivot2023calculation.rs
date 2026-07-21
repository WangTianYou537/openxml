//! Auto-generated from `schemas_microsoft_com_office_spreadsheetml_2023_pivot2023Calculation.json`.
//! Target namespace: `http://schemas.microsoft.com/office/spreadsheetml/2023/pivot2023Calculation` (prefix `xlpcalc`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/spreadsheetml/2023/pivot2023Calculation";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "xlpcalc";

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

static ATTRS_AGGREGATION_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":aggregationType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":sourceField", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_FEATURE_SUPPORT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":featureName", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PIVOT_FIELD_SUBTOTALS: &[ChildInfo] = &[
    ChildInfo { name: "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotal", property_name: None },
];
static CHILDREN_PIVOT_AREA_REFERENCE_SUBTOTALS: &[ChildInfo] = &[
    ChildInfo { name: "xlpcalc:CT_PivotSubtotalType/xlpcalc:subtotal", property_name: None },
];
static CHILDREN_PIVOT_TABLE_SUBTOTAL_LINE_ITEMS: &[ChildInfo] = &[
    ChildInfo { name: "xlpcalc:CT_PivotItemSubtotal/xlpcalc:subtotalLineItem", property_name: None },
];
static ATTRS_SUBTOTAL_PIVOT_ITEM_SUBTOTAL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":subtotalType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":itemLocation", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_SUBTOTAL_LINE_ITEM_PIVOT_ITEM_SUBTOTAL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":subtotalType", property_name: None, type_name: "EnumValue" },
    AttributeInfo { qname: ":itemLocation", property_name: None, type_name: "UInt32Value" },
];
static ATTRS_PIVOT_SUBTOTAL_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":subtotalType", property_name: None, type_name: "EnumValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "AggregationInfo", local_name: "aggregationInfo", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AGGREGATION_INFO, children: &[] },
    ElementInfo { class_name: "FeatureSupport", local_name: "featureSupportInfo", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FEATURE_SUPPORT, children: &[] },
    ElementInfo { class_name: "PivotFieldSubtotals", local_name: "pivotFieldSubtotals", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_FIELD_SUBTOTALS },
    ElementInfo { class_name: "PivotAreaReferenceSubtotals", local_name: "pivotAreaReferenceSubtotals", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_AREA_REFERENCE_SUBTOTALS },
    ElementInfo { class_name: "PivotTableSubtotalLineItems", local_name: "pivotFieldSubtotalLineItems", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PIVOT_TABLE_SUBTOTAL_LINE_ITEMS },
    ElementInfo { class_name: "SubtotalPivotItemSubtotal", local_name: "subtotal", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SUBTOTAL_PIVOT_ITEM_SUBTOTAL, children: &[] },
    ElementInfo { class_name: "SubtotalLineItemPivotItemSubtotal", local_name: "subtotalLineItem", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SUBTOTAL_LINE_ITEM_PIVOT_ITEM_SUBTOTAL, children: &[] },
    ElementInfo { class_name: "PivotSubtotalType", local_name: "subtotal", prefix: "xlpcalc", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PIVOT_SUBTOTAL_TYPE, children: &[] },
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

/// Create a `<xlpcalc:aggregationInfo>` element (`AggregationInfo`).
pub fn aggregation_info() -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "aggregationInfo")
}

/// Create a `<xlpcalc:featureSupportInfo>` element (`FeatureSupport`).
pub fn feature_support() -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "featureSupportInfo")
}

/// Create a `<xlpcalc:pivotFieldSubtotals>` element (`PivotFieldSubtotals`).
pub fn pivot_field_subtotals(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "pivotFieldSubtotals").with_children(children)
}

/// Create a `<xlpcalc:pivotAreaReferenceSubtotals>` element (`PivotAreaReferenceSubtotals`).
pub fn pivot_area_reference_subtotals(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "pivotAreaReferenceSubtotals").with_children(children)
}

/// Create a `<xlpcalc:pivotFieldSubtotalLineItems>` element (`PivotTableSubtotalLineItems`).
pub fn pivot_table_subtotal_line_items(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "pivotFieldSubtotalLineItems").with_children(children)
}

/// Create a `<xlpcalc:subtotal>` element (`SubtotalPivotItemSubtotal`).
pub fn subtotal_pivot_item_subtotal() -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "subtotal")
}

/// Create a `<xlpcalc:subtotalLineItem>` element (`SubtotalLineItemPivotItemSubtotal`).
pub fn subtotal_line_item_pivot_item_subtotal() -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "subtotalLineItem")
}

/// Create a `<xlpcalc:subtotal>` element (`PivotSubtotalType`).
pub fn pivot_subtotal_type() -> OpenXmlElement {
    OpenXmlElement::new("xlpcalc", NAMESPACE_URI, "subtotal")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 9;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 8;
