//! Auto-generated from `schemas_microsoft_com_office_drawing_2017_model3d.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2017/model3d` (prefix `am3d`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2017/model3d";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "am3d";

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

static ATTRS_MODEL3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
];
static CHILDREN_MODEL3_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/am3d:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "am3d:CT_Model3DCamera/am3d:camera", property_name: Some("Model3DCamera") },
    ChildInfo { name: "am3d:CT_Model3DTransform/am3d:trans", property_name: Some("Model3DTransform") },
    ChildInfo { name: "a1611:CT_PictureAttributionSourceURL/am3d:attrSrcUrl", property_name: Some("PictureAttributionSourceURL") },
    ChildInfo { name: "am3d:CT_Model3DRaster/am3d:raster", property_name: Some("Model3DRaster") },
    ChildInfo { name: "am3d:CT_Model3DExtensionList/am3d:extLst", property_name: Some("Model3DExtensionList") },
    ChildInfo { name: "am3d:CT_ObjectViewport/am3d:objViewport", property_name: None },
    ChildInfo { name: "am3d:CT_WindowViewport/am3d:winViewport", property_name: None },
    ChildInfo { name: "am3d:CT_AmbientLight/am3d:ambientLight", property_name: None },
    ChildInfo { name: "am3d:CT_PointLight/am3d:ptLight", property_name: None },
    ChildInfo { name: "am3d:CT_SpotLight/am3d:spotLight", property_name: None },
    ChildInfo { name: "am3d:CT_DirectionalLight/am3d:dirLight", property_name: None },
    ChildInfo { name: "am3d:CT_UnknownLight/am3d:unkLight", property_name: None },
];
static ATTRS_SX_RATIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Numerator"), type_name: "Int32Value" },
    AttributeInfo { qname: ":d", property_name: Some("Denominator"), type_name: "Int32Value" },
];
static ATTRS_SY_RATIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Numerator"), type_name: "Int32Value" },
    AttributeInfo { qname: ":d", property_name: Some("Denominator"), type_name: "Int32Value" },
];
static ATTRS_SZ_RATIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Numerator"), type_name: "Int32Value" },
    AttributeInfo { qname: ":d", property_name: Some("Denominator"), type_name: "Int32Value" },
];
static ATTRS_METER_PER_MODEL_UNIT_POSITIVE_RATIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":d", property_name: None, type_name: "UInt64Value" },
];
static ATTRS_SZ_POSITIVE_RATIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":d", property_name: None, type_name: "UInt64Value" },
];
static ATTRS_ILLUMINANCE_POSITIVE_RATIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":d", property_name: None, type_name: "UInt64Value" },
];
static ATTRS_INTENSITY_POSITIVE_RATIO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: None, type_name: "UInt64Value" },
    AttributeInfo { qname: ":d", property_name: None, type_name: "UInt64Value" },
];
static ATTRS_PRE_TRANS_VECTOR3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dx", property_name: Some("Dx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dy", property_name: Some("Dy"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dz", property_name: Some("Dz"), type_name: "Int64Value" },
];
static ATTRS_POST_TRANS_VECTOR3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dx", property_name: Some("Dx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dy", property_name: Some("Dy"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dz", property_name: Some("Dz"), type_name: "Int64Value" },
];
static ATTRS_UP_VECTOR3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dx", property_name: Some("Dx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dy", property_name: Some("Dy"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dz", property_name: Some("Dz"), type_name: "Int64Value" },
];
static CHILDREN_SCALE3_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Ratio/am3d:sx", property_name: Some("SxRatio") },
    ChildInfo { name: "a:CT_Ratio/am3d:sy", property_name: Some("SyRatio") },
    ChildInfo { name: "a:CT_Ratio/am3d:sz", property_name: Some("SzRatio") },
];
static ATTRS_ROTATE3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ax", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":ay", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":az", property_name: None, type_name: "Int32Value" },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_POS_POINT3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
    AttributeInfo { qname: ":z", property_name: Some("Z"), type_name: "Int64Value" },
];
static ATTRS_LOOK_AT_POINT3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
    AttributeInfo { qname: ":z", property_name: Some("Z"), type_name: "Int64Value" },
];
static CHILDREN_ORTHOGRAPHIC_PROJECTION: &[ChildInfo] = &[
    ChildInfo { name: "am3d:CT_PositiveRatio/am3d:sz", property_name: Some("SzPositiveRatio") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_PERSPECTIVE_PROJECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fov", property_name: None, type_name: "Int32Value" },
];
static CHILDREN_PERSPECTIVE_PROJECTION: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_BLIP: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
    AttributeInfo { qname: ":cstate", property_name: Some("CompressionState"), type_name: "EnumValue" },
];
static CHILDREN_BLIP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AlphaBiLevelEffect/a:alphaBiLevel", property_name: None },
    ChildInfo { name: "a:CT_AlphaCeilingEffect/a:alphaCeiling", property_name: None },
    ChildInfo { name: "a:CT_AlphaFloorEffect/a:alphaFloor", property_name: None },
    ChildInfo { name: "a:CT_AlphaInverseEffect/a:alphaInv", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateEffect/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateFixedEffect/a:alphaModFix", property_name: None },
    ChildInfo { name: "a:CT_AlphaReplaceEffect/a:alphaRepl", property_name: None },
    ChildInfo { name: "a:CT_BiLevelEffect/a:biLevel", property_name: None },
    ChildInfo { name: "a:CT_BlurEffect/a:blur", property_name: None },
    ChildInfo { name: "a:CT_ColorChangeEffect/a:clrChange", property_name: None },
    ChildInfo { name: "a:CT_ColorReplaceEffect/a:clrRepl", property_name: None },
    ChildInfo { name: "a:CT_DuotoneEffect/a:duotone", property_name: None },
    ChildInfo { name: "a:CT_FillOverlayEffect/a:fillOverlay", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleEffect/a:grayscl", property_name: None },
    ChildInfo { name: "a:CT_HSLEffect/a:hsl", property_name: None },
    ChildInfo { name: "a:CT_LuminanceEffect/a:lum", property_name: None },
    ChildInfo { name: "a:CT_TintEffect/a:tint", property_name: None },
    ChildInfo { name: "a:CT_BlipExtensionList/a:extLst", property_name: None },
];
static CHILDREN_COLOR_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_MODEL3_D_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_MODEL3_D_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a3danim:CT_EmbeddedAnimation/a3danim:embedAnim", property_name: Some("EmbeddedAnimation") },
    ChildInfo { name: "a3danim:CT_PosterFrame/a3danim:posterFrame", property_name: Some("PosterFrame") },
];
static ATTRS_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Transform2D/a:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_CustomGeometry2D/a:custGeom", property_name: None },
    ChildInfo { name: "a:CT_PresetGeometry2D/a:prstGeom", property_name: None },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
    ChildInfo { name: "a:CT_ShapePropertiesExtensionList/a:extLst", property_name: None },
];
static CHILDREN_MODEL3_D_CAMERA: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point3D/am3d:pos", property_name: Some("PosPoint3D") },
    ChildInfo { name: "a:CT_Vector3D/am3d:up", property_name: Some("UpVector3D") },
    ChildInfo { name: "a:CT_Point3D/am3d:lookAt", property_name: Some("LookAtPoint3D") },
    ChildInfo { name: "am3d:CT_OrthographicProjection/am3d:orthographic", property_name: None },
    ChildInfo { name: "am3d:CT_PerspectiveProjection/am3d:perspective", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: None },
];
static CHILDREN_MODEL3_D_TRANSFORM: &[ChildInfo] = &[
    ChildInfo { name: "am3d:CT_PositiveRatio/am3d:meterPerModelUnit", property_name: Some("MeterPerModelUnitPositiveRatio") },
    ChildInfo { name: "a:CT_Vector3D/am3d:preTrans", property_name: Some("PreTransVector3D") },
    ChildInfo { name: "am3d:CT_Scale3D/am3d:scale", property_name: Some("Scale3D") },
    ChildInfo { name: "am3d:CT_Rotate3D/am3d:rot", property_name: Some("Rotate3D") },
    ChildInfo { name: "a:CT_Vector3D/am3d:postTrans", property_name: Some("PostTransVector3D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_PICTURE_ATTRIBUTION_SOURCE_U_R_L: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: None, type_name: "StringValue" },
];
static ATTRS_MODEL3_D_RASTER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rName", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":rVer", property_name: None, type_name: "StringValue" },
];
static CHILDREN_MODEL3_D_RASTER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Blip/am3d:blip", property_name: Some("Blip") },
];
static CHILDREN_MODEL3_D_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "am3d:CT_Model3DExtension/am3d:ext", property_name: None },
];
static ATTRS_OBJECT_VIEWPORT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":viewportSz", property_name: None, type_name: "Int64Value" },
];
static CHILDREN_OBJECT_VIEWPORT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_WINDOW_VIEWPORT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_AMBIENT_LIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_AMBIENT_LIGHT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color/am3d:clr", property_name: Some("ColorType") },
    ChildInfo { name: "am3d:CT_PositiveRatio/am3d:illuminance", property_name: Some("IlluminancePositiveRatio") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_POINT_LIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":rad", property_name: None, type_name: "Int64Value" },
];
static CHILDREN_POINT_LIGHT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color/am3d:clr", property_name: Some("ColorType") },
    ChildInfo { name: "am3d:CT_PositiveRatio/am3d:intensity", property_name: Some("IntensityPositiveRatio") },
    ChildInfo { name: "a:CT_Point3D/am3d:pos", property_name: Some("PosPoint3D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_SPOT_LIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":rad", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":spotAng", property_name: None, type_name: "Int32Value" },
];
static CHILDREN_SPOT_LIGHT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color/am3d:clr", property_name: Some("ColorType") },
    ChildInfo { name: "am3d:CT_PositiveRatio/am3d:intensity", property_name: Some("IntensityPositiveRatio") },
    ChildInfo { name: "a:CT_Point3D/am3d:pos", property_name: Some("PosPoint3D") },
    ChildInfo { name: "a:CT_Point3D/am3d:lookAt", property_name: Some("LookAtPoint3D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_DIRECTIONAL_LIGHT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":enabled", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":angularRad", property_name: None, type_name: "Int32Value" },
];
static CHILDREN_DIRECTIONAL_LIGHT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color/am3d:clr", property_name: Some("ColorType") },
    ChildInfo { name: "am3d:CT_PositiveRatio/am3d:illuminance", property_name: Some("IlluminancePositiveRatio") },
    ChildInfo { name: "a:CT_Point3D/am3d:pos", property_name: Some("PosPoint3D") },
    ChildInfo { name: "a:CT_Point3D/am3d:lookAt", property_name: Some("LookAtPoint3D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/am3d:extLst", property_name: Some("OfficeArtExtensionList") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Model3D", local_name: "model3d", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MODEL3_D, children: CHILDREN_MODEL3_D },
    ElementInfo { class_name: "SxRatio", local_name: "sx", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SX_RATIO, children: &[] },
    ElementInfo { class_name: "SyRatio", local_name: "sy", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SY_RATIO, children: &[] },
    ElementInfo { class_name: "SzRatio", local_name: "sz", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SZ_RATIO, children: &[] },
    ElementInfo { class_name: "MeterPerModelUnitPositiveRatio", local_name: "meterPerModelUnit", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_METER_PER_MODEL_UNIT_POSITIVE_RATIO, children: &[] },
    ElementInfo { class_name: "SzPositiveRatio", local_name: "sz", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SZ_POSITIVE_RATIO, children: &[] },
    ElementInfo { class_name: "IlluminancePositiveRatio", local_name: "illuminance", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ILLUMINANCE_POSITIVE_RATIO, children: &[] },
    ElementInfo { class_name: "IntensityPositiveRatio", local_name: "intensity", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INTENSITY_POSITIVE_RATIO, children: &[] },
    ElementInfo { class_name: "PreTransVector3D", local_name: "preTrans", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRE_TRANS_VECTOR3_D, children: &[] },
    ElementInfo { class_name: "PostTransVector3D", local_name: "postTrans", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POST_TRANS_VECTOR3_D, children: &[] },
    ElementInfo { class_name: "UpVector3D", local_name: "up", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UP_VECTOR3_D, children: &[] },
    ElementInfo { class_name: "Scale3D", local_name: "scale", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCALE3_D },
    ElementInfo { class_name: "Rotate3D", local_name: "rot", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROTATE3_D, children: &[] },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "PosPoint3D", local_name: "pos", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POS_POINT3_D, children: &[] },
    ElementInfo { class_name: "LookAtPoint3D", local_name: "lookAt", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LOOK_AT_POINT3_D, children: &[] },
    ElementInfo { class_name: "OrthographicProjection", local_name: "orthographic", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ORTHOGRAPHIC_PROJECTION },
    ElementInfo { class_name: "PerspectiveProjection", local_name: "perspective", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PERSPECTIVE_PROJECTION, children: CHILDREN_PERSPECTIVE_PROJECTION },
    ElementInfo { class_name: "Blip", local_name: "blip", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP, children: CHILDREN_BLIP },
    ElementInfo { class_name: "ColorType", local_name: "clr", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_TYPE },
    ElementInfo { class_name: "Model3DExtension", local_name: "ext", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MODEL3_D_EXTENSION, children: CHILDREN_MODEL3_D_EXTENSION },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Model3DCamera", local_name: "camera", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODEL3_D_CAMERA },
    ElementInfo { class_name: "Model3DTransform", local_name: "trans", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODEL3_D_TRANSFORM },
    ElementInfo { class_name: "PictureAttributionSourceURL", local_name: "attrSrcUrl", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PICTURE_ATTRIBUTION_SOURCE_U_R_L, children: &[] },
    ElementInfo { class_name: "Model3DRaster", local_name: "raster", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MODEL3_D_RASTER, children: CHILDREN_MODEL3_D_RASTER },
    ElementInfo { class_name: "Model3DExtensionList", local_name: "extLst", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODEL3_D_EXTENSION_LIST },
    ElementInfo { class_name: "ObjectViewport", local_name: "objViewport", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OBJECT_VIEWPORT, children: CHILDREN_OBJECT_VIEWPORT },
    ElementInfo { class_name: "WindowViewport", local_name: "winViewport", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WINDOW_VIEWPORT },
    ElementInfo { class_name: "AmbientLight", local_name: "ambientLight", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AMBIENT_LIGHT, children: CHILDREN_AMBIENT_LIGHT },
    ElementInfo { class_name: "PointLight", local_name: "ptLight", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_POINT_LIGHT, children: CHILDREN_POINT_LIGHT },
    ElementInfo { class_name: "SpotLight", local_name: "spotLight", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SPOT_LIGHT, children: CHILDREN_SPOT_LIGHT },
    ElementInfo { class_name: "DirectionalLight", local_name: "dirLight", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIRECTIONAL_LIGHT, children: CHILDREN_DIRECTIONAL_LIGHT },
    ElementInfo { class_name: "UnknownLight", local_name: "unkLight", prefix: "am3d", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
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

/// Create a `<am3d:model3d>` element (`Model3D`).
pub fn model3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "model3d").with_children(children)
}

/// Create a `<am3d:sx>` element (`SxRatio`).
pub fn sx_ratio() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "sx")
}

/// Create a `<am3d:sy>` element (`SyRatio`).
pub fn sy_ratio() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "sy")
}

/// Create a `<am3d:sz>` element (`SzRatio`).
pub fn sz_ratio() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "sz")
}

/// Create a `<am3d:meterPerModelUnit>` element (`MeterPerModelUnitPositiveRatio`).
pub fn meter_per_model_unit_positive_ratio() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "meterPerModelUnit")
}

/// Create a `<am3d:sz>` element (`SzPositiveRatio`).
pub fn sz_positive_ratio() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "sz")
}

/// Create a `<am3d:illuminance>` element (`IlluminancePositiveRatio`).
pub fn illuminance_positive_ratio() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "illuminance")
}

/// Create a `<am3d:intensity>` element (`IntensityPositiveRatio`).
pub fn intensity_positive_ratio() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "intensity")
}

/// Create a `<am3d:preTrans>` element (`PreTransVector3D`).
pub fn pre_trans_vector3_d() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "preTrans")
}

/// Create a `<am3d:postTrans>` element (`PostTransVector3D`).
pub fn post_trans_vector3_d() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "postTrans")
}

/// Create a `<am3d:up>` element (`UpVector3D`).
pub fn up_vector3_d() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "up")
}

/// Create a `<am3d:scale>` element (`Scale3D`).
pub fn scale3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "scale").with_children(children)
}

/// Create a `<am3d:rot>` element (`Rotate3D`).
pub fn rotate3_d() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "rot")
}

/// Create a `<am3d:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<am3d:pos>` element (`PosPoint3D`).
pub fn pos_point3_d() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "pos")
}

/// Create a `<am3d:lookAt>` element (`LookAtPoint3D`).
pub fn look_at_point3_d() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "lookAt")
}

/// Create a `<am3d:orthographic>` element (`OrthographicProjection`).
pub fn orthographic_projection(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "orthographic").with_children(children)
}

/// Create a `<am3d:perspective>` element (`PerspectiveProjection`).
pub fn perspective_projection(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "perspective").with_children(children)
}

/// Create a `<am3d:blip>` element (`Blip`).
pub fn blip(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "blip").with_children(children)
}

/// Create a `<am3d:clr>` element (`ColorType`).
pub fn color_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "clr").with_children(children)
}

/// Create a `<am3d:ext>` element (`Model3DExtension`).
pub fn model3_d_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<am3d:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<am3d:camera>` element (`Model3DCamera`).
pub fn model3_d_camera(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "camera").with_children(children)
}

/// Create a `<am3d:trans>` element (`Model3DTransform`).
pub fn model3_d_transform(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "trans").with_children(children)
}

/// Create a `<am3d:attrSrcUrl>` element (`PictureAttributionSourceURL`).
pub fn picture_attribution_source_u_r_l() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "attrSrcUrl")
}

/// Create a `<am3d:raster>` element (`Model3DRaster`).
pub fn model3_d_raster(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "raster").with_children(children)
}

/// Create a `<am3d:extLst>` element (`Model3DExtensionList`).
pub fn model3_d_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<am3d:objViewport>` element (`ObjectViewport`).
pub fn object_viewport(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "objViewport").with_children(children)
}

/// Create a `<am3d:winViewport>` element (`WindowViewport`).
pub fn window_viewport(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "winViewport").with_children(children)
}

/// Create a `<am3d:ambientLight>` element (`AmbientLight`).
pub fn ambient_light(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "ambientLight").with_children(children)
}

/// Create a `<am3d:ptLight>` element (`PointLight`).
pub fn point_light(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "ptLight").with_children(children)
}

/// Create a `<am3d:spotLight>` element (`SpotLight`).
pub fn spot_light(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "spotLight").with_children(children)
}

/// Create a `<am3d:dirLight>` element (`DirectionalLight`).
pub fn directional_light(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "dirLight").with_children(children)
}

/// Create a `<am3d:unkLight>` element (`UnknownLight`).
pub fn unknown_light() -> OpenXmlElement {
    OpenXmlElement::new("am3d", NAMESPACE_URI, "unkLight")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 38;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 34;
