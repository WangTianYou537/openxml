//! Auto-generated from `schemas_microsoft_com_office_word_2010_wordprocessingShape.json`.
//! Target namespace: `http://schemas.microsoft.com/office/word/2010/wordprocessingShape` (prefix `wps`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.microsoft.com/office/word/2010/wordprocessingShape";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "wps";

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

static ATTRS_WORDPROCESSING_SHAPE: &[AttributeInfo] = &[
    AttributeInfo { qname: ":normalEastAsianFlow", property_name: Some("NormalEastAsianFlow"), type_name: "BooleanValue" },
];
static CHILDREN_WORDPROCESSING_SHAPE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_NonVisualDrawingProps/wps:cNvPr", property_name: Some("NonVisualDrawingProperties") },
    ChildInfo { name: "a:CT_NonVisualDrawingShapeProps/wps:cNvSpPr", property_name: None },
    ChildInfo { name: "a:CT_NonVisualConnectorProperties/wps:cNvCnPr", property_name: None },
    ChildInfo { name: "a:CT_ShapeProperties/wps:spPr", property_name: None },
    ChildInfo { name: "a:CT_ShapeStyle/wps:style", property_name: None },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wps:extLst", property_name: None },
    ChildInfo { name: "wps:CT_TextboxInfo/wps:txbx", property_name: None },
    ChildInfo { name: "wps:CT_LinkedTextboxInformation/wps:linkedTxbx", property_name: None },
    ChildInfo { name: "a:CT_TextBodyProperties/wps:bodyPr", property_name: None },
];
static CHILDREN_OFFICE_ART_EXTENSION_LIST: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtension/a:ext", property_name: None },
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
static ATTRS_NON_VISUAL_DRAWING_SHAPE_PROPERTIES: &[AttributeInfo] = &[
    AttributeInfo { qname: ":txBox", property_name: Some("TextBox"), type_name: "BooleanValue" },
];
static CHILDREN_NON_VISUAL_DRAWING_SHAPE_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ShapeLocking/a:spLocks", property_name: Some("ShapeLocks") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
];
static CHILDREN_NON_VISUAL_CONNECTOR_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_ConnectorLocking/a:cxnSpLocks", property_name: Some("ConnectionShapeLocks") },
    ChildInfo { name: "a:CT_Connection/a:stCxn", property_name: Some("StartConnection") },
    ChildInfo { name: "a:CT_Connection/a:endCxn", property_name: Some("EndConnection") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/a:extLst", property_name: Some("ExtensionList") },
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
static CHILDREN_SHAPE_STYLE: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_StyleMatrixReference/a:lnRef", property_name: Some("LineReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:fillRef", property_name: Some("FillReference") },
    ChildInfo { name: "a:CT_StyleMatrixReference/a:effectRef", property_name: Some("EffectReference") },
    ChildInfo { name: "a:CT_FontReference/a:fontRef", property_name: Some("FontReference") },
];
static ATTRS_TEXT_BOX_INFO2: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt16Value" },
];
static CHILDREN_TEXT_BOX_INFO2: &[ChildInfo] = &[
    ChildInfo { name: "w:CT_TxbxContent/w:txbxContent", property_name: Some("TextBoxContent") },
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wps:extLst", property_name: Some("OfficeArtExtensionList") },
];
static ATTRS_LINKED_TEXT_BOX: &[AttributeInfo] = &[
    AttributeInfo { qname: ":id", property_name: Some("Id"), type_name: "UInt16Value" },
    AttributeInfo { qname: ":seq", property_name: Some("Sequence"), type_name: "UInt16Value" },
];
static CHILDREN_LINKED_TEXT_BOX: &[ChildInfo] = &[
    ChildInfo { name: "a:CT_OfficeArtExtensionList/wps:extLst", property_name: Some("OfficeArtExtensionList") },
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

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "WordprocessingShape", local_name: "wsp", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_WORDPROCESSING_SHAPE, children: CHILDREN_WORDPROCESSING_SHAPE },
    ElementInfo { class_name: "OfficeArtExtensionList", local_name: "extLst", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_OFFICE_ART_EXTENSION_LIST },
    ElementInfo { class_name: "NonVisualDrawingProperties", local_name: "cNvPr", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_PROPERTIES },
    ElementInfo { class_name: "NonVisualDrawingShapeProperties", local_name: "cNvSpPr", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_NON_VISUAL_DRAWING_SHAPE_PROPERTIES, children: CHILDREN_NON_VISUAL_DRAWING_SHAPE_PROPERTIES },
    ElementInfo { class_name: "NonVisualConnectorProperties", local_name: "cNvCnPr", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_NON_VISUAL_CONNECTOR_PROPERTIES },
    ElementInfo { class_name: "ShapeProperties", local_name: "spPr", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_SHAPE_PROPERTIES, children: CHILDREN_SHAPE_PROPERTIES },
    ElementInfo { class_name: "ShapeStyle", local_name: "style", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_SHAPE_STYLE },
    ElementInfo { class_name: "TextBoxInfo2", local_name: "txbx", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_BOX_INFO2, children: CHILDREN_TEXT_BOX_INFO2 },
    ElementInfo { class_name: "LinkedTextBox", local_name: "linkedTxbx", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_LINKED_TEXT_BOX, children: CHILDREN_LINKED_TEXT_BOX },
    ElementInfo { class_name: "TextBodyProperties", local_name: "bodyPr", prefix: "wps", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: ATTRS_TEXT_BODY_PROPERTIES, children: CHILDREN_TEXT_BODY_PROPERTIES },
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

/// Create a `<wps:wsp>` element (`WordprocessingShape`).
pub fn wordprocessing_shape(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "wsp").with_children(children)
}

/// Create a `<wps:extLst>` element (`OfficeArtExtensionList`).
pub fn office_art_extension_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "extLst").with_children(children)
}

/// Create a `<wps:cNvPr>` element (`NonVisualDrawingProperties`).
pub fn non_visual_drawing_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "cNvPr").with_children(children)
}

/// Create a `<wps:cNvSpPr>` element (`NonVisualDrawingShapeProperties`).
pub fn non_visual_drawing_shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "cNvSpPr").with_children(children)
}

/// Create a `<wps:cNvCnPr>` element (`NonVisualConnectorProperties`).
pub fn non_visual_connector_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "cNvCnPr").with_children(children)
}

/// Create a `<wps:spPr>` element (`ShapeProperties`).
pub fn shape_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "spPr").with_children(children)
}

/// Create a `<wps:style>` element (`ShapeStyle`).
pub fn shape_style(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "style").with_children(children)
}

/// Create a `<wps:txbx>` element (`TextBoxInfo2`).
pub fn text_box_info2(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "txbx").with_children(children)
}

/// Create a `<wps:linkedTxbx>` element (`LinkedTextBox`).
pub fn linked_text_box(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "linkedTxbx").with_children(children)
}

/// Create a `<wps:bodyPr>` element (`TextBodyProperties`).
pub fn text_body_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("wps", NAMESPACE_URI, "bodyPr").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 10;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 10;
