//! Semantic validation (relationship-existence + unique-attribute subset).
//!
//! Full C# coverage uses ~948 Schematron rules. This module implements the
//! practical patterns that cover a large share of those rules without an XPath
//! engine:
//!
//! - `document(rels)//Relationship[@Id=current()/@r:id]` (+ optional `@Type`)
//! - `count(distinct-values(//elem/@attr)) = count(//elem/@attr)` uniqueness

use super::ValidationError;
use crate::element::OpenXmlElement;
use crate::opc::{OpcPackage, PackUri};
use std::collections::HashMap;

/// A relationship-existence semantic rule.
#[derive(Debug, Clone)]
pub struct RelationshipExistRule {
    /// Local name of the element that carries the relationship id attribute.
    pub element_local_name: String,
    /// Attribute local name holding the rId (usually `"id"`).
    pub id_attribute: String,
    /// Expected relationship type URI (optional; when set, type must match).
    pub expected_relationship_type: Option<String>,
}

impl RelationshipExistRule {
    pub fn new(
        element_local_name: impl Into<String>,
        id_attribute: impl Into<String>,
        expected_relationship_type: Option<impl Into<String>>,
    ) -> Self {
        Self {
            element_local_name: element_local_name.into(),
            id_attribute: id_attribute.into(),
            expected_relationship_type: expected_relationship_type.map(|s| s.into()),
        }
    }
}

/// A unique-attribute semantic rule: all elements with `element_local_name` must
/// have distinct values for `attribute`.
#[derive(Debug, Clone)]
pub struct UniqueAttributeRule {
    pub element_local_name: String,
    pub attribute: String,
    /// When true, compare values case-insensitively.
    pub case_insensitive: bool,
}

impl UniqueAttributeRule {
    pub fn new(
        element_local_name: impl Into<String>,
        attribute: impl Into<String>,
        case_insensitive: bool,
    ) -> Self {
        Self {
            element_local_name: element_local_name.into(),
            attribute: attribute.into(),
            case_insensitive,
        }
    }
}

/// Built-in relationship-existence rules for common WordprocessingML references.
///
/// Sourced from the C# `schematrons.json` typed `document(rels)` rules that apply
/// to Word documents, plus the common image/hyperlink patterns.
pub fn word_relationship_rules() -> Vec<RelationshipExistRule> {
    use crate::namespace::rel;
    vec![
        RelationshipExistRule::new("hyperlink", "id", Some(rel::HYPERLINK)),
        RelationshipExistRule::new("headerReference", "id", Some(rel::HEADER)),
        RelationshipExistRule::new("footerReference", "id", Some(rel::FOOTER)),
        RelationshipExistRule::new("blip", "embed", Some(rel::IMAGE)),
        RelationshipExistRule::new("imagedata", "id", Some(rel::IMAGE)),
        RelationshipExistRule::new("imagedata", "href", Some(rel::IMAGE)),
        RelationshipExistRule::new("imagedata", "pict", Some(rel::IMAGE)),
        RelationshipExistRule::new("imagedata", "relid", Some(rel::IMAGE)),
        RelationshipExistRule::new("altChunk", "id", Some(rel::AF_CHUNK)),
        RelationshipExistRule::new("control", "id", Some(rel::CONTROL)),
        RelationshipExistRule::new("embedBold", "id", Some(rel::FONT)),
        RelationshipExistRule::new("embedBoldItalic", "id", Some(rel::FONT)),
        RelationshipExistRule::new("embedItalic", "id", Some(rel::FONT)),
        RelationshipExistRule::new("embedRegular", "id", Some(rel::FONT)),
        RelationshipExistRule::new("printerSettings", "id", Some(rel::PRINTER_SETTINGS)),
        RelationshipExistRule::new("fill", "id", Some(rel::IMAGE)),
        RelationshipExistRule::new("stroke", "id", Some(rel::IMAGE)),
        RelationshipExistRule::new("shape", "blip", Some(rel::IMAGE)),
        // Diagram relationship ids (on `dgm:relIds`)
        RelationshipExistRule::new("relIds", "cs", Some(rel::DIAGRAM_COLORS)),
        RelationshipExistRule::new("relIds", "dm", Some(rel::DIAGRAM_DATA)),
        RelationshipExistRule::new("relIds", "lo", Some(rel::DIAGRAM_LAYOUT)),
        RelationshipExistRule::new("relIds", "qs", Some(rel::DIAGRAM_STYLE)),
        // Footnote/endnote refs only require the id attribute to exist as a rel
        // when used as relationship targets; most are internal id refs — skip type.
        RelationshipExistRule::new("footnoteReference", "id", None::<&str>),
        RelationshipExistRule::new("endnoteReference", "id", None::<&str>),
    ]
}

/// SpreadsheetML relationship-existence rules (workbook / worksheet).
pub fn spreadsheet_relationship_rules() -> Vec<RelationshipExistRule> {
    use crate::namespace::rel;
    vec![
        // sheet may point at worksheet / chartsheet / dialogsheet / macrosheet
        RelationshipExistRule::new("sheet", "id", None::<&str>),
        RelationshipExistRule::new("drawing", "id", Some(rel::DRAWING)),
        RelationshipExistRule::new("legacyDrawing", "id", Some(rel::VML_DRAWING)),
        RelationshipExistRule::new("tablePart", "id", Some(rel::TABLE)),
        RelationshipExistRule::new("pivotTable", "id", None::<&str>),
        RelationshipExistRule::new("hyperlink", "id", Some(rel::HYPERLINK)),
        RelationshipExistRule::new("externalReference", "id", Some(rel::EXTERNAL_LINK)),
        RelationshipExistRule::new("chartReference", "id", Some(rel::CHART)),
        RelationshipExistRule::new("blip", "embed", Some(rel::IMAGE)),
        RelationshipExistRule::new("printerSettings", "id", Some(rel::PRINTER_SETTINGS)),
    ]
}

/// PresentationML relationship-existence rules.
pub fn presentation_relationship_rules() -> Vec<RelationshipExistRule> {
    use crate::namespace::rel;
    vec![
        RelationshipExistRule::new("sldId", "id", Some(rel::SLIDE)),
        RelationshipExistRule::new("sldMasterId", "id", Some(rel::SLIDE_MASTER)),
        RelationshipExistRule::new("sldLayoutId", "id", Some(rel::SLIDE_LAYOUT)),
        RelationshipExistRule::new("notesMasterId", "id", Some(rel::NOTES_MASTER)),
        RelationshipExistRule::new("handoutMasterId", "id", Some(rel::HANDOUT_MASTER)),
        RelationshipExistRule::new("blip", "embed", Some(rel::IMAGE)),
        RelationshipExistRule::new("audioFile", "link", None::<&str>),
        RelationshipExistRule::new("videoFile", "link", None::<&str>),
    ]
}

/// Common unique-attribute rules for Word documents.
pub fn word_unique_attribute_rules() -> Vec<UniqueAttributeRule> {
    vec![
        UniqueAttributeRule::new("comment", "id", false),
        UniqueAttributeRule::new("commentRangeStart", "id", false),
        UniqueAttributeRule::new("commentRangeEnd", "id", false),
        UniqueAttributeRule::new("commentReference", "id", false),
        UniqueAttributeRule::new("bookmarkStart", "id", false),
        UniqueAttributeRule::new("bookmarkEnd", "id", false),
        UniqueAttributeRule::new("footnote", "id", false),
        UniqueAttributeRule::new("endnote", "id", false),
        UniqueAttributeRule::new("abstractNum", "abstractNumId", false),
        UniqueAttributeRule::new("style", "styleId", false),
        UniqueAttributeRule::new("ins", "id", false),
        UniqueAttributeRule::new("del", "id", false),
        UniqueAttributeRule::new("docPr", "id", false),
    ]
}

/// Common unique-attribute rules for Excel workbooks.
pub fn spreadsheet_unique_attribute_rules() -> Vec<UniqueAttributeRule> {
    vec![
        UniqueAttributeRule::new("sheet", "name", true),
        UniqueAttributeRule::new("sheet", "sheetId", false),
        UniqueAttributeRule::new("connection", "name", false),
        UniqueAttributeRule::new("connection", "id", false),
        UniqueAttributeRule::new("tableColumn", "id", false),
        UniqueAttributeRule::new("tableColumn", "name", false),
        UniqueAttributeRule::new("definedName", "name", true),
        UniqueAttributeRule::new("pivotCache", "cacheId", false),
        UniqueAttributeRule::new("cNvPr", "id", false),
    ]
}

/// Common unique-attribute rules for PowerPoint presentations.
pub fn presentation_unique_attribute_rules() -> Vec<UniqueAttributeRule> {
    vec![
        UniqueAttributeRule::new("sldId", "id", false),
        UniqueAttributeRule::new("sldMasterId", "id", true),
        UniqueAttributeRule::new("sldLayoutId", "id", false),
        UniqueAttributeRule::new("cmAuthor", "id", true),
        UniqueAttributeRule::new("tag", "name", true),
        UniqueAttributeRule::new("cNvPr", "id", false),
    ]
}

/// Validate that relationship-id attributes on `root` resolve against `package`
/// relationships for `part_uri`.
pub fn validate_relationship_refs(
    package: &OpcPackage,
    part_uri: &PackUri,
    root: &OpenXmlElement,
    rules: &[RelationshipExistRule],
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let rels = package.part_relationships(&part_uri);
    let pkg_rels = package.package_relationships();

    for el in root.descendants() {
        for rule in rules {
            if el.local_name != rule.element_local_name {
                continue;
            }
            let id = el
                .get_attribute(&rule.id_attribute)
                .or_else(|| el.get_attribute_qname(&format!("r:{}", rule.id_attribute)))
                .or_else(|| {
                    el.attributes
                        .iter()
                        .find(|a| a.local_name == rule.id_attribute)
                        .map(|a| a.value.as_str())
                });
            let Some(rid) = id else {
                continue;
            };
            // Skip empty / non-rId values (internal footnote ids, etc.)
            if rid.is_empty() || (!rid.starts_with('r') && rule.expected_relationship_type.is_some())
            {
                // Still check existence if it looks like an rId
                if !rid.starts_with('r') {
                    continue;
                }
            }
            let found = rels
                .and_then(|r| r.get(rid))
                .or_else(|| pkg_rels.get(rid));
            match found {
                None => {
                    // Only report missing when expected type is set, or id looks like rId*
                    if rule.expected_relationship_type.is_some() || rid.starts_with("rId") {
                        errors.push(ValidationError {
                            path: format!(
                                "{}/{}@{}",
                                part_uri.as_str(),
                                el.local_name,
                                rule.id_attribute
                            ),
                            message: format!(
                                "relationship id `{rid}` on `<{}>` does not exist",
                                el.local_name
                            ),
                        });
                    }
                }
                Some(rel) => {
                    if let Some(expected) = &rule.expected_relationship_type {
                        if !relationship_type_matches(&rel.relationship_type, expected) {
                            errors.push(ValidationError {
                                path: format!(
                                    "{}/{}@{}",
                                    part_uri.as_str(),
                                    el.local_name,
                                    rule.id_attribute
                                ),
                                message: format!(
                                    "relationship `{rid}` has type `{}`, expected `{}`",
                                    rel.relationship_type, expected
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
    errors
}

fn relationship_type_matches(actual: &str, expected: &str) -> bool {
    if actual == expected {
        return true;
    }
    let exp_suffix = expected.rsplit('/').next().unwrap_or(expected);
    actual.ends_with(exp_suffix) || actual.contains(exp_suffix)
}

/// Validate that attribute values are unique for the given rules.
pub fn validate_unique_attributes(
    root: &OpenXmlElement,
    rules: &[UniqueAttributeRule],
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    // For each rule, collect values of matching elements
    for rule in rules {
        let mut seen: HashMap<String, usize> = HashMap::new();
        for el in root.descendants() {
            if el.local_name != rule.element_local_name {
                continue;
            }
            let val = el
                .get_attribute(&rule.attribute)
                .or_else(|| el.get_attribute_qname(&format!("w:{}", rule.attribute)))
                .or_else(|| el.get_attribute_qname(&format!("x:{}", rule.attribute)))
                .or_else(|| el.get_attribute_qname(&format!("p:{}", rule.attribute)))
                .or_else(|| {
                    el.attributes
                        .iter()
                        .find(|a| a.local_name == rule.attribute)
                        .map(|a| a.value.as_str())
                });
            let Some(raw) = val else { continue };
            let key = if rule.case_insensitive {
                raw.to_ascii_lowercase()
            } else {
                raw.to_string()
            };
            let count = seen.entry(key.clone()).or_insert(0);
            *count += 1;
            if *count == 2 {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element_local_name, rule.attribute),
                    message: format!(
                        "duplicate {} `@{}` value `{}`",
                        rule.element_local_name, rule.attribute, raw
                    ),
                });
            }
        }
    }
    errors
}

/// Run relationship + uniqueness semantic checks for a part root.
pub fn validate_semantic(
    package: &OpcPackage,
    part_uri: &PackUri,
    root: &OpenXmlElement,
    rel_rules: &[RelationshipExistRule],
    unique_rules: &[UniqueAttributeRule],
) -> Vec<ValidationError> {
    let mut errors = validate_relationship_refs(package, part_uri, root, rel_rules);
    errors.extend(validate_unique_attributes(root, unique_rules));
    errors
}

/// Validate using the full extractable Schematron subset:
/// - relationship existence + unique attributes
/// - numeric ranges, string lengths, patterns, enums, conditionals, …
/// - cross-part Index-of / count bounds when referenced parts are present
///
/// Remaining source rules (complex boolean logic, FLWOR, …)
/// still require a general XPath engine.
pub fn validate_schematron_subset(
    package: &OpcPackage,
    part_uri: &PackUri,
    root: &OpenXmlElement,
) -> Vec<ValidationError> {
    let rel_rules = crate::validation::schematron_relationship_rules();
    let unique_rules = crate::validation::schematron_unique_attribute_rules();
    let mut errors = validate_semantic(package, part_uri, root, &rel_rules, &unique_rules);
    errors.extend(crate::validation::validate_schematron_constraints(root));
    errors.extend(crate::validation::validate_schematron_cross_part(package, root));
    errors
}

/// Validate Schematron attribute constraints only (no relationship package lookup).
pub fn validate_schematron_attributes(root: &OpenXmlElement) -> Vec<ValidationError> {
    crate::validation::validate_schematron_constraints(root)
}

/// Merge hand-curated and Schematron-extracted rules (dedupe by element/attr/type).
pub fn merged_relationship_rules(
    base: Vec<RelationshipExistRule>,
) -> Vec<RelationshipExistRule> {
    let mut out = base;
    let mut seen: std::collections::HashSet<(String, String, Option<String>)> =
        out.iter()
            .map(|r| {
                (
                    r.element_local_name.clone(),
                    r.id_attribute.clone(),
                    r.expected_relationship_type.clone(),
                )
            })
            .collect();
    for r in crate::validation::schematron_relationship_rules() {
        let key = (
            r.element_local_name.clone(),
            r.id_attribute.clone(),
            r.expected_relationship_type.clone(),
        );
        if seen.insert(key) {
            out.push(r);
        }
    }
    out
}

/// Merge hand-curated and Schematron-extracted unique-attribute rules.
pub fn merged_unique_attribute_rules(
    base: Vec<UniqueAttributeRule>,
) -> Vec<UniqueAttributeRule> {
    let mut out = base;
    let mut seen: std::collections::HashSet<(String, String, bool)> = out
        .iter()
        .map(|r| {
            (
                r.element_local_name.clone(),
                r.attribute.clone(),
                r.case_insensitive,
            )
        })
        .collect();
    for r in crate::validation::schematron_unique_attribute_rules() {
        let key = (
            r.element_local_name.clone(),
            r.attribute.clone(),
            r.case_insensitive,
        );
        if seen.insert(key) {
            out.push(r);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::{content_type, rel};
    use crate::opc::RelationshipTargetMode;
    use crate::wordprocessing::{body, document, paragraph, run, text};

    #[test]
    fn missing_hyperlink_rel_detected() {
        let mut pkg = OpcPackage::create();
        let uri = PackUri::new("/word/document.xml");
        let mut hl = crate::element::OpenXmlElement::w("hyperlink");
        hl.set_attribute_ns(
            "r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            "id",
            "rId999",
        );
        hl.append_child(run(vec![text("x")]));
        let doc = document(vec![body(vec![paragraph(vec![hl])])]);
        let xml = crate::element::write_element(&doc).unwrap();
        pkg.set_part(uri.clone(), content_type::WORD_DOCUMENT, xml);
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let root = crate::element::parse_element(pkg.get_part(&uri).unwrap()).unwrap();
        let errs = validate_relationship_refs(&pkg, &uri, &root, &word_relationship_rules());
        assert!(
            errs.iter().any(|e| e.message.contains("rId999")),
            "{errs:?}"
        );
    }

    #[test]
    fn unique_style_ids_detected() {
        let styles = crate::element::OpenXmlElement::w("styles")
            .with_child(
                crate::element::OpenXmlElement::w("style")
                    .with_attribute("styleId", "Normal")
                    .with_attribute("type", "paragraph"),
            )
            .with_child(
                crate::element::OpenXmlElement::w("style")
                    .with_attribute("styleId", "Normal")
                    .with_attribute("type", "paragraph"),
            );
        let errs = validate_unique_attributes(&styles, &word_unique_attribute_rules());
        assert!(
            errs.iter().any(|e| e.message.contains("duplicate")),
            "{errs:?}"
        );
    }

    #[test]
    fn sheet_name_case_insensitive_unique() {
        let wb = crate::element::OpenXmlElement::new(
            "x",
            crate::namespace::ns::SPREADSHEETML.uri,
            "workbook",
        )
        .with_child(
            crate::element::OpenXmlElement::new(
                "x",
                crate::namespace::ns::SPREADSHEETML.uri,
                "sheets",
            )
            .with_child(
                crate::element::OpenXmlElement::new(
                    "x",
                    crate::namespace::ns::SPREADSHEETML.uri,
                    "sheet",
                )
                .with_attribute("name", "Sheet1")
                .with_attribute("sheetId", "1")
                .with_attribute("id", "rId1"),
            )
            .with_child(
                crate::element::OpenXmlElement::new(
                    "x",
                    crate::namespace::ns::SPREADSHEETML.uri,
                    "sheet",
                )
                .with_attribute("name", "sheet1")
                .with_attribute("sheetId", "2")
                .with_attribute("id", "rId2"),
            ),
        );
        let errs = validate_unique_attributes(&wb, &spreadsheet_unique_attribute_rules());
        assert!(
            errs.iter().any(|e| e.message.contains("duplicate")),
            "{errs:?}"
        );
    }
}
