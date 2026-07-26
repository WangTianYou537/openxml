//! Package/part validation orchestration (C# `DocumentValidator`).

use super::{
    is_reserved_element, validate_package_constraints_for_version, validate_schema_types_in_tree,
    ValidationCache, ValidationContext, ValidationError, ValidationErrorType, ValidationSettings,
};
use crate::element::{parse_element, OpenXmlElement};
use crate::error::Result;
use crate::namespace::rel;
use crate::opc::{OpcPackage, PackUri};

/// C# `DocumentValidator` — orchestrates package structure + per-part schema passes.
#[derive(Debug)]
pub struct DocumentValidator {
    cache: ValidationCache,
}

impl DocumentValidator {
    /// C# `DocumentValidator(ValidationCache)`.
    pub fn new(cache: ValidationCache) -> Self {
        Self { cache }
    }

    pub fn cache(&self) -> &ValidationCache {
        &self.cache
    }

    /// C# `Validate(OpenXmlPackage, settings, token)` — package frame, package
    /// structure errors, then every reachable XML part.
    pub fn validate_package(
        &self,
        package: &OpcPackage,
        context: &mut ValidationContext,
    ) -> Result<()> {
        context.stack_mut().push_package("/");
        let result = self.validate_package_inner(package, context);
        context.stack_mut().pop();
        result
    }

    fn validate_package_inner(
        &self,
        package: &OpcPackage,
        context: &mut ValidationContext,
    ) -> Result<()> {
        // C# PackageValidator.Validate(version) — filter rules by target FileFormatVersions.
        for error in validate_package_constraints_for_version(package, self.cache.version()) {
            if !context.try_add_error(error)? {
                return Ok(());
            }
        }

        for part_uri in self.parts_to_be_validated(package) {
            self.validate_part_uri(package, &part_uri, context)?;
        }
        Ok(())
    }

    /// C# `Validate(OpenXmlPart, settings, token)`.
    pub fn validate_part(
        &self,
        package: &OpcPackage,
        part_uri: &PackUri,
        context: &mut ValidationContext,
    ) -> Result<()> {
        self.validate_part_uri(package, part_uri, context)
    }

    fn validate_part_uri(
        &self,
        package: &OpcPackage,
        part_uri: &PackUri,
        context: &mut ValidationContext,
    ) -> Result<()> {
        context.stack_mut().push_part(part_uri.as_str());
        let result = self.validate_part_inner(package, part_uri, context);
        context.stack_mut().pop();
        result
    }

    fn validate_part_inner(
        &self,
        package: &OpcPackage,
        part_uri: &PackUri,
        context: &mut ValidationContext,
    ) -> Result<()> {
        let Ok(Some(bytes)) = package.load_part(part_uri) else {
            return Ok(());
        };

        if bytes.is_empty() {
            // C# `part.IsEmptyPart()` → Sch_MissingPartRootElement
            context.try_add_error(
                ValidationError::with_id(
                    part_uri.as_str(),
                    "Sch_MissingPartRootElement",
                    format!("The '{}' part is missing its root element.", part_uri.as_str()),
                )
                .with_error_type(ValidationErrorType::Schema),
            )?;
            return Ok(());
        }

        let root = match parse_element(&bytes) {
            Ok(root) => root,
            Err(error) => {
                // C# XmlException → "ExceptionError" ValidationErrorInfo
                context.try_add_error(
                    ValidationError::with_id(
                        part_uri.as_str(),
                        "ExceptionError",
                        format!("Inner exception: {error}."),
                    )
                    .with_error_type(ValidationErrorType::Schema),
                )?;
                return Ok(());
            }
        };

        self.validate_element(&root, context)?;
        self.validate_part_semantic(package, part_uri, &root, context)
    }

    /// C# `Validate(ValidationContext)` — schema pass then constraint pass over
    /// the current element, both via the MC-aware traverser.
    pub fn validate_element(
        &self,
        root: &OpenXmlElement,
        context: &mut ValidationContext,
    ) -> Result<()> {
        // Keep context cache version aligned with this validator's cache.
        context.cache_mut().set_version(self.cache.version());
        if root.is_misc_node() || root.is_unknown() {
            return Ok(());
        }
        if is_reserved_element(root) {
            // C# rejects validating AC/Choice/Fallback as the top-level target
            // elsewhere; as a part root they are skipped here.
            return Ok(());
        }

        // Schema pass (C# SchemaTypeValidator.Validate per element via traverser).
        validate_schema_types_in_tree(root, context)?;

        // C# AlternateContentValidator + CompatibilityRuleAttributesValidator passes.
        let mut mc_errors = super::validate_alternate_content(root);
        mc_errors.extend(super::validate_mc_attributes(root));
        for error in mc_errors {
            if !context.try_add_error(
                error.with_error_type(ValidationErrorType::MarkupCompatibility),
            )? {
                return Ok(());
            }
        }

        // Constraint pass (C# element.Metadata.Constraints per element).
        // Full extractable Schematron attribute/content table pass.
        for error in super::validate_schematron_constraints(root) {
            if !context.try_add_error(error)? {
                return Ok(());
            }
        }
        // C# CellType IValidator — Sem_CellValue for boolean/date/number cells.
        for error in super::validate_spreadsheet_cell_values(root) {
            if !context.try_add_error(error)? {
                return Ok(());
            }
        }
        Ok(())
    }

    /// Package-aware constraint pass for a part root (relationships, uniqueness,
    /// cross-part index/count) — C# semantic constraints that need part/package.
    fn validate_part_semantic(
        &self,
        package: &OpcPackage,
        part_uri: &PackUri,
        root: &OpenXmlElement,
        context: &mut ValidationContext,
    ) -> Result<()> {
        let rel_rules = super::merged_relationship_rules(super::word_relationship_rules());
        let unique_rules = super::merged_unique_attribute_rules(super::word_unique_attribute_rules());
        for error in super::validate_semantic(package, part_uri, root, &rel_rules, &unique_rules) {
            if !context.try_add_error(
                error.with_error_type(ValidationErrorType::Semantic),
            )? {
                return Ok(());
            }
        }
        for error in super::validate_schematron_cross_part(package, root) {
            if !context.try_add_error(
                error.with_error_type(ValidationErrorType::Semantic),
            )? {
                return Ok(());
            }
        }
        Ok(())
    }

    /// C# `PartsToBeValidated` — reachable XML parts from the main-document root
    /// (only parts defined in the target version are yielded).
    pub fn parts_to_be_validated(&self, package: &OpcPackage) -> Vec<PackUri> {
        let Some(main) = package
            .package_relationships()
            .get_by_type(rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        let root = PackUri::new("/");
        let Ok(main_uri) = crate::opc::resolve_uri(&root, &main.target) else {
            return Vec::new();
        };

        let mut visited = Vec::new();
        let mut queue = vec![main_uri];
        while let Some(uri) = queue.pop() {
            if visited.contains(&uri) || !package.has_part(&uri) {
                continue;
            }
            if let Some(rels) = package.part_relationships(&uri) {
                for rel in rels.iter() {
                    if rel.target_mode != crate::opc::RelationshipTargetMode::Internal {
                        continue;
                    }
                    if let Ok(target) = crate::opc::resolve_uri(&uri, &rel.target) {
                        if target.as_str().ends_with(".xml") {
                            queue.push(target);
                        }
                    }
                }
            }
            visited.push(uri);
        }
        visited
    }
}

impl Default for DocumentValidator {
    fn default() -> Self {
        Self::new(ValidationCache::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_format::FileFormatVersions;
    use crate::namespace::content_type;
    use crate::opc::RelationshipTargetMode;

    fn minimal_package(document_xml: &[u8]) -> OpcPackage {
        let mut package = OpcPackage::create();
        let uri = PackUri::new("/word/document.xml");
        package.set_part(uri.clone(), content_type::WORD_DOCUMENT, document_xml.to_vec());
        package.add_package_relationship(rel::OFFICE_DOCUMENT, &uri, RelationshipTargetMode::Internal);
        package
    }

    #[test]
    fn validates_reachable_parts_and_reports_schema_errors() {
        let package = minimal_package(
            br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/><w:body/></w:document>"#,
        );
        let validator = DocumentValidator::default();
        assert_eq!(
            validator.parts_to_be_validated(&package),
            [PackUri::new("/word/document.xml")]
        );

        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_package(&package, &mut context).unwrap();
        assert!(
            context
                .errors()
                .iter()
                .any(|e| e.message.contains("at most one")
                    || e.message.contains("Sch_InvalidElementContent")
                    || e.message.contains("Sch_UnexpectedElementContent")
                    || e.message.contains("invalid child")
                    || e.message.contains("unexpected child")),
            "{:?}",
            context.errors()
        );
        assert!(context.stack().is_empty());
    }

    #[test]
    fn package_validation_reports_missing_relationship_ids() {
        use crate::namespace::rel as rel_ns;

        let mut package = OpcPackage::create();
        let uri = PackUri::new("/word/document.xml");
        package.set_part(
            uri.clone(),
            content_type::WORD_DOCUMENT,
            br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><w:body><w:p><w:hyperlink r:id="rIdMissing"/></w:p></w:body></w:document>"#.to_vec(),
        );
        package.add_package_relationship(rel::OFFICE_DOCUMENT, &uri, RelationshipTargetMode::Internal);
        // No rIdMissing relationship on the part.

        let validator = DocumentValidator::default();
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_package(&package, &mut context).unwrap();
        assert!(
            context.errors().iter().any(|e| {
                e.message.contains("rIdMissing")
                    || e.description().contains("rIdMissing")
                    || e.message.contains("does not exist")
            }),
            "{:?}",
            context.errors()
        );
        let _ = rel_ns::HYPERLINK; // keep import used if needed later
    }

    #[test]
    fn empty_part_reports_missing_root_element() {
        let package = minimal_package(b"");
        let validator = DocumentValidator::default();
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_package(&package, &mut context).unwrap();
        assert_eq!(context.errors().len(), 1);
        assert_eq!(context.errors()[0].id(), Some("Sch_MissingPartRootElement"));
        assert_eq!(
            context.errors()[0].error_type(),
            ValidationErrorType::Schema
        );
    }

    #[test]
    fn malformed_xml_reports_exception_error() {
        let package = minimal_package(b"<w:document><unclosed>");
        let validator = DocumentValidator::default();
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_package(&package, &mut context).unwrap();
        assert_eq!(context.errors().len(), 1);
        assert_eq!(context.errors()[0].id(), Some("ExceptionError"));
        assert!(context.errors()[0].description().starts_with("Inner exception:"));
    }

    #[test]
    fn constraint_pass_reports_schematron_violations() {
        let root = parse_element(
            br#"<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><x:col min="0" max="1"/></x:worksheet>"#,
        )
        .unwrap();
        let validator = DocumentValidator::default();
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&root, &mut context).unwrap();
        assert!(
            context.errors().iter().any(|e| e.path == "col/@min"),
            "{:?}",
            context.errors()
        );

        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator
            .validate_element(&OpenXmlElement::comment("skip"), &mut context)
            .unwrap();
        assert!(context.errors().is_empty());
    }

    #[test]
    fn schema_pass_reports_mc_structural_errors() {
        let root = crate::element::OpenXmlElement::w("document").with_children(vec![
            crate::markup_compatibility::alternate_content(Vec::<OpenXmlElement>::new()),
        ]);
        let validator = DocumentValidator::default();
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&root, &mut context).unwrap();
        let error = context
            .errors()
            .iter()
            .find(|e| e.message.contains("MC_ShallContainChoice"))
            .expect("MC structural error");
        assert_eq!(
            error.error_type(),
            ValidationErrorType::MarkupCompatibility
        );
    }

    #[test]
    fn non_document_roots_use_particle_registry() {
        use crate::wordprocessing::{paragraph, run, text};

        let validator = DocumentValidator::default();

        let good = paragraph(vec![run(vec![text("ok")])]);
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&good, &mut context).unwrap();
        assert!(context.errors().is_empty(), "{:?}", context.errors());

        let mut bad = paragraph(vec![]);
        bad.append_child(OpenXmlElement::w("body"));
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&bad, &mut context).unwrap();
        assert!(
            context
                .errors()
                .iter()
                .any(|e| {
                    e.id() == Some("Sch_InvalidElementContentExpectingComplex")
                        || e.message.contains("invalid child")
                }),
            "{:?}",
            context.errors()
        );
    }

    #[test]
    fn attribute_values_checked_against_declared_types() {
        use crate::wordprocessing::{paragraph, run, text};

        let validator = DocumentValidator::default();

        let mut good = paragraph(vec![run(vec![text("x")])]);
        good.set_attribute_qname("w:rsidR", "00AB12CD");
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&good, &mut context).unwrap();
        assert!(context.errors().is_empty(), "{:?}", context.errors());

        let mut bad = paragraph(vec![run(vec![text("x")])]);
        bad.set_attribute_qname("w:rsidR", "not-hex");
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&bad, &mut context).unwrap();
        let error = context
            .errors()
            .iter()
            .find(|e| e.id() == Some("Sch_AttributeValueDataTypeDetailed"))
            .expect("typed attribute error");
        assert!(
            error.description().contains("'w:rsidR' has invalid value 'not-hex'"),
            "{error:?}"
        );
    }

    #[test]
    fn leaf_elements_reject_element_children() {        use crate::wordprocessing::{paragraph, run, text};

        let validator = DocumentValidator::default();

        let good = paragraph(vec![run(vec![text("plain")])]);
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&good, &mut context).unwrap();
        assert!(context.errors().is_empty(), "{:?}", context.errors());

        let mut bad_text = text("broken");
        bad_text.append_child(OpenXmlElement::w("nested"));
        let bad = paragraph(vec![run(vec![bad_text])]);
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&bad, &mut context).unwrap();
        assert!(
            context
                .errors()
                .iter()
                .any(|e| e.id() == Some("Sch_InvalidChildinLeafElement")
                    && e.description().contains("w:t")),
            "{:?}",
            context.errors()
        );
    }

    #[test]
    fn undeclared_attributes_reported_unless_ignorable() {        use crate::wordprocessing::paragraph;

        let validator = DocumentValidator::default();

        let mut bad = paragraph(vec![]);
        bad.set_attribute_ns("foo", "urn:foo", "custom", "1");
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&bad, &mut context).unwrap();
        assert!(
            context
                .errors()
                .iter()
                .any(|e| e.id() == Some("Sch_UndeclaredAttribute")
                    && e.description().contains("foo:custom")),
            "{:?}",
            context.errors()
        );

        // Declared w:rsidR and inherited mc:Ignorable-covered foo:custom are fine.
        let mut inner = paragraph(vec![crate::wordprocessing::run(vec![
            crate::wordprocessing::text("hello"),
        ])]);
        inner.set_attribute_ns("foo", "urn:foo", "custom", "1");
        inner.set_attribute_qname("w:rsidR", "00AB12CD");
        let mut root = crate::element::OpenXmlElement::w("document");
        root.add_namespace_declaration("foo", "urn:foo");
        root.set_attribute_ns(
            "mc",
            crate::namespace::ns::MARKUP_COMPATIBILITY.uri,
            "Ignorable",
            "foo",
        );
        let mut body_el = crate::element::OpenXmlElement::w("body");
        body_el.append_child(inner);
        root.append_child(body_el);
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        validator.validate_element(&root, &mut context).unwrap();
        assert!(
            !context
                .errors()
                .iter()
                .any(|e| e.id() == Some("Sch_UndeclaredAttribute")),
            "{:?}",
            context.errors()
        );
    }

    #[test]
    fn cancellation_stops_package_validation() {
        let package = minimal_package(b"");
        let validator = DocumentValidator::default();
        let token = super::super::ValidationCancellationToken::new();
        let mut context = ValidationContext::with_cancellation_token(
            ValidationSettings::new(FileFormatVersions::OFFICE2007),
            token.clone(),
        );
        token.cancel();
        assert!(matches!(
            validator.validate_package(&package, &mut context),
            Err(crate::error::Error::Cancelled)
        ));
        assert!(context.stack().is_empty());
        assert!(context.errors().is_empty());
    }
}
