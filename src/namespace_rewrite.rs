//! Strict ↔ Transitional namespace and relationship URI rewrite.
//!
//! ISO/IEC 29500 defines both Strict (`http://purl.oclc.org/ooxml/...`) and
//! Transitional (`http://schemas.openxmlformats.org/...`) namespaces. Office
//! and this SDK treat them as equivalent; on load we typically normalize Strict
//! URIs to Transitional so the rest of the DOM/part graph can use a single set
//! of constants.
//!
//! Mappings mirror `OpenXmlNamespaceResolver` in the C# Open XML SDK.

use crate::element::{parse_element, write_element, OpenXmlElement};
use crate::error::Result;
use crate::opc::OpcPackage;

/// Strict → Transitional namespace URI pairs (core set used by Word/Excel/PPT).
pub static STRICT_TO_TRANSITIONAL_NAMESPACES: &[(&str, &str)] = &[
    (
        "http://purl.oclc.org/ooxml/descriptions/base",
        "http://descriptions.openxmlformats.org/description/base",
    ),
    (
        "http://purl.oclc.org/ooxml/descriptions/full",
        "http://descriptions.openxmlformats.org/description/full",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/chart",
        "http://schemas.openxmlformats.org/drawingml/2006/chart",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/chartDrawing",
        "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/diagram",
        "http://schemas.openxmlformats.org/drawingml/2006/diagram",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/main",
        "http://schemas.openxmlformats.org/drawingml/2006/main",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/picture",
        "http://schemas.openxmlformats.org/drawingml/2006/picture",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/spreadsheetDrawing",
        "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/wordprocessingDrawing",
        "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/lockedCanvas",
        "http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas",
    ),
    (
        "http://purl.oclc.org/ooxml/drawingml/compatibility",
        "http://schemas.openxmlformats.org/drawingml/2006/compatibility",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/bibliography",
        "http://schemas.openxmlformats.org/officeDocument/2006/bibliography",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/customProperties",
        "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/customXml",
        "http://schemas.openxmlformats.org/officeDocument/2006/customXml",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/customXmlDataProps",
        "http://schemas.openxmlformats.org/officeDocument/2006/customXmlDataProps",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/docPropsVTypes",
        "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/extendedProperties",
        "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/math",
        "http://schemas.openxmlformats.org/officeDocument/2006/math",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/sharedTypes",
        "http://schemas.openxmlformats.org/officeDocument/2006/sharedTypes",
    ),
    (
        "http://purl.oclc.org/ooxml/presentationml/main",
        "http://schemas.openxmlformats.org/presentationml/2006/main",
    ),
    (
        "http://purl.oclc.org/ooxml/schemaLibrary/main",
        "http://schemas.openxmlformats.org/schemaLibrary/2006/main",
    ),
    (
        "http://purl.oclc.org/ooxml/spreadsheetml/main",
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    ),
    (
        "http://purl.oclc.org/ooxml/wordprocessingml/main",
        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
    ),
    // ISO spec workaround
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/customXml",
        "http://schemas.openxmlformats.org/officeDocument/2006/customXml",
    ),
];

/// Strict → Transitional relationship type URI pairs (commonly used subset).
pub static STRICT_TO_TRANSITIONAL_RELATIONSHIPS: &[(&str, &str)] = &[
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/styles",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/settings",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/webSettings",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/fontTable",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/theme",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/numbering",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/header",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/footer",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/comments",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/image",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/hyperlink",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/aFChunk",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/slide",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/extendedProperties",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/customProperties",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
    ),
    (
        "http://purl.oclc.org/ooxml/officeDocument/relationships/metadata/thumbnail",
        "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    ),
];

/// Look up the Transitional equivalent of a Strict namespace URI.
pub fn to_transitional_namespace(uri: &str) -> Option<&'static str> {
    STRICT_TO_TRANSITIONAL_NAMESPACES
        .iter()
        .find(|(s, _)| *s == uri)
        .map(|(_, t)| *t)
}

/// Look up the Strict equivalent of a Transitional namespace URI.
pub fn to_strict_namespace(uri: &str) -> Option<&'static str> {
    STRICT_TO_TRANSITIONAL_NAMESPACES
        .iter()
        .find(|(_, t)| *t == uri)
        .map(|(s, _)| *s)
}

/// Look up the Transitional equivalent of a Strict relationship type URI.
pub fn to_transitional_relationship(uri: &str) -> Option<&'static str> {
    STRICT_TO_TRANSITIONAL_RELATIONSHIPS
        .iter()
        .find(|(s, _)| *s == uri)
        .map(|(_, t)| *t)
}

/// Look up the Strict equivalent of a Transitional relationship type URI.
pub fn to_strict_relationship(uri: &str) -> Option<&'static str> {
    STRICT_TO_TRANSITIONAL_RELATIONSHIPS
        .iter()
        .find(|(_, t)| *t == uri)
        .map(|(s, _)| *s)
}

/// Returns true if `uri` is a known Strict OOXML namespace.
pub fn is_strict_namespace(uri: &str) -> bool {
    uri.starts_with("http://purl.oclc.org/ooxml/")
}

/// Rewrite a single URI from Strict → Transitional (namespace or relationship).
/// Returns the original if no mapping exists.
pub fn normalize_uri(uri: &str) -> &str {
    to_transitional_namespace(uri)
        .or_else(|| to_transitional_relationship(uri))
        .unwrap_or(uri)
}

/// Recursively rewrite namespace URIs on an element tree from Strict → Transitional.
///
/// Updates `namespace_uri`, `namespace_declarations`, and namespaced attribute URIs.
/// Returns the number of replacements performed.
pub fn rewrite_element_to_transitional(elem: &mut OpenXmlElement) -> usize {
    let mut count = 0;

    if let Some(t) = to_transitional_namespace(&elem.namespace_uri) {
        if elem.namespace_uri != t {
            elem.namespace_uri = t.to_string();
            count += 1;
        }
    }

    for (prefix, uri) in &mut elem.namespace_declarations {
        let _ = prefix;
        if let Some(t) = to_transitional_namespace(uri) {
            if uri.as_str() != t {
                *uri = t.to_string();
                count += 1;
            }
        }
    }

    for attr in &mut elem.attributes {
        if let Some(ref mut ns) = attr.namespace_uri {
            if let Some(t) = to_transitional_namespace(ns) {
                if ns.as_str() != t {
                    *ns = t.to_string();
                    count += 1;
                }
            }
        }
        // Relationship ids in r:id values are not URIs; relationship *types*
        // live in .rels files, handled separately.
    }

    for child in &mut elem.children {
        count += rewrite_element_to_transitional(child);
    }
    count
}

/// Rewrite relationship type strings in-place (Strict → Transitional).
pub fn rewrite_relationship_type(relationship_type: &str) -> String {
    to_transitional_relationship(relationship_type)
        .unwrap_or(relationship_type)
        .to_string()
}

/// Normalize an entire OPC package: rewrite Strict namespaces in XML parts and
/// Strict relationship types in all relationship collections.
///
/// Returns `(xml_replacements, relationship_replacements)`.
pub fn rewrite_package_to_transitional(package: &mut OpcPackage) -> Result<(usize, usize)> {
    let mut xml_count = 0usize;
    let mut rel_count = 0usize;

    // Package-level relationships
    {
        let rels = package.package_relationships_mut();
        // Relationships is not directly mutable for type field via public API —
        // rebuild by collecting and re-adding is heavy; instead rewrite via
        // internal access pattern: parse/rebuild XML.
        let xml = rels.to_xml()?;
        let rewritten = rewrite_rels_xml(&xml, &mut rel_count)?;
        if rel_count > 0 {
            *rels = crate::opc::Relationships::parse(&rewritten)?;
        }
    }

    // Collect part URIs first (avoid borrow issues)
    let part_uris: Vec<_> = package.part_uris();

    for uri in part_uris {
        // Rewrite part relationships
        if let Some(rels) = package.part_relationships(&uri) {
            if !rels.is_empty() {
                let xml = rels.to_xml()?;
                let mut local = 0usize;
                let rewritten = rewrite_rels_xml(&xml, &mut local)?;
                if local > 0 {
                    *package.part_relationships_mut(&uri) =
                        crate::opc::Relationships::parse(&rewritten)?;
                    rel_count += local;
                }
            }
        }

        // Rewrite XML part content
        let Some(data) = package.get_part(&uri) else {
            continue;
        };
        // Only try to parse as XML
        let trimmed: Vec<u8> = data
            .iter()
            .skip_while(|b| b.is_ascii_whitespace())
            .copied()
            .take(5)
            .collect();
        if !(trimmed.starts_with(b"<?xml") || trimmed.starts_with(b"<")) {
            continue;
        }
        // Skip if no strict URI present (fast path)
        if !data.windows(b"purl.oclc.org/ooxml".len()).any(|w| w == b"purl.oclc.org/ooxml") {
            continue;
        }
        let data = data.to_vec();
        let mut root = match parse_element(&data) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let n = rewrite_element_to_transitional(&mut root);
        if n > 0 {
            let ct = package
                .content_types()
                .content_type_for(uri.as_str())
                .unwrap_or("application/xml")
                .to_string();
            let xml = write_element(&root)?;
            package.set_part(uri, ct, xml);
            xml_count += n;
        }
    }

    Ok((xml_count, rel_count))
}

fn rewrite_rels_xml(xml: &[u8], count: &mut usize) -> Result<Vec<u8>> {
    let rels = crate::opc::Relationships::parse(xml)?;
    // Rebuild with rewritten types
    let mut new_rels = crate::opc::Relationships::new();
    for r in rels.iter() {
        let new_type = if let Some(t) = to_transitional_relationship(&r.relationship_type) {
            if t != r.relationship_type {
                *count += 1;
            }
            t.to_string()
        } else {
            r.relationship_type.clone()
        };
        new_rels.add_with_id(
            r.id.clone(),
            new_type,
            r.target.clone(),
            r.target_mode,
        );
    }
    // If nothing changed and we didn't need rebuild, still ok
    let _ = rels;
    new_rels.to_xml()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;
    use crate::namespace::{content_type, rel};
    use crate::opc::{PackUri, RelationshipTargetMode};

    #[test]
    fn namespace_lookup() {
        assert_eq!(
            to_transitional_namespace("http://purl.oclc.org/ooxml/wordprocessingml/main"),
            Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
        );
        assert_eq!(
            to_strict_namespace("http://schemas.openxmlformats.org/wordprocessingml/2006/main"),
            Some("http://purl.oclc.org/ooxml/wordprocessingml/main")
        );
        assert!(is_strict_namespace(
            "http://purl.oclc.org/ooxml/spreadsheetml/main"
        ));
        assert!(!is_strict_namespace(
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
        ));
    }

    #[test]
    fn element_rewrite() {
        let mut el = OpenXmlElement::new(
            "w",
            "http://purl.oclc.org/ooxml/wordprocessingml/main",
            "document",
        )
        .with_ns_decl("w", "http://purl.oclc.org/ooxml/wordprocessingml/main")
        .with_child(OpenXmlElement::new(
            "w",
            "http://purl.oclc.org/ooxml/wordprocessingml/main",
            "body",
        ));
        let n = rewrite_element_to_transitional(&mut el);
        assert!(n >= 2);
        assert_eq!(
            el.namespace_uri,
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        );
        assert_eq!(
            el.children[0].namespace_uri,
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        );
    }

    #[test]
    fn package_rewrite() {
        let mut pkg = OpcPackage::create();
        pkg.set_part(
            "/word/document.xml",
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?>
            <w:document xmlns:w="http://purl.oclc.org/ooxml/wordprocessingml/main">
              <w:body><w:p><w:r><w:t>Hi</w:t></w:r></w:p></w:body>
            </w:document>"#
                .to_vec(),
        );
        // Use strict relationship type
        pkg.package_relationships_mut().add_with_id(
            "rId1",
            "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument",
            "word/document.xml",
            RelationshipTargetMode::Internal,
        );

        let (xml_n, rel_n) = rewrite_package_to_transitional(&mut pkg).unwrap();
        assert!(xml_n > 0);
        assert_eq!(rel_n, 1);

        let rel = pkg
            .package_relationships()
            .get("rId1")
            .unwrap();
        assert_eq!(rel.relationship_type, rel::OFFICE_DOCUMENT);

        let doc = pkg
            .get_part_str(&PackUri::new("/word/document.xml"))
            .unwrap()
            .unwrap();
        assert!(doc.contains("schemas.openxmlformats.org/wordprocessingml/2006/main"));
        assert!(!doc.contains("purl.oclc.org/ooxml/wordprocessingml"));
    }
}
