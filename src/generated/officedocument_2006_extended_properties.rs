//! Auto-generated from `schemas_openxmlformats_org_officeDocument_2006_extended-properties.json`.
//! Target namespace: `http://schemas.openxmlformats.org/officeDocument/2006/extended-properties` (prefix `ap`).

use crate::element::OpenXmlElement;

/// Target namespace URI for this schema module.
pub const NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties";
/// Conventional prefix for this schema module.
pub const NAMESPACE_PREFIX: &str = "ap";

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

static CHILDREN_PROPERTIES: &[ChildInfo] = &[
    ChildInfo { name: "xsd:string/ap:Template", property_name: Some("Template") },
    ChildInfo { name: "xsd:string/ap:Manager", property_name: Some("Manager") },
    ChildInfo { name: "xsd:string/ap:Company", property_name: Some("Company") },
    ChildInfo { name: "xsd:int/ap:Pages", property_name: Some("Pages") },
    ChildInfo { name: "xsd:int/ap:Words", property_name: Some("Words") },
    ChildInfo { name: "xsd:int/ap:Characters", property_name: Some("Characters") },
    ChildInfo { name: "xsd:string/ap:PresentationFormat", property_name: Some("PresentationFormat") },
    ChildInfo { name: "xsd:int/ap:Lines", property_name: Some("Lines") },
    ChildInfo { name: "xsd:int/ap:Paragraphs", property_name: Some("Paragraphs") },
    ChildInfo { name: "xsd:int/ap:Slides", property_name: Some("Slides") },
    ChildInfo { name: "xsd:int/ap:Notes", property_name: Some("Notes") },
    ChildInfo { name: "xsd:int/ap:TotalTime", property_name: Some("TotalTime") },
    ChildInfo { name: "xsd:int/ap:HiddenSlides", property_name: Some("HiddenSlides") },
    ChildInfo { name: "xsd:int/ap:MMClips", property_name: Some("MultimediaClips") },
    ChildInfo { name: "xsd:boolean/ap:ScaleCrop", property_name: Some("ScaleCrop") },
    ChildInfo { name: "ap:CT_VectorVariant/ap:HeadingPairs", property_name: Some("HeadingPairs") },
    ChildInfo { name: "ap:CT_VectorLpstr/ap:TitlesOfParts", property_name: Some("TitlesOfParts") },
    ChildInfo { name: "xsd:boolean/ap:LinksUpToDate", property_name: Some("LinksUpToDate") },
    ChildInfo { name: "xsd:int/ap:CharactersWithSpaces", property_name: Some("CharactersWithSpaces") },
    ChildInfo { name: "xsd:boolean/ap:SharedDoc", property_name: Some("SharedDocument") },
    ChildInfo { name: "xsd:string/ap:HyperlinkBase", property_name: Some("HyperlinkBase") },
    ChildInfo { name: "ap:CT_VectorVariant/ap:HLinks", property_name: Some("HyperlinkList") },
    ChildInfo { name: "xsd:boolean/ap:HyperlinksChanged", property_name: Some("HyperlinksChanged") },
    ChildInfo { name: "ap:CT_DigSigBlob/ap:DigSig", property_name: Some("DigitalSignature") },
    ChildInfo { name: "xsd:string/ap:Application", property_name: Some("Application") },
    ChildInfo { name: "xsd:string/ap:AppVersion", property_name: Some("ApplicationVersion") },
    ChildInfo { name: "xsd:int/ap:DocSecurity", property_name: Some("DocumentSecurity") },
];
static CHILDREN_HEADING_PAIRS: &[ChildInfo] = &[
    ChildInfo { name: "vt:CT_Vector/vt:vector", property_name: Some("VTVector") },
];
static CHILDREN_HYPERLINK_LIST: &[ChildInfo] = &[
    ChildInfo { name: "vt:CT_Vector/vt:vector", property_name: Some("VTVector") },
];
static CHILDREN_TITLES_OF_PARTS: &[ChildInfo] = &[
    ChildInfo { name: "vt:CT_Vector/vt:vector", property_name: Some("VTVector") },
];
static CHILDREN_DIGITAL_SIGNATURE: &[ChildInfo] = &[
    ChildInfo { name: "xsd:base64Binary/vt:blob", property_name: Some("VTBlob") },
];

/// All concrete elements in this schema.
pub static ELEMENTS: &[ElementInfo] = &[
    ElementInfo { class_name: "Properties", local_name: "Properties", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_PROPERTIES },
    ElementInfo { class_name: "Template", local_name: "Template", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Manager", local_name: "Manager", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Company", local_name: "Company", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "PresentationFormat", local_name: "PresentationFormat", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HyperlinkBase", local_name: "HyperlinkBase", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Application", local_name: "Application", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ApplicationVersion", local_name: "AppVersion", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Pages", local_name: "Pages", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Words", local_name: "Words", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Characters", local_name: "Characters", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Lines", local_name: "Lines", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Paragraphs", local_name: "Paragraphs", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Slides", local_name: "Slides", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "Notes", local_name: "Notes", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "TotalTime", local_name: "TotalTime", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HiddenSlides", local_name: "HiddenSlides", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "MultimediaClips", local_name: "MMClips", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "CharactersWithSpaces", local_name: "CharactersWithSpaces", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "DocumentSecurity", local_name: "DocSecurity", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "ScaleCrop", local_name: "ScaleCrop", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "LinksUpToDate", local_name: "LinksUpToDate", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "SharedDocument", local_name: "SharedDoc", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HyperlinksChanged", local_name: "HyperlinksChanged", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: true, is_leaf_text: true, attributes: &[], children: &[] },
    ElementInfo { class_name: "HeadingPairs", local_name: "HeadingPairs", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HEADING_PAIRS },
    ElementInfo { class_name: "HyperlinkList", local_name: "HLinks", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_HYPERLINK_LIST },
    ElementInfo { class_name: "TitlesOfParts", local_name: "TitlesOfParts", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_TITLES_OF_PARTS },
    ElementInfo { class_name: "DigitalSignature", local_name: "DigSig", prefix: "ap", namespace_uri: NAMESPACE_URI, is_leaf: false, is_leaf_text: false, attributes: &[], children: CHILDREN_DIGITAL_SIGNATURE },
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

/// Create a `<ap:Properties>` element (`Properties`).
pub fn properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Properties").with_children(children)
}

/// Create a `<ap:Template>` element (`Template`).
pub fn template(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Template").with_text(value)
}

/// Create a `<ap:Manager>` element (`Manager`).
pub fn manager(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Manager").with_text(value)
}

/// Create a `<ap:Company>` element (`Company`).
pub fn company(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Company").with_text(value)
}

/// Create a `<ap:PresentationFormat>` element (`PresentationFormat`).
pub fn presentation_format(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "PresentationFormat").with_text(value)
}

/// Create a `<ap:HyperlinkBase>` element (`HyperlinkBase`).
pub fn hyperlink_base(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "HyperlinkBase").with_text(value)
}

/// Create a `<ap:Application>` element (`Application`).
pub fn application(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Application").with_text(value)
}

/// Create a `<ap:AppVersion>` element (`ApplicationVersion`).
pub fn application_version(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "AppVersion").with_text(value)
}

/// Create a `<ap:Pages>` element (`Pages`).
pub fn pages(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Pages").with_text(value)
}

/// Create a `<ap:Words>` element (`Words`).
pub fn words(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Words").with_text(value)
}

/// Create a `<ap:Characters>` element (`Characters`).
pub fn characters(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Characters").with_text(value)
}

/// Create a `<ap:Lines>` element (`Lines`).
pub fn lines(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Lines").with_text(value)
}

/// Create a `<ap:Paragraphs>` element (`Paragraphs`).
pub fn paragraphs(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Paragraphs").with_text(value)
}

/// Create a `<ap:Slides>` element (`Slides`).
pub fn slides(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Slides").with_text(value)
}

/// Create a `<ap:Notes>` element (`Notes`).
pub fn notes(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "Notes").with_text(value)
}

/// Create a `<ap:TotalTime>` element (`TotalTime`).
pub fn total_time(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "TotalTime").with_text(value)
}

/// Create a `<ap:HiddenSlides>` element (`HiddenSlides`).
pub fn hidden_slides(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "HiddenSlides").with_text(value)
}

/// Create a `<ap:MMClips>` element (`MultimediaClips`).
pub fn multimedia_clips(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "MMClips").with_text(value)
}

/// Create a `<ap:CharactersWithSpaces>` element (`CharactersWithSpaces`).
pub fn characters_with_spaces(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "CharactersWithSpaces").with_text(value)
}

/// Create a `<ap:DocSecurity>` element (`DocumentSecurity`).
pub fn document_security(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "DocSecurity").with_text(value)
}

/// Create a `<ap:ScaleCrop>` element (`ScaleCrop`).
pub fn scale_crop(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "ScaleCrop").with_text(value)
}

/// Create a `<ap:LinksUpToDate>` element (`LinksUpToDate`).
pub fn links_up_to_date(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "LinksUpToDate").with_text(value)
}

/// Create a `<ap:SharedDoc>` element (`SharedDocument`).
pub fn shared_document(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "SharedDoc").with_text(value)
}

/// Create a `<ap:HyperlinksChanged>` element (`HyperlinksChanged`).
pub fn hyperlinks_changed(value: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "HyperlinksChanged").with_text(value)
}

/// Create a `<ap:HeadingPairs>` element (`HeadingPairs`).
pub fn heading_pairs(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "HeadingPairs").with_children(children)
}

/// Create a `<ap:HLinks>` element (`HyperlinkList`).
pub fn hyperlink_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "HLinks").with_children(children)
}

/// Create a `<ap:TitlesOfParts>` element (`TitlesOfParts`).
pub fn titles_of_parts(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "TitlesOfParts").with_children(children)
}

/// Create a `<ap:DigSig>` element (`DigitalSignature`).
pub fn digital_signature(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("ap", NAMESPACE_URI, "DigSig").with_children(children)
}

/// Number of schema types in the source JSON (including abstract).
pub const TYPE_COUNT: usize = 29;
/// Number of concrete elements with a local name.
pub const ELEMENT_COUNT: usize = 28;
