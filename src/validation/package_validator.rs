//! Package relationship-constraint validation (C# `PackageValidator`).
//!
//! Walks the part graph from the package root (officeDocument main part) and
//! checks each container's children against [`PartConstraintFeature`]:
//! - `PartIsNotAllowed` — relationship type not in the parent rule table
//! - `RequiredPartDoNotExist` — minOccurs > 0 and missing
//! - `OnlyOnePartAllowed` — maxOccurs = 1 but multiple instances
//! - `InvalidContentTypePart` — fixed content type mismatch
//! - `DataPartReferenceIsNotAllowed` — media/audio/video ref not allowed here

use super::ValidationError;
use crate::generated::parts::{part_by_name, part_by_relationship_type, PartInfo, PARTS};
use crate::namespace::rel;
use crate::opc::{media_rel, OpcPackage, PackUri, RelationshipTargetMode};
use crate::packaging::PartConstraintFeature;
use std::collections::{HashMap, HashSet};

/// C# `OpenXmlPackageValidationResult` shell (internal packaging validation event).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OpenXmlPackageValidationResult {
    pub message: String,
    pub message_id: Option<String>,
    pub relationship_type: Option<String>,
    pub part_uri: Option<String>,
    pub sub_part_uri: Option<String>,
    /// Data-part reference relationship id when the fault is a media/audio/video ref
    /// (C# `DataPartReferenceRelationship` shell).
    pub data_part_reference_id: Option<String>,
}

impl OpenXmlPackageValidationResult {
    pub fn new(message_id: impl Into<String>, detail: impl Into<String>) -> Self {
        let message_id = message_id.into();
        let detail = detail.into();
        Self {
            message: format!("{message_id}: {detail}"),
            message_id: Some(message_id),
            relationship_type: None,
            part_uri: None,
            sub_part_uri: None,
            data_part_reference_id: None,
        }
    }

    pub fn with_relationship_type(mut self, relationship_type: impl Into<String>) -> Self {
        self.relationship_type = Some(relationship_type.into());
        self
    }

    pub fn with_part_uri(mut self, uri: impl Into<String>) -> Self {
        self.part_uri = Some(uri.into());
        self
    }

    pub fn with_sub_part_uri(mut self, uri: impl Into<String>) -> Self {
        self.sub_part_uri = Some(uri.into());
        self
    }

    pub fn with_data_part_reference_id(mut self, id: impl Into<String>) -> Self {
        self.data_part_reference_id = Some(id.into());
        self
    }

    pub fn into_validation_error(self) -> ValidationError {
        let mut err = ValidationError {
            path: self.part_uri.clone().unwrap_or_default(),
            message: self.message,
            ..Default::default()
        }
        .with_error_type(super::ValidationErrorType::Package);
        if let Some(sub) = self.sub_part_uri {
            err = err.with_related_part_uri(sub);
        }
        err
    }

    /// Message id when present (C# `OpenXmlPackageValidationResult.MessageId`).
    pub fn message_id_str(&self) -> Option<&str> {
        self.message_id.as_deref()
    }

    /// Relationship type associated with this result.
    pub fn relationship_type_str(&self) -> Option<&str> {
        self.relationship_type.as_deref()
    }

    pub fn part_is_not_allowed(
        part_uri: impl Into<String>,
        relationship_type: impl Into<String>,
    ) -> Self {
        let relationship_type = relationship_type.into();
        Self::new(message_id::PART_IS_NOT_ALLOWED, relationship_type.clone())
            .with_part_uri(part_uri)
            .with_relationship_type(relationship_type)
    }

    pub fn required_part_do_not_exist(
        part_uri: impl Into<String>,
        relationship_type: impl Into<String>,
    ) -> Self {
        let relationship_type = relationship_type.into();
        Self::new(message_id::REQUIRED_PART_DO_NOT_EXIST, relationship_type.clone())
            .with_part_uri(part_uri)
            .with_relationship_type(relationship_type)
    }

    pub fn only_one_part_allowed(
        part_uri: impl Into<String>,
        relationship_type: impl Into<String>,
    ) -> Self {
        let relationship_type = relationship_type.into();
        Self::new(message_id::ONLY_ONE_PART_ALLOWED, relationship_type.clone())
            .with_part_uri(part_uri)
            .with_relationship_type(relationship_type)
    }

    pub fn invalid_content_type_part(
        part_uri: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(message_id::INVALID_CONTENT_TYPE_PART, detail).with_part_uri(part_uri)
    }

    pub fn data_part_reference_is_not_allowed(
        part_uri: impl Into<String>,
        relationship_type: impl Into<String>,
    ) -> Self {
        let relationship_type = relationship_type.into();
        Self::new(
            message_id::DATA_PART_REFERENCE_IS_NOT_ALLOWED,
            relationship_type.clone(),
        )
        .with_part_uri(part_uri)
        .with_relationship_type(relationship_type)
    }

    /// Like [`data_part_reference_is_not_allowed`](Self::data_part_reference_is_not_allowed)
    /// but records the offending relationship id.
    pub fn data_part_reference_is_not_allowed_with_id(
        part_uri: impl Into<String>,
        relationship_type: impl Into<String>,
        relationship_id: impl Into<String>,
    ) -> Self {
        Self::data_part_reference_is_not_allowed(part_uri, relationship_type)
            .with_data_part_reference_id(relationship_id)
    }

    /// Best-effort parse of a package constraint [`ValidationError`] back into a result shell.
    pub fn from_validation_error(error: &ValidationError) -> Self {
        let message_id = error.id().map(|s| s.to_string());
        let mut r = Self {
            message: error.message.clone(),
            message_id,
            relationship_type: None,
            part_uri: if error.path.is_empty() {
                None
            } else {
                Some(error.path.clone())
            },
            sub_part_uri: error.related_part_uri.clone(),
            data_part_reference_id: None,
        };
        // Detail after `Id: ` often is the relationship type for package constraints.
        if let Some(id) = error.id() {
            let detail = error.description();
            if !detail.is_empty()
                && (id == message_id::PART_IS_NOT_ALLOWED
                    || id == message_id::REQUIRED_PART_DO_NOT_EXIST
                    || id == message_id::ONLY_ONE_PART_ALLOWED
                    || id == message_id::DATA_PART_REFERENCE_IS_NOT_ALLOWED)
            {
                r.relationship_type = Some(detail.to_string());
            }
        }
        r
    }
}

/// Message ids mirroring C# `OpenXmlPackageValidationResult.MessageId`.
pub mod message_id {
    pub const PART_IS_NOT_ALLOWED: &str = "PartIsNotAllowed";
    pub const REQUIRED_PART_DO_NOT_EXIST: &str = "RequiredPartDoNotExist";
    pub const ONLY_ONE_PART_ALLOWED: &str = "OnlyOnePartAllowed";
    pub const INVALID_CONTENT_TYPE_PART: &str = "InvalidContentTypePart";
    pub const DATA_PART_REFERENCE_IS_NOT_ALLOWED: &str = "DataPartReferenceIsNotAllowed";
}

fn err(path: impl Into<String>, message_id: &str, detail: impl Into<String>) -> ValidationError {
    ValidationError {
        path: path.into(),
        message: format!("{message_id}: {}", detail.into()),
        ..Default::default()
    }
    .with_error_type(super::ValidationErrorType::Package)
}

/// Infer the parent part type name for the package container (C# package-level constraints).
///
/// Package-level children are core/app/custom props, thumbnail, digsig origin, and the main
/// officeDocument. We do not enforce a fixed package PartInfo (C# uses a package feature bag);
/// validation starts at the main document part.
fn main_part_type_name(content_type: &str) -> Option<&'static str> {
    if content_type.contains("wordprocessingml") {
        Some("MainDocumentPart")
    } else if content_type.contains("spreadsheetml") && content_type.contains("sheet.main") {
        Some("WorkbookPart")
    } else if content_type.contains("spreadsheetml") && content_type.contains("template.main") {
        Some("WorkbookPart")
    } else if content_type.contains("presentationml")
        && (content_type.contains("presentation.main") || content_type.contains("template.main"))
    {
        Some("PresentationPart")
    } else if content_type.contains("wordprocessingml.document.main")
        || content_type.contains("wordprocessingml.template.main")
    {
        Some("MainDocumentPart")
    } else {
        None
    }
}

/// Resolve a child part's `PartInfo` name from relationship type + content type.
fn child_part_name(relationship_type: &str, content_type: Option<&str>) -> Option<&'static str> {
    // Prefer content-type match when several PartInfo share a relationship URI
    // (comments, printerSettings, styles, officeDocument).
    if let Some(ct) = content_type {
        if let Some(info) = part_by_content_type(ct) {
            return Some(info.name);
        }
    }
    part_by_relationship_type(relationship_type).map(|p| p.name)
}

fn part_by_content_type(ct: &str) -> Option<&'static PartInfo> {
    PARTS.iter().find(|p| p.content_type == Some(ct))
}

/// Known data-part reference relationship types (C# `IKnownDataPartFeature` subset).
fn is_known_data_part_rel(relationship_type: &str) -> bool {
    matches!(
        relationship_type,
        media_rel::AUDIO | media_rel::VIDEO | media_rel::MEDIA
    ) || relationship_type.ends_with("/relationships/audio")
        || relationship_type.ends_with("/relationships/video")
        || relationship_type.ends_with("/relationships/media")
}

/// Validate package part-relationship constraints (C# `PackageValidator.Validate`).
///
/// Does **not** re-check missing relationship targets (see [`super::validate_package`]);
/// focuses on PartConstraintFeature rules walked from the main part.
/// Run package constraint validation and return typed results
/// (C# `PackageValidator.Validate` yield of `OpenXmlPackageValidationResult`).
pub fn validate_package_constraint_results(
    package: &OpcPackage,
) -> Vec<OpenXmlPackageValidationResult> {
    validate_package_constraints(package)
        .iter()
        .map(OpenXmlPackageValidationResult::from_validation_error)
        .collect()
}

pub fn validate_package_constraints(package: &OpcPackage) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut processed: HashSet<String> = HashSet::new();

    let Some(main_rel) = package
        .package_relationships()
        .get_by_type(rel::OFFICE_DOCUMENT)
    else {
        return errors;
    };
    if main_rel.target_mode == RelationshipTargetMode::External {
        return errors;
    }
    let Ok(main_uri) = resolve_pkg_target(&main_rel.target) else {
        return errors;
    };
    if !package.has_part(&main_uri) {
        return errors;
    }
    let main_ct = package
        .content_types()
        .content_type_for(main_uri.as_str())
        .unwrap_or("");
    let Some(main_type) = main_part_type_name(main_ct) else {
        // Unknown main content type — skip constraint walk.
        return errors;
    };

    validate_container(
        package,
        &main_uri,
        main_type,
        &mut processed,
        &mut errors,
    );
    errors
}

/// Validate a single container (part) and recurse into its children.
pub fn validate_part_constraints(
    package: &OpcPackage,
    part_uri: &PackUri,
    parent_part_name: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut processed: HashSet<String> = HashSet::new();
    validate_container(
        package,
        part_uri,
        parent_part_name,
        &mut processed,
        &mut errors,
    );
    errors
}

fn validate_container(
    package: &OpcPackage,
    container_uri: &PackUri,
    parent_part_name: &str,
    processed: &mut HashSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let key = container_uri.as_str().to_string();
    if !processed.insert(key) {
        return;
    }

    let feature = PartConstraintFeature::for_name(parent_part_name);
    let Some(rels) = package.part_relationships(container_uri) else {
        // Still check required children when there are no relationships at all.
        for missing in feature.missing_required(std::iter::empty()) {
            errors.push(err(
                container_uri.as_str(),
                message_id::REQUIRED_PART_DO_NOT_EXIST,
                format!("required child `{missing}` missing on `{parent_part_name}`"),
            ));
        }
        return;
    };

    // Count internal part relationships by type (exclude external + pure hyperlinks).
    let mut part_occurs: HashMap<String, usize> = HashMap::new();
    let mut child_entries: Vec<(String, String, Option<String>, PackUri)> = Vec::new();
    // (rel_type, rel_id, content_type, target_uri)

    for rel_item in rels.iter() {
        // Data-part references (audio/video/media) — allowed check only; not walked as parts.
        if is_known_data_part_rel(&rel_item.relationship_type) {
            if !feature.is_data_part_reference_allowed(&rel_item.relationship_type) {
                errors.push(err(
                    format!("{}#{}", container_uri.as_str(), rel_item.id),
                    message_id::DATA_PART_REFERENCE_IS_NOT_ALLOWED,
                    format!(
                        "data part reference `{}` not allowed on `{parent_part_name}`",
                        rel_item.relationship_type
                    ),
                ));
            }
            continue;
        }
        if rel_item.target_mode == RelationshipTargetMode::External {
            continue;
        }
        if rel_item.target.contains("://") {
            continue;
        }
        let Ok(target) = crate::opc::resolve_uri(container_uri, &rel_item.target) else {
            continue;
        };
        *part_occurs
            .entry(rel_item.relationship_type.clone())
            .or_insert(0) += 1;

        let ct = package
            .content_types()
            .content_type_for(target.as_str())
            .map(|s| s.to_string());
        child_entries.push((
            rel_item.relationship_type.clone(),
            rel_item.id.clone(),
            ct,
            target,
        ));
    }

    // PartIsNotAllowed for each child relationship.
    for (rel_type, rel_id, _ct, _target) in &child_entries {
        if feature.try_get_rule(rel_type).is_none() {
            // Treat as ExtendedPart: ignore unknown non-Office relationship types;
            // still report openxmlformats officeDocument relationships that are not allowed.
            let is_office_rel = rel_type.contains("schemas.openxmlformats.org")
                || rel_type.contains("schemas.microsoft.com/office");
            if is_office_rel {
                errors.push(err(
                    format!("{}#{}", container_uri.as_str(), rel_id),
                    message_id::PART_IS_NOT_ALLOWED,
                    format!("relationship `{rel_type}` is not allowed on `{parent_part_name}`"),
                ));
            }
        }
    }

    // Required + maxOccurs rules.
    for rule in feature.rules() {
        if rule.is_data_part_reference {
            continue;
        }
        let occurs = part_occurs.get(rule.relationship_type).copied().unwrap_or(0);
        if rule.required() && occurs == 0 {
            errors.push(err(
                container_uri.as_str(),
                message_id::REQUIRED_PART_DO_NOT_EXIST,
                format!(
                    "required child `{}` ({}) missing on `{parent_part_name}`",
                    rule.part_name, rule.relationship_type
                ),
            ));
        }
        if !rule.allows_multiple() && occurs > 1 {
            errors.push(err(
                container_uri.as_str(),
                message_id::ONLY_ONE_PART_ALLOWED,
                format!(
                    "`{}` occurs {occurs} times on `{parent_part_name}` (maxOccurs=1)",
                    rule.part_name
                ),
            ));
        }
    }

    // Content-type checks + recurse.
    for (rel_type, rel_id, ct, target) in &child_entries {
        if let Some(rule) = feature.try_get_rule(rel_type) {
            if let Some(expected) = rule.content_type {
                match ct.as_deref() {
                    Some(actual) if actual == expected => {}
                    Some(actual) => {
                        errors.push(err(
                            target.as_str(),
                            message_id::INVALID_CONTENT_TYPE_PART,
                            format!(
                                "part content type `{actual}` != expected `{expected}` (via {rel_id})"
                            ),
                        ));
                    }
                    None => {
                        errors.push(err(
                            target.as_str(),
                            message_id::INVALID_CONTENT_TYPE_PART,
                            format!("part has no content type; expected `{expected}` (via {rel_id})"),
                        ));
                    }
                }
            }
            // Recurse with child part type name from rule (more precise than CT lookup).
            if package.has_part(target) {
                validate_container(package, target, rule.part_name, processed, errors);
            }
        } else if package.has_part(target) {
            // Unknown / extended: try to infer type for deeper walk when possible.
            if let Some(name) = child_part_name(rel_type, ct.as_deref()) {
                validate_container(package, target, name, processed, errors);
            }
        }
    }
}

fn resolve_pkg_target(target: &str) -> std::result::Result<PackUri, String> {
    let root = PackUri::new("/");
    crate::opc::resolve_uri(&root, target).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::opc::OpcPackage;
    #[test]
    fn open_xml_package_validation_result_shell() {
        let r = super::OpenXmlPackageValidationResult::new(
            super::message_id::PART_IS_NOT_ALLOWED,
            "chart not allowed under styles",
        )
        .with_relationship_type("http://rel/chart")
        .with_part_uri("/word/styles.xml")
        .with_sub_part_uri("/word/charts/chart1.xml");
        assert_eq!(
            r.message_id.as_deref(),
            Some(super::message_id::PART_IS_NOT_ALLOWED)
        );
        assert!(r.message.contains("PartIsNotAllowed"));
        let e = r.into_validation_error();
        assert_eq!(e.path, "/word/styles.xml");
        assert_eq!(e.error_type(), super::super::ValidationErrorType::Package);
    }

    use super::*;
    use crate::namespace::content_type;
    use crate::opc::RelationshipTargetMode;
    use crate::packaging::PartConstraintFeature;

    fn minimal_word() -> OpcPackage {
        let mut pkg = OpcPackage::create();
        let uri = PackUri::new("/word/document.xml");
        pkg.set_part(
            uri.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#.to_vec(),
        );
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        pkg
    }

    #[test]
    fn minimal_word_has_no_constraint_errors() {
        let pkg = minimal_word();
        let errs = validate_package_constraints(&pkg);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn dual_styles_reports_only_one_allowed() {
        let mut pkg = minimal_word();
        let main = PackUri::new("/word/document.xml");
        let s1 = PackUri::new("/word/styles.xml");
        let s2 = PackUri::new("/word/styles2.xml");
        let styles_ct = content_type::WORD_STYLES;
        pkg.set_part(s1.clone(), styles_ct, b"<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>".to_vec());
        pkg.set_part(s2.clone(), styles_ct, b"<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>".to_vec());
        let styles_rel = part_by_name("StyleDefinitionsPart")
            .unwrap()
            .relationship_type;
        pkg.part_relationships_mut(&main).add(
            styles_rel,
            "styles.xml",
            RelationshipTargetMode::Internal,
        );
        pkg.part_relationships_mut(&main).add(
            styles_rel,
            "styles2.xml",
            RelationshipTargetMode::Internal,
        );
        let errs = validate_package_constraints(&pkg);
        assert!(
            errs.iter()
                .any(|e| e.message.contains(message_id::ONLY_ONE_PART_ALLOWED)),
            "{errs:?}"
        );
    }

    #[test]
    fn disallowed_calc_chain_on_main_document() {
        let mut pkg = minimal_word();
        let main = PackUri::new("/word/document.xml");
        // calcChain is Excel-only — not allowed on MainDocumentPart.
        let calc = PackUri::new("/word/calcChain.xml");
        pkg.set_part(
            calc,
            content_type::SPREADSHEET_CALC_CHAIN,
            b"<calcChain/>".to_vec(),
        );
        let calc_rel = part_by_name("CalculationChainPart")
            .unwrap()
            .relationship_type;
        pkg.part_relationships_mut(&main).add(
            calc_rel,
            "calcChain.xml",
            RelationshipTargetMode::Internal,
        );
        let errs = validate_package_constraints(&pkg);
        assert!(
            errs.iter()
                .any(|e| e.message.contains(message_id::PART_IS_NOT_ALLOWED)),
            "{errs:?}"
        );
    }

    #[test]
    fn video_ref_not_allowed_on_styles_part() {
        // StyleDefinitionsPart has no data-part reference children.
        let mut pkg = minimal_word();
        let main = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        pkg.set_part(
            styles.clone(),
            content_type::WORD_STYLES,
            b"<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>"
                .to_vec(),
        );
        let styles_rel = part_by_name("StyleDefinitionsPart")
            .unwrap()
            .relationship_type;
        pkg.part_relationships_mut(&main).add(
            styles_rel,
            "styles.xml",
            RelationshipTargetMode::Internal,
        );
        pkg.part_relationships_mut(&styles).add(
            media_rel::VIDEO,
            "../media/video1.mp4",
            RelationshipTargetMode::Internal,
        );
        let errs = validate_package_constraints(&pkg);
        assert!(
            errs.iter()
                .any(|e| e.message.contains(message_id::DATA_PART_REFERENCE_IS_NOT_ALLOWED)),
            "{errs:?}"
        );
    }

    #[test]
    fn invalid_content_type_on_styles() {
        let mut pkg = minimal_word();
        let main = PackUri::new("/word/document.xml");
        let s1 = PackUri::new("/word/styles.xml");
        pkg.set_part(
            s1.clone(),
            "application/xml", // wrong
            b"<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>".to_vec(),
        );
        let styles_rel = part_by_name("StyleDefinitionsPart")
            .unwrap()
            .relationship_type;
        pkg.part_relationships_mut(&main).add(
            styles_rel,
            "styles.xml",
            RelationshipTargetMode::Internal,
        );
        let errs = validate_package_constraints(&pkg);
        assert!(
            errs.iter()
                .any(|e| e.message.contains(message_id::INVALID_CONTENT_TYPE_PART)),
            "{errs:?}"
        );
    }

    #[test]
    fn presentation_requires_slide_master() {
        let mut pkg = OpcPackage::create();
        let uri = PackUri::new("/ppt/presentation.xml");
        pkg.set_part(
            uri.clone(),
            content_type::PRESENTATION,
            br#"<?xml version="1.0"?><p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>"#.to_vec(),
        );
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let errs = validate_package_constraints(&pkg);
        assert!(
            errs.iter()
                .any(|e| e.message.contains(message_id::REQUIRED_PART_DO_NOT_EXIST)
                    && e.message.contains("SlideMaster")),
            "{errs:?}"
        );
    }

    #[test]
    fn feature_data_part_rules_resolve() {
        let f = PartConstraintFeature::new("SlidePart");
        assert!(f.is_data_part_reference_allowed(media_rel::VIDEO)
            || f.rules().iter().any(|r| r.is_data_part_reference));
    }

    #[test]
    fn package_validation_result_factories() {
        let r = OpenXmlPackageValidationResult::part_is_not_allowed(
            "/word/document.xml",
            "http://example/rel",
        );
        assert_eq!(r.message_id.as_deref(), Some(message_id::PART_IS_NOT_ALLOWED));
        assert_eq!(r.message_id_str(), Some(message_id::PART_IS_NOT_ALLOWED));
        assert_eq!(r.relationship_type_str(), Some("http://example/rel"));
        assert_eq!(r.part_uri.as_deref(), Some("/word/document.xml"));
        let e = r
            .clone()
            .with_sub_part_uri("/word/styles.xml")
            .into_validation_error();
        assert!(e.message.contains("PartIsNotAllowed"));
        assert_eq!(e.related_part_uri.as_deref(), Some("/word/styles.xml"));
        assert_eq!(
            OpenXmlPackageValidationResult::required_part_do_not_exist("/a", "r")
                .message_id
                .as_deref(),
            Some(message_id::REQUIRED_PART_DO_NOT_EXIST)
        );
        let invalid = OpenXmlPackageValidationResult::invalid_content_type_part(
            "/word/styles.xml",
            "expected styles",
        );
        assert_eq!(
            invalid.message_id_str(),
            Some(message_id::INVALID_CONTENT_TYPE_PART)
        );
        let dpr = OpenXmlPackageValidationResult::data_part_reference_is_not_allowed_with_id(
            "/ppt/slides/slide1.xml",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video",
            "rIdVid",
        );
        assert_eq!(dpr.data_part_reference_id.as_deref(), Some("rIdVid"));
        assert_eq!(
            dpr.message_id_str(),
            Some(message_id::DATA_PART_REFERENCE_IS_NOT_ALLOWED)
        );
        let back = OpenXmlPackageValidationResult::from_validation_error(
            &dpr.clone().into_validation_error(),
        );
        assert_eq!(
            back.message_id_str(),
            Some(message_id::DATA_PART_REFERENCE_IS_NOT_ALLOWED)
        );
    }

    #[test]
    fn validate_package_constraint_results_roundtrip_empty() {
        let pkg = OpcPackage::create();
        let results = validate_package_constraint_results(&pkg);
        assert!(results.is_empty());
    }
}
