//! Auto-generated from `www_w3_org_2003_04_emma.json`.
//! Target namespace: `http://www.w3.org/2003/04/emma` (prefix `emma`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://www.w3.org/2003/04/emma";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "emma";

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

static ATTRS_DERIVED_FROM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":resource", property_name: Some("Resource"), type_name: "StringValue" },
    AttributeInfo { qname: ":composite", property_name: Some("Composite"), type_name: "BooleanValue" },
];
static ATTRS_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_LATTICE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":initial", property_name: Some("Initial"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":final", property_name: Some("Final"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:time-ref-uri", property_name: Some("TimeReference"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:time-ref-anchor-point", property_name: Some("TimeReferenceAnchorPoint"), type_name: "EnumValue" },
];
static CHILDREN_LATTICE: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_Arc/emma:arc", property_name: None },
    ChildInfo { name: "emma:CT_Node/emma:node", property_name: None },
];
static ATTRS_INTERPRETATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:tokens", property_name: Some("Tokens"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:process", property_name: Some("Process"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal", property_name: Some("Signal"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal-size", property_name: Some("SignalSize"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:media-type", property_name: Some("MediaType"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:confidence", property_name: Some("Confidence"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:source", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:start", property_name: Some("Start"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:end", property_name: Some("End"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:time-ref-uri", property_name: Some("TimeReference"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:time-ref-anchor-point", property_name: Some("TimeReferenceAnchorPoint"), type_name: "EnumValue" },
    AttributeInfo { qname: "emma:offset-to-start", property_name: Some("OffsetToStart"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:duration", property_name: Some("Duration"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:medium", property_name: Some("Medium"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:mode", property_name: Some("Mode"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:function", property_name: Some("Function"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:verbal", property_name: Some("Verbal"), type_name: "BooleanValue" },
    AttributeInfo { qname: "emma:cost", property_name: Some("Cost"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:grammar-ref", property_name: Some("GrammarRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:endpoint-info-ref", property_name: Some("EndpointInfoRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:model-ref", property_name: Some("ModelRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:dialog-turn", property_name: Some("DialogTurn"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:no-input", property_name: Some("NoInput"), type_name: "BooleanValue" },
    AttributeInfo { qname: "emma:uninterpreted", property_name: Some("Uninterpreted"), type_name: "BooleanValue" },
];
static CHILDREN_INTERPRETATION: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_DerivedFrom/emma:derived-from", property_name: None },
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
    ChildInfo { name: "emma:CT_Lattice/emma:lattice", property_name: None },
    ChildInfo { name: "emma:CT_Literal/emma:literal", property_name: None },
    ChildInfo { name: "msink:CT_CtxNode/msink:context", property_name: None },
];
static ATTRS_ONE_OF: &[AttributeInfo] = &[
    AttributeInfo { qname: ":disjunction-type", property_name: Some("DisjunctionType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:tokens", property_name: Some("Tokens"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:process", property_name: Some("Process"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal", property_name: Some("Signal"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal-size", property_name: Some("SignalSize"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:media-type", property_name: Some("MediaType"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:confidence", property_name: Some("Confidence"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:source", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:start", property_name: Some("Start"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:end", property_name: Some("End"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:time-ref-uri", property_name: Some("TimeReference"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:time-ref-anchor-point", property_name: Some("TimeReferenceAnchorPoint"), type_name: "EnumValue" },
    AttributeInfo { qname: "emma:offset-to-start", property_name: Some("OffsetToStart"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:duration", property_name: Some("Duration"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:medium", property_name: Some("Medium"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:mode", property_name: Some("Mode"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:function", property_name: Some("Function"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:verbal", property_name: Some("Verbal"), type_name: "BooleanValue" },
    AttributeInfo { qname: "emma:cost", property_name: Some("Cost"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:grammar-ref", property_name: Some("GrammarRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:endpoint-info-ref", property_name: Some("EndpointInfoRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:model-ref", property_name: Some("ModelRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:dialog-turn", property_name: Some("DialogTurn"), type_name: "StringValue" },
];
static CHILDREN_ONE_OF: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_DerivedFrom/emma:derived-from", property_name: None },
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
    ChildInfo { name: "emma:CT_Interpretation/emma:interpretation", property_name: None },
    ChildInfo { name: "emma:CT_OneOf/emma:one-of", property_name: None },
    ChildInfo { name: "emma:CT_Group/emma:group", property_name: None },
    ChildInfo { name: "emma:CT_Sequence/emma:sequence", property_name: None },
];
static ATTRS_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:tokens", property_name: Some("Tokens"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:process", property_name: Some("Process"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal", property_name: Some("Signal"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal-size", property_name: Some("SignalSize"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:media-type", property_name: Some("MediaType"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:confidence", property_name: Some("Confidence"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:source", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:start", property_name: Some("Start"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:end", property_name: Some("End"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:time-ref-uri", property_name: Some("TimeReference"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:time-ref-anchor-point", property_name: Some("TimeReferenceAnchorPoint"), type_name: "EnumValue" },
    AttributeInfo { qname: "emma:offset-to-start", property_name: Some("OffsetToStart"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:duration", property_name: Some("Duration"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:medium", property_name: Some("Medium"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:mode", property_name: Some("Mode"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:function", property_name: Some("Function"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:verbal", property_name: Some("Verbal"), type_name: "BooleanValue" },
    AttributeInfo { qname: "emma:cost", property_name: Some("Cost"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:grammar-ref", property_name: Some("GrammarRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:endpoint-info-ref", property_name: Some("EndpointInfoRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:model-ref", property_name: Some("ModelRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:dialog-turn", property_name: Some("DialogTurn"), type_name: "StringValue" },
];
static CHILDREN_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_DerivedFrom/emma:derived-from", property_name: None },
    ChildInfo { name: "emma:CT_GroupInfo/emma:group-info", property_name: None },
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
    ChildInfo { name: "emma:CT_Interpretation/emma:interpretation", property_name: None },
    ChildInfo { name: "emma:CT_OneOf/emma:one-of", property_name: None },
    ChildInfo { name: "emma:CT_Group/emma:group", property_name: None },
    ChildInfo { name: "emma:CT_Sequence/emma:sequence", property_name: None },
];
static ATTRS_SEQUENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:tokens", property_name: Some("Tokens"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:process", property_name: Some("Process"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal", property_name: Some("Signal"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:signal-size", property_name: Some("SignalSize"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:media-type", property_name: Some("MediaType"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:confidence", property_name: Some("Confidence"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:source", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:start", property_name: Some("Start"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:end", property_name: Some("End"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:time-ref-uri", property_name: Some("TimeReference"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:time-ref-anchor-point", property_name: Some("TimeReferenceAnchorPoint"), type_name: "EnumValue" },
    AttributeInfo { qname: "emma:offset-to-start", property_name: Some("OffsetToStart"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:duration", property_name: Some("Duration"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:medium", property_name: Some("Medium"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:mode", property_name: Some("Mode"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:function", property_name: Some("Function"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:verbal", property_name: Some("Verbal"), type_name: "BooleanValue" },
    AttributeInfo { qname: "emma:cost", property_name: Some("Cost"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:grammar-ref", property_name: Some("GrammarRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:endpoint-info-ref", property_name: Some("EndpointInfoRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:model-ref", property_name: Some("ModelRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:dialog-turn", property_name: Some("DialogTurn"), type_name: "StringValue" },
];
static CHILDREN_SEQUENCE: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_DerivedFrom/emma:derived-from", property_name: None },
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
    ChildInfo { name: "emma:CT_Interpretation/emma:interpretation", property_name: None },
    ChildInfo { name: "emma:CT_OneOf/emma:one-of", property_name: None },
    ChildInfo { name: "emma:CT_Group/emma:group", property_name: None },
    ChildInfo { name: "emma:CT_Sequence/emma:sequence", property_name: None },
];
static ATTRS_GROUP_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];
static CHILDREN_DERIVATION: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_Interpretation/emma:interpretation", property_name: None },
    ChildInfo { name: "emma:CT_OneOf/emma:one-of", property_name: None },
    ChildInfo { name: "emma:CT_Sequence/emma:sequence", property_name: None },
    ChildInfo { name: "emma:CT_Group/emma:group", property_name: None },
];
static ATTRS_GRAMMAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];
static ATTRS_MODEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];
static ATTRS_END_POINT_INFO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_END_POINT_INFO: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_EndPoint/emma:endpoint", property_name: None },
];
static ATTRS_END_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:endpoint-role", property_name: Some("EndpointRole"), type_name: "EnumValue" },
    AttributeInfo { qname: "emma:endpoint-address", property_name: Some("EndPointAddress"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:message-id", property_name: Some("MessageId"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:port-num", property_name: Some("PortNumber"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:port-type", property_name: Some("PortType"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:endpoint-pair-ref", property_name: Some("EndpointPairRef"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:service-name", property_name: Some("ServiceName"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:media-type", property_name: Some("MediaType"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:medium", property_name: Some("Medium"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:mode", property_name: Some("Mode"), type_name: "ListValue" },
];
static ATTRS_NODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":node-number", property_name: Some("NodeNumber"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:confidence", property_name: Some("Confidence"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:cost", property_name: Some("Cost"), type_name: "DecimalValue" },
];
static CHILDREN_NODE: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
];
static ATTRS_ARC: &[AttributeInfo] = &[
    AttributeInfo { qname: ":from", property_name: Some("From"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":to", property_name: Some("To"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:start", property_name: Some("Start"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:end", property_name: Some("End"), type_name: "UInt64Value" },
    AttributeInfo { qname: "emma:offset-to-start", property_name: Some("OffsetToStart"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:duration", property_name: Some("Duration"), type_name: "IntegerValue" },
    AttributeInfo { qname: "emma:confidence", property_name: Some("Confidence"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:cost", property_name: Some("Cost"), type_name: "DecimalValue" },
    AttributeInfo { qname: "emma:lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: "emma:medium", property_name: Some("Medium"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:mode", property_name: Some("Mode"), type_name: "ListValue" },
    AttributeInfo { qname: "emma:source", property_name: Some("Source"), type_name: "StringValue" },
];
static CHILDREN_ARC: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
];
static ATTRS_EMMA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":version", property_name: Some("Version"), type_name: "StringValue" },
];
static CHILDREN_EMMA: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_Derivation/emma:derivation", property_name: None },
    ChildInfo { name: "emma:CT_Grammar/emma:grammar", property_name: None },
    ChildInfo { name: "emma:CT_Model/emma:model", property_name: None },
    ChildInfo { name: "emma:CT_EndPointInfo/emma:endpoint-info", property_name: None },
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
    ChildInfo { name: "emma:CT_Interpretation/emma:interpretation", property_name: None },
    ChildInfo { name: "emma:CT_OneOf/emma:one-of", property_name: None },
    ChildInfo { name: "emma:CT_Group/emma:group", property_name: None },
    ChildInfo { name: "emma:CT_Sequence/emma:sequence", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "DerivedFrom", local_name: "derived-from", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DERIVED_FROM, children: &[] },
    ElementInfo { class_name: "Info", local_name: "info", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_INFO, children: &[] },
    ElementInfo { class_name: "Lattice", local_name: "lattice", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LATTICE, children: CHILDREN_LATTICE },
    ElementInfo { class_name: "Literal", local_name: "literal", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Interpretation", local_name: "interpretation", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_INTERPRETATION, children: CHILDREN_INTERPRETATION },
    ElementInfo { class_name: "OneOf", local_name: "one-of", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ONE_OF, children: CHILDREN_ONE_OF },
    ElementInfo { class_name: "Group", local_name: "group", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP, children: CHILDREN_GROUP },
    ElementInfo { class_name: "Sequence", local_name: "sequence", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SEQUENCE, children: CHILDREN_SEQUENCE },
    ElementInfo { class_name: "GroupInfo", local_name: "group-info", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_INFO, children: &[] },
    ElementInfo { class_name: "Derivation", local_name: "derivation", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DERIVATION },
    ElementInfo { class_name: "Grammar", local_name: "grammar", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GRAMMAR, children: &[] },
    ElementInfo { class_name: "Model", local_name: "model", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MODEL, children: &[] },
    ElementInfo { class_name: "EndPointInfo", local_name: "endpoint-info", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_END_POINT_INFO, children: CHILDREN_END_POINT_INFO },
    ElementInfo { class_name: "EndPoint", local_name: "endpoint", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_END_POINT, children: &[] },
    ElementInfo { class_name: "Node", local_name: "node", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NODE, children: CHILDREN_NODE },
    ElementInfo { class_name: "Arc", local_name: "arc", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ARC, children: CHILDREN_ARC },
    ElementInfo { class_name: "Emma", local_name: "emma", prefix: "emma", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EMMA, children: CHILDREN_EMMA },
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

/// Create a `<emma:derived-from>` element (`DerivedFrom`).
pub fn derived_from() -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "derived-from")
}

/// Create a `<emma:info>` element (`Info`).
pub fn info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "info").with_children(children)
}

/// Create a `<emma:lattice>` element (`Lattice`).
pub fn lattice(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "lattice").with_children(children)
}

/// Create a `<emma:literal>` element (`Literal`).
pub fn literal(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "literal").with_text(value)
}

/// Create a `<emma:interpretation>` element (`Interpretation`).
pub fn interpretation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "interpretation").with_children(children)
}

/// Create a `<emma:one-of>` element (`OneOf`).
pub fn one_of(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "one-of").with_children(children)
}

/// Create a `<emma:group>` element (`Group`).
pub fn group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "group").with_children(children)
}

/// Create a `<emma:sequence>` element (`Sequence`).
pub fn sequence(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "sequence").with_children(children)
}

/// Create a `<emma:group-info>` element (`GroupInfo`).
pub fn group_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "group-info").with_children(children)
}

/// Create a `<emma:derivation>` element (`Derivation`).
pub fn derivation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "derivation").with_children(children)
}

/// Create a `<emma:grammar>` element (`Grammar`).
pub fn grammar() -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "grammar")
}

/// Create a `<emma:model>` element (`Model`).
pub fn model(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "model").with_children(children)
}

/// Create a `<emma:endpoint-info>` element (`EndPointInfo`).
pub fn end_point_info(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "endpoint-info").with_children(children)
}

/// Create a `<emma:endpoint>` element (`EndPoint`).
pub fn end_point(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "endpoint").with_children(children)
}

/// Create a `<emma:node>` element (`Node`).
pub fn node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "node").with_children(children)
}

/// Create a `<emma:arc>` element (`Arc`).
pub fn arc(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "arc").with_children(children)
}

/// Create a `<emma:emma>` element (`Emma`).
pub fn emma(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("emma", NAMESPACE_URI, "emma").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 17;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 17;
