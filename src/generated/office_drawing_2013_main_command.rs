//! Auto-generated from `schemas_microsoft_com_office_drawing_2013_main_command.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2013/main/command` (prefix `oac`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2013/main/command";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "oac";

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

static ATTRS_SHAPE_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":creationId", property_name: None, type_name: "StringValue" },
];
static ATTRS_GROUP_SHAPE_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":creationId", property_name: None, type_name: "StringValue" },
];
static ATTRS_GRAPHIC_FRAME_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":creationId", property_name: None, type_name: "StringValue" },
];
static ATTRS_CONNECTOR_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":creationId", property_name: None, type_name: "StringValue" },
];
static ATTRS_PICTURE_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":creationId", property_name: None, type_name: "StringValue" },
];
static ATTRS_INK_MONIKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":creationId", property_name: None, type_name: "StringValue" },
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
static ATTRS_RESOURCE_URL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":src", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":linkage", property_name: None, type_name: "EnumValue" },
];
static ATTRS_GROUP_COMMAND: &[AttributeInfo] = &[
    AttributeInfo { qname: ":verId", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":preventRegroup", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":grpId", property_name: None, type_name: "UInt32Value" },
];
static CHILDREN_GROUP_COMMAND: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_DrawingMonikerList/oac:dgMkLst", property_name: Some("DrawingMonikerList") },
    ChildInfo { name: "oac:CT_ShapeMoniker/oac:spMk", property_name: None },
    ChildInfo { name: "oac:CT_GroupShapeMoniker/oac:grpSpMk", property_name: None },
    ChildInfo { name: "oac:CT_GraphicFrameMoniker/oac:graphicFrameMk", property_name: None },
    ChildInfo { name: "oac:CT_ConnectorMoniker/oac:cxnSpMk", property_name: None },
    ChildInfo { name: "oac:CT_PictureMoniker/oac:picMk", property_name: None },
    ChildInfo { name: "oac:CT_InkMoniker/oac:inkMk", property_name: None },
    ChildInfo { name: "a:CT_GroupShapeProperties/oac:grpSpPr", property_name: None },
    ChildInfo { name: "a:CT_NonVisualDrawingProps/oac:cNvPr", property_name: None },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapeProps/oac:cNvGrpSpPr", property_name: None },
];
static ATTRS_IMG_LINK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":tgt", property_name: None, type_name: "StringValue" },
];
static ATTRS_MODIFY_NON_VISUAL_DRAWING_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":descr", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":title", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":decor", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":scriptLink", property_name: None, type_name: "StringValue" },
];
static ATTRS_MODIFY_TRANSFORM_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":cx", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":cy", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":rot", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":flipH", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":flipV", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_POINT2_D_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x", property_name: Some("X"), type_name: "Int64Value" },
    AttributeInfo { qname: ":y", property_name: Some("Y"), type_name: "Int64Value" },
];
static ATTRS_TEXT_PARAGRAPH_PROPERTIES_TYPE: &[AttributeInfo] = &[
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
static CHILDREN_TEXT_PARAGRAPH_PROPERTIES_TYPE: &[ChildInfo] = &[
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
static ATTRS_TEXT_BODY_PROPERTIES: &[AttributeInfo] = &[
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
static CHILDREN_TEXT_BODY_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PresetTextShape/a:prstTxWarp", property_name: Some("PresetTextWarp") },
    ChildInfo { name: "a:CT_TextNoAutofit/a:noAutofit", property_name: None },
    ChildInfo { name: "a:CT_TextNormalAutofit/a:normAutofit", property_name: None },
    ChildInfo { name: "a:CT_TextShapeAutofit/a:spAutoFit", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/a:scene3d", property_name: None },
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: None },
    ChildInfo { name: "a:CT_FlatText/a:flatTx", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: None },
];
static ATTRS_MODIFY_NON_VISUAL_DRAWING_SHAPE_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noEditPoints", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noAdjustHandles", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeArrowheads", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeShapeType", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noTextEdit", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":txBox", property_name: None, type_name: "BooleanValue" },
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
static CHILDREN_RESET_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_Empty/oac:xfrm", property_name: Some("XfrmEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:geom", property_name: Some("GeomEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:fill", property_name: Some("FillEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:ln", property_name: Some("LnEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:effect", property_name: Some("EffectEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:scene3d", property_name: Some("Scene3dEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:sp3d", property_name: Some("Sp3dEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:extLst", property_name: Some("ExtLstEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:bwMode", property_name: Some("BwModeEmpty") },
];
static ATTRS_LN_REF_STYLE_MATRIX_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_LN_REF_STYLE_MATRIX_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_FILL_REF_STYLE_MATRIX_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_FILL_REF_STYLE_MATRIX_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
];
static ATTRS_EFFECT_REF_STYLE_MATRIX_REFERENCE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static CHILDREN_EFFECT_REF_STYLE_MATRIX_REFERENCE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: Some("RgbColorModelPercentage") },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: Some("RgbColorModelHex") },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: Some("HslColor") },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: Some("SystemColor") },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: Some("SchemeColor") },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: Some("PresetColor") },
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
static CHILDREN_MODIFY_SHAPE_STYLE_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_StyleMatrixReference/oac:lnRef", property_name: Some("LnRefStyleMatrixReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/oac:fillRef", property_name: Some("FillRefStyleMatrixReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/oac:effectRef", property_name: Some("EffectRefStyleMatrixReference") },
    ChildInfo { name: "a:CT_FontReference/oac:fontRef", property_name: Some("FontReference") },
];
static ATTRS_BLIP_FILL_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":dpi", property_name: Some("Dpi"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":rotWithShape", property_name: Some("RotateWithShape"), type_name: "BooleanValue" },
];
static CHILDREN_BLIP_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Blip/a:blip", property_name: Some("Blip") },
    ChildInfo { name: "a:CT_RelativeRect/a:srcRect", property_name: Some("SourceRectangle") },
    ChildInfo { name: "a:CT_TileInfoProperties/a:tile", property_name: None },
    ChildInfo { name: "a:CT_StretchInfoProperties/a:stretch", property_name: None },
];
static ATTRS_FILL_RECT_RELATIVE_RECT_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":r", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: None, type_name: "Int32Value" },
];
static ATTRS_SRC_RECT_RELATIVE_RECT_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":r", property_name: None, type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: None, type_name: "Int32Value" },
];
static CHILDREN_RESET_BLIP_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_Empty/oac:srcRect", property_name: Some("SrcRectEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:fillMode", property_name: Some("FillModeEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:dpi", property_name: Some("DpiEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:rotWithShape", property_name: Some("RotWithShapeEmpty") },
];
static ATTRS_MODIFY_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noUngrp", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_GROUP_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
];
static CHILDREN_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
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
static CHILDREN_RESET_GROUP_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_Empty/oac:xfrm", property_name: Some("XfrmEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:fill", property_name: Some("FillEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:effect", property_name: Some("EffectEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:scene3d", property_name: Some("Scene3dEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:extLst", property_name: Some("ExtLstEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:bwMode", property_name: Some("BwModeEmpty") },
];
static ATTRS_NON_VISUAL_DRAWING_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":descr", property_name: Some("Description"), type_name: "StringValue" },
    AttributeInfo { qname: ":hidden", property_name: Some("Hidden"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":title", property_name: Some("Title"), type_name: "StringValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkClick", property_name: Some("HyperlinkOnClick") },
    ChildInfo { name: "a:CT_Hyperlink/a:hlinkHover", property_name: Some("HyperlinkOnHover") },
    ChildInfo { name: "a:CT_NonVisualDrawingPropsExtensionList/a:extLst", property_name: Some("NonVisualDrawingPropertiesExtensionList") },
];
static CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_GroupLocking/a:grpSpLocks", property_name: Some("GroupShapeLocks") },
    ChildInfo { name: "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst", property_name: Some("NonVisualGroupDrawingShapePropsExtensionList") },
];
static ATTRS_MODIFY_NON_VISUAL_GRAPHIC_FRAME_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noDrilldown", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_ST_CXN_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static ATTRS_END_CXN_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
];
static ATTRS_MODIFY_NON_VISUAL_CONNECTOR_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noEditPoints", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noAdjustHandles", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeArrowheads", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeShapeType", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_MODIFY_NON_VISUAL_CONNECTOR_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Connection/oac:stCxn", property_name: Some("StCxnConnection") },
    ChildInfo { name: "a:CT_Connection/oac:endCxn", property_name: Some("EndCxnConnection") },
];
static CHILDREN_RESET_NON_VISUAL_CONNECTOR_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_Empty/oac:stCxn", property_name: Some("StCxnEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:endCxn", property_name: Some("EndCxnEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noGrp", property_name: Some("NoGrpEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noSelect", property_name: Some("NoSelectEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noRot", property_name: Some("NoRotEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noChangeAspect", property_name: Some("NoChangeAspectEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noMove", property_name: Some("NoMoveEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noResize", property_name: Some("NoResizeEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noEditPoints", property_name: Some("NoEditPointsEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noAdjustHandles", property_name: Some("NoAdjustHandlesEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noChangeArrowheads", property_name: Some("NoChangeArrowheadsEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:noChangeShapeType", property_name: Some("NoChangeShapeTypeEmpty") },
];
static ATTRS_COMPRESS_PICTURE_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":removeCrop", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":useLocalDpi", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":cstate", property_name: None, type_name: "EnumValue" },
];
static ATTRS_MODIFY_NON_VISUAL_PICTURE_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noEditPoints", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noAdjustHandles", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeArrowheads", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeShapeType", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noCrop", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":preferRelativeResize", property_name: None, type_name: "BooleanValue" },
];
static CHILDREN_RESET_NON_VISUAL_PICTURE_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_Empty/oac:lfPr", property_name: Some("LfPrEmpty") },
];
static ATTRS_BOUND_RECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":l", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":t", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":r", property_name: None, type_name: "Int64Value" },
    AttributeInfo { qname: ":b", property_name: None, type_name: "Int64Value" },
];
static ATTRS_LINE_PROPERTIES_TYPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_LINE_PROPERTIES_TYPE: &[ChildInfo] = &[
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
static ATTRS_MODIFY_NON_VISUAL_INK_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":noGrp", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noSelect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noRot", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeAspect", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noMove", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noResize", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noEditPoints", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noAdjustHandles", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeArrowheads", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":noChangeShapeType", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":isComment", property_name: None, type_name: "BooleanValue" },
];
static ATTRS_HLINK_CLICK_HYPERLINK_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":source", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":action", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":tgtFrame", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":tooltip", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":highlightClick", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":endSnd", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":sndName", property_name: None, type_name: "StringValue" },
];
static CHILDREN_HLINK_CLICK_HYPERLINK_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_ImgData/oac:sndData", property_name: Some("SndDataImgData") },
];
static ATTRS_HLINK_HOVER_HYPERLINK_PROPS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":source", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":action", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":tgtFrame", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":tooltip", property_name: None, type_name: "StringValue" },
    AttributeInfo { qname: ":highlightClick", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":endSnd", property_name: None, type_name: "BooleanValue" },
    AttributeInfo { qname: ":sndName", property_name: None, type_name: "StringValue" },
];
static CHILDREN_HLINK_HOVER_HYPERLINK_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_ImgData/oac:sndData", property_name: Some("SndDataImgData") },
];
static CHILDREN_MODIFY_HYPERLINK_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_HyperlinkProps/oac:hlinkClick", property_name: Some("HlinkClickHyperlinkProps") },
    ChildInfo { name: "oac:CT_HyperlinkProps/oac:hlinkHover", property_name: Some("HlinkHoverHyperlinkProps") },
];
static CHILDREN_RESET_HYPERLINK_PROPS: &[ChildInfo] = &[
    ChildInfo { name: "oac:CT_Empty/oac:hlinkClick", property_name: Some("HlinkClickEmpty") },
    ChildInfo { name: "oac:CT_Empty/oac:hlinkHover", property_name: Some("HlinkHoverEmpty") },
];
static ATTRS_TEXT_CHAR_RANGE_CONTEXT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":len", property_name: None, type_name: "UInt32Value" },
    AttributeInfo { qname: ":hash", property_name: None, type_name: "UInt32Value" },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ShapeMoniker", local_name: "spMk", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHAPE_MONIKER, children: &[] },
    ElementInfo { class_name: "GroupShapeMoniker", local_name: "grpSpMk", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_MONIKER, children: &[] },
    ElementInfo { class_name: "GraphicFrameMoniker", local_name: "graphicFrameMk", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_GRAPHIC_FRAME_MONIKER, children: &[] },
    ElementInfo { class_name: "ConnectorMoniker", local_name: "cxnSpMk", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CONNECTOR_MONIKER, children: &[] },
    ElementInfo { class_name: "PictureMoniker", local_name: "picMk", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PICTURE_MONIKER, children: &[] },
    ElementInfo { class_name: "InkMoniker", local_name: "inkMk", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_INK_MONIKER, children: &[] },
    ElementInfo { class_name: "DrawingMonikerList", local_name: "dgMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Transform2D", local_name: "xfrm", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
    ElementInfo { class_name: "GroupShapeMonikerList", local_name: "grpSpMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DrawingElementPackage", local_name: "dePkg", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DeMkLstDrawingElementMonikerList", local_name: "deMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DeMasterMkLstDrawingElementMonikerList", local_name: "deMasterMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DeSrcMkLstDrawingElementMonikerList", local_name: "deSrcMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DeTgtMkLstDrawingElementMonikerList", local_name: "deTgtMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ImgDataImgData", local_name: "imgData", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "OrigImgDataImgData", local_name: "origImgData", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SndDataImgData", local_name: "sndData", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ResourceUrl", local_name: "imgUrl", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RESOURCE_URL, children: &[] },
    ElementInfo { class_name: "TextBodyPackage", local_name: "txBodyPkg", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "GroupCommand", local_name: "grpCmd", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_COMMAND, children: CHILDREN_GROUP_COMMAND },
    ElementInfo { class_name: "ImgLink", local_name: "imgLink", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_IMG_LINK, children: &[] },
    ElementInfo { class_name: "DocumentContextMonikerList", local_name: "dcMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "GraphicParentMonikerList", local_name: "graphicParentMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ShapeMonikerList", local_name: "spMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "GraphicFrameMonikerList", local_name: "graphicFrameMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ConnectorMonikerList", local_name: "cxnSpMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "PictureMonikerList", local_name: "picMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "InkMonikerList", local_name: "inkMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TextBodyMonikerList", local_name: "txBodyMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TextCharRangeMonikerList", local_name: "txMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "HyperlinkMonikerList", local_name: "hlinkMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Model3DMonikerList", local_name: "model3DMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ViewSelectionStgList", local_name: "viewSelLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "EditorSelectionStgList", local_name: "editorSelLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DrawingSelectionStgList", local_name: "drSelLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TableMonikerList", local_name: "tblMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TableCellMonikerList", local_name: "tcMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TableRowMonikerList", local_name: "trMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "TableColumnMonikerList", local_name: "gridColMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ModifyNonVisualDrawingProps", local_name: "cNvPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFY_NON_VISUAL_DRAWING_PROPS, children: &[] },
    ElementInfo { class_name: "ModifyTransformProps", local_name: "xfrm", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFY_TRANSFORM_PROPS, children: &[] },
    ElementInfo { class_name: "Point2DType", local_name: "off", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_POINT2_D_TYPE, children: &[] },
    ElementInfo { class_name: "TextParagraphPropertiesType", local_name: "pPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_PARAGRAPH_PROPERTIES_TYPE, children: CHILDREN_TEXT_PARAGRAPH_PROPERTIES_TYPE },
    ElementInfo { class_name: "TextBodyProperties", local_name: "bodyPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_BODY_PROPERTIES, children: CHILDREN_TEXT_BODY_PROPERTIES },
    ElementInfo { class_name: "ModifyNonVisualDrawingShapeProps", local_name: "cNvSpPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFY_NON_VISUAL_DRAWING_SHAPE_PROPS, children: &[] },
    ElementInfo { class_name: "ShapePropsMonikerList", local_name: "spMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "XfrmEmpty", local_name: "xfrm", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "GeomEmpty", local_name: "geom", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "FillEmpty", local_name: "fill", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LnEmpty", local_name: "ln", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "EffectEmpty", local_name: "effect", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Scene3dEmpty", local_name: "scene3d", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "Sp3dEmpty", local_name: "sp3d", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ExtLstEmpty", local_name: "extLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "BwModeEmpty", local_name: "bwMode", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "SrcRectEmpty", local_name: "srcRect", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "FillModeEmpty", local_name: "fillMode", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "DpiEmpty", local_name: "dpi", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "RotWithShapeEmpty", local_name: "rotWithShape", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "StCxnEmpty", local_name: "stCxn", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "EndCxnEmpty", local_name: "endCxn", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoGrpEmpty", local_name: "noGrp", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoSelectEmpty", local_name: "noSelect", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoRotEmpty", local_name: "noRot", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoChangeAspectEmpty", local_name: "noChangeAspect", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoMoveEmpty", local_name: "noMove", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoResizeEmpty", local_name: "noResize", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoEditPointsEmpty", local_name: "noEditPoints", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoAdjustHandlesEmpty", local_name: "noAdjustHandles", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoChangeArrowheadsEmpty", local_name: "noChangeArrowheads", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "NoChangeShapeTypeEmpty", local_name: "noChangeShapeType", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LfPrEmpty", local_name: "lfPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "HlinkClickEmpty", local_name: "hlinkClick", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "HlinkHoverEmpty", local_name: "hlinkHover", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "ResetShapeProperties", local_name: "spPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RESET_SHAPE_PROPERTIES },
    ElementInfo { class_name: "LnRefStyleMatrixReference", local_name: "lnRef", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LN_REF_STYLE_MATRIX_REFERENCE, children: CHILDREN_LN_REF_STYLE_MATRIX_REFERENCE },
    ElementInfo { class_name: "FillRefStyleMatrixReference", local_name: "fillRef", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FILL_REF_STYLE_MATRIX_REFERENCE, children: CHILDREN_FILL_REF_STYLE_MATRIX_REFERENCE },
    ElementInfo { class_name: "EffectRefStyleMatrixReference", local_name: "effectRef", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EFFECT_REF_STYLE_MATRIX_REFERENCE, children: CHILDREN_EFFECT_REF_STYLE_MATRIX_REFERENCE },
    ElementInfo { class_name: "FontReference", local_name: "fontRef", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FONT_REFERENCE, children: CHILDREN_FONT_REFERENCE },
    ElementInfo { class_name: "ModifyShapeStyleProps", local_name: "style", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODIFY_SHAPE_STYLE_PROPS },
    ElementInfo { class_name: "ResetXsdboolean", local_name: "reset", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "UseBoundsXsdboolean", local_name: "useBounds", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "BlipFillProperties", local_name: "blipFill", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BLIP_FILL_PROPERTIES, children: CHILDREN_BLIP_FILL_PROPERTIES },
    ElementInfo { class_name: "FillRectRelativeRectProps", local_name: "fillRect", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FILL_RECT_RELATIVE_RECT_PROPS, children: &[] },
    ElementInfo { class_name: "SrcRectRelativeRectProps", local_name: "srcRect", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SRC_RECT_RELATIVE_RECT_PROPS, children: &[] },
    ElementInfo { class_name: "ResetBlipFillProperties", local_name: "blipFill", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RESET_BLIP_FILL_PROPERTIES },
    ElementInfo { class_name: "ModifyNonVisualGroupDrawingShapeProps", local_name: "cNvGrpSpPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFY_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS, children: &[] },
    ElementInfo { class_name: "GroupShapeProperties", local_name: "grpSpPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GROUP_SHAPE_PROPERTIES, children: CHILDREN_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ResetGroupShapeProperties", local_name: "grpSpPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RESET_GROUP_SHAPE_PROPERTIES },
    ElementInfo { class_name: "NonVisualDrawingProps", local_name: "cNvPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPS, children: CHILDREN_NON_VISUAL_DRAWING_PROPS },
    ElementInfo { class_name: "NonVisualGroupDrawingShapeProps", local_name: "cNvGrpSpPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_GROUP_DRAWING_SHAPE_PROPS },
    ElementInfo { class_name: "ModifyNonVisualGraphicFrameProps", local_name: "cNvGraphicFramePr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFY_NON_VISUAL_GRAPHIC_FRAME_PROPS, children: &[] },
    ElementInfo { class_name: "StCxnConnection", local_name: "stCxn", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ST_CXN_CONNECTION, children: &[] },
    ElementInfo { class_name: "EndCxnConnection", local_name: "endCxn", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_END_CXN_CONNECTION, children: &[] },
    ElementInfo { class_name: "ModifyNonVisualConnectorProps", local_name: "cNvCxnSpPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_MODIFY_NON_VISUAL_CONNECTOR_PROPS, children: CHILDREN_MODIFY_NON_VISUAL_CONNECTOR_PROPS },
    ElementInfo { class_name: "ResetNonVisualConnectorProps", local_name: "cNvCxnSpPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RESET_NON_VISUAL_CONNECTOR_PROPS },
    ElementInfo { class_name: "CompressPictureProps", local_name: "compressPicPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMPRESS_PICTURE_PROPS, children: &[] },
    ElementInfo { class_name: "ModifyNonVisualPictureProps", local_name: "cNvPicPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFY_NON_VISUAL_PICTURE_PROPS, children: &[] },
    ElementInfo { class_name: "ResetNonVisualPictureProps", local_name: "cNvPicPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RESET_NON_VISUAL_PICTURE_PROPS },
    ElementInfo { class_name: "BoundRect", local_name: "bounds", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BOUND_RECT, children: &[] },
    ElementInfo { class_name: "SVGBlipMonikerList", local_name: "svgBlipMkLst", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "LinePropertiesType", local_name: "lineProps", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_PROPERTIES_TYPE, children: CHILDREN_LINE_PROPERTIES_TYPE },
    ElementInfo { class_name: "ModifyNonVisualInkProps", local_name: "cNvInkPr", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MODIFY_NON_VISUAL_INK_PROPS, children: &[] },
    ElementInfo { class_name: "HlinkClickHyperlinkProps", local_name: "hlinkClick", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HLINK_CLICK_HYPERLINK_PROPS, children: CHILDREN_HLINK_CLICK_HYPERLINK_PROPS },
    ElementInfo { class_name: "HlinkHoverHyperlinkProps", local_name: "hlinkHover", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HLINK_HOVER_HYPERLINK_PROPS, children: CHILDREN_HLINK_HOVER_HYPERLINK_PROPS },
    ElementInfo { class_name: "ModifyHyperlinkProps", local_name: "hlink", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_MODIFY_HYPERLINK_PROPS },
    ElementInfo { class_name: "ResetHyperlinkProps", local_name: "hlink", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RESET_HYPERLINK_PROPS },
    ElementInfo { class_name: "TextCharRangeContext", local_name: "context", prefix: "oac", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TEXT_CHAR_RANGE_CONTEXT, children: &[] },
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

/// Create a `<oac:spMk>` element (`ShapeMoniker`).
pub fn shape_moniker() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "spMk")
}

/// Create a `<oac:grpSpMk>` element (`GroupShapeMoniker`).
pub fn group_shape_moniker() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "grpSpMk")
}

/// Create a `<oac:graphicFrameMk>` element (`GraphicFrameMoniker`).
pub fn graphic_frame_moniker() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "graphicFrameMk")
}

/// Create a `<oac:cxnSpMk>` element (`ConnectorMoniker`).
pub fn connector_moniker() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cxnSpMk")
}

/// Create a `<oac:picMk>` element (`PictureMoniker`).
pub fn picture_moniker() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "picMk")
}

/// Create a `<oac:inkMk>` element (`InkMoniker`).
pub fn ink_moniker() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "inkMk")
}

/// Create a `<oac:dgMkLst>` element (`DrawingMonikerList`).
pub fn drawing_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "dgMkLst").with_children(children)
}

/// Create a `<oac:xfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Create a `<oac:grpSpMkLst>` element (`GroupShapeMonikerList`).
pub fn group_shape_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "grpSpMkLst").with_children(children)
}

/// Create a `<oac:dePkg>` element (`DrawingElementPackage`).
pub fn drawing_element_package() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "dePkg")
}

/// Create a `<oac:deMkLst>` element (`DeMkLstDrawingElementMonikerList`).
pub fn de_mk_lst_drawing_element_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "deMkLst").with_children(children)
}

/// Create a `<oac:deMasterMkLst>` element (`DeMasterMkLstDrawingElementMonikerList`).
pub fn de_master_mk_lst_drawing_element_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "deMasterMkLst").with_children(children)
}

/// Create a `<oac:deSrcMkLst>` element (`DeSrcMkLstDrawingElementMonikerList`).
pub fn de_src_mk_lst_drawing_element_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "deSrcMkLst").with_children(children)
}

/// Create a `<oac:deTgtMkLst>` element (`DeTgtMkLstDrawingElementMonikerList`).
pub fn de_tgt_mk_lst_drawing_element_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "deTgtMkLst").with_children(children)
}

/// Create a `<oac:imgData>` element (`ImgDataImgData`).
pub fn img_data_img_data(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "imgData").with_text(value)
}

/// Create a `<oac:origImgData>` element (`OrigImgDataImgData`).
pub fn orig_img_data_img_data(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "origImgData").with_text(value)
}

/// Create a `<oac:sndData>` element (`SndDataImgData`).
pub fn snd_data_img_data(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "sndData").with_text(value)
}

/// Create a `<oac:imgUrl>` element (`ResourceUrl`).
pub fn resource_url() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "imgUrl")
}

/// Create a `<oac:txBodyPkg>` element (`TextBodyPackage`).
pub fn text_body_package() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "txBodyPkg")
}

/// Create a `<oac:grpCmd>` element (`GroupCommand`).
pub fn group_command(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "grpCmd").with_children(children)
}

/// Create a `<oac:imgLink>` element (`ImgLink`).
pub fn img_link() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "imgLink")
}

/// Create a `<oac:dcMkLst>` element (`DocumentContextMonikerList`).
pub fn document_context_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "dcMkLst").with_children(children)
}

/// Create a `<oac:graphicParentMkLst>` element (`GraphicParentMonikerList`).
pub fn graphic_parent_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "graphicParentMkLst").with_children(children)
}

/// Create a `<oac:spMkLst>` element (`ShapeMonikerList`).
pub fn shape_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "spMkLst").with_children(children)
}

/// Create a `<oac:graphicFrameMkLst>` element (`GraphicFrameMonikerList`).
pub fn graphic_frame_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "graphicFrameMkLst").with_children(children)
}

/// Create a `<oac:cxnSpMkLst>` element (`ConnectorMonikerList`).
pub fn connector_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cxnSpMkLst").with_children(children)
}

/// Create a `<oac:picMkLst>` element (`PictureMonikerList`).
pub fn picture_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "picMkLst").with_children(children)
}

/// Create a `<oac:inkMkLst>` element (`InkMonikerList`).
pub fn ink_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "inkMkLst").with_children(children)
}

/// Create a `<oac:txBodyMkLst>` element (`TextBodyMonikerList`).
pub fn text_body_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "txBodyMkLst").with_children(children)
}

/// Create a `<oac:txMkLst>` element (`TextCharRangeMonikerList`).
pub fn text_char_range_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "txMkLst").with_children(children)
}

/// Create a `<oac:hlinkMkLst>` element (`HyperlinkMonikerList`).
pub fn hyperlink_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "hlinkMkLst").with_children(children)
}

/// Create a `<oac:model3DMkLst>` element (`Model3DMonikerList`).
pub fn model3_d_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "model3DMkLst").with_children(children)
}

/// Create a `<oac:viewSelLst>` element (`ViewSelectionStgList`).
pub fn view_selection_stg_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "viewSelLst").with_children(children)
}

/// Create a `<oac:editorSelLst>` element (`EditorSelectionStgList`).
pub fn editor_selection_stg_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "editorSelLst").with_children(children)
}

/// Create a `<oac:drSelLst>` element (`DrawingSelectionStgList`).
pub fn drawing_selection_stg_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "drSelLst").with_children(children)
}

/// Create a `<oac:tblMkLst>` element (`TableMonikerList`).
pub fn table_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "tblMkLst").with_children(children)
}

/// Create a `<oac:tcMkLst>` element (`TableCellMonikerList`).
pub fn table_cell_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "tcMkLst").with_children(children)
}

/// Create a `<oac:trMkLst>` element (`TableRowMonikerList`).
pub fn table_row_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "trMkLst").with_children(children)
}

/// Create a `<oac:gridColMkLst>` element (`TableColumnMonikerList`).
pub fn table_column_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "gridColMkLst").with_children(children)
}

/// Create a `<oac:cNvPr>` element (`ModifyNonVisualDrawingProps`).
pub fn modify_non_visual_drawing_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvPr")
}

/// Create a `<oac:xfrm>` element (`ModifyTransformProps`).
pub fn modify_transform_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "xfrm")
}

/// Create a `<oac:off>` element (`Point2DType`).
pub fn point2_d_type() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "off")
}

/// Create a `<oac:pPr>` element (`TextParagraphPropertiesType`).
pub fn text_paragraph_properties_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "pPr").with_children(children)
}

/// Create a `<oac:bodyPr>` element (`TextBodyProperties`).
pub fn text_body_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "bodyPr").with_children(children)
}

/// Create a `<oac:cNvSpPr>` element (`ModifyNonVisualDrawingShapeProps`).
pub fn modify_non_visual_drawing_shape_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvSpPr")
}

/// Create a `<oac:spMkLst>` element (`ShapePropsMonikerList`).
pub fn shape_props_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "spMkLst").with_children(children)
}

/// Create a `<oac:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<oac:xfrm>` element (`XfrmEmpty`).
pub fn xfrm_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "xfrm")
}

/// Create a `<oac:geom>` element (`GeomEmpty`).
pub fn geom_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "geom")
}

/// Create a `<oac:fill>` element (`FillEmpty`).
pub fn fill_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "fill")
}

/// Create a `<oac:ln>` element (`LnEmpty`).
pub fn ln_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "ln")
}

/// Create a `<oac:effect>` element (`EffectEmpty`).
pub fn effect_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "effect")
}

/// Create a `<oac:scene3d>` element (`Scene3dEmpty`).
pub fn scene3d_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "scene3d")
}

/// Create a `<oac:sp3d>` element (`Sp3dEmpty`).
pub fn sp3d_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "sp3d")
}

/// Create a `<oac:extLst>` element (`ExtLstEmpty`).
pub fn ext_lst_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "extLst")
}

/// Create a `<oac:bwMode>` element (`BwModeEmpty`).
pub fn bw_mode_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "bwMode")
}

/// Create a `<oac:srcRect>` element (`SrcRectEmpty`).
pub fn src_rect_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "srcRect")
}

/// Create a `<oac:fillMode>` element (`FillModeEmpty`).
pub fn fill_mode_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "fillMode")
}

/// Create a `<oac:dpi>` element (`DpiEmpty`).
pub fn dpi_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "dpi")
}

/// Create a `<oac:rotWithShape>` element (`RotWithShapeEmpty`).
pub fn rot_with_shape_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "rotWithShape")
}

/// Create a `<oac:stCxn>` element (`StCxnEmpty`).
pub fn st_cxn_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "stCxn")
}

/// Create a `<oac:endCxn>` element (`EndCxnEmpty`).
pub fn end_cxn_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "endCxn")
}

/// Create a `<oac:noGrp>` element (`NoGrpEmpty`).
pub fn no_grp_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noGrp")
}

/// Create a `<oac:noSelect>` element (`NoSelectEmpty`).
pub fn no_select_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noSelect")
}

/// Create a `<oac:noRot>` element (`NoRotEmpty`).
pub fn no_rot_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noRot")
}

/// Create a `<oac:noChangeAspect>` element (`NoChangeAspectEmpty`).
pub fn no_change_aspect_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noChangeAspect")
}

/// Create a `<oac:noMove>` element (`NoMoveEmpty`).
pub fn no_move_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noMove")
}

/// Create a `<oac:noResize>` element (`NoResizeEmpty`).
pub fn no_resize_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noResize")
}

/// Create a `<oac:noEditPoints>` element (`NoEditPointsEmpty`).
pub fn no_edit_points_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noEditPoints")
}

/// Create a `<oac:noAdjustHandles>` element (`NoAdjustHandlesEmpty`).
pub fn no_adjust_handles_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noAdjustHandles")
}

/// Create a `<oac:noChangeArrowheads>` element (`NoChangeArrowheadsEmpty`).
pub fn no_change_arrowheads_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noChangeArrowheads")
}

/// Create a `<oac:noChangeShapeType>` element (`NoChangeShapeTypeEmpty`).
pub fn no_change_shape_type_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "noChangeShapeType")
}

/// Create a `<oac:lfPr>` element (`LfPrEmpty`).
pub fn lf_pr_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "lfPr")
}

/// Create a `<oac:hlinkClick>` element (`HlinkClickEmpty`).
pub fn hlink_click_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "hlinkClick")
}

/// Create a `<oac:hlinkHover>` element (`HlinkHoverEmpty`).
pub fn hlink_hover_empty() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "hlinkHover")
}

/// Create a `<oac:spPr>` element (`ResetShapeProperties`).
pub fn reset_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<oac:lnRef>` element (`LnRefStyleMatrixReference`).
pub fn ln_ref_style_matrix_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "lnRef").with_children(children)
}

/// Create a `<oac:fillRef>` element (`FillRefStyleMatrixReference`).
pub fn fill_ref_style_matrix_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "fillRef").with_children(children)
}

/// Create a `<oac:effectRef>` element (`EffectRefStyleMatrixReference`).
pub fn effect_ref_style_matrix_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "effectRef").with_children(children)
}

/// Create a `<oac:fontRef>` element (`FontReference`).
pub fn font_reference(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "fontRef").with_children(children)
}

/// Create a `<oac:style>` element (`ModifyShapeStyleProps`).
pub fn modify_shape_style_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<oac:reset>` element (`ResetXsdboolean`).
pub fn reset_xsdboolean(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "reset").with_text(value)
}

/// Create a `<oac:useBounds>` element (`UseBoundsXsdboolean`).
pub fn use_bounds_xsdboolean(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "useBounds").with_text(value)
}

/// Create a `<oac:blipFill>` element (`BlipFillProperties`).
pub fn blip_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "blipFill").with_children(children)
}

/// Create a `<oac:fillRect>` element (`FillRectRelativeRectProps`).
pub fn fill_rect_relative_rect_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "fillRect")
}

/// Create a `<oac:srcRect>` element (`SrcRectRelativeRectProps`).
pub fn src_rect_relative_rect_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "srcRect")
}

/// Create a `<oac:blipFill>` element (`ResetBlipFillProperties`).
pub fn reset_blip_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "blipFill").with_children(children)
}

/// Create a `<oac:cNvGrpSpPr>` element (`ModifyNonVisualGroupDrawingShapeProps`).
pub fn modify_non_visual_group_drawing_shape_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvGrpSpPr")
}

/// Create a `<oac:grpSpPr>` element (`GroupShapeProperties`).
pub fn group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Create a `<oac:grpSpPr>` element (`ResetGroupShapeProperties`).
pub fn reset_group_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "grpSpPr").with_children(children)
}

/// Create a `<oac:cNvPr>` element (`NonVisualDrawingProps`).
pub fn non_visual_drawing_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<oac:cNvGrpSpPr>` element (`NonVisualGroupDrawingShapeProps`).
pub fn non_visual_group_drawing_shape_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvGrpSpPr").with_children(children)
}

/// Create a `<oac:cNvGraphicFramePr>` element (`ModifyNonVisualGraphicFrameProps`).
pub fn modify_non_visual_graphic_frame_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvGraphicFramePr")
}

/// Create a `<oac:stCxn>` element (`StCxnConnection`).
pub fn st_cxn_connection() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "stCxn")
}

/// Create a `<oac:endCxn>` element (`EndCxnConnection`).
pub fn end_cxn_connection() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "endCxn")
}

/// Create a `<oac:cNvCxnSpPr>` element (`ModifyNonVisualConnectorProps`).
pub fn modify_non_visual_connector_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvCxnSpPr").with_children(children)
}

/// Create a `<oac:cNvCxnSpPr>` element (`ResetNonVisualConnectorProps`).
pub fn reset_non_visual_connector_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvCxnSpPr").with_children(children)
}

/// Create a `<oac:compressPicPr>` element (`CompressPictureProps`).
pub fn compress_picture_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "compressPicPr")
}

/// Create a `<oac:cNvPicPr>` element (`ModifyNonVisualPictureProps`).
pub fn modify_non_visual_picture_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvPicPr")
}

/// Create a `<oac:cNvPicPr>` element (`ResetNonVisualPictureProps`).
pub fn reset_non_visual_picture_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvPicPr").with_children(children)
}

/// Create a `<oac:bounds>` element (`BoundRect`).
pub fn bound_rect() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "bounds")
}

/// Create a `<oac:svgBlipMkLst>` element (`SVGBlipMonikerList`).
pub fn s_v_g_blip_moniker_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "svgBlipMkLst").with_children(children)
}

/// Create a `<oac:lineProps>` element (`LinePropertiesType`).
pub fn line_properties_type(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "lineProps").with_children(children)
}

/// Create a `<oac:cNvInkPr>` element (`ModifyNonVisualInkProps`).
pub fn modify_non_visual_ink_props() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "cNvInkPr")
}

/// Create a `<oac:hlinkClick>` element (`HlinkClickHyperlinkProps`).
pub fn hlink_click_hyperlink_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "hlinkClick").with_children(children)
}

/// Create a `<oac:hlinkHover>` element (`HlinkHoverHyperlinkProps`).
pub fn hlink_hover_hyperlink_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "hlinkHover").with_children(children)
}

/// Create a `<oac:hlink>` element (`ModifyHyperlinkProps`).
pub fn modify_hyperlink_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "hlink").with_children(children)
}

/// Create a `<oac:hlink>` element (`ResetHyperlinkProps`).
pub fn reset_hyperlink_props(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "hlink").with_children(children)
}

/// Create a `<oac:context>` element (`TextCharRangeContext`).
pub fn text_char_range_context() -> OpenXmlElement {
    OpenXmlElement::new("oac", NAMESPACE_URI, "context")
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 116;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 109;
