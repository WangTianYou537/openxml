//! Unified validation entry point (C# `OpenXmlValidator` shell).
//!
//! Composes package structure, part-constraint, and DOM particle/attribute checks
//! with a max-error budget and target [`FileFormatVersions`].

use super::{
    validate_alternate_content, validate_mc_attributes, validate_package, validate_package_constraints,
    validate_word_document_for_version, validate_word_document_full_for_version, ValidationCache,
    ValidationError, ValidationErrorEventArgs,
};
use crate::element::OpenXmlElement;
use crate::error::{Error, Result};
use crate::file_format::FileFormatVersions;
use crate::opc::OpcPackage;
use crate::packaging::{
    PresentationDocument, SpreadsheetDocument, WordprocessingDocument,
};

/// Settings for validation (C# `ValidationSettings`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationSettings {
    /// Target file format for version-aware rules.
    pub file_format: FileFormatVersions,
    /// Maximum errors to return. `0` means no limit (C# default is 1000).
    pub max_number_of_errors: usize,
}

impl ValidationSettings {
    pub const DEFAULT_MAX_ERRORS: usize = 1000;

    pub fn new(file_format: FileFormatVersions) -> Self {
        // C# ValidationSettings ctor throws if format unsupported; we ignore empty quietly
        // but callers can use ensure_supported explicitly.
        let _ = file_format.ensure_supported();
        Self {
            file_format,
            max_number_of_errors: Self::DEFAULT_MAX_ERRORS,
        }
    }

    pub fn with_max_number_of_errors(mut self, value: usize) -> Self {
        self.max_number_of_errors = value;
        self
    }
}

impl Default for ValidationSettings {
    fn default() -> Self {
        Self::new(FileFormatVersions::OFFICE2007)
    }
}

/// Settings / facade for package and element validation (C# `OpenXmlValidator`).
pub struct OpenXmlValidator {
    settings: ValidationSettings,
    /// Version-scoped particle/memo cache (C# `ValidationCache`).
    cache: ValidationCache,
    /// Optional per-error callback retained for compatibility.
    error_callback: Option<Box<dyn FnMut(&ValidationError) + Send>>,
    /// Mutable per-error callback (C# `ValidationErrorEventArgs` subscriber).
    error_event_callback: Option<Box<dyn FnMut(&mut ValidationErrorEventArgs) + Send>>,
}

impl std::fmt::Debug for OpenXmlValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenXmlValidator")
            .field("file_format", &self.settings.file_format)
            .field("max_number_of_errors", &self.settings.max_number_of_errors)
            .field("has_error_callback", &self.error_callback.is_some())
            .field("has_error_event_callback", &self.error_event_callback.is_some())
            .field("cache_version", &self.cache.version())
            .finish()
    }
}

impl Default for OpenXmlValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenXmlValidator {
    /// Defaults to [`FileFormatVersions::OFFICE2007`] and max 1000 errors.
    pub fn new() -> Self {
        let settings = ValidationSettings::default();
        Self {
            cache: ValidationCache::new(settings.file_format),
            settings,
            error_callback: None,
            error_event_callback: None,
        }
    }

    pub fn with_file_format(file_format: FileFormatVersions) -> Self {
        Self {
            settings: ValidationSettings::new(file_format),
            cache: ValidationCache::new(file_format),
            error_callback: None,
            error_event_callback: None,
        }
    }

    pub fn with_settings(settings: ValidationSettings) -> Self {
        let cache = ValidationCache::new(settings.file_format);
        Self {
            settings,
            cache,
            error_callback: None,
            error_event_callback: None,
        }
    }

    pub fn settings(&self) -> ValidationSettings {
        self.settings
    }

    pub fn cache(&self) -> &ValidationCache {
        &self.cache
    }

    pub fn cache_mut(&mut self) -> &mut ValidationCache {
        &mut self.cache
    }

    pub fn set_settings(&mut self, settings: ValidationSettings) {
        self.cache.set_version(settings.file_format);
        self.settings = settings;
    }

    pub fn file_format(&self) -> FileFormatVersions {
        self.settings.file_format
    }

    pub fn set_file_format(&mut self, file_format: FileFormatVersions) {
        self.settings.file_format = file_format;
        self.cache.set_version(file_format);
    }

    pub fn max_number_of_errors(&self) -> usize {
        self.settings.max_number_of_errors
    }

    /// Set max errors (`0` = unlimited).
    pub fn set_max_number_of_errors(&mut self, value: usize) {
        self.settings.max_number_of_errors = value;
    }

    pub fn with_max_number_of_errors(mut self, value: usize) -> Self {
        self.settings.max_number_of_errors = value;
        self
    }

    /// Register a callback invoked for each validation error before the list is returned.
    pub fn on_validation_error<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&ValidationError) + Send + 'static,
    {
        self.error_callback = Some(Box::new(callback));
        self
    }

    /// Register a mutable validation-error callback (C# `ValidationErrorEventArgs` event).
    pub fn on_validation_error_event<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&mut ValidationErrorEventArgs) + Send + 'static,
    {
        self.error_event_callback = Some(Box::new(callback));
        self
    }

    /// Clear all validation-error callbacks.
    pub fn clear_validation_error_callback(&mut self) {
        self.error_callback = None;
        self.error_event_callback = None;
    }

    fn cap(&mut self, mut errors: Vec<ValidationError>) -> Vec<ValidationError> {
        let max = self.settings.max_number_of_errors;
        if max > 0 && errors.len() > max {
            errors.truncate(max);
        }
        for error in &mut errors {
            if let Some(cb) = self.error_event_callback.as_mut() {
                let mut args = ValidationErrorEventArgs::new(error.clone());
                cb(&mut args);
                *error = args.validation_error;
            }
            if let Some(cb) = self.error_callback.as_mut() {
                cb(error);
            }
        }
        errors
    }


    /// Validate OPC structure + part constraints (no full DOM schema pass).
    pub fn validate_package(&mut self, package: &OpcPackage) -> Vec<ValidationError> {
        let errors = validate_package(package, true);
        let _ = self.settings.file_format;
        self.cap(errors)
    }

    /// Part-constraint walk only (C# `PackageValidator` subset).
    pub fn validate_package_constraints_only(
        &mut self,
        package: &OpcPackage,
    ) -> Vec<ValidationError> {
        self.cap(validate_package_constraints(package))
    }

    /// Typed package constraint results (C# `PackageValidator.Validate` results shell).
    pub fn validate_package_constraint_results(
        &mut self,
        package: &OpcPackage,
    ) -> Vec<super::OpenXmlPackageValidationResult> {
        let errors = self.validate_package_constraints_only(package);
        errors
            .iter()
            .map(super::OpenXmlPackageValidationResult::from_validation_error)
            .collect()
    }

    /// Validate a WordprocessingML document element tree (rejects misc/unknown roots).
    pub fn validate_element(&mut self, element: &OpenXmlElement) -> Result<Vec<ValidationError>> {
        if element.is_misc_node() {
            return Err(Error::Validation(
                "OpenXmlValidator cannot validate OpenXmlMiscNode".into(),
            ));
        }
        if element.is_unknown() {
            return Err(Error::Validation(
                "OpenXmlValidator cannot validate OpenXmlUnknownElement".into(),
            ));
        }
        if element.local_name == "AlternateContent"
            || element.local_name == "Choice"
            || element.local_name == "Fallback"
        {
            return Err(Error::Validation(
                "OpenXmlValidator cannot validate AlternateContent nodes directly".into(),
            ));
        }

        let mut errors = if element.local_name == "document" {
            validate_word_document_full_for_version(element, self.settings.file_format)
        } else {
            validate_word_document_for_version(element, self.settings.file_format)
        };
        errors.extend(validate_alternate_content(element));
        errors.extend(validate_mc_attributes(element));
        Ok(self.cap(errors))
    }

    /// Validate a [`WordprocessingDocument`]: package + main DOM full rules.
    pub fn validate_word(&mut self, doc: &mut WordprocessingDocument) -> Result<Vec<ValidationError>> {
        let mut errors = doc.validate_package()?;
        errors.extend(doc.validate_full()?);
        Ok(self.cap(errors))
    }

    /// Validate a [`SpreadsheetDocument`]: package structure (+ constraints).
    pub fn validate_spreadsheet(
        &mut self,
        doc: &SpreadsheetDocument,
    ) -> Result<Vec<ValidationError>> {
        Ok(self.cap(doc.validate_package()?))
    }

    /// Validate a [`PresentationDocument`]: package structure (+ constraints).
    pub fn validate_presentation(
        &mut self,
        doc: &PresentationDocument,
    ) -> Result<Vec<ValidationError>> {
        Ok(self.cap(doc.validate_package()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opc::OpcPackage;
    use crate::element::OpenXmlElement;
    use crate::namespace::content_type;
    use crate::namespace::rel;
    use crate::opc::{PackUri, RelationshipTargetMode};
    use crate::wordprocessing::{body, document, paragraph, run, text};

    #[test]
    fn rejects_unknown_element() {
        let mut v = OpenXmlValidator::new();
        let el = OpenXmlElement::unknown("ex", "foo", "urn:x");
        assert!(v.validate_element(&el).is_err());
    }

    #[test]
    fn rejects_misc() {
        let mut v = OpenXmlValidator::new();
        assert!(v.validate_element(&OpenXmlElement::comment("x")).is_err());
    }

    #[test]
    fn validates_word_document_ok() {
        let mut v = OpenXmlValidator::new().with_max_number_of_errors(10);
        let doc = document(vec![body(vec![paragraph(vec![run(vec![text("hi")])])])]);
        let errs = v.validate_element(&doc).unwrap();
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn caps_errors() {
        let mut v = OpenXmlValidator::new().with_max_number_of_errors(1);
        // Missing body → at least one error; dual body → more.
        let doc = document(vec![]);
        let errs = v.validate_element(&doc).unwrap();
        assert!(errs.len() <= 1);
    }

    #[test]
    fn package_minimal_ok() {
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
        let errs = OpenXmlValidator::new().validate_package(&pkg); // new() is mut via temporary
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn validator_uses_target_version_for_alternate_content() {
        use crate::markup_compatibility::alternate_content_with;

        let alternate = alternate_content_with(
            "w14",
            vec![OpenXmlElement::w("notBody")],
            vec![body(vec![paragraph(vec![run(vec![text("fallback")])])])],
        )
        .with_ns_decl("w14", "http://schemas.microsoft.com/office/word/2010/wordml");
        let document = document(vec![alternate]);
        let office_2007 = OpenXmlValidator::with_file_format(FileFormatVersions::OFFICE2007)
            .validate_element(&document)
            .unwrap();
        assert!(office_2007.is_empty(), "{office_2007:?}");

        let office_2010 = OpenXmlValidator::with_file_format(FileFormatVersions::OFFICE2010)
            .validate_element(&document)
            .unwrap();
        assert!(!office_2010.is_empty());
        assert!(office_2010.iter().any(|error| {
            error.message.contains("notBody") || error.message.contains("required particle")
        }));
    }

    #[test]
    fn validation_error_callback_fires() {
        use std::sync::{Arc, Mutex};
        let seen = Arc::new(Mutex::new(0usize));
        let seen2 = Arc::clone(&seen);
        let mut v = OpenXmlValidator::new()
            .with_max_number_of_errors(10)
            .on_validation_error(move |_e| {
                *seen2.lock().unwrap() += 1;
            });
        let doc = document(vec![]);
        let _ = v.validate_element(&doc).unwrap();
        assert!(*seen.lock().unwrap() >= 1);
    }

    #[test]
    fn validation_error_event_can_replace_error() {
        use std::sync::{Arc, Mutex};
        let observed_path = Arc::new(Mutex::new(String::new()));
        let observed_path2 = Arc::clone(&observed_path);
        let mut v = OpenXmlValidator::new()
            .on_validation_error_event(|args| {
                args.set_validation_error(
                    ValidationError::with_id("/replacement", "Sem_Replaced", "replaced")
                        .with_error_type(crate::validation::ValidationErrorType::Semantic),
                );
            })
            .on_validation_error(move |error| {
                *observed_path2.lock().unwrap() = error.path.clone();
            });
        let errors = v.validate_element(&document(vec![])).unwrap();
        assert!(!errors.is_empty());
        assert!(errors.iter().all(|error| error.path == "/replacement"));
        assert!(errors.iter().all(|error| {
            error.error_type() == crate::validation::ValidationErrorType::Semantic
        }));
        assert_eq!(*observed_path.lock().unwrap(), "/replacement");

        v.clear_validation_error_callback();
        let errors = v.validate_element(&document(vec![])).unwrap();
        assert!(errors.iter().all(|error| error.path != "/replacement"));
    }

    #[test]
    fn error_id_and_type_from_message() {
        let e = ValidationError {
            path: "root".into(),
            message: "MC_ShallContainChoice: missing Choice".into(),
            ..Default::default()
        };
        assert_eq!(e.id(), Some("MC_ShallContainChoice"));
        assert_eq!(e.description(), "missing Choice");
        assert_eq!(
            e.error_type(),
            crate::validation::ValidationErrorType::MarkupCompatibility
        );

        let e = ValidationError {
            path: "/word".into(),
            message: "PartIsNotAllowed: foo".into(),
            ..Default::default()
        };
        assert_eq!(e.id(), Some("PartIsNotAllowed"));
        assert_eq!(e.error_type(), crate::validation::ValidationErrorType::Package);
        assert_eq!(e.xml_path().part_uri.as_deref(), Some("/word"));

        let e2 = ValidationError::with_id("/w:p[1]", "Sch_InvalidElementContent", "bad child");
        assert_eq!(e2.id(), Some("Sch_InvalidElementContent"));
        assert_eq!(e2.description(), "bad child");
        assert!(e2.xml_path().xpath.contains("w:p"), "{:?}", e2.xml_path());

        let mut explicit = ValidationError::with_id("/a", "Sch_Invalid", "bad")
            .with_error_type(crate::validation::ValidationErrorType::Semantic);
        assert_eq!(explicit.error_type(), crate::validation::ValidationErrorType::Semantic);
        explicit.clear_error_type();
        assert_eq!(explicit.error_type(), crate::validation::ValidationErrorType::Schema);
        explicit.set_error_type(crate::validation::ValidationErrorType::Package);
        assert_eq!(explicit.error_type(), crate::validation::ValidationErrorType::Package);
    }

    #[test]
    fn validation_settings_roundtrip() {
        let s = ValidationSettings::new(FileFormatVersions::OFFICE2016)
            .with_max_number_of_errors(42);
        let mut v = OpenXmlValidator::with_settings(s);
        assert_eq!(v.file_format(), FileFormatVersions::OFFICE2016);
        assert_eq!(v.max_number_of_errors(), 42);
        v.set_file_format(FileFormatVersions::OFFICE2010);
        assert_eq!(v.settings().file_format, FileFormatVersions::OFFICE2010);
        assert_eq!(v.cache().version(), FileFormatVersions::OFFICE2010);
    }

    #[test]
    fn package_constraint_results_empty_package() {
        let mut v = OpenXmlValidator::new();
        let pkg = OpcPackage::create();
        let results = v.validate_package_constraint_results(&pkg);
        assert!(results.is_empty());
    }
}
