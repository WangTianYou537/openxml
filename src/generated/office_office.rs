//! Auto-generated from `schemas-microsoft-com_office_office.json`.
//! Target namespace: `urn:schemas-microsoft-com:office:office` (prefix `o`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "urn:schemas-microsoft-com:office:office";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "o";

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

static ATTRS_SHAPE_DEFAULTS: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":spidmax", property_name: Some("MaxShapeId"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":style", property_name: Some("Style"), type_name: "StringValue" },
    AttributeInfo { qname: ":fill", property_name: Some("BeFilled"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":fillcolor", property_name: Some("FillColor"), type_name: "StringValue" },
    AttributeInfo { qname: ":stroke", property_name: Some("IsStroke"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":strokecolor", property_name: Some("StrokeColor"), type_name: "StringValue" },
    AttributeInfo { qname: "o:allowincell", property_name: Some("AllowInCell"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: "o:allowoverlap", property_name: Some("AllowOverlap"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: "o:insetmode", property_name: Some("InsetMode"), type_name: "EnumValue" },
];
static CHILDREN_SHAPE_DEFAULTS: &[ChildInfo] = &[
    ChildInfo { name: "v:CT_Fill/v:fill", property_name: Some("Fill") },
    ChildInfo { name: "v:CT_ImageData/v:imagedata", property_name: Some("ImageData") },
    ChildInfo { name: "v:CT_Stroke/v:stroke", property_name: Some("Stroke") },
    ChildInfo { name: "v:CT_Textbox/v:textbox", property_name: Some("TextBox") },
    ChildInfo { name: "v:CT_Shadow/v:shadow", property_name: Some("Shadow") },
    ChildInfo { name: "o:CT_Skew/o:skew", property_name: Some("Skew") },
    ChildInfo { name: "o:CT_Extrusion/o:extrusion", property_name: Some("Extrusion") },
    ChildInfo { name: "o:CT_Callout/o:callout", property_name: Some("Callout") },
    ChildInfo { name: "o:CT_Lock/o:lock", property_name: Some("Lock") },
    ChildInfo { name: "o:CT_ColorMru/o:colormru", property_name: Some("ColorMostRecentlyUsed") },
    ChildInfo { name: "o:CT_ColorMenu/o:colormenu", property_name: Some("ColorMenu") },
];
static ATTRS_SHAPE_LAYOUT: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
];
static CHILDREN_SHAPE_LAYOUT: &[ChildInfo] = &[
    ChildInfo { name: "o:CT_IdMap/o:idmap", property_name: Some("ShapeIdMap") },
    ChildInfo { name: "o:CT_RegroupTable/o:regrouptable", property_name: Some("RegroupTable") },
    ChildInfo { name: "o:CT_Rules/o:rules", property_name: Some("Rules") },
];
static ATTRS_SIGNATURE_LINE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":issignatureline", property_name: Some("IsSignatureLine"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":provid", property_name: Some("ProviderId"), type_name: "StringValue" },
    AttributeInfo { qname: ":signinginstructionsset", property_name: Some("SigningInstructionsSet"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":allowcomments", property_name: Some("AllowComments"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":showsigndate", property_name: Some("ShowSignDate"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: "o:suggestedsigner", property_name: Some("SuggestedSigner"), type_name: "StringValue" },
    AttributeInfo { qname: "o:suggestedsigner2", property_name: Some("SuggestedSigner2"), type_name: "StringValue" },
    AttributeInfo { qname: "o:suggestedsigneremail", property_name: Some("SuggestedSignerEmail"), type_name: "StringValue" },
    AttributeInfo { qname: ":signinginstructions", property_name: Some("SigningInstructions"), type_name: "StringValue" },
    AttributeInfo { qname: ":addlxml", property_name: Some("AdditionalXml"), type_name: "StringValue" },
    AttributeInfo { qname: ":sigprovurl", property_name: Some("SignatureProviderUrl"), type_name: "StringValue" },
];
static ATTRS_INK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":i", property_name: Some("InkData"), type_name: "Base64BinaryValue" },
    AttributeInfo { qname: ":annotation", property_name: Some("AnnotationFlag"), type_name: "TrueFalseValue" },
];
static ATTRS_DIAGRAM: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dgmstyle", property_name: Some("Style"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":autoformat", property_name: Some("AutoFormat"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":reverse", property_name: Some("Reverse"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":autolayout", property_name: Some("AutoLayout"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":dgmscalex", property_name: Some("ScaleX"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":dgmscaley", property_name: Some("ScaleY"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":dgmfontsize", property_name: Some("FontSize"), type_name: "IntegerValue" },
    AttributeInfo { qname: ":constrainbounds", property_name: Some("ConstrainBounds"), type_name: "StringValue" },
    AttributeInfo { qname: ":dgmbasetextscale", property_name: Some("BaseTextScale"), type_name: "IntegerValue" },
];
static CHILDREN_DIAGRAM: &[ChildInfo] = &[
    ChildInfo { name: "o:CT_RelationTable/o:relationtable", property_name: Some("RelationTable") },
];
static ATTRS_SKEW: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":offset", property_name: Some("Offset"), type_name: "StringValue" },
    AttributeInfo { qname: ":origin", property_name: Some("Origin"), type_name: "StringValue" },
    AttributeInfo { qname: ":matrix", property_name: Some("Matrix"), type_name: "StringValue" },
];
static ATTRS_EXTRUSION: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":render", property_name: Some("Render"), type_name: "EnumValue" },
    AttributeInfo { qname: ":viewpointorigin", property_name: Some("ViewpointOrigin"), type_name: "StringValue" },
    AttributeInfo { qname: ":viewpoint", property_name: Some("Viewpoint"), type_name: "StringValue" },
    AttributeInfo { qname: ":skewangle", property_name: Some("SkewAngle"), type_name: "SingleValue" },
    AttributeInfo { qname: ":skewamt", property_name: Some("SkewAmount"), type_name: "StringValue" },
    AttributeInfo { qname: ":foredepth", property_name: Some("ForceDepth"), type_name: "StringValue" },
    AttributeInfo { qname: ":backdepth", property_name: Some("BackDepth"), type_name: "StringValue" },
    AttributeInfo { qname: ":orientation", property_name: Some("Orientation"), type_name: "StringValue" },
    AttributeInfo { qname: ":orientationangle", property_name: Some("OrientationAngle"), type_name: "SingleValue" },
    AttributeInfo { qname: ":lockrotationcenter", property_name: Some("LockRotationCenter"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":autorotationcenter", property_name: Some("AutoRotationCenter"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":rotationcenter", property_name: Some("RotationCenter"), type_name: "StringValue" },
    AttributeInfo { qname: ":rotationangle", property_name: Some("RotationAngle"), type_name: "StringValue" },
    AttributeInfo { qname: ":color", property_name: Some("Color"), type_name: "StringValue" },
    AttributeInfo { qname: ":shininess", property_name: Some("Shininess"), type_name: "SingleValue" },
    AttributeInfo { qname: ":specularity", property_name: Some("Specularity"), type_name: "StringValue" },
    AttributeInfo { qname: ":diffusity", property_name: Some("Diffusity"), type_name: "StringValue" },
    AttributeInfo { qname: ":metal", property_name: Some("Metal"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":edge", property_name: Some("Edge"), type_name: "StringValue" },
    AttributeInfo { qname: ":facet", property_name: Some("Facet"), type_name: "StringValue" },
    AttributeInfo { qname: ":lightface", property_name: Some("LightFace"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":brightness", property_name: Some("Brightness"), type_name: "StringValue" },
    AttributeInfo { qname: ":lightposition", property_name: Some("LightPosition"), type_name: "StringValue" },
    AttributeInfo { qname: ":lightlevel", property_name: Some("LightLevel"), type_name: "StringValue" },
    AttributeInfo { qname: ":lightharsh", property_name: Some("LightHarsh"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":lightposition2", property_name: Some("LightPosition2"), type_name: "StringValue" },
    AttributeInfo { qname: ":lightlevel2", property_name: Some("LightLevel2"), type_name: "StringValue" },
    AttributeInfo { qname: ":lightharsh2", property_name: Some("LightHarsh2"), type_name: "TrueFalseValue" },
];
static ATTRS_CALLOUT: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":gap", property_name: Some("Gap"), type_name: "StringValue" },
    AttributeInfo { qname: ":angle", property_name: Some("Angle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dropauto", property_name: Some("DropAuto"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":drop", property_name: Some("Drop"), type_name: "StringValue" },
    AttributeInfo { qname: ":distance", property_name: Some("Distance"), type_name: "StringValue" },
    AttributeInfo { qname: ":lengthspecified", property_name: Some("LengthSpecified"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":length", property_name: Some("Length"), type_name: "StringValue" },
    AttributeInfo { qname: ":accentbar", property_name: Some("AccentBar"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":textborder", property_name: Some("TextBorder"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":minusx", property_name: Some("MinusX"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":minusy", property_name: Some("MinusY"), type_name: "TrueFalseValue" },
];
static ATTRS_LOCK: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":position", property_name: Some("Position"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":selection", property_name: Some("Selection"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":grouping", property_name: Some("Grouping"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":ungrouping", property_name: Some("Ungrouping"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":rotation", property_name: Some("Rotation"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":cropping", property_name: Some("Cropping"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":verticies", property_name: Some("Verticies"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":adjusthandles", property_name: Some("AdjustHandles"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":text", property_name: Some("TextLock"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":aspectratio", property_name: Some("AspectRatio"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":shapetype", property_name: Some("ShapeType"), type_name: "TrueFalseValue" },
];
static ATTRS_OLE_OBJECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":Type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":ProgID", property_name: Some("ProgId"), type_name: "StringValue" },
    AttributeInfo { qname: ":ShapeID", property_name: Some("ShapeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":DrawAspect", property_name: Some("DrawAspect"), type_name: "EnumValue" },
    AttributeInfo { qname: ":ObjectID", property_name: Some("ObjectId"), type_name: "StringValue" },
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":UpdateMode", property_name: Some("UpdateMode"), type_name: "EnumValue" },
];
static CHILDREN_OLE_OBJECT: &[ChildInfo] = &[
    ChildInfo { name: "o:ST_OLELinkType/o:LinkType", property_name: Some("LinkType") },
    ChildInfo { name: "o:ST_TrueFalseBlank/o:LockedField", property_name: Some("LockedField") },
    ChildInfo { name: "xsd:string/o:FieldCodes", property_name: Some("FieldCodes") },
];
static ATTRS_COMPLEX: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
];
static ATTRS_LEFT_STROKE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":weight", property_name: Some("Weight"), type_name: "StringValue" },
    AttributeInfo { qname: ":color", property_name: Some("Color"), type_name: "StringValue" },
    AttributeInfo { qname: ":color2", property_name: Some("Color2"), type_name: "StringValue" },
    AttributeInfo { qname: ":opacity", property_name: Some("Opacity"), type_name: "StringValue" },
    AttributeInfo { qname: ":linestyle", property_name: Some("LineStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":miterlimit", property_name: Some("MiterLimit"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":joinstyle", property_name: Some("JoinStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endcap", property_name: Some("EndCap"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dashstyle", property_name: Some("DashStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":insetpen", property_name: Some("InsetPen"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":filltype", property_name: Some("FillType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":src", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageaspect", property_name: Some("ImageAspect"), type_name: "EnumValue" },
    AttributeInfo { qname: ":imagesize", property_name: Some("ImageSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":imagealignshape", property_name: Some("ImageAlignShape"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":startarrow", property_name: Some("StartArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowwidth", property_name: Some("StartArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowlength", property_name: Some("StartArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrow", property_name: Some("EndArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowwidth", property_name: Some("EndArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowlength", property_name: Some("EndArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: "o:href", property_name: Some("Href"), type_name: "StringValue" },
    AttributeInfo { qname: "o:althref", property_name: Some("AlternateImageReference"), type_name: "StringValue" },
    AttributeInfo { qname: "o:title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: "o:forcedash", property_name: Some("ForceDash"), type_name: "TrueFalseValue" },
];
static ATTRS_TOP_STROKE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":weight", property_name: Some("Weight"), type_name: "StringValue" },
    AttributeInfo { qname: ":color", property_name: Some("Color"), type_name: "StringValue" },
    AttributeInfo { qname: ":color2", property_name: Some("Color2"), type_name: "StringValue" },
    AttributeInfo { qname: ":opacity", property_name: Some("Opacity"), type_name: "StringValue" },
    AttributeInfo { qname: ":linestyle", property_name: Some("LineStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":miterlimit", property_name: Some("MiterLimit"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":joinstyle", property_name: Some("JoinStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endcap", property_name: Some("EndCap"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dashstyle", property_name: Some("DashStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":insetpen", property_name: Some("InsetPen"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":filltype", property_name: Some("FillType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":src", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageaspect", property_name: Some("ImageAspect"), type_name: "EnumValue" },
    AttributeInfo { qname: ":imagesize", property_name: Some("ImageSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":imagealignshape", property_name: Some("ImageAlignShape"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":startarrow", property_name: Some("StartArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowwidth", property_name: Some("StartArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowlength", property_name: Some("StartArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrow", property_name: Some("EndArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowwidth", property_name: Some("EndArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowlength", property_name: Some("EndArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: "o:href", property_name: Some("Href"), type_name: "StringValue" },
    AttributeInfo { qname: "o:althref", property_name: Some("AlternateImageReference"), type_name: "StringValue" },
    AttributeInfo { qname: "o:title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: "o:forcedash", property_name: Some("ForceDash"), type_name: "TrueFalseValue" },
];
static ATTRS_RIGHT_STROKE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":weight", property_name: Some("Weight"), type_name: "StringValue" },
    AttributeInfo { qname: ":color", property_name: Some("Color"), type_name: "StringValue" },
    AttributeInfo { qname: ":color2", property_name: Some("Color2"), type_name: "StringValue" },
    AttributeInfo { qname: ":opacity", property_name: Some("Opacity"), type_name: "StringValue" },
    AttributeInfo { qname: ":linestyle", property_name: Some("LineStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":miterlimit", property_name: Some("MiterLimit"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":joinstyle", property_name: Some("JoinStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endcap", property_name: Some("EndCap"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dashstyle", property_name: Some("DashStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":insetpen", property_name: Some("InsetPen"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":filltype", property_name: Some("FillType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":src", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageaspect", property_name: Some("ImageAspect"), type_name: "EnumValue" },
    AttributeInfo { qname: ":imagesize", property_name: Some("ImageSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":imagealignshape", property_name: Some("ImageAlignShape"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":startarrow", property_name: Some("StartArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowwidth", property_name: Some("StartArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowlength", property_name: Some("StartArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrow", property_name: Some("EndArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowwidth", property_name: Some("EndArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowlength", property_name: Some("EndArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: "o:href", property_name: Some("Href"), type_name: "StringValue" },
    AttributeInfo { qname: "o:althref", property_name: Some("AlternateImageReference"), type_name: "StringValue" },
    AttributeInfo { qname: "o:title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: "o:forcedash", property_name: Some("ForceDash"), type_name: "TrueFalseValue" },
];
static ATTRS_BOTTOM_STROKE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":weight", property_name: Some("Weight"), type_name: "StringValue" },
    AttributeInfo { qname: ":color", property_name: Some("Color"), type_name: "StringValue" },
    AttributeInfo { qname: ":color2", property_name: Some("Color2"), type_name: "StringValue" },
    AttributeInfo { qname: ":opacity", property_name: Some("Opacity"), type_name: "StringValue" },
    AttributeInfo { qname: ":linestyle", property_name: Some("LineStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":miterlimit", property_name: Some("MiterLimit"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":joinstyle", property_name: Some("JoinStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endcap", property_name: Some("EndCap"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dashstyle", property_name: Some("DashStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":insetpen", property_name: Some("InsetPen"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":filltype", property_name: Some("FillType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":src", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageaspect", property_name: Some("ImageAspect"), type_name: "EnumValue" },
    AttributeInfo { qname: ":imagesize", property_name: Some("ImageSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":imagealignshape", property_name: Some("ImageAlignShape"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":startarrow", property_name: Some("StartArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowwidth", property_name: Some("StartArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowlength", property_name: Some("StartArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrow", property_name: Some("EndArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowwidth", property_name: Some("EndArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowlength", property_name: Some("EndArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: "o:href", property_name: Some("Href"), type_name: "StringValue" },
    AttributeInfo { qname: "o:althref", property_name: Some("AlternateImageReference"), type_name: "StringValue" },
    AttributeInfo { qname: "o:title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: "o:forcedash", property_name: Some("ForceDash"), type_name: "TrueFalseValue" },
];
static ATTRS_COLUMN_STROKE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":on", property_name: Some("On"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":weight", property_name: Some("Weight"), type_name: "StringValue" },
    AttributeInfo { qname: ":color", property_name: Some("Color"), type_name: "StringValue" },
    AttributeInfo { qname: ":color2", property_name: Some("Color2"), type_name: "StringValue" },
    AttributeInfo { qname: ":opacity", property_name: Some("Opacity"), type_name: "StringValue" },
    AttributeInfo { qname: ":linestyle", property_name: Some("LineStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":miterlimit", property_name: Some("MiterLimit"), type_name: "DecimalValue" },
    AttributeInfo { qname: ":joinstyle", property_name: Some("JoinStyle"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endcap", property_name: Some("EndCap"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dashstyle", property_name: Some("DashStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":insetpen", property_name: Some("InsetPen"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":filltype", property_name: Some("FillType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":src", property_name: Some("Source"), type_name: "StringValue" },
    AttributeInfo { qname: ":imageaspect", property_name: Some("ImageAspect"), type_name: "EnumValue" },
    AttributeInfo { qname: ":imagesize", property_name: Some("ImageSize"), type_name: "StringValue" },
    AttributeInfo { qname: ":imagealignshape", property_name: Some("ImageAlignShape"), type_name: "TrueFalseValue" },
    AttributeInfo { qname: ":startarrow", property_name: Some("StartArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowwidth", property_name: Some("StartArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startarrowlength", property_name: Some("StartArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrow", property_name: Some("EndArrow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowwidth", property_name: Some("EndArrowWidth"), type_name: "EnumValue" },
    AttributeInfo { qname: ":endarrowlength", property_name: Some("EndArrowLength"), type_name: "EnumValue" },
    AttributeInfo { qname: "o:href", property_name: Some("Href"), type_name: "StringValue" },
    AttributeInfo { qname: "o:althref", property_name: Some("AlternateImageReference"), type_name: "StringValue" },
    AttributeInfo { qname: "o:title", property_name: Some("Title"), type_name: "StringValue" },
    AttributeInfo { qname: "o:forcedash", property_name: Some("ForceDash"), type_name: "TrueFalseValue" },
];
static ATTRS_CLIP_PATH: &[AttributeInfo] = &[
    AttributeInfo { qname: "o:v", property_name: Some("Value"), type_name: "StringValue" },
];
static ATTRS_FILL_EXTENDED_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
];
static ATTRS_SHAPE_ID_MAP: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":data", property_name: Some("Data"), type_name: "StringValue" },
];
static ATTRS_REGROUP_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
];
static CHILDREN_REGROUP_TABLE: &[ChildInfo] = &[
    ChildInfo { name: "o:CT_Entry/o:entry", property_name: None },
];
static ATTRS_RULES: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
];
static CHILDREN_RULES: &[ChildInfo] = &[
    ChildInfo { name: "o:CT_R/o:r", property_name: None },
];
static ATTRS_ENTRY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":new", property_name: Some("New"), type_name: "Int32Value" },
    AttributeInfo { qname: ":old", property_name: Some("Old"), type_name: "Int32Value" },
];
static ATTRS_RULE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":how", property_name: Some("How"), type_name: "EnumValue" },
    AttributeInfo { qname: ":idref", property_name: Some("ShapeReference"), type_name: "StringValue" },
];
static CHILDREN_RULE: &[ChildInfo] = &[
    ChildInfo { name: "o:CT_Proxy/o:proxy", property_name: None },
];
static ATTRS_RELATION_TABLE: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
];
static CHILDREN_RELATION_TABLE: &[ChildInfo] = &[
    ChildInfo { name: "o:CT_Relation/o:rel", property_name: None },
];
static ATTRS_RELATION: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":idsrc", property_name: Some("SourceId"), type_name: "StringValue" },
    AttributeInfo { qname: ":iddest", property_name: Some("DestinationId"), type_name: "StringValue" },
    AttributeInfo { qname: ":idcntr", property_name: Some("CenterShapeId"), type_name: "StringValue" },
];
static ATTRS_PROXY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":start", property_name: Some("Start"), type_name: "TrueFalseBlankValue" },
    AttributeInfo { qname: ":end", property_name: Some("End"), type_name: "TrueFalseBlankValue" },
    AttributeInfo { qname: ":idref", property_name: Some("ShapeReference"), type_name: "StringValue" },
    AttributeInfo { qname: ":connectloc", property_name: Some("ConnectionLocation"), type_name: "Int32Value" },
];
static ATTRS_COLOR_MOST_RECENTLY_USED: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":colors", property_name: Some("Colors"), type_name: "StringValue" },
];
static ATTRS_COLOR_MENU: &[AttributeInfo] = &[
    AttributeInfo { qname: "v:ext", property_name: Some("Extension"), type_name: "EnumValue" },
    AttributeInfo { qname: ":strokecolor", property_name: Some("StrokeColor"), type_name: "StringValue" },
    AttributeInfo { qname: ":fillcolor", property_name: Some("FillColor"), type_name: "StringValue" },
    AttributeInfo { qname: ":shadowcolor", property_name: Some("ShadowColor"), type_name: "StringValue" },
    AttributeInfo { qname: ":extrusioncolor", property_name: Some("ExtrusionColor"), type_name: "StringValue" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ShapeDefaults", local_name: "shapedefaults", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_DEFAULTS, children: CHILDREN_SHAPE_DEFAULTS },
    ElementInfo { class_name: "ShapeLayout", local_name: "shapelayout", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_LAYOUT, children: CHILDREN_SHAPE_LAYOUT },
    ElementInfo { class_name: "SignatureLine", local_name: "signatureline", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SIGNATURE_LINE, children: &[] },
    ElementInfo { class_name: "Ink", local_name: "ink", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INK, children: &[] },
    ElementInfo { class_name: "Diagram", local_name: "diagram", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIAGRAM, children: CHILDREN_DIAGRAM },
    ElementInfo { class_name: "Skew", local_name: "skew", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SKEW, children: &[] },
    ElementInfo { class_name: "Extrusion", local_name: "extrusion", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXTRUSION, children: &[] },
    ElementInfo { class_name: "Callout", local_name: "callout", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CALLOUT, children: &[] },
    ElementInfo { class_name: "Lock", local_name: "lock", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LOCK, children: &[] },
    ElementInfo { class_name: "OleObject", local_name: "OLEObject", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OLE_OBJECT, children: CHILDREN_OLE_OBJECT },
    ElementInfo { class_name: "Complex", local_name: "complex", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMPLEX, children: &[] },
    ElementInfo { class_name: "LeftStroke", local_name: "left", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LEFT_STROKE, children: &[] },
    ElementInfo { class_name: "TopStroke", local_name: "top", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TOP_STROKE, children: &[] },
    ElementInfo { class_name: "RightStroke", local_name: "right", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RIGHT_STROKE, children: &[] },
    ElementInfo { class_name: "BottomStroke", local_name: "bottom", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOTTOM_STROKE, children: &[] },
    ElementInfo { class_name: "ColumnStroke", local_name: "column", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLUMN_STROKE, children: &[] },
    ElementInfo { class_name: "ClipPath", local_name: "clippath", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CLIP_PATH, children: &[] },
    ElementInfo { class_name: "FillExtendedProperties", local_name: "fill", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FILL_EXTENDED_PROPERTIES, children: &[] },
    ElementInfo { class_name: "ShapeIdMap", local_name: "idmap", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHAPE_ID_MAP, children: &[] },
    ElementInfo { class_name: "RegroupTable", local_name: "regrouptable", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_REGROUP_TABLE, children: CHILDREN_REGROUP_TABLE },
    ElementInfo { class_name: "Rules", local_name: "rules", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RULES, children: CHILDREN_RULES },
    ElementInfo { class_name: "Entry", local_name: "entry", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ENTRY, children: &[] },
    ElementInfo { class_name: "Rule", local_name: "r", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RULE, children: CHILDREN_RULE },
    ElementInfo { class_name: "RelationTable", local_name: "relationtable", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RELATION_TABLE, children: CHILDREN_RELATION_TABLE },
    ElementInfo { class_name: "Relation", local_name: "rel", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RELATION, children: &[] },
    ElementInfo { class_name: "LinkType", local_name: "LinkType", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "LockedField", local_name: "LockedField", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "FieldCodes", local_name: "FieldCodes", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Proxy", local_name: "proxy", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PROXY, children: &[] },
    ElementInfo { class_name: "ColorMostRecentlyUsed", local_name: "colormru", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR_MOST_RECENTLY_USED, children: &[] },
    ElementInfo { class_name: "ColorMenu", local_name: "colormenu", prefix: "o", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR_MENU, children: &[] },
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

/// Create a `<o:shapedefaults>` element (`ShapeDefaults`).
pub fn shape_defaults(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "shapedefaults").with_children(children)
}

/// Create a `<o:shapelayout>` element (`ShapeLayout`).
pub fn shape_layout(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "shapelayout").with_children(children)
}

/// Create a `<o:signatureline>` element (`SignatureLine`).
pub fn signature_line() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "signatureline")
}

/// Create a `<o:ink>` element (`Ink`).
pub fn ink() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "ink")
}

/// Create a `<o:diagram>` element (`Diagram`).
pub fn diagram(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "diagram").with_children(children)
}

/// Create a `<o:skew>` element (`Skew`).
pub fn skew() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "skew")
}

/// Create a `<o:extrusion>` element (`Extrusion`).
pub fn extrusion() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "extrusion")
}

/// Create a `<o:callout>` element (`Callout`).
pub fn callout() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "callout")
}

/// Create a `<o:lock>` element (`Lock`).
pub fn lock() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "lock")
}

/// Create a `<o:OLEObject>` element (`OleObject`).
pub fn ole_object(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "OLEObject").with_children(children)
}

/// Create a `<o:complex>` element (`Complex`).
pub fn complex() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "complex")
}

/// Create a `<o:left>` element (`LeftStroke`).
pub fn left_stroke() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "left")
}

/// Create a `<o:top>` element (`TopStroke`).
pub fn top_stroke() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "top")
}

/// Create a `<o:right>` element (`RightStroke`).
pub fn right_stroke() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "right")
}

/// Create a `<o:bottom>` element (`BottomStroke`).
pub fn bottom_stroke() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "bottom")
}

/// Create a `<o:column>` element (`ColumnStroke`).
pub fn column_stroke() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "column")
}

/// Create a `<o:clippath>` element (`ClipPath`).
pub fn clip_path() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "clippath")
}

/// Create a `<o:fill>` element (`FillExtendedProperties`).
pub fn fill_extended_properties() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "fill")
}

/// Create a `<o:idmap>` element (`ShapeIdMap`).
pub fn shape_id_map() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "idmap")
}

/// Create a `<o:regrouptable>` element (`RegroupTable`).
pub fn regroup_table(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "regrouptable").with_children(children)
}

/// Create a `<o:rules>` element (`Rules`).
pub fn rules(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "rules").with_children(children)
}

/// Create a `<o:entry>` element (`Entry`).
pub fn entry() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "entry")
}

/// Create a `<o:r>` element (`Rule`).
pub fn rule(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "r").with_children(children)
}

/// Create a `<o:relationtable>` element (`RelationTable`).
pub fn relation_table(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "relationtable").with_children(children)
}

/// Create a `<o:rel>` element (`Relation`).
pub fn relation() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "rel")
}

/// Create a `<o:LinkType>` element (`LinkType`).
pub fn link_type(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "LinkType").with_text(value)
}

/// Create a `<o:LockedField>` element (`LockedField`).
pub fn locked_field(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "LockedField").with_text(value)
}

/// Create a `<o:FieldCodes>` element (`FieldCodes`).
pub fn field_codes(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "FieldCodes").with_text(value)
}

/// Create a `<o:proxy>` element (`Proxy`).
pub fn proxy() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "proxy")
}

/// Create a `<o:colormru>` element (`ColorMostRecentlyUsed`).
pub fn color_most_recently_used() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "colormru")
}

/// Create a `<o:colormenu>` element (`ColorMenu`).
pub fn color_menu() -> OpenXmlElement {
    OpenXmlElement::new("o", NAMESPACE_URI, "colormenu")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 32;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 31;
