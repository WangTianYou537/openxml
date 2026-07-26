//! High-level Open XML packaging (documents and parts).
//!
//! Mirrors `DocumentFormat.OpenXml.Packaging`.

mod builder;
mod open_xml_package;
mod open_xml_part;
mod parts;
mod presentation_document;
mod spreadsheet_document;
mod part_constraints;
mod typed_part;
mod wordprocessing_document;

pub use builder::{
    presentation as presentation_builder, spreadsheet as spreadsheet_builder, word as word_builder,
    PackageMiddleware, PresentationDocumentBuilder, SpreadsheetDocumentBuilder,
    WordprocessingDocumentBuilder,
};
pub use open_xml_package::{
    CompatibilityLevel, MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings,
    OpenSettings, OpenXmlPackage,
};
pub use open_xml_part::{ExtendedPart, MainDocumentPart, OpenXmlPart, PartTypeInfo};
pub use parts::{
    default_settings, default_styles, footer, footer_reference, header, header_reference,
    hyperlink, hyperlink_anchor, AlternativeFormatImportType, DocumentSettingsPart, ImageFormat,
    ImagePart, StyleDefinitionsPart,
};
pub use presentation_document::{
    PresentationDocument, PresentationDocumentType, SlideInfo, SlideLayoutInfo, SlideMasterInfo,
    SvgFontEmbedMode, SvgShapesOnSlideOptions,
};
pub use spreadsheet_document::{
    SpreadsheetDocument, SpreadsheetDocumentType, WorksheetInfo,
};
pub use part_constraints::{
    constraints_for, relationship_introduced_in, PartConstraintFeature, PartConstraintRule,
};
pub use typed_part::{
    add_typed_part, add_typed_part_element, delete_typed_part_by_id, find_typed_parts,
    find_typed_parts_recursive, open_typed_part, part_info_for_relationship, TypedPart,
};
pub use wordprocessing_document::{WordprocessingDocument, WordprocessingDocumentType};
