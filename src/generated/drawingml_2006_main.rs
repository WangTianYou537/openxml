//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_main.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/main` (prefix `a`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "a";

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

static CHILDREN_AUDIO_FROM_C_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AudioCDTime/a:st", property_name: Some("StartTime") },
    ChildInfo { name: "a:CT_AudioCDTime/a:end", property_name: Some("EndTime") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_WAVE_AUDIO_FILE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":builtIn", property_name: Some("BuiltIn"), type_name: "BooleanValue" },
];
static ATTRS_HYPERLINK_SOUND: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":builtIn", property_name: Some("BuiltIn"), type_name: "BooleanValue" },
];
static ATTRS_AUDIO_FROM_FILE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
];
static CHILDREN_AUDIO_FROM_FILE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_VIDEO_FROM_FILE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
];
static CHILDREN_VIDEO_FROM_FILE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_QUICK_TIME_FROM_FILE: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:link", property_name: Some("Link"), type_name: "StringValue" },
];
static CHILDREN_QUICK_TIME_FROM_FILE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SHADE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_ALPHA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_ALPHA_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_ALPHA_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_HUE_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_HUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_HUE_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SATURATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SATURATION_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SATURATION_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_LUMINANCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_LUMINANCE_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_LUMINANCE_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_RED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_RED_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_RED_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_GREEN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_GREEN_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_GREEN_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_BLUE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_BLUE_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_BLUE_MODULATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: Some("Uri"), type_name: "StringValue" },
];
static ATTRS_RGB_COLOR_MODEL_PERCENTAGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":r", property_name: Some("RedPortion"), type_name: "Int32Value" },
    AttributeInfo { qname: ":g", property_name: Some("GreenPortion"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("BluePortion"), type_name: "Int32Value" },
];
static CHILDREN_RGB_COLOR_MODEL_PERCENTAGE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static ATTRS_RGB_COLOR_MODEL_HEX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: "a14:legacySpreadsheetColorIndex", property_name: Some("LegacySpreadsheetColorIndex"), type_name: "Int32Value" },
];
static CHILDREN_RGB_COLOR_MODEL_HEX: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static ATTRS_HSL_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":hue", property_name: Some("HueValue"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sat", property_name: Some("SatValue"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lum", property_name: Some("LumValue"), type_name: "Int32Value" },
];
static CHILDREN_HSL_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static ATTRS_SYSTEM_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
    AttributeInfo { qname: ":lastClr", property_name: Some("LastColor"), type_name: "HexBinaryValue" },
];
static CHILDREN_SYSTEM_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static ATTRS_SCHEME_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_SCHEME_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static ATTRS_PRESET_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_PRESET_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:tint", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:shade", property_name: None },
    ChildInfo { name: "a:CT_ComplementTransform/a:comp", property_name: None },
    ChildInfo { name: "a:CT_InverseTransform/a:inv", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleTransform/a:gray", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedPercentage/a:alpha", property_name: None },
    ChildInfo { name: "a:CT_FixedPercentage/a:alphaOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_PositiveFixedAngle/a:hue", property_name: None },
    ChildInfo { name: "a:CT_Angle/a:hueOff", property_name: None },
    ChildInfo { name: "a:CT_PositivePercentage/a:hueMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:sat", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:satMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lum", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:lumMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:red", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:redMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:green", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:greenMod", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blue", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueOff", property_name: None },
    ChildInfo { name: "a:CT_Percentage/a:blueMod", property_name: None },
    ChildInfo { name: "a:CT_GammaTransform/a:gamma", property_name: None },
    ChildInfo { name: "a:CT_InverseGammaTransform/a:invGamma", property_name: None },
];
static ATTRS_SHAPE3_D_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":z", property_name: Some("Z"), type_name: "Int64Value" },
    AttributeInfo { qname: ":extrusionH", property_name: Some("ExtrusionHeight"), type_name: "Int64Value" },
    AttributeInfo { qname: ":contourW", property_name: Some("ContourWidth"), type_name: "Int64Value" },
    AttributeInfo { qname: ":prstMaterial", property_name: Some("PresetMaterial"), type_name: "EnumValue" },
];
static CHILDREN_SHAPE3_D_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Bevel/a:bevelT", property_name: Some("BevelTop") },
    ChildInfo { name: "a:CT_Bevel/a:bevelB", property_name: Some("BevelBottom") },
    ChildInfo { name: "a:CT_Color/a:extrusionClr", property_name: Some("ExtrusionColor") },
    ChildInfo { name: "a:CT_Color/a:contourClr", property_name: Some("ContourColor") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FLAT_TEXT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":z", property_name: Some("Z"), type_name: "Int64Value" },
];
static ATTRS_LINEAR_GRADIENT_FILL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ang", property_name: Some("Angle"), type_name: "Int32Value" },
    AttributeInfo { qname: ":scaled", property_name: Some("Scaled"), type_name: "BooleanValue" },
];
static ATTRS_PATH_GRADIENT_FILL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":path", property_name: Some("Path"), type_name: "EnumValue" },
];
static CHILDREN_PATH_GRADIENT_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_RelativeRect/a:fillToRect", property_name: Some("FillToRectangle") },
];
static ATTRS_TILE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tx", property_name: Some("HorizontalOffset"), type_name: "Int64Value" },
    AttributeInfo { qname: ":ty", property_name: Some("VerticalOffset"), type_name: "Int64Value" },
    AttributeInfo { qname: ":sx", property_name: Some("HorizontalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sy", property_name: Some("VerticalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":flip", property_name: Some("Flip"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_STRETCH: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_RelativeRect/a:fillRect", property_name: Some("FillRectangle") },
];
static CHILDREN_SOLID_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_GRADIENT_FILL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":flip", property_name: Some("Flip"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rotWithShape", property_name: Some("RotateWithShape"), type_name: "BooleanValue" },
];
static CHILDREN_GRADIENT_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GradientStopList/a:gsLst", property_name: Some("GradientStopList") },
    ChildInfo { name: "a:CT_LinearShadeProperties/a:lin", property_name: None },
    ChildInfo { name: "a:CT_PathShadeProperties/a:path", property_name: None },
    ChildInfo { name: "a:CT_RelativeRect/a:tileRect", property_name: None },
];
static ATTRS_BLIP_FILL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dpi", property_name: Some("Dpi"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rotWithShape", property_name: Some("RotateWithShape"), type_name: "BooleanValue" },
];
static CHILDREN_BLIP_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Blip/a:blip", property_name: Some("Blip") },
    ChildInfo { name: "a:CT_RelativeRect/a:srcRect", property_name: Some("SourceRectangle") },
    ChildInfo { name: "a:CT_TileInfoProperties/a:tile", property_name: None },
    ChildInfo { name: "a:CT_StretchInfoProperties/a:stretch", property_name: None },
];
static ATTRS_PATTERN_FILL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
];
static CHILDREN_PATTERN_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color/a:fgClr", property_name: Some("ForegroundColor") },
    ChildInfo { name: "a:CT_Color/a:bgClr", property_name: Some("BackgroundColor") },
];
static ATTRS_EFFECT_CONTAINER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_EFFECT_CONTAINER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectContainer/a:cont", property_name: None },
    ChildInfo { name: "a:CT_EffectReference/a:effect", property_name: None },
    ChildInfo { name: "a:CT_AlphaBiLevelEffect/a:alphaBiLevel", property_name: None },
    ChildInfo { name: "a:CT_AlphaCeilingEffect/a:alphaCeiling", property_name: None },
    ChildInfo { name: "a:CT_AlphaFloorEffect/a:alphaFloor", property_name: None },
    ChildInfo { name: "a:CT_AlphaInverseEffect/a:alphaInv", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateEffect/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateFixedEffect/a:alphaModFix", property_name: None },
    ChildInfo { name: "a:CT_AlphaOutsetEffect/a:alphaOutset", property_name: None },
    ChildInfo { name: "a:CT_AlphaReplaceEffect/a:alphaRepl", property_name: None },
    ChildInfo { name: "a:CT_BiLevelEffect/a:biLevel", property_name: None },
    ChildInfo { name: "a:CT_BlendEffect/a:blend", property_name: None },
    ChildInfo { name: "a:CT_BlurEffect/a:blur", property_name: None },
    ChildInfo { name: "a:CT_ColorChangeEffect/a:clrChange", property_name: None },
    ChildInfo { name: "a:CT_ColorReplaceEffect/a:clrRepl", property_name: None },
    ChildInfo { name: "a:CT_DuotoneEffect/a:duotone", property_name: None },
    ChildInfo { name: "a:CT_FillEffect/a:fill", property_name: None },
    ChildInfo { name: "a:CT_FillOverlayEffect/a:fillOverlay", property_name: None },
    ChildInfo { name: "a:CT_GlowEffect/a:glow", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleEffect/a:grayscl", property_name: None },
    ChildInfo { name: "a:CT_HSLEffect/a:hsl", property_name: None },
    ChildInfo { name: "a:CT_InnerShadowEffect/a:innerShdw", property_name: None },
    ChildInfo { name: "a:CT_LuminanceEffect/a:lum", property_name: None },
    ChildInfo { name: "a:CT_OuterShadowEffect/a:outerShdw", property_name: None },
    ChildInfo { name: "a:CT_PresetShadowEffect/a:prstShdw", property_name: None },
    ChildInfo { name: "a:CT_ReflectionEffect/a:reflection", property_name: None },
    ChildInfo { name: "a:CT_RelativeOffsetEffect/a:relOff", property_name: None },
    ChildInfo { name: "a:CT_SoftEdgesEffect/a:softEdge", property_name: None },
    ChildInfo { name: "a:CT_TintEffect/a:tint", property_name: None },
    ChildInfo { name: "a:CT_TransformEffect/a:xfrm", property_name: None },
];
static ATTRS_EFFECT_DAG: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_EFFECT_DAG: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectContainer/a:cont", property_name: None },
    ChildInfo { name: "a:CT_EffectReference/a:effect", property_name: None },
    ChildInfo { name: "a:CT_AlphaBiLevelEffect/a:alphaBiLevel", property_name: None },
    ChildInfo { name: "a:CT_AlphaCeilingEffect/a:alphaCeiling", property_name: None },
    ChildInfo { name: "a:CT_AlphaFloorEffect/a:alphaFloor", property_name: None },
    ChildInfo { name: "a:CT_AlphaInverseEffect/a:alphaInv", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateEffect/a:alphaMod", property_name: None },
    ChildInfo { name: "a:CT_AlphaModulateFixedEffect/a:alphaModFix", property_name: None },
    ChildInfo { name: "a:CT_AlphaOutsetEffect/a:alphaOutset", property_name: None },
    ChildInfo { name: "a:CT_AlphaReplaceEffect/a:alphaRepl", property_name: None },
    ChildInfo { name: "a:CT_BiLevelEffect/a:biLevel", property_name: None },
    ChildInfo { name: "a:CT_BlendEffect/a:blend", property_name: None },
    ChildInfo { name: "a:CT_BlurEffect/a:blur", property_name: None },
    ChildInfo { name: "a:CT_ColorChangeEffect/a:clrChange", property_name: None },
    ChildInfo { name: "a:CT_ColorReplaceEffect/a:clrRepl", property_name: None },
    ChildInfo { name: "a:CT_DuotoneEffect/a:duotone", property_name: None },
    ChildInfo { name: "a:CT_FillEffect/a:fill", property_name: None },
    ChildInfo { name: "a:CT_FillOverlayEffect/a:fillOverlay", property_name: None },
    ChildInfo { name: "a:CT_GlowEffect/a:glow", property_name: None },
    ChildInfo { name: "a:CT_GrayscaleEffect/a:grayscl", property_name: None },
    ChildInfo { name: "a:CT_HSLEffect/a:hsl", property_name: None },
    ChildInfo { name: "a:CT_InnerShadowEffect/a:innerShdw", property_name: None },
    ChildInfo { name: "a:CT_LuminanceEffect/a:lum", property_name: None },
    ChildInfo { name: "a:CT_OuterShadowEffect/a:outerShdw", property_name: None },
    ChildInfo { name: "a:CT_PresetShadowEffect/a:prstShdw", property_name: None },
    ChildInfo { name: "a:CT_ReflectionEffect/a:reflection", property_name: None },
    ChildInfo { name: "a:CT_RelativeOffsetEffect/a:relOff", property_name: None },
    ChildInfo { name: "a:CT_SoftEdgesEffect/a:softEdge", property_name: None },
    ChildInfo { name: "a:CT_TintEffect/a:tint", property_name: None },
    ChildInfo { name: "a:CT_TransformEffect/a:xfrm", property_name: None },
];
static ATTRS_EFFECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
];
static ATTRS_ALPHA_BI_LEVEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":thresh", property_name: Some("Threshold"), type_name: "Int32Value" },
];
static CHILDREN_ALPHA_INVERSE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_ALPHA_MODULATION_EFFECT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectContainer/a:cont", property_name: Some("EffectContainer") },
];
static ATTRS_ALPHA_MODULATION_FIXED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":amt", property_name: Some("Amount"), type_name: "Int32Value" },
];
static ATTRS_ALPHA_OUTSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rad", property_name: Some("Radius"), type_name: "Int64Value" },
];
static ATTRS_ALPHA_REPLACE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":a", property_name: Some("Alpha"), type_name: "Int32Value" },
];
static ATTRS_BI_LEVEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":thresh", property_name: Some("Threshold"), type_name: "Int32Value" },
];
static ATTRS_BLEND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":blend", property_name: Some("BlendMode"), type_name: "EnumValue" },
];
static CHILDREN_BLEND: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectContainer/a:cont", property_name: Some("EffectContainer") },
];
static ATTRS_BLUR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rad", property_name: Some("Radius"), type_name: "Int64Value" },
    AttributeInfo { qname: ":grow", property_name: Some("Grow"), type_name: "BooleanValue" },
];
static ATTRS_COLOR_CHANGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":useA", property_name: Some("UseAlpha"), type_name: "BooleanValue" },
];
static CHILDREN_COLOR_CHANGE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color/a:clrFrom", property_name: Some("ColorFrom") },
    ChildInfo { name: "a:CT_Color/a:clrTo", property_name: Some("ColorTo") },
];
static CHILDREN_COLOR_REPLACEMENT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_DUOTONE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static CHILDREN_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: Some("NoFill") },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: Some("SolidFill") },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: Some("GradientFill") },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: Some("PatternFill") },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: Some("GroupFill") },
];
static ATTRS_FILL_OVERLAY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":blend", property_name: Some("Blend"), type_name: "EnumValue" },
];
static CHILDREN_FILL_OVERLAY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: Some("NoFill") },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: Some("SolidFill") },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: Some("GradientFill") },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: Some("PatternFill") },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: Some("GroupFill") },
];
static ATTRS_GLOW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rad", property_name: Some("Radius"), type_name: "Int64Value" },
];
static CHILDREN_GLOW: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_HSL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":hue", property_name: Some("Hue"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sat", property_name: Some("Saturation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lum", property_name: Some("Luminance"), type_name: "Int32Value" },
];
static ATTRS_INNER_SHADOW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":blurRad", property_name: Some("BlurRadius"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dist", property_name: Some("Distance"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "Int32Value" },
];
static CHILDREN_INNER_SHADOW: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_LUMINANCE_EFFECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bright", property_name: Some("Brightness"), type_name: "Int32Value" },
    AttributeInfo { qname: ":contrast", property_name: Some("Contrast"), type_name: "Int32Value" },
];
static ATTRS_OUTER_SHADOW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":blurRad", property_name: Some("BlurRadius"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dist", property_name: Some("Distance"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sx", property_name: Some("HorizontalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sy", property_name: Some("VerticalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":kx", property_name: Some("HorizontalSkew"), type_name: "Int32Value" },
    AttributeInfo { qname: ":ky", property_name: Some("VerticalSkew"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rotWithShape", property_name: Some("RotateWithShape"), type_name: "BooleanValue" },
];
static CHILDREN_OUTER_SHADOW: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_PRESET_SHADOW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dist", property_name: Some("Distance"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "Int32Value" },
];
static CHILDREN_PRESET_SHADOW: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_REFLECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":blurRad", property_name: Some("BlurRadius"), type_name: "Int64Value" },
    AttributeInfo { qname: ":stA", property_name: Some("StartOpacity"), type_name: "Int32Value" },
    AttributeInfo { qname: ":stPos", property_name: Some("StartPosition"), type_name: "Int32Value" },
    AttributeInfo { qname: ":endA", property_name: Some("EndAlpha"), type_name: "Int32Value" },
    AttributeInfo { qname: ":endPos", property_name: Some("EndPosition"), type_name: "Int32Value" },
    AttributeInfo { qname: ":dist", property_name: Some("Distance"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "Int32Value" },
    AttributeInfo { qname: ":fadeDir", property_name: Some("FadeDirection"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sx", property_name: Some("HorizontalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sy", property_name: Some("VerticalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":kx", property_name: Some("HorizontalSkew"), type_name: "Int32Value" },
    AttributeInfo { qname: ":ky", property_name: Some("VerticalSkew"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rotWithShape", property_name: Some("RotateWithShape"), type_name: "BooleanValue" },
];
static ATTRS_RELATIVE_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tx", property_name: Some("OffsetX"), type_name: "Int32Value" },
    AttributeInfo { qname: ":ty", property_name: Some("OffsetY"), type_name: "Int32Value" },
];
static ATTRS_SOFT_EDGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rad", property_name: Some("Radius"), type_name: "Int64Value" },
];
static ATTRS_TINT_EFFECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":hue", property_name: Some("Hue"), type_name: "Int32Value" },
    AttributeInfo { qname: ":amt", property_name: Some("Amount"), type_name: "Int32Value" },
];
static ATTRS_TRANSFORM_EFFECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sx", property_name: Some("HorizontalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sy", property_name: Some("VerticalRatio"), type_name: "Int32Value" },
    AttributeInfo { qname: ":kx", property_name: Some("HorizontalSkew"), type_name: "Int32Value" },
    AttributeInfo { qname: ":ky", property_name: Some("VerticalSkew"), type_name: "Int32Value" },
    AttributeInfo { qname: ":tx", property_name: Some("HorizontalShift"), type_name: "Int64Value" },
    AttributeInfo { qname: ":ty", property_name: Some("VerticalShift"), type_name: "Int64Value" },
];
static CHILDREN_EFFECT_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_BlurEffect/a:blur", property_name: Some("Blur") },
    ChildInfo { name: "a:CT_FillOverlayEffect/a:fillOverlay", property_name: Some("FillOverlay") },
    ChildInfo { name: "a:CT_GlowEffect/a:glow", property_name: Some("Glow") },
    ChildInfo { name: "a:CT_InnerShadowEffect/a:innerShdw", property_name: Some("InnerShadow") },
    ChildInfo { name: "a:CT_OuterShadowEffect/a:outerShdw", property_name: Some("OuterShadow") },
    ChildInfo { name: "a:CT_PresetShadowEffect/a:prstShdw", property_name: Some("PresetShadow") },
    ChildInfo { name: "a:CT_ReflectionEffect/a:reflection", property_name: Some("Reflection") },
    ChildInfo { name: "a:CT_SoftEdgesEffect/a:softEdge", property_name: Some("SoftEdge") },
];
static CHILDREN_CUSTOM_GEOMETRY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GeomGuideList/a:avLst", property_name: Some("AdjustValueList") },
    ChildInfo { name: "a:CT_GeomGuideList/a:gdLst", property_name: Some("ShapeGuideList") },
    ChildInfo { name: "a:CT_AdjustHandleList/a:ahLst", property_name: Some("AdjustHandleList") },
    ChildInfo { name: "a:CT_ConnectionSiteList/a:cxnLst", property_name: Some("ConnectionSiteList") },
    ChildInfo { name: "a:CT_GeomRect/a:rect", property_name: Some("Rectangle") },
    ChildInfo { name: "a:CT_Path2DList/a:pathLst", property_name: Some("PathList") },
];
static ATTRS_PRESET_GEOMETRY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
];
static CHILDREN_PRESET_GEOMETRY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GeomGuideList/a:avLst", property_name: Some("AdjustValueList") },
];
static ATTRS_PRESET_TEXT_WARP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
];
static CHILDREN_PRESET_TEXT_WARP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GeomGuideList/a:avLst", property_name: Some("AdjustValueList") },
];
static ATTRS_MITER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lim", property_name: Some("Limit"), type_name: "Int32Value" },
];
static ATTRS_PRESET_DASH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static CHILDREN_CUSTOM_DASH: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_DashStop/a:ds", property_name: None },
];
static CHILDREN_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: Some("NoFill") },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: Some("SolidFill") },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: Some("GradientFill") },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: Some("PatternFill") },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: Some("GroupFill") },
];
static ATTRS_FILL_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_FILL_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_EFFECT_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_EFFECT_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_LINE_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_LINE_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_EFFECT_PROPERTIES_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: Some("EffectList") },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: Some("EffectDag") },
];
static CHILDREN_FONTS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextFont/a:latin", property_name: Some("LatinFont") },
    ChildInfo { name: "a:CT_TextFont/a:ea", property_name: Some("EastAsianFont") },
    ChildInfo { name: "a:CT_TextFont/a:cs", property_name: Some("ComplexScriptFont") },
    ChildInfo { name: "a:CT_SupplementalFont/a:font", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_MAJOR_FONT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextFont/a:latin", property_name: Some("LatinFont") },
    ChildInfo { name: "a:CT_TextFont/a:ea", property_name: Some("EastAsianFont") },
    ChildInfo { name: "a:CT_TextFont/a:cs", property_name: Some("ComplexScriptFont") },
    ChildInfo { name: "a:CT_SupplementalFont/a:font", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_MINOR_FONT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextFont/a:latin", property_name: Some("LatinFont") },
    ChildInfo { name: "a:CT_TextFont/a:ea", property_name: Some("EastAsianFont") },
    ChildInfo { name: "a:CT_TextFont/a:cs", property_name: Some("ComplexScriptFont") },
    ChildInfo { name: "a:CT_SupplementalFont/a:font", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_FONT_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "EnumValue" },
];
static CHILDREN_FONT_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_NORMAL_AUTO_FIT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":fontScale", property_name: Some("FontScale"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lnSpcReduction", property_name: Some("LineSpaceReduction"), type_name: "Int32Value" },
];
static CHILDREN_BULLET_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_EXTRUSION_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_CONTOUR_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_COLOR_FROM: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_COLOR_TO: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_FOREGROUND_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_BACKGROUND_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_HIGHLIGHT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_BULLET_SIZE_PERCENTAGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_BULLET_SIZE_POINTS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_BULLET_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":typeface", property_name: Some("Typeface"), type_name: "StringValue" },
    AttributeInfo { qname: ":panose", property_name: Some("Panose"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":pitchFamily", property_name: Some("PitchFamily"), type_name: "SByteValue" },
    AttributeInfo { qname: ":charset", property_name: Some("CharacterSet"), type_name: "SByteValue" },
];
static ATTRS_LATIN_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":typeface", property_name: Some("Typeface"), type_name: "StringValue" },
    AttributeInfo { qname: ":panose", property_name: Some("Panose"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":pitchFamily", property_name: Some("PitchFamily"), type_name: "SByteValue" },
    AttributeInfo { qname: ":charset", property_name: Some("CharacterSet"), type_name: "SByteValue" },
];
static ATTRS_EAST_ASIAN_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":typeface", property_name: Some("Typeface"), type_name: "StringValue" },
    AttributeInfo { qname: ":panose", property_name: Some("Panose"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":pitchFamily", property_name: Some("PitchFamily"), type_name: "SByteValue" },
    AttributeInfo { qname: ":charset", property_name: Some("CharacterSet"), type_name: "SByteValue" },
];
static ATTRS_COMPLEX_SCRIPT_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":typeface", property_name: Some("Typeface"), type_name: "StringValue" },
    AttributeInfo { qname: ":panose", property_name: Some("Panose"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":pitchFamily", property_name: Some("PitchFamily"), type_name: "SByteValue" },
    AttributeInfo { qname: ":charset", property_name: Some("CharacterSet"), type_name: "SByteValue" },
];
static ATTRS_SYMBOL_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":typeface", property_name: Some("Typeface"), type_name: "StringValue" },
    AttributeInfo { qname: ":panose", property_name: Some("Panose"), type_name: "HexBinaryValue" },
    AttributeInfo { qname: ":pitchFamily", property_name: Some("PitchFamily"), type_name: "SByteValue" },
    AttributeInfo { qname: ":charset", property_name: Some("CharacterSet"), type_name: "SByteValue" },
];
static ATTRS_AUTO_NUMBERED_BULLET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":startAt", property_name: Some("StartAt"), type_name: "Int32Value" },
];
static ATTRS_CHARACTER_BULLET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":char", property_name: Some("Char"), type_name: "StringValue" },
];
static CHILDREN_PICTURE_BULLET: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Blip/a:blip", property_name: Some("Blip") },
];
static ATTRS_UNDERLINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_UNDERLINE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_OUTLINE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_OUTLINE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEFT_BORDER_LINE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_LEFT_BORDER_LINE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_RIGHT_BORDER_LINE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_RIGHT_BORDER_LINE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_TOP_BORDER_LINE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_TOP_BORDER_LINE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_BOTTOM_BORDER_LINE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_BOTTOM_BORDER_LINE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_TOP_LEFT_TO_BOTTOM_RIGHT_BORDER_LINE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_TOP_LEFT_TO_BOTTOM_RIGHT_BORDER_LINE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static ATTRS_BOTTOM_LEFT_TO_TOP_RIGHT_BORDER_LINE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_BOTTOM_LEFT_TO_TOP_RIGHT_BORDER_LINE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_PresetLineDashProperties/a:prstDash", property_name: None },
    ChildInfo { name: "a:CT_DashStopList/a:custDash", property_name: None },
    ChildInfo { name: "a:CT_LineJoinRound/a:round", property_name: None },
    ChildInfo { name: "a:CT_LineJoinBevel/a:bevel", property_name: None },
    ChildInfo { name: "a:CT_LineJoinMiterProperties/a:miter", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:headEnd", property_name: None },
    ChildInfo { name: "a:CT_LineEndProperties/a:tailEnd", property_name: None },
    ChildInfo { name: "a:CT_LinePropertiesExtensionList/a:extLst", property_name: None },
];
static CHILDREN_UNDERLINE_FILL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: Some("NoFill") },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: Some("SolidFill") },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: Some("GradientFill") },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: Some("PatternFill") },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: Some("GroupFill") },
];
static CHILDREN_RUN: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextCharacterProperties/a:rPr", property_name: Some("RunProperties") },
    ChildInfo { name: "xsd:string/a:t", property_name: Some("Text") },
];
static CHILDREN_BREAK_: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextCharacterProperties/a:rPr", property_name: Some("RunProperties") },
];
static ATTRS_FIELD: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
];
static CHILDREN_FIELD: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextCharacterProperties/a:rPr", property_name: Some("RunProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:pPr", property_name: Some("ParagraphProperties") },
    ChildInfo { name: "xsd:string/a:t", property_name: Some("Text") },
];
static CHILDREN_GRAPHIC: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObjectData/a:graphicData", property_name: Some("GraphicData") },
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
static ATTRS_THEME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: "thm15:id", property_name: Some("ThemeId"), type_name: "StringValue" },
];
static CHILDREN_THEME: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_BaseStyles/a:themeElements", property_name: Some("ThemeElements") },
    ChildInfo { name: "a:CT_ObjectStyleDefaults/a:objectDefaults", property_name: Some("ObjectDefaults") },
    ChildInfo { name: "a:CT_ColorSchemeList/a:extraClrSchemeLst", property_name: Some("ExtraColorSchemeList") },
    ChildInfo { name: "a:CT_CustomColorList/a:custClrLst", property_name: Some("CustomColorList") },
    ChildInfo { name: "a:CT_OfficeStyleSheetExtensionList/a:extLst", property_name: Some("OfficeStyleSheetExtensionList") },
];
static CHILDREN_THEME_OVERRIDE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ColorScheme/a:clrScheme", property_name: Some("ColorScheme") },
    ChildInfo { name: "a:CT_FontScheme/a:fontScheme", property_name: Some("FontScheme") },
    ChildInfo { name: "a:CT_StyleMatrix/a:fmtScheme", property_name: Some("FormatScheme") },
];
static CHILDREN_TABLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableProperties/a:tblPr", property_name: Some("TableProperties") },
    ChildInfo { name: "a:CT_TableGrid/a:tblGrid", property_name: Some("TableGrid") },
    ChildInfo { name: "a:CT_TableRow/a:tr", property_name: None },
];
static ATTRS_TABLE_STYLE_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":def", property_name: Some("Default"), type_name: "StringValue" },
];
static CHILDREN_TABLE_STYLE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyle/a:tblStyle", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_START_TIME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":track", property_name: Some("Track"), type_name: "ByteValue" },
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "UInt32Value" },
];
static ATTRS_END_TIME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":track", property_name: Some("Track"), type_name: "ByteValue" },
    AttributeInfo { qname: ":time", property_name: Some("Time"), type_name: "UInt32Value" },
];
static ATTRS_CUSTOM_COLOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_CUSTOM_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_SUPPLEMENTAL_FONT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":script", property_name: Some("Script"), type_name: "StringValue" },
    AttributeInfo { qname: ":typeface", property_name: Some("Typeface"), type_name: "StringValue" },
];
static CHILDREN_SCENE3_D_TYPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Camera/a:camera", property_name: Some("Camera") },
    ChildInfo { name: "a:CT_LightRig/a:lightRig", property_name: Some("LightRig") },
    ChildInfo { name: "a:CT_Backdrop/a:backdrop", property_name: Some("Backdrop") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_EFFECT_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
];
static CHILDREN_FILL_STYLE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
];
static CHILDREN_LINE_STYLE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: None },
];
static CHILDREN_EFFECT_STYLE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectStyleItem/a:effectStyle", property_name: None },
];
static CHILDREN_BACKGROUND_FILL_STYLE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
];
static ATTRS_COLOR_SCHEME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_COLOR_SCHEME: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Color2/a:dk1", property_name: Some("Dark1Color") },
    ChildInfo { name: "a:CT_Color2/a:lt1", property_name: Some("Light1Color") },
    ChildInfo { name: "a:CT_Color2/a:dk2", property_name: Some("Dark2Color") },
    ChildInfo { name: "a:CT_Color2/a:lt2", property_name: Some("Light2Color") },
    ChildInfo { name: "a:CT_Color2/a:accent1", property_name: Some("Accent1Color") },
    ChildInfo { name: "a:CT_Color2/a:accent2", property_name: Some("Accent2Color") },
    ChildInfo { name: "a:CT_Color2/a:accent3", property_name: Some("Accent3Color") },
    ChildInfo { name: "a:CT_Color2/a:accent4", property_name: Some("Accent4Color") },
    ChildInfo { name: "a:CT_Color2/a:accent5", property_name: Some("Accent5Color") },
    ChildInfo { name: "a:CT_Color2/a:accent6", property_name: Some("Accent6Color") },
    ChildInfo { name: "a:CT_Color2/a:hlink", property_name: Some("Hyperlink") },
    ChildInfo { name: "a:CT_Color2/a:folHlink", property_name: Some("FollowedHyperlinkColor") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FONT_SCHEME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_FONT_SCHEME: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_FontCollection/a:majorFont", property_name: Some("MajorFont") },
    ChildInfo { name: "a:CT_FontCollection/a:minorFont", property_name: Some("MinorFont") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_FORMAT_SCHEME: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_FORMAT_SCHEME: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_FillStyleList/a:fillStyleLst", property_name: Some("FillStyleList") },
    ChildInfo { name: "a:CT_LineStyleList/a:lnStyleLst", property_name: Some("LineStyleList") },
    ChildInfo { name: "a:CT_EffectStyleList/a:effectStyleLst", property_name: Some("EffectStyleList") },
    ChildInfo { name: "a:CT_BackgroundFillStyleList/a:bgFillStyleLst", property_name: Some("BackgroundFillStyleList") },
];
static CHILDREN_DARK1_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_LIGHT1_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_DARK2_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_LIGHT2_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_ACCENT1_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_ACCENT2_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_ACCENT3_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_ACCENT4_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_ACCENT5_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_ACCENT6_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_HYPERLINK: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_FOLLOWED_HYPERLINK_COLOR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_SCALE_X: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Numerator"), type_name: "Int32Value" },
    AttributeInfo { qname: ":d", property_name: Some("Denominator"), type_name: "Int32Value" },
];
static ATTRS_SCALE_Y: &[AttributeInfo] = &[
    AttributeInfo { qname: ":n", property_name: Some("Numerator"), type_name: "Int32Value" },
    AttributeInfo { qname: ":d", property_name: Some("Denominator"), type_name: "Int32Value" },
];
static ATTRS_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_CHILD_OFFSET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_EXTENTS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int64Value" },
];
static ATTRS_CHILD_EXTENTS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cx", property_name: Some("Cx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: Some("Cy"), type_name: "Int64Value" },
];
static ATTRS_SHAPE_LOCKS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: Some("NoGrouping"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: Some("NoSelection"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: Some("NoRotation"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: Some("NoChangeAspect"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: Some("NoMove"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: Some("NoResize"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noEditPoints", property_name: Some("NoEditPoints"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noAdjustHandles", property_name: Some("NoAdjustHandles"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeArrowheads", property_name: Some("NoChangeArrowheads"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeShapeType", property_name: Some("NoChangeShapeType"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noTextEdit", property_name: Some("NoTextEdit"), type_name: "BooleanValue" },
];
static CHILDREN_SHAPE_LOCKS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_CONNECTION_SHAPE_LOCKS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: Some("NoGrouping"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: Some("NoSelection"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: Some("NoRotation"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: Some("NoChangeAspect"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: Some("NoMove"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: Some("NoResize"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noEditPoints", property_name: Some("NoEditPoints"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noAdjustHandles", property_name: Some("NoAdjustHandles"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeArrowheads", property_name: Some("NoChangeArrowheads"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeShapeType", property_name: Some("NoChangeShapeType"), type_name: "BooleanValue" },
];
static CHILDREN_CONNECTION_SHAPE_LOCKS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectorLockingExtensionList/a:extLst", property_name: Some("ConnectorLockingExtensionList") },
];
static ATTRS_START_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static ATTRS_END_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static ATTRS_GRAPHIC_FRAME_LOCKS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: Some("NoGrouping"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noDrilldown", property_name: Some("NoDrilldown"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: Some("NoSelection"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: Some("NoChangeAspect"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: Some("NoMove"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: Some("NoResize"), type_name: "BooleanValue" },
];
static CHILDREN_GRAPHIC_FRAME_LOCKS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_GRAPHIC_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: Some("Uri"), type_name: "StringValue" },
];
static CHILDREN_GRAPHIC_DATA: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: None },
    ChildInfo { name: "a:CT_Blip/a:blip", property_name: None },
    ChildInfo { name: "a:CT_OfficeStyleSheet/a:theme", property_name: None },
    ChildInfo { name: "a:CT_BaseStylesOverride/a:themeOverride", property_name: None },
    ChildInfo { name: "a:CT_EmptyElement/a:themeManager", property_name: None },
    ChildInfo { name: "a:CT_Table/a:tbl", property_name: None },
    ChildInfo { name: "a:CT_TableStyleList/a:tblStyleLst", property_name: None },
    ChildInfo { name: "aoe:CT_OEmbedShared/aoe:oembedShared", property_name: None },
    ChildInfo { name: "woe:CT_OEmbed/woe:oembed", property_name: None },
    ChildInfo { name: "aif:CT_ImageFormula/aif:imageFormula", property_name: None },
    ChildInfo { name: "alf:CT_LiveFeedProperties/alf:liveFeedProps", property_name: None },
    ChildInfo { name: "asl:CT_ScriptLink/asl:scriptLink", property_name: None },
    ChildInfo { name: "aclsh:CT_ClassificationOutcome/aclsh:classification", property_name: None },
    ChildInfo { name: "ask:CT_LineSketchStyleProperties/ask:lineSketchStyleProps", property_name: None },
    ChildInfo { name: "a1611:CT_PictureAttributionSourceURL/a1611:picAttrSrcUrl", property_name: None },
    ChildInfo { name: "asvg:CT_SVGBlip/asvg:svgBlip", property_name: None },
    ChildInfo { name: "adec:CT_Decorative/adec:decorative", property_name: None },
    ChildInfo { name: "a16:CT_CreationId/a16:creationId", property_name: None },
    ChildInfo { name: "a16:CT_PredecessorDrawingElementReference/a16:predDERef", property_name: None },
    ChildInfo { name: "a16:CT_ConnectableReferences/a16:cxnDERefs", property_name: None },
    ChildInfo { name: "a16:CT_Identifier/a16:rowId", property_name: None },
    ChildInfo { name: "a16:CT_Identifier/a16:colId", property_name: None },
    ChildInfo { name: "ahyp:CT_HyperlinkColor/ahyp:hlinkClr", property_name: None },
    ChildInfo { name: "wp15:CT_WebVideoPr/wp15:webVideoPr", property_name: None },
    ChildInfo { name: "thm15:CT_ThemeFamily/thm15:themeFamily", property_name: None },
    ChildInfo { name: "a15:CT_BackgroundPr/a15:backgroundPr", property_name: None },
    ChildInfo { name: "a15:CT_NonVisualGroupProps/a15:nonVisualGroupProps", property_name: None },
    ChildInfo { name: "a15:CT_ObjectPr/a15:objectPr", property_name: None },
    ChildInfo { name: "a15:CT_SignatureLine/a15:signatureLine", property_name: None },
    ChildInfo { name: "a14:CT_CameraTool/a14:cameraTool", property_name: None },
    ChildInfo { name: "a14:CT_CompatExt/a14:compatExt", property_name: None },
    ChildInfo { name: "a14:CT_IsGvmlCanvas/a14:isCanvas", property_name: None },
    ChildInfo { name: "a14:CT_GvmlContentPart/a14:contentPart", property_name: None },
    ChildInfo { name: "a14:CT_ShadowObscured/a14:shadowObscured", property_name: None },
    ChildInfo { name: "a:CT_FillProperties/a14:hiddenFill", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a14:hiddenLine", property_name: None },
    ChildInfo { name: "a:CT_EffectProperties/a14:hiddenEffects", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a14:hiddenScene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a14:hiddenSp3d", property_name: None },
    ChildInfo { name: "a14:CT_Photo/a14:imgProps", property_name: None },
    ChildInfo { name: "a14:CT_UseLocalDpi/a14:useLocalDpi", property_name: None },
    ChildInfo { name: "a14:CT_TextMath/a14:m", property_name: None },
    ChildInfo { name: "a:CT_NonVisualDrawingProps/dgm14:cNvPr", property_name: None },
    ChildInfo { name: "dgm14:CT_Boolean/dgm14:recolorImg", property_name: None },
    ChildInfo { name: "dsp:CT_Drawing/dsp:drawing", property_name: None },
    ChildInfo { name: "dsp:CT_DataModelExtBlock/dsp:dataModelExt", property_name: None },
    ChildInfo { name: "dgm:CT_ColorTransform/dgm:colorsDef", property_name: None },
    ChildInfo { name: "dgm:CT_ColorTransformHeader/dgm:colorsDefHdr", property_name: None },
    ChildInfo { name: "dgm:CT_ColorTransformHeaderLst/dgm:colorsDefHdrLst", property_name: None },
    ChildInfo { name: "dgm:CT_DataModelRoot/dgm:dataModel", property_name: None },
    ChildInfo { name: "dgm:CT_DiagramDefinition/dgm:layoutDef", property_name: None },
    ChildInfo { name: "dgm:CT_DiagramDefinitionHeader/dgm:layoutDefHdr", property_name: None },
    ChildInfo { name: "dgm:CT_DiagramDefinitionHeaderLst/dgm:layoutDefHdrLst", property_name: None },
    ChildInfo { name: "dgm:CT_RelIds/dgm:relIds", property_name: None },
    ChildInfo { name: "dgm:CT_StyleDefinition/dgm:styleDef", property_name: None },
    ChildInfo { name: "dgm:CT_StyleDefinitionHeader/dgm:styleDefHdr", property_name: None },
    ChildInfo { name: "dgm:CT_StyleDefinitionHeaderLst/dgm:styleDefHdrLst", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/dgm1612:spPr", property_name: None },
    ChildInfo { name: "a:CT_TextListStyle/dgm1612:lstStyle", property_name: None },
    ChildInfo { name: "dgm1611:CT_NumberDiagramInfoList/dgm1611:autoBuNodeInfoLst", property_name: None },
    ChildInfo { name: "c:CT_ChartSpace/c:chartSpace", property_name: None },
    ChildInfo { name: "cdr:CT_Drawing/c:userShapes", property_name: None },
    ChildInfo { name: "c:CT_RelId/c:chart", property_name: None },
    ChildInfo { name: "c16r3:CT_DataDisplayOptions16/c16r3:dataDisplayOptions16", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/c16:spPr", property_name: None },
    ChildInfo { name: "c:CT_UnsignedInt/c16:explosion", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c16:invertIfNegative", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c16:bubble3D", property_name: None },
    ChildInfo { name: "c:CT_Marker/c16:marker", property_name: None },
    ChildInfo { name: "c:CT_DLbl/c16:dLbl", property_name: None },
    ChildInfo { name: "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions", property_name: None },
    ChildInfo { name: "c16:CT_PivotOptions16/c16:pivotOptions16", property_name: None },
    ChildInfo { name: "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap", property_name: None },
    ChildInfo { name: "c16:CT_ChartUniqueID/c16:uniqueId", property_name: None },
    ChildInfo { name: "c:CT_PivotSource/c15:pivotSource", property_name: None },
    ChildInfo { name: "c:CT_NumFmt/c15:numFmt", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/c15:spPr", property_name: None },
    ChildInfo { name: "c:CT_Layout/c15:layout", property_name: None },
    ChildInfo { name: "c15:CT_FullRef/c15:fullRef", property_name: None },
    ChildInfo { name: "c15:CT_LevelRef/c15:levelRef", property_name: None },
    ChildInfo { name: "c15:CT_FormulaRef/c15:formulaRef", property_name: None },
    ChildInfo { name: "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle", property_name: None },
    ChildInfo { name: "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle", property_name: None },
    ChildInfo { name: "c15:CT_FilteredAreaSer/c15:filteredAreaSeries", property_name: None },
    ChildInfo { name: "c15:CT_FilteredBarSer/c15:filteredBarSeries", property_name: None },
    ChildInfo { name: "c15:CT_FilteredBubbleSer/c15:filteredBubbleSeries", property_name: None },
    ChildInfo { name: "c15:CT_FilteredLineSer/c15:filteredLineSeries", property_name: None },
    ChildInfo { name: "c15:CT_FilteredPieSer/c15:filteredPieSeries", property_name: None },
    ChildInfo { name: "c15:CT_FilteredRadarSer/c15:filteredRadarSeries", property_name: None },
    ChildInfo { name: "c15:CT_FilteredScatterSer/c15:filteredScatterSeries", property_name: None },
    ChildInfo { name: "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries", property_name: None },
    ChildInfo { name: "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange", property_name: None },
    ChildInfo { name: "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions", property_name: None },
    ChildInfo { name: "c15:CT_DataLabelFieldTable/c15:dlblFieldTable", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c15:xForSave", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c15:showDataLabelsRange", property_name: None },
    ChildInfo { name: "c:CT_Tx/c15:tx", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c15:showLeaderLines", property_name: None },
    ChildInfo { name: "c:CT_ChartLines/c15:leaderLines", property_name: None },
    ChildInfo { name: "c:CT_Boolean/c15:autoCat", property_name: None },
    ChildInfo { name: "c14:CT_PivotOptions/c14:pivotOptions", property_name: None },
    ChildInfo { name: "c14:CT_SketchOptions/c14:sketchOptions", property_name: None },
    ChildInfo { name: "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt", property_name: None },
    ChildInfo { name: "c14:CT_Style/c14:style", property_name: None },
    ChildInfo { name: "cdr14:CT_ContentPart/cdr14:contentPart", property_name: None },
    ChildInfo { name: "comp:CT_Compat/comp:legacyDrawing", property_name: None },
    ChildInfo { name: "a:CT_GvmlGroupShape/lc:lockedCanvas", property_name: None },
    ChildInfo { name: "wp:CT_Inline/wp:inline", property_name: None },
    ChildInfo { name: "wp:CT_Anchor/wp:anchor", property_name: None },
    ChildInfo { name: "a:ST_Percentage/wp14:pctPosHOffset", property_name: None },
    ChildInfo { name: "a:ST_Percentage/wp14:pctPosVOffset", property_name: None },
    ChildInfo { name: "wp14:CT_SizeRelH/wp14:sizeRelH", property_name: None },
    ChildInfo { name: "wp14:CT_SizeRelV/wp14:sizeRelV", property_name: None },
    ChildInfo { name: "pic:CT_Picture/pic:pic", property_name: None },
    ChildInfo { name: "a:CT_ShapeStyle/pic14:style", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/pic14:extLst", property_name: None },
    ChildInfo { name: "xdr:CT_Drawing/xdr:wsDr", property_name: None },
    ChildInfo { name: "xdr14:CT_ContentPart/xdr:contentPart", property_name: None },
    ChildInfo { name: "xdr14:CT_ContentPart/xdr14:contentPart", property_name: None },
    ChildInfo { name: "pc:CT_CommentAuthorMonikerList/pc:cmAuthorMkLst", property_name: None },
    ChildInfo { name: "pc:CT_CommentMonikerList/pc:cmMkLst", property_name: None },
    ChildInfo { name: "pc:CT_StringTagMonikerList/pc:tagMkLst", property_name: None },
    ChildInfo { name: "pc:CT_CustomShowMonikerList/pc:custShowMkLst", property_name: None },
    ChildInfo { name: "pc:CT_DocumentMonikerList/pc:docMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SectionMonikerList/pc:sectionMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SlideBaseMonikerList/pc:sldBaseMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst", property_name: None },
    ChildInfo { name: "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SlideMonikerList/pc:sldMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SlidePosMonikerList/pc:sldPosMkLst", property_name: None },
    ChildInfo { name: "pc:CT_NotesMonikerList/pc:notesMkLst", property_name: None },
    ChildInfo { name: "pc:CT_NotesTextMonikerList/pc:notesTxtMkLst", property_name: None },
    ChildInfo { name: "pc:CT_NotesMasterMonikerList/pc:notesMasterMkLst", property_name: None },
    ChildInfo { name: "pc:CT_HandoutMonikerList/pc:handoutMkLst", property_name: None },
    ChildInfo { name: "pc:CT_AnimationEffectMonikerList/pc:animEffectMkLst", property_name: None },
    ChildInfo { name: "pc:CT_AnimationEffectMonikerList/pc:animEffectParentMkLst", property_name: None },
    ChildInfo { name: "pc:CT_OsfTaskPaneAppMonikerList/pc:tkAppMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SummaryZoomMonikerList/pc:tocMkLst", property_name: None },
    ChildInfo { name: "pc:CT_SectionLinkObjMonikerList/pc:sectionLnkObjMkLst", property_name: None },
    ChildInfo { name: "pc:CT_DesignerTagMonikerList/pc:designTagMkLst", property_name: None },
    ChildInfo { name: "pc:CT_CustomXmlPartMonikerList/pc:cXmlMkLst", property_name: None },
    ChildInfo { name: "p:CT_CommentAuthorList/p:cmAuthorLst", property_name: None },
    ChildInfo { name: "p:CT_CommentList/p:cmLst", property_name: None },
    ChildInfo { name: "p:CT_OleObject/p:oleObj", property_name: None },
    ChildInfo { name: "p:CT_Presentation/p:presentation", property_name: None },
    ChildInfo { name: "p:CT_PresentationProperties/p:presentationPr", property_name: None },
    ChildInfo { name: "p:CT_Slide/p:sld", property_name: None },
    ChildInfo { name: "p:CT_SlideLayout/p:sldLayout", property_name: None },
    ChildInfo { name: "p:CT_SlideMaster/p:sldMaster", property_name: None },
    ChildInfo { name: "p:CT_HandoutMaster/p:handoutMaster", property_name: None },
    ChildInfo { name: "p:CT_NotesMaster/p:notesMaster", property_name: None },
    ChildInfo { name: "p:CT_NotesSlide/p:notes", property_name: None },
    ChildInfo { name: "p:CT_SlideSyncProperties/p:sldSyncPr", property_name: None },
    ChildInfo { name: "p:CT_TagList/p:tagLst", property_name: None },
    ChildInfo { name: "p:CT_ViewProperties/p:viewPr", property_name: None },
    ChildInfo { name: "p:CT_ContentPart/p:contentPart", property_name: None },
    ChildInfo { name: "p232:CT_PlaceholderTypeExtension/p232:phTypeExt", property_name: None },
    ChildInfo { name: "p188:CT_AuthorList/p188:authorLst", property_name: None },
    ChildInfo { name: "p188:CT_CommentList/p188:cmLst", property_name: None },
    ChildInfo { name: "p188:CT_CommentRelationship/p188:commentRel", property_name: None },
    ChildInfo { name: "p223:CT_Reactions/p223:reactions", property_name: None },
    ChildInfo { name: "p228:CT_TaskDetails/p228:taskDetails", property_name: None },
    ChildInfo { name: "p1912:CT_TaskHistoryDetails/p1912:taskHistoryDetails", property_name: None },
    ChildInfo { name: "oac:CT_TextBodyPackage/oac:txBodyPkg", property_name: None },
    ChildInfo { name: "oac:CT_GroupCommand/oac:grpCmd", property_name: None },
    ChildInfo { name: "oac:CT_ImgData/oac:imgData", property_name: None },
    ChildInfo { name: "oac:CT_ImgData/oac:origImgData", property_name: None },
    ChildInfo { name: "oac:CT_ImgLink/oac:imgLink", property_name: None },
    ChildInfo { name: "oac:CT_DrawingMonikerList/oac:dgMkLst", property_name: None },
    ChildInfo { name: "oac:CT_DocumentContextMonikerList/oac:dcMkLst", property_name: None },
    ChildInfo { name: "oac:CT_GraphicParentMonikerList/oac:graphicParentMkLst", property_name: None },
    ChildInfo { name: "oac:CT_DrawingElementMonikerList/oac:deMkLst", property_name: None },
    ChildInfo { name: "oac:CT_DrawingElementMonikerList/oac:deMasterMkLst", property_name: None },
    ChildInfo { name: "oac:CT_ShapeMonikerList/oac:spMkLst", property_name: None },
    ChildInfo { name: "oac:CT_GroupShapeMonikerList/oac:grpSpMkLst", property_name: None },
    ChildInfo { name: "oac:CT_GraphicFrameMonikerList/oac:graphicFrameMkLst", property_name: None },
    ChildInfo { name: "oac:CT_ConnectorMonikerList/oac:cxnSpMkLst", property_name: None },
    ChildInfo { name: "oac:CT_PictureMonikerList/oac:picMkLst", property_name: None },
    ChildInfo { name: "oac:CT_InkMonikerList/oac:inkMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TextBodyMonikerList/oac:txBodyMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TextCharRangeMonikerList/oac:txMkLst", property_name: None },
    ChildInfo { name: "oac:CT_HyperlinkMonikerList/oac:hlinkMkLst", property_name: None },
    ChildInfo { name: "oac:CT_Model3DMonikerList/oac:model3DMkLst", property_name: None },
    ChildInfo { name: "oac:CT_ViewSelectionStgList/oac:viewSelLst", property_name: None },
    ChildInfo { name: "oac:CT_EditorSelectionStgList/oac:editorSelLst", property_name: None },
    ChildInfo { name: "oac:CT_DrawingSelectionStgList/oac:drSelLst", property_name: None },
    ChildInfo { name: "oac:CT_TableMonikerList/oac:tblMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TableCellMonikerList/oac:tcMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TableRowMonikerList/oac:trMkLst", property_name: None },
    ChildInfo { name: "oac:CT_TableColumnMonikerList/oac:gridColMkLst", property_name: None },
    ChildInfo { name: "inkml:CT_Ink/inkml:ink", property_name: None },
    ChildInfo { name: "emma:CT_OneOf/emma:one-of", property_name: None },
    ChildInfo { name: "emma:CT_Group/emma:group", property_name: None },
    ChildInfo { name: "emma:CT_Sequence/emma:sequence", property_name: None },
    ChildInfo { name: "emma:CT_EndPoint/emma:endpoint", property_name: None },
    ChildInfo { name: "emma:CT_EndPointInfo/emma:endpoint-info", property_name: None },
    ChildInfo { name: "emma:CT_Info/emma:info", property_name: None },
    ChildInfo { name: "emma:CT_Grammar/emma:grammar", property_name: None },
    ChildInfo { name: "emma:CT_DerivedFrom/emma:derived-from", property_name: None },
    ChildInfo { name: "emma:CT_Node/emma:node", property_name: None },
    ChildInfo { name: "emma:CT_Arc/emma:arc", property_name: None },
    ChildInfo { name: "emma:CT_Lattice/emma:lattice", property_name: None },
    ChildInfo { name: "emma:CT_Literal/emma:literal", property_name: None },
    ChildInfo { name: "emma:CT_Interpretation/emma:interpretation", property_name: None },
    ChildInfo { name: "emma:CT_GroupInfo/emma:group-info", property_name: None },
    ChildInfo { name: "emma:CT_Derivation/emma:derivation", property_name: None },
    ChildInfo { name: "emma:CT_Model/emma:model", property_name: None },
    ChildInfo { name: "emma:CT_Emma/emma:emma", property_name: None },
    ChildInfo { name: "msink:CT_CtxNode/msink:context", property_name: None },
    ChildInfo { name: "p15:CT_PresetTransition/p15:prstTrans", property_name: None },
    ChildInfo { name: "p15:CT_PresenceInfo/p15:presenceInfo", property_name: None },
    ChildInfo { name: "p15:CT_CommentThreading/p15:threadingInfo", property_name: None },
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:sldGuideLst", property_name: None },
    ChildInfo { name: "p15:CT_ExtendedGuideList/p15:notesGuideLst", property_name: None },
    ChildInfo { name: "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased", property_name: None },
    ChildInfo { name: "p14:CT_ContentPartNonVisual/p14:nvContentPartPr", property_name: None },
    ChildInfo { name: "a:CT_Transform2D/p14:xfrm", property_name: None },
    ChildInfo { name: "p:CT_ExtensionListModify/p14:extLst", property_name: None },
    ChildInfo { name: "p14:CT_Media/p14:media", property_name: None },
    ChildInfo { name: "p:CT_SideDirectionTransition/p14:vortex", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:switch", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:flip", property_name: None },
    ChildInfo { name: "p14:CT_RippleTransition/p14:ripple", property_name: None },
    ChildInfo { name: "p:CT_Empty/p14:honeycomb", property_name: None },
    ChildInfo { name: "p14:CT_PrismTransition/p14:prism", property_name: None },
    ChildInfo { name: "p:CT_OrientationTransition/p14:doors", property_name: None },
    ChildInfo { name: "p:CT_OrientationTransition/p14:window", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:ferris", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:gallery", property_name: None },
    ChildInfo { name: "p14:CT_LeftRightDirectionTransition/p14:conveyor", property_name: None },
    ChildInfo { name: "p:CT_SideDirectionTransition/p14:pan", property_name: None },
    ChildInfo { name: "p14:CT_GlitterTransition/p14:glitter", property_name: None },
    ChildInfo { name: "p:CT_InOutTransition/p14:warp", property_name: None },
    ChildInfo { name: "p14:CT_FlyThroughTransition/p14:flythrough", property_name: None },
    ChildInfo { name: "p:CT_Empty/p14:flash", property_name: None },
    ChildInfo { name: "p14:CT_ShredTransition/p14:shred", property_name: None },
    ChildInfo { name: "p14:CT_RevealTransition/p14:reveal", property_name: None },
    ChildInfo { name: "p:CT_WheelTransition/p14:wheelReverse", property_name: None },
    ChildInfo { name: "p14:CT_MediaBookmarkTarget/p14:bmkTgt", property_name: None },
    ChildInfo { name: "p14:CT_SectionProperties/p14:sectionPr", property_name: None },
    ChildInfo { name: "p14:CT_SectionList/p14:sectionLst", property_name: None },
    ChildInfo { name: "p14:CT_BrowseMode/p14:browseMode", property_name: None },
    ChildInfo { name: "a:CT_Color/p14:laserClr", property_name: None },
    ChildInfo { name: "p14:CT_DefaultImageDpi/p14:defaultImageDpi", property_name: None },
    ChildInfo { name: "p14:CT_DiscardImageEditData/p14:discardImageEditData", property_name: None },
    ChildInfo { name: "p14:CT_ShowMediaControls/p14:showMediaCtrls", property_name: None },
    ChildInfo { name: "p14:CT_LaserTraceList/p14:laserTraceLst", property_name: None },
    ChildInfo { name: "p14:CT_RandomId/p14:creationId", property_name: None },
    ChildInfo { name: "p14:CT_RandomId/p14:modId", property_name: None },
    ChildInfo { name: "p14:CT_ShowEventRecordList/p14:showEvtLst", property_name: None },
    ChildInfo { name: "w:CT_Recipients/w:recipients", property_name: None },
    ChildInfo { name: "w:CT_TxbxContent/w:txbxContent", property_name: None },
    ChildInfo { name: "w:CT_Comments/w:comments", property_name: None },
    ChildInfo { name: "w:CT_Footnotes/w:footnotes", property_name: None },
    ChildInfo { name: "w:CT_Endnotes/w:endnotes", property_name: None },
    ChildInfo { name: "w:CT_HdrFtr/w:hdr", property_name: None },
    ChildInfo { name: "w:CT_HdrFtr/w:ftr", property_name: None },
    ChildInfo { name: "w:CT_Settings/w:settings", property_name: None },
    ChildInfo { name: "w:CT_WebSettings/w:webSettings", property_name: None },
    ChildInfo { name: "w:CT_FontsList/w:fonts", property_name: None },
    ChildInfo { name: "w:CT_Numbering/w:numbering", property_name: None },
    ChildInfo { name: "w:CT_Styles/w:styles", property_name: None },
    ChildInfo { name: "w:CT_Document/w:document", property_name: None },
    ChildInfo { name: "w:CT_GlossaryDocument/w:glossaryDocument", property_name: None },
    ChildInfo { name: "w:CT_Color/w15:color", property_name: None },
    ChildInfo { name: "w:CT_DataBinding/w15:dataBinding", property_name: None },
    ChildInfo { name: "w15:CT_SdtAppearance/w15:appearance", property_name: None },
    ChildInfo { name: "w15:CT_CommentsEx/w15:commentsEx", property_name: None },
    ChildInfo { name: "w15:CT_People/w15:people", property_name: None },
    ChildInfo { name: "w15:CT_SdtRepeatedSection/w15:repeatingSection", property_name: None },
    ChildInfo { name: "w:CT_Empty/w15:repeatingSectionItem", property_name: None },
    ChildInfo { name: "w:CT_OnOff/w15:chartTrackingRefBased", property_name: None },
    ChildInfo { name: "w:CT_OnOff/w15:collapsed", property_name: None },
    ChildInfo { name: "w15:CT_Guid/w15:docId", property_name: None },
    ChildInfo { name: "w:CT_DecimalNumber/w15:footnoteColumns", property_name: None },
    ChildInfo { name: "w:CT_OnOff/w15:webExtensionLinked", property_name: None },
    ChildInfo { name: "w:CT_OnOff/w15:webExtensionCreated", property_name: None },
    ChildInfo { name: "w14:CT_WordContentPart/w14:contentPart", property_name: None },
    ChildInfo { name: "w14:CT_LongHexNumber/w14:docId", property_name: None },
    ChildInfo { name: "w14:CT_OnOff/w14:conflictMode", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictInsRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictInsRangeEnd", property_name: None },
    ChildInfo { name: "w:CT_TrackChange/w14:customXmlConflictDelRangeStart", property_name: None },
    ChildInfo { name: "w:CT_Markup/w14:customXmlConflictDelRangeEnd", property_name: None },
    ChildInfo { name: "w14:CT_OnOff/w14:discardImageEditingData", property_name: None },
    ChildInfo { name: "w14:CT_DefaultImageDpi/w14:defaultImageDpi", property_name: None },
    ChildInfo { name: "w:CT_Empty/w14:entityPicker", property_name: None },
    ChildInfo { name: "w14:CT_SdtCheckbox/w14:checkbox", property_name: None },
    ChildInfo { name: "sl:CT_SchemaLibrary/sl:schemaLibrary", property_name: None },
    ChildInfo { name: "m:CT_MathPr/m:mathPr", property_name: None },
    ChildInfo { name: "m:CT_OMathPara/m:oMathPara", property_name: None },
    ChildInfo { name: "m:CT_OMath/m:oMath", property_name: None },
    ChildInfo { name: "v:CT_Shape/v:shape", property_name: None },
    ChildInfo { name: "v:CT_Shapetype/v:shapetype", property_name: None },
    ChildInfo { name: "v:CT_Group/v:group", property_name: None },
    ChildInfo { name: "v:CT_Background/v:background", property_name: None },
    ChildInfo { name: "v:CT_Fill/v:fill", property_name: None },
    ChildInfo { name: "v:CT_Formulas/v:formulas", property_name: None },
    ChildInfo { name: "v:CT_Handles/v:handles", property_name: None },
    ChildInfo { name: "v:CT_ImageData/v:imagedata", property_name: None },
    ChildInfo { name: "v:CT_Path/v:path", property_name: None },
    ChildInfo { name: "v:CT_Textbox/v:textbox", property_name: None },
    ChildInfo { name: "v:CT_Shadow/v:shadow", property_name: None },
    ChildInfo { name: "v:CT_Stroke/v:stroke", property_name: None },
    ChildInfo { name: "v:CT_TextPath/v:textpath", property_name: None },
    ChildInfo { name: "v:CT_Arc/v:arc", property_name: None },
    ChildInfo { name: "v:CT_Curve/v:curve", property_name: None },
    ChildInfo { name: "v:CT_Image/v:image", property_name: None },
    ChildInfo { name: "v:CT_Line/v:line", property_name: None },
    ChildInfo { name: "v:CT_Oval/v:oval", property_name: None },
    ChildInfo { name: "v:CT_PolyLine/v:polyline", property_name: None },
    ChildInfo { name: "v:CT_Rect/v:rect", property_name: None },
    ChildInfo { name: "v:CT_RoundRect/v:roundrect", property_name: None },
    ChildInfo { name: "o:CT_ShapeDefaults/o:shapedefaults", property_name: None },
    ChildInfo { name: "o:CT_ShapeLayout/o:shapelayout", property_name: None },
    ChildInfo { name: "o:CT_SignatureLine/o:signatureline", property_name: None },
    ChildInfo { name: "o:CT_Ink/o:ink", property_name: None },
    ChildInfo { name: "o:CT_Diagram/o:diagram", property_name: None },
    ChildInfo { name: "o:CT_Skew/o:skew", property_name: None },
    ChildInfo { name: "o:CT_Extrusion/o:extrusion", property_name: None },
    ChildInfo { name: "o:CT_Callout/o:callout", property_name: None },
    ChildInfo { name: "o:CT_Lock/o:lock", property_name: None },
    ChildInfo { name: "o:CT_OLEObject/o:OLEObject", property_name: None },
    ChildInfo { name: "o:CT_Complex/o:complex", property_name: None },
    ChildInfo { name: "o:CT_StrokeChild/o:left", property_name: None },
    ChildInfo { name: "o:CT_StrokeChild/o:top", property_name: None },
    ChildInfo { name: "o:CT_StrokeChild/o:right", property_name: None },
    ChildInfo { name: "o:CT_StrokeChild/o:bottom", property_name: None },
    ChildInfo { name: "o:CT_StrokeChild/o:column", property_name: None },
    ChildInfo { name: "o:CT_ClipPath/o:clippath", property_name: None },
    ChildInfo { name: "o:CT_Fill/o:fill", property_name: None },
    ChildInfo { name: "w10:CT_Border/w10:bordertop", property_name: None },
    ChildInfo { name: "w10:CT_Border/w10:borderleft", property_name: None },
    ChildInfo { name: "w10:CT_Border/w10:borderright", property_name: None },
    ChildInfo { name: "w10:CT_Border/w10:borderbottom", property_name: None },
    ChildInfo { name: "w10:CT_Wrap/w10:wrap", property_name: None },
    ChildInfo { name: "w10:CT_AnchorLock/w10:anchorlock", property_name: None },
    ChildInfo { name: "xvml:CT_ClientData/xvml:ClientData", property_name: None },
    ChildInfo { name: "pvml:CT_Empty/pvml:iscomment", property_name: None },
    ChildInfo { name: "pvml:CT_Rel/pvml:textdata", property_name: None },
    ChildInfo { name: "wpc:CT_WordprocessingCanvas/wpc:wpc", property_name: None },
    ChildInfo { name: "wpg:CT_WordprocessingGroup/wpg:wgp", property_name: None },
    ChildInfo { name: "wps:CT_WordprocessingShape/wps:wsp", property_name: None },
    ChildInfo { name: "sle:CT_Slicer/sle:slicer", property_name: None },
    ChildInfo { name: "cs:CT_ColorStyle/cs:colorStyle", property_name: None },
    ChildInfo { name: "cs:CT_ChartStyle/cs:chartStyle", property_name: None },
    ChildInfo { name: "we:CT_OsfWebExtension/we:webextension", property_name: None },
    ChildInfo { name: "we:CT_WebExtensionPartRef/we:webextensionref", property_name: None },
    ChildInfo { name: "tsle:CT_Timeline/tsle:timeslicer", property_name: None },
];
static ATTRS_DIAGRAM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":bldStep", property_name: Some("BuildStep"), type_name: "EnumValue" },
];
static ATTRS_CHART: &[AttributeInfo] = &[
    AttributeInfo { qname: ":seriesIdx", property_name: Some("SeriesIndex"), type_name: "Int32Value" },
    AttributeInfo { qname: ":categoryIdx", property_name: Some("CategoryIndex"), type_name: "Int32Value" },
    AttributeInfo { qname: ":bldStep", property_name: Some("BuildStep"), type_name: "EnumValue" },
];
static ATTRS_BUILD_DIAGRAM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bld", property_name: Some("Build"), type_name: "StringValue" },
    AttributeInfo { qname: ":rev", property_name: Some("ReverseAnimation"), type_name: "BooleanValue" },
];
static ATTRS_BUILD_CHART: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bld", property_name: Some("Build"), type_name: "StringValue" },
    AttributeInfo { qname: ":animBg", property_name: Some("AnimateBackground"), type_name: "BooleanValue" },
];
static CHILDREN_TEXT_BODY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
];
static ATTRS_TRANSFORM2_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":flipH", property_name: Some("HorizontalFlip"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":flipV", property_name: Some("VerticalFlip"), type_name: "BooleanValue" },
];
static CHILDREN_TRANSFORM2_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/a:off", property_name: Some("Offset") },
    ChildInfo { name: "a:CT_PositiveSize2D/a:ext", property_name: Some("Extents") },
];
static ATTRS_NON_VISUAL_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":descr", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: Some("HyperlinkOnClick") },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkHover", property_name: Some("HyperlinkOnHover") },
    ChildInfo { name: "a:CT_NonVisualDrawingPropsExtensionList/a:extLst", property_name: Some("NonVisualDrawingPropertiesExtensionList") },
];
static ATTRS_NON_VISUAL_SHAPE_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":txBox", property_name: Some("TextBox"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeLocking/a:spLocks", property_name: Some("ShapeLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/a:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualDrawingShapeProps/a:cNvSpPr", property_name: Some("NonVisualShapeDrawingProperties") },
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
static CHILDREN_TEXT_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBody/a:txBody", property_name: Some("TextBody") },
    ChildInfo { name: "a:CT_GvmlUseShapeRectangle/a:useSpRect", property_name: None },
    ChildInfo { name: "a:CT_Transform2D/a:xfrm", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_SHAPE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "a:CT_FontReference/a:fontRef", property_name: Some("FontReference") },
];
static CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectorLocking/a:cxnSpLocks", property_name: Some("ConnectionShapeLocks") },
    ChildInfo { name: "a:CT_Connection/a:stCxn", property_name: Some("StartConnection") },
    ChildInfo { name: "a:CT_Connection/a:endCxn", property_name: Some("EndConnection") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/a:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualConnectorProperties/a:cNvCxnSpPr", property_name: Some("NonVisualConnectorShapeDrawingProperties") },
];
static ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":preferRelativeResize", property_name: Some("PreferRelativeResize"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PictureLocking/a:picLocks", property_name: Some("PictureLocks") },
    ChildInfo { name: "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst", property_name: Some("NonVisualPicturePropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_PICTURE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/a:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualPictureProperties/a:cNvPicPr", property_name: Some("NonVisualPictureDrawingProperties") },
];
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks", property_name: Some("GraphicFrameLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/a:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGraphicFrameProperties/a:cNvGraphicFramePr", property_name: Some("NonVisualGraphicFrameDrawingProperties") },
];
static CHILDREN_NON_VISUAL_GROUP_SHAPE_DRAWING_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GroupLocking/a:grpSpLocks", property_name: Some("GroupShapeLocks") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst", property_name: Some("NonVisualGroupDrawingShapePropsExtensionList") },
];
static ATTRS_ROTATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lat", property_name: Some("Latitude"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lon", property_name: Some("Longitude"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rev", property_name: Some("Revolution"), type_name: "Int32Value" },
];
static ATTRS_CAMERA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
    AttributeInfo { qname: ":fov", property_name: Some("FieldOfView"), type_name: "Int32Value" },
    AttributeInfo { qname: ":zoom", property_name: Some("Zoom"), type_name: "Int32Value" },
];
static CHILDREN_CAMERA: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SphereCoords/a:rot", property_name: Some("Rotation") },
];
static ATTRS_LIGHT_RIG: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rig", property_name: Some("Rig"), type_name: "EnumValue" },
    AttributeInfo { qname: ":dir", property_name: Some("Direction"), type_name: "EnumValue" },
];
static CHILDREN_LIGHT_RIG: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_SphereCoords/a:rot", property_name: Some("Rotation") },
];
static CHILDREN_BACKDROP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point3D/a:anchor", property_name: Some("Anchor") },
    ChildInfo { name: "a:CT_Vector3D/a:norm", property_name: Some("Normal") },
    ChildInfo { name: "a:CT_Vector3D/a:up", property_name: Some("UpVector") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_ANCHOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
    AttributeInfo { qname: ":z", property_name: Some("Z"), type_name: "Int64Value" },
];
static ATTRS_NORMAL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dx", property_name: Some("Dx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dy", property_name: Some("Dy"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dz", property_name: Some("Dz"), type_name: "Int64Value" },
];
static ATTRS_UP_VECTOR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dx", property_name: Some("Dx"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dy", property_name: Some("Dy"), type_name: "Int64Value" },
    AttributeInfo { qname: ":dz", property_name: Some("Dz"), type_name: "Int64Value" },
];
static ATTRS_BEVEL_TOP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int64Value" },
    AttributeInfo { qname: ":h", property_name: Some("Height"), type_name: "Int64Value" },
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
];
static ATTRS_BEVEL_BOTTOM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int64Value" },
    AttributeInfo { qname: ":h", property_name: Some("Height"), type_name: "Int64Value" },
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
];
static ATTRS_BEVEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int64Value" },
    AttributeInfo { qname: ":h", property_name: Some("Height"), type_name: "Int64Value" },
    AttributeInfo { qname: ":prst", property_name: Some("Preset"), type_name: "EnumValue" },
];
static ATTRS_FILL_TO_RECTANGLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: Some("Left"), type_name: "Int32Value" },
    AttributeInfo { qname: ":t", property_name: Some("Top"), type_name: "Int32Value" },
    AttributeInfo { qname: ":r", property_name: Some("Right"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bottom"), type_name: "Int32Value" },
];
static ATTRS_TILE_RECTANGLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: Some("Left"), type_name: "Int32Value" },
    AttributeInfo { qname: ":t", property_name: Some("Top"), type_name: "Int32Value" },
    AttributeInfo { qname: ":r", property_name: Some("Right"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bottom"), type_name: "Int32Value" },
];
static ATTRS_FILL_RECTANGLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: Some("Left"), type_name: "Int32Value" },
    AttributeInfo { qname: ":t", property_name: Some("Top"), type_name: "Int32Value" },
    AttributeInfo { qname: ":r", property_name: Some("Right"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bottom"), type_name: "Int32Value" },
];
static ATTRS_SOURCE_RECTANGLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: Some("Left"), type_name: "Int32Value" },
    AttributeInfo { qname: ":t", property_name: Some("Top"), type_name: "Int32Value" },
    AttributeInfo { qname: ":r", property_name: Some("Right"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bottom"), type_name: "Int32Value" },
];
static ATTRS_GRADIENT_STOP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pos", property_name: Some("Position"), type_name: "Int32Value" },
];
static CHILDREN_GRADIENT_STOP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static CHILDREN_GRADIENT_STOP_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GradientStop/a:gs", property_name: None },
];
static ATTRS_SHAPE_GUIDE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":fmla", property_name: Some("Formula"), type_name: "StringValue" },
];
static ATTRS_POSITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "StringValue" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "StringValue" },
];
static ATTRS_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "StringValue" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "StringValue" },
];
static ATTRS_ADJUST_HANDLE_X_Y: &[AttributeInfo] = &[
    AttributeInfo { qname: ":gdRefX", property_name: Some("XAdjustmentGuide"), type_name: "StringValue" },
    AttributeInfo { qname: ":minX", property_name: Some("MinX"), type_name: "StringValue" },
    AttributeInfo { qname: ":maxX", property_name: Some("MaxX"), type_name: "StringValue" },
    AttributeInfo { qname: ":gdRefY", property_name: Some("YAdjustmentGuide"), type_name: "StringValue" },
    AttributeInfo { qname: ":minY", property_name: Some("MinY"), type_name: "StringValue" },
    AttributeInfo { qname: ":maxY", property_name: Some("MaxY"), type_name: "StringValue" },
];
static CHILDREN_ADJUST_HANDLE_X_Y: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AdjPoint2D/a:pos", property_name: Some("Position") },
];
static ATTRS_ADJUST_HANDLE_POLAR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":gdRefR", property_name: Some("RadialAdjustmentGuide"), type_name: "StringValue" },
    AttributeInfo { qname: ":minR", property_name: Some("MinRadial"), type_name: "StringValue" },
    AttributeInfo { qname: ":maxR", property_name: Some("MaxRadial"), type_name: "StringValue" },
    AttributeInfo { qname: ":gdRefAng", property_name: Some("AngleAdjustmentGuide"), type_name: "StringValue" },
    AttributeInfo { qname: ":minAng", property_name: Some("MinAngle"), type_name: "StringValue" },
    AttributeInfo { qname: ":maxAng", property_name: Some("MaxAngle"), type_name: "StringValue" },
];
static CHILDREN_ADJUST_HANDLE_POLAR: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AdjPoint2D/a:pos", property_name: Some("Position") },
];
static ATTRS_CONNECTION_SITE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":ang", property_name: Some("Angle"), type_name: "StringValue" },
];
static CHILDREN_CONNECTION_SITE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AdjPoint2D/a:pos", property_name: Some("Position") },
];
static CHILDREN_MOVE_TO: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AdjPoint2D/a:pt", property_name: Some("Point") },
];
static CHILDREN_LINE_TO: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AdjPoint2D/a:pt", property_name: Some("Point") },
];
static ATTRS_ARC_TO: &[AttributeInfo] = &[
    AttributeInfo { qname: ":wR", property_name: Some("WidthRadius"), type_name: "StringValue" },
    AttributeInfo { qname: ":hR", property_name: Some("HeightRadius"), type_name: "StringValue" },
    AttributeInfo { qname: ":stAng", property_name: Some("StartAngle"), type_name: "StringValue" },
    AttributeInfo { qname: ":swAng", property_name: Some("SwingAngle"), type_name: "StringValue" },
];
static CHILDREN_QUADRATIC_BEZIER_CURVE_TO: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AdjPoint2D/a:pt", property_name: None },
];
static CHILDREN_CUBIC_BEZIER_CURVE_TO: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_AdjPoint2D/a:pt", property_name: None },
];
static ATTRS_PATH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int64Value" },
    AttributeInfo { qname: ":h", property_name: Some("Height"), type_name: "Int64Value" },
    AttributeInfo { qname: ":fill", property_name: Some("Fill"), type_name: "EnumValue" },
    AttributeInfo { qname: ":stroke", property_name: Some("Stroke"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":extrusionOk", property_name: Some("ExtrusionOk"), type_name: "BooleanValue" },
];
static CHILDREN_PATH: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Path2DClose/a:close", property_name: None },
    ChildInfo { name: "a:CT_Path2DMoveTo/a:moveTo", property_name: None },
    ChildInfo { name: "a:CT_Path2DLineTo/a:lnTo", property_name: None },
    ChildInfo { name: "a:CT_Path2DArcTo/a:arcTo", property_name: None },
    ChildInfo { name: "a:CT_Path2DQuadBezierTo/a:quadBezTo", property_name: None },
    ChildInfo { name: "a:CT_Path2DCubicBezierTo/a:cubicBezTo", property_name: None },
];
static CHILDREN_ADJUST_VALUE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GeomGuide/a:gd", property_name: None },
];
static CHILDREN_SHAPE_GUIDE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GeomGuide/a:gd", property_name: None },
];
static CHILDREN_ADJUST_HANDLE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_XYAdjustHandle/a:ahXY", property_name: None },
    ChildInfo { name: "a:CT_PolarAdjustHandle/a:ahPolar", property_name: None },
];
static CHILDREN_CONNECTION_SITE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectionSite/a:cxn", property_name: None },
];
static ATTRS_RECTANGLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: Some("Left"), type_name: "StringValue" },
    AttributeInfo { qname: ":t", property_name: Some("Top"), type_name: "StringValue" },
    AttributeInfo { qname: ":r", property_name: Some("Right"), type_name: "StringValue" },
    AttributeInfo { qname: ":b", property_name: Some("Bottom"), type_name: "StringValue" },
];
static CHILDREN_PATH_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Path2D/a:path", property_name: None },
];
static ATTRS_DASH_STOP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":d", property_name: Some("DashLength"), type_name: "Int32Value" },
    AttributeInfo { qname: ":sp", property_name: Some("SpaceLength"), type_name: "Int32Value" },
];
static ATTRS_TRANSFORM_GROUP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":flipH", property_name: Some("HorizontalFlip"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":flipV", property_name: Some("VerticalFlip"), type_name: "BooleanValue" },
];
static CHILDREN_TRANSFORM_GROUP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Point2D/a:off", property_name: Some("Offset") },
    ChildInfo { name: "a:CT_PositiveSize2D/a:ext", property_name: Some("Extents") },
    ChildInfo { name: "a:CT_Point2D/a:chOff", property_name: Some("ChildOffset") },
    ChildInfo { name: "a:CT_PositiveSize2D/a:chExt", property_name: Some("ChildExtents") },
];
static ATTRS_BODY_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":spcFirstLastPara", property_name: Some("UseParagraphSpacing"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":vertOverflow", property_name: Some("VerticalOverflow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":horzOverflow", property_name: Some("HorizontalOverflow"), type_name: "EnumValue" },
    AttributeInfo { qname: ":vert", property_name: Some("Vertical"), type_name: "EnumValue" },
    AttributeInfo { qname: ":wrap", property_name: Some("Wrap"), type_name: "EnumValue" },
    AttributeInfo { qname: ":lIns", property_name: Some("LeftInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":tIns", property_name: Some("TopInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rIns", property_name: Some("RightInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":bIns", property_name: Some("BottomInset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":numCol", property_name: Some("ColumnCount"), type_name: "Int32Value" },
    AttributeInfo { qname: ":spcCol", property_name: Some("ColumnSpacing"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtlCol", property_name: Some("RightToLeftColumns"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fromWordArt", property_name: Some("FromWordArt"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":anchor", property_name: Some("Anchor"), type_name: "EnumValue" },
    AttributeInfo { qname: ":anchorCtr", property_name: Some("AnchorCenter"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":forceAA", property_name: Some("ForceAntiAlias"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":upright", property_name: Some("UpRight"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":compatLnSpc", property_name: Some("CompatibleLineSpacing"), type_name: "BooleanValue" },
];
static CHILDREN_BODY_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PresetTextShape/a:prstTxWarp", property_name: Some("PresetTextWarp") },
    ChildInfo { name: "a:CT_TextNoAutofit/a:noAutofit", property_name: None },
    ChildInfo { name: "a:CT_TextNormalAutofit/a:normAutofit", property_name: None },
    ChildInfo { name: "a:CT_TextShapeAutofit/a:spAutoFit", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
    ChildInfo { name: "a:CT_FlatText/a:flatTx", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_LIST_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:defPPr", property_name: Some("DefaultParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl1pPr", property_name: Some("Level1ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl2pPr", property_name: Some("Level2ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl3pPr", property_name: Some("Level3ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl4pPr", property_name: Some("Level4ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl5pPr", property_name: Some("Level5ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl6pPr", property_name: Some("Level6ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl7pPr", property_name: Some("Level7ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl8pPr", property_name: Some("Level8ParagraphProperties") },
    ChildInfo { name: "a:CT_TextParagraphProperties/a:lvl9pPr", property_name: Some("Level9ParagraphProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_SHAPE_DEFAULT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/a:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_ShapeStyle/a:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_LINE_DEFAULT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/a:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_ShapeStyle/a:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_TEXT_DEFAULT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeProperties/a:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_ShapeStyle/a:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_OVERRIDE_COLOR_MAPPING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bg1", property_name: Some("Background1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx1", property_name: Some("Text1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bg2", property_name: Some("Background2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx2", property_name: Some("Text2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent1", property_name: Some("Accent1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent2", property_name: Some("Accent2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent3", property_name: Some("Accent3"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent4", property_name: Some("Accent4"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent5", property_name: Some("Accent5"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent6", property_name: Some("Accent6"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hlink", property_name: Some("Hyperlink"), type_name: "EnumValue" },
    AttributeInfo { qname: ":folHlink", property_name: Some("FollowedHyperlink"), type_name: "EnumValue" },
];
static CHILDREN_OVERRIDE_COLOR_MAPPING: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_COLOR_MAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bg1", property_name: Some("Background1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx1", property_name: Some("Text1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":bg2", property_name: Some("Background2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":tx2", property_name: Some("Text2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent1", property_name: Some("Accent1"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent2", property_name: Some("Accent2"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent3", property_name: Some("Accent3"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent4", property_name: Some("Accent4"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent5", property_name: Some("Accent5"), type_name: "EnumValue" },
    AttributeInfo { qname: ":accent6", property_name: Some("Accent6"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hlink", property_name: Some("Hyperlink"), type_name: "EnumValue" },
    AttributeInfo { qname: ":folHlink", property_name: Some("FollowedHyperlink"), type_name: "EnumValue" },
];
static CHILDREN_COLOR_MAP: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_EXTRA_COLOR_SCHEME: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ColorScheme/a:clrScheme", property_name: Some("ColorScheme") },
    ChildInfo { name: "a:CT_ColorMapping/a:clrMap", property_name: Some("ColorMap") },
];
static CHILDREN_THEME_ELEMENTS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ColorScheme/a:clrScheme", property_name: Some("ColorScheme") },
    ChildInfo { name: "a:CT_FontScheme/a:fontScheme", property_name: Some("FontScheme") },
    ChildInfo { name: "a:CT_StyleMatrix/a:fmtScheme", property_name: Some("FormatScheme") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_CELL3_D_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":prstMaterial", property_name: Some("PresetMaterial"), type_name: "EnumValue" },
];
static CHILDREN_CELL3_D_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Bevel/a:bevel", property_name: Some("Bevel") },
    ChildInfo { name: "a:CT_LightRig/a:lightRig", property_name: Some("LightRig") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TABLE_CELL_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marT", property_name: Some("TopMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marB", property_name: Some("BottomMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":vert", property_name: Some("Vertical"), type_name: "EnumValue" },
    AttributeInfo { qname: ":anchor", property_name: Some("Anchor"), type_name: "EnumValue" },
    AttributeInfo { qname: ":anchorCtr", property_name: Some("AnchorCenter"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":horzOverflow", property_name: Some("HorizontalOverflow"), type_name: "EnumValue" },
];
static CHILDREN_TABLE_CELL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:lnL", property_name: Some("LeftBorderLineProperties") },
    ChildInfo { name: "a:CT_LineProperties/a:lnR", property_name: Some("RightBorderLineProperties") },
    ChildInfo { name: "a:CT_LineProperties/a:lnT", property_name: Some("TopBorderLineProperties") },
    ChildInfo { name: "a:CT_LineProperties/a:lnB", property_name: Some("BottomBorderLineProperties") },
    ChildInfo { name: "a:CT_LineProperties/a:lnTlToBr", property_name: Some("TopLeftToBottomRightBorderLineProperties") },
    ChildInfo { name: "a:CT_LineProperties/a:lnBlToTr", property_name: Some("BottomLeftToTopRightBorderLineProperties") },
    ChildInfo { name: "a:CT_Cell3D/a:cell3D", property_name: Some("Cell3DProperties") },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_TABLE_CELL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rowSpan", property_name: Some("RowSpan"), type_name: "Int32Value" },
    AttributeInfo { qname: ":gridSpan", property_name: Some("GridSpan"), type_name: "Int32Value" },
    AttributeInfo { qname: ":hMerge", property_name: Some("HorizontalMerge"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":vMerge", property_name: Some("VerticalMerge"), type_name: "BooleanValue" },
];
static CHILDREN_TABLE_CELL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBody/a:txBody", property_name: Some("TextBody") },
    ChildInfo { name: "a:CT_TableCellProperties/a:tcPr", property_name: Some("TableCellProperties") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TABLE_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":styleId", property_name: Some("StyleId"), type_name: "StringValue" },
    AttributeInfo { qname: ":styleName", property_name: Some("StyleName"), type_name: "StringValue" },
];
static CHILDREN_TABLE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableBackgroundStyle/a:tblBg", property_name: Some("TableBackground") },
    ChildInfo { name: "a:CT_TablePartStyle/a:wholeTbl", property_name: Some("WholeTable") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band1H", property_name: Some("Band1Horizontal") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band2H", property_name: Some("Band2Horizontal") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band1V", property_name: Some("Band1Vertical") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band2V", property_name: Some("Band2Vertical") },
    ChildInfo { name: "a:CT_TablePartStyle/a:lastCol", property_name: Some("LastColumn") },
    ChildInfo { name: "a:CT_TablePartStyle/a:firstCol", property_name: Some("FirstColumn") },
    ChildInfo { name: "a:CT_TablePartStyle/a:lastRow", property_name: Some("LastRow") },
    ChildInfo { name: "a:CT_TablePartStyle/a:seCell", property_name: Some("SoutheastCell") },
    ChildInfo { name: "a:CT_TablePartStyle/a:swCell", property_name: Some("SouthwestCell") },
    ChildInfo { name: "a:CT_TablePartStyle/a:firstRow", property_name: Some("FirstRow") },
    ChildInfo { name: "a:CT_TablePartStyle/a:neCell", property_name: Some("NortheastCell") },
    ChildInfo { name: "a:CT_TablePartStyle/a:nwCell", property_name: Some("NorthwestCell") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TABLE_STYLE_ENTRY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":styleId", property_name: Some("StyleId"), type_name: "StringValue" },
    AttributeInfo { qname: ":styleName", property_name: Some("StyleName"), type_name: "StringValue" },
];
static CHILDREN_TABLE_STYLE_ENTRY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableBackgroundStyle/a:tblBg", property_name: Some("TableBackground") },
    ChildInfo { name: "a:CT_TablePartStyle/a:wholeTbl", property_name: Some("WholeTable") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band1H", property_name: Some("Band1Horizontal") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band2H", property_name: Some("Band2Horizontal") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band1V", property_name: Some("Band1Vertical") },
    ChildInfo { name: "a:CT_TablePartStyle/a:band2V", property_name: Some("Band2Vertical") },
    ChildInfo { name: "a:CT_TablePartStyle/a:lastCol", property_name: Some("LastColumn") },
    ChildInfo { name: "a:CT_TablePartStyle/a:firstCol", property_name: Some("FirstColumn") },
    ChildInfo { name: "a:CT_TablePartStyle/a:lastRow", property_name: Some("LastRow") },
    ChildInfo { name: "a:CT_TablePartStyle/a:seCell", property_name: Some("SoutheastCell") },
    ChildInfo { name: "a:CT_TablePartStyle/a:swCell", property_name: Some("SouthwestCell") },
    ChildInfo { name: "a:CT_TablePartStyle/a:firstRow", property_name: Some("FirstRow") },
    ChildInfo { name: "a:CT_TablePartStyle/a:neCell", property_name: Some("NortheastCell") },
    ChildInfo { name: "a:CT_TablePartStyle/a:nwCell", property_name: Some("NorthwestCell") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_GRID_COLUMN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int64Value" },
];
static CHILDREN_GRID_COLUMN: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TABLE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":firstRow", property_name: Some("FirstRow"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":firstCol", property_name: Some("FirstColumn"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lastRow", property_name: Some("LastRow"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lastCol", property_name: Some("LastColumn"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":bandRow", property_name: Some("BandRow"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":bandCol", property_name: Some("BandColumn"), type_name: "BooleanValue" },
];
static CHILDREN_TABLE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_TableStyle/a:tableStyle", property_name: None },
    ChildInfo { name: "a:ST_Guid/a:tableStyleId", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_TABLE_GRID: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableCol/a:gridCol", property_name: None },
];
static ATTRS_TABLE_ROW: &[AttributeInfo] = &[
    AttributeInfo { qname: ":h", property_name: Some("Height"), type_name: "Int64Value" },
];
static CHILDREN_TABLE_ROW: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableCell/a:tc", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_LEFT_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_RIGHT_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_TOP_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_BOTTOM_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_INSIDE_HORIZONTAL_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_INSIDE_VERTICAL_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_TOP_LEFT_TO_BOTTOM_RIGHT_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_TOP_RIGHT_TO_BOTTOM_LEFT_BORDER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
];
static CHILDREN_TABLE_CELL_BORDERS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:left", property_name: Some("LeftBorder") },
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:right", property_name: Some("RightBorder") },
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:top", property_name: Some("TopBorder") },
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:bottom", property_name: Some("BottomBorder") },
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:insideH", property_name: Some("InsideHorizontalBorder") },
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:insideV", property_name: Some("InsideVerticalBorder") },
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:tl2br", property_name: Some("TopLeftToBottomRightBorder") },
    ChildInfo { name: "a:CT_ThemeableLineStyle/a:tr2bl", property_name: Some("TopRightToBottomLeftBorder") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_TABLE_CELL_TEXT_STYLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":b", property_name: Some("Bold"), type_name: "EnumValue" },
    AttributeInfo { qname: ":i", property_name: Some("Italic"), type_name: "EnumValue" },
];
static CHILDREN_TABLE_CELL_TEXT_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_FontCollection/a:font", property_name: None },
    ChildInfo { name: "a:CT_FontReference/a:fontRef", property_name: None },
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_TABLE_CELL_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableCellBorderStyle/a:tcBdr", property_name: Some("TableCellBorders") },
    ChildInfo { name: "a:CT_FillProperties/a:fill", property_name: None },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:fillRef", property_name: None },
    ChildInfo { name: "a:CT_Cell3D/a:cell3D", property_name: None },
];
static CHILDREN_TABLE_BACKGROUND: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_FillProperties/a:fill", property_name: None },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:fillRef", property_name: None },
    ChildInfo { name: "a:CT_EffectProperties/a:effect", property_name: None },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:effectRef", property_name: None },
];
static CHILDREN_WHOLE_TABLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_BAND1_HORIZONTAL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_BAND2_HORIZONTAL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_BAND1_VERTICAL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_BAND2_VERTICAL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_LAST_COLUMN: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_FIRST_COLUMN: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_LAST_ROW: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_SOUTHEAST_CELL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_SOUTHWEST_CELL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_FIRST_ROW: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_NORTHEAST_CELL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static CHILDREN_NORTHWEST_CELL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TableStyleTextStyle/a:tcTxStyle", property_name: Some("TableCellTextStyle") },
    ChildInfo { name: "a:CT_TableStyleCellStyle/a:tcStyle", property_name: Some("TableCellStyle") },
];
static ATTRS_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_DEFAULT_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_DEFAULT_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL1_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL1_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL2_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL2_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL3_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL3_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL4_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL4_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL5_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL5_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL6_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL6_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL7_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL7_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL8_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL8_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_LEVEL9_PARAGRAPH_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":marL", property_name: Some("LeftMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":marR", property_name: Some("RightMargin"), type_name: "Int32Value" },
    AttributeInfo { qname: ":lvl", property_name: Some("Level"), type_name: "Int32Value" },
    AttributeInfo { qname: ":indent", property_name: Some("Indent"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":defTabSz", property_name: Some("DefaultTabSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":rtl", property_name: Some("RightToLeft"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":eaLnBrk", property_name: Some("EastAsianLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":fontAlgn", property_name: Some("FontAlignment"), type_name: "EnumValue" },
    AttributeInfo { qname: ":latinLnBrk", property_name: Some("LatinLineBreak"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":hangingPunct", property_name: Some("Height"), type_name: "BooleanValue" },
];
static CHILDREN_LEVEL9_PARAGRAPH_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacing/a:lnSpc", property_name: Some("LineSpacing") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcBef", property_name: Some("SpaceBefore") },
    ChildInfo { name: "a:CT_TextSpacing/a:spcAft", property_name: Some("SpaceAfter") },
    ChildInfo { name: "a:CT_TextBulletColorFollowText/a:buClrTx", property_name: None },
    ChildInfo { name: "a:CT_Color/a:buClr", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizeFollowText/a:buSzTx", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePercent/a:buSzPct", property_name: None },
    ChildInfo { name: "a:CT_TextBulletSizePoint/a:buSzPts", property_name: None },
    ChildInfo { name: "a:CT_TextBulletTypefaceFollowText/a:buFontTx", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:buFont", property_name: None },
    ChildInfo { name: "a:CT_TextNoBullet/a:buNone", property_name: None },
    ChildInfo { name: "a:CT_TextAutonumberBullet/a:buAutoNum", property_name: None },
    ChildInfo { name: "a:CT_TextCharBullet/a:buChar", property_name: None },
    ChildInfo { name: "a:CT_TextBlipBullet/a:buBlip", property_name: None },
    ChildInfo { name: "a:CT_TextTabStopList/a:tabLst", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:defRPr", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_END_PARAGRAPH_RUN_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":kumimoji", property_name: Some("Kumimoji"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":altLang", property_name: Some("AlternativeLanguage"), type_name: "StringValue" },
    AttributeInfo { qname: ":sz", property_name: Some("FontSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bold"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":i", property_name: Some("Italic"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":u", property_name: Some("Underline"), type_name: "EnumValue" },
    AttributeInfo { qname: ":strike", property_name: Some("Strike"), type_name: "EnumValue" },
    AttributeInfo { qname: ":kern", property_name: Some("Kerning"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("Capital"), type_name: "EnumValue" },
    AttributeInfo { qname: ":spc", property_name: Some("Spacing"), type_name: "Int32Value" },
    AttributeInfo { qname: ":normalizeH", property_name: Some("NormalizeHeight"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":baseline", property_name: Some("Baseline"), type_name: "Int32Value" },
    AttributeInfo { qname: ":noProof", property_name: Some("NoProof"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dirty", property_name: Some("Dirty"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":err", property_name: Some("SpellingError"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtClean", property_name: Some("SmartTagClean"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtId", property_name: Some("SmartTagId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":bmk", property_name: Some("Bookmark"), type_name: "StringValue" },
];
static CHILDREN_END_PARAGRAPH_RUN_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Color/a:highlight", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineLineFollowText/a:uLnTx", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:uLn", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillFollowText/a:uFillTx", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillGroupWrapper/a:uFill", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:latin", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:ea", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:cs", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:sym", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkMouseOver", property_name: None },
    ChildInfo { name: "a:CT_Bool/a:rtl", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_RUN_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":kumimoji", property_name: Some("Kumimoji"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":altLang", property_name: Some("AlternativeLanguage"), type_name: "StringValue" },
    AttributeInfo { qname: ":sz", property_name: Some("FontSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bold"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":i", property_name: Some("Italic"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":u", property_name: Some("Underline"), type_name: "EnumValue" },
    AttributeInfo { qname: ":strike", property_name: Some("Strike"), type_name: "EnumValue" },
    AttributeInfo { qname: ":kern", property_name: Some("Kerning"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("Capital"), type_name: "EnumValue" },
    AttributeInfo { qname: ":spc", property_name: Some("Spacing"), type_name: "Int32Value" },
    AttributeInfo { qname: ":normalizeH", property_name: Some("NormalizeHeight"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":baseline", property_name: Some("Baseline"), type_name: "Int32Value" },
    AttributeInfo { qname: ":noProof", property_name: Some("NoProof"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dirty", property_name: Some("Dirty"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":err", property_name: Some("SpellingError"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtClean", property_name: Some("SmartTagClean"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtId", property_name: Some("SmartTagId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":bmk", property_name: Some("Bookmark"), type_name: "StringValue" },
];
static CHILDREN_RUN_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Color/a:highlight", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineLineFollowText/a:uLnTx", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:uLn", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillFollowText/a:uFillTx", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillGroupWrapper/a:uFill", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:latin", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:ea", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:cs", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:sym", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkMouseOver", property_name: None },
    ChildInfo { name: "a:CT_Bool/a:rtl", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_DEFAULT_RUN_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":kumimoji", property_name: Some("Kumimoji"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":altLang", property_name: Some("AlternativeLanguage"), type_name: "StringValue" },
    AttributeInfo { qname: ":sz", property_name: Some("FontSize"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("Bold"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":i", property_name: Some("Italic"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":u", property_name: Some("Underline"), type_name: "EnumValue" },
    AttributeInfo { qname: ":strike", property_name: Some("Strike"), type_name: "EnumValue" },
    AttributeInfo { qname: ":kern", property_name: Some("Kerning"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("Capital"), type_name: "EnumValue" },
    AttributeInfo { qname: ":spc", property_name: Some("Spacing"), type_name: "Int32Value" },
    AttributeInfo { qname: ":normalizeH", property_name: Some("NormalizeHeight"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":baseline", property_name: Some("Baseline"), type_name: "Int32Value" },
    AttributeInfo { qname: ":noProof", property_name: Some("NoProof"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":dirty", property_name: Some("Dirty"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":err", property_name: Some("SpellingError"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtClean", property_name: Some("SmartTagClean"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":smtId", property_name: Some("SmartTagId"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":bmk", property_name: Some("Bookmark"), type_name: "StringValue" },
];
static CHILDREN_DEFAULT_RUN_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Color/a:highlight", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineLineFollowText/a:uLnTx", property_name: None },
    ChildInfo { name: "a:CT_LineProperties/a:uLn", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillFollowText/a:uFillTx", property_name: None },
    ChildInfo { name: "a:CT_TextUnderlineFillGroupWrapper/a:uFill", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:latin", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:ea", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:cs", property_name: None },
    ChildInfo { name: "a:CT_TextFont/a:sym", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: None },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkMouseOver", property_name: None },
    ChildInfo { name: "a:CT_Bool/a:rtl", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_PARAGRAPH: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextParagraphProperties/a:pPr", property_name: Some("ParagraphProperties") },
    ChildInfo { name: "a:CT_RegularTextRun/a:r", property_name: None },
    ChildInfo { name: "a:CT_TextLineBreak/a:br", property_name: None },
    ChildInfo { name: "a:CT_TextField/a:fld", property_name: None },
    ChildInfo { name: "a14:CT_TextMath/a14:m", property_name: None },
    ChildInfo { name: "a:CT_TextCharacterProperties/a:endParaRPr", property_name: None },
];
static ATTRS_TAB_STOP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":pos", property_name: Some("Position"), type_name: "Int32Value" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static ATTRS_SPACING_PERCENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_SPACING_POINTS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static CHILDREN_LINE_SPACING: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacingPercent/a:spcPct", property_name: Some("SpacingPercent") },
    ChildInfo { name: "a:CT_TextSpacingPoint/a:spcPts", property_name: Some("SpacingPoints") },
];
static CHILDREN_SPACE_BEFORE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacingPercent/a:spcPct", property_name: Some("SpacingPercent") },
    ChildInfo { name: "a:CT_TextSpacingPoint/a:spcPts", property_name: Some("SpacingPoints") },
];
static CHILDREN_SPACE_AFTER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextSpacingPercent/a:spcPct", property_name: Some("SpacingPercent") },
    ChildInfo { name: "a:CT_TextSpacingPoint/a:spcPts", property_name: Some("SpacingPoints") },
];
static CHILDREN_TAB_STOP_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextTabStop/a:tab", property_name: None },
];
static ATTRS_SHAPE_PROPERTIES_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_SHAPE_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_FillProperties/a14:hiddenFill", property_name: Some("HiddenFillProperties") },
    ChildInfo { name: "a:CT_LineProperties/a14:hiddenLine", property_name: Some("HiddenLineProperties") },
    ChildInfo { name: "a:CT_EffectProperties/a14:hiddenEffects", property_name: Some("HiddenEffectsProperties") },
    ChildInfo { name: "a:CT_Scene3D/a14:hiddenScene3d", property_name: Some("HiddenScene3D") },
    ChildInfo { name: "a:CT_Shape3D/a14:hiddenSp3d", property_name: Some("HiddenShape3D") },
    ChildInfo { name: "a14:CT_ShadowObscured/a14:shadowObscured", property_name: Some("ShadowObscured") },
];
static ATTRS_GVML_GROUP_SHAPE_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_GVML_GROUP_SHAPE_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_IsGvmlCanvas/a14:isCanvas", property_name: Some("IsCanvas") },
];
static CHILDREN_SHAPE_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapePropertiesExtension/a:ext", property_name: None },
];
static CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/a:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/a:cNvGrpSpPr", property_name: Some("NonVisualGroupShapeDrawingProperties") },
];
static ATTRS_VISUAL_GROUP_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_VISUAL_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GroupTransform2D/a:xfrm", property_name: Some("TransformGroup") },
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static CHILDREN_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GvmlShapeNonVisual/a:nvSpPr", property_name: Some("NonVisualShapeProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/a:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_GvmlTextShape/a:txSp", property_name: Some("TextShape") },
    ChildInfo { name: "a:CT_ShapeStyle/a:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_CONNECTION_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GvmlConnectorNonVisual/a:nvCxnSpPr", property_name: Some("NonVisualConnectionShapeProperties") },
    ChildInfo { name: "a:CT_ShapeProperties/a:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/a:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_PICTURE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GvmlPictureNonVisual/a:nvPicPr", property_name: Some("NonVisualPictureProperties") },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_ShapeProperties/a:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/a:style", property_name: Some("ShapeStyle") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_GRAPHIC_FRAME: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GvmlGraphicFrameNonVisual/a:nvGraphicFramePr", property_name: Some("NonVisualGraphicFrameProperties") },
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
    ChildInfo { name: "a:CT_Transform2D/a:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_GROUP_SHAPE: &[ChildInfo] = &[
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
static CHILDREN_GVML_GROUP_SHAPE_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GvmlGroupShapeExtension/a:ext", property_name: None },
];
static ATTRS_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a15:CT_NonVisualGroupProps/a15:nonVisualGroupProps", property_name: Some("NonVisualGroupProperties") },
];
static ATTRS_OFFICE_STYLE_SHEET_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_OFFICE_STYLE_SHEET_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "thm15:CT_ThemeFamily/thm15:themeFamily", property_name: Some("ThemeFamily") },
];
static ATTRS_CONNECTOR_LOCKING_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_CONNECTOR_LOCKING_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GraphicalObject/a:graphic", property_name: Some("Graphic") },
];
static ATTRS_GROUP_SHAPE_LOCKS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: Some("NoGrouping"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noUngrp", property_name: Some("NoUngrouping"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: Some("NoSelection"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: Some("NoRotation"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: Some("NoChangeAspect"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: Some("NoMove"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: Some("NoResize"), type_name: "BooleanValue" },
];
static CHILDREN_GROUP_SHAPE_LOCKS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapePropsExtension/a:ext", property_name: None },
];
static CHILDREN_OBJECT_DEFAULTS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_DefaultShapeDefinition/a:spDef", property_name: Some("ShapeDefault") },
    ChildInfo { name: "a:CT_DefaultShapeDefinition/a:lnDef", property_name: Some("LineDefault") },
    ChildInfo { name: "a:CT_DefaultShapeDefinition/a:txDef", property_name: Some("TextDefault") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_EXTRA_COLOR_SCHEME_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ColorSchemeAndMapping/a:extraClrScheme", property_name: None },
];
static CHILDREN_CUSTOM_COLOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_CustomColor/a:custClr", property_name: None },
];
static CHILDREN_OFFICE_STYLE_SHEET_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeStyleSheetExtension/a:ext", property_name: None },
];
static ATTRS_HYPERLINK_ON_CLICK: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidUrl", property_name: Some("InvalidUrl"), type_name: "StringValue" },
    AttributeInfo { qname: ":action", property_name: Some("Action"), type_name: "StringValue" },
    AttributeInfo { qname: ":tgtFrame", property_name: Some("TargetFrame"), type_name: "StringValue" },
    AttributeInfo { qname: ":tooltip", property_name: Some("Tooltip"), type_name: "StringValue" },
    AttributeInfo { qname: ":history", property_name: Some("History"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":highlightClick", property_name: Some("HighlightClick"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":endSnd", property_name: Some("EndSound"), type_name: "BooleanValue" },
];
static CHILDREN_HYPERLINK_ON_CLICK: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EmbeddedWAVAudioFile/a:snd", property_name: Some("HyperlinkSound") },
    ChildInfo { name: "a:CT_HyperlinkExtensionList/a:extLst", property_name: Some("HyperlinkExtensionList") },
];
static ATTRS_HYPERLINK_ON_MOUSE_OVER: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidUrl", property_name: Some("InvalidUrl"), type_name: "StringValue" },
    AttributeInfo { qname: ":action", property_name: Some("Action"), type_name: "StringValue" },
    AttributeInfo { qname: ":tgtFrame", property_name: Some("TargetFrame"), type_name: "StringValue" },
    AttributeInfo { qname: ":tooltip", property_name: Some("Tooltip"), type_name: "StringValue" },
    AttributeInfo { qname: ":history", property_name: Some("History"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":highlightClick", property_name: Some("HighlightClick"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":endSnd", property_name: Some("EndSound"), type_name: "BooleanValue" },
];
static CHILDREN_HYPERLINK_ON_MOUSE_OVER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EmbeddedWAVAudioFile/a:snd", property_name: Some("HyperlinkSound") },
    ChildInfo { name: "a:CT_HyperlinkExtensionList/a:extLst", property_name: Some("HyperlinkExtensionList") },
];
static ATTRS_HYPERLINK_ON_HOVER: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:id", property_name: Some("Id"), type_name: "StringValue" },
    AttributeInfo { qname: ":invalidUrl", property_name: Some("InvalidUrl"), type_name: "StringValue" },
    AttributeInfo { qname: ":action", property_name: Some("Action"), type_name: "StringValue" },
    AttributeInfo { qname: ":tgtFrame", property_name: Some("TargetFrame"), type_name: "StringValue" },
    AttributeInfo { qname: ":tooltip", property_name: Some("Tooltip"), type_name: "StringValue" },
    AttributeInfo { qname: ":history", property_name: Some("History"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":highlightClick", property_name: Some("HighlightClick"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":endSnd", property_name: Some("EndSound"), type_name: "BooleanValue" },
];
static CHILDREN_HYPERLINK_ON_HOVER: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EmbeddedWAVAudioFile/a:snd", property_name: Some("HyperlinkSound") },
    ChildInfo { name: "a:CT_HyperlinkExtensionList/a:extLst", property_name: Some("HyperlinkExtensionList") },
];
static ATTRS_RIGHT_TO_LEFT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingPropsExtension/a:ext", property_name: None },
];
static CHILDREN_CONNECTOR_LOCKING_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectorLockingExtension/a:ext", property_name: None },
];
static ATTRS_DATA_MODEL_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_DATA_MODEL_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "dsp:CT_DataModelExtBlock/dsp:dataModelExt", property_name: Some("DataModelExtensionBlock") },
    ChildInfo { name: "dgm14:CT_Boolean/dgm14:recolorImg", property_name: Some("RecolorImages") },
];
static ATTRS_PT_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_PT_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/dgm14:cNvPr", property_name: Some("NonVisualDrawingProperties") },
];
static ATTRS_HYPERLINK_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_HYPERLINK_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "ahyp:CT_HyperlinkColor/ahyp:hlinkClr", property_name: Some("HyperlinkColor") },
];
static CHILDREN_HYPERLINK_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_HyperlinkExtension/a:ext", property_name: None },
];
static ATTRS_LINE_PROPERTIES_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_LINE_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "ask:CT_LineSketchStyleProperties/ask:lineSketchStyleProps", property_name: Some("LineSketchStyleProperties") },
];
static ATTRS_HEAD_END: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "EnumValue" },
    AttributeInfo { qname: ":len", property_name: Some("Length"), type_name: "EnumValue" },
];
static ATTRS_TAIL_END: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "EnumValue" },
    AttributeInfo { qname: ":len", property_name: Some("Length"), type_name: "EnumValue" },
];
static CHILDREN_LINE_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LinePropertiesExtension/a:ext", property_name: None },
];
static ATTRS_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_CompatExt/a14:compatExt", property_name: Some("CompatExtension") },
    ChildInfo { name: "a15:CT_BackgroundPr/a15:backgroundPr", property_name: Some("BackgroundProperties") },
    ChildInfo { name: "a16:CT_CreationId/a16:creationId", property_name: Some("CreationId") },
    ChildInfo { name: "a16:CT_PredecessorDrawingElementReference/a16:predDERef", property_name: Some("PredecessorDrawingElementReference") },
    ChildInfo { name: "adec:CT_Decorative/adec:decorative", property_name: Some("Decorative") },
    ChildInfo { name: "aclsh:CT_ClassificationOutcome/aclsh:classification", property_name: Some("ClassificationOutcome") },
    ChildInfo { name: "asl:CT_ScriptLink/asl:scriptLink", property_name: Some("ScriptLink") },
];
static ATTRS_PICTURE_LOCKS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: Some("NoGrouping"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: Some("NoSelection"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: Some("NoRotation"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: Some("NoChangeAspect"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: Some("NoMove"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: Some("NoResize"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noEditPoints", property_name: Some("NoEditPoints"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noAdjustHandles", property_name: Some("NoAdjustHandles"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeArrowheads", property_name: Some("NoChangeArrowheads"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeShapeType", property_name: Some("NoChangeShapeType"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":noCrop", property_name: Some("NoCrop"), type_name: "BooleanValue" },
];
static CHILDREN_PICTURE_LOCKS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_PICTURE_PROPERTIES_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualPicturePropertiesExtension/a:ext", property_name: None },
];
static ATTRS_NON_VISUAL_PICTURE_PROPERTIES_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_NON_VISUAL_PICTURE_PROPERTIES_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_CameraTool/a14:cameraTool", property_name: Some("CameraTool") },
    ChildInfo { name: "a15:CT_SignatureLine/a15:signatureLine", property_name: Some("SignatureLine") },
    ChildInfo { name: "a15:CT_ObjectPr/a15:objectPr", property_name: Some("ObjectProperties") },
    ChildInfo { name: "alf:CT_LiveFeedProperties/alf:liveFeedProps", property_name: Some("LiveFeedProperties") },
    ChildInfo { name: "aif:CT_ImageFormula/aif:imageFormula", property_name: Some("ImageFormula") },
];
static CHILDREN_BLIP_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_BlipExtension/a:ext", property_name: None },
];
static ATTRS_BLIP_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_BLIP_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_Photo/a14:imgProps", property_name: Some("ImageProperties") },
    ChildInfo { name: "a14:CT_UseLocalDpi/a14:useLocalDpi", property_name: Some("UseLocalDpi") },
    ChildInfo { name: "wp15:CT_WebVideoPr/wp15:webVideoPr", property_name: Some("WebVideoProperty") },
    ChildInfo { name: "asvg:CT_SVGBlip/asvg:svgBlip", property_name: Some("SVGBlip") },
    ChildInfo { name: "a1611:CT_PictureAttributionSourceURL/a1611:picAttrSrcUrl", property_name: Some("PictureAttributionSourceURL") },
    ChildInfo { name: "woe:CT_OEmbed/woe:oembed", property_name: Some("OEmbed") },
    ChildInfo { name: "aoe:CT_OEmbedShared/aoe:oembedShared", property_name: Some("OEmbedShared") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "AudioFromCD", local_name: "audioCd", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_AUDIO_FROM_C_D },
    ElementInfo { class_name: "WaveAudioFile", local_name: "wavAudioFile", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_WAVE_AUDIO_FILE, children: &[] },
    ElementInfo { class_name: "HyperlinkSound", local_name: "snd", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HYPERLINK_SOUND, children: &[] },
    ElementInfo { class_name: "AudioFromFile", local_name: "audioFile", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_AUDIO_FROM_FILE, children: CHILDREN_AUDIO_FROM_FILE },
    ElementInfo { class_name: "VideoFromFile", local_name: "videoFile", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VIDEO_FROM_FILE, children: CHILDREN_VIDEO_FROM_FILE },
    ElementInfo { class_name: "QuickTimeFromFile", local_name: "quickTimeFile", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_QUICK_TIME_FROM_FILE, children: CHILDREN_QUICK_TIME_FROM_FILE },
    ElementInfo { class_name: "Tint", local_name: "tint", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TINT, children: &[] },
    ElementInfo { class_name: "Shade", local_name: "shade", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHADE, children: &[] },
    ElementInfo { class_name: "Alpha", local_name: "alpha", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA, children: &[] },
    ElementInfo { class_name: "Complement", local_name: "comp", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Inverse", local_name: "inv", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Gray", local_name: "gray", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "AlphaOffset", local_name: "alphaOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA_OFFSET, children: &[] },
    ElementInfo { class_name: "AlphaModulation", local_name: "alphaMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA_MODULATION, children: &[] },
    ElementInfo { class_name: "HueModulation", local_name: "hueMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HUE_MODULATION, children: &[] },
    ElementInfo { class_name: "Hue", local_name: "hue", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HUE, children: &[] },
    ElementInfo { class_name: "HueOffset", local_name: "hueOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HUE_OFFSET, children: &[] },
    ElementInfo { class_name: "Saturation", local_name: "sat", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SATURATION, children: &[] },
    ElementInfo { class_name: "SaturationOffset", local_name: "satOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SATURATION_OFFSET, children: &[] },
    ElementInfo { class_name: "SaturationModulation", local_name: "satMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SATURATION_MODULATION, children: &[] },
    ElementInfo { class_name: "Luminance", local_name: "lum", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LUMINANCE, children: &[] },
    ElementInfo { class_name: "LuminanceOffset", local_name: "lumOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LUMINANCE_OFFSET, children: &[] },
    ElementInfo { class_name: "LuminanceModulation", local_name: "lumMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LUMINANCE_MODULATION, children: &[] },
    ElementInfo { class_name: "Red", local_name: "red", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RED, children: &[] },
    ElementInfo { class_name: "RedOffset", local_name: "redOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RED_OFFSET, children: &[] },
    ElementInfo { class_name: "RedModulation", local_name: "redMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RED_MODULATION, children: &[] },
    ElementInfo { class_name: "Green", local_name: "green", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GREEN, children: &[] },
    ElementInfo { class_name: "GreenOffset", local_name: "greenOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GREEN_OFFSET, children: &[] },
    ElementInfo { class_name: "GreenModulation", local_name: "greenMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GREEN_MODULATION, children: &[] },
    ElementInfo { class_name: "Blue", local_name: "blue", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BLUE, children: &[] },
    ElementInfo { class_name: "BlueOffset", local_name: "blueOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BLUE_OFFSET, children: &[] },
    ElementInfo { class_name: "BlueModulation", local_name: "blueMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BLUE_MODULATION, children: &[] },
    ElementInfo { class_name: "Gamma", local_name: "gamma", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "InverseGamma", local_name: "invGamma", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Extension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EXTENSION, children: &[] },
    ElementInfo { class_name: "RgbColorModelPercentage", local_name: "scrgbClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RGB_COLOR_MODEL_PERCENTAGE, children: CHILDREN_RGB_COLOR_MODEL_PERCENTAGE },
    ElementInfo { class_name: "RgbColorModelHex", local_name: "srgbClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RGB_COLOR_MODEL_HEX, children: CHILDREN_RGB_COLOR_MODEL_HEX },
    ElementInfo { class_name: "HslColor", local_name: "hslClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HSL_COLOR, children: CHILDREN_HSL_COLOR },
    ElementInfo { class_name: "SystemColor", local_name: "sysClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SYSTEM_COLOR, children: CHILDREN_SYSTEM_COLOR },
    ElementInfo { class_name: "SchemeColor", local_name: "schemeClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SCHEME_COLOR, children: CHILDREN_SCHEME_COLOR },
    ElementInfo { class_name: "PresetColor", local_name: "prstClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESET_COLOR, children: CHILDREN_PRESET_COLOR },
    ElementInfo { class_name: "Shape3DType", local_name: "sp3d", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE3_D_TYPE, children: CHILDREN_SHAPE3_D_TYPE },
    ElementInfo { class_name: "FlatText", local_name: "flatTx", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FLAT_TEXT, children: &[] },
    ElementInfo { class_name: "LinearGradientFill", local_name: "lin", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LINEAR_GRADIENT_FILL, children: &[] },
    ElementInfo { class_name: "PathGradientFill", local_name: "path", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PATH_GRADIENT_FILL, children: CHILDREN_PATH_GRADIENT_FILL },
    ElementInfo { class_name: "Tile", local_name: "tile", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TILE, children: &[] },
    ElementInfo { class_name: "Stretch", local_name: "stretch", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STRETCH },
    ElementInfo { class_name: "NoFill", local_name: "noFill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SolidFill", local_name: "solidFill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SOLID_FILL },
    ElementInfo { class_name: "GradientFill", local_name: "gradFill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRADIENT_FILL, children: CHILDREN_GRADIENT_FILL },
    ElementInfo { class_name: "BlipFill", local_name: "blipFill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP_FILL, children: CHILDREN_BLIP_FILL },
    ElementInfo { class_name: "PatternFill", local_name: "pattFill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PATTERN_FILL, children: CHILDREN_PATTERN_FILL },
    ElementInfo { class_name: "GroupFill", local_name: "grpFill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "EffectContainer", local_name: "cont", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EFFECT_CONTAINER, children: CHILDREN_EFFECT_CONTAINER },
    ElementInfo { class_name: "EffectDag", local_name: "effectDag", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EFFECT_DAG, children: CHILDREN_EFFECT_DAG },
    ElementInfo { class_name: "Effect", local_name: "effect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EFFECT, children: &[] },
    ElementInfo { class_name: "AlphaBiLevel", local_name: "alphaBiLevel", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA_BI_LEVEL, children: &[] },
    ElementInfo { class_name: "AlphaCeiling", local_name: "alphaCeiling", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "AlphaFloor", local_name: "alphaFloor", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "AlphaInverse", local_name: "alphaInv", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ALPHA_INVERSE },
    ElementInfo { class_name: "AlphaModulationEffect", local_name: "alphaMod", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ALPHA_MODULATION_EFFECT },
    ElementInfo { class_name: "AlphaModulationFixed", local_name: "alphaModFix", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA_MODULATION_FIXED, children: &[] },
    ElementInfo { class_name: "AlphaOutset", local_name: "alphaOutset", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA_OUTSET, children: &[] },
    ElementInfo { class_name: "AlphaReplace", local_name: "alphaRepl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ALPHA_REPLACE, children: &[] },
    ElementInfo { class_name: "BiLevel", local_name: "biLevel", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BI_LEVEL, children: &[] },
    ElementInfo { class_name: "Blend", local_name: "blend", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLEND, children: CHILDREN_BLEND },
    ElementInfo { class_name: "Blur", local_name: "blur", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BLUR, children: &[] },
    ElementInfo { class_name: "ColorChange", local_name: "clrChange", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_CHANGE, children: CHILDREN_COLOR_CHANGE },
    ElementInfo { class_name: "ColorReplacement", local_name: "clrRepl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_REPLACEMENT },
    ElementInfo { class_name: "Duotone", local_name: "duotone", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DUOTONE },
    ElementInfo { class_name: "Fill", local_name: "fill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILL },
    ElementInfo { class_name: "FillOverlay", local_name: "fillOverlay", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FILL_OVERLAY, children: CHILDREN_FILL_OVERLAY },
    ElementInfo { class_name: "Glow", local_name: "glow", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GLOW, children: CHILDREN_GLOW },
    ElementInfo { class_name: "Grayscale", local_name: "grayscl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Hsl", local_name: "hsl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HSL, children: &[] },
    ElementInfo { class_name: "InnerShadow", local_name: "innerShdw", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_INNER_SHADOW, children: CHILDREN_INNER_SHADOW },
    ElementInfo { class_name: "LuminanceEffect", local_name: "lum", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LUMINANCE_EFFECT, children: &[] },
    ElementInfo { class_name: "OuterShadow", local_name: "outerShdw", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OUTER_SHADOW, children: CHILDREN_OUTER_SHADOW },
    ElementInfo { class_name: "PresetShadow", local_name: "prstShdw", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESET_SHADOW, children: CHILDREN_PRESET_SHADOW },
    ElementInfo { class_name: "Reflection", local_name: "reflection", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_REFLECTION, children: &[] },
    ElementInfo { class_name: "RelativeOffset", local_name: "relOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RELATIVE_OFFSET, children: &[] },
    ElementInfo { class_name: "SoftEdge", local_name: "softEdge", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SOFT_EDGE, children: &[] },
    ElementInfo { class_name: "TintEffect", local_name: "tint", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TINT_EFFECT, children: &[] },
    ElementInfo { class_name: "TransformEffect", local_name: "xfrm", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TRANSFORM_EFFECT, children: &[] },
    ElementInfo { class_name: "EffectList", local_name: "effectLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EFFECT_LIST },
    ElementInfo { class_name: "CustomGeometry", local_name: "custGeom", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_GEOMETRY },
    ElementInfo { class_name: "PresetGeometry", local_name: "prstGeom", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESET_GEOMETRY, children: CHILDREN_PRESET_GEOMETRY },
    ElementInfo { class_name: "PresetTextWarp", local_name: "prstTxWarp", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESET_TEXT_WARP, children: CHILDREN_PRESET_TEXT_WARP },
    ElementInfo { class_name: "Round", local_name: "round", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LineJoinBevel", local_name: "bevel", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Miter", local_name: "miter", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MITER, children: &[] },
    ElementInfo { class_name: "PresetDash", local_name: "prstDash", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PRESET_DASH, children: &[] },
    ElementInfo { class_name: "CustomDash", local_name: "custDash", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_DASH },
    ElementInfo { class_name: "FillProperties", local_name: "fill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILL_PROPERTIES },
    ElementInfo { class_name: "FillReference", local_name: "fillRef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FILL_REFERENCE, children: CHILDREN_FILL_REFERENCE },
    ElementInfo { class_name: "EffectReference", local_name: "effectRef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EFFECT_REFERENCE, children: CHILDREN_EFFECT_REFERENCE },
    ElementInfo { class_name: "LineReference", local_name: "lnRef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_REFERENCE, children: CHILDREN_LINE_REFERENCE },
    ElementInfo { class_name: "EffectPropertiesType", local_name: "effect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EFFECT_PROPERTIES_TYPE },
    ElementInfo { class_name: "Fonts", local_name: "font", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FONTS },
    ElementInfo { class_name: "MajorFont", local_name: "majorFont", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MAJOR_FONT },
    ElementInfo { class_name: "MinorFont", local_name: "minorFont", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MINOR_FONT },
    ElementInfo { class_name: "FontReference", local_name: "fontRef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FONT_REFERENCE, children: CHILDREN_FONT_REFERENCE },
    ElementInfo { class_name: "NoAutoFit", local_name: "noAutofit", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NormalAutoFit", local_name: "normAutofit", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NORMAL_AUTO_FIT, children: &[] },
    ElementInfo { class_name: "ShapeAutoFit", local_name: "spAutoFit", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BulletColorText", local_name: "buClrTx", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BulletColor", local_name: "buClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BULLET_COLOR },
    ElementInfo { class_name: "ExtrusionColor", local_name: "extrusionClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTRUSION_COLOR },
    ElementInfo { class_name: "ContourColor", local_name: "contourClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONTOUR_COLOR },
    ElementInfo { class_name: "ColorFrom", local_name: "clrFrom", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_FROM },
    ElementInfo { class_name: "ColorTo", local_name: "clrTo", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_TO },
    ElementInfo { class_name: "ForegroundColor", local_name: "fgClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FOREGROUND_COLOR },
    ElementInfo { class_name: "BackgroundColor", local_name: "bgClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND_COLOR },
    ElementInfo { class_name: "Highlight", local_name: "highlight", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HIGHLIGHT },
    ElementInfo { class_name: "BulletSizeText", local_name: "buSzTx", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BulletSizePercentage", local_name: "buSzPct", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BULLET_SIZE_PERCENTAGE, children: &[] },
    ElementInfo { class_name: "BulletSizePoints", local_name: "buSzPts", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BULLET_SIZE_POINTS, children: &[] },
    ElementInfo { class_name: "BulletFontText", local_name: "buFontTx", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BulletFont", local_name: "buFont", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BULLET_FONT, children: &[] },
    ElementInfo { class_name: "LatinFont", local_name: "latin", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_LATIN_FONT, children: &[] },
    ElementInfo { class_name: "EastAsianFont", local_name: "ea", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EAST_ASIAN_FONT, children: &[] },
    ElementInfo { class_name: "ComplexScriptFont", local_name: "cs", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMPLEX_SCRIPT_FONT, children: &[] },
    ElementInfo { class_name: "SymbolFont", local_name: "sym", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SYMBOL_FONT, children: &[] },
    ElementInfo { class_name: "NoBullet", local_name: "buNone", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "AutoNumberedBullet", local_name: "buAutoNum", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_AUTO_NUMBERED_BULLET, children: &[] },
    ElementInfo { class_name: "CharacterBullet", local_name: "buChar", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHARACTER_BULLET, children: &[] },
    ElementInfo { class_name: "PictureBullet", local_name: "buBlip", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PICTURE_BULLET },
    ElementInfo { class_name: "UnderlineFollowsText", local_name: "uLnTx", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Underline", local_name: "uLn", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_UNDERLINE, children: CHILDREN_UNDERLINE },
    ElementInfo { class_name: "Outline", local_name: "ln", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OUTLINE, children: CHILDREN_OUTLINE },
    ElementInfo { class_name: "LeftBorderLineProperties", local_name: "lnL", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEFT_BORDER_LINE_PROPERTIES, children: CHILDREN_LEFT_BORDER_LINE_PROPERTIES },
    ElementInfo { class_name: "RightBorderLineProperties", local_name: "lnR", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RIGHT_BORDER_LINE_PROPERTIES, children: CHILDREN_RIGHT_BORDER_LINE_PROPERTIES },
    ElementInfo { class_name: "TopBorderLineProperties", local_name: "lnT", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TOP_BORDER_LINE_PROPERTIES, children: CHILDREN_TOP_BORDER_LINE_PROPERTIES },
    ElementInfo { class_name: "BottomBorderLineProperties", local_name: "lnB", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BOTTOM_BORDER_LINE_PROPERTIES, children: CHILDREN_BOTTOM_BORDER_LINE_PROPERTIES },
    ElementInfo { class_name: "TopLeftToBottomRightBorderLineProperties", local_name: "lnTlToBr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TOP_LEFT_TO_BOTTOM_RIGHT_BORDER_LINE_PROPERTIES, children: CHILDREN_TOP_LEFT_TO_BOTTOM_RIGHT_BORDER_LINE_PROPERTIES },
    ElementInfo { class_name: "BottomLeftToTopRightBorderLineProperties", local_name: "lnBlToTr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BOTTOM_LEFT_TO_TOP_RIGHT_BORDER_LINE_PROPERTIES, children: CHILDREN_BOTTOM_LEFT_TO_TOP_RIGHT_BORDER_LINE_PROPERTIES },
    ElementInfo { class_name: "UnderlineFillText", local_name: "uFillTx", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "UnderlineFill", local_name: "uFill", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_UNDERLINE_FILL },
    ElementInfo { class_name: "Run", local_name: "r", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RUN },
    ElementInfo { class_name: "Break", local_name: "br", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BREAK_ },
    ElementInfo { class_name: "Field", local_name: "fld", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FIELD, children: CHILDREN_FIELD },
    ElementInfo { class_name: "Graphic", local_name: "graphic", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRAPHIC },
    ElementInfo { class_name: "Blip", local_name: "blip", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP, children: CHILDREN_BLIP },
    ElementInfo { class_name: "Theme", local_name: "theme", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_THEME, children: CHILDREN_THEME },
    ElementInfo { class_name: "ThemeOverride", local_name: "themeOverride", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_THEME_OVERRIDE },
    ElementInfo { class_name: "ThemeManager", local_name: "themeManager", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "MasterColorMapping", local_name: "masterClrMapping", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Table", local_name: "tbl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABLE },
    ElementInfo { class_name: "TableStyleList", local_name: "tblStyleLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_STYLE_LIST, children: CHILDREN_TABLE_STYLE_LIST },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "StartTime", local_name: "st", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_START_TIME, children: &[] },
    ElementInfo { class_name: "EndTime", local_name: "end", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_END_TIME, children: &[] },
    ElementInfo { class_name: "CustomColor", local_name: "custClr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CUSTOM_COLOR, children: CHILDREN_CUSTOM_COLOR },
    ElementInfo { class_name: "SupplementalFont", local_name: "font", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SUPPLEMENTAL_FONT, children: &[] },
    ElementInfo { class_name: "Scene3DType", local_name: "scene3d", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCENE3_D_TYPE },
    ElementInfo { class_name: "EffectStyle", local_name: "effectStyle", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EFFECT_STYLE },
    ElementInfo { class_name: "FillStyleList", local_name: "fillStyleLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FILL_STYLE_LIST },
    ElementInfo { class_name: "LineStyleList", local_name: "lnStyleLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_STYLE_LIST },
    ElementInfo { class_name: "EffectStyleList", local_name: "effectStyleLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EFFECT_STYLE_LIST },
    ElementInfo { class_name: "BackgroundFillStyleList", local_name: "bgFillStyleLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND_FILL_STYLE_LIST },
    ElementInfo { class_name: "ColorScheme", local_name: "clrScheme", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_SCHEME, children: CHILDREN_COLOR_SCHEME },
    ElementInfo { class_name: "FontScheme", local_name: "fontScheme", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FONT_SCHEME, children: CHILDREN_FONT_SCHEME },
    ElementInfo { class_name: "FormatScheme", local_name: "fmtScheme", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FORMAT_SCHEME, children: CHILDREN_FORMAT_SCHEME },
    ElementInfo { class_name: "Dark1Color", local_name: "dk1", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DARK1_COLOR },
    ElementInfo { class_name: "Light1Color", local_name: "lt1", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIGHT1_COLOR },
    ElementInfo { class_name: "Dark2Color", local_name: "dk2", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DARK2_COLOR },
    ElementInfo { class_name: "Light2Color", local_name: "lt2", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIGHT2_COLOR },
    ElementInfo { class_name: "Accent1Color", local_name: "accent1", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT1_COLOR },
    ElementInfo { class_name: "Accent2Color", local_name: "accent2", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT2_COLOR },
    ElementInfo { class_name: "Accent3Color", local_name: "accent3", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT3_COLOR },
    ElementInfo { class_name: "Accent4Color", local_name: "accent4", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT4_COLOR },
    ElementInfo { class_name: "Accent5Color", local_name: "accent5", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT5_COLOR },
    ElementInfo { class_name: "Accent6Color", local_name: "accent6", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ACCENT6_COLOR },
    ElementInfo { class_name: "Hyperlink", local_name: "hlink", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HYPERLINK },
    ElementInfo { class_name: "FollowedHyperlinkColor", local_name: "folHlink", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FOLLOWED_HYPERLINK_COLOR },
    ElementInfo { class_name: "ScaleX", local_name: "sx", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SCALE_X, children: &[] },
    ElementInfo { class_name: "ScaleY", local_name: "sy", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SCALE_Y, children: &[] },
    ElementInfo { class_name: "Offset", local_name: "off", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_OFFSET, children: &[] },
    ElementInfo { class_name: "ChildOffset", local_name: "chOff", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHILD_OFFSET, children: &[] },
    ElementInfo { class_name: "Extents", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_EXTENTS, children: &[] },
    ElementInfo { class_name: "ChildExtents", local_name: "chExt", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHILD_EXTENTS, children: &[] },
    ElementInfo { class_name: "ShapeLocks", local_name: "spLocks", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_LOCKS, children: CHILDREN_SHAPE_LOCKS },
    ElementInfo { class_name: "ConnectionShapeLocks", local_name: "cxnSpLocks", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTION_SHAPE_LOCKS, children: CHILDREN_CONNECTION_SHAPE_LOCKS },
    ElementInfo { class_name: "StartConnection", local_name: "stCxn", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_START_CONNECTION, children: &[] },
    ElementInfo { class_name: "EndConnection", local_name: "endCxn", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_END_CONNECTION, children: &[] },
    ElementInfo { class_name: "GraphicFrameLocks", local_name: "graphicFrameLocks", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRAPHIC_FRAME_LOCKS, children: CHILDREN_GRAPHIC_FRAME_LOCKS },
    ElementInfo { class_name: "GraphicData", local_name: "graphicData", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRAPHIC_DATA, children: CHILDREN_GRAPHIC_DATA },
    ElementInfo { class_name: "Diagram", local_name: "dgm", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DIAGRAM, children: &[] },
    ElementInfo { class_name: "Chart", local_name: "chart", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CHART, children: &[] },
    ElementInfo { class_name: "BuildDiagram", local_name: "bldDgm", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUILD_DIAGRAM, children: &[] },
    ElementInfo { class_name: "BuildChart", local_name: "bldChart", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BUILD_CHART, children: &[] },
    ElementInfo { class_name: "TextBody", local_name: "txBody", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_BODY },
    ElementInfo { class_name: "UseShapeRectangle", local_name: "useSpRect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Transform2D", local_name: "xfrm", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualShapeDrawingProperties", local_name: "cNvSpPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_SHAPE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualShapeProperties", local_name: "nvSpPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "TextShape", local_name: "txSp", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_SHAPE },
    ElementInfo { class_name: "ShapeStyle", local_name: "style", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_STYLE },
    ElementInfo { class_name: "NonVisualConnectorShapeDrawingProperties", local_name: "cNvCxnSpPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTOR_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualConnectionShapeProperties", local_name: "nvCxnSpPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTION_SHAPE_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureDrawingProperties", local_name: "cNvPicPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_PICTURE_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_PICTURE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualPictureProperties", local_name: "nvPicPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_PICTURE_PROPERTIES },
    ElementInfo { class_name: "NonVisualGraphicFrameDrawingProperties", local_name: "cNvGraphicFramePr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualGraphicFrameProperties", local_name: "nvGraphicFramePr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GRAPHIC_FRAME_PROPERTIES },
    ElementInfo { class_name: "NonVisualGroupShapeDrawingProperties", local_name: "cNvGrpSpPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_DRAWING_PROPERTIES },
    ElementInfo { class_name: "Rotation", local_name: "rot", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ROTATION, children: &[] },
    ElementInfo { class_name: "Camera", local_name: "camera", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CAMERA, children: CHILDREN_CAMERA },
    ElementInfo { class_name: "LightRig", local_name: "lightRig", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LIGHT_RIG, children: CHILDREN_LIGHT_RIG },
    ElementInfo { class_name: "Backdrop", local_name: "backdrop", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKDROP },
    ElementInfo { class_name: "Anchor", local_name: "anchor", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ANCHOR, children: &[] },
    ElementInfo { class_name: "Normal", local_name: "norm", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_NORMAL, children: &[] },
    ElementInfo { class_name: "UpVector", local_name: "up", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_UP_VECTOR, children: &[] },
    ElementInfo { class_name: "BevelTop", local_name: "bevelT", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BEVEL_TOP, children: &[] },
    ElementInfo { class_name: "BevelBottom", local_name: "bevelB", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BEVEL_BOTTOM, children: &[] },
    ElementInfo { class_name: "Bevel", local_name: "bevel", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BEVEL, children: &[] },
    ElementInfo { class_name: "FillToRectangle", local_name: "fillToRect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FILL_TO_RECTANGLE, children: &[] },
    ElementInfo { class_name: "TileRectangle", local_name: "tileRect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TILE_RECTANGLE, children: &[] },
    ElementInfo { class_name: "FillRectangle", local_name: "fillRect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FILL_RECTANGLE, children: &[] },
    ElementInfo { class_name: "SourceRectangle", local_name: "srcRect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SOURCE_RECTANGLE, children: &[] },
    ElementInfo { class_name: "GradientStop", local_name: "gs", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRADIENT_STOP, children: CHILDREN_GRADIENT_STOP },
    ElementInfo { class_name: "GradientStopList", local_name: "gsLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRADIENT_STOP_LIST },
    ElementInfo { class_name: "ShapeGuide", local_name: "gd", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHAPE_GUIDE, children: &[] },
    ElementInfo { class_name: "Position", local_name: "pos", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POSITION, children: &[] },
    ElementInfo { class_name: "Point", local_name: "pt", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POINT, children: &[] },
    ElementInfo { class_name: "AdjustHandleXY", local_name: "ahXY", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ADJUST_HANDLE_X_Y, children: CHILDREN_ADJUST_HANDLE_X_Y },
    ElementInfo { class_name: "AdjustHandlePolar", local_name: "ahPolar", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ADJUST_HANDLE_POLAR, children: CHILDREN_ADJUST_HANDLE_POLAR },
    ElementInfo { class_name: "ConnectionSite", local_name: "cxn", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTION_SITE, children: CHILDREN_CONNECTION_SITE },
    ElementInfo { class_name: "CloseShapePath", local_name: "close", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "MoveTo", local_name: "moveTo", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MOVE_TO },
    ElementInfo { class_name: "LineTo", local_name: "lnTo", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_TO },
    ElementInfo { class_name: "ArcTo", local_name: "arcTo", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARC_TO, children: &[] },
    ElementInfo { class_name: "QuadraticBezierCurveTo", local_name: "quadBezTo", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_QUADRATIC_BEZIER_CURVE_TO },
    ElementInfo { class_name: "CubicBezierCurveTo", local_name: "cubicBezTo", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUBIC_BEZIER_CURVE_TO },
    ElementInfo { class_name: "Path", local_name: "path", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PATH, children: CHILDREN_PATH },
    ElementInfo { class_name: "AdjustValueList", local_name: "avLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ADJUST_VALUE_LIST },
    ElementInfo { class_name: "ShapeGuideList", local_name: "gdLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_GUIDE_LIST },
    ElementInfo { class_name: "AdjustHandleList", local_name: "ahLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ADJUST_HANDLE_LIST },
    ElementInfo { class_name: "ConnectionSiteList", local_name: "cxnLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONNECTION_SITE_LIST },
    ElementInfo { class_name: "Rectangle", local_name: "rect", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RECTANGLE, children: &[] },
    ElementInfo { class_name: "PathList", local_name: "pathLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PATH_LIST },
    ElementInfo { class_name: "DashStop", local_name: "ds", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DASH_STOP, children: &[] },
    ElementInfo { class_name: "TransformGroup", local_name: "xfrm", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM_GROUP, children: CHILDREN_TRANSFORM_GROUP },
    ElementInfo { class_name: "BodyProperties", local_name: "bodyPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BODY_PROPERTIES, children: CHILDREN_BODY_PROPERTIES },
    ElementInfo { class_name: "ListStyle", local_name: "lstStyle", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LIST_STYLE },
    ElementInfo { class_name: "ShapeDefault", local_name: "spDef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_DEFAULT },
    ElementInfo { class_name: "LineDefault", local_name: "lnDef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_DEFAULT },
    ElementInfo { class_name: "TextDefault", local_name: "txDef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_DEFAULT },
    ElementInfo { class_name: "OverrideColorMapping", local_name: "overrideClrMapping", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OVERRIDE_COLOR_MAPPING, children: CHILDREN_OVERRIDE_COLOR_MAPPING },
    ElementInfo { class_name: "ColorMap", local_name: "clrMap", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_MAP, children: CHILDREN_COLOR_MAP },
    ElementInfo { class_name: "ExtraColorScheme", local_name: "extraClrScheme", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTRA_COLOR_SCHEME },
    ElementInfo { class_name: "ThemeElements", local_name: "themeElements", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_THEME_ELEMENTS },
    ElementInfo { class_name: "Cell3DProperties", local_name: "cell3D", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CELL3_D_PROPERTIES, children: CHILDREN_CELL3_D_PROPERTIES },
    ElementInfo { class_name: "TableCellProperties", local_name: "tcPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_CELL_PROPERTIES, children: CHILDREN_TABLE_CELL_PROPERTIES },
    ElementInfo { class_name: "TableCell", local_name: "tc", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_CELL, children: CHILDREN_TABLE_CELL },
    ElementInfo { class_name: "TableStyle", local_name: "tableStyle", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_STYLE, children: CHILDREN_TABLE_STYLE },
    ElementInfo { class_name: "TableStyleEntry", local_name: "tblStyle", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_STYLE_ENTRY, children: CHILDREN_TABLE_STYLE_ENTRY },
    ElementInfo { class_name: "TableStyleId", local_name: "tableStyleId", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "GridColumn", local_name: "gridCol", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GRID_COLUMN, children: CHILDREN_GRID_COLUMN },
    ElementInfo { class_name: "TableProperties", local_name: "tblPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_PROPERTIES, children: CHILDREN_TABLE_PROPERTIES },
    ElementInfo { class_name: "TableGrid", local_name: "tblGrid", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABLE_GRID },
    ElementInfo { class_name: "TableRow", local_name: "tr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_ROW, children: CHILDREN_TABLE_ROW },
    ElementInfo { class_name: "LeftBorder", local_name: "left", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LEFT_BORDER },
    ElementInfo { class_name: "RightBorder", local_name: "right", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RIGHT_BORDER },
    ElementInfo { class_name: "TopBorder", local_name: "top", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TOP_BORDER },
    ElementInfo { class_name: "BottomBorder", local_name: "bottom", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BOTTOM_BORDER },
    ElementInfo { class_name: "InsideHorizontalBorder", local_name: "insideH", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_INSIDE_HORIZONTAL_BORDER },
    ElementInfo { class_name: "InsideVerticalBorder", local_name: "insideV", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_INSIDE_VERTICAL_BORDER },
    ElementInfo { class_name: "TopLeftToBottomRightBorder", local_name: "tl2br", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TOP_LEFT_TO_BOTTOM_RIGHT_BORDER },
    ElementInfo { class_name: "TopRightToBottomLeftBorder", local_name: "tr2bl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TOP_RIGHT_TO_BOTTOM_LEFT_BORDER },
    ElementInfo { class_name: "TableCellBorders", local_name: "tcBdr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABLE_CELL_BORDERS },
    ElementInfo { class_name: "TableCellTextStyle", local_name: "tcTxStyle", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TABLE_CELL_TEXT_STYLE, children: CHILDREN_TABLE_CELL_TEXT_STYLE },
    ElementInfo { class_name: "TableCellStyle", local_name: "tcStyle", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABLE_CELL_STYLE },
    ElementInfo { class_name: "TableBackground", local_name: "tblBg", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TABLE_BACKGROUND },
    ElementInfo { class_name: "WholeTable", local_name: "wholeTbl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WHOLE_TABLE },
    ElementInfo { class_name: "Band1Horizontal", local_name: "band1H", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAND1_HORIZONTAL },
    ElementInfo { class_name: "Band2Horizontal", local_name: "band2H", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAND2_HORIZONTAL },
    ElementInfo { class_name: "Band1Vertical", local_name: "band1V", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAND1_VERTICAL },
    ElementInfo { class_name: "Band2Vertical", local_name: "band2V", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BAND2_VERTICAL },
    ElementInfo { class_name: "LastColumn", local_name: "lastCol", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LAST_COLUMN },
    ElementInfo { class_name: "FirstColumn", local_name: "firstCol", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FIRST_COLUMN },
    ElementInfo { class_name: "LastRow", local_name: "lastRow", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LAST_ROW },
    ElementInfo { class_name: "SoutheastCell", local_name: "seCell", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SOUTHEAST_CELL },
    ElementInfo { class_name: "SouthwestCell", local_name: "swCell", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SOUTHWEST_CELL },
    ElementInfo { class_name: "FirstRow", local_name: "firstRow", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_FIRST_ROW },
    ElementInfo { class_name: "NortheastCell", local_name: "neCell", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NORTHEAST_CELL },
    ElementInfo { class_name: "NorthwestCell", local_name: "nwCell", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NORTHWEST_CELL },
    ElementInfo { class_name: "ParagraphProperties", local_name: "pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PARAGRAPH_PROPERTIES, children: CHILDREN_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "DefaultParagraphProperties", local_name: "defPPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DEFAULT_PARAGRAPH_PROPERTIES, children: CHILDREN_DEFAULT_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level1ParagraphProperties", local_name: "lvl1pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL1_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL1_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level2ParagraphProperties", local_name: "lvl2pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL2_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL2_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level3ParagraphProperties", local_name: "lvl3pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL3_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL3_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level4ParagraphProperties", local_name: "lvl4pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL4_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL4_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level5ParagraphProperties", local_name: "lvl5pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL5_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL5_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level6ParagraphProperties", local_name: "lvl6pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL6_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL6_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level7ParagraphProperties", local_name: "lvl7pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL7_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL7_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level8ParagraphProperties", local_name: "lvl8pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL8_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL8_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "Level9ParagraphProperties", local_name: "lvl9pPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LEVEL9_PARAGRAPH_PROPERTIES, children: CHILDREN_LEVEL9_PARAGRAPH_PROPERTIES },
    ElementInfo { class_name: "EndParagraphRunProperties", local_name: "endParaRPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_END_PARAGRAPH_RUN_PROPERTIES, children: CHILDREN_END_PARAGRAPH_RUN_PROPERTIES },
    ElementInfo { class_name: "RunProperties", local_name: "rPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RUN_PROPERTIES, children: CHILDREN_RUN_PROPERTIES },
    ElementInfo { class_name: "DefaultRunProperties", local_name: "defRPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DEFAULT_RUN_PROPERTIES, children: CHILDREN_DEFAULT_RUN_PROPERTIES },
    ElementInfo { class_name: "Paragraph", local_name: "p", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PARAGRAPH },
    ElementInfo { class_name: "TabStop", local_name: "tab", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TAB_STOP, children: &[] },
    ElementInfo { class_name: "SpacingPercent", local_name: "spcPct", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SPACING_PERCENT, children: &[] },
    ElementInfo { class_name: "SpacingPoints", local_name: "spcPts", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SPACING_POINTS, children: &[] },
    ElementInfo { class_name: "LineSpacing", local_name: "lnSpc", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_SPACING },
    ElementInfo { class_name: "SpaceBefore", local_name: "spcBef", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SPACE_BEFORE },
    ElementInfo { class_name: "SpaceAfter", local_name: "spcAft", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SPACE_AFTER },
    ElementInfo { class_name: "TabStopList", local_name: "tabLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TAB_STOP_LIST },
    ElementInfo { class_name: "Text", local_name: "t", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ShapePropertiesExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES_EXTENSION, children: CHILDREN_SHAPE_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "GvmlGroupShapeExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GVML_GROUP_SHAPE_EXTENSION, children: CHILDREN_GVML_GROUP_SHAPE_EXTENSION },
    ElementInfo { class_name: "ShapePropertiesExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualGroupShapeProperties", local_name: "nvGrpSpPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "VisualGroupShapeProperties", local_name: "grpSpPr", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_VISUAL_GROUP_SHAPE_PROPERTIES, children: CHILDREN_VISUAL_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "Shape", local_name: "sp", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE },
    ElementInfo { class_name: "ConnectionShape", local_name: "cxnSp", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONNECTION_SHAPE },
    ElementInfo { class_name: "Picture", local_name: "pic", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PICTURE },
    ElementInfo { class_name: "GraphicFrame", local_name: "graphicFrame", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GRAPHIC_FRAME },
    ElementInfo { class_name: "GroupShape", local_name: "grpSp", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GROUP_SHAPE },
    ElementInfo { class_name: "GvmlGroupShapeExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_GVML_GROUP_SHAPE_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualGroupDrawingShapePropsExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS_EXTENSION, children: CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS_EXTENSION },
    ElementInfo { class_name: "OfficeStyleSheetExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_OFFICE_STYLE_SHEET_EXTENSION, children: CHILDREN_OFFICE_STYLE_SHEET_EXTENSION },
    ElementInfo { class_name: "ConnectorLockingExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTOR_LOCKING_EXTENSION, children: CHILDREN_CONNECTOR_LOCKING_EXTENSION },
    ElementInfo { class_name: "GroupShapeLocks", local_name: "grpSpLocks", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_LOCKS, children: CHILDREN_GROUP_SHAPE_LOCKS },
    ElementInfo { class_name: "NonVisualGroupDrawingShapePropsExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS_EXTENSION_LIST },
    ElementInfo { class_name: "ObjectDefaults", local_name: "objectDefaults", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OBJECT_DEFAULTS },
    ElementInfo { class_name: "ExtraColorSchemeList", local_name: "extraClrSchemeLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTRA_COLOR_SCHEME_LIST },
    ElementInfo { class_name: "CustomColorList", local_name: "custClrLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CUSTOM_COLOR_LIST },
    ElementInfo { class_name: "OfficeStyleSheetExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_STYLE_SHEET_EXTENSION_LIST },
    ElementInfo { class_name: "HyperlinkOnClick", local_name: "hlinkClick", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HYPERLINK_ON_CLICK, children: CHILDREN_HYPERLINK_ON_CLICK },
    ElementInfo { class_name: "HyperlinkOnMouseOver", local_name: "hlinkMouseOver", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HYPERLINK_ON_MOUSE_OVER, children: CHILDREN_HYPERLINK_ON_MOUSE_OVER },
    ElementInfo { class_name: "HyperlinkOnHover", local_name: "hlinkHover", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HYPERLINK_ON_HOVER, children: CHILDREN_HYPERLINK_ON_HOVER },
    ElementInfo { class_name: "RightToLeft", local_name: "rtl", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RIGHT_TO_LEFT, children: &[] },
    ElementInfo { class_name: "NonVisualDrawingPropertiesExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "ConnectorLockingExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONNECTOR_LOCKING_EXTENSION_LIST },
    ElementInfo { class_name: "DataModelExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DATA_MODEL_EXTENSION, children: CHILDREN_DATA_MODEL_EXTENSION },
    ElementInfo { class_name: "PtExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PT_EXTENSION, children: CHILDREN_PT_EXTENSION },
    ElementInfo { class_name: "HyperlinkExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HYPERLINK_EXTENSION, children: CHILDREN_HYPERLINK_EXTENSION },
    ElementInfo { class_name: "HyperlinkExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HYPERLINK_EXTENSION_LIST },
    ElementInfo { class_name: "LinePropertiesExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_PROPERTIES_EXTENSION, children: CHILDREN_LINE_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "HeadEnd", local_name: "headEnd", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HEAD_END, children: &[] },
    ElementInfo { class_name: "TailEnd", local_name: "tailEnd", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TAIL_END, children: &[] },
    ElementInfo { class_name: "LinePropertiesExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LINE_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualDrawingPropertiesExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "PictureLocks", local_name: "picLocks", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PICTURE_LOCKS, children: CHILDREN_PICTURE_LOCKS },
    ElementInfo { class_name: "NonVisualPicturePropertiesExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_PICTURE_PROPERTIES_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualPicturePropertiesExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_PICTURE_PROPERTIES_EXTENSION, children: CHILDREN_NON_VISUAL_PICTURE_PROPERTIES_EXTENSION },
    ElementInfo { class_name: "BlipExtensionList", local_name: "extLst", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BLIP_EXTENSION_LIST },
    ElementInfo { class_name: "BlipExtension", local_name: "ext", prefix: "a", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP_EXTENSION, children: CHILDREN_BLIP_EXTENSION },
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

/// Create a `<a:audioCd>` element (`AudioFromCD`).
pub fn audio_from_c_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "audioCd").with_children(children)
}

/// Create a `<a:wavAudioFile>` element (`WaveAudioFile`).
pub fn wave_audio_file() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "wavAudioFile")
}

/// Set `Embed` (`r:embed`) on a `WaveAudioFile` element.
pub fn wave_audio_file_with_embed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:embed", value);
    el
}

/// Set `Name` (`:name`) on a `WaveAudioFile` element.
pub fn wave_audio_file_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `BuiltIn` (`:builtIn`) on a `WaveAudioFile` element.
pub fn wave_audio_file_with_built_in(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("builtIn", value);
    el
}

/// Create a `<a:snd>` element (`HyperlinkSound`).
pub fn hyperlink_sound() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "snd")
}

/// Set `Embed` (`r:embed`) on a `HyperlinkSound` element.
pub fn hyperlink_sound_with_embed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:embed", value);
    el
}

/// Set `Name` (`:name`) on a `HyperlinkSound` element.
pub fn hyperlink_sound_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `BuiltIn` (`:builtIn`) on a `HyperlinkSound` element.
pub fn hyperlink_sound_with_built_in(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("builtIn", value);
    el
}

/// Create a `<a:audioFile>` element (`AudioFromFile`).
pub fn audio_from_file(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "audioFile").with_children(children)
}

/// Set `Link` (`r:link`) on a `AudioFromFile` element.
pub fn audio_from_file_with_link(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:link", value);
    el
}

/// Create a `<a:videoFile>` element (`VideoFromFile`).
pub fn video_from_file(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "videoFile").with_children(children)
}

/// Set `Link` (`r:link`) on a `VideoFromFile` element.
pub fn video_from_file_with_link(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:link", value);
    el
}

/// Create a `<a:quickTimeFile>` element (`QuickTimeFromFile`).
pub fn quick_time_from_file(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "quickTimeFile").with_children(children)
}

/// Set `Link` (`r:link`) on a `QuickTimeFromFile` element.
pub fn quick_time_from_file_with_link(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:link", value);
    el
}

/// Create a `<a:tint>` element (`Tint`).
pub fn tint() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tint")
}

/// Set `Val` (`:val`) on a `Tint` element.
pub fn tint_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:tint>` with `Val` set.
pub fn tint_val(value: impl Into<String>) -> OpenXmlElement {
    tint_with_val(tint(), value)
}

/// Create a `<a:shade>` element (`Shade`).
pub fn shade() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "shade")
}

/// Set `Val` (`:val`) on a `Shade` element.
pub fn shade_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:shade>` with `Val` set.
pub fn shade_val(value: impl Into<String>) -> OpenXmlElement {
    shade_with_val(shade(), value)
}

/// Create a `<a:alpha>` element (`Alpha`).
pub fn alpha() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alpha")
}

/// Set `Val` (`:val`) on a `Alpha` element.
pub fn alpha_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:alpha>` with `Val` set.
pub fn alpha_val(value: impl Into<String>) -> OpenXmlElement {
    alpha_with_val(alpha(), value)
}

/// Create a `<a:comp>` element (`Complement`).
pub fn complement() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "comp")
}

/// Create a `<a:inv>` element (`Inverse`).
pub fn inverse() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "inv")
}

/// Create a `<a:gray>` element (`Gray`).
pub fn gray() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gray")
}

/// Create a `<a:alphaOff>` element (`AlphaOffset`).
pub fn alpha_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaOff")
}

/// Set `Val` (`:val`) on a `AlphaOffset` element.
pub fn alpha_offset_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:alphaOff>` with `Val` set.
pub fn alpha_offset_val(value: impl Into<String>) -> OpenXmlElement {
    alpha_offset_with_val(alpha_offset(), value)
}

/// Create a `<a:alphaMod>` element (`AlphaModulation`).
pub fn alpha_modulation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaMod")
}

/// Set `Val` (`:val`) on a `AlphaModulation` element.
pub fn alpha_modulation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:alphaMod>` with `Val` set.
pub fn alpha_modulation_val(value: impl Into<String>) -> OpenXmlElement {
    alpha_modulation_with_val(alpha_modulation(), value)
}

/// Create a `<a:hueMod>` element (`HueModulation`).
pub fn hue_modulation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hueMod")
}

/// Set `Val` (`:val`) on a `HueModulation` element.
pub fn hue_modulation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:hueMod>` with `Val` set.
pub fn hue_modulation_val(value: impl Into<String>) -> OpenXmlElement {
    hue_modulation_with_val(hue_modulation(), value)
}

/// Create a `<a:hue>` element (`Hue`).
pub fn hue() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hue")
}

/// Set `Val` (`:val`) on a `Hue` element.
pub fn hue_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:hue>` with `Val` set.
pub fn hue_val(value: impl Into<String>) -> OpenXmlElement {
    hue_with_val(hue(), value)
}

/// Create a `<a:hueOff>` element (`HueOffset`).
pub fn hue_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hueOff")
}

/// Set `Val` (`:val`) on a `HueOffset` element.
pub fn hue_offset_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:hueOff>` with `Val` set.
pub fn hue_offset_val(value: impl Into<String>) -> OpenXmlElement {
    hue_offset_with_val(hue_offset(), value)
}

/// Create a `<a:sat>` element (`Saturation`).
pub fn saturation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "sat")
}

/// Set `Val` (`:val`) on a `Saturation` element.
pub fn saturation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:sat>` with `Val` set.
pub fn saturation_val(value: impl Into<String>) -> OpenXmlElement {
    saturation_with_val(saturation(), value)
}

/// Create a `<a:satOff>` element (`SaturationOffset`).
pub fn saturation_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "satOff")
}

/// Set `Val` (`:val`) on a `SaturationOffset` element.
pub fn saturation_offset_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:satOff>` with `Val` set.
pub fn saturation_offset_val(value: impl Into<String>) -> OpenXmlElement {
    saturation_offset_with_val(saturation_offset(), value)
}

/// Create a `<a:satMod>` element (`SaturationModulation`).
pub fn saturation_modulation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "satMod")
}

/// Set `Val` (`:val`) on a `SaturationModulation` element.
pub fn saturation_modulation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:satMod>` with `Val` set.
pub fn saturation_modulation_val(value: impl Into<String>) -> OpenXmlElement {
    saturation_modulation_with_val(saturation_modulation(), value)
}

/// Create a `<a:lum>` element (`Luminance`).
pub fn luminance() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lum")
}

/// Set `Val` (`:val`) on a `Luminance` element.
pub fn luminance_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:lum>` with `Val` set.
pub fn luminance_val(value: impl Into<String>) -> OpenXmlElement {
    luminance_with_val(luminance(), value)
}

/// Create a `<a:lumOff>` element (`LuminanceOffset`).
pub fn luminance_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lumOff")
}

/// Set `Val` (`:val`) on a `LuminanceOffset` element.
pub fn luminance_offset_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:lumOff>` with `Val` set.
pub fn luminance_offset_val(value: impl Into<String>) -> OpenXmlElement {
    luminance_offset_with_val(luminance_offset(), value)
}

/// Create a `<a:lumMod>` element (`LuminanceModulation`).
pub fn luminance_modulation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lumMod")
}

/// Set `Val` (`:val`) on a `LuminanceModulation` element.
pub fn luminance_modulation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:lumMod>` with `Val` set.
pub fn luminance_modulation_val(value: impl Into<String>) -> OpenXmlElement {
    luminance_modulation_with_val(luminance_modulation(), value)
}

/// Create a `<a:red>` element (`Red`).
pub fn red() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "red")
}

/// Set `Val` (`:val`) on a `Red` element.
pub fn red_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:red>` with `Val` set.
pub fn red_val(value: impl Into<String>) -> OpenXmlElement {
    red_with_val(red(), value)
}

/// Create a `<a:redOff>` element (`RedOffset`).
pub fn red_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "redOff")
}

/// Set `Val` (`:val`) on a `RedOffset` element.
pub fn red_offset_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:redOff>` with `Val` set.
pub fn red_offset_val(value: impl Into<String>) -> OpenXmlElement {
    red_offset_with_val(red_offset(), value)
}

/// Create a `<a:redMod>` element (`RedModulation`).
pub fn red_modulation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "redMod")
}

/// Set `Val` (`:val`) on a `RedModulation` element.
pub fn red_modulation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:redMod>` with `Val` set.
pub fn red_modulation_val(value: impl Into<String>) -> OpenXmlElement {
    red_modulation_with_val(red_modulation(), value)
}

/// Create a `<a:green>` element (`Green`).
pub fn green() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "green")
}

/// Set `Val` (`:val`) on a `Green` element.
pub fn green_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:green>` with `Val` set.
pub fn green_val(value: impl Into<String>) -> OpenXmlElement {
    green_with_val(green(), value)
}

/// Create a `<a:greenOff>` element (`GreenOffset`).
pub fn green_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "greenOff")
}

/// Set `Val` (`:val`) on a `GreenOffset` element.
pub fn green_offset_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:greenOff>` with `Val` set.
pub fn green_offset_val(value: impl Into<String>) -> OpenXmlElement {
    green_offset_with_val(green_offset(), value)
}

/// Create a `<a:greenMod>` element (`GreenModulation`).
pub fn green_modulation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "greenMod")
}

/// Set `Val` (`:val`) on a `GreenModulation` element.
pub fn green_modulation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:greenMod>` with `Val` set.
pub fn green_modulation_val(value: impl Into<String>) -> OpenXmlElement {
    green_modulation_with_val(green_modulation(), value)
}

/// Create a `<a:blue>` element (`Blue`).
pub fn blue() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "blue")
}

/// Set `Val` (`:val`) on a `Blue` element.
pub fn blue_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:blue>` with `Val` set.
pub fn blue_val(value: impl Into<String>) -> OpenXmlElement {
    blue_with_val(blue(), value)
}

/// Create a `<a:blueOff>` element (`BlueOffset`).
pub fn blue_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "blueOff")
}

/// Set `Val` (`:val`) on a `BlueOffset` element.
pub fn blue_offset_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:blueOff>` with `Val` set.
pub fn blue_offset_val(value: impl Into<String>) -> OpenXmlElement {
    blue_offset_with_val(blue_offset(), value)
}

/// Create a `<a:blueMod>` element (`BlueModulation`).
pub fn blue_modulation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "blueMod")
}

/// Set `Val` (`:val`) on a `BlueModulation` element.
pub fn blue_modulation_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:blueMod>` with `Val` set.
pub fn blue_modulation_val(value: impl Into<String>) -> OpenXmlElement {
    blue_modulation_with_val(blue_modulation(), value)
}

/// Create a `<a:gamma>` element (`Gamma`).
pub fn gamma() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gamma")
}

/// Create a `<a:invGamma>` element (`InverseGamma`).
pub fn inverse_gamma() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "invGamma")
}

/// Create a `<a:ext>` element (`Extension`).
pub fn extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Set `Uri` (`:uri`) on a `Extension` element.
pub fn extension_with_uri(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("uri", value);
    el
}

/// Create a `<a:scrgbClr>` element (`RgbColorModelPercentage`).
pub fn rgb_color_model_percentage(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "scrgbClr").with_children(children)
}

/// Set `RedPortion` (`:r`) on a `RgbColorModelPercentage` element.
pub fn rgb_color_model_percentage_with_red_portion(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r", value);
    el
}

/// Set `GreenPortion` (`:g`) on a `RgbColorModelPercentage` element.
pub fn rgb_color_model_percentage_with_green_portion(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("g", value);
    el
}

/// Set `BluePortion` (`:b`) on a `RgbColorModelPercentage` element.
pub fn rgb_color_model_percentage_with_blue_portion(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Create a `<a:srgbClr>` element (`RgbColorModelHex`).
pub fn rgb_color_model_hex(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "srgbClr").with_children(children)
}

/// Set `Val` (`:val`) on a `RgbColorModelHex` element.
pub fn rgb_color_model_hex_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Set `LegacySpreadsheetColorIndex` (`a14:legacySpreadsheetColorIndex`) on a `RgbColorModelHex` element.
pub fn rgb_color_model_hex_with_legacy_spreadsheet_color_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("a14:legacySpreadsheetColorIndex", value);
    el
}

/// Create a `<a:hslClr>` element (`HslColor`).
pub fn hsl_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hslClr").with_children(children)
}

/// Set `HueValue` (`:hue`) on a `HslColor` element.
pub fn hsl_color_with_hue_value(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hue", value);
    el
}

/// Set `SatValue` (`:sat`) on a `HslColor` element.
pub fn hsl_color_with_sat_value(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sat", value);
    el
}

/// Set `LumValue` (`:lum`) on a `HslColor` element.
pub fn hsl_color_with_lum_value(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lum", value);
    el
}

/// Create a `<a:sysClr>` element (`SystemColor`).
pub fn system_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "sysClr").with_children(children)
}

/// Set `Val` (`:val`) on a `SystemColor` element.
pub fn system_color_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Set `LastColor` (`:lastClr`) on a `SystemColor` element.
pub fn system_color_with_last_color(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lastClr", value);
    el
}

/// Create a `<a:schemeClr>` element (`SchemeColor`).
pub fn scheme_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "schemeClr").with_children(children)
}

/// Set `Val` (`:val`) on a `SchemeColor` element.
pub fn scheme_color_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create a `<a:prstClr>` element (`PresetColor`).
pub fn preset_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "prstClr").with_children(children)
}

/// Set `Val` (`:val`) on a `PresetColor` element.
pub fn preset_color_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create a `<a:sp3d>` element (`Shape3DType`).
pub fn shape3_d_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "sp3d").with_children(children)
}

/// Set `Z` (`:z`) on a `Shape3DType` element.
pub fn shape3_d_type_with_z(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("z", value);
    el
}

/// Set `ExtrusionHeight` (`:extrusionH`) on a `Shape3DType` element.
pub fn shape3_d_type_with_extrusion_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("extrusionH", value);
    el
}

/// Set `ContourWidth` (`:contourW`) on a `Shape3DType` element.
pub fn shape3_d_type_with_contour_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("contourW", value);
    el
}

/// Set `PresetMaterial` (`:prstMaterial`) on a `Shape3DType` element.
pub fn shape3_d_type_with_preset_material(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prstMaterial", value);
    el
}

/// Create a `<a:flatTx>` element (`FlatText`).
pub fn flat_text() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "flatTx")
}

/// Set `Z` (`:z`) on a `FlatText` element.
pub fn flat_text_with_z(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("z", value);
    el
}

/// Create `<a:flatTx>` with `Z` set.
pub fn flat_text_z(value: impl Into<String>) -> OpenXmlElement {
    flat_text_with_z(flat_text(), value)
}

/// Create a `<a:lin>` element (`LinearGradientFill`).
pub fn linear_gradient_fill() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lin")
}

/// Set `Angle` (`:ang`) on a `LinearGradientFill` element.
pub fn linear_gradient_fill_with_angle(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ang", value);
    el
}

/// Set `Scaled` (`:scaled`) on a `LinearGradientFill` element.
pub fn linear_gradient_fill_with_scaled(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("scaled", value);
    el
}

/// Create a `<a:path>` element (`PathGradientFill`).
pub fn path_gradient_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "path").with_children(children)
}

/// Set `Path` (`:path`) on a `PathGradientFill` element.
pub fn path_gradient_fill_with_path(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("path", value);
    el
}

/// Create a `<a:tile>` element (`Tile`).
pub fn tile() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tile")
}

/// Set `HorizontalOffset` (`:tx`) on a `Tile` element.
pub fn tile_with_horizontal_offset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx", value);
    el
}

/// Set `VerticalOffset` (`:ty`) on a `Tile` element.
pub fn tile_with_vertical_offset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ty", value);
    el
}

/// Set `HorizontalRatio` (`:sx`) on a `Tile` element.
pub fn tile_with_horizontal_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sx", value);
    el
}

/// Set `VerticalRatio` (`:sy`) on a `Tile` element.
pub fn tile_with_vertical_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sy", value);
    el
}

/// Set `Flip` (`:flip`) on a `Tile` element.
pub fn tile_with_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flip", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Tile` element.
pub fn tile_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:stretch>` element (`Stretch`).
pub fn stretch(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "stretch").with_children(children)
}

/// Create a `<a:noFill>` element (`NoFill`).
pub fn no_fill() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "noFill")
}

/// Create a `<a:solidFill>` element (`SolidFill`).
pub fn solid_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "solidFill").with_children(children)
}

/// Create a `<a:gradFill>` element (`GradientFill`).
pub fn gradient_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gradFill").with_children(children)
}

/// Set `Flip` (`:flip`) on a `GradientFill` element.
pub fn gradient_fill_with_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flip", value);
    el
}

/// Set `RotateWithShape` (`:rotWithShape`) on a `GradientFill` element.
pub fn gradient_fill_with_rotate_with_shape(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rotWithShape", value);
    el
}

/// Create a `<a:blipFill>` element (`BlipFill`).
pub fn blip_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "blipFill").with_children(children)
}

/// Set `Dpi` (`:dpi`) on a `BlipFill` element.
pub fn blip_fill_with_dpi(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dpi", value);
    el
}

/// Set `RotateWithShape` (`:rotWithShape`) on a `BlipFill` element.
pub fn blip_fill_with_rotate_with_shape(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rotWithShape", value);
    el
}

/// Create a `<a:pattFill>` element (`PatternFill`).
pub fn pattern_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "pattFill").with_children(children)
}

/// Set `Preset` (`:prst`) on a `PatternFill` element.
pub fn pattern_fill_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Create a `<a:grpFill>` element (`GroupFill`).
pub fn group_fill() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "grpFill")
}

/// Create a `<a:cont>` element (`EffectContainer`).
pub fn effect_container(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cont").with_children(children)
}

/// Set `Type` (`:type`) on a `EffectContainer` element.
pub fn effect_container_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Name` (`:name`) on a `EffectContainer` element.
pub fn effect_container_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Create a `<a:effectDag>` element (`EffectDag`).
pub fn effect_dag(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "effectDag").with_children(children)
}

/// Set `Type` (`:type`) on a `EffectDag` element.
pub fn effect_dag_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Name` (`:name`) on a `EffectDag` element.
pub fn effect_dag_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Create a `<a:effect>` element (`Effect`).
pub fn effect() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "effect")
}

/// Set `Reference` (`:ref`) on a `Effect` element.
pub fn effect_with_reference(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ref", value);
    el
}

/// Create `<a:effect>` with `Reference` set.
pub fn effect_reference(value: impl Into<String>) -> OpenXmlElement {
    effect_with_reference(effect(), value)
}

/// Create a `<a:alphaBiLevel>` element (`AlphaBiLevel`).
pub fn alpha_bi_level() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaBiLevel")
}

/// Set `Threshold` (`:thresh`) on a `AlphaBiLevel` element.
pub fn alpha_bi_level_with_threshold(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("thresh", value);
    el
}

/// Create `<a:alphaBiLevel>` with `Threshold` set.
pub fn alpha_bi_level_threshold(value: impl Into<String>) -> OpenXmlElement {
    alpha_bi_level_with_threshold(alpha_bi_level(), value)
}

/// Create a `<a:alphaCeiling>` element (`AlphaCeiling`).
pub fn alpha_ceiling() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaCeiling")
}

/// Create a `<a:alphaFloor>` element (`AlphaFloor`).
pub fn alpha_floor() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaFloor")
}

/// Create a `<a:alphaInv>` element (`AlphaInverse`).
pub fn alpha_inverse(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaInv").with_children(children)
}

/// Create a `<a:alphaMod>` element (`AlphaModulationEffect`).
pub fn alpha_modulation_effect(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaMod").with_children(children)
}

/// Create a `<a:alphaModFix>` element (`AlphaModulationFixed`).
pub fn alpha_modulation_fixed() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaModFix")
}

/// Set `Amount` (`:amt`) on a `AlphaModulationFixed` element.
pub fn alpha_modulation_fixed_with_amount(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("amt", value);
    el
}

/// Create `<a:alphaModFix>` with `Amount` set.
pub fn alpha_modulation_fixed_amount(value: impl Into<String>) -> OpenXmlElement {
    alpha_modulation_fixed_with_amount(alpha_modulation_fixed(), value)
}

/// Create a `<a:alphaOutset>` element (`AlphaOutset`).
pub fn alpha_outset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaOutset")
}

/// Set `Radius` (`:rad`) on a `AlphaOutset` element.
pub fn alpha_outset_with_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rad", value);
    el
}

/// Create `<a:alphaOutset>` with `Radius` set.
pub fn alpha_outset_radius(value: impl Into<String>) -> OpenXmlElement {
    alpha_outset_with_radius(alpha_outset(), value)
}

/// Create a `<a:alphaRepl>` element (`AlphaReplace`).
pub fn alpha_replace() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "alphaRepl")
}

/// Set `Alpha` (`:a`) on a `AlphaReplace` element.
pub fn alpha_replace_with_alpha(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("a", value);
    el
}

/// Create `<a:alphaRepl>` with `Alpha` set.
pub fn alpha_replace_alpha(value: impl Into<String>) -> OpenXmlElement {
    alpha_replace_with_alpha(alpha_replace(), value)
}

/// Create a `<a:biLevel>` element (`BiLevel`).
pub fn bi_level() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "biLevel")
}

/// Set `Threshold` (`:thresh`) on a `BiLevel` element.
pub fn bi_level_with_threshold(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("thresh", value);
    el
}

/// Create `<a:biLevel>` with `Threshold` set.
pub fn bi_level_threshold(value: impl Into<String>) -> OpenXmlElement {
    bi_level_with_threshold(bi_level(), value)
}

/// Create a `<a:blend>` element (`Blend`).
pub fn blend(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "blend").with_children(children)
}

/// Set `BlendMode` (`:blend`) on a `Blend` element.
pub fn blend_with_blend_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("blend", value);
    el
}

/// Create a `<a:blur>` element (`Blur`).
pub fn blur() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "blur")
}

/// Set `Radius` (`:rad`) on a `Blur` element.
pub fn blur_with_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rad", value);
    el
}

/// Set `Grow` (`:grow`) on a `Blur` element.
pub fn blur_with_grow(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("grow", value);
    el
}

/// Create a `<a:clrChange>` element (`ColorChange`).
pub fn color_change(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "clrChange").with_children(children)
}

/// Set `UseAlpha` (`:useA`) on a `ColorChange` element.
pub fn color_change_with_use_alpha(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("useA", value);
    el
}

/// Create a `<a:clrRepl>` element (`ColorReplacement`).
pub fn color_replacement(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "clrRepl").with_children(children)
}

/// Create a `<a:duotone>` element (`Duotone`).
pub fn duotone(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "duotone").with_children(children)
}

/// Create a `<a:fill>` element (`Fill`).
pub fn fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fill").with_children(children)
}

/// Create a `<a:fillOverlay>` element (`FillOverlay`).
pub fn fill_overlay(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fillOverlay").with_children(children)
}

/// Set `Blend` (`:blend`) on a `FillOverlay` element.
pub fn fill_overlay_with_blend(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("blend", value);
    el
}

/// Create a `<a:glow>` element (`Glow`).
pub fn glow(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "glow").with_children(children)
}

/// Set `Radius` (`:rad`) on a `Glow` element.
pub fn glow_with_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rad", value);
    el
}

/// Create a `<a:grayscl>` element (`Grayscale`).
pub fn grayscale() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "grayscl")
}

/// Create a `<a:hsl>` element (`Hsl`).
pub fn hsl() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hsl")
}

/// Set `Hue` (`:hue`) on a `Hsl` element.
pub fn hsl_with_hue(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hue", value);
    el
}

/// Set `Saturation` (`:sat`) on a `Hsl` element.
pub fn hsl_with_saturation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sat", value);
    el
}

/// Set `Luminance` (`:lum`) on a `Hsl` element.
pub fn hsl_with_luminance(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lum", value);
    el
}

/// Create a `<a:innerShdw>` element (`InnerShadow`).
pub fn inner_shadow(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "innerShdw").with_children(children)
}

/// Set `BlurRadius` (`:blurRad`) on a `InnerShadow` element.
pub fn inner_shadow_with_blur_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("blurRad", value);
    el
}

/// Set `Distance` (`:dist`) on a `InnerShadow` element.
pub fn inner_shadow_with_distance(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dist", value);
    el
}

/// Set `Direction` (`:dir`) on a `InnerShadow` element.
pub fn inner_shadow_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create a `<a:lum>` element (`LuminanceEffect`).
pub fn luminance_effect() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lum")
}

/// Set `Brightness` (`:bright`) on a `LuminanceEffect` element.
pub fn luminance_effect_with_brightness(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bright", value);
    el
}

/// Set `Contrast` (`:contrast`) on a `LuminanceEffect` element.
pub fn luminance_effect_with_contrast(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("contrast", value);
    el
}

/// Create a `<a:outerShdw>` element (`OuterShadow`).
pub fn outer_shadow(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "outerShdw").with_children(children)
}

/// Set `BlurRadius` (`:blurRad`) on a `OuterShadow` element.
pub fn outer_shadow_with_blur_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("blurRad", value);
    el
}

/// Set `Distance` (`:dist`) on a `OuterShadow` element.
pub fn outer_shadow_with_distance(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dist", value);
    el
}

/// Set `Direction` (`:dir`) on a `OuterShadow` element.
pub fn outer_shadow_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Set `HorizontalRatio` (`:sx`) on a `OuterShadow` element.
pub fn outer_shadow_with_horizontal_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sx", value);
    el
}

/// Set `VerticalRatio` (`:sy`) on a `OuterShadow` element.
pub fn outer_shadow_with_vertical_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sy", value);
    el
}

/// Set `HorizontalSkew` (`:kx`) on a `OuterShadow` element.
pub fn outer_shadow_with_horizontal_skew(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kx", value);
    el
}

/// Set `VerticalSkew` (`:ky`) on a `OuterShadow` element.
pub fn outer_shadow_with_vertical_skew(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ky", value);
    el
}

/// Set `Alignment` (`:algn`) on a `OuterShadow` element.
pub fn outer_shadow_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `RotateWithShape` (`:rotWithShape`) on a `OuterShadow` element.
pub fn outer_shadow_with_rotate_with_shape(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rotWithShape", value);
    el
}

/// Create a `<a:prstShdw>` element (`PresetShadow`).
pub fn preset_shadow(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "prstShdw").with_children(children)
}

/// Set `Preset` (`:prst`) on a `PresetShadow` element.
pub fn preset_shadow_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Set `Distance` (`:dist`) on a `PresetShadow` element.
pub fn preset_shadow_with_distance(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dist", value);
    el
}

/// Set `Direction` (`:dir`) on a `PresetShadow` element.
pub fn preset_shadow_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create a `<a:reflection>` element (`Reflection`).
pub fn reflection() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "reflection")
}

/// Set `BlurRadius` (`:blurRad`) on a `Reflection` element.
pub fn reflection_with_blur_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("blurRad", value);
    el
}

/// Set `StartOpacity` (`:stA`) on a `Reflection` element.
pub fn reflection_with_start_opacity(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("stA", value);
    el
}

/// Set `StartPosition` (`:stPos`) on a `Reflection` element.
pub fn reflection_with_start_position(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("stPos", value);
    el
}

/// Set `EndAlpha` (`:endA`) on a `Reflection` element.
pub fn reflection_with_end_alpha(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("endA", value);
    el
}

/// Set `EndPosition` (`:endPos`) on a `Reflection` element.
pub fn reflection_with_end_position(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("endPos", value);
    el
}

/// Set `Distance` (`:dist`) on a `Reflection` element.
pub fn reflection_with_distance(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dist", value);
    el
}

/// Set `Direction` (`:dir`) on a `Reflection` element.
pub fn reflection_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Set `FadeDirection` (`:fadeDir`) on a `Reflection` element.
pub fn reflection_with_fade_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fadeDir", value);
    el
}

/// Set `HorizontalRatio` (`:sx`) on a `Reflection` element.
pub fn reflection_with_horizontal_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sx", value);
    el
}

/// Set `VerticalRatio` (`:sy`) on a `Reflection` element.
pub fn reflection_with_vertical_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sy", value);
    el
}

/// Set `HorizontalSkew` (`:kx`) on a `Reflection` element.
pub fn reflection_with_horizontal_skew(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kx", value);
    el
}

/// Set `VerticalSkew` (`:ky`) on a `Reflection` element.
pub fn reflection_with_vertical_skew(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ky", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Reflection` element.
pub fn reflection_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `RotateWithShape` (`:rotWithShape`) on a `Reflection` element.
pub fn reflection_with_rotate_with_shape(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rotWithShape", value);
    el
}

/// Create a `<a:relOff>` element (`RelativeOffset`).
pub fn relative_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "relOff")
}

/// Set `OffsetX` (`:tx`) on a `RelativeOffset` element.
pub fn relative_offset_with_offset_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx", value);
    el
}

/// Set `OffsetY` (`:ty`) on a `RelativeOffset` element.
pub fn relative_offset_with_offset_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ty", value);
    el
}

/// Create a `<a:softEdge>` element (`SoftEdge`).
pub fn soft_edge() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "softEdge")
}

/// Set `Radius` (`:rad`) on a `SoftEdge` element.
pub fn soft_edge_with_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rad", value);
    el
}

/// Create `<a:softEdge>` with `Radius` set.
pub fn soft_edge_radius(value: impl Into<String>) -> OpenXmlElement {
    soft_edge_with_radius(soft_edge(), value)
}

/// Create a `<a:tint>` element (`TintEffect`).
pub fn tint_effect() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tint")
}

/// Set `Hue` (`:hue`) on a `TintEffect` element.
pub fn tint_effect_with_hue(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hue", value);
    el
}

/// Set `Amount` (`:amt`) on a `TintEffect` element.
pub fn tint_effect_with_amount(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("amt", value);
    el
}

/// Create a `<a:xfrm>` element (`TransformEffect`).
pub fn transform_effect() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "xfrm")
}

/// Set `HorizontalRatio` (`:sx`) on a `TransformEffect` element.
pub fn transform_effect_with_horizontal_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sx", value);
    el
}

/// Set `VerticalRatio` (`:sy`) on a `TransformEffect` element.
pub fn transform_effect_with_vertical_ratio(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sy", value);
    el
}

/// Set `HorizontalSkew` (`:kx`) on a `TransformEffect` element.
pub fn transform_effect_with_horizontal_skew(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kx", value);
    el
}

/// Set `VerticalSkew` (`:ky`) on a `TransformEffect` element.
pub fn transform_effect_with_vertical_skew(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ky", value);
    el
}

/// Set `HorizontalShift` (`:tx`) on a `TransformEffect` element.
pub fn transform_effect_with_horizontal_shift(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx", value);
    el
}

/// Set `VerticalShift` (`:ty`) on a `TransformEffect` element.
pub fn transform_effect_with_vertical_shift(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ty", value);
    el
}

/// Create a `<a:effectLst>` element (`EffectList`).
pub fn effect_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "effectLst").with_children(children)
}

/// Create a `<a:custGeom>` element (`CustomGeometry`).
pub fn custom_geometry(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "custGeom").with_children(children)
}

/// Create a `<a:prstGeom>` element (`PresetGeometry`).
pub fn preset_geometry(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "prstGeom").with_children(children)
}

/// Set `Preset` (`:prst`) on a `PresetGeometry` element.
pub fn preset_geometry_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Create a `<a:prstTxWarp>` element (`PresetTextWarp`).
pub fn preset_text_warp(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "prstTxWarp").with_children(children)
}

/// Set `Preset` (`:prst`) on a `PresetTextWarp` element.
pub fn preset_text_warp_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Create a `<a:round>` element (`Round`).
pub fn round() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "round")
}

/// Create a `<a:bevel>` element (`LineJoinBevel`).
pub fn line_join_bevel() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bevel")
}

/// Create a `<a:miter>` element (`Miter`).
pub fn miter() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "miter")
}

/// Set `Limit` (`:lim`) on a `Miter` element.
pub fn miter_with_limit(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lim", value);
    el
}

/// Create `<a:miter>` with `Limit` set.
pub fn miter_limit(value: impl Into<String>) -> OpenXmlElement {
    miter_with_limit(miter(), value)
}

/// Create a `<a:prstDash>` element (`PresetDash`).
pub fn preset_dash() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "prstDash")
}

/// Set `Val` (`:val`) on a `PresetDash` element.
pub fn preset_dash_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:prstDash>` with `Val` set.
pub fn preset_dash_val(value: impl Into<String>) -> OpenXmlElement {
    preset_dash_with_val(preset_dash(), value)
}

/// Create a `<a:custDash>` element (`CustomDash`).
pub fn custom_dash(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "custDash").with_children(children)
}

/// Create a `<a:fill>` element (`FillProperties`).
pub fn fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fill").with_children(children)
}

/// Create a `<a:fillRef>` element (`FillReference`).
pub fn fill_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fillRef").with_children(children)
}

/// Set `Index` (`:idx`) on a `FillReference` element.
pub fn fill_reference_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Create a `<a:lnRef>` element (`LineReference`).
pub fn line_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnRef").with_children(children)
}

/// Set `Index` (`:idx`) on a `LineReference` element.
pub fn line_reference_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Create a `<a:effect>` element (`EffectPropertiesType`).
pub fn effect_properties_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "effect").with_children(children)
}

/// Create a `<a:font>` element (`Fonts`).
pub fn fonts(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "font").with_children(children)
}

/// Create a `<a:majorFont>` element (`MajorFont`).
pub fn major_font(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "majorFont").with_children(children)
}

/// Create a `<a:minorFont>` element (`MinorFont`).
pub fn minor_font(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "minorFont").with_children(children)
}

/// Create a `<a:fontRef>` element (`FontReference`).
pub fn font_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fontRef").with_children(children)
}

/// Set `Index` (`:idx`) on a `FontReference` element.
pub fn font_reference_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Create a `<a:noAutofit>` element (`NoAutoFit`).
pub fn no_auto_fit() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "noAutofit")
}

/// Create a `<a:normAutofit>` element (`NormalAutoFit`).
pub fn normal_auto_fit() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "normAutofit")
}

/// Set `FontScale` (`:fontScale`) on a `NormalAutoFit` element.
pub fn normal_auto_fit_with_font_scale(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontScale", value);
    el
}

/// Set `LineSpaceReduction` (`:lnSpcReduction`) on a `NormalAutoFit` element.
pub fn normal_auto_fit_with_line_space_reduction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lnSpcReduction", value);
    el
}

/// Create a `<a:spAutoFit>` element (`ShapeAutoFit`).
pub fn shape_auto_fit() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spAutoFit")
}

/// Create a `<a:buClrTx>` element (`BulletColorText`).
pub fn bullet_color_text() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buClrTx")
}

/// Create a `<a:buClr>` element (`BulletColor`).
pub fn bullet_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buClr").with_children(children)
}

/// Create a `<a:extrusionClr>` element (`ExtrusionColor`).
pub fn extrusion_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extrusionClr").with_children(children)
}

/// Create a `<a:contourClr>` element (`ContourColor`).
pub fn contour_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "contourClr").with_children(children)
}

/// Create a `<a:clrFrom>` element (`ColorFrom`).
pub fn color_from(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "clrFrom").with_children(children)
}

/// Create a `<a:clrTo>` element (`ColorTo`).
pub fn color_to(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "clrTo").with_children(children)
}

/// Create a `<a:fgClr>` element (`ForegroundColor`).
pub fn foreground_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fgClr").with_children(children)
}

/// Create a `<a:bgClr>` element (`BackgroundColor`).
pub fn background_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bgClr").with_children(children)
}

/// Create a `<a:highlight>` element (`Highlight`).
pub fn highlight(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "highlight").with_children(children)
}

/// Create a `<a:buSzTx>` element (`BulletSizeText`).
pub fn bullet_size_text() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buSzTx")
}

/// Create a `<a:buSzPct>` element (`BulletSizePercentage`).
pub fn bullet_size_percentage() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buSzPct")
}

/// Set `Val` (`:val`) on a `BulletSizePercentage` element.
pub fn bullet_size_percentage_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:buSzPct>` with `Val` set.
pub fn bullet_size_percentage_val(value: impl Into<String>) -> OpenXmlElement {
    bullet_size_percentage_with_val(bullet_size_percentage(), value)
}

/// Create a `<a:buSzPts>` element (`BulletSizePoints`).
pub fn bullet_size_points() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buSzPts")
}

/// Set `Val` (`:val`) on a `BulletSizePoints` element.
pub fn bullet_size_points_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:buSzPts>` with `Val` set.
pub fn bullet_size_points_val(value: impl Into<String>) -> OpenXmlElement {
    bullet_size_points_with_val(bullet_size_points(), value)
}

/// Create a `<a:buFontTx>` element (`BulletFontText`).
pub fn bullet_font_text() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buFontTx")
}

/// Create a `<a:buFont>` element (`BulletFont`).
pub fn bullet_font() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buFont")
}

/// Set `Typeface` (`:typeface`) on a `BulletFont` element.
pub fn bullet_font_with_typeface(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("typeface", value);
    el
}

/// Set `Panose` (`:panose`) on a `BulletFont` element.
pub fn bullet_font_with_panose(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("panose", value);
    el
}

/// Set `PitchFamily` (`:pitchFamily`) on a `BulletFont` element.
pub fn bullet_font_with_pitch_family(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pitchFamily", value);
    el
}

/// Set `CharacterSet` (`:charset`) on a `BulletFont` element.
pub fn bullet_font_with_character_set(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("charset", value);
    el
}

/// Create a `<a:latin>` element (`LatinFont`).
pub fn latin_font() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "latin")
}

/// Set `Typeface` (`:typeface`) on a `LatinFont` element.
pub fn latin_font_with_typeface(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("typeface", value);
    el
}

/// Set `Panose` (`:panose`) on a `LatinFont` element.
pub fn latin_font_with_panose(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("panose", value);
    el
}

/// Set `PitchFamily` (`:pitchFamily`) on a `LatinFont` element.
pub fn latin_font_with_pitch_family(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pitchFamily", value);
    el
}

/// Set `CharacterSet` (`:charset`) on a `LatinFont` element.
pub fn latin_font_with_character_set(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("charset", value);
    el
}

/// Create a `<a:ea>` element (`EastAsianFont`).
pub fn east_asian_font() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ea")
}

/// Set `Typeface` (`:typeface`) on a `EastAsianFont` element.
pub fn east_asian_font_with_typeface(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("typeface", value);
    el
}

/// Set `Panose` (`:panose`) on a `EastAsianFont` element.
pub fn east_asian_font_with_panose(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("panose", value);
    el
}

/// Set `PitchFamily` (`:pitchFamily`) on a `EastAsianFont` element.
pub fn east_asian_font_with_pitch_family(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pitchFamily", value);
    el
}

/// Set `CharacterSet` (`:charset`) on a `EastAsianFont` element.
pub fn east_asian_font_with_character_set(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("charset", value);
    el
}

/// Create a `<a:cs>` element (`ComplexScriptFont`).
pub fn complex_script_font() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cs")
}

/// Set `Typeface` (`:typeface`) on a `ComplexScriptFont` element.
pub fn complex_script_font_with_typeface(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("typeface", value);
    el
}

/// Set `Panose` (`:panose`) on a `ComplexScriptFont` element.
pub fn complex_script_font_with_panose(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("panose", value);
    el
}

/// Set `PitchFamily` (`:pitchFamily`) on a `ComplexScriptFont` element.
pub fn complex_script_font_with_pitch_family(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pitchFamily", value);
    el
}

/// Set `CharacterSet` (`:charset`) on a `ComplexScriptFont` element.
pub fn complex_script_font_with_character_set(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("charset", value);
    el
}

/// Create a `<a:sym>` element (`SymbolFont`).
pub fn symbol_font() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "sym")
}

/// Set `Typeface` (`:typeface`) on a `SymbolFont` element.
pub fn symbol_font_with_typeface(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("typeface", value);
    el
}

/// Set `Panose` (`:panose`) on a `SymbolFont` element.
pub fn symbol_font_with_panose(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("panose", value);
    el
}

/// Set `PitchFamily` (`:pitchFamily`) on a `SymbolFont` element.
pub fn symbol_font_with_pitch_family(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pitchFamily", value);
    el
}

/// Set `CharacterSet` (`:charset`) on a `SymbolFont` element.
pub fn symbol_font_with_character_set(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("charset", value);
    el
}

/// Create a `<a:buNone>` element (`NoBullet`).
pub fn no_bullet() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buNone")
}

/// Create a `<a:buAutoNum>` element (`AutoNumberedBullet`).
pub fn auto_numbered_bullet() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buAutoNum")
}

/// Set `Type` (`:type`) on a `AutoNumberedBullet` element.
pub fn auto_numbered_bullet_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `StartAt` (`:startAt`) on a `AutoNumberedBullet` element.
pub fn auto_numbered_bullet_with_start_at(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("startAt", value);
    el
}

/// Create a `<a:buChar>` element (`CharacterBullet`).
pub fn character_bullet() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buChar")
}

/// Set `Char` (`:char`) on a `CharacterBullet` element.
pub fn character_bullet_with_char(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("char", value);
    el
}

/// Create `<a:buChar>` with `Char` set.
pub fn character_bullet_char(value: impl Into<String>) -> OpenXmlElement {
    character_bullet_with_char(character_bullet(), value)
}

/// Create a `<a:buBlip>` element (`PictureBullet`).
pub fn picture_bullet(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "buBlip").with_children(children)
}

/// Create a `<a:uLnTx>` element (`UnderlineFollowsText`).
pub fn underline_follows_text() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "uLnTx")
}

/// Create a `<a:uLn>` element (`Underline`).
pub fn underline(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "uLn").with_children(children)
}

/// Set `Width` (`:w`) on a `Underline` element.
pub fn underline_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `Underline` element.
pub fn underline_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `Underline` element.
pub fn underline_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Underline` element.
pub fn underline_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:ln>` element (`Outline`).
pub fn outline(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ln").with_children(children)
}

/// Set `Width` (`:w`) on a `Outline` element.
pub fn outline_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `Outline` element.
pub fn outline_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `Outline` element.
pub fn outline_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Outline` element.
pub fn outline_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:lnL>` element (`LeftBorderLineProperties`).
pub fn left_border_line_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnL").with_children(children)
}

/// Set `Width` (`:w`) on a `LeftBorderLineProperties` element.
pub fn left_border_line_properties_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `LeftBorderLineProperties` element.
pub fn left_border_line_properties_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `LeftBorderLineProperties` element.
pub fn left_border_line_properties_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `LeftBorderLineProperties` element.
pub fn left_border_line_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:lnR>` element (`RightBorderLineProperties`).
pub fn right_border_line_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnR").with_children(children)
}

/// Set `Width` (`:w`) on a `RightBorderLineProperties` element.
pub fn right_border_line_properties_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `RightBorderLineProperties` element.
pub fn right_border_line_properties_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `RightBorderLineProperties` element.
pub fn right_border_line_properties_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `RightBorderLineProperties` element.
pub fn right_border_line_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:lnT>` element (`TopBorderLineProperties`).
pub fn top_border_line_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnT").with_children(children)
}

/// Set `Width` (`:w`) on a `TopBorderLineProperties` element.
pub fn top_border_line_properties_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `TopBorderLineProperties` element.
pub fn top_border_line_properties_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `TopBorderLineProperties` element.
pub fn top_border_line_properties_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `TopBorderLineProperties` element.
pub fn top_border_line_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:lnB>` element (`BottomBorderLineProperties`).
pub fn bottom_border_line_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnB").with_children(children)
}

/// Set `Width` (`:w`) on a `BottomBorderLineProperties` element.
pub fn bottom_border_line_properties_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `BottomBorderLineProperties` element.
pub fn bottom_border_line_properties_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `BottomBorderLineProperties` element.
pub fn bottom_border_line_properties_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `BottomBorderLineProperties` element.
pub fn bottom_border_line_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:lnTlToBr>` element (`TopLeftToBottomRightBorderLineProperties`).
pub fn top_left_to_bottom_right_border_line_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnTlToBr").with_children(children)
}

/// Set `Width` (`:w`) on a `TopLeftToBottomRightBorderLineProperties` element.
pub fn top_left_to_bottom_right_border_line_properties_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `TopLeftToBottomRightBorderLineProperties` element.
pub fn top_left_to_bottom_right_border_line_properties_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `TopLeftToBottomRightBorderLineProperties` element.
pub fn top_left_to_bottom_right_border_line_properties_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `TopLeftToBottomRightBorderLineProperties` element.
pub fn top_left_to_bottom_right_border_line_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:lnBlToTr>` element (`BottomLeftToTopRightBorderLineProperties`).
pub fn bottom_left_to_top_right_border_line_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnBlToTr").with_children(children)
}

/// Set `Width` (`:w`) on a `BottomLeftToTopRightBorderLineProperties` element.
pub fn bottom_left_to_top_right_border_line_properties_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `CapType` (`:cap`) on a `BottomLeftToTopRightBorderLineProperties` element.
pub fn bottom_left_to_top_right_border_line_properties_with_cap_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `CompoundLineType` (`:cmpd`) on a `BottomLeftToTopRightBorderLineProperties` element.
pub fn bottom_left_to_top_right_border_line_properties_with_compound_line_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cmpd", value);
    el
}

/// Set `Alignment` (`:algn`) on a `BottomLeftToTopRightBorderLineProperties` element.
pub fn bottom_left_to_top_right_border_line_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:uFillTx>` element (`UnderlineFillText`).
pub fn underline_fill_text() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "uFillTx")
}

/// Create a `<a:uFill>` element (`UnderlineFill`).
pub fn underline_fill(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "uFill").with_children(children)
}

/// Create a `<a:r>` element (`Run`).
pub fn run(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "r").with_children(children)
}

/// Create a `<a:br>` element (`Break`).
pub fn break_(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "br").with_children(children)
}

/// Create a `<a:fld>` element (`Field`).
pub fn field(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fld").with_children(children)
}

/// Set `Id` (`:id`) on a `Field` element.
pub fn field_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `Type` (`:type`) on a `Field` element.
pub fn field_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Create a `<a:graphic>` element (`Graphic`).
pub fn graphic(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "graphic").with_children(children)
}

/// Create a `<a:blip>` element (`Blip`).
pub fn blip(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "blip").with_children(children)
}

/// Set `Embed` (`r:embed`) on a `Blip` element.
pub fn blip_with_embed(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:embed", value);
    el
}

/// Set `Link` (`r:link`) on a `Blip` element.
pub fn blip_with_link(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:link", value);
    el
}

/// Set `CompressionState` (`:cstate`) on a `Blip` element.
pub fn blip_with_compression_state(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cstate", value);
    el
}

/// Create a `<a:theme>` element (`Theme`).
pub fn theme(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "theme").with_children(children)
}

/// Set `Name` (`:name`) on a `Theme` element.
pub fn theme_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `ThemeId` (`thm15:id`) on a `Theme` element.
pub fn theme_with_theme_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("thm15:id", value);
    el
}

/// Create a `<a:themeOverride>` element (`ThemeOverride`).
pub fn theme_override(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "themeOverride").with_children(children)
}

/// Create a `<a:themeManager>` element (`ThemeManager`).
pub fn theme_manager() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "themeManager")
}

/// Create a `<a:masterClrMapping>` element (`MasterColorMapping`).
pub fn master_color_mapping() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "masterClrMapping")
}

/// Create a `<a:tbl>` element (`Table`).
pub fn table(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tbl").with_children(children)
}

/// Create a `<a:tblStyleLst>` element (`TableStyleList`).
pub fn table_style_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tblStyleLst").with_children(children)
}

/// Set `Default` (`:def`) on a `TableStyleList` element.
pub fn table_style_list_with_default(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("def", value);
    el
}

/// Create a `<a:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:st>` element (`StartTime`).
pub fn start_time() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "st")
}

/// Set `Track` (`:track`) on a `StartTime` element.
pub fn start_time_with_track(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("track", value);
    el
}

/// Set `Time` (`:time`) on a `StartTime` element.
pub fn start_time_with_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("time", value);
    el
}

/// Create a `<a:end>` element (`EndTime`).
pub fn end_time() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "end")
}

/// Set `Track` (`:track`) on a `EndTime` element.
pub fn end_time_with_track(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("track", value);
    el
}

/// Set `Time` (`:time`) on a `EndTime` element.
pub fn end_time_with_time(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("time", value);
    el
}

/// Create a `<a:custClr>` element (`CustomColor`).
pub fn custom_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "custClr").with_children(children)
}

/// Set `Name` (`:name`) on a `CustomColor` element.
pub fn custom_color_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Create a `<a:font>` element (`SupplementalFont`).
pub fn supplemental_font() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "font")
}

/// Set `Script` (`:script`) on a `SupplementalFont` element.
pub fn supplemental_font_with_script(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("script", value);
    el
}

/// Set `Typeface` (`:typeface`) on a `SupplementalFont` element.
pub fn supplemental_font_with_typeface(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("typeface", value);
    el
}

/// Create a `<a:scene3d>` element (`Scene3DType`).
pub fn scene3_d_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "scene3d").with_children(children)
}

/// Create a `<a:effectStyle>` element (`EffectStyle`).
pub fn effect_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "effectStyle").with_children(children)
}

/// Create a `<a:fillStyleLst>` element (`FillStyleList`).
pub fn fill_style_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fillStyleLst").with_children(children)
}

/// Create a `<a:lnStyleLst>` element (`LineStyleList`).
pub fn line_style_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnStyleLst").with_children(children)
}

/// Create a `<a:effectStyleLst>` element (`EffectStyleList`).
pub fn effect_style_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "effectStyleLst").with_children(children)
}

/// Create a `<a:bgFillStyleLst>` element (`BackgroundFillStyleList`).
pub fn background_fill_style_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bgFillStyleLst").with_children(children)
}

/// Create a `<a:clrScheme>` element (`ColorScheme`).
pub fn color_scheme(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "clrScheme").with_children(children)
}

/// Set `Name` (`:name`) on a `ColorScheme` element.
pub fn color_scheme_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Create a `<a:fontScheme>` element (`FontScheme`).
pub fn font_scheme(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fontScheme").with_children(children)
}

/// Set `Name` (`:name`) on a `FontScheme` element.
pub fn font_scheme_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Create a `<a:fmtScheme>` element (`FormatScheme`).
pub fn format_scheme(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fmtScheme").with_children(children)
}

/// Set `Name` (`:name`) on a `FormatScheme` element.
pub fn format_scheme_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Create a `<a:dk1>` element (`Dark1Color`).
pub fn dark1_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "dk1").with_children(children)
}

/// Create a `<a:lt1>` element (`Light1Color`).
pub fn light1_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lt1").with_children(children)
}

/// Create a `<a:dk2>` element (`Dark2Color`).
pub fn dark2_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "dk2").with_children(children)
}

/// Create a `<a:lt2>` element (`Light2Color`).
pub fn light2_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lt2").with_children(children)
}

/// Create a `<a:accent1>` element (`Accent1Color`).
pub fn accent1_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "accent1").with_children(children)
}

/// Create a `<a:accent2>` element (`Accent2Color`).
pub fn accent2_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "accent2").with_children(children)
}

/// Create a `<a:accent3>` element (`Accent3Color`).
pub fn accent3_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "accent3").with_children(children)
}

/// Create a `<a:accent4>` element (`Accent4Color`).
pub fn accent4_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "accent4").with_children(children)
}

/// Create a `<a:accent5>` element (`Accent5Color`).
pub fn accent5_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "accent5").with_children(children)
}

/// Create a `<a:accent6>` element (`Accent6Color`).
pub fn accent6_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "accent6").with_children(children)
}

/// Create a `<a:hlink>` element (`Hyperlink`).
pub fn hyperlink(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hlink").with_children(children)
}

/// Create a `<a:folHlink>` element (`FollowedHyperlinkColor`).
pub fn followed_hyperlink_color(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "folHlink").with_children(children)
}

/// Create a `<a:sx>` element (`ScaleX`).
pub fn scale_x() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "sx")
}

/// Set `Numerator` (`:n`) on a `ScaleX` element.
pub fn scale_x_with_numerator(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("n", value);
    el
}

/// Set `Denominator` (`:d`) on a `ScaleX` element.
pub fn scale_x_with_denominator(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("d", value);
    el
}

/// Create a `<a:sy>` element (`ScaleY`).
pub fn scale_y() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "sy")
}

/// Set `Numerator` (`:n`) on a `ScaleY` element.
pub fn scale_y_with_numerator(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("n", value);
    el
}

/// Set `Denominator` (`:d`) on a `ScaleY` element.
pub fn scale_y_with_denominator(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("d", value);
    el
}

/// Create a `<a:off>` element (`Offset`).
pub fn offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "off")
}

/// Set `X` (`:x`) on a `Offset` element.
pub fn offset_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `Offset` element.
pub fn offset_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<a:chOff>` element (`ChildOffset`).
pub fn child_offset() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "chOff")
}

/// Set `X` (`:x`) on a `ChildOffset` element.
pub fn child_offset_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `ChildOffset` element.
pub fn child_offset_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<a:ext>` element (`Extents`).
pub fn extents() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext")
}

/// Set `Cx` (`:cx`) on a `Extents` element.
pub fn extents_with_cx(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cx", value);
    el
}

/// Set `Cy` (`:cy`) on a `Extents` element.
pub fn extents_with_cy(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cy", value);
    el
}

/// Create a `<a:chExt>` element (`ChildExtents`).
pub fn child_extents() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "chExt")
}

/// Set `Cx` (`:cx`) on a `ChildExtents` element.
pub fn child_extents_with_cx(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cx", value);
    el
}

/// Set `Cy` (`:cy`) on a `ChildExtents` element.
pub fn child_extents_with_cy(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cy", value);
    el
}

/// Create a `<a:spLocks>` element (`ShapeLocks`).
pub fn shape_locks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spLocks").with_children(children)
}

/// Set `NoGrouping` (`:noGrp`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_grouping(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noGrp", value);
    el
}

/// Set `NoSelection` (`:noSelect`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_selection(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noSelect", value);
    el
}

/// Set `NoRotation` (`:noRot`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noRot", value);
    el
}

/// Set `NoChangeAspect` (`:noChangeAspect`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_change_aspect(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeAspect", value);
    el
}

/// Set `NoMove` (`:noMove`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_move(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noMove", value);
    el
}

/// Set `NoResize` (`:noResize`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_resize(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noResize", value);
    el
}

/// Set `NoEditPoints` (`:noEditPoints`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_edit_points(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noEditPoints", value);
    el
}

/// Set `NoAdjustHandles` (`:noAdjustHandles`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_adjust_handles(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noAdjustHandles", value);
    el
}

/// Set `NoChangeArrowheads` (`:noChangeArrowheads`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_change_arrowheads(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeArrowheads", value);
    el
}

/// Set `NoChangeShapeType` (`:noChangeShapeType`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_change_shape_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeShapeType", value);
    el
}

/// Set `NoTextEdit` (`:noTextEdit`) on a `ShapeLocks` element.
pub fn shape_locks_with_no_text_edit(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noTextEdit", value);
    el
}

/// Create a `<a:cxnSpLocks>` element (`ConnectionShapeLocks`).
pub fn connection_shape_locks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cxnSpLocks").with_children(children)
}

/// Set `NoGrouping` (`:noGrp`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_grouping(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noGrp", value);
    el
}

/// Set `NoSelection` (`:noSelect`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_selection(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noSelect", value);
    el
}

/// Set `NoRotation` (`:noRot`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noRot", value);
    el
}

/// Set `NoChangeAspect` (`:noChangeAspect`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_change_aspect(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeAspect", value);
    el
}

/// Set `NoMove` (`:noMove`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_move(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noMove", value);
    el
}

/// Set `NoResize` (`:noResize`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_resize(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noResize", value);
    el
}

/// Set `NoEditPoints` (`:noEditPoints`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_edit_points(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noEditPoints", value);
    el
}

/// Set `NoAdjustHandles` (`:noAdjustHandles`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_adjust_handles(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noAdjustHandles", value);
    el
}

/// Set `NoChangeArrowheads` (`:noChangeArrowheads`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_change_arrowheads(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeArrowheads", value);
    el
}

/// Set `NoChangeShapeType` (`:noChangeShapeType`) on a `ConnectionShapeLocks` element.
pub fn connection_shape_locks_with_no_change_shape_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeShapeType", value);
    el
}

/// Create a `<a:stCxn>` element (`StartConnection`).
pub fn start_connection() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "stCxn")
}

/// Set `Id` (`:id`) on a `StartConnection` element.
pub fn start_connection_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `Index` (`:idx`) on a `StartConnection` element.
pub fn start_connection_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Create a `<a:endCxn>` element (`EndConnection`).
pub fn end_connection() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "endCxn")
}

/// Set `Id` (`:id`) on a `EndConnection` element.
pub fn end_connection_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `Index` (`:idx`) on a `EndConnection` element.
pub fn end_connection_with_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("idx", value);
    el
}

/// Create a `<a:graphicFrameLocks>` element (`GraphicFrameLocks`).
pub fn graphic_frame_locks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "graphicFrameLocks").with_children(children)
}

/// Set `NoGrouping` (`:noGrp`) on a `GraphicFrameLocks` element.
pub fn graphic_frame_locks_with_no_grouping(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noGrp", value);
    el
}

/// Set `NoDrilldown` (`:noDrilldown`) on a `GraphicFrameLocks` element.
pub fn graphic_frame_locks_with_no_drilldown(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noDrilldown", value);
    el
}

/// Set `NoSelection` (`:noSelect`) on a `GraphicFrameLocks` element.
pub fn graphic_frame_locks_with_no_selection(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noSelect", value);
    el
}

/// Set `NoChangeAspect` (`:noChangeAspect`) on a `GraphicFrameLocks` element.
pub fn graphic_frame_locks_with_no_change_aspect(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeAspect", value);
    el
}

/// Set `NoMove` (`:noMove`) on a `GraphicFrameLocks` element.
pub fn graphic_frame_locks_with_no_move(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noMove", value);
    el
}

/// Set `NoResize` (`:noResize`) on a `GraphicFrameLocks` element.
pub fn graphic_frame_locks_with_no_resize(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noResize", value);
    el
}

/// Create a `<a:graphicData>` element (`GraphicData`).
pub fn graphic_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "graphicData").with_children(children)
}

/// Set `Uri` (`:uri`) on a `GraphicData` element.
pub fn graphic_data_with_uri(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("uri", value);
    el
}

/// Create a `<a:dgm>` element (`Diagram`).
pub fn diagram() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "dgm")
}

/// Set `Id` (`:id`) on a `Diagram` element.
pub fn diagram_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `BuildStep` (`:bldStep`) on a `Diagram` element.
pub fn diagram_with_build_step(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bldStep", value);
    el
}

/// Create a `<a:chart>` element (`Chart`).
pub fn chart() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "chart")
}

/// Set `SeriesIndex` (`:seriesIdx`) on a `Chart` element.
pub fn chart_with_series_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("seriesIdx", value);
    el
}

/// Set `CategoryIndex` (`:categoryIdx`) on a `Chart` element.
pub fn chart_with_category_index(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("categoryIdx", value);
    el
}

/// Set `BuildStep` (`:bldStep`) on a `Chart` element.
pub fn chart_with_build_step(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bldStep", value);
    el
}

/// Create a `<a:bldDgm>` element (`BuildDiagram`).
pub fn build_diagram() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bldDgm")
}

/// Set `Build` (`:bld`) on a `BuildDiagram` element.
pub fn build_diagram_with_build(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bld", value);
    el
}

/// Set `ReverseAnimation` (`:rev`) on a `BuildDiagram` element.
pub fn build_diagram_with_reverse_animation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rev", value);
    el
}

/// Create a `<a:bldChart>` element (`BuildChart`).
pub fn build_chart() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bldChart")
}

/// Set `Build` (`:bld`) on a `BuildChart` element.
pub fn build_chart_with_build(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bld", value);
    el
}

/// Set `AnimateBackground` (`:animBg`) on a `BuildChart` element.
pub fn build_chart_with_animate_background(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("animBg", value);
    el
}

/// Create a `<a:txBody>` element (`TextBody`).
pub fn text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "txBody").with_children(children)
}

/// Create a `<a:useSpRect>` element (`UseShapeRectangle`).
pub fn use_shape_rectangle() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "useSpRect")
}

/// Create a `<a:xfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Set `Rotation` (`:rot`) on a `Transform2D` element.
pub fn transform2_d_with_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rot", value);
    el
}

/// Set `HorizontalFlip` (`:flipH`) on a `Transform2D` element.
pub fn transform2_d_with_horizontal_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flipH", value);
    el
}

/// Set `VerticalFlip` (`:flipV`) on a `Transform2D` element.
pub fn transform2_d_with_vertical_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flipV", value);
    el
}

/// Create a `<a:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Set `Id` (`:id`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("id", value);
    el
}

/// Set `Name` (`:name`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `Description` (`:descr`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_description(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("descr", value);
    el
}

/// Set `Hidden` (`:hidden`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_hidden(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hidden", value);
    el
}

/// Set `Title` (`:title`) on a `NonVisualDrawingProperties` element.
pub fn non_visual_drawing_properties_with_title(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("title", value);
    el
}

/// Create a `<a:cNvSpPr>` element (`NonVisualShapeDrawingProperties`).
pub fn non_visual_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cNvSpPr").with_children(children)
}

/// Set `TextBox` (`:txBox`) on a `NonVisualShapeDrawingProperties` element.
pub fn non_visual_shape_drawing_properties_with_text_box(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("txBox", value);
    el
}

/// Create a `<a:nvSpPr>` element (`NonVisualShapeProperties`).
pub fn non_visual_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "nvSpPr").with_children(children)
}

/// Create a `<a:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spPr").with_children(children)
}

/// Set `BlackWhiteMode` (`:bwMode`) on a `ShapeProperties` element.
pub fn shape_properties_with_black_white_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bwMode", value);
    el
}

/// Create a `<a:txSp>` element (`TextShape`).
pub fn text_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "txSp").with_children(children)
}

/// Create a `<a:style>` element (`ShapeStyle`).
pub fn shape_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<a:cNvCxnSpPr>` element (`NonVisualConnectorShapeDrawingProperties`).
pub fn non_visual_connector_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cNvCxnSpPr").with_children(children)
}

/// Create a `<a:nvCxnSpPr>` element (`NonVisualConnectionShapeProperties`).
pub fn non_visual_connection_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "nvCxnSpPr").with_children(children)
}

/// Create a `<a:cNvPicPr>` element (`NonVisualPictureDrawingProperties`).
pub fn non_visual_picture_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cNvPicPr").with_children(children)
}

/// Set `PreferRelativeResize` (`:preferRelativeResize`) on a `NonVisualPictureDrawingProperties` element.
pub fn non_visual_picture_drawing_properties_with_prefer_relative_resize(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("preferRelativeResize", value);
    el
}

/// Create a `<a:nvPicPr>` element (`NonVisualPictureProperties`).
pub fn non_visual_picture_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "nvPicPr").with_children(children)
}

/// Create a `<a:cNvGraphicFramePr>` element (`NonVisualGraphicFrameDrawingProperties`).
pub fn non_visual_graphic_frame_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cNvGraphicFramePr").with_children(children)
}

/// Create a `<a:nvGraphicFramePr>` element (`NonVisualGraphicFrameProperties`).
pub fn non_visual_graphic_frame_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "nvGraphicFramePr").with_children(children)
}

/// Create a `<a:cNvGrpSpPr>` element (`NonVisualGroupShapeDrawingProperties`).
pub fn non_visual_group_shape_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cNvGrpSpPr").with_children(children)
}

/// Create a `<a:rot>` element (`Rotation`).
pub fn rotation() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "rot")
}

/// Set `Latitude` (`:lat`) on a `Rotation` element.
pub fn rotation_with_latitude(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lat", value);
    el
}

/// Set `Longitude` (`:lon`) on a `Rotation` element.
pub fn rotation_with_longitude(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lon", value);
    el
}

/// Set `Revolution` (`:rev`) on a `Rotation` element.
pub fn rotation_with_revolution(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rev", value);
    el
}

/// Create a `<a:camera>` element (`Camera`).
pub fn camera(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "camera").with_children(children)
}

/// Set `Preset` (`:prst`) on a `Camera` element.
pub fn camera_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Set `FieldOfView` (`:fov`) on a `Camera` element.
pub fn camera_with_field_of_view(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fov", value);
    el
}

/// Set `Zoom` (`:zoom`) on a `Camera` element.
pub fn camera_with_zoom(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("zoom", value);
    el
}

/// Create a `<a:lightRig>` element (`LightRig`).
pub fn light_rig(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lightRig").with_children(children)
}

/// Set `Rig` (`:rig`) on a `LightRig` element.
pub fn light_rig_with_rig(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rig", value);
    el
}

/// Set `Direction` (`:dir`) on a `LightRig` element.
pub fn light_rig_with_direction(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dir", value);
    el
}

/// Create a `<a:backdrop>` element (`Backdrop`).
pub fn backdrop(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "backdrop").with_children(children)
}

/// Create a `<a:anchor>` element (`Anchor`).
pub fn anchor() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "anchor")
}

/// Set `X` (`:x`) on a `Anchor` element.
pub fn anchor_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `Anchor` element.
pub fn anchor_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Set `Z` (`:z`) on a `Anchor` element.
pub fn anchor_with_z(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("z", value);
    el
}

/// Create a `<a:norm>` element (`Normal`).
pub fn normal() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "norm")
}

/// Set `Dx` (`:dx`) on a `Normal` element.
pub fn normal_with_dx(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dx", value);
    el
}

/// Set `Dy` (`:dy`) on a `Normal` element.
pub fn normal_with_dy(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dy", value);
    el
}

/// Set `Dz` (`:dz`) on a `Normal` element.
pub fn normal_with_dz(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dz", value);
    el
}

/// Create a `<a:up>` element (`UpVector`).
pub fn up_vector() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "up")
}

/// Set `Dx` (`:dx`) on a `UpVector` element.
pub fn up_vector_with_dx(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dx", value);
    el
}

/// Set `Dy` (`:dy`) on a `UpVector` element.
pub fn up_vector_with_dy(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dy", value);
    el
}

/// Set `Dz` (`:dz`) on a `UpVector` element.
pub fn up_vector_with_dz(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dz", value);
    el
}

/// Create a `<a:bevelT>` element (`BevelTop`).
pub fn bevel_top() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bevelT")
}

/// Set `Width` (`:w`) on a `BevelTop` element.
pub fn bevel_top_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `Height` (`:h`) on a `BevelTop` element.
pub fn bevel_top_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("h", value);
    el
}

/// Set `Preset` (`:prst`) on a `BevelTop` element.
pub fn bevel_top_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Create a `<a:bevelB>` element (`BevelBottom`).
pub fn bevel_bottom() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bevelB")
}

/// Set `Width` (`:w`) on a `BevelBottom` element.
pub fn bevel_bottom_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `Height` (`:h`) on a `BevelBottom` element.
pub fn bevel_bottom_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("h", value);
    el
}

/// Set `Preset` (`:prst`) on a `BevelBottom` element.
pub fn bevel_bottom_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Create a `<a:bevel>` element (`Bevel`).
pub fn bevel() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bevel")
}

/// Set `Width` (`:w`) on a `Bevel` element.
pub fn bevel_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `Height` (`:h`) on a `Bevel` element.
pub fn bevel_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("h", value);
    el
}

/// Set `Preset` (`:prst`) on a `Bevel` element.
pub fn bevel_with_preset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prst", value);
    el
}

/// Create a `<a:fillToRect>` element (`FillToRectangle`).
pub fn fill_to_rectangle() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fillToRect")
}

/// Set `Left` (`:l`) on a `FillToRectangle` element.
pub fn fill_to_rectangle_with_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("l", value);
    el
}

/// Set `Top` (`:t`) on a `FillToRectangle` element.
pub fn fill_to_rectangle_with_top(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("t", value);
    el
}

/// Set `Right` (`:r`) on a `FillToRectangle` element.
pub fn fill_to_rectangle_with_right(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r", value);
    el
}

/// Set `Bottom` (`:b`) on a `FillToRectangle` element.
pub fn fill_to_rectangle_with_bottom(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Create a `<a:tileRect>` element (`TileRectangle`).
pub fn tile_rectangle() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tileRect")
}

/// Set `Left` (`:l`) on a `TileRectangle` element.
pub fn tile_rectangle_with_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("l", value);
    el
}

/// Set `Top` (`:t`) on a `TileRectangle` element.
pub fn tile_rectangle_with_top(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("t", value);
    el
}

/// Set `Right` (`:r`) on a `TileRectangle` element.
pub fn tile_rectangle_with_right(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r", value);
    el
}

/// Set `Bottom` (`:b`) on a `TileRectangle` element.
pub fn tile_rectangle_with_bottom(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Create a `<a:fillRect>` element (`FillRectangle`).
pub fn fill_rectangle() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "fillRect")
}

/// Set `Left` (`:l`) on a `FillRectangle` element.
pub fn fill_rectangle_with_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("l", value);
    el
}

/// Set `Top` (`:t`) on a `FillRectangle` element.
pub fn fill_rectangle_with_top(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("t", value);
    el
}

/// Set `Right` (`:r`) on a `FillRectangle` element.
pub fn fill_rectangle_with_right(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r", value);
    el
}

/// Set `Bottom` (`:b`) on a `FillRectangle` element.
pub fn fill_rectangle_with_bottom(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Create a `<a:srcRect>` element (`SourceRectangle`).
pub fn source_rectangle() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "srcRect")
}

/// Set `Left` (`:l`) on a `SourceRectangle` element.
pub fn source_rectangle_with_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("l", value);
    el
}

/// Set `Top` (`:t`) on a `SourceRectangle` element.
pub fn source_rectangle_with_top(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("t", value);
    el
}

/// Set `Right` (`:r`) on a `SourceRectangle` element.
pub fn source_rectangle_with_right(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r", value);
    el
}

/// Set `Bottom` (`:b`) on a `SourceRectangle` element.
pub fn source_rectangle_with_bottom(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Create a `<a:gs>` element (`GradientStop`).
pub fn gradient_stop(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gs").with_children(children)
}

/// Set `Position` (`:pos`) on a `GradientStop` element.
pub fn gradient_stop_with_position(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pos", value);
    el
}

/// Create a `<a:gsLst>` element (`GradientStopList`).
pub fn gradient_stop_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gsLst").with_children(children)
}

/// Create a `<a:gd>` element (`ShapeGuide`).
pub fn shape_guide() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gd")
}

/// Set `Name` (`:name`) on a `ShapeGuide` element.
pub fn shape_guide_with_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("name", value);
    el
}

/// Set `Formula` (`:fmla`) on a `ShapeGuide` element.
pub fn shape_guide_with_formula(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fmla", value);
    el
}

/// Create a `<a:pos>` element (`Position`).
pub fn position() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "pos")
}

/// Set `X` (`:x`) on a `Position` element.
pub fn position_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `Position` element.
pub fn position_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<a:pt>` element (`Point`).
pub fn point() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "pt")
}

/// Set `X` (`:x`) on a `Point` element.
pub fn point_with_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("x", value);
    el
}

/// Set `Y` (`:y`) on a `Point` element.
pub fn point_with_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("y", value);
    el
}

/// Create a `<a:ahXY>` element (`AdjustHandleXY`).
pub fn adjust_handle_x_y(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ahXY").with_children(children)
}

/// Set `XAdjustmentGuide` (`:gdRefX`) on a `AdjustHandleXY` element.
pub fn adjust_handle_x_y_with_x_adjustment_guide(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("gdRefX", value);
    el
}

/// Set `MinX` (`:minX`) on a `AdjustHandleXY` element.
pub fn adjust_handle_x_y_with_min_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("minX", value);
    el
}

/// Set `MaxX` (`:maxX`) on a `AdjustHandleXY` element.
pub fn adjust_handle_x_y_with_max_x(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("maxX", value);
    el
}

/// Set `YAdjustmentGuide` (`:gdRefY`) on a `AdjustHandleXY` element.
pub fn adjust_handle_x_y_with_y_adjustment_guide(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("gdRefY", value);
    el
}

/// Set `MinY` (`:minY`) on a `AdjustHandleXY` element.
pub fn adjust_handle_x_y_with_min_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("minY", value);
    el
}

/// Set `MaxY` (`:maxY`) on a `AdjustHandleXY` element.
pub fn adjust_handle_x_y_with_max_y(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("maxY", value);
    el
}

/// Create a `<a:ahPolar>` element (`AdjustHandlePolar`).
pub fn adjust_handle_polar(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ahPolar").with_children(children)
}

/// Set `RadialAdjustmentGuide` (`:gdRefR`) on a `AdjustHandlePolar` element.
pub fn adjust_handle_polar_with_radial_adjustment_guide(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("gdRefR", value);
    el
}

/// Set `MinRadial` (`:minR`) on a `AdjustHandlePolar` element.
pub fn adjust_handle_polar_with_min_radial(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("minR", value);
    el
}

/// Set `MaxRadial` (`:maxR`) on a `AdjustHandlePolar` element.
pub fn adjust_handle_polar_with_max_radial(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("maxR", value);
    el
}

/// Set `AngleAdjustmentGuide` (`:gdRefAng`) on a `AdjustHandlePolar` element.
pub fn adjust_handle_polar_with_angle_adjustment_guide(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("gdRefAng", value);
    el
}

/// Set `MinAngle` (`:minAng`) on a `AdjustHandlePolar` element.
pub fn adjust_handle_polar_with_min_angle(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("minAng", value);
    el
}

/// Set `MaxAngle` (`:maxAng`) on a `AdjustHandlePolar` element.
pub fn adjust_handle_polar_with_max_angle(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("maxAng", value);
    el
}

/// Create a `<a:cxn>` element (`ConnectionSite`).
pub fn connection_site(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cxn").with_children(children)
}

/// Set `Angle` (`:ang`) on a `ConnectionSite` element.
pub fn connection_site_with_angle(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("ang", value);
    el
}

/// Create a `<a:close>` element (`CloseShapePath`).
pub fn close_shape_path() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "close")
}

/// Create a `<a:moveTo>` element (`MoveTo`).
pub fn move_to(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "moveTo").with_children(children)
}

/// Create a `<a:lnTo>` element (`LineTo`).
pub fn line_to(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnTo").with_children(children)
}

/// Create a `<a:arcTo>` element (`ArcTo`).
pub fn arc_to() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "arcTo")
}

/// Set `WidthRadius` (`:wR`) on a `ArcTo` element.
pub fn arc_to_with_width_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("wR", value);
    el
}

/// Set `HeightRadius` (`:hR`) on a `ArcTo` element.
pub fn arc_to_with_height_radius(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hR", value);
    el
}

/// Set `StartAngle` (`:stAng`) on a `ArcTo` element.
pub fn arc_to_with_start_angle(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("stAng", value);
    el
}

/// Set `SwingAngle` (`:swAng`) on a `ArcTo` element.
pub fn arc_to_with_swing_angle(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("swAng", value);
    el
}

/// Create a `<a:quadBezTo>` element (`QuadraticBezierCurveTo`).
pub fn quadratic_bezier_curve_to(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "quadBezTo").with_children(children)
}

/// Create a `<a:cubicBezTo>` element (`CubicBezierCurveTo`).
pub fn cubic_bezier_curve_to(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cubicBezTo").with_children(children)
}

/// Create a `<a:path>` element (`Path`).
pub fn path(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "path").with_children(children)
}

/// Set `Width` (`:w`) on a `Path` element.
pub fn path_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `Height` (`:h`) on a `Path` element.
pub fn path_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("h", value);
    el
}

/// Set `Fill` (`:fill`) on a `Path` element.
pub fn path_with_fill(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fill", value);
    el
}

/// Set `Stroke` (`:stroke`) on a `Path` element.
pub fn path_with_stroke(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("stroke", value);
    el
}

/// Set `ExtrusionOk` (`:extrusionOk`) on a `Path` element.
pub fn path_with_extrusion_ok(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("extrusionOk", value);
    el
}

/// Create a `<a:avLst>` element (`AdjustValueList`).
pub fn adjust_value_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "avLst").with_children(children)
}

/// Create a `<a:gdLst>` element (`ShapeGuideList`).
pub fn shape_guide_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gdLst").with_children(children)
}

/// Create a `<a:ahLst>` element (`AdjustHandleList`).
pub fn adjust_handle_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ahLst").with_children(children)
}

/// Create a `<a:cxnLst>` element (`ConnectionSiteList`).
pub fn connection_site_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cxnLst").with_children(children)
}

/// Create a `<a:rect>` element (`Rectangle`).
pub fn rectangle() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "rect")
}

/// Set `Left` (`:l`) on a `Rectangle` element.
pub fn rectangle_with_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("l", value);
    el
}

/// Set `Top` (`:t`) on a `Rectangle` element.
pub fn rectangle_with_top(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("t", value);
    el
}

/// Set `Right` (`:r`) on a `Rectangle` element.
pub fn rectangle_with_right(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r", value);
    el
}

/// Set `Bottom` (`:b`) on a `Rectangle` element.
pub fn rectangle_with_bottom(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Create a `<a:pathLst>` element (`PathList`).
pub fn path_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "pathLst").with_children(children)
}

/// Create a `<a:ds>` element (`DashStop`).
pub fn dash_stop() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ds")
}

/// Set `DashLength` (`:d`) on a `DashStop` element.
pub fn dash_stop_with_dash_length(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("d", value);
    el
}

/// Set `SpaceLength` (`:sp`) on a `DashStop` element.
pub fn dash_stop_with_space_length(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sp", value);
    el
}

/// Create a `<a:xfrm>` element (`TransformGroup`).
pub fn transform_group(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Set `Rotation` (`:rot`) on a `TransformGroup` element.
pub fn transform_group_with_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rot", value);
    el
}

/// Set `HorizontalFlip` (`:flipH`) on a `TransformGroup` element.
pub fn transform_group_with_horizontal_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flipH", value);
    el
}

/// Set `VerticalFlip` (`:flipV`) on a `TransformGroup` element.
pub fn transform_group_with_vertical_flip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("flipV", value);
    el
}

/// Create a `<a:bodyPr>` element (`BodyProperties`).
pub fn body_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bodyPr").with_children(children)
}

/// Set `Rotation` (`:rot`) on a `BodyProperties` element.
pub fn body_properties_with_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rot", value);
    el
}

/// Set `UseParagraphSpacing` (`:spcFirstLastPara`) on a `BodyProperties` element.
pub fn body_properties_with_use_paragraph_spacing(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spcFirstLastPara", value);
    el
}

/// Set `VerticalOverflow` (`:vertOverflow`) on a `BodyProperties` element.
pub fn body_properties_with_vertical_overflow(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("vertOverflow", value);
    el
}

/// Set `HorizontalOverflow` (`:horzOverflow`) on a `BodyProperties` element.
pub fn body_properties_with_horizontal_overflow(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("horzOverflow", value);
    el
}

/// Set `Vertical` (`:vert`) on a `BodyProperties` element.
pub fn body_properties_with_vertical(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("vert", value);
    el
}

/// Set `Wrap` (`:wrap`) on a `BodyProperties` element.
pub fn body_properties_with_wrap(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("wrap", value);
    el
}

/// Set `LeftInset` (`:lIns`) on a `BodyProperties` element.
pub fn body_properties_with_left_inset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lIns", value);
    el
}

/// Set `TopInset` (`:tIns`) on a `BodyProperties` element.
pub fn body_properties_with_top_inset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tIns", value);
    el
}

/// Set `RightInset` (`:rIns`) on a `BodyProperties` element.
pub fn body_properties_with_right_inset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rIns", value);
    el
}

/// Set `BottomInset` (`:bIns`) on a `BodyProperties` element.
pub fn body_properties_with_bottom_inset(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bIns", value);
    el
}

/// Set `ColumnCount` (`:numCol`) on a `BodyProperties` element.
pub fn body_properties_with_column_count(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("numCol", value);
    el
}

/// Set `ColumnSpacing` (`:spcCol`) on a `BodyProperties` element.
pub fn body_properties_with_column_spacing(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spcCol", value);
    el
}

/// Set `RightToLeftColumns` (`:rtlCol`) on a `BodyProperties` element.
pub fn body_properties_with_right_to_left_columns(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtlCol", value);
    el
}

/// Set `FromWordArt` (`:fromWordArt`) on a `BodyProperties` element.
pub fn body_properties_with_from_word_art(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fromWordArt", value);
    el
}

/// Set `Anchor` (`:anchor`) on a `BodyProperties` element.
pub fn body_properties_with_anchor(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("anchor", value);
    el
}

/// Set `AnchorCenter` (`:anchorCtr`) on a `BodyProperties` element.
pub fn body_properties_with_anchor_center(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("anchorCtr", value);
    el
}

/// Set `ForceAntiAlias` (`:forceAA`) on a `BodyProperties` element.
pub fn body_properties_with_force_anti_alias(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("forceAA", value);
    el
}

/// Set `UpRight` (`:upright`) on a `BodyProperties` element.
pub fn body_properties_with_up_right(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("upright", value);
    el
}

/// Set `CompatibleLineSpacing` (`:compatLnSpc`) on a `BodyProperties` element.
pub fn body_properties_with_compatible_line_spacing(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("compatLnSpc", value);
    el
}

/// Create a `<a:lstStyle>` element (`ListStyle`).
pub fn list_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lstStyle").with_children(children)
}

/// Create a `<a:spDef>` element (`ShapeDefault`).
pub fn shape_default(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spDef").with_children(children)
}

/// Create a `<a:lnDef>` element (`LineDefault`).
pub fn line_default(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnDef").with_children(children)
}

/// Create a `<a:txDef>` element (`TextDefault`).
pub fn text_default(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "txDef").with_children(children)
}

/// Create a `<a:overrideClrMapping>` element (`OverrideColorMapping`).
pub fn override_color_mapping(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "overrideClrMapping").with_children(children)
}

/// Set `Background1` (`:bg1`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_background1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bg1", value);
    el
}

/// Set `Text1` (`:tx1`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_text1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx1", value);
    el
}

/// Set `Background2` (`:bg2`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_background2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bg2", value);
    el
}

/// Set `Text2` (`:tx2`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_text2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx2", value);
    el
}

/// Set `Accent1` (`:accent1`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_accent1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent1", value);
    el
}

/// Set `Accent2` (`:accent2`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_accent2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent2", value);
    el
}

/// Set `Accent3` (`:accent3`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_accent3(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent3", value);
    el
}

/// Set `Accent4` (`:accent4`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_accent4(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent4", value);
    el
}

/// Set `Accent5` (`:accent5`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_accent5(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent5", value);
    el
}

/// Set `Accent6` (`:accent6`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_accent6(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent6", value);
    el
}

/// Set `Hyperlink` (`:hlink`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_hyperlink(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hlink", value);
    el
}

/// Set `FollowedHyperlink` (`:folHlink`) on a `OverrideColorMapping` element.
pub fn override_color_mapping_with_followed_hyperlink(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("folHlink", value);
    el
}

/// Create a `<a:clrMap>` element (`ColorMap`).
pub fn color_map(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "clrMap").with_children(children)
}

/// Set `Background1` (`:bg1`) on a `ColorMap` element.
pub fn color_map_with_background1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bg1", value);
    el
}

/// Set `Text1` (`:tx1`) on a `ColorMap` element.
pub fn color_map_with_text1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx1", value);
    el
}

/// Set `Background2` (`:bg2`) on a `ColorMap` element.
pub fn color_map_with_background2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bg2", value);
    el
}

/// Set `Text2` (`:tx2`) on a `ColorMap` element.
pub fn color_map_with_text2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tx2", value);
    el
}

/// Set `Accent1` (`:accent1`) on a `ColorMap` element.
pub fn color_map_with_accent1(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent1", value);
    el
}

/// Set `Accent2` (`:accent2`) on a `ColorMap` element.
pub fn color_map_with_accent2(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent2", value);
    el
}

/// Set `Accent3` (`:accent3`) on a `ColorMap` element.
pub fn color_map_with_accent3(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent3", value);
    el
}

/// Set `Accent4` (`:accent4`) on a `ColorMap` element.
pub fn color_map_with_accent4(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent4", value);
    el
}

/// Set `Accent5` (`:accent5`) on a `ColorMap` element.
pub fn color_map_with_accent5(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent5", value);
    el
}

/// Set `Accent6` (`:accent6`) on a `ColorMap` element.
pub fn color_map_with_accent6(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("accent6", value);
    el
}

/// Set `Hyperlink` (`:hlink`) on a `ColorMap` element.
pub fn color_map_with_hyperlink(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hlink", value);
    el
}

/// Set `FollowedHyperlink` (`:folHlink`) on a `ColorMap` element.
pub fn color_map_with_followed_hyperlink(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("folHlink", value);
    el
}

/// Create a `<a:extraClrScheme>` element (`ExtraColorScheme`).
pub fn extra_color_scheme(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extraClrScheme").with_children(children)
}

/// Create a `<a:themeElements>` element (`ThemeElements`).
pub fn theme_elements(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "themeElements").with_children(children)
}

/// Create a `<a:cell3D>` element (`Cell3DProperties`).
pub fn cell3_d_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cell3D").with_children(children)
}

/// Set `PresetMaterial` (`:prstMaterial`) on a `Cell3DProperties` element.
pub fn cell3_d_properties_with_preset_material(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("prstMaterial", value);
    el
}

/// Create a `<a:tcPr>` element (`TableCellProperties`).
pub fn table_cell_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tcPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `TopMargin` (`:marT`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_top_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marT", value);
    el
}

/// Set `BottomMargin` (`:marB`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_bottom_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marB", value);
    el
}

/// Set `Vertical` (`:vert`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_vertical(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("vert", value);
    el
}

/// Set `Anchor` (`:anchor`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_anchor(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("anchor", value);
    el
}

/// Set `AnchorCenter` (`:anchorCtr`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_anchor_center(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("anchorCtr", value);
    el
}

/// Set `HorizontalOverflow` (`:horzOverflow`) on a `TableCellProperties` element.
pub fn table_cell_properties_with_horizontal_overflow(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("horzOverflow", value);
    el
}

/// Create a `<a:tc>` element (`TableCell`).
pub fn table_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tc").with_children(children)
}

/// Set `RowSpan` (`:rowSpan`) on a `TableCell` element.
pub fn table_cell_with_row_span(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rowSpan", value);
    el
}

/// Set `GridSpan` (`:gridSpan`) on a `TableCell` element.
pub fn table_cell_with_grid_span(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("gridSpan", value);
    el
}

/// Set `HorizontalMerge` (`:hMerge`) on a `TableCell` element.
pub fn table_cell_with_horizontal_merge(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hMerge", value);
    el
}

/// Set `VerticalMerge` (`:vMerge`) on a `TableCell` element.
pub fn table_cell_with_vertical_merge(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("vMerge", value);
    el
}

/// Create a `<a:tableStyle>` element (`TableStyle`).
pub fn table_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tableStyle").with_children(children)
}

/// Set `StyleId` (`:styleId`) on a `TableStyle` element.
pub fn table_style_with_style_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("styleId", value);
    el
}

/// Set `StyleName` (`:styleName`) on a `TableStyle` element.
pub fn table_style_with_style_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("styleName", value);
    el
}

/// Create a `<a:tblStyle>` element (`TableStyleEntry`).
pub fn table_style_entry(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tblStyle").with_children(children)
}

/// Set `StyleId` (`:styleId`) on a `TableStyleEntry` element.
pub fn table_style_entry_with_style_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("styleId", value);
    el
}

/// Set `StyleName` (`:styleName`) on a `TableStyleEntry` element.
pub fn table_style_entry_with_style_name(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("styleName", value);
    el
}

/// Create a `<a:tableStyleId>` element (`TableStyleId`).
pub fn table_style_id(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tableStyleId").with_text(value)
}

/// Create a `<a:gridCol>` element (`GridColumn`).
pub fn grid_column(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "gridCol").with_children(children)
}

/// Set `Width` (`:w`) on a `GridColumn` element.
pub fn grid_column_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Create a `<a:tblPr>` element (`TableProperties`).
pub fn table_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tblPr").with_children(children)
}

/// Set `RightToLeft` (`:rtl`) on a `TableProperties` element.
pub fn table_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `FirstRow` (`:firstRow`) on a `TableProperties` element.
pub fn table_properties_with_first_row(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("firstRow", value);
    el
}

/// Set `FirstColumn` (`:firstCol`) on a `TableProperties` element.
pub fn table_properties_with_first_column(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("firstCol", value);
    el
}

/// Set `LastRow` (`:lastRow`) on a `TableProperties` element.
pub fn table_properties_with_last_row(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lastRow", value);
    el
}

/// Set `LastColumn` (`:lastCol`) on a `TableProperties` element.
pub fn table_properties_with_last_column(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lastCol", value);
    el
}

/// Set `BandRow` (`:bandRow`) on a `TableProperties` element.
pub fn table_properties_with_band_row(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bandRow", value);
    el
}

/// Set `BandColumn` (`:bandCol`) on a `TableProperties` element.
pub fn table_properties_with_band_column(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bandCol", value);
    el
}

/// Create a `<a:tblGrid>` element (`TableGrid`).
pub fn table_grid(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tblGrid").with_children(children)
}

/// Create a `<a:tr>` element (`TableRow`).
pub fn table_row(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tr").with_children(children)
}

/// Set `Height` (`:h`) on a `TableRow` element.
pub fn table_row_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("h", value);
    el
}

/// Create a `<a:left>` element (`LeftBorder`).
pub fn left_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "left").with_children(children)
}

/// Create a `<a:right>` element (`RightBorder`).
pub fn right_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "right").with_children(children)
}

/// Create a `<a:top>` element (`TopBorder`).
pub fn top_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "top").with_children(children)
}

/// Create a `<a:bottom>` element (`BottomBorder`).
pub fn bottom_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "bottom").with_children(children)
}

/// Create a `<a:insideH>` element (`InsideHorizontalBorder`).
pub fn inside_horizontal_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "insideH").with_children(children)
}

/// Create a `<a:insideV>` element (`InsideVerticalBorder`).
pub fn inside_vertical_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "insideV").with_children(children)
}

/// Create a `<a:tl2br>` element (`TopLeftToBottomRightBorder`).
pub fn top_left_to_bottom_right_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tl2br").with_children(children)
}

/// Create a `<a:tr2bl>` element (`TopRightToBottomLeftBorder`).
pub fn top_right_to_bottom_left_border(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tr2bl").with_children(children)
}

/// Create a `<a:tcBdr>` element (`TableCellBorders`).
pub fn table_cell_borders(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tcBdr").with_children(children)
}

/// Create a `<a:tcTxStyle>` element (`TableCellTextStyle`).
pub fn table_cell_text_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tcTxStyle").with_children(children)
}

/// Set `Bold` (`:b`) on a `TableCellTextStyle` element.
pub fn table_cell_text_style_with_bold(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Set `Italic` (`:i`) on a `TableCellTextStyle` element.
pub fn table_cell_text_style_with_italic(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("i", value);
    el
}

/// Create a `<a:tcStyle>` element (`TableCellStyle`).
pub fn table_cell_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tcStyle").with_children(children)
}

/// Create a `<a:tblBg>` element (`TableBackground`).
pub fn table_background(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tblBg").with_children(children)
}

/// Create a `<a:wholeTbl>` element (`WholeTable`).
pub fn whole_table(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "wholeTbl").with_children(children)
}

/// Create a `<a:band1H>` element (`Band1Horizontal`).
pub fn band1_horizontal(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "band1H").with_children(children)
}

/// Create a `<a:band2H>` element (`Band2Horizontal`).
pub fn band2_horizontal(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "band2H").with_children(children)
}

/// Create a `<a:band1V>` element (`Band1Vertical`).
pub fn band1_vertical(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "band1V").with_children(children)
}

/// Create a `<a:band2V>` element (`Band2Vertical`).
pub fn band2_vertical(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "band2V").with_children(children)
}

/// Create a `<a:lastCol>` element (`LastColumn`).
pub fn last_column(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lastCol").with_children(children)
}

/// Create a `<a:firstCol>` element (`FirstColumn`).
pub fn first_column(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "firstCol").with_children(children)
}

/// Create a `<a:lastRow>` element (`LastRow`).
pub fn last_row(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lastRow").with_children(children)
}

/// Create a `<a:seCell>` element (`SoutheastCell`).
pub fn southeast_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "seCell").with_children(children)
}

/// Create a `<a:swCell>` element (`SouthwestCell`).
pub fn southwest_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "swCell").with_children(children)
}

/// Create a `<a:firstRow>` element (`FirstRow`).
pub fn first_row(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "firstRow").with_children(children)
}

/// Create a `<a:neCell>` element (`NortheastCell`).
pub fn northeast_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "neCell").with_children(children)
}

/// Create a `<a:nwCell>` element (`NorthwestCell`).
pub fn northwest_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "nwCell").with_children(children)
}

/// Create a `<a:pPr>` element (`ParagraphProperties`).
pub fn paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `ParagraphProperties` element.
pub fn paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:defPPr>` element (`DefaultParagraphProperties`).
pub fn default_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "defPPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `DefaultParagraphProperties` element.
pub fn default_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl1pPr>` element (`Level1ParagraphProperties`).
pub fn level1_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl1pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level1ParagraphProperties` element.
pub fn level1_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl2pPr>` element (`Level2ParagraphProperties`).
pub fn level2_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl2pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level2ParagraphProperties` element.
pub fn level2_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl3pPr>` element (`Level3ParagraphProperties`).
pub fn level3_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl3pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level3ParagraphProperties` element.
pub fn level3_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl4pPr>` element (`Level4ParagraphProperties`).
pub fn level4_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl4pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level4ParagraphProperties` element.
pub fn level4_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl5pPr>` element (`Level5ParagraphProperties`).
pub fn level5_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl5pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level5ParagraphProperties` element.
pub fn level5_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl6pPr>` element (`Level6ParagraphProperties`).
pub fn level6_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl6pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level6ParagraphProperties` element.
pub fn level6_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl7pPr>` element (`Level7ParagraphProperties`).
pub fn level7_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl7pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level7ParagraphProperties` element.
pub fn level7_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl8pPr>` element (`Level8ParagraphProperties`).
pub fn level8_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl8pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level8ParagraphProperties` element.
pub fn level8_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:lvl9pPr>` element (`Level9ParagraphProperties`).
pub fn level9_paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lvl9pPr").with_children(children)
}

/// Set `LeftMargin` (`:marL`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_left_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marL", value);
    el
}

/// Set `RightMargin` (`:marR`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_right_margin(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("marR", value);
    el
}

/// Set `Level` (`:lvl`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_level(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lvl", value);
    el
}

/// Set `Indent` (`:indent`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_indent(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("indent", value);
    el
}

/// Set `Alignment` (`:algn`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Set `DefaultTabSize` (`:defTabSz`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_default_tab_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("defTabSz", value);
    el
}

/// Set `RightToLeft` (`:rtl`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_right_to_left(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("rtl", value);
    el
}

/// Set `EastAsianLineBreak` (`:eaLnBrk`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_east_asian_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("eaLnBrk", value);
    el
}

/// Set `FontAlignment` (`:fontAlgn`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_font_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("fontAlgn", value);
    el
}

/// Set `LatinLineBreak` (`:latinLnBrk`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_latin_line_break(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("latinLnBrk", value);
    el
}

/// Set `Height` (`:hangingPunct`) on a `Level9ParagraphProperties` element.
pub fn level9_paragraph_properties_with_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("hangingPunct", value);
    el
}

/// Create a `<a:endParaRPr>` element (`EndParagraphRunProperties`).
pub fn end_paragraph_run_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "endParaRPr").with_children(children)
}

/// Set `Kumimoji` (`:kumimoji`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_kumimoji(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kumimoji", value);
    el
}

/// Set `Language` (`:lang`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_language(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lang", value);
    el
}

/// Set `AlternativeLanguage` (`:altLang`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_alternative_language(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("altLang", value);
    el
}

/// Set `FontSize` (`:sz`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_font_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sz", value);
    el
}

/// Set `Bold` (`:b`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_bold(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Set `Italic` (`:i`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_italic(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("i", value);
    el
}

/// Set `Underline` (`:u`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_underline(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("u", value);
    el
}

/// Set `Strike` (`:strike`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_strike(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("strike", value);
    el
}

/// Set `Kerning` (`:kern`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_kerning(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kern", value);
    el
}

/// Set `Capital` (`:cap`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_capital(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `Spacing` (`:spc`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_spacing(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spc", value);
    el
}

/// Set `NormalizeHeight` (`:normalizeH`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_normalize_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("normalizeH", value);
    el
}

/// Set `Baseline` (`:baseline`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_baseline(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("baseline", value);
    el
}

/// Set `NoProof` (`:noProof`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_no_proof(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noProof", value);
    el
}

/// Set `Dirty` (`:dirty`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_dirty(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dirty", value);
    el
}

/// Set `SpellingError` (`:err`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_spelling_error(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("err", value);
    el
}

/// Set `SmartTagClean` (`:smtClean`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_smart_tag_clean(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("smtClean", value);
    el
}

/// Set `SmartTagId` (`:smtId`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_smart_tag_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("smtId", value);
    el
}

/// Set `Bookmark` (`:bmk`) on a `EndParagraphRunProperties` element.
pub fn end_paragraph_run_properties_with_bookmark(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bmk", value);
    el
}

/// Create a `<a:rPr>` element (`RunProperties`).
pub fn run_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "rPr").with_children(children)
}

/// Set `Kumimoji` (`:kumimoji`) on a `RunProperties` element.
pub fn run_properties_with_kumimoji(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kumimoji", value);
    el
}

/// Set `Language` (`:lang`) on a `RunProperties` element.
pub fn run_properties_with_language(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lang", value);
    el
}

/// Set `AlternativeLanguage` (`:altLang`) on a `RunProperties` element.
pub fn run_properties_with_alternative_language(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("altLang", value);
    el
}

/// Set `FontSize` (`:sz`) on a `RunProperties` element.
pub fn run_properties_with_font_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sz", value);
    el
}

/// Set `Bold` (`:b`) on a `RunProperties` element.
pub fn run_properties_with_bold(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Set `Italic` (`:i`) on a `RunProperties` element.
pub fn run_properties_with_italic(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("i", value);
    el
}

/// Set `Underline` (`:u`) on a `RunProperties` element.
pub fn run_properties_with_underline(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("u", value);
    el
}

/// Set `Strike` (`:strike`) on a `RunProperties` element.
pub fn run_properties_with_strike(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("strike", value);
    el
}

/// Set `Kerning` (`:kern`) on a `RunProperties` element.
pub fn run_properties_with_kerning(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kern", value);
    el
}

/// Set `Capital` (`:cap`) on a `RunProperties` element.
pub fn run_properties_with_capital(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `Spacing` (`:spc`) on a `RunProperties` element.
pub fn run_properties_with_spacing(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spc", value);
    el
}

/// Set `NormalizeHeight` (`:normalizeH`) on a `RunProperties` element.
pub fn run_properties_with_normalize_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("normalizeH", value);
    el
}

/// Set `Baseline` (`:baseline`) on a `RunProperties` element.
pub fn run_properties_with_baseline(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("baseline", value);
    el
}

/// Set `NoProof` (`:noProof`) on a `RunProperties` element.
pub fn run_properties_with_no_proof(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noProof", value);
    el
}

/// Set `Dirty` (`:dirty`) on a `RunProperties` element.
pub fn run_properties_with_dirty(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dirty", value);
    el
}

/// Set `SpellingError` (`:err`) on a `RunProperties` element.
pub fn run_properties_with_spelling_error(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("err", value);
    el
}

/// Set `SmartTagClean` (`:smtClean`) on a `RunProperties` element.
pub fn run_properties_with_smart_tag_clean(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("smtClean", value);
    el
}

/// Set `SmartTagId` (`:smtId`) on a `RunProperties` element.
pub fn run_properties_with_smart_tag_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("smtId", value);
    el
}

/// Set `Bookmark` (`:bmk`) on a `RunProperties` element.
pub fn run_properties_with_bookmark(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bmk", value);
    el
}

/// Create a `<a:defRPr>` element (`DefaultRunProperties`).
pub fn default_run_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "defRPr").with_children(children)
}

/// Set `Kumimoji` (`:kumimoji`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_kumimoji(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kumimoji", value);
    el
}

/// Set `Language` (`:lang`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_language(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("lang", value);
    el
}

/// Set `AlternativeLanguage` (`:altLang`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_alternative_language(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("altLang", value);
    el
}

/// Set `FontSize` (`:sz`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_font_size(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("sz", value);
    el
}

/// Set `Bold` (`:b`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_bold(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("b", value);
    el
}

/// Set `Italic` (`:i`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_italic(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("i", value);
    el
}

/// Set `Underline` (`:u`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_underline(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("u", value);
    el
}

/// Set `Strike` (`:strike`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_strike(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("strike", value);
    el
}

/// Set `Kerning` (`:kern`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_kerning(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("kern", value);
    el
}

/// Set `Capital` (`:cap`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_capital(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("cap", value);
    el
}

/// Set `Spacing` (`:spc`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_spacing(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("spc", value);
    el
}

/// Set `NormalizeHeight` (`:normalizeH`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_normalize_height(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("normalizeH", value);
    el
}

/// Set `Baseline` (`:baseline`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_baseline(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("baseline", value);
    el
}

/// Set `NoProof` (`:noProof`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_no_proof(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noProof", value);
    el
}

/// Set `Dirty` (`:dirty`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_dirty(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("dirty", value);
    el
}

/// Set `SpellingError` (`:err`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_spelling_error(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("err", value);
    el
}

/// Set `SmartTagClean` (`:smtClean`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_smart_tag_clean(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("smtClean", value);
    el
}

/// Set `SmartTagId` (`:smtId`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_smart_tag_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("smtId", value);
    el
}

/// Set `Bookmark` (`:bmk`) on a `DefaultRunProperties` element.
pub fn default_run_properties_with_bookmark(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bmk", value);
    el
}

/// Create a `<a:p>` element (`Paragraph`).
pub fn paragraph(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "p").with_children(children)
}

/// Create a `<a:tab>` element (`TabStop`).
pub fn tab_stop() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tab")
}

/// Set `Position` (`:pos`) on a `TabStop` element.
pub fn tab_stop_with_position(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("pos", value);
    el
}

/// Set `Alignment` (`:algn`) on a `TabStop` element.
pub fn tab_stop_with_alignment(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("algn", value);
    el
}

/// Create a `<a:spcPct>` element (`SpacingPercent`).
pub fn spacing_percent() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spcPct")
}

/// Set `Val` (`:val`) on a `SpacingPercent` element.
pub fn spacing_percent_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:spcPct>` with `Val` set.
pub fn spacing_percent_val(value: impl Into<String>) -> OpenXmlElement {
    spacing_percent_with_val(spacing_percent(), value)
}

/// Create a `<a:spcPts>` element (`SpacingPoints`).
pub fn spacing_points() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spcPts")
}

/// Set `Val` (`:val`) on a `SpacingPoints` element.
pub fn spacing_points_with_val(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("val", value);
    el
}

/// Create `<a:spcPts>` with `Val` set.
pub fn spacing_points_val(value: impl Into<String>) -> OpenXmlElement {
    spacing_points_with_val(spacing_points(), value)
}

/// Create a `<a:lnSpc>` element (`LineSpacing`).
pub fn line_spacing(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "lnSpc").with_children(children)
}

/// Create a `<a:spcBef>` element (`SpaceBefore`).
pub fn space_before(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spcBef").with_children(children)
}

/// Create a `<a:spcAft>` element (`SpaceAfter`).
pub fn space_after(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "spcAft").with_children(children)
}

/// Create a `<a:tabLst>` element (`TabStopList`).
pub fn tab_stop_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tabLst").with_children(children)
}

/// Create a `<a:t>` element (`Text`).
pub fn text(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "t").with_text(value)
}

/// Create a `<a:ext>` element (`ShapePropertiesExtension`).
pub fn shape_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:ext>` element (`GvmlGroupShapeExtension`).
pub fn gvml_group_shape_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:extLst>` element (`ShapePropertiesExtensionList`).
pub fn shape_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:nvGrpSpPr>` element (`NonVisualGroupShapeProperties`).
pub fn non_visual_group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "nvGrpSpPr").with_children(children)
}

/// Create a `<a:grpSpPr>` element (`VisualGroupShapeProperties`).
pub fn visual_group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Set `BlackWhiteMode` (`:bwMode`) on a `VisualGroupShapeProperties` element.
pub fn visual_group_shape_properties_with_black_white_mode(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("bwMode", value);
    el
}

/// Create a `<a:sp>` element (`Shape`).
pub fn shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "sp").with_children(children)
}

/// Create a `<a:cxnSp>` element (`ConnectionShape`).
pub fn connection_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "cxnSp").with_children(children)
}

/// Create a `<a:pic>` element (`Picture`).
pub fn picture(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "pic").with_children(children)
}

/// Create a `<a:graphicFrame>` element (`GraphicFrame`).
pub fn graphic_frame(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "graphicFrame").with_children(children)
}

/// Create a `<a:grpSp>` element (`GroupShape`).
pub fn group_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "grpSp").with_children(children)
}

/// Create a `<a:extLst>` element (`GvmlGroupShapeExtensionList`).
pub fn gvml_group_shape_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:ext>` element (`NonVisualGroupDrawingShapePropsExtension`).
pub fn non_visual_group_drawing_shape_props_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:ext>` element (`OfficeStyleSheetExtension`).
pub fn office_style_sheet_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:ext>` element (`ConnectorLockingExtension`).
pub fn connector_locking_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:grpSpLocks>` element (`GroupShapeLocks`).
pub fn group_shape_locks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "grpSpLocks").with_children(children)
}

/// Set `NoGrouping` (`:noGrp`) on a `GroupShapeLocks` element.
pub fn group_shape_locks_with_no_grouping(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noGrp", value);
    el
}

/// Set `NoUngrouping` (`:noUngrp`) on a `GroupShapeLocks` element.
pub fn group_shape_locks_with_no_ungrouping(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noUngrp", value);
    el
}

/// Set `NoSelection` (`:noSelect`) on a `GroupShapeLocks` element.
pub fn group_shape_locks_with_no_selection(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noSelect", value);
    el
}

/// Set `NoRotation` (`:noRot`) on a `GroupShapeLocks` element.
pub fn group_shape_locks_with_no_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noRot", value);
    el
}

/// Set `NoChangeAspect` (`:noChangeAspect`) on a `GroupShapeLocks` element.
pub fn group_shape_locks_with_no_change_aspect(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeAspect", value);
    el
}

/// Set `NoMove` (`:noMove`) on a `GroupShapeLocks` element.
pub fn group_shape_locks_with_no_move(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noMove", value);
    el
}

/// Set `NoResize` (`:noResize`) on a `GroupShapeLocks` element.
pub fn group_shape_locks_with_no_resize(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noResize", value);
    el
}

/// Create a `<a:extLst>` element (`NonVisualGroupDrawingShapePropsExtensionList`).
pub fn non_visual_group_drawing_shape_props_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:objectDefaults>` element (`ObjectDefaults`).
pub fn object_defaults(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "objectDefaults").with_children(children)
}

/// Create a `<a:extraClrSchemeLst>` element (`ExtraColorSchemeList`).
pub fn extra_color_scheme_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extraClrSchemeLst").with_children(children)
}

/// Create a `<a:custClrLst>` element (`CustomColorList`).
pub fn custom_color_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "custClrLst").with_children(children)
}

/// Create a `<a:extLst>` element (`OfficeStyleSheetExtensionList`).
pub fn office_style_sheet_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:hlinkClick>` element (`HyperlinkOnClick`).
pub fn hyperlink_on_click(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hlinkClick").with_children(children)
}

/// Set `Id` (`r:id`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Set `InvalidUrl` (`:invalidUrl`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_invalid_url(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("invalidUrl", value);
    el
}

/// Set `Action` (`:action`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_action(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("action", value);
    el
}

/// Set `TargetFrame` (`:tgtFrame`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_target_frame(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tgtFrame", value);
    el
}

/// Set `Tooltip` (`:tooltip`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_tooltip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tooltip", value);
    el
}

/// Set `History` (`:history`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_history(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("history", value);
    el
}

/// Set `HighlightClick` (`:highlightClick`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_highlight_click(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("highlightClick", value);
    el
}

/// Set `EndSound` (`:endSnd`) on a `HyperlinkOnClick` element.
pub fn hyperlink_on_click_with_end_sound(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("endSnd", value);
    el
}

/// Create a `<a:hlinkMouseOver>` element (`HyperlinkOnMouseOver`).
pub fn hyperlink_on_mouse_over(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hlinkMouseOver").with_children(children)
}

/// Set `Id` (`r:id`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Set `InvalidUrl` (`:invalidUrl`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_invalid_url(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("invalidUrl", value);
    el
}

/// Set `Action` (`:action`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_action(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("action", value);
    el
}

/// Set `TargetFrame` (`:tgtFrame`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_target_frame(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tgtFrame", value);
    el
}

/// Set `Tooltip` (`:tooltip`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_tooltip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tooltip", value);
    el
}

/// Set `History` (`:history`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_history(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("history", value);
    el
}

/// Set `HighlightClick` (`:highlightClick`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_highlight_click(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("highlightClick", value);
    el
}

/// Set `EndSound` (`:endSnd`) on a `HyperlinkOnMouseOver` element.
pub fn hyperlink_on_mouse_over_with_end_sound(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("endSnd", value);
    el
}

/// Create a `<a:hlinkHover>` element (`HyperlinkOnHover`).
pub fn hyperlink_on_hover(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "hlinkHover").with_children(children)
}

/// Set `Id` (`r:id`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_id(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("r:id", value);
    el
}

/// Set `InvalidUrl` (`:invalidUrl`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_invalid_url(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("invalidUrl", value);
    el
}

/// Set `Action` (`:action`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_action(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("action", value);
    el
}

/// Set `TargetFrame` (`:tgtFrame`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_target_frame(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tgtFrame", value);
    el
}

/// Set `Tooltip` (`:tooltip`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_tooltip(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("tooltip", value);
    el
}

/// Set `History` (`:history`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_history(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("history", value);
    el
}

/// Set `HighlightClick` (`:highlightClick`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_highlight_click(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("highlightClick", value);
    el
}

/// Set `EndSound` (`:endSnd`) on a `HyperlinkOnHover` element.
pub fn hyperlink_on_hover_with_end_sound(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("endSnd", value);
    el
}

/// Create a `<a:rtl>` element (`RightToLeft`).
pub fn right_to_left() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "rtl")
}

/// Create a `<a:extLst>` element (`NonVisualDrawingPropertiesExtensionList`).
pub fn non_visual_drawing_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:extLst>` element (`ConnectorLockingExtensionList`).
pub fn connector_locking_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:ext>` element (`DataModelExtension`).
pub fn data_model_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:ext>` element (`PtExtension`).
pub fn pt_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:ext>` element (`HyperlinkExtension`).
pub fn hyperlink_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:extLst>` element (`HyperlinkExtensionList`).
pub fn hyperlink_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:ext>` element (`LinePropertiesExtension`).
pub fn line_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:headEnd>` element (`HeadEnd`).
pub fn head_end() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "headEnd")
}

/// Set `Type` (`:type`) on a `HeadEnd` element.
pub fn head_end_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Width` (`:w`) on a `HeadEnd` element.
pub fn head_end_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `Length` (`:len`) on a `HeadEnd` element.
pub fn head_end_with_length(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("len", value);
    el
}

/// Create a `<a:tailEnd>` element (`TailEnd`).
pub fn tail_end() -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "tailEnd")
}

/// Set `Type` (`:type`) on a `TailEnd` element.
pub fn tail_end_with_type_(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("type", value);
    el
}

/// Set `Width` (`:w`) on a `TailEnd` element.
pub fn tail_end_with_width(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("w", value);
    el
}

/// Set `Length` (`:len`) on a `TailEnd` element.
pub fn tail_end_with_length(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("len", value);
    el
}

/// Create a `<a:extLst>` element (`LinePropertiesExtensionList`).
pub fn line_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:ext>` element (`NonVisualDrawingPropertiesExtension`).
pub fn non_visual_drawing_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:picLocks>` element (`PictureLocks`).
pub fn picture_locks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "picLocks").with_children(children)
}

/// Set `NoGrouping` (`:noGrp`) on a `PictureLocks` element.
pub fn picture_locks_with_no_grouping(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noGrp", value);
    el
}

/// Set `NoSelection` (`:noSelect`) on a `PictureLocks` element.
pub fn picture_locks_with_no_selection(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noSelect", value);
    el
}

/// Set `NoRotation` (`:noRot`) on a `PictureLocks` element.
pub fn picture_locks_with_no_rotation(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noRot", value);
    el
}

/// Set `NoChangeAspect` (`:noChangeAspect`) on a `PictureLocks` element.
pub fn picture_locks_with_no_change_aspect(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeAspect", value);
    el
}

/// Set `NoMove` (`:noMove`) on a `PictureLocks` element.
pub fn picture_locks_with_no_move(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noMove", value);
    el
}

/// Set `NoResize` (`:noResize`) on a `PictureLocks` element.
pub fn picture_locks_with_no_resize(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noResize", value);
    el
}

/// Set `NoEditPoints` (`:noEditPoints`) on a `PictureLocks` element.
pub fn picture_locks_with_no_edit_points(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noEditPoints", value);
    el
}

/// Set `NoAdjustHandles` (`:noAdjustHandles`) on a `PictureLocks` element.
pub fn picture_locks_with_no_adjust_handles(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noAdjustHandles", value);
    el
}

/// Set `NoChangeArrowheads` (`:noChangeArrowheads`) on a `PictureLocks` element.
pub fn picture_locks_with_no_change_arrowheads(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeArrowheads", value);
    el
}

/// Set `NoChangeShapeType` (`:noChangeShapeType`) on a `PictureLocks` element.
pub fn picture_locks_with_no_change_shape_type(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noChangeShapeType", value);
    el
}

/// Set `NoCrop` (`:noCrop`) on a `PictureLocks` element.
pub fn picture_locks_with_no_crop(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {
    el.set_attribute_qname("noCrop", value);
    el
}

/// Create a `<a:extLst>` element (`NonVisualPicturePropertiesExtensionList`).
pub fn non_visual_picture_properties_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:ext>` element (`NonVisualPicturePropertiesExtension`).
pub fn non_visual_picture_properties_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<a:extLst>` element (`BlipExtensionList`).
pub fn blip_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a:ext>` element (`BlipExtension`).
pub fn blip_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a", NAMESPACE_URI, "ext").with_children(children)
}

// ---------------------------------------------------------------------------
// Schema particles (content models)
// ---------------------------------------------------------------------------

use crate::validation::{Occurs, Particle};

/// Content model particle for `AudioFromCD`.
pub fn particle_audio_from_c_d() -> Particle {
    Particle::sequence(vec![
      Particle::element("st", Occurs::STAR),
      Particle::element("end", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `AudioFromFile`.
pub fn particle_audio_from_file() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `VideoFromFile`.
pub fn particle_video_from_file() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `QuickTimeFromFile`.
pub fn particle_quick_time_from_file() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Extension`.
pub fn particle_extension() -> Particle {
    Particle::sequence(vec![
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `RgbColorModelPercentage`.
pub fn particle_rgb_color_model_percentage() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tint", Occurs::STAR),
              Particle::element("shade", Occurs::STAR),
              Particle::element("comp", Occurs::STAR),
              Particle::element("inv", Occurs::STAR),
              Particle::element("gray", Occurs::STAR),
              Particle::element("alpha", Occurs::STAR),
              Particle::element("alphaOff", Occurs::STAR),
              Particle::element("alphaMod", Occurs::STAR),
              Particle::element("hue", Occurs::STAR),
              Particle::element("hueOff", Occurs::STAR),
              Particle::element("hueMod", Occurs::STAR),
              Particle::element("sat", Occurs::STAR),
              Particle::element("satOff", Occurs::STAR),
              Particle::element("satMod", Occurs::STAR),
              Particle::element("lum", Occurs::STAR),
              Particle::element("lumOff", Occurs::STAR),
              Particle::element("lumMod", Occurs::STAR),
              Particle::element("red", Occurs::STAR),
              Particle::element("redOff", Occurs::STAR),
              Particle::element("redMod", Occurs::STAR),
              Particle::element("green", Occurs::STAR),
              Particle::element("greenOff", Occurs::STAR),
              Particle::element("greenMod", Occurs::STAR),
              Particle::element("blue", Occurs::STAR),
              Particle::element("blueOff", Occurs::STAR),
              Particle::element("blueMod", Occurs::STAR),
              Particle::element("gamma", Occurs::STAR),
              Particle::element("invGamma", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `RgbColorModelHex`.
pub fn particle_rgb_color_model_hex() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tint", Occurs::STAR),
              Particle::element("shade", Occurs::STAR),
              Particle::element("comp", Occurs::STAR),
              Particle::element("inv", Occurs::STAR),
              Particle::element("gray", Occurs::STAR),
              Particle::element("alpha", Occurs::STAR),
              Particle::element("alphaOff", Occurs::STAR),
              Particle::element("alphaMod", Occurs::STAR),
              Particle::element("hue", Occurs::STAR),
              Particle::element("hueOff", Occurs::STAR),
              Particle::element("hueMod", Occurs::STAR),
              Particle::element("sat", Occurs::STAR),
              Particle::element("satOff", Occurs::STAR),
              Particle::element("satMod", Occurs::STAR),
              Particle::element("lum", Occurs::STAR),
              Particle::element("lumOff", Occurs::STAR),
              Particle::element("lumMod", Occurs::STAR),
              Particle::element("red", Occurs::STAR),
              Particle::element("redOff", Occurs::STAR),
              Particle::element("redMod", Occurs::STAR),
              Particle::element("green", Occurs::STAR),
              Particle::element("greenOff", Occurs::STAR),
              Particle::element("greenMod", Occurs::STAR),
              Particle::element("blue", Occurs::STAR),
              Particle::element("blueOff", Occurs::STAR),
              Particle::element("blueMod", Occurs::STAR),
              Particle::element("gamma", Occurs::STAR),
              Particle::element("invGamma", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `HslColor`.
pub fn particle_hsl_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tint", Occurs::STAR),
              Particle::element("shade", Occurs::STAR),
              Particle::element("comp", Occurs::STAR),
              Particle::element("inv", Occurs::STAR),
              Particle::element("gray", Occurs::STAR),
              Particle::element("alpha", Occurs::STAR),
              Particle::element("alphaOff", Occurs::STAR),
              Particle::element("alphaMod", Occurs::STAR),
              Particle::element("hue", Occurs::STAR),
              Particle::element("hueOff", Occurs::STAR),
              Particle::element("hueMod", Occurs::STAR),
              Particle::element("sat", Occurs::STAR),
              Particle::element("satOff", Occurs::STAR),
              Particle::element("satMod", Occurs::STAR),
              Particle::element("lum", Occurs::STAR),
              Particle::element("lumOff", Occurs::STAR),
              Particle::element("lumMod", Occurs::STAR),
              Particle::element("red", Occurs::STAR),
              Particle::element("redOff", Occurs::STAR),
              Particle::element("redMod", Occurs::STAR),
              Particle::element("green", Occurs::STAR),
              Particle::element("greenOff", Occurs::STAR),
              Particle::element("greenMod", Occurs::STAR),
              Particle::element("blue", Occurs::STAR),
              Particle::element("blueOff", Occurs::STAR),
              Particle::element("blueMod", Occurs::STAR),
              Particle::element("gamma", Occurs::STAR),
              Particle::element("invGamma", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SystemColor`.
pub fn particle_system_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tint", Occurs::STAR),
              Particle::element("shade", Occurs::STAR),
              Particle::element("comp", Occurs::STAR),
              Particle::element("inv", Occurs::STAR),
              Particle::element("gray", Occurs::STAR),
              Particle::element("alpha", Occurs::STAR),
              Particle::element("alphaOff", Occurs::STAR),
              Particle::element("alphaMod", Occurs::STAR),
              Particle::element("hue", Occurs::STAR),
              Particle::element("hueOff", Occurs::STAR),
              Particle::element("hueMod", Occurs::STAR),
              Particle::element("sat", Occurs::STAR),
              Particle::element("satOff", Occurs::STAR),
              Particle::element("satMod", Occurs::STAR),
              Particle::element("lum", Occurs::STAR),
              Particle::element("lumOff", Occurs::STAR),
              Particle::element("lumMod", Occurs::STAR),
              Particle::element("red", Occurs::STAR),
              Particle::element("redOff", Occurs::STAR),
              Particle::element("redMod", Occurs::STAR),
              Particle::element("green", Occurs::STAR),
              Particle::element("greenOff", Occurs::STAR),
              Particle::element("greenMod", Occurs::STAR),
              Particle::element("blue", Occurs::STAR),
              Particle::element("blueOff", Occurs::STAR),
              Particle::element("blueMod", Occurs::STAR),
              Particle::element("gamma", Occurs::STAR),
              Particle::element("invGamma", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SchemeColor`.
pub fn particle_scheme_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tint", Occurs::STAR),
              Particle::element("shade", Occurs::STAR),
              Particle::element("comp", Occurs::STAR),
              Particle::element("inv", Occurs::STAR),
              Particle::element("gray", Occurs::STAR),
              Particle::element("alpha", Occurs::STAR),
              Particle::element("alphaOff", Occurs::STAR),
              Particle::element("alphaMod", Occurs::STAR),
              Particle::element("hue", Occurs::STAR),
              Particle::element("hueOff", Occurs::STAR),
              Particle::element("hueMod", Occurs::STAR),
              Particle::element("sat", Occurs::STAR),
              Particle::element("satOff", Occurs::STAR),
              Particle::element("satMod", Occurs::STAR),
              Particle::element("lum", Occurs::STAR),
              Particle::element("lumOff", Occurs::STAR),
              Particle::element("lumMod", Occurs::STAR),
              Particle::element("red", Occurs::STAR),
              Particle::element("redOff", Occurs::STAR),
              Particle::element("redMod", Occurs::STAR),
              Particle::element("green", Occurs::STAR),
              Particle::element("greenOff", Occurs::STAR),
              Particle::element("greenMod", Occurs::STAR),
              Particle::element("blue", Occurs::STAR),
              Particle::element("blueOff", Occurs::STAR),
              Particle::element("blueMod", Occurs::STAR),
              Particle::element("gamma", Occurs::STAR),
              Particle::element("invGamma", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PresetColor`.
pub fn particle_preset_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tint", Occurs::STAR),
              Particle::element("shade", Occurs::STAR),
              Particle::element("comp", Occurs::STAR),
              Particle::element("inv", Occurs::STAR),
              Particle::element("gray", Occurs::STAR),
              Particle::element("alpha", Occurs::STAR),
              Particle::element("alphaOff", Occurs::STAR),
              Particle::element("alphaMod", Occurs::STAR),
              Particle::element("hue", Occurs::STAR),
              Particle::element("hueOff", Occurs::STAR),
              Particle::element("hueMod", Occurs::STAR),
              Particle::element("sat", Occurs::STAR),
              Particle::element("satOff", Occurs::STAR),
              Particle::element("satMod", Occurs::STAR),
              Particle::element("lum", Occurs::STAR),
              Particle::element("lumOff", Occurs::STAR),
              Particle::element("lumMod", Occurs::STAR),
              Particle::element("red", Occurs::STAR),
              Particle::element("redOff", Occurs::STAR),
              Particle::element("redMod", Occurs::STAR),
              Particle::element("green", Occurs::STAR),
              Particle::element("greenOff", Occurs::STAR),
              Particle::element("greenMod", Occurs::STAR),
              Particle::element("blue", Occurs::STAR),
              Particle::element("blueOff", Occurs::STAR),
              Particle::element("blueMod", Occurs::STAR),
              Particle::element("gamma", Occurs::STAR),
              Particle::element("invGamma", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Shape3DType`.
pub fn particle_shape3_d_type() -> Particle {
    Particle::sequence(vec![
      Particle::element("bevelT", Occurs::OPTIONAL),
      Particle::element("bevelB", Occurs::OPTIONAL),
      Particle::element("extrusionClr", Occurs::OPTIONAL),
      Particle::element("contourClr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PathGradientFill`.
pub fn particle_path_gradient_fill() -> Particle {
    Particle::sequence(vec![
      Particle::element("fillToRect", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Stretch`.
pub fn particle_stretch() -> Particle {
    Particle::sequence(vec![
      Particle::element("fillRect", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SolidFill`.
pub fn particle_solid_fill() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GradientFill`.
pub fn particle_gradient_fill() -> Particle {
    Particle::sequence(vec![
      Particle::element("gsLst", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("lin", Occurs::STAR),
              Particle::element("path", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tileRect", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BlipFill`.
pub fn particle_blip_fill() -> Particle {
    Particle::sequence(vec![
      Particle::element("blip", Occurs::OPTIONAL),
      Particle::element("srcRect", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("tile", Occurs::STAR),
              Particle::element("stretch", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PatternFill`.
pub fn particle_pattern_fill() -> Particle {
    Particle::sequence(vec![
      Particle::element("fgClr", Occurs::OPTIONAL),
      Particle::element("bgClr", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `EffectContainer`.
pub fn particle_effect_container() -> Particle {
    Particle::group(vec![
      Particle::choice(vec![
          Particle::element("cont", Occurs::STAR),
          Particle::element("effect", Occurs::STAR),
          Particle::element("alphaBiLevel", Occurs::STAR),
          Particle::element("alphaCeiling", Occurs::STAR),
          Particle::element("alphaFloor", Occurs::STAR),
          Particle::element("alphaInv", Occurs::STAR),
          Particle::element("alphaMod", Occurs::STAR),
          Particle::element("alphaModFix", Occurs::STAR),
          Particle::element("alphaOutset", Occurs::STAR),
          Particle::element("alphaRepl", Occurs::STAR),
          Particle::element("biLevel", Occurs::STAR),
          Particle::element("blend", Occurs::STAR),
          Particle::element("blur", Occurs::STAR),
          Particle::element("clrChange", Occurs::STAR),
          Particle::element("clrRepl", Occurs::STAR),
          Particle::element("duotone", Occurs::STAR),
          Particle::element("fill", Occurs::STAR),
          Particle::element("fillOverlay", Occurs::STAR),
          Particle::element("glow", Occurs::STAR),
          Particle::element("grayscl", Occurs::STAR),
          Particle::element("hsl", Occurs::STAR),
          Particle::element("innerShdw", Occurs::STAR),
          Particle::element("lum", Occurs::STAR),
          Particle::element("outerShdw", Occurs::STAR),
          Particle::element("prstShdw", Occurs::STAR),
          Particle::element("reflection", Occurs::STAR),
          Particle::element("relOff", Occurs::STAR),
          Particle::element("softEdge", Occurs::STAR),
          Particle::element("tint", Occurs::STAR),
          Particle::element("xfrm", Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `EffectDag`.
pub fn particle_effect_dag() -> Particle {
    Particle::group(vec![
      Particle::choice(vec![
          Particle::element("cont", Occurs::STAR),
          Particle::element("effect", Occurs::STAR),
          Particle::element("alphaBiLevel", Occurs::STAR),
          Particle::element("alphaCeiling", Occurs::STAR),
          Particle::element("alphaFloor", Occurs::STAR),
          Particle::element("alphaInv", Occurs::STAR),
          Particle::element("alphaMod", Occurs::STAR),
          Particle::element("alphaModFix", Occurs::STAR),
          Particle::element("alphaOutset", Occurs::STAR),
          Particle::element("alphaRepl", Occurs::STAR),
          Particle::element("biLevel", Occurs::STAR),
          Particle::element("blend", Occurs::STAR),
          Particle::element("blur", Occurs::STAR),
          Particle::element("clrChange", Occurs::STAR),
          Particle::element("clrRepl", Occurs::STAR),
          Particle::element("duotone", Occurs::STAR),
          Particle::element("fill", Occurs::STAR),
          Particle::element("fillOverlay", Occurs::STAR),
          Particle::element("glow", Occurs::STAR),
          Particle::element("grayscl", Occurs::STAR),
          Particle::element("hsl", Occurs::STAR),
          Particle::element("innerShdw", Occurs::STAR),
          Particle::element("lum", Occurs::STAR),
          Particle::element("outerShdw", Occurs::STAR),
          Particle::element("prstShdw", Occurs::STAR),
          Particle::element("reflection", Occurs::STAR),
          Particle::element("relOff", Occurs::STAR),
          Particle::element("softEdge", Occurs::STAR),
          Particle::element("tint", Occurs::STAR),
          Particle::element("xfrm", Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `AlphaInverse`.
pub fn particle_alpha_inverse() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `AlphaModulationEffect`.
pub fn particle_alpha_modulation_effect() -> Particle {
    Particle::sequence(vec![
      Particle::element("cont", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Blend`.
pub fn particle_blend() -> Particle {
    Particle::sequence(vec![
      Particle::element("cont", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ColorChange`.
pub fn particle_color_change() -> Particle {
    Particle::sequence(vec![
      Particle::element("clrFrom", Occurs::STAR),
      Particle::element("clrTo", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ColorReplacement`.
pub fn particle_color_replacement() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Duotone`.
pub fn particle_duotone() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::new(2, Some(2))),
  ], Occurs::STAR)
}

/// Content model particle for `Fill`.
pub fn particle_fill() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `FillOverlay`.
pub fn particle_fill_overlay() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Glow`.
pub fn particle_glow() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `InnerShadow`.
pub fn particle_inner_shadow() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `OuterShadow`.
pub fn particle_outer_shadow() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PresetShadow`.
pub fn particle_preset_shadow() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `EffectList`.
pub fn particle_effect_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("blur", Occurs::OPTIONAL),
      Particle::element("fillOverlay", Occurs::OPTIONAL),
      Particle::element("glow", Occurs::OPTIONAL),
      Particle::element("innerShdw", Occurs::OPTIONAL),
      Particle::element("outerShdw", Occurs::OPTIONAL),
      Particle::element("prstShdw", Occurs::OPTIONAL),
      Particle::element("reflection", Occurs::OPTIONAL),
      Particle::element("softEdge", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CustomGeometry`.
pub fn particle_custom_geometry() -> Particle {
    Particle::sequence(vec![
      Particle::element("avLst", Occurs::OPTIONAL),
      Particle::element("gdLst", Occurs::OPTIONAL),
      Particle::element("ahLst", Occurs::OPTIONAL),
      Particle::element("cxnLst", Occurs::OPTIONAL),
      Particle::element("rect", Occurs::OPTIONAL),
      Particle::element("pathLst", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PresetGeometry`.
pub fn particle_preset_geometry() -> Particle {
    Particle::sequence(vec![
      Particle::element("avLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PresetTextWarp`.
pub fn particle_preset_text_warp() -> Particle {
    Particle::sequence(vec![
      Particle::element("avLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `CustomDash`.
pub fn particle_custom_dash() -> Particle {
    Particle::sequence(vec![
      Particle::element("ds", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `FillProperties`.
pub fn particle_fill_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `FillReference`.
pub fn particle_fill_reference() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `EffectReference`.
pub fn particle_effect_reference() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LineReference`.
pub fn particle_line_reference() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `EffectPropertiesType`.
pub fn particle_effect_properties_type() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Fonts`.
pub fn particle_fonts() -> Particle {
    Particle::sequence(vec![
      Particle::element("latin", Occurs::STAR),
      Particle::element("ea", Occurs::STAR),
      Particle::element("cs", Occurs::STAR),
      Particle::element("font", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `MajorFont`.
pub fn particle_major_font() -> Particle {
    Particle::sequence(vec![
      Particle::element("latin", Occurs::STAR),
      Particle::element("ea", Occurs::STAR),
      Particle::element("cs", Occurs::STAR),
      Particle::element("font", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `MinorFont`.
pub fn particle_minor_font() -> Particle {
    Particle::sequence(vec![
      Particle::element("latin", Occurs::STAR),
      Particle::element("ea", Occurs::STAR),
      Particle::element("cs", Occurs::STAR),
      Particle::element("font", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `FontReference`.
pub fn particle_font_reference() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BulletColor`.
pub fn particle_bullet_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ExtrusionColor`.
pub fn particle_extrusion_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ContourColor`.
pub fn particle_contour_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ColorFrom`.
pub fn particle_color_from() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ColorTo`.
pub fn particle_color_to() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ForegroundColor`.
pub fn particle_foreground_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `BackgroundColor`.
pub fn particle_background_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Highlight`.
pub fn particle_highlight() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PictureBullet`.
pub fn particle_picture_bullet() -> Particle {
    Particle::sequence(vec![
      Particle::element("blip", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Underline`.
pub fn particle_underline() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Outline`.
pub fn particle_outline() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LeftBorderLineProperties`.
pub fn particle_left_border_line_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `RightBorderLineProperties`.
pub fn particle_right_border_line_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TopBorderLineProperties`.
pub fn particle_top_border_line_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BottomBorderLineProperties`.
pub fn particle_bottom_border_line_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TopLeftToBottomRightBorderLineProperties`.
pub fn particle_top_left_to_bottom_right_border_line_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BottomLeftToTopRightBorderLineProperties`.
pub fn particle_bottom_left_to_top_right_border_line_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("prstDash", Occurs::STAR),
              Particle::element("custDash", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("round", Occurs::STAR),
              Particle::element("bevel", Occurs::STAR),
              Particle::element("miter", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("headEnd", Occurs::OPTIONAL),
      Particle::element("tailEnd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `UnderlineFill`.
pub fn particle_underline_fill() -> Particle {
    Particle::group(vec![
      Particle::choice(vec![
          Particle::element("noFill", Occurs::STAR),
          Particle::element("solidFill", Occurs::STAR),
          Particle::element("gradFill", Occurs::STAR),
          Particle::element("blipFill", Occurs::STAR),
          Particle::element("pattFill", Occurs::STAR),
          Particle::element("grpFill", Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Run`.
pub fn particle_run() -> Particle {
    Particle::sequence(vec![
      Particle::element("rPr", Occurs::OPTIONAL),
      Particle::element("t", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Break`.
pub fn particle_break_() -> Particle {
    Particle::sequence(vec![
      Particle::element("rPr", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Field`.
pub fn particle_field() -> Particle {
    Particle::sequence(vec![
      Particle::element("rPr", Occurs::OPTIONAL),
      Particle::element("pPr", Occurs::OPTIONAL),
      Particle::element("t", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Graphic`.
pub fn particle_graphic() -> Particle {
    Particle::sequence(vec![
      Particle::element("graphicData", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Blip`.
pub fn particle_blip() -> Particle {
    Particle::sequence(vec![
      Particle::choice(vec![
          Particle::element("alphaBiLevel", Occurs::STAR),
          Particle::element("alphaCeiling", Occurs::STAR),
          Particle::element("alphaFloor", Occurs::STAR),
          Particle::element("alphaInv", Occurs::STAR),
          Particle::element("alphaMod", Occurs::STAR),
          Particle::element("alphaModFix", Occurs::STAR),
          Particle::element("alphaRepl", Occurs::STAR),
          Particle::element("biLevel", Occurs::STAR),
          Particle::element("blur", Occurs::STAR),
          Particle::element("clrChange", Occurs::STAR),
          Particle::element("clrRepl", Occurs::STAR),
          Particle::element("duotone", Occurs::STAR),
          Particle::element("fillOverlay", Occurs::STAR),
          Particle::element("grayscl", Occurs::STAR),
          Particle::element("hsl", Occurs::STAR),
          Particle::element("lum", Occurs::STAR),
          Particle::element("tint", Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Theme`.
pub fn particle_theme() -> Particle {
    Particle::sequence(vec![
      Particle::element("themeElements", Occurs::STAR),
      Particle::element("objectDefaults", Occurs::OPTIONAL),
      Particle::element("extraClrSchemeLst", Occurs::OPTIONAL),
      Particle::element("custClrLst", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ThemeOverride`.
pub fn particle_theme_override() -> Particle {
    Particle::sequence(vec![
      Particle::element("clrScheme", Occurs::OPTIONAL),
      Particle::element("fontScheme", Occurs::OPTIONAL),
      Particle::element("fmtScheme", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Table`.
pub fn particle_table() -> Particle {
    Particle::sequence(vec![
      Particle::element("tblPr", Occurs::OPTIONAL),
      Particle::element("tblGrid", Occurs::STAR),
      Particle::element("tr", Occurs::new(1, Some(1000))),
  ], Occurs::STAR)
}

/// Content model particle for `TableStyleList`.
pub fn particle_table_style_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("tblStyle", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ExtensionList`.
pub fn particle_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::sequence(vec![
              Particle::element("ext", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CustomColor`.
pub fn particle_custom_color() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Scene3DType`.
pub fn particle_scene3_d_type() -> Particle {
    Particle::sequence(vec![
      Particle::element("camera", Occurs::STAR),
      Particle::element("lightRig", Occurs::STAR),
      Particle::element("backdrop", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `EffectStyle`.
pub fn particle_effect_style() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("scene3d", Occurs::OPTIONAL),
      Particle::element("sp3d", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `FillStyleList`.
pub fn particle_fill_style_list() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::new(3, None)),
  ], Occurs::STAR)
}

/// Content model particle for `LineStyleList`.
pub fn particle_line_style_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ln", Occurs::new(3, None)),
  ], Occurs::STAR)
}

/// Content model particle for `EffectStyleList`.
pub fn particle_effect_style_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("effectStyle", Occurs::new(3, None)),
  ], Occurs::STAR)
}

/// Content model particle for `BackgroundFillStyleList`.
pub fn particle_background_fill_style_list() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::new(3, None)),
  ], Occurs::STAR)
}

/// Content model particle for `ColorScheme`.
pub fn particle_color_scheme() -> Particle {
    Particle::sequence(vec![
      Particle::element("dk1", Occurs::STAR),
      Particle::element("lt1", Occurs::STAR),
      Particle::element("dk2", Occurs::STAR),
      Particle::element("lt2", Occurs::STAR),
      Particle::element("accent1", Occurs::STAR),
      Particle::element("accent2", Occurs::STAR),
      Particle::element("accent3", Occurs::STAR),
      Particle::element("accent4", Occurs::STAR),
      Particle::element("accent5", Occurs::STAR),
      Particle::element("accent6", Occurs::STAR),
      Particle::element("hlink", Occurs::STAR),
      Particle::element("folHlink", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `FontScheme`.
pub fn particle_font_scheme() -> Particle {
    Particle::sequence(vec![
      Particle::element("majorFont", Occurs::STAR),
      Particle::element("minorFont", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `FormatScheme`.
pub fn particle_format_scheme() -> Particle {
    Particle::sequence(vec![
      Particle::element("fillStyleLst", Occurs::STAR),
      Particle::element("lnStyleLst", Occurs::STAR),
      Particle::element("effectStyleLst", Occurs::STAR),
      Particle::element("bgFillStyleLst", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Dark1Color`.
pub fn particle_dark1_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Light1Color`.
pub fn particle_light1_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Dark2Color`.
pub fn particle_dark2_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Light2Color`.
pub fn particle_light2_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Accent1Color`.
pub fn particle_accent1_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Accent2Color`.
pub fn particle_accent2_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Accent3Color`.
pub fn particle_accent3_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Accent4Color`.
pub fn particle_accent4_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Accent5Color`.
pub fn particle_accent5_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Accent6Color`.
pub fn particle_accent6_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `Hyperlink`.
pub fn particle_hyperlink() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `FollowedHyperlinkColor`.
pub fn particle_followed_hyperlink_color() -> Particle {
    Particle::choice(vec![
      Particle::element("scrgbClr", Occurs::STAR),
      Particle::element("srgbClr", Occurs::STAR),
      Particle::element("hslClr", Occurs::STAR),
      Particle::element("sysClr", Occurs::STAR),
      Particle::element("prstClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeLocks`.
pub fn particle_shape_locks() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ConnectionShapeLocks`.
pub fn particle_connection_shape_locks() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GraphicFrameLocks`.
pub fn particle_graphic_frame_locks() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GraphicData`.
pub fn particle_graphic_data() -> Particle {
    Particle::sequence(vec![
      Particle::any(Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TextBody`.
pub fn particle_text_body() -> Particle {
    Particle::sequence(vec![
      Particle::element("bodyPr", Occurs::STAR),
      Particle::element("lstStyle", Occurs::OPTIONAL),
      Particle::element("p", Occurs::PLUS),
  ], Occurs::STAR)
}

/// Content model particle for `Transform2D`.
pub fn particle_transform2_d() -> Particle {
    Particle::sequence(vec![
      Particle::element("off", Occurs::OPTIONAL),
      Particle::element("ext", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualDrawingProperties`.
pub fn particle_non_visual_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("hlinkClick", Occurs::OPTIONAL),
      Particle::element("hlinkHover", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualShapeDrawingProperties`.
pub fn particle_non_visual_shape_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("spLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualShapeProperties`.
pub fn particle_non_visual_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvSpPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeProperties`.
pub fn particle_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("xfrm", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("custGeom", Occurs::STAR),
              Particle::element("prstGeom", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("ln", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("scene3d", Occurs::OPTIONAL),
      Particle::element("sp3d", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TextShape`.
pub fn particle_text_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("txBody", Occurs::STAR),
      Particle::choice(vec![
          Particle::element("useSpRect", Occurs::STAR),
          Particle::element("xfrm", Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeStyle`.
pub fn particle_shape_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnRef", Occurs::STAR),
      Particle::element("fillRef", Occurs::STAR),
      Particle::element("effectRef", Occurs::STAR),
      Particle::element("fontRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualConnectorShapeDrawingProperties`.
pub fn particle_non_visual_connector_shape_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cxnSpLocks", Occurs::OPTIONAL),
      Particle::element("stCxn", Occurs::OPTIONAL),
      Particle::element("endCxn", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualConnectionShapeProperties`.
pub fn particle_non_visual_connection_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvCxnSpPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualPictureDrawingProperties`.
pub fn particle_non_visual_picture_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("picLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualPictureProperties`.
pub fn particle_non_visual_picture_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvPicPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGraphicFrameDrawingProperties`.
pub fn particle_non_visual_graphic_frame_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("graphicFrameLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGraphicFrameProperties`.
pub fn particle_non_visual_graphic_frame_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvGraphicFramePr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGroupShapeDrawingProperties`.
pub fn particle_non_visual_group_shape_drawing_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("grpSpLocks", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Camera`.
pub fn particle_camera() -> Particle {
    Particle::sequence(vec![
      Particle::element("rot", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LightRig`.
pub fn particle_light_rig() -> Particle {
    Particle::sequence(vec![
      Particle::element("rot", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Backdrop`.
pub fn particle_backdrop() -> Particle {
    Particle::sequence(vec![
      Particle::element("anchor", Occurs::STAR),
      Particle::element("norm", Occurs::STAR),
      Particle::element("up", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GradientStop`.
pub fn particle_gradient_stop() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `GradientStopList`.
pub fn particle_gradient_stop_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("gs", Occurs::new(2, None)),
  ], Occurs::STAR)
}

/// Content model particle for `AdjustHandleXY`.
pub fn particle_adjust_handle_x_y() -> Particle {
    Particle::sequence(vec![
      Particle::element("pos", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `AdjustHandlePolar`.
pub fn particle_adjust_handle_polar() -> Particle {
    Particle::sequence(vec![
      Particle::element("pos", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ConnectionSite`.
pub fn particle_connection_site() -> Particle {
    Particle::sequence(vec![
      Particle::element("pos", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `MoveTo`.
pub fn particle_move_to() -> Particle {
    Particle::sequence(vec![
      Particle::element("pt", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `LineTo`.
pub fn particle_line_to() -> Particle {
    Particle::sequence(vec![
      Particle::element("pt", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `QuadraticBezierCurveTo`.
pub fn particle_quadratic_bezier_curve_to() -> Particle {
    Particle::sequence(vec![
      Particle::element("pt", Occurs::new(2, Some(2))),
  ], Occurs::STAR)
}

/// Content model particle for `CubicBezierCurveTo`.
pub fn particle_cubic_bezier_curve_to() -> Particle {
    Particle::sequence(vec![
      Particle::element("pt", Occurs::new(3, Some(3))),
  ], Occurs::STAR)
}

/// Content model particle for `Path`.
pub fn particle_path() -> Particle {
    Particle::choice(vec![
      Particle::element("close", Occurs::STAR),
      Particle::element("moveTo", Occurs::STAR),
      Particle::element("lnTo", Occurs::STAR),
      Particle::element("arcTo", Occurs::STAR),
      Particle::element("quadBezTo", Occurs::STAR),
      Particle::element("cubicBezTo", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `AdjustValueList`.
pub fn particle_adjust_value_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("gd", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeGuideList`.
pub fn particle_shape_guide_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("gd", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `AdjustHandleList`.
pub fn particle_adjust_handle_list() -> Particle {
    Particle::choice(vec![
      Particle::element("ahXY", Occurs::STAR),
      Particle::element("ahPolar", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ConnectionSiteList`.
pub fn particle_connection_site_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("cxn", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `PathList`.
pub fn particle_path_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("path", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TransformGroup`.
pub fn particle_transform_group() -> Particle {
    Particle::sequence(vec![
      Particle::element("off", Occurs::OPTIONAL),
      Particle::element("ext", Occurs::OPTIONAL),
      Particle::element("chOff", Occurs::OPTIONAL),
      Particle::element("chExt", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BodyProperties`.
pub fn particle_body_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("prstTxWarp", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noAutofit", Occurs::STAR),
              Particle::element("normAutofit", Occurs::STAR),
              Particle::element("spAutoFit", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("scene3d", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("sp3d", Occurs::STAR),
              Particle::element("flatTx", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ListStyle`.
pub fn particle_list_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("defPPr", Occurs::OPTIONAL),
      Particle::element("lvl1pPr", Occurs::OPTIONAL),
      Particle::element("lvl2pPr", Occurs::OPTIONAL),
      Particle::element("lvl3pPr", Occurs::OPTIONAL),
      Particle::element("lvl4pPr", Occurs::OPTIONAL),
      Particle::element("lvl5pPr", Occurs::OPTIONAL),
      Particle::element("lvl6pPr", Occurs::OPTIONAL),
      Particle::element("lvl7pPr", Occurs::OPTIONAL),
      Particle::element("lvl8pPr", Occurs::OPTIONAL),
      Particle::element("lvl9pPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ShapeDefault`.
pub fn particle_shape_default() -> Particle {
    Particle::sequence(vec![
      Particle::element("spPr", Occurs::STAR),
      Particle::element("bodyPr", Occurs::STAR),
      Particle::element("lstStyle", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LineDefault`.
pub fn particle_line_default() -> Particle {
    Particle::sequence(vec![
      Particle::element("spPr", Occurs::STAR),
      Particle::element("bodyPr", Occurs::STAR),
      Particle::element("lstStyle", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TextDefault`.
pub fn particle_text_default() -> Particle {
    Particle::sequence(vec![
      Particle::element("spPr", Occurs::STAR),
      Particle::element("bodyPr", Occurs::STAR),
      Particle::element("lstStyle", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `OverrideColorMapping`.
pub fn particle_override_color_mapping() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ColorMap`.
pub fn particle_color_map() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ExtraColorScheme`.
pub fn particle_extra_color_scheme() -> Particle {
    Particle::sequence(vec![
      Particle::element("clrScheme", Occurs::STAR),
      Particle::element("clrMap", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ThemeElements`.
pub fn particle_theme_elements() -> Particle {
    Particle::sequence(vec![
      Particle::element("clrScheme", Occurs::STAR),
      Particle::element("fontScheme", Occurs::STAR),
      Particle::element("fmtScheme", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Cell3DProperties`.
pub fn particle_cell3_d_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("bevel", Occurs::STAR),
      Particle::element("lightRig", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableCellProperties`.
pub fn particle_table_cell_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnL", Occurs::OPTIONAL),
      Particle::element("lnR", Occurs::OPTIONAL),
      Particle::element("lnT", Occurs::OPTIONAL),
      Particle::element("lnB", Occurs::OPTIONAL),
      Particle::element("lnTlToBr", Occurs::OPTIONAL),
      Particle::element("lnBlToTr", Occurs::OPTIONAL),
      Particle::element("cell3D", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableCell`.
pub fn particle_table_cell() -> Particle {
    Particle::sequence(vec![
      Particle::element("txBody", Occurs::OPTIONAL),
      Particle::element("tcPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableStyle`.
pub fn particle_table_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("tblBg", Occurs::OPTIONAL),
      Particle::element("wholeTbl", Occurs::OPTIONAL),
      Particle::element("band1H", Occurs::OPTIONAL),
      Particle::element("band2H", Occurs::OPTIONAL),
      Particle::element("band1V", Occurs::OPTIONAL),
      Particle::element("band2V", Occurs::OPTIONAL),
      Particle::element("lastCol", Occurs::OPTIONAL),
      Particle::element("firstCol", Occurs::OPTIONAL),
      Particle::element("lastRow", Occurs::OPTIONAL),
      Particle::element("seCell", Occurs::OPTIONAL),
      Particle::element("swCell", Occurs::OPTIONAL),
      Particle::element("firstRow", Occurs::OPTIONAL),
      Particle::element("neCell", Occurs::OPTIONAL),
      Particle::element("nwCell", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableStyleEntry`.
pub fn particle_table_style_entry() -> Particle {
    Particle::sequence(vec![
      Particle::element("tblBg", Occurs::OPTIONAL),
      Particle::element("wholeTbl", Occurs::OPTIONAL),
      Particle::element("band1H", Occurs::OPTIONAL),
      Particle::element("band2H", Occurs::OPTIONAL),
      Particle::element("band1V", Occurs::OPTIONAL),
      Particle::element("band2V", Occurs::OPTIONAL),
      Particle::element("lastCol", Occurs::OPTIONAL),
      Particle::element("firstCol", Occurs::OPTIONAL),
      Particle::element("lastRow", Occurs::OPTIONAL),
      Particle::element("seCell", Occurs::OPTIONAL),
      Particle::element("swCell", Occurs::OPTIONAL),
      Particle::element("firstRow", Occurs::OPTIONAL),
      Particle::element("neCell", Occurs::OPTIONAL),
      Particle::element("nwCell", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GridColumn`.
pub fn particle_grid_column() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableProperties`.
pub fn particle_table_properties() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::choice(vec![
          Particle::element("tableStyle", Occurs::STAR),
          Particle::element("tableStyleId", Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableGrid`.
pub fn particle_table_grid() -> Particle {
    Particle::sequence(vec![
      Particle::element("gridCol", Occurs::new(1, Some(1000))),
  ], Occurs::STAR)
}

/// Content model particle for `TableRow`.
pub fn particle_table_row() -> Particle {
    Particle::sequence(vec![
      Particle::element("tc", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LeftBorder`.
pub fn particle_left_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `RightBorder`.
pub fn particle_right_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TopBorder`.
pub fn particle_top_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `BottomBorder`.
pub fn particle_bottom_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `InsideHorizontalBorder`.
pub fn particle_inside_horizontal_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `InsideVerticalBorder`.
pub fn particle_inside_vertical_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TopLeftToBottomRightBorder`.
pub fn particle_top_left_to_bottom_right_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TopRightToBottomLeftBorder`.
pub fn particle_top_right_to_bottom_left_border() -> Particle {
    Particle::choice(vec![
      Particle::element("ln", Occurs::STAR),
      Particle::element("lnRef", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TableCellBorders`.
pub fn particle_table_cell_borders() -> Particle {
    Particle::sequence(vec![
      Particle::element("left", Occurs::OPTIONAL),
      Particle::element("right", Occurs::OPTIONAL),
      Particle::element("top", Occurs::OPTIONAL),
      Particle::element("bottom", Occurs::OPTIONAL),
      Particle::element("insideH", Occurs::OPTIONAL),
      Particle::element("insideV", Occurs::OPTIONAL),
      Particle::element("tl2br", Occurs::OPTIONAL),
      Particle::element("tr2bl", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableCellTextStyle`.
pub fn particle_table_cell_text_style() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("font", Occurs::STAR),
              Particle::element("fontRef", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("scrgbClr", Occurs::STAR),
              Particle::element("srgbClr", Occurs::STAR),
              Particle::element("hslClr", Occurs::STAR),
              Particle::element("sysClr", Occurs::STAR),
              Particle::element("schemeClr", Occurs::STAR),
              Particle::element("prstClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableCellStyle`.
pub fn particle_table_cell_style() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcBdr", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("fill", Occurs::STAR),
              Particle::element("fillRef", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("cell3D", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `TableBackground`.
pub fn particle_table_background() -> Particle {
    Particle::sequence(vec![
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("fill", Occurs::STAR),
              Particle::element("fillRef", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effect", Occurs::STAR),
              Particle::element("effectRef", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `WholeTable`.
pub fn particle_whole_table() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Band1Horizontal`.
pub fn particle_band1_horizontal() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Band2Horizontal`.
pub fn particle_band2_horizontal() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Band1Vertical`.
pub fn particle_band1_vertical() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Band2Vertical`.
pub fn particle_band2_vertical() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LastColumn`.
pub fn particle_last_column() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `FirstColumn`.
pub fn particle_first_column() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LastRow`.
pub fn particle_last_row() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SoutheastCell`.
pub fn particle_southeast_cell() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `SouthwestCell`.
pub fn particle_southwest_cell() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `FirstRow`.
pub fn particle_first_row() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NortheastCell`.
pub fn particle_northeast_cell() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NorthwestCell`.
pub fn particle_northwest_cell() -> Particle {
    Particle::sequence(vec![
      Particle::element("tcTxStyle", Occurs::OPTIONAL),
      Particle::element("tcStyle", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ParagraphProperties`.
pub fn particle_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `DefaultParagraphProperties`.
pub fn particle_default_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level1ParagraphProperties`.
pub fn particle_level1_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level2ParagraphProperties`.
pub fn particle_level2_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level3ParagraphProperties`.
pub fn particle_level3_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level4ParagraphProperties`.
pub fn particle_level4_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level5ParagraphProperties`.
pub fn particle_level5_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level6ParagraphProperties`.
pub fn particle_level6_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level7ParagraphProperties`.
pub fn particle_level7_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level8ParagraphProperties`.
pub fn particle_level8_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Level9ParagraphProperties`.
pub fn particle_level9_paragraph_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("lnSpc", Occurs::OPTIONAL),
      Particle::element("spcBef", Occurs::OPTIONAL),
      Particle::element("spcAft", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buClrTx", Occurs::STAR),
              Particle::element("buClr", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buSzTx", Occurs::STAR),
              Particle::element("buSzPct", Occurs::STAR),
              Particle::element("buSzPts", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buFontTx", Occurs::STAR),
              Particle::element("buFont", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("buNone", Occurs::STAR),
              Particle::element("buAutoNum", Occurs::STAR),
              Particle::element("buChar", Occurs::STAR),
              Particle::element("buBlip", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("tabLst", Occurs::OPTIONAL),
      Particle::element("defRPr", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `EndParagraphRunProperties`.
pub fn particle_end_paragraph_run_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("ln", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("highlight", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("uLnTx", Occurs::STAR),
              Particle::element("uLn", Occurs::OPTIONAL),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("uFillTx", Occurs::STAR),
              Particle::element("uFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("latin", Occurs::OPTIONAL),
      Particle::element("ea", Occurs::OPTIONAL),
      Particle::element("cs", Occurs::OPTIONAL),
      Particle::element("sym", Occurs::OPTIONAL),
      Particle::element("hlinkClick", Occurs::OPTIONAL),
      Particle::element("hlinkMouseOver", Occurs::OPTIONAL),
      Particle::element("rtl", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `RunProperties`.
pub fn particle_run_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("ln", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("highlight", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("uLnTx", Occurs::STAR),
              Particle::element("uLn", Occurs::OPTIONAL),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("uFillTx", Occurs::STAR),
              Particle::element("uFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("latin", Occurs::OPTIONAL),
      Particle::element("ea", Occurs::OPTIONAL),
      Particle::element("cs", Occurs::OPTIONAL),
      Particle::element("sym", Occurs::OPTIONAL),
      Particle::element("hlinkClick", Occurs::OPTIONAL),
      Particle::element("hlinkMouseOver", Occurs::OPTIONAL),
      Particle::element("rtl", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `DefaultRunProperties`.
pub fn particle_default_run_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("ln", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("highlight", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("uLnTx", Occurs::STAR),
              Particle::element("uLn", Occurs::OPTIONAL),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("uFillTx", Occurs::STAR),
              Particle::element("uFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("latin", Occurs::OPTIONAL),
      Particle::element("ea", Occurs::OPTIONAL),
      Particle::element("cs", Occurs::OPTIONAL),
      Particle::element("sym", Occurs::OPTIONAL),
      Particle::element("hlinkClick", Occurs::OPTIONAL),
      Particle::element("hlinkMouseOver", Occurs::OPTIONAL),
      Particle::element("rtl", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Paragraph`.
pub fn particle_paragraph() -> Particle {
    Particle::sequence(vec![
      Particle::element("pPr", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("r", Occurs::STAR),
              Particle::element("br", Occurs::STAR),
              Particle::element("fld", Occurs::STAR),
              Particle::element("m", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("endParaRPr", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LineSpacing`.
pub fn particle_line_spacing() -> Particle {
    Particle::choice(vec![
      Particle::element("spcPct", Occurs::STAR),
      Particle::element("spcPts", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SpaceBefore`.
pub fn particle_space_before() -> Particle {
    Particle::choice(vec![
      Particle::element("spcPct", Occurs::STAR),
      Particle::element("spcPts", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `SpaceAfter`.
pub fn particle_space_after() -> Particle {
    Particle::choice(vec![
      Particle::element("spcPct", Occurs::STAR),
      Particle::element("spcPts", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `TabStopList`.
pub fn particle_tab_stop_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("tab", Occurs::new(0, Some(32))),
  ], Occurs::STAR)
}

/// Content model particle for `ShapePropertiesExtension`.
pub fn particle_shape_properties_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("hiddenFill", Occurs::STAR),
      Particle::element("hiddenLine", Occurs::STAR),
      Particle::element("hiddenEffects", Occurs::STAR),
      Particle::element("hiddenScene3d", Occurs::STAR),
      Particle::element("hiddenSp3d", Occurs::STAR),
      Particle::element("shadowObscured", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GvmlGroupShapeExtension`.
pub fn particle_gvml_group_shape_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("isCanvas", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ShapePropertiesExtensionList`.
pub fn particle_shape_properties_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGroupShapeProperties`.
pub fn particle_non_visual_group_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::element("cNvGrpSpPr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `VisualGroupShapeProperties`.
pub fn particle_visual_group_shape_properties() -> Particle {
    Particle::sequence(vec![
      Particle::element("xfrm", Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("noFill", Occurs::STAR),
              Particle::element("solidFill", Occurs::STAR),
              Particle::element("gradFill", Occurs::STAR),
              Particle::element("blipFill", Occurs::STAR),
              Particle::element("pattFill", Occurs::STAR),
              Particle::element("grpFill", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::group(vec![
          Particle::choice(vec![
              Particle::element("effectLst", Occurs::STAR),
              Particle::element("effectDag", Occurs::STAR),
          ], Occurs::STAR),
      ], Occurs::OPTIONAL),
      Particle::element("scene3d", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Shape`.
pub fn particle_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvSpPr", Occurs::STAR),
      Particle::element("spPr", Occurs::STAR),
      Particle::element("txSp", Occurs::OPTIONAL),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ConnectionShape`.
pub fn particle_connection_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvCxnSpPr", Occurs::STAR),
      Particle::element("spPr", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `Picture`.
pub fn particle_picture() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvPicPr", Occurs::STAR),
      Particle::element("blipFill", Occurs::STAR),
      Particle::element("spPr", Occurs::STAR),
      Particle::element("style", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GraphicFrame`.
pub fn particle_graphic_frame() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvGraphicFramePr", Occurs::STAR),
      Particle::element("graphic", Occurs::STAR),
      Particle::element("xfrm", Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GroupShape`.
pub fn particle_group_shape() -> Particle {
    Particle::sequence(vec![
      Particle::element("nvGrpSpPr", Occurs::STAR),
      Particle::element("grpSpPr", Occurs::STAR),
      Particle::choice(vec![
          Particle::element("txSp", Occurs::STAR),
          Particle::element("sp", Occurs::STAR),
          Particle::element("cxnSp", Occurs::STAR),
          Particle::element("pic", Occurs::STAR),
          Particle::element("contentPart", Occurs::STAR),
          Particle::element("graphicFrame", Occurs::STAR),
          Particle::element("grpSp", Occurs::STAR),
      ], Occurs::STAR),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GvmlGroupShapeExtensionList`.
pub fn particle_gvml_group_shape_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGroupDrawingShapePropsExtension`.
pub fn particle_non_visual_group_drawing_shape_props_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("nonVisualGroupProps", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `OfficeStyleSheetExtension`.
pub fn particle_office_style_sheet_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("themeFamily", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ConnectorLockingExtension`.
pub fn particle_connector_locking_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("graphic", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `GroupShapeLocks`.
pub fn particle_group_shape_locks() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualGroupDrawingShapePropsExtensionList`.
pub fn particle_non_visual_group_drawing_shape_props_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ObjectDefaults`.
pub fn particle_object_defaults() -> Particle {
    Particle::sequence(vec![
      Particle::element("spDef", Occurs::OPTIONAL),
      Particle::element("lnDef", Occurs::OPTIONAL),
      Particle::element("txDef", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `ExtraColorSchemeList`.
pub fn particle_extra_color_scheme_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("extraClrScheme", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `CustomColorList`.
pub fn particle_custom_color_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("custClr", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `OfficeStyleSheetExtensionList`.
pub fn particle_office_style_sheet_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `HyperlinkOnClick`.
pub fn particle_hyperlink_on_click() -> Particle {
    Particle::sequence(vec![
      Particle::element("snd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HyperlinkOnMouseOver`.
pub fn particle_hyperlink_on_mouse_over() -> Particle {
    Particle::sequence(vec![
      Particle::element("snd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HyperlinkOnHover`.
pub fn particle_hyperlink_on_hover() -> Particle {
    Particle::sequence(vec![
      Particle::element("snd", Occurs::OPTIONAL),
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualDrawingPropertiesExtensionList`.
pub fn particle_non_visual_drawing_properties_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `ConnectorLockingExtensionList`.
pub fn particle_connector_locking_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `DataModelExtension`.
pub fn particle_data_model_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("dataModelExt", Occurs::STAR),
      Particle::element("recolorImg", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PtExtension`.
pub fn particle_pt_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("cNvPr", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HyperlinkExtension`.
pub fn particle_hyperlink_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("hlinkClr", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `HyperlinkExtensionList`.
pub fn particle_hyperlink_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `LinePropertiesExtension`.
pub fn particle_line_properties_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("lineSketchStyleProps", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `LinePropertiesExtensionList`.
pub fn particle_line_properties_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualDrawingPropertiesExtension`.
pub fn particle_non_visual_drawing_properties_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("compatExt", Occurs::STAR),
      Particle::element("backgroundPr", Occurs::STAR),
      Particle::element("creationId", Occurs::STAR),
      Particle::element("predDERef", Occurs::STAR),
      Particle::element("decorative", Occurs::STAR),
      Particle::element("classification", Occurs::STAR),
      Particle::element("scriptLink", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `PictureLocks`.
pub fn particle_picture_locks() -> Particle {
    Particle::sequence(vec![
      Particle::element("extLst", Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualPicturePropertiesExtensionList`.
pub fn particle_non_visual_picture_properties_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `NonVisualPicturePropertiesExtension`.
pub fn particle_non_visual_picture_properties_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("cameraTool", Occurs::STAR),
      Particle::element("signatureLine", Occurs::STAR),
      Particle::element("objectPr", Occurs::STAR),
      Particle::element("liveFeedProps", Occurs::STAR),
      Particle::element("imageFormula", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Content model particle for `BlipExtensionList`.
pub fn particle_blip_extension_list() -> Particle {
    Particle::sequence(vec![
      Particle::element("ext", Occurs::STAR),
  ], Occurs::STAR)
}

/// Content model particle for `BlipExtension`.
pub fn particle_blip_extension() -> Particle {
    Particle::choice(vec![
      Particle::element("imgProps", Occurs::STAR),
      Particle::element("useLocalDpi", Occurs::STAR),
      Particle::element("webVideoPr", Occurs::STAR),
      Particle::element("svgBlip", Occurs::STAR),
      Particle::element("picAttrSrcUrl", Occurs::STAR),
      Particle::element("oembed", Occurs::STAR),
      Particle::element("oembedShared", Occurs::STAR),
      Particle::any(Occurs::OPTIONAL),
  ], Occurs::STAR)
}

/// Look up a content-model particle by schema class name.
pub fn particle_for_class(class_name: &str) -> Option<Particle> {
    match class_name {
        "AudioFromCD" => Some(particle_audio_from_c_d()),
        "AudioFromFile" => Some(particle_audio_from_file()),
        "VideoFromFile" => Some(particle_video_from_file()),
        "QuickTimeFromFile" => Some(particle_quick_time_from_file()),
        "Extension" => Some(particle_extension()),
        "RgbColorModelPercentage" => Some(particle_rgb_color_model_percentage()),
        "RgbColorModelHex" => Some(particle_rgb_color_model_hex()),
        "HslColor" => Some(particle_hsl_color()),
        "SystemColor" => Some(particle_system_color()),
        "SchemeColor" => Some(particle_scheme_color()),
        "PresetColor" => Some(particle_preset_color()),
        "Shape3DType" => Some(particle_shape3_d_type()),
        "PathGradientFill" => Some(particle_path_gradient_fill()),
        "Stretch" => Some(particle_stretch()),
        "SolidFill" => Some(particle_solid_fill()),
        "GradientFill" => Some(particle_gradient_fill()),
        "BlipFill" => Some(particle_blip_fill()),
        "PatternFill" => Some(particle_pattern_fill()),
        "EffectContainer" => Some(particle_effect_container()),
        "EffectDag" => Some(particle_effect_dag()),
        "AlphaInverse" => Some(particle_alpha_inverse()),
        "AlphaModulationEffect" => Some(particle_alpha_modulation_effect()),
        "Blend" => Some(particle_blend()),
        "ColorChange" => Some(particle_color_change()),
        "ColorReplacement" => Some(particle_color_replacement()),
        "Duotone" => Some(particle_duotone()),
        "Fill" => Some(particle_fill()),
        "FillOverlay" => Some(particle_fill_overlay()),
        "Glow" => Some(particle_glow()),
        "InnerShadow" => Some(particle_inner_shadow()),
        "OuterShadow" => Some(particle_outer_shadow()),
        "PresetShadow" => Some(particle_preset_shadow()),
        "EffectList" => Some(particle_effect_list()),
        "CustomGeometry" => Some(particle_custom_geometry()),
        "PresetGeometry" => Some(particle_preset_geometry()),
        "PresetTextWarp" => Some(particle_preset_text_warp()),
        "CustomDash" => Some(particle_custom_dash()),
        "FillProperties" => Some(particle_fill_properties()),
        "FillReference" => Some(particle_fill_reference()),
        "EffectReference" => Some(particle_effect_reference()),
        "LineReference" => Some(particle_line_reference()),
        "EffectPropertiesType" => Some(particle_effect_properties_type()),
        "Fonts" => Some(particle_fonts()),
        "MajorFont" => Some(particle_major_font()),
        "MinorFont" => Some(particle_minor_font()),
        "FontReference" => Some(particle_font_reference()),
        "BulletColor" => Some(particle_bullet_color()),
        "ExtrusionColor" => Some(particle_extrusion_color()),
        "ContourColor" => Some(particle_contour_color()),
        "ColorFrom" => Some(particle_color_from()),
        "ColorTo" => Some(particle_color_to()),
        "ForegroundColor" => Some(particle_foreground_color()),
        "BackgroundColor" => Some(particle_background_color()),
        "Highlight" => Some(particle_highlight()),
        "PictureBullet" => Some(particle_picture_bullet()),
        "Underline" => Some(particle_underline()),
        "Outline" => Some(particle_outline()),
        "LeftBorderLineProperties" => Some(particle_left_border_line_properties()),
        "RightBorderLineProperties" => Some(particle_right_border_line_properties()),
        "TopBorderLineProperties" => Some(particle_top_border_line_properties()),
        "BottomBorderLineProperties" => Some(particle_bottom_border_line_properties()),
        "TopLeftToBottomRightBorderLineProperties" => Some(particle_top_left_to_bottom_right_border_line_properties()),
        "BottomLeftToTopRightBorderLineProperties" => Some(particle_bottom_left_to_top_right_border_line_properties()),
        "UnderlineFill" => Some(particle_underline_fill()),
        "Run" => Some(particle_run()),
        "Break" => Some(particle_break_()),
        "Field" => Some(particle_field()),
        "Graphic" => Some(particle_graphic()),
        "Blip" => Some(particle_blip()),
        "Theme" => Some(particle_theme()),
        "ThemeOverride" => Some(particle_theme_override()),
        "Table" => Some(particle_table()),
        "TableStyleList" => Some(particle_table_style_list()),
        "ExtensionList" => Some(particle_extension_list()),
        "CustomColor" => Some(particle_custom_color()),
        "Scene3DType" => Some(particle_scene3_d_type()),
        "EffectStyle" => Some(particle_effect_style()),
        "FillStyleList" => Some(particle_fill_style_list()),
        "LineStyleList" => Some(particle_line_style_list()),
        "EffectStyleList" => Some(particle_effect_style_list()),
        "BackgroundFillStyleList" => Some(particle_background_fill_style_list()),
        "ColorScheme" => Some(particle_color_scheme()),
        "FontScheme" => Some(particle_font_scheme()),
        "FormatScheme" => Some(particle_format_scheme()),
        "Dark1Color" => Some(particle_dark1_color()),
        "Light1Color" => Some(particle_light1_color()),
        "Dark2Color" => Some(particle_dark2_color()),
        "Light2Color" => Some(particle_light2_color()),
        "Accent1Color" => Some(particle_accent1_color()),
        "Accent2Color" => Some(particle_accent2_color()),
        "Accent3Color" => Some(particle_accent3_color()),
        "Accent4Color" => Some(particle_accent4_color()),
        "Accent5Color" => Some(particle_accent5_color()),
        "Accent6Color" => Some(particle_accent6_color()),
        "Hyperlink" => Some(particle_hyperlink()),
        "FollowedHyperlinkColor" => Some(particle_followed_hyperlink_color()),
        "ShapeLocks" => Some(particle_shape_locks()),
        "ConnectionShapeLocks" => Some(particle_connection_shape_locks()),
        "GraphicFrameLocks" => Some(particle_graphic_frame_locks()),
        "GraphicData" => Some(particle_graphic_data()),
        "TextBody" => Some(particle_text_body()),
        "Transform2D" => Some(particle_transform2_d()),
        "NonVisualDrawingProperties" => Some(particle_non_visual_drawing_properties()),
        "NonVisualShapeDrawingProperties" => Some(particle_non_visual_shape_drawing_properties()),
        "NonVisualShapeProperties" => Some(particle_non_visual_shape_properties()),
        "ShapeProperties" => Some(particle_shape_properties()),
        "TextShape" => Some(particle_text_shape()),
        "ShapeStyle" => Some(particle_shape_style()),
        "NonVisualConnectorShapeDrawingProperties" => Some(particle_non_visual_connector_shape_drawing_properties()),
        "NonVisualConnectionShapeProperties" => Some(particle_non_visual_connection_shape_properties()),
        "NonVisualPictureDrawingProperties" => Some(particle_non_visual_picture_drawing_properties()),
        "NonVisualPictureProperties" => Some(particle_non_visual_picture_properties()),
        "NonVisualGraphicFrameDrawingProperties" => Some(particle_non_visual_graphic_frame_drawing_properties()),
        "NonVisualGraphicFrameProperties" => Some(particle_non_visual_graphic_frame_properties()),
        "NonVisualGroupShapeDrawingProperties" => Some(particle_non_visual_group_shape_drawing_properties()),
        "Camera" => Some(particle_camera()),
        "LightRig" => Some(particle_light_rig()),
        "Backdrop" => Some(particle_backdrop()),
        "GradientStop" => Some(particle_gradient_stop()),
        "GradientStopList" => Some(particle_gradient_stop_list()),
        "AdjustHandleXY" => Some(particle_adjust_handle_x_y()),
        "AdjustHandlePolar" => Some(particle_adjust_handle_polar()),
        "ConnectionSite" => Some(particle_connection_site()),
        "MoveTo" => Some(particle_move_to()),
        "LineTo" => Some(particle_line_to()),
        "QuadraticBezierCurveTo" => Some(particle_quadratic_bezier_curve_to()),
        "CubicBezierCurveTo" => Some(particle_cubic_bezier_curve_to()),
        "Path" => Some(particle_path()),
        "AdjustValueList" => Some(particle_adjust_value_list()),
        "ShapeGuideList" => Some(particle_shape_guide_list()),
        "AdjustHandleList" => Some(particle_adjust_handle_list()),
        "ConnectionSiteList" => Some(particle_connection_site_list()),
        "PathList" => Some(particle_path_list()),
        "TransformGroup" => Some(particle_transform_group()),
        "BodyProperties" => Some(particle_body_properties()),
        "ListStyle" => Some(particle_list_style()),
        "ShapeDefault" => Some(particle_shape_default()),
        "LineDefault" => Some(particle_line_default()),
        "TextDefault" => Some(particle_text_default()),
        "OverrideColorMapping" => Some(particle_override_color_mapping()),
        "ColorMap" => Some(particle_color_map()),
        "ExtraColorScheme" => Some(particle_extra_color_scheme()),
        "ThemeElements" => Some(particle_theme_elements()),
        "Cell3DProperties" => Some(particle_cell3_d_properties()),
        "TableCellProperties" => Some(particle_table_cell_properties()),
        "TableCell" => Some(particle_table_cell()),
        "TableStyle" => Some(particle_table_style()),
        "TableStyleEntry" => Some(particle_table_style_entry()),
        "GridColumn" => Some(particle_grid_column()),
        "TableProperties" => Some(particle_table_properties()),
        "TableGrid" => Some(particle_table_grid()),
        "TableRow" => Some(particle_table_row()),
        "LeftBorder" => Some(particle_left_border()),
        "RightBorder" => Some(particle_right_border()),
        "TopBorder" => Some(particle_top_border()),
        "BottomBorder" => Some(particle_bottom_border()),
        "InsideHorizontalBorder" => Some(particle_inside_horizontal_border()),
        "InsideVerticalBorder" => Some(particle_inside_vertical_border()),
        "TopLeftToBottomRightBorder" => Some(particle_top_left_to_bottom_right_border()),
        "TopRightToBottomLeftBorder" => Some(particle_top_right_to_bottom_left_border()),
        "TableCellBorders" => Some(particle_table_cell_borders()),
        "TableCellTextStyle" => Some(particle_table_cell_text_style()),
        "TableCellStyle" => Some(particle_table_cell_style()),
        "TableBackground" => Some(particle_table_background()),
        "WholeTable" => Some(particle_whole_table()),
        "Band1Horizontal" => Some(particle_band1_horizontal()),
        "Band2Horizontal" => Some(particle_band2_horizontal()),
        "Band1Vertical" => Some(particle_band1_vertical()),
        "Band2Vertical" => Some(particle_band2_vertical()),
        "LastColumn" => Some(particle_last_column()),
        "FirstColumn" => Some(particle_first_column()),
        "LastRow" => Some(particle_last_row()),
        "SoutheastCell" => Some(particle_southeast_cell()),
        "SouthwestCell" => Some(particle_southwest_cell()),
        "FirstRow" => Some(particle_first_row()),
        "NortheastCell" => Some(particle_northeast_cell()),
        "NorthwestCell" => Some(particle_northwest_cell()),
        "ParagraphProperties" => Some(particle_paragraph_properties()),
        "DefaultParagraphProperties" => Some(particle_default_paragraph_properties()),
        "Level1ParagraphProperties" => Some(particle_level1_paragraph_properties()),
        "Level2ParagraphProperties" => Some(particle_level2_paragraph_properties()),
        "Level3ParagraphProperties" => Some(particle_level3_paragraph_properties()),
        "Level4ParagraphProperties" => Some(particle_level4_paragraph_properties()),
        "Level5ParagraphProperties" => Some(particle_level5_paragraph_properties()),
        "Level6ParagraphProperties" => Some(particle_level6_paragraph_properties()),
        "Level7ParagraphProperties" => Some(particle_level7_paragraph_properties()),
        "Level8ParagraphProperties" => Some(particle_level8_paragraph_properties()),
        "Level9ParagraphProperties" => Some(particle_level9_paragraph_properties()),
        "EndParagraphRunProperties" => Some(particle_end_paragraph_run_properties()),
        "RunProperties" => Some(particle_run_properties()),
        "DefaultRunProperties" => Some(particle_default_run_properties()),
        "Paragraph" => Some(particle_paragraph()),
        "LineSpacing" => Some(particle_line_spacing()),
        "SpaceBefore" => Some(particle_space_before()),
        "SpaceAfter" => Some(particle_space_after()),
        "TabStopList" => Some(particle_tab_stop_list()),
        "ShapePropertiesExtension" => Some(particle_shape_properties_extension()),
        "GvmlGroupShapeExtension" => Some(particle_gvml_group_shape_extension()),
        "ShapePropertiesExtensionList" => Some(particle_shape_properties_extension_list()),
        "NonVisualGroupShapeProperties" => Some(particle_non_visual_group_shape_properties()),
        "VisualGroupShapeProperties" => Some(particle_visual_group_shape_properties()),
        "Shape" => Some(particle_shape()),
        "ConnectionShape" => Some(particle_connection_shape()),
        "Picture" => Some(particle_picture()),
        "GraphicFrame" => Some(particle_graphic_frame()),
        "GroupShape" => Some(particle_group_shape()),
        "GvmlGroupShapeExtensionList" => Some(particle_gvml_group_shape_extension_list()),
        "NonVisualGroupDrawingShapePropsExtension" => Some(particle_non_visual_group_drawing_shape_props_extension()),
        "OfficeStyleSheetExtension" => Some(particle_office_style_sheet_extension()),
        "ConnectorLockingExtension" => Some(particle_connector_locking_extension()),
        "GroupShapeLocks" => Some(particle_group_shape_locks()),
        "NonVisualGroupDrawingShapePropsExtensionList" => Some(particle_non_visual_group_drawing_shape_props_extension_list()),
        "ObjectDefaults" => Some(particle_object_defaults()),
        "ExtraColorSchemeList" => Some(particle_extra_color_scheme_list()),
        "CustomColorList" => Some(particle_custom_color_list()),
        "OfficeStyleSheetExtensionList" => Some(particle_office_style_sheet_extension_list()),
        "HyperlinkOnClick" => Some(particle_hyperlink_on_click()),
        "HyperlinkOnMouseOver" => Some(particle_hyperlink_on_mouse_over()),
        "HyperlinkOnHover" => Some(particle_hyperlink_on_hover()),
        "NonVisualDrawingPropertiesExtensionList" => Some(particle_non_visual_drawing_properties_extension_list()),
        "ConnectorLockingExtensionList" => Some(particle_connector_locking_extension_list()),
        "DataModelExtension" => Some(particle_data_model_extension()),
        "PtExtension" => Some(particle_pt_extension()),
        "HyperlinkExtension" => Some(particle_hyperlink_extension()),
        "HyperlinkExtensionList" => Some(particle_hyperlink_extension_list()),
        "LinePropertiesExtension" => Some(particle_line_properties_extension()),
        "LinePropertiesExtensionList" => Some(particle_line_properties_extension_list()),
        "NonVisualDrawingPropertiesExtension" => Some(particle_non_visual_drawing_properties_extension()),
        "PictureLocks" => Some(particle_picture_locks()),
        "NonVisualPicturePropertiesExtensionList" => Some(particle_non_visual_picture_properties_extension_list()),
        "NonVisualPicturePropertiesExtension" => Some(particle_non_visual_picture_properties_extension()),
        "BlipExtensionList" => Some(particle_blip_extension_list()),
        "BlipExtension" => Some(particle_blip_extension()),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Schema enums
// ---------------------------------------------------------------------------

/// Font Collection Index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontCollectionIndexValues {
    Major,
    Minor,
    None_,
}

impl FontCollectionIndexValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::None_ => "none",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "major" => Some(Self::Major),
            "minor" => Some(Self::Minor),
            "none" => Some(Self::None_),
            _ => None,
        }
    }
}

impl core::fmt::Display for FontCollectionIndexValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for FontCollectionIndexValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Theme Color Reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSchemeIndexValues {
    Dark1,
    Light1,
    Dark2,
    Light2,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Hyperlink,
    FollowedHyperlink,
}

impl ColorSchemeIndexValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dark1 => "dk1",
            Self::Light1 => "lt1",
            Self::Dark2 => "dk2",
            Self::Light2 => "lt2",
            Self::Accent1 => "accent1",
            Self::Accent2 => "accent2",
            Self::Accent3 => "accent3",
            Self::Accent4 => "accent4",
            Self::Accent5 => "accent5",
            Self::Accent6 => "accent6",
            Self::Hyperlink => "hlink",
            Self::FollowedHyperlink => "folHlink",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "dk1" => Some(Self::Dark1),
            "lt1" => Some(Self::Light1),
            "dk2" => Some(Self::Dark2),
            "lt2" => Some(Self::Light2),
            "accent1" => Some(Self::Accent1),
            "accent2" => Some(Self::Accent2),
            "accent3" => Some(Self::Accent3),
            "accent4" => Some(Self::Accent4),
            "accent5" => Some(Self::Accent5),
            "accent6" => Some(Self::Accent6),
            "hlink" => Some(Self::Hyperlink),
            "folHlink" => Some(Self::FollowedHyperlink),
            _ => None,
        }
    }
}

impl core::fmt::Display for ColorSchemeIndexValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for ColorSchemeIndexValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// System Color Value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemColorValues {
    ScrollBar,
    Background,
    ActiveCaption,
    InactiveCaption,
    Menu,
    Window,
    WindowFrame,
    MenuText,
    WindowText,
    CaptionText,
    ActiveBorder,
    InactiveBorder,
    ApplicationWorkspace,
    Highlight,
    HighlightText,
    ButtonFace,
    ButtonShadow,
    GrayText,
    ButtonText,
    InactiveCaptionText,
    ButtonHighlight,
    ThreeDDarkShadow,
    ThreeDLight,
    InfoText,
    InfoBack,
    HotLight,
    GradientActiveCaption,
    GradientInactiveCaption,
    MenuHighlight,
    MenuBar,
}

impl SystemColorValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ScrollBar => "scrollBar",
            Self::Background => "background",
            Self::ActiveCaption => "activeCaption",
            Self::InactiveCaption => "inactiveCaption",
            Self::Menu => "menu",
            Self::Window => "window",
            Self::WindowFrame => "windowFrame",
            Self::MenuText => "menuText",
            Self::WindowText => "windowText",
            Self::CaptionText => "captionText",
            Self::ActiveBorder => "activeBorder",
            Self::InactiveBorder => "inactiveBorder",
            Self::ApplicationWorkspace => "appWorkspace",
            Self::Highlight => "highlight",
            Self::HighlightText => "highlightText",
            Self::ButtonFace => "btnFace",
            Self::ButtonShadow => "btnShadow",
            Self::GrayText => "grayText",
            Self::ButtonText => "btnText",
            Self::InactiveCaptionText => "inactiveCaptionText",
            Self::ButtonHighlight => "btnHighlight",
            Self::ThreeDDarkShadow => "3dDkShadow",
            Self::ThreeDLight => "3dLight",
            Self::InfoText => "infoText",
            Self::InfoBack => "infoBk",
            Self::HotLight => "hotLight",
            Self::GradientActiveCaption => "gradientActiveCaption",
            Self::GradientInactiveCaption => "gradientInactiveCaption",
            Self::MenuHighlight => "menuHighlight",
            Self::MenuBar => "menuBar",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "scrollBar" => Some(Self::ScrollBar),
            "background" => Some(Self::Background),
            "activeCaption" => Some(Self::ActiveCaption),
            "inactiveCaption" => Some(Self::InactiveCaption),
            "menu" => Some(Self::Menu),
            "window" => Some(Self::Window),
            "windowFrame" => Some(Self::WindowFrame),
            "menuText" => Some(Self::MenuText),
            "windowText" => Some(Self::WindowText),
            "captionText" => Some(Self::CaptionText),
            "activeBorder" => Some(Self::ActiveBorder),
            "inactiveBorder" => Some(Self::InactiveBorder),
            "appWorkspace" => Some(Self::ApplicationWorkspace),
            "highlight" => Some(Self::Highlight),
            "highlightText" => Some(Self::HighlightText),
            "btnFace" => Some(Self::ButtonFace),
            "btnShadow" => Some(Self::ButtonShadow),
            "grayText" => Some(Self::GrayText),
            "btnText" => Some(Self::ButtonText),
            "inactiveCaptionText" => Some(Self::InactiveCaptionText),
            "btnHighlight" => Some(Self::ButtonHighlight),
            "3dDkShadow" => Some(Self::ThreeDDarkShadow),
            "3dLight" => Some(Self::ThreeDLight),
            "infoText" => Some(Self::InfoText),
            "infoBk" => Some(Self::InfoBack),
            "hotLight" => Some(Self::HotLight),
            "gradientActiveCaption" => Some(Self::GradientActiveCaption),
            "gradientInactiveCaption" => Some(Self::GradientInactiveCaption),
            "menuHighlight" => Some(Self::MenuHighlight),
            "menuBar" => Some(Self::MenuBar),
            _ => None,
        }
    }
}

impl core::fmt::Display for SystemColorValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for SystemColorValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Scheme Color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchemeColorValues {
    Background1,
    Text1,
    Background2,
    Text2,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Hyperlink,
    FollowedHyperlink,
    PhColor,
    Dark1,
    Light1,
    Dark2,
    Light2,
}

impl SchemeColorValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Background1 => "bg1",
            Self::Text1 => "tx1",
            Self::Background2 => "bg2",
            Self::Text2 => "tx2",
            Self::Accent1 => "accent1",
            Self::Accent2 => "accent2",
            Self::Accent3 => "accent3",
            Self::Accent4 => "accent4",
            Self::Accent5 => "accent5",
            Self::Accent6 => "accent6",
            Self::Hyperlink => "hlink",
            Self::FollowedHyperlink => "folHlink",
            Self::PhColor => "phClr",
            Self::Dark1 => "dk1",
            Self::Light1 => "lt1",
            Self::Dark2 => "dk2",
            Self::Light2 => "lt2",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "bg1" => Some(Self::Background1),
            "tx1" => Some(Self::Text1),
            "bg2" => Some(Self::Background2),
            "tx2" => Some(Self::Text2),
            "accent1" => Some(Self::Accent1),
            "accent2" => Some(Self::Accent2),
            "accent3" => Some(Self::Accent3),
            "accent4" => Some(Self::Accent4),
            "accent5" => Some(Self::Accent5),
            "accent6" => Some(Self::Accent6),
            "hlink" => Some(Self::Hyperlink),
            "folHlink" => Some(Self::FollowedHyperlink),
            "phClr" => Some(Self::PhColor),
            "dk1" => Some(Self::Dark1),
            "lt1" => Some(Self::Light1),
            "dk2" => Some(Self::Dark2),
            "lt2" => Some(Self::Light2),
            _ => None,
        }
    }
}

impl core::fmt::Display for SchemeColorValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for SchemeColorValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Rectangle Alignments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RectangleAlignmentValues {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl RectangleAlignmentValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TopLeft => "tl",
            Self::Top => "t",
            Self::TopRight => "tr",
            Self::Left => "l",
            Self::Center => "ctr",
            Self::Right => "r",
            Self::BottomLeft => "bl",
            Self::Bottom => "b",
            Self::BottomRight => "br",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "tl" => Some(Self::TopLeft),
            "t" => Some(Self::Top),
            "tr" => Some(Self::TopRight),
            "l" => Some(Self::Left),
            "ctr" => Some(Self::Center),
            "r" => Some(Self::Right),
            "bl" => Some(Self::BottomLeft),
            "b" => Some(Self::Bottom),
            "br" => Some(Self::BottomRight),
            _ => None,
        }
    }
}

impl core::fmt::Display for RectangleAlignmentValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for RectangleAlignmentValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Black and White Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlackWhiteModeValues {
    Color,
    Auto,
    Gray,
    LightGray,
    InvGray,
    GrayWhite,
    BlackGray,
    BlackWhite,
    Black,
    White,
    Hidden,
}

impl BlackWhiteModeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Color => "clr",
            Self::Auto => "auto",
            Self::Gray => "gray",
            Self::LightGray => "ltGray",
            Self::InvGray => "invGray",
            Self::GrayWhite => "grayWhite",
            Self::BlackGray => "blackGray",
            Self::BlackWhite => "blackWhite",
            Self::Black => "black",
            Self::White => "white",
            Self::Hidden => "hidden",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "clr" => Some(Self::Color),
            "auto" => Some(Self::Auto),
            "gray" => Some(Self::Gray),
            "ltGray" => Some(Self::LightGray),
            "invGray" => Some(Self::InvGray),
            "grayWhite" => Some(Self::GrayWhite),
            "blackGray" => Some(Self::BlackGray),
            "blackWhite" => Some(Self::BlackWhite),
            "black" => Some(Self::Black),
            "white" => Some(Self::White),
            "hidden" => Some(Self::Hidden),
            _ => None,
        }
    }
}

impl core::fmt::Display for BlackWhiteModeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BlackWhiteModeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Chart Animation Build Step
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChartBuildStepValues {
    Category,
    CategoryPoints,
    Series,
    SeriesPoints,
    AllPoints,
    GridLegend,
}

impl ChartBuildStepValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::CategoryPoints => "ptInCategory",
            Self::Series => "series",
            Self::SeriesPoints => "ptInSeries",
            Self::AllPoints => "allPts",
            Self::GridLegend => "gridLegend",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "category" => Some(Self::Category),
            "ptInCategory" => Some(Self::CategoryPoints),
            "series" => Some(Self::Series),
            "ptInSeries" => Some(Self::SeriesPoints),
            "allPts" => Some(Self::AllPoints),
            "gridLegend" => Some(Self::GridLegend),
            _ => None,
        }
    }
}

impl core::fmt::Display for ChartBuildStepValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for ChartBuildStepValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Diagram Animation Build Steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagramBuildStepValues {
    Shape,
    Background,
}

impl DiagramBuildStepValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shape => "sp",
            Self::Background => "bg",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sp" => Some(Self::Shape),
            "bg" => Some(Self::Background),
            _ => None,
        }
    }
}

impl core::fmt::Display for DiagramBuildStepValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for DiagramBuildStepValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Animation Build Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationBuildValues {
    AllAtOnce,
}

impl AnimationBuildValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllAtOnce => "allAtOnce",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allAtOnce" => Some(Self::AllAtOnce),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimationBuildValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimationBuildValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Diagram only Animation Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationDiagramOnlyBuildValues {
    One,
    LevelOne,
    LevelAtOnce,
}

impl AnimationDiagramOnlyBuildValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::One => "one",
            Self::LevelOne => "lvlOne",
            Self::LevelAtOnce => "lvlAtOnce",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "one" => Some(Self::One),
            "lvlOne" => Some(Self::LevelOne),
            "lvlAtOnce" => Some(Self::LevelAtOnce),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimationDiagramOnlyBuildValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimationDiagramOnlyBuildValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Chart only Animation Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationChartOnlyBuildValues {
    Series,
    Category,
    SeriesElement,
    CategoryElement,
}

impl AnimationChartOnlyBuildValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Series => "series",
            Self::Category => "category",
            Self::SeriesElement => "seriesEl",
            Self::CategoryElement => "categoryEl",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "series" => Some(Self::Series),
            "category" => Some(Self::Category),
            "seriesEl" => Some(Self::SeriesElement),
            "categoryEl" => Some(Self::CategoryElement),
            _ => None,
        }
    }
}

impl core::fmt::Display for AnimationChartOnlyBuildValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for AnimationChartOnlyBuildValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Camera Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetCameraValues {
    LegacyObliqueTopLeft,
    LegacyObliqueTop,
    LegacyObliqueTopRight,
    LegacyObliqueLeft,
    LegacyObliqueFront,
    LegacyObliqueRight,
    LegacyObliqueBottomLeft,
    LegacyObliqueBottom,
    LegacyObliqueBottomRight,
    LegacyPerspectiveTopLeft,
    LegacyPerspectiveTop,
    LegacyPerspectiveTopRight,
    LegacyPerspectiveLeft,
    LegacyPerspectiveFront,
    LegacyPerspectiveRight,
    LegacyPerspectiveBottomLeft,
    LegacyPerspectiveBottom,
    LegacyPerspectiveBottomRight,
    OrthographicFront,
    IsometricTopUp,
    IsometricTopDown,
    IsometricBottomUp,
    IsometricBottomDown,
    IsometricLeftUp,
    IsometricLeftDown,
    IsometricRightUp,
    IsometricRightDown,
    IsometricOffAxis1Left,
    IsometricOffAxis1Right,
    IsometricOffAxis1Top,
    IsometricOffAxis2Left,
    IsometricOffAxis2Right,
    IsometricOffAxis2Top,
    IsometricOffAxis3Left,
    IsometricOffAxis3Right,
    IsometricOffAxis3Bottom,
    IsometricOffAxis4Left,
    IsometricOffAxis4Right,
    IsometricOffAxis4Bottom,
    ObliqueTopLeft,
    ObliqueTop,
    ObliqueTopRight,
    ObliqueLeft,
    ObliqueRight,
    ObliqueBottomLeft,
    ObliqueBottom,
    ObliqueBottomRight,
    PerspectiveFront,
    PerspectiveLeft,
    PerspectiveRight,
    PerspectiveAbove,
    PerspectiveBelow,
    PerspectiveAboveLeftFacing,
    PerspectiveAboveRightFacing,
    PerspectiveContrastingLeftFacing,
    PerspectiveContrastingRightFacing,
    PerspectiveHeroicLeftFacing,
    PerspectiveHeroicRightFacing,
    PerspectiveHeroicExtremeLeftFacing,
    PerspectiveHeroicExtremeRightFacing,
    PerspectiveRelaxed,
    PerspectiveRelaxedModerately,
}

impl PresetCameraValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LegacyObliqueTopLeft => "legacyObliqueTopLeft",
            Self::LegacyObliqueTop => "legacyObliqueTop",
            Self::LegacyObliqueTopRight => "legacyObliqueTopRight",
            Self::LegacyObliqueLeft => "legacyObliqueLeft",
            Self::LegacyObliqueFront => "legacyObliqueFront",
            Self::LegacyObliqueRight => "legacyObliqueRight",
            Self::LegacyObliqueBottomLeft => "legacyObliqueBottomLeft",
            Self::LegacyObliqueBottom => "legacyObliqueBottom",
            Self::LegacyObliqueBottomRight => "legacyObliqueBottomRight",
            Self::LegacyPerspectiveTopLeft => "legacyPerspectiveTopLeft",
            Self::LegacyPerspectiveTop => "legacyPerspectiveTop",
            Self::LegacyPerspectiveTopRight => "legacyPerspectiveTopRight",
            Self::LegacyPerspectiveLeft => "legacyPerspectiveLeft",
            Self::LegacyPerspectiveFront => "legacyPerspectiveFront",
            Self::LegacyPerspectiveRight => "legacyPerspectiveRight",
            Self::LegacyPerspectiveBottomLeft => "legacyPerspectiveBottomLeft",
            Self::LegacyPerspectiveBottom => "legacyPerspectiveBottom",
            Self::LegacyPerspectiveBottomRight => "legacyPerspectiveBottomRight",
            Self::OrthographicFront => "orthographicFront",
            Self::IsometricTopUp => "isometricTopUp",
            Self::IsometricTopDown => "isometricTopDown",
            Self::IsometricBottomUp => "isometricBottomUp",
            Self::IsometricBottomDown => "isometricBottomDown",
            Self::IsometricLeftUp => "isometricLeftUp",
            Self::IsometricLeftDown => "isometricLeftDown",
            Self::IsometricRightUp => "isometricRightUp",
            Self::IsometricRightDown => "isometricRightDown",
            Self::IsometricOffAxis1Left => "isometricOffAxis1Left",
            Self::IsometricOffAxis1Right => "isometricOffAxis1Right",
            Self::IsometricOffAxis1Top => "isometricOffAxis1Top",
            Self::IsometricOffAxis2Left => "isometricOffAxis2Left",
            Self::IsometricOffAxis2Right => "isometricOffAxis2Right",
            Self::IsometricOffAxis2Top => "isometricOffAxis2Top",
            Self::IsometricOffAxis3Left => "isometricOffAxis3Left",
            Self::IsometricOffAxis3Right => "isometricOffAxis3Right",
            Self::IsometricOffAxis3Bottom => "isometricOffAxis3Bottom",
            Self::IsometricOffAxis4Left => "isometricOffAxis4Left",
            Self::IsometricOffAxis4Right => "isometricOffAxis4Right",
            Self::IsometricOffAxis4Bottom => "isometricOffAxis4Bottom",
            Self::ObliqueTopLeft => "obliqueTopLeft",
            Self::ObliqueTop => "obliqueTop",
            Self::ObliqueTopRight => "obliqueTopRight",
            Self::ObliqueLeft => "obliqueLeft",
            Self::ObliqueRight => "obliqueRight",
            Self::ObliqueBottomLeft => "obliqueBottomLeft",
            Self::ObliqueBottom => "obliqueBottom",
            Self::ObliqueBottomRight => "obliqueBottomRight",
            Self::PerspectiveFront => "perspectiveFront",
            Self::PerspectiveLeft => "perspectiveLeft",
            Self::PerspectiveRight => "perspectiveRight",
            Self::PerspectiveAbove => "perspectiveAbove",
            Self::PerspectiveBelow => "perspectiveBelow",
            Self::PerspectiveAboveLeftFacing => "perspectiveAboveLeftFacing",
            Self::PerspectiveAboveRightFacing => "perspectiveAboveRightFacing",
            Self::PerspectiveContrastingLeftFacing => "perspectiveContrastingLeftFacing",
            Self::PerspectiveContrastingRightFacing => "perspectiveContrastingRightFacing",
            Self::PerspectiveHeroicLeftFacing => "perspectiveHeroicLeftFacing",
            Self::PerspectiveHeroicRightFacing => "perspectiveHeroicRightFacing",
            Self::PerspectiveHeroicExtremeLeftFacing => "perspectiveHeroicExtremeLeftFacing",
            Self::PerspectiveHeroicExtremeRightFacing => "perspectiveHeroicExtremeRightFacing",
            Self::PerspectiveRelaxed => "perspectiveRelaxed",
            Self::PerspectiveRelaxedModerately => "perspectiveRelaxedModerately",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "legacyObliqueTopLeft" => Some(Self::LegacyObliqueTopLeft),
            "legacyObliqueTop" => Some(Self::LegacyObliqueTop),
            "legacyObliqueTopRight" => Some(Self::LegacyObliqueTopRight),
            "legacyObliqueLeft" => Some(Self::LegacyObliqueLeft),
            "legacyObliqueFront" => Some(Self::LegacyObliqueFront),
            "legacyObliqueRight" => Some(Self::LegacyObliqueRight),
            "legacyObliqueBottomLeft" => Some(Self::LegacyObliqueBottomLeft),
            "legacyObliqueBottom" => Some(Self::LegacyObliqueBottom),
            "legacyObliqueBottomRight" => Some(Self::LegacyObliqueBottomRight),
            "legacyPerspectiveTopLeft" => Some(Self::LegacyPerspectiveTopLeft),
            "legacyPerspectiveTop" => Some(Self::LegacyPerspectiveTop),
            "legacyPerspectiveTopRight" => Some(Self::LegacyPerspectiveTopRight),
            "legacyPerspectiveLeft" => Some(Self::LegacyPerspectiveLeft),
            "legacyPerspectiveFront" => Some(Self::LegacyPerspectiveFront),
            "legacyPerspectiveRight" => Some(Self::LegacyPerspectiveRight),
            "legacyPerspectiveBottomLeft" => Some(Self::LegacyPerspectiveBottomLeft),
            "legacyPerspectiveBottom" => Some(Self::LegacyPerspectiveBottom),
            "legacyPerspectiveBottomRight" => Some(Self::LegacyPerspectiveBottomRight),
            "orthographicFront" => Some(Self::OrthographicFront),
            "isometricTopUp" => Some(Self::IsometricTopUp),
            "isometricTopDown" => Some(Self::IsometricTopDown),
            "isometricBottomUp" => Some(Self::IsometricBottomUp),
            "isometricBottomDown" => Some(Self::IsometricBottomDown),
            "isometricLeftUp" => Some(Self::IsometricLeftUp),
            "isometricLeftDown" => Some(Self::IsometricLeftDown),
            "isometricRightUp" => Some(Self::IsometricRightUp),
            "isometricRightDown" => Some(Self::IsometricRightDown),
            "isometricOffAxis1Left" => Some(Self::IsometricOffAxis1Left),
            "isometricOffAxis1Right" => Some(Self::IsometricOffAxis1Right),
            "isometricOffAxis1Top" => Some(Self::IsometricOffAxis1Top),
            "isometricOffAxis2Left" => Some(Self::IsometricOffAxis2Left),
            "isometricOffAxis2Right" => Some(Self::IsometricOffAxis2Right),
            "isometricOffAxis2Top" => Some(Self::IsometricOffAxis2Top),
            "isometricOffAxis3Left" => Some(Self::IsometricOffAxis3Left),
            "isometricOffAxis3Right" => Some(Self::IsometricOffAxis3Right),
            "isometricOffAxis3Bottom" => Some(Self::IsometricOffAxis3Bottom),
            "isometricOffAxis4Left" => Some(Self::IsometricOffAxis4Left),
            "isometricOffAxis4Right" => Some(Self::IsometricOffAxis4Right),
            "isometricOffAxis4Bottom" => Some(Self::IsometricOffAxis4Bottom),
            "obliqueTopLeft" => Some(Self::ObliqueTopLeft),
            "obliqueTop" => Some(Self::ObliqueTop),
            "obliqueTopRight" => Some(Self::ObliqueTopRight),
            "obliqueLeft" => Some(Self::ObliqueLeft),
            "obliqueRight" => Some(Self::ObliqueRight),
            "obliqueBottomLeft" => Some(Self::ObliqueBottomLeft),
            "obliqueBottom" => Some(Self::ObliqueBottom),
            "obliqueBottomRight" => Some(Self::ObliqueBottomRight),
            "perspectiveFront" => Some(Self::PerspectiveFront),
            "perspectiveLeft" => Some(Self::PerspectiveLeft),
            "perspectiveRight" => Some(Self::PerspectiveRight),
            "perspectiveAbove" => Some(Self::PerspectiveAbove),
            "perspectiveBelow" => Some(Self::PerspectiveBelow),
            "perspectiveAboveLeftFacing" => Some(Self::PerspectiveAboveLeftFacing),
            "perspectiveAboveRightFacing" => Some(Self::PerspectiveAboveRightFacing),
            "perspectiveContrastingLeftFacing" => Some(Self::PerspectiveContrastingLeftFacing),
            "perspectiveContrastingRightFacing" => Some(Self::PerspectiveContrastingRightFacing),
            "perspectiveHeroicLeftFacing" => Some(Self::PerspectiveHeroicLeftFacing),
            "perspectiveHeroicRightFacing" => Some(Self::PerspectiveHeroicRightFacing),
            "perspectiveHeroicExtremeLeftFacing" => Some(Self::PerspectiveHeroicExtremeLeftFacing),
            "perspectiveHeroicExtremeRightFacing" => Some(Self::PerspectiveHeroicExtremeRightFacing),
            "perspectiveRelaxed" => Some(Self::PerspectiveRelaxed),
            "perspectiveRelaxedModerately" => Some(Self::PerspectiveRelaxedModerately),
            _ => None,
        }
    }
}

impl core::fmt::Display for PresetCameraValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PresetCameraValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Light Rig Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LightRigDirectionValues {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl LightRigDirectionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TopLeft => "tl",
            Self::Top => "t",
            Self::TopRight => "tr",
            Self::Left => "l",
            Self::Right => "r",
            Self::BottomLeft => "bl",
            Self::Bottom => "b",
            Self::BottomRight => "br",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "tl" => Some(Self::TopLeft),
            "t" => Some(Self::Top),
            "tr" => Some(Self::TopRight),
            "l" => Some(Self::Left),
            "r" => Some(Self::Right),
            "bl" => Some(Self::BottomLeft),
            "b" => Some(Self::Bottom),
            "br" => Some(Self::BottomRight),
            _ => None,
        }
    }
}

impl core::fmt::Display for LightRigDirectionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for LightRigDirectionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Light Rig Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LightRigValues {
    LegacyFlat1,
    LegacyFlat2,
    LegacyFlat3,
    LegacyFlat4,
    LegacyNormal1,
    LegacyNormal2,
    LegacyNormal3,
    LegacyNormal4,
    LegacyHarsh1,
    LegacyHarsh2,
    LegacyHarsh3,
    LegacyHarsh4,
    ThreePoints,
    Balanced,
    Soft,
    Harsh,
    Flood,
    Contrasting,
    Morning,
    Sunrise,
    Sunset,
    Chilly,
    Freezing,
    Flat,
    TwoPoints,
    Glow,
    BrightRoom,
}

impl LightRigValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LegacyFlat1 => "legacyFlat1",
            Self::LegacyFlat2 => "legacyFlat2",
            Self::LegacyFlat3 => "legacyFlat3",
            Self::LegacyFlat4 => "legacyFlat4",
            Self::LegacyNormal1 => "legacyNormal1",
            Self::LegacyNormal2 => "legacyNormal2",
            Self::LegacyNormal3 => "legacyNormal3",
            Self::LegacyNormal4 => "legacyNormal4",
            Self::LegacyHarsh1 => "legacyHarsh1",
            Self::LegacyHarsh2 => "legacyHarsh2",
            Self::LegacyHarsh3 => "legacyHarsh3",
            Self::LegacyHarsh4 => "legacyHarsh4",
            Self::ThreePoints => "threePt",
            Self::Balanced => "balanced",
            Self::Soft => "soft",
            Self::Harsh => "harsh",
            Self::Flood => "flood",
            Self::Contrasting => "contrasting",
            Self::Morning => "morning",
            Self::Sunrise => "sunrise",
            Self::Sunset => "sunset",
            Self::Chilly => "chilly",
            Self::Freezing => "freezing",
            Self::Flat => "flat",
            Self::TwoPoints => "twoPt",
            Self::Glow => "glow",
            Self::BrightRoom => "brightRoom",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "legacyFlat1" => Some(Self::LegacyFlat1),
            "legacyFlat2" => Some(Self::LegacyFlat2),
            "legacyFlat3" => Some(Self::LegacyFlat3),
            "legacyFlat4" => Some(Self::LegacyFlat4),
            "legacyNormal1" => Some(Self::LegacyNormal1),
            "legacyNormal2" => Some(Self::LegacyNormal2),
            "legacyNormal3" => Some(Self::LegacyNormal3),
            "legacyNormal4" => Some(Self::LegacyNormal4),
            "legacyHarsh1" => Some(Self::LegacyHarsh1),
            "legacyHarsh2" => Some(Self::LegacyHarsh2),
            "legacyHarsh3" => Some(Self::LegacyHarsh3),
            "legacyHarsh4" => Some(Self::LegacyHarsh4),
            "threePt" => Some(Self::ThreePoints),
            "balanced" => Some(Self::Balanced),
            "soft" => Some(Self::Soft),
            "harsh" => Some(Self::Harsh),
            "flood" => Some(Self::Flood),
            "contrasting" => Some(Self::Contrasting),
            "morning" => Some(Self::Morning),
            "sunrise" => Some(Self::Sunrise),
            "sunset" => Some(Self::Sunset),
            "chilly" => Some(Self::Chilly),
            "freezing" => Some(Self::Freezing),
            "flat" => Some(Self::Flat),
            "twoPt" => Some(Self::TwoPoints),
            "glow" => Some(Self::Glow),
            "brightRoom" => Some(Self::BrightRoom),
            _ => None,
        }
    }
}

impl core::fmt::Display for LightRigValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for LightRigValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Bevel Presets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BevelPresetValues {
    RelaxedInset,
    Circle,
    Slope,
    Cross,
    Angle,
    SoftRound,
    Convex,
    CoolSlant,
    Divot,
    Riblet,
    HardEdge,
    ArtDeco,
}

impl BevelPresetValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RelaxedInset => "relaxedInset",
            Self::Circle => "circle",
            Self::Slope => "slope",
            Self::Cross => "cross",
            Self::Angle => "angle",
            Self::SoftRound => "softRound",
            Self::Convex => "convex",
            Self::CoolSlant => "coolSlant",
            Self::Divot => "divot",
            Self::Riblet => "riblet",
            Self::HardEdge => "hardEdge",
            Self::ArtDeco => "artDeco",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "relaxedInset" => Some(Self::RelaxedInset),
            "circle" => Some(Self::Circle),
            "slope" => Some(Self::Slope),
            "cross" => Some(Self::Cross),
            "angle" => Some(Self::Angle),
            "softRound" => Some(Self::SoftRound),
            "convex" => Some(Self::Convex),
            "coolSlant" => Some(Self::CoolSlant),
            "divot" => Some(Self::Divot),
            "riblet" => Some(Self::Riblet),
            "hardEdge" => Some(Self::HardEdge),
            "artDeco" => Some(Self::ArtDeco),
            _ => None,
        }
    }
}

impl core::fmt::Display for BevelPresetValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BevelPresetValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Material Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetMaterialTypeValues {
    LegacyMatte,
    LegacyPlastic,
    LegacyMetal,
    LegacyWireframe,
    Matte,
    Plastic,
    Metal,
    WarmMatte,
    TranslucentPowder,
    Powder,
    DarkEdge,
    SoftEdge,
    Clear,
    Flat,
    SoftMetal,
}

impl PresetMaterialTypeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LegacyMatte => "legacyMatte",
            Self::LegacyPlastic => "legacyPlastic",
            Self::LegacyMetal => "legacyMetal",
            Self::LegacyWireframe => "legacyWireframe",
            Self::Matte => "matte",
            Self::Plastic => "plastic",
            Self::Metal => "metal",
            Self::WarmMatte => "warmMatte",
            Self::TranslucentPowder => "translucentPowder",
            Self::Powder => "powder",
            Self::DarkEdge => "dkEdge",
            Self::SoftEdge => "softEdge",
            Self::Clear => "clear",
            Self::Flat => "flat",
            Self::SoftMetal => "softmetal",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "legacyMatte" => Some(Self::LegacyMatte),
            "legacyPlastic" => Some(Self::LegacyPlastic),
            "legacyMetal" => Some(Self::LegacyMetal),
            "legacyWireframe" => Some(Self::LegacyWireframe),
            "matte" => Some(Self::Matte),
            "plastic" => Some(Self::Plastic),
            "metal" => Some(Self::Metal),
            "warmMatte" => Some(Self::WarmMatte),
            "translucentPowder" => Some(Self::TranslucentPowder),
            "powder" => Some(Self::Powder),
            "dkEdge" => Some(Self::DarkEdge),
            "softEdge" => Some(Self::SoftEdge),
            "clear" => Some(Self::Clear),
            "flat" => Some(Self::Flat),
            "softmetal" => Some(Self::SoftMetal),
            _ => None,
        }
    }
}

impl core::fmt::Display for PresetMaterialTypeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PresetMaterialTypeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Shadow Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetShadowValues {
    TopLeftDropShadow,
    TopRightDropShadow,
    BackLeftPerspectiveShadow,
    BackRightPerspectiveShadow,
    BottomLeftDropShadow,
    BottomRightDropShadow,
    FrontLeftPerspectiveShadow,
    FrontRightPerspectiveShadow,
    TopLeftSmallDropShadow,
    TopLeftLargeDropShadow,
    BackLeftLongPerspectiveShadow,
    BackRightLongPerspectiveShadow,
    TopLeftDoubleDropShadow,
    BottomRightSmallDropShadow,
    FrontLeftLongPerspectiveShadow,
    FrontRightLongPerspectiveShadow,
    ThreeDimensionalOuterBoxShadow,
    ThreeDimensionalInnerBoxShadow,
    BackCenterPerspectiveShadow,
    FrontBottomShadow,
}

impl PresetShadowValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TopLeftDropShadow => "shdw1",
            Self::TopRightDropShadow => "shdw2",
            Self::BackLeftPerspectiveShadow => "shdw3",
            Self::BackRightPerspectiveShadow => "shdw4",
            Self::BottomLeftDropShadow => "shdw5",
            Self::BottomRightDropShadow => "shdw6",
            Self::FrontLeftPerspectiveShadow => "shdw7",
            Self::FrontRightPerspectiveShadow => "shdw8",
            Self::TopLeftSmallDropShadow => "shdw9",
            Self::TopLeftLargeDropShadow => "shdw10",
            Self::BackLeftLongPerspectiveShadow => "shdw11",
            Self::BackRightLongPerspectiveShadow => "shdw12",
            Self::TopLeftDoubleDropShadow => "shdw13",
            Self::BottomRightSmallDropShadow => "shdw14",
            Self::FrontLeftLongPerspectiveShadow => "shdw15",
            Self::FrontRightLongPerspectiveShadow => "shdw16",
            Self::ThreeDimensionalOuterBoxShadow => "shdw17",
            Self::ThreeDimensionalInnerBoxShadow => "shdw18",
            Self::BackCenterPerspectiveShadow => "shdw19",
            Self::FrontBottomShadow => "shdw20",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "shdw1" => Some(Self::TopLeftDropShadow),
            "shdw2" => Some(Self::TopRightDropShadow),
            "shdw3" => Some(Self::BackLeftPerspectiveShadow),
            "shdw4" => Some(Self::BackRightPerspectiveShadow),
            "shdw5" => Some(Self::BottomLeftDropShadow),
            "shdw6" => Some(Self::BottomRightDropShadow),
            "shdw7" => Some(Self::FrontLeftPerspectiveShadow),
            "shdw8" => Some(Self::FrontRightPerspectiveShadow),
            "shdw9" => Some(Self::TopLeftSmallDropShadow),
            "shdw10" => Some(Self::TopLeftLargeDropShadow),
            "shdw11" => Some(Self::BackLeftLongPerspectiveShadow),
            "shdw12" => Some(Self::BackRightLongPerspectiveShadow),
            "shdw13" => Some(Self::TopLeftDoubleDropShadow),
            "shdw14" => Some(Self::BottomRightSmallDropShadow),
            "shdw15" => Some(Self::FrontLeftLongPerspectiveShadow),
            "shdw16" => Some(Self::FrontRightLongPerspectiveShadow),
            "shdw17" => Some(Self::ThreeDimensionalOuterBoxShadow),
            "shdw18" => Some(Self::ThreeDimensionalInnerBoxShadow),
            "shdw19" => Some(Self::BackCenterPerspectiveShadow),
            "shdw20" => Some(Self::FrontBottomShadow),
            _ => None,
        }
    }
}

impl core::fmt::Display for PresetShadowValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PresetShadowValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Path Shade Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathShadeValues {
    Shape,
    Circle,
    Rectangle,
}

impl PathShadeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shape => "shape",
            Self::Circle => "circle",
            Self::Rectangle => "rect",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "shape" => Some(Self::Shape),
            "circle" => Some(Self::Circle),
            "rect" => Some(Self::Rectangle),
            _ => None,
        }
    }
}

impl core::fmt::Display for PathShadeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PathShadeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Tile Flip Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileFlipValues {
    None_,
    Horizontal,
    Vertical,
    HorizontalAndVertical,
}

impl TileFlipValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Horizontal => "x",
            Self::Vertical => "y",
            Self::HorizontalAndVertical => "xy",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "x" => Some(Self::Horizontal),
            "y" => Some(Self::Vertical),
            "xy" => Some(Self::HorizontalAndVertical),
            _ => None,
        }
    }
}

impl core::fmt::Display for TileFlipValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TileFlipValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Blip Compression Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlipCompressionValues {
    Email,
    Screen,
    Print,
    HighQualityPrint,
    None_,
}

impl BlipCompressionValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Screen => "screen",
            Self::Print => "print",
            Self::HighQualityPrint => "hqprint",
            Self::None_ => "none",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "email" => Some(Self::Email),
            "screen" => Some(Self::Screen),
            "print" => Some(Self::Print),
            "hqprint" => Some(Self::HighQualityPrint),
            "none" => Some(Self::None_),
            _ => None,
        }
    }
}

impl core::fmt::Display for BlipCompressionValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BlipCompressionValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Pattern Value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetPatternValues {
    Percent5,
    Percent10,
    Percent20,
    Percent25,
    Percent30,
    Percent40,
    Percent50,
    Percent60,
    Percent70,
    Percent75,
    Percent80,
    Percent90,
    Horizontal,
    Vertical,
    LightHorizontal,
    LightVertical,
    DarkHorizontal,
    DarkVertical,
    NarrowHorizontal,
    NarrowVertical,
    DashedHorizontal,
    DashedVertical,
    Cross,
    DownwardDiagonal,
    UpwardDiagonal,
    LightDownwardDiagonal,
    LightUpwardDiagonal,
    DarkDownwardDiagonal,
    DarkUpwardDiagonal,
    WideDownwardDiagonal,
    WideUpwardDiagonal,
    DashedDownwardDiagonal,
    DashedUpwardDiagonal,
    DiagonalCross,
    SmallCheck,
    LargeCheck,
    SmallGrid,
    LargeGrid,
    DotGrid,
    SmallConfetti,
    LargeConfetti,
    HorizontalBrick,
    DiagonalBrick,
    SolidDiamond,
    OpenDiamond,
    DottedDiamond,
    Plaid,
    Sphere,
    Weave,
    Divot,
    Shingle,
    Wave,
    Trellis,
    ZigZag,
}

impl PresetPatternValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Percent5 => "pct5",
            Self::Percent10 => "pct10",
            Self::Percent20 => "pct20",
            Self::Percent25 => "pct25",
            Self::Percent30 => "pct30",
            Self::Percent40 => "pct40",
            Self::Percent50 => "pct50",
            Self::Percent60 => "pct60",
            Self::Percent70 => "pct70",
            Self::Percent75 => "pct75",
            Self::Percent80 => "pct80",
            Self::Percent90 => "pct90",
            Self::Horizontal => "horz",
            Self::Vertical => "vert",
            Self::LightHorizontal => "ltHorz",
            Self::LightVertical => "ltVert",
            Self::DarkHorizontal => "dkHorz",
            Self::DarkVertical => "dkVert",
            Self::NarrowHorizontal => "narHorz",
            Self::NarrowVertical => "narVert",
            Self::DashedHorizontal => "dashHorz",
            Self::DashedVertical => "dashVert",
            Self::Cross => "cross",
            Self::DownwardDiagonal => "dnDiag",
            Self::UpwardDiagonal => "upDiag",
            Self::LightDownwardDiagonal => "ltDnDiag",
            Self::LightUpwardDiagonal => "ltUpDiag",
            Self::DarkDownwardDiagonal => "dkDnDiag",
            Self::DarkUpwardDiagonal => "dkUpDiag",
            Self::WideDownwardDiagonal => "wdDnDiag",
            Self::WideUpwardDiagonal => "wdUpDiag",
            Self::DashedDownwardDiagonal => "dashDnDiag",
            Self::DashedUpwardDiagonal => "dashUpDiag",
            Self::DiagonalCross => "diagCross",
            Self::SmallCheck => "smCheck",
            Self::LargeCheck => "lgCheck",
            Self::SmallGrid => "smGrid",
            Self::LargeGrid => "lgGrid",
            Self::DotGrid => "dotGrid",
            Self::SmallConfetti => "smConfetti",
            Self::LargeConfetti => "lgConfetti",
            Self::HorizontalBrick => "horzBrick",
            Self::DiagonalBrick => "diagBrick",
            Self::SolidDiamond => "solidDmnd",
            Self::OpenDiamond => "openDmnd",
            Self::DottedDiamond => "dotDmnd",
            Self::Plaid => "plaid",
            Self::Sphere => "sphere",
            Self::Weave => "weave",
            Self::Divot => "divot",
            Self::Shingle => "shingle",
            Self::Wave => "wave",
            Self::Trellis => "trellis",
            Self::ZigZag => "zigZag",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pct5" => Some(Self::Percent5),
            "pct10" => Some(Self::Percent10),
            "pct20" => Some(Self::Percent20),
            "pct25" => Some(Self::Percent25),
            "pct30" => Some(Self::Percent30),
            "pct40" => Some(Self::Percent40),
            "pct50" => Some(Self::Percent50),
            "pct60" => Some(Self::Percent60),
            "pct70" => Some(Self::Percent70),
            "pct75" => Some(Self::Percent75),
            "pct80" => Some(Self::Percent80),
            "pct90" => Some(Self::Percent90),
            "horz" => Some(Self::Horizontal),
            "vert" => Some(Self::Vertical),
            "ltHorz" => Some(Self::LightHorizontal),
            "ltVert" => Some(Self::LightVertical),
            "dkHorz" => Some(Self::DarkHorizontal),
            "dkVert" => Some(Self::DarkVertical),
            "narHorz" => Some(Self::NarrowHorizontal),
            "narVert" => Some(Self::NarrowVertical),
            "dashHorz" => Some(Self::DashedHorizontal),
            "dashVert" => Some(Self::DashedVertical),
            "cross" => Some(Self::Cross),
            "dnDiag" => Some(Self::DownwardDiagonal),
            "upDiag" => Some(Self::UpwardDiagonal),
            "ltDnDiag" => Some(Self::LightDownwardDiagonal),
            "ltUpDiag" => Some(Self::LightUpwardDiagonal),
            "dkDnDiag" => Some(Self::DarkDownwardDiagonal),
            "dkUpDiag" => Some(Self::DarkUpwardDiagonal),
            "wdDnDiag" => Some(Self::WideDownwardDiagonal),
            "wdUpDiag" => Some(Self::WideUpwardDiagonal),
            "dashDnDiag" => Some(Self::DashedDownwardDiagonal),
            "dashUpDiag" => Some(Self::DashedUpwardDiagonal),
            "diagCross" => Some(Self::DiagonalCross),
            "smCheck" => Some(Self::SmallCheck),
            "lgCheck" => Some(Self::LargeCheck),
            "smGrid" => Some(Self::SmallGrid),
            "lgGrid" => Some(Self::LargeGrid),
            "dotGrid" => Some(Self::DotGrid),
            "smConfetti" => Some(Self::SmallConfetti),
            "lgConfetti" => Some(Self::LargeConfetti),
            "horzBrick" => Some(Self::HorizontalBrick),
            "diagBrick" => Some(Self::DiagonalBrick),
            "solidDmnd" => Some(Self::SolidDiamond),
            "openDmnd" => Some(Self::OpenDiamond),
            "dotDmnd" => Some(Self::DottedDiamond),
            "plaid" => Some(Self::Plaid),
            "sphere" => Some(Self::Sphere),
            "weave" => Some(Self::Weave),
            "divot" => Some(Self::Divot),
            "shingle" => Some(Self::Shingle),
            "wave" => Some(Self::Wave),
            "trellis" => Some(Self::Trellis),
            "zigZag" => Some(Self::ZigZag),
            _ => None,
        }
    }
}

impl core::fmt::Display for PresetPatternValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PresetPatternValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Blend Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendModeValues {
    Overlay,
    Multiply,
    Screen,
    Darken,
    Lighten,
}

impl BlendModeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Overlay => "over",
            Self::Multiply => "mult",
            Self::Screen => "screen",
            Self::Darken => "darken",
            Self::Lighten => "lighten",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "over" => Some(Self::Overlay),
            "mult" => Some(Self::Multiply),
            "screen" => Some(Self::Screen),
            "darken" => Some(Self::Darken),
            "lighten" => Some(Self::Lighten),
            _ => None,
        }
    }
}

impl core::fmt::Display for BlendModeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BlendModeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Effect Container Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectContainerValues {
    Sibling,
    Tree,
}

impl EffectContainerValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sibling => "sib",
            Self::Tree => "tree",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sib" => Some(Self::Sibling),
            "tree" => Some(Self::Tree),
            _ => None,
        }
    }
}

impl core::fmt::Display for EffectContainerValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for EffectContainerValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Shape Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeTypeValues {
    Line,
    LineInverse,
    Triangle,
    RightTriangle,
    Rectangle,
    Diamond,
    Parallelogram,
    Trapezoid,
    NonIsoscelesTrapezoid,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Decagon,
    Dodecagon,
    Star4,
    Star5,
    Star6,
    Star7,
    Star8,
    Star10,
    Star12,
    Star16,
    Star24,
    Star32,
    RoundRectangle,
    Round1Rectangle,
    Round2SameRectangle,
    Round2DiagonalRectangle,
    SnipRoundRectangle,
    Snip1Rectangle,
    Snip2SameRectangle,
    Snip2DiagonalRectangle,
    Plaque,
    Ellipse,
    Teardrop,
    HomePlate,
    Chevron,
    PieWedge,
    Pie,
    BlockArc,
    Donut,
    NoSmoking,
    RightArrow,
    LeftArrow,
    UpArrow,
    DownArrow,
    StripedRightArrow,
    NotchedRightArrow,
    BentUpArrow,
    LeftRightArrow,
    UpDownArrow,
    LeftUpArrow,
    LeftRightUpArrow,
    QuadArrow,
    LeftArrowCallout,
    RightArrowCallout,
    UpArrowCallout,
    DownArrowCallout,
    LeftRightArrowCallout,
    UpDownArrowCallout,
    QuadArrowCallout,
    BentArrow,
    UTurnArrow,
    CircularArrow,
    LeftCircularArrow,
    LeftRightCircularArrow,
    CurvedRightArrow,
    CurvedLeftArrow,
    CurvedUpArrow,
    CurvedDownArrow,
    SwooshArrow,
    Cube,
    Can,
    LightningBolt,
    Heart,
    Sun,
    Moon,
    SmileyFace,
    IrregularSeal1,
    IrregularSeal2,
    FoldedCorner,
    Bevel,
    Frame,
    HalfFrame,
    Corner,
    DiagonalStripe,
    Chord,
    Arc,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    BracketPair,
    BracePair,
    StraightConnector1,
    BentConnector2,
    BentConnector3,
    BentConnector4,
    BentConnector5,
    CurvedConnector2,
    CurvedConnector3,
    CurvedConnector4,
    CurvedConnector5,
    Callout1,
    Callout2,
    Callout3,
    AccentCallout1,
    AccentCallout2,
    AccentCallout3,
    BorderCallout1,
    BorderCallout2,
    BorderCallout3,
    AccentBorderCallout1,
    AccentBorderCallout2,
    AccentBorderCallout3,
    WedgeRectangleCallout,
    WedgeRoundRectangleCallout,
    WedgeEllipseCallout,
    CloudCallout,
    Cloud,
    Ribbon,
    Ribbon2,
    EllipseRibbon,
    EllipseRibbon2,
    LeftRightRibbon,
    VerticalScroll,
    HorizontalScroll,
    Wave,
    DoubleWave,
    Plus,
    FlowChartProcess,
    FlowChartDecision,
    FlowChartInputOutput,
    FlowChartPredefinedProcess,
    FlowChartInternalStorage,
    FlowChartDocument,
    FlowChartMultidocument,
    FlowChartTerminator,
    FlowChartPreparation,
    FlowChartManualInput,
    FlowChartManualOperation,
    FlowChartConnector,
    FlowChartPunchedCard,
    FlowChartPunchedTape,
    FlowChartSummingJunction,
    FlowChartOr,
    FlowChartCollate,
    FlowChartSort,
    FlowChartExtract,
    FlowChartMerge,
    FlowChartOfflineStorage,
    FlowChartOnlineStorage,
    FlowChartMagneticTape,
    FlowChartMagneticDisk,
    FlowChartMagneticDrum,
    FlowChartDisplay,
    FlowChartDelay,
    FlowChartAlternateProcess,
    FlowChartOffpageConnector,
    ActionButtonBlank,
    ActionButtonHome,
    ActionButtonHelp,
    ActionButtonInformation,
    ActionButtonForwardNext,
    ActionButtonBackPrevious,
    ActionButtonEnd,
    ActionButtonBeginning,
    ActionButtonReturn,
    ActionButtonDocument,
    ActionButtonSound,
    ActionButtonMovie,
    Gear6,
    Gear9,
    Funnel,
    MathPlus,
    MathMinus,
    MathMultiply,
    MathDivide,
    MathEqual,
    MathNotEqual,
    CornerTabs,
    SquareTabs,
    PlaqueTabs,
    ChartX,
    ChartStar,
    ChartPlus,
}

impl ShapeTypeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::LineInverse => "lineInv",
            Self::Triangle => "triangle",
            Self::RightTriangle => "rtTriangle",
            Self::Rectangle => "rect",
            Self::Diamond => "diamond",
            Self::Parallelogram => "parallelogram",
            Self::Trapezoid => "trapezoid",
            Self::NonIsoscelesTrapezoid => "nonIsoscelesTrapezoid",
            Self::Pentagon => "pentagon",
            Self::Hexagon => "hexagon",
            Self::Heptagon => "heptagon",
            Self::Octagon => "octagon",
            Self::Decagon => "decagon",
            Self::Dodecagon => "dodecagon",
            Self::Star4 => "star4",
            Self::Star5 => "star5",
            Self::Star6 => "star6",
            Self::Star7 => "star7",
            Self::Star8 => "star8",
            Self::Star10 => "star10",
            Self::Star12 => "star12",
            Self::Star16 => "star16",
            Self::Star24 => "star24",
            Self::Star32 => "star32",
            Self::RoundRectangle => "roundRect",
            Self::Round1Rectangle => "round1Rect",
            Self::Round2SameRectangle => "round2SameRect",
            Self::Round2DiagonalRectangle => "round2DiagRect",
            Self::SnipRoundRectangle => "snipRoundRect",
            Self::Snip1Rectangle => "snip1Rect",
            Self::Snip2SameRectangle => "snip2SameRect",
            Self::Snip2DiagonalRectangle => "snip2DiagRect",
            Self::Plaque => "plaque",
            Self::Ellipse => "ellipse",
            Self::Teardrop => "teardrop",
            Self::HomePlate => "homePlate",
            Self::Chevron => "chevron",
            Self::PieWedge => "pieWedge",
            Self::Pie => "pie",
            Self::BlockArc => "blockArc",
            Self::Donut => "donut",
            Self::NoSmoking => "noSmoking",
            Self::RightArrow => "rightArrow",
            Self::LeftArrow => "leftArrow",
            Self::UpArrow => "upArrow",
            Self::DownArrow => "downArrow",
            Self::StripedRightArrow => "stripedRightArrow",
            Self::NotchedRightArrow => "notchedRightArrow",
            Self::BentUpArrow => "bentUpArrow",
            Self::LeftRightArrow => "leftRightArrow",
            Self::UpDownArrow => "upDownArrow",
            Self::LeftUpArrow => "leftUpArrow",
            Self::LeftRightUpArrow => "leftRightUpArrow",
            Self::QuadArrow => "quadArrow",
            Self::LeftArrowCallout => "leftArrowCallout",
            Self::RightArrowCallout => "rightArrowCallout",
            Self::UpArrowCallout => "upArrowCallout",
            Self::DownArrowCallout => "downArrowCallout",
            Self::LeftRightArrowCallout => "leftRightArrowCallout",
            Self::UpDownArrowCallout => "upDownArrowCallout",
            Self::QuadArrowCallout => "quadArrowCallout",
            Self::BentArrow => "bentArrow",
            Self::UTurnArrow => "uturnArrow",
            Self::CircularArrow => "circularArrow",
            Self::LeftCircularArrow => "leftCircularArrow",
            Self::LeftRightCircularArrow => "leftRightCircularArrow",
            Self::CurvedRightArrow => "curvedRightArrow",
            Self::CurvedLeftArrow => "curvedLeftArrow",
            Self::CurvedUpArrow => "curvedUpArrow",
            Self::CurvedDownArrow => "curvedDownArrow",
            Self::SwooshArrow => "swooshArrow",
            Self::Cube => "cube",
            Self::Can => "can",
            Self::LightningBolt => "lightningBolt",
            Self::Heart => "heart",
            Self::Sun => "sun",
            Self::Moon => "moon",
            Self::SmileyFace => "smileyFace",
            Self::IrregularSeal1 => "irregularSeal1",
            Self::IrregularSeal2 => "irregularSeal2",
            Self::FoldedCorner => "foldedCorner",
            Self::Bevel => "bevel",
            Self::Frame => "frame",
            Self::HalfFrame => "halfFrame",
            Self::Corner => "corner",
            Self::DiagonalStripe => "diagStripe",
            Self::Chord => "chord",
            Self::Arc => "arc",
            Self::LeftBracket => "leftBracket",
            Self::RightBracket => "rightBracket",
            Self::LeftBrace => "leftBrace",
            Self::RightBrace => "rightBrace",
            Self::BracketPair => "bracketPair",
            Self::BracePair => "bracePair",
            Self::StraightConnector1 => "straightConnector1",
            Self::BentConnector2 => "bentConnector2",
            Self::BentConnector3 => "bentConnector3",
            Self::BentConnector4 => "bentConnector4",
            Self::BentConnector5 => "bentConnector5",
            Self::CurvedConnector2 => "curvedConnector2",
            Self::CurvedConnector3 => "curvedConnector3",
            Self::CurvedConnector4 => "curvedConnector4",
            Self::CurvedConnector5 => "curvedConnector5",
            Self::Callout1 => "callout1",
            Self::Callout2 => "callout2",
            Self::Callout3 => "callout3",
            Self::AccentCallout1 => "accentCallout1",
            Self::AccentCallout2 => "accentCallout2",
            Self::AccentCallout3 => "accentCallout3",
            Self::BorderCallout1 => "borderCallout1",
            Self::BorderCallout2 => "borderCallout2",
            Self::BorderCallout3 => "borderCallout3",
            Self::AccentBorderCallout1 => "accentBorderCallout1",
            Self::AccentBorderCallout2 => "accentBorderCallout2",
            Self::AccentBorderCallout3 => "accentBorderCallout3",
            Self::WedgeRectangleCallout => "wedgeRectCallout",
            Self::WedgeRoundRectangleCallout => "wedgeRoundRectCallout",
            Self::WedgeEllipseCallout => "wedgeEllipseCallout",
            Self::CloudCallout => "cloudCallout",
            Self::Cloud => "cloud",
            Self::Ribbon => "ribbon",
            Self::Ribbon2 => "ribbon2",
            Self::EllipseRibbon => "ellipseRibbon",
            Self::EllipseRibbon2 => "ellipseRibbon2",
            Self::LeftRightRibbon => "leftRightRibbon",
            Self::VerticalScroll => "verticalScroll",
            Self::HorizontalScroll => "horizontalScroll",
            Self::Wave => "wave",
            Self::DoubleWave => "doubleWave",
            Self::Plus => "plus",
            Self::FlowChartProcess => "flowChartProcess",
            Self::FlowChartDecision => "flowChartDecision",
            Self::FlowChartInputOutput => "flowChartInputOutput",
            Self::FlowChartPredefinedProcess => "flowChartPredefinedProcess",
            Self::FlowChartInternalStorage => "flowChartInternalStorage",
            Self::FlowChartDocument => "flowChartDocument",
            Self::FlowChartMultidocument => "flowChartMultidocument",
            Self::FlowChartTerminator => "flowChartTerminator",
            Self::FlowChartPreparation => "flowChartPreparation",
            Self::FlowChartManualInput => "flowChartManualInput",
            Self::FlowChartManualOperation => "flowChartManualOperation",
            Self::FlowChartConnector => "flowChartConnector",
            Self::FlowChartPunchedCard => "flowChartPunchedCard",
            Self::FlowChartPunchedTape => "flowChartPunchedTape",
            Self::FlowChartSummingJunction => "flowChartSummingJunction",
            Self::FlowChartOr => "flowChartOr",
            Self::FlowChartCollate => "flowChartCollate",
            Self::FlowChartSort => "flowChartSort",
            Self::FlowChartExtract => "flowChartExtract",
            Self::FlowChartMerge => "flowChartMerge",
            Self::FlowChartOfflineStorage => "flowChartOfflineStorage",
            Self::FlowChartOnlineStorage => "flowChartOnlineStorage",
            Self::FlowChartMagneticTape => "flowChartMagneticTape",
            Self::FlowChartMagneticDisk => "flowChartMagneticDisk",
            Self::FlowChartMagneticDrum => "flowChartMagneticDrum",
            Self::FlowChartDisplay => "flowChartDisplay",
            Self::FlowChartDelay => "flowChartDelay",
            Self::FlowChartAlternateProcess => "flowChartAlternateProcess",
            Self::FlowChartOffpageConnector => "flowChartOffpageConnector",
            Self::ActionButtonBlank => "actionButtonBlank",
            Self::ActionButtonHome => "actionButtonHome",
            Self::ActionButtonHelp => "actionButtonHelp",
            Self::ActionButtonInformation => "actionButtonInformation",
            Self::ActionButtonForwardNext => "actionButtonForwardNext",
            Self::ActionButtonBackPrevious => "actionButtonBackPrevious",
            Self::ActionButtonEnd => "actionButtonEnd",
            Self::ActionButtonBeginning => "actionButtonBeginning",
            Self::ActionButtonReturn => "actionButtonReturn",
            Self::ActionButtonDocument => "actionButtonDocument",
            Self::ActionButtonSound => "actionButtonSound",
            Self::ActionButtonMovie => "actionButtonMovie",
            Self::Gear6 => "gear6",
            Self::Gear9 => "gear9",
            Self::Funnel => "funnel",
            Self::MathPlus => "mathPlus",
            Self::MathMinus => "mathMinus",
            Self::MathMultiply => "mathMultiply",
            Self::MathDivide => "mathDivide",
            Self::MathEqual => "mathEqual",
            Self::MathNotEqual => "mathNotEqual",
            Self::CornerTabs => "cornerTabs",
            Self::SquareTabs => "squareTabs",
            Self::PlaqueTabs => "plaqueTabs",
            Self::ChartX => "chartX",
            Self::ChartStar => "chartStar",
            Self::ChartPlus => "chartPlus",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "line" => Some(Self::Line),
            "lineInv" => Some(Self::LineInverse),
            "triangle" => Some(Self::Triangle),
            "rtTriangle" => Some(Self::RightTriangle),
            "rect" => Some(Self::Rectangle),
            "diamond" => Some(Self::Diamond),
            "parallelogram" => Some(Self::Parallelogram),
            "trapezoid" => Some(Self::Trapezoid),
            "nonIsoscelesTrapezoid" => Some(Self::NonIsoscelesTrapezoid),
            "pentagon" => Some(Self::Pentagon),
            "hexagon" => Some(Self::Hexagon),
            "heptagon" => Some(Self::Heptagon),
            "octagon" => Some(Self::Octagon),
            "decagon" => Some(Self::Decagon),
            "dodecagon" => Some(Self::Dodecagon),
            "star4" => Some(Self::Star4),
            "star5" => Some(Self::Star5),
            "star6" => Some(Self::Star6),
            "star7" => Some(Self::Star7),
            "star8" => Some(Self::Star8),
            "star10" => Some(Self::Star10),
            "star12" => Some(Self::Star12),
            "star16" => Some(Self::Star16),
            "star24" => Some(Self::Star24),
            "star32" => Some(Self::Star32),
            "roundRect" => Some(Self::RoundRectangle),
            "round1Rect" => Some(Self::Round1Rectangle),
            "round2SameRect" => Some(Self::Round2SameRectangle),
            "round2DiagRect" => Some(Self::Round2DiagonalRectangle),
            "snipRoundRect" => Some(Self::SnipRoundRectangle),
            "snip1Rect" => Some(Self::Snip1Rectangle),
            "snip2SameRect" => Some(Self::Snip2SameRectangle),
            "snip2DiagRect" => Some(Self::Snip2DiagonalRectangle),
            "plaque" => Some(Self::Plaque),
            "ellipse" => Some(Self::Ellipse),
            "teardrop" => Some(Self::Teardrop),
            "homePlate" => Some(Self::HomePlate),
            "chevron" => Some(Self::Chevron),
            "pieWedge" => Some(Self::PieWedge),
            "pie" => Some(Self::Pie),
            "blockArc" => Some(Self::BlockArc),
            "donut" => Some(Self::Donut),
            "noSmoking" => Some(Self::NoSmoking),
            "rightArrow" => Some(Self::RightArrow),
            "leftArrow" => Some(Self::LeftArrow),
            "upArrow" => Some(Self::UpArrow),
            "downArrow" => Some(Self::DownArrow),
            "stripedRightArrow" => Some(Self::StripedRightArrow),
            "notchedRightArrow" => Some(Self::NotchedRightArrow),
            "bentUpArrow" => Some(Self::BentUpArrow),
            "leftRightArrow" => Some(Self::LeftRightArrow),
            "upDownArrow" => Some(Self::UpDownArrow),
            "leftUpArrow" => Some(Self::LeftUpArrow),
            "leftRightUpArrow" => Some(Self::LeftRightUpArrow),
            "quadArrow" => Some(Self::QuadArrow),
            "leftArrowCallout" => Some(Self::LeftArrowCallout),
            "rightArrowCallout" => Some(Self::RightArrowCallout),
            "upArrowCallout" => Some(Self::UpArrowCallout),
            "downArrowCallout" => Some(Self::DownArrowCallout),
            "leftRightArrowCallout" => Some(Self::LeftRightArrowCallout),
            "upDownArrowCallout" => Some(Self::UpDownArrowCallout),
            "quadArrowCallout" => Some(Self::QuadArrowCallout),
            "bentArrow" => Some(Self::BentArrow),
            "uturnArrow" => Some(Self::UTurnArrow),
            "circularArrow" => Some(Self::CircularArrow),
            "leftCircularArrow" => Some(Self::LeftCircularArrow),
            "leftRightCircularArrow" => Some(Self::LeftRightCircularArrow),
            "curvedRightArrow" => Some(Self::CurvedRightArrow),
            "curvedLeftArrow" => Some(Self::CurvedLeftArrow),
            "curvedUpArrow" => Some(Self::CurvedUpArrow),
            "curvedDownArrow" => Some(Self::CurvedDownArrow),
            "swooshArrow" => Some(Self::SwooshArrow),
            "cube" => Some(Self::Cube),
            "can" => Some(Self::Can),
            "lightningBolt" => Some(Self::LightningBolt),
            "heart" => Some(Self::Heart),
            "sun" => Some(Self::Sun),
            "moon" => Some(Self::Moon),
            "smileyFace" => Some(Self::SmileyFace),
            "irregularSeal1" => Some(Self::IrregularSeal1),
            "irregularSeal2" => Some(Self::IrregularSeal2),
            "foldedCorner" => Some(Self::FoldedCorner),
            "bevel" => Some(Self::Bevel),
            "frame" => Some(Self::Frame),
            "halfFrame" => Some(Self::HalfFrame),
            "corner" => Some(Self::Corner),
            "diagStripe" => Some(Self::DiagonalStripe),
            "chord" => Some(Self::Chord),
            "arc" => Some(Self::Arc),
            "leftBracket" => Some(Self::LeftBracket),
            "rightBracket" => Some(Self::RightBracket),
            "leftBrace" => Some(Self::LeftBrace),
            "rightBrace" => Some(Self::RightBrace),
            "bracketPair" => Some(Self::BracketPair),
            "bracePair" => Some(Self::BracePair),
            "straightConnector1" => Some(Self::StraightConnector1),
            "bentConnector2" => Some(Self::BentConnector2),
            "bentConnector3" => Some(Self::BentConnector3),
            "bentConnector4" => Some(Self::BentConnector4),
            "bentConnector5" => Some(Self::BentConnector5),
            "curvedConnector2" => Some(Self::CurvedConnector2),
            "curvedConnector3" => Some(Self::CurvedConnector3),
            "curvedConnector4" => Some(Self::CurvedConnector4),
            "curvedConnector5" => Some(Self::CurvedConnector5),
            "callout1" => Some(Self::Callout1),
            "callout2" => Some(Self::Callout2),
            "callout3" => Some(Self::Callout3),
            "accentCallout1" => Some(Self::AccentCallout1),
            "accentCallout2" => Some(Self::AccentCallout2),
            "accentCallout3" => Some(Self::AccentCallout3),
            "borderCallout1" => Some(Self::BorderCallout1),
            "borderCallout2" => Some(Self::BorderCallout2),
            "borderCallout3" => Some(Self::BorderCallout3),
            "accentBorderCallout1" => Some(Self::AccentBorderCallout1),
            "accentBorderCallout2" => Some(Self::AccentBorderCallout2),
            "accentBorderCallout3" => Some(Self::AccentBorderCallout3),
            "wedgeRectCallout" => Some(Self::WedgeRectangleCallout),
            "wedgeRoundRectCallout" => Some(Self::WedgeRoundRectangleCallout),
            "wedgeEllipseCallout" => Some(Self::WedgeEllipseCallout),
            "cloudCallout" => Some(Self::CloudCallout),
            "cloud" => Some(Self::Cloud),
            "ribbon" => Some(Self::Ribbon),
            "ribbon2" => Some(Self::Ribbon2),
            "ellipseRibbon" => Some(Self::EllipseRibbon),
            "ellipseRibbon2" => Some(Self::EllipseRibbon2),
            "leftRightRibbon" => Some(Self::LeftRightRibbon),
            "verticalScroll" => Some(Self::VerticalScroll),
            "horizontalScroll" => Some(Self::HorizontalScroll),
            "wave" => Some(Self::Wave),
            "doubleWave" => Some(Self::DoubleWave),
            "plus" => Some(Self::Plus),
            "flowChartProcess" => Some(Self::FlowChartProcess),
            "flowChartDecision" => Some(Self::FlowChartDecision),
            "flowChartInputOutput" => Some(Self::FlowChartInputOutput),
            "flowChartPredefinedProcess" => Some(Self::FlowChartPredefinedProcess),
            "flowChartInternalStorage" => Some(Self::FlowChartInternalStorage),
            "flowChartDocument" => Some(Self::FlowChartDocument),
            "flowChartMultidocument" => Some(Self::FlowChartMultidocument),
            "flowChartTerminator" => Some(Self::FlowChartTerminator),
            "flowChartPreparation" => Some(Self::FlowChartPreparation),
            "flowChartManualInput" => Some(Self::FlowChartManualInput),
            "flowChartManualOperation" => Some(Self::FlowChartManualOperation),
            "flowChartConnector" => Some(Self::FlowChartConnector),
            "flowChartPunchedCard" => Some(Self::FlowChartPunchedCard),
            "flowChartPunchedTape" => Some(Self::FlowChartPunchedTape),
            "flowChartSummingJunction" => Some(Self::FlowChartSummingJunction),
            "flowChartOr" => Some(Self::FlowChartOr),
            "flowChartCollate" => Some(Self::FlowChartCollate),
            "flowChartSort" => Some(Self::FlowChartSort),
            "flowChartExtract" => Some(Self::FlowChartExtract),
            "flowChartMerge" => Some(Self::FlowChartMerge),
            "flowChartOfflineStorage" => Some(Self::FlowChartOfflineStorage),
            "flowChartOnlineStorage" => Some(Self::FlowChartOnlineStorage),
            "flowChartMagneticTape" => Some(Self::FlowChartMagneticTape),
            "flowChartMagneticDisk" => Some(Self::FlowChartMagneticDisk),
            "flowChartMagneticDrum" => Some(Self::FlowChartMagneticDrum),
            "flowChartDisplay" => Some(Self::FlowChartDisplay),
            "flowChartDelay" => Some(Self::FlowChartDelay),
            "flowChartAlternateProcess" => Some(Self::FlowChartAlternateProcess),
            "flowChartOffpageConnector" => Some(Self::FlowChartOffpageConnector),
            "actionButtonBlank" => Some(Self::ActionButtonBlank),
            "actionButtonHome" => Some(Self::ActionButtonHome),
            "actionButtonHelp" => Some(Self::ActionButtonHelp),
            "actionButtonInformation" => Some(Self::ActionButtonInformation),
            "actionButtonForwardNext" => Some(Self::ActionButtonForwardNext),
            "actionButtonBackPrevious" => Some(Self::ActionButtonBackPrevious),
            "actionButtonEnd" => Some(Self::ActionButtonEnd),
            "actionButtonBeginning" => Some(Self::ActionButtonBeginning),
            "actionButtonReturn" => Some(Self::ActionButtonReturn),
            "actionButtonDocument" => Some(Self::ActionButtonDocument),
            "actionButtonSound" => Some(Self::ActionButtonSound),
            "actionButtonMovie" => Some(Self::ActionButtonMovie),
            "gear6" => Some(Self::Gear6),
            "gear9" => Some(Self::Gear9),
            "funnel" => Some(Self::Funnel),
            "mathPlus" => Some(Self::MathPlus),
            "mathMinus" => Some(Self::MathMinus),
            "mathMultiply" => Some(Self::MathMultiply),
            "mathDivide" => Some(Self::MathDivide),
            "mathEqual" => Some(Self::MathEqual),
            "mathNotEqual" => Some(Self::MathNotEqual),
            "cornerTabs" => Some(Self::CornerTabs),
            "squareTabs" => Some(Self::SquareTabs),
            "plaqueTabs" => Some(Self::PlaqueTabs),
            "chartX" => Some(Self::ChartX),
            "chartStar" => Some(Self::ChartStar),
            "chartPlus" => Some(Self::ChartPlus),
            _ => None,
        }
    }
}

impl core::fmt::Display for ShapeTypeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for ShapeTypeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Text Shape Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextShapeValues {
    TextNoShape,
    TextPlain,
    TextStop,
    TextTriangle,
    TextTriangleInverted,
    TextChevron,
    TextChevronInverted,
    TextRingInside,
    TextRingOutside,
    TextArchUp,
    TextArchDown,
    TextCircle,
    TextButton,
    TextArchUpPour,
    TextArchDownPour,
    TextCirclePour,
    TextButtonPour,
    TextCurveUp,
    TextCurveDown,
    TextCanUp,
    TextCanDown,
    TextWave1,
    TextWave2,
    TextDoubleWave1,
    TextWave4,
    TextInflate,
    TextDeflate,
    TextInflateBottom,
    TextDeflateBottom,
    TextInflateTop,
    TextDeflateTop,
    TextDeflateInflate,
    TextDeflateInflateDeflate,
    TextFadeRight,
    TextFadeLeft,
    TextFadeUp,
    TextFadeDown,
    TextSlantUp,
    TextSlantDown,
    TextCascadeUp,
    TextCascadeDown,
}

impl TextShapeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TextNoShape => "textNoShape",
            Self::TextPlain => "textPlain",
            Self::TextStop => "textStop",
            Self::TextTriangle => "textTriangle",
            Self::TextTriangleInverted => "textTriangleInverted",
            Self::TextChevron => "textChevron",
            Self::TextChevronInverted => "textChevronInverted",
            Self::TextRingInside => "textRingInside",
            Self::TextRingOutside => "textRingOutside",
            Self::TextArchUp => "textArchUp",
            Self::TextArchDown => "textArchDown",
            Self::TextCircle => "textCircle",
            Self::TextButton => "textButton",
            Self::TextArchUpPour => "textArchUpPour",
            Self::TextArchDownPour => "textArchDownPour",
            Self::TextCirclePour => "textCirclePour",
            Self::TextButtonPour => "textButtonPour",
            Self::TextCurveUp => "textCurveUp",
            Self::TextCurveDown => "textCurveDown",
            Self::TextCanUp => "textCanUp",
            Self::TextCanDown => "textCanDown",
            Self::TextWave1 => "textWave1",
            Self::TextWave2 => "textWave2",
            Self::TextDoubleWave1 => "textDoubleWave1",
            Self::TextWave4 => "textWave4",
            Self::TextInflate => "textInflate",
            Self::TextDeflate => "textDeflate",
            Self::TextInflateBottom => "textInflateBottom",
            Self::TextDeflateBottom => "textDeflateBottom",
            Self::TextInflateTop => "textInflateTop",
            Self::TextDeflateTop => "textDeflateTop",
            Self::TextDeflateInflate => "textDeflateInflate",
            Self::TextDeflateInflateDeflate => "textDeflateInflateDeflate",
            Self::TextFadeRight => "textFadeRight",
            Self::TextFadeLeft => "textFadeLeft",
            Self::TextFadeUp => "textFadeUp",
            Self::TextFadeDown => "textFadeDown",
            Self::TextSlantUp => "textSlantUp",
            Self::TextSlantDown => "textSlantDown",
            Self::TextCascadeUp => "textCascadeUp",
            Self::TextCascadeDown => "textCascadeDown",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "textNoShape" => Some(Self::TextNoShape),
            "textPlain" => Some(Self::TextPlain),
            "textStop" => Some(Self::TextStop),
            "textTriangle" => Some(Self::TextTriangle),
            "textTriangleInverted" => Some(Self::TextTriangleInverted),
            "textChevron" => Some(Self::TextChevron),
            "textChevronInverted" => Some(Self::TextChevronInverted),
            "textRingInside" => Some(Self::TextRingInside),
            "textRingOutside" => Some(Self::TextRingOutside),
            "textArchUp" => Some(Self::TextArchUp),
            "textArchDown" => Some(Self::TextArchDown),
            "textCircle" => Some(Self::TextCircle),
            "textButton" => Some(Self::TextButton),
            "textArchUpPour" => Some(Self::TextArchUpPour),
            "textArchDownPour" => Some(Self::TextArchDownPour),
            "textCirclePour" => Some(Self::TextCirclePour),
            "textButtonPour" => Some(Self::TextButtonPour),
            "textCurveUp" => Some(Self::TextCurveUp),
            "textCurveDown" => Some(Self::TextCurveDown),
            "textCanUp" => Some(Self::TextCanUp),
            "textCanDown" => Some(Self::TextCanDown),
            "textWave1" => Some(Self::TextWave1),
            "textWave2" => Some(Self::TextWave2),
            "textDoubleWave1" => Some(Self::TextDoubleWave1),
            "textWave4" => Some(Self::TextWave4),
            "textInflate" => Some(Self::TextInflate),
            "textDeflate" => Some(Self::TextDeflate),
            "textInflateBottom" => Some(Self::TextInflateBottom),
            "textDeflateBottom" => Some(Self::TextDeflateBottom),
            "textInflateTop" => Some(Self::TextInflateTop),
            "textDeflateTop" => Some(Self::TextDeflateTop),
            "textDeflateInflate" => Some(Self::TextDeflateInflate),
            "textDeflateInflateDeflate" => Some(Self::TextDeflateInflateDeflate),
            "textFadeRight" => Some(Self::TextFadeRight),
            "textFadeLeft" => Some(Self::TextFadeLeft),
            "textFadeUp" => Some(Self::TextFadeUp),
            "textFadeDown" => Some(Self::TextFadeDown),
            "textSlantUp" => Some(Self::TextSlantUp),
            "textSlantDown" => Some(Self::TextSlantDown),
            "textCascadeUp" => Some(Self::TextCascadeUp),
            "textCascadeDown" => Some(Self::TextCascadeDown),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextShapeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextShapeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Path Fill Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathFillModeValues {
    None_,
    Norm,
    Lighten,
    LightenLess,
    Darken,
    DarkenLess,
}

impl PathFillModeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Norm => "norm",
            Self::Lighten => "lighten",
            Self::LightenLess => "lightenLess",
            Self::Darken => "darken",
            Self::DarkenLess => "darkenLess",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "norm" => Some(Self::Norm),
            "lighten" => Some(Self::Lighten),
            "lightenLess" => Some(Self::LightenLess),
            "darken" => Some(Self::Darken),
            "darkenLess" => Some(Self::DarkenLess),
            _ => None,
        }
    }
}

impl core::fmt::Display for PathFillModeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PathFillModeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Line End Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineEndValues {
    None_,
    Triangle,
    Stealth,
    Diamond,
    Oval,
    Arrow,
}

impl LineEndValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Triangle => "triangle",
            Self::Stealth => "stealth",
            Self::Diamond => "diamond",
            Self::Oval => "oval",
            Self::Arrow => "arrow",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "triangle" => Some(Self::Triangle),
            "stealth" => Some(Self::Stealth),
            "diamond" => Some(Self::Diamond),
            "oval" => Some(Self::Oval),
            "arrow" => Some(Self::Arrow),
            _ => None,
        }
    }
}

impl core::fmt::Display for LineEndValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for LineEndValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Line End Width
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineEndWidthValues {
    Small,
    Medium,
    Large,
}

impl LineEndWidthValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "med",
            Self::Large => "lg",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sm" => Some(Self::Small),
            "med" => Some(Self::Medium),
            "lg" => Some(Self::Large),
            _ => None,
        }
    }
}

impl core::fmt::Display for LineEndWidthValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for LineEndWidthValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Line End Length
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineEndLengthValues {
    Small,
    Medium,
    Large,
}

impl LineEndLengthValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "med",
            Self::Large => "lg",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sm" => Some(Self::Small),
            "med" => Some(Self::Medium),
            "lg" => Some(Self::Large),
            _ => None,
        }
    }
}

impl core::fmt::Display for LineEndLengthValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for LineEndLengthValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Line Dash Value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetLineDashValues {
    Solid,
    Dot,
    Dash,
    LargeDash,
    DashDot,
    LargeDashDot,
    LargeDashDotDot,
    SystemDash,
    SystemDot,
    SystemDashDot,
    SystemDashDotDot,
}

impl PresetLineDashValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Dot => "dot",
            Self::Dash => "dash",
            Self::LargeDash => "lgDash",
            Self::DashDot => "dashDot",
            Self::LargeDashDot => "lgDashDot",
            Self::LargeDashDotDot => "lgDashDotDot",
            Self::SystemDash => "sysDash",
            Self::SystemDot => "sysDot",
            Self::SystemDashDot => "sysDashDot",
            Self::SystemDashDotDot => "sysDashDotDot",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "solid" => Some(Self::Solid),
            "dot" => Some(Self::Dot),
            "dash" => Some(Self::Dash),
            "lgDash" => Some(Self::LargeDash),
            "dashDot" => Some(Self::DashDot),
            "lgDashDot" => Some(Self::LargeDashDot),
            "lgDashDotDot" => Some(Self::LargeDashDotDot),
            "sysDash" => Some(Self::SystemDash),
            "sysDot" => Some(Self::SystemDot),
            "sysDashDot" => Some(Self::SystemDashDot),
            "sysDashDotDot" => Some(Self::SystemDashDotDot),
            _ => None,
        }
    }
}

impl core::fmt::Display for PresetLineDashValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PresetLineDashValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// End Line Cap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineCapValues {
    Round,
    Square,
    Flat,
}

impl LineCapValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Round => "rnd",
            Self::Square => "sq",
            Self::Flat => "flat",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rnd" => Some(Self::Round),
            "sq" => Some(Self::Square),
            "flat" => Some(Self::Flat),
            _ => None,
        }
    }
}

impl core::fmt::Display for LineCapValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for LineCapValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Alignment Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PenAlignmentValues {
    Center,
    Insert,
}

impl PenAlignmentValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Center => "ctr",
            Self::Insert => "in",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ctr" => Some(Self::Center),
            "in" => Some(Self::Insert),
            _ => None,
        }
    }
}

impl core::fmt::Display for PenAlignmentValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PenAlignmentValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Compound Line Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompoundLineValues {
    Single,
    Double,
    ThickThin,
    ThinThick,
    Triple,
}

impl CompoundLineValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Single => "sng",
            Self::Double => "dbl",
            Self::ThickThin => "thickThin",
            Self::ThinThick => "thinThick",
            Self::Triple => "tri",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sng" => Some(Self::Single),
            "dbl" => Some(Self::Double),
            "thickThin" => Some(Self::ThickThin),
            "thinThick" => Some(Self::ThinThick),
            "tri" => Some(Self::Triple),
            _ => None,
        }
    }
}

impl core::fmt::Display for CompoundLineValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for CompoundLineValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// On/Off Style Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BooleanStyleValues {
    On,
    Off,
    Default,
}

impl BooleanStyleValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Default => "def",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "on" => Some(Self::On),
            "off" => Some(Self::Off),
            "def" => Some(Self::Default),
            _ => None,
        }
    }
}

impl core::fmt::Display for BooleanStyleValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for BooleanStyleValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Vertical Overflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextVerticalOverflowValues {
    Overflow,
    Ellipsis,
    Clip,
}

impl TextVerticalOverflowValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Overflow => "overflow",
            Self::Ellipsis => "ellipsis",
            Self::Clip => "clip",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "overflow" => Some(Self::Overflow),
            "ellipsis" => Some(Self::Ellipsis),
            "clip" => Some(Self::Clip),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextVerticalOverflowValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextVerticalOverflowValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Horizontal Overflow Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextHorizontalOverflowValues {
    Overflow,
    Clip,
}

impl TextHorizontalOverflowValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Overflow => "overflow",
            Self::Clip => "clip",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "overflow" => Some(Self::Overflow),
            "clip" => Some(Self::Clip),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextHorizontalOverflowValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextHorizontalOverflowValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Vertical Text Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextVerticalValues {
    Horizontal,
    Vertical,
    Vertical270,
    WordArtVertical,
    EastAsianVetical,
    MongolianVertical,
    WordArtLeftToRight,
}

impl TextVerticalValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horz",
            Self::Vertical => "vert",
            Self::Vertical270 => "vert270",
            Self::WordArtVertical => "wordArtVert",
            Self::EastAsianVetical => "eaVert",
            Self::MongolianVertical => "mongolianVert",
            Self::WordArtLeftToRight => "wordArtVertRtl",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "horz" => Some(Self::Horizontal),
            "vert" => Some(Self::Vertical),
            "vert270" => Some(Self::Vertical270),
            "wordArtVert" => Some(Self::WordArtVertical),
            "eaVert" => Some(Self::EastAsianVetical),
            "mongolianVert" => Some(Self::MongolianVertical),
            "wordArtVertRtl" => Some(Self::WordArtLeftToRight),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextVerticalValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextVerticalValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Wrapping Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextWrappingValues {
    None_,
    Square,
}

impl TextWrappingValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Square => "square",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "square" => Some(Self::Square),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextWrappingValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextWrappingValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Anchoring Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAnchoringTypeValues {
    Top,
    Center,
    Bottom,
}

impl TextAnchoringTypeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "t",
            Self::Center => "ctr",
            Self::Bottom => "b",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "t" => Some(Self::Top),
            "ctr" => Some(Self::Center),
            "b" => Some(Self::Bottom),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextAnchoringTypeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextAnchoringTypeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Auto-number Schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAutoNumberSchemeValues {
    AlphaLowerCharacterParenBoth,
    AlphaUpperCharacterParenBoth,
    AlphaLowerCharacterParenR,
    AlphaUpperCharacterParenR,
    AlphaLowerCharacterPeriod,
    AlphaUpperCharacterPeriod,
    ArabicParenBoth,
    ArabicParenR,
    ArabicPeriod,
    ArabicPlain,
    RomanLowerCharacterParenBoth,
    RomanUpperCharacterParenBoth,
    RomanLowerCharacterParenR,
    RomanUpperCharacterParenR,
    RomanLowerCharacterPeriod,
    RomanUpperCharacterPeriod,
    CircleNumberDoubleBytePlain,
    CircleNumberWingdingsBlackPlain,
    CircleNumberWingdingsWhitePlain,
    ArabicDoubleBytePeriod,
    ArabicDoubleBytePlain,
    EastAsianSimplifiedChinesePeriod,
    EastAsianSimplifiedChinesePlain,
    EastAsianTraditionalChinesePeriod,
    EastAsianTraditionalChinesePlain,
    EastAsianJapaneseDoubleBytePeriod,
    EastAsianJapaneseKoreanPlain,
    EastAsianJapaneseKoreanPeriod,
    Arabic1Minus,
    Arabic2Minus,
    Hebrew2Minus,
    ThaiAlphaPeriod,
    ThaiAlphaParenthesisRight,
    ThaiAlphaParenthesisBoth,
    ThaiNumberPeriod,
    ThaiNumberParenthesisRight,
    ThaiNumberParenthesisBoth,
    HindiAlphaPeriod,
    HindiNumPeriod,
    HindiNumberParenthesisRight,
    HindiAlpha1Period,
}

impl TextAutoNumberSchemeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlphaLowerCharacterParenBoth => "alphaLcParenBoth",
            Self::AlphaUpperCharacterParenBoth => "alphaUcParenBoth",
            Self::AlphaLowerCharacterParenR => "alphaLcParenR",
            Self::AlphaUpperCharacterParenR => "alphaUcParenR",
            Self::AlphaLowerCharacterPeriod => "alphaLcPeriod",
            Self::AlphaUpperCharacterPeriod => "alphaUcPeriod",
            Self::ArabicParenBoth => "arabicParenBoth",
            Self::ArabicParenR => "arabicParenR",
            Self::ArabicPeriod => "arabicPeriod",
            Self::ArabicPlain => "arabicPlain",
            Self::RomanLowerCharacterParenBoth => "romanLcParenBoth",
            Self::RomanUpperCharacterParenBoth => "romanUcParenBoth",
            Self::RomanLowerCharacterParenR => "romanLcParenR",
            Self::RomanUpperCharacterParenR => "romanUcParenR",
            Self::RomanLowerCharacterPeriod => "romanLcPeriod",
            Self::RomanUpperCharacterPeriod => "romanUcPeriod",
            Self::CircleNumberDoubleBytePlain => "circleNumDbPlain",
            Self::CircleNumberWingdingsBlackPlain => "circleNumWdBlackPlain",
            Self::CircleNumberWingdingsWhitePlain => "circleNumWdWhitePlain",
            Self::ArabicDoubleBytePeriod => "arabicDbPeriod",
            Self::ArabicDoubleBytePlain => "arabicDbPlain",
            Self::EastAsianSimplifiedChinesePeriod => "ea1ChsPeriod",
            Self::EastAsianSimplifiedChinesePlain => "ea1ChsPlain",
            Self::EastAsianTraditionalChinesePeriod => "ea1ChtPeriod",
            Self::EastAsianTraditionalChinesePlain => "ea1ChtPlain",
            Self::EastAsianJapaneseDoubleBytePeriod => "ea1JpnChsDbPeriod",
            Self::EastAsianJapaneseKoreanPlain => "ea1JpnKorPlain",
            Self::EastAsianJapaneseKoreanPeriod => "ea1JpnKorPeriod",
            Self::Arabic1Minus => "arabic1Minus",
            Self::Arabic2Minus => "arabic2Minus",
            Self::Hebrew2Minus => "hebrew2Minus",
            Self::ThaiAlphaPeriod => "thaiAlphaPeriod",
            Self::ThaiAlphaParenthesisRight => "thaiAlphaParenR",
            Self::ThaiAlphaParenthesisBoth => "thaiAlphaParenBoth",
            Self::ThaiNumberPeriod => "thaiNumPeriod",
            Self::ThaiNumberParenthesisRight => "thaiNumParenR",
            Self::ThaiNumberParenthesisBoth => "thaiNumParenBoth",
            Self::HindiAlphaPeriod => "hindiAlphaPeriod",
            Self::HindiNumPeriod => "hindiNumPeriod",
            Self::HindiNumberParenthesisRight => "hindiNumParenR",
            Self::HindiAlpha1Period => "hindiAlpha1Period",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "alphaLcParenBoth" => Some(Self::AlphaLowerCharacterParenBoth),
            "alphaUcParenBoth" => Some(Self::AlphaUpperCharacterParenBoth),
            "alphaLcParenR" => Some(Self::AlphaLowerCharacterParenR),
            "alphaUcParenR" => Some(Self::AlphaUpperCharacterParenR),
            "alphaLcPeriod" => Some(Self::AlphaLowerCharacterPeriod),
            "alphaUcPeriod" => Some(Self::AlphaUpperCharacterPeriod),
            "arabicParenBoth" => Some(Self::ArabicParenBoth),
            "arabicParenR" => Some(Self::ArabicParenR),
            "arabicPeriod" => Some(Self::ArabicPeriod),
            "arabicPlain" => Some(Self::ArabicPlain),
            "romanLcParenBoth" => Some(Self::RomanLowerCharacterParenBoth),
            "romanUcParenBoth" => Some(Self::RomanUpperCharacterParenBoth),
            "romanLcParenR" => Some(Self::RomanLowerCharacterParenR),
            "romanUcParenR" => Some(Self::RomanUpperCharacterParenR),
            "romanLcPeriod" => Some(Self::RomanLowerCharacterPeriod),
            "romanUcPeriod" => Some(Self::RomanUpperCharacterPeriod),
            "circleNumDbPlain" => Some(Self::CircleNumberDoubleBytePlain),
            "circleNumWdBlackPlain" => Some(Self::CircleNumberWingdingsBlackPlain),
            "circleNumWdWhitePlain" => Some(Self::CircleNumberWingdingsWhitePlain),
            "arabicDbPeriod" => Some(Self::ArabicDoubleBytePeriod),
            "arabicDbPlain" => Some(Self::ArabicDoubleBytePlain),
            "ea1ChsPeriod" => Some(Self::EastAsianSimplifiedChinesePeriod),
            "ea1ChsPlain" => Some(Self::EastAsianSimplifiedChinesePlain),
            "ea1ChtPeriod" => Some(Self::EastAsianTraditionalChinesePeriod),
            "ea1ChtPlain" => Some(Self::EastAsianTraditionalChinesePlain),
            "ea1JpnChsDbPeriod" => Some(Self::EastAsianJapaneseDoubleBytePeriod),
            "ea1JpnKorPlain" => Some(Self::EastAsianJapaneseKoreanPlain),
            "ea1JpnKorPeriod" => Some(Self::EastAsianJapaneseKoreanPeriod),
            "arabic1Minus" => Some(Self::Arabic1Minus),
            "arabic2Minus" => Some(Self::Arabic2Minus),
            "hebrew2Minus" => Some(Self::Hebrew2Minus),
            "thaiAlphaPeriod" => Some(Self::ThaiAlphaPeriod),
            "thaiAlphaParenR" => Some(Self::ThaiAlphaParenthesisRight),
            "thaiAlphaParenBoth" => Some(Self::ThaiAlphaParenthesisBoth),
            "thaiNumPeriod" => Some(Self::ThaiNumberPeriod),
            "thaiNumParenR" => Some(Self::ThaiNumberParenthesisRight),
            "thaiNumParenBoth" => Some(Self::ThaiNumberParenthesisBoth),
            "hindiAlphaPeriod" => Some(Self::HindiAlphaPeriod),
            "hindiNumPeriod" => Some(Self::HindiNumPeriod),
            "hindiNumParenR" => Some(Self::HindiNumberParenthesisRight),
            "hindiAlpha1Period" => Some(Self::HindiAlpha1Period),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextAutoNumberSchemeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextAutoNumberSchemeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Underline Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextUnderlineValues {
    None_,
    Words,
    Single,
    Double,
    Heavy,
    Dotted,
    HeavyDotted,
    Dash,
    DashHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DotDashHeavy,
    DotDotDash,
    DotDotDashHeavy,
    Wavy,
    WavyHeavy,
    WavyDouble,
}

impl TextUnderlineValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Words => "words",
            Self::Single => "sng",
            Self::Double => "dbl",
            Self::Heavy => "heavy",
            Self::Dotted => "dotted",
            Self::HeavyDotted => "dottedHeavy",
            Self::Dash => "dash",
            Self::DashHeavy => "dashHeavy",
            Self::DashLong => "dashLong",
            Self::DashLongHeavy => "dashLongHeavy",
            Self::DotDash => "dotDash",
            Self::DotDashHeavy => "dotDashHeavy",
            Self::DotDotDash => "dotDotDash",
            Self::DotDotDashHeavy => "dotDotDashHeavy",
            Self::Wavy => "wavy",
            Self::WavyHeavy => "wavyHeavy",
            Self::WavyDouble => "wavyDbl",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "words" => Some(Self::Words),
            "sng" => Some(Self::Single),
            "dbl" => Some(Self::Double),
            "heavy" => Some(Self::Heavy),
            "dotted" => Some(Self::Dotted),
            "dottedHeavy" => Some(Self::HeavyDotted),
            "dash" => Some(Self::Dash),
            "dashHeavy" => Some(Self::DashHeavy),
            "dashLong" => Some(Self::DashLong),
            "dashLongHeavy" => Some(Self::DashLongHeavy),
            "dotDash" => Some(Self::DotDash),
            "dotDashHeavy" => Some(Self::DotDashHeavy),
            "dotDotDash" => Some(Self::DotDotDash),
            "dotDotDashHeavy" => Some(Self::DotDotDashHeavy),
            "wavy" => Some(Self::Wavy),
            "wavyHeavy" => Some(Self::WavyHeavy),
            "wavyDbl" => Some(Self::WavyDouble),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextUnderlineValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextUnderlineValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Strike Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextStrikeValues {
    NoStrike,
    SingleStrike,
    DoubleStrike,
}

impl TextStrikeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoStrike => "noStrike",
            Self::SingleStrike => "sngStrike",
            Self::DoubleStrike => "dblStrike",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "noStrike" => Some(Self::NoStrike),
            "sngStrike" => Some(Self::SingleStrike),
            "dblStrike" => Some(Self::DoubleStrike),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextStrikeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextStrikeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Cap Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextCapsValues {
    None_,
    Small,
    All,
}

impl TextCapsValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None_ => "none",
            Self::Small => "small",
            Self::All => "all",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None_),
            "small" => Some(Self::Small),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextCapsValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextCapsValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Tab Alignment Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextTabAlignmentValues {
    Left,
    Center,
    Right,
    Decimal,
}

impl TextTabAlignmentValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "l",
            Self::Center => "ctr",
            Self::Right => "r",
            Self::Decimal => "dec",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "l" => Some(Self::Left),
            "ctr" => Some(Self::Center),
            "r" => Some(Self::Right),
            "dec" => Some(Self::Decimal),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextTabAlignmentValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextTabAlignmentValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Text Alignment Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlignmentTypeValues {
    Left,
    Center,
    Right,
    Justified,
    JustifiedLow,
    Distributed,
    ThaiDistributed,
}

impl TextAlignmentTypeValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "l",
            Self::Center => "ctr",
            Self::Right => "r",
            Self::Justified => "just",
            Self::JustifiedLow => "justLow",
            Self::Distributed => "dist",
            Self::ThaiDistributed => "thaiDist",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "l" => Some(Self::Left),
            "ctr" => Some(Self::Center),
            "r" => Some(Self::Right),
            "just" => Some(Self::Justified),
            "justLow" => Some(Self::JustifiedLow),
            "dist" => Some(Self::Distributed),
            "thaiDist" => Some(Self::ThaiDistributed),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextAlignmentTypeValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextAlignmentTypeValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Font Alignment Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextFontAlignmentValues {
    Automatic,
    Top,
    Center,
    Baseline,
    Bottom,
}

impl TextFontAlignmentValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "auto",
            Self::Top => "t",
            Self::Center => "ctr",
            Self::Baseline => "base",
            Self::Bottom => "b",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(Self::Automatic),
            "t" => Some(Self::Top),
            "ctr" => Some(Self::Center),
            "base" => Some(Self::Baseline),
            "b" => Some(Self::Bottom),
            _ => None,
        }
    }
}

impl core::fmt::Display for TextFontAlignmentValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for TextFontAlignmentValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Preset Color Value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetColorValues {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenrod,
    DarkGray,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DodgerBlue,
    Firebrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    Goldenrod,
    Gray,
    Green,
    GreenYellow,
    Honeydew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenrodYellow,
    LightGray,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MedAquamarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenrod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
    DarkBlue2010,
    DarkCyan2010,
    DarkGoldenrod2010,
    DarkGray2010,
    DarkGrey2010,
    DarkGreen2010,
    DarkKhaki2010,
    DarkMagenta2010,
    DarkOliveGreen2010,
    DarkOrange2010,
    DarkOrchid2010,
    DarkRed2010,
    DarkSalmon2010,
    DarkSeaGreen2010,
    DarkSlateBlue2010,
    DarkSlateGray2010,
    DarkSlateGrey2010,
    DarkTurquoise2010,
    DarkViolet2010,
    LightBlue2010,
    LightCoral2010,
    LightCyan2010,
    LightGoldenrodYellow2010,
    LightGray2010,
    LightGrey2010,
    LightGreen2010,
    LightPink2010,
    LightSalmon2010,
    LightSeaGreen2010,
    LightSkyBlue2010,
    LightSlateGray2010,
    LightSlateGrey2010,
    LightSteelBlue2010,
    LightYellow2010,
    MediumAquamarine2010,
    MediumBlue2010,
    MediumOrchid2010,
    MediumPurple2010,
    MediumSeaGreen2010,
    MediumSlateBlue2010,
    MediumSpringGreen2010,
    MediumTurquoise2010,
    MediumVioletRed2010,
    DarkGrey,
    DimGrey,
    DarkSlateGrey,
    Grey,
    LightGrey,
    LightSlateGrey,
    SlateGrey,
}

impl PresetColorValues {
    /// Schema string value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliceBlue => "aliceBlue",
            Self::AntiqueWhite => "antiqueWhite",
            Self::Aqua => "aqua",
            Self::Aquamarine => "aquamarine",
            Self::Azure => "azure",
            Self::Beige => "beige",
            Self::Bisque => "bisque",
            Self::Black => "black",
            Self::BlanchedAlmond => "blanchedAlmond",
            Self::Blue => "blue",
            Self::BlueViolet => "blueViolet",
            Self::Brown => "brown",
            Self::BurlyWood => "burlyWood",
            Self::CadetBlue => "cadetBlue",
            Self::Chartreuse => "chartreuse",
            Self::Chocolate => "chocolate",
            Self::Coral => "coral",
            Self::CornflowerBlue => "cornflowerBlue",
            Self::Cornsilk => "cornsilk",
            Self::Crimson => "crimson",
            Self::Cyan => "cyan",
            Self::DarkBlue => "dkBlue",
            Self::DarkCyan => "dkCyan",
            Self::DarkGoldenrod => "dkGoldenrod",
            Self::DarkGray => "dkGray",
            Self::DarkGreen => "dkGreen",
            Self::DarkKhaki => "dkKhaki",
            Self::DarkMagenta => "dkMagenta",
            Self::DarkOliveGreen => "dkOliveGreen",
            Self::DarkOrange => "dkOrange",
            Self::DarkOrchid => "dkOrchid",
            Self::DarkRed => "dkRed",
            Self::DarkSalmon => "dkSalmon",
            Self::DarkSeaGreen => "dkSeaGreen",
            Self::DarkSlateBlue => "dkSlateBlue",
            Self::DarkSlateGray => "dkSlateGray",
            Self::DarkTurquoise => "dkTurquoise",
            Self::DarkViolet => "dkViolet",
            Self::DeepPink => "deepPink",
            Self::DeepSkyBlue => "deepSkyBlue",
            Self::DimGray => "dimGray",
            Self::DodgerBlue => "dodgerBlue",
            Self::Firebrick => "firebrick",
            Self::FloralWhite => "floralWhite",
            Self::ForestGreen => "forestGreen",
            Self::Fuchsia => "fuchsia",
            Self::Gainsboro => "gainsboro",
            Self::GhostWhite => "ghostWhite",
            Self::Gold => "gold",
            Self::Goldenrod => "goldenrod",
            Self::Gray => "gray",
            Self::Green => "green",
            Self::GreenYellow => "greenYellow",
            Self::Honeydew => "honeydew",
            Self::HotPink => "hotPink",
            Self::IndianRed => "indianRed",
            Self::Indigo => "indigo",
            Self::Ivory => "ivory",
            Self::Khaki => "khaki",
            Self::Lavender => "lavender",
            Self::LavenderBlush => "lavenderBlush",
            Self::LawnGreen => "lawnGreen",
            Self::LemonChiffon => "lemonChiffon",
            Self::LightBlue => "ltBlue",
            Self::LightCoral => "ltCoral",
            Self::LightCyan => "ltCyan",
            Self::LightGoldenrodYellow => "ltGoldenrodYellow",
            Self::LightGray => "ltGray",
            Self::LightGreen => "ltGreen",
            Self::LightPink => "ltPink",
            Self::LightSalmon => "ltSalmon",
            Self::LightSeaGreen => "ltSeaGreen",
            Self::LightSkyBlue => "ltSkyBlue",
            Self::LightSlateGray => "ltSlateGray",
            Self::LightSteelBlue => "ltSteelBlue",
            Self::LightYellow => "ltYellow",
            Self::Lime => "lime",
            Self::LimeGreen => "limeGreen",
            Self::Linen => "linen",
            Self::Magenta => "magenta",
            Self::Maroon => "maroon",
            Self::MedAquamarine => "medAquamarine",
            Self::MediumBlue => "medBlue",
            Self::MediumOrchid => "medOrchid",
            Self::MediumPurple => "medPurple",
            Self::MediumSeaGreen => "medSeaGreen",
            Self::MediumSlateBlue => "medSlateBlue",
            Self::MediumSpringGreen => "medSpringGreen",
            Self::MediumTurquoise => "medTurquoise",
            Self::MediumVioletRed => "medVioletRed",
            Self::MidnightBlue => "midnightBlue",
            Self::MintCream => "mintCream",
            Self::MistyRose => "mistyRose",
            Self::Moccasin => "moccasin",
            Self::NavajoWhite => "navajoWhite",
            Self::Navy => "navy",
            Self::OldLace => "oldLace",
            Self::Olive => "olive",
            Self::OliveDrab => "oliveDrab",
            Self::Orange => "orange",
            Self::OrangeRed => "orangeRed",
            Self::Orchid => "orchid",
            Self::PaleGoldenrod => "paleGoldenrod",
            Self::PaleGreen => "paleGreen",
            Self::PaleTurquoise => "paleTurquoise",
            Self::PaleVioletRed => "paleVioletRed",
            Self::PapayaWhip => "papayaWhip",
            Self::PeachPuff => "peachPuff",
            Self::Peru => "peru",
            Self::Pink => "pink",
            Self::Plum => "plum",
            Self::PowderBlue => "powderBlue",
            Self::Purple => "purple",
            Self::Red => "red",
            Self::RosyBrown => "rosyBrown",
            Self::RoyalBlue => "royalBlue",
            Self::SaddleBrown => "saddleBrown",
            Self::Salmon => "salmon",
            Self::SandyBrown => "sandyBrown",
            Self::SeaGreen => "seaGreen",
            Self::SeaShell => "seaShell",
            Self::Sienna => "sienna",
            Self::Silver => "silver",
            Self::SkyBlue => "skyBlue",
            Self::SlateBlue => "slateBlue",
            Self::SlateGray => "slateGray",
            Self::Snow => "snow",
            Self::SpringGreen => "springGreen",
            Self::SteelBlue => "steelBlue",
            Self::Tan => "tan",
            Self::Teal => "teal",
            Self::Thistle => "thistle",
            Self::Tomato => "tomato",
            Self::Turquoise => "turquoise",
            Self::Violet => "violet",
            Self::Wheat => "wheat",
            Self::White => "white",
            Self::WhiteSmoke => "whiteSmoke",
            Self::Yellow => "yellow",
            Self::YellowGreen => "yellowGreen",
            Self::DarkBlue2010 => "darkBlue",
            Self::DarkCyan2010 => "darkCyan",
            Self::DarkGoldenrod2010 => "darkGoldenrod",
            Self::DarkGray2010 => "darkGray",
            Self::DarkGrey2010 => "darkGrey",
            Self::DarkGreen2010 => "darkGreen",
            Self::DarkKhaki2010 => "darkKhaki",
            Self::DarkMagenta2010 => "darkMagenta",
            Self::DarkOliveGreen2010 => "darkOliveGreen",
            Self::DarkOrange2010 => "darkOrange",
            Self::DarkOrchid2010 => "darkOrchid",
            Self::DarkRed2010 => "darkRed",
            Self::DarkSalmon2010 => "darkSalmon",
            Self::DarkSeaGreen2010 => "darkSeaGreen",
            Self::DarkSlateBlue2010 => "darkSlateBlue",
            Self::DarkSlateGray2010 => "darkSlateGray",
            Self::DarkSlateGrey2010 => "darkSlateGrey",
            Self::DarkTurquoise2010 => "darkTurquoise",
            Self::DarkViolet2010 => "darkViolet",
            Self::LightBlue2010 => "lightBlue",
            Self::LightCoral2010 => "lightCoral",
            Self::LightCyan2010 => "lightCyan",
            Self::LightGoldenrodYellow2010 => "lightGoldenrodYellow",
            Self::LightGray2010 => "lightGray",
            Self::LightGrey2010 => "lightGrey",
            Self::LightGreen2010 => "lightGreen",
            Self::LightPink2010 => "lightPink",
            Self::LightSalmon2010 => "lightSalmon",
            Self::LightSeaGreen2010 => "lightSeaGreen",
            Self::LightSkyBlue2010 => "lightSkyBlue",
            Self::LightSlateGray2010 => "lightSlateGray",
            Self::LightSlateGrey2010 => "lightSlateGrey",
            Self::LightSteelBlue2010 => "lightSteelBlue",
            Self::LightYellow2010 => "lightYellow",
            Self::MediumAquamarine2010 => "mediumAquamarine",
            Self::MediumBlue2010 => "mediumBlue",
            Self::MediumOrchid2010 => "mediumOrchid",
            Self::MediumPurple2010 => "mediumPurple",
            Self::MediumSeaGreen2010 => "mediumSeaGreen",
            Self::MediumSlateBlue2010 => "mediumSlateBlue",
            Self::MediumSpringGreen2010 => "mediumSpringGreen",
            Self::MediumTurquoise2010 => "mediumTurquoise",
            Self::MediumVioletRed2010 => "mediumVioletRed",
            Self::DarkGrey => "dkGrey",
            Self::DimGrey => "dimGrey",
            Self::DarkSlateGrey => "dkSlateGrey",
            Self::Grey => "grey",
            Self::LightGrey => "ltGrey",
            Self::LightSlateGrey => "ltSlateGrey",
            Self::SlateGrey => "slateGrey",
        }
    }

    /// Parse from the schema string value.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "aliceBlue" => Some(Self::AliceBlue),
            "antiqueWhite" => Some(Self::AntiqueWhite),
            "aqua" => Some(Self::Aqua),
            "aquamarine" => Some(Self::Aquamarine),
            "azure" => Some(Self::Azure),
            "beige" => Some(Self::Beige),
            "bisque" => Some(Self::Bisque),
            "black" => Some(Self::Black),
            "blanchedAlmond" => Some(Self::BlanchedAlmond),
            "blue" => Some(Self::Blue),
            "blueViolet" => Some(Self::BlueViolet),
            "brown" => Some(Self::Brown),
            "burlyWood" => Some(Self::BurlyWood),
            "cadetBlue" => Some(Self::CadetBlue),
            "chartreuse" => Some(Self::Chartreuse),
            "chocolate" => Some(Self::Chocolate),
            "coral" => Some(Self::Coral),
            "cornflowerBlue" => Some(Self::CornflowerBlue),
            "cornsilk" => Some(Self::Cornsilk),
            "crimson" => Some(Self::Crimson),
            "cyan" => Some(Self::Cyan),
            "dkBlue" => Some(Self::DarkBlue),
            "dkCyan" => Some(Self::DarkCyan),
            "dkGoldenrod" => Some(Self::DarkGoldenrod),
            "dkGray" => Some(Self::DarkGray),
            "dkGreen" => Some(Self::DarkGreen),
            "dkKhaki" => Some(Self::DarkKhaki),
            "dkMagenta" => Some(Self::DarkMagenta),
            "dkOliveGreen" => Some(Self::DarkOliveGreen),
            "dkOrange" => Some(Self::DarkOrange),
            "dkOrchid" => Some(Self::DarkOrchid),
            "dkRed" => Some(Self::DarkRed),
            "dkSalmon" => Some(Self::DarkSalmon),
            "dkSeaGreen" => Some(Self::DarkSeaGreen),
            "dkSlateBlue" => Some(Self::DarkSlateBlue),
            "dkSlateGray" => Some(Self::DarkSlateGray),
            "dkTurquoise" => Some(Self::DarkTurquoise),
            "dkViolet" => Some(Self::DarkViolet),
            "deepPink" => Some(Self::DeepPink),
            "deepSkyBlue" => Some(Self::DeepSkyBlue),
            "dimGray" => Some(Self::DimGray),
            "dodgerBlue" => Some(Self::DodgerBlue),
            "firebrick" => Some(Self::Firebrick),
            "floralWhite" => Some(Self::FloralWhite),
            "forestGreen" => Some(Self::ForestGreen),
            "fuchsia" => Some(Self::Fuchsia),
            "gainsboro" => Some(Self::Gainsboro),
            "ghostWhite" => Some(Self::GhostWhite),
            "gold" => Some(Self::Gold),
            "goldenrod" => Some(Self::Goldenrod),
            "gray" => Some(Self::Gray),
            "green" => Some(Self::Green),
            "greenYellow" => Some(Self::GreenYellow),
            "honeydew" => Some(Self::Honeydew),
            "hotPink" => Some(Self::HotPink),
            "indianRed" => Some(Self::IndianRed),
            "indigo" => Some(Self::Indigo),
            "ivory" => Some(Self::Ivory),
            "khaki" => Some(Self::Khaki),
            "lavender" => Some(Self::Lavender),
            "lavenderBlush" => Some(Self::LavenderBlush),
            "lawnGreen" => Some(Self::LawnGreen),
            "lemonChiffon" => Some(Self::LemonChiffon),
            "ltBlue" => Some(Self::LightBlue),
            "ltCoral" => Some(Self::LightCoral),
            "ltCyan" => Some(Self::LightCyan),
            "ltGoldenrodYellow" => Some(Self::LightGoldenrodYellow),
            "ltGray" => Some(Self::LightGray),
            "ltGreen" => Some(Self::LightGreen),
            "ltPink" => Some(Self::LightPink),
            "ltSalmon" => Some(Self::LightSalmon),
            "ltSeaGreen" => Some(Self::LightSeaGreen),
            "ltSkyBlue" => Some(Self::LightSkyBlue),
            "ltSlateGray" => Some(Self::LightSlateGray),
            "ltSteelBlue" => Some(Self::LightSteelBlue),
            "ltYellow" => Some(Self::LightYellow),
            "lime" => Some(Self::Lime),
            "limeGreen" => Some(Self::LimeGreen),
            "linen" => Some(Self::Linen),
            "magenta" => Some(Self::Magenta),
            "maroon" => Some(Self::Maroon),
            "medAquamarine" => Some(Self::MedAquamarine),
            "medBlue" => Some(Self::MediumBlue),
            "medOrchid" => Some(Self::MediumOrchid),
            "medPurple" => Some(Self::MediumPurple),
            "medSeaGreen" => Some(Self::MediumSeaGreen),
            "medSlateBlue" => Some(Self::MediumSlateBlue),
            "medSpringGreen" => Some(Self::MediumSpringGreen),
            "medTurquoise" => Some(Self::MediumTurquoise),
            "medVioletRed" => Some(Self::MediumVioletRed),
            "midnightBlue" => Some(Self::MidnightBlue),
            "mintCream" => Some(Self::MintCream),
            "mistyRose" => Some(Self::MistyRose),
            "moccasin" => Some(Self::Moccasin),
            "navajoWhite" => Some(Self::NavajoWhite),
            "navy" => Some(Self::Navy),
            "oldLace" => Some(Self::OldLace),
            "olive" => Some(Self::Olive),
            "oliveDrab" => Some(Self::OliveDrab),
            "orange" => Some(Self::Orange),
            "orangeRed" => Some(Self::OrangeRed),
            "orchid" => Some(Self::Orchid),
            "paleGoldenrod" => Some(Self::PaleGoldenrod),
            "paleGreen" => Some(Self::PaleGreen),
            "paleTurquoise" => Some(Self::PaleTurquoise),
            "paleVioletRed" => Some(Self::PaleVioletRed),
            "papayaWhip" => Some(Self::PapayaWhip),
            "peachPuff" => Some(Self::PeachPuff),
            "peru" => Some(Self::Peru),
            "pink" => Some(Self::Pink),
            "plum" => Some(Self::Plum),
            "powderBlue" => Some(Self::PowderBlue),
            "purple" => Some(Self::Purple),
            "red" => Some(Self::Red),
            "rosyBrown" => Some(Self::RosyBrown),
            "royalBlue" => Some(Self::RoyalBlue),
            "saddleBrown" => Some(Self::SaddleBrown),
            "salmon" => Some(Self::Salmon),
            "sandyBrown" => Some(Self::SandyBrown),
            "seaGreen" => Some(Self::SeaGreen),
            "seaShell" => Some(Self::SeaShell),
            "sienna" => Some(Self::Sienna),
            "silver" => Some(Self::Silver),
            "skyBlue" => Some(Self::SkyBlue),
            "slateBlue" => Some(Self::SlateBlue),
            "slateGray" => Some(Self::SlateGray),
            "snow" => Some(Self::Snow),
            "springGreen" => Some(Self::SpringGreen),
            "steelBlue" => Some(Self::SteelBlue),
            "tan" => Some(Self::Tan),
            "teal" => Some(Self::Teal),
            "thistle" => Some(Self::Thistle),
            "tomato" => Some(Self::Tomato),
            "turquoise" => Some(Self::Turquoise),
            "violet" => Some(Self::Violet),
            "wheat" => Some(Self::Wheat),
            "white" => Some(Self::White),
            "whiteSmoke" => Some(Self::WhiteSmoke),
            "yellow" => Some(Self::Yellow),
            "yellowGreen" => Some(Self::YellowGreen),
            "darkBlue" => Some(Self::DarkBlue2010),
            "darkCyan" => Some(Self::DarkCyan2010),
            "darkGoldenrod" => Some(Self::DarkGoldenrod2010),
            "darkGray" => Some(Self::DarkGray2010),
            "darkGrey" => Some(Self::DarkGrey2010),
            "darkGreen" => Some(Self::DarkGreen2010),
            "darkKhaki" => Some(Self::DarkKhaki2010),
            "darkMagenta" => Some(Self::DarkMagenta2010),
            "darkOliveGreen" => Some(Self::DarkOliveGreen2010),
            "darkOrange" => Some(Self::DarkOrange2010),
            "darkOrchid" => Some(Self::DarkOrchid2010),
            "darkRed" => Some(Self::DarkRed2010),
            "darkSalmon" => Some(Self::DarkSalmon2010),
            "darkSeaGreen" => Some(Self::DarkSeaGreen2010),
            "darkSlateBlue" => Some(Self::DarkSlateBlue2010),
            "darkSlateGray" => Some(Self::DarkSlateGray2010),
            "darkSlateGrey" => Some(Self::DarkSlateGrey2010),
            "darkTurquoise" => Some(Self::DarkTurquoise2010),
            "darkViolet" => Some(Self::DarkViolet2010),
            "lightBlue" => Some(Self::LightBlue2010),
            "lightCoral" => Some(Self::LightCoral2010),
            "lightCyan" => Some(Self::LightCyan2010),
            "lightGoldenrodYellow" => Some(Self::LightGoldenrodYellow2010),
            "lightGray" => Some(Self::LightGray2010),
            "lightGrey" => Some(Self::LightGrey2010),
            "lightGreen" => Some(Self::LightGreen2010),
            "lightPink" => Some(Self::LightPink2010),
            "lightSalmon" => Some(Self::LightSalmon2010),
            "lightSeaGreen" => Some(Self::LightSeaGreen2010),
            "lightSkyBlue" => Some(Self::LightSkyBlue2010),
            "lightSlateGray" => Some(Self::LightSlateGray2010),
            "lightSlateGrey" => Some(Self::LightSlateGrey2010),
            "lightSteelBlue" => Some(Self::LightSteelBlue2010),
            "lightYellow" => Some(Self::LightYellow2010),
            "mediumAquamarine" => Some(Self::MediumAquamarine2010),
            "mediumBlue" => Some(Self::MediumBlue2010),
            "mediumOrchid" => Some(Self::MediumOrchid2010),
            "mediumPurple" => Some(Self::MediumPurple2010),
            "mediumSeaGreen" => Some(Self::MediumSeaGreen2010),
            "mediumSlateBlue" => Some(Self::MediumSlateBlue2010),
            "mediumSpringGreen" => Some(Self::MediumSpringGreen2010),
            "mediumTurquoise" => Some(Self::MediumTurquoise2010),
            "mediumVioletRed" => Some(Self::MediumVioletRed2010),
            "dkGrey" => Some(Self::DarkGrey),
            "dimGrey" => Some(Self::DimGrey),
            "dkSlateGrey" => Some(Self::DarkSlateGrey),
            "grey" => Some(Self::Grey),
            "ltGrey" => Some(Self::LightGrey),
            "ltSlateGrey" => Some(Self::LightSlateGrey),
            "slateGrey" => Some(Self::SlateGrey),
            _ => None,
        }
    }
}

impl core::fmt::Display for PresetColorValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::simple_types::OpenXmlSimpleType for PresetColorValues {
    fn as_inner_text(&self) -> String {
        self.as_str().to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Self::from_str(text)
    }
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 383;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 351;
/// Number of generated enums.
pub const ENUM_COUNT: usize = 47;
/// Number of generated content-model particles.
pub const PARTICLE_COUNT: usize = 233;
