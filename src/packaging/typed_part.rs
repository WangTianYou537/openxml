//! Strongly-typed part handles backed by generated [`PartInfo`] metadata.
//!
//! C# generates a full `*Part` partial class per part. Rust keeps one runtime
//! handle ([`TypedPart`]) parameterized by [`PartInfo`], plus helpers to add /
//! list children using relationship-type and content-type constraints.

use crate::element::{parse_element, write_element, OpenXmlElement};
use crate::error::{Error, Result};
use crate::generated::parts::{
    allows_multiple, is_allowed_child, part_by_name, part_by_relationship_type, PartInfo,
};
use crate::opc::{
    OpcPackage, PackUri, PartUriHelper, RelatedPart, RelationshipTargetMode,
};
use crate::packaging::OpenXmlPackage;

/// A part instance identified by generated metadata (C# `OpenXmlPart` subclass shell).
#[derive(Debug, Clone)]
pub struct TypedPart {
    pub info: &'static PartInfo,
    pub uri: PackUri,
    pub relationship_id: Option<String>,
}

impl TypedPart {
    pub fn new(info: &'static PartInfo, uri: PackUri) -> Self {
        Self {
            info,
            uri,
            relationship_id: None,
        }
    }

    pub fn with_relationship_id(mut self, id: impl Into<String>) -> Self {
        self.relationship_id = Some(id.into());
        self
    }

    pub fn name(&self) -> &'static str {
        self.info.name
    }

    pub fn relationship_type(&self) -> &'static str {
        self.info.relationship_type
    }

    pub fn content_type(&self) -> Option<&'static str> {
        self.info.content_type
    }

    pub fn root_element_name(&self) -> Option<&'static str> {
        self.info.root_element
    }

    /// Whether this part is available in `version` (C# `OpenXmlPart.IsInVersion`).
    ///
    /// Uses the relationship type year heuristic from
    /// [`crate::packaging::relationship_introduced_in`] (same approach as
    /// PackageValidator), so Office 2010+ relationships are rejected when
    /// validating against Office 2007.
    pub fn is_in_version(&self, version: crate::file_format::FileFormatVersions) -> bool {
        let intro =
            crate::packaging::relationship_introduced_in(self.info.relationship_type);
        version.at_least(intro) || version.includes_introduction(intro)
    }

    /// Load and parse the part root element from the package.
    pub fn root(&self, package: &OpenXmlPackage) -> Result<OpenXmlElement> {
        let data = package
            .opc()
            .get_part(&self.uri)
            .ok_or_else(|| Error::PartNotFound(self.uri.to_string()))?;
        parse_element(data)
    }

    /// Write `element` as this part's content (marks package dirty via set_part).
    pub fn save_root(&self, package: &mut OpenXmlPackage, element: &OpenXmlElement) -> Result<()> {
        let ct = self
            .info
            .content_type
            .ok_or_else(|| Error::Package(format!("{} has no fixed content type", self.info.name)))?;
        let xml = write_element(element)?;
        package.set_part(self.uri.clone(), ct, xml);
        Ok(())
    }

    /// Allowed child part constraints from schema metadata.
    pub fn child_constraints(&self) -> &'static [crate::generated::parts::PartChildConstraint] {
        self.info.children
    }

    /// List related child parts that match a child constraint name (e.g. `"ImagePart"`).
    pub fn children_of(
        &self,
        package: &OpenXmlPackage,
        child_part_name: &str,
    ) -> Vec<RelatedPart> {
        let Some(child_info) = part_by_name(child_part_name) else {
            return Vec::new();
        };
        package
            .opc()
            .parts_of_relationship_type(Some(&self.uri), child_info.relationship_type)
    }

    /// All internal related parts from this part.
    pub fn children(&self, package: &OpenXmlPackage) -> Vec<RelatedPart> {
        package.opc().related_parts(Some(&self.uri), None)
    }
}

/// Look up metadata and open an existing part at `uri` if content/rel types match.
pub fn open_typed_part(
    package: &OpenXmlPackage,
    part_name: &str,
    uri: &PackUri,
) -> Result<TypedPart> {
    let info = part_by_name(part_name)
        .ok_or_else(|| Error::Package(format!("unknown part type `{part_name}`")))?;
    if !package.opc().has_part(uri) {
        return Err(Error::PartNotFound(uri.to_string()));
    }
    if let Some(ct) = info.content_type {
        if let Some(actual) = package.opc().content_types().content_type_for(uri.as_str()) {
            if actual != ct {
                return Err(Error::Package(format!(
                    "part `{uri}` content type `{actual}` != expected `{ct}` for {part_name}"
                )));
            }
        }
    }
    Ok(TypedPart::new(info, uri.clone()))
}

/// Find parts of type `part_name` related from `source` (package-level if `None`).
pub fn find_typed_parts(
    package: &OpenXmlPackage,
    source: Option<&PackUri>,
    part_name: &str,
) -> Result<Vec<TypedPart>> {
    let info = part_by_name(part_name)
        .ok_or_else(|| Error::Package(format!("unknown part type `{part_name}`")))?;
    let related = package
        .opc()
        .parts_of_relationship_type(source, info.relationship_type);
    Ok(related
        .into_iter()
        .filter(|r| {
            if let Some(ct) = info.content_type {
                r.content_type.as_deref() == Some(ct)
            } else {
                true
            }
        })
        .map(|r| {
            TypedPart::new(info, r.uri).with_relationship_id(r.id)
        })
        .collect())
}

/// Add a new part of type `part_name` under `parent`, using [`PartUriHelper`] for the URI.
///
/// Returns the created [`TypedPart`] (with relationship id set).
pub fn add_typed_part(
    package: &mut OpenXmlPackage,
    parent: &PackUri,
    parent_part_name: Option<&str>,
    part_name: &str,
    data: impl Into<Vec<u8>>,
) -> Result<TypedPart> {
    let info = part_by_name(part_name)
        .ok_or_else(|| Error::Package(format!("unknown part type `{part_name}`")))?;
    if let Some(parent_name) = parent_part_name {
        if !is_allowed_child(parent_name, part_name) {
            return Err(Error::Package(format!(
                "`{part_name}` is not an allowed child of `{parent_name}`"
            )));
        }
        if !allows_multiple(parent_name, part_name) {
            let existing = package
                .opc()
                .parts_of_relationship_type(Some(parent), info.relationship_type);
            if !existing.is_empty() {
                return Err(Error::Package(format!(
                    "`{parent_name}` already has a `{part_name}` (maxOccurs=1)"
                )));
            }
        }
    }

    let ct = info.content_type.unwrap_or("application/octet-stream");
    let ext = if info.root_element.is_some() || ct.ends_with("+xml") || ct.contains("xml") {
        ".xml"
    } else {
        ""
    };
    let mut helper = PartUriHelper::from_package(package.opc());
    let uri = helper.create_part_uri(
        ct,
        parent,
        info.path_general,
        info.target,
        ext,
        true,
    )?;
    package.set_part(uri.clone(), ct, data.into());
    let rid = package.add_part_relationship(
        parent,
        info.relationship_type,
        &uri,
        RelationshipTargetMode::Internal,
    );
    Ok(TypedPart::new(info, uri).with_relationship_id(rid))
}

/// Add a typed part whose root is an [`OpenXmlElement`].
pub fn add_typed_part_element(
    package: &mut OpenXmlPackage,
    parent: &PackUri,
    parent_part_name: Option<&str>,
    part_name: &str,
    element: &OpenXmlElement,
) -> Result<TypedPart> {
    let xml = write_element(element)?;
    add_typed_part(package, parent, parent_part_name, part_name, xml)
}

/// C# `AddNewPart<T>(contentType, id)` shell — create a typed child part with an
/// optional fixed relationship id and optional content-type override.
///
/// When `content_type` is `None`, uses the generated PartInfo fixed content type
/// (or `application/octet-stream`). When `relationship_id` is `Some`, that id is
/// used if free; otherwise a unique id is generated.
pub fn add_new_part(
    package: &mut OpenXmlPackage,
    parent: &PackUri,
    parent_part_name: Option<&str>,
    part_name: &str,
    content_type: Option<&str>,
    relationship_id: Option<&str>,
    data: impl Into<Vec<u8>>,
) -> Result<TypedPart> {
    let info = part_by_name(part_name)
        .ok_or_else(|| Error::Package(format!("unknown part type `{part_name}`")))?;
    if let Some(parent_name) = parent_part_name {
        if !is_allowed_child(parent_name, part_name) {
            return Err(Error::Package(format!(
                "`{part_name}` is not an allowed child of `{parent_name}`"
            )));
        }
        if !allows_multiple(parent_name, part_name) {
            let existing = package
                .opc()
                .parts_of_relationship_type(Some(parent), info.relationship_type);
            if !existing.is_empty() {
                return Err(Error::Package(format!(
                    "`{parent_name}` already has a `{part_name}` (maxOccurs=1)"
                )));
            }
        }
    }

    let ct = content_type
        .or(info.content_type)
        .unwrap_or("application/octet-stream");
    let ext = if info.root_element.is_some() || ct.ends_with("+xml") || ct.contains("xml") {
        ".xml"
    } else {
        ""
    };
    let mut helper = PartUriHelper::from_package(package.opc());
    let uri = helper.create_part_uri(
        ct,
        parent,
        info.path_general,
        info.target,
        ext,
        true,
    )?;
    package.set_part(uri.clone(), ct, data.into());
    let rid = if let Some(id) = relationship_id {
        package.create_relationship_to_part(
            parent,
            &uri,
            info.relationship_type,
            Some(id),
        )?
    } else {
        package.add_part_relationship(
            parent,
            info.relationship_type,
            &uri,
            RelationshipTargetMode::Internal,
        )
    };
    Ok(TypedPart::new(info, uri).with_relationship_id(rid))
}

/// Resolve part metadata from a relationship type URI.
pub fn part_info_for_relationship(relationship_type: &str) -> Option<&'static PartInfo> {
    part_by_relationship_type(relationship_type)
}

/// Convenience: list package-level office document main part as TypedPart when possible.
pub fn main_typed_part(package: &OpenXmlPackage, part_name: &str) -> Result<Option<TypedPart>> {
    let info = part_by_name(part_name)
        .ok_or_else(|| Error::Package(format!("unknown part type `{part_name}`")))?;
    let Ok(uri) = package.opc().main_part_uri(info.relationship_type) else {
        // Main document uses officeDocument rel, not the part's own rel type.
        return Ok(None);
    };
    if !package.opc().has_part(&uri) {
        return Ok(None);
    }
    Ok(Some(TypedPart::new(info, uri)))
}

/// Walk relationship graph and collect TypedParts whose metadata name is `part_name`.
pub fn find_typed_parts_recursive(
    package: &OpenXmlPackage,
    part_name: &str,
) -> Result<Vec<TypedPart>> {
    let info = part_by_name(part_name)
        .ok_or_else(|| Error::Package(format!("unknown part type `{part_name}`")))?;
    let reachable = package.opc().reachable_parts(None);
    let mut out = Vec::new();
    for uri in reachable {
        let Some(ct) = package.opc().content_types().content_type_for(uri.as_str()) else {
            continue;
        };
        if let Some(expected) = info.content_type {
            if ct != expected {
                continue;
            }
        } else {
            continue;
        }
        out.push(TypedPart::new(info, uri));
    }
    out.sort_by(|a, b| a.uri.as_str().cmp(b.uri.as_str()));
    Ok(out)
}

/// Delete a typed part by relationship id from `parent` (orphan cascade).
pub fn delete_typed_part_by_id(
    package: &mut OpenXmlPackage,
    parent: &PackUri,
    relationship_id: &str,
) -> bool {
    package.delete_part_by_id(Some(parent), relationship_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::{content_type, rel, ns};
    use crate::opc::OpcPackage;

    fn pkg_with_doc() -> (OpenXmlPackage, PackUri) {
        let mut opc = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        opc.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#.to_vec(),
        );
        opc.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &doc,
            RelationshipTargetMode::Internal,
        );
        let pkg = OpenXmlPackage::from_opc(opc, Default::default());
        (pkg, doc)
    }

    #[test]
    fn add_styles_typed_part() {
        let (mut pkg, doc) = pkg_with_doc();
        let styles_xml = OpenXmlElement::new("w", ns::WORDPROCESSINGML.uri, "styles")
            .with_ns_decl("w", ns::WORDPROCESSINGML.uri);
        let part = add_typed_part_element(
            &mut pkg,
            &doc,
            Some("MainDocumentPart"),
            "StyleDefinitionsPart",
            &styles_xml,
        )
        .unwrap();
        assert_eq!(part.name(), "StyleDefinitionsPart");
        assert!(pkg.opc().has_part(&part.uri));
        assert!(part.relationship_id.is_some());
        let kids = find_typed_parts(&pkg, Some(&doc), "StyleDefinitionsPart").unwrap();
        assert_eq!(kids.len(), 1);
        let root = kids[0].root(&pkg).unwrap();
        assert_eq!(root.local_name, "styles");
    }

    #[test]
    fn rejects_disallowed_child() {
        let (mut pkg, doc) = pkg_with_doc();
        let err = add_typed_part(
            &mut pkg,
            &doc,
            Some("MainDocumentPart"),
            "WorksheetPart",
            b"<x:worksheet/>",
        );
        assert!(err.is_err());
    }

    #[test]
    fn is_in_version_default_true() {
        use crate::file_format::FileFormatVersions;
        use crate::generated::parts::part_by_name;
        let info = part_by_name("MainDocumentPart").expect("MainDocumentPart");
        let part = TypedPart::new(info, PackUri::new("/word/document.xml"));
        assert!(part.is_in_version(FileFormatVersions::OFFICE2007));
        assert!(part.is_in_version(FileFormatVersions::OFFICE2016));
    }

    #[test]
    fn is_in_version_uses_relationship_year() {
        use crate::file_format::FileFormatVersions;
        use crate::generated::parts::part_by_name;
        // 2011 relationship → Office2010 introduction.
        let info = part_by_name("WordprocessingCommentsExPart").expect("CommentsEx");
        let part = TypedPart::new(info, PackUri::new("/word/commentsExtended.xml"));
        assert!(!part.is_in_version(FileFormatVersions::OFFICE2007));
        assert!(part.is_in_version(FileFormatVersions::OFFICE2010));
        assert!(part.is_in_version(FileFormatVersions::OFFICE2016));
    }

    #[test]
    fn add_new_part_with_explicit_id() {
        let (mut pkg, doc) = pkg_with_doc();
        let part = add_new_part(
            &mut pkg,
            &doc,
            Some("MainDocumentPart"),
            "StyleDefinitionsPart",
            None,
            Some("rIdStyles"),
            b"<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>",
        )
        .expect("add styles");
        assert_eq!(part.relationship_id.as_deref(), Some("rIdStyles"));
        assert!(pkg.opc().has_part(&part.uri));
        assert_eq!(
            pkg.try_get_part_by_id(Some(&doc), "rIdStyles"),
            Some(part.uri.clone())
        );
    }
}
