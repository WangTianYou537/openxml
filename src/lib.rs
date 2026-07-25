//! # officexml
//!
//! Rust port of the [Open XML SDK](https://github.com/OfficeDev/Open-XML-SDK).
//! Work with Microsoft Office Word (`.docx`), Excel (`.xlsx`), and PowerPoint
//! (`.pptx`) documents.
//!
//! ## Architecture
//!
//! ```text
//! packaging        Word / Excel / PowerPoint documents + parts
//! wordprocessing   WordprocessingML element helpers
//! spreadsheet      SpreadsheetML element helpers
//! presentation     PresentationML element helpers
//! element          Open XML DOM (OpenXmlElement)
//! opc              ZIP package, content types, relationships
//! ```
//!
//! ## Quick start — create a Word document
//!
//! ```no_run
//! use officexml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
//! use officexml::wordprocessing::{body, document, paragraph, run, text};
//!
//! let mut doc = WordprocessingDocument::create(
//!     "hello.docx",
//!     WordprocessingDocumentType::Document,
//! ).unwrap();
//!
//! doc.add_main_document_part().set_document(document(vec![body(vec![
//!     paragraph(vec![run(vec![text("Hello from Rust!")])]),
//! ])]));
//!
//! doc.save().unwrap();
//! ```
//!
//! ## Quick start — read paragraphs
//!
//! ```no_run
//! use officexml::packaging::WordprocessingDocument;
//!
//! let mut doc = WordprocessingDocument::open("hello.docx", false).unwrap();
//! for p in doc.paragraph_texts().unwrap() {
//!     println!("{p}");
//! }
//! ```

// Docs are filled in as the API stabilizes; avoid drowning builds in field-level noise.
#![allow(missing_docs)]

pub mod element;
pub mod error;
pub mod features;
pub mod file_format;
pub mod generated;
pub mod markup_compatibility;
pub mod namespace;
pub mod namespace_rewrite;
pub mod opc;
pub mod packaging;
pub mod presentation;
pub mod simple_types;
pub mod spreadsheet;
pub mod validation;
pub mod wordprocessing;

pub use element::{OpenXmlAttribute, OpenXmlContent, OpenXmlElement, OpenXmlQualifiedName, OpenXmlDomReader, OpenXmlElementContext, OpenXmlLoadMode, ElementEvent, ElementEventKind, OpenXmlMiscKind, OpenXmlPartReader, OpenXmlPartReaderOptions, OpenXmlPartWriter, OpenXmlPartWriterSettings, ElementState, OpenXmlUnknownMarker, XmlLineInfo, XmlPath, LAZY_STEPS, XMLNS_PREFIX, XMLNS_URI};
pub use error::{Error, Result};
pub use features::{
    AnnotationsFeature, ApplicationType, ContentTypeFeature, DataPartsFeature, DisposableFeature,
    DocumentTypeFeature, FeatureCollection, KnownDataPartFeature, LockFeature, MainPartFeature,
    PackageCapabilities, PackageEvent, PackageEventType, PackageEvents, PackageFactoryFeature,
    PackageFeature, PackageInitializerFeature, PackagePartFeature, PackageRelationshipBuilder,
    PackageStreamFeature, ParagraphIdGenerator, PartEvents, PartFactoryFeature,
    PartRelationshipsFeature, PartRootEvents, PartsFeature, PartUriFeature,
    ProgrammaticIdentifierFeature, ReferenceRelationshipsFeature, RelationshipFilterFeature,
    RootElementFeature, SaveFeature, SchemaTrackingFeature, StrictNamespaceFeature, TargetFeature,
    TypedPartFactoryFeature,
};
pub use file_format::FileFormatVersions;
pub use markup_compatibility::{
    AttributeAction, ElementAction, McContext, MarkupCompatibilityAttributes,
};
pub use namespace::{Namespace, OpenXmlNamespace};
pub use opc::{
    add_media_part, AudioReferenceRelationship, CompressionOption, CopyPartOptions, FileOpenAccess,
    PackageDiff, PackageEqualityOptions, PackageMode, compare_packages, packages_equal,
    CustomProperties, DataPart, DataPartReferenceRelationship, IdPartPair, CustomProperty,
    CustomPropertyValue, ExtendedProperties, HyperlinkRelationship, MediaKind, media_rel,
    MediaPartInfo, MediaReferenceRelationship, PackageProperties, PartExtensionProvider,
    PartUriHelper, ReferenceRelationship, RelatedPart, VideoReferenceRelationship,
};
pub use packaging::{
    add_typed_part, add_typed_part_element, default_settings, default_styles, find_typed_parts,
    footer, footer_reference, header, header_reference, hyperlink, AlternativeFormatImportType,
    DocumentSettingsPart, ExtendedPart, ImageFormat, ImagePart, MainDocumentPart,
    CompatibilityLevel, MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings, OpenSettings,
    OpenXmlPackage, PackageMiddleware, PresentationDocument, PresentationDocumentBuilder,
    PresentationDocumentType, SlideInfo, SlideLayoutInfo, SlideMasterInfo, SpreadsheetDocument,
    SpreadsheetDocumentBuilder, SpreadsheetDocumentType, StyleDefinitionsPart, TypedPart,
    PartConstraintFeature, PartConstraintRule, WordprocessingDocumentBuilder, constraints_for,
    presentation_builder, spreadsheet_builder, word_builder, WordprocessingDocument,
    WordprocessingDocumentType, WorksheetInfo,
};
