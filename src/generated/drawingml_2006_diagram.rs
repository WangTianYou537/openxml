//! Auto-generated from `schemas_openxmlformats_org_drawingml_2006_diagram.json`.
//! Target namespace: `http://schemas.openxmlformats.org/drawingml/2006/diagram` (prefix `dgm`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/diagram";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "dgm";

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

static ATTRS_COLORS_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueId", property_name: Some("UniqueId"), type_name: "StringValue" },
    AttributeInfo { qname: ":minVer", property_name: Some("MinVersion"), type_name: "StringValue" },
];
static CHILDREN_COLORS_DEFINITION: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_CTName/dgm:title", property_name: None },
    ChildInfo { name: "dgm:CT_CTDescription/dgm:desc", property_name: None },
    ChildInfo { name: "dgm:CT_CTCategories/dgm:catLst", property_name: None },
    ChildInfo { name: "dgm:CT_CTStyleLabel/dgm:styleLbl", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static ATTRS_COLORS_DEFINITION_HEADER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueId", property_name: Some("UniqueId"), type_name: "StringValue" },
    AttributeInfo { qname: ":minVer", property_name: Some("MinVersion"), type_name: "StringValue" },
    AttributeInfo { qname: ":resId", property_name: Some("ResourceId"), type_name: "Int32Value" },
];
static CHILDREN_COLORS_DEFINITION_HEADER: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_CTName/dgm:title", property_name: None },
    ChildInfo { name: "dgm:CT_CTDescription/dgm:desc", property_name: None },
    ChildInfo { name: "dgm:CT_CTCategories/dgm:catLst", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static CHILDREN_COLORS_DEFINITION_HEADER_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_ColorTransformHeader/dgm:colorsDefHdr", property_name: None },
];
static CHILDREN_DATA_MODEL_ROOT: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_PtList/dgm:ptLst", property_name: Some("PointList") },
    ChildInfo { name: "dgm:CT_CxnList/dgm:cxnLst", property_name: Some("ConnectionList") },
    ChildInfo { name: "a:CT_BackgroundFormatting/dgm:bg", property_name: Some("Background") },
    ChildInfo { name: "a:CT_WholeE2oFormatting/dgm:whole", property_name: Some("Whole") },
    ChildInfo { name: "a:CT_DataModelExtensionList/dgm:extLst", property_name: Some("DataModelExtensionList") },
];
static ATTRS_LAYOUT_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueId", property_name: Some("UniqueId"), type_name: "StringValue" },
    AttributeInfo { qname: ":minVer", property_name: Some("MinVersion"), type_name: "StringValue" },
    AttributeInfo { qname: ":defStyle", property_name: Some("DefaultStyle"), type_name: "StringValue" },
];
static CHILDREN_LAYOUT_DEFINITION: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Name/dgm:title", property_name: None },
    ChildInfo { name: "dgm:CT_Description/dgm:desc", property_name: None },
    ChildInfo { name: "dgm:CT_Categories/dgm:catLst", property_name: None },
    ChildInfo { name: "dgm:CT_SampleData/dgm:sampData", property_name: None },
    ChildInfo { name: "dgm:CT_SampleData/dgm:styleData", property_name: None },
    ChildInfo { name: "dgm:CT_SampleData/dgm:clrData", property_name: None },
    ChildInfo { name: "dgm:CT_LayoutNode/dgm:layoutNode", property_name: None },
    ChildInfo { name: "dgm:CT_DiagramDefinitionExtensionList/dgm:extLst", property_name: None },
];
static ATTRS_LAYOUT_DEFINITION_HEADER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueId", property_name: Some("UniqueId"), type_name: "StringValue" },
    AttributeInfo { qname: ":minVer", property_name: Some("MinVersion"), type_name: "StringValue" },
    AttributeInfo { qname: ":defStyle", property_name: Some("DefaultStyle"), type_name: "StringValue" },
    AttributeInfo { qname: ":resId", property_name: Some("ResourceId"), type_name: "Int32Value" },
];
static CHILDREN_LAYOUT_DEFINITION_HEADER: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Name/dgm:title", property_name: None },
    ChildInfo { name: "dgm:CT_Description/dgm:desc", property_name: None },
    ChildInfo { name: "dgm:CT_Categories/dgm:catLst", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static CHILDREN_LAYOUT_DEFINITION_HEADER_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_DiagramDefinitionHeader/dgm:layoutDefHdr", property_name: None },
];
static ATTRS_RELATIONSHIP_IDS: &[AttributeInfo] = &[
    AttributeInfo { qname: "r:dm", property_name: Some("DataPart"), type_name: "StringValue" },
    AttributeInfo { qname: "r:lo", property_name: Some("LayoutPart"), type_name: "StringValue" },
    AttributeInfo { qname: "r:qs", property_name: Some("StylePart"), type_name: "StringValue" },
    AttributeInfo { qname: "r:cs", property_name: Some("ColorPart"), type_name: "StringValue" },
];
static ATTRS_STYLE_DEFINITION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueId", property_name: Some("UniqueId"), type_name: "StringValue" },
    AttributeInfo { qname: ":minVer", property_name: Some("MinVersion"), type_name: "StringValue" },
];
static CHILDREN_STYLE_DEFINITION: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_SDName/dgm:title", property_name: None },
    ChildInfo { name: "dgm:CT_SDDescription/dgm:desc", property_name: None },
    ChildInfo { name: "dgm:CT_SDCategories/dgm:catLst", property_name: None },
    ChildInfo { name: "a:CT_Scene3D/dgm:scene3d", property_name: None },
    ChildInfo { name: "dgm:CT_StyleLabel/dgm:styleLbl", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static ATTRS_STYLE_DEFINITION_HEADER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uniqueId", property_name: Some("UniqueId"), type_name: "StringValue" },
    AttributeInfo { qname: ":minVer", property_name: Some("MinVersion"), type_name: "StringValue" },
    AttributeInfo { qname: ":resId", property_name: Some("ResourceId"), type_name: "Int32Value" },
];
static CHILDREN_STYLE_DEFINITION_HEADER: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_SDName/dgm:title", property_name: None },
    ChildInfo { name: "dgm:CT_SDDescription/dgm:desc", property_name: None },
    ChildInfo { name: "dgm:CT_SDCategories/dgm:catLst", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static CHILDREN_STYLE_DEFINITION_HEADER_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_StyleDefinitionHeader/dgm:styleDefHdr", property_name: None },
];
static ATTRS_COLOR_TRANSFORM_CATEGORY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":pri", property_name: Some("Priority"), type_name: "UInt32Value" },
];
static ATTRS_FILL_COLOR_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":meth", property_name: Some("Method"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hueDir", property_name: Some("HueDirection"), type_name: "EnumValue" },
];
static CHILDREN_FILL_COLOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static ATTRS_LINE_COLOR_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":meth", property_name: Some("Method"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hueDir", property_name: Some("HueDirection"), type_name: "EnumValue" },
];
static CHILDREN_LINE_COLOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static ATTRS_EFFECT_COLOR_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":meth", property_name: Some("Method"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hueDir", property_name: Some("HueDirection"), type_name: "EnumValue" },
];
static CHILDREN_EFFECT_COLOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static ATTRS_TEXT_LINE_COLOR_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":meth", property_name: Some("Method"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hueDir", property_name: Some("HueDirection"), type_name: "EnumValue" },
];
static CHILDREN_TEXT_LINE_COLOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static ATTRS_TEXT_FILL_COLOR_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":meth", property_name: Some("Method"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hueDir", property_name: Some("HueDirection"), type_name: "EnumValue" },
];
static CHILDREN_TEXT_FILL_COLOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static ATTRS_TEXT_EFFECT_COLOR_LIST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":meth", property_name: Some("Method"), type_name: "EnumValue" },
    AttributeInfo { qname: ":hueDir", property_name: Some("HueDirection"), type_name: "EnumValue" },
];
static CHILDREN_TEXT_EFFECT_COLOR_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ScRgbColor/a:scrgbClr", property_name: None },
    ChildInfo { name: "a:CT_SRgbColor/a:srgbClr", property_name: None },
    ChildInfo { name: "a:CT_HslColor/a:hslClr", property_name: None },
    ChildInfo { name: "a:CT_SystemColor/a:sysClr", property_name: None },
    ChildInfo { name: "a:CT_SchemeColor/a:schemeClr", property_name: None },
    ChildInfo { name: "a:CT_PresetColor/a:prstClr", property_name: None },
];
static CHILDREN_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
];
static ATTRS_COLOR_DEFINITION_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_COLOR_TRANSFORM_DESCRIPTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_COLOR_TRANSFORM_CATEGORIES: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_CTCategory/dgm:cat", property_name: None },
];
static ATTRS_COLOR_TRANSFORM_STYLE_LABEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_COLOR_TRANSFORM_STYLE_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Colors/dgm:fillClrLst", property_name: Some("FillColorList") },
    ChildInfo { name: "dgm:CT_Colors/dgm:linClrLst", property_name: Some("LineColorList") },
    ChildInfo { name: "dgm:CT_Colors/dgm:effectClrLst", property_name: Some("EffectColorList") },
    ChildInfo { name: "dgm:CT_Colors/dgm:txLinClrLst", property_name: Some("TextLineColorList") },
    ChildInfo { name: "dgm:CT_Colors/dgm:txFillClrLst", property_name: Some("TextFillColorList") },
    ChildInfo { name: "dgm:CT_Colors/dgm:txEffectClrLst", property_name: Some("TextEffectColorList") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_POINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":modelId", property_name: Some("ModelId"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":cxnId", property_name: Some("ConnectionId"), type_name: "StringValue" },
];
static CHILDREN_POINT: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_ElemPropSet/dgm:prSet", property_name: Some("PropertySet") },
    ChildInfo { name: "a:CT_ShapeProperties/dgm:spPr", property_name: Some("ShapeProperties") },
    ChildInfo { name: "a:CT_TextBody/dgm:t", property_name: Some("TextBody") },
    ChildInfo { name: "a:CT_PtExtensionList/dgm:extLst", property_name: Some("PtExtensionList") },
];
static ATTRS_CONNECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":modelId", property_name: Some("ModelId"), type_name: "StringValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":srcId", property_name: Some("SourceId"), type_name: "StringValue" },
    AttributeInfo { qname: ":destId", property_name: Some("DestinationId"), type_name: "StringValue" },
    AttributeInfo { qname: ":srcOrd", property_name: Some("SourcePosition"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":destOrd", property_name: Some("DestinationPosition"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":parTransId", property_name: Some("ParentTransitionId"), type_name: "StringValue" },
    AttributeInfo { qname: ":sibTransId", property_name: Some("SiblingTransitionId"), type_name: "StringValue" },
    AttributeInfo { qname: ":presId", property_name: Some("PresentationId"), type_name: "StringValue" },
];
static CHILDREN_CONNECTION: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_CONSTRAINT: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":for", property_name: Some("For"), type_name: "EnumValue" },
    AttributeInfo { qname: ":forName", property_name: Some("ForName"), type_name: "StringValue" },
    AttributeInfo { qname: ":ptType", property_name: Some("PointType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":refType", property_name: Some("ReferenceType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":refFor", property_name: Some("ReferenceFor"), type_name: "EnumValue" },
    AttributeInfo { qname: ":refForName", property_name: Some("ReferenceForName"), type_name: "StringValue" },
    AttributeInfo { qname: ":refPtType", property_name: Some("ReferencePointType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":op", property_name: Some("Operator"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":fact", property_name: Some("Fact"), type_name: "DoubleValue" },
];
static CHILDREN_CONSTRAINT: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_RULE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":for", property_name: Some("For"), type_name: "EnumValue" },
    AttributeInfo { qname: ":forName", property_name: Some("ForName"), type_name: "StringValue" },
    AttributeInfo { qname: ":ptType", property_name: Some("PointType"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":fact", property_name: Some("Fact"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":max", property_name: Some("Max"), type_name: "DoubleValue" },
];
static CHILDREN_RULE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_ADJUST: &[AttributeInfo] = &[
    AttributeInfo { qname: ":idx", property_name: Some("Index"), type_name: "UInt32Value" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "DoubleValue" },
];
static CHILDREN_ADJUST_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Adj/dgm:adj", property_name: None },
];
static ATTRS_PARAMETER: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_ALGORITHM: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "EnumValue" },
    AttributeInfo { qname: ":rev", property_name: Some("Revision"), type_name: "UInt32Value" },
];
static CHILDREN_ALGORITHM: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Parameter/dgm:param", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static ATTRS_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":rot", property_name: Some("Rotation"), type_name: "DoubleValue" },
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: "r:blip", property_name: Some("Blip"), type_name: "StringValue" },
    AttributeInfo { qname: ":zOrderOff", property_name: Some("ZOrderOffset"), type_name: "Int32Value" },
    AttributeInfo { qname: ":hideGeom", property_name: Some("HideGeometry"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":lkTxEntry", property_name: Some("LockedText"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":blipPhldr", property_name: Some("BlipPlaceholder"), type_name: "BooleanValue" },
];
static CHILDREN_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_AdjLst/dgm:adjLst", property_name: Some("AdjustList") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_PRESENTATION_OF: &[AttributeInfo] = &[
    AttributeInfo { qname: ":axis", property_name: Some("Axis"), type_name: "ListValue" },
    AttributeInfo { qname: ":ptType", property_name: Some("PointType"), type_name: "ListValue" },
    AttributeInfo { qname: ":hideLastTrans", property_name: Some("HideLastTrans"), type_name: "ListValue" },
    AttributeInfo { qname: ":st", property_name: Some("Start"), type_name: "ListValue" },
    AttributeInfo { qname: ":cnt", property_name: Some("Count"), type_name: "ListValue" },
    AttributeInfo { qname: ":step", property_name: Some("Step"), type_name: "ListValue" },
];
static CHILDREN_PRESENTATION_OF: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_CONSTRAINTS: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Constraint/dgm:constr", property_name: None },
];
static CHILDREN_RULE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_NumericRule/dgm:rule", property_name: None },
];
static CHILDREN_VARIABLE_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_OrgChart/dgm:orgChart", property_name: Some("OrganizationChart") },
    ChildInfo { name: "dgm:CT_ChildMax/dgm:chMax", property_name: Some("MaxNumberOfChildren") },
    ChildInfo { name: "dgm:CT_ChildPref/dgm:chPref", property_name: Some("PreferredNumberOfChildren") },
    ChildInfo { name: "dgm:CT_BulletEnabled/dgm:bulletEnabled", property_name: Some("BulletEnabled") },
    ChildInfo { name: "dgm:CT_Direction/dgm:dir", property_name: Some("Direction") },
    ChildInfo { name: "dgm:CT_HierBranchStyle/dgm:hierBranch", property_name: Some("HierarchyBranch") },
    ChildInfo { name: "dgm:CT_AnimOne/dgm:animOne", property_name: Some("AnimateOneByOne") },
    ChildInfo { name: "dgm:CT_AnimLvl/dgm:animLvl", property_name: Some("AnimationLevel") },
    ChildInfo { name: "dgm:CT_ResizeHandles/dgm:resizeHandles", property_name: Some("ResizeHandles") },
];
static CHILDREN_PRESENTATION_LAYOUT_VARIABLES: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_OrgChart/dgm:orgChart", property_name: Some("OrganizationChart") },
    ChildInfo { name: "dgm:CT_ChildMax/dgm:chMax", property_name: Some("MaxNumberOfChildren") },
    ChildInfo { name: "dgm:CT_ChildPref/dgm:chPref", property_name: Some("PreferredNumberOfChildren") },
    ChildInfo { name: "dgm:CT_BulletEnabled/dgm:bulletEnabled", property_name: Some("BulletEnabled") },
    ChildInfo { name: "dgm:CT_Direction/dgm:dir", property_name: Some("Direction") },
    ChildInfo { name: "dgm:CT_HierBranchStyle/dgm:hierBranch", property_name: Some("HierarchyBranch") },
    ChildInfo { name: "dgm:CT_AnimOne/dgm:animOne", property_name: Some("AnimateOneByOne") },
    ChildInfo { name: "dgm:CT_AnimLvl/dgm:animLvl", property_name: Some("AnimationLevel") },
    ChildInfo { name: "dgm:CT_ResizeHandles/dgm:resizeHandles", property_name: Some("ResizeHandles") },
];
static ATTRS_FOR_EACH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":ref", property_name: Some("Reference"), type_name: "StringValue" },
    AttributeInfo { qname: ":axis", property_name: Some("Axis"), type_name: "ListValue" },
    AttributeInfo { qname: ":ptType", property_name: Some("PointType"), type_name: "ListValue" },
    AttributeInfo { qname: ":hideLastTrans", property_name: Some("HideLastTrans"), type_name: "ListValue" },
    AttributeInfo { qname: ":st", property_name: Some("Start"), type_name: "ListValue" },
    AttributeInfo { qname: ":cnt", property_name: Some("Count"), type_name: "ListValue" },
    AttributeInfo { qname: ":step", property_name: Some("Step"), type_name: "ListValue" },
];
static CHILDREN_FOR_EACH: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Algorithm/dgm:alg", property_name: None },
    ChildInfo { name: "dgm:CT_Shape/dgm:shape", property_name: None },
    ChildInfo { name: "dgm:CT_PresentationOf/dgm:presOf", property_name: None },
    ChildInfo { name: "dgm:CT_Constraints/dgm:constrLst", property_name: None },
    ChildInfo { name: "dgm:CT_Rules/dgm:ruleLst", property_name: None },
    ChildInfo { name: "dgm:CT_ForEach/dgm:forEach", property_name: None },
    ChildInfo { name: "dgm:CT_LayoutNode/dgm:layoutNode", property_name: None },
    ChildInfo { name: "dgm:CT_Choose/dgm:choose", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static ATTRS_LAYOUT_NODE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":styleLbl", property_name: Some("StyleLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":chOrder", property_name: Some("ChildOrder"), type_name: "EnumValue" },
    AttributeInfo { qname: ":moveWith", property_name: Some("MoveWith"), type_name: "StringValue" },
];
static CHILDREN_LAYOUT_NODE: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Algorithm/dgm:alg", property_name: None },
    ChildInfo { name: "dgm:CT_Shape/dgm:shape", property_name: None },
    ChildInfo { name: "dgm:CT_PresentationOf/dgm:presOf", property_name: None },
    ChildInfo { name: "dgm:CT_Constraints/dgm:constrLst", property_name: None },
    ChildInfo { name: "dgm:CT_Rules/dgm:ruleLst", property_name: None },
    ChildInfo { name: "dgm:CT_LayoutVariablePropertySet/dgm:varLst", property_name: None },
    ChildInfo { name: "dgm:CT_ForEach/dgm:forEach", property_name: None },
    ChildInfo { name: "dgm:CT_LayoutNode/dgm:layoutNode", property_name: None },
    ChildInfo { name: "dgm:CT_Choose/dgm:choose", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static ATTRS_CHOOSE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_CHOOSE: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_When/dgm:if", property_name: None },
    ChildInfo { name: "dgm:CT_Otherwise/dgm:else", property_name: None },
];
static ATTRS_DIAGRAM_CHOOSE_IF: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
    AttributeInfo { qname: ":axis", property_name: Some("Axis"), type_name: "ListValue" },
    AttributeInfo { qname: ":ptType", property_name: Some("PointType"), type_name: "ListValue" },
    AttributeInfo { qname: ":hideLastTrans", property_name: Some("HideLastTrans"), type_name: "ListValue" },
    AttributeInfo { qname: ":st", property_name: Some("Start"), type_name: "ListValue" },
    AttributeInfo { qname: ":cnt", property_name: Some("Count"), type_name: "ListValue" },
    AttributeInfo { qname: ":step", property_name: Some("Step"), type_name: "ListValue" },
    AttributeInfo { qname: ":func", property_name: Some("Function"), type_name: "EnumValue" },
    AttributeInfo { qname: ":arg", property_name: Some("Argument"), type_name: "StringValue" },
    AttributeInfo { qname: ":op", property_name: Some("Operator"), type_name: "EnumValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_DIAGRAM_CHOOSE_IF: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Algorithm/dgm:alg", property_name: None },
    ChildInfo { name: "dgm:CT_Shape/dgm:shape", property_name: None },
    ChildInfo { name: "dgm:CT_PresentationOf/dgm:presOf", property_name: None },
    ChildInfo { name: "dgm:CT_Constraints/dgm:constrLst", property_name: None },
    ChildInfo { name: "dgm:CT_Rules/dgm:ruleLst", property_name: None },
    ChildInfo { name: "dgm:CT_ForEach/dgm:forEach", property_name: None },
    ChildInfo { name: "dgm:CT_LayoutNode/dgm:layoutNode", property_name: None },
    ChildInfo { name: "dgm:CT_Choose/dgm:choose", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static ATTRS_DIAGRAM_CHOOSE_ELSE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_DIAGRAM_CHOOSE_ELSE: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Algorithm/dgm:alg", property_name: None },
    ChildInfo { name: "dgm:CT_Shape/dgm:shape", property_name: None },
    ChildInfo { name: "dgm:CT_PresentationOf/dgm:presOf", property_name: None },
    ChildInfo { name: "dgm:CT_Constraints/dgm:constrLst", property_name: None },
    ChildInfo { name: "dgm:CT_Rules/dgm:ruleLst", property_name: None },
    ChildInfo { name: "dgm:CT_ForEach/dgm:forEach", property_name: None },
    ChildInfo { name: "dgm:CT_LayoutNode/dgm:layoutNode", property_name: None },
    ChildInfo { name: "dgm:CT_Choose/dgm:choose", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: None },
];
static CHILDREN_DATA_MODEL: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_PtList/dgm:ptLst", property_name: Some("PointList") },
    ChildInfo { name: "dgm:CT_CxnList/dgm:cxnLst", property_name: Some("ConnectionList") },
    ChildInfo { name: "a:CT_BackgroundFormatting/dgm:bg", property_name: Some("Background") },
    ChildInfo { name: "a:CT_WholeE2oFormatting/dgm:whole", property_name: Some("Whole") },
    ChildInfo { name: "a:CT_DataModelExtensionList/dgm:extLst", property_name: Some("DataModelExtensionList") },
];
static ATTRS_CATEGORY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":pri", property_name: Some("Priority"), type_name: "UInt32Value" },
];
static ATTRS_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_DESCRIPTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_CATEGORY_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Category/dgm:cat", property_name: None },
];
static CHILDREN_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "a:CT_FontReference/a:fontRef", property_name: Some("FontReference") },
];
static ATTRS_ORGANIZATION_CHART: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_MAX_NUMBER_OF_CHILDREN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_PREFERRED_NUMBER_OF_CHILDREN: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "Int32Value" },
];
static ATTRS_BULLET_ENABLED: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "BooleanValue" },
];
static ATTRS_DIRECTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_HIERARCHY_BRANCH: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ANIMATE_ONE_BY_ONE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_ANIMATION_LEVEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_RESIZE_HANDLES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "EnumValue" },
];
static ATTRS_STYLE_DISPLAY_CATEGORY: &[AttributeInfo] = &[
    AttributeInfo { qname: ":type", property_name: Some("Type"), type_name: "StringValue" },
    AttributeInfo { qname: ":pri", property_name: Some("Priority"), type_name: "UInt32Value" },
];
static CHILDREN_SCENE3_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Camera/a:camera", property_name: Some("Camera") },
    ChildInfo { name: "a:CT_LightRig/a:lightRig", property_name: Some("LightRig") },
    ChildInfo { name: "a:CT_Backdrop/a:backdrop", property_name: Some("Backdrop") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static ATTRS_SHAPE3_D: &[AttributeInfo] = &[
    AttributeInfo { qname: ":z", property_name: Some("Z"), type_name: "Int64Value" },
    AttributeInfo { qname: ":extrusionH", property_name: Some("ExtrusionHeight"), type_name: "Int64Value" },
    AttributeInfo { qname: ":contourW", property_name: Some("ContourWidth"), type_name: "Int64Value" },
    AttributeInfo { qname: ":prstMaterial", property_name: Some("PresetMaterial"), type_name: "EnumValue" },
];
static CHILDREN_SHAPE3_D: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Bevel/a:bevelT", property_name: Some("BevelTop") },
    ChildInfo { name: "a:CT_Bevel/a:bevelB", property_name: Some("BevelBottom") },
    ChildInfo { name: "a:CT_Color/a:extrusionClr", property_name: Some("ExtrusionColor") },
    ChildInfo { name: "a:CT_Color/a:contourClr", property_name: Some("ContourColor") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_TEXT_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Shape3D/a:sp3d", property_name: Some("Shape3DType") },
    ChildInfo { name: "a:CT_FlatText/a:flatTx", property_name: Some("FlatText") },
];
static ATTRS_STYLE_DEFINITION_TITLE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static ATTRS_STYLE_LABEL_DESCRIPTION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":lang", property_name: Some("Language"), type_name: "StringValue" },
    AttributeInfo { qname: ":val", property_name: Some("Val"), type_name: "StringValue" },
];
static CHILDREN_STYLE_DISPLAY_CATEGORIES: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_SDCategory/dgm:cat", property_name: None },
];
static ATTRS_STYLE_LABEL: &[AttributeInfo] = &[
    AttributeInfo { qname: ":name", property_name: Some("Name"), type_name: "StringValue" },
];
static CHILDREN_STYLE_LABEL: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_Scene3D/dgm:scene3d", property_name: Some("Scene3D") },
    ChildInfo { name: "a:CT_Shape3D/dgm:sp3d", property_name: Some("Shape3D") },
    ChildInfo { name: "dgm:CT_TextProps/dgm:txPr", property_name: Some("TextProperties") },
    ChildInfo { name: "a:CT_ShapeStyle/dgm:style", property_name: Some("Style") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/dgm:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_POINT_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Pt/dgm:pt", property_name: None },
];
static CHILDREN_CONNECTION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_Cxn/dgm:cxn", property_name: None },
];
static CHILDREN_BACKGROUND: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NoFillProperties/a:noFill", property_name: None },
    ChildInfo { name: "a:CT_SolidColorFillProperties/a:solidFill", property_name: None },
    ChildInfo { name: "a:CT_GradientFillProperties/a:gradFill", property_name: None },
    ChildInfo { name: "a:CT_BlipFillProperties/a:blipFill", property_name: None },
    ChildInfo { name: "a:CT_PatternFillProperties/a:pattFill", property_name: None },
    ChildInfo { name: "a:CT_GroupFillProperties/a:grpFill", property_name: None },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
];
static CHILDREN_WHOLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_LineProperties/a:ln", property_name: Some("Outline") },
    ChildInfo { name: "a:CT_EffectList/a:effectLst", property_name: None },
    ChildInfo { name: "a:CT_EffectContainer/a:effectDag", property_name: None },
];
static CHILDREN_DATA_MODEL_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_DataModelExtension/a:ext", property_name: None },
];
static ATTRS_PROPERTY_SET: &[AttributeInfo] = &[
    AttributeInfo { qname: ":presAssocID", property_name: Some("PresentationElementId"), type_name: "StringValue" },
    AttributeInfo { qname: ":presName", property_name: Some("PresentationName"), type_name: "StringValue" },
    AttributeInfo { qname: ":presStyleLbl", property_name: Some("PresentationStyleLabel"), type_name: "StringValue" },
    AttributeInfo { qname: ":presStyleIdx", property_name: Some("PresentationStyleIndex"), type_name: "Int32Value" },
    AttributeInfo { qname: ":presStyleCnt", property_name: Some("PresentationStyleCount"), type_name: "Int32Value" },
    AttributeInfo { qname: ":loTypeId", property_name: Some("LayoutTypeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":loCatId", property_name: Some("LayoutCategoryId"), type_name: "StringValue" },
    AttributeInfo { qname: ":qsTypeId", property_name: Some("QuickStyleTypeId"), type_name: "StringValue" },
    AttributeInfo { qname: ":qsCatId", property_name: Some("QuickStyleCategoryId"), type_name: "StringValue" },
    AttributeInfo { qname: ":csTypeId", property_name: Some("ColorType"), type_name: "StringValue" },
    AttributeInfo { qname: ":csCatId", property_name: Some("ColorCategoryId"), type_name: "StringValue" },
    AttributeInfo { qname: ":coherent3DOff", property_name: Some("Coherent3D"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":phldrT", property_name: Some("PlaceholderText"), type_name: "StringValue" },
    AttributeInfo { qname: ":phldr", property_name: Some("Placeholder"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":custAng", property_name: Some("Rotation"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custFlipVert", property_name: Some("VerticalFlip"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":custFlipHor", property_name: Some("HorizontalFlip"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":custSzX", property_name: Some("FixedWidthOverride"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custSzY", property_name: Some("FixedHeightOverride"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custScaleX", property_name: Some("WidthScale"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custScaleY", property_name: Some("HeightScale"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custT", property_name: Some("TextChanged"), type_name: "BooleanValue" },
    AttributeInfo { qname: ":custLinFactX", property_name: Some("FactorWidth"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custLinFactY", property_name: Some("FactorHeight"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custLinFactNeighborX", property_name: Some("NeighborOffsetWidth"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custLinFactNeighborY", property_name: Some("NeighborOffsetHeight"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custRadScaleRad", property_name: Some("RadiusScale"), type_name: "Int32Value" },
    AttributeInfo { qname: ":custRadScaleInc", property_name: Some("IncludeAngleScale"), type_name: "Int32Value" },
];
static CHILDREN_PROPERTY_SET: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_LayoutVariablePropertySet/dgm:presLayoutVars", property_name: Some("PresentationLayoutVariables") },
    ChildInfo { name: "a:CT_ShapeStyle/dgm:style", property_name: Some("Style") },
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
static CHILDREN_TEXT_BODY: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_TextBodyProperties/a:bodyPr", property_name: Some("BodyProperties") },
    ChildInfo { name: "a:CT_TextListStyle/a:lstStyle", property_name: Some("ListStyle") },
    ChildInfo { name: "a:CT_TextParagraph/a:p", property_name: None },
];
static CHILDREN_PT_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_PtExtension/a:ext", property_name: None },
];
static ATTRS_DIAGRAM_DEFINITION_EXTENSION: &[AttributeInfo] = &[
    AttributeInfo { qname: ":uri", property_name: None, type_name: "StringValue" },
];
static CHILDREN_DIAGRAM_DEFINITION_EXTENSION: &[ChildInfo] = &[
    ChildInfo { name: "dgm1611:CT_NumberDiagramInfoList/dgm1611:autoBuNodeInfoLst", property_name: Some("NumberDiagramInfoList") },
    ChildInfo { name: "a:CT_TextListStyle/dgm1612:lstStyle", property_name: Some("TextListStyleType") },
];
static ATTRS_SAMPLE_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":useDef", property_name: Some("UseDefault"), type_name: "BooleanValue" },
];
static CHILDREN_SAMPLE_DATA: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_DataModel/dgm:dataModel", property_name: Some("DataModel") },
];
static ATTRS_STYLE_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":useDef", property_name: Some("UseDefault"), type_name: "BooleanValue" },
];
static CHILDREN_STYLE_DATA: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_DataModel/dgm:dataModel", property_name: Some("DataModel") },
];
static ATTRS_COLOR_DATA: &[AttributeInfo] = &[
    AttributeInfo { qname: ":useDef", property_name: Some("UseDefault"), type_name: "BooleanValue" },
];
static CHILDREN_COLOR_DATA: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_DataModel/dgm:dataModel", property_name: Some("DataModel") },
];
static CHILDREN_DIAGRAM_DEFINITION_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "dgm:CT_DiagramDefinitionExtension/dgm:ext", property_name: None },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "ColorsDefinition", local_name: "colorsDef", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLORS_DEFINITION, children: CHILDREN_COLORS_DEFINITION },
    ElementInfo { class_name: "ColorsDefinitionHeader", local_name: "colorsDefHdr", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLORS_DEFINITION_HEADER, children: CHILDREN_COLORS_DEFINITION_HEADER },
    ElementInfo { class_name: "ColorsDefinitionHeaderList", local_name: "colorsDefHdrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLORS_DEFINITION_HEADER_LIST },
    ElementInfo { class_name: "DataModelRoot", local_name: "dataModel", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_MODEL_ROOT },
    ElementInfo { class_name: "LayoutDefinition", local_name: "layoutDef", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LAYOUT_DEFINITION, children: CHILDREN_LAYOUT_DEFINITION },
    ElementInfo { class_name: "LayoutDefinitionHeader", local_name: "layoutDefHdr", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LAYOUT_DEFINITION_HEADER, children: CHILDREN_LAYOUT_DEFINITION_HEADER },
    ElementInfo { class_name: "LayoutDefinitionHeaderList", local_name: "layoutDefHdrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_LAYOUT_DEFINITION_HEADER_LIST },
    ElementInfo { class_name: "RelationshipIds", local_name: "relIds", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RELATIONSHIP_IDS, children: &[] },
    ElementInfo { class_name: "StyleDefinition", local_name: "styleDef", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STYLE_DEFINITION, children: CHILDREN_STYLE_DEFINITION },
    ElementInfo { class_name: "StyleDefinitionHeader", local_name: "styleDefHdr", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STYLE_DEFINITION_HEADER, children: CHILDREN_STYLE_DEFINITION_HEADER },
    ElementInfo { class_name: "StyleDefinitionHeaderList", local_name: "styleDefHdrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STYLE_DEFINITION_HEADER_LIST },
    ElementInfo { class_name: "ColorTransformCategory", local_name: "cat", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR_TRANSFORM_CATEGORY, children: &[] },
    ElementInfo { class_name: "FillColorList", local_name: "fillClrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FILL_COLOR_LIST, children: CHILDREN_FILL_COLOR_LIST },
    ElementInfo { class_name: "LineColorList", local_name: "linClrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINE_COLOR_LIST, children: CHILDREN_LINE_COLOR_LIST },
    ElementInfo { class_name: "EffectColorList", local_name: "effectClrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_EFFECT_COLOR_LIST, children: CHILDREN_EFFECT_COLOR_LIST },
    ElementInfo { class_name: "TextLineColorList", local_name: "txLinClrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_LINE_COLOR_LIST, children: CHILDREN_TEXT_LINE_COLOR_LIST },
    ElementInfo { class_name: "TextFillColorList", local_name: "txFillClrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_FILL_COLOR_LIST, children: CHILDREN_TEXT_FILL_COLOR_LIST },
    ElementInfo { class_name: "TextEffectColorList", local_name: "txEffectClrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_EFFECT_COLOR_LIST, children: CHILDREN_TEXT_EFFECT_COLOR_LIST },
    ElementInfo { class_name: "ExtensionList", local_name: "extLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_EXTENSION_LIST },
    ElementInfo { class_name: "ColorDefinitionTitle", local_name: "title", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR_DEFINITION_TITLE, children: &[] },
    ElementInfo { class_name: "ColorTransformDescription", local_name: "desc", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_COLOR_TRANSFORM_DESCRIPTION, children: &[] },
    ElementInfo { class_name: "ColorTransformCategories", local_name: "catLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_COLOR_TRANSFORM_CATEGORIES },
    ElementInfo { class_name: "ColorTransformStyleLabel", local_name: "styleLbl", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_TRANSFORM_STYLE_LABEL, children: CHILDREN_COLOR_TRANSFORM_STYLE_LABEL },
    ElementInfo { class_name: "Point", local_name: "pt", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_POINT, children: CHILDREN_POINT },
    ElementInfo { class_name: "Connection", local_name: "cxn", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONNECTION, children: CHILDREN_CONNECTION },
    ElementInfo { class_name: "Constraint", local_name: "constr", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CONSTRAINT, children: CHILDREN_CONSTRAINT },
    ElementInfo { class_name: "Rule", local_name: "rule", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_RULE, children: CHILDREN_RULE },
    ElementInfo { class_name: "Adjust", local_name: "adj", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ADJUST, children: &[] },
    ElementInfo { class_name: "AdjustList", local_name: "adjLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_ADJUST_LIST },
    ElementInfo { class_name: "Parameter", local_name: "param", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PARAMETER, children: &[] },
    ElementInfo { class_name: "Algorithm", local_name: "alg", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_ALGORITHM, children: CHILDREN_ALGORITHM },
    ElementInfo { class_name: "Shape", local_name: "shape", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE, children: CHILDREN_SHAPE },
    ElementInfo { class_name: "PresentationOf", local_name: "presOf", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PRESENTATION_OF, children: CHILDREN_PRESENTATION_OF },
    ElementInfo { class_name: "Constraints", local_name: "constrLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONSTRAINTS },
    ElementInfo { class_name: "RuleList", local_name: "ruleLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_RULE_LIST },
    ElementInfo { class_name: "VariableList", local_name: "varLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_VARIABLE_LIST },
    ElementInfo { class_name: "PresentationLayoutVariables", local_name: "presLayoutVars", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PRESENTATION_LAYOUT_VARIABLES },
    ElementInfo { class_name: "ForEach", local_name: "forEach", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_FOR_EACH, children: CHILDREN_FOR_EACH },
    ElementInfo { class_name: "LayoutNode", local_name: "layoutNode", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LAYOUT_NODE, children: CHILDREN_LAYOUT_NODE },
    ElementInfo { class_name: "Choose", local_name: "choose", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_CHOOSE, children: CHILDREN_CHOOSE },
    ElementInfo { class_name: "DiagramChooseIf", local_name: "if", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIAGRAM_CHOOSE_IF, children: CHILDREN_DIAGRAM_CHOOSE_IF },
    ElementInfo { class_name: "DiagramChooseElse", local_name: "else", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIAGRAM_CHOOSE_ELSE, children: CHILDREN_DIAGRAM_CHOOSE_ELSE },
    ElementInfo { class_name: "DataModel", local_name: "dataModel", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_MODEL },
    ElementInfo { class_name: "Category", local_name: "cat", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_CATEGORY, children: &[] },
    ElementInfo { class_name: "Title", local_name: "title", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_TITLE, children: &[] },
    ElementInfo { class_name: "Description", local_name: "desc", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DESCRIPTION, children: &[] },
    ElementInfo { class_name: "CategoryList", local_name: "catLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CATEGORY_LIST },
    ElementInfo { class_name: "Style", local_name: "style", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STYLE },
    ElementInfo { class_name: "OrganizationChart", local_name: "orgChart", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ORGANIZATION_CHART, children: &[] },
    ElementInfo { class_name: "MaxNumberOfChildren", local_name: "chMax", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_MAX_NUMBER_OF_CHILDREN, children: &[] },
    ElementInfo { class_name: "PreferredNumberOfChildren", local_name: "chPref", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_PREFERRED_NUMBER_OF_CHILDREN, children: &[] },
    ElementInfo { class_name: "BulletEnabled", local_name: "bulletEnabled", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_BULLET_ENABLED, children: &[] },
    ElementInfo { class_name: "Direction", local_name: "dir", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_DIRECTION, children: &[] },
    ElementInfo { class_name: "HierarchyBranch", local_name: "hierBranch", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_HIERARCHY_BRANCH, children: &[] },
    ElementInfo { class_name: "AnimateOneByOne", local_name: "animOne", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ANIMATE_ONE_BY_ONE, children: &[] },
    ElementInfo { class_name: "AnimationLevel", local_name: "animLvl", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_ANIMATION_LEVEL, children: &[] },
    ElementInfo { class_name: "ResizeHandles", local_name: "resizeHandles", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_RESIZE_HANDLES, children: &[] },
    ElementInfo { class_name: "StyleDisplayCategory", local_name: "cat", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STYLE_DISPLAY_CATEGORY, children: &[] },
    ElementInfo { class_name: "Scene3D", local_name: "scene3d", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SCENE3_D },
    ElementInfo { class_name: "Shape3D", local_name: "sp3d", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE3_D, children: CHILDREN_SHAPE3_D },
    ElementInfo { class_name: "TextProperties", local_name: "txPr", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_PROPERTIES },
    ElementInfo { class_name: "StyleDefinitionTitle", local_name: "title", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STYLE_DEFINITION_TITLE, children: &[] },
    ElementInfo { class_name: "StyleLabelDescription", local_name: "desc", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: false, attributes: ATTRS_STYLE_LABEL_DESCRIPTION, children: &[] },
    ElementInfo { class_name: "StyleDisplayCategories", local_name: "catLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_STYLE_DISPLAY_CATEGORIES },
    ElementInfo { class_name: "StyleLabel", local_name: "styleLbl", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STYLE_LABEL, children: CHILDREN_STYLE_LABEL },
    ElementInfo { class_name: "PointList", local_name: "ptLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_POINT_LIST },
    ElementInfo { class_name: "ConnectionList", local_name: "cxnLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_CONNECTION_LIST },
    ElementInfo { class_name: "Background", local_name: "bg", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_BACKGROUND },
    ElementInfo { class_name: "Whole", local_name: "whole", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_WHOLE },
    ElementInfo { class_name: "DataModelExtensionList", local_name: "extLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DATA_MODEL_EXTENSION_LIST },
    ElementInfo { class_name: "PropertySet", local_name: "prSet", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_PROPERTY_SET, children: CHILDREN_PROPERTY_SET },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "TextBody", local_name: "t", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TEXT_BODY },
    ElementInfo { class_name: "PtExtensionList", local_name: "extLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PT_EXTENSION_LIST },
    ElementInfo { class_name: "DiagramDefinitionExtension", local_name: "ext", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_DIAGRAM_DEFINITION_EXTENSION, children: CHILDREN_DIAGRAM_DEFINITION_EXTENSION },
    ElementInfo { class_name: "SampleData", local_name: "sampData", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SAMPLE_DATA, children: CHILDREN_SAMPLE_DATA },
    ElementInfo { class_name: "StyleData", local_name: "styleData", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_STYLE_DATA, children: CHILDREN_STYLE_DATA },
    ElementInfo { class_name: "ColorData", local_name: "clrData", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_COLOR_DATA, children: CHILDREN_COLOR_DATA },
    ElementInfo { class_name: "DiagramDefinitionExtensionList", local_name: "extLst", prefix: "dgm", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIAGRAM_DEFINITION_EXTENSION_LIST },
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

/// Create a `<dgm:colorsDef>` element (`ColorsDefinition`).
pub fn colors_definition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "colorsDef").with_children(children)
}

/// Create a `<dgm:colorsDefHdr>` element (`ColorsDefinitionHeader`).
pub fn colors_definition_header(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "colorsDefHdr").with_children(children)
}

/// Create a `<dgm:colorsDefHdrLst>` element (`ColorsDefinitionHeaderList`).
pub fn colors_definition_header_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "colorsDefHdrLst").with_children(children)
}

/// Create a `<dgm:dataModel>` element (`DataModelRoot`).
pub fn data_model_root(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "dataModel").with_children(children)
}

/// Create a `<dgm:layoutDef>` element (`LayoutDefinition`).
pub fn layout_definition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "layoutDef").with_children(children)
}

/// Create a `<dgm:layoutDefHdr>` element (`LayoutDefinitionHeader`).
pub fn layout_definition_header(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "layoutDefHdr").with_children(children)
}

/// Create a `<dgm:layoutDefHdrLst>` element (`LayoutDefinitionHeaderList`).
pub fn layout_definition_header_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "layoutDefHdrLst").with_children(children)
}

/// Create a `<dgm:relIds>` element (`RelationshipIds`).
pub fn relationship_ids() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "relIds")
}

/// Create a `<dgm:styleDef>` element (`StyleDefinition`).
pub fn style_definition(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "styleDef").with_children(children)
}

/// Create a `<dgm:styleDefHdr>` element (`StyleDefinitionHeader`).
pub fn style_definition_header(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "styleDefHdr").with_children(children)
}

/// Create a `<dgm:styleDefHdrLst>` element (`StyleDefinitionHeaderList`).
pub fn style_definition_header_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "styleDefHdrLst").with_children(children)
}

/// Create a `<dgm:cat>` element (`ColorTransformCategory`).
pub fn color_transform_category() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "cat")
}

/// Create a `<dgm:fillClrLst>` element (`FillColorList`).
pub fn fill_color_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "fillClrLst").with_children(children)
}

/// Create a `<dgm:linClrLst>` element (`LineColorList`).
pub fn line_color_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "linClrLst").with_children(children)
}

/// Create a `<dgm:effectClrLst>` element (`EffectColorList`).
pub fn effect_color_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "effectClrLst").with_children(children)
}

/// Create a `<dgm:txLinClrLst>` element (`TextLineColorList`).
pub fn text_line_color_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "txLinClrLst").with_children(children)
}

/// Create a `<dgm:txFillClrLst>` element (`TextFillColorList`).
pub fn text_fill_color_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "txFillClrLst").with_children(children)
}

/// Create a `<dgm:txEffectClrLst>` element (`TextEffectColorList`).
pub fn text_effect_color_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "txEffectClrLst").with_children(children)
}

/// Create a `<dgm:extLst>` element (`ExtensionList`).
pub fn extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<dgm:title>` element (`ColorDefinitionTitle`).
pub fn color_definition_title() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "title")
}

/// Create a `<dgm:desc>` element (`ColorTransformDescription`).
pub fn color_transform_description() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "desc")
}

/// Create a `<dgm:catLst>` element (`ColorTransformCategories`).
pub fn color_transform_categories(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "catLst").with_children(children)
}

/// Create a `<dgm:styleLbl>` element (`ColorTransformStyleLabel`).
pub fn color_transform_style_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "styleLbl").with_children(children)
}

/// Create a `<dgm:pt>` element (`Point`).
pub fn point(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "pt").with_children(children)
}

/// Create a `<dgm:cxn>` element (`Connection`).
pub fn connection(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "cxn").with_children(children)
}

/// Create a `<dgm:constr>` element (`Constraint`).
pub fn constraint(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "constr").with_children(children)
}

/// Create a `<dgm:rule>` element (`Rule`).
pub fn rule(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "rule").with_children(children)
}

/// Create a `<dgm:adj>` element (`Adjust`).
pub fn adjust() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "adj")
}

/// Create a `<dgm:adjLst>` element (`AdjustList`).
pub fn adjust_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "adjLst").with_children(children)
}

/// Create a `<dgm:param>` element (`Parameter`).
pub fn parameter() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "param")
}

/// Create a `<dgm:alg>` element (`Algorithm`).
pub fn algorithm(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "alg").with_children(children)
}

/// Create a `<dgm:shape>` element (`Shape`).
pub fn shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "shape").with_children(children)
}

/// Create a `<dgm:presOf>` element (`PresentationOf`).
pub fn presentation_of(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "presOf").with_children(children)
}

/// Create a `<dgm:constrLst>` element (`Constraints`).
pub fn constraints(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "constrLst").with_children(children)
}

/// Create a `<dgm:ruleLst>` element (`RuleList`).
pub fn rule_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "ruleLst").with_children(children)
}

/// Create a `<dgm:varLst>` element (`VariableList`).
pub fn variable_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "varLst").with_children(children)
}

/// Create a `<dgm:presLayoutVars>` element (`PresentationLayoutVariables`).
pub fn presentation_layout_variables(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "presLayoutVars").with_children(children)
}

/// Create a `<dgm:forEach>` element (`ForEach`).
pub fn for_each(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "forEach").with_children(children)
}

/// Create a `<dgm:layoutNode>` element (`LayoutNode`).
pub fn layout_node(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "layoutNode").with_children(children)
}

/// Create a `<dgm:choose>` element (`Choose`).
pub fn choose(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "choose").with_children(children)
}

/// Create a `<dgm:if>` element (`DiagramChooseIf`).
pub fn diagram_choose_if(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "if").with_children(children)
}

/// Create a `<dgm:else>` element (`DiagramChooseElse`).
pub fn diagram_choose_else(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "else").with_children(children)
}

/// Create a `<dgm:dataModel>` element (`DataModel`).
pub fn data_model(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "dataModel").with_children(children)
}

/// Create a `<dgm:cat>` element (`Category`).
pub fn category() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "cat")
}

/// Create a `<dgm:title>` element (`Title`).
pub fn title() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "title")
}

/// Create a `<dgm:desc>` element (`Description`).
pub fn description() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "desc")
}

/// Create a `<dgm:catLst>` element (`CategoryList`).
pub fn category_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "catLst").with_children(children)
}

/// Create a `<dgm:style>` element (`Style`).
pub fn style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<dgm:orgChart>` element (`OrganizationChart`).
pub fn organization_chart() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "orgChart")
}

/// Create a `<dgm:chMax>` element (`MaxNumberOfChildren`).
pub fn max_number_of_children() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "chMax")
}

/// Create a `<dgm:chPref>` element (`PreferredNumberOfChildren`).
pub fn preferred_number_of_children() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "chPref")
}

/// Create a `<dgm:bulletEnabled>` element (`BulletEnabled`).
pub fn bullet_enabled() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "bulletEnabled")
}

/// Create a `<dgm:dir>` element (`Direction`).
pub fn direction() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "dir")
}

/// Create a `<dgm:hierBranch>` element (`HierarchyBranch`).
pub fn hierarchy_branch() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "hierBranch")
}

/// Create a `<dgm:animOne>` element (`AnimateOneByOne`).
pub fn animate_one_by_one() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "animOne")
}

/// Create a `<dgm:animLvl>` element (`AnimationLevel`).
pub fn animation_level() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "animLvl")
}

/// Create a `<dgm:resizeHandles>` element (`ResizeHandles`).
pub fn resize_handles() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "resizeHandles")
}

/// Create a `<dgm:cat>` element (`StyleDisplayCategory`).
pub fn style_display_category() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "cat")
}

/// Create a `<dgm:scene3d>` element (`Scene3D`).
pub fn scene3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "scene3d").with_children(children)
}

/// Create a `<dgm:sp3d>` element (`Shape3D`).
pub fn shape3_d(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "sp3d").with_children(children)
}

/// Create a `<dgm:txPr>` element (`TextProperties`).
pub fn text_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "txPr").with_children(children)
}

/// Create a `<dgm:title>` element (`StyleDefinitionTitle`).
pub fn style_definition_title() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "title")
}

/// Create a `<dgm:desc>` element (`StyleLabelDescription`).
pub fn style_label_description() -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "desc")
}

/// Create a `<dgm:catLst>` element (`StyleDisplayCategories`).
pub fn style_display_categories(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "catLst").with_children(children)
}

/// Create a `<dgm:styleLbl>` element (`StyleLabel`).
pub fn style_label(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "styleLbl").with_children(children)
}

/// Create a `<dgm:ptLst>` element (`PointList`).
pub fn point_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "ptLst").with_children(children)
}

/// Create a `<dgm:cxnLst>` element (`ConnectionList`).
pub fn connection_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "cxnLst").with_children(children)
}

/// Create a `<dgm:bg>` element (`Background`).
pub fn background(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "bg").with_children(children)
}

/// Create a `<dgm:whole>` element (`Whole`).
pub fn whole(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "whole").with_children(children)
}

/// Create a `<dgm:extLst>` element (`DataModelExtensionList`).
pub fn data_model_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<dgm:prSet>` element (`PropertySet`).
pub fn property_set(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "prSet").with_children(children)
}

/// Create a `<dgm:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<dgm:t>` element (`TextBody`).
pub fn text_body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "t").with_children(children)
}

/// Create a `<dgm:extLst>` element (`PtExtensionList`).
pub fn pt_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<dgm:ext>` element (`DiagramDefinitionExtension`).
pub fn diagram_definition_extension(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "ext").with_children(children)
}

/// Create a `<dgm:sampData>` element (`SampleData`).
pub fn sample_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "sampData").with_children(children)
}

/// Create a `<dgm:styleData>` element (`StyleData`).
pub fn style_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "styleData").with_children(children)
}

/// Create a `<dgm:clrData>` element (`ColorData`).
pub fn color_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "clrData").with_children(children)
}

/// Create a `<dgm:extLst>` element (`DiagramDefinitionExtensionList`).
pub fn diagram_definition_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("dgm", NAMESPACE_URI, "extLst").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 82;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 79;
