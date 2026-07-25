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
