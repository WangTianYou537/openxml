//! Open XML namespace constants and helpers.
//!
//! Mirrors `DocumentFormat.OpenXml.Framework.OpenXmlNamespace` / `data/namespaces.json`.

/// A namespace URI with its conventional prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Namespace {
    pub prefix: &'static str,
    pub uri: &'static str,
}

impl Namespace {
    pub const fn new(prefix: &'static str, uri: &'static str) -> Self {
        Self { prefix, uri }
    }
}

/// Lightweight namespace URI value (C# `OpenXmlNamespace` struct shell).
///
/// Distinct from [`Namespace`], which pairs a conventional prefix with a static URI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct OpenXmlNamespace {
    uri: String,
}

impl OpenXmlNamespace {
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn is_empty(&self) -> bool {
        self.uri.is_empty()
    }

    pub fn from_static(ns: Namespace) -> Self {
        Self {
            uri: ns.uri.to_string(),
        }
    }
}

impl From<&str> for OpenXmlNamespace {
    fn from(uri: &str) -> Self {
        Self::new(uri)
    }
}

impl From<String> for OpenXmlNamespace {
    fn from(uri: String) -> Self {
        Self::new(uri)
    }
}

impl From<Namespace> for OpenXmlNamespace {
    fn from(ns: Namespace) -> Self {
        Self::from_static(ns)
    }
}

impl std::fmt::Display for OpenXmlNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.uri)
    }
}

impl PartialOrd for OpenXmlNamespace {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpenXmlNamespace {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uri.cmp(&other.uri)
    }
}

#[cfg(test)]
mod open_xml_namespace_tests {
    use super::*;

    #[test]
    fn open_xml_namespace_basics() {
        let ns = OpenXmlNamespace::new("http://example.com/ns");
        assert_eq!(ns.uri(), "http://example.com/ns");
        assert!(!ns.is_empty());
        assert_eq!(ns.to_string(), "http://example.com/ns");
        let empty = OpenXmlNamespace::default();
        assert!(empty.is_empty());
        let w = OpenXmlNamespace::from_static(ns::WORDPROCESSINGML);
        assert_eq!(w.uri(), ns::WORDPROCESSINGML.uri);
        assert!(OpenXmlNamespace::new("a") < OpenXmlNamespace::new("b"));
    }
}

/// Common Open XML namespaces.
pub mod ns {
    use super::Namespace;

    pub const WORDPROCESSINGML: Namespace = Namespace::new(
        "w",
        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
    );

    pub const SPREADSHEETML: Namespace = Namespace::new(
        "x",
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    );

    pub const PRESENTATIONML: Namespace = Namespace::new(
        "p",
        "http://schemas.openxmlformats.org/presentationml/2006/main",
    );

    pub const DRAWINGML: Namespace = Namespace::new(
        "a",
        "http://schemas.openxmlformats.org/drawingml/2006/main",
    );

    pub const RELATIONSHIPS: Namespace = Namespace::new(
        "r",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    );

    pub const PACKAGE_RELATIONSHIPS: Namespace = Namespace::new(
        "pr",
        "http://schemas.openxmlformats.org/package/2006/relationships",
    );

    pub const CONTENT_TYPES: Namespace = Namespace::new(
        "ct",
        "http://schemas.openxmlformats.org/package/2006/content-types",
    );

    pub const EXTENDED_PROPERTIES: Namespace = Namespace::new(
        "ep",
        "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
    );

    /// Extended properties also use the conventional `ap:` prefix in many docs.
    pub const EXTENDED_PROPERTIES_AP: Namespace = Namespace::new(
        "ap",
        "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
    );

    pub const CUSTOM_PROPERTIES: Namespace = Namespace::new(
        "op",
        "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties",
    );

    pub const DOC_PROPS_VTYPES: Namespace = Namespace::new(
        "vt",
        "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
    );

    pub const CORE_PROPERTIES: Namespace = Namespace::new(
        "cp",
        "http://schemas.openxmlformats.org/package/2006/metadata/core-properties",
    );

    pub const DC: Namespace = Namespace::new("dc", "http://purl.org/dc/elements/1.1/");
    pub const DCTERMS: Namespace = Namespace::new("dcterms", "http://purl.org/dc/terms/");
    pub const DCMITYPE: Namespace = Namespace::new("dcmitype", "http://purl.org/dc/dcmitype/");
    pub const XSI: Namespace =
        Namespace::new("xsi", "http://www.w3.org/2001/XMLSchema-instance");
    pub const XML: Namespace = Namespace::new("xml", "http://www.w3.org/XML/1998/namespace");
    pub const MARKUP_COMPATIBILITY: Namespace = Namespace::new(
        "mc",
        "http://schemas.openxmlformats.org/markup-compatibility/2006",
    );
}

/// Relationship type URIs used by Office Open XML packages.
pub mod rel {
    pub const OFFICE_DOCUMENT: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
    pub const CORE_PROPERTIES: &str =
        "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
    pub const EXTENDED_PROPERTIES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";
    pub const CUSTOM_PROPERTIES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties";
    pub const THUMBNAIL: &str =
        "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail";
    pub const DIGITAL_SIGNATURE_ORIGIN: &str =
        "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin";
    pub const DIGITAL_SIGNATURE: &str =
        "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature";
    pub const CUSTOM_XML: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
    pub const CUSTOM_XML_PROPS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps";
    pub const STYLES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
    pub const SETTINGS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings";
    pub const WEB_SETTINGS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings";
    pub const FONT_TABLE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable";
    pub const GLOSSARY_DOCUMENT: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument";
    pub const THEME: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
    pub const NUMBERING: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
    pub const IMAGE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
    pub const HYPERLINK: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
    pub const HEADER: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header";
    pub const FOOTER: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer";
    pub const COMMENTS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
    pub const VML_DRAWING: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing";
    pub const FOOTNOTES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes";
    pub const ENDNOTES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes";
    pub const WORKSHEET: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
    pub const CHARTSHEET: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet";
    pub const SHARED_STRINGS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";
    pub const AF_CHUNK: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk";
    pub const PACKAGE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
    pub const OLE_OBJECT: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
    pub const CHART: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
    pub const DRAWING: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
    pub const PIVOT_TABLE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable";
    pub const PIVOT_CACHE_DEFINITION: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition";
    pub const PIVOT_CACHE_RECORDS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords";
    pub const TABLE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table";
    pub const CALC_CHAIN: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain";
    pub const EXTERNAL_LINK: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink";
    pub const VBA_PROJECT: &str =
        "http://schemas.microsoft.com/office/2006/relationships/vbaProject";
    pub const SLICER: &str =
        "http://schemas.microsoft.com/office/2007/relationships/slicer";
    pub const SLICER_CACHE: &str =
        "http://schemas.microsoft.com/office/2007/relationships/slicerCache";
    pub const COMMENTS_EXTENDED: &str =
        "http://schemas.microsoft.com/office/2011/relationships/commentsExtended";
    pub const PEOPLE: &str =
        "http://schemas.microsoft.com/office/2011/relationships/people";
    pub const CONNECTIONS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections";
    pub const PRINTER_SETTINGS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings";
    pub const PRES_PROPS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps";
    pub const VIEW_PROPS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps";
    pub const TABLE_STYLES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles";
    pub const QUERY_TABLE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable";
    pub const VOLATILE_DEPENDENCIES: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies";
    pub const COMMENT_AUTHORS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors";
    pub const CUSTOM_UI: &str =
        "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility";
    pub const CUSTOM_UI_2007: &str =
        "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility";
    pub const TIMELINE: &str =
        "http://schemas.microsoft.com/office/2011/relationships/timeline";
    pub const TIMELINE_CACHE: &str =
        "http://schemas.microsoft.com/office/2011/relationships/timelineCache";
    pub const DOCUMENT_TASKS: &str =
        "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks";
    pub const WEB_EXTENSION: &str =
        "http://schemas.microsoft.com/office/2011/relationships/webextension";
    pub const WEB_EXTENSION_TASKPANES: &str =
        "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes";
    pub const DIAGRAM_DATA: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData";
    pub const DIAGRAM_LAYOUT: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout";
    pub const DIAGRAM_COLORS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors";
    pub const DIAGRAM_STYLE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle";
    pub const DIAGRAM_PERSIST_LAYOUT: &str =
        "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing";
    pub const CUSTOM_XML_MAPPINGS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps";
    pub const CHART_STYLE: &str =
        "http://schemas.microsoft.com/office/2011/relationships/chartStyle";
    pub const CHART_COLOR_STYLE: &str =
        "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle";
    pub const DIALOGSHEET: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet";
    pub const NAMED_SHEET_VIEW: &str =
        "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView";
    pub const CUSTOM_DATA: &str =
        "http://schemas.microsoft.com/office/2007/relationships/customData";
    pub const CUSTOM_DATA_PROPS: &str =
        "http://schemas.microsoft.com/office/2007/relationships/customDataProps";
    pub const LABEL_INFO: &str =
        "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels";
    pub const EXTENDED_CHART: &str =
        "http://schemas.microsoft.com/office/2014/relationships/chartEx";
    pub const PPT_MODERN_COMMENTS: &str =
        "http://schemas.microsoft.com/office/2018/10/relationships/comments";
    pub const PPT_AUTHORS: &str =
        "http://schemas.microsoft.com/office/2018/10/relationships/authors";
    pub const THEME_OVERRIDE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride";
    pub const SINGLE_CELL_TABLE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells";
    pub const USER_DEFINED_TAGS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags";
    pub const SLIDE_SYNC: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo";
    pub const MODEL_3D: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/model3d";
    pub const VBA_DATA: &str =
        "http://schemas.microsoft.com/office/2006/relationships/wordVbaData";
    pub const STYLES_WITH_EFFECTS: &str =
        "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects";
    pub const FEATURE_PROPERTY_BAG: &str =
        "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag";
    pub const RICH_VALUE: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue";
    pub const RICH_VALUE_STRUCTURE: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure";
    pub const RICH_VALUE_TYPES: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes";
    pub const MACRO_SHEET: &str =
        "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet";
    pub const CUSTOMIZATION: &str =
        "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations";
    pub const QAT: &str =
        "http://schemas.microsoft.com/office/2006/relationships/ui/customization";
    pub const CONTROL: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control";
    pub const CONTROL_PROPS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp";
    pub const THREADED_COMMENT: &str =
        "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment";
    pub const PERSON: &str =
        "http://schemas.microsoft.com/office/2017/10/relationships/person";
    pub const REVISION_HEADERS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders";
    pub const REVISION_LOG: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog";
    pub const USERS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames";
    pub const SORT_MAP: &str =
        "http://schemas.microsoft.com/office/2006/relationships/wsSortMap";
    pub const COMMENTS_IDS: &str =
        "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds";
    pub const COMMENTS_EXTENSIBLE: &str =
        "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible";
    pub const ATTACHED_TOOLBARS: &str =
        "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars";
    pub const RICH_STYLES: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/richStyles";
    pub const SUPPORTING_PROPERTY_BAG: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag";
    pub const SUPPORTING_PROPERTY_BAG_STRUCTURE: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure";
    pub const RD_ARRAY: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/rdArray";
    pub const RD_RICH_VALUE_WEB_IMAGE: &str =
        "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueWebImage";
    pub const CELL_METADATA: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata";
    pub const INT_MACRO_SHEET: &str =
        "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet";
    pub const EMBEDDED_CONTROL: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control";
    pub const CHART_DRAWING: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes";
    pub const EXTERNAL_WORKBOOK: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath";
    pub const LEGACY_DIAGRAM_TEXT: &str =
        "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText";
    pub const LEGACY_DIAGRAM_TEXT_INFO: &str =
        "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText";
    pub const EMBEDDED_CONTROL_PERSISTENCE: &str =
        "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary";
    pub const PACKAGE_EMBEDDED: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
    pub const CUSTOM_PROPERTY: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty";
    pub const FONT: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font";
    pub const SLIDE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
    pub const SLIDE_MASTER: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
    pub const SLIDE_LAYOUT: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
    pub const NOTES_SLIDE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide";
    pub const NOTES_MASTER: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster";
    pub const HANDOUT_MASTER: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster";
}

/// Content type strings used by Office Open XML packages.
pub mod content_type {
    pub const RELATIONSHIPS: &str =
        "application/vnd.openxmlformats-package.relationships+xml";
    pub const CORE_PROPERTIES: &str =
        "application/vnd.openxmlformats-package.core-properties+xml";
    pub const EXTENDED_PROPERTIES: &str =
        "application/vnd.openxmlformats-officedocument.extended-properties+xml";
    pub const CUSTOM_PROPERTIES: &str =
        "application/vnd.openxmlformats-officedocument.custom-properties+xml";
    pub const CUSTOM_XML: &str = "application/xml";
    pub const CUSTOM_XML_PROPERTIES: &str =
        "application/vnd.openxmlformats-officedocument.customXmlProperties+xml";
    pub const DIGITAL_SIGNATURE_ORIGIN: &str =
        "application/vnd.openxmlformats-package.digital-signature-origin";
    pub const DIGITAL_SIGNATURE_XML: &str =
        "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml";

    pub const WORD_DOCUMENT: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
    pub const WORD_TEMPLATE: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml";
    pub const WORD_STYLES: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";
    pub const WORD_SETTINGS: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml";
    pub const WORD_WEB_SETTINGS: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml";
    pub const WORD_FONT_TABLE: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml";
    pub const WORD_GLOSSARY: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml";
    pub const WORD_HEADER: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml";
    pub const WORD_FOOTER: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml";
    pub const WORD_COMMENTS: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml";
    pub const WORD_NUMBERING: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml";
    pub const WORD_FOOTNOTES: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml";
    pub const WORD_ENDNOTES: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml";

    pub const SPREADSHEET_SHEET: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
    pub const SPREADSHEET_WORKSHEET: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
    pub const SPREADSHEET_CHARTSHEET: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml";
    pub const SPREADSHEET_SHARED_STRINGS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";
    pub const SPREADSHEET_STYLES: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml";
    pub const DRAWINGML_CHART: &str =
        "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
    pub const SPREADSHEET_DRAWING: &str =
        "application/vnd.openxmlformats-officedocument.drawing+xml";
    pub const SPREADSHEET_COMMENTS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml";
    pub const VML_DRAWING: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";
    pub const SPREADSHEET_PIVOT_TABLE: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml";
    pub const SPREADSHEET_PIVOT_CACHE_DEFINITION: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml";
    pub const SPREADSHEET_PIVOT_CACHE_RECORDS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml";
    pub const SPREADSHEET_TABLE: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml";
    pub const SPREADSHEET_CALC_CHAIN: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml";
    pub const SPREADSHEET_EXTERNAL_LINK: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml";
    pub const VBA_PROJECT: &str = "application/vnd.ms-office.vbaProject";
    pub const SLICER: &str = "application/vnd.ms-excel.slicer+xml";
    pub const SLICER_CACHE: &str = "application/vnd.ms-excel.slicerCache+xml";
    pub const WORD_COMMENTS_EXTENDED: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml";
    pub const WORD_PEOPLE: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml";
    pub const SPREADSHEET_CONNECTIONS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml";
    pub const WORD_PRINTER_SETTINGS: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings";
    pub const SPREADSHEET_PRINTER_SETTINGS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings";
    pub const PRESENTATION_PROPS: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml";
    pub const PRESENTATION_VIEW_PROPS: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml";
    pub const PRESENTATION_TABLE_STYLES: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml";
    pub const SPREADSHEET_QUERY_TABLE: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml";
    pub const SPREADSHEET_VOLATILE_DEPS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml";
    pub const PRESENTATION_COMMENT_AUTHORS: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml";
    pub const PRESENTATION_COMMENTS: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.comments+xml";
    pub const CUSTOM_UI: &str = "application/xml";
    pub const TIMELINE: &str = "application/vnd.ms-excel.timeline+xml";
    pub const TIMELINE_CACHE: &str = "application/vnd.ms-excel.timelineCache+xml";
    pub const DOCUMENT_TASKS: &str = "application/vnd.ms-office.documenttasks+xml";
    pub const WEB_EXTENSION: &str = "application/vnd.ms-office.webextension+xml";
    pub const WEB_EXTENSION_TASKPANES: &str =
        "application/vnd.ms-office.webextensiontaskpanes+xml";
    pub const DIAGRAM_DATA: &str =
        "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml";
    pub const DIAGRAM_LAYOUT: &str =
        "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml";
    pub const DIAGRAM_COLORS: &str =
        "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml";
    pub const DIAGRAM_STYLE: &str =
        "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml";
    pub const DIAGRAM_PERSIST_LAYOUT: &str =
        "application/vnd.ms-office.drawingml.diagramDrawing+xml";
    pub const CUSTOM_XML_MAPPINGS: &str = "application/xml";
    pub const CHART_STYLE: &str = "application/vnd.ms-office.chartstyle+xml";
    pub const CHART_COLOR_STYLE: &str = "application/vnd.ms-office.chartcolorstyle+xml";
    pub const SPREADSHEET_DIALOGSHEET: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml";
    pub const NAMED_SHEET_VIEW: &str =
        "application/vnd.ms-excel.namedsheetviews+xml";
    pub const CUSTOM_DATA: &str = "application/binary";
    pub const CUSTOM_DATA_PROPS: &str =
        "application/vnd.ms-excel.customDataProperties+xml";
    pub const LABEL_INFO: &str = "application/vnd.ms-office.classificationlabels+xml";
    pub const EXTENDED_CHART: &str = "application/vnd.ms-office.chartex+xml";
    pub const PPT_MODERN_COMMENTS: &str =
        "application/vnd.ms-powerpoint.comments+xml";
    pub const PPT_AUTHORS: &str = "application/vnd.ms-powerpoint.authors+xml";
    pub const THEME_OVERRIDE: &str =
        "application/vnd.openxmlformats-officedocument.themeOverride+xml";
    pub const SINGLE_CELL_TABLE: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml";
    pub const USER_DEFINED_TAGS: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.tags+xml";
    pub const SLIDE_SYNC: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml";
    pub const MODEL_3D: &str = "model/gltf-binary";
    pub const VBA_DATA: &str = "application/vnd.ms-word.vbaData+xml";
    pub const STYLES_WITH_EFFECTS: &str =
        "application/vnd.ms-word.stylesWithEffects+xml";
    pub const FEATURE_PROPERTY_BAG: &str =
        "application/vnd.ms-excel.featurepropertybag+xml";
    pub const RICH_VALUE: &str = "application/vnd.ms-excel.rdrichvalue+xml";
    pub const RICH_VALUE_STRUCTURE: &str =
        "application/vnd.ms-excel.rdrichvaluestructure+xml";
    pub const RICH_VALUE_TYPES: &str =
        "application/vnd.ms-excel.rdrichvaluetypes+xml";
    pub const MACRO_SHEET: &str =
        "application/vnd.ms-excel.macrosheet+xml";
    pub const CUSTOMIZATION: &str =
        "application/vnd.ms-word.keyMapCustomizations+xml";
    pub const QAT: &str = "application/xml";
    pub const CONTROL_PROPS: &str =
        "application/vnd.ms-excel.controlproperties+xml";
    pub const THREADED_COMMENT: &str =
        "application/vnd.ms-excel.threadedcomments+xml";
    pub const PERSON: &str = "application/vnd.ms-excel.person+xml";
    pub const REVISION_HEADERS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml";
    pub const REVISION_LOG: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml";
    pub const USERS: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml";
    pub const SORT_MAP: &str =
        "application/vnd.ms-excel.wsSortMap+xml";
    pub const WORD_COMMENTS_IDS: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml";
    pub const WORD_COMMENTS_EXTENSIBLE: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml";
    pub const ATTACHED_TOOLBARS: &str =
        "application/vnd.ms-word.attachedToolbars";
    pub const EXCEL_ATTACHED_TOOLBARS: &str =
        "application/vnd.ms-excel.attachedToolbars";
    pub const RICH_STYLES: &str =
        "application/vnd.ms-excel.richstyles+xml";
    pub const SUPPORTING_PROPERTY_BAG: &str =
        "application/vnd.ms-excel.rdsupportingpropertybag+xml";
    pub const SUPPORTING_PROPERTY_BAG_STRUCTURE: &str =
        "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml";
    pub const RD_ARRAY: &str = "application/vnd.ms-excel.rdarray+xml";
    pub const RD_RICH_VALUE_WEB_IMAGE: &str =
        "application/vnd.ms-excel.rdrichvaluewebimage+xml";
    pub const CELL_METADATA: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml";
    pub const INT_MACRO_SHEET: &str =
        "application/vnd.ms-excel.intlmacrosheet+xml";
    pub const EMBEDDED_CONTROL: &str =
        "application/vnd.ms-office.activeX+xml";
    pub const EMBEDDED_CONTROL_BIN: &str =
        "application/vnd.ms-office.activeX";
    pub const CHART_DRAWING: &str =
        "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml";
    pub const LEGACY_DIAGRAM_TEXT: &str = "application/vnd.ms-office.legacyDiagramText";
    pub const LEGACY_DIAGRAM_TEXT_INFO: &str =
        "application/vnd.ms-office.legacyDocTextInfo";
    pub const EMBEDDED_CONTROL_PERSISTENCE: &str =
        "application/vnd.ms-office.activeX";
    pub const PACKAGE_EMBEDDED: &str =
        "application/vnd.openxmlformats-officedocument.package";
    pub const CUSTOM_PROPERTY_SPREADSHEET: &str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.customProperty";
    pub const CUSTOM_PROPERTY_XML: &str = "application/xml";
    pub const FONT_DATA: &str = "application/x-fontdata";
    pub const FONT_TTF: &str = "application/x-font-ttf";
    pub const FONT_ODTTF: &str =
        "application/vnd.openxmlformats-officedocument.obfuscatedFont";

    pub const PRESENTATION: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
    pub const PRESENTATION_SLIDE: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
    pub const PRESENTATION_SLIDE_MASTER: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
    pub const PRESENTATION_SLIDE_LAYOUT: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
    pub const PRESENTATION_NOTES_SLIDE: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml";
    pub const PRESENTATION_NOTES_MASTER: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml";
    pub const PRESENTATION_HANDOUT_MASTER: &str =
        "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml";

    pub const THEME: &str = "application/vnd.openxmlformats-officedocument.theme+xml";

    pub const IMAGE_PNG: &str = "image/png";
    pub const IMAGE_JPEG: &str = "image/jpeg";
    pub const IMAGE_GIF: &str = "image/gif";
    pub const IMAGE_BMP: &str = "image/bmp";
    pub const IMAGE_TIFF: &str = "image/tiff";
    pub const IMAGE_EMF: &str = "image/x-emf";
    pub const IMAGE_WMF: &str = "image/x-wmf";
    pub const IMAGE_SVG: &str = "image/svg+xml";
}
