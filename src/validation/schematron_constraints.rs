// @generated from Open-XML-SDK/data/schematrons.json — do not edit by hand
// Numeric range, string-length, matches(), and enum constraints extractable without full XPath.
// Regenerate: python3 scripts/generate_schematron_rules.py

/// A numeric attribute range constraint: element/@attr ∈ [min, max].
#[derive(Debug, Clone, Copy)]
pub struct NumericRangeRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub min: f64,
    pub max: f64,
}

/// A string-length constraint on an attribute.
#[derive(Debug, Clone, Copy)]
pub struct StringLengthRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub min: usize,
    pub max: usize,
}

/// A regex-like pattern constraint (`matches(@attr, "pattern")`).
/// Patterns are ECMAScript-ish Schematron strings; only simple subsets are enforced.
#[derive(Debug, Clone, Copy)]
pub struct PatternRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub pattern: &'static str,
}

/// An enumeration constraint: attribute value must be one of the listed tokens.
#[derive(Debug, Clone, Copy)]
pub struct EnumRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub values: &'static [&'static str],
}

/// Numeric range rules from schematrons.json.
pub fn schematron_numeric_range_rules() -> Vec<NumericRangeRule> {
    vec![
        NumericRangeRule { element: "customWorkbookView", attribute: "windowWidth", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "customWorkbookView", attribute: "tabRatio", min: f64::NEG_INFINITY, max: 1000.0 }, // All max-only
        NumericRangeRule { element: "customWorkbookView", attribute: "activeSheetId", min: 1.0, max: 65534.0 }, // All All
        NumericRangeRule { element: "functionGroups", attribute: "builtInGroupCount", min: f64::NEG_INFINITY, max: 255.0 }, // All max-only
        NumericRangeRule { element: "sheet", attribute: "sheetId", min: 1.0, max: 65534.0 }, // All All
        NumericRangeRule { element: "workbookView", attribute: "windowWidth", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "workbookView", attribute: "windowHeight", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "workbookView", attribute: "tabRatio", min: f64::NEG_INFINITY, max: 1000.0 }, // All max-only
        NumericRangeRule { element: "workbookView", attribute: "activeTab", min: 0.0, max: 32766.0 }, // All All
        NumericRangeRule { element: "c", attribute: "cm", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "c", attribute: "vm", min: f64::NEG_INFINITY, max: 2147483648.0 }, // All max-only
        NumericRangeRule { element: "c", attribute: "s", min: 0.0, max: 65490.0 }, // All All
        NumericRangeRule { element: "col", attribute: "outlineLevel", min: 0.0, max: 7.0 }, // All All
        NumericRangeRule { element: "col", attribute: "min", min: 1.0, max: 16384.0 }, // All All
        NumericRangeRule { element: "col", attribute: "max", min: 1.0, max: 16384.0 }, // All All
        NumericRangeRule { element: "col", attribute: "width", min: 0.0, max: 255.0 }, // All All
        NumericRangeRule { element: "col", attribute: "style", min: 0.0, max: 65429.0 }, // All All
        NumericRangeRule { element: "colBreaks", attribute: "count", min: f64::NEG_INFINITY, max: 1023.0 }, // All max-only
        NumericRangeRule { element: "colBreaks", attribute: "manualBreakCount", min: f64::NEG_INFINITY, max: 1023.0 }, // All max-only
        NumericRangeRule { element: "control", attribute: "shapeId", min: 1.0, max: 67098623.0 }, // All All
        NumericRangeRule { element: "customSheetView", attribute: "scale", min: 10.0, max: 400.0 }, // All All
        NumericRangeRule { element: "customSheetView", attribute: "colorId", min: f64::NEG_INFINITY, max: 64.0 }, // All max-only
        NumericRangeRule { element: "dataValidation", attribute: "sqref", min: 1.0, max: 32767.0 }, // All All
        NumericRangeRule { element: "ignoredError", attribute: "sqref", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "oleObject", attribute: "shapeId", min: 1.0, max: 67098623.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "left", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "right", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "top", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "bottom", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "header", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "footer", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "protectedRange", attribute: "sqref", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "row", attribute: "r", min: 1.0, max: 1048576.0 }, // All All
        NumericRangeRule { element: "row", attribute: "outlineLevel", min: 0.0, max: 7.0 }, // All All
        NumericRangeRule { element: "row", attribute: "s", min: 0.0, max: 65490.0 }, // All All
        NumericRangeRule { element: "rowBreaks", attribute: "count", min: f64::NEG_INFINITY, max: 1022.0 }, // All max-only
        NumericRangeRule { element: "rowBreaks", attribute: "manualBreakCount", min: f64::NEG_INFINITY, max: 1022.0 }, // All max-only
        NumericRangeRule { element: "sheetFormatPr", attribute: "outlineLevelCol", min: 0.0, max: 7.0 }, // All All
        NumericRangeRule { element: "sheetFormatPr", attribute: "outlineLevelRow", min: 0.0, max: 7.0 }, // All All
        NumericRangeRule { element: "sheetFormatPr", attribute: "baseColWidth", min: f64::NEG_INFINITY, max: 255.0 }, // All max-only
        NumericRangeRule { element: "sheetFormatPr", attribute: "defaultColWidth", min: 0.0, max: 65535.0 }, // All All
        NumericRangeRule { element: "sheetView", attribute: "colorId", min: f64::NEG_INFINITY, max: 64.0 }, // All max-only
        NumericRangeRule { element: "webPublishItem", attribute: "id", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "sst", attribute: "uniqueCount", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "sst", attribute: "count", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "sz", attribute: "val", min: 1.0, max: 409.55 }, // All All
        NumericRangeRule { element: "tableColumn", attribute: "queryTableFieldId", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "tableColumn", attribute: "id", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "singleXmlCell", attribute: "id", min: 1.0, max: 4294967294.0 }, // All All
        NumericRangeRule { element: "singleXmlCell", attribute: "connectionId", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "xmlPr", attribute: "mapId", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "cellStyle", attribute: "builtinId", min: 0.0, max: 53.0 }, // All All
        NumericRangeRule { element: "cellStyle", attribute: "iLevel", min: 0.0, max: 7.0 }, // All All
        NumericRangeRule { element: "gradientFill", attribute: "top", min: 0.0, max: 1.0 }, // All All
        NumericRangeRule { element: "gradientFill", attribute: "bottom", min: 0.0, max: 1.0 }, // All All
        NumericRangeRule { element: "gradientFill", attribute: "left", min: 0.0, max: 1.0 }, // All All
        NumericRangeRule { element: "gradientFill", attribute: "right", min: 0.0, max: 1.0 }, // All All
        NumericRangeRule { element: "gradientFill", attribute: "degree", min: -1.7e+308, max: 1.7e+308 }, // All All
        NumericRangeRule { element: "stop", attribute: "position", min: 0.0, max: 1.0 }, // All All
        NumericRangeRule { element: "tableStyleElement", attribute: "size", min: 1.0, max: 9.0 }, // All All
        NumericRangeRule { element: "rc", attribute: "t", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "connection", attribute: "type", min: 1.0, max: 8.0 }, // All All
        NumericRangeRule { element: "sheetData", attribute: "sheetId", min: 0.0, max: 65533.0 }, // All All
        NumericRangeRule { element: "arc", attribute: "hrpct", min: 0.0, max: 1000.0 }, // All All
        NumericRangeRule { element: "arc", attribute: "dgmnodekind", min: 0.0, max: 6.0 }, // All All
        NumericRangeRule { element: "pivotArea", attribute: "fieldPosition", min: 0.0, max: 255.0 }, // All All
        NumericRangeRule { element: "sheetView", attribute: "zoomScale", min: 10.0, max: 400.0 }, // All All
        NumericRangeRule { element: "sheetView", attribute: "zoomScaleNormal", min: 10.0, max: 400.0 }, // All All
        NumericRangeRule { element: "sheetView", attribute: "zoomScalePageLayoutView", min: 10.0, max: 400.0 }, // All All
        NumericRangeRule { element: "sheetView", attribute: "zoomScaleSheetLayoutView", min: 10.0, max: 400.0 }, // All All
        NumericRangeRule { element: "tabColor", attribute: "tint", min: -1.0, max: 1.0 }, // All All
        NumericRangeRule { element: "dateGroupItem", attribute: "day", min: 1.0, max: 31.0 }, // All All
        NumericRangeRule { element: "dateGroupItem", attribute: "hour", min: 0.0, max: 23.0 }, // All All
        NumericRangeRule { element: "dateGroupItem", attribute: "minute", min: 0.0, max: 59.0 }, // All All
        NumericRangeRule { element: "dateGroupItem", attribute: "month", min: 1.0, max: 12.0 }, // All All
        NumericRangeRule { element: "dateGroupItem", attribute: "second", min: 0.0, max: 59.0 }, // All All
        NumericRangeRule { element: "dateGroupItem", attribute: "year", min: 1000.0, max: 9999.0 }, // All All
        NumericRangeRule { element: "charset", attribute: "val", min: 0.0, max: 255.0 }, // All All
        NumericRangeRule { element: "alignment", attribute: "readingOrder", min: 0.0, max: 2.0 }, // All All
        NumericRangeRule { element: "bgColor", attribute: "tint", min: -1.0, max: 1.0 }, // All All
        NumericRangeRule { element: "fgColor", attribute: "tint", min: -1.0, max: 1.0 }, // All All
        NumericRangeRule { element: "xf", attribute: "borderId", min: 0.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "cellMetadata", attribute: "count", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "futureMetadata", attribute: "count", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "k", attribute: "n", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "k", attribute: "np", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "mdx", attribute: "n", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "mdxMetadata", attribute: "count", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "metadataStrings", attribute: "count", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "metadataTypes", attribute: "count", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "ms", attribute: "ns", min: 0.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "ms", attribute: "c", min: 0.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "n", attribute: "x", min: 0.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "n", attribute: "x", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "p", attribute: "n", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "p", attribute: "np", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "t", attribute: "si", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "t", attribute: "c", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "t", attribute: "fi", min: f64::NEG_INFINITY, max: 58.0 }, // All max-only
        NumericRangeRule { element: "valueMetadata", attribute: "count", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "fieldUsage", attribute: "x", min: -1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "group", attribute: "id", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "pivotTableDefinition", attribute: "autoFormatId", min: 0.0, max: 16.0 }, // All All
        NumericRangeRule { element: "pivotTableDefinition", attribute: "indent", min: f64::NEG_INFINITY, max: 127.0 }, // All max-only
        NumericRangeRule { element: "pivotTableDefinition", attribute: "pageWrap", min: f64::NEG_INFINITY, max: 255.0 }, // All max-only
        NumericRangeRule { element: "headers", attribute: "version", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "headers", attribute: "preserveHistory", min: 0.0, max: 32768.0 }, // All All
        NumericRangeRule { element: "headers", attribute: "revisionId", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "raf", attribute: "autoFormatId", min: 0.0, max: 16.0 }, // All All
        NumericRangeRule { element: "rcft", attribute: "sheetId", min: 0.0, max: 32767.0 }, // All All
        NumericRangeRule { element: "rdn", attribute: "functionGroupId", min: 1.0, max: 14.0 }, // All All
        NumericRangeRule { element: "rdn", attribute: "localSheetId", min: f64::NEG_INFINITY, max: 32767.0 }, // All max-only
        NumericRangeRule { element: "reviewed", attribute: "rId", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "rfmt", attribute: "sheetId", min: 0.0, max: 32767.0 }, // All All
        NumericRangeRule { element: "rm", attribute: "sheetId", min: f64::NEG_INFINITY, max: 32767.0 }, // All max-only
        NumericRangeRule { element: "rm", attribute: "sourceSheetId", min: f64::NEG_INFINITY, max: 32767.0 }, // All max-only
        NumericRangeRule { element: "sheetId", attribute: "val", min: f64::NEG_INFINITY, max: 65535.0 }, // All max-only
        NumericRangeRule { element: "undo", attribute: "sId", min: f64::NEG_INFINITY, max: 32767.0 }, // All max-only
        NumericRangeRule { element: "queryTable", attribute: "connectionId", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "queryTableRefresh", attribute: "unboundColumnsLeft", min: f64::NEG_INFINITY, max: 16383.0 }, // All max-only
        NumericRangeRule { element: "queryTableRefresh", attribute: "unboundColumnsRight", min: f64::NEG_INFINITY, max: 16383.0 }, // All max-only
        NumericRangeRule { element: "queryTableRefresh", attribute: "nextId", min: f64::NEG_INFINITY, max: 65535.0 }, // All max-only
        NumericRangeRule { element: "queryTableRefresh", attribute: "minimumVersion", min: 0.0, max: 31.0 }, // All All
        NumericRangeRule { element: "dbPr", attribute: "commandType", min: 1.0, max: 5.0 }, // All All
        NumericRangeRule { element: "olapPr", attribute: "rowDrillCount", min: 1.0, max: 1048576.0 }, // All All
        NumericRangeRule { element: "textField", attribute: "position", min: 0.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "definedName", attribute: "sheetId", min: 0.0, max: 65533.0 }, // All All
        NumericRangeRule { element: "row", attribute: "r", min: f64::NEG_INFINITY, max: 1048576.0 }, // All max-only
        NumericRangeRule { element: "DataBinding", attribute: "DataBindingLoadMode", min: 0.0, max: 4.0 }, // All All
        NumericRangeRule { element: "Map", attribute: "ID", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "cmAuthor", attribute: "id", min: 0.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "lum", attribute: "val", min: 0.0, max: 100000.0 }, // All All
        NumericRangeRule { element: "tc", attribute: "rowSpan", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "tc", attribute: "gridSpan", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "ds", attribute: "d", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "ds", attribute: "sp", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "axId", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "backward", attribute: "val", min: 0.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "crossAx", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "explosion", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "fmtId", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "forward", attribute: "val", min: 0.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "idx", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "order", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "pageMargins", attribute: "b", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "l", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "r", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageMargins", attribute: "t", min: 0.0, max: 48.0 }, // All All
        NumericRangeRule { element: "pageSetup", attribute: "copies", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "pt", attribute: "idx", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "pt", attribute: "idx", min: 0.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "ptCount", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "secondPiePt", attribute: "val", min: f64::NEG_INFINITY, max: 2147483647.0 }, // All max-only
        NumericRangeRule { element: "fill", attribute: "angle", min: -32767.0, max: 32767.0 }, // All All
        NumericRangeRule { element: "fill", attribute: "origin", min: -32767.0, max: 32767.0 }, // All All
        NumericRangeRule { element: "imagedata", attribute: "blacklevel", min: -0.5, max: 0.5 }, // All All
        NumericRangeRule { element: "stroke", attribute: "miterlimit", min: f64::NEG_INFINITY, max: 32767.0 }, // All max-only
        NumericRangeRule { element: "stroke", attribute: "weight", min: 0.0, max: 20116800.0 }, // All All
        NumericRangeRule { element: "bottom", attribute: "weight", min: 0.0, max: 20116800.0 }, // All All
        NumericRangeRule { element: "bottom", attribute: "miterlimit", min: f64::NEG_INFINITY, max: 32767.0 }, // All max-only
        NumericRangeRule { element: "extrusion", attribute: "facet", min: 1.0, max: 65536.0 }, // All All
        NumericRangeRule { element: "extrusion", attribute: "orientationangle", min: -32767.0, max: 32767.0 }, // All All
        NumericRangeRule { element: "extrusion", attribute: "skewangle", min: -32767.0, max: 32767.0 }, // All All
        NumericRangeRule { element: "property", attribute: "pid", min: 2.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "outlineLvl", attribute: "val", min: 0.0, max: 9.0 }, // All All
        NumericRangeRule { element: "customWorkbookView", attribute: "windowHeight", min: f64::NEG_INFINITY, max: 2147483647.0 }, // Excel max-only
        NumericRangeRule { element: "definedName", attribute: "localSheetId", min: f64::NEG_INFINITY, max: 32766.0 }, // Excel max-only
        NumericRangeRule { element: "webPublishObject", attribute: "id", min: 1.0, max: 2147483647.0 }, // Excel All
        NumericRangeRule { element: "brk", attribute: "min", min: f64::NEG_INFINITY, max: 1048576.0 }, // Excel max-only
        NumericRangeRule { element: "brk", attribute: "id", min: 1.0, max: 1048576.0 }, // Excel All
        NumericRangeRule { element: "brk", attribute: "max", min: 1.0, max: 1048576.0 }, // Excel All
        NumericRangeRule { element: "color", attribute: "tint", min: -1.0, max: 1.0 }, // Excel All
        NumericRangeRule { element: "color", attribute: "indexed", min: f64::NEG_INFINITY, max: 255.0 }, // Excel max-only
        NumericRangeRule { element: "color", attribute: "theme", min: 0.0, max: 255.0 }, // Excel All
        NumericRangeRule { element: "dataBar", attribute: "maxLength", min: f64::NEG_INFINITY, max: 100.0 }, // Excel max-only
        NumericRangeRule { element: "dataBar", attribute: "minLength", min: f64::NEG_INFINITY, max: 100.0 }, // Excel max-only
        NumericRangeRule { element: "dataValidations", attribute: "xWindow", min: f64::NEG_INFINITY, max: 65535.0 }, // Excel max-only
        NumericRangeRule { element: "dataValidations", attribute: "yWindow", min: f64::NEG_INFINITY, max: 65535.0 }, // Excel max-only
        NumericRangeRule { element: "dataValidations", attribute: "count", min: f64::NEG_INFINITY, max: 65535.0 }, // Excel max-only
        NumericRangeRule { element: "pageSetup", attribute: "fitToWidth", min: f64::NEG_INFINITY, max: 32767.0 }, // Excel max-only
        NumericRangeRule { element: "pageSetup", attribute: "fitToHeight", min: f64::NEG_INFINITY, max: 32767.0 }, // Excel max-only
        NumericRangeRule { element: "pageSetup", attribute: "copies", min: 1.0, max: 32767.0 }, // Excel All
        NumericRangeRule { element: "pageSetup", attribute: "horizontalDpi", min: 1.0, max: f64::INFINITY }, // Excel min-only
        NumericRangeRule { element: "pageSetup", attribute: "verticalDpi", min: 1.0, max: f64::INFINITY }, // Excel min-only
        NumericRangeRule { element: "pivotSelection", attribute: "previousCol", min: f64::NEG_INFINITY, max: 16383.0 }, // Excel max-only
        NumericRangeRule { element: "pivotSelection", attribute: "previousRow", min: f64::NEG_INFINITY, max: 1048575.0 }, // Excel max-only
        NumericRangeRule { element: "scenario", attribute: "count", min: 1.0, max: 32.0 }, // Excel All
        NumericRangeRule { element: "selection", attribute: "activeCell", min: f64::NEG_INFINITY, max: 8191.0 }, // Excel max-only
        NumericRangeRule { element: "table", attribute: "id", min: 1.0, max: 4294967294.0 }, // Excel All
        NumericRangeRule { element: "table", attribute: "connectionId", min: f64::NEG_INFINITY, max: 2147483647.0 }, // Excel max-only
        NumericRangeRule { element: "xmlColumnPr", attribute: "mapId", min: 1.0, max: 21474836477.0 }, // Excel All
        NumericRangeRule { element: "c", attribute: "i", min: 1.0, max: 65534.0 }, // Excel All
        NumericRangeRule { element: "alignment", attribute: "indent", min: f64::NEG_INFINITY, max: 255.0 }, // Excel max-only
        NumericRangeRule { element: "family", attribute: "val", min: 0.0, max: 5.0 }, // Excel All
        NumericRangeRule { element: "cacheHierarchy", attribute: "iconSet", min: 0.0, max: 11.0 }, // Excel All
        NumericRangeRule { element: "set", attribute: "maxRank", min: 0.0, max: 1048576.0 }, // Excel All
        NumericRangeRule { element: "header", attribute: "maxSheetId", min: f64::NEG_INFINITY, max: 32767.0 }, // Excel max-only
        NumericRangeRule { element: "header", attribute: "minRId", min: f64::NEG_INFINITY, max: 2147483647.0 }, // Excel max-only
        NumericRangeRule { element: "raf", attribute: "sheetId", min: f64::NEG_INFINITY, max: 32767.0 }, // Excel max-only
        NumericRangeRule { element: "rcc", attribute: "sId", min: f64::NEG_INFINITY, max: 32767.0 }, // Excel max-only
        NumericRangeRule { element: "rdn", attribute: "rId", min: f64::NEG_INFINITY, max: 2147483647.0 }, // Excel max-only
        NumericRangeRule { element: "ris", attribute: "sheetId", min: f64::NEG_INFINITY, max: 32767.0 }, // Excel max-only
        NumericRangeRule { element: "ris", attribute: "sheetPosition", min: f64::NEG_INFINITY, max: 65533.0 }, // Excel max-only
        NumericRangeRule { element: "users", attribute: "count", min: f64::NEG_INFINITY, max: 256.0 }, // Excel max-only
        NumericRangeRule { element: "connection", attribute: "interval", min: f64::NEG_INFINITY, max: 32767.0 }, // Excel max-only
        NumericRangeRule { element: "textPr", attribute: "firstRow", min: f64::NEG_INFINITY, max: 2147483647.0 }, // Excel max-only
        NumericRangeRule { element: "values", attribute: "cols", min: 1.0, max: 16384.0 }, // Excel All
        NumericRangeRule { element: "values", attribute: "rows", min: 1.0, max: 1048576.0 }, // Excel All
        NumericRangeRule { element: "tr", attribute: "s", min: 1.0, max: 65534.0 }, // Excel All
        NumericRangeRule { element: "DataBinding", attribute: "ConnectionID", min: f64::NEG_INFINITY, max: 2147483647.0 }, // Excel max-only
        NumericRangeRule { element: "pageSetup", attribute: "paperSize", min: f64::NEG_INFINITY, max: 2147483647.0 }, // Excel max-only
        NumericRangeRule { element: "presentation", attribute: "firstSlideNum", min: 0.0, max: 9999.0 }, // PowerPoint All
        NumericRangeRule { element: "transition", attribute: "advTm", min: 0.0, max: 2147483647.0 }, // PowerPoint All
        NumericRangeRule { element: "animMotion", attribute: "rAng", min: -2147483554.0, max: 2147483554.0 }, // PowerPoint All
        NumericRangeRule { element: "animRot", attribute: "by", min: -2147483554.0, max: 2147483554.0 }, // PowerPoint All
        NumericRangeRule { element: "animRot", attribute: "from", min: -2147483554.0, max: 2147483554.0 }, // PowerPoint All
        NumericRangeRule { element: "animRot", attribute: "to", min: -2147483554.0, max: 2147483554.0 }, // PowerPoint All
        NumericRangeRule { element: "tmPct", attribute: "val", min: f64::NEG_INFINITY, max: 2147483625.0 }, // PowerPoint max-only
        NumericRangeRule { element: "tmpl", attribute: "lvl", min: f64::NEG_INFINITY, max: 9.0 }, // PowerPoint max-only
        NumericRangeRule { element: "docId", attribute: "val", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "p", attribute: "paraId", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "tr", attribute: "paraId", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "p", attribute: "textId", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "tr", attribute: "textId", min: 1.0, max: 2147483647.0 }, // All All
        NumericRangeRule { element: "styleSet", attribute: "id", min: 1.0, max: 20.0 }, // All All
        NumericRangeRule { element: "sparklineGroup", attribute: "lineWeight", min: 0.0, max: 1584.0 }, // All All
        NumericRangeRule { element: "setLevel", attribute: "hierarchy", min: -2.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "cfRule", attribute: "priority", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "conditionalFormat", attribute: "priority", min: 1.0, max: f64::INFINITY }, // All min-only
        NumericRangeRule { element: "protectedRange", attribute: "spinCount", min: f64::NEG_INFINITY, max: 10000000.0 }, // All max-only
        NumericRangeRule { element: "formControlPr", attribute: "dropLines", min: 0.0, max: 30000.0 }, // All All
        NumericRangeRule { element: "formControlPr", attribute: "inc", min: 0.0, max: 30000.0 }, // All All
        NumericRangeRule { element: "formControlPr", attribute: "max", min: 0.0, max: 30000.0 }, // All All
        NumericRangeRule { element: "formControlPr", attribute: "min", min: 0.0, max: 30000.0 }, // All All
        NumericRangeRule { element: "formControlPr", attribute: "page", min: 0.0, max: 30000.0 }, // All All
        NumericRangeRule { element: "slicer", attribute: "columnCount", min: 1.0, max: 20000.0 }, // All All
    ]
}

/// String-length rules from schematrons.json.
pub fn schematron_string_length_rules() -> Vec<StringLengthRule> {
    vec![
        StringLengthRule { element: "fileSharing", attribute: "userName", min: 1, max: 54 }, // All All
        StringLengthRule { element: "fileVersion", attribute: "appName", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "fileVersion", attribute: "lastEdited", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "fileVersion", attribute: "lowestEdited", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "fileVersion", attribute: "rupBuild", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "sheet", attribute: "name", min: 1, max: 31 }, // All All
        StringLengthRule { element: "sheet", attribute: "id", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "conditionalFormatting", attribute: "sqref", min: 1, max: usize::MAX }, // All min-only
        StringLengthRule { element: "dataValidation", attribute: "errorTitle", min: 0, max: 32 }, // All max-only
        StringLengthRule { element: "dataValidation", attribute: "promptTitle", min: 0, max: 32 }, // All max-only
        StringLengthRule { element: "hyperlink", attribute: "location", min: 0, max: 2084 }, // All max-only
        StringLengthRule { element: "hyperlink", attribute: "display", min: 0, max: 2084 }, // All max-only
        StringLengthRule { element: "hyperlink", attribute: "tooltip", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "oleObject", attribute: "progId", min: 0, max: 39 }, // All max-only
        StringLengthRule { element: "protectedRange", attribute: "name", min: 1, max: 255 }, // Word All
        StringLengthRule { element: "sheetPr", attribute: "codeName", min: 0, max: 32 }, // All max-only
        StringLengthRule { element: "webPublishItem", attribute: "title", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "webPublishItem", attribute: "destinationFile", min: 1, max: 255 }, // All All
        StringLengthRule { element: "webPublishItem", attribute: "divId", min: 1, max: 255 }, // All All
        StringLengthRule { element: "tableColumn", attribute: "name", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "tableColumn", attribute: "headerRowCellStyle", min: 1, max: 255 }, // All All
        StringLengthRule { element: "tableColumn", attribute: "totalsRowCellStyle", min: 1, max: 255 }, // All All
        StringLengthRule { element: "tableColumn", attribute: "totalsRowLabel", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "tableColumn", attribute: "dataCellStyle", min: 1, max: 255 }, // All All
        StringLengthRule { element: "xmlColumnPr", attribute: "xpath", min: 0, max: 32000 }, // All max-only
        StringLengthRule { element: "xmlPr", attribute: "xpath", min: 0, max: 32000 }, // All max-only
        StringLengthRule { element: "cellStyle", attribute: "name", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "numFmt", attribute: "formatCode", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "tableStyles", attribute: "defaultTableStyle", min: 1, max: 255 }, // All All
        StringLengthRule { element: "rFont", attribute: "val", min: 0, max: 31 }, // All max-only
        StringLengthRule { element: "futureMetadata", attribute: "name", min: 1, max: 65535 }, // All All
        StringLengthRule { element: "metadataType", attribute: "name", min: 1, max: 65535 }, // All All
        StringLengthRule { element: "calculatedMember", attribute: "name", min: 1, max: 63999 }, // All All
        StringLengthRule { element: "group", attribute: "uniqueParent", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "pivotCacheDefinition", attribute: "refreshedBy", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "pivotHierarchy", attribute: "caption", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "query", attribute: "mdx", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "rangeSet", attribute: "sheet", min: 1, max: 31 }, // All All
        StringLengthRule { element: "serverFormat", attribute: "culture", min: 0, max: 31 }, // All max-only
        StringLengthRule { element: "serverFormat", attribute: "format", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "worksheetSource", attribute: "name", min: 1, max: 255 }, // All All
        StringLengthRule { element: "worksheetSource", attribute: "sheet", min: 0, max: 31 }, // All max-only
        StringLengthRule { element: "rcmt", attribute: "author", min: 1, max: 52 }, // All All
        StringLengthRule { element: "rdn", attribute: "customMenu", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rdn", attribute: "oldCustomMenu", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rdn", attribute: "description", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rdn", attribute: "oldDescription", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rdn", attribute: "help", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rdn", attribute: "oldHelp", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rdn", attribute: "statusBar", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rdn", attribute: "oldStatusBar", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rrc", attribute: "sId", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "rsnm", attribute: "sheetId", min: 0, max: 32767 }, // All max-only
        StringLengthRule { element: "queryTable", attribute: "name", min: 1, max: 255 }, // All All
        StringLengthRule { element: "queryTableField", attribute: "name", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "parameter", attribute: "name", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "parameter", attribute: "string", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "parameter", attribute: "prompt", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "textPr", attribute: "decimal", min: 1, max: 255 }, // All All
        StringLengthRule { element: "textPr", attribute: "thousands", min: 1, max: 255 }, // All All
        StringLengthRule { element: "ddeItem", attribute: "name", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "ddeLink", attribute: "ddeService", min: 1, max: 255 }, // All All
        StringLengthRule { element: "ddeLink", attribute: "ddeTopic", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "oleItem", attribute: "name", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "oleLink", attribute: "progId", min: 1, max: 255 }, // All All
        StringLengthRule { element: "webPr", attribute: "url", min: 1, max: usize::MAX }, // All min-only
        StringLengthRule { element: "sheetName", attribute: "val", min: 0, max: 31 }, // All max-only
        StringLengthRule { element: "Map", attribute: "Name", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "Map", attribute: "RootElement", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "cxnSp", attribute: "macro", min: 0, max: 256 }, // All max-only
        StringLengthRule { element: "graphicFrame", attribute: "macro", min: 0, max: 256 }, // All max-only
        StringLengthRule { element: "sp", attribute: "macro", min: 0, max: 256 }, // All max-only
        StringLengthRule { element: "pic", attribute: "macro", min: 0, max: 256 }, // All max-only
        StringLengthRule { element: "num", attribute: "numId", min: 0, max: 32 }, // Word max-only
        StringLengthRule { element: "pStyle", attribute: "val", min: 0, max: 253 }, // Word max-only
        StringLengthRule { element: "numberingChange", attribute: "original", min: 0, max: 15 }, // All max-only
        StringLengthRule { element: "schema", attribute: "manifestLocation", min: 0, max: 2083 }, // Word max-only
        StringLengthRule { element: "schema", attribute: "schemaLocation", min: 0, max: 2083 }, // Word max-only
        StringLengthRule { element: "schema", attribute: "uri", min: 0, max: 255 }, // Word max-only
        StringLengthRule { element: "oleItem", attribute: "name", min: 1, max: usize::MAX }, // All min-only
        StringLengthRule { element: "definedName", attribute: "comment", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "definedName", attribute: "name", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "functionGroup", attribute: "name", min: 0, max: 32 }, // Excel max-only
        StringLengthRule { element: "webPublishObject", attribute: "title", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "webPublishObject", attribute: "sourceObject", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "webPublishObject", attribute: "destinationFile", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "webPublishObject", attribute: "divId", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "control", attribute: "name", min: 0, max: 32 }, // Excel max-only
        StringLengthRule { element: "inputCells", attribute: "val", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "scenario", attribute: "name", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "scenario", attribute: "user", min: 1, max: 54 }, // Excel All
        StringLengthRule { element: "scenario", attribute: "comment", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "table", attribute: "name", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "table", attribute: "comment", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "table", attribute: "dataCellStyle", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "table", attribute: "headerRowCellStyle", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "table", attribute: "totalsRowCellStyle", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "tableColumn", attribute: "uniqueName", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "tableStyleInfo", attribute: "name", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "xmlCellPr", attribute: "uniqueName", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "name", attribute: "val", min: 1, max: 31 }, // Excel All
        StringLengthRule { element: "tableStyle", attribute: "name", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "b", attribute: "c", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "cacheHierarchy", attribute: "displayFolder", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "cacheHierarchy", attribute: "measureGroup", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "calculatedMember", attribute: "memberName", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "calculatedMember", attribute: "hierarchy", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "calculatedMember", attribute: "parent", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "d", attribute: "c", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "dimension", attribute: "caption", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "dimension", attribute: "name", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "dimension", attribute: "uniqueName", min: 1, max: 32767 }, // Excel All
        StringLengthRule { element: "e", attribute: "c", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "groupMember", attribute: "uniqueName", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "kpi", attribute: "caption", min: 1, max: 32767 }, // Excel All
        StringLengthRule { element: "kpi", attribute: "displayFolder", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "kpi", attribute: "measureGroup", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "kpi", attribute: "parent", min: 0, max: 32767 }, // Excel max-only
        StringLengthRule { element: "m", attribute: "c", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "measureGroup", attribute: "caption", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "measureGroup", attribute: "name", min: 1, max: 65535 }, // Excel All
        StringLengthRule { element: "n", attribute: "c", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "dataCaption", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "grandTotalCaption", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "errorCaption", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "missingCaption", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "pageStyle", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "pivotTableStyle", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "vacatedStyle", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "name", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "tag", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "pivotTableStyleInfo", attribute: "name", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "s", attribute: "c", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "set", attribute: "setDefinition", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "header", attribute: "userName", min: 1, max: 54 }, // Excel All
        StringLengthRule { element: "rdn", attribute: "comment", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "rdn", attribute: "oldComment", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "deletedField", attribute: "name", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "connection", attribute: "name", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "connection", attribute: "description", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "connection", attribute: "singleSignOnId", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "connection", attribute: "odcFile", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "connection", attribute: "sourceFile", min: 0, max: 255 }, // Excel max-only
        StringLengthRule { element: "dbPr", attribute: "connection", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "textPr", attribute: "sourceFile", min: 1, max: 218 }, // Excel All
        StringLengthRule { element: "main", attribute: "first", min: 1, max: 255 }, // Excel All
        StringLengthRule { element: "DataBinding", attribute: "DataBindingName", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "DataBinding", attribute: "FileBindingName", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "Schema", attribute: "ID", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "Schema", attribute: "SchemaRef", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "Schema", attribute: "Namespace", min: 0, max: 65535 }, // Excel max-only
        StringLengthRule { element: "dataValidation", attribute: "error", min: 0, max: 225 }, // All max-only
        StringLengthRule { element: "dataValidation", attribute: "prompt", min: 0, max: 225 }, // All max-only
        StringLengthRule { element: "calculatedMember", attribute: "displayFolder", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "calculatedMember", attribute: "mdxLong", min: 32766, max: 1073741822 }, // All All
        StringLengthRule { element: "header", attribute: "uniqueName", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "header", attribute: "hierarchyName", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "rowItem", attribute: "u", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "rowItem", attribute: "d", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "dataField", attribute: "uniqueName", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "altText", min: 0, max: 2000 }, // All max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "altTextSummary", min: 0, max: 2000 }, // All max-only
        StringLengthRule { element: "pivotTableDefinition", attribute: "weightExpression", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "connection", attribute: "culture", min: 0, max: 84 }, // All max-only
        StringLengthRule { element: "connection", attribute: "embeddedDataId", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "table", attribute: "altText", min: 0, max: 25000 }, // All max-only
        StringLengthRule { element: "table", attribute: "altTextSummary", min: 0, max: 50000 }, // All max-only
        StringLengthRule { element: "pivotChange", attribute: "weightExpression", min: 1, max: 65535 }, // All All
        StringLengthRule { element: "editValue", attribute: "valueType", min: 1, max: 32767 }, // All All
        StringLengthRule { element: "slicerStyle", attribute: "name", min: 1, max: 255 }, // All All
        StringLengthRule { element: "slicerStyles", attribute: "defaultSlicerStyle", min: 1, max: 255 }, // All All
        StringLengthRule { element: "datastoreItem", attribute: "id", min: 0, max: 65535 }, // All max-only
        StringLengthRule { element: "slicer", attribute: "name", min: 1, max: 32767 }, // All All
        StringLengthRule { element: "slicer", attribute: "caption", min: 1, max: usize::MAX }, // All min-only
        StringLengthRule { element: "level", attribute: "uniqueName", min: 1, max: 32767 }, // All All
        StringLengthRule { element: "mcd", attribute: "name", min: 0, max: 255 }, // All max-only
        StringLengthRule { element: "commentEx", attribute: "done", min: 1, max: usize::MAX }, // Word min-only
        StringLengthRule { element: "webextension", attribute: "id", min: 1, max: 1000 }, // Word, Excel All
        StringLengthRule { element: "reference", attribute: "id", min: 1, max: 1000 }, // Word, Excel All
        StringLengthRule { element: "presenceInfo", attribute: "providerId", min: 1, max: 100 }, // Word All
        StringLengthRule { element: "presenceInfo", attribute: "userId", min: 1, max: 300 }, // Word All
        StringLengthRule { element: "themeFamily", attribute: "id", min: 1, max: 100 }, // Excel, PowerPoint All
        StringLengthRule { element: "absPath", attribute: "url", min: 1, max: 1000 }, // Excel All
        StringLengthRule { element: "timeline", attribute: "name", min: 1, max: 1000 }, // Excel All
    ]
}

/// Pattern (matches) rules from schematrons.json (no Unicode property classes).
pub fn schematron_pattern_rules() -> Vec<PatternRule> {
    vec![
        PatternRule { element: "decimalSymbol", attribute: "val", pattern: ".{1}" }, // All
        PatternRule { element: "listSeparator", attribute: "val", pattern: ".{1}" }, // All
        PatternRule { element: "oleObject", attribute: "progId", pattern: "[^\\d].*" }, // All
        PatternRule { element: "rdn", attribute: "name", pattern: "[a-zA-Z_\\\\][a-zA-Z0-9_.]*" }, // All
        PatternRule { element: "fill", attribute: "focus", pattern: "-?(\\d{1,2}|100)%" }, // All
        PatternRule { element: "extrusion", attribute: "edge", pattern: "(\\d{1,5}|1[0-6][0-8]\\d{3}|1690[0-8]\\d|16909[0-3])pt" }, // All
        PatternRule { element: "OLEObject", attribute: "ObjectID", pattern: "_(\\d{1,9}|1\\d{9}|20\\d{8}|21[0-3]\\d{7}|214[0-6]\\d{6}|2147[0-3]\\d{5}|21474[0-7]\\d{4}|214748[0-2]\\d{3}|2147483[0-5]\\d{2}|21474836[0-3]\\d|214748364[0-7])" }, // All
        PatternRule { element: "name", attribute: "val", pattern: "[^,]*" }, // Word
        PatternRule { element: "interpretation", attribute: "id", pattern: "[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}" }, // All
        PatternRule { element: "cameraTool", attribute: "spid", pattern: "_x0000_s(102[5-9]|10[3-9][0-9]|1[1-9][0-9]{2}|[1-9][0-9]{3,7}|1[0-9]{8}|2[0-5][0-9]{7}|26[0-7][0-9]{6}|268[0-3][0-9]{5}|2684[0-2][0-9]{4}|26843[0-4][0-9]{3}|268435[0-3][0-9]{2}|2684354[0-4][0-9]|26843545[0-6])" }, // All
        PatternRule { element: "compatExt", attribute: "spid", pattern: "_x0000_s(102[5-9]|10[3-9][0-9]|1[1-9][0-9]{2}|[1-9][0-9]{3,7}|1[0-9]{8}|2[0-5][0-9]{7}|26[0-7][0-9]{6}|268[0-3][0-9]{5}|2684[0-2][0-9]{4}|26843[0-4][0-9]{3}|268435[0-3][0-9]{2}|2684354[0-4][0-9]|26843545[0-6])" }, // All
        PatternRule { element: "range", attribute: "startItem", pattern: "(0|[1-9][0-9]*000)" }, // All
        PatternRule { element: "sig", attribute: "csb0", pattern: "[0-9a-fA-F]{8}" }, // All
        PatternRule { element: "sig", attribute: "csb1", pattern: "[0-9a-fA-F]{8}" }, // All
        PatternRule { element: "commentEx", attribute: "paraId", pattern: "[0-9a-fA-F]{8}" }, // Word
    ]
}

/// Enumeration rules from schematrons.json (`@attr = a or @attr = b …`).
pub fn schematron_enum_rules() -> Vec<EnumRule> {
    vec![
        EnumRule { element: "arc", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "arc", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "curve", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "curve", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "group", attribute: "tableproperties", values: &["1", "2", "3"] }, // All
        EnumRule { element: "parameter", attribute: "sqlType", values: &["-22", "-20", "-11", "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110", "111", "112", "113"] }, // All
        EnumRule { element: "group", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "group", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "image", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "image", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "line", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "oval", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "oval", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "polyline", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "polyline", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "rect", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "rect", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "roundrect", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "roundrect", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "shape", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "shape", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "shapetype", attribute: "dgmlayout", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "shapetype", attribute: "dgmlayoutmru", values: &["0", "1", "2", "3"] }, // All
        EnumRule { element: "callout", attribute: "type", values: &["rightAngle", "oneSegment", "twoSegment", "threeSegment"] }, // All
        EnumRule { element: "column", attribute: "dashstyle", values: &["solid", "shortdash", "shortdot", "shortdashdot", "shortdashdotdot", "dot", "dash", "longdash", "longdashdotdot", "dashdot"] }, // All
        EnumRule { element: "documentProtection", attribute: "cryptAlgorithmSid", values: &["1", "2", "3", "4", "12", "13", "14"] }, // All
        EnumRule { element: "modifyVerifier", attribute: "cryptAlgorithmSid", values: &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14"] }, // Word, Excel
        EnumRule { element: "alignment", attribute: "readingOrder", values: &["0", "1", "2"] }, // Excel
        EnumRule { element: "modifyVerifier", attribute: "cryptAlgorithmSid", values: &["1", "2", "3", "4", "12", "13", "14"] }, // PowerPoint
        EnumRule { element: "modifyVerifier", attribute: "cryptProviderTypeExtSource", values: &["wincrypt", ""] }, // PowerPoint
        EnumRule { element: "modifyVerifier", attribute: "algIdExtSource", values: &["wincrypt", ""] }, // PowerPoint
        EnumRule { element: "channel", attribute: "units", values: &["dev", "in", "cm", "deg", "rad", "s", "lb", "g"] }, // All
        EnumRule { element: "workbookPr", attribute: "defaultImageDpi", values: &["96", "150", "220"] }, // All
        EnumRule { element: "activeArea", attribute: "units", values: &["dev", "in", "cm", "deg", "rad", "s", "lb", "g"] }, // All
        EnumRule { element: "srcProperty", attribute: "units", values: &["dev", "in", "cm", "deg", "rad", "s", "lb", "g"] }, // All
        EnumRule { element: "channelProperty", attribute: "units", values: &["dev", "in", "cm", "deg", "rad", "s", "lb", "g"] }, // All
        EnumRule { element: "brushProperty", attribute: "units", values: &["dev", "in", "cm", "deg", "rad", "s", "lb", "g"] }, // All
    ]
}

/// Ancestor-scoped uniqueness: within each ancestor, child @attr is unique.
#[derive(Debug, Clone, Copy)]
pub struct AncestorUniqueRule {
    pub ancestor: &'static str,
    pub element: &'static str,
    pub attribute: &'static str,
    pub case_insensitive: bool,
}

/// Conditional attribute: when `flag` equals `flag_value`, `required` must be present.
#[derive(Debug, Clone, Copy)]
pub struct ConditionalAttrRule {
    pub element: &'static str,
    pub required_attribute: &'static str,
    pub flag_attribute: &'static str,
    pub flag_value: &'static str,
}

/// Attribute must not be the nil UUID.
#[derive(Debug, Clone, Copy)]
pub struct NonZeroGuidRule {
    pub element: &'static str,
    pub attribute: &'static str,
}

/// Ancestor-scoped unique-attribute rules.
pub fn schematron_ancestor_unique_rules() -> Vec<AncestorUniqueRule> {
    vec![
        AncestorUniqueRule { ancestor: "cellWatches", element: "cellWatch", attribute: "r", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "protectedRanges", element: "protectedRange", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "worksheet", element: "scenario", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "cellStyles", element: "cellStyle", attribute: "xfId", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "graphicData", element: "cNvPr", attribute: "id", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "pivotCaches", element: "pivotCache", attribute: "cacheId", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "table", element: "tableColumn", attribute: "id", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "table", element: "tableColumn", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "metadata", element: "futureMetadata", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "metadataTypes", element: "metadataType", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "colFields", element: "field", attribute: "x", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "groups", element: "group", attribute: "id", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "pivotField", element: "item", attribute: "n", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "revisions", element: "rdn", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "cmAuthorLst", element: "cmAuthor", attribute: "clrIdx", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "cxnLst", element: "cxn", attribute: "modelId", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "tagLst", element: "tag", attribute: "name", case_insensitive: true }, // PowerPoint
        AncestorUniqueRule { ancestor: "tmplLst", element: "tmpl", attribute: "lvl", case_insensitive: false }, // PowerPoint
        AncestorUniqueRule { ancestor: "bmkLst", element: "bmk", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "bmkLst", element: "bmk", attribute: "time", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "slicerStyles", element: "slicerStyle", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "slicerStyleElements", element: "slicerStyleElement", attribute: "type", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "items", element: "i", attribute: "x", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "definedNames", element: "definedName", attribute: "name", case_insensitive: false }, // All
        AncestorUniqueRule { ancestor: "argumentDescriptions", element: "argumentDescription", attribute: "index", case_insensitive: false }, // All
    ]
}

/// Conditional attribute presence rules.
pub fn schematron_conditional_attr_rules() -> Vec<ConditionalAttrRule> {
    vec![
        ConditionalAttrRule { element: "cfRule", required_attribute: "operator", flag_attribute: "type", flag_value: "cells" }, // All
        ConditionalAttrRule { element: "cfRule", required_attribute: "timePeriod", flag_attribute: "type", flag_value: "timePeriod" }, // All
        ConditionalAttrRule { element: "f", required_attribute: "si", flag_attribute: "t", flag_value: "shared" }, // All
        ConditionalAttrRule { element: "webPublishItem", required_attribute: "sourceRef", flag_attribute: "sourceType", flag_value: "range" }, // All
        ConditionalAttrRule { element: "tableColumn", required_attribute: "totalsRowLabel", flag_attribute: "totalsRowFunction", flag_value: "custom" }, // All
        ConditionalAttrRule { element: "calculatedMember", required_attribute: "hierarchy", flag_attribute: "set", flag_value: "1" }, // All
        ConditionalAttrRule { element: "calculatedMember", required_attribute: "hierarchy", flag_attribute: "set", flag_value: "0" }, // All
        ConditionalAttrRule { element: "calculatedMember", required_attribute: "parent", flag_attribute: "set", flag_value: "1" }, // All
        ConditionalAttrRule { element: "calculatedMember", required_attribute: "memberName", flag_attribute: "set", flag_value: "1" }, // All
        ConditionalAttrRule { element: "calculatedMember", required_attribute: "memberName", flag_attribute: "set", flag_value: "0" }, // All
        ConditionalAttrRule { element: "parameter", required_attribute: "cell", flag_attribute: "parameterType", flag_value: "cell" }, // All
        ConditionalAttrRule { element: "DataBinding", required_attribute: "ConnectionID", flag_attribute: "FileBinding", flag_value: "false" }, // All
        ConditionalAttrRule { element: "DataBinding", required_attribute: "ConnectionID", flag_attribute: "FileBinding", flag_value: "true" }, // All
        ConditionalAttrRule { element: "item", required_attribute: "x", flag_attribute: "t", flag_value: "data" }, // Excel
        ConditionalAttrRule { element: "textPr", required_attribute: "sourceFile", flag_attribute: "prompt", flag_value: "false" }, // Excel
        ConditionalAttrRule { element: "DataBinding", required_attribute: "FileBindingName", flag_attribute: "FileBinding", flag_value: "false" }, // Excel
    ]
}

/// Non-zero GUID rules.
pub fn schematron_nonzero_guid_rules() -> Vec<NonZeroGuidRule> {
    vec![
        NonZeroGuidRule { element: "customSheetView", attribute: "guid" }, // All
        NonZeroGuidRule { element: "headers", attribute: "guid" }, // All
        NonZeroGuidRule { element: "customWorkbookView", attribute: "guid" }, // Excel
    ]
}

/// Same-element attribute comparison: @left OP @right.
#[derive(Debug, Clone, Copy)]
pub struct AttrCompareRule {
    pub element: &'static str,
    pub left: &'static str,
    pub op: &'static str,
    pub right: &'static str,
}

/// Attribute must equal a fixed boolean.
#[derive(Debug, Clone, Copy)]
pub struct FixedBoolRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub expected: bool,
}

/// Cross-part Index-of: context/@attr must exist among target part leaf/@path_attr.
#[derive(Debug, Clone, Copy)]
pub struct CrossPartIndexRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub part_hint: &'static str,
    pub target_element: &'static str,
    pub target_attribute: &'static str,
}

/// Cross-part bound: @attr < count(target_element in part) + offset.
#[derive(Debug, Clone, Copy)]
pub struct CrossPartCountRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub part_hint: &'static str,
    pub target_element: &'static str,
    pub offset: i64,
}

pub fn schematron_attr_compare_rules() -> Vec<AttrCompareRule> {
    vec![
        AttrCompareRule { element: "rPh", left: "sb", op: "<", right: "eb" }, // All
        AttrCompareRule { element: "rangePr", left: "startNum", op: "<", right: "endNum" }, // All
        AttrCompareRule { element: "sharedItems", left: "minValue", op: "<=", right: "maxValue" }, // All
        AttrCompareRule { element: "sldRg", left: "st", op: "<=", right: "end" }, // All
        AttrCompareRule { element: "col", left: "min", op: "<=", right: "max" }, // Excel
        AttrCompareRule { element: "dataBar", left: "minLength", op: "<=", right: "maxLength" }, // All
    ]
}

pub fn schematron_fixed_bool_rules() -> Vec<FixedBoolRule> {
    vec![
        FixedBoolRule { element: "f", attribute: "bx", expected: false }, // All
        FixedBoolRule { element: "colorSeries", attribute: "auto", expected: false }, // All
        FixedBoolRule { element: "colorNegative", attribute: "auto", expected: false }, // All
        FixedBoolRule { element: "colorAxis", attribute: "auto", expected: false }, // All
        FixedBoolRule { element: "colorMarkers", attribute: "auto", expected: false }, // All
        FixedBoolRule { element: "colorFirst", attribute: "auto", expected: false }, // All
        FixedBoolRule { element: "colorLast", attribute: "auto", expected: false }, // All
        FixedBoolRule { element: "colorHigh", attribute: "auto", expected: false }, // All
    ]
}

pub fn schematron_cross_part_index_rules() -> Vec<CrossPartIndexRule> {
    vec![
        CrossPartIndexRule { element: "footnoteReference", attribute: "id", part_hint: "FootnotesPart", target_element: "footnote", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "table", attribute: "connectionId", part_hint: "/WorkbookPart/ConnectionsPart", target_element: "connection", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "xmlColumnPr", attribute: "mapId", part_hint: "CustomXmlMappingsPart", target_element: "Map", target_attribute: "ID" }, // All
        CrossPartIndexRule { element: "c", attribute: "i", part_hint: "/WorkbookPart", target_element: "sheet", target_attribute: "sheetId" }, // All
        CrossPartIndexRule { element: "cacheSource", attribute: "connectionId", part_hint: "/WorkbookPart/ConnectionsPart", target_element: "connection", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "pivotTableDefinition", attribute: "cacheId", part_hint: "/WorkbookPart", target_element: "pivotCache", target_attribute: "cacheId" }, // All
        CrossPartIndexRule { element: "queryTable", attribute: "connectionId", part_hint: "/WorkbookPart/ConnectionsPart", target_element: "connection", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "OLEObject", attribute: "ShapeID", part_hint: ".", target_element: "shape", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "endnote", attribute: "id", part_hint: "/MainDocumentPart/EndnotesPart", target_element: "endnote", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "footnote", attribute: "id", part_hint: "/MainDocumentPart/FootnotesPart", target_element: "footnote", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "commentRangeEnd", attribute: "id", part_hint: "WordprocessingCommentsPart", target_element: "comment", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "commentRangeStart", attribute: "id", part_hint: "WordprocessingCommentsPart", target_element: "comment", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "commentReference", attribute: "id", part_hint: "WordprocessingCommentsPart", target_element: "comment", target_attribute: "id" }, // All
        CrossPartIndexRule { element: "bldDgm", attribute: "grpId", part_hint: ".", target_element: "cTn", target_attribute: "grpId" }, // PowerPoint
        CrossPartIndexRule { element: "bldDgm", attribute: "spid", part_hint: ".", target_element: "cNvPr", target_attribute: "id" }, // PowerPoint
        CrossPartIndexRule { element: "bldGraphic", attribute: "spid", part_hint: ".", target_element: "cNvPr", target_attribute: "id" }, // PowerPoint
        CrossPartIndexRule { element: "bldGraphic", attribute: "grpId", part_hint: ".", target_element: "cTn", target_attribute: "grpId" }, // PowerPoint
        CrossPartIndexRule { element: "bldOleChart", attribute: "spid", part_hint: ".", target_element: "cNvPr", target_attribute: "id" }, // PowerPoint
        CrossPartIndexRule { element: "bldOleChart", attribute: "grpId", part_hint: ".", target_element: "cTn", target_attribute: "grpId" }, // PowerPoint
        CrossPartIndexRule { element: "bldP", attribute: "spid", part_hint: ".", target_element: "cNvPr", target_attribute: "id" }, // PowerPoint
        CrossPartIndexRule { element: "slicerStyle", attribute: "name", part_hint: ".", target_element: "tableStyle", target_attribute: "name" }, // All
        CrossPartIndexRule { element: "conditionalFormat", attribute: "priority", part_hint: "..", target_element: "cfRule", target_attribute: "priority" }, // All
        CrossPartIndexRule { element: "pivotTable", attribute: "tabId", part_hint: "/WorkbookPart", target_element: "sheet", target_attribute: "sheetId" }, // All
    ]
}

pub fn schematron_cross_part_count_rules() -> Vec<CrossPartCountRule> {
    vec![
        CrossPartCountRule { element: "c", attribute: "cm", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "bk", offset: 1 }, // All
        CrossPartCountRule { element: "c", attribute: "vm", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "bk", offset: 1 }, // All
        CrossPartCountRule { element: "c", attribute: "s", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "xf", offset: 0 }, // All
        CrossPartCountRule { element: "table", attribute: "dataDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "table", attribute: "headerRowBorderDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "table", attribute: "headerRowDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "table", attribute: "tableBorderDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "table", attribute: "totalsRowBorderDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "table", attribute: "totalsRowDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "rc", attribute: "t", part_hint: ".", target_element: "metadataType", offset: 1 }, // All
        CrossPartCountRule { element: "cfRule", attribute: "dxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "col", attribute: "style", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "xf", offset: 0 }, // All
        CrossPartCountRule { element: "sheetView", attribute: "workbookViewId", part_hint: "/WorkbookPart", target_element: "workbookView", offset: 0 }, // All
        CrossPartCountRule { element: "colorFilter", attribute: "dxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "phoneticPr", attribute: "fontId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "font", offset: 0 }, // All
        CrossPartCountRule { element: "tableColumn", attribute: "dataDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "tableColumn", attribute: "headerRowDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "tableColumn", attribute: "totalsRowDxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "comment", attribute: "authorId", part_hint: ".", target_element: "author", offset: 0 }, // All
        CrossPartCountRule { element: "cellStyle", attribute: "xfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "xf", offset: 0 }, // All
        CrossPartCountRule { element: "tableStyleElement", attribute: "dxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "xf", attribute: "borderId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "border", offset: 0 }, // All
        CrossPartCountRule { element: "xf", attribute: "fillId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "fill", offset: 0 }, // All
        CrossPartCountRule { element: "xf", attribute: "fontId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "font", offset: 0 }, // All
        CrossPartCountRule { element: "xf", attribute: "xfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "xf", offset: 0 }, // All
        CrossPartCountRule { element: "k", attribute: "n", part_hint: ".", target_element: "s", offset: 0 }, // All
        CrossPartCountRule { element: "k", attribute: "np", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "s", offset: 0 }, // All
        CrossPartCountRule { element: "mdx", attribute: "n", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "s", offset: 0 }, // All
        CrossPartCountRule { element: "ms", attribute: "ns", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "s", offset: 0 }, // All
        CrossPartCountRule { element: "p", attribute: "n", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "s", offset: 0 }, // All
        CrossPartCountRule { element: "p", attribute: "np", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "s", offset: 0 }, // All
        CrossPartCountRule { element: "t", attribute: "si", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "s", offset: 0 }, // All
        CrossPartCountRule { element: "dataField", attribute: "fld", part_hint: ".", target_element: "pivotField", offset: 0 }, // All
        CrossPartCountRule { element: "e", attribute: "in", part_hint: ".", target_element: "serverFormat", offset: 0 }, // All
        CrossPartCountRule { element: "fieldUsage", attribute: "x", part_hint: ".", target_element: "cacheField", offset: 0 }, // All
        CrossPartCountRule { element: "filter", attribute: "fld", part_hint: ".", target_element: "pivotField", offset: 0 }, // All
        CrossPartCountRule { element: "filter", attribute: "iMeasureFld", part_hint: ".", target_element: "pivotField", offset: 0 }, // All
        CrossPartCountRule { element: "filter", attribute: "iMeasureHier", part_hint: ".", target_element: "pivotHierarchy", offset: 0 }, // All
        CrossPartCountRule { element: "format", attribute: "dxfId", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "dxf", offset: 0 }, // All
        CrossPartCountRule { element: "i", attribute: "i", part_hint: ".", target_element: "dataField", offset: 0 }, // All
        CrossPartCountRule { element: "m", attribute: "in", part_hint: ".", target_element: "serverFormat", offset: 0 }, // All
        CrossPartCountRule { element: "mp", attribute: "field", part_hint: "PivotTableCacheDefinitionPart", target_element: "cacheField", offset: 0 }, // All
        CrossPartCountRule { element: "mpMap", attribute: "v", part_hint: ".", target_element: "cacheField", offset: 0 }, // All
        CrossPartCountRule { element: "n", attribute: "in", part_hint: ".", target_element: "serverFormat", offset: 0 }, // All
        CrossPartCountRule { element: "pageField", attribute: "fld", part_hint: "PivotTableCacheDefinitionPart", target_element: "cacheField", offset: 0 }, // All
        CrossPartCountRule { element: "s", attribute: "in", part_hint: ".", target_element: "serverFormat", offset: 0 }, // All
        CrossPartCountRule { element: "tpl", attribute: "fld", part_hint: ".", target_element: "cacheField", offset: 0 }, // All
        CrossPartCountRule { element: "tpl", attribute: "hier", part_hint: ".", target_element: "cacheHierarchy", offset: 0 }, // All
        CrossPartCountRule { element: "oc", attribute: "cm", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "cellMetadata", offset: 0 }, // All
        CrossPartCountRule { element: "oc", attribute: "s", part_hint: "/WorkbookPart/WorkbookStylesPart", target_element: "cellStyle", offset: 0 }, // All
        CrossPartCountRule { element: "oc", attribute: "vm", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "valueMetadata", offset: 0 }, // All
        CrossPartCountRule { element: "cell", attribute: "vm", part_hint: "/WorkbookPart/CellMetadataPart", target_element: "valueMetadata", offset: 0 }, // All
        CrossPartCountRule { element: "slicerStyleElement", attribute: "dxfId", part_hint: ".", target_element: "dxf", offset: 0 }, // All
    ]
}

/// Attribute must equal a fixed literal.
#[derive(Debug, Clone, Copy)]
pub struct FixedValueRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub value: &'static str,
}

/// Attribute must not equal a fixed literal.
#[derive(Debug, Clone, Copy)]
pub struct FixedNeRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub forbidden: &'static str,
}

/// Attribute must not be any of the listed values.
#[derive(Debug, Clone, Copy)]
pub struct MultiNeRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub forbidden: &'static [&'static str],
}

/// Both attributes must be present together.
#[derive(Debug, Clone, Copy)]
pub struct BothPresentRule {
    pub element: &'static str,
    pub left: &'static str,
    pub right: &'static str,
}

/// Attribute must not be NaN / INF / -INF.
#[derive(Debug, Clone, Copy)]
pub struct FiniteNumberRule {
    pub element: &'static str,
    pub attribute: &'static str,
}

pub fn schematron_fixed_value_rules() -> Vec<FixedValueRule> {
    vec![
        FixedValueRule { element: "xmlCellPr", attribute: "id", value: "1" }, // All
        FixedValueRule { element: "arc", attribute: "spt", value: "19" }, // All
        FixedValueRule { element: "colorsDef", attribute: "minVer", value: "12.0" }, // All
        FixedValueRule { element: "curve", attribute: "spt", value: "0" }, // All
        FixedValueRule { element: "image", attribute: "spt", value: "75" }, // All
        FixedValueRule { element: "line", attribute: "spt", value: "20" }, // All
        FixedValueRule { element: "oval", attribute: "spt", value: "3" }, // All
        FixedValueRule { element: "polyline", attribute: "spt", value: "0" }, // All
        FixedValueRule { element: "rect", attribute: "spt", value: "1" }, // All
        FixedValueRule { element: "roundrect", attribute: "spt", value: "2" }, // All
        FixedValueRule { element: "nc", attribute: "cm", value: "0" }, // Excel
        FixedValueRule { element: "nc", attribute: "vm", value: "0" }, // Excel
        FixedValueRule { element: "nc", attribute: "s", value: "0" }, // Excel
        FixedValueRule { element: "interpretation", attribute: "mode", value: "ink" }, // All
        FixedValueRule { element: "one-of", attribute: "disjunction-type", value: "recognition" }, // All
        FixedValueRule { element: "conditionalFormat", attribute: "type", value: "none" }, // All
        FixedValueRule { element: "mcd", attribute: "bEncrypt", value: "0" }, // All
        FixedValueRule { element: "mcd", attribute: "cmg", value: "56" }, // All
    ]
}

pub fn schematron_fixed_ne_rules() -> Vec<FixedNeRule> {
    vec![
        FixedNeRule { element: "pivotSelection", attribute: "axis", forbidden: "axisValues" }, // All
        FixedNeRule { element: "futureMetadata", attribute: "name", forbidden: "XLMDX" }, // All
        FixedNeRule { element: "pivotField", attribute: "axis", forbidden: "axisValues" }, // All
        FixedNeRule { element: "cell", attribute: "t", forbidden: "s" }, // All
        FixedNeRule { element: "divId", attribute: "val", forbidden: "0" }, // Word
        FixedNeRule { element: "ClientData", attribute: "ObjectType", forbidden: "Movie" }, // Excel
        FixedNeRule { element: "cTn", attribute: "spd", forbidden: "0" }, // PowerPoint
    ]
}

pub fn schematron_multi_ne_rules() -> Vec<MultiNeRule> {
    vec![
        MultiNeRule { element: "table", attribute: "id", forbidden: &["0", ""] }, // All
        MultiNeRule { element: "item", attribute: "t", forbidden: &["blank", "grand"] }, // All
        MultiNeRule { element: "sharedItems", attribute: "maxValue", forbidden: &["NaN", "INF", "-INF"] }, // All
        MultiNeRule { element: "sharedItems", attribute: "minValue", forbidden: &["NaN", "INF", "-INF"] }, // All
        MultiNeRule { element: "crossesAt", attribute: "val", forbidden: &["INF", "-INF", "NaN"] }, // All
        MultiNeRule { element: "custUnit", attribute: "val", forbidden: &["INF", "-INF", "NaN"] }, // All
        MultiNeRule { element: "forward", attribute: "val", forbidden: &["INF", "-INF", "NaN"] }, // All
        MultiNeRule { element: "splitPos", attribute: "val", forbidden: &["INF", "-INF", "NaN"] }, // All
        MultiNeRule { element: "fill", attribute: "title", forbidden: &["slashes", "colons"] }, // All
        MultiNeRule { element: "stylePaneFormatFilter", attribute: "val", forbidden: &["0x0040", "0x0080", "0x0800"] }, // Word
        MultiNeRule { element: "n", attribute: "v", forbidden: &["INF", "-INF", "NaN"] }, // Excel
        MultiNeRule { element: "ClientData", attribute: "ObjectType", forbidden: &["LineA", "RectA"] }, // Excel
    ]
}

pub fn schematron_both_present_rules() -> Vec<BothPresentRule> {
    vec![
        BothPresentRule { element: "c", left: "l", right: "s" }, // All
        BothPresentRule { element: "rangeSet", left: "name", right: "ref" }, // All
        BothPresentRule { element: "serverFormat", left: "culture", right: "format" }, // All
        BothPresentRule { element: "undo", left: "dn", right: "r" }, // All
        BothPresentRule { element: "undo", left: "dn", right: "sId" }, // All
        BothPresentRule { element: "tpl", left: "fld", right: "hier" }, // Excel
        BothPresentRule { element: "protectedRange", left: "password", right: "algorithmName" }, // All
        BothPresentRule { element: "control", left: "size", right: "getSize" }, // All
        BothPresentRule { element: "button", left: "size", right: "getSize" }, // All
    ]
}

pub fn schematron_finite_number_rules() -> Vec<FiniteNumberRule> {
    vec![
        FiniteNumberRule { element: "sharedItems", attribute: "maxValue" }, // All
        FiniteNumberRule { element: "sharedItems", attribute: "minValue" }, // All
        FiniteNumberRule { element: "crossesAt", attribute: "val" }, // All
        FiniteNumberRule { element: "custUnit", attribute: "val" }, // All
        FiniteNumberRule { element: "forward", attribute: "val" }, // All
        FiniteNumberRule { element: "splitPos", attribute: "val" }, // All
        FiniteNumberRule { element: "n", attribute: "v" }, // Excel
    ]
}

/// Attribute must be present.
#[derive(Debug, Clone, Copy)]
pub struct RequiredAttrRule {
    pub element: &'static str,
    pub attribute: &'static str,
}

pub fn schematron_required_attr_rules() -> Vec<RequiredAttrRule> {
    vec![
        RequiredAttrRule { element: "hlinkHover", attribute: "id" }, // All
        RequiredAttrRule { element: "hlinkClick", attribute: "id" }, // All
        RequiredAttrRule { element: "hlinkMouseOver", attribute: "id" }, // All
        RequiredAttrRule { element: "bottom", attribute: "type" }, // Word
        RequiredAttrRule { element: "bottom", attribute: "w" }, // Word
    ]
}

/// Attribute must be absent when condition attr is not in allowed values (1.15).
#[derive(Debug, Clone, Copy)]
pub struct AbsentWhenNotRule {
    pub element: &'static str,
    pub absent_attribute: &'static str,
    pub condition_attribute: &'static str,
    pub allowed_values: &'static [&'static str],
}

/// At most one of the listed attributes may be present (1.16).
#[derive(Debug, Clone, Copy)]
pub struct MutualExclusiveRule {
    pub element: &'static str,
    pub attributes: &'static [&'static str],
}

pub fn schematron_absent_when_not_rules() -> Vec<AbsentWhenNotRule> {
    vec![
        AbsentWhenNotRule { element: "webPublishItem", absent_attribute: "sourceRef", condition_attribute: "sourceType", allowed_values: &["range"] }, // All
        AbsentWhenNotRule { element: "cfRule", absent_attribute: "aboveAverage", condition_attribute: "type", allowed_values: &["aboveAverage"] }, // All
        AbsentWhenNotRule { element: "cfRule", absent_attribute: "percent", condition_attribute: "type", allowed_values: &["top10"] }, // All
        AbsentWhenNotRule { element: "cfRule", absent_attribute: "bottom", condition_attribute: "type", allowed_values: &["top10"] }, // All
        AbsentWhenNotRule { element: "cfRule", absent_attribute: "text", condition_attribute: "type", allowed_values: &["beginsWith", "containsText", "endsWith", "notContainsText"] }, // All
        AbsentWhenNotRule { element: "cfRule", absent_attribute: "timePeriod", condition_attribute: "type", allowed_values: &["timePeriod"] }, // All
        AbsentWhenNotRule { element: "cfRule", absent_attribute: "stdDev", condition_attribute: "type", allowed_values: &["aboveAverage"] }, // All
        AbsentWhenNotRule { element: "cfRule", absent_attribute: "equalAverage", condition_attribute: "type", allowed_values: &["aboveAverage"] }, // All
        AbsentWhenNotRule { element: "sortCondition", absent_attribute: "iconSet", condition_attribute: "sortBy", allowed_values: &["icon"] }, // All
        AbsentWhenNotRule { element: "sortCondition", absent_attribute: "iconId", condition_attribute: "sortBy", allowed_values: &["icon"] }, // All
    ]
}

pub fn schematron_mutual_exclusive_rules() -> Vec<MutualExclusiveRule> {
    vec![
        MutualExclusiveRule { element: "tabColor", attributes: &["auto", "indexed", "rgb", "theme"] }, // All
        MutualExclusiveRule { element: "button", attributes: &["insertAfterMso", "insertAfterQ", "insertBeforeMso", "insertBeforeQ"] }, // All
    ]
}

/// When flag==flag_val, other must equal other_val.
#[derive(Debug, Clone, Copy)]
pub struct BoolPairImplRule {
    pub element: &'static str,
    pub other_attribute: &'static str,
    pub other_value: &'static str,
    pub flag_attribute: &'static str,
    pub flag_value: &'static str,
}

/// When required attr present, flag attr must be one of values.
#[derive(Debug, Clone, Copy)]
pub struct AttrAndEnumRule {
    pub element: &'static str,
    pub required_attribute: &'static str,
    pub flag_attribute: &'static str,
    pub flag_values: &'static [&'static str],
}

pub fn schematron_bool_pair_impl_rules() -> Vec<BoolPairImplRule> {
    vec![
        BoolPairImplRule { element: "undo", other_attribute: "ref3D", other_value: "false", flag_attribute: "nf", flag_value: "true" }, // All
        BoolPairImplRule { element: "queryTable", other_attribute: "backgroundRefresh", other_value: "true", flag_attribute: "firstBackgroundRefresh", flag_value: "true" }, // All
        BoolPairImplRule { element: "queryTableField", other_attribute: "dataBound", other_value: "true", flag_attribute: "clipped", flag_value: "true" }, // All
        BoolPairImplRule { element: "queryTableField", other_attribute: "dataBound", other_value: "false", flag_attribute: "fillFormulas", flag_value: "true" }, // All
        BoolPairImplRule { element: "queryTableField", other_attribute: "dataBound", other_value: "true", flag_attribute: "rowNumbers", flag_value: "true" }, // All
        BoolPairImplRule { element: "ddeItem", other_attribute: "name", other_value: "StdDocumentName", flag_attribute: "ole", flag_value: "true" }, // All
        BoolPairImplRule { element: "cacheHierarchy", other_attribute: "flattenHierarchies", other_value: "false", flag_attribute: "ignore", flag_value: "true" }, // Excel
        BoolPairImplRule { element: "cacheHierarchy", other_attribute: "measuresSet", other_value: "false", flag_attribute: "ignore", flag_value: "true" }, // Excel
        BoolPairImplRule { element: "cacheHierarchy", other_attribute: "hierarchizeDistinct", other_value: "false", flag_attribute: "ignore", flag_value: "true" }, // Excel
    ]
}

pub fn schematron_attr_and_enum_rules() -> Vec<AttrAndEnumRule> {
    vec![
        AttrAndEnumRule { element: "sortCondition", required_attribute: "dxfId", flag_attribute: "sortBy", flag_values: &["icon", "value"] }, // All
    ]
}

/// When flag is one of flag_values, other must be one of other_values.
#[derive(Debug, Clone, Copy)]
pub struct EnumWhenFlagRule {
    pub element: &'static str,
    pub other_attribute: &'static str,
    pub other_values: &'static [&'static str],
    pub flag_attribute: &'static str,
    pub flag_values: &'static [&'static str],
}

/// Special matches patterns (excel sheet name / codeName).
#[derive(Debug, Clone, Copy)]
pub struct SpecialPatternRule {
    pub element: &'static str,
    pub attribute: &'static str,
    pub kind: &'static str,
}

pub fn schematron_enum_when_flag_rules() -> Vec<EnumWhenFlagRule> {
    vec![
        EnumWhenFlagRule { element: "conditionalFormat", other_attribute: "type", other_values: &["none", "all"], flag_attribute: "scope", flag_values: &["data", "selection"] }, // All
        EnumWhenFlagRule { element: "compatSetting", other_attribute: "val", other_values: &["11", "12", "14", "15"], flag_attribute: "name", flag_values: &["compatibilityMode"] }, // All
        EnumWhenFlagRule { element: "sparklineGroup", other_attribute: "manualMin", other_values: &["0"], flag_attribute: "minAxisType", flag_values: &["individual", "group"] }, // All
    ]
}

pub fn schematron_special_pattern_rules() -> Vec<SpecialPatternRule> {
    vec![
        SpecialPatternRule { element: "sheet", attribute: "name", kind: "excel_sheet_name" }, // All
        SpecialPatternRule { element: "sheetPr", attribute: "codeName", kind: "excel_codename" }, // All
    ]
}

pub const SCHEMATRON_NUMERIC_RANGE_COUNT: usize = 236;
pub const SCHEMATRON_STRING_LENGTH_COUNT: usize = 184;
pub const SCHEMATRON_PATTERN_COUNT: usize = 15;
pub const SCHEMATRON_ENUM_COUNT: usize = 37;
pub const SCHEMATRON_ANCESTOR_UNIQUE_COUNT: usize = 25;
pub const SCHEMATRON_CONDITIONAL_ATTR_COUNT: usize = 16;
pub const SCHEMATRON_NONZERO_GUID_COUNT: usize = 3;
pub const SCHEMATRON_ATTR_COMPARE_COUNT: usize = 6;
pub const SCHEMATRON_FIXED_BOOL_COUNT: usize = 8;
pub const SCHEMATRON_CROSS_PART_INDEX_COUNT: usize = 23;
pub const SCHEMATRON_CROSS_PART_COUNT_COUNT: usize = 53;
pub const SCHEMATRON_FIXED_VALUE_COUNT: usize = 18;
pub const SCHEMATRON_FIXED_NE_COUNT: usize = 7;
pub const SCHEMATRON_MULTI_NE_COUNT: usize = 12;
pub const SCHEMATRON_BOTH_PRESENT_COUNT: usize = 9;
pub const SCHEMATRON_FINITE_NUMBER_COUNT: usize = 7;
pub const SCHEMATRON_REQUIRED_ATTR_COUNT: usize = 5;
pub const SCHEMATRON_ABSENT_WHEN_NOT_COUNT: usize = 10;
pub const SCHEMATRON_MUTUAL_EXCLUSIVE_COUNT: usize = 2;
pub const SCHEMATRON_BOOL_PAIR_IMPL_COUNT: usize = 9;
pub const SCHEMATRON_ATTR_AND_ENUM_COUNT: usize = 1;
pub const SCHEMATRON_ENUM_WHEN_FLAG_COUNT: usize = 3;
pub const SCHEMATRON_SPECIAL_PATTERN_COUNT: usize = 2;
