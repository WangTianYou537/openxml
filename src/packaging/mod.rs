//! High-level Open XML packaging (documents and parts).
//!
//! Mirrors `DocumentFormat.OpenXml.Packaging`.

mod open_xml_package;
mod open_xml_part;
mod parts;
mod presentation_document;
mod spreadsheet_document;
mod wordprocessing_document;

pub use open_xml_package::{
    MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings, OpenSettings,
    OpenXmlPackage,
};
pub use open_xml_part::{MainDocumentPart, OpenXmlPart};
pub use parts::{
    default_settings, default_styles, footer, footer_reference, header, header_reference,
    hyperlink, hyperlink_anchor, AlternativeFormatImportType, DocumentSettingsPart, ImageFormat,
    ImagePart, StyleDefinitionsPart,
};
pub use presentation_document::{
    PresentationDocument, PresentationDocumentType, SlideInfo, SlideLayoutInfo, SlideMasterInfo,
};
pub use spreadsheet_document::{
    SpreadsheetDocument, SpreadsheetDocumentType, WorksheetInfo,
};
pub use wordprocessing_document::{WordprocessingDocument, WordprocessingDocumentType};
