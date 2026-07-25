//! Unified validation entry point (C# `OpenXmlValidator` shell).
//!
//! Composes package structure, part-constraint, and DOM particle/attribute checks
//! with a max-error budget and target [`FileFormatVersions`].

use super::{
    validate_alternate_content, validate_package, validate_package_constraints,
    validate_word_document, validate_word_document_full, ValidationError,
};
use crate::element::OpenXmlElement;
use crate::error::{Error, Result};
use crate::file_format::FileFormatVersions;
use crate::opc::OpcPackage;
use crate::packaging::{
    PresentationDocument, SpreadsheetDocument, WordprocessingDocument,
};

/// Settings / facade for package and element validation (C# `OpenXmlValidator`).
#[derive(Debug, Clone)]
pub struct OpenXmlValidator {
    file_format: FileFormatVersions,
    /// Maximum errors to return. `0` means no limit (C# default is 1000; `0` = unlimited).
    max_number_of_errors: usize,
}

impl Default for OpenXmlValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenXmlValidator {
    /// Defaults to [`FileFormatVersions::OFFICE2007`] and max 1000 errors.
    pub fn new() -> Self {
        Self {
            file_format: FileFormatVersions::OFFICE2007,
            max_number_of_errors: 1000,
        }
    }

    pub fn with_file_format(file_format: FileFormatVersions) -> Self {
        Self {
            file_format,
            max_number_of_errors: 1000,
        }
    }

    pub fn file_format(&self) -> FileFormatVersions {
        self.file_format
    }

    pub fn max_number_of_errors(&self) -> usize {
        self.max_number_of_errors
    }

    /// Set max errors (`0` = unlimited). Panics if called with negative — use `usize`.
    pub fn set_max_number_of_errors(&mut self, value: usize) {
        self.max_number_of_errors = value;
    }

    pub fn with_max_number_of_errors(mut self, value: usize) -> Self {
        self.max_number_of_errors = value;
        self
    }

    fn cap(&self, mut errors: Vec<ValidationError>) -> Vec<ValidationError> {
        if self.max_number_of_errors > 0 && errors.len() > self.max_number_of_errors {
            errors.truncate(self.max_number_of_errors);
        }
        errors
    }

    /// Validate OPC structure + part constraints (no full DOM schema pass).
    pub fn validate_package(&self, package: &OpcPackage) -> Vec<ValidationError> {
        let mut errors = validate_package(package, true);
        let _ = self.file_format;
        self.cap(errors)
    }

    /// Part-constraint walk only (C# `PackageValidator` subset).
    pub fn validate_package_constraints_only(
        &self,
        package: &OpcPackage,
    ) -> Vec<ValidationError> {
        self.cap(validate_package_constraints(package))
    }

    /// Validate a WordprocessingML document element tree (rejects misc/unknown roots).
    pub fn validate_element(&self, element: &OpenXmlElement) -> Result<Vec<ValidationError>> {
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
            validate_word_document_full(element)
        } else {
            validate_word_document(element)
        };
        errors.extend(validate_alternate_content(element));
        let _ = self.file_format;
        Ok(self.cap(errors))
    }

    /// Validate a [`WordprocessingDocument`]: package + main DOM full rules.
    pub fn validate_word(&self, doc: &mut WordprocessingDocument) -> Result<Vec<ValidationError>> {
        let mut errors = doc.validate_package()?;
        errors.extend(doc.validate_full()?);
        Ok(self.cap(errors))
    }

    /// Validate a [`SpreadsheetDocument`]: package structure (+ constraints).
    pub fn validate_spreadsheet(
        &self,
        doc: &SpreadsheetDocument,
    ) -> Result<Vec<ValidationError>> {
        Ok(self.cap(doc.validate_package()?))
    }

    /// Validate a [`PresentationDocument`]: package structure (+ constraints).
    pub fn validate_presentation(
        &self,
        doc: &PresentationDocument,
    ) -> Result<Vec<ValidationError>> {
        Ok(self.cap(doc.validate_package()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;
    use crate::namespace::content_type;
    use crate::namespace::rel;
    use crate::opc::{PackUri, RelationshipTargetMode};
    use crate::wordprocessing::{body, document, paragraph, run, text};

    #[test]
    fn rejects_unknown_element() {
        let v = OpenXmlValidator::new();
        let el = OpenXmlElement::unknown("ex", "foo", "urn:x");
        assert!(v.validate_element(&el).is_err());
    }

    #[test]
    fn rejects_misc() {
        let v = OpenXmlValidator::new();
        assert!(v.validate_element(&OpenXmlElement::comment("x")).is_err());
    }

    #[test]
    fn validates_word_document_ok() {
        let v = OpenXmlValidator::new().with_max_number_of_errors(10);
        let doc = document(vec![body(vec![paragraph(vec![run(vec![text("hi")])])])]);
        let errs = v.validate_element(&doc).unwrap();
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn caps_errors() {
        let v = OpenXmlValidator::new().with_max_number_of_errors(1);
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
        let errs = OpenXmlValidator::new().validate_package(&pkg);
        assert!(errs.is_empty(), "{errs:?}");
    }
}
