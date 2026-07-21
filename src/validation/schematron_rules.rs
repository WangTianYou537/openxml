// @generated from Open-XML-SDK/data/schematrons.json — do not edit by hand
// Relationship-existence and unique-attribute rules extracted from Schematron entries.
// Regenerate: python3 scripts/generate_schematron_rules.py
use super::semantic::{RelationshipExistRule, UniqueAttributeRule};

/// All extractable relationship-existence rules from schematrons.json.
pub fn schematron_relationship_rules() -> Vec<RelationshipExistRule> {
    vec![
        RelationshipExistRule::new("control", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/control")), // All
        RelationshipExistRule::new("embedBold", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/font")), // All
        RelationshipExistRule::new("altChunk", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk")), // All
        RelationshipExistRule::new("relIds", "cs", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors")), // All
        RelationshipExistRule::new("relIds", "dm", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData")), // All
        RelationshipExistRule::new("relIds", "lo", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout")), // All
        RelationshipExistRule::new("relIds", "qs", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle")), // All
        RelationshipExistRule::new("shape", "blip", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")), // All
        RelationshipExistRule::new("fill", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")), // All
        RelationshipExistRule::new("imagedata", "href", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")), // All
        RelationshipExistRule::new("imagedata", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")), // All
        RelationshipExistRule::new("imagedata", "pict", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")), // All
        RelationshipExistRule::new("imagedata", "relid", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")), // All
        RelationshipExistRule::new("stroke", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")), // All
        RelationshipExistRule::new("footerReference", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer")), // All
        RelationshipExistRule::new("headerReference", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/header")), // All
        RelationshipExistRule::new("dataSource", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/mailMergeSource")), // All
        RelationshipExistRule::new("recipientData", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData")), // All
        RelationshipExistRule::new("src", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/mailMergeSource")), // All
        RelationshipExistRule::new("attachedTemplate", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/attachedTemplate")), // All
        RelationshipExistRule::new("saveThroughXslt", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/transform")), // All
        RelationshipExistRule::new("sourceFileName", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/frame")), // All
        RelationshipExistRule::new("subDoc", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/subDocument")), // All
        RelationshipExistRule::new("printerSettings", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings")), // All
        RelationshipExistRule::new("embedBoldItalic", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/font")), // All
        RelationshipExistRule::new("embedItalic", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/font")), // All
        RelationshipExistRule::new("embedRegular", "id", Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/font")), // All
        RelationshipExistRule::new("sldLayoutId", "id", None::<&str>), // All
        RelationshipExistRule::new("sldId", "id", None::<&str>), // All
        RelationshipExistRule::new("sheet", "id", None::<&str>), // All
        RelationshipExistRule::new("sldMasterId", "id", None::<&str>), // All
        RelationshipExistRule::new("stroke", "id", None::<&str>), // All
        RelationshipExistRule::new("hlinkClick", "id", None::<&str>), // All
        RelationshipExistRule::new("chart", "id", None::<&str>), // All
        RelationshipExistRule::new("hyperlink", "id", None::<&str>), // All
        RelationshipExistRule::new("drawing", "id", None::<&str>), // All
        RelationshipExistRule::new("attachedTemplate", "id", None::<&str>), // All
        RelationshipExistRule::new("fill", "id", None::<&str>), // All
        RelationshipExistRule::new("oleObj", "id", None::<&str>), // All
        RelationshipExistRule::new("externalData", "id", None::<&str>), // All
        RelationshipExistRule::new("pivotCache", "id", None::<&str>), // All
        RelationshipExistRule::new("notesMasterId", "id", None::<&str>), // All
        RelationshipExistRule::new("imagedata", "id", None::<&str>), // All
        RelationshipExistRule::new("footerReference", "id", None::<&str>), // All
        RelationshipExistRule::new("headerReference", "id", None::<&str>), // All
        RelationshipExistRule::new("handoutMasterId", "id", None::<&str>), // All
        RelationshipExistRule::new("legacyDrawing", "id", None::<&str>), // All
        RelationshipExistRule::new("embedRegular", "id", None::<&str>), // All
        RelationshipExistRule::new("sld", "id", None::<&str>), // All
        RelationshipExistRule::new("hlinkMouseOver", "id", None::<&str>), // All
        RelationshipExistRule::new("worksheetSource", "id", None::<&str>), // All
        RelationshipExistRule::new("shape", "blip", None::<&str>), // All
        RelationshipExistRule::new("blip", "embed", None::<&str>), // All
        RelationshipExistRule::new("blip", "link", None::<&str>), // All
        RelationshipExistRule::new("relIds", "dm", None::<&str>), // All
        RelationshipExistRule::new("imagedata", "href", None::<&str>), // All
        RelationshipExistRule::new("altChunk", "id", None::<&str>), // All
        RelationshipExistRule::new("contentPart", "id", None::<&str>), // All
        RelationshipExistRule::new("slicerCache", "id", None::<&str>), // All
        RelationshipExistRule::new("slicer", "id", None::<&str>), // All
        RelationshipExistRule::new("media", "link", None::<&str>), // All
        RelationshipExistRule::new("media", "embed", None::<&str>), // All
        RelationshipExistRule::new("webextensionref", "id", None::<&str>), // Word, Excel
    ]
}

/// All extractable unique-attribute rules from schematrons.json.
pub fn schematron_unique_attribute_rules() -> Vec<UniqueAttributeRule> {
    vec![
        UniqueAttributeRule::new("endnote", "id", false), // All
        UniqueAttributeRule::new("footnote", "id", false), // All
        UniqueAttributeRule::new("sheet", "name", true), // All
        UniqueAttributeRule::new("sheet", "sheetId", false), // All
        UniqueAttributeRule::new("customPr", "name", true), // All
        UniqueAttributeRule::new("webPublishItem", "id", false), // All
        UniqueAttributeRule::new("connection", "name", false), // All
        UniqueAttributeRule::new("DataBinding", "DataBindingName", false), // All
        UniqueAttributeRule::new("DataBinding", "FileBindingName", false), // All
        UniqueAttributeRule::new("docPr", "id", false), // All
        UniqueAttributeRule::new("cNvPr", "id", false), // All
        UniqueAttributeRule::new("singleXmlCell", "id", false), // All
        UniqueAttributeRule::new("xmlCellPr", "uniqueName", false), // All
        UniqueAttributeRule::new("comment", "guid", true), // All
        UniqueAttributeRule::new("cacheField", "name", false), // All
        UniqueAttributeRule::new("cacheHierarchy", "allUniqueName", false), // All
        UniqueAttributeRule::new("cacheHierarchy", "defaultMemberUniqueName", false), // All
        UniqueAttributeRule::new("cacheHierarchy", "uniqueName", false), // All
        UniqueAttributeRule::new("cacheSource", "connectionId", false), // All
        UniqueAttributeRule::new("dimension", "uniqueName", false), // All
        UniqueAttributeRule::new("filter", "id", true), // All
        UniqueAttributeRule::new("group", "uniqueName", false), // All
        UniqueAttributeRule::new("groupLevel", "uniqueName", false), // All
        UniqueAttributeRule::new("groupMember", "uniqueName", false), // All
        UniqueAttributeRule::new("kpi", "goal", false), // All
        UniqueAttributeRule::new("kpi", "status", false), // All
        UniqueAttributeRule::new("kpi", "trend", false), // All
        UniqueAttributeRule::new("kpi", "uniqueName", false), // All
        UniqueAttributeRule::new("kpi", "value", false), // All
        UniqueAttributeRule::new("kpi", "weight", false), // All
        UniqueAttributeRule::new("pageField", "name", false), // All
        UniqueAttributeRule::new("queryTableField", "id", false), // All
        UniqueAttributeRule::new("queryTableField", "name", false), // All
        UniqueAttributeRule::new("connection", "id", false), // All
        UniqueAttributeRule::new("Map", "SchemaID", false), // All
        UniqueAttributeRule::new("Schema", "ID", false), // All
        UniqueAttributeRule::new("sldMasterId", "id", true), // All
        UniqueAttributeRule::new("sldLayoutId", "id", false), // All
        UniqueAttributeRule::new("cmAuthor", "id", true), // All
        UniqueAttributeRule::new("tblStyle", "styleId", false), // All
        UniqueAttributeRule::new("tableStyle", "styleId", false), // All
        UniqueAttributeRule::new("legacyDrawing", "spid", false), // All
        UniqueAttributeRule::new("choose", "name", false), // All
        UniqueAttributeRule::new("else", "name", false), // All
        UniqueAttributeRule::new("forEach", "name", false), // All
        UniqueAttributeRule::new("if", "name", false), // All
        UniqueAttributeRule::new("layoutNode", "name", false), // All
        UniqueAttributeRule::new("cxn", "parTransId", false), // All
        UniqueAttributeRule::new("arc", "id", false), // All
        UniqueAttributeRule::new("background", "id", false), // All
        UniqueAttributeRule::new("curve", "id", false), // All
        UniqueAttributeRule::new("fill", "id", true), // All
        UniqueAttributeRule::new("group", "id", false), // All
        UniqueAttributeRule::new("image", "id", false), // All
        UniqueAttributeRule::new("imagedata", "id", true), // All
        UniqueAttributeRule::new("oval", "id", false), // All
        UniqueAttributeRule::new("path", "id", false), // All
        UniqueAttributeRule::new("rect", "id", false), // All
        UniqueAttributeRule::new("roundrect", "id", false), // All
        UniqueAttributeRule::new("shadow", "id", false), // All
        UniqueAttributeRule::new("shape", "id", false), // All
        UniqueAttributeRule::new("shapetype", "id", false), // All
        UniqueAttributeRule::new("stroke", "id", true), // All
        UniqueAttributeRule::new("textbox", "id", false), // All
        UniqueAttributeRule::new("textpath", "id", false), // All
        UniqueAttributeRule::new("signatureline", "id", false), // All
        UniqueAttributeRule::new("property", "name", true), // All
        UniqueAttributeRule::new("vstream", "version", false), // All
        UniqueAttributeRule::new("abstractNum", "abstractNumId", false), // All
        UniqueAttributeRule::new("numPicBullet", "numPicBulletId", false), // All
        UniqueAttributeRule::new("guid", "val", true), // All
        UniqueAttributeRule::new("comment", "id", false), // All
        UniqueAttributeRule::new("commentRangeEnd", "id", false), // All
        UniqueAttributeRule::new("commentRangeStart", "id", false), // All
        UniqueAttributeRule::new("commentReference", "id", false), // All
        UniqueAttributeRule::new("cellDel", "id", false), // All
        UniqueAttributeRule::new("cellIns", "id", false), // All
        UniqueAttributeRule::new("cellMerge", "id", false), // All
        UniqueAttributeRule::new("customXmlDelRangeEnd", "id", true), // All
        UniqueAttributeRule::new("customXmlDelRangeStart", "id", true), // All
        UniqueAttributeRule::new("customXmlInsRangeEnd", "id", false), // All
        UniqueAttributeRule::new("customXmlInsRangeStart", "id", false), // All
        UniqueAttributeRule::new("customXmlMoveFromRangeEnd", "id", false), // All
        UniqueAttributeRule::new("customXmlMoveFromRangeStart", "id", false), // All
        UniqueAttributeRule::new("customXmlMoveToRangeStart", "id", false), // All
        UniqueAttributeRule::new("moveFrom", "id", false), // All
        UniqueAttributeRule::new("moveFromRangeEnd", "id", false), // All
        UniqueAttributeRule::new("moveFromRangeStart", "id", false), // All
        UniqueAttributeRule::new("moveTo", "id", false), // All
        UniqueAttributeRule::new("moveToRangeEnd", "id", false), // All
        UniqueAttributeRule::new("moveToRangeStart", "id", true), // All
        UniqueAttributeRule::new("numberingChange", "id", false), // All
        UniqueAttributeRule::new("pPrChange", "id", false), // All
        UniqueAttributeRule::new("sectPrChange", "id", false), // All
        UniqueAttributeRule::new("tblGridChange", "id", false), // All
        UniqueAttributeRule::new("tblPrChange", "id", false), // All
        UniqueAttributeRule::new("tblPrExChange", "id", false), // All
        UniqueAttributeRule::new("tcPrChange", "id", false), // All
        UniqueAttributeRule::new("trPrChange", "id", false), // All
        UniqueAttributeRule::new("bookmarkEnd", "id", false), // All
        UniqueAttributeRule::new("bookmarkStart", "id", false), // All
        UniqueAttributeRule::new("permEnd", "id", false), // All
        UniqueAttributeRule::new("permStart", "id", false), // All
        UniqueAttributeRule::new("autoCaption", "name", false), // All
        UniqueAttributeRule::new("div", "id", false), // All
        UniqueAttributeRule::new("control", "name", false), // All
        UniqueAttributeRule::new("style", "styleId", false), // All
        UniqueAttributeRule::new("del", "id", false), // All
        UniqueAttributeRule::new("ins", "id", false), // All
        UniqueAttributeRule::new("rPrChange", "id", false), // All
        UniqueAttributeRule::new("sldId", "id", false), // All
        UniqueAttributeRule::new("custShow", "id", true), // PowerPoint
        UniqueAttributeRule::new("dataField", "uniqueName", false), // All
        UniqueAttributeRule::new("cfRule", "priority", false), // All
        UniqueAttributeRule::new("slicer", "name", true), // All
    ]
}

/// Counts: 63 relationship rules, 115 unique-attribute rules (of 948 source rules).
pub const SCHEMATRON_EXTRACTED_REL_COUNT: usize = 63;
pub const SCHEMATRON_EXTRACTED_UNIQUE_COUNT: usize = 115;
pub const SCHEMATRON_TOTAL_SOURCE_RULES: usize = 948;
