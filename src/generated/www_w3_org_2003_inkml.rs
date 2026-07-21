//! Auto-generated from `www_w3_org_2003_InkML.json`.
//! Target namespace: `http://www.w3.org/2003/InkML` (prefix `inkml`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://www.w3.org/2003/InkML";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "inkml";

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

static ATTRS_INK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":documentID", property_name: Some("DocumentId"), type_name: "StringValue" },
];
static CHILDREN_INK: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Annotation/inkml:annotation", property_name: None },
    ChildInfo { name: "inkml:CT_AnnotationXML/inkml:annotationXML", property_name: None },
    ChildInfo { name: "inkml:CT_Definitions/inkml:definitions", property_name: None },
    ChildInfo { name: "inkml:CT_Context/inkml:context", property_name: None },
    ChildInfo { name: "inkml:CT_Trace/inkml:trace", property_name: None },
    ChildInfo { name: "inkml:CT_TraceGroup/inkml:traceGroup", property_name: None },
    ChildInfo { name: "inkml:CT_TraceView/inkml:traceView", property_name: None },
];
static ATTRS_BIND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":source", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: ":target", property_name: Some("Target"), type_name: "StringValue" },
    AttributeInfo { qname: ":column", property_name: Some("Column"), type_name: "StringValue" },
    AttributeInfo { qname: ":variable", property_name: Some("Variable"), type_name: "StringValue" },
];
static ATTRS_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":apply", property_name: Some("Apply"), type_name: "EnumValue" },
    AttributeInfo { qname: ":interpolation", property_name: Some("Interpolation"), type_name: "EnumValue" },
];
static ATTRS_MATRIX: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
];
static ATTRS_MAPPING: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":mappingRef", property_name: Some("MappingRef"), type_name: "StringValue" },
];
static CHILDREN_MAPPING: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Bind/inkml:bind", property_name: None },
    ChildInfo { name: "inkml:CT_Table/inkml:table", property_name: None },
    ChildInfo { name: "inkml:CT_Matrix/inkml:matrix", property_name: None },
    ChildInfo { name: "inkml:CT_Mapping/inkml:mapping", property_name: None },
];
static ATTRS_CHANNEL: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":default", property_name: Some("Default"), type_name: "StringValue" },
    AttributeInfo { qname: ":min", property_name: Some("Min"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":max", property_name: Some("Max"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":orientation", property_name: Some("Orientation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":respectTo", property_name: Some("RespectTo"), type_name: "StringValue" },
    AttributeInfo { qname: ":units", property_name: Some("Units"), type_name: "StringValue" },
];
static CHILDREN_CHANNEL: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Mapping/inkml:mapping", property_name: None },
];
static CHILDREN_INTERMITTENT_CHANNELS: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Channel/inkml:channel", property_name: None },
];
static ATTRS_CHANNEL_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":channel", property_name: Some("Channel"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":value", property_name: Some("Value"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":units", property_name: Some("Units"), type_name: "StringValue" },
];
static ATTRS_TRACE_FORMAT: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
];
static CHILDREN_TRACE_FORMAT: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Channel/inkml:channel", property_name: None },
    ChildInfo { name: "inkml:CT_IntermittentChannels/inkml:intermittentChannels", property_name: None },
];
static ATTRS_SAMPLE_RATE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniform", property_name: Some("Uniform"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":value", property_name: Some("Value"), type_name: "DecimalValue" },
];
static ATTRS_LATENCY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":value", property_name: Some("Value"), type_name: "DecimalValue" },
];
static ATTRS_ACTIVE_AREA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "StringValue" },
    AttributeInfo { qname: ":height", property_name: Some("Height"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":width", property_name: Some("Width"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":units", property_name: Some("Units"), type_name: "StringValue" },
];
static ATTRS_SOURCE_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":value", property_name: Some("Value"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":units", property_name: Some("Units"), type_name: "StringValue" },
];
static CHILDREN_CHANNEL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_ChannelProperty/inkml:channelProperty", property_name: None },
];
static ATTRS_ANNOTATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":encoding", property_name: Some("Encoding"), type_name: "StringValue" },
];
static ATTRS_ANNOTATION_XML: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":encoding", property_name: Some("Encoding"), type_name: "StringValue" },
    AttributeInfo { qname: ":href", property_name: Some("Href"), type_name: "StringValue" },
];
static CHILDREN_ANNOTATION_XML: &[ChildInfo] = &[
    ChildInfo { name: "emma:CT_Emma/emma:emma", property_name: Some("Emma") },
];
static ATTRS_BRUSH_PROPERTY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":value", property_name: Some("Value"), type_name: "StringValue" },
    AttributeInfo { qname: ":units", property_name: Some("Units"), type_name: "StringValue" },
];
static CHILDREN_BRUSH_PROPERTY: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Annotation/inkml:annotation", property_name: None },
    ChildInfo { name: "inkml:CT_AnnotationXML/inkml:annotationXML", property_name: None },
];
static ATTRS_CANVAS: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":traceFormatRef", property_name: Some("TraceFormatRef"), type_name: "StringValue" },
];
static CHILDREN_CANVAS: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_TraceFormat/inkml:traceFormat", property_name: Some("TraceFormat") },
];
static ATTRS_CANVAS_TRANSFORM: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":invertible", property_name: Some("Invertible"), type_name: "BooleanValue" },
];
static CHILDREN_CANVAS_TRANSFORM: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Mapping/inkml:mapping", property_name: None },
];
static ATTRS_INK_SOURCE: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":manufacturer", property_name: Some("Manufacturer"), type_name: "StringValue" },
    AttributeInfo { qname: ":model", property_name: Some("Model"), type_name: "StringValue" },
    AttributeInfo { qname: ":serialNo", property_name: Some("SerialNo"), type_name: "StringValue" },
    AttributeInfo { qname: ":specificationRef", property_name: Some("SpecificationRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":description", property_name: Some("Description"), type_name: "StringValue" },
];
static CHILDREN_INK_SOURCE: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_TraceFormat/inkml:traceFormat", property_name: Some("TraceFormat") },
    ChildInfo { name: "inkml:CT_SampleRate/inkml:sampleRate", property_name: Some("SampleRate") },
    ChildInfo { name: "inkml:CT_Latency/inkml:latency", property_name: Some("Latency") },
    ChildInfo { name: "inkml:CT_ActiveArea/inkml:activeArea", property_name: Some("ActiveArea") },
    ChildInfo { name: "inkml:CT_SrcProperty/inkml:srcProperty", property_name: None },
    ChildInfo { name: "inkml:CT_ChannelProperties/inkml:channelProperties", property_name: None },
];
static ATTRS_BRUSH: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":brushRef", property_name: Some("BrushRef"), type_name: "StringValue" },
];
static CHILDREN_BRUSH: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Annotation/inkml:annotation", property_name: None },
    ChildInfo { name: "inkml:CT_AnnotationXML/inkml:annotationXML", property_name: None },
    ChildInfo { name: "inkml:CT_BrushProperty/inkml:brushProperty", property_name: None },
];
static ATTRS_TIMESTAMP: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":timestampRef", property_name: Some("TimestampRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":timeString", property_name: Some("TimeString"), type_name: "DateTimeValue" },
    AttributeInfo { qname: ":timeOffset", property_name: Some("TimeOffset"), type_name: "DecimalValue" },
];
static ATTRS_TRACE: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":continuation", property_name: Some("Continuation"), type_name: "EnumValue" },
    AttributeInfo { qname: ":priorRef", property_name: Some("PriorRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":contextRef", property_name: Some("ContextRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":brushRef", property_name: Some("BrushRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":duration", property_name: Some("Duration"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":timeOffset", property_name: Some("TimeOffset"), type_name: "DecimalValue" },
];
static ATTRS_TRACE_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":contextRef", property_name: Some("ContextRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":brushRef", property_name: Some("BrushRef"), type_name: "StringValue" },
];
static CHILDREN_TRACE_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Annotation/inkml:annotation", property_name: None },
    ChildInfo { name: "inkml:CT_AnnotationXML/inkml:annotationXML", property_name: None },
    ChildInfo { name: "inkml:CT_Trace/inkml:trace", property_name: None },
    ChildInfo { name: "inkml:CT_TraceGroup/inkml:traceGroup", property_name: None },
];
static ATTRS_TRACE_VIEW: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":contextRef", property_name: Some("ContextRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":traceDataRef", property_name: Some("TraceDataRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":from", property_name: Some("From"), type_name: "StringValue" },
    AttributeInfo { qname: ":to", property_name: Some("To"), type_name: "StringValue" },
];
static CHILDREN_TRACE_VIEW: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Annotation/inkml:annotation", property_name: None },
    ChildInfo { name: "inkml:CT_AnnotationXML/inkml:annotationXML", property_name: None },
    ChildInfo { name: "inkml:CT_TraceView/inkml:traceView", property_name: None },
];
static ATTRS_CONTEXT: &[AttributeInfo] = &[
    AttributeInfo { qname: "xml:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":contextRef", property_name: Some("ContextRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":canvasRef", property_name: Some("CanvasRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":canvasTransformRef", property_name: Some("CanvasTransformRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":traceFormatRef", property_name: Some("TraceFromatRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":inkSourceRef", property_name: Some("InkSourceRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":brushRef", property_name: Some("BrushRef"), type_name: "StringValue" },
    AttributeInfo { qname: ":timestampRef", property_name: Some("TimestampRef"), type_name: "StringValue" },
];
static CHILDREN_CONTEXT: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Canvas/inkml:canvas", property_name: Some("Canvas") },
    ChildInfo { name: "inkml:CT_CanvasTransform/inkml:canvasTransform", property_name: Some("CanvasTransform") },
    ChildInfo { name: "inkml:CT_TraceFormat/inkml:traceFormat", property_name: Some("TraceFormat") },
    ChildInfo { name: "inkml:CT_InkSource/inkml:inkSource", property_name: Some("InkSource") },
    ChildInfo { name: "inkml:CT_Brush/inkml:brush", property_name: Some("Brush") },
    ChildInfo { name: "inkml:CT_Timestamp/inkml:timestamp", property_name: Some("Timestamp") },
];
static CHILDREN_DEFINITIONS: &[ChildInfo] = &[
    ChildInfo { name: "inkml:CT_Brush/inkml:brush", property_name: None },
    ChildInfo { name: "inkml:CT_Canvas/inkml:canvas", property_name: None },
    ChildInfo { name: "inkml:CT_CanvasTransform/inkml:canvasTransform", property_name: None },
    ChildInfo { name: "inkml:CT_Context/inkml:context", property_name: None },
    ChildInfo { name: "inkml:CT_InkSource/inkml:inkSource", property_name: None },
    ChildInfo { name: "inkml:CT_Mapping/inkml:mapping", property_name: None },
    ChildInfo { name: "inkml:CT_Timestamp/inkml:timestamp", property_name: None },
    ChildInfo { name: "inkml:CT_Trace/inkml:trace", property_name: None },
    ChildInfo { name: "inkml:CT_TraceFormat/inkml:traceFormat", property_name: None },
    ChildInfo { name: "inkml:CT_TraceGroup/inkml:traceGroup", property_name: None },
    ChildInfo { name: "inkml:CT_TraceView/inkml:traceView", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Ink", local_name: "ink", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_INK, children: CHILDREN_INK },
    ElementInfo { class_name: "Bind", local_name: "bind", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BIND, children: &[] },
    ElementInfo { class_name: "Table", local_name: "table", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_TABLE, children: &[] },
    ElementInfo { class_name: "Matrix", local_name: "matrix", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_MATRIX, children: &[] },
    ElementInfo { class_name: "Mapping", local_name: "mapping", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MAPPING, children: CHILDREN_MAPPING },
    ElementInfo { class_name: "Channel", local_name: "channel", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHANNEL, children: CHILDREN_CHANNEL },
    ElementInfo { class_name: "IntermittentChannels", local_name: "intermittentChannels", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_INTERMITTENT_CHANNELS },
    ElementInfo { class_name: "ChannelProperty", local_name: "channelProperty", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHANNEL_PROPERTY, children: &[] },
    ElementInfo { class_name: "TraceFormat", local_name: "traceFormat", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRACE_FORMAT, children: CHILDREN_TRACE_FORMAT },
    ElementInfo { class_name: "SampleRate", local_name: "sampleRate", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SAMPLE_RATE, children: &[] },
    ElementInfo { class_name: "Latency", local_name: "latency", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LATENCY, children: &[] },
    ElementInfo { class_name: "ActiveArea", local_name: "activeArea", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ACTIVE_AREA, children: &[] },
    ElementInfo { class_name: "SourceProperty", local_name: "srcProperty", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SOURCE_PROPERTY, children: &[] },
    ElementInfo { class_name: "ChannelProperties", local_name: "channelProperties", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CHANNEL_PROPERTIES },
    ElementInfo { class_name: "Annotation", local_name: "annotation", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_ANNOTATION, children: &[] },
    ElementInfo { class_name: "AnnotationXml", local_name: "annotationXML", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ANNOTATION_XML, children: CHILDREN_ANNOTATION_XML },
    ElementInfo { class_name: "BrushProperty", local_name: "brushProperty", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BRUSH_PROPERTY, children: CHILDREN_BRUSH_PROPERTY },
    ElementInfo { class_name: "Canvas", local_name: "canvas", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CANVAS, children: CHILDREN_CANVAS },
    ElementInfo { class_name: "CanvasTransform", local_name: "canvasTransform", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CANVAS_TRANSFORM, children: CHILDREN_CANVAS_TRANSFORM },
    ElementInfo { class_name: "InkSource", local_name: "inkSource", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_INK_SOURCE, children: CHILDREN_INK_SOURCE },
    ElementInfo { class_name: "Brush", local_name: "brush", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BRUSH, children: CHILDREN_BRUSH },
    ElementInfo { class_name: "Timestamp", local_name: "timestamp", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TIMESTAMP, children: &[] },
    ElementInfo { class_name: "Trace", local_name: "trace", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: ATTRS_TRACE, children: &[] },
    ElementInfo { class_name: "TraceGroup", local_name: "traceGroup", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRACE_GROUP, children: CHILDREN_TRACE_GROUP },
    ElementInfo { class_name: "TraceView", local_name: "traceView", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRACE_VIEW, children: CHILDREN_TRACE_VIEW },
    ElementInfo { class_name: "Context", local_name: "context", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTEXT, children: CHILDREN_CONTEXT },
    ElementInfo { class_name: "Definitions", local_name: "definitions", prefix: "inkml", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DEFINITIONS },
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

/// Create a `<inkml:ink>` element (`Ink`).
pub fn ink(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "ink").with_children(children)
}

/// Create a `<inkml:bind>` element (`Bind`).
pub fn bind() -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "bind")
}

/// Create a `<inkml:table>` element (`Table`).
pub fn table(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "table").with_text(value)
}

/// Create a `<inkml:matrix>` element (`Matrix`).
pub fn matrix(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "matrix").with_text(value)
}

/// Create a `<inkml:mapping>` element (`Mapping`).
pub fn mapping(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "mapping").with_children(children)
}

/// Create a `<inkml:channel>` element (`Channel`).
pub fn channel(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "channel").with_children(children)
}

/// Create a `<inkml:intermittentChannels>` element (`IntermittentChannels`).
pub fn intermittent_channels(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "intermittentChannels").with_children(children)
}

/// Create a `<inkml:channelProperty>` element (`ChannelProperty`).
pub fn channel_property() -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "channelProperty")
}

/// Create a `<inkml:traceFormat>` element (`TraceFormat`).
pub fn trace_format(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "traceFormat").with_children(children)
}

/// Create a `<inkml:sampleRate>` element (`SampleRate`).
pub fn sample_rate() -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "sampleRate")
}

/// Create a `<inkml:latency>` element (`Latency`).
pub fn latency() -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "latency")
}

/// Create a `<inkml:activeArea>` element (`ActiveArea`).
pub fn active_area() -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "activeArea")
}

/// Create a `<inkml:srcProperty>` element (`SourceProperty`).
pub fn source_property() -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "srcProperty")
}

/// Create a `<inkml:channelProperties>` element (`ChannelProperties`).
pub fn channel_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "channelProperties").with_children(children)
}

/// Create a `<inkml:annotation>` element (`Annotation`).
pub fn annotation(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "annotation").with_text(value)
}

/// Create a `<inkml:annotationXML>` element (`AnnotationXml`).
pub fn annotation_xml(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "annotationXML").with_children(children)
}

/// Create a `<inkml:brushProperty>` element (`BrushProperty`).
pub fn brush_property(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "brushProperty").with_children(children)
}

/// Create a `<inkml:canvas>` element (`Canvas`).
pub fn canvas(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "canvas").with_children(children)
}

/// Create a `<inkml:canvasTransform>` element (`CanvasTransform`).
pub fn canvas_transform(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "canvasTransform").with_children(children)
}

/// Create a `<inkml:inkSource>` element (`InkSource`).
pub fn ink_source(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "inkSource").with_children(children)
}

/// Create a `<inkml:brush>` element (`Brush`).
pub fn brush(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "brush").with_children(children)
}

/// Create a `<inkml:timestamp>` element (`Timestamp`).
pub fn timestamp() -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "timestamp")
}

/// Create a `<inkml:trace>` element (`Trace`).
pub fn trace(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "trace").with_text(value)
}

/// Create a `<inkml:traceGroup>` element (`TraceGroup`).
pub fn trace_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "traceGroup").with_children(children)
}

/// Create a `<inkml:traceView>` element (`TraceView`).
pub fn trace_view(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "traceView").with_children(children)
}

/// Create a `<inkml:context>` element (`Context`).
pub fn context(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "context").with_children(children)
}

/// Create a `<inkml:definitions>` element (`Definitions`).
pub fn definitions(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("inkml", NAMESPACE_URI, "definitions").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 27;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 27;
