//! Structural comparison of OPC packages (shell for package-level equality checks).

use super::OpcPackage;
use crate::element::{elements_equal_with, EqualityOptions};
use crate::element::parse_element;
use std::collections::BTreeSet;

/// Options for comparing two packages.
#[derive(Debug, Clone, Copy)]
pub struct PackageEqualityOptions {
    pub compare_content_types: bool,
    pub compare_relationships: bool,
    pub compare_part_bytes: bool,
    /// When part content is XML, compare as element trees instead of raw bytes.
    pub compare_xml_as_dom: bool,
    pub xml_options: EqualityOptions,
}

impl Default for PackageEqualityOptions {
    fn default() -> Self {
        Self {
            compare_content_types: true,
            compare_relationships: true,
            compare_part_bytes: true,
            compare_xml_as_dom: true,
            xml_options: EqualityOptions::default(),
        }
    }
}

/// Result of a package comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDiff {
    pub only_in_left: Vec<String>,
    pub only_in_right: Vec<String>,
    pub content_mismatch: Vec<String>,
    pub relationship_mismatch: Vec<String>,
}

impl PackageDiff {
    pub fn is_empty(&self) -> bool {
        self.only_in_left.is_empty()
            && self.only_in_right.is_empty()
            && self.content_mismatch.is_empty()
            && self.relationship_mismatch.is_empty()
    }
}

/// Compare two packages; returns a structured diff (empty if equal).
pub fn compare_packages(
    left: &OpcPackage,
    right: &OpcPackage,
    opts: &PackageEqualityOptions,
) -> PackageDiff {
    let mut diff = PackageDiff {
        only_in_left: Vec::new(),
        only_in_right: Vec::new(),
        content_mismatch: Vec::new(),
        relationship_mismatch: Vec::new(),
    };

    let left_uris: BTreeSet<_> = left.part_uris().into_iter().map(|u| u.to_string()).collect();
    let right_uris: BTreeSet<_> = right.part_uris().into_iter().map(|u| u.to_string()).collect();

    for u in left_uris.difference(&right_uris) {
        diff.only_in_left.push(u.clone());
    }
    for u in right_uris.difference(&left_uris) {
        diff.only_in_right.push(u.clone());
    }

    if opts.compare_part_bytes {
        for u in left_uris.intersection(&right_uris) {
            let uri = super::PackUri::new(u.clone());
            let lb = left.get_part(&uri).unwrap_or_default();
            let rb = right.get_part(&uri).unwrap_or_default();
            let equal = if opts.compare_xml_as_dom && looks_like_xml(&lb) && looks_like_xml(&rb) {
                match (parse_element(lb), parse_element(rb)) {
                    (Ok(a), Ok(b)) => elements_equal_with(&a, &b, &opts.xml_options),
                    _ => lb == rb,
                }
            } else {
                lb == rb
            };
            if !equal {
                diff.content_mismatch.push(u.clone());
            }
        }
    }

    if opts.compare_relationships {
        // Package-level rels
        let lr = left.package_relationships();
        let rr = right.package_relationships();
        if rels_mismatch(lr, rr) {
            diff.relationship_mismatch.push("/_rels/.rels".into());
        }
        for u in left_uris.intersection(&right_uris) {
            let uri = super::PackUri::new(u.clone());
            let lr = left.part_relationships(&uri);
            let rr = right.part_relationships(&uri);
            match (lr, rr) {
                (None, None) => {}
                (Some(a), Some(b)) => {
                    if rels_mismatch(a, b) {
                        diff.relationship_mismatch.push(format!("{u}.rels"));
                    }
                }
                _ => diff.relationship_mismatch.push(format!("{u}.rels")),
            }
        }
    }

    if opts.compare_content_types {
        // Compare override set as strings
        let mut lt: Vec<_> = left
            .content_types()
            .overrides
            .iter()
            .map(|(k, v)| format!("{k}=>{v}"))
            .collect();
        let mut rt: Vec<_> = right
            .content_types()
            .overrides
            .iter()
            .map(|(k, v)| format!("{k}=>{v}"))
            .collect();
        lt.sort();
        rt.sort();
        if lt != rt {
            diff.content_mismatch.push("[Content_Types].xml".into());
        }
    }

    diff
}

pub fn packages_equal(left: &OpcPackage, right: &OpcPackage) -> bool {
    compare_packages(left, right, &PackageEqualityOptions::default()).is_empty()
}

fn looks_like_xml(data: &[u8]) -> bool {
    let s = std::str::from_utf8(data).unwrap_or("");
    let t = s.trim_start();
    t.starts_with("<?xml") || t.starts_with('<')
}

fn rels_mismatch(a: &super::Relationships, b: &super::Relationships) -> bool {
    let mut av: Vec<_> = a
        .iter()
        .map(|r| {
            (
                r.relationship_type.clone(),
                r.target.clone(),
                format!("{:?}", r.target_mode),
            )
        })
        .collect();
    let mut bv: Vec<_> = b
        .iter()
        .map(|r| {
            (
                r.relationship_type.clone(),
                r.target.clone(),
                format!("{:?}", r.target_mode),
            )
        })
        .collect();
    av.sort();
    bv.sort();
    av != bv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::content_type;
    use crate::opc::PackUri;

    #[test]
    fn packages_equal_roundtrip() {
        let mut a = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        a.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        let mut b = OpcPackage::create();
        b.set_part(doc, content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        assert!(packages_equal(&a, &b));
        b.set_part(
            PackUri::new("/word/styles.xml"),
            content_type::WORD_STYLES,
            b"<w:styles/>".to_vec(),
        );
        let d = compare_packages(&a, &b, &PackageEqualityOptions::default());
        assert!(!d.is_empty());
        assert!(d.only_in_right.iter().any(|u| u.contains("styles")));
    }
}
