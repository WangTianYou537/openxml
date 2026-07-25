//! Error types for the Open XML SDK.

use std::io;
use std::path::PathBuf;

/// Result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors produced by Open XML operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("ZIP package error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML error: {0}")]
    Xml(String),

    #[error("package error: {0}")]
    Package(String),

    #[error("part not found: {0}")]
    PartNotFound(String),

    #[error("relationship not found: {0}")]
    RelationshipNotFound(String),

    #[error("invalid content type: expected {expected}, got {actual}")]
    InvalidContentType { expected: String, actual: String },

    #[error("invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("document is closed")]
    Closed,

    #[error("part has no root element loaded")]
    NoRootElement,

    #[error("encrypted Office package is not supported")]
    EncryptedPackage,

    #[error("part exceeds MaxCharactersInPart limit ({limit}): {uri}")]
    PartTooLarge { uri: String, limit: u64 },

    #[error("validation error: {0}")]
    Validation(String),

    /// Markup Compatibility content error (C# `InvalidMCContentException`).
    #[error("invalid markup compatibility content: {0}")]
    InvalidMcContent(String),

    /// MustUnderstand / unknown namespace (C# `NamespaceNotUnderstandException`).
    #[error("namespace not understood: {0}")]
    NamespaceNotUnderstand(String),

    #[error("{0}")]
    Other(String),
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Error::Xml(value.to_string())
    }
}

impl From<quick_xml::events::attributes::AttrError> for Error {
    fn from(value: quick_xml::events::attributes::AttrError) -> Self {
        Error::Xml(value.to_string())
    }
}


/// C# `OpenXmlPackageException` — packaging errors that map to [`Error::Package`].
///
/// Prefer constructing [`Error::Package`] directly; this helper centralizes the
/// well-known C# `ExceptionMessages` strings used by the managed SDK.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenXmlPackageException {
    pub message: String,
}

impl OpenXmlPackageException {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn same_part_different_relationship_type() -> Self {
        Self::new("SamePartWithDifferentRelationshipType")
    }

    pub fn foreign_data_part() -> Self {
        Self::new("ForeignDataPart")
    }

    pub fn data_part_is_in_use() -> Self {
        Self::new("DataPartIsInUse")
    }

    pub fn cannot_change_document_type() -> Self {
        Self::new("CannotChangeDocumentType")
    }

    pub fn part_is_not_allowed() -> Self {
        Self::new("PartIsNotAllowed")
    }

    pub fn only_one_part_allowed() -> Self {
        Self::new("OnlyOnePartAllowed")
    }

    pub fn error_content_type() -> Self {
        Self::new("ErrorContentType")
    }

    pub fn invalid_main_part_content_type() -> Self {
        Self::new("InvalidMainPartContentType")
    }

    pub fn invalid_package_type() -> Self {
        Self::new("InvalidPackageType")
    }

    pub fn extended_part_is_open_xml_part() -> Self {
        Self::new("ExtendedPartIsOpenXmlPart")
    }

    pub fn extended_part_not_allowed() -> Self {
        Self::new("ExtendedPartNotAllowed")
    }

    pub fn foreign_open_xml_part() -> Self {
        Self::new("ForeignOpenXmlPart")
    }

    pub fn foreign_media_data_part() -> Self {
        Self::new("ForeignMediaDataPart")
    }

    pub fn multiple_relationships_to_same_part() -> Self {
        Self::new("MultipleRelationshipsToSamePart")
    }

    pub fn part_exists_with_different_relationship_id() -> Self {
        Self::new("PartExistsWithDifferentRelationshipId")
    }

    pub fn malformed_uri() -> Self {
        Self::new("MalformedUri")
    }

    pub fn failed_to_open_package() -> Self {
        Self::new("FailedToOpenPackage")
    }

    pub fn package_access_mode_is_readonly() -> Self {
        Self::new("PackageAccessModeIsReadonly")
    }

    pub fn into_error(self) -> Error {
        Error::Package(self.message)
    }
}

impl From<OpenXmlPackageException> for Error {
    fn from(value: OpenXmlPackageException) -> Self {
        value.into_error()
    }
}

impl std::fmt::Display for OpenXmlPackageException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OpenXmlPackageException {}
