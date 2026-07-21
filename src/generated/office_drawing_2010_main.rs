//! Auto-generated from `schemas_microsoft_com_office_drawing_2010_main.json`.
//! Target namespace: `http://schemas.microsoft.com/office/drawing/2010/main` (prefix `a14`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/drawing/2010/main";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "a14";

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

static ATTRS_CAMERA_TOOL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":cellRange", property_name: Some("CellRange"), type_name: "StringValue" },
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
];
static ATTRS_COMPAT_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":spid", property_name: Some("ShapeId"), type_name: "StringValue" },
];
static ATTRS_IS_CANVAS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_GVML_CONTENT_PART: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bwMode", property_name: Some("BlackWhiteMode"), type_name: "EnumValue" },
    AttributeInfo { qname: "r:id", property_name: Some("RelationshipId"), type_name: "StringValue" },
];
static CHILDREN_GVML_CONTENT_PART: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_GvmlContentPartNonVisual/a14:nvContentPartPr", property_name: Some("NonVisualContentPartProperties") },
    ChildInfo { name: "a:CT_Transform2D/a14:xfrm", property_name: Some("Transform2D") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a14:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_SHADOW_OBSCURED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static CHILDREN_HIDDEN_FILL_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: Some("NoFill") },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: Some("SolidFill") },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: Some("GradientFill") },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: Some("BlipFill") },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: Some("PatternFill") },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: Some("GroupFill") },
];
static ATTRS_HIDDEN_LINE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":w", property_name: Some("Width"), type_name: "Int32Value" },
    AttributeInfo { qname: ":cap", property_name: Some("CapType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cmpd", property_name: Some("CompoundLineType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":algn", property_name: Some("Alignment"), type_name: "EnumValue" },
];
static CHILDREN_HIDDEN_LINE_PROPERTIES: &[ChildInfo] = &[
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
static CHILDREN_HIDDEN_EFFECTS_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: Some("EffectList") },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: Some("EffectDag") },
];
static CHILDREN_HIDDEN_SCENE3_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Camera/a:camera", property_name: Some("Camera") },
    ChildInfo { name: "a:CT_LightRig/a:lightRig", property_name: Some("LightRig") },
    ChildInfo { name: "a:CT_Backdrop/a:backdrop", property_name: Some("Backdrop") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_HIDDEN_SHAPE3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":z", property_name: Some("Z"), type_name: "Int64Value" },
    AttributeInfo { qname: ":extrusionH", property_name: Some("ExtrusionHeight"), type_name: "Int64Value" },
    AttributeInfo { qname: ":contourW", property_name: Some("ContourWidth"), type_name: "Int64Value" },
    AttributeInfo { qname: ":prstMaterial", property_name: Some("PresetMaterial"), type_name: "EnumValue" },
];
static CHILDREN_HIDDEN_SHAPE3_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Bevel/a:bevelT", property_name: Some("BevelTop") },
    ChildInfo { name: "a:CT_Bevel/a:bevelB", property_name: Some("BevelBottom") },
    ChildInfo { name: "a:CT_Color/a:extrusionClr", property_name: Some("ExtrusionColor") },
    ChildInfo { name: "a:CT_Color/a:contourClr", property_name: Some("ContourColor") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_IMAGE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_PictureLayer/a14:imgLayer", property_name: Some("ImageLayer") },
];
static ATTRS_USE_LOCAL_DPI: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_CONTENT_PART_LOCKS: &[AttributeInfo] = &[
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
static CHILDREN_CONTENT_PART_LOCKS: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a14:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_FOREGROUND_MARK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x1", property_name: Some("FirstXCoordinate"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y1", property_name: Some("FirstYCoordinate"), type_name: "Int32Value" },
    AttributeInfo { qname: ":x2", property_name: Some("SecondXCoordinate"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y2", property_name: Some("SecondYCoordinate"), type_name: "Int32Value" },
];
static ATTRS_BACKGROUND_MARK: &[AttributeInfo] = &[
    AttributeInfo { qname: ":x1", property_name: Some("FirstXCoordinate"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y1", property_name: Some("FirstYCoordinate"), type_name: "Int32Value" },
    AttributeInfo { qname: ":x2", property_name: Some("SecondXCoordinate"), type_name: "Int32Value" },
    AttributeInfo { qname: ":y2", property_name: Some("SecondYCoordinate"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_BLUR: &[AttributeInfo] = &[
    AttributeInfo { qname: ":radius", property_name: Some("Radius"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_CEMENT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":crackSpacing", property_name: Some("CrackSpacing"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_CHALK_SKETCH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":pressure", property_name: Some("Pressure"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_CRISSCROSS_ETCHING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":pressure", property_name: Some("Pressure"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_CUTOUT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":numberOfShades", property_name: Some("NumberOfShades"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_FILM_GRAIN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":grainSize", property_name: Some("GrainSize"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_GLASS: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":scaling", property_name: Some("Scaling"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_GLOW_DIFFUSED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":intensity", property_name: Some("Intensity"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_GLOW_EDGES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":smoothness", property_name: Some("Smoothness"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_LIGHT_SCREEN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":gridSize", property_name: Some("GridSize"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_LINE_DRAWING: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":pencilSize", property_name: Some("PencilSize"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_MARKER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":size", property_name: Some("Size"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_MOSAIC_BUBBLES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":pressure", property_name: Some("Pressure"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_PAINT_STROKES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":intensity", property_name: Some("Intensity"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_PAINT_BRUSH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":brushSize", property_name: Some("BrushSize"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_PASTELS_SMOOTH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":scaling", property_name: Some("BrushSize"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_PENCIL_GRAYSCALE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":pencilSize", property_name: Some("BrushSize"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_PENCIL_SKETCH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":pressure", property_name: Some("Pressure"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_PHOTOCOPY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":detail", property_name: Some("Detail"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_PLASTIC_WRAP: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":smoothness", property_name: Some("Smoothness"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_TEXTURIZER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":scaling", property_name: Some("Scaling"), type_name: "Int32Value" },
];
static ATTRS_ARTISTIC_WATERCOLOR_SPONGE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":trans", property_name: Some("Transparancy"), type_name: "Int32Value" },
    AttributeInfo { qname: ":brushSize", property_name: Some("BrushSize"), type_name: "Int32Value" },
];
static ATTRS_BACKGROUND_REMOVAL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":t", property_name: Some("MarqueeTop"), type_name: "Int32Value" },
    AttributeInfo { qname: ":b", property_name: Some("MarqueeBottom"), type_name: "Int32Value" },
    AttributeInfo { qname: ":l", property_name: Some("MarqueeLeft"), type_name: "Int32Value" },
    AttributeInfo { qname: ":r", property_name: Some("MarqueeRight"), type_name: "Int32Value" },
];
static CHILDREN_BACKGROUND_REMOVAL: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_PictureEffectBackgroundRemovalForegroundMark/a14:foregroundMark", property_name: None },
    ChildInfo { name: "a14:CT_PictureEffectBackgroundRemovalBackgroundMark/a14:backgroundMark", property_name: None },
];
static ATTRS_BRIGHTNESS_CONTRAST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":bright", property_name: Some("Bright"), type_name: "Int32Value" },
    AttributeInfo { qname: ":contrast", property_name: Some("Contrast"), type_name: "Int32Value" },
];
static ATTRS_COLOR_TEMPERATURE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":colorTemp", property_name: Some("ColorTemperatureValue"), type_name: "Int32Value" },
];
static ATTRS_SATURATION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":sat", property_name: Some("SaturationAmount"), type_name: "Int32Value" },
];
static ATTRS_SHARPEN_SOFTEN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":amount", property_name: Some("Amount"), type_name: "Int32Value" },
];
static ATTRS_IMAGE_EFFECT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":visible", property_name: Some("Visible"), type_name: "BooleanValue" },
];
static CHILDREN_IMAGE_EFFECT: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_PictureEffectBlur/a14:artisticBlur", property_name: Some("ArtisticBlur") },
    ChildInfo { name: "a14:CT_PictureEffectCement/a14:artisticCement", property_name: Some("ArtisticCement") },
    ChildInfo { name: "a14:CT_PictureEffectChalkSketch/a14:artisticChalkSketch", property_name: Some("ArtisticChalkSketch") },
    ChildInfo { name: "a14:CT_PictureEffectCrisscrossEtching/a14:artisticCrisscrossEtching", property_name: Some("ArtisticCrisscrossEtching") },
    ChildInfo { name: "a14:CT_PictureEffectCutout/a14:artisticCutout", property_name: Some("ArtisticCutout") },
    ChildInfo { name: "a14:CT_PictureEffectFilmGrain/a14:artisticFilmGrain", property_name: Some("ArtisticFilmGrain") },
    ChildInfo { name: "a14:CT_PictureEffectGlass/a14:artisticGlass", property_name: Some("ArtisticGlass") },
    ChildInfo { name: "a14:CT_PictureEffectGlowDiffused/a14:artisticGlowDiffused", property_name: Some("ArtisticGlowDiffused") },
    ChildInfo { name: "a14:CT_PictureEffectGlowEdges/a14:artisticGlowEdges", property_name: Some("ArtisticGlowEdges") },
    ChildInfo { name: "a14:CT_PictureEffectLightScreen/a14:artisticLightScreen", property_name: Some("ArtisticLightScreen") },
    ChildInfo { name: "a14:CT_PictureEffectLineDrawing/a14:artisticLineDrawing", property_name: Some("ArtisticLineDrawing") },
    ChildInfo { name: "a14:CT_PictureEffectMarker/a14:artisticMarker", property_name: Some("ArtisticMarker") },
    ChildInfo { name: "a14:CT_PictureEffectMosiaicBubbles/a14:artisticMosiaicBubbles", property_name: Some("ArtisticMosaicBubbles") },
    ChildInfo { name: "a14:CT_PictureEffectPaintStrokes/a14:artisticPaintStrokes", property_name: Some("ArtisticPaintStrokes") },
    ChildInfo { name: "a14:CT_PictureEffectPaintBrush/a14:artisticPaintBrush", property_name: Some("ArtisticPaintBrush") },
    ChildInfo { name: "a14:CT_PictureEffectPastelsSmooth/a14:artisticPastelsSmooth", property_name: Some("ArtisticPastelsSmooth") },
    ChildInfo { name: "a14:CT_PictureEffectPencilGrayscale/a14:artisticPencilGrayscale", property_name: Some("ArtisticPencilGrayscale") },
    ChildInfo { name: "a14:CT_PictureEffectPencilSketch/a14:artisticPencilSketch", property_name: Some("ArtisticPencilSketch") },
    ChildInfo { name: "a14:CT_PictureEffectPhotocopy/a14:artisticPhotocopy", property_name: Some("ArtisticPhotocopy") },
    ChildInfo { name: "a14:CT_PictureEffectPlasticWrap/a14:artisticPlasticWrap", property_name: Some("ArtisticPlasticWrap") },
    ChildInfo { name: "a14:CT_PictureEffectTexturizer/a14:artisticTexturizer", property_name: Some("ArtisticTexturizer") },
    ChildInfo { name: "a14:CT_PictureEffectWatercolorSponge/a14:artisticWatercolorSponge", property_name: Some("ArtisticWatercolorSponge") },
    ChildInfo { name: "a14:CT_PictureEffectBackgroundRemoval/a14:backgroundRemoval", property_name: Some("BackgroundRemoval") },
    ChildInfo { name: "a14:CT_PictureEffectBrightnessContrast/a14:brightnessContrast", property_name: Some("BrightnessContrast") },
    ChildInfo { name: "a14:CT_PictureEffectColorTemperature/a14:colorTemperature", property_name: Some("ColorTemperature") },
    ChildInfo { name: "a14:CT_PictureEffectSaturation/a14:saturation", property_name: Some("Saturation") },
    ChildInfo { name: "a14:CT_PictureEffectSharpenSoften/a14:sharpenSoften", property_name: Some("SharpenSoften") },
];
static ATTRS_IMAGE_LAYER: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:embed", property_name: Some("Embed"), type_name: "StringValue" },
];
static CHILDREN_IMAGE_LAYER: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_PictureEffect/a14:imgEffect", property_name: None },
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
static ATTRS_NON_VISUAL_INK_CONTENT_PART_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":isComment", property_name: Some("IsComment"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_INK_CONTENT_PART_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a14:CT_ContentPartLocking/a14:cpLocks", property_name: Some("ContentPartLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a14:extLst", property_name: Some("OfficeArtExtensionList") },
];
static CHILDREN_NON_VISUAL_CONTENT_PART_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/a14:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a14:CT_NonVisualInkContentPartProperties/a14:cNvContentPartPr", property_name: Some("NonVisualInkContentPartProperties") },
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "CameraTool", local_name: "cameraTool", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CAMERA_TOOL, children: &[] },
    ElementInfo { class_name: "CompatExtension", local_name: "compatExt", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COMPAT_EXTENSION, children: &[] },
    ElementInfo { class_name: "IsCanvas", local_name: "isCanvas", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_IS_CANVAS, children: &[] },
    ElementInfo { class_name: "GvmlContentPart", local_name: "contentPart", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_GVML_CONTENT_PART, children: CHILDREN_GVML_CONTENT_PART },
    ElementInfo { class_name: "ShadowObscured", local_name: "shadowObscured", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHADOW_OBSCURED, children: &[] },
    ElementInfo { class_name: "HiddenFillProperties", local_name: "hiddenFill", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HIDDEN_FILL_PROPERTIES },
    ElementInfo { class_name: "HiddenLineProperties", local_name: "hiddenLine", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HIDDEN_LINE_PROPERTIES, children: CHILDREN_HIDDEN_LINE_PROPERTIES },
    ElementInfo { class_name: "HiddenEffectsProperties", local_name: "hiddenEffects", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HIDDEN_EFFECTS_PROPERTIES },
    ElementInfo { class_name: "HiddenScene3D", local_name: "hiddenScene3d", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HIDDEN_SCENE3_D },
    ElementInfo { class_name: "HiddenShape3D", local_name: "hiddenSp3d", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_HIDDEN_SHAPE3_D, children: CHILDREN_HIDDEN_SHAPE3_D },
    ElementInfo { class_name: "ImageProperties", local_name: "imgProps", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_IMAGE_PROPERTIES },
    ElementInfo { class_name: "UseLocalDpi", local_name: "useLocalDpi", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_USE_LOCAL_DPI, children: &[] },
    ElementInfo { class_name: "TextMath", local_name: "m", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: &[], children: &[] },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "ContentPartLocks", local_name: "cpLocks", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONTENT_PART_LOCKS, children: CHILDREN_CONTENT_PART_LOCKS },
    ElementInfo { class_name: "ForegroundMark", local_name: "foregroundMark", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_FOREGROUND_MARK, children: &[] },
    ElementInfo { class_name: "BackgroundMark", local_name: "backgroundMark", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BACKGROUND_MARK, children: &[] },
    ElementInfo { class_name: "ArtisticBlur", local_name: "artisticBlur", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_BLUR, children: &[] },
    ElementInfo { class_name: "ArtisticCement", local_name: "artisticCement", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_CEMENT, children: &[] },
    ElementInfo { class_name: "ArtisticChalkSketch", local_name: "artisticChalkSketch", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_CHALK_SKETCH, children: &[] },
    ElementInfo { class_name: "ArtisticCrisscrossEtching", local_name: "artisticCrisscrossEtching", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_CRISSCROSS_ETCHING, children: &[] },
    ElementInfo { class_name: "ArtisticCutout", local_name: "artisticCutout", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_CUTOUT, children: &[] },
    ElementInfo { class_name: "ArtisticFilmGrain", local_name: "artisticFilmGrain", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_FILM_GRAIN, children: &[] },
    ElementInfo { class_name: "ArtisticGlass", local_name: "artisticGlass", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_GLASS, children: &[] },
    ElementInfo { class_name: "ArtisticGlowDiffused", local_name: "artisticGlowDiffused", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_GLOW_DIFFUSED, children: &[] },
    ElementInfo { class_name: "ArtisticGlowEdges", local_name: "artisticGlowEdges", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_GLOW_EDGES, children: &[] },
    ElementInfo { class_name: "ArtisticLightScreen", local_name: "artisticLightScreen", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_LIGHT_SCREEN, children: &[] },
    ElementInfo { class_name: "ArtisticLineDrawing", local_name: "artisticLineDrawing", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_LINE_DRAWING, children: &[] },
    ElementInfo { class_name: "ArtisticMarker", local_name: "artisticMarker", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_MARKER, children: &[] },
    ElementInfo { class_name: "ArtisticMosaicBubbles", local_name: "artisticMosiaicBubbles", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_MOSAIC_BUBBLES, children: &[] },
    ElementInfo { class_name: "ArtisticPaintStrokes", local_name: "artisticPaintStrokes", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_PAINT_STROKES, children: &[] },
    ElementInfo { class_name: "ArtisticPaintBrush", local_name: "artisticPaintBrush", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_PAINT_BRUSH, children: &[] },
    ElementInfo { class_name: "ArtisticPastelsSmooth", local_name: "artisticPastelsSmooth", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_PASTELS_SMOOTH, children: &[] },
    ElementInfo { class_name: "ArtisticPencilGrayscale", local_name: "artisticPencilGrayscale", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_PENCIL_GRAYSCALE, children: &[] },
    ElementInfo { class_name: "ArtisticPencilSketch", local_name: "artisticPencilSketch", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_PENCIL_SKETCH, children: &[] },
    ElementInfo { class_name: "ArtisticPhotocopy", local_name: "artisticPhotocopy", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_PHOTOCOPY, children: &[] },
    ElementInfo { class_name: "ArtisticPlasticWrap", local_name: "artisticPlasticWrap", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_PLASTIC_WRAP, children: &[] },
    ElementInfo { class_name: "ArtisticTexturizer", local_name: "artisticTexturizer", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_TEXTURIZER, children: &[] },
    ElementInfo { class_name: "ArtisticWatercolorSponge", local_name: "artisticWatercolorSponge", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ARTISTIC_WATERCOLOR_SPONGE, children: &[] },
    ElementInfo { class_name: "BackgroundRemoval", local_name: "backgroundRemoval", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_BACKGROUND_REMOVAL, children: CHILDREN_BACKGROUND_REMOVAL },
    ElementInfo { class_name: "BrightnessContrast", local_name: "brightnessContrast", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BRIGHTNESS_CONTRAST, children: &[] },
    ElementInfo { class_name: "ColorTemperature", local_name: "colorTemperature", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR_TEMPERATURE, children: &[] },
    ElementInfo { class_name: "Saturation", local_name: "saturation", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SATURATION, children: &[] },
    ElementInfo { class_name: "SharpenSoften", local_name: "sharpenSoften", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_SHARPEN_SOFTEN, children: &[] },
    ElementInfo { class_name: "ImageEffect", local_name: "imgEffect", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_IMAGE_EFFECT, children: CHILDREN_IMAGE_EFFECT },
    ElementInfo { class_name: "ImageLayer", local_name: "imgLayer", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_IMAGE_LAYER, children: CHILDREN_IMAGE_LAYER },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualInkContentPartProperties", local_name: "cNvContentPartPr", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_INK_CONTENT_PART_PROPERTIES, children: CHILDREN_NON_VISUAL_INK_CONTENT_PART_PROPERTIES },
    ElementInfo { class_name: "NonVisualContentPartProperties", local_name: "nvContentPartPr", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONTENT_PART_PROPERTIES },
    ElementInfo { class_name: "Transform2D", local_name: "xfrm", prefix: "a14", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TRANSFORM2_D, children: CHILDREN_TRANSFORM2_D },
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

/// Create a `<a14:cameraTool>` element (`CameraTool`).
pub fn camera_tool() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "cameraTool")
}

/// Create a `<a14:compatExt>` element (`CompatExtension`).
pub fn compat_extension() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "compatExt")
}

/// Create a `<a14:isCanvas>` element (`IsCanvas`).
pub fn is_canvas() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "isCanvas")
}

/// Create a `<a14:contentPart>` element (`GvmlContentPart`).
pub fn gvml_content_part(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "contentPart").with_children(children)
}

/// Create a `<a14:shadowObscured>` element (`ShadowObscured`).
pub fn shadow_obscured() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "shadowObscured")
}

/// Create a `<a14:hiddenFill>` element (`HiddenFillProperties`).
pub fn hidden_fill_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "hiddenFill").with_children(children)
}

/// Create a `<a14:hiddenLine>` element (`HiddenLineProperties`).
pub fn hidden_line_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "hiddenLine").with_children(children)
}

/// Create a `<a14:hiddenEffects>` element (`HiddenEffectsProperties`).
pub fn hidden_effects_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "hiddenEffects").with_children(children)
}

/// Create a `<a14:hiddenScene3d>` element (`HiddenScene3D`).
pub fn hidden_scene3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "hiddenScene3d").with_children(children)
}

/// Create a `<a14:hiddenSp3d>` element (`HiddenShape3D`).
pub fn hidden_shape3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "hiddenSp3d").with_children(children)
}

/// Create a `<a14:imgProps>` element (`ImageProperties`).
pub fn image_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "imgProps").with_children(children)
}

/// Create a `<a14:useLocalDpi>` element (`UseLocalDpi`).
pub fn use_local_dpi() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "useLocalDpi")
}

/// Create a `<a14:m>` element (`TextMath`).
pub fn text_math() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "m")
}

/// Create a `<a14:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<a14:cpLocks>` element (`ContentPartLocks`).
pub fn content_part_locks(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "cpLocks").with_children(children)
}

/// Create a `<a14:foregroundMark>` element (`ForegroundMark`).
pub fn foreground_mark() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "foregroundMark")
}

/// Create a `<a14:backgroundMark>` element (`BackgroundMark`).
pub fn background_mark() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "backgroundMark")
}

/// Create a `<a14:artisticBlur>` element (`ArtisticBlur`).
pub fn artistic_blur() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticBlur")
}

/// Create a `<a14:artisticCement>` element (`ArtisticCement`).
pub fn artistic_cement() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticCement")
}

/// Create a `<a14:artisticChalkSketch>` element (`ArtisticChalkSketch`).
pub fn artistic_chalk_sketch() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticChalkSketch")
}

/// Create a `<a14:artisticCrisscrossEtching>` element (`ArtisticCrisscrossEtching`).
pub fn artistic_crisscross_etching() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticCrisscrossEtching")
}

/// Create a `<a14:artisticCutout>` element (`ArtisticCutout`).
pub fn artistic_cutout() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticCutout")
}

/// Create a `<a14:artisticFilmGrain>` element (`ArtisticFilmGrain`).
pub fn artistic_film_grain() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticFilmGrain")
}

/// Create a `<a14:artisticGlass>` element (`ArtisticGlass`).
pub fn artistic_glass() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticGlass")
}

/// Create a `<a14:artisticGlowDiffused>` element (`ArtisticGlowDiffused`).
pub fn artistic_glow_diffused() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticGlowDiffused")
}

/// Create a `<a14:artisticGlowEdges>` element (`ArtisticGlowEdges`).
pub fn artistic_glow_edges() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticGlowEdges")
}

/// Create a `<a14:artisticLightScreen>` element (`ArtisticLightScreen`).
pub fn artistic_light_screen() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticLightScreen")
}

/// Create a `<a14:artisticLineDrawing>` element (`ArtisticLineDrawing`).
pub fn artistic_line_drawing() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticLineDrawing")
}

/// Create a `<a14:artisticMarker>` element (`ArtisticMarker`).
pub fn artistic_marker() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticMarker")
}

/// Create a `<a14:artisticMosiaicBubbles>` element (`ArtisticMosaicBubbles`).
pub fn artistic_mosaic_bubbles() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticMosiaicBubbles")
}

/// Create a `<a14:artisticPaintStrokes>` element (`ArtisticPaintStrokes`).
pub fn artistic_paint_strokes() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticPaintStrokes")
}

/// Create a `<a14:artisticPaintBrush>` element (`ArtisticPaintBrush`).
pub fn artistic_paint_brush() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticPaintBrush")
}

/// Create a `<a14:artisticPastelsSmooth>` element (`ArtisticPastelsSmooth`).
pub fn artistic_pastels_smooth() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticPastelsSmooth")
}

/// Create a `<a14:artisticPencilGrayscale>` element (`ArtisticPencilGrayscale`).
pub fn artistic_pencil_grayscale() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticPencilGrayscale")
}

/// Create a `<a14:artisticPencilSketch>` element (`ArtisticPencilSketch`).
pub fn artistic_pencil_sketch() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticPencilSketch")
}

/// Create a `<a14:artisticPhotocopy>` element (`ArtisticPhotocopy`).
pub fn artistic_photocopy() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticPhotocopy")
}

/// Create a `<a14:artisticPlasticWrap>` element (`ArtisticPlasticWrap`).
pub fn artistic_plastic_wrap() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticPlasticWrap")
}

/// Create a `<a14:artisticTexturizer>` element (`ArtisticTexturizer`).
pub fn artistic_texturizer() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticTexturizer")
}

/// Create a `<a14:artisticWatercolorSponge>` element (`ArtisticWatercolorSponge`).
pub fn artistic_watercolor_sponge() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "artisticWatercolorSponge")
}

/// Create a `<a14:backgroundRemoval>` element (`BackgroundRemoval`).
pub fn background_removal(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "backgroundRemoval").with_children(children)
}

/// Create a `<a14:brightnessContrast>` element (`BrightnessContrast`).
pub fn brightness_contrast() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "brightnessContrast")
}

/// Create a `<a14:colorTemperature>` element (`ColorTemperature`).
pub fn color_temperature() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "colorTemperature")
}

/// Create a `<a14:saturation>` element (`Saturation`).
pub fn saturation() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "saturation")
}

/// Create a `<a14:sharpenSoften>` element (`SharpenSoften`).
pub fn sharpen_soften() -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "sharpenSoften")
}

/// Create a `<a14:imgEffect>` element (`ImageEffect`).
pub fn image_effect(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "imgEffect").with_children(children)
}

/// Create a `<a14:imgLayer>` element (`ImageLayer`).
pub fn image_layer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "imgLayer").with_children(children)
}

/// Create a `<a14:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<a14:cNvContentPartPr>` element (`NonVisualInkContentPartProperties`).
pub fn non_visual_ink_content_part_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "cNvContentPartPr").with_children(children)
}

/// Create a `<a14:nvContentPartPr>` element (`NonVisualContentPartProperties`).
pub fn non_visual_content_part_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "nvContentPartPr").with_children(children)
}

/// Create a `<a14:xfrm>` element (`Transform2D`).
pub fn transform2_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("a14", NAMESPACE_URI, "xfrm").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 50;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 50;
