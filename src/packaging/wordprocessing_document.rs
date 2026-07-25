//! WordprocessingDocument — Word (.docx) package.

use super::open_xml_package::{OpenSettings, OpenXmlPackage};
use super::open_xml_part::MainDocumentPart;
use super::parts::{
    footer, footer_reference, header, header_reference, hyperlink, AlternativeFormatImportType,
    ImageFormat, ImagePart,
};
use crate::element::{parse_element, OpenXmlElement};
use crate::error::{Error, Result};
use crate::namespace::{content_type, rel};
use crate::opc::{
    from_flat_opc, progid, to_flat_opc, CustomProperties, ExtendedProperties, OpcPackage,
    PackageMode, PackageProperties, PackUri, RelationshipTargetMode,
};
use crate::wordprocessing::{
    accept_revisions, alt_chunk, body, comments, default_endnotes_with, default_font_table,
    default_footnotes_with, default_numbering, default_theme, default_web_settings, document,
    document_protection, endnote_ref_run, footnote_ref_run, glossary_document, paragraph,
    reject_revisions, replace_text, run, text,
};
use crate::validation::{
    validate_word_document, validate_word_document_full, ValidationError,
};
use std::path::Path;

/// Type of WordprocessingML document package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WordprocessingDocumentType {
    #[default]
    Document,
    Template,
    MacroEnabledDocument,
    MacroEnabledTemplate,
}

impl WordprocessingDocumentType {
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Document => content_type::WORD_DOCUMENT,
            Self::Template => content_type::WORD_TEMPLATE,
            Self::MacroEnabledDocument => {
                "application/vnd.ms-word.document.macroEnabled.main+xml"
            }
            Self::MacroEnabledTemplate => {
                "application/vnd.ms-word.template.macroEnabledTemplate.main+xml"
            }
        }
    }

    pub fn from_content_type(ct: &str) -> Option<Self> {
        match ct {
            content_type::WORD_DOCUMENT => Some(Self::Document),
            content_type::WORD_TEMPLATE => Some(Self::Template),
            "application/vnd.ms-word.document.macroEnabled.main+xml" => {
                Some(Self::MacroEnabledDocument)
            }
            "application/vnd.ms-word.template.macroEnabledTemplate.main+xml" => {
                Some(Self::MacroEnabledTemplate)
            }
            _ => None,
        }
    }
}

/// An Open XML Wordprocessing document (`.docx` / `.dotx`).
///
/// # Example
///
/// ```no_run
/// use officexml::packaging::{WordprocessingDocument, WordprocessingDocumentType};
/// use officexml::wordprocessing::{document, body, paragraph, run, text};
///
/// let mut doc = WordprocessingDocument::create(
///     "hello.docx",
///     WordprocessingDocumentType::Document,
/// ).unwrap();
///
/// let main = doc.add_main_document_part();
/// main.set_document(
///     document(vec![
///         body(vec![
///             paragraph(vec![
///                 run(vec![text("Hello, world!")]),
///             ]),
///         ]),
///     ]),
/// );
/// doc.save().unwrap();
/// ```
#[derive(Debug)]
pub struct WordprocessingDocument {
    package: OpenXmlPackage,
    document_type: WordprocessingDocumentType,
    main_document_part: Option<MainDocumentPart>,
}

impl WordprocessingDocument {
    /// Create a new Word document at `path`.
    pub fn create(
        path: impl AsRef<Path>,
        document_type: WordprocessingDocumentType,
    ) -> Result<Self> {
        Self::create_with_settings(path, document_type, OpenSettings::default())
    }

    pub fn create_with_settings(
        path: impl AsRef<Path>,
        document_type: WordprocessingDocumentType,
        settings: OpenSettings,
    ) -> Result<Self> {
        let opc = OpcPackage::create_file(path.as_ref());
        let mut package = OpenXmlPackage::from_opc(opc, settings);
        package.set_application_type(crate::features::ApplicationType::WORD);
        package.set_package_factory_feature("WordprocessingDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "WordprocessingDocument",
        ));
        Ok(Self {
            package,
            document_type,
            main_document_part: None,
        })
    }

    /// Create an in-memory Word document (save with [`save_as`](Self::save_as) or [`to_bytes`](Self::to_bytes)).
    pub fn create_in_memory(document_type: WordprocessingDocumentType) -> Result<Self> {
        let opc = OpcPackage::create();
        let mut package = OpenXmlPackage::from_opc(opc, OpenSettings::default());
        package.set_application_type(crate::features::ApplicationType::WORD);
        package.set_package_factory_feature("WordprocessingDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "WordprocessingDocument",
        ));
        Ok(Self {
            package,
            document_type,
            main_document_part: None,
        })
    }

    /// Create a document by copying an existing package (template).
    ///
    /// Opens `template_path`, clones all parts into a new in-memory package, and
    /// optionally switches the main part content type to `document_type`.
    /// When `document_type` is `None`, the template's document type is kept.
    pub fn create_from_template(
        template_path: impl AsRef<Path>,
        document_type: Option<WordprocessingDocumentType>,
    ) -> Result<Self> {
        let mut src = Self::open(template_path, false)?;
        let mut cloned = src.clone_document()?;
        if let Some(dt) = document_type {
            cloned.change_document_type(dt)?;
        }
        Ok(cloned)
    }

    /// Change the main document content type (e.g. `.dotx` → `.docx`).
    pub fn change_document_type(&mut self, new_type: WordprocessingDocumentType) -> Result<()> {
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let uri = main.part().uri.clone();
        let ct = new_type.content_type();
        // Re-apply content type override while preserving bytes
        let data = self
            .package
            .opc()
            .get_part(&uri)
            .map(|b| b.to_vec())
            .unwrap_or_default();
        self.package
            .opc_mut()
            .set_part(uri, ct, data);
        main.part_mut().content_type = ct.to_string();
        self.document_type = new_type;
        Ok(())
    }

    /// Open an existing Word document.
    pub fn open(path: impl AsRef<Path>, is_editable: bool) -> Result<Self> {
        Self::open_with_settings(path, is_editable, OpenSettings::default())
    }

    pub fn open_with_settings(
        path: impl AsRef<Path>,
        is_editable: bool,
        mut settings: OpenSettings,
    ) -> Result<Self> {
        if !is_editable {
            settings.auto_save = false;
        }
        let opc = OpcPackage::open(path)?;
        Self::from_opc(opc, settings)
    }

    /// Open from raw package bytes.
    pub fn open_bytes(data: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = data.as_ref().to_vec();
        let opc = OpcPackage::open_bytes(&bytes)?;
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        let mut doc = Self::from_opc(opc, settings)?;
        doc.package_mut().set_package_stream_bytes(bytes);
        Ok(doc)
    }

    /// Open a Word package from any `Read + Seek` stream (C# `Open(Stream, …)`).
    pub fn open_stream<R: std::io::Read + std::io::Seek>(
        reader: R,
        is_editable: bool,
    ) -> Result<Self> {
        Self::open_stream_with_settings(reader, is_editable, OpenSettings::default())
    }

    /// Open from a stream with custom [`OpenSettings`].
    pub fn open_stream_with_settings<R: std::io::Read + std::io::Seek>(
        reader: R,
        is_editable: bool,
        mut settings: OpenSettings,
    ) -> Result<Self> {
        if !is_editable {
            settings.auto_save = false;
        }
        let opc = OpcPackage::open_reader(reader)?;
        Self::from_opc(opc, settings)
    }

    /// Write the package ZIP to a stream (C# stream save).
    pub fn write_to<W: std::io::Write>(&mut self, writer: W) -> Result<()> {
        self.flush_parts()?;
        self.package.write_to(writer)
    }


    fn from_opc(opc: OpcPackage, settings: OpenSettings) -> Result<Self> {
        let mut package = OpenXmlPackage::from_opc(opc, settings);
        package.set_application_type(crate::features::ApplicationType::WORD);
        package.set_package_factory_feature("WordprocessingDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "WordprocessingDocument",
        ));
        let main_uri = package.opc().main_part_uri(rel::OFFICE_DOCUMENT).ok();
        let (document_type, main_document_part) = if let Some(uri) = main_uri {
            let ct = package
                .opc()
                .content_types()
                .content_type_for(uri.as_str())
                .unwrap_or(content_type::WORD_DOCUMENT)
                .to_string();
            let doc_type =
                WordprocessingDocumentType::from_content_type(&ct).unwrap_or_default();
            package.set_main_part_feature(crate::features::MainPartFeature::new(
                rel::OFFICE_DOCUMENT,
                ct.clone(),
                Some(uri.as_str().to_string()),
            ));
            let mut part = MainDocumentPart::new(ct);
            // Ensure URI matches actual package part
            if uri.as_str() != MainDocumentPart::URI {
                part.part_mut().uri = uri;
            }
            (doc_type, Some(part))
        } else {
            (WordprocessingDocumentType::Document, None)
        };

        Ok(Self {
            package,
            document_type,
            main_document_part,
        })
    }

    pub fn document_type(&self) -> WordprocessingDocumentType {
        self.document_type
    }

    pub fn package(&self) -> &OpenXmlPackage {
        &self.package
    }

    pub fn package_mut(&mut self) -> &mut OpenXmlPackage {
        &mut self.package
    }


    /// Alias for [`open_bytes`](Self::open_bytes).
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Result<Self> {
        Self::open_bytes(data)
    }

    /// Get the main document part, if present.
    pub fn main_document_part(&self) -> Option<&MainDocumentPart> {
        self.main_document_part.as_ref()
    }

    pub fn main_document_part_mut(&mut self) -> Option<&mut MainDocumentPart> {
        self.main_document_part.as_mut()
    }

    /// Create and add the main document part.
    pub fn add_main_document_part(&mut self) -> &mut MainDocumentPart {
        let ct = self.document_type.content_type().to_string();
        let part = MainDocumentPart::new(ct.clone());

        // Register content type + package relationship
        self.package.set_part(
            MainDocumentPart::URI,
            ct,
            // Minimal empty document shell; caller typically sets document immediately.
            br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body/>
</w:document>"#.to_vec(),
        );
        self.package.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &PackUri::new(MainDocumentPart::URI),
            RelationshipTargetMode::Internal,
        );

        self.main_document_part = Some(part);
        self.main_document_part.as_mut().unwrap()
    }

    /// Convenience: create a simple document with one paragraph of text.
    pub fn create_simple(
        path: impl AsRef<Path>,
        text_content: &str,
    ) -> Result<Self> {
        let mut doc = Self::create(path, WordprocessingDocumentType::Document)?;
        let main = doc.add_main_document_part();
        main.set_document(document(vec![body(vec![paragraph(vec![run(vec![
            text(text_content),
        ])])])]));
        Ok(doc)
    }

    /// Read package core properties (`docProps/core.xml`).
    pub fn package_properties(&self) -> Result<PackageProperties> {
        PackageProperties::load_from(self.package.opc())
    }

    /// Write package core properties.
    pub fn set_package_properties(&mut self, props: &PackageProperties) -> Result<()> {
        props.save_to(self.package.opc_mut())
    }

    /// Read extended properties (`docProps/app.xml`).
    pub fn extended_properties(&self) -> Result<ExtendedProperties> {
        ExtendedProperties::load_from(self.package.opc())
    }

    /// Write extended properties (`docProps/app.xml`).
    pub fn set_extended_properties(&mut self, props: &ExtendedProperties) -> Result<()> {
        props.save_to(self.package.opc_mut())
    }

    /// Read custom properties (`docProps/custom.xml`).
    pub fn custom_properties(&self) -> Result<CustomProperties> {
        CustomProperties::load_from(self.package.opc())
    }

    /// Write custom properties (`docProps/custom.xml`).
    pub fn set_custom_properties(&mut self, props: &CustomProperties) -> Result<()> {
        props.save_to(self.package.opc_mut())
    }

    /// Whether a core properties part exists.
    pub fn has_package_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docProps/core.xml"))
    }

    /// Whether an extended properties part exists.
    pub fn has_extended_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docProps/app.xml"))
    }

    /// Whether a custom properties part exists.
    pub fn has_custom_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docProps/custom.xml"))
    }

    /// Number of custom properties.
    pub fn custom_property_count(&self) -> Result<usize> {
        Ok(self.custom_properties()?.properties.len())
    }

    /// Convenience: set the document title in core properties.
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.title = Some(title.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read the document title from core properties.
    pub fn title(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.title)
    }

    /// Convenience: set the document creator in core properties.
    /// Whether core `title` is set.
    pub fn has_title(&self) -> Result<bool> {
        Ok(self.title()?.is_some())
    }

    /// Clear core `title`. Returns whether it was present.
    pub fn clear_title(&mut self) -> Result<bool> {
        let had = self.title()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.title = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_creator(&mut self, creator: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.creator = Some(creator.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read the document creator from core properties.
    pub fn creator(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.creator)
    }

    /// Convenience: set core subject.
    /// Whether core `creator` is set.
    pub fn has_creator(&self) -> Result<bool> {
        Ok(self.creator()?.is_some())
    }

    /// Clear core `creator`. Returns whether it was present.
    pub fn clear_creator(&mut self) -> Result<bool> {
        let had = self.creator()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.creator = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_subject(&mut self, subject: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.subject = Some(subject.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core subject.
    pub fn subject(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.subject)
    }

    /// Convenience: set core keywords.
    /// Whether core `subject` is set.
    pub fn has_subject(&self) -> Result<bool> {
        Ok(self.subject()?.is_some())
    }

    /// Clear core `subject`. Returns whether it was present.
    pub fn clear_subject(&mut self) -> Result<bool> {
        let had = self.subject()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.subject = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_keywords(&mut self, keywords: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.keywords = Some(keywords.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core keywords.
    pub fn keywords(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.keywords)
    }

    /// Convenience: set core description.
    /// Whether core `keywords` is set.
    pub fn has_keywords(&self) -> Result<bool> {
        Ok(self.keywords()?.is_some())
    }

    /// Clear core `keywords`. Returns whether it was present.
    pub fn clear_keywords(&mut self) -> Result<bool> {
        let had = self.keywords()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.keywords = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.description = Some(description.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core description.
    pub fn description(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.description)
    }

    /// Convenience: set core category.
    /// Whether core `description` is set.
    pub fn has_description(&self) -> Result<bool> {
        Ok(self.description()?.is_some())
    }

    /// Clear core `description`. Returns whether it was present.
    pub fn clear_description(&mut self) -> Result<bool> {
        let had = self.description()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.description = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_category(&mut self, category: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.category = Some(category.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core category.
    pub fn category(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.category)
    }

    /// Convenience: set extended Application name.
    /// Whether core `category` is set.
    pub fn has_category(&self) -> Result<bool> {
        Ok(self.category()?.is_some())
    }

    /// Clear core `category`. Returns whether it was present.
    pub fn clear_category(&mut self) -> Result<bool> {
        let had = self.category()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.category = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_application(&mut self, application: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.application = Some(application.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Application name.
    pub fn application(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.application)
    }

    /// Convenience: set extended Company.
    /// Whether extended `application` is set.
    pub fn has_application(&self) -> Result<bool> {
        Ok(self.application()?.is_some())
    }

    /// Clear extended `application`. Returns whether it was present.
    pub fn clear_application(&mut self) -> Result<bool> {
        let had = self.application()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.application = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_company(&mut self, company: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.company = Some(company.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Company.
    pub fn company(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.company)
    }

    /// Convenience: set core lastModifiedBy.
    /// Whether extended `company` is set.
    pub fn has_company(&self) -> Result<bool> {
        Ok(self.company()?.is_some())
    }

    /// Clear extended `company`. Returns whether it was present.
    pub fn clear_company(&mut self) -> Result<bool> {
        let had = self.company()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.company = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_last_modified_by(&mut self, name: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.last_modified_by = Some(name.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core lastModifiedBy.
    pub fn last_modified_by(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.last_modified_by)
    }

    /// Convenience: set core revision.
    /// Whether core `last_modified_by` is set.
    pub fn has_last_modified_by(&self) -> Result<bool> {
        Ok(self.last_modified_by()?.is_some())
    }

    /// Clear core `last_modified_by`. Returns whether it was present.
    pub fn clear_last_modified_by(&mut self) -> Result<bool> {
        let had = self.last_modified_by()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.last_modified_by = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_revision(&mut self, revision: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.revision = Some(revision.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core revision.
    pub fn revision(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.revision)
    }

    /// Convenience: set core language.
    /// Whether core `revision` is set.
    pub fn has_revision(&self) -> Result<bool> {
        Ok(self.revision()?.is_some())
    }

    /// Clear core `revision`. Returns whether it was present.
    pub fn clear_revision(&mut self) -> Result<bool> {
        let had = self.revision()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.revision = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_language(&mut self, language: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.language = Some(language.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core language.
    pub fn language(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.language)
    }

    /// Convenience: set core version.
    /// Whether a document language is set on core properties.
    pub fn has_language(&self) -> Result<bool> {
        Ok(self.language()?.is_some())
    }

    /// Clear core language. Returns whether it was present.
    pub fn clear_language(&mut self) -> Result<bool> {
        let had = self.language()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.language = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_version(&mut self, version: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.version = Some(version.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core version.
    pub fn version(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.version)
    }

    /// Convenience: set core contentStatus.
    /// Whether core `version` is set.
    pub fn has_version(&self) -> Result<bool> {
        Ok(self.version()?.is_some())
    }

    /// Clear core `version`. Returns whether it was present.
    pub fn clear_version(&mut self) -> Result<bool> {
        let had = self.version()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.version = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_content_status(&mut self, status: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.content_status = Some(status.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core contentStatus.
    pub fn content_status(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.content_status)
    }

    /// Convenience: set extended Manager.
    /// Whether core `content_status` is set.
    pub fn has_content_status(&self) -> Result<bool> {
        Ok(self.content_status()?.is_some())
    }

    /// Clear core `content_status`. Returns whether it was present.
    pub fn clear_content_status(&mut self) -> Result<bool> {
        let had = self.content_status()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.content_status = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_manager(&mut self, manager: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.manager = Some(manager.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Manager.
    pub fn manager(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.manager)
    }

    /// Convenience: set extended Template.
    /// Whether extended `manager` is set.
    pub fn has_manager(&self) -> Result<bool> {
        Ok(self.manager()?.is_some())
    }

    /// Clear extended `manager`. Returns whether it was present.
    pub fn clear_manager(&mut self) -> Result<bool> {
        let had = self.manager()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.manager = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_template(&mut self, template: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.template = Some(template.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Template.
    pub fn template(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.template)
    }

    /// Convenience: set extended HyperlinkBase.
    /// Whether extended `template` is set.
    pub fn has_template(&self) -> Result<bool> {
        Ok(self.template()?.is_some())
    }

    /// Clear extended `template`. Returns whether it was present.
    pub fn clear_template(&mut self) -> Result<bool> {
        let had = self.template()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.template = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_hyperlink_base(&mut self, base: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.hyperlink_base = Some(base.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended HyperlinkBase.
    pub fn hyperlink_base(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.hyperlink_base)
    }

    /// Convenience: set core `dcterms:created` timestamp (ISO-8601 string).
    /// Whether a hyperlink base is set.
    pub fn has_hyperlink_base(&self) -> Result<bool> {
        Ok(self.hyperlink_base()?.is_some())
    }

    /// Clear hyperlink base. Returns whether it was present.
    pub fn clear_hyperlink_base(&mut self) -> Result<bool> {
        let had = self.hyperlink_base()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.hyperlink_base = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_created(&mut self, created: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.created = Some(created.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core `dcterms:created`.
    pub fn created(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.created)
    }

    /// Convenience: set core `dcterms:modified` timestamp (ISO-8601 string).
    /// Whether core `created` is set.
    pub fn has_created(&self) -> Result<bool> {
        Ok(self.created()?.is_some())
    }

    /// Clear core `created`. Returns whether it was present.
    pub fn clear_created(&mut self) -> Result<bool> {
        let had = self.created()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.created = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_modified(&mut self, modified: &str) -> Result<()> {
        let mut props = self.package_properties()?;
        props.modified = Some(modified.to_string());
        self.set_package_properties(&props)
    }

    /// Convenience: read core `dcterms:modified`.
    pub fn modified(&self) -> Result<Option<String>> {
        Ok(self.package_properties()?.modified)
    }

    /// Resolve the content type of a part URI (override or default by extension).
    /// Whether core `modified` is set.
    pub fn has_modified(&self) -> Result<bool> {
        Ok(self.modified()?.is_some())
    }

    /// Clear core `modified`. Returns whether it was present.
    pub fn clear_modified(&mut self) -> Result<bool> {
        let had = self.modified()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.modified = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    pub fn part_content_type(&self, uri: &str) -> Option<String> {
        self.package
            .opc()
            .content_types()
            .content_type_for(uri)
            .map(|s| s.to_string())
    }

    /// List package-level relationships as `(id, type, target)`.
    pub fn list_package_relationships(&self) -> Vec<(String, String, String)> {
        self.package
            .opc()
            .package_relationships()
            .iter()
            .map(|r| {
                (
                    r.id.clone(),
                    r.relationship_type.clone(),
                    r.target.clone(),
                )
            })
            .collect()
    }

    /// List main-document relationships as `(id, type, target)`.
    pub fn list_main_relationships(&self) -> Vec<(String, String, String)> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        self.package
            .opc()
            .part_relationships(&main.part().uri)
            .map(|rels| {
                rels.iter()
                    .map(|r| {
                        (
                            r.id.clone(),
                            r.relationship_type.clone(),
                            r.target.clone(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Whether a part exists at `uri`.
    pub fn has_part(&self, uri: &str) -> bool {
        self.package.opc().has_part(&PackUri::new(uri))
    }

    /// Read raw part bytes by URI.
    pub fn get_part_bytes(&self, uri: &str) -> Option<Vec<u8>> {
        self.package
            .opc()
            .get_part(&PackUri::new(uri))
            .map(|b| b.to_vec())
    }

    /// Write/replace raw part bytes and content type.
    pub fn set_part_bytes(
        &mut self,
        uri: &str,
        content_type: &str,
        data: impl Into<Vec<u8>>,
    ) {
        self.package
            .opc_mut()
            .set_part(PackUri::new(uri), content_type, data);
    }

    /// Byte length of a part, if present.
    pub fn part_size(&self, uri: &str) -> Option<usize> {
        self.package
            .opc()
            .get_part(&PackUri::new(uri))
            .map(|b| b.len())
    }

    /// Resolve a package-level relationship target by id.
    pub fn package_relationship_target(&self, id: &str) -> Option<String> {
        self.package
            .opc()
            .package_relationships()
            .get(id)
            .map(|r| r.target.clone())
    }

    /// List content-type overrides as `(part_name, content_type)`.
    pub fn list_content_type_overrides(&self) -> Vec<(String, String)> {
        self.package
            .opc()
            .content_types()
            .overrides
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Convenience: set extended AppVersion.
    /// Whether any content-type overrides are registered.
    pub fn has_content_type_overrides(&self) -> bool {
        !self.list_content_type_overrides().is_empty()
    }

    /// Count content-type overrides.
    pub fn content_type_override_count(&self) -> usize {
        self.list_content_type_overrides().len()
    }

    pub fn set_application_version(&mut self, version: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.application_version = Some(version.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended AppVersion.
    pub fn application_version(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.application_version)
    }

    /// Convenience: set extended DocSecurity.
    /// Whether extended `application_version` is set.
    pub fn has_application_version(&self) -> Result<bool> {
        Ok(self.application_version()?.is_some())
    }

    /// Clear extended `application_version`. Returns whether it was present.
    pub fn clear_application_version(&mut self) -> Result<bool> {
        let had = self.application_version()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.application_version = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    pub fn set_doc_security(&mut self, security: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.doc_security = Some(security);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended DocSecurity.
    pub fn doc_security(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.doc_security)
    }

    /// Whether DocSecurity is set.
    pub fn has_doc_security(&self) -> Result<bool> {
        Ok(self.doc_security()?.is_some())
    }

    /// Clear DocSecurity.
    pub fn clear_doc_security(&mut self) -> Result<bool> {
        let had = self.doc_security()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.doc_security = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Convenience: set extended Pages count.
    pub fn set_pages(&mut self, pages: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.pages = Some(pages);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Pages.
    pub fn pages(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.pages)
    }

    /// Convenience: set extended Words count.
    pub fn set_words(&mut self, words: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.words = Some(words);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Words.
    pub fn words(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.words)
    }

    /// Convenience: set extended Characters count.
    pub fn set_characters(&mut self, characters: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.characters = Some(characters);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Characters.
    pub fn characters(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.characters)
    }

    /// Convenience: set extended CharactersWithSpaces.
    pub fn set_characters_with_spaces(&mut self, n: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.characters_with_spaces = Some(n);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended CharactersWithSpaces.
    pub fn characters_with_spaces(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.characters_with_spaces)
    }

    /// Whether pages count is set.
    pub fn has_pages(&self) -> Result<bool> {
        Ok(self.pages()?.is_some())
    }

    /// Clear extended Pages.
    pub fn clear_pages(&mut self) -> Result<bool> {
        let mut props = self.extended_properties()?;
        if props.pages.is_none() {
            return Ok(false);
        }
        props.pages = None;
        self.set_extended_properties(&props)?;
        Ok(true)
    }

    /// Whether words count is set.
    pub fn has_words(&self) -> Result<bool> {
        Ok(self.words()?.is_some())
    }

    /// Clear extended Words.
    pub fn clear_words(&mut self) -> Result<bool> {
        let mut props = self.extended_properties()?;
        if props.words.is_none() {
            return Ok(false);
        }
        props.words = None;
        self.set_extended_properties(&props)?;
        Ok(true)
    }

    /// Whether characters count is set.
    pub fn has_characters(&self) -> Result<bool> {
        Ok(self.characters()?.is_some())
    }

    /// Clear extended Characters.
    pub fn clear_characters(&mut self) -> Result<bool> {
        let mut props = self.extended_properties()?;
        if props.characters.is_none() {
            return Ok(false);
        }
        props.characters = None;
        self.set_extended_properties(&props)?;
        Ok(true)
    }

    /// Whether characters_with_spaces is set.
    pub fn has_characters_with_spaces(&self) -> Result<bool> {
        Ok(self.characters_with_spaces()?.is_some())
    }

    /// Clear extended CharactersWithSpaces.
    pub fn clear_characters_with_spaces(&mut self) -> Result<bool> {
        let mut props = self.extended_properties()?;
        if props.characters_with_spaces.is_none() {
            return Ok(false);
        }
        props.characters_with_spaces = None;
        self.set_extended_properties(&props)?;
        Ok(true)
    }

    /// Whether lines count is set.
    pub fn has_lines(&self) -> Result<bool> {
        Ok(self.lines()?.is_some())
    }

    /// Clear extended Lines.
    pub fn clear_lines(&mut self) -> Result<bool> {
        let mut props = self.extended_properties()?;
        if props.lines.is_none() {
            return Ok(false);
        }
        props.lines = None;
        self.set_extended_properties(&props)?;
        Ok(true)
    }

    /// Whether paragraphs count is set.
    pub fn has_paragraphs_count(&self) -> Result<bool> {
        Ok(self.paragraphs_count()?.is_some())
    }

    /// Clear extended Paragraphs.
    pub fn clear_paragraphs_count(&mut self) -> Result<bool> {
        let mut props = self.extended_properties()?;
        if props.paragraphs.is_none() {
            return Ok(false);
        }
        props.paragraphs = None;
        self.set_extended_properties(&props)?;
        Ok(true)
    }

    /// Convenience: set extended Lines.
    pub fn set_lines(&mut self, n: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.lines = Some(n);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Lines.
    pub fn lines(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.lines)
    }

    /// Convenience: set extended Paragraphs.
    pub fn set_paragraphs_count(&mut self, n: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.paragraphs = Some(n);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Paragraphs.
    pub fn paragraphs_count(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.paragraphs)
    }

    /// Convenience: set extended SharedDoc.
    pub fn set_shared_doc(&mut self, shared: bool) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.shared_doc = Some(shared);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended SharedDoc.
    pub fn shared_doc(&self) -> Result<Option<bool>> {
        Ok(self.extended_properties()?.shared_doc)
    }

    /// Convenience: set extended TotalTime (minutes).
    pub fn set_total_time(&mut self, minutes: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.total_time = Some(minutes);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended TotalTime.
    pub fn total_time(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.total_time)
    }

    /// Whether SharedDoc is set.
    pub fn has_shared_doc(&self) -> Result<bool> {
        Ok(self.shared_doc()?.is_some())
    }

    /// Clear SharedDoc.
    pub fn clear_shared_doc(&mut self) -> Result<bool> {
        let had = self.shared_doc()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.shared_doc = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Whether TotalTime is set.
    pub fn has_total_time(&self) -> Result<bool> {
        Ok(self.total_time()?.is_some())
    }

    /// Clear TotalTime.
    pub fn clear_total_time(&mut self) -> Result<bool> {
        let had = self.total_time()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.total_time = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Set an integer custom property by name.
    pub fn set_custom_property_i4(&mut self, name: &str, value: i32) -> Result<()> {
        let mut props = self.custom_properties()?;
        props.set_i4(name, value);
        self.set_custom_properties(&props)
    }

    /// Read an integer custom property by name.
    pub fn get_custom_property_i4(&self, name: &str) -> Result<Option<i32>> {
        Ok(self.custom_properties()?.get(name).and_then(|p| match &p.value {
            crate::opc::CustomPropertyValue::I4(v) => Some(*v),
            _ => None,
        }))
    }

    /// Set a boolean custom property by name.
    pub fn set_custom_property_bool(&mut self, name: &str, value: bool) -> Result<()> {
        let mut props = self.custom_properties()?;
        props.set_bool(name, value);
        self.set_custom_properties(&props)
    }

    /// Read a boolean custom property by name.
    pub fn get_custom_property_bool(&self, name: &str) -> Result<Option<bool>> {
        Ok(self.custom_properties()?.get(name).and_then(|p| match &p.value {
            crate::opc::CustomPropertyValue::Bool(v) => Some(*v),
            _ => None,
        }))
    }

    /// List custom property names.
    pub fn list_custom_property_names(&self) -> Result<Vec<String>> {
        Ok(self
            .custom_properties()?
            .names()
            .into_iter()
            .map(|s| s.to_string())
            .collect())
    }

    /// Set a string custom property by name.
    pub fn set_custom_property_string(&mut self, name: &str, value: &str) -> Result<()> {
        let mut props = self.custom_properties()?;
        props.set_string(name, value);
        self.set_custom_properties(&props)
    }

    /// Read a custom property string value by name.
    pub fn get_custom_property_string(&self, name: &str) -> Result<Option<String>> {
        Ok(self
            .custom_properties()?
            .get(name)
            .and_then(|p| p.value.as_str().map(|s| s.to_string())))
    }

    /// Remove one custom property by name. Returns whether it was present.
    pub fn remove_custom_property(&mut self, name: &str) -> Result<bool> {
        let mut props = self.custom_properties()?;
        let removed = props.remove(name);
        if removed {
            self.set_custom_properties(&props)?;
        }
        Ok(removed)
    }

    /// Remove all custom properties (writes empty collection).
    pub fn clear_custom_properties(&mut self) -> Result<bool> {
        if !self.has_custom_properties() {
            return Ok(false);
        }
        let mut props = self.custom_properties()?;
        if props.is_empty() {
            return Ok(false);
        }
        props.clear();
        self.set_custom_properties(&props)?;
        Ok(true)
    }

    /// Add or replace a Custom XML part related from the main document.
    ///
    /// Returns `(relationship_id, part_uri)`.
    pub fn add_custom_xml_part(
        &mut self,
        xml: impl AsRef<[u8]>,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let item_uri = loop {
            let candidate = PackUri::new(format!("/customXml/item{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            item_uri.clone(),
            content_type::CUSTOM_XML,
            xml.as_ref().to_vec(),
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::CUSTOM_XML,
            &item_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, item_uri))
    }

    /// List Custom XML parts related from the main document: `(rId, uri, bytes)`.
    pub fn custom_xml_parts(&self) -> Result<Vec<(String, PackUri, Vec<u8>)>> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = &main.part().uri;
        let Some(rels) = self.package.opc().part_relationships(main_uri) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for r in rels.find_all_by_type(rel::CUSTOM_XML) {
            let target = crate::opc::resolve_uri(main_uri, &r.target)?;
            if let Some(data) = self.package.opc().get_part(&target) {
                out.push((r.id.clone(), target, data.to_vec()));
            }
        }
        Ok(out)
    }

    /// Add a package thumbnail image (`docProps/thumbnail.{ext}`).
    ///
    /// Returns the relationship id. `content_type` e.g. `image/jpeg`, `image/png`.
    pub fn add_thumbnail(
        &mut self,
        image_bytes: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<String> {
        let uri = PackUri::new(format!("/docProps/thumbnail.{extension}"));
        self.package.set_part(
            uri.clone(),
            content_type_str,
            image_bytes.into(),
        );
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .map(|r| r.id.clone())
        {
            return Ok(existing);
        }
        Ok(self.package.add_package_relationship(
            rel::THUMBNAIL,
            &uri,
            RelationshipTargetMode::Internal,
        ))
    }

    /// Whether a package thumbnail relationship exists.
    pub fn has_thumbnail(&self) -> bool {
        self.package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .is_some()
            || self
                .package
                .opc()
                .part_uris().into_iter().any(|u| u.as_str().starts_with("/docProps/thumbnail."))
    }

    /// Remove the package thumbnail part and relationship.
    pub fn clear_thumbnail(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/docProps/thumbnail."))
            
            .collect();
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .is_some();
        if uris.is_empty() && !had_rel {
            return Ok(false);
        }
        if let Some(rel) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .map(|r| r.id.clone())
        {
            self.package
                .opc_mut()
                .package_relationships_mut()
                .remove(&rel);
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Add paragraph styles to the styles part (creates default styles if missing).
    ///
    /// Each entry is `(style_id, name, based_on)`.
    pub fn add_paragraph_styles(
        &mut self,
        styles: &[(&str, &str, Option<&str>)],
    ) -> Result<()> {
        use crate::wordprocessing::paragraph_style;
        let styles_uri = PackUri::new("/word/styles.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&styles_uri) {
            parse_element(data)?
        } else {
            self.add_default_styles()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&styles_uri)
                    .ok_or_else(|| Error::PartNotFound(styles_uri.to_string()))?,
            )?
        };
        for (id, name, based) in styles {
            // Remove existing style with same id
            root.children.retain(|c| {
                !(c.local_name == "style"
                    && c.get_attribute_qname("w:styleId")
                        .or_else(|| c.get_attribute("styleId"))
                        == Some(*id))
            });
            root.append_child(paragraph_style(id, name, *based, false));
        }
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            styles_uri,
            content_type::WORD_STYLES,
            xml,
        );
        Ok(())
    }

    /// Set document default font and size in styles (`w:docDefaults`).
    ///
    /// `font_size_half_points` is Word half-points (24 = 12pt).
    pub fn set_document_defaults(
        &mut self,
        ascii_font: &str,
        font_size_half_points: u32,
    ) -> Result<()> {
        use crate::wordprocessing::doc_defaults;
        let styles_uri = PackUri::new("/word/styles.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&styles_uri) {
            parse_element(data)?
        } else {
            self.add_default_styles()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&styles_uri)
                    .ok_or_else(|| Error::PartNotFound(styles_uri.to_string()))?,
            )?
        };
        root.children.retain(|c| c.local_name != "docDefaults");
        root.children
            .insert(0, doc_defaults(ascii_font, font_size_half_points));
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            styles_uri,
            content_type::WORD_STYLES,
            xml,
        );
        Ok(())
    }

    /// Remove `w:docDefaults` from styles.
    pub fn clear_document_defaults(&mut self) -> Result<bool> {
        let styles_uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&styles_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "docDefaults");
        if root.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(styles_uri, content_type::WORD_STYLES, xml);
        Ok(true)
    }

    /// Add a bibliography sources document as a Custom XML part.
    ///
    /// Each entry is `(tag, title)`. Returns the custom XML relationship id.
    pub fn add_bibliography(
        &mut self,
        sources: &[(&str, &str)],
    ) -> Result<(String, PackUri)> {
        use crate::wordprocessing::bibliography_sources;
        let xml = crate::element::write_element(&bibliography_sources(
            sources.iter().map(|(t, title)| (*t, *title)),
        ))?;
        self.add_custom_xml_part(xml)
    }

    /// List bibliography sources from Custom XML parts as `(tag, title)`.
    pub fn list_bibliography_sources(&self) -> Result<Vec<(String, String)>> {
        let mut out = Vec::new();
        for (_rid, _uri, data) in self.custom_xml_parts()? {
            if let Ok(root) = parse_element(&data) {
                if root.local_name != "Sources" {
                    continue;
                }
                for src in root.children_by_name("Source") {
                    let tag = src
                        .child("Tag")
                        .map(|t| t.inner_text())
                        .unwrap_or_default();
                    let title = src
                        .child("Title")
                        .map(|t| t.inner_text())
                        .unwrap_or_default();
                    out.push((tag, title));
                }
            }
        }
        Ok(out)
    }

    /// Whether any bibliography Sources custom XML is present.
    pub fn has_bibliography(&self) -> Result<bool> {
        Ok(!self.list_bibliography_sources()?.is_empty())
    }

    /// Number of bibliography sources across custom XML parts.
    pub fn bibliography_source_count(&self) -> Result<usize> {
        Ok(self.list_bibliography_sources()?.len())
    }


    /// Whether any bibliography sources are present.
    pub fn has_bibliography_sources(&self) -> Result<bool> {
        Ok(self.bibliography_source_count()? > 0)
    }

    /// Remove bibliography sources matching `tag` from all Sources custom XML parts.
    /// Returns the number of Source elements removed.
    pub fn remove_bibliography_source(&mut self, tag: &str) -> Result<usize> {
        let mut removed = 0usize;
        let parts = self.custom_xml_parts()?;
        for (_rid, uri, data) in parts {
            let Ok(mut root) = parse_element(&data) else { continue };
            if root.local_name != "Sources" {
                continue;
            }
            let before = root.children.len();
            root.children.retain(|c| {
                if c.local_name != "Source" {
                    return true;
                }
                let t = c.child("Tag").map(|x| x.inner_text()).unwrap_or_default();
                t != tag
            });
            let n = before - root.children.len();
            if n == 0 {
                continue;
            }
            removed += n;
            let xml = crate::element::write_element(&root)?;
            // Preserve existing content type if any
            let ct = self
                .package
                .opc()
                .content_types()
                .content_type_for(uri.as_str())
                .unwrap_or("application/xml")
                .to_string();
            self.package.set_part(uri, ct, xml);
        }
        Ok(removed)
    }

    /// Clear all bibliography Source entries (leaves empty Sources roots). Returns count removed.
    pub fn clear_bibliography_sources(&mut self) -> Result<usize> {
        let tags: Vec<String> = self
            .list_bibliography_sources()?
            .into_iter()
            .map(|(t, _)| t)
            .collect();
        let mut total = 0usize;
        for tag in tags {
            total += self.remove_bibliography_source(&tag)?;
        }
        Ok(total)
    }

    /// Update the Title of bibliography sources matching `tag`. Returns count updated.
    pub fn set_bibliography_source_title(&mut self, tag: &str, title: &str) -> Result<usize> {
        let mut updated = 0usize;
        let parts = self.custom_xml_parts()?;
        for (_rid, uri, data) in parts {
            let Ok(mut root) = parse_element(&data) else { continue };
            if root.local_name != "Sources" {
                continue;
            }
            let mut changed = false;
            for src in root.children.iter_mut() {
                if src.local_name != "Source" {
                    continue;
                }
                let t = src.child("Tag").map(|x| x.inner_text()).unwrap_or_default();
                if t != tag {
                    continue;
                }
                if let Some(title_el) = src.child_mut("Title") {
                    title_el.set_text(title);
                } else {
                    let b = "http://schemas.openxmlformats.org/officeDocument/2006/bibliography";
                    src.append_child(OpenXmlElement::new("b", b, "Title").with_text(title));
                }
                changed = true;
                updated += 1;
            }
            if changed {
                let xml = crate::element::write_element(&root)?;
                let ct = self
                    .package
                    .opc()
                    .content_types()
                    .content_type_for(uri.as_str())
                    .unwrap_or("application/xml")
                    .to_string();
                self.package.set_part(uri, ct, xml);
            }
        }
        Ok(updated)
    }

    /// Remove Title from bibliography sources matching `tag`.
    pub fn clear_bibliography_source_title(&mut self, tag: &str) -> Result<usize> {
        let mut updated = 0usize;
        let parts = self.custom_xml_parts()?;
        for (_rid, uri, data) in parts {
            let Ok(mut root) = parse_element(&data) else { continue };
            if root.local_name != "Sources" {
                continue;
            }
            let mut changed = false;
            for src in root.children.iter_mut() {
                if src.local_name != "Source" {
                    continue;
                }
                let t = src.child("Tag").map(|x| x.inner_text()).unwrap_or_default();
                if t != tag {
                    continue;
                }
                let before = src.children.len();
                src.children.retain(|c| c.local_name != "Title");
                if src.children.len() < before {
                    changed = true;
                    updated += 1;
                }
            }
            if changed {
                let xml = crate::element::write_element(&root)?;
                let ct = self
                    .package
                    .opc()
                    .content_types()
                    .content_type_for(uri.as_str())
                    .unwrap_or("application/xml")
                    .to_string();
                self.package.set_part(uri, ct, xml);
            }
        }
        Ok(updated)
    }

    /// Add a default styles part (`/word/styles.xml`) related from the main document.
    pub fn add_default_styles(&mut self) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_default_styles_part(&mut self.package)
    }

    /// Add a styles part with a custom `w:styles` root.
    pub fn add_styles(&mut self, styles: OpenXmlElement) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_styles_part(&mut self.package, styles)
    }

    /// Add a default settings part (`/word/settings.xml`).
    pub fn add_default_settings(&mut self) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_default_settings_part(&mut self.package)
    }


    /// Set document zoom percent in settings (`w:zoom w:percent`).
    pub fn set_zoom(&mut self, percent: u32) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children.retain(|c| c.local_name != "zoom");
        root.append_child(
            OpenXmlElement::w("zoom").with_attribute_qname("w:percent", percent.to_string()),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }


    /// Set settings view type (`w:view w:val`), e.g. `"print"`, `"web"`, `"outline"`.
    pub fn set_view(&mut self, val: &str) -> Result<()> {
        self.upsert_settings_child("view", |el| {
            el.set_attribute_qname("w:val", val);
        })
    }

    /// Read settings view type.
    pub fn view(&self) -> Result<Option<String>> {
        self.settings_child_attr("view", "val")
    }

    /// Whether settings define a view type.
    pub fn has_view(&self) -> Result<bool> {
        Ok(self.view()?.is_some())
    }

    /// Remove settings view. Returns whether present.
    pub fn clear_view(&mut self) -> Result<bool> {
        self.remove_settings_child("view")
    }

    /// Set default tab stop twips in settings (`w:defaultTabStop w:val`).
    pub fn set_default_tab_stop(&mut self, twips: u32) -> Result<()> {
        self.upsert_settings_child("defaultTabStop", |el| {
            el.set_attribute_qname("w:val", twips.to_string());
        })
    }

    /// Read default tab stop twips.
    pub fn default_tab_stop(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("defaultTabStop", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether settings define defaultTabStop.
    pub fn has_default_tab_stop(&self) -> Result<bool> {
        Ok(self.default_tab_stop()?.is_some())
    }

    /// Remove defaultTabStop. Returns whether present.
    pub fn clear_default_tab_stop(&mut self) -> Result<bool> {
        self.remove_settings_child("defaultTabStop")
    }

    /// Set document grid line pitch (`w:docGrid w:linePitch`).
    pub fn set_document_grid(&mut self, line_pitch: u32) -> Result<()> {
        self.upsert_settings_child("docGrid", |el| {
            el.set_attribute_qname("w:linePitch", line_pitch.to_string());
        })
    }

    /// Set document grid with type and optional character spacing.
    ///
    /// `grid_type` e.g. `"default"`, `"lines"`, `"linesAndChars"`, `"snapToChars"`.
    pub fn set_document_grid_ex(
        &mut self,
        line_pitch: u32,
        grid_type: Option<&str>,
        char_space: Option<i32>,
    ) -> Result<()> {
        self.upsert_settings_child("docGrid", |el| {
            el.set_attribute_qname("w:linePitch", line_pitch.to_string());
            if let Some(t) = grid_type {
                el.set_attribute_qname("w:type", t);
            }
            if let Some(cs) = char_space {
                el.set_attribute_qname("w:charSpace", cs.to_string());
            }
        })
    }

    /// Clear document grid (alias for [`clear_document_grid`](Self::clear_document_grid)).
    pub fn clear_document_grid_ex(&mut self) -> Result<bool> {
        self.clear_document_grid()
    }

    /// Read document grid type.
    pub fn document_grid_type(&self) -> Result<Option<String>> {
        self.settings_child_attr("docGrid", "type")
    }

    /// Read document grid character spacing.
    pub fn document_grid_char_space(&self) -> Result<Option<i32>> {
        Ok(self
            .settings_child_attr("docGrid", "charSpace")?
            .and_then(|s| s.parse().ok()))
    }

    /// Read document grid line pitch.
    pub fn document_grid_line_pitch(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("docGrid", "linePitch")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether settings define docGrid.
    pub fn has_document_grid(&self) -> Result<bool> {
        Ok(self.document_grid_line_pitch()?.is_some()
            || self.document_grid_type()?.is_some())
    }

    /// Remove docGrid. Returns whether present.
    pub fn clear_document_grid(&mut self) -> Result<bool> {
        self.remove_settings_child("docGrid")
    }

    fn ensure_settings_root(&mut self) -> Result<(PackUri, OpenXmlElement)> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        Ok((settings_uri, root))
    }

    fn upsert_settings_child(
        &mut self,
        local_name: &str,
        f: impl FnOnce(&mut OpenXmlElement),
    ) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if let Some(el) = root.child_mut(local_name) {
            f(el);
        } else {
            let mut el = OpenXmlElement::w(local_name);
            f(&mut el);
            root.append_child(el);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    fn settings_child_attr(&self, local_name: &str, attr: &str) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.child(local_name).and_then(|el| {
            el.get_attribute(attr)
                .or_else(|| el.get_attribute_qname(&format!("w:{attr}")))
                .map(|s| s.to_string())
        }))
    }

    fn remove_settings_child(&mut self, local_name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != local_name);
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(removed)
    }

    /// Read zoom percent from settings, if present.
    pub fn zoom(&self) -> Result<Option<u32>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("zoom")
            .and_then(|z| {
                z.get_attribute("percent")
                    .or_else(|| z.get_attribute_qname("w:percent"))
            })
            .and_then(|s| s.parse().ok()))
    }

    /// Whether settings define a zoom percent.
    pub fn has_zoom(&self) -> Result<bool> {
        Ok(self.zoom()?.is_some())
    }

    /// Remove zoom from settings. Returns whether it was present.
    pub fn clear_zoom(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "zoom");
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(removed)
    }

    /// Add even/odd header pair and enable `w:evenAndOddHeaders` in settings.
    pub fn add_even_odd_headers(
        &mut self,
        default_text: &str,
        even_text: &str,
    ) -> Result<(String, String)> {
        let default_rid = self.add_default_header(default_text)?;
        let even_rid = self.add_header(vec![paragraph(vec![run(vec![text(even_text)])])])?;
        self.ensure_sect_pr_reference(header_reference(&even_rid, "even"))?;
        // Enable evenAndOddHeaders in settings
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        if root.child("evenAndOddHeaders").is_none() {
            root.append_child(OpenXmlElement::w("evenAndOddHeaders"));
            let xml = crate::element::write_element(&root)?;
            self.package.set_part(
                settings_uri,
                content_type::WORD_SETTINGS,
                xml,
            );
        }
        Ok((default_rid, even_rid))
    }

    /// Whether even/odd headers are enabled in settings.
    pub fn even_odd_headers_enabled(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("evenAndOddHeaders").is_some())
    }

    /// Disable even/odd headers in settings (does not remove header parts).
    pub fn clear_even_odd_headers(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children
            .retain(|c| c.local_name != "evenAndOddHeaders");
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package.set_part(
                settings_uri,
                content_type::WORD_SETTINGS,
                xml,
            );
        }
        Ok(removed)
    }

    /// Add a text watermark via a header containing a VML textpath shape.
    pub fn add_watermark(&mut self, text_value: &str) -> Result<String> {
        use crate::wordprocessing::watermark_header;
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let (rid, _) = main.add_header_part(&mut self.package, watermark_header(text_value))?;
        self.ensure_sect_pr_reference(header_reference(&rid, "default"))?;
        Ok(rid)
    }

    /// Set the document background color (`w:background` under `w:document`).
    ///
    /// `color` is 6-digit hex RGB (e.g. `"FFFFCC"`).
    pub fn set_document_background(&mut self, color: &str) -> Result<()> {
        use crate::wordprocessing::document_background;
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        doc.children.retain(|c| c.local_name != "background");
        // background is typically the first child of document
        doc.children.insert(0, document_background(color));
        Ok(())
    }

    /// Whether the document has a `w:background` element.
    pub fn has_document_background(&mut self) -> Result<bool> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(doc.child("background").is_some())
    }

    /// Read document background color if present.
    pub fn document_background_color(&mut self) -> Result<Option<String>> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(doc
            .child("background")
            .and_then(|b| {
                b.get_attribute("color")
                    .or_else(|| {
                        b.attributes
                            .iter()
                            .find(|a| a.local_name == "color")
                            .map(|a| a.value.as_str())
                    })
                    .map(|s| s.to_string())
            }))
    }

    /// Remove the document background element. Returns whether one was present.
    pub fn clear_document_background(&mut self) -> Result<bool> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let before = doc.children.len();
        doc.children.retain(|c| c.local_name != "background");
        Ok(doc.children.len() < before)
    }

    /// Set document variables in settings (`w:docVars`).
    ///
    /// Creates settings if missing. Replaces any existing docVars.
    pub fn set_document_variables(&mut self, vars: &[(&str, &str)]) -> Result<()> {
        use crate::wordprocessing::document_variables;
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children.retain(|c| c.local_name != "docVars");
        if !vars.is_empty() {
            root.append_child(document_variables(vars.iter().copied()));
        }
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Read document variables from settings as `(name, value)`.
    pub fn document_variables(&self) -> Result<Vec<(String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(vars) = root.child("docVars") else {
            return Ok(Vec::new());
        };
        Ok(vars
            .children_by_name("docVar")
            .filter_map(|el| {
                let name = el
                    .get_attribute_qname("w:name")
                    .or_else(|| el.get_attribute("name"))?
                    .to_string();
                let val = el
                    .get_attribute_qname("w:val")
                    .or_else(|| el.get_attribute("val"))?
                    .to_string();
                Some((name, val))
            })
            .collect())
    }

    /// Whether any document variables are present.
    pub fn has_document_variables(&self) -> Result<bool> {
        Ok(!self.document_variables()?.is_empty())
    }


    /// Alias for [`document_variables`](Self::document_variables).
    pub fn list_document_variables(&self) -> Result<Vec<(String, String)>> {
        self.document_variables()
    }

    /// Number of document variables.
    pub fn document_variable_count(&self) -> Result<usize> {
        Ok(self.document_variables()?.len())
    }

    /// Whether a document variable with the given name exists.
    pub fn has_document_variable(&self, name: &str) -> Result<bool> {
        Ok(self.get_document_variable(name)?.is_some())
    }

    /// Look up one document variable value by name.
    pub fn get_document_variable(&self, name: &str) -> Result<Option<String>> {
        Ok(self
            .document_variables()?
            .into_iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v))
    }

    /// Set or replace a single document variable.
    pub fn set_document_variable(&mut self, name: &str, value: &str) -> Result<()> {
        let mut vars = self.document_variables()?;
        if let Some(existing) = vars.iter_mut().find(|(n, _)| n == name) {
            existing.1 = value.to_string();
        } else {
            vars.push((name.to_string(), value.to_string()));
        }
        let refs: Vec<(&str, &str)> = vars
            .iter()
            .map(|(n, v)| (n.as_str(), v.as_str()))
            .collect();
        self.set_document_variables(&refs)
    }

    /// Remove one document variable by name. Returns whether it was present.
    pub fn remove_document_variable(&mut self, name: &str) -> Result<bool> {
        let mut vars = self.document_variables()?;
        let before = vars.len();
        vars.retain(|(n, _)| n != name);
        if vars.len() == before {
            return Ok(false);
        }
        let refs: Vec<(&str, &str)> = vars
            .iter()
            .map(|(n, v)| (n.as_str(), v.as_str()))
            .collect();
        self.set_document_variables(&refs)?;
        Ok(true)
    }

    /// Clear all document variables.
    /// Alias for [`remove_document_variable`](Self::remove_document_variable).
    pub fn clear_document_variable(&mut self, name: &str) -> Result<bool> {
        self.remove_document_variable(name)
    }

    pub fn clear_document_variables(&mut self) -> Result<bool> {
        if !self.has_document_variables()? {
            return Ok(false);
        }
        self.set_document_variables(&[])?;
        Ok(true)
    }

    /// Whether a glossary document part is present.
    pub fn has_glossary(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/glossary/document.xml"))
    }

    /// Remove glossary document part and relationship.
    pub fn clear_glossary(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/glossary/document.xml");
        if !self.package.opc().has_part(&uri) {
            // Also try removing any glossary/* parts
            let extras: Vec<PackUri> = self
                .package
                .opc()
                .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/glossary/"))
                
                .collect();
            if extras.is_empty() {
                return Ok(false);
            }
            for u in extras {
                self.package.opc_mut().remove_part(&u);
            }
        } else {
            self.package.opc_mut().remove_part(&uri);
            let extras: Vec<PackUri> = self
                .package
                .opc()
                .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/glossary/"))
                
                .collect();
            for u in extras {
                self.package.opc_mut().remove_part(&u);
            }
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::GLOSSARY_DOCUMENT)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        Ok(true)
    }

    /// Number of custom XML parts.
    pub fn custom_xml_part_count(&self) -> Result<usize> {
        Ok(self.custom_xml_parts()?.len())
    }

    /// Whether any custom XML parts are present.
    /// Whether any custom XML parts exist.
    /// List custom XML part URIs.
    pub fn list_custom_xml_part_uris(&self) -> Result<Vec<PackUri>> {
        Ok(self
            .custom_xml_parts()?
            .into_iter()
            .map(|(_rid, uri, _data)| uri)
            .collect())
    }

    pub fn has_custom_xml(&self) -> Result<bool> {
        Ok(self.custom_xml_part_count()? > 0)
    }

    pub fn has_custom_xml_parts(&self) -> Result<bool> {
        Ok(!self.custom_xml_parts()?.is_empty())
    }

    /// Remove a custom XML part by relationship id. Returns whether it was found.
    pub fn remove_custom_xml_part(&mut self, relationship_id: &str) -> Result<bool> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let target = {
            let Some(rels) = self.package.opc().part_relationships(&main_uri) else {
                return Ok(false);
            };
            let Some(rel) = rels.get(relationship_id) else {
                return Ok(false);
            };
            if rel.relationship_type != rel::CUSTOM_XML {
                return Ok(false);
            }
            crate::opc::resolve_uri(&main_uri, &rel.target)?
        };
        self.package
            .opc_mut()
            .part_relationships_mut(&main_uri)
            .remove(relationship_id);
        self.package.opc_mut().remove_part(&target);
        Ok(true)
    }

    /// Remove all custom XML parts related from the main document.
    pub fn clear_custom_xml_parts(&mut self) -> Result<usize> {
        let parts = self.custom_xml_parts()?;
        let n = parts.len();
        for (rid, _, _) in parts {
            let _ = self.remove_custom_xml_part(&rid)?;
        }
        Ok(n)
    }

    /// Append a DATE field paragraph to the body.
    pub fn append_date_field(&mut self) -> Result<()> {
        use crate::wordprocessing::date_field;
        let body = self.body_mut()?;
        let para = paragraph(vec![date_field()]);
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(())
    }

    /// Append a simple field paragraph (`w:fldSimple`) with instruction and cached result.
    pub fn append_simple_field(&mut self, instruction: &str, result: &str) -> Result<()> {
        use crate::wordprocessing::simple_field;
        let body = self.body_mut()?;
        let para = paragraph(vec![simple_field(instruction, result)]);
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(())
    }

    /// Append a TOC field paragraph.
    pub fn append_toc_field(&mut self, switches: &str) -> Result<()> {
        use crate::wordprocessing::toc_field;
        let body = self.body_mut()?;
        let para = paragraph(vec![toc_field(switches)]);
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(())
    }

    /// List simple field instructions in the main document (`w:fldSimple/@w:instr`).
    pub fn list_simple_fields(&mut self) -> Result<Vec<String>> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(doc
            .descendants()
            .filter(|e| e.local_name == "fldSimple")
            .filter_map(|e| {
                e.get_attribute_qname("w:instr")
                    .or_else(|| e.get_attribute("instr"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Count simple fields (`w:fldSimple`) in the main document.
    pub fn simple_field_count(&mut self) -> Result<usize> {
        Ok(self.list_simple_fields()?.len())
    }

    /// Whether any simple fields exist in the body.
    pub fn has_simple_fields(&mut self) -> Result<bool> {
        Ok(self.simple_field_count()? > 0)
    }

    /// Alias for [`simple_field_count`](Self::simple_field_count).
    pub fn field_count(&mut self) -> Result<usize> {
        self.simple_field_count()
    }

    /// Count simple fields whose instruction contains `instr_substr`.
    pub fn count_simple_fields_matching(&mut self, instr_substr: &str) -> Result<usize> {
        Ok(self
            .list_simple_fields()?
            .into_iter()
            .filter(|s| s.contains(instr_substr))
            .count())
    }

    /// Remove simple fields whose instruction contains `instr_substr`.
    ///
    /// Matching `w:fldSimple` elements are unwrapped (children promoted). Returns count removed.
    pub fn remove_simple_fields_matching(&mut self, instr_substr: &str) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn visit(el: &mut OpenXmlElement, substr: &str, count: &mut usize) {
            let mut i = 0;
            while i < el.children.len() {
                let is_match = el.children[i].local_name == "fldSimple"
                    && el.children[i]
                        .get_attribute_qname("w:instr")
                        .or_else(|| el.children[i].get_attribute("instr"))
                        .map(|s| s.contains(substr))
                        .unwrap_or(false);
                if is_match {
                    let removed = el.children.remove(i);
                    let kids = removed.children;
                    let n = kids.len();
                    for (offset, kid) in kids.into_iter().enumerate() {
                        el.children.insert(i + offset, kid);
                    }
                    *count += 1;
                    i += n;
                } else {
                    visit(&mut el.children[i], substr, count);
                    i += 1;
                }
            }
        }
        let mut count = 0usize;
        visit(doc, instr_substr, &mut count);
        Ok(count)
    }

    /// Clear all simple fields (`w:fldSimple`), promoting children. Returns count removed.
    pub fn clear_simple_fields(&mut self) -> Result<usize> {
        self.remove_simple_fields_matching("")
    }

    /// Insert a page-break paragraph at the end of the body (before sectPr).
    /// List complex field instructions (`w:instrText`) in the main document body.
    pub fn list_complex_field_instructions(&mut self) -> Result<Vec<String>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for e in body.descendants() {
            if e.local_name == "instrText" {
                let t = e.inner_text();
                if !t.is_empty() {
                    out.push(t);
                }
            }
        }
        Ok(out)
    }

    /// Whether any complex field instruction text exists in the body.
    pub fn has_complex_fields(&mut self) -> Result<bool> {
        Ok(!self.list_complex_field_instructions()?.is_empty())
    }

    /// Count complex field instructions in the body.
    pub fn complex_field_count(&mut self) -> Result<usize> {
        Ok(self.list_complex_field_instructions()?.len())
    }

    /// Append a complex field paragraph to the body.
    pub fn append_complex_field(&mut self, instruction: &str, result: &str) -> Result<()> {
        use crate::wordprocessing::complex_field_paragraph;
        self.append_paragraph(complex_field_paragraph(instruction, result))
    }

    /// Remove complex field markup from the body, keeping cached result text runs.
    ///
    /// Drops `w:fldChar` runs and `w:instrText` runs; leaves other runs (including field results).
    /// Returns the number of field-related runs removed.
    pub fn clear_complex_fields(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn is_field_markup_run(el: &OpenXmlElement) -> bool {
            if el.local_name != "r" {
                return false;
            }
            el.children.iter().any(|c| {
                c.local_name == "fldChar"
                    || c.local_name == "instrText"
                    || c.local_name == "delInstrText"
            })
        }
        fn visit(el: &mut OpenXmlElement, count: &mut usize) {
            let mut i = 0;
            while i < el.children.len() {
                if is_field_markup_run(&el.children[i]) {
                    el.children.remove(i);
                    *count += 1;
                } else {
                    visit(&mut el.children[i], count);
                    i += 1;
                }
            }
        }
        let mut count = 0usize;
        visit(doc, &mut count);
        Ok(count)
    }

    /// Clear both simple (`w:fldSimple`) and complex field markup from the body.
    ///
    /// Returns `(simple_fields_removed, complex_markup_runs_removed)`.
    /// Remove complex field markup whose instruction contains `instr_substr`.
    ///
    /// Drops begin/instr/separate/end runs for matching fields; keeps result text when possible.
    /// Returns the number of instruction runs removed.
    pub fn remove_complex_fields_matching(&mut self, instr_substr: &str) -> Result<usize> {
        if instr_substr.is_empty() {
            return Ok(0);
        }
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        // Collect matching instrText text nodes' ancestor run indices is hard; instead walk
        // paragraphs and process run sequences.
        fn process_runs(runs: &mut Vec<OpenXmlElement>, substr: &str, count: &mut usize) {
            // Find instrText runs that match, then expand to field span begin..end
            let mut i = 0;
            while i < runs.len() {
                let is_instr = runs[i].local_name == "r"
                    && runs[i].children.iter().any(|c| c.local_name == "instrText");
                if is_instr {
                    let instr = runs[i]
                        .children
                        .iter()
                        .find(|c| c.local_name == "instrText")
                        .map(|c| c.inner_text())
                        .unwrap_or_default();
                    if instr.contains(substr) {
                        // walk back to begin
                        let mut start = i;
                        while start > 0 {
                            let is_begin = runs[start].local_name == "r"
                                && runs[start].children.iter().any(|c| {
                                    c.local_name == "fldChar"
                                        && (c.get_attribute_qname("w:fldCharType")
                                            .or_else(|| c.get_attribute("fldCharType"))
                                            == Some("begin"))
                                });
                            if is_begin {
                                break;
                            }
                            start -= 1;
                        }
                        // walk forward to end
                        let mut end = i;
                        while end < runs.len() {
                            let is_end = runs[end].local_name == "r"
                                && runs[end].children.iter().any(|c| {
                                    c.local_name == "fldChar"
                                        && (c.get_attribute_qname("w:fldCharType")
                                            .or_else(|| c.get_attribute("fldCharType"))
                                            == Some("end"))
                                });
                            if is_end {
                                break;
                            }
                            end += 1;
                        }
                        if end >= runs.len() {
                            end = runs.len() - 1;
                        }
                        // Keep runs between separate and end (result), drop markup
                        let mut keep = Vec::new();
                        let mut after_sep = false;
                        for r in runs.drain(start..=end) {
                            let is_sep = r.local_name == "r"
                                && r.children.iter().any(|c| {
                                    c.local_name == "fldChar"
                                        && (c.get_attribute_qname("w:fldCharType")
                                            .or_else(|| c.get_attribute("fldCharType"))
                                            == Some("separate"))
                                });
                            let is_markup = r.local_name == "r"
                                && r.children.iter().any(|c| {
                                    c.local_name == "fldChar"
                                        || c.local_name == "instrText"
                                        || c.local_name == "delInstrText"
                                });
                            if is_sep {
                                after_sep = true;
                                *count += 1;
                                continue;
                            }
                            if is_markup {
                                *count += 1;
                                continue;
                            }
                            if after_sep {
                                keep.push(r);
                            }
                        }
                        for (off, r) in keep.into_iter().enumerate() {
                            runs.insert(start + off, r);
                        }
                        i = start;
                        continue;
                    }
                }
                i += 1;
            }
        }
        fn visit(el: &mut OpenXmlElement, substr: &str, count: &mut usize) {
            // If this element has run children, process them as a sequence
            if el.children.iter().any(|c| c.local_name == "r") {
                process_runs(&mut el.children, substr, count);
            }
            for c in el.children.iter_mut() {
                visit(c, substr, count);
            }
        }
        let mut count = 0usize;
        visit(doc, instr_substr, &mut count);
        Ok(count)
    }

    pub fn clear_all_fields(&mut self) -> Result<(usize, usize)> {
        let simple = self.clear_simple_fields()?;
        let complex = self.clear_complex_fields()?;
        Ok((simple, complex))
    }

    pub fn append_page_break(&mut self) -> Result<()> {
        use crate::wordprocessing::page_break_run;
        let body = self.body_mut()?;
        let para = paragraph(vec![page_break_run()]);
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(())
    }

    /// Set starting page number on the section properties (`w:pgNumType w:start`).
    pub fn set_page_number_start(&mut self, start: u32) -> Result<()> {
        let body = self.body_mut()?;
        if let Some(sect) = body.child_mut("sectPr") {
            sect.children.retain(|c| c.local_name != "pgNumType");
            sect.append_child(
                OpenXmlElement::w("pgNumType")
                    .with_attribute_qname("w:start", start.to_string()),
            );
        } else {
            let mut sect = crate::wordprocessing::section_properties();
            sect.append_child(
                OpenXmlElement::w("pgNumType")
                    .with_attribute_qname("w:start", start.to_string()),
            );
            body.append_child(sect);
        }
        Ok(())
    }

    /// Clear page number start (alias for [`clear_page_number_type_start`](Self::clear_page_number_type_start)).
    pub fn clear_page_number_start(&mut self) -> Result<bool> {
        self.clear_page_number_type_start()
    }

    /// Set document-level footnote properties in settings (`w:footnotePr`).
    ///
    /// `pos` e.g. `"pageBottom"`, `"beneathText"`, `"sectEnd"`, `"docEnd"`.
    /// `num_fmt` e.g. `"decimal"`, `"lowerRoman"`, `"upperLetter"`.
    /// `num_start` starting number; `num_restart` e.g. `"continuous"`, `"eachSect"`, `"eachPage"`.
    pub fn set_footnote_properties(
        &mut self,
        pos: Option<&str>,
        num_fmt: Option<&str>,
        num_start: Option<u32>,
        num_restart: Option<&str>,
    ) -> Result<()> {
        self.set_note_properties("footnotePr", pos, num_fmt, num_start, num_restart)
    }

    /// Set document-level endnote properties in settings (`w:endnotePr`).
    pub fn set_endnote_properties(
        &mut self,
        pos: Option<&str>,
        num_fmt: Option<&str>,
        num_start: Option<u32>,
        num_restart: Option<&str>,
    ) -> Result<()> {
        self.set_note_properties("endnotePr", pos, num_fmt, num_start, num_restart)
    }

    fn set_note_properties(
        &mut self,
        local_name: &str,
        pos: Option<&str>,
        num_fmt: Option<&str>,
        num_start: Option<u32>,
        num_restart: Option<&str>,
    ) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        root.children.retain(|c| c.local_name != local_name);
        let mut pr = OpenXmlElement::w(local_name);
        if let Some(p) = pos {
            pr.append_child(OpenXmlElement::w("pos").with_attribute_qname("w:val", p));
        }
        if let Some(f) = num_fmt {
            pr.append_child(OpenXmlElement::w("numFmt").with_attribute_qname("w:val", f));
        }
        if let Some(s) = num_start {
            pr.append_child(
                OpenXmlElement::w("numStart").with_attribute_qname("w:val", s.to_string()),
            );
        }
        if let Some(r) = num_restart {
            pr.append_child(
                OpenXmlElement::w("numRestart").with_attribute_qname("w:val", r),
            );
        }
        root.append_child(pr);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read footnote properties as `(pos?, num_fmt?, num_start?, num_restart?)`.
    pub fn footnote_properties(
        &self,
    ) -> Result<Option<(Option<String>, Option<String>, Option<u32>, Option<String>)>> {
        self.note_properties("footnotePr")
    }

    /// Read endnote properties as `(pos?, num_fmt?, num_start?, num_restart?)`.
    pub fn endnote_properties(
        &self,
    ) -> Result<Option<(Option<String>, Option<String>, Option<u32>, Option<String>)>> {
        self.note_properties("endnotePr")
    }

    fn note_properties(
        &self,
        local_name: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<u32>, Option<String>)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(pr) = root.child(local_name) else {
            return Ok(None);
        };
        let attr = |el: &OpenXmlElement| {
            el.get_attribute_qname("w:val")
                .or_else(|| el.get_attribute("val"))
                .map(|s| s.to_string())
        };
        let pos = pr.child("pos").and_then(attr);
        let num_fmt = pr.child("numFmt").and_then(attr);
        let num_start = pr
            .child("numStart")
            .and_then(attr)
            .and_then(|s| s.parse().ok());
        let num_restart = pr.child("numRestart").and_then(attr);
        Ok(Some((pos, num_fmt, num_start, num_restart)))
    }

    /// Whether footnotePr is present in settings.
    pub fn has_footnote_properties(&self) -> Result<bool> {
        self.settings_has_child("footnotePr")
    }

    /// Whether endnotePr is present in settings.
    pub fn has_endnote_properties(&self) -> Result<bool> {
        self.settings_has_child("endnotePr")
    }

    /// Clear footnotePr from settings.
    pub fn clear_footnote_properties(&mut self) -> Result<bool> {
        self.remove_settings_child("footnotePr")
    }

    /// Clear endnotePr from settings.
    pub fn clear_endnote_properties(&mut self) -> Result<bool> {
        self.remove_settings_child("endnotePr")
    }

    /// Enable mirror margins (`w:mirrorMargins` in settings).
    pub fn set_mirror_margins(&mut self, enabled: bool) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children.retain(|c| c.local_name != "mirrorMargins");
        if enabled {
            root.append_child(OpenXmlElement::w("mirrorMargins"));
        }
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Whether mirror margins are enabled in settings.
    pub fn mirror_margins_enabled(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("mirrorMargins").is_some())
    }

    /// Disable mirror margins. Returns whether they were enabled.
    pub fn clear_mirror_margins(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "mirrorMargins");
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package.set_part(
                settings_uri,
                content_type::WORD_SETTINGS,
                xml,
            );
        }
        Ok(removed)
    }

    /// Enable updating fields when the document is opened (`w:updateFields`).
    pub fn set_update_fields_on_open(&mut self, enabled: bool) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children.retain(|c| c.local_name != "updateFields");
        if enabled {
            root.append_child(
                OpenXmlElement::w("updateFields").with_attribute_qname("w:val", "true"),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Enable or disable track revisions in document settings (`w:trackRevisions`).
    pub fn set_track_revisions(&mut self, enabled: bool) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children.retain(|c| c.local_name != "trackRevisions");
        if enabled {
            root.append_child(OpenXmlElement::w("trackRevisions"));
        }
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Whether settings enable track revisions (`w:trackRevisions` present).
    pub fn track_revisions_enabled(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("trackRevisions").is_some())
    }


    /// Alias for [`track_revisions_enabled`](Self::track_revisions_enabled).
    pub fn has_track_revisions(&self) -> Result<bool> {
        self.track_revisions_enabled()
    }


    /// Enable/disable `w:doNotHyphenateCaps` style flag as `w:autoHyphenation` inverse helper:
    /// sets/clears `w:autoHyphenation` when enabled=false/true respectively for simple control,
    /// and exposes `w:doNotHyphenateCaps` separately.
    /// Disable track revisions. Returns whether it was enabled.
    pub fn clear_track_revisions(&mut self) -> Result<bool> {
        let had = self.has_track_revisions()?;
        if had {
            self.set_track_revisions(false)?;
        }
        Ok(had)
    }

    pub fn set_auto_hyphenation(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("autoHyphenation", |_| {})
        } else {
            let _ = self.remove_settings_child("autoHyphenation")?;
            Ok(())
        }
    }

    /// Whether `w:autoHyphenation` is present.
    pub fn has_auto_hyphenation(&self) -> Result<bool> {
        Ok(self.settings_child_attr("autoHyphenation", "val")?.is_some()
            || self.settings_has_child("autoHyphenation")?)
    }

    /// Set `w:embedTrueTypeFonts` presence.
    /// Disable `auto hyphenation`. Returns whether it was enabled.
    pub fn clear_auto_hyphenation(&mut self) -> Result<bool> {
        let had = self.has_auto_hyphenation()?;
        if had {
            self.set_auto_hyphenation(false)?;
        }
        Ok(had)
    }

    pub fn set_embed_true_type_fonts(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("embedTrueTypeFonts", |_| {})
        } else {
            let _ = self.remove_settings_child("embedTrueTypeFonts")?;
            Ok(())
        }
    }

    /// Whether embedTrueTypeFonts is enabled.
    pub fn has_embed_true_type_fonts(&self) -> Result<bool> {
        self.settings_has_child("embedTrueTypeFonts")
    }

    /// Set `w:savePreviewPicture` presence.
    /// Disable embed-true-type-fonts. Returns whether it was enabled.
    pub fn clear_embed_true_type_fonts(&mut self) -> Result<bool> {
        let had = self.has_embed_true_type_fonts()?;
        if had {
            self.set_embed_true_type_fonts(false)?;
        }
        Ok(had)
    }

    pub fn set_save_preview_picture(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("savePreviewPicture", |_| {})
        } else {
            let _ = self.remove_settings_child("savePreviewPicture")?;
            Ok(())
        }
    }

    /// Whether savePreviewPicture is present.
    pub fn has_save_preview_picture(&self) -> Result<bool> {
        self.settings_has_child("savePreviewPicture")
    }

    /// Alias for [`mirror_margins_enabled`](Self::mirror_margins_enabled).
    /// Disable `save preview picture`. Returns whether it was enabled.
    pub fn clear_save_preview_picture(&mut self) -> Result<bool> {
        let had = self.has_save_preview_picture()?;
        if had {
            self.set_save_preview_picture(false)?;
        }
        Ok(had)
    }

    pub fn has_mirror_margins(&self) -> Result<bool> {
        self.mirror_margins_enabled()
    }

    /// Set `w:gutterAtTop` presence.
    pub fn set_gutter_at_top(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("gutterAtTop", |_| {})
        } else {
            let _ = self.remove_settings_child("gutterAtTop")?;
            Ok(())
        }
    }

    /// Whether gutterAtTop is present.
    pub fn has_gutter_at_top(&self) -> Result<bool> {
        self.settings_has_child("gutterAtTop")
    }

    fn settings_has_child(&self, local_name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child(local_name).is_some())
    }


    /// Set `w:hideSpellingErrors` presence.
    /// Disable `gutter at top`. Returns whether it was enabled.
    pub fn clear_gutter_at_top(&mut self) -> Result<bool> {
        let had = self.has_gutter_at_top()?;
        if had {
            self.set_gutter_at_top(false)?;
        }
        Ok(had)
    }

    pub fn set_hide_spelling_errors(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("hideSpellingErrors", |_| {})
        } else {
            let _ = self.remove_settings_child("hideSpellingErrors")?;
            Ok(())
        }
    }

    /// Whether hideSpellingErrors is present.
    pub fn has_hide_spelling_errors(&self) -> Result<bool> {
        self.settings_has_child("hideSpellingErrors")
    }

    /// Set `w:hideGrammaticalErrors` presence.
    /// Disable `hide spelling errors`. Returns whether it was enabled.
    pub fn clear_hide_spelling_errors(&mut self) -> Result<bool> {
        let had = self.has_hide_spelling_errors()?;
        if had {
            self.set_hide_spelling_errors(false)?;
        }
        Ok(had)
    }

    pub fn set_hide_grammatical_errors(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("hideGrammaticalErrors", |_| {})
        } else {
            let _ = self.remove_settings_child("hideGrammaticalErrors")?;
            Ok(())
        }
    }

    /// Whether hideGrammaticalErrors is present.
    pub fn has_hide_grammatical_errors(&self) -> Result<bool> {
        self.settings_has_child("hideGrammaticalErrors")
    }

    /// Set proofing state (`w:proofState` with `w:spelling` / `w:grammar` = `"clean"` or `"dirty"`).
    /// Disable `hide grammatical errors`. Returns whether it was enabled.
    pub fn clear_hide_grammatical_errors(&mut self) -> Result<bool> {
        let had = self.has_hide_grammatical_errors()?;
        if had {
            self.set_hide_grammatical_errors(false)?;
        }
        Ok(had)
    }

    pub fn set_proof_state(&mut self, spelling: &str, grammar: &str) -> Result<()> {
        self.upsert_settings_child("proofState", |el| {
            el.set_attribute_qname("w:spelling", spelling);
            el.set_attribute_qname("w:grammar", grammar);
        })
    }

    /// Read proofState as `(spelling, grammar)`.
    pub fn proof_state(&self) -> Result<Option<(String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(ps) = root.child("proofState") else {
            return Ok(None);
        };
        let spelling = ps
            .get_attribute("spelling")
            .or_else(|| ps.get_attribute_qname("w:spelling"))
            .unwrap_or("dirty")
            .to_string();
        let grammar = ps
            .get_attribute("grammar")
            .or_else(|| ps.get_attribute_qname("w:grammar"))
            .unwrap_or("dirty")
            .to_string();
        Ok(Some((spelling, grammar)))
    }

    /// Whether proofState is present.
    pub fn has_proof_state(&self) -> Result<bool> {
        self.settings_has_child("proofState")
    }

    /// Clear proofState.
    pub fn clear_proof_state(&mut self) -> Result<bool> {
        self.remove_settings_child("proofState")
    }

    /// Set `w:printHiddenText` presence.
    pub fn set_print_hidden_text(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("printHiddenText", |_| {})
        } else {
            let _ = self.remove_settings_child("printHiddenText")?;
            Ok(())
        }
    }

    /// Whether printHiddenText is present.
    pub fn has_print_hidden_text(&self) -> Result<bool> {
        self.settings_has_child("printHiddenText")
    }

    /// Set `w:printFormsData` presence.
    /// Disable `print hidden text`. Returns whether it was enabled.
    pub fn clear_print_hidden_text(&mut self) -> Result<bool> {
        let had = self.has_print_hidden_text()?;
        if had {
            self.set_print_hidden_text(false)?;
        }
        Ok(had)
    }

    pub fn set_print_forms_data(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("printFormsData", |_| {})
        } else {
            let _ = self.remove_settings_child("printFormsData")?;
            Ok(())
        }
    }

    /// Whether printFormsData is present.
    pub fn has_print_forms_data(&self) -> Result<bool> {
        self.settings_has_child("printFormsData")
    }


    /// Set `w:displayBackgroundShape` presence.
    /// Disable `print forms data`. Returns whether it was enabled.
    pub fn clear_print_forms_data(&mut self) -> Result<bool> {
        let had = self.has_print_forms_data()?;
        if had {
            self.set_print_forms_data(false)?;
        }
        Ok(had)
    }

    pub fn set_display_background_shape(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("displayBackgroundShape", |_| {})
        } else {
            let _ = self.remove_settings_child("displayBackgroundShape")?;
            Ok(())
        }
    }

    /// Whether `w:displayBackgroundShape` is present.
    pub fn has_display_background_shape(&self) -> Result<bool> {
        self.settings_has_child("displayBackgroundShape")
    }


    /// Set `w:doNotDisplayPageBoundaries` presence.
    /// Disable `display background shape`. Returns whether it was enabled.
    pub fn clear_display_background_shape(&mut self) -> Result<bool> {
        let had = self.has_display_background_shape()?;
        if had {
            self.set_display_background_shape(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_display_page_boundaries(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotDisplayPageBoundaries", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotDisplayPageBoundaries")?;
            Ok(())
        }
    }

    /// Whether `w:doNotDisplayPageBoundaries` is present.
    pub fn has_do_not_display_page_boundaries(&self) -> Result<bool> {
        self.settings_has_child("doNotDisplayPageBoundaries")
    }


    /// Set `w:doNotAutoCompressPictures` presence.
    /// Disable `do_not_display_page_boundaries`. Returns whether it was enabled.
    pub fn clear_do_not_display_page_boundaries(&mut self) -> Result<bool> {
        let had = self.has_do_not_display_page_boundaries()?;
        if had {
            self.set_do_not_display_page_boundaries(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_auto_compress_pictures(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotAutoCompressPictures", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotAutoCompressPictures")?;
            Ok(())
        }
    }

    /// Whether `w:doNotAutoCompressPictures` is present.
    pub fn has_do_not_auto_compress_pictures(&self) -> Result<bool> {
        self.settings_has_child("doNotAutoCompressPictures")
    }

    /// Set `w:doNotIncludeSubdocsInStats` presence.
    /// Disable `do_not_auto_compress_pictures`. Returns whether it was enabled.
    pub fn clear_do_not_auto_compress_pictures(&mut self) -> Result<bool> {
        let had = self.has_do_not_auto_compress_pictures()?;
        if had {
            self.set_do_not_auto_compress_pictures(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_include_subdocs_in_stats(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotIncludeSubdocsInStats", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotIncludeSubdocsInStats")?;
            Ok(())
        }
    }

    /// Whether doNotIncludeSubdocsInStats is present.
    pub fn has_do_not_include_subdocs_in_stats(&self) -> Result<bool> {
        self.settings_has_child("doNotIncludeSubdocsInStats")
    }

    /// Set `w:printTwoOnOne` presence.
    /// Disable `do not include subdocs in stats`. Returns whether it was enabled.
    pub fn clear_do_not_include_subdocs_in_stats(&mut self) -> Result<bool> {
        let had = self.has_do_not_include_subdocs_in_stats()?;
        if had {
            self.set_do_not_include_subdocs_in_stats(false)?;
        }
        Ok(had)
    }

    pub fn set_print_two_on_one(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("printTwoOnOne", |_| {})
        } else {
            let _ = self.remove_settings_child("printTwoOnOne")?;
            Ok(())
        }
    }

    /// Whether `w:printTwoOnOne` is present.
    pub fn has_print_two_on_one(&self) -> Result<bool> {
        self.settings_has_child("printTwoOnOne")
    }


    /// Set `w:strictFirstAndLastChars` presence.
    /// Disable `print_two_on_one`. Returns whether it was enabled.
    pub fn clear_print_two_on_one(&mut self) -> Result<bool> {
        let had = self.has_print_two_on_one()?;
        if had {
            self.set_print_two_on_one(false)?;
        }
        Ok(had)
    }

    pub fn set_strict_first_and_last_chars(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("strictFirstAndLastChars", |_| {})
        } else {
            let _ = self.remove_settings_child("strictFirstAndLastChars")?;
            Ok(())
        }
    }

    /// Whether `w:strictFirstAndLastChars` is present.
    pub fn has_strict_first_and_last_chars(&self) -> Result<bool> {
        self.settings_has_child("strictFirstAndLastChars")
    }


    /// Set `w:formsDesign` presence.
    /// Disable strict first/last chars. Returns whether it was enabled.
    pub fn clear_strict_first_and_last_chars(&mut self) -> Result<bool> {
        let had = self.has_strict_first_and_last_chars()?;
        if had {
            self.set_strict_first_and_last_chars(false)?;
        }
        Ok(had)
    }

    pub fn set_forms_design(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("formsDesign", |_| {})
        } else {
            let _ = self.remove_settings_child("formsDesign")?;
            Ok(())
        }
    }

    /// Whether `w:formsDesign` is present.
    pub fn has_forms_design(&self) -> Result<bool> {
        self.settings_has_child("formsDesign")
    }


    /// Set `w:removeDateAndTime` presence.
    /// Disable `forms_design`. Returns whether it was enabled.
    pub fn clear_forms_design(&mut self) -> Result<bool> {
        let had = self.has_forms_design()?;
        if had {
            self.set_forms_design(false)?;
        }
        Ok(had)
    }

    pub fn set_remove_date_and_time(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("removeDateAndTime", |_| {})
        } else {
            let _ = self.remove_settings_child("removeDateAndTime")?;
            Ok(())
        }
    }

    /// Whether `w:removeDateAndTime` is present.
    pub fn has_remove_date_and_time(&self) -> Result<bool> {
        self.settings_has_child("removeDateAndTime")
    }


    /// Set `w:removePersonalInformation` presence.
    /// Disable `remove date and time`. Returns whether it was enabled.
    pub fn clear_remove_date_and_time(&mut self) -> Result<bool> {
        let had = self.has_remove_date_and_time()?;
        if had {
            self.set_remove_date_and_time(false)?;
        }
        Ok(had)
    }

    pub fn set_remove_personal_information(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("removePersonalInformation", |_| {})
        } else {
            let _ = self.remove_settings_child("removePersonalInformation")?;
            Ok(())
        }
    }

    /// Whether `w:removePersonalInformation` is present.
    pub fn has_remove_personal_information(&self) -> Result<bool> {
        self.settings_has_child("removePersonalInformation")
    }


    /// Set `w:doNotShadeFormData` presence.
    /// Disable `remove_personal_information`. Returns whether it was enabled.
    pub fn clear_remove_personal_information(&mut self) -> Result<bool> {
        let had = self.has_remove_personal_information()?;
        if had {
            self.set_remove_personal_information(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_shade_form_data(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotShadeFormData", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotShadeFormData")?;
            Ok(())
        }
    }

    /// Whether `w:doNotShadeFormData` is present.
    pub fn has_do_not_shade_form_data(&self) -> Result<bool> {
        self.settings_has_child("doNotShadeFormData")
    }

    /// Set `w:saveFormsData` presence.
    /// Disable `do not shade form data`. Returns whether it was enabled.
    pub fn clear_do_not_shade_form_data(&mut self) -> Result<bool> {
        let had = self.has_do_not_shade_form_data()?;
        if had {
            self.set_do_not_shade_form_data(false)?;
        }
        Ok(had)
    }

    pub fn set_save_forms_data(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("saveFormsData", |_| {})
        } else {
            let _ = self.remove_settings_child("saveFormsData")?;
            Ok(())
        }
    }

    /// Whether `w:saveFormsData` is present.
    pub fn has_save_forms_data(&self) -> Result<bool> {
        self.settings_has_child("saveFormsData")
    }

    /// Set `w:doNotEmbedSmartTags` presence.
    /// Disable `save_forms_data`. Returns whether it was enabled.
    pub fn clear_save_forms_data(&mut self) -> Result<bool> {
        let had = self.has_save_forms_data()?;
        if had {
            self.set_save_forms_data(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_embed_smart_tags(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotEmbedSmartTags", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotEmbedSmartTags")?;
            Ok(())
        }
    }

    /// Whether `w:doNotEmbedSmartTags` is present.
    pub fn has_do_not_embed_smart_tags(&self) -> Result<bool> {
        self.settings_has_child("doNotEmbedSmartTags")
    }

    /// Set `w:printFractionalCharacterWidth` presence.
    /// Disable `do_not_embed_smart_tags`. Returns whether it was enabled.
    pub fn clear_do_not_embed_smart_tags(&mut self) -> Result<bool> {
        let had = self.has_do_not_embed_smart_tags()?;
        if had {
            self.set_do_not_embed_smart_tags(false)?;
        }
        Ok(had)
    }

    pub fn set_print_fractional_character_width(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("printFractionalCharacterWidth", |_| {})
        } else {
            let _ = self.remove_settings_child("printFractionalCharacterWidth")?;
            Ok(())
        }
    }

    /// Whether `w:printFractionalCharacterWidth` is present.
    pub fn has_print_fractional_character_width(&self) -> Result<bool> {
        self.settings_has_child("printFractionalCharacterWidth")
    }


    /// Set `w:printPostScriptOverText` presence.
    /// Disable `print fractional character width`. Returns whether it was enabled.
    pub fn clear_print_fractional_character_width(&mut self) -> Result<bool> {
        let had = self.has_print_fractional_character_width()?;
        if had {
            self.set_print_fractional_character_width(false)?;
        }
        Ok(had)
    }

    pub fn set_print_post_script_over_text(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("printPostScriptOverText", |_| {})
        } else {
            let _ = self.remove_settings_child("printPostScriptOverText")?;
            Ok(())
        }
    }

    /// Whether `w:printPostScriptOverText` is present.
    pub fn has_print_post_script_over_text(&self) -> Result<bool> {
        self.settings_has_child("printPostScriptOverText")
    }


    /// Set `w:alignBordersAndEdges` presence.
    /// Disable `print post script over text`. Returns whether it was enabled.
    pub fn clear_print_post_script_over_text(&mut self) -> Result<bool> {
        let had = self.has_print_post_script_over_text()?;
        if had {
            self.set_print_post_script_over_text(false)?;
        }
        Ok(had)
    }

    pub fn set_align_borders_and_edges(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("alignBordersAndEdges", |_| {})
        } else {
            let _ = self.remove_settings_child("alignBordersAndEdges")?;
            Ok(())
        }
    }

    /// Whether `w:alignBordersAndEdges` is present.
    pub fn has_align_borders_and_edges(&self) -> Result<bool> {
        self.settings_has_child("alignBordersAndEdges")
    }


    /// Set `w:bordersDoNotSurroundHeader` presence.
    /// Disable `align borders and edges`. Returns whether it was enabled.
    pub fn clear_align_borders_and_edges(&mut self) -> Result<bool> {
        let had = self.has_align_borders_and_edges()?;
        if had {
            self.set_align_borders_and_edges(false)?;
        }
        Ok(had)
    }

    pub fn set_borders_do_not_surround_header(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("bordersDoNotSurroundHeader", |_| {})
        } else {
            let _ = self.remove_settings_child("bordersDoNotSurroundHeader")?;
            Ok(())
        }
    }

    /// Whether `w:bordersDoNotSurroundHeader` is present.
    pub fn has_borders_do_not_surround_header(&self) -> Result<bool> {
        self.settings_has_child("bordersDoNotSurroundHeader")
    }


    /// Set `w:bordersDoNotSurroundFooter` presence.
    /// Disable `borders do not surround header`. Returns whether it was enabled.
    pub fn clear_borders_do_not_surround_header(&mut self) -> Result<bool> {
        let had = self.has_borders_do_not_surround_header()?;
        if had {
            self.set_borders_do_not_surround_header(false)?;
        }
        Ok(had)
    }

    pub fn set_borders_do_not_surround_footer(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("bordersDoNotSurroundFooter", |_| {})
        } else {
            let _ = self.remove_settings_child("bordersDoNotSurroundFooter")?;
            Ok(())
        }
    }

    /// Whether `w:bordersDoNotSurroundFooter` is present.
    pub fn has_borders_do_not_surround_footer(&self) -> Result<bool> {
        self.settings_has_child("bordersDoNotSurroundFooter")
    }


    /// Set `w:doNotUseHTMLParagraphAutoSpacing` presence.
    /// Disable `borders do not surround footer`. Returns whether it was enabled.
    pub fn clear_borders_do_not_surround_footer(&mut self) -> Result<bool> {
        let had = self.has_borders_do_not_surround_footer()?;
        if had {
            self.set_borders_do_not_surround_footer(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_use_html_paragraph_auto_spacing(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotUseHTMLParagraphAutoSpacing", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotUseHTMLParagraphAutoSpacing")?;
            Ok(())
        }
    }

    /// Whether `w:doNotUseHTMLParagraphAutoSpacing` is present.
    pub fn has_do_not_use_html_paragraph_auto_spacing(&self) -> Result<bool> {
        self.settings_has_child("doNotUseHTMLParagraphAutoSpacing")
    }


    /// Set `w:doNotUseIndentAsNumberingTabStop` presence.
    /// Disable `do not use html paragraph auto spacing`. Returns whether it was enabled.
    pub fn clear_do_not_use_html_paragraph_auto_spacing(&mut self) -> Result<bool> {
        let had = self.has_do_not_use_html_paragraph_auto_spacing()?;
        if had {
            self.set_do_not_use_html_paragraph_auto_spacing(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_use_indent_as_numbering_tab_stop(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotUseIndentAsNumberingTabStop", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotUseIndentAsNumberingTabStop")?;
            Ok(())
        }
    }

    /// Whether `w:doNotUseIndentAsNumberingTabStop` is present.
    pub fn has_do_not_use_indent_as_numbering_tab_stop(&self) -> Result<bool> {
        self.settings_has_child("doNotUseIndentAsNumberingTabStop")
    }

    /// Set `w:growAutofit` presence.
    /// Disable `do not use indent as numbering tab stop`. Returns whether it was enabled.
    pub fn clear_do_not_use_indent_as_numbering_tab_stop(&mut self) -> Result<bool> {
        let had = self.has_do_not_use_indent_as_numbering_tab_stop()?;
        if had {
            self.set_do_not_use_indent_as_numbering_tab_stop(false)?;
        }
        Ok(had)
    }

    pub fn set_grow_autofit(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("growAutofit", |_| {})
        } else {
            let _ = self.remove_settings_child("growAutofit")?;
            Ok(())
        }
    }

    /// Whether `w:growAutofit` is present.
    pub fn has_grow_autofit(&self) -> Result<bool> {
        self.settings_has_child("growAutofit")
    }

    /// Set `w:useNormalStyleForList` presence.
    /// Disable `grow autofit`. Returns whether it was enabled.
    pub fn clear_grow_autofit(&mut self) -> Result<bool> {
        let had = self.has_grow_autofit()?;
        if had {
            self.set_grow_autofit(false)?;
        }
        Ok(had)
    }

    pub fn set_use_normal_style_for_list(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("useNormalStyleForList", |_| {})
        } else {
            let _ = self.remove_settings_child("useNormalStyleForList")?;
            Ok(())
        }
    }

    /// Whether `w:useNormalStyleForList` is present.
    pub fn has_use_normal_style_for_list(&self) -> Result<bool> {
        self.settings_has_child("useNormalStyleForList")
    }

    /// Set `w:useWord2002TableStyleRules` presence.
    /// Disable `use normal style for list`. Returns whether it was enabled.
    pub fn clear_use_normal_style_for_list(&mut self) -> Result<bool> {
        let had = self.has_use_normal_style_for_list()?;
        if had {
            self.set_use_normal_style_for_list(false)?;
        }
        Ok(had)
    }

    pub fn set_use_word2002_table_style_rules(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("useWord2002TableStyleRules", |_| {})
        } else {
            let _ = self.remove_settings_child("useWord2002TableStyleRules")?;
            Ok(())
        }
    }

    /// Whether `w:useWord2002TableStyleRules` is present.
    pub fn has_use_word2002_table_style_rules(&self) -> Result<bool> {
        self.settings_has_child("useWord2002TableStyleRules")
    }

    /// Set `w:layoutRawTableWidth` presence.
    /// Disable `use word2002 table style rules`. Returns whether it was enabled.
    pub fn clear_use_word2002_table_style_rules(&mut self) -> Result<bool> {
        let had = self.has_use_word2002_table_style_rules()?;
        if had {
            self.set_use_word2002_table_style_rules(false)?;
        }
        Ok(had)
    }

    pub fn set_layout_raw_table_width(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("layoutRawTableWidth", |_| {})
        } else {
            let _ = self.remove_settings_child("layoutRawTableWidth")?;
            Ok(())
        }
    }

    /// Whether `w:layoutRawTableWidth` is present.
    pub fn has_layout_raw_table_width(&self) -> Result<bool> {
        self.settings_has_child("layoutRawTableWidth")
    }

    /// Set `w:layoutTableRowsApart` presence.
    /// Disable `layout raw table width`. Returns whether it was enabled.
    pub fn clear_layout_raw_table_width(&mut self) -> Result<bool> {
        let had = self.has_layout_raw_table_width()?;
        if had {
            self.set_layout_raw_table_width(false)?;
        }
        Ok(had)
    }

    pub fn set_layout_table_rows_apart(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("layoutTableRowsApart", |_| {})
        } else {
            let _ = self.remove_settings_child("layoutTableRowsApart")?;
            Ok(())
        }
    }

    /// Whether `w:layoutTableRowsApart` is present.
    pub fn has_layout_table_rows_apart(&self) -> Result<bool> {
        self.settings_has_child("layoutTableRowsApart")
    }

    /// Set `w:useSingleBorderforContiguousCells` presence.
    /// Disable `layout table rows apart`. Returns whether it was enabled.
    pub fn clear_layout_table_rows_apart(&mut self) -> Result<bool> {
        let had = self.has_layout_table_rows_apart()?;
        if had {
            self.set_layout_table_rows_apart(false)?;
        }
        Ok(had)
    }

    pub fn set_use_single_border_for_contiguous_cells(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("useSingleBorderforContiguousCells", |_| {})
        } else {
            let _ = self.remove_settings_child("useSingleBorderforContiguousCells")?;
            Ok(())
        }
    }

    /// Whether `w:useSingleBorderforContiguousCells` is present.
    pub fn has_use_single_border_for_contiguous_cells(&self) -> Result<bool> {
        self.settings_has_child("useSingleBorderforContiguousCells")
    }

    /// Set `w:doNotAutofitConstrainedTables` presence.
    /// Disable `use single border for contiguous cells`. Returns whether it was enabled.
    pub fn clear_use_single_border_for_contiguous_cells(&mut self) -> Result<bool> {
        let had = self.has_use_single_border_for_contiguous_cells()?;
        if had {
            self.set_use_single_border_for_contiguous_cells(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_autofit_constrained_tables(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotAutofitConstrainedTables", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotAutofitConstrainedTables")?;
            Ok(())
        }
    }

    /// Whether `w:doNotAutofitConstrainedTables` is present.
    pub fn has_do_not_autofit_constrained_tables(&self) -> Result<bool> {
        self.settings_has_child("doNotAutofitConstrainedTables")
    }

    /// Set `w:autofitToFirstFixedWidthCell` presence.
    /// Disable `do not autofit constrained tables`. Returns whether it was enabled.
    pub fn clear_do_not_autofit_constrained_tables(&mut self) -> Result<bool> {
        let had = self.has_do_not_autofit_constrained_tables()?;
        if had {
            self.set_do_not_autofit_constrained_tables(false)?;
        }
        Ok(had)
    }

    pub fn set_autofit_to_first_fixed_width_cell(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("autofitToFirstFixedWidthCell", |_| {})
        } else {
            let _ = self.remove_settings_child("autofitToFirstFixedWidthCell")?;
            Ok(())
        }
    }

    /// Whether `w:autofitToFirstFixedWidthCell` is present.
    pub fn has_autofit_to_first_fixed_width_cell(&self) -> Result<bool> {
        self.settings_has_child("autofitToFirstFixedWidthCell")
    }

    /// Set `w:displayHangulFixedWidth` presence.
    /// Disable `autofit to first fixed width cell`. Returns whether it was enabled.
    pub fn clear_autofit_to_first_fixed_width_cell(&mut self) -> Result<bool> {
        let had = self.has_autofit_to_first_fixed_width_cell()?;
        if had {
            self.set_autofit_to_first_fixed_width_cell(false)?;
        }
        Ok(had)
    }

    pub fn set_display_hangul_fixed_width(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("displayHangulFixedWidth", |_| {})
        } else {
            let _ = self.remove_settings_child("displayHangulFixedWidth")?;
            Ok(())
        }
    }

    /// Whether `w:displayHangulFixedWidth` is present.
    pub fn has_display_hangul_fixed_width(&self) -> Result<bool> {
        self.settings_has_child("displayHangulFixedWidth")
    }

    /// Set `w:splitPgBreakAndParaMark` presence.
    /// Disable `display hangul fixed width`. Returns whether it was enabled.
    pub fn clear_display_hangul_fixed_width(&mut self) -> Result<bool> {
        let had = self.has_display_hangul_fixed_width()?;
        if had {
            self.set_display_hangul_fixed_width(false)?;
        }
        Ok(had)
    }

    pub fn set_split_pg_break_and_para_mark(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("splitPgBreakAndParaMark", |_| {})
        } else {
            let _ = self.remove_settings_child("splitPgBreakAndParaMark")?;
            Ok(())
        }
    }

    /// Whether `w:splitPgBreakAndParaMark` is present.
    pub fn has_split_pg_break_and_para_mark(&self) -> Result<bool> {
        self.settings_has_child("splitPgBreakAndParaMark")
    }

    /// Set `w:doNotBreakConstrainedForcedTable` presence.
    /// Disable `split pg break and para mark`. Returns whether it was enabled.
    pub fn clear_split_pg_break_and_para_mark(&mut self) -> Result<bool> {
        let had = self.has_split_pg_break_and_para_mark()?;
        if had {
            self.set_split_pg_break_and_para_mark(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_break_constrained_forced_table(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotBreakConstrainedForcedTable", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotBreakConstrainedForcedTable")?;
            Ok(())
        }
    }

    /// Whether `w:doNotBreakConstrainedForcedTable` is present.
    pub fn has_do_not_break_constrained_forced_table(&self) -> Result<bool> {
        self.settings_has_child("doNotBreakConstrainedForcedTable")
    }

    /// Set `w:doNotVertAlignCellWithSp` presence.
    /// Disable `do not break constrained forced table`. Returns whether it was enabled.
    pub fn clear_do_not_break_constrained_forced_table(&mut self) -> Result<bool> {
        let had = self.has_do_not_break_constrained_forced_table()?;
        if had {
            self.set_do_not_break_constrained_forced_table(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_vert_align_cell_with_sp(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotVertAlignCellWithSp", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotVertAlignCellWithSp")?;
            Ok(())
        }
    }

    /// Whether `w:doNotVertAlignCellWithSp` is present.
    pub fn has_do_not_vert_align_cell_with_sp(&self) -> Result<bool> {
        self.settings_has_child("doNotVertAlignCellWithSp")
    }


    /// Set `w:doNotVertAlignInTxbx` presence.
    /// Disable `do not vert align cell with sp`. Returns whether it was enabled.
    pub fn clear_do_not_vert_align_cell_with_sp(&mut self) -> Result<bool> {
        let had = self.has_do_not_vert_align_cell_with_sp()?;
        if had {
            self.set_do_not_vert_align_cell_with_sp(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_vert_align_in_txbx(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotVertAlignInTxbx", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotVertAlignInTxbx")?;
            Ok(())
        }
    }

    /// Whether `w:doNotVertAlignInTxbx` is present.
    pub fn has_do_not_vert_align_in_txbx(&self) -> Result<bool> {
        self.settings_has_child("doNotVertAlignInTxbx")
    }


    /// Set `w:doNotWrapTextWithPunct` presence.
    /// Disable `do not vert align in txbx`. Returns whether it was enabled.
    pub fn clear_do_not_vert_align_in_txbx(&mut self) -> Result<bool> {
        let had = self.has_do_not_vert_align_in_txbx()?;
        if had {
            self.set_do_not_vert_align_in_txbx(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_wrap_text_with_punct(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotWrapTextWithPunct", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotWrapTextWithPunct")?;
            Ok(())
        }
    }

    /// Whether `w:doNotWrapTextWithPunct` is present.
    pub fn has_do_not_wrap_text_with_punct(&self) -> Result<bool> {
        self.settings_has_child("doNotWrapTextWithPunct")
    }

    /// Set `w:doNotBreakWrappedTables` presence.
    /// Disable `do not wrap text with punct`. Returns whether it was enabled.
    pub fn clear_do_not_wrap_text_with_punct(&mut self) -> Result<bool> {
        let had = self.has_do_not_wrap_text_with_punct()?;
        if had {
            self.set_do_not_wrap_text_with_punct(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_break_wrapped_tables(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotBreakWrappedTables", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotBreakWrappedTables")?;
            Ok(())
        }
    }

    /// Whether `w:doNotBreakWrappedTables` is present.
    pub fn has_do_not_break_wrapped_tables(&self) -> Result<bool> {
        self.settings_has_child("doNotBreakWrappedTables")
    }

    /// Set `w:doNotSnapToGridInCell` presence.
    /// Disable `do not break wrapped tables`. Returns whether it was enabled.
    pub fn clear_do_not_break_wrapped_tables(&mut self) -> Result<bool> {
        let had = self.has_do_not_break_wrapped_tables()?;
        if had {
            self.set_do_not_break_wrapped_tables(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_snap_to_grid_in_cell(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotSnapToGridInCell", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotSnapToGridInCell")?;
            Ok(())
        }
    }

    /// Whether `w:doNotSnapToGridInCell` is present.
    pub fn has_do_not_snap_to_grid_in_cell(&self) -> Result<bool> {
        self.settings_has_child("doNotSnapToGridInCell")
    }

    /// Set `w:selectFldWithFirstOrLastChar` presence.
    /// Disable `do not snap to grid in cell`. Returns whether it was enabled.
    pub fn clear_do_not_snap_to_grid_in_cell(&mut self) -> Result<bool> {
        let had = self.has_do_not_snap_to_grid_in_cell()?;
        if had {
            self.set_do_not_snap_to_grid_in_cell(false)?;
        }
        Ok(had)
    }

    pub fn set_select_fld_with_first_or_last_char(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("selectFldWithFirstOrLastChar", |_| {})
        } else {
            let _ = self.remove_settings_child("selectFldWithFirstOrLastChar")?;
            Ok(())
        }
    }

    /// Whether `w:selectFldWithFirstOrLastChar` is present.
    pub fn has_select_fld_with_first_or_last_char(&self) -> Result<bool> {
        self.settings_has_child("selectFldWithFirstOrLastChar")
    }

    /// Set `w:doNotUseEastAsianBreakRules` presence.
    /// Disable `select fld with first or last char`. Returns whether it was enabled.
    pub fn clear_select_fld_with_first_or_last_char(&mut self) -> Result<bool> {
        let had = self.has_select_fld_with_first_or_last_char()?;
        if had {
            self.set_select_fld_with_first_or_last_char(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_use_east_asian_break_rules(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotUseEastAsianBreakRules", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotUseEastAsianBreakRules")?;
            Ok(())
        }
    }

    /// Whether `w:doNotUseEastAsianBreakRules` is present.
    pub fn has_do_not_use_east_asian_break_rules(&self) -> Result<bool> {
        self.settings_has_child("doNotUseEastAsianBreakRules")
    }

    /// Set `w:useAltKinsokuLineBreakRules` presence.
    /// Disable `do not use east asian break rules`. Returns whether it was enabled.
    pub fn clear_do_not_use_east_asian_break_rules(&mut self) -> Result<bool> {
        let had = self.has_do_not_use_east_asian_break_rules()?;
        if had {
            self.set_do_not_use_east_asian_break_rules(false)?;
        }
        Ok(had)
    }

    pub fn set_use_alt_kinsoku_line_break_rules(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("useAltKinsokuLineBreakRules", |_| {})
        } else {
            let _ = self.remove_settings_child("useAltKinsokuLineBreakRules")?;
            Ok(())
        }
    }

    /// Whether `w:useAltKinsokuLineBreakRules` is present.
    pub fn has_use_alt_kinsoku_line_break_rules(&self) -> Result<bool> {
        self.settings_has_child("useAltKinsokuLineBreakRules")
    }

    /// Set `w:doNotLeaveBackslashAlone` presence.
    /// Disable `use alt kinsoku line break rules`. Returns whether it was enabled.
    pub fn clear_use_alt_kinsoku_line_break_rules(&mut self) -> Result<bool> {
        let had = self.has_use_alt_kinsoku_line_break_rules()?;
        if had {
            self.set_use_alt_kinsoku_line_break_rules(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_leave_backslash_alone(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotLeaveBackslashAlone", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotLeaveBackslashAlone")?;
            Ok(())
        }
    }

    /// Whether `w:doNotLeaveBackslashAlone` is present.
    pub fn has_do_not_leave_backslash_alone(&self) -> Result<bool> {
        self.settings_has_child("doNotLeaveBackslashAlone")
    }

    /// Set `w:ulTrailSpace` presence (underline trailing spaces).
    /// Disable `do not leave backslash alone`. Returns whether it was enabled.
    pub fn clear_do_not_leave_backslash_alone(&mut self) -> Result<bool> {
        let had = self.has_do_not_leave_backslash_alone()?;
        if had {
            self.set_do_not_leave_backslash_alone(false)?;
        }
        Ok(had)
    }

    pub fn set_ul_trail_space(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("ulTrailSpace", |_| {})
        } else {
            let _ = self.remove_settings_child("ulTrailSpace")?;
            Ok(())
        }
    }

    /// Whether `w:ulTrailSpace` is present.
    pub fn has_ul_trail_space(&self) -> Result<bool> {
        self.settings_has_child("ulTrailSpace")
    }

    /// Set `w:printBodyTextBeforeHeader` presence.
    /// Disable `ul trail space`. Returns whether it was enabled.
    pub fn clear_ul_trail_space(&mut self) -> Result<bool> {
        let had = self.has_ul_trail_space()?;
        if had {
            self.set_ul_trail_space(false)?;
        }
        Ok(had)
    }

    pub fn set_print_body_text_before_header(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("printBodyTextBeforeHeader", |_| {})
        } else {
            let _ = self.remove_settings_child("printBodyTextBeforeHeader")?;
            Ok(())
        }
    }

    /// Whether `w:printBodyTextBeforeHeader` is present.
    pub fn has_print_body_text_before_header(&self) -> Result<bool> {
        self.settings_has_child("printBodyTextBeforeHeader")
    }

    /// Set `w:printColBlack` presence (print in black and white).
    /// Disable `print body text before header`. Returns whether it was enabled.
    pub fn clear_print_body_text_before_header(&mut self) -> Result<bool> {
        let had = self.has_print_body_text_before_header()?;
        if had {
            self.set_print_body_text_before_header(false)?;
        }
        Ok(had)
    }

    pub fn set_print_col_black(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("printColBlack", |_| {})
        } else {
            let _ = self.remove_settings_child("printColBlack")?;
            Ok(())
        }
    }

    /// Whether `w:printColBlack` is present.
    pub fn has_print_col_black(&self) -> Result<bool> {
        self.settings_has_child("printColBlack")
    }

    /// Set `w:mwSmallCaps` presence (Mac Word small caps).
    /// Disable `print col black`. Returns whether it was enabled.
    pub fn clear_print_col_black(&mut self) -> Result<bool> {
        let had = self.has_print_col_black()?;
        if had {
            self.set_print_col_black(false)?;
        }
        Ok(had)
    }

    pub fn set_mw_small_caps(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("mwSmallCaps", |_| {})
        } else {
            let _ = self.remove_settings_child("mwSmallCaps")?;
            Ok(())
        }
    }

    /// Whether `w:mwSmallCaps` is present.
    pub fn has_mw_small_caps(&self) -> Result<bool> {
        self.settings_has_child("mwSmallCaps")
    }

    /// Set `w:shapeLayoutLikeWW8` presence.
    /// Disable `mw small caps`. Returns whether it was enabled.
    pub fn clear_mw_small_caps(&mut self) -> Result<bool> {
        let had = self.has_mw_small_caps()?;
        if had {
            self.set_mw_small_caps(false)?;
        }
        Ok(had)
    }

    pub fn set_shape_layout_like_ww8(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("shapeLayoutLikeWW8", |_| {})
        } else {
            let _ = self.remove_settings_child("shapeLayoutLikeWW8")?;
            Ok(())
        }
    }

    /// Whether `w:shapeLayoutLikeWW8` is present.
    pub fn has_shape_layout_like_ww8(&self) -> Result<bool> {
        self.settings_has_child("shapeLayoutLikeWW8")
    }

    /// Set `w:footnoteLayoutLikeWW8` presence.
    /// Disable `shape layout like ww8`. Returns whether it was enabled.
    pub fn clear_shape_layout_like_ww8(&mut self) -> Result<bool> {
        let had = self.has_shape_layout_like_ww8()?;
        if had {
            self.set_shape_layout_like_ww8(false)?;
        }
        Ok(had)
    }

    pub fn set_footnote_layout_like_ww8(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("footnoteLayoutLikeWW8", |_| {})
        } else {
            let _ = self.remove_settings_child("footnoteLayoutLikeWW8")?;
            Ok(())
        }
    }

    /// Whether `w:footnoteLayoutLikeWW8` is present.
    pub fn has_footnote_layout_like_ww8(&self) -> Result<bool> {
        self.settings_has_child("footnoteLayoutLikeWW8")
    }

    /// Set `w:spaceForUL` presence (add space for underline).
    /// Disable `footnote layout like ww8`. Returns whether it was enabled.
    pub fn clear_footnote_layout_like_ww8(&mut self) -> Result<bool> {
        let had = self.has_footnote_layout_like_ww8()?;
        if had {
            self.set_footnote_layout_like_ww8(false)?;
        }
        Ok(had)
    }

    pub fn set_space_for_ul(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("spaceForUL", |_| {})
        } else {
            let _ = self.remove_settings_child("spaceForUL")?;
            Ok(())
        }
    }

    /// Whether `w:spaceForUL` is present.
    pub fn has_space_for_ul(&self) -> Result<bool> {
        self.settings_has_child("spaceForUL")
    }

    /// Set `w:noPunctuationKerning` presence.
    /// Disable `space for ul`. Returns whether it was enabled.
    pub fn clear_space_for_ul(&mut self) -> Result<bool> {
        let had = self.has_space_for_ul()?;
        if had {
            self.set_space_for_ul(false)?;
        }
        Ok(had)
    }

    pub fn set_no_punctuation_kerning(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("noPunctuationKerning", |_| {})
        } else {
            let _ = self.remove_settings_child("noPunctuationKerning")?;
            Ok(())
        }
    }

    /// Whether `w:noPunctuationKerning` is present.
    pub fn has_no_punctuation_kerning(&self) -> Result<bool> {
        self.settings_has_child("noPunctuationKerning")
    }


    /// Set custom characters that cannot end a line (`w:noLineBreaksAfter` with lang/val).
    ///
    /// Note: the class name is `NoLineBreaksAfterKinsoku`; the element local name is
    /// `noLineBreaksAfter`.
    /// Disable `no punctuation kerning`. Returns whether it was enabled.
    pub fn clear_no_punctuation_kerning(&mut self) -> Result<bool> {
        let had = self.has_no_punctuation_kerning()?;
        if had {
            self.set_no_punctuation_kerning(false)?;
        }
        Ok(had)
    }

    pub fn set_no_line_breaks_after(&mut self, lang: &str, chars: &str) -> Result<()> {
        self.upsert_settings_child("noLineBreaksAfter", |el| {
            el.set_attribute_qname("w:lang", lang);
            el.set_attribute_qname("w:val", chars);
        })
    }

    /// Read `(lang, val)` for `w:noLineBreaksAfter`.
    pub fn no_line_breaks_after(&self) -> Result<Option<(String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(el) = root.child("noLineBreaksAfter") else {
            return Ok(None);
        };
        let lang = el
            .get_attribute("lang")
            .or_else(|| el.get_attribute_qname("w:lang"))
            .unwrap_or("")
            .to_string();
        let val = el
            .get_attribute("val")
            .or_else(|| el.get_attribute_qname("w:val"))
            .unwrap_or("")
            .to_string();
        Ok(Some((lang, val)))
    }

    /// Whether `w:noLineBreaksAfter` is present.
    pub fn has_no_line_breaks_after(&self) -> Result<bool> {
        self.settings_has_child("noLineBreaksAfter")
    }

    /// Clear `w:noLineBreaksAfter`.
    pub fn clear_no_line_breaks_after(&mut self) -> Result<bool> {
        self.remove_settings_child("noLineBreaksAfter")
    }

    /// Alias for [`set_no_line_breaks_after`](Self::set_no_line_breaks_after).
    pub fn set_no_line_breaks_after_kinsoku(&mut self, lang: &str, chars: &str) -> Result<()> {
        self.set_no_line_breaks_after(lang, chars)
    }

    /// Alias for [`has_no_line_breaks_after`](Self::has_no_line_breaks_after).
    pub fn has_no_line_breaks_after_kinsoku(&self) -> Result<bool> {
        self.has_no_line_breaks_after()
    }

    /// Set custom characters that cannot begin a line (`w:noLineBreaksBefore` with lang/val).
    /// Alias for [`clear_no_line_breaks_after`](Self::clear_no_line_breaks_after).
    pub fn clear_no_line_breaks_after_kinsoku(&mut self) -> Result<bool> {
        self.clear_no_line_breaks_after()
    }

    pub fn set_no_line_breaks_before(&mut self, lang: &str, chars: &str) -> Result<()> {
        self.upsert_settings_child("noLineBreaksBefore", |el| {
            el.set_attribute_qname("w:lang", lang);
            el.set_attribute_qname("w:val", chars);
        })
    }

    /// Read `(lang, val)` for `w:noLineBreaksBefore`.
    pub fn no_line_breaks_before(&self) -> Result<Option<(String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(el) = root.child("noLineBreaksBefore") else {
            return Ok(None);
        };
        let lang = el
            .get_attribute("lang")
            .or_else(|| el.get_attribute_qname("w:lang"))
            .unwrap_or("")
            .to_string();
        let val = el
            .get_attribute("val")
            .or_else(|| el.get_attribute_qname("w:val"))
            .unwrap_or("")
            .to_string();
        Ok(Some((lang, val)))
    }

    /// Whether `w:noLineBreaksBefore` is present.
    pub fn has_no_line_breaks_before(&self) -> Result<bool> {
        self.settings_has_child("noLineBreaksBefore")
    }

    /// Clear `w:noLineBreaksBefore`.
    pub fn clear_no_line_breaks_before(&mut self) -> Result<bool> {
        self.remove_settings_child("noLineBreaksBefore")
    }

    /// Alias for [`set_no_line_breaks_before`](Self::set_no_line_breaks_before).
    pub fn set_no_line_breaks_before_kinsoku(&mut self, lang: &str, chars: &str) -> Result<()> {
        self.set_no_line_breaks_before(lang, chars)
    }

    /// Alias for [`has_no_line_breaks_before`](Self::has_no_line_breaks_before).
    pub fn has_no_line_breaks_before_kinsoku(&self) -> Result<bool> {
        self.has_no_line_breaks_before()
    }

    /// Set `w:doNotSuppressIndentation` presence.
    /// Alias for [`clear_no_line_breaks_before`](Self::clear_no_line_breaks_before).
    pub fn clear_no_line_breaks_before_kinsoku(&mut self) -> Result<bool> {
        self.clear_no_line_breaks_before()
    }

    pub fn set_do_not_suppress_indentation(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotSuppressIndentation", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotSuppressIndentation")?;
            Ok(())
        }
    }

    /// Whether `w:doNotSuppressIndentation` is present.
    pub fn has_do_not_suppress_indentation(&self) -> Result<bool> {
        self.settings_has_child("doNotSuppressIndentation")
    }

    /// Set `w:alignTablesRowByRow` presence.
    /// Disable `do not suppress indentation`. Returns whether it was enabled.
    pub fn clear_do_not_suppress_indentation(&mut self) -> Result<bool> {
        let had = self.has_do_not_suppress_indentation()?;
        if had {
            self.set_do_not_suppress_indentation(false)?;
        }
        Ok(had)
    }

    pub fn set_align_tables_row_by_row(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("alignTablesRowByRow", |_| {})
        } else {
            let _ = self.remove_settings_child("alignTablesRowByRow")?;
            Ok(())
        }
    }

    /// Whether `w:alignTablesRowByRow` is present.
    pub fn has_align_tables_row_by_row(&self) -> Result<bool> {
        self.settings_has_child("alignTablesRowByRow")
    }

    /// Set `w:forgetLastTabAlignment` presence.
    /// Disable `align tables row by row`. Returns whether it was enabled.
    pub fn clear_align_tables_row_by_row(&mut self) -> Result<bool> {
        let had = self.has_align_tables_row_by_row()?;
        if had {
            self.set_align_tables_row_by_row(false)?;
        }
        Ok(had)
    }

    pub fn set_forget_last_tab_alignment(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("forgetLastTabAlignment", |_| {})
        } else {
            let _ = self.remove_settings_child("forgetLastTabAlignment")?;
            Ok(())
        }
    }

    /// Whether `w:forgetLastTabAlignment` is present.
    pub fn has_forget_last_tab_alignment(&self) -> Result<bool> {
        self.settings_has_child("forgetLastTabAlignment")
    }

    /// Set `w:useAnsiKerningPairs` presence.
    /// Disable `forget last tab alignment`. Returns whether it was enabled.
    pub fn clear_forget_last_tab_alignment(&mut self) -> Result<bool> {
        let had = self.has_forget_last_tab_alignment()?;
        if had {
            self.set_forget_last_tab_alignment(false)?;
        }
        Ok(had)
    }

    pub fn set_use_ansi_kerning_pairs(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("useAnsiKerningPairs", |_| {})
        } else {
            let _ = self.remove_settings_child("useAnsiKerningPairs")?;
            Ok(())
        }
    }

    /// Whether `w:useAnsiKerningPairs` is present.
    pub fn has_use_ansi_kerning_pairs(&self) -> Result<bool> {
        self.settings_has_child("useAnsiKerningPairs")
    }

    /// Set `w:cachedColBalance` presence.
    /// Disable `use ansi kerning pairs`. Returns whether it was enabled.
    pub fn clear_use_ansi_kerning_pairs(&mut self) -> Result<bool> {
        let had = self.has_use_ansi_kerning_pairs()?;
        if had {
            self.set_use_ansi_kerning_pairs(false)?;
        }
        Ok(had)
    }

    pub fn set_cached_col_balance(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("cachedColBalance", |_| {})
        } else {
            let _ = self.remove_settings_child("cachedColBalance")?;
            Ok(())
        }
    }

    /// Whether `w:cachedColBalance` is present.
    pub fn has_cached_col_balance(&self) -> Result<bool> {
        self.settings_has_child("cachedColBalance")
    }

    /// Set `w:doNotSuppressParagraphBorders` presence.
    /// Disable `cached col balance`. Returns whether it was enabled.
    pub fn clear_cached_col_balance(&mut self) -> Result<bool> {
        let had = self.has_cached_col_balance()?;
        if had {
            self.set_cached_col_balance(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_suppress_paragraph_borders(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotSuppressParagraphBorders", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotSuppressParagraphBorders")?;
            Ok(())
        }
    }

    /// Whether `w:doNotSuppressParagraphBorders` is present.
    pub fn has_do_not_suppress_paragraph_borders(&self) -> Result<bool> {
        self.settings_has_child("doNotSuppressParagraphBorders")
    }


    /// Set `w:suppressBottomSpacing` presence.
    /// Disable `do not suppress paragraph borders`. Returns whether it was enabled.
    pub fn clear_do_not_suppress_paragraph_borders(&mut self) -> Result<bool> {
        let had = self.has_do_not_suppress_paragraph_borders()?;
        if had {
            self.set_do_not_suppress_paragraph_borders(false)?;
        }
        Ok(had)
    }

    pub fn set_suppress_bottom_spacing(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("suppressBottomSpacing", |_| {})
        } else {
            let _ = self.remove_settings_child("suppressBottomSpacing")?;
            Ok(())
        }
    }

    /// Whether `w:suppressBottomSpacing` is present.
    pub fn has_suppress_bottom_spacing(&self) -> Result<bool> {
        self.settings_has_child("suppressBottomSpacing")
    }


    /// Set `w:suppressTopSpacing` presence.
    /// Disable `suppress bottom spacing`. Returns whether it was enabled.
    pub fn clear_suppress_bottom_spacing(&mut self) -> Result<bool> {
        let had = self.has_suppress_bottom_spacing()?;
        if had {
            self.set_suppress_bottom_spacing(false)?;
        }
        Ok(had)
    }

    pub fn set_suppress_top_spacing(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("suppressTopSpacing", |_| {})
        } else {
            let _ = self.remove_settings_child("suppressTopSpacing")?;
            Ok(())
        }
    }

    /// Whether `w:suppressTopSpacing` is present.
    pub fn has_suppress_top_spacing(&self) -> Result<bool> {
        self.settings_has_child("suppressTopSpacing")
    }


    /// Set `w:suppressSpacingAtTopOfPage` presence.
    /// Disable `suppress top spacing`. Returns whether it was enabled.
    pub fn clear_suppress_top_spacing(&mut self) -> Result<bool> {
        let had = self.has_suppress_top_spacing()?;
        if had {
            self.set_suppress_top_spacing(false)?;
        }
        Ok(had)
    }

    pub fn set_suppress_spacing_at_top_of_page(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("suppressSpacingAtTopOfPage", |_| {})
        } else {
            let _ = self.remove_settings_child("suppressSpacingAtTopOfPage")?;
            Ok(())
        }
    }

    /// Whether `w:suppressSpacingAtTopOfPage` is present.
    pub fn has_suppress_spacing_at_top_of_page(&self) -> Result<bool> {
        self.settings_has_child("suppressSpacingAtTopOfPage")
    }


    /// Set `w:swapBordersFacingPages` presence.
    /// Disable `suppress spacing at top of page`. Returns whether it was enabled.
    pub fn clear_suppress_spacing_at_top_of_page(&mut self) -> Result<bool> {
        let had = self.has_suppress_spacing_at_top_of_page()?;
        if had {
            self.set_suppress_spacing_at_top_of_page(false)?;
        }
        Ok(had)
    }

    pub fn set_swap_borders_facing_pages(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("swapBordersFacingPages", |_| {})
        } else {
            let _ = self.remove_settings_child("swapBordersFacingPages")?;
            Ok(())
        }
    }

    /// Whether `w:swapBordersFacingPages` is present.
    pub fn has_swap_borders_facing_pages(&self) -> Result<bool> {
        self.settings_has_child("swapBordersFacingPages")
    }

    /// Set `w:suppressSpBfAfterPgBrk` presence.
    /// Disable `swap borders facing pages`. Returns whether it was enabled.
    pub fn clear_swap_borders_facing_pages(&mut self) -> Result<bool> {
        let had = self.has_swap_borders_facing_pages()?;
        if had {
            self.set_swap_borders_facing_pages(false)?;
        }
        Ok(had)
    }

    pub fn set_suppress_sp_bf_after_pg_brk(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("suppressSpBfAfterPgBrk", |_| {})
        } else {
            let _ = self.remove_settings_child("suppressSpBfAfterPgBrk")?;
            Ok(())
        }
    }

    /// Whether `w:suppressSpBfAfterPgBrk` is present.
    pub fn has_suppress_sp_bf_after_pg_brk(&self) -> Result<bool> {
        self.settings_has_child("suppressSpBfAfterPgBrk")
    }

    /// Set `w:convMailMergeEsc` presence.
    /// Disable `suppress sp bf after pg brk`. Returns whether it was enabled.
    pub fn clear_suppress_sp_bf_after_pg_brk(&mut self) -> Result<bool> {
        let had = self.has_suppress_sp_bf_after_pg_brk()?;
        if had {
            self.set_suppress_sp_bf_after_pg_brk(false)?;
        }
        Ok(had)
    }

    pub fn set_conv_mail_merge_esc(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("convMailMergeEsc", |_| {})
        } else {
            let _ = self.remove_settings_child("convMailMergeEsc")?;
            Ok(())
        }
    }

    /// Whether `w:convMailMergeEsc` is present.
    pub fn has_conv_mail_merge_esc(&self) -> Result<bool> {
        self.settings_has_child("convMailMergeEsc")
    }

    /// Set `w:truncateFontHeightsLikeWP6` presence.
    /// Disable `conv mail merge esc`. Returns whether it was enabled.
    pub fn clear_conv_mail_merge_esc(&mut self) -> Result<bool> {
        let had = self.has_conv_mail_merge_esc()?;
        if had {
            self.set_conv_mail_merge_esc(false)?;
        }
        Ok(had)
    }

    pub fn set_truncate_font_heights_like_wp6(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("truncateFontHeightsLikeWP6", |_| {})
        } else {
            let _ = self.remove_settings_child("truncateFontHeightsLikeWP6")?;
            Ok(())
        }
    }

    /// Whether `w:truncateFontHeightsLikeWP6` is present.
    pub fn has_truncate_font_heights_like_wp6(&self) -> Result<bool> {
        self.settings_has_child("truncateFontHeightsLikeWP6")
    }

    /// Set `w:subFontBySize` presence.
    /// Disable `truncate font heights like wp6`. Returns whether it was enabled.
    pub fn clear_truncate_font_heights_like_wp6(&mut self) -> Result<bool> {
        let had = self.has_truncate_font_heights_like_wp6()?;
        if had {
            self.set_truncate_font_heights_like_wp6(false)?;
        }
        Ok(had)
    }

    pub fn set_sub_font_by_size(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("subFontBySize", |_| {})
        } else {
            let _ = self.remove_settings_child("subFontBySize")?;
            Ok(())
        }
    }

    /// Whether `w:subFontBySize` is present.
    pub fn has_sub_font_by_size(&self) -> Result<bool> {
        self.settings_has_child("subFontBySize")
    }

    /// Set `w:compat/w:balanceSingleByteDoubleByteWidth` presence.
    /// Disable `sub font by size`. Returns whether it was enabled.
    pub fn clear_sub_font_by_size(&mut self) -> Result<bool> {
        let had = self.has_sub_font_by_size()?;
        if had {
            self.set_sub_font_by_size(false)?;
        }
        Ok(had)
    }

    pub fn set_balance_single_byte_double_byte_width(&mut self, enabled: bool) -> Result<()> {
        self.set_compat_flag("balanceSingleByteDoubleByteWidth", enabled)
    }

    /// Whether `w:compat/w:balanceSingleByteDoubleByteWidth` is present.
    pub fn has_balance_single_byte_double_byte_width(&self) -> Result<bool> {
        self.has_compat_flag("balanceSingleByteDoubleByteWidth")
    }


    /// Set `w:useFELayout` presence.
    /// Disable `balance single byte double byte width`. Returns whether it was enabled.
    pub fn clear_balance_single_byte_double_byte_width(&mut self) -> Result<bool> {
        let had = self.has_balance_single_byte_double_byte_width()?;
        if had {
            self.set_balance_single_byte_double_byte_width(false)?;
        }
        Ok(had)
    }

    pub fn set_use_fe_layout(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("useFELayout", |_| {})
        } else {
            let _ = self.remove_settings_child("useFELayout")?;
            Ok(())
        }
    }

    /// Whether `w:useFELayout` is present.
    pub fn has_use_fe_layout(&self) -> Result<bool> {
        self.settings_has_child("useFELayout")
    }


    /// Set `w:characterSpacingControl w:val` (e.g. `"doNotCompress"`, `"compressPunctuation"`).
    /// Disable `use fe layout`. Returns whether it was enabled.
    pub fn clear_use_fe_layout(&mut self) -> Result<bool> {
        let had = self.has_use_fe_layout()?;
        if had {
            self.set_use_fe_layout(false)?;
        }
        Ok(had)
    }

    pub fn set_character_spacing_control(&mut self, val: &str) -> Result<()> {
        self.upsert_settings_child("characterSpacingControl", |el| {
            el.set_attribute_qname("w:val", val);
        })
    }


    /// Set settings attached template path (`w:attachedTemplate r:id` requires rel);
    /// this stores `w:attachedTemplate` with `r:id` after creating an external relationship.
    pub fn set_attached_template(&mut self, template_path: &str) -> Result<String> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        let rid = self
            .package
            .opc_mut()
            .part_relationships_mut(&settings_uri)
            .add(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/attachedTemplate",
                template_path,
                RelationshipTargetMode::External,
            )
            .id
            .clone();
        root.children.retain(|c| c.local_name != "attachedTemplate");
        root.append_child(
            OpenXmlElement::w("attachedTemplate").with_attribute_qname("r:id", &rid),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(rid)
    }

    /// Whether attachedTemplate is present in settings.
    pub fn has_attached_template(&self) -> Result<bool> {
        self.settings_has_child("attachedTemplate")
    }

    /// Remove attachedTemplate element and its relationship when possible.
    pub fn clear_attached_template(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let rid = root
            .child("attachedTemplate")
            .and_then(|e| {
                e.get_attribute_qname("r:id")
                    .or_else(|| e.get_attribute("id"))
                    .map(|s| s.to_string())
            });
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "attachedTemplate");
        let removed = root.children.len() < before;
        if let Some(id) = rid {
            self.package
                .opc_mut()
                .part_relationships_mut(&settings_uri)
                .remove(&id);
        }
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(removed)
    }

    /// Read characterSpacingControl val.
    pub fn character_spacing_control(&self) -> Result<Option<String>> {
        self.settings_child_attr("characterSpacingControl", "val")
    }

    /// Whether characterSpacingControl is present.
    pub fn has_character_spacing_control(&self) -> Result<bool> {
        Ok(self.character_spacing_control()?.is_some())
    }

    /// Remove characterSpacingControl.
    pub fn clear_character_spacing_control(&mut self) -> Result<bool> {
        self.remove_settings_child("characterSpacingControl")
    }

    /// Set decimal symbol (`w:decimalSymbol w:val`), e.g. `"."` or `","`.
    pub fn set_decimal_symbol(&mut self, symbol: &str) -> Result<()> {
        self.upsert_settings_child("decimalSymbol", |el| {
            el.set_attribute_qname("w:val", symbol);
        })
    }

    /// Read decimal symbol.
    pub fn decimal_symbol(&self) -> Result<Option<String>> {
        self.settings_child_attr("decimalSymbol", "val")
    }

    /// Whether decimalSymbol is set.
    pub fn has_decimal_symbol(&self) -> Result<bool> {
        Ok(self.decimal_symbol()?.is_some())
    }

    /// Clear decimalSymbol.
    pub fn clear_decimal_symbol(&mut self) -> Result<bool> {
        self.remove_settings_child("decimalSymbol")
    }

    /// Set list separator (`w:listSeparator w:val`), e.g. `","` or `";"`.
    pub fn set_list_separator(&mut self, separator: &str) -> Result<()> {
        self.upsert_settings_child("listSeparator", |el| {
            el.set_attribute_qname("w:val", separator);
        })
    }

    /// Read list separator.
    pub fn list_separator(&self) -> Result<Option<String>> {
        self.settings_child_attr("listSeparator", "val")
    }

    /// Whether listSeparator is set.
    pub fn has_list_separator(&self) -> Result<bool> {
        Ok(self.list_separator()?.is_some())
    }

    /// Clear listSeparator.
    pub fn clear_list_separator(&mut self) -> Result<bool> {
        self.remove_settings_child("listSeparator")
    }

    /// Set consecutive hyphen limit (`w:consecutiveHyphenLimit w:val`).
    pub fn set_consecutive_hyphen_limit(&mut self, limit: u32) -> Result<()> {
        self.upsert_settings_child("consecutiveHyphenLimit", |el| {
            el.set_attribute_qname("w:val", limit.to_string());
        })
    }

    /// Read consecutiveHyphenLimit.
    pub fn consecutive_hyphen_limit(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("consecutiveHyphenLimit", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Clear consecutiveHyphenLimit.
    pub fn clear_consecutive_hyphen_limit(&mut self) -> Result<bool> {
        self.remove_settings_child("consecutiveHyphenLimit")
    }

    /// Set hyphenation zone in twips (`w:hyphenationZone w:val`).
    pub fn set_hyphenation_zone(&mut self, twips: u32) -> Result<()> {
        self.upsert_settings_child("hyphenationZone", |el| {
            el.set_attribute_qname("w:val", twips.to_string());
        })
    }

    /// Read hyphenationZone in twips.
    pub fn hyphenation_zone(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("hyphenationZone", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Clear hyphenationZone.
    pub fn clear_hyphenation_zone(&mut self) -> Result<bool> {
        self.remove_settings_child("hyphenationZone")
    }

    /// Set `w:doNotHyphenateCaps` presence.
    pub fn set_do_not_hyphenate_caps(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotHyphenateCaps", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotHyphenateCaps")?;
            Ok(())
        }
    }

    /// Whether doNotHyphenateCaps is present.
    pub fn has_do_not_hyphenate_caps(&self) -> Result<bool> {
        self.settings_has_child("doNotHyphenateCaps")
    }

    /// Set `w:saveSubsetFonts` presence.
    /// Disable `do not hyphenate caps`. Returns whether it was enabled.
    pub fn clear_do_not_hyphenate_caps(&mut self) -> Result<bool> {
        let had = self.has_do_not_hyphenate_caps()?;
        if had {
            self.set_do_not_hyphenate_caps(false)?;
        }
        Ok(had)
    }

    pub fn set_save_subset_fonts(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("saveSubsetFonts", |_| {})
        } else {
            let _ = self.remove_settings_child("saveSubsetFonts")?;
            Ok(())
        }
    }

    /// Whether saveSubsetFonts is present.
    pub fn has_save_subset_fonts(&self) -> Result<bool> {
        self.settings_has_child("saveSubsetFonts")
    }

    /// Set `w:embedSystemFonts` presence.
    /// Disable `save subset fonts`. Returns whether it was enabled.
    pub fn clear_save_subset_fonts(&mut self) -> Result<bool> {
        let had = self.has_save_subset_fonts()?;
        if had {
            self.set_save_subset_fonts(false)?;
        }
        Ok(had)
    }

    pub fn set_embed_system_fonts(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("embedSystemFonts", |_| {})
        } else {
            let _ = self.remove_settings_child("embedSystemFonts")?;
            Ok(())
        }
    }

    /// Whether embedSystemFonts is present.
    pub fn has_embed_system_fonts(&self) -> Result<bool> {
        self.settings_has_child("embedSystemFonts")
    }

    /// Set `w:linkStyles` presence.
    /// Disable `embed system fonts`. Returns whether it was enabled.
    pub fn clear_embed_system_fonts(&mut self) -> Result<bool> {
        let had = self.has_embed_system_fonts()?;
        if had {
            self.set_embed_system_fonts(false)?;
        }
        Ok(had)
    }

    pub fn set_link_styles(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("linkStyles", |_| {})
        } else {
            let _ = self.remove_settings_child("linkStyles")?;
            Ok(())
        }
    }

    /// Whether linkStyles is present.
    pub fn has_link_styles(&self) -> Result<bool> {
        self.settings_has_child("linkStyles")
    }

    /// Set `w:styleLockTheme` presence.
    /// Disable `link styles`. Returns whether it was enabled.
    pub fn clear_link_styles(&mut self) -> Result<bool> {
        let had = self.has_link_styles()?;
        if had {
            self.set_link_styles(false)?;
        }
        Ok(had)
    }

    pub fn set_style_lock_theme(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("styleLockTheme", |_| {})
        } else {
            let _ = self.remove_settings_child("styleLockTheme")?;
            Ok(())
        }
    }

    /// Whether styleLockTheme is present.
    pub fn has_style_lock_theme(&self) -> Result<bool> {
        self.settings_has_child("styleLockTheme")
    }

    /// Set `w:styleLockQFSet` presence (lock Quick Format styles).
    /// Disable `style lock theme`. Returns whether it was enabled.
    pub fn clear_style_lock_theme(&mut self) -> Result<bool> {
        let had = self.has_style_lock_theme()?;
        if had {
            self.set_style_lock_theme(false)?;
        }
        Ok(had)
    }

    pub fn set_style_lock_qf_set(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("styleLockQFSet", |_| {})
        } else {
            let _ = self.remove_settings_child("styleLockQFSet")?;
            Ok(())
        }
    }

    /// Whether styleLockQFSet is present.
    pub fn has_style_lock_qf_set(&self) -> Result<bool> {
        self.settings_has_child("styleLockQFSet")
    }

    /// Set `w:doNotTrackMoves` presence.
    /// Disable `style lock qf set`. Returns whether it was enabled.
    pub fn clear_style_lock_qf_set(&mut self) -> Result<bool> {
        let had = self.has_style_lock_qf_set()?;
        if had {
            self.set_style_lock_qf_set(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_track_moves(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotTrackMoves", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotTrackMoves")?;
            Ok(())
        }
    }

    /// Whether doNotTrackMoves is present.
    pub fn has_do_not_track_moves(&self) -> Result<bool> {
        self.settings_has_child("doNotTrackMoves")
    }

    /// Set `w:doNotTrackFormatting` presence.
    /// Disable `do not track moves`. Returns whether it was enabled.
    pub fn clear_do_not_track_moves(&mut self) -> Result<bool> {
        let had = self.has_do_not_track_moves()?;
        if had {
            self.set_do_not_track_moves(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_track_formatting(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotTrackFormatting", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotTrackFormatting")?;
            Ok(())
        }
    }

    /// Whether doNotTrackFormatting is present.
    pub fn has_do_not_track_formatting(&self) -> Result<bool> {
        self.settings_has_child("doNotTrackFormatting")
    }

    /// Set `w:bookFoldPrinting` presence.
    /// Disable `do not track formatting`. Returns whether it was enabled.
    pub fn clear_do_not_track_formatting(&mut self) -> Result<bool> {
        let had = self.has_do_not_track_formatting()?;
        if had {
            self.set_do_not_track_formatting(false)?;
        }
        Ok(had)
    }

    pub fn set_book_fold_printing(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("bookFoldPrinting", |_| {})
        } else {
            let _ = self.remove_settings_child("bookFoldPrinting")?;
            Ok(())
        }
    }

    /// Whether bookFoldPrinting is present.
    pub fn has_book_fold_printing(&self) -> Result<bool> {
        self.settings_has_child("bookFoldPrinting")
    }

    /// Set `w:bookFoldRevPrinting` presence.
    /// Disable `book fold printing`. Returns whether it was enabled.
    pub fn clear_book_fold_printing(&mut self) -> Result<bool> {
        let had = self.has_book_fold_printing()?;
        if had {
            self.set_book_fold_printing(false)?;
        }
        Ok(had)
    }

    pub fn set_book_fold_rev_printing(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("bookFoldRevPrinting", |_| {})
        } else {
            let _ = self.remove_settings_child("bookFoldRevPrinting")?;
            Ok(())
        }
    }

    /// Whether bookFoldRevPrinting is present.
    pub fn has_book_fold_rev_printing(&self) -> Result<bool> {
        self.settings_has_child("bookFoldRevPrinting")
    }

    /// Set `w:bookFoldPrintingSheets w:val`.
    /// Disable `book fold rev printing`. Returns whether it was enabled.
    pub fn clear_book_fold_rev_printing(&mut self) -> Result<bool> {
        let had = self.has_book_fold_rev_printing()?;
        if had {
            self.set_book_fold_rev_printing(false)?;
        }
        Ok(had)
    }

    pub fn set_book_fold_printing_sheets(&mut self, sheets: u32) -> Result<()> {
        self.upsert_settings_child("bookFoldPrintingSheets", |el| {
            el.set_attribute_qname("w:val", sheets.to_string());
        })
    }

    /// Read bookFoldPrintingSheets.
    pub fn book_fold_printing_sheets(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("bookFoldPrintingSheets", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether bookFoldPrintingSheets is present.
    pub fn has_book_fold_printing_sheets(&self) -> Result<bool> {
        Ok(self.book_fold_printing_sheets()?.is_some())
    }

    /// Clear bookFoldPrintingSheets.
    pub fn clear_book_fold_printing_sheets(&mut self) -> Result<bool> {
        self.remove_settings_child("bookFoldPrintingSheets")
    }

    /// Set default table style (`w:defaultTableStyle w:val`).
    pub fn set_default_table_style(&mut self, style_id: &str) -> Result<()> {
        self.upsert_settings_child("defaultTableStyle", |el| {
            el.set_attribute_qname("w:val", style_id);
        })
    }

    /// Read defaultTableStyle.
    pub fn default_table_style(&self) -> Result<Option<String>> {
        self.settings_child_attr("defaultTableStyle", "val")
    }

    /// Clear defaultTableStyle.
    pub fn clear_default_table_style(&mut self) -> Result<bool> {
        self.remove_settings_child("defaultTableStyle")
    }

    /// Set click-and-type style (`w:clickAndTypeStyle w:val`).
    pub fn set_click_and_type_style(&mut self, style_id: &str) -> Result<()> {
        self.upsert_settings_child("clickAndTypeStyle", |el| {
            el.set_attribute_qname("w:val", style_id);
        })
    }

    /// Read clickAndTypeStyle.
    pub fn click_and_type_style(&self) -> Result<Option<String>> {
        self.settings_child_attr("clickAndTypeStyle", "val")
    }

    /// Clear clickAndTypeStyle.
    pub fn clear_click_and_type_style(&mut self) -> Result<bool> {
        self.remove_settings_child("clickAndTypeStyle")
    }

    /// Set theme font language (`w:themeFontLang` with `w:val` and optional `w:eastAsia`/`w:bidi`).
    pub fn set_theme_font_lang(&mut self, val: &str) -> Result<()> {
        self.upsert_settings_child("themeFontLang", |el| {
            el.set_attribute_qname("w:val", val);
        })
    }

    /// Set theme font languages including East Asian and bidirectional locales.
    pub fn set_theme_font_lang_ex(
        &mut self,
        val: &str,
        east_asia: Option<&str>,
        bidi: Option<&str>,
    ) -> Result<()> {
        self.upsert_settings_child("themeFontLang", |el| {
            el.set_attribute_qname("w:val", val);
            if let Some(ea) = east_asia {
                el.set_attribute_qname("w:eastAsia", ea);
            }
            if let Some(b) = bidi {
                el.set_attribute_qname("w:bidi", b);
            }
        })
    }

    /// Read themeFontLang as `(val, eastAsia?, bidi?)`.
    pub fn theme_font_lang_ex(&self) -> Result<Option<(String, Option<String>, Option<String>)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(el) = root.child("themeFontLang") else {
            return Ok(None);
        };
        let val = el
            .get_attribute_qname("w:val")
            .or_else(|| el.get_attribute("val"))
            .unwrap_or("")
            .to_string();
        let ea = el
            .get_attribute_qname("w:eastAsia")
            .or_else(|| el.get_attribute("eastAsia"))
            .map(|s| s.to_string());
        let bidi = el
            .get_attribute_qname("w:bidi")
            .or_else(|| el.get_attribute("bidi"))
            .map(|s| s.to_string());
        Ok(Some((val, ea, bidi)))
    }

    /// Clear themeFontLang eastAsia/bidi attributes (keeps `w:val` if present).
    pub fn clear_theme_font_lang_ex(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(el) = root.child_mut("themeFontLang") else {
            return Ok(false);
        };
        let before = el.attributes.len();
        el.attributes.retain(|a| a.local_name != "eastAsia" && a.local_name != "bidi");
        if el.attributes.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    /// Read themeFontLang `@w:val`.
    pub fn theme_font_lang(&self) -> Result<Option<String>> {
        self.settings_child_attr("themeFontLang", "val")
    }

    /// Clear themeFontLang.
    pub fn clear_theme_font_lang(&mut self) -> Result<bool> {
        self.remove_settings_child("themeFontLang")
    }

    /// Add an auto-caption mapping under captions (`w:autoCaptions/w:autoCaption`).
    pub fn add_auto_caption(&mut self, name: &str, caption: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        let captions = if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "captions")
        {
            &mut root.children[pos]
        } else {
            root.append_child(OpenXmlElement::w("captions"));
            root.children.last_mut().unwrap()
        };
        let auto = if let Some(pos) = captions
            .children
            .iter()
            .position(|c| c.local_name == "autoCaptions")
        {
            &mut captions.children[pos]
        } else {
            captions.append_child(OpenXmlElement::w("autoCaptions"));
            captions.children.last_mut().unwrap()
        };
        auto.children.retain(|c| {
            !(c.local_name == "autoCaption"
                && (c.get_attribute_qname("w:name").or_else(|| c.get_attribute("name"))
                    == Some(name)))
        });
        auto.append_child(
            OpenXmlElement::w("autoCaption")
                .with_attribute_qname("w:name", name)
                .with_attribute_qname("w:caption", caption),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// List auto-captions as `(name, caption)`.
    pub fn list_auto_captions(&self) -> Result<Vec<(String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(captions) = root.child("captions") else {
            return Ok(Vec::new());
        };
        let Some(auto) = captions.child("autoCaptions") else {
            return Ok(Vec::new());
        };
        Ok(auto
            .children_by_name("autoCaption")
            .map(|c| {
                (
                    c.get_attribute_qname("w:name")
                        .or_else(|| c.get_attribute("name"))
                        .unwrap_or("")
                        .to_string(),
                    c.get_attribute_qname("w:caption")
                        .or_else(|| c.get_attribute("caption"))
                        .unwrap_or("")
                        .to_string(),
                )
            })
            .collect())
    }

    /// Whether any auto-captions are configured.
    pub fn has_auto_captions(&self) -> Result<bool> {
        Ok(!self.list_auto_captions()?.is_empty())
    }

    /// Count auto-caption entries.
    pub fn auto_caption_count(&self) -> Result<usize> {
        Ok(self.list_auto_captions()?.len())
    }

    /// Remove one auto-caption by name. Returns whether found.
    pub fn remove_auto_caption(&mut self, name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(captions) = root.child_mut("captions") else {
            return Ok(false);
        };
        let Some(auto) = captions.child_mut("autoCaptions") else {
            return Ok(false);
        };
        let before = auto.children.len();
        auto.children.retain(|c| {
            !(c.local_name == "autoCaption"
                && (c.get_attribute_qname("w:name").or_else(|| c.get_attribute("name"))
                    == Some(name)))
        });
        if auto.children.len() == before {
            return Ok(false);
        }
        if auto.children.is_empty() {
            captions.children.retain(|c| c.local_name != "autoCaptions");
        }
        if captions.children.is_empty() {
            root.children.retain(|c| c.local_name != "captions");
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    /// Clear all auto-captions. Returns how many were removed.
    pub fn clear_auto_captions(&mut self) -> Result<usize> {
        let n = self.auto_caption_count()?;
        if n == 0 {
            return Ok(0);
        }
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        if let Some(captions) = root.child_mut("captions") {
            captions.children.retain(|c| c.local_name != "autoCaptions");
            if captions.children.is_empty() {
                root.children.retain(|c| c.local_name != "captions");
            }
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(n)
    }

    /// Set summary length (`w:summaryLength w:val`).
    pub fn set_summary_length(&mut self, length: u32) -> Result<()> {
        self.upsert_settings_child("summaryLength", |el| {
            el.set_attribute_qname("w:val", length.to_string());
        })
    }

    /// Read summaryLength.
    pub fn summary_length(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("summaryLength", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether summaryLength is set.
    pub fn has_summary_length(&self) -> Result<bool> {
        Ok(self.summary_length()?.is_some())
    }

    /// Clear summaryLength.
    pub fn clear_summary_length(&mut self) -> Result<bool> {
        self.remove_settings_child("summaryLength")
    }

    /// Set reading-mode ink lock-down (`w:readModeInkLockDown`).
    ///
    /// `font_sz` is typically a percent string like `"100%"` or a decimal number string.
    pub fn set_read_mode_ink_lock_down(
        &mut self,
        width: u32,
        height: u32,
        font_sz: &str,
        actual_pages: Option<bool>,
    ) -> Result<()> {
        self.upsert_settings_child("readModeInkLockDown", |el| {
            el.set_attribute_qname("w:w", width.to_string());
            el.set_attribute_qname("w:h", height.to_string());
            el.set_attribute_qname("w:fontSz", font_sz);
            if let Some(ap) = actual_pages {
                el.set_attribute_qname("w:actualPg", if ap { "1" } else { "0" });
            }
        })
    }

    /// Read readModeInkLockDown as `(width, height, font_sz, actual_pages)`.
    pub fn read_mode_ink_lock_down(
        &self,
    ) -> Result<Option<(u32, u32, String, Option<bool>)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(el) = root.child("readModeInkLockDown") else {
            return Ok(None);
        };
        let w = el
            .get_attribute("w")
            .or_else(|| el.get_attribute_qname("w:w"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let h = el
            .get_attribute("h")
            .or_else(|| el.get_attribute_qname("w:h"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let font = el
            .get_attribute("fontSz")
            .or_else(|| el.get_attribute_qname("w:fontSz"))
            .unwrap_or("100%")
            .to_string();
        let actual = el
            .get_attribute("actualPg")
            .or_else(|| el.get_attribute_qname("w:actualPg"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"));
        Ok(Some((w, h, font, actual)))
    }

    /// Whether readModeInkLockDown is present.
    pub fn has_read_mode_ink_lock_down(&self) -> Result<bool> {
        self.settings_has_child("readModeInkLockDown")
    }

    /// Clear readModeInkLockDown.
    pub fn clear_read_mode_ink_lock_down(&mut self) -> Result<bool> {
        self.remove_settings_child("readModeInkLockDown")
    }

    /// Set drawing grid horizontal spacing in twips.
    pub fn set_drawing_grid_horizontal_spacing(&mut self, twips: u32) -> Result<()> {
        self.upsert_settings_child("drawingGridHorizontalSpacing", |el| {
            el.set_attribute_qname("w:val", twips.to_string());
        })
    }

    /// Read drawingGridHorizontalSpacing.
    pub fn drawing_grid_horizontal_spacing(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("drawingGridHorizontalSpacing", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set drawing grid vertical spacing in twips.
    pub fn set_drawing_grid_vertical_spacing(&mut self, twips: u32) -> Result<()> {
        self.upsert_settings_child("drawingGridVerticalSpacing", |el| {
            el.set_attribute_qname("w:val", twips.to_string());
        })
    }

    /// Read drawingGridVerticalSpacing.
    pub fn drawing_grid_vertical_spacing(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("drawingGridVerticalSpacing", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set drawing grid horizontal origin in twips.
    pub fn set_drawing_grid_horizontal_origin(&mut self, twips: u32) -> Result<()> {
        self.upsert_settings_child("drawingGridHorizontalOrigin", |el| {
            el.set_attribute_qname("w:val", twips.to_string());
        })
    }

    /// Read drawingGridHorizontalOrigin.
    pub fn drawing_grid_horizontal_origin(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("drawingGridHorizontalOrigin", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set drawing grid vertical origin in twips.
    pub fn set_drawing_grid_vertical_origin(&mut self, twips: u32) -> Result<()> {
        self.upsert_settings_child("drawingGridVerticalOrigin", |el| {
            el.set_attribute_qname("w:val", twips.to_string());
        })
    }

    /// Read drawingGridVerticalOrigin.
    pub fn drawing_grid_vertical_origin(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("drawingGridVerticalOrigin", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set how many horizontal grid units between displayed gridlines.
    pub fn set_display_horizontal_drawing_grid_every(&mut self, n: u32) -> Result<()> {
        self.upsert_settings_child("displayHorizontalDrawingGridEvery", |el| {
            el.set_attribute_qname("w:val", n.to_string());
        })
    }

    /// Read displayHorizontalDrawingGridEvery.
    pub fn display_horizontal_drawing_grid_every(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("displayHorizontalDrawingGridEvery", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set how many vertical grid units between displayed gridlines.
    pub fn set_display_vertical_drawing_grid_every(&mut self, n: u32) -> Result<()> {
        self.upsert_settings_child("displayVerticalDrawingGridEvery", |el| {
            el.set_attribute_qname("w:val", n.to_string());
        })
    }

    /// Read displayVerticalDrawingGridEvery.
    pub fn display_vertical_drawing_grid_every(&self) -> Result<Option<u32>> {
        Ok(self
            .settings_child_attr("displayVerticalDrawingGridEvery", "val")?
            .and_then(|s| s.parse().ok()))
    }

    /// Whether drawingGridHorizontalSpacing is set.
    pub fn has_drawing_grid_horizontal_spacing(&self) -> Result<bool> {
        Ok(self.drawing_grid_horizontal_spacing()?.is_some())
    }

    /// Clear drawingGridHorizontalSpacing.
    pub fn clear_drawing_grid_horizontal_spacing(&mut self) -> Result<bool> {
        self.remove_settings_child("drawingGridHorizontalSpacing")
    }

    /// Whether drawingGridVerticalSpacing is set.
    pub fn has_drawing_grid_vertical_spacing(&self) -> Result<bool> {
        Ok(self.drawing_grid_vertical_spacing()?.is_some())
    }

    /// Clear drawingGridVerticalSpacing.
    pub fn clear_drawing_grid_vertical_spacing(&mut self) -> Result<bool> {
        self.remove_settings_child("drawingGridVerticalSpacing")
    }

    /// Whether drawingGridHorizontalOrigin is set.
    pub fn has_drawing_grid_horizontal_origin(&self) -> Result<bool> {
        Ok(self.drawing_grid_horizontal_origin()?.is_some())
    }

    /// Clear drawingGridHorizontalOrigin.
    pub fn clear_drawing_grid_horizontal_origin(&mut self) -> Result<bool> {
        self.remove_settings_child("drawingGridHorizontalOrigin")
    }

    /// Whether drawingGridVerticalOrigin is set.
    pub fn has_drawing_grid_vertical_origin(&self) -> Result<bool> {
        Ok(self.drawing_grid_vertical_origin()?.is_some())
    }

    /// Clear drawingGridVerticalOrigin.
    pub fn clear_drawing_grid_vertical_origin(&mut self) -> Result<bool> {
        self.remove_settings_child("drawingGridVerticalOrigin")
    }

    /// Whether displayHorizontalDrawingGridEvery is set.
    pub fn has_display_horizontal_drawing_grid_every(&self) -> Result<bool> {
        Ok(self.display_horizontal_drawing_grid_every()?.is_some())
    }

    /// Clear displayHorizontalDrawingGridEvery.
    pub fn clear_display_horizontal_drawing_grid_every(&mut self) -> Result<bool> {
        self.remove_settings_child("displayHorizontalDrawingGridEvery")
    }

    /// Whether displayVerticalDrawingGridEvery is set.
    pub fn has_display_vertical_drawing_grid_every(&self) -> Result<bool> {
        Ok(self.display_vertical_drawing_grid_every()?.is_some())
    }

    /// Clear displayVerticalDrawingGridEvery.
    pub fn clear_display_vertical_drawing_grid_every(&mut self) -> Result<bool> {
        self.remove_settings_child("displayVerticalDrawingGridEvery")
    }

    /// Set `w:doNotUseMarginsForDrawingGridOrigin` presence.
    pub fn set_do_not_use_margins_for_drawing_grid_origin(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotUseMarginsForDrawingGridOrigin", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotUseMarginsForDrawingGridOrigin")?;
            Ok(())
        }
    }

    /// Whether doNotUseMarginsForDrawingGridOrigin is present.
    pub fn has_do_not_use_margins_for_drawing_grid_origin(&self) -> Result<bool> {
        self.settings_has_child("doNotUseMarginsForDrawingGridOrigin")
    }

    /// Set `w:showEnvelope` presence (show mail envelope toolbar).
    /// Disable `do not use margins for drawing grid origin`. Returns whether it was enabled.
    pub fn clear_do_not_use_margins_for_drawing_grid_origin(&mut self) -> Result<bool> {
        let had = self.has_do_not_use_margins_for_drawing_grid_origin()?;
        if had {
            self.set_do_not_use_margins_for_drawing_grid_origin(false)?;
        }
        Ok(had)
    }

    pub fn set_show_envelope(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("showEnvelope", |_| {})
        } else {
            let _ = self.remove_settings_child("showEnvelope")?;
            Ok(())
        }
    }

    /// Whether showEnvelope is present.
    pub fn has_show_envelope(&self) -> Result<bool> {
        self.settings_has_child("showEnvelope")
    }

    /// Set `w:autoFormatOverride` presence.
    /// Disable `show envelope`. Returns whether it was enabled.
    pub fn clear_show_envelope(&mut self) -> Result<bool> {
        let had = self.has_show_envelope()?;
        if had {
            self.set_show_envelope(false)?;
        }
        Ok(had)
    }

    pub fn set_auto_format_override(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("autoFormatOverride", |_| {})
        } else {
            let _ = self.remove_settings_child("autoFormatOverride")?;
            Ok(())
        }
    }

    /// Whether autoFormatOverride is present.
    pub fn has_auto_format_override(&self) -> Result<bool> {
        self.settings_has_child("autoFormatOverride")
    }

    /// Set `w:uiCompat97To2003` presence (UI compatibility mode).
    /// Disable `auto format override`. Returns whether it was enabled.
    pub fn clear_auto_format_override(&mut self) -> Result<bool> {
        let had = self.has_auto_format_override()?;
        if had {
            self.set_auto_format_override(false)?;
        }
        Ok(had)
    }

    pub fn set_ui_compat_97_to_2003(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("uiCompat97To2003", |_| {})
        } else {
            let _ = self.remove_settings_child("uiCompat97To2003")?;
            Ok(())
        }
    }

    /// Whether uiCompat97To2003 is present.
    pub fn has_ui_compat_97_to_2003(&self) -> Result<bool> {
        self.settings_has_child("uiCompat97To2003")
    }

    /// Whether any altChunk parts are related from the main document.
    /// Disable `ui compat 97 to 2003`. Returns whether it was enabled.
    pub fn clear_ui_compat_97_to_2003(&mut self) -> Result<bool> {
        let had = self.has_ui_compat_97_to_2003()?;
        if had {
            self.set_ui_compat_97_to_2003(false)?;
        }
        Ok(had)
    }

    pub fn has_alt_chunks(&self) -> bool {
        !self.list_related_parts(rel::AF_CHUNK).is_empty()
    }

    /// Count altChunk import parts.
    pub fn alt_chunk_count(&self) -> usize {
        self.list_alt_chunks().len()
    }

    /// List altChunk part URIs related from the main document.
    pub fn list_alt_chunks(&self) -> Vec<PackUri> {
        self.list_related_parts(rel::AF_CHUNK)
    }

    /// Remove all altChunk parts and relationships. Returns how many were removed.
    pub fn clear_alt_chunks(&mut self) -> Result<usize> {
        let parts = self.list_alt_chunks();
        let n = parts.len();
        if n == 0 {
            return Ok(0);
        }
        for _ in 0..n {
            self.remove_related_part_at(rel::AF_CHUNK, 0)?;
        }
        // Also drop w:altChunk elements from body
        if let Ok(body) = self.body_mut() {
            body.children.retain(|c| c.local_name != "altChunk");
            // Recurse into nested? altChunk is body-level typically
        }
        Ok(n)
    }

    /// Remove an altChunk part by 0-based index and drop matching body `w:altChunk` elements.
    pub fn remove_alt_chunk_at(&mut self, index: usize) -> Result<bool> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let rels: Vec<(String, String)> = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .map(|r| {
                r.find_all_by_type(rel::AF_CHUNK)
                    .into_iter()
                    .map(|rel| (rel.id.clone(), rel.target.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let Some((rid, _target)) = rels.get(index).cloned() else {
            return Ok(false);
        };
        self.remove_related_part_at(rel::AF_CHUNK, index)?;
        // Drop body altChunk elements with matching r:id
        if let Ok(body) = self.body_mut() {
            body.children.retain(|c| {
                if c.local_name != "altChunk" {
                    return true;
                }
                let id = c
                    .get_attribute_qname("r:id")
                    .or_else(|| c.get_attribute("id"));
                id != Some(rid.as_str())
            });
        }
        Ok(true)
    }

    /// Heuristic: whether a watermark header appears present (header contains VML shape text).
    pub fn has_watermark(&self) -> bool {
        for uri in self.list_headers() {
            if let Some(data) = self.package.opc().get_part(&uri) {
                if let Ok(root) = parse_element(data) {
                    let has_vml = root.descendants().any(|e| {
                        e.local_name == "shape"
                            || e.local_name == "pict"
                            || e.local_name == "imagedata"
                    });
                    if has_vml {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Remove watermark headers (headers containing VML shape/pict). Returns how many were removed.
    pub fn clear_watermark(&mut self) -> Result<usize> {
        let headers = self.list_headers();
        let mut remove_indices = Vec::new();
        for (i, uri) in headers.iter().enumerate() {
            if let Some(data) = self.package.opc().get_part(&uri) {
                if let Ok(root) = parse_element(data) {
                    let has_vml = root.descendants().any(|e| {
                        e.local_name == "shape"
                            || e.local_name == "pict"
                            || e.local_name == "imagedata"
                    });
                    if has_vml {
                        remove_indices.push(i);
                    }
                }
            }
        }
        // Remove from highest index first so indices stay valid
        remove_indices.sort_unstable();
        let n = remove_indices.len();
        for i in remove_indices.into_iter().rev() {
            self.remove_header_at(i)?;
        }
        Ok(n)
    }

    /// Set compatibility mode (`w:compat` / `w:compatSetting` for Office version).
    ///
    /// `version` is typically `"15"` (Office 2013) or `"16"` (Office 2016+).
    pub fn set_compatibility_mode(&mut self, version: &str) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children.retain(|c| c.local_name != "compat");
        let compat = OpenXmlElement::w("compat").with_child(
            OpenXmlElement::w("compatSetting")
                .with_attribute_qname("w:name", "compatibilityMode")
                .with_attribute_qname("w:uri", "http://schemas.microsoft.com/office/word")
                .with_attribute_qname("w:val", version),
        );
        root.append_child(compat);
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Whether settings enable update-fields-on-open (`w:updateFields`).
    pub fn update_fields_on_open(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("updateFields").is_some())
    }

    /// Read compatibility mode version from settings, if present.
    /// Alias for [`update_fields_on_open`](Self::update_fields_on_open).
    pub fn has_update_fields_on_open(&self) -> Result<bool> {
        self.update_fields_on_open()
    }

    /// Clear update-fields-on-open setting. Returns whether it was enabled.
    pub fn clear_update_fields_on_open(&mut self) -> Result<bool> {
        let had = self.update_fields_on_open()?;
        if had {
            self.set_update_fields_on_open(false)?;
        }
        Ok(had)
    }

    pub fn compatibility_mode(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(compat) = root.child("compat") else {
            return Ok(None);
        };
        for setting in compat.children_by_name("compatSetting") {
            if setting.get_attribute("name") == Some("compatibilityMode")
                || setting
                    .attributes
                    .iter()
                    .any(|a| a.local_name == "name" && a.value == "compatibilityMode")
            {
                if let Some(v) = setting
                    .get_attribute("val")
                    .or_else(|| {
                        setting
                            .attributes
                            .iter()
                            .find(|a| a.local_name == "val")
                            .map(|a| a.value.as_str())
                    })
                {
                    return Ok(Some(v.to_string()));
                }
            }
        }
        Ok(None)
    }

    /// Whether a compatibilityMode setting is present.
    pub fn has_compatibility_mode(&self) -> Result<bool> {
        Ok(self.compatibility_mode()?.is_some())
    }

    /// Remove the entire `w:compat` element. Returns whether it was present.
    pub fn clear_compatibility_mode(&mut self) -> Result<bool> {
        self.remove_settings_child("compat")
    }

    /// Set or clear a legacy `w:compat` OnOff child (e.g. `"usePrinterMetrics"`).
    pub fn set_compat_flag(&mut self, local_name: &str, enabled: bool) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        let compat = if let Some(pos) = root.children.iter().position(|c| c.local_name == "compat") {
            &mut root.children[pos]
        } else {
            root.append_child(OpenXmlElement::w("compat"));
            root.children.last_mut().unwrap()
        };
        compat.children.retain(|c| c.local_name != local_name);
        if enabled {
            compat.append_child(OpenXmlElement::w(local_name));
        }
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Whether a legacy `w:compat` OnOff child is present.
    pub fn has_compat_flag(&self, local_name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("compat")
            .map(|c| c.child(local_name).is_some())
            .unwrap_or(false))
    }

    /// List local names of all OnOff children under `w:compat` (excludes `compatSetting`).
    /// Disable a named compatibility flag. Returns whether it was enabled.
    pub fn clear_compat_flag(&mut self, local_name: &str) -> Result<bool> {
        let had = self.has_compat_flag(local_name)?;
        if had {
            self.set_compat_flag(local_name, false)?;
        }
        Ok(had)
    }

    pub fn list_compat_flags(&self) -> Result<Vec<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(compat) = root.child("compat") else {
            return Ok(Vec::new());
        };
        Ok(compat
            .children
            .iter()
            .filter(|c| c.local_name != "compatSetting")
            .map(|c| c.local_name.clone())
            .collect())
    }

    /// Set or update a `w:compatSetting` entry by name.
    pub fn set_compat_setting(&mut self, name: &str, uri: &str, val: &str) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        let compat = if let Some(pos) = root.children.iter().position(|c| c.local_name == "compat") {
            &mut root.children[pos]
        } else {
            root.append_child(OpenXmlElement::w("compat"));
            root.children.last_mut().unwrap()
        };
        // Remove existing setting with same name
        compat.children.retain(|c| {
            if c.local_name != "compatSetting" {
                return true;
            }
            let n = c
                .get_attribute("name")
                .or_else(|| {
                    c.attributes
                        .iter()
                        .find(|a| a.local_name == "name")
                        .map(|a| a.value.as_str())
                })
                .unwrap_or("");
            n != name
        });
        compat.append_child(
            OpenXmlElement::w("compatSetting")
                .with_attribute_qname("w:name", name)
                .with_attribute_qname("w:uri", uri)
                .with_attribute_qname("w:val", val),
        );
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Remove a named compat setting (`w:compat/w:compatSetting[@w:name]`).
    pub fn clear_compat_setting(&mut self, name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(compat) = root.child_mut("compat") else {
            return Ok(false);
        };
        let before = compat.children.len();
        compat.children.retain(|c| {
            if c.local_name != "compatSetting" {
                return true;
            }
            let n = c
                .get_attribute_qname("w:name")
                .or_else(|| c.get_attribute("name"))
                .unwrap_or("");
            n != name
        });
        if compat.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    /// List `(name, uri, val)` of all `w:compatSetting` entries.
    pub fn list_compat_settings(&self) -> Result<Vec<(String, String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(compat) = root.child("compat") else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for setting in compat.children_by_name("compatSetting") {
            let name = setting
                .get_attribute("name")
                .or_else(|| {
                    setting
                        .attributes
                        .iter()
                        .find(|a| a.local_name == "name")
                        .map(|a| a.value.as_str())
                })
                .unwrap_or("")
                .to_string();
            let uri = setting
                .get_attribute("uri")
                .or_else(|| {
                    setting
                        .attributes
                        .iter()
                        .find(|a| a.local_name == "uri")
                        .map(|a| a.value.as_str())
                })
                .unwrap_or("")
                .to_string();
            let val = setting
                .get_attribute("val")
                .or_else(|| {
                    setting
                        .attributes
                        .iter()
                        .find(|a| a.local_name == "val")
                        .map(|a| a.value.as_str())
                })
                .unwrap_or("")
                .to_string();
            out.push((name, uri, val));
        }
        Ok(out)
    }

    /// Remove a compatSetting by name. Returns whether found.


    /// Whether any compatibility settings are present.
    pub fn has_compat_settings(&self) -> Result<bool> {
        Ok(!self.list_compat_settings()?.is_empty())
    }

    /// Count compatibility settings.
    pub fn compat_setting_count(&self) -> Result<usize> {
        Ok(self.list_compat_settings()?.len())
    }

    /// Remove a compatSetting by name. Returns whether found.
    pub fn remove_compat_setting(&mut self, name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(compat) = root.child_mut("compat") else {
            return Ok(false);
        };
        let before = compat.children.len();
        compat.children.retain(|c| {
            if c.local_name != "compatSetting" {
                return true;
            }
            let n = c
                .get_attribute("name")
                .or_else(|| {
                    c.attributes
                        .iter()
                        .find(|a| a.local_name == "name")
                        .map(|a| a.value.as_str())
                })
                .unwrap_or("");
            n != name
        });
        let removed = compat.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package.set_part(
                settings_uri,
                content_type::WORD_SETTINGS,
                xml,
            );
        }
        Ok(removed)
    }

    /// Read a single compatSetting value by name.
    pub fn compat_setting_val(&self, name: &str) -> Result<Option<String>> {
        Ok(self
            .list_compat_settings()?
            .into_iter()
            .find(|(n, _, _)| n == name)
            .map(|(_, _, v)| v))
    }

    /// Convenience: set `usePrinterMetrics` compat flag.
    pub fn set_use_printer_metrics(&mut self, enabled: bool) -> Result<()> {
        self.set_compat_flag("usePrinterMetrics", enabled)
    }

    /// Whether `usePrinterMetrics` is present.
    pub fn has_use_printer_metrics(&self) -> Result<bool> {
        self.has_compat_flag("usePrinterMetrics")
    }

    /// Convenience: set `doNotExpandShiftReturn` compat flag.
    /// Disable `use printer metrics`. Returns whether it was enabled.
    pub fn clear_use_printer_metrics(&mut self) -> Result<bool> {
        let had = self.has_use_printer_metrics()?;
        if had {
            self.set_use_printer_metrics(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_expand_shift_return(&mut self, enabled: bool) -> Result<()> {
        self.set_compat_flag("doNotExpandShiftReturn", enabled)
    }

    /// Whether `doNotExpandShiftReturn` is present.
    pub fn has_do_not_expand_shift_return(&self) -> Result<bool> {
        self.has_compat_flag("doNotExpandShiftReturn")
    }

    /// Convenience: set `adjustLineHeightInTable` compat flag.
    /// Disable `do not expand shift return`. Returns whether it was enabled.
    pub fn clear_do_not_expand_shift_return(&mut self) -> Result<bool> {
        let had = self.has_do_not_expand_shift_return()?;
        if had {
            self.set_do_not_expand_shift_return(false)?;
        }
        Ok(had)
    }

    pub fn set_adjust_line_height_in_table(&mut self, enabled: bool) -> Result<()> {
        self.set_compat_flag("adjustLineHeightInTable", enabled)
    }

    /// Whether `adjustLineHeightInTable` is present.
    pub fn has_adjust_line_height_in_table(&self) -> Result<bool> {
        self.has_compat_flag("adjustLineHeightInTable")
    }

    /// Set document protection in settings (`w:documentProtection`).
    ///
    /// Creates a settings part if missing. `edit` is typically `"readOnly"`,
    /// `"forms"`, `"comments"`, or `"trackedChanges"`. No password hashing is performed.
    /// Disable `adjust line height in table`. Returns whether it was enabled.
    pub fn clear_adjust_line_height_in_table(&mut self) -> Result<bool> {
        let had = self.has_adjust_line_height_in_table()?;
        if had {
            self.set_adjust_line_height_in_table(false)?;
        }
        Ok(had)
    }

    pub fn set_document_protection(&mut self, edit: &str, enforcement: bool) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            // Ensure settings part + relationship exist
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children
            .retain(|c| c.local_name != "documentProtection");
        root.append_child(document_protection(edit, enforcement));
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Whether settings contain `w:documentProtection`.
    pub fn is_document_protected(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("documentProtection").is_some())
    }


    /// Alias for [`is_document_protected`](Self::is_document_protected).
    pub fn has_document_protection(&mut self) -> Result<bool> {
        self.is_document_protected()
    }

    /// Read document protection `edit` attribute when present.
    pub fn document_protection_edit(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.child("documentProtection").and_then(|p| {
            p.get_attribute_qname("w:edit")
                .or_else(|| p.get_attribute("edit"))
                .map(|s| s.to_string())
        }))
    }

    /// Whether document protection enforcement is on.
    pub fn document_protection_enforcement(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("documentProtection")
            .and_then(|p| {
                p.get_attribute_qname("w:enforcement")
                    .or_else(|| p.get_attribute("enforcement"))
            })
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true") || s == "on")
            .unwrap_or(false))
    }

    /// Whether document protection locks formatting (`w:formatting`).
    pub fn document_protection_formatting(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("documentProtection")
            .and_then(|p| {
                p.get_attribute_qname("w:formatting")
                    .or_else(|| p.get_attribute("formatting"))
            })
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true") || s == "on")
            .unwrap_or(false))
    }

    /// Set document protection with optional formatting lock.
    pub fn set_document_protection_ex(
        &mut self,
        edit: &str,
        enforcement: bool,
        formatting: bool,
    ) -> Result<()> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&settings_uri) {
            parse_element(data)?
        } else {
            self.add_default_settings()?;
            parse_element(
                self.package
                    .opc()
                    .get_part(&settings_uri)
                    .ok_or_else(|| Error::PartNotFound(settings_uri.to_string()))?,
            )?
        };
        root.children
            .retain(|c| c.local_name != "documentProtection");
        let mut el = document_protection(edit, enforcement);
        if formatting {
            el.set_attribute_qname("w:formatting", "1");
        }
        root.append_child(el);
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            settings_uri,
            content_type::WORD_SETTINGS,
            xml,
        );
        Ok(())
    }

    /// Clear documentProtection (same as clear_document_protection).
    pub fn clear_document_protection_ex(&mut self) -> Result<bool> {
        self.clear_document_protection()
    }

    /// Remove `w:documentProtection` from settings. Returns whether it was present.
    pub fn clear_document_protection(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children
            .retain(|c| c.local_name != "documentProtection");
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package.set_part(
                settings_uri,
                content_type::WORD_SETTINGS,
                xml,
            );
        }
        Ok(removed)
    }

    /// Set write protection shell (`w:writeProtection`).
    ///
    /// `recommended` maps to `w:recommended`. No password hashing is performed;
    /// optional `password` is stored as plain `w:hash` only for shell/test use.
    pub fn set_write_protection(&mut self, recommended: bool) -> Result<()> {
        self.upsert_settings_child("writeProtection", |el| {
            el.set_attribute_qname("w:recommended", if recommended { "1" } else { "0" });
        })
    }

    /// Set write protection recommended flag and optional algorithm name shell.
    pub fn set_write_protection_ex(
        &mut self,
        recommended: bool,
        algorithm_name: Option<&str>,
    ) -> Result<()> {
        self.upsert_settings_child("writeProtection", |el| {
            el.set_attribute_qname("w:recommended", if recommended { "1" } else { "0" });
            if let Some(alg) = algorithm_name {
                el.set_attribute_qname("w:algorithmName", alg);
            }
        })
    }

    /// Clear writeProtection (alias for [`clear_write_protection`](Self::clear_write_protection)).
    pub fn clear_write_protection_ex(&mut self) -> Result<bool> {
        self.clear_write_protection()
    }

    /// Read write protection algorithm name when present.
    pub fn write_protection_algorithm_name(&self) -> Result<Option<String>> {
        Ok(self.settings_child_attr("writeProtection", "algorithmName")?)
    }

    /// Whether writeProtection algorithmName is set.
    pub fn has_write_protection_algorithm_name(&self) -> Result<bool> {
        Ok(self.write_protection_algorithm_name()?.is_some())
    }

    /// Clear writeProtection `@algorithmName`.
    pub fn clear_write_protection_algorithm_name(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(el) = root.child_mut("writeProtection") else {
            return Ok(false);
        };
        let before = el.attributes.len();
        el.attributes.retain(|a| a.local_name != "algorithmName");
        if el.attributes.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    /// Whether writeProtection is present.
    pub fn has_write_protection(&self) -> Result<bool> {
        self.settings_has_child("writeProtection")
    }

    /// Whether write protection is recommended.
    pub fn write_protection_recommended(&self) -> Result<bool> {
        Ok(self
            .settings_child_attr("writeProtection", "recommended")?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear writeProtection. Returns whether present.
    pub fn clear_write_protection(&mut self) -> Result<bool> {
        self.remove_settings_child("writeProtection")
    }

    /// Set track-changes view options (`w:revisionView`).
    pub fn set_revision_view(
        &mut self,
        markup: bool,
        comments: bool,
        ins_del: bool,
        formatting: bool,
    ) -> Result<()> {
        self.upsert_settings_child("revisionView", |el| {
            el.set_attribute_qname("w:markup", if markup { "1" } else { "0" });
            el.set_attribute_qname("w:comments", if comments { "1" } else { "0" });
            el.set_attribute_qname("w:insDel", if ins_del { "1" } else { "0" });
            el.set_attribute_qname("w:formatting", if formatting { "1" } else { "0" });
        })
    }

    /// Read revision view as `(markup, comments, ins_del, formatting)`.
    pub fn revision_view(&self) -> Result<Option<(bool, bool, bool, bool)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(rv) = root.child("revisionView") else {
            return Ok(None);
        };
        let on = |name: &str| {
            rv.get_attribute(name)
                .or_else(|| rv.get_attribute_qname(&format!("w:{name}")))
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(true)
        };
        Ok(Some((
            on("markup"),
            on("comments"),
            on("insDel"),
            on("formatting"),
        )))
    }

    /// Whether revisionView is present.
    pub fn has_revision_view(&self) -> Result<bool> {
        self.settings_has_child("revisionView")
    }

    /// Clear revisionView.
    pub fn clear_revision_view(&mut self) -> Result<bool> {
        self.remove_settings_child("revisionView")
    }

    /// Set document type in settings (`w:documentType w:val`), e.g. `"notSpecified"`, `"letter"`, `"eMail"`.
    pub fn set_document_type_setting(&mut self, val: &str) -> Result<()> {
        self.upsert_settings_child("documentType", |el| {
            el.set_attribute_qname("w:val", val);
        })
    }

    /// Read documentType setting.
    pub fn document_type_setting(&self) -> Result<Option<String>> {
        self.settings_child_attr("documentType", "val")
    }

    /// Clear documentType setting.
    pub fn clear_document_type_setting(&mut self) -> Result<bool> {
        self.remove_settings_child("documentType")
    }

    /// Set style pane sort method (`w:stylePaneSortMethod w:val`).
    pub fn set_style_pane_sort_method(&mut self, val: &str) -> Result<()> {
        self.upsert_settings_child("stylePaneSortMethod", |el| {
            el.set_attribute_qname("w:val", val);
        })
    }

    /// Read stylePaneSortMethod.
    pub fn style_pane_sort_method(&self) -> Result<Option<String>> {
        self.settings_child_attr("stylePaneSortMethod", "val")
    }

    /// Whether stylePaneSortMethod is set.
    pub fn has_style_pane_sort_method(&self) -> Result<bool> {
        Ok(self.style_pane_sort_method()?.is_some())
    }

    /// Clear stylePaneSortMethod.
    pub fn clear_style_pane_sort_method(&mut self) -> Result<bool> {
        self.remove_settings_child("stylePaneSortMethod")
    }

    /// Set style pane format filter flags (`w:stylePaneFormatFilter`).
    ///
    /// Common flags: `allStyles`, `customStyles`, `latentStyles`, `stylesInUse`,
    /// `headingStyles`, `numberingStyles`, `tableStyles`.
    pub fn set_style_pane_format_filter(&mut self, flags: &[(&str, bool)]) -> Result<()> {
        self.upsert_settings_child("stylePaneFormatFilter", |el| {
            for (name, enabled) in flags {
                el.set_attribute_qname(
                    &format!("w:{name}"),
                    if *enabled { "1" } else { "0" },
                );
            }
        })
    }

    /// Read a style pane format filter flag.
    pub fn style_pane_format_filter(&self, flag: &str) -> Result<Option<bool>> {
        Ok(self
            .settings_child_attr("stylePaneFormatFilter", flag)?
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true")))
    }

    /// Whether stylePaneFormatFilter is present.
    pub fn has_style_pane_format_filter(&self) -> Result<bool> {
        self.settings_has_child("stylePaneFormatFilter")
    }

    /// Clear stylePaneFormatFilter.
    pub fn clear_style_pane_format_filter(&mut self) -> Result<bool> {
        self.remove_settings_child("stylePaneFormatFilter")
    }

    /// Add a caption definition under settings (`w:captions/w:caption`).
    pub fn add_caption_definition(
        &mut self,
        name: &str,
        pos: &str,
        num_fmt: &str,
    ) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        let captions = if let Some(pos_i) = root
            .children
            .iter()
            .position(|c| c.local_name == "captions")
        {
            &mut root.children[pos_i]
        } else {
            root.append_child(OpenXmlElement::w("captions"));
            root.children.last_mut().unwrap()
        };
        // replace same name
        captions.children.retain(|c| {
            if c.local_name != "caption" {
                return true;
            }
            c.get_attribute_qname("w:name")
                .or_else(|| c.get_attribute("name"))
                != Some(name)
        });
        captions.append_child(
            OpenXmlElement::w("caption")
                .with_attribute_qname("w:name", name)
                .with_attribute_qname("w:pos", pos)
                .with_attribute_qname("w:numFmt", num_fmt),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// List caption definitions as `(name, pos, num_fmt)`.
    pub fn list_caption_definitions(&self) -> Result<Vec<(String, String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(captions) = root.child("captions") else {
            return Ok(Vec::new());
        };
        Ok(captions
            .children_by_name("caption")
            .map(|c| {
                let name = c
                    .get_attribute_qname("w:name")
                    .or_else(|| c.get_attribute("name"))
                    .unwrap_or("")
                    .to_string();
                let pos = c
                    .get_attribute_qname("w:pos")
                    .or_else(|| c.get_attribute("pos"))
                    .unwrap_or("below")
                    .to_string();
                let fmt = c
                    .get_attribute_qname("w:numFmt")
                    .or_else(|| c.get_attribute("numFmt"))
                    .unwrap_or("decimal")
                    .to_string();
                (name, pos, fmt)
            })
            .collect())
    }

    /// Whether captions settings exist.


    /// Count caption definitions in settings.
    pub fn caption_definition_count(&self) -> Result<usize> {
        Ok(self.list_caption_definitions()?.len())
    }

    /// Whether a caption definition with the given name exists.
    pub fn has_caption_definition(&self, name: &str) -> Result<bool> {
        Ok(self
            .list_caption_definitions()?
            .iter()
            .any(|(n, _, _)| n == name))
    }

    /// Remove one caption definition by name. Returns whether found.
    pub fn remove_caption_definition(&mut self, name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(captions) = root.child_mut("captions") else {
            return Ok(false);
        };
        let before = captions.children.len();
        captions.children.retain(|c| {
            if c.local_name != "caption" {
                return true;
            }
            c.get_attribute_qname("w:name")
                .or_else(|| c.get_attribute("name"))
                != Some(name)
        });
        if captions.children.len() == before {
            return Ok(false);
        }
        // keep autoCaptions etc.; only drop captions container if fully empty of caption+autoCaptions
        if captions.children.is_empty() {
            root.children.retain(|c| c.local_name != "captions");
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    /// Whether captions settings exist.
    pub fn has_captions(&self) -> Result<bool> {
        self.settings_has_child("captions")
    }

    /// Clear captions settings.
    pub fn clear_captions(&mut self) -> Result<bool> {
        self.remove_settings_child("captions")
    }

    /// Set math properties font (`m:mathPr/m:mathFont`) in settings.
    pub fn set_math_font(&mut self, font: &str) -> Result<()> {
        const M: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        let math_pr = if let Some(pos) = root.children.iter().position(|c| c.local_name == "mathPr")
        {
            &mut root.children[pos]
        } else {
            root.append_child(
                OpenXmlElement::new("m", M, "mathPr").with_ns_decl("m", M),
            );
            root.children.last_mut().unwrap()
        };
        math_pr.children.retain(|c| c.local_name != "mathFont");
        math_pr.append_child(
            OpenXmlElement::new("m", M, "mathFont").with_attribute_qname("m:val", font),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read math font from settings.
    pub fn math_font(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(math_pr) = root.child("mathPr") else {
            return Ok(None);
        };
        let Some(mf) = math_pr.child("mathFont") else {
            return Ok(None);
        };
        Ok(mf
            .get_attribute_qname("m:val")
            .or_else(|| mf.get_attribute("val"))
            .map(|s| s.to_string()))
    }

    /// Set math display defaults (`m:mathPr/m:dispDef` presence and `m:defJc`).
    /// Whether math font is set.
    pub fn has_math_font(&self) -> Result<bool> {
        Ok(self.math_font()?.is_some())
    }

    /// Clear `m:mathFont` under math properties. Returns whether present.
    pub fn clear_math_font(&mut self) -> Result<bool> {
        const M: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(math_pr) = root.child_mut("mathPr") else {
            return Ok(false);
        };
        let before = math_pr.children.len();
        math_pr.children.retain(|c| c.local_name != "mathFont");
        if math_pr.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        let _ = M;
        Ok(true)
    }


    pub fn set_math_display_defaults(&mut self, enabled: bool, justify: &str) -> Result<()> {
        const M: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        let math_pr = if let Some(pos) = root.children.iter().position(|c| c.local_name == "mathPr")
        {
            &mut root.children[pos]
        } else {
            root.append_child(
                OpenXmlElement::new("m", M, "mathPr").with_ns_decl("m", M),
            );
            root.children.last_mut().unwrap()
        };
        math_pr
            .children
            .retain(|c| c.local_name != "dispDef" && c.local_name != "defJc");
        if enabled {
            math_pr.append_child(OpenXmlElement::new("m", M, "dispDef"));
        }
        math_pr.append_child(
            OpenXmlElement::new("m", M, "defJc").with_attribute_qname("m:val", justify),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Clear math display defaults (`dispDef` / `defJc`) under mathPr.
    pub fn clear_math_display_defaults(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(math_pr) = root.child_mut("mathPr") else {
            return Ok(false);
        };
        let before = math_pr.children.len();
        math_pr
            .children
            .retain(|c| c.local_name != "dispDef" && c.local_name != "defJc");
        if math_pr.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    /// Whether mathPr is present in settings.
    pub fn has_math_properties(&self) -> Result<bool> {
        self.settings_has_child("mathPr")
    }

    /// Clear mathPr from settings.
    pub fn clear_math_properties(&mut self) -> Result<bool> {
        self.remove_settings_child("mathPr")
    }

    /// Set color scheme mapping for a theme color role (`w:clrSchemeMapping`).
    ///
    /// `role` e.g. `"accent1"`, `theme_color` e.g. `"accent1"`, `"dark1"`.
    pub fn set_color_scheme_mapping(&mut self, role: &str, theme_color: &str) -> Result<()> {
        self.upsert_settings_child("clrSchemeMapping", |el| {
            el.set_attribute_qname(&format!("w:{role}"), theme_color);
        })
    }

    /// Read a color scheme mapping role value.
    pub fn color_scheme_mapping(&self, role: &str) -> Result<Option<String>> {
        self.settings_child_attr("clrSchemeMapping", role)
    }

    /// Whether clrSchemeMapping is present.
    pub fn has_color_scheme_mapping(&self) -> Result<bool> {
        self.settings_has_child("clrSchemeMapping")
    }

    /// Clear clrSchemeMapping.
    pub fn clear_color_scheme_mapping(&mut self) -> Result<bool> {
        self.remove_settings_child("clrSchemeMapping")
    }

    /// Set `w:forceUpgrade` presence (force document upgrade on open).
    pub fn set_force_upgrade(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("forceUpgrade", |_| {})
        } else {
            let _ = self.remove_settings_child("forceUpgrade")?;
            Ok(())
        }
    }

    /// Whether forceUpgrade is present.
    pub fn has_force_upgrade(&self) -> Result<bool> {
        self.settings_has_child("forceUpgrade")
    }

    /// Clear forceUpgrade. Returns whether present.
    pub fn clear_force_upgrade(&mut self) -> Result<bool> {
        self.remove_settings_child("forceUpgrade")
    }

    /// Set `w:doNotValidateAgainstSchema` presence.
    pub fn set_do_not_validate_against_schema(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotValidateAgainstSchema", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotValidateAgainstSchema")?;
            Ok(())
        }
    }

    /// Whether doNotValidateAgainstSchema is present.
    pub fn has_do_not_validate_against_schema(&self) -> Result<bool> {
        self.settings_has_child("doNotValidateAgainstSchema")
    }

    /// Set `w:saveInvalidXml` presence.
    /// Disable `do not validate against schema`. Returns whether it was enabled.
    pub fn clear_do_not_validate_against_schema(&mut self) -> Result<bool> {
        let had = self.has_do_not_validate_against_schema()?;
        if had {
            self.set_do_not_validate_against_schema(false)?;
        }
        Ok(had)
    }

    pub fn set_save_invalid_xml(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("saveInvalidXml", |_| {})
        } else {
            let _ = self.remove_settings_child("saveInvalidXml")?;
            Ok(())
        }
    }

    /// Whether saveInvalidXml is present.
    pub fn has_save_invalid_xml(&self) -> Result<bool> {
        self.settings_has_child("saveInvalidXml")
    }

    /// Set `w:ignoreMixedContent` presence.
    /// Disable `save invalid xml`. Returns whether it was enabled.
    pub fn clear_save_invalid_xml(&mut self) -> Result<bool> {
        let had = self.has_save_invalid_xml()?;
        if had {
            self.set_save_invalid_xml(false)?;
        }
        Ok(had)
    }

    pub fn set_ignore_mixed_content(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("ignoreMixedContent", |_| {})
        } else {
            let _ = self.remove_settings_child("ignoreMixedContent")?;
            Ok(())
        }
    }

    /// Whether ignoreMixedContent is present.
    pub fn has_ignore_mixed_content(&self) -> Result<bool> {
        self.settings_has_child("ignoreMixedContent")
    }

    /// Set `w:alwaysShowPlaceholderText` presence.
    /// Disable `ignore mixed content`. Returns whether it was enabled.
    pub fn clear_ignore_mixed_content(&mut self) -> Result<bool> {
        let had = self.has_ignore_mixed_content()?;
        if had {
            self.set_ignore_mixed_content(false)?;
        }
        Ok(had)
    }

    pub fn set_always_show_placeholder_text(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("alwaysShowPlaceholderText", |_| {})
        } else {
            let _ = self.remove_settings_child("alwaysShowPlaceholderText")?;
            Ok(())
        }
    }

    /// Whether alwaysShowPlaceholderText is present.
    pub fn has_always_show_placeholder_text(&self) -> Result<bool> {
        self.settings_has_child("alwaysShowPlaceholderText")
    }

    /// Set `w:showXMLTags` presence.
    /// Disable `always show placeholder text`. Returns whether it was enabled.
    pub fn clear_always_show_placeholder_text(&mut self) -> Result<bool> {
        let had = self.has_always_show_placeholder_text()?;
        if had {
            self.set_always_show_placeholder_text(false)?;
        }
        Ok(had)
    }

    pub fn set_show_xml_tags(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("showXMLTags", |_| {})
        } else {
            let _ = self.remove_settings_child("showXMLTags")?;
            Ok(())
        }
    }

    /// Whether showXMLTags is present.
    pub fn has_show_xml_tags(&self) -> Result<bool> {
        self.settings_has_child("showXMLTags")
    }

    /// Set `w:doNotDemarcateInvalidXml` presence.
    /// Disable `show xml tags`. Returns whether it was enabled.
    pub fn clear_show_xml_tags(&mut self) -> Result<bool> {
        let had = self.has_show_xml_tags()?;
        if had {
            self.set_show_xml_tags(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_demarcate_invalid_xml(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("doNotDemarcateInvalidXml", |_| {})
        } else {
            let _ = self.remove_settings_child("doNotDemarcateInvalidXml")?;
            Ok(())
        }
    }

    /// Whether doNotDemarcateInvalidXml is present.
    pub fn has_do_not_demarcate_invalid_xml(&self) -> Result<bool> {
        self.settings_has_child("doNotDemarcateInvalidXml")
    }

    /// Set `w:saveXmlDataOnly` presence.
    /// Disable `do not demarcate invalid xml`. Returns whether it was enabled.
    pub fn clear_do_not_demarcate_invalid_xml(&mut self) -> Result<bool> {
        let had = self.has_do_not_demarcate_invalid_xml()?;
        if had {
            self.set_do_not_demarcate_invalid_xml(false)?;
        }
        Ok(had)
    }

    pub fn set_save_xml_data_only(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("saveXmlDataOnly", |_| {})
        } else {
            let _ = self.remove_settings_child("saveXmlDataOnly")?;
            Ok(())
        }
    }

    /// Whether saveXmlDataOnly is present.
    pub fn has_save_xml_data_only(&self) -> Result<bool> {
        self.settings_has_child("saveXmlDataOnly")
    }

    /// Set `w:useXSLTWhenSaving` presence.
    /// Disable `save xml data only`. Returns whether it was enabled.
    pub fn clear_save_xml_data_only(&mut self) -> Result<bool> {
        let had = self.has_save_xml_data_only()?;
        if had {
            self.set_save_xml_data_only(false)?;
        }
        Ok(had)
    }

    pub fn set_use_xslt_when_saving(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("useXSLTWhenSaving", |_| {})
        } else {
            let _ = self.remove_settings_child("useXSLTWhenSaving")?;
            Ok(())
        }
    }

    /// Whether useXSLTWhenSaving is present.
    pub fn has_use_xslt_when_saving(&self) -> Result<bool> {
        self.settings_has_child("useXSLTWhenSaving")
    }

    /// Set `w:alwaysMergeEmptyNamespace` presence.
    /// Disable `use xslt when saving`. Returns whether it was enabled.
    pub fn clear_use_xslt_when_saving(&mut self) -> Result<bool> {
        let had = self.has_use_xslt_when_saving()?;
        if had {
            self.set_use_xslt_when_saving(false)?;
        }
        Ok(had)
    }

    pub fn set_always_merge_empty_namespace(&mut self, enabled: bool) -> Result<()> {
        if enabled {
            self.upsert_settings_child("alwaysMergeEmptyNamespace", |_| {})
        } else {
            let _ = self.remove_settings_child("alwaysMergeEmptyNamespace")?;
            Ok(())
        }
    }

    /// Whether alwaysMergeEmptyNamespace is present.
    pub fn has_always_merge_empty_namespace(&self) -> Result<bool> {
        self.settings_has_child("alwaysMergeEmptyNamespace")
    }

    /// Set document rsid root and list of revision save IDs (`w:rsids`).
    ///
    /// Values are 8-digit hex strings without `0x` prefix (e.g. `"00AB12CD"`).
    /// Disable `always merge empty namespace`. Returns whether it was enabled.
    pub fn clear_always_merge_empty_namespace(&mut self) -> Result<bool> {
        let had = self.has_always_merge_empty_namespace()?;
        if had {
            self.set_always_merge_empty_namespace(false)?;
        }
        Ok(had)
    }

    pub fn set_rsids(&mut self, rsid_root: &str, rsids: &[&str]) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        root.children.retain(|c| c.local_name != "rsids");
        let mut rsids_el = OpenXmlElement::w("rsids").with_child(
            OpenXmlElement::w("rsidRoot").with_attribute_qname("w:val", rsid_root),
        );
        for r in rsids {
            rsids_el.append_child(
                OpenXmlElement::w("rsid").with_attribute_qname("w:val", *r),
            );
        }
        root.append_child(rsids_el);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read rsids as `(rsid_root, list of rsids)`.
    pub fn rsids(&self) -> Result<Option<(String, Vec<String>)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(rs) = root.child("rsids") else {
            return Ok(None);
        };
        let root_val = rs
            .child("rsidRoot")
            .and_then(|e| {
                e.get_attribute_qname("w:val")
                    .or_else(|| e.get_attribute("val"))
            })
            .unwrap_or("")
            .to_string();
        let list = rs
            .children_by_name("rsid")
            .filter_map(|e| {
                e.get_attribute_qname("w:val")
                    .or_else(|| e.get_attribute("val"))
                    .map(|s| s.to_string())
            })
            .collect();
        Ok(Some((root_val, list)))
    }

    /// Whether rsids are present.
    pub fn has_rsids(&self) -> Result<bool> {
        self.settings_has_child("rsids")
    }

    /// Clear rsids.
    pub fn clear_rsids(&mut self) -> Result<bool> {
        self.remove_settings_child("rsids")
    }

    /// Set active writing style (`w:activeWritingStyle`).
    pub fn set_active_writing_style(
        &mut self,
        lang: &str,
        vendor_id: u32,
        dll_version: u32,
        app_name: &str,
    ) -> Result<()> {
        self.upsert_settings_child("activeWritingStyle", |el| {
            el.set_attribute_qname("w:lang", lang);
            el.set_attribute_qname("w:vendorID", vendor_id.to_string());
            el.set_attribute_qname("w:dllVersion", dll_version.to_string());
            el.set_attribute_qname("w:appName", app_name);
        })
    }

    /// Read active writing style as `(lang, vendor_id, dll_version, app_name)`.
    pub fn active_writing_style(&self) -> Result<Option<(String, u32, u32, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(aws) = root.child("activeWritingStyle") else {
            return Ok(None);
        };
        let lang = aws
            .get_attribute_qname("w:lang")
            .or_else(|| aws.get_attribute("lang"))
            .unwrap_or("")
            .to_string();
        let vendor = aws
            .get_attribute_qname("w:vendorID")
            .or_else(|| aws.get_attribute("vendorID"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let dll = aws
            .get_attribute_qname("w:dllVersion")
            .or_else(|| aws.get_attribute("dllVersion"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let app = aws
            .get_attribute_qname("w:appName")
            .or_else(|| aws.get_attribute("appName"))
            .unwrap_or("")
            .to_string();
        Ok(Some((lang, vendor, dll, app)))
    }

    /// Whether activeWritingStyle is present.
    pub fn has_active_writing_style(&self) -> Result<bool> {
        self.settings_has_child("activeWritingStyle")
    }

    /// Clear activeWritingStyle.
    pub fn clear_active_writing_style(&mut self) -> Result<bool> {
        self.remove_settings_child("activeWritingStyle")
    }

    /// Add an attached schema URI (`w:attachedSchema w:val`).
    pub fn add_attached_schema(&mut self, schema_uri: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        // avoid duplicates
        let exists = root.children.iter().any(|c| {
            c.local_name == "attachedSchema"
                && (c.get_attribute_qname("w:val").or_else(|| c.get_attribute("val"))
                    == Some(schema_uri))
        });
        if !exists {
            root.append_child(
                OpenXmlElement::w("attachedSchema").with_attribute_qname("w:val", schema_uri),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// List attached schema URIs.
    pub fn list_attached_schemas(&self) -> Result<Vec<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children
            .iter()
            .filter(|c| c.local_name == "attachedSchema")
            .filter_map(|c| {
                c.get_attribute_qname("w:val")
                    .or_else(|| c.get_attribute("val"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Clear all attached schemas. Returns how many were removed.
    pub fn clear_attached_schemas(&mut self) -> Result<usize> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children
            .retain(|c| c.local_name != "attachedSchema");
        let n = before - root.children.len();
        if n > 0 {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(n)
    }

    /// Add a smart tag type registration (`w:smartTagType`).
    ///
    /// Attributes: `namespaceuri`, `name`, optional `url`.
    pub fn add_smart_tag_type(
        &mut self,
        namespace_uri: &str,
        name: &str,
        url: Option<&str>,
    ) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        let exists = root.children.iter().any(|c| {
            c.local_name == "smartTagType"
                && c.get_attribute_qname("w:name").or_else(|| c.get_attribute("name"))
                    == Some(name)
                && c.get_attribute_qname("w:namespaceuri")
                    .or_else(|| c.get_attribute("namespaceuri"))
                    == Some(namespace_uri)
        });
        if !exists {
            let mut el = OpenXmlElement::w("smartTagType")
                .with_attribute_qname("w:namespaceuri", namespace_uri)
                .with_attribute_qname("w:name", name);
            if let Some(u) = url {
                el.set_attribute_qname("w:url", u);
            }
            root.append_child(el);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// List smart tag types as `(namespaceuri, name, url)`.
    pub fn list_smart_tag_types(&self) -> Result<Vec<(String, String, Option<String>)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children
            .iter()
            .filter(|c| c.local_name == "smartTagType")
            .map(|c| {
                (
                    c.get_attribute_qname("w:namespaceuri")
                        .or_else(|| c.get_attribute("namespaceuri"))
                        .unwrap_or("")
                        .to_string(),
                    c.get_attribute_qname("w:name")
                        .or_else(|| c.get_attribute("name"))
                        .unwrap_or("")
                        .to_string(),
                    c.get_attribute_qname("w:url")
                        .or_else(|| c.get_attribute("url"))
                        .map(|s| s.to_string()),
                )
            })
            .collect())
    }

    /// Whether any smartTagType is registered.
    pub fn has_smart_tag_types(&self) -> Result<bool> {
        Ok(!self.list_smart_tag_types()?.is_empty())
    }

    /// Clear all smartTagType entries. Returns how many were removed.
    pub fn clear_smart_tag_types(&mut self) -> Result<usize> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "smartTagType");
        let n = before - root.children.len();
        if n > 0 {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(n)
    }

    /// Add a schema library entry (`sl:schemaLibrary/sl:schema`).
    ///
    /// Creates the `schemaLibrary` container under settings if needed.
    pub fn add_schema_library_entry(
        &mut self,
        uri: &str,
        schema_location: Option<&str>,
        manifest_location: Option<&str>,
    ) -> Result<()> {
        let sl = crate::generated::schemalibrary_2006_main::NAMESPACE_URI;
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("schemaLibrary").is_none() {
            root.append_child(
                OpenXmlElement::new("sl", sl, "schemaLibrary").with_ns_decl("sl", sl),
            );
        }
        let library = root.child_mut("schemaLibrary").expect("schemaLibrary");
        // replace same uri
        library.children.retain(|c| {
            !(c.local_name == "schema"
                && c.get_attribute_qname("sl:uri").or_else(|| c.get_attribute("uri"))
                    == Some(uri))
        });
        let mut schema = OpenXmlElement::new("sl", sl, "schema")
            .with_attribute_qname("sl:uri", uri);
        if let Some(loc) = schema_location {
            schema.set_attribute_qname("sl:schemaLocation", loc);
        }
        if let Some(man) = manifest_location {
            schema.set_attribute_qname("sl:manifestLocation", man);
        }
        library.append_child(schema);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// List schema library entries as `(uri, schema_location?, manifest_location?)`.
    pub fn list_schema_library_entries(
        &self,
    ) -> Result<Vec<(String, Option<String>, Option<String>)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(library) = root.child("schemaLibrary") else {
            return Ok(Vec::new());
        };
        Ok(library
            .children
            .iter()
            .filter(|c| c.local_name == "schema")
            .map(|c| {
                (
                    c.get_attribute_qname("sl:uri")
                        .or_else(|| c.get_attribute("uri"))
                        .unwrap_or("")
                        .to_string(),
                    c.get_attribute_qname("sl:schemaLocation")
                        .or_else(|| c.get_attribute("schemaLocation"))
                        .map(|s| s.to_string()),
                    c.get_attribute_qname("sl:manifestLocation")
                        .or_else(|| c.get_attribute("manifestLocation"))
                        .map(|s| s.to_string()),
                )
            })
            .collect())
    }

    /// Whether schemaLibrary is present with any entries.
    pub fn has_schema_library(&self) -> Result<bool> {
        Ok(!self.list_schema_library_entries()?.is_empty())
    }

    /// Clear schemaLibrary. Returns how many schema entries were removed.
    pub fn clear_schema_library(&mut self) -> Result<usize> {
        let n = self.list_schema_library_entries()?.len();
        if n == 0 {
            // still drop empty container if present
            let _ = self.remove_settings_child("schemaLibrary")?;
            return Ok(0);
        }
        let _ = self.remove_settings_child("schemaLibrary")?;
        Ok(n)
    }

    /// Set save-through-XSLT shell (`w:saveThroughXslt` with optional solutionID).
    pub fn set_save_through_xslt(&mut self, solution_id: Option<&str>) -> Result<()> {
        self.upsert_settings_child("saveThroughXslt", |el| {
            if let Some(id) = solution_id {
                el.set_attribute_qname("w:solutionID", id);
            }
        })
    }

    /// Whether saveThroughXslt is present.
    pub fn has_save_through_xslt(&self) -> Result<bool> {
        self.settings_has_child("saveThroughXslt")
    }

    /// Clear saveThroughXslt.
    pub fn clear_save_through_xslt(&mut self) -> Result<bool> {
        self.remove_settings_child("saveThroughXslt")
    }

    /// Set mail merge settings shell (`w:mailMerge`) in document settings.
    ///
    /// `main_document_type` e.g. `"formLetters"`, `"email"`, `"envelopes"`, `"mailingLabels"`, `"catalog"`.
    /// `data_type` e.g. `"textFile"`, `"database"`, `"spreadsheet"`, `"native"`, `"query"`.
    pub fn set_mail_merge(
        &mut self,
        main_document_type: &str,
        data_type: &str,
        query: Option<&str>,
        view_merged_data: bool,
    ) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        root.children.retain(|c| c.local_name != "mailMerge");
        let mut mm = OpenXmlElement::w("mailMerge")
            .with_child(
                OpenXmlElement::w("mainDocumentType")
                    .with_attribute_qname("w:val", main_document_type),
            )
            .with_child(
                OpenXmlElement::w("dataType").with_attribute_qname("w:val", data_type),
            );
        if let Some(q) = query {
            mm.append_child(OpenXmlElement::w("query").with_attribute_qname("w:val", q));
        }
        if view_merged_data {
            mm.append_child(OpenXmlElement::w("viewMergedData"));
        }
        root.append_child(mm);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Whether mailMerge settings exist.
    pub fn has_mail_merge(&self) -> Result<bool> {
        self.settings_has_child("mailMerge")
    }

    /// Read mail merge main document type.
    pub fn mail_merge_main_document_type(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(mm) = root.child("mailMerge") else {
            return Ok(None);
        };
        Ok(mm.child("mainDocumentType").and_then(|e| {
            e.get_attribute_qname("w:val")
                .or_else(|| e.get_attribute("val"))
                .map(|s| s.to_string())
        }))
    }

    /// Whether `mainDocumentType` is set under mailMerge.
    pub fn has_mail_merge_main_document_type(&self) -> Result<bool> {
        Ok(self.mail_merge_main_document_type()?.is_some())
    }

    /// Clear `mainDocumentType` under mailMerge.
    pub fn clear_mail_merge_main_document_type(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("mainDocumentType")
    }

    /// Set mail merge main document type (creates mailMerge if missing).
    pub fn set_mail_merge_main_document_type(&mut self, val: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "database"),
                    ),
            );
        }
        if let Some(mm) = root.child_mut("mailMerge") {
            mm.children.retain(|c| c.local_name != "mainDocumentType");
            mm.append_child(
                OpenXmlElement::w("mainDocumentType").with_attribute_qname("w:val", val),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read mail merge data type.
    pub fn mail_merge_data_type(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(mm) = root.child("mailMerge") else {
            return Ok(None);
        };
        Ok(mm.child("dataType").and_then(|e| {
            e.get_attribute_qname("w:val")
                .or_else(|| e.get_attribute("val"))
                .map(|s| s.to_string())
        }))
    }

    /// Whether `dataType` is set under mailMerge.
    pub fn has_mail_merge_data_type(&self) -> Result<bool> {
        Ok(self.mail_merge_data_type()?.is_some())
    }

    /// Clear `dataType` under mailMerge.
    pub fn clear_mail_merge_data_type(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("dataType")
    }

    /// Set mail merge data type (creates mailMerge if missing).
    pub fn set_mail_merge_data_type(&mut self, val: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    ),
            );
        }
        if let Some(mm) = root.child_mut("mailMerge") {
            mm.children.retain(|c| c.local_name != "dataType");
            mm.append_child(OpenXmlElement::w("dataType").with_attribute_qname("w:val", val));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Whether viewMergedData is set.
    pub fn mail_merge_view_merged_data(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .map(|m| m.child("viewMergedData").is_some())
            .unwrap_or(false))
    }

    /// Clear `viewMergedData` under mailMerge.
    pub fn clear_mail_merge_view_merged_data(&mut self) -> Result<bool> {
        let had = self.mail_merge_view_merged_data()?;
        if had {
            self.clear_mail_merge_child("viewMergedData")?;
        }
        Ok(had)
    }

    /// Set or clear mail merge `viewMergedData`.
    pub fn set_mail_merge_view_merged_data(&mut self, enabled: bool) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "database"),
                    ),
            );
        }
        if let Some(mm) = root.child_mut("mailMerge") {
            mm.children.retain(|c| c.local_name != "viewMergedData");
            if enabled {
                mm.append_child(OpenXmlElement::w("viewMergedData"));
            }
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read mail merge query string when present.
    pub fn mail_merge_query(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(mm) = root.child("mailMerge") else {
            return Ok(None);
        };
        Ok(mm.child("query").and_then(|e| {
            e.get_attribute_qname("w:val")
                .or_else(|| e.get_attribute("val"))
                .map(|s| s.to_string())
        }))
    }

    /// Whether `query` is set under mailMerge.
    pub fn has_mail_merge_query(&self) -> Result<bool> {
        Ok(self.mail_merge_query()?.is_some())
    }

    /// Clear `query` under mailMerge.
    pub fn clear_mail_merge_query(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("query")
    }

    /// Set or replace the mail merge query string (creates mailMerge if missing).
    pub fn set_mail_merge_query(&mut self, query: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "database"),
                    ),
            );
        }
        if let Some(mm) = root.child_mut("mailMerge") {
            mm.children.retain(|c| c.local_name != "query");
            mm.append_child(OpenXmlElement::w("query").with_attribute_qname("w:val", query));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Clear mailMerge settings.
    pub fn clear_mail_merge(&mut self) -> Result<bool> {
        self.remove_settings_child("mailMerge")
    }

    /// Set mail merge ODSO shell (`w:mailMerge/w:odso`) with table and data source type.
    ///
    /// Creates mailMerge if missing. `table` is the ODSO table name; `src` e.g. `"database"`.
    pub fn set_mail_merge_odso(&mut self, table: &str, src: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "odso");
        mm.append_child(
            OpenXmlElement::w("odso")
                .with_child(OpenXmlElement::w("table").with_attribute_qname("w:val", table))
                .with_child(OpenXmlElement::w("src").with_attribute_qname("w:val", src)),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Whether ODSO is present under mailMerge.
    pub fn has_mail_merge_odso(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .map(|m| m.child("odso").is_some())
            .unwrap_or(false))
    }

    /// Read ODSO table name.
    pub fn mail_merge_odso_table(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("odso"))
            .and_then(|o| o.child("table"))
            .and_then(|t| {
                t.get_attribute_qname("w:val")
                    .or_else(|| t.get_attribute("val"))
                    .map(|s| s.to_string())
            }))
    }

    /// Whether odso `table` is set.
    pub fn has_mail_merge_odso_table(&self) -> Result<bool> {
        Ok(self.mail_merge_odso_table()?.is_some())
    }

    /// Clear odso `table`.
    pub fn clear_mail_merge_odso_table(&mut self) -> Result<bool> {
        self.clear_mail_merge_odso_child("table")
    }

    /// Set ODSO table name only (creates odso if needed).
    pub fn set_mail_merge_odso_table(&mut self, table: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.children.retain(|c| c.local_name != "table");
            odso.append_child(OpenXmlElement::w("table").with_attribute_qname("w:val", table));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Set ODSO src only (creates odso if needed).
    pub fn set_mail_merge_odso_src(&mut self, src: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.children.retain(|c| c.local_name != "src");
            odso.append_child(OpenXmlElement::w("src").with_attribute_qname("w:val", src));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read ODSO data source type (`w:odso/w:src`).
    pub fn mail_merge_odso_src(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("odso"))
            .and_then(|o| o.child("src"))
            .and_then(|t| {
                t.get_attribute_qname("w:val")
                    .or_else(|| t.get_attribute("val"))
                    .map(|s| s.to_string())
            }))
    }

    /// Whether odso `src` is set.
    pub fn has_mail_merge_odso_src(&self) -> Result<bool> {
        Ok(self.mail_merge_odso_src()?.is_some())
    }

    /// Clear odso `src`.
    pub fn clear_mail_merge_odso_src(&mut self) -> Result<bool> {
        self.clear_mail_merge_odso_child("src")
    }

    /// Set ODSO column delimiter (`w:odso/w:colDelim`).
    pub fn set_mail_merge_odso_col_delim(&mut self, delim: u32) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.children.retain(|c| c.local_name != "colDelim");
            odso.append_child(
                OpenXmlElement::w("colDelim").with_attribute_qname("w:val", delim.to_string()),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read ODSO column delimiter.
    pub fn mail_merge_odso_col_delim(&self) -> Result<Option<u32>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("odso"))
            .and_then(|o| o.child("colDelim"))
            .and_then(|t| {
                t.get_attribute_qname("w:val")
                    .or_else(|| t.get_attribute("val"))
            })
            .and_then(|s| s.parse().ok()))
    }

    /// Whether odso `colDelim` is set.
    pub fn has_mail_merge_odso_col_delim(&self) -> Result<bool> {
        Ok(self.mail_merge_odso_col_delim()?.is_some())
    }

    /// Clear odso `colDelim`.
    pub fn clear_mail_merge_odso_col_delim(&mut self) -> Result<bool> {
        self.clear_mail_merge_odso_child("colDelim")
    }

    /// Set whether the data source has a header row (`w:odso/w:fHdr`).
    pub fn set_mail_merge_odso_f_hdr(&mut self, enabled: bool) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.children.retain(|c| c.local_name != "fHdr");
            if enabled {
                odso.append_child(OpenXmlElement::w("fHdr"));
            }
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Whether ODSO fHdr is set.
    pub fn mail_merge_odso_f_hdr(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("odso"))
            .map(|o| o.child("fHdr").is_some())
            .unwrap_or(false))
    }

    /// Clear odso `fHdr`.
    pub fn clear_mail_merge_odso_f_hdr(&mut self) -> Result<bool> {
        let had = self.mail_merge_odso_f_hdr()?;
        if had {
            self.clear_mail_merge_odso_child("fHdr")?;
        }
        Ok(had)
    }

    /// Set ODSO UDL connection string (`w:odso/w:udl`).
    pub fn set_mail_merge_odso_udl(&mut self, udl: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.children.retain(|c| c.local_name != "udl");
            odso.append_child(OpenXmlElement::w("udl").with_attribute_qname("w:val", udl));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read ODSO UDL connection string.
    pub fn mail_merge_odso_udl(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("odso"))
            .and_then(|o| o.child("udl"))
            .and_then(|t| {
                t.get_attribute_qname("w:val")
                    .or_else(|| t.get_attribute("val"))
                    .map(|s| s.to_string())
            }))
    }

    /// Whether odso `udl` is set.
    pub fn has_mail_merge_odso_udl(&self) -> Result<bool> {
        Ok(self.mail_merge_odso_udl()?.is_some())
    }

    /// Clear odso `udl`.
    pub fn clear_mail_merge_odso_udl(&mut self) -> Result<bool> {
        self.clear_mail_merge_odso_child("udl")
    }

    /// Set ODSO source type (`w:odso/w:type`), e.g. `"database"`, `"addressBook"`, `"document1"`, `"document2"`, `"textFile"`, `"email"`, `"native"`, `"legacy"`, `"master"`.
    pub fn set_mail_merge_odso_type(&mut self, src_type: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.children.retain(|c| c.local_name != "type");
            odso.append_child(
                OpenXmlElement::w("type").with_attribute_qname("w:val", src_type),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read ODSO source type.
    pub fn mail_merge_odso_type(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("odso"))
            .and_then(|o| o.child("type"))
            .and_then(|t| {
                t.get_attribute_qname("w:val")
                    .or_else(|| t.get_attribute("val"))
                    .map(|s| s.to_string())
            }))
    }

    /// Whether odso `type` is set.
    pub fn has_mail_merge_odso_type(&self) -> Result<bool> {
        Ok(self.mail_merge_odso_type()?.is_some())
    }

    /// Clear odso `type`.
    pub fn clear_mail_merge_odso_type(&mut self) -> Result<bool> {
        self.clear_mail_merge_odso_child("type")
    }

    /// Set ODSO recipient data relationship id (`w:odso/w:recipientData r:id`).
    pub fn set_mail_merge_odso_recipient_data(&mut self, rid: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.children.retain(|c| c.local_name != "recipientData");
            odso.append_child(
                OpenXmlElement::w("recipientData").with_attribute_qname("r:id", rid),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read ODSO recipient data relationship id.
    pub fn mail_merge_odso_recipient_data(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("odso"))
            .and_then(|o| o.child("recipientData"))
            .and_then(|t| {
                t.get_attribute_qname("r:id")
                    .or_else(|| t.get_attribute("id"))
                    .map(|s| s.to_string())
            }))
    }

    /// Whether odso `recipientData` is set.
    pub fn has_mail_merge_odso_recipient_data(&self) -> Result<bool> {
        Ok(self.mail_merge_odso_recipient_data()?.is_some())
    }

    /// Clear odso `recipientData`.
    pub fn clear_mail_merge_odso_recipient_data(&mut self) -> Result<bool> {
        self.clear_mail_merge_odso_child("recipientData")
    }

    /// Clear ODSO field maps only. Returns how many were removed.
    pub fn clear_mail_merge_odso_field_maps(&mut self) -> Result<usize> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        let Some(odso) = root
            .child_mut("mailMerge")
            .and_then(|m| m.child_mut("odso"))
        else {
            return Ok(0);
        };
        let before = odso.children.len();
        odso.children.retain(|c| c.local_name != "fieldMapData");
        let n = before - odso.children.len();
        if n > 0 {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(n)
    }

    /// Set ODSO field mapping entry (`w:odso/w:fieldMapData`).
    ///
    /// Appends a fieldMapData child with type, name, and mappedName.
    pub fn add_mail_merge_odso_field_map(
        &mut self,
        field_type: &str,
        name: &str,
        mapped_name: &str,
    ) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(
                OpenXmlElement::w("mailMerge")
                    .with_child(
                        OpenXmlElement::w("mainDocumentType")
                            .with_attribute_qname("w:val", "formLetters"),
                    )
                    .with_child(
                        OpenXmlElement::w("dataType").with_attribute_qname("w:val", "native"),
                    ),
            );
        }
        let mm = root.child_mut("mailMerge").unwrap();
        if mm.child("odso").is_none() {
            mm.append_child(OpenXmlElement::w("odso"));
        }
        if let Some(odso) = mm.child_mut("odso") {
            odso.append_child(
                OpenXmlElement::w("fieldMapData")
                    .with_child(
                        OpenXmlElement::w("type").with_attribute_qname("w:val", field_type),
                    )
                    .with_child(OpenXmlElement::w("name").with_attribute_qname("w:val", name))
                    .with_child(
                        OpenXmlElement::w("mappedName")
                            .with_attribute_qname("w:val", mapped_name),
                    ),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// List ODSO field maps as `(type, name, mapped_name)`.
    pub fn list_mail_merge_odso_field_maps(
        &self,
    ) -> Result<Vec<(String, String, String)>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(odso) = root.child("mailMerge").and_then(|m| m.child("odso")) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for fm in odso.children_by_name("fieldMapData") {
            let ty = fm
                .child("type")
                .and_then(|e| {
                    e.get_attribute_qname("w:val")
                        .or_else(|| e.get_attribute("val"))
                })
                .unwrap_or("")
                .to_string();
            let name = fm
                .child("name")
                .and_then(|e| {
                    e.get_attribute_qname("w:val")
                        .or_else(|| e.get_attribute("val"))
                })
                .unwrap_or("")
                .to_string();
            let mapped = fm
                .child("mappedName")
                .and_then(|e| {
                    e.get_attribute_qname("w:val")
                        .or_else(|| e.get_attribute("val"))
                })
                .unwrap_or("")
                .to_string();
            out.push((ty, name, mapped));
        }
        Ok(out)
    }

    /// Whether any ODSO field maps are configured.
    pub fn has_mail_merge_odso_field_maps(&self) -> Result<bool> {
        Ok(!self.list_mail_merge_odso_field_maps()?.is_empty())
    }

    /// Count ODSO field map entries.
    pub fn mail_merge_odso_field_map_count(&self) -> Result<usize> {
        Ok(self.list_mail_merge_odso_field_maps()?.len())
    }

    /// Remove ODSO field map entries whose `name` matches. Returns how many were removed.
    pub fn remove_mail_merge_odso_field_map(&mut self, name: &str) -> Result<usize> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        let Some(odso) = root
            .child_mut("mailMerge")
            .and_then(|m| m.child_mut("odso"))
        else {
            return Ok(0);
        };
        let before = odso.children.len();
        odso.children.retain(|c| {
            if c.local_name != "fieldMapData" {
                return true;
            }
            let n = c
                .child("name")
                .and_then(|e| {
                    e.get_attribute_qname("w:val")
                        .or_else(|| e.get_attribute("val"))
                })
                .unwrap_or("");
            n != name
        });
        let removed = before - odso.children.len();
        if removed > 0 {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(removed)
    }

    pub fn clear_mail_merge_odso(&mut self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(mm) = root.child_mut("mailMerge") else {
            return Ok(false);
        };
        let before = mm.children.len();
        mm.children.retain(|c| c.local_name != "odso");
        let removed = mm.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        }
        Ok(removed)
    }


    fn clear_mail_merge_child(&mut self, local_name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(mm) = root.child_mut("mailMerge") else {
            return Ok(false);
        };
        let before = mm.children.len();
        mm.children.retain(|c| c.local_name != local_name);
        if mm.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    fn clear_mail_merge_odso_child(&mut self, local_name: &str) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(mm) = root.child_mut("mailMerge") else {
            return Ok(false);
        };
        let Some(odso) = mm.child_mut("odso") else {
            return Ok(false);
        };
        let before = odso.children.len();
        odso.children.retain(|c| c.local_name != local_name);
        if odso.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(true)
    }

    /// Set active mail merge record index (`w:activeRecord`).
    pub fn set_mail_merge_active_record(&mut self, index: u32) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "activeRecord");
        mm.append_child(
            OpenXmlElement::w("activeRecord").with_attribute_qname("w:val", index.to_string()),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read active mail merge record index.
    pub fn mail_merge_active_record(&self) -> Result<Option<u32>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("activeRecord"))
            .and_then(|a| {
                a.get_attribute_qname("w:val")
                    .or_else(|| a.get_attribute("val"))
            })
            .and_then(|s| s.parse().ok()))
    }

    /// Whether `activeRecord` is set under mailMerge.
    pub fn has_mail_merge_active_record(&self) -> Result<bool> {
        Ok(self.mail_merge_active_record()?.is_some())
    }

    /// Clear `activeRecord` under mailMerge.
    pub fn clear_mail_merge_active_record(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("activeRecord")
    }

    /// Set mail merge destination (`w:destination w:val`), e.g. `"newDocument"`, `"printer"`, `"email"`, `"fax"`.
    pub fn set_mail_merge_destination(&mut self, val: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "destination");
        mm.append_child(
            OpenXmlElement::w("destination").with_attribute_qname("w:val", val),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read mail merge destination.
    pub fn mail_merge_destination(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("destination"))
            .and_then(|d| {
                d.get_attribute_qname("w:val")
                    .or_else(|| d.get_attribute("val"))
                    .map(|s| s.to_string())
            }))
    }

    /// Whether `destination` is set under mailMerge.
    pub fn has_mail_merge_destination(&self) -> Result<bool> {
        Ok(self.mail_merge_destination()?.is_some())
    }

    /// Clear `destination` under mailMerge.
    pub fn clear_mail_merge_destination(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("destination")
    }

    /// Set mail subject for email merge (`w:mailSubject w:val`).
    pub fn set_mail_merge_subject(&mut self, subject: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "mailSubject");
        mm.append_child(
            OpenXmlElement::w("mailSubject").with_attribute_qname("w:val", subject),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read mail merge subject.
    pub fn mail_merge_subject(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("mailSubject"))
            .and_then(|s| {
                s.get_attribute_qname("w:val")
                    .or_else(|| s.get_attribute("val"))
                    .map(|v| v.to_string())
            }))
    }

    /// Whether `mailSubject` is set under mailMerge.
    pub fn has_mail_merge_subject(&self) -> Result<bool> {
        Ok(self.mail_merge_subject()?.is_some())
    }

    /// Clear `mailSubject` under mailMerge.
    pub fn clear_mail_merge_subject(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("mailSubject")
    }

    /// Set address field name (`w:addressFieldName w:val`).
    pub fn set_mail_merge_address_field_name(&mut self, name: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children
            .retain(|c| c.local_name != "addressFieldName");
        mm.append_child(
            OpenXmlElement::w("addressFieldName").with_attribute_qname("w:val", name),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read address field name.
    pub fn mail_merge_address_field_name(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("addressFieldName"))
            .and_then(|s| {
                s.get_attribute_qname("w:val")
                    .or_else(|| s.get_attribute("val"))
                    .map(|v| v.to_string())
            }))
    }

    /// Whether `addressFieldName` is set under mailMerge.
    pub fn has_mail_merge_address_field_name(&self) -> Result<bool> {
        Ok(self.mail_merge_address_field_name()?.is_some())
    }

    /// Clear `addressFieldName` under mailMerge.
    pub fn clear_mail_merge_address_field_name(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("addressFieldName")
    }

    /// Set `w:mailAsAttachment` presence.
    pub fn set_mail_merge_as_attachment(&mut self, enabled: bool) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "mailAsAttachment");
        if enabled {
            mm.append_child(OpenXmlElement::w("mailAsAttachment"));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Whether mailAsAttachment is present.
    pub fn mail_merge_as_attachment(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .map(|m| m.child("mailAsAttachment").is_some())
            .unwrap_or(false))
    }

    /// Clear `mailAsAttachment` under mailMerge.
    pub fn clear_mail_merge_as_attachment(&mut self) -> Result<bool> {
        let had = self.mail_merge_as_attachment()?;
        if had {
            self.clear_mail_merge_child("mailAsAttachment")?;
        }
        Ok(had)
    }

    /// Set `w:doNotSuppressBlankLines` under mailMerge.
    pub fn set_mail_merge_do_not_suppress_blank_lines(&mut self, enabled: bool) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children
            .retain(|c| c.local_name != "doNotSuppressBlankLines");
        if enabled {
            mm.append_child(OpenXmlElement::w("doNotSuppressBlankLines"));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Whether doNotSuppressBlankLines is present.
    pub fn mail_merge_do_not_suppress_blank_lines(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .map(|m| m.child("doNotSuppressBlankLines").is_some())
            .unwrap_or(false))
    }

    /// Clear `doNotSuppressBlankLines` under mailMerge.
    pub fn clear_mail_merge_do_not_suppress_blank_lines(&mut self) -> Result<bool> {
        let had = self.mail_merge_do_not_suppress_blank_lines()?;
        if had {
            self.clear_mail_merge_child("doNotSuppressBlankLines")?;
        }
        Ok(had)
    }

    /// Set `w:linkToQuery` under mailMerge.
    pub fn set_mail_merge_link_to_query(&mut self, enabled: bool) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "linkToQuery");
        if enabled {
            mm.append_child(OpenXmlElement::w("linkToQuery"));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Whether linkToQuery is present.
    pub fn mail_merge_link_to_query(&self) -> Result<bool> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .map(|m| m.child("linkToQuery").is_some())
            .unwrap_or(false))
    }

    /// Clear `linkToQuery` under mailMerge.
    pub fn clear_mail_merge_link_to_query(&mut self) -> Result<bool> {
        let had = self.mail_merge_link_to_query()?;
        if had {
            self.clear_mail_merge_child("linkToQuery")?;
        }
        Ok(had)
    }

    /// Set mail merge checkErrors value (`w:checkErrors w:val`).
    pub fn set_mail_merge_check_errors(&mut self, val: u32) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "checkErrors");
        mm.append_child(
            OpenXmlElement::w("checkErrors").with_attribute_qname("w:val", val.to_string()),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read checkErrors value.
    pub fn mail_merge_check_errors(&self) -> Result<Option<u32>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("checkErrors"))
            .and_then(|c| {
                c.get_attribute_qname("w:val")
                    .or_else(|| c.get_attribute("val"))
            })
            .and_then(|s| s.parse().ok()))
    }

    /// Whether `checkErrors` is set under mailMerge.
    pub fn has_mail_merge_check_errors(&self) -> Result<bool> {
        Ok(self.mail_merge_check_errors()?.is_some())
    }

    /// Clear `checkErrors` under mailMerge.
    pub fn clear_mail_merge_check_errors(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("checkErrors")
    }

    /// Set connect string under mailMerge (`w:connectString w:val`).
    pub fn set_mail_merge_connect_string(&mut self, connect: &str) -> Result<()> {
        let (settings_uri, mut root) = self.ensure_settings_root()?;
        if root.child("mailMerge").is_none() {
            root.append_child(OpenXmlElement::w("mailMerge"));
        }
        let mm = root.child_mut("mailMerge").unwrap();
        mm.children.retain(|c| c.local_name != "connectString");
        mm.append_child(
            OpenXmlElement::w("connectString").with_attribute_qname("w:val", connect),
        );
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(settings_uri, content_type::WORD_SETTINGS, xml);
        Ok(())
    }

    /// Read connect string.
    pub fn mail_merge_connect_string(&self) -> Result<Option<String>> {
        let settings_uri = PackUri::new("/word/settings.xml");
        let Some(data) = self.package.opc().get_part(&settings_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("mailMerge")
            .and_then(|m| m.child("connectString"))
            .and_then(|c| {
                c.get_attribute_qname("w:val")
                    .or_else(|| c.get_attribute("val"))
                    .map(|s| s.to_string())
            }))
    }

    /// Whether `connectString` is set under mailMerge.
    pub fn has_mail_merge_connect_string(&self) -> Result<bool> {
        Ok(self.mail_merge_connect_string()?.is_some())
    }

    /// Clear `connectString` under mailMerge.
    pub fn clear_mail_merge_connect_string(&mut self) -> Result<bool> {
        self.clear_mail_merge_child("connectString")
    }

    /// Embed an arbitrary package (e.g. another Office file) as an embedded package part.
    ///
    /// Returns `(relationship_id, part_uri)`. Does not insert an OLE drawing into the body.
    pub fn add_embedded_package(
        &mut self,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let uri = loop {
            let candidate =
                PackUri::new(format!("/word/embeddings/Microsoft_Object{index}.{extension}"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::PACKAGE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a glossary document part with a single named doc part entry.
    ///
    /// Returns the relationship id.
    pub fn add_glossary_document(
        &mut self,
        doc_part_name: &str,
        body_paragraphs: impl IntoIterator<Item = OpenXmlElement>,
    ) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/glossary/document.xml");
        let root = glossary_document(doc_part_name, body_paragraphs);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_GLOSSARY, xml);
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .and_then(|rels| {
                rels.get_by_type(rel::GLOSSARY_DOCUMENT)
                    .map(|r| r.id.clone())
            })
        {
            return Ok(existing);
        }
        Ok(self.package.add_part_relationship(
            &main_uri,
            rel::GLOSSARY_DOCUMENT,
            &uri,
            RelationshipTargetMode::Internal,
        ))
    }

    /// List glossary doc part names.
    pub fn list_glossary_doc_parts(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/word/glossary/document.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let mut out = Vec::new();
        for part in root.descendants().filter(|e| e.local_name == "docPart") {
            if let Some(name) = part
                .child("docPartPr")
                .and_then(|pr| pr.child("name"))
                .and_then(|n| {
                    n.get_attribute_qname("w:val")
                        .or_else(|| n.get_attribute("val"))
                })
            {
                out.push(name.to_string());
            }
        }
        Ok(out)
    }


    /// Whether the glossary contains any named doc parts.
    pub fn has_glossary_doc_parts(&self) -> Result<bool> {
        Ok(!self.list_glossary_doc_parts()?.is_empty())
    }

    /// Count named glossary doc parts.
    pub fn glossary_doc_part_count(&self) -> Result<usize> {
        Ok(self.list_glossary_doc_parts()?.len())
    }

    /// Append a doc part entry to the glossary (creates glossary if missing).
    pub fn append_glossary_doc_part(
        &mut self,
        doc_part_name: &str,
        body_paragraphs: impl IntoIterator<Item = OpenXmlElement>,
    ) -> Result<()> {
        use crate::wordprocessing::glossary_document;
        let uri = PackUri::new("/word/glossary/document.xml");
        if !self.package.opc().has_part(&uri) {
            let _ = self.add_glossary_document(doc_part_name, body_paragraphs)?;
            return Ok(());
        }
        let data = self
            .package
            .opc()
            .get_part(&uri)
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))?;
        let mut root = parse_element(data)?;
        let built = glossary_document(doc_part_name, body_paragraphs);
        let Some(new_part) = built
            .child("docParts")
            .and_then(|dp| dp.children_by_name("docPart").next())
            .cloned()
        else {
            return Err(Error::Package("failed to build glossary docPart".into()));
        };
        if let Some(parts) = root.child_mut("docParts") {
            parts.children.retain(|c| {
                if c.local_name != "docPart" {
                    return true;
                }
                let name = c
                    .child("docPartPr")
                    .and_then(|pr| pr.child("name"))
                    .and_then(|n| {
                        n.get_attribute_qname("w:val")
                            .or_else(|| n.get_attribute("val"))
                    });
                name != Some(doc_part_name)
            });
            parts.append_child(new_part);
        } else {
            root.append_child(OpenXmlElement::w("docParts").with_child(new_part));
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::WORD_GLOSSARY, xml);
        Ok(())
    }

    /// Remove a glossary doc part by name. Returns whether found.
    pub fn remove_glossary_doc_part(&mut self, doc_part_name: &str) -> Result<bool> {
        let uri = PackUri::new("/word/glossary/document.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(parts) = root.child_mut("docParts") else {
            return Ok(false);
        };
        let before = parts.children.len();
        parts.children.retain(|c| {
            if c.local_name != "docPart" {
                return true;
            }
            let name = c
                .child("docPartPr")
                .and_then(|pr| pr.child("name"))
                .and_then(|n| {
                    n.get_attribute_qname("w:val")
                        .or_else(|| n.get_attribute("val"))
                });
            name != Some(doc_part_name)
        });
        let removed = parts.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_GLOSSARY, xml);
        }
        Ok(removed)
    }

    /// Add a binary image under `/word/media/` and return its relationship info.
    pub fn add_image(
        &mut self,
        format: ImageFormat,
        data: impl Into<Vec<u8>>,
    ) -> Result<ImagePart> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_image_part(&mut self.package, format, data)
    }

    /// Add a header part with the given paragraphs/blocks. Returns relationship id.
    ///
    /// Does **not** automatically insert a `w:headerReference` into the document body —
    /// call [`header_reference`](crate::packaging::header_reference) and append it to
    /// `w:sectPr` yourself, or use [`add_default_header`](Self::add_default_header).
    pub fn add_header(
        &mut self,
        children: impl IntoIterator<Item = OpenXmlElement>,
    ) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let (rid, _) = main.add_header_part(&mut self.package, header(children))?;
        Ok(rid)
    }

    /// Add a footer part. Returns relationship id.
    pub fn add_footer(
        &mut self,
        children: impl IntoIterator<Item = OpenXmlElement>,
    ) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let (rid, _) = main.add_footer_part(&mut self.package, footer(children))?;
        Ok(rid)
    }

    /// Add a simple text header and wire a default `headerReference` into the body sectPr.
    pub fn add_default_header(&mut self, text_content: &str) -> Result<String> {
        let rid = self.add_header(vec![paragraph(vec![run(vec![text(text_content)])])])?;
        self.ensure_sect_pr_reference(header_reference(&rid, "default"))?;
        Ok(rid)
    }

    /// Add a simple text footer and wire a default `footerReference` into the body sectPr.
    pub fn add_default_footer(&mut self, text_content: &str) -> Result<String> {
        let rid = self.add_footer(vec![paragraph(vec![run(vec![text(text_content)])])])?;
        self.ensure_sect_pr_reference(footer_reference(&rid, "default"))?;
        Ok(rid)
    }

    /// Register an external hyperlink and return a `w:hyperlink` element wrapping the text.
    pub fn create_hyperlink(
        &mut self,
        url: &str,
        link_text: &str,
    ) -> Result<OpenXmlElement> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let rid = main.add_hyperlink(&mut self.package, url);
        Ok(hyperlink(
            &rid,
            vec![run(vec![text(link_text)])],
        ))
    }

    /// Add a default footer with a PAGE field (page number).
    pub fn add_page_number_footer(&mut self) -> Result<String> {
        use crate::wordprocessing::{page_number_field, paragraph, run, text};
        let rid = self.add_footer(vec![paragraph(vec![
            run(vec![text("Page ")]),
            page_number_field(),
        ])])?;
        self.ensure_sect_pr_reference(footer_reference(&rid, "default"))?;
        Ok(rid)
    }

    /// Create an internal hyperlink to a bookmark (`w:anchor`).
    pub fn create_anchor_hyperlink(
        &self,
        bookmark_name: &str,
        link_text: &str,
    ) -> OpenXmlElement {
        use super::parts::hyperlink_anchor;
        hyperlink_anchor(bookmark_name, vec![run(vec![text(link_text)])])
    }

    /// Add a mail-merge recipient data part (raw XML, typically Word recipients list).
    ///
    /// Related from the document settings part when present; otherwise from the main
    /// document. Returns `(relationship_id, part_uri)`.
    pub fn add_mail_merge_recipients(
        &mut self,
        recipients_xml: impl AsRef<[u8]>,
    ) -> Result<(String, PackUri)> {
        let settings_uri = PackUri::new("/word/settings.xml");
        // Ensure settings exists so recipientData can hang off it (C# model).
        if !self.package.opc().has_part(&settings_uri) {
            self.add_default_settings()?;
        }
        let mut index = 1u32;
        let uri = loop {
            let candidate = PackUri::new(format!("/word/recipients{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.recipients+xml",
            recipients_xml.as_ref().to_vec(),
        );
        const RECIPIENT_DATA: &str =
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData";
        let rid = self.package.add_part_relationship(
            &settings_uri,
            RECIPIENT_DATA,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Replace all occurrences of `from` with `to` in the main document body text.
    ///
    /// Returns the number of replacements performed.
    pub fn replace_text(&mut self, from: &str, to: &str) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        Ok(replace_text(doc, from, to))
    }

    /// Add/replace the comments part with the given comment elements.
    pub fn set_comments(
        &mut self,
        comment_elems: impl IntoIterator<Item = OpenXmlElement>,
    ) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_comments_part(&mut self.package, comments(comment_elems))
    }

    /// Add a default numbering definitions part (one bullet list, numId=1).
    pub fn add_default_numbering(&mut self) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_numbering_part(&mut self.package, default_numbering())
    }

    /// Ensure default numbering exists, then append bullet paragraphs (`numId=1`).
    ///
    /// Returns the numbering relationship id.
    pub fn append_bullet_list(&mut self, items: &[&str]) -> Result<String> {
        use crate::wordprocessing::numbered_paragraph;
        let rid = self.add_default_numbering()?;
        for item in items {
            self.append_paragraph(numbered_paragraph(1, *item))?;
        }
        Ok(rid)
    }

    /// Append a heading paragraph with the given style id (e.g. `"Heading1"`).
    ///
    /// Does not create the style definition; pair with
    /// [`add_paragraph_styles`](Self::add_paragraph_styles) or
    /// [`add_default_styles`](Self::add_default_styles).
    pub fn append_heading(&mut self, style_id: &str, text: &str) -> Result<()> {
        use crate::wordprocessing::paragraph_with_style;
        self.append_paragraph(paragraph_with_style(style_id, text))
    }

    /// Append a paragraph containing an external hyperlink.
    pub fn append_hyperlink(&mut self, url: &str, link_text: &str) -> Result<()> {
        let hl = self.create_hyperlink(url, link_text)?;
        self.append_paragraph(paragraph(vec![hl]))
    }

    /// List external hyperlink targets related from the main document as `(rId, url)`.
    pub fn list_external_hyperlinks(&self) -> Vec<(String, String)> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        let main_uri = main.part().uri.clone();
        self.package
            .opc()
            .part_relationships(&main_uri)
            .map(|rels| {
                rels.find_all_by_type(rel::HYPERLINK)
                    .into_iter()
                    .map(|r| (r.id.clone(), r.target.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }


    /// Whether any external hyperlink relationships exist.
    pub fn has_hyperlinks(&self) -> bool {
        !self.list_external_hyperlinks().is_empty()
    }

    /// Count external hyperlinks.
    pub fn hyperlink_count(&self) -> usize {
        self.list_external_hyperlinks().len()
    }

    /// Alias for [`list_external_hyperlinks`](Self::list_external_hyperlinks).
    pub fn list_hyperlinks(&self) -> Vec<(String, String)> {
        self.list_external_hyperlinks()
    }

    /// Alias for [`remove`/clear external hyperlinks by clearing all listed targets].
    pub fn clear_hyperlinks(&mut self) -> usize {
        let links = self.list_external_hyperlinks();
        let mut n = 0;
        for (_id, url) in links {
            if self.remove_external_hyperlink(&url) {
                n += 1;
            }
        }
        n
    }

    /// List internal hyperlink anchors from body `w:hyperlink/@w:anchor` values.
    pub fn list_anchor_hyperlinks(&mut self) -> Result<Vec<String>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for e in body.descendants() {
            if e.local_name != "hyperlink" {
                continue;
            }
            if let Some(a) = e
                .get_attribute_qname("w:anchor")
                .or_else(|| e.get_attribute("anchor"))
            {
                if !out.iter().any(|s| s == a) {
                    out.push(a.to_string());
                }
            }
        }
        Ok(out)
    }


    /// Whether any body hyperlinks target a bookmark anchor.
    pub fn has_anchor_hyperlinks(&mut self) -> Result<bool> {
        Ok(!self.list_anchor_hyperlinks()?.is_empty())
    }

    /// Count distinct bookmark anchors referenced by body hyperlinks.
    pub fn anchor_hyperlink_count(&mut self) -> Result<usize> {
        Ok(self.list_anchor_hyperlinks()?.len())
    }

    /// Update the target URL of an external hyperlink relationship by rId.
    pub fn set_hyperlink_target(&mut self, rid: &str, new_url: &str) -> Result<bool> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Ok(false);
        };
        let main_uri = main.part().uri.clone();
        let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
        // remove and re-add with same id if present
        let Some(old) = rels.get(rid).cloned() else {
            return Ok(false);
        };
        if !old.relationship_type.contains("hyperlink") && old.relationship_type != rel::HYPERLINK {
            return Ok(false);
        }
        let mode = old.target_mode;
        let ty = old.relationship_type.clone();
        rels.remove(rid);
        rels.add_with_id(rid, ty, new_url, mode);
        Ok(true)
    }

    /// Remove an external hyperlink relationship by rId. Returns whether found.
    ///
    /// Does not rewrite `w:hyperlink` elements in the body DOM; use
    /// [`remove_body_hyperlink`](Self::remove_body_hyperlink) to also unwrap the
    /// corresponding body element(s).
    pub fn remove_hyperlink_by_id(&mut self, rid: &str) -> Result<bool> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Ok(false);
        };
        let main_uri = main.part().uri.clone();
        let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
        Ok(rels.remove(rid).is_some())
    }

    /// List body `w:hyperlink` elements as `(rId_or_empty, anchor_or_empty, display_text)`.
    ///
    /// External links have a non-empty rId; internal bookmark links have a non-empty anchor.
    pub fn list_body_hyperlinks(&mut self) -> Result<Vec<(String, String, String)>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for e in body.descendants() {
            if e.local_name != "hyperlink" {
                continue;
            }
            let rid = e
                .get_attribute_qname("r:id")
                .or_else(|| e.get_attribute("id"))
                .unwrap_or("")
                .to_string();
            let anchor = e
                .get_attribute_qname("w:anchor")
                .or_else(|| e.get_attribute("anchor"))
                .unwrap_or("")
                .to_string();
            let text = e.inner_text();
            out.push((rid, anchor, text));
        }
        Ok(out)
    }

    /// Whether the body contains any `w:hyperlink` elements.
    pub fn has_body_hyperlinks(&mut self) -> Result<bool> {
        Ok(!self.list_body_hyperlinks()?.is_empty())
    }

    /// Count body `w:hyperlink` elements.
    pub fn body_hyperlink_count(&mut self) -> Result<usize> {
        Ok(self.list_body_hyperlinks()?.len())
    }

    /// Update `w:anchor` on body hyperlinks that currently target `old_anchor`.
    /// Returns the number of elements updated.
    pub fn set_body_hyperlink_anchor(&mut self, old_anchor: &str, new_anchor: &str) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn visit(el: &mut OpenXmlElement, old: &str, new: &str, count: &mut usize) {
            if el.local_name == "hyperlink" {
                let a = el
                    .get_attribute_qname("w:anchor")
                    .or_else(|| el.get_attribute("anchor"));
                if a == Some(old) {
                    el.set_attribute_qname("w:anchor", new);
                    *count += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, old, new, count);
            }
        }
        let mut count = 0usize;
        visit(doc, old_anchor, new_anchor, &mut count);
        Ok(count)
    }

    /// Unwrap body `w:hyperlink` elements matching `rid`, promoting their children in place.
    ///
    /// Returns the number of hyperlink elements unwrapped. Does not touch relationships.
    pub fn unwrap_body_hyperlink(&mut self, rid: &str) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn visit(el: &mut OpenXmlElement, rid: &str, count: &mut usize) {
            let mut i = 0;
            while i < el.children.len() {
                let is_match = el.children[i].local_name == "hyperlink"
                    && el.children[i]
                        .get_attribute_qname("r:id")
                        .or_else(|| el.children[i].get_attribute("id"))
                        == Some(rid);
                if is_match {
                    let removed = el.children.remove(i);
                    let kids = removed.children;
                    let n = kids.len();
                    for (offset, kid) in kids.into_iter().enumerate() {
                        el.children.insert(i + offset, kid);
                    }
                    *count += 1;
                    // Continue after the promoted children; do not re-walk them as parents
                    // of nested hyperlinks in this pass — nested hyperlinks are uncommon.
                    i += n;
                } else {
                    visit(&mut el.children[i], rid, count);
                    i += 1;
                }
            }
        }
        let mut count = 0usize;
        visit(doc, rid, &mut count);
        Ok(count)
    }

    /// Remove the relationship for `rid` and unwrap matching body `w:hyperlink` elements.
    ///
    /// Returns `(relationship_removed, body_elements_unwrapped)`.
    pub fn remove_body_hyperlink(&mut self, rid: &str) -> Result<(bool, usize)> {
        let unwrapped = self.unwrap_body_hyperlink(rid)?;
        let removed = self.remove_hyperlink_by_id(rid)?;
        Ok((removed, unwrapped))
    }

    /// Unwrap all body `w:hyperlink` elements (external and anchor) and drop their relationships.
    ///
    /// Returns `(relationships_removed, body_elements_unwrapped)`.
    pub fn clear_body_hyperlinks(&mut self) -> Result<(usize, usize)> {
        let links = self.list_body_hyperlinks()?;
        let mut rel_removed = 0usize;
        let mut body_unwrapped = 0usize;
        let mut seen_rids = std::collections::HashSet::new();
        for (rid, _anchor, _text) in &links {
            if !rid.is_empty() && seen_rids.insert(rid.clone()) {
                let (r, u) = self.remove_body_hyperlink(rid)?;
                if r {
                    rel_removed += 1;
                }
                body_unwrapped += u;
            }
        }
        // Unwrap remaining anchor-only hyperlinks (no rId)
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn visit_anchor(el: &mut OpenXmlElement, count: &mut usize) {
            let mut i = 0;
            while i < el.children.len() {
                if el.children[i].local_name == "hyperlink" {
                    let removed = el.children.remove(i);
                    let kids = removed.children;
                    let n = kids.len();
                    for (offset, kid) in kids.into_iter().enumerate() {
                        el.children.insert(i + offset, kid);
                    }
                    *count += 1;
                    i += n;
                } else {
                    visit_anchor(&mut el.children[i], count);
                    i += 1;
                }
            }
        }
        visit_anchor(doc, &mut body_unwrapped);
        Ok((rel_removed, body_unwrapped))
    }

    /// Remove external hyperlink relationship(s) by URL and unwrap matching body `w:hyperlink`s.
    ///
    /// Returns whether any relationships were removed. Body elements whose `r:id` matched a
    /// removed relationship are unwrapped (children promoted) so the document stays coherent.
    pub fn remove_external_hyperlink(&mut self, url: &str) -> bool {
        let Some(main) = self.main_document_part.as_ref() else {
            return false;
        };
        let main_uri = main.part().uri.clone();
        let ids: Vec<String> = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .map(|rels| {
                rels.find_all_by_type(rel::HYPERLINK)
                    .into_iter()
                    .filter(|r| r.target == url)
                    .map(|r| r.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        if ids.is_empty() {
            return false;
        }
        // Unwrap body hyperlinks before dropping relationships so rIds still resolve for matching.
        for id in &ids {
            let _ = self.unwrap_body_hyperlink(id);
        }
        let Some(main) = self.main_document_part.as_ref() else {
            return true;
        };
        let main_uri = main.part().uri.clone();
        let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
        for id in ids {
            rels.remove(&id);
        }
        true
    }

    /// Remove external hyperlink relationship(s) by URL without rewriting the body DOM.
    ///
    /// Prefer [`remove_external_hyperlink`](Self::remove_external_hyperlink) which also unwraps
    /// matching body elements.
    pub fn remove_external_hyperlink_relationship_only(&mut self, url: &str) -> bool {
        let Some(main) = self.main_document_part.as_ref() else {
            return false;
        };
        let main_uri = main.part().uri.clone();
        let ids: Vec<String> = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .map(|rels| {
                rels.find_all_by_type(rel::HYPERLINK)
                    .into_iter()
                    .filter(|r| r.target == url)
                    .map(|r| r.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        if ids.is_empty() {
            return false;
        }
        let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
        for id in ids {
            rels.remove(&id);
        }
        true
    }

    /// Remove all body children except a trailing `sectPr` (if present).
    /// Remove all external hyperlink relationships and unwrap matching body elements.
    ///
    /// Returns `(relationships_removed, body_elements_unwrapped)`.
    pub fn clear_external_hyperlinks(&mut self) -> Result<(usize, usize)> {
        let links = self.list_external_hyperlinks();
        let mut rels = 0usize;
        let mut body = 0usize;
        let mut seen = std::collections::HashSet::new();
        for (rid, _url) in links {
            if seen.insert(rid.clone()) {
                let u = self.unwrap_body_hyperlink(&rid)?;
                body += u;
                if self.remove_hyperlink_by_id(&rid)? {
                    rels += 1;
                }
            }
        }
        Ok((rels, body))
    }

    pub fn clear_body(&mut self) -> Result<()> {
        let body = self.body_mut()?;
        let sect = body
            .children
            .iter()
            .find(|c| c.local_name == "sectPr")
            .cloned();
        body.children.clear();
        if let Some(s) = sect {
            body.append_child(s);
        }
        Ok(())
    }

    /// Add a minimal Office theme part.
    pub fn add_default_theme(&mut self) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_theme_part(&mut self.package, default_theme("Office Theme"))
    }

    /// Add a minimal font table part (`/word/fontTable.xml`).
    pub fn add_default_font_table(&mut self) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_font_table_part(&mut self.package, default_font_table())
    }

    /// Add a minimal web settings part (`/word/webSettings.xml`).
    pub fn add_default_web_settings(&mut self) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        main.add_web_settings_part(&mut self.package, default_web_settings())
    }

    fn ensure_web_settings_root(&mut self) -> Result<(PackUri, OpenXmlElement)> {
        let uri = PackUri::new("/word/webSettings.xml");
        if let Some(data) = self.package.opc().get_part(&uri) {
            return Ok((uri, parse_element(data)?));
        }
        self.add_default_web_settings()?;
        let data = self
            .package
            .opc()
            .get_part(&uri)
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))?;
        Ok((uri, parse_element(data)?))
    }

    fn save_web_settings(&mut self, uri: PackUri, root: &OpenXmlElement) -> Result<()> {
        self.package.set_part(
            uri,
            content_type::WORD_WEB_SETTINGS,
            crate::element::write_element(root)?,
        );
        Ok(())
    }

    /// Set a web settings OnOff flag (e.g. `"optimizeForBrowser"`, `"allowPNG"`).
    pub fn set_web_settings_flag(&mut self, local_name: &str, enabled: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_web_settings_root()?;
        root.children.retain(|c| c.local_name != local_name);
        if enabled {
            root.append_child(OpenXmlElement::w(local_name));
        }
        self.save_web_settings(uri, &root)
    }

    /// Whether a web settings flag is present.
    pub fn has_web_settings_flag(&self, local_name: &str) -> Result<bool> {
        let uri = PackUri::new("/word/webSettings.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child(local_name).is_some())
    }

    /// Convenience: optimizeForBrowser.
    /// Disable a named web settings flag. Returns whether it was enabled.
    pub fn clear_web_settings_flag(&mut self, local_name: &str) -> Result<bool> {
        let had = self.has_web_settings_flag(local_name)?;
        if had {
            self.set_web_settings_flag(local_name, false)?;
        }
        Ok(had)
    }

    pub fn set_optimize_for_browser(&mut self, enabled: bool) -> Result<()> {
        self.set_web_settings_flag("optimizeForBrowser", enabled)
    }

    /// Whether optimizeForBrowser is set.
    pub fn has_optimize_for_browser(&self) -> Result<bool> {
        self.has_web_settings_flag("optimizeForBrowser")
    }

    /// Convenience: allowPNG.
    /// Disable `optimize for browser`. Returns whether it was enabled.
    pub fn clear_optimize_for_browser(&mut self) -> Result<bool> {
        let had = self.has_optimize_for_browser()?;
        if had {
            self.set_optimize_for_browser(false)?;
        }
        Ok(had)
    }

    pub fn set_allow_png(&mut self, enabled: bool) -> Result<()> {
        self.set_web_settings_flag("allowPNG", enabled)
    }

    /// Whether allowPNG is set.
    pub fn has_allow_png(&self) -> Result<bool> {
        self.has_web_settings_flag("allowPNG")
    }

    /// Convenience: relyOnVML.
    /// Disable `allow png`. Returns whether it was enabled.
    pub fn clear_allow_png(&mut self) -> Result<bool> {
        let had = self.has_allow_png()?;
        if had {
            self.set_allow_png(false)?;
        }
        Ok(had)
    }

    pub fn set_rely_on_vml(&mut self, enabled: bool) -> Result<()> {
        self.set_web_settings_flag("relyOnVML", enabled)
    }

    /// Whether relyOnVML is set.
    pub fn has_rely_on_vml(&self) -> Result<bool> {
        self.has_web_settings_flag("relyOnVML")
    }

    /// Convenience: doNotRelyOnCSS.
    /// Disable `rely on vml`. Returns whether it was enabled.
    pub fn clear_rely_on_vml(&mut self) -> Result<bool> {
        let had = self.has_rely_on_vml()?;
        if had {
            self.set_rely_on_vml(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_rely_on_css(&mut self, enabled: bool) -> Result<()> {
        self.set_web_settings_flag("doNotRelyOnCSS", enabled)
    }

    /// Whether doNotRelyOnCSS is set.
    pub fn has_do_not_rely_on_css(&self) -> Result<bool> {
        self.has_web_settings_flag("doNotRelyOnCSS")
    }

    /// Convenience: doNotOrganizeInFolder.
    /// Disable `do not rely on css`. Returns whether it was enabled.
    pub fn clear_do_not_rely_on_css(&mut self) -> Result<bool> {
        let had = self.has_do_not_rely_on_css()?;
        if had {
            self.set_do_not_rely_on_css(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_organize_in_folder(&mut self, enabled: bool) -> Result<()> {
        self.set_web_settings_flag("doNotOrganizeInFolder", enabled)
    }

    /// Whether doNotOrganizeInFolder is set.
    pub fn has_do_not_organize_in_folder(&self) -> Result<bool> {
        self.has_web_settings_flag("doNotOrganizeInFolder")
    }

    /// Convenience: doNotUseLongFileNames.
    /// Disable `do not organize in folder`. Returns whether it was enabled.
    pub fn clear_do_not_organize_in_folder(&mut self) -> Result<bool> {
        let had = self.has_do_not_organize_in_folder()?;
        if had {
            self.set_do_not_organize_in_folder(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_use_long_file_names(&mut self, enabled: bool) -> Result<()> {
        self.set_web_settings_flag("doNotUseLongFileNames", enabled)
    }

    /// Whether doNotUseLongFileNames is set.
    pub fn has_do_not_use_long_file_names(&self) -> Result<bool> {
        self.has_web_settings_flag("doNotUseLongFileNames")
    }

    /// Convenience: doNotSaveAsSingleFile.
    /// Disable `do not use long file names`. Returns whether it was enabled.
    pub fn clear_do_not_use_long_file_names(&mut self) -> Result<bool> {
        let had = self.has_do_not_use_long_file_names()?;
        if had {
            self.set_do_not_use_long_file_names(false)?;
        }
        Ok(had)
    }

    pub fn set_do_not_save_as_single_file(&mut self, enabled: bool) -> Result<()> {
        self.set_web_settings_flag("doNotSaveAsSingleFile", enabled)
    }

    /// Whether doNotSaveAsSingleFile is set.
    pub fn has_do_not_save_as_single_file(&self) -> Result<bool> {
        self.has_web_settings_flag("doNotSaveAsSingleFile")
    }

    /// Set target screen size (`w:targetScreenSz w:val`), e.g. `"800x600"`, `"1024x768"`.
    /// Disable `do not save as single file`. Returns whether it was enabled.
    pub fn clear_do_not_save_as_single_file(&mut self) -> Result<bool> {
        let had = self.has_do_not_save_as_single_file()?;
        if had {
            self.set_do_not_save_as_single_file(false)?;
        }
        Ok(had)
    }

    pub fn set_target_screen_size(&mut self, val: &str) -> Result<()> {
        let (uri, mut root) = self.ensure_web_settings_root()?;
        root.children.retain(|c| c.local_name != "targetScreenSz");
        root.append_child(
            OpenXmlElement::w("targetScreenSz").with_attribute_qname("w:val", val),
        );
        self.save_web_settings(uri, &root)
    }

    /// Read targetScreenSz.
    pub fn target_screen_size(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/word/webSettings.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.child("targetScreenSz").and_then(|e| {
            e.get_attribute_qname("w:val")
                .or_else(|| e.get_attribute("val"))
                .map(|s| s.to_string())
        }))
    }

    /// Set pixels per inch in web settings (`w:pixelsPerInch w:val`).
    pub fn set_pixels_per_inch(&mut self, ppi: u32) -> Result<()> {
        let (uri, mut root) = self.ensure_web_settings_root()?;
        root.children.retain(|c| c.local_name != "pixelsPerInch");
        root.append_child(
            OpenXmlElement::w("pixelsPerInch").with_attribute_qname("w:val", ppi.to_string()),
        );
        self.save_web_settings(uri, &root)
    }

    /// Read pixelsPerInch.
    pub fn pixels_per_inch(&self) -> Result<Option<u32>> {
        let uri = PackUri::new("/word/webSettings.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.child("pixelsPerInch").and_then(|e| {
            e.get_attribute_qname("w:val")
                .or_else(|| e.get_attribute("val"))
                .and_then(|s| s.parse().ok())
        }))
    }

    /// Set web encoding (`w:encoding w:val`).
    pub fn set_web_encoding(&mut self, encoding: &str) -> Result<()> {
        let (uri, mut root) = self.ensure_web_settings_root()?;
        root.children.retain(|c| c.local_name != "encoding");
        root.append_child(
            OpenXmlElement::w("encoding").with_attribute_qname("w:val", encoding),
        );
        self.save_web_settings(uri, &root)
    }

    /// Read web encoding.
    pub fn web_encoding(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/word/webSettings.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.child("encoding").and_then(|e| {
            e.get_attribute_qname("w:val")
                .or_else(|| e.get_attribute("val"))
                .map(|s| s.to_string())
        }))
    }

    /// Clear web encoding. Returns whether present.
    pub fn clear_web_encoding(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/webSettings.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "encoding");
        let removed = root.children.len() < before;
        if removed {
            self.save_web_settings(uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear targetScreenSz. Returns whether present.
    pub fn clear_target_screen_size(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/webSettings.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "targetScreenSz");
        let removed = root.children.len() < before;
        if removed {
            self.save_web_settings(uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear pixelsPerInch. Returns whether present.
    pub fn clear_pixels_per_inch(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/webSettings.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "pixelsPerInch");
        let removed = root.children.len() < before;
        if removed {
            self.save_web_settings(uri, &root)?;
        }
        Ok(removed)
    }

    /// Clone this document into a new in-memory package (deep copy of all parts).
    ///
    /// C# `CloneableExtensions.Clone()` (MemoryStream, editable).
    pub fn clone_document(&mut self) -> Result<Self> {
        self.flush_parts()?;
        let bytes = self.package.to_bytes()?;
        Self::open_bytes(bytes)
    }

    /// Clone to a new file path (C# `Clone(string path)` / `Clone(path, isEditable, settings)`).
    pub fn clone_to_path(&mut self, path: impl AsRef<std::path::Path>) -> Result<Self> {
        self.flush_parts()?;
        let path = path.as_ref();
        let bytes = self.package.to_bytes()?;
        let mut cloned = Self::open_bytes(bytes)?;
        *cloned.settings_mut() = self.settings().clone();
        cloned.save_as(path)?;
        // Re-open from path so the clone is path-backed like C# File-based Clone.
        let settings = cloned.settings().clone();
        drop(cloned);
        Self::open_with_settings(path, true, settings)
    }

    /// Clone into raw package bytes (caller may open with [`open_bytes`](Self::open_bytes)).
    pub fn clone_to_bytes(&mut self) -> Result<Vec<u8>> {
        self.flush_parts()?;
        self.package.to_bytes()
    }

    /// Clone and write ZIP bytes to a writer (C# `Clone(Stream)` shell).
    pub fn clone_to_writer<W: std::io::Write>(&mut self, mut writer: W) -> Result<()> {
        let bytes = self.clone_to_bytes()?;
        writer.write_all(&bytes)?;
        Ok(())
    }

    /// Embed an alternative format chunk (HTML, plain text, RTF, …) and append
    /// a `w:altChunk` reference to the document body.
    ///
    /// Returns the relationship id of the imported part.
    pub fn add_alt_chunk(
        &mut self,
        format: AlternativeFormatImportType,
        data: impl Into<Vec<u8>>,
    ) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let (rid, _) = main.add_alt_chunk_part(&mut self.package, format, data)?;

        // Append altChunk to body
        {
            let package = &self.package;
            let main = self.main_document_part.as_mut().unwrap();
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let body = doc
            .child_mut("body")
            .ok_or_else(|| Error::Package("document has no body".into()))?;
        // Insert before trailing sectPr if present
        let chunk = alt_chunk(&rid);
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, chunk);
        } else {
            body.append_child(chunk);
        }
        Ok(rid)
    }

    /// Convert to Flat OPC XML (single XML document representation).
    pub fn to_flat_opc_string(&mut self) -> Result<String> {
        self.flush_parts()?;
        let bytes = to_flat_opc(self.package.opc(), Some(progid::WORD))?;
        String::from_utf8(bytes).map_err(|e| Error::Xml(e.to_string()))
    }

    /// Open a Word document from a Flat OPC XML string/bytes.
    pub fn from_flat_opc(xml: impl AsRef<[u8]>) -> Result<Self> {
        let opc = from_flat_opc(xml)?;
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        Self::from_opc(opc, settings)
    }

    /// Normalize Strict OOXML namespaces/relationships to Transitional.
    ///
    /// Returns `(xml_replacements, relationship_replacements)`.
    pub fn rewrite_strict_to_transitional(&mut self) -> Result<(usize, usize)> {
        self.flush_parts()?;
        // Clear loaded DOM so it reloads from rewritten parts
        if let Some(main) = &mut self.main_document_part {
            main.part_mut().root = None;
            main.part_mut().dirty = false;
        }
        crate::namespace_rewrite::rewrite_package_to_transitional(self.package.opc_mut())
    }

    /// Normalize Transitional OOXML namespaces/relationships to Strict.
    pub fn rewrite_transitional_to_strict(&mut self) -> Result<(usize, usize)> {
        self.flush_parts()?;
        if let Some(main) = &mut self.main_document_part {
            main.part_mut().root = None;
            main.part_mut().dirty = false;
        }
        crate::namespace_rewrite::rewrite_package_to_strict(self.package.opc_mut())
    }

    /// Access open settings.
    pub fn settings(&self) -> &OpenSettings {
        self.package.settings()
    }

    /// Mutable access to open settings.
    pub fn settings_mut(&mut self) -> &mut OpenSettings {
        self.package.settings_mut()
    }

    /// Whether auto-save is enabled.
    pub fn auto_save(&self) -> bool {
        self.package.auto_save()
    }

    /// Detect an encrypted Office compound file at `path`.
    pub fn is_encrypted_office_file(path: impl AsRef<std::path::Path>) -> Result<bool> {
        OpcPackage::is_encrypted_office_file(path)
    }

    /// Add a footnotes part and insert a footnote reference at the end of the body.
    ///
    /// Creates separator/continuation notes plus one normal note with `id`.
    pub fn add_footnote(&mut self, id: &str, body_text: &str) -> Result<String> {
        use crate::wordprocessing::footnote;
        let uri = PackUri::new("/word/footnotes.xml");
        let rid = if self.package.opc().has_part(&uri) {
            // Append/replace note entry in existing part
            let data = self.package.opc().get_part(&uri).unwrap();
            let mut root = parse_element(data)?;
            root.children.retain(|c| {
                !(c.local_name == "footnote"
                    && c.get_attribute_qname("w:id").or_else(|| c.get_attribute("id"))
                        == Some(id))
            });
            root.append_child(footnote(id, None, body_text));
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_FOOTNOTES, xml);
            // existing relationship id if any
            let main = self
                .main_document_part
                .as_ref()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            self.package
                .opc()
                .part_relationships(&main.part().uri)
                .and_then(|r| r.get_by_type(rel::FOOTNOTES).map(|x| x.id.clone()))
                .unwrap_or_else(|| "rIdFootnotes".into())
        } else {
            let main = self
                .main_document_part
                .as_ref()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.add_footnotes_part(
                &mut self.package,
                default_footnotes_with(id, body_text),
            )?
        };
        // Append a paragraph with the reference
        {
            let package = &self.package;
            let main = self.main_document_part.as_mut().unwrap();
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let body = doc
            .child_mut("body")
            .ok_or_else(|| Error::Package("document has no body".into()))?;
        let para = paragraph(vec![
            run(vec![text("See note")]),
            footnote_ref_run(id),
        ]);
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(rid)
    }

    /// Add an endnotes part and insert an endnote reference at the end of the body.
    pub fn add_endnote(&mut self, id: &str, body_text: &str) -> Result<String> {
        use crate::wordprocessing::endnote;
        let uri = PackUri::new("/word/endnotes.xml");
        let rid = if self.package.opc().has_part(&uri) {
            let data = self.package.opc().get_part(&uri).unwrap();
            let mut root = parse_element(data)?;
            root.children.retain(|c| {
                !(c.local_name == "endnote"
                    && c.get_attribute_qname("w:id").or_else(|| c.get_attribute("id"))
                        == Some(id))
            });
            root.append_child(endnote(id, None, body_text));
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_ENDNOTES, xml);
            let main = self
                .main_document_part
                .as_ref()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            self.package
                .opc()
                .part_relationships(&main.part().uri)
                .and_then(|r| r.get_by_type(rel::ENDNOTES).map(|x| x.id.clone()))
                .unwrap_or_else(|| "rIdEndnotes".into())
        } else {
            let main = self
                .main_document_part
                .as_ref()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.add_endnotes_part(
                &mut self.package,
                default_endnotes_with(id, body_text),
            )?
        };
        {
            let package = &self.package;
            let main = self.main_document_part.as_mut().unwrap();
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let body = doc
            .child_mut("body")
            .ok_or_else(|| Error::Package("document has no body".into()))?;
        let para = paragraph(vec![
            run(vec![text("See endnote")]),
            endnote_ref_run(id),
        ]);
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(rid)
    }

    /// Add an Office Web Extension part shell + taskpanes part.
    ///
    /// Returns `(webextension_uri, taskpanes_uri)`.
    pub fn add_web_extension_shell(
        &mut self,
        store_id: &str,
        version: &str,
    ) -> Result<(PackUri, PackUri)> {
        let we_uri = PackUri::new("/word/webextensions/webextension1.xml");
        let tp_uri = PackUri::new("/word/webextensions/taskpanes.xml");
        let we = "http://schemas.microsoft.com/office/webextensions/webextension/2010/11";
        let wetp =
            "http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11";
        let ext = OpenXmlElement::new("we", we, "webextension")
            .with_ns_decl("we", we)
            .with_attribute("id", format!("{{{}-0000-0000-0000-000000000000}}", store_id))
            .with_child(
                OpenXmlElement::new("we", we, "reference")
                    .with_attribute("id", store_id)
                    .with_attribute("version", version)
                    .with_attribute("store", "developer")
                    .with_attribute("storeType", "Registry"),
            )
            .with_child(OpenXmlElement::new("we", we, "alternateReferences"))
            .with_child(
                OpenXmlElement::new("we", we, "properties").with_child(
                    OpenXmlElement::new("we", we, "property")
                        .with_attribute("name", "Office.AutoShowTaskpaneWithDocument")
                        .with_attribute("value", "true"),
                ),
            )
            .with_child(OpenXmlElement::new("we", we, "bindings"))
            .with_child(OpenXmlElement::new("we", we, "snapshot"));
        let taskpanes = OpenXmlElement::new("wetp", wetp, "taskpanes")
            .with_ns_decl("wetp", wetp)
            .with_ns_decl("r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships")
            .with_child(
                OpenXmlElement::new("wetp", wetp, "taskpane")
                    .with_attribute("dockstate", "right")
                    .with_attribute("visibility", "1")
                    .with_attribute("width", "350")
                    .with_attribute("row", "1")
                    .with_child(
                        OpenXmlElement::new("wetp", wetp, "webextensionref")
                            .with_attribute_qname("r:id", "rId1"),
                    ),
            );
        self.package.set_part(
            we_uri.clone(),
            content_type::WEB_EXTENSION,
            crate::element::write_element(&ext)?,
        );
        self.package.set_part(
            tp_uri.clone(),
            content_type::WEB_EXTENSION_TASKPANES,
            crate::element::write_element(&taskpanes)?,
        );
        // taskpanes → webextension
        self.package.add_part_relationship(
            &tp_uri,
            rel::WEB_EXTENSION,
            &we_uri,
            RelationshipTargetMode::Internal,
        );
        // package → taskpanes
        if self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::WEB_EXTENSION_TASKPANES)
            .is_none()
        {
            self.package.add_package_relationship(
                rel::WEB_EXTENSION_TASKPANES,
                &tp_uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok((we_uri, tp_uri))
    }

    /// Add comments IDs part shell (Office 2016+).
    pub fn add_comments_ids(
        &mut self,
        ids: &[(&str, &str)],
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/commentsIds.xml");
        let w16cid = "http://schemas.microsoft.com/office/word/2016/wordml/cid";
        let mut root =
            OpenXmlElement::new("w16cid", w16cid, "commentsIds").with_ns_decl("w16cid", w16cid);
        for (para_id, durable_id) in ids {
            root.append_child(
                OpenXmlElement::new("w16cid", w16cid, "commentId")
                    .with_attribute_qname("w16cid:paraId", *para_id)
                    .with_attribute_qname("w16cid:durableId", *durable_id),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::WORD_COMMENTS_IDS,
            crate::element::write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::COMMENTS_IDS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add comments extensible part shell (Office 2018+).
    pub fn add_comments_extensible(&mut self) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/commentsExtensible.xml");
        let w16cex = "http://schemas.microsoft.com/office/word/2018/wordml/cex";
        let root = OpenXmlElement::new("w16cex", w16cex, "commentsExtensible")
            .with_ns_decl("w16cex", w16cex);
        self.package.set_part(
            uri.clone(),
            content_type::WORD_COMMENTS_EXTENSIBLE,
            crate::element::write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::COMMENTS_EXTENSIBLE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add Word attached toolbars binary shell.
    pub fn add_attached_toolbars(
        &mut self,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/attachedToolbars.bin");
        self.package.set_part(
            uri.clone(),
            content_type::ATTACHED_TOOLBARS,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::ATTACHED_TOOLBARS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a custom property part related from the main document.
    ///
    /// Corresponds to C# `CustomPropertyPart` (generic XML content type).
    /// Returns `(relationship_id, uri)`.
    pub fn add_custom_property(
        &mut self,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/word/customProperty{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            content_type::CUSTOM_PROPERTY_XML,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::CUSTOM_PROPERTY,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add an embedded font part related from the main document.
    ///
    /// `content_type_str` should be one of `content_type::FONT_DATA`, `FONT_TTF`,
    /// or `FONT_ODTTF`. Returns `(relationship_id, uri)`.
    pub fn add_font_part(
        &mut self,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let ext = extension.trim_start_matches('.');
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/word/fonts/font{index}.{ext}"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::FONT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }


    /// Whether any embedded font parts exist under `/word/fonts/`.
    pub fn has_font_parts(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/word/fonts/"))
    }

    /// Count embedded font parts under `/word/fonts/`.
    pub fn font_part_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/fonts/"))
            .count()
    }

    /// List embedded font part URIs.
    pub fn list_font_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/fonts/"))
            
            .collect()
    }

    /// Remove all embedded font parts and related main-part font relationships.
    pub fn clear_font_parts(&mut self) -> Result<usize> {
        let uris = self.list_font_parts();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::FONT)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Add a DrawingML chart part shell related from the main document.
    ///
    /// Creates a minimal bar chart under `/word/charts/`. Returns `(relationship_id, uri)`.
    pub fn add_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/word/charts/chart{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let chart = crate::spreadsheet::bar_chart_space(title, categories, values);
        self.package.set_part(
            uri.clone(),
            content_type::DRAWINGML_CHART,
            crate::element::write_element(&chart)?,
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::CHART,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }



    /// Whether any drawing/chart-related drawing parts exist under `/word/drawings/`.
    pub fn has_drawings(&self) -> bool {
        self.drawing_count() > 0
    }

    /// Count parts under `/word/drawings/`.
    pub fn drawing_count(&self) -> usize {
        self.list_drawings().len()
    }

    /// List drawing part URIs under `/word/drawings/`.
    pub fn list_drawings(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/drawings/"))
            
            .collect()
    }

    /// Remove a single drawing part by URI and drop main-document relationships that target it.
    pub fn remove_drawing(&mut self, drawing_uri: &PackUri) -> Result<bool> {
        if !drawing_uri.as_str().starts_with("/word/drawings/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(drawing_uri) {
            return Ok(false);
        }
        let target = drawing_uri.as_str().to_string();
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            let t = r.target.as_str();
                            crate::opc::resolve_uri(&main_uri, t)
                                .map(|u| u.as_str() == target)
                                .unwrap_or(false)
                                || t == target
                                || t.ends_with(target.trim_start_matches('/'))
                                || target.ends_with(t.trim_start_matches("./"))
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(drawing_uri);
        Ok(true)
    }

    /// Remove `/word/drawings/` parts and related relationships from the main document.
    pub fn clear_drawings(&mut self) -> Result<usize> {
        let uris = self.list_drawings();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type.contains("drawing")
                                || r.target.contains("drawings/")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Whether any chart parts exist under `/word/charts/`.
    pub fn has_charts(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/word/charts/"))
    }

    /// Count chart parts under `/word/charts/`.
    pub fn chart_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/charts/"))
            .count()
    }

    /// List chart part URIs.
    pub fn list_charts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/charts/"))
            
            .collect()
    }

    /// Remove a single chart part by URI and drop main-document relationships that target it.
    pub fn remove_chart(&mut self, chart_uri: &PackUri) -> Result<bool> {
        if !chart_uri.as_str().starts_with("/word/charts/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(chart_uri) {
            return Ok(false);
        }
        let target = chart_uri.as_str().to_string();
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            let t = r.target.as_str();
                            crate::opc::resolve_uri(&main_uri, t)
                                .map(|u| u.as_str() == target)
                                .unwrap_or(false)
                                || t == target
                                || t.ends_with(target.trim_start_matches('/'))
                                || target.ends_with(t.trim_start_matches("./"))
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(chart_uri);
        Ok(true)
    }

    /// Remove chart part at 0-based index among [`list_charts`](Self::list_charts).
    pub fn remove_chart_at(&mut self, index: usize) -> Result<bool> {
        let charts = self.list_charts();
        let Some(uri) = charts.get(index).cloned() else {
            return Ok(false);
        };
        self.remove_chart(&uri)
    }

    /// Remove all chart parts and main-document chart relationships.
    pub fn clear_charts(&mut self) -> Result<usize> {
        let uris = self.list_charts();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::CHART
                                || r.relationship_type.contains("chart")
                                || r.target.contains("charts/")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Add legacy diagram text parts shell (VML diagram text).
    pub fn add_legacy_diagram_text(
        &mut self,
        text_data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let text_uri = loop {
            let c = PackUri::new(format!("/word/diagrams/legacy/text{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let info_uri = PackUri::new(format!("/word/diagrams/legacy/textInfo{index}.xml"));
        self.package.set_part(
            text_uri.clone(),
            content_type::LEGACY_DIAGRAM_TEXT,
            text_data.into(),
        );
        let dgm = "http://schemas.microsoft.com/office/drawing/2008/diagram";
        let info = OpenXmlElement::new("dgm", dgm, "textInfo")
            .with_ns_decl("dgm", dgm);
        self.package.set_part(
            info_uri.clone(),
            content_type::LEGACY_DIAGRAM_TEXT_INFO,
            crate::element::write_element(&info)?,
        );
        self.package.add_part_relationship(
            &main_uri,
            rel::LEGACY_DIAGRAM_TEXT,
            &text_uri,
            RelationshipTargetMode::Internal,
        );
        self.package.add_part_relationship(
            &text_uri,
            rel::LEGACY_DIAGRAM_TEXT_INFO,
            &info_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((text_uri, info_uri))
    }

    /// Add an embedded package (e.g. nested xlsx/docx) part shell.
    pub fn add_embedded_package_part(
        &mut self,
        data: impl Into<Vec<u8>>,
        extension: &str,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!(
                "/word/embeddings/Microsoft_Object{index}.{extension}"
            ));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            content_type::PACKAGE_EMBEDDED,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::PACKAGE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add styles with effects part shell (Office 2007 compatibility).
    pub fn add_styles_with_effects(&mut self) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/stylesWithEffects.xml");
        // Copy default styles content
        let styles = crate::packaging::parts::default_styles();
        self.package.set_part(
            uri.clone(),
            content_type::STYLES_WITH_EFFECTS,
            crate::element::write_element(&styles)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .and_then(|rels| {
                rels.get_by_type(rel::STYLES_WITH_EFFECTS)
                    .map(|r| r.id.clone())
            })
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::STYLES_WITH_EFFECTS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add VBA data part shell (companion to vbaProject).
    pub fn add_vba_data(&mut self) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        // Prefer relating from vbaProject if present
        let parent = if self.package.opc().has_part(&PackUri::new("/word/vbaProject.bin")) {
            PackUri::new("/word/vbaProject.bin")
        } else {
            main_uri
        };
        let uri = PackUri::new("/word/vbaData.xml");
        let wne = "http://schemas.microsoft.com/office/word/2006/wordml";
        let root = OpenXmlElement::new("wne", wne, "vbaSuppData")
            .with_ns_decl("wne", wne)
            .with_child(OpenXmlElement::new("wne", wne, "mcds"));
        self.package.set_part(
            uri.clone(),
            content_type::VBA_DATA,
            crate::element::write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &parent,
            rel::VBA_DATA,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add key-map customizations part shell.
    pub fn add_customization(&mut self) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/customizations/customization.xml");
        let wne = "http://schemas.microsoft.com/office/word/2006/wordml";
        let root = OpenXmlElement::new("wne", wne, "tcg")
            .with_ns_decl("wne", wne)
            .with_child(OpenXmlElement::new("wne", wne, "keymaps"));
        self.package.set_part(
            uri.clone(),
            content_type::CUSTOMIZATION,
            crate::element::write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::CUSTOMIZATION,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add Quick Access Toolbar customizations part shell.
    pub fn add_quick_access_toolbar(&mut self) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/customUI/qatCustomizations.xml");
        let mso = "http://schemas.microsoft.com/office/2006/01/customui";
        let root = OpenXmlElement::new("mso", mso, "customUI")
            .with_ns_decl("mso", mso)
            .with_child(
                OpenXmlElement::new("mso", mso, "ribbon").with_child(
                    OpenXmlElement::new("mso", mso, "qat").with_child(
                        OpenXmlElement::new("mso", mso, "sharedControls"),
                    ),
                ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::QAT,
            crate::element::write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::QAT)
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::QAT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a sensitivity / classification label info part shell.
    pub fn add_label_info(&mut self, label_id: &str, name: &str) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/docMetadata/LabelInfo.xml");
        let clbl = "http://schemas.microsoft.com/office/2020/mipLabelMetadata";
        let root = OpenXmlElement::new("clbl", clbl, "labelList")
            .with_ns_decl("clbl", clbl)
            .with_child(
                OpenXmlElement::new("clbl", clbl, "label")
                    .with_attribute("id", label_id)
                    .with_attribute("name", name)
                    .with_attribute("enabled", "1"),
            );
        self.package.set_part(
            uri.clone(),
            content_type::LABEL_INFO,
            crate::element::write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::LABEL_INFO)
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::LABEL_INFO,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add an embedded OLE object binary part shell.
    pub fn add_embedded_object(
        &mut self,
        data: impl Into<Vec<u8>>,
        prog_id: &str,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/word/embeddings/oleObject{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let _ = prog_id;
        self.package.set_part(
            uri.clone(),
            "application/vnd.openxmlformats-officedocument.oleObject",
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::OLE_OBJECT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a SmartArt / diagram parts shell (data, layout, colors, style, persist layout).
    ///
    /// Creates minimal diagram parts under `/word/diagrams/` related from the main
    /// document (persist layout is related from the data part). Returns the data part URI.
    pub fn add_diagram_shell(&mut self, unique_id: &str) -> Result<PackUri> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let data_uri = loop {
            let c = PackUri::new(format!("/word/diagrams/data{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let layout_uri = PackUri::new(format!("/word/diagrams/layout{index}.xml"));
        let colors_uri = PackUri::new(format!("/word/diagrams/colors{index}.xml"));
        let style_uri = PackUri::new(format!("/word/diagrams/quickStyle{index}.xml"));
        let drawing_uri = PackUri::new(format!("/word/diagrams/drawing{index}.xml"));
        let dgm = "http://schemas.openxmlformats.org/drawingml/2006/diagram";
        let a = crate::namespace::ns::DRAWINGML.uri;
        let dsp = "http://schemas.microsoft.com/office/drawing/2008/diagram";
        // Minimal data model
        let data = OpenXmlElement::new("dgm", dgm, "dataModel")
            .with_ns_decl("dgm", dgm)
            .with_ns_decl("a", a)
            .with_child(
                OpenXmlElement::new("dgm", dgm, "ptLst")
                    .with_child(
                        OpenXmlElement::new("dgm", dgm, "pt")
                            .with_attribute("modelId", unique_id)
                            .with_attribute("type", "doc"),
                    )
                    .with_child(
                        OpenXmlElement::new("dgm", dgm, "pt")
                            .with_attribute("modelId", format!("{unique_id}-1"))
                            .with_child(
                                OpenXmlElement::new("dgm", dgm, "prSet")
                                    .with_attribute("phldr", "1"),
                            )
                            .with_child(
                                OpenXmlElement::new("a", a, "t").with_child(
                                    OpenXmlElement::new("a", a, "p").with_child(
                                        OpenXmlElement::new("a", a, "r").with_child(
                                            OpenXmlElement::new("a", a, "t").with_text("Node"),
                                        ),
                                    ),
                                ),
                            ),
                    ),
            )
            .with_child(OpenXmlElement::new("dgm", dgm, "cxnLst"));
        let layout = OpenXmlElement::new("dgm", dgm, "layoutDef")
            .with_ns_decl("dgm", dgm)
            .with_attribute("uniqueId", format!("layout-{unique_id}"))
            .with_child(OpenXmlElement::new("dgm", dgm, "title").with_attribute("val", ""))
            .with_child(OpenXmlElement::new("dgm", dgm, "desc").with_attribute("val", ""));
        let colors = OpenXmlElement::new("dgm", dgm, "colorsDef")
            .with_ns_decl("dgm", dgm)
            .with_attribute("uniqueId", format!("colors-{unique_id}"))
            .with_child(OpenXmlElement::new("dgm", dgm, "title").with_attribute("val", ""))
            .with_child(OpenXmlElement::new("dgm", dgm, "desc").with_attribute("val", ""));
        let style = OpenXmlElement::new("dgm", dgm, "styleDef")
            .with_ns_decl("dgm", dgm)
            .with_attribute("uniqueId", format!("style-{unique_id}"))
            .with_child(OpenXmlElement::new("dgm", dgm, "title").with_attribute("val", ""))
            .with_child(OpenXmlElement::new("dgm", dgm, "desc").with_attribute("val", ""));
        // Persist layout / drawing shell (dsp:drawing)
        let drawing = OpenXmlElement::new("dsp", dsp, "drawing")
            .with_ns_decl("dsp", dsp)
            .with_ns_decl("a", a)
            .with_child(OpenXmlElement::new("dsp", dsp, "spTree"));
        for (uri, ct, el) in [
            (&data_uri, content_type::DIAGRAM_DATA, data),
            (&layout_uri, content_type::DIAGRAM_LAYOUT, layout),
            (&colors_uri, content_type::DIAGRAM_COLORS, colors),
            (&style_uri, content_type::DIAGRAM_STYLE, style),
            (&drawing_uri, content_type::DIAGRAM_PERSIST_LAYOUT, drawing),
        ] {
            self.package.set_part(
                uri.clone(),
                ct,
                crate::element::write_element(&el)?,
            );
        }
        // Relate four core diagram parts from main document
        for (uri, rel_ty) in [
            (&data_uri, rel::DIAGRAM_DATA),
            (&layout_uri, rel::DIAGRAM_LAYOUT),
            (&colors_uri, rel::DIAGRAM_COLORS),
            (&style_uri, rel::DIAGRAM_STYLE),
        ] {
            self.package.add_part_relationship(
                &main_uri,
                rel_ty,
                uri,
                RelationshipTargetMode::Internal,
            );
        }
        // Persist layout is a child of the data part
        self.package.add_part_relationship(
            &data_uri,
            rel::DIAGRAM_PERSIST_LAYOUT,
            &drawing_uri,
            RelationshipTargetMode::Internal,
        );
        Ok(data_uri)
    }

    /// Add a Ribbon / Custom UI part shell (`customUI/customUI.xml`).
    ///
    /// `custom_ui_xml` is the raw Custom UI XML document.
    pub fn add_custom_ui(
        &mut self,
        custom_ui_xml: impl AsRef<[u8]>,
    ) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/customUI/customUI.xml");
        self.package.set_part(
            uri.clone(),
            content_type::CUSTOM_UI,
            custom_ui_xml.as_ref().to_vec(),
        );
        // Package-level relationship (Office 2007 custom UI)
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::CUSTOM_UI_2007)
            .or_else(|| {
                self.package
                    .opc()
                    .package_relationships()
                    .get_by_type(rel::CUSTOM_UI)
            })
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::CUSTOM_UI_2007,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a document tasks part shell (Office 2019+).
    pub fn add_document_tasks(
        &mut self,
        task_titles: &[&str],
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/tasks/tasks.xml");
        let t = "http://schemas.microsoft.com/office/tasks/2019/documenttasks";
        let mut root = OpenXmlElement::new("t", t, "Tasks").with_ns_decl("t", t);
        for (i, title) in task_titles.iter().enumerate() {
            root.append_child(
                OpenXmlElement::new("t", t, "Task")
                    .with_attribute("id", format!("{{{i}}}"))
                    .with_child(
                        OpenXmlElement::new("t", t, "Title").with_text(*title),
                    ),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type::DOCUMENT_TASKS, xml);
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .and_then(|rels| {
                rels.get_by_type(rel::DOCUMENT_TASKS)
                    .map(|r| r.id.clone())
            })
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::DOCUMENT_TASKS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a printer settings binary part shell related from the main document.
    pub fn add_printer_settings(
        &mut self,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let mut index = 1u32;
        let uri = loop {
            let candidate =
                PackUri::new(format!("/word/printerSettings/printerSettings{index}.bin"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            uri.clone(),
            content_type::WORD_PRINTER_SETTINGS,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::PRINTER_SETTINGS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a people part listing comment authors (`w15:people`).
    ///
    /// Each entry is `(author, provider_id)` e.g. `("Alice", "AD")`.
    pub fn add_people(&mut self, people: &[(&str, &str)]) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/people.xml");
        let w15 = "http://schemas.microsoft.com/office/word/2012/wordml";
        let mut root =
            OpenXmlElement::new("w15", w15, "people").with_ns_decl("w15", w15);
        for (author, provider) in people {
            root.append_child(
                OpenXmlElement::new("w15", w15, "person")
                    .with_attribute_qname("w15:author", *author)
                    .with_attribute_qname("w15:providerId", *provider),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_PEOPLE, xml);
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .and_then(|rels| rels.get_by_type(rel::PEOPLE).map(|r| r.id.clone()))
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::PEOPLE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add itemProps for a custom XML part (datastore item).
    ///
    /// Creates `/customXml/itemPropsN.xml` related from the customXml item.
    pub fn add_custom_xml_properties(
        &mut self,
        custom_xml_uri: &PackUri,
        item_id: &str,
    ) -> Result<(String, PackUri)> {
        let mut index = 1u32;
        let props_uri = loop {
            let candidate = PackUri::new(format!("/customXml/itemProps{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let ds =
            "http://schemas.openxmlformats.org/officeDocument/2006/customXml";
        let root = OpenXmlElement::new("ds", ds, "datastoreItem")
            .with_ns_decl("ds", ds)
            .with_attribute_qname("ds:itemID", item_id)
            .with_child(OpenXmlElement::new("ds", ds, "schemaRefs"));
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            props_uri.clone(),
            content_type::CUSTOM_XML_PROPERTIES,
            xml,
        );
        let rid = self.package.add_part_relationship(
            custom_xml_uri,
            rel::CUSTOM_XML_PROPS,
            &props_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, props_uri))
    }

    /// Add a VBA project binary part shell (no macro execution).
    ///
    /// Stores raw `vbaProject.bin` bytes and relates from the main document.
    /// Prefer `MacroEnabledDocument` content type for the package to open in Word.
    pub fn add_vba_project(&mut self, data: impl Into<Vec<u8>>) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/vbaProject.bin");
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type::VBA_PROJECT, data.into());
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .and_then(|rels| rels.get_by_type(rel::VBA_PROJECT).map(|r| r.id.clone()))
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::VBA_PROJECT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a commentsExtended part (`commentsEx`) for modern threaded-comment metadata.
    ///
    /// `entries` are `(para_id, durable_id, done)` for each extended comment.
    pub fn add_comments_extended(
        &mut self,
        entries: &[(&str, &str, bool)],
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let uri = PackUri::new("/word/commentsExtended.xml");
        let w15 = "http://schemas.microsoft.com/office/word/2012/wordml";
        let mut root = OpenXmlElement::new("w15", w15, "commentsEx")
            .with_ns_decl("w15", w15)
            .with_ns_decl("w14", "http://schemas.microsoft.com/office/word/2010/wordml");
        for (para_id, durable_id, done) in entries {
            root.append_child(
                OpenXmlElement::new("w15", w15, "commentEx")
                    .with_attribute_qname("w15:paraId", *para_id)
                    .with_attribute_qname("w15:paraIdParent", "")
                    .with_attribute_qname("w15:done", if *done { "1" } else { "0" })
                    .with_attribute("durableId", *durable_id),
            );
        }
        let xml = crate::element::write_element(&root)?;
        self.package.set_part(
            uri.clone(),
            content_type::WORD_COMMENTS_EXTENDED,
            xml,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .and_then(|rels| {
                rels.get_by_type(rel::COMMENTS_EXTENDED)
                    .map(|r| r.id.clone())
            })
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::COMMENTS_EXTENDED,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Add a digital signature origin part shell (no crypto).
    ///
    /// Creates `/_xmlsignatures/origin.sigs` (empty) and a package relationship.
    /// Does **not** create or verify actual signatures.
    pub fn add_digital_signature_origin(&mut self) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/_xmlsignatures/origin.sigs");
        if !self.package.opc().has_part(&uri) {
            // Empty origin part (binary placeholder)
            self.package.set_part(
                uri.clone(),
                content_type::DIGITAL_SIGNATURE_ORIGIN,
                Vec::new(),
            );
        }
        if let Some(existing) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
            .map(|r| r.id.clone())
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_package_relationship(
            rel::DIGITAL_SIGNATURE_ORIGIN,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Attach an XML signature part under the origin (shell only — stores raw XML).
    ///
    /// Returns `(relationship_id, signature_uri)`.
    pub fn add_xml_signature_part(
        &mut self,
        signature_xml: impl AsRef<[u8]>,
    ) -> Result<(String, PackUri)> {
        let (origin_rid, origin_uri) = self.add_digital_signature_origin()?;
        let _ = origin_rid;
        let mut index = 1u32;
        let sig_uri = loop {
            let candidate = PackUri::new(format!("/_xmlsignatures/sig{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            sig_uri.clone(),
            content_type::DIGITAL_SIGNATURE_XML,
            signature_xml.as_ref().to_vec(),
        );
        let rid = self.package.add_part_relationship(
            &origin_uri,
            rel::DIGITAL_SIGNATURE,
            &sig_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, sig_uri))
    }

    /// Whether a digital signature origin part is present.
    pub fn has_digital_signature_origin(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/_xmlsignatures/origin.sigs"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
                .is_some()
    }

    /// Count XML signature parts under `/_xmlsignatures/`.
    pub fn digital_signature_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.starts_with("/_xmlsignatures/") && s.ends_with(".xml")
            })
            .count()
    }

    /// Remove all digital signature parts and package origin relationship.
    pub fn clear_digital_signatures(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/_xmlsignatures/"))
            
            .collect();
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
            .is_some();
        if uris.is_empty() && !had_rel {
            return Ok(false);
        }
        if let Some(id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
            .map(|r| r.id.clone())
        {
            self.package
                .opc_mut()
                .package_relationships_mut()
                .remove(&id);
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Set page setup including page borders on the trailing `w:sectPr`.
    pub fn set_page_borders(&mut self, color: &str, sz: u32) -> Result<()> {
        use crate::wordprocessing::page_borders;
        let body = self.body_mut()?;
        if let Some(sect) = body.child_mut("sectPr") {
            sect.children.retain(|c| c.local_name != "pgBorders");
            sect.append_child(page_borders(color, sz));
        } else {
            let mut sect = crate::wordprocessing::section_properties_with_page(
                12240, 15840, 1440, 1440, 1440, 1440,
            );
            sect.append_child(page_borders(color, sz));
            body.append_child(sect);
        }
        Ok(())
    }

    /// Whether page borders are present on the section properties.
    pub fn has_page_borders(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .map(|s| s.child("pgBorders").is_some())
            .unwrap_or(false))
    }

    /// Read page border color from the first border side, if present.
    pub fn page_border_color(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(borders) = sect.child("pgBorders") else {
            return Ok(None);
        };
        for side in ["top", "left", "bottom", "right"] {
            if let Some(b) = borders.child(side) {
                if let Some(c) = b
                    .get_attribute_qname("w:color")
                    .or_else(|| b.get_attribute("color"))
                {
                    return Ok(Some(c.to_string()));
                }
            }
        }
        Ok(None)
    }

    /// Set page border container attributes (`zOrder`, `display`, `offsetFrom`).
    ///
    /// `display` e.g. `"allPages"`, `"firstPage"`, `"notFirstPage"`.
    /// `offset_from` e.g. `"page"`, `"text"`.
    pub fn set_page_border_options(
        &mut self,
        display: Option<&str>,
        offset_from: Option<&str>,
        z_order: Option<&str>,
    ) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(crate::wordprocessing::section_properties());
        }
        let sect = body.child_mut("sectPr").unwrap();
        if sect.child("pgBorders").is_none() {
            use crate::wordprocessing::page_borders;
            sect.append_child(page_borders("auto", 4));
        }
        if let Some(pb) = sect.child_mut("pgBorders") {
            if let Some(d) = display {
                pb.set_attribute_qname("w:display", d);
            }
            if let Some(o) = offset_from {
                pb.set_attribute_qname("w:offsetFrom", o);
            }
            if let Some(z) = z_order {
                pb.set_attribute_qname("w:zOrder", z);
            }
        }
        Ok(())
    }

    /// Clear `w:pgBorders` display/offsetFrom/zOrder options (keeps border sides).
    pub fn clear_page_border_options(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let Some(pb) = sect.child_mut("pgBorders") else {
            return Ok(false);
        };
        let before = pb.attributes.len();
        pb.attributes.retain(|a| {
            !matches!(a.local_name.as_str(), "display" | "offsetFrom" | "zOrder")
        });
        Ok(pb.attributes.len() < before)
    }

    /// Read page border display option.
    pub fn page_border_display(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .and_then(|s| s.child("pgBorders"))
            .and_then(|pb| {
                pb.get_attribute_qname("w:display")
                    .or_else(|| pb.get_attribute("display"))
                    .map(|s| s.to_string())
            }))
    }

    /// Read page border offsetFrom option.
    pub fn page_border_offset_from(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .and_then(|s| s.child("pgBorders"))
            .and_then(|pb| {
                pb.get_attribute_qname("w:offsetFrom")
                    .or_else(|| pb.get_attribute("offsetFrom"))
                    .map(|s| s.to_string())
            }))
    }

    /// Remove page borders from section properties. Returns whether they were present.
    pub fn clear_page_borders(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "pgBorders");
        Ok(sect.children.len() < before)
    }

    /// Ensure the document body ends with a `w:sectPr` configured for page size/margins (twips).
    pub fn set_page_setup(
        &mut self,
        page_w: u32,
        page_h: u32,
        margin_top: u32,
        margin_right: u32,
        margin_bottom: u32,
        margin_left: u32,
    ) -> Result<()> {
        use crate::wordprocessing::section_properties_with_page;
        let body = self.body_mut()?;
        body.children.retain(|c| c.local_name != "sectPr");
        body.append_child(section_properties_with_page(
            page_w,
            page_h,
            margin_top,
            margin_right,
            margin_bottom,
            margin_left,
        ));
        Ok(())
    }

    /// Clear section page size and margins (`pgSz` / `pgMar`).
    pub fn clear_page_setup(&mut self) -> Result<bool> {
        let mut removed = false;
        if self.clear_page_size()? {
            removed = true;
        }
        if self.clear_page_margins()? {
            removed = true;
        }
        Ok(removed)
    }

    /// Read page size from `w:sectPr/w:pgSz` as `(w, h)` twips, if present.
    pub fn page_size(&mut self) -> Result<Option<(u32, u32)>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(sz) = sect.child("pgSz") else {
            return Ok(None);
        };
        let w = sz
            .get_attribute_qname("w:w")
            .or_else(|| sz.get_attribute("w"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let h = sz
            .get_attribute_qname("w:h")
            .or_else(|| sz.get_attribute("h"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        Ok(Some((w, h)))
    }

    /// Read page margins from `w:sectPr/w:pgMar` as `(top, right, bottom, left)` twips.
    pub fn page_margins(&mut self) -> Result<Option<(u32, u32, u32, u32)>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(m) = sect.child("pgMar") else {
            return Ok(None);
        };
        let parse = |names: &[&str]| {
            names
                .iter()
                .find_map(|n| {
                    m.get_attribute_qname(&format!("w:{n}"))
                        .or_else(|| m.get_attribute(n))
                })
                .and_then(|s| s.parse().ok())
                .unwrap_or(0)
        };
        Ok(Some((
            parse(&["top"]),
            parse(&["right"]),
            parse(&["bottom"]),
            parse(&["left"]),
        )))
    }

    /// Whether page size is set in `sectPr`.
    pub fn has_page_size(&mut self) -> Result<bool> {
        Ok(self.page_size()?.is_some())
    }

    /// Whether page margins are set in `sectPr`.
    pub fn has_page_margins(&mut self) -> Result<bool> {
        Ok(self.page_margins()?.is_some())
    }

    /// Set page size on `w:pgSz` in twips.
    pub fn set_page_size(&mut self, width: u32, height: u32) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        if sect.child("pgSz").is_none() {
            sect.append_child(OpenXmlElement::w("pgSz"));
        }
        if let Some(sz) = sect.child_mut("pgSz") {
            sz.set_attribute_qname("w:w", width.to_string());
            sz.set_attribute_qname("w:h", height.to_string());
        }
        Ok(())
    }

    /// Set page margins on `w:pgMar` as `(top, right, bottom, left)` twips.
    pub fn set_page_margins(
        &mut self,
        top: u32,
        right: u32,
        bottom: u32,
        left: u32,
    ) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        if sect.child("pgMar").is_none() {
            sect.append_child(OpenXmlElement::w("pgMar"));
        }
        if let Some(m) = sect.child_mut("pgMar") {
            m.set_attribute_qname("w:top", top.to_string());
            m.set_attribute_qname("w:right", right.to_string());
            m.set_attribute_qname("w:bottom", bottom.to_string());
            m.set_attribute_qname("w:left", left.to_string());
        }
        Ok(())
    }

    /// Clear `w:pgSz`. Returns whether present.
    pub fn clear_page_size(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "pgSz");
        Ok(sect.children.len() < before)
    }

    /// Clear `w:pgMar`. Returns whether present.
    pub fn clear_page_margins(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "pgMar");
        Ok(sect.children.len() < before)
    }

    /// Set page orientation on `w:pgSz` (`"portrait"` or `"landscape"`).
    pub fn set_page_orientation(&mut self, orient: &str) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        if sect.child("pgSz").is_none() {
            sect.append_child(
                OpenXmlElement::w("pgSz")
                    .with_attribute_qname("w:w", "12240")
                    .with_attribute_qname("w:h", "15840"),
            );
        }
        if let Some(sz) = sect.child_mut("pgSz") {
            sz.set_attribute_qname("w:orient", orient);
        }
        Ok(())
    }

    /// Read page orientation from `w:pgSz/@w:orient`.
    pub fn page_orientation(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(sz) = sect.child("pgSz") else {
            return Ok(None);
        };
        Ok(sz
            .get_attribute_qname("w:orient")
            .or_else(|| sz.get_attribute("orient"))
            .map(|s| s.to_string()))
    }

    /// Whether page orientation is set.
    pub fn has_page_orientation(&mut self) -> Result<bool> {
        Ok(self.page_orientation()?.is_some())
    }

    /// Clear `w:pgSz/@w:orient`.
    pub fn clear_page_orientation(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let Some(sz) = sect.child_mut("pgSz") else {
            return Ok(false);
        };
        let before = sz.attributes.len();
        sz.attributes.retain(|a| a.local_name != "orient");
        Ok(sz.attributes.len() < before)
    }

    /// Set multi-column layout on `w:cols` (`num` columns, optional space twips, equalWidth).
    pub fn set_columns(&mut self, num: u32, space_twips: Option<u32>, equal_width: bool) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "cols");
        let mut cols = OpenXmlElement::w("cols")
            .with_attribute_qname("w:num", num.to_string())
            .with_attribute_qname("w:equalWidth", if equal_width { "1" } else { "0" });
        if let Some(sp) = space_twips {
            cols = cols.with_attribute_qname("w:space", sp.to_string());
        }
        sect.append_child(cols);
        Ok(())
    }

    /// Read column count from `w:cols/@w:num`.
    pub fn column_count(&mut self) -> Result<Option<u32>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(cols) = sect.child("cols") else {
            return Ok(None);
        };
        Ok(cols
            .get_attribute_qname("w:num")
            .or_else(|| cols.get_attribute("num"))
            .and_then(|s| s.parse().ok()))
    }

    /// Whether multi-column layout is present.
    pub fn has_columns(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .map(|s| s.child("cols").is_some())
            .unwrap_or(false))
    }

    /// Clear `w:cols`. Returns whether present.
    pub fn clear_columns(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "cols");
        Ok(sect.children.len() < before)
    }

    /// Enable different first page header/footer (`w:titlePg`).
    pub fn set_title_page(&mut self, enabled: bool) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "titlePg");
        if enabled {
            sect.append_child(OpenXmlElement::w("titlePg"));
        }
        Ok(())
    }

    /// Whether titlePg is present.
    pub fn has_title_page(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .map(|s| s.child("titlePg").is_some())
            .unwrap_or(false))
    }

    /// Set vertical alignment of text on page (`w:vAlign w:val`), e.g. `"center"`, `"both"`, `"top"`, `"bottom"`.
    /// Disable `title page`. Returns whether it was enabled.
    pub fn clear_title_page(&mut self) -> Result<bool> {
        let had = self.has_title_page()?;
        if had {
            self.set_title_page(false)?;
        }
        Ok(had)
    }

    pub fn set_vertical_page_align(&mut self, val: &str) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "vAlign");
        sect.append_child(
            OpenXmlElement::w("vAlign").with_attribute_qname("w:val", val),
        );
        Ok(())
    }

    /// Read vertical page alignment.
    pub fn vertical_page_align(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(va) = sect.child("vAlign") else {
            return Ok(None);
        };
        Ok(va
            .get_attribute_qname("w:val")
            .or_else(|| va.get_attribute("val"))
            .map(|s| s.to_string()))
    }

    /// Clear vertical page alignment.
    pub fn clear_vertical_page_align(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "vAlign");
        Ok(sect.children.len() < before)
    }

    /// Set section type (`w:type w:val`), e.g. `"nextPage"`, `"continuous"`, `"oddPage"`, `"evenPage"`.
    pub fn set_section_type(&mut self, val: &str) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "type");
        sect.append_child(OpenXmlElement::w("type").with_attribute_qname("w:val", val));
        Ok(())
    }

    /// Read section type.
    pub fn section_type(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(t) = sect.child("type") else {
            return Ok(None);
        };
        Ok(t.get_attribute_qname("w:val")
            .or_else(|| t.get_attribute("val"))
            .map(|s| s.to_string()))
    }

    /// Whether section type is set.
    pub fn has_section_type(&mut self) -> Result<bool> {
        Ok(self.section_type()?.is_some())
    }

    /// Clear `w:type` under sectPr.
    pub fn clear_section_type(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "type");
        Ok(sect.children.len() < before)
    }

    /// Set page number start via `w:pgNumType w:start`.
    pub fn set_page_number_type_start(&mut self, start: u32) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        if let Some(pg) = sect.child_mut("pgNumType") {
            pg.set_attribute_qname("w:start", start.to_string());
        } else {
            sect.append_child(
                OpenXmlElement::w("pgNumType").with_attribute_qname("w:start", start.to_string()),
            );
        }
        Ok(())
    }

    /// Read `w:pgNumType/@w:start`.
    pub fn page_number_type_start(&mut self) -> Result<Option<u32>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(pg) = sect.child("pgNumType") else {
            return Ok(None);
        };
        Ok(pg
            .get_attribute_qname("w:start")
            .or_else(|| pg.get_attribute("start"))
            .and_then(|s| s.parse().ok()))
    }

    /// Whether page number start is set on pgNumType.
    pub fn has_page_number_type_start(&mut self) -> Result<bool> {
        Ok(self.page_number_type_start()?.is_some())
    }

    /// Clear `w:pgNumType/@w:start`.
    pub fn clear_page_number_type_start(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let Some(pg) = sect.child_mut("pgNumType") else {
            return Ok(false);
        };
        let before = pg.attributes.len();
        pg.attributes.retain(|a| a.local_name != "start");
        Ok(pg.attributes.len() < before)
    }

    /// Set section form protection (`w:formProt` presence).
    pub fn set_form_protection(&mut self, enabled: bool) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "formProt");
        if enabled {
            sect.append_child(OpenXmlElement::w("formProt"));
        }
        Ok(())
    }

    /// Whether section form protection is enabled.
    pub fn has_form_protection(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .map(|s| s.child("formProt").is_some())
            .unwrap_or(false))
    }

    /// Set bidi (right-to-left section) flag.
    /// Disable `form protection`. Returns whether it was enabled.
    pub fn clear_form_protection(&mut self) -> Result<bool> {
        let had = self.has_form_protection()?;
        if had {
            self.set_form_protection(false)?;
        }
        Ok(had)
    }

    pub fn set_section_bidi(&mut self, enabled: bool) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "bidi");
        if enabled {
            sect.append_child(OpenXmlElement::w("bidi"));
        }
        Ok(())
    }

    /// Whether section bidi is enabled.
    pub fn has_section_bidi(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .map(|s| s.child("bidi").is_some())
            .unwrap_or(false))
    }

    /// Set header/footer distances in twips on `w:pgMar` (`w:header`, `w:footer`).
    /// Disable `section bidi`. Returns whether it was enabled.
    pub fn clear_section_bidi(&mut self) -> Result<bool> {
        let had = self.has_section_bidi()?;
        if had {
            self.set_section_bidi(false)?;
        }
        Ok(had)
    }

    pub fn set_header_footer_distance(&mut self, header: u32, footer: u32) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        if sect.child("pgMar").is_none() {
            sect.append_child(
                OpenXmlElement::w("pgMar")
                    .with_attribute_qname("w:top", "1440")
                    .with_attribute_qname("w:right", "1440")
                    .with_attribute_qname("w:bottom", "1440")
                    .with_attribute_qname("w:left", "1440"),
            );
        }
        if let Some(m) = sect.child_mut("pgMar") {
            m.set_attribute_qname("w:header", header.to_string());
            m.set_attribute_qname("w:footer", footer.to_string());
        }
        Ok(())
    }

    /// Read header/footer distances as `(header, footer)` twips.
    pub fn header_footer_distance(&mut self) -> Result<Option<(u32, u32)>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(m) = sect.child("pgMar") else {
            return Ok(None);
        };
        let header = m
            .get_attribute_qname("w:header")
            .or_else(|| m.get_attribute("header"))
            .and_then(|s| s.parse().ok());
        let footer = m
            .get_attribute_qname("w:footer")
            .or_else(|| m.get_attribute("footer"))
            .and_then(|s| s.parse().ok());
        match (header, footer) {
            (Some(h), Some(f)) => Ok(Some((h, f))),
            (Some(h), None) => Ok(Some((h, 0))),
            (None, Some(f)) => Ok(Some((0, f))),
            _ => Ok(None),
        }
    }

    /// Whether header or footer distance is set on `w:pgMar`.
    pub fn has_header_footer_distance(&mut self) -> Result<bool> {
        Ok(self.header_footer_distance()?.is_some())
    }

    /// Clear `w:header` / `w:footer` on `w:pgMar`.
    pub fn clear_header_footer_distance(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let Some(m) = sect.child_mut("pgMar") else {
            return Ok(false);
        };
        let before = m.attributes.len();
        m.attributes.retain(|a| a.local_name != "header" && a.local_name != "footer");
        Ok(m.attributes.len() < before)
    }

    /// Set line numbering on the section (`w:lnNumType`).
    ///
    /// `count_by` is the increment, `start` the starting number, `restart` e.g. `"newPage"`, `"newSection"`, `"continuous"`.
    pub fn set_line_numbering(
        &mut self,
        count_by: u32,
        start: u32,
        restart: &str,
    ) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "lnNumType");
        sect.append_child(
            OpenXmlElement::w("lnNumType")
                .with_attribute_qname("w:countBy", count_by.to_string())
                .with_attribute_qname("w:start", start.to_string())
                .with_attribute_qname("w:restart", restart),
        );
        Ok(())
    }

    /// Read line numbering as `(count_by, start, restart)`.
    pub fn line_numbering(&mut self) -> Result<Option<(u32, u32, String)>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(ln) = sect.child("lnNumType") else {
            return Ok(None);
        };
        let count_by = ln
            .get_attribute_qname("w:countBy")
            .or_else(|| ln.get_attribute("countBy"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let start = ln
            .get_attribute_qname("w:start")
            .or_else(|| ln.get_attribute("start"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let restart = ln
            .get_attribute_qname("w:restart")
            .or_else(|| ln.get_attribute("restart"))
            .unwrap_or("newPage")
            .to_string();
        Ok(Some((count_by, start, restart)))
    }

    /// Whether line numbering is present.
    pub fn has_line_numbering(&mut self) -> Result<bool> {
        Ok(self.line_numbering()?.is_some())
    }

    /// Clear line numbering. Returns whether present.
    pub fn clear_line_numbering(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "lnNumType");
        Ok(sect.children.len() < before)
    }

    /// Set page number format on `w:pgNumType` (`w:fmt`), e.g. `"decimal"`, `"upperRoman"`, `"lowerLetter"`.
    pub fn set_page_number_format(&mut self, fmt: &str) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        if let Some(pg) = sect.child_mut("pgNumType") {
            pg.set_attribute_qname("w:fmt", fmt);
        } else {
            sect.append_child(
                OpenXmlElement::w("pgNumType").with_attribute_qname("w:fmt", fmt),
            );
        }
        Ok(())
    }

    /// Read page number format.
    pub fn page_number_format(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(pg) = sect.child("pgNumType") else {
            return Ok(None);
        };
        Ok(pg
            .get_attribute_qname("w:fmt")
            .or_else(|| pg.get_attribute("fmt"))
            .map(|s| s.to_string()))
    }

    /// Whether page number format is set.
    pub fn has_page_number_format(&mut self) -> Result<bool> {
        Ok(self.page_number_format()?.is_some())
    }

    /// Clear `w:pgNumType/@w:fmt`.
    pub fn clear_page_number_format(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let Some(pg) = sect.child_mut("pgNumType") else {
            return Ok(false);
        };
        let before = pg.attributes.len();
        pg.attributes.retain(|a| a.local_name != "fmt");
        Ok(pg.attributes.len() < before)
    }

    /// Set text direction on the section (`w:textDirection w:val`), e.g. `"lrTb"`, `"tbRl"`, `"btLr"`.
    pub fn set_text_direction(&mut self, val: &str) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "textDirection");
        sect.append_child(
            OpenXmlElement::w("textDirection").with_attribute_qname("w:val", val),
        );
        Ok(())
    }

    /// Read text direction.
    pub fn text_direction(&mut self) -> Result<Option<String>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(td) = sect.child("textDirection") else {
            return Ok(None);
        };
        Ok(td
            .get_attribute_qname("w:val")
            .or_else(|| td.get_attribute("val"))
            .map(|s| s.to_string()))
    }

    /// Clear text direction. Returns whether present.
    pub fn clear_text_direction(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "textDirection");
        Ok(sect.children.len() < before)
    }

    /// Set gutter margin in twips on `w:pgMar/@w:gutter`.
    pub fn set_gutter(&mut self, twips: u32) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        if sect.child("pgMar").is_none() {
            sect.append_child(
                OpenXmlElement::w("pgMar")
                    .with_attribute_qname("w:top", "1440")
                    .with_attribute_qname("w:right", "1440")
                    .with_attribute_qname("w:bottom", "1440")
                    .with_attribute_qname("w:left", "1440"),
            );
        }
        if let Some(m) = sect.child_mut("pgMar") {
            m.set_attribute_qname("w:gutter", twips.to_string());
        }
        Ok(())
    }

    /// Read gutter margin in twips.
    pub fn gutter(&mut self) -> Result<Option<u32>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(m) = sect.child("pgMar") else {
            return Ok(None);
        };
        Ok(m.get_attribute_qname("w:gutter")
            .or_else(|| m.get_attribute("gutter"))
            .and_then(|s| s.parse().ok()))
    }

    /// Whether gutter is set on `w:pgMar`.
    pub fn has_gutter(&mut self) -> Result<bool> {
        Ok(self.gutter()?.is_some())
    }

    /// Clear `w:gutter` on `w:pgMar`.
    pub fn clear_gutter(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let Some(m) = sect.child_mut("pgMar") else {
            return Ok(false);
        };
        let before = m.attributes.len();
        m.attributes.retain(|a| a.local_name != "gutter");
        Ok(m.attributes.len() < before)
    }

    /// Set paper source (`w:paperSrc` with `w:first` / `w:other` tray codes).
    pub fn set_paper_source(&mut self, first: u32, other: u32) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "paperSrc");
        sect.append_child(
            OpenXmlElement::w("paperSrc")
                .with_attribute_qname("w:first", first.to_string())
                .with_attribute_qname("w:other", other.to_string()),
        );
        Ok(())
    }

    /// Read paper source as `(first, other)`.
    pub fn paper_source(&mut self) -> Result<Option<(u32, u32)>> {
        let body = self.body_mut()?;
        let Some(sect) = body.child("sectPr") else {
            return Ok(None);
        };
        let Some(ps) = sect.child("paperSrc") else {
            return Ok(None);
        };
        let first = ps
            .get_attribute_qname("w:first")
            .or_else(|| ps.get_attribute("first"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let other = ps
            .get_attribute_qname("w:other")
            .or_else(|| ps.get_attribute("other"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        Ok(Some((first, other)))
    }

    /// Clear paper source. Returns whether present.
    pub fn clear_paper_source(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        let Some(sect) = body.child_mut("sectPr") else {
            return Ok(false);
        };
        let before = sect.children.len();
        sect.children.retain(|c| c.local_name != "paperSrc");
        Ok(sect.children.len() < before)
    }

    /// Set `w:rtlGutter` presence.
    pub fn set_rtl_gutter(&mut self, enabled: bool) -> Result<()> {
        let body = self.body_mut()?;
        if body.child("sectPr").is_none() {
            body.append_child(OpenXmlElement::w("sectPr"));
        }
        let sect = body.child_mut("sectPr").unwrap();
        sect.children.retain(|c| c.local_name != "rtlGutter");
        if enabled {
            sect.append_child(OpenXmlElement::w("rtlGutter"));
        }
        Ok(())
    }

    /// Whether rtlGutter is present.
    pub fn has_rtl_gutter(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        Ok(body
            .child("sectPr")
            .map(|s| s.child("rtlGutter").is_some())
            .unwrap_or(false))
    }

    /// Accept all tracked insertions/deletions in the main document body.
    ///
    /// Returns the number of revision markers processed.
    /// Disable `rtl gutter`. Returns whether it was enabled.
    pub fn clear_rtl_gutter(&mut self) -> Result<bool> {
        let had = self.has_rtl_gutter()?;
        if had {
            self.set_rtl_gutter(false)?;
        }
        Ok(had)
    }

    pub fn accept_all_revisions(&mut self) -> Result<usize> {
        let body = self.body_mut()?;
        Ok(accept_revisions(body))
    }

    /// Reject all tracked insertions/deletions in the main document body.
    pub fn reject_all_revisions(&mut self) -> Result<usize> {
        let body = self.body_mut()?;
        Ok(reject_revisions(body))
    }

    /// Accept tracked changes in all header and footer parts.
    ///
    /// Returns the number of revision markers processed across those parts.
    pub fn accept_all_revisions_in_headers_footers(&mut self) -> Result<usize> {
        let mut uris: Vec<PackUri> = self.list_headers();
        uris.extend(self.list_footers());
        let mut total = 0usize;
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri).map(|d| d.to_vec()) else {
                continue;
            };
            let Ok(mut root) = parse_element(&data) else {
                continue;
            };
            let n = accept_revisions(&mut root);
            if n > 0 {
                total += n;
                let ct = self
                    .package
                    .opc()
                    .content_types()
                    .overrides
                    .get(uri.as_str())
                    .cloned()
                    .unwrap_or_else(|| {
                        if uri.as_str().contains("header") {
                            content_type::WORD_HEADER.to_string()
                        } else {
                            content_type::WORD_FOOTER.to_string()
                        }
                    });
                let xml = crate::element::write_element(&root)?;
                self.package.set_part(uri, ct, xml);
            }
        }
        Ok(total)
    }

    /// Reject tracked changes in all header and footer parts.
    pub fn reject_all_revisions_in_headers_footers(&mut self) -> Result<usize> {
        let mut uris: Vec<PackUri> = self.list_headers();
        uris.extend(self.list_footers());
        let mut total = 0usize;
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri).map(|d| d.to_vec()) else {
                continue;
            };
            let Ok(mut root) = parse_element(&data) else {
                continue;
            };
            let n = reject_revisions(&mut root);
            if n > 0 {
                total += n;
                let ct = self
                    .package
                    .opc()
                    .content_types()
                    .overrides
                    .get(uri.as_str())
                    .cloned()
                    .unwrap_or_else(|| {
                        if uri.as_str().contains("header") {
                            content_type::WORD_HEADER.to_string()
                        } else {
                            content_type::WORD_FOOTER.to_string()
                        }
                    });
                let xml = crate::element::write_element(&root)?;
                self.package.set_part(uri, ct, xml);
            }
        }
        Ok(total)
    }

    /// Accept revisions in body, headers, and footers. Returns total markers processed.
    pub fn accept_all_revisions_everywhere(&mut self) -> Result<usize> {
        let a = self.accept_all_revisions()?;
        let b = self.accept_all_revisions_in_headers_footers()?;
        Ok(a + b)
    }

    /// Reject revisions in body, headers, and footers. Returns total markers processed.
    pub fn reject_all_revisions_everywhere(&mut self) -> Result<usize> {
        let a = self.reject_all_revisions()?;
        let b = self.reject_all_revisions_in_headers_footers()?;
        Ok(a + b)
    }

    /// Count tracked revision markers (`w:ins` / `w:del`) in the main document body.
    pub fn revision_marker_count(&mut self) -> Result<usize> {
        let body = self.body_mut()?;
        Ok(body
            .descendants()
            .filter(|e| e.local_name == "ins" || e.local_name == "del")
            .count())
    }

    /// Whether the body contains any tracked revision markers.
    pub fn has_revision_markers(&mut self) -> Result<bool> {
        Ok(self.revision_marker_count()? > 0)
    }

    /// List tracked revision markers as `(kind, author_or_empty, date_or_empty, text)`.
    ///
    /// `kind` is `"ins"` or `"del"`.
    pub fn list_revision_markers(&mut self) -> Result<Vec<(String, String, String, String)>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for e in body.descendants() {
            if e.local_name != "ins" && e.local_name != "del" {
                continue;
            }
            let author = e
                .get_attribute_qname("w:author")
                .or_else(|| e.get_attribute("author"))
                .unwrap_or("")
                .to_string();
            let date = e
                .get_attribute_qname("w:date")
                .or_else(|| e.get_attribute("date"))
                .unwrap_or("")
                .to_string();
            out.push((e.local_name.clone(), author, date, e.inner_text()));
        }
        Ok(out)
    }

    /// Count tracked insertions (`w:ins`) in the body.
    pub fn insertion_count(&mut self) -> Result<usize> {
        Ok(self
            .list_revision_markers()?
            .into_iter()
            .filter(|(k, _, _, _)| k == "ins")
            .count())
    }

    /// Count tracked deletions (`w:del`) in the body.
    pub fn deletion_count(&mut self) -> Result<usize> {
        Ok(self
            .list_revision_markers()?
            .into_iter()
            .filter(|(k, _, _, _)| k == "del")
            .count())
    }

    /// Validate the main document DOM against lightweight WordprocessingML rules.
    pub fn validate(&mut self) -> Result<Vec<ValidationError>> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(validate_word_document(doc))
    }

    /// Validate using lightweight rules **and** ordered particle matching.
    pub fn validate_full(&mut self) -> Result<Vec<ValidationError>> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(validate_word_document_full(doc))
    }

    /// Validate OPC package structure (main part + relationship targets).
    pub fn validate_package(&self) -> Result<Vec<ValidationError>> {
        Ok(crate::validation::validate_package(
            self.package.opc(),
            true,
        ))
    }

    /// Validate part relationship constraints (C# `PackageValidator`).
    pub fn validate_package_constraints(&self) -> Result<Vec<ValidationError>> {
        Ok(crate::validation::validate_package_constraints(
            self.package.opc(),
        ))
    }

    /// Validate relationship-id attributes in the main document against part relationships.
    ///
    /// Uses hand-curated Word rules merged with the extractable Schematron subset.
    pub fn validate_relationships(&mut self) -> Result<Vec<ValidationError>> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let uri = main.part().uri.clone();
        let doc = main.document(package)?.clone();
        let rel_rules =
            crate::validation::merged_relationship_rules(crate::validation::word_relationship_rules());
        let unique_rules = crate::validation::merged_unique_attribute_rules(
            crate::validation::word_unique_attribute_rules(),
        );
        Ok(crate::validation::validate_semantic(
            package.opc(),
            &uri,
            &doc,
            &rel_rules,
            &unique_rules,
        ))
    }

    /// Validate the main document with the full extractable Schematron subset.
    pub fn validate_schematron(&mut self) -> Result<Vec<ValidationError>> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let uri = main.part().uri.clone();
        let doc = main.document(package)?.clone();
        Ok(crate::validation::validate_schematron_subset(
            package.opc(),
            &uri,
            &doc,
        ))
    }


    /// Validate Schematron attribute constraints on the main document part root.
    pub fn validate_schematron_attributes(
        &self,
    ) -> Result<Vec<crate::validation::ValidationError>> {
        let main_uri = match self.package.opc().main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let Some(data) = self.package.opc().get_part(&main_uri) else {
            return Ok(Vec::new());
        };
        let root = crate::element::parse_element(data)?;
        Ok(crate::validation::validate_schematron_attributes(&root))
    }

    /// Remove a part from the package (content-type override, child rels, inbound rels).
    pub fn delete_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.package.delete_part(uri)
    }

    /// Alias for [`delete_part`](Self::delete_part).
    pub fn remove_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.delete_part(uri)
    }

    /// Delete a part and cascade to parts that become unreachable (C# DeletePart orphan cascade).
    pub fn delete_part_and_orphans(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.package.delete_part_and_orphans(uri)
    }

    /// Delete the part identified by relationship id on the main document (or package if no main).
    pub fn delete_part_by_id(&mut self, id: &str) -> bool {
        let source = self
            .main_document_part
            .as_ref()
            .map(|m| m.uri().clone());
        self.package
            .delete_part_by_id(source.as_ref(), id)
    }

    /// Delete every part with the given content type, cascading orphans
    /// (approximate C# `DeletePartsRecursivelyOfType<T>`).
    /// Delete multiple parts by URI (C# `DeleteParts`).
    pub fn delete_parts(&mut self, uris: &[PackUri]) -> usize {
        self.package.delete_parts(uris)
    }

    /// C# `StrictRelationshipFound`.
    pub fn strict_relationship_found(&self) -> bool {
        self.package.strict_relationship_found()
    }

    pub fn delete_parts_of_content_type(&mut self, content_type: &str) -> usize {
        self.package
            .delete_parts_of_content_type(content_type)
    }

    /// Recursively delete parts of a relationship type from the package root
    /// (C# `DeletePartsRecursivelyOfType` stand-in by relationship URI).
    pub fn delete_parts_recursively_of_relationship_type(
        &mut self,
        relationship_type: &str,
    ) -> usize {
        self.package
            .delete_parts_recursively_of_relationship_type(relationship_type)
    }

    /// Add an external relationship from the main document part.
    pub fn add_external_relationship(
        &mut self,
        relationship_type: &str,
        external_uri: &str,
    ) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let uri = main.uri().clone();
        Ok(self
            .package
            .add_external_relationship(Some(&uri), relationship_type, external_uri))
    }

    /// External relationships on the main document part.
    pub fn external_relationships(&self) -> Vec<&crate::opc::Relationship> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        self.package
            .opc()
            .external_relationships(Some(main.uri()))
    }

    /// Ensure a [`PackageEvents`](crate::features::PackageEvents) feature exists and return it.
    pub fn package_events(&mut self) -> &crate::features::PackageEvents {
        self.package.package_events()
    }

    /// Part-container events (C# `IPartEventsFeature`).
    pub fn part_events(&mut self) -> &crate::features::PartEvents {
        self.package.part_events()
    }

    /// Child parts related from the main document (C# `MainDocumentPart` children / `GetPartsOfType`).
    pub fn related_parts(
        &self,
        relationship_type: Option<&str>,
    ) -> Vec<crate::opc::RelatedPart> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        self.package
            .opc()
            .related_parts(Some(main.uri()), relationship_type)
    }

    /// Allocate a unique part URI under the main document using [`PartUriHelper`](crate::opc::PartUriHelper).
    pub fn create_unique_part_uri(
        &self,
        content_type: &str,
        target_path: &str,
        target_name: &str,
        target_ext: &str,
    ) -> Result<PackUri> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        self.package.opc().create_unique_part_uri(
            content_type,
            main.uri(),
            target_path,
            target_name,
            target_ext,
        )
    }

    /// Hyperlink relationships on the main document (C# `HyperlinkRelationships`).
    pub fn hyperlink_relationships(&self) -> Vec<crate::opc::HyperlinkRelationship> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        self.package
            .hyperlink_relationships(Some(main.uri()))
    }

    /// Relationship id of a part under the main document (C# `GetIdOfPart`).
    pub fn get_id_of_part(&self, part_uri: &PackUri) -> Option<String> {
        let main = self.main_document_part.as_ref()?;
        self.package
            .get_id_of_part(Some(main.uri()), part_uri)
    }

    /// Part URI for relationship id on the main document (C# `GetPartById`).
    pub fn get_part_by_id(&self, id: &str) -> Option<PackUri> {
        let main = self.main_document_part.as_ref()?;
        self.package.get_part_by_id(Some(main.uri()), id)
    }

    /// Change the relationship id of a child part (C# `ChangeIdOfPart`).
    pub fn change_id_of_part(&mut self, part_uri: &PackUri, new_id: &str) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?
            .uri()
            .clone();
        self.package
            .change_id_of_part(Some(&main), part_uri, new_id)
    }

    /// Child parts as IdPartPair under the main document (C# `Parts`).
    pub fn id_part_pairs(&self) -> Vec<crate::opc::IdPartPair> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        self.package.id_part_pairs(Some(main.uri()))
    }

    /// Create a media data part in the package (C# `CreateMediaDataPart`).
    pub fn create_media_data_part(
        &mut self,
        content_type: &str,
        extension: Option<&str>,
    ) -> Result<crate::opc::DataPart> {
        self.package
            .create_media_data_part(content_type, extension)
    }

    /// Delete a package data part if unreferenced (C# `DeletePart(DataPart)`).
    pub fn delete_data_part(&mut self, uri: &PackUri) -> Result<bool> {
        self.package.delete_data_part(uri)
    }



    /// Add a data-part reference from the main document (C# `AddDataPartReferenceRelationship`).
    pub fn add_data_part_reference_relationship(
        &mut self,
        data_part: &crate::opc::DataPart,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<crate::opc::DataPartReferenceRelationship> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?
            .uri()
            .clone();
        self.package.add_data_part_reference_relationship(
            &main,
            data_part,
            relationship_type,
            id,
        )
    }

    /// Data-part references on the main document.
    pub fn data_part_reference_relationships(
        &self,
    ) -> Vec<crate::opc::DataPartReferenceRelationship> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        self.package
            .data_part_reference_relationships(Some(main.uri()))
    }

    /// Delete a reference relationship by id on the main document
    /// (C# `DeleteReferenceRelationship`).
    pub fn delete_reference_relationship(&mut self, id: &str) -> Option<crate::opc::Relationship> {
        let main = self.main_document_part.as_ref()?.uri().clone();
        self.package
            .delete_reference_relationship(Some(&main), id)
    }

    /// Get a reference relationship by id on the main document.
    pub fn get_reference_relationship(&self, id: &str) -> Option<crate::opc::ReferenceRelationship> {
        let main = self.main_document_part.as_ref()?.uri();
        self.package
            .get_reference_relationship(Some(main), id)
    }



    /// Add an arbitrary extended part related from the main document.
    ///
    /// Corresponds to C# `ExtendedPart`. Returns `(relationship_id, uri)`.
    pub fn add_extended_part(
        &mut self,
        uri: &str,
        content_type_str: &str,
        relationship_type: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let part_uri = PackUri::new(if uri.starts_with('/') {
            uri.to_string()
        } else {
            format!("/{uri}")
        });
        self.package
            .opc_mut()
            .set_part(part_uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &main_uri,
            relationship_type,
            &part_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, part_uri))
    }

    /// Create an [`ExtendedPart`] under `udata/` with auto URI (C# `ExtendedPart` defaults).
    ///
    /// Returns `(relationship_id, ExtendedPart handle)`.
    /// Copy a part (and optionally its relationship subgraph) from another package
    /// into this document's package (C# cross-package `AddPart` shell).
    pub fn copy_part_from_package(
        &mut self,
        source: &WordprocessingDocument,
        source_uri: &PackUri,
        dest_uri: &PackUri,
        opts: crate::opc::CopyPartOptions,
    ) -> Result<std::collections::HashMap<PackUri, PackUri>> {
        self.package
            .copy_part_from(source.package(), source_uri, dest_uri, opts)
    }

    /// Create a relationship from the main document part to an existing part
    /// in this package (C# `CreateRelationshipToPart` same-package).
    pub fn create_relationship_to_part(
        &mut self,
        target: &PackUri,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<String> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?
            .uri()
            .clone();
        self.package
            .create_relationship_to_part(&main, target, relationship_type, id)
    }

    /// Add a new typed child part under the main document via generated PartInfo
    /// (C# `AddNewPart<T>` shell).
    pub fn add_typed_child_part(
        &mut self,
        part_name: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<crate::packaging::TypedPart> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?
            .uri()
            .clone();
        crate::packaging::add_typed_part(
            &mut self.package,
            &main,
            Some("MainDocumentPart"),
            part_name,
            data,
        )
    }

    pub fn create_extended_part(
        &mut self,
        content_type_str: &str,
        relationship_type: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, crate::packaging::ExtendedPart)> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?
            .uri()
            .clone();
        let mut index = 1u32;
        let part_uri = loop {
            let candidate = PackUri::new(format!("/word/udata/data{index}.dat"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package
            .opc_mut()
            .set_part(part_uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &main,
            relationship_type,
            &part_uri,
            RelationshipTargetMode::Internal,
        );
        let part = crate::packaging::ExtendedPart::new(
            part_uri,
            content_type_str,
            relationship_type,
        );
        Ok((rid, part))
    }

    fn ensure_sect_pr_reference(&mut self, reference: OpenXmlElement) -> Result<()> {
        // Load body, find or create trailing sectPr, append the reference.
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let body = doc
            .child_mut("body")
            .ok_or_else(|| Error::Package("document has no body".into()))?;

        if let Some(sect) = body.child_mut("sectPr") {
            sect.append_child(reference);
        } else {
            let mut sect = OpenXmlElement::w("sectPr");
            sect.append_child(reference);
            body.append_child(sect);
        }
        Ok(())
    }

    /// Flush dirty parts into the OPC package and save to disk.
    pub fn save(&mut self) -> Result<()> {
        self.flush_parts()?;
        self.package.save()
    }

    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.flush_parts()?;
        self.package.save_as(path)
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>> {
        self.flush_parts()?;
        self.package.to_bytes()
    }

    fn flush_parts(&mut self) -> Result<()> {
        self.package.ensure_open()?;
        if let Some(main) = &mut self.main_document_part {
            main.save_to_package(&mut self.package)?;
        }
        Ok(())
    }

    /// Close the document, saving if `auto_save` is enabled.
    pub fn close(mut self) -> Result<()> {
        if self.package.auto_save()
            && matches!(
                self.package.opc().mode(),
                PackageMode::Create | PackageMode::ReadWrite
            )
            && self.package.path().is_some()
        {
            self.save()?;
        }
        self.package.mark_closed();
        Ok(())
    }

    /// Read all paragraph texts from the main document (including nested tables).
    pub fn paragraph_texts(&mut self) -> Result<Vec<String>> {
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        let body = doc
            .child("body")
            .ok_or_else(|| Error::Package("document has no body".into()))?;
        // `descendants()` walks all nested children, so table cell paragraphs are included.
        Ok(body
            .descendants()
            .filter(|e| e.local_name == "p")
            .map(|p| p.inner_text())
            .collect())
    }

    /// Count body-level paragraphs (does not count nested table paragraphs).
    pub fn paragraph_count(&mut self) -> Result<usize> {
        let body = self.body_mut()?;
        Ok(body.children.iter().filter(|c| c.local_name == "p").count())
    }

    /// Approximate word count across all paragraph texts (whitespace-split).
    pub fn word_count(&mut self) -> Result<usize> {
        let texts = self.paragraph_texts()?;
        Ok(texts
            .iter()
            .flat_map(|t| t.split_whitespace())
            .filter(|w| !w.is_empty())
            .count())
    }

    /// Collect document bookmarks as `(id, name)` pairs.
    pub fn bookmarks(&mut self) -> Result<Vec<(String, String)>> {
        use crate::wordprocessing::collect_bookmarks;
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(collect_bookmarks(doc))
    }

    /// Remove bookmark start/end markers with the given name. Returns how many start markers were removed.
    pub fn remove_bookmark(&mut self, name: &str) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut ids = Vec::new();
        // Collect ids for matching starts
        for e in doc.descendants() {
            if e.local_name == "bookmarkStart" {
                let n = e
                    .get_attribute_qname("w:name")
                    .or_else(|| e.get_attribute("name"));
                if n == Some(name) {
                    if let Some(id) = e
                        .get_attribute_qname("w:id")
                        .or_else(|| e.get_attribute("id"))
                    {
                        ids.push(id.to_string());
                    }
                }
            }
        }
        if ids.is_empty() {
            return Ok(0);
        }
        let count = ids.len();
        fn strip(el: &mut OpenXmlElement, ids: &[String]) {
            el.children.retain(|c| {
                if c.local_name == "bookmarkStart" || c.local_name == "bookmarkEnd" {
                    let id = c
                        .get_attribute_qname("w:id")
                        .or_else(|| c.get_attribute("id"))
                        .unwrap_or("");
                    !ids.iter().any(|x| x == id)
                } else {
                    true
                }
            });
            for child in &mut el.children {
                strip(child, ids);
            }
        }
        strip(doc, &ids);
        Ok(count)
    }

    /// Rename a bookmark (`w:bookmarkStart/@w:name`). Returns whether found.
    /// Remove all bookmarks from the main document. Returns how many bookmark pairs were removed.
    pub fn clear_bookmarks(&mut self) -> Result<usize> {
        let names = self.list_bookmark_names()?;
        let mut n = 0usize;
        for name in names {
            n += self.remove_bookmark(&name)?;
        }
        Ok(n)
    }

    pub fn rename_bookmark(&mut self, old_name: &str, new_name: &str) -> Result<bool> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, old: &str, new: &str, found: &mut bool) {
            if el.local_name == "bookmarkStart" {
                let n = el
                    .get_attribute_qname("w:name")
                    .or_else(|| el.get_attribute("name"));
                if n == Some(old) {
                    el.set_attribute_qname("w:name", new);
                    *found = true;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, old, new, found);
            }
        }
        visit(doc, old_name, new_name, &mut found);
        Ok(found)
    }

    /// Insert a paragraph at a body-level index among `w:p` children.
    ///
    /// Index is among paragraphs only (not tables). If `index >= paragraph_count`,
    /// the paragraph is appended (before trailing `sectPr`).
    pub fn insert_paragraph_at(
        &mut self,
        index: usize,
        para: OpenXmlElement,
    ) -> Result<()> {
        let body = self.body_mut()?;
        let para_positions: Vec<usize> = body
            .children
            .iter()
            .enumerate()
            .filter(|(_, c)| c.local_name == "p")
            .map(|(i, _)| i)
            .collect();
        if let Some(&pos) = para_positions.get(index) {
            body.children.insert(pos, para);
        } else if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(())
    }

    /// Access the document body element (mutable).
    pub fn body_mut(&mut self) -> Result<&mut OpenXmlElement> {
        if self.main_document_part.is_none() {
            return Err(Error::Package("no main document part".into()));
        }
        // Ensure the root is loaded before taking a long-lived mutable borrow.
        {
            let package = &self.package;
            let main = self.main_document_part.as_mut().unwrap();
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        doc.child_mut("body")
            .ok_or_else(|| Error::Package("document has no body".into()))
    }

    /// Apply a paragraph style (`w:pStyle`) to every body-level paragraph.
    ///
    /// Use [`crate::wordprocessing::paragraph_with_style`] when building content,
    /// or this method to stamp a style onto an existing document.
    pub fn apply_style_to_paragraphs(&mut self, style_id: &str) -> Result<usize> {
        use crate::wordprocessing::apply_paragraph_style;
        let body = self.body_mut()?;
        let mut count = 0usize;
        for child in &mut body.children {
            if child.local_name == "p" {
                apply_paragraph_style(child, style_id);
                count += 1;
            }
        }
        Ok(count)
    }

    /// Apply numbering (`w:numPr`) to every body-level paragraph.
    ///
    /// Sets `numId` and `ilvl` on each paragraph's `pPr`. Returns the number of paragraphs updated.
    pub fn apply_numbering_to_paragraphs(&mut self, num_id: u32, ilvl: u32) -> Result<usize> {
        use crate::wordprocessing::numbered_paragraph_properties;
        let body = self.body_mut()?;
        let mut count = 0usize;
        for child in &mut body.children {
            if child.local_name != "p" {
                continue;
            }
            // Ensure pPr, replace/insert numPr
            let ppr = if let Some(ppr) = child.child_mut("pPr") {
                ppr
            } else {
                child.children.insert(0, OpenXmlElement::w("pPr"));
                child.child_mut("pPr").unwrap()
            };
            ppr.children.retain(|c| c.local_name != "numPr");
            // Steal numPr child from a temporary numbered_paragraph_properties
            let tmp = numbered_paragraph_properties(num_id, ilvl);
            if let Some(num_pr) = tmp.children.into_iter().find(|c| c.local_name == "numPr") {
                ppr.append_child(num_pr);
            }
            count += 1;
        }
        Ok(count)
    }

    /// Clear numbering (`w:numPr`) from every body-level paragraph. Returns count cleared.
    pub fn clear_paragraph_numbering(&mut self) -> Result<usize> {
        let body = self.body_mut()?;
        let mut count = 0usize;
        for child in &mut body.children {
            if child.local_name != "p" {
                continue;
            }
            if let Some(ppr) = child.child_mut("pPr") {
                let before = ppr.children.len();
                ppr.children.retain(|c| c.local_name != "numPr");
                if ppr.children.len() < before {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// Set numbering on the body paragraph at 0-based index among `w:p` children.
    pub fn set_paragraph_numbering_at(
        &mut self,
        index: usize,
        num_id: u32,
        ilvl: u32,
    ) -> Result<bool> {
        use crate::wordprocessing::numbered_paragraph_properties;
        let body = self.body_mut()?;
        let mut p_i = 0usize;
        for child in &mut body.children {
            if child.local_name != "p" {
                continue;
            }
            if p_i == index {
                let ppr = if let Some(ppr) = child.child_mut("pPr") {
                    ppr
                } else {
                    child.children.insert(0, OpenXmlElement::w("pPr"));
                    child.child_mut("pPr").unwrap()
                };
                ppr.children.retain(|c| c.local_name != "numPr");
                let tmp = numbered_paragraph_properties(num_id, ilvl);
                if let Some(num_pr) = tmp.children.into_iter().find(|c| c.local_name == "numPr") {
                    ppr.append_child(num_pr);
                }
                return Ok(true);
            }
            p_i += 1;
        }
        Ok(false)
    }

    /// Clear numbering properties from the paragraph at `index`.
    pub fn clear_paragraph_numbering_at(&mut self, index: usize) -> Result<bool> {
        let body = self.body_mut()?;
        let mut p_i = 0usize;
        for child in &mut body.children {
            if child.local_name != "p" {
                continue;
            }
            if p_i == index {
                if let Some(ppr) = child.child_mut("pPr") {
                    let before = ppr.children.len();
                    ppr.children.retain(|c| c.local_name != "numPr");
                    return Ok(ppr.children.len() < before);
                }
                return Ok(false);
            }
            p_i += 1;
        }
        Ok(false)
    }

    /// List `(numId, ilvl)` for body paragraphs that have numbering, in order.
    pub fn list_paragraph_numbering(&mut self) -> Result<Vec<(u32, u32)>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for child in &body.children {
            if child.local_name != "p" {
                continue;
            }
            let Some(ppr) = child.child("pPr") else { continue };
            let Some(num_pr) = ppr.child("numPr") else { continue };
            let num_id = num_pr
                .child("numId")
                .and_then(|c| {
                    c.get_attribute_qname("w:val")
                        .or_else(|| c.get_attribute("val"))
                })
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let ilvl = num_pr
                .child("ilvl")
                .and_then(|c| {
                    c.get_attribute_qname("w:val")
                        .or_else(|| c.get_attribute("val"))
                })
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            out.push((num_id, ilvl));
        }
        Ok(out)
    }

    /// Whether any body paragraph has numbering properties.
    pub fn has_paragraph_numbering(&mut self) -> Result<bool> {
        Ok(!self.list_paragraph_numbering()?.is_empty())
    }

    /// Collect paragraph style ids currently referenced by body paragraphs (`w:pStyle`).
    pub fn paragraph_style_ids(&mut self) -> Result<Vec<String>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for child in &body.children {
            if child.local_name != "p" {
                continue;
            }
            if let Some(id) = child
                .child("pPr")
                .and_then(|ppr| ppr.child("pStyle"))
                .and_then(|ps| {
                    ps.get_attribute_qname("w:val")
                        .or_else(|| ps.get_attribute("val"))
                })
            {
                if !out.iter().any(|s| s == id) {
                    out.push(id.to_string());
                }
            }
        }
        Ok(out)
    }

    /// Apply a run style (`w:rStyle`) to every run in body-level paragraphs.
    ///
    /// Returns the number of runs updated.
    pub fn apply_run_style_to_runs(&mut self, style_id: &str) -> Result<usize> {
        let body = self.body_mut()?;
        let mut count = 0usize;
        fn visit(el: &mut OpenXmlElement, style_id: &str, count: &mut usize) {
            if el.local_name == "r" {
                let rpr = if let Some(rpr) = el.child_mut("rPr") {
                    rpr
                } else {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                    el.child_mut("rPr").unwrap()
                };
                rpr.children.retain(|c| c.local_name != "rStyle");
                rpr.append_child(
                    OpenXmlElement::w("rStyle").with_attribute_qname("w:val", style_id),
                );
                *count += 1;
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, style_id, count);
            }
        }
        visit(body, style_id, &mut count);
        Ok(count)
    }

    /// Collect unique run style ids (`w:rStyle`) referenced in the body.
    pub fn run_style_ids(&mut self) -> Result<Vec<String>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for e in body.descendants() {
            if e.local_name != "rStyle" {
                continue;
            }
            if let Some(id) = e
                .get_attribute_qname("w:val")
                .or_else(|| e.get_attribute("val"))
            {
                if !out.iter().any(|s| s == id) {
                    out.push(id.to_string());
                }
            }
        }
        Ok(out)
    }

    /// Clear all run styles (`w:rStyle`) from body runs. Returns count cleared.
    pub fn clear_run_styles(&mut self) -> Result<usize> {
        let body = self.body_mut()?;
        let mut count = 0usize;
        fn visit(el: &mut OpenXmlElement, count: &mut usize) {
            if el.local_name == "rPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "rStyle");
                *count += before - el.children.len();
            }
            for c in el.children.iter_mut() {
                visit(c, count);
            }
        }
        visit(body, &mut count);
        Ok(count)
    }

    /// Remove the numbering part and its relationship. Returns whether it existed.
    /// Remove `w:pStyle` from all paragraphs. Returns how many were cleared.
    /// Set or clear bold on all runs in the main document. Returns runs updated.
    pub fn set_all_runs_bold(&mut self, bold: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, bold: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "b");
                    if bold {
                        rpr.append_child(OpenXmlElement::w("b"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, bold, n);
            }
        }
        visit(doc, bold, &mut n);
        Ok(n)
    }

    /// Clear bold from all runs. Returns runs updated.
    pub fn clear_all_runs_bold(&mut self) -> Result<usize> {
        self.set_all_runs_bold(false)
    }


    /// Set or clear italic on all runs. Returns runs updated.
    pub fn set_all_runs_italic(&mut self, italic: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, italic: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "i");
                    if italic {
                        rpr.append_child(OpenXmlElement::w("i"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, italic, n);
            }
        }
        visit(doc, italic, &mut n);
        Ok(n)
    }

    /// Clear italic from all runs.
    pub fn clear_all_runs_italic(&mut self) -> Result<usize> {
        self.set_all_runs_italic(false)
    }

    /// Set underline on all runs (`w:u w:val`). Pass `None` to clear.
    pub fn set_all_runs_underline(&mut self, val: Option<&str>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, val: Option<&str>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "u");
                    if let Some(v) = val {
                        rpr.append_child(
                            OpenXmlElement::w("u").with_attribute_qname("w:val", v),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, val, n);
            }
        }
        visit(doc, val, &mut n);
        Ok(n)
    }

    /// Clear underline from all runs.
    pub fn clear_all_runs_underline(&mut self) -> Result<usize> {
        self.set_all_runs_underline(None)
    }


    /// Set run color (`w:color w:val`) on all runs. Pass `None` to clear.
    pub fn set_all_runs_color(&mut self, rgb: Option<&str>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, rgb: Option<&str>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "color");
                    if let Some(v) = rgb {
                        rpr.append_child(
                            OpenXmlElement::w("color").with_attribute_qname("w:val", v),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, rgb, n);
            }
        }
        visit(doc, rgb, &mut n);
        Ok(n)
    }

    /// Clear color from all runs.
    pub fn clear_all_runs_color(&mut self) -> Result<usize> {
        self.set_all_runs_color(None)
    }


    /// Set highlight on all runs (`w:highlight w:val`). Pass `None` to clear.
    pub fn set_all_runs_highlight(&mut self, val: Option<&str>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, val: Option<&str>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "highlight");
                    if let Some(v) = val {
                        rpr.append_child(
                            OpenXmlElement::w("highlight").with_attribute_qname("w:val", v),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, val, n);
            }
        }
        visit(doc, val, &mut n);
        Ok(n)
    }

    /// Clear highlight from all runs.
    pub fn clear_all_runs_highlight(&mut self) -> Result<usize> {
        self.set_all_runs_highlight(None)
    }

    /// Set or clear strike on all runs (`w:strike`).
    pub fn set_all_runs_strike(&mut self, strike: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, strike: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "strike");
                    if strike {
                        rpr.append_child(OpenXmlElement::w("strike"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, strike, n);
            }
        }
        visit(doc, strike, &mut n);
        Ok(n)
    }

    /// Clear strike from all runs.
    pub fn clear_all_runs_strike(&mut self) -> Result<usize> {
        self.set_all_runs_strike(false)
    }

    /// Set or clear all caps on all runs (`w:caps`).
    pub fn set_all_runs_caps(&mut self, caps: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, caps: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "caps");
                    if caps {
                        rpr.append_child(OpenXmlElement::w("caps"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, caps, n);
            }
        }
        visit(doc, caps, &mut n);
        Ok(n)
    }

    /// Clear caps from all runs.
    pub fn clear_all_runs_caps(&mut self) -> Result<usize> {
        self.set_all_runs_caps(false)
    }

    /// Set or clear vanish (hidden) on all runs (`w:vanish`).
    pub fn set_all_runs_vanish(&mut self, vanish: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, vanish: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "vanish");
                    if vanish {
                        rpr.append_child(OpenXmlElement::w("vanish"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, vanish, n);
            }
        }
        visit(doc, vanish, &mut n);
        Ok(n)
    }

    /// Clear vanish from all runs.
    pub fn clear_all_runs_vanish(&mut self) -> Result<usize> {
        self.set_all_runs_vanish(false)
    }


    /// Set font size on all runs (`w:sz w:val` half-points). Pass `None` to clear.
    pub fn set_all_runs_size(&mut self, half_points: Option<u32>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, half_points: Option<u32>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "sz");
                    if let Some(v) = half_points {
                        rpr.append_child(
                            OpenXmlElement::w("sz")
                                .with_attribute_qname("w:val", v.to_string()),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, half_points, n);
            }
        }
        visit(doc, half_points, &mut n);
        Ok(n)
    }

    /// Clear font size from all runs.
    pub fn clear_all_runs_size(&mut self) -> Result<usize> {
        self.set_all_runs_size(None)
    }


    /// Set ASCII font on all runs (`w:rFonts w:ascii`). Pass `None` to clear rFonts.
    pub fn set_all_runs_font(&mut self, font: Option<&str>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, font: Option<&str>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "rFonts");
                    if let Some(f) = font {
                        rpr.append_child(
                            OpenXmlElement::w("rFonts")
                                .with_attribute_qname("w:ascii", f)
                                .with_attribute_qname("w:hAnsi", f),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, font, n);
            }
        }
        visit(doc, font, &mut n);
        Ok(n)
    }

    /// Clear rFonts from all runs.
    pub fn clear_all_runs_font(&mut self) -> Result<usize> {
        self.set_all_runs_font(None)
    }


    /// Set or clear small caps on all runs (`w:smallCaps`).
    pub fn set_all_runs_small_caps(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "smallCaps");
                    if enabled {
                        rpr.append_child(OpenXmlElement::w("smallCaps"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear small caps from all runs.
    pub fn clear_all_runs_small_caps(&mut self) -> Result<usize> {
        self.set_all_runs_small_caps(false)
    }

    /// Set or clear double strike on all runs (`w:dstrike`).
    pub fn set_all_runs_double_strike(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "dstrike");
                    if enabled {
                        rpr.append_child(OpenXmlElement::w("dstrike"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear double strike from all runs.
    pub fn clear_all_runs_double_strike(&mut self) -> Result<usize> {
        self.set_all_runs_double_strike(false)
    }


    /// Set paragraph alignment on all paragraphs (`w:jc w:val`). Pass `None` to clear.
    /// Set character spacing on all runs (`w:spacing w:val` in twentieths of a point).
    /// Pass `None` to clear.
    pub fn set_all_runs_spacing(&mut self, twips_20: Option<i32>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, twips_20: Option<i32>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "spacing");
                    if let Some(v) = twips_20 {
                        rpr.append_child(
                            OpenXmlElement::w("spacing")
                                .with_attribute_qname("w:val", v.to_string()),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, twips_20, n);
            }
        }
        visit(doc, twips_20, &mut n);
        Ok(n)
    }

    /// Clear character spacing from all runs.
    pub fn clear_all_runs_spacing(&mut self) -> Result<usize> {
        self.set_all_runs_spacing(None)
    }

    /// Set vertical position on all runs (`w:position w:val` in half-points).
    /// Pass `None` to clear.
    pub fn set_all_runs_position(&mut self, half_points: Option<i32>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, half_points: Option<i32>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "position");
                    if let Some(v) = half_points {
                        rpr.append_child(
                            OpenXmlElement::w("position")
                                .with_attribute_qname("w:val", v.to_string()),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, half_points, n);
            }
        }
        visit(doc, half_points, &mut n);
        Ok(n)
    }

    /// Clear vertical position from all runs.
    pub fn clear_all_runs_position(&mut self) -> Result<usize> {
        self.set_all_runs_position(None)
    }


    /// Set font kerning on all runs (`w:kern w:val` half-points). Pass `None` to clear.
    pub fn set_all_runs_kern(&mut self, half_points: Option<u32>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, half_points: Option<u32>, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != "kern");
                    if let Some(v) = half_points {
                        rpr.append_child(
                            OpenXmlElement::w("kern")
                                .with_attribute_qname("w:val", v.to_string()),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, half_points, n);
            }
        }
        visit(doc, half_points, &mut n);
        Ok(n)
    }

    /// Clear kerning from all runs.
    pub fn clear_all_runs_kern(&mut self) -> Result<usize> {
        self.set_all_runs_kern(None)
    }

    /// Set or clear outline effect on all runs (`w:outline`).
    pub fn set_all_runs_outline(&mut self, enabled: bool) -> Result<usize> {
        self.set_all_runs_onoff("outline", enabled)
    }

    /// Clear outline from all runs.
    pub fn clear_all_runs_outline(&mut self) -> Result<usize> {
        self.set_all_runs_outline(false)
    }

    /// Set or clear shadow effect on all runs (`w:shadow`).
    pub fn set_all_runs_shadow(&mut self, enabled: bool) -> Result<usize> {
        self.set_all_runs_onoff("shadow", enabled)
    }

    /// Clear shadow from all runs.
    pub fn clear_all_runs_shadow(&mut self) -> Result<usize> {
        self.set_all_runs_shadow(false)
    }

    /// Set or clear emboss on all runs (`w:emboss`).
    pub fn set_all_runs_emboss(&mut self, enabled: bool) -> Result<usize> {
        self.set_all_runs_onoff("emboss", enabled)
    }

    /// Clear emboss from all runs.
    pub fn clear_all_runs_emboss(&mut self) -> Result<usize> {
        self.set_all_runs_emboss(false)
    }

    /// Set or clear imprint on all runs (`w:imprint`).
    pub fn set_all_runs_imprint(&mut self, enabled: bool) -> Result<usize> {
        self.set_all_runs_onoff("imprint", enabled)
    }

    /// Clear imprint from all runs.
    pub fn clear_all_runs_imprint(&mut self) -> Result<usize> {
        self.set_all_runs_imprint(false)
    }

    fn set_all_runs_onoff(&mut self, local: &str, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, local: &str, enabled: bool, n: &mut usize) {
            if el.local_name == "r" {
                if el.child("rPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("rPr"));
                }
                if let Some(rpr) = el.child_mut("rPr") {
                    rpr.children.retain(|c| c.local_name != local);
                    if enabled {
                        rpr.append_child(OpenXmlElement::w(local));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, local, enabled, n);
            }
        }
        visit(doc, local, enabled, &mut n);
        Ok(n)
    }


    pub fn set_all_paragraphs_alignment(&mut self, val: Option<&str>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, val: Option<&str>, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "jc");
                    if let Some(v) = val {
                        ppr.append_child(
                            OpenXmlElement::w("jc").with_attribute_qname("w:val", v),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, val, n);
            }
        }
        visit(doc, val, &mut n);
        Ok(n)
    }

    /// Clear alignment from all paragraphs.
    pub fn clear_all_paragraphs_alignment(&mut self) -> Result<usize> {
        self.set_all_paragraphs_alignment(None)
    }

    /// Set or clear keep-next on all paragraphs (`w:keepNext`).
    pub fn set_all_paragraphs_keep_next(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "keepNext");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("keepNext"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear keepNext from all paragraphs.
    pub fn clear_all_paragraphs_keep_next(&mut self) -> Result<usize> {
        self.set_all_paragraphs_keep_next(false)
    }

    /// Set or clear widow/orphan control on all paragraphs (`w:widowControl`).
    pub fn set_all_paragraphs_widow_control(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "widowControl");
                    // OOXML: presence with val=0 disables; we use empty element for enable
                    // and omit for default, or val=0 for disable when clearing from enabled
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("widowControl"));
                    } else {
                        ppr.append_child(
                            OpenXmlElement::w("widowControl").with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Remove explicit widowControl from all paragraphs (restore default).
    pub fn clear_all_paragraphs_widow_control(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "widowControl");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }


    /// Set spacing before/after (twips) on all paragraphs. `None` clears that side.
    pub fn set_all_paragraphs_spacing(
        &mut self,
        before: Option<u32>,
        after: Option<u32>,
    ) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(
            el: &mut OpenXmlElement,
            before: Option<u32>,
            after: Option<u32>,
            n: &mut usize,
        ) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    if before.is_none() && after.is_none() {
                        ppr.children.retain(|c| c.local_name != "spacing");
                    } else {
                        let mut spacing = OpenXmlElement::w("spacing");
                        if let Some(b) = before {
                            spacing.set_attribute_qname("w:before", b.to_string());
                        }
                        if let Some(a) = after {
                            spacing.set_attribute_qname("w:after", a.to_string());
                        }
                        ppr.children.retain(|c| c.local_name != "spacing");
                        ppr.append_child(spacing);
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, before, after, n);
            }
        }
        visit(doc, before, after, &mut n);
        Ok(n)
    }

    /// Clear spacing from all paragraphs.
    pub fn clear_all_paragraphs_spacing(&mut self) -> Result<usize> {
        self.set_all_paragraphs_spacing(None, None)
    }

    /// Set left/right indent (twips) on all paragraphs. `None` clears that side.
    pub fn set_all_paragraphs_indent(
        &mut self,
        left: Option<u32>,
        right: Option<u32>,
    ) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(
            el: &mut OpenXmlElement,
            left: Option<u32>,
            right: Option<u32>,
            n: &mut usize,
        ) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    if left.is_none() && right.is_none() {
                        ppr.children.retain(|c| c.local_name != "ind");
                    } else {
                        let mut ind = OpenXmlElement::w("ind");
                        if let Some(l) = left {
                            ind.set_attribute_qname("w:left", l.to_string());
                        }
                        if let Some(r) = right {
                            ind.set_attribute_qname("w:right", r.to_string());
                        }
                        ppr.children.retain(|c| c.local_name != "ind");
                        ppr.append_child(ind);
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, left, right, n);
            }
        }
        visit(doc, left, right, &mut n);
        Ok(n)
    }

    /// Clear indent from all paragraphs.
    pub fn clear_all_paragraphs_indent(&mut self) -> Result<usize> {
        self.set_all_paragraphs_indent(None, None)
    }


    /// Set or clear keep-lines on all paragraphs (`w:keepLines`).
    pub fn set_all_paragraphs_keep_lines(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "keepLines");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("keepLines"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear keepLines from all paragraphs.
    pub fn clear_all_paragraphs_keep_lines(&mut self) -> Result<usize> {
        self.set_all_paragraphs_keep_lines(false)
    }

    /// Set or clear page break before on all paragraphs (`w:pageBreakBefore`).
    pub fn set_all_paragraphs_page_break_before(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "pageBreakBefore");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("pageBreakBefore"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear pageBreakBefore from all paragraphs.
    pub fn clear_all_paragraphs_page_break_before(&mut self) -> Result<usize> {
        self.set_all_paragraphs_page_break_before(false)
    }


    /// Set or clear contextual spacing on all paragraphs (`w:contextualSpacing`).
    pub fn set_all_paragraphs_contextual_spacing(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "contextualSpacing");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("contextualSpacing"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear contextualSpacing from all paragraphs.
    pub fn clear_all_paragraphs_contextual_spacing(&mut self) -> Result<usize> {
        self.set_all_paragraphs_contextual_spacing(false)
    }

    /// Set outline level on all paragraphs (`w:outlineLvl w:val`). Pass `None` to clear.
    pub fn set_all_paragraphs_outline_level(&mut self, level: Option<u32>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, level: Option<u32>, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "outlineLvl");
                    if let Some(lv) = level {
                        ppr.append_child(
                            OpenXmlElement::w("outlineLvl")
                                .with_attribute_qname("w:val", lv.to_string()),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, level, n);
            }
        }
        visit(doc, level, &mut n);
        Ok(n)
    }

    /// Clear outlineLvl from all paragraphs.
    pub fn clear_all_paragraphs_outline_level(&mut self) -> Result<usize> {
        self.set_all_paragraphs_outline_level(None)
    }


    /// Set or clear suppress line numbers on all paragraphs (`w:suppressLineNumbers`).
    pub fn set_all_paragraphs_suppress_line_numbers(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "suppressLineNumbers");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("suppressLineNumbers"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear suppressLineNumbers from all paragraphs.
    pub fn clear_all_paragraphs_suppress_line_numbers(&mut self) -> Result<usize> {
        self.set_all_paragraphs_suppress_line_numbers(false)
    }

    /// Set or clear bidi on all paragraphs (`w:bidi`).
    pub fn set_all_paragraphs_bidi(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "bidi");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("bidi"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear bidi from all paragraphs.
    pub fn clear_all_paragraphs_bidi(&mut self) -> Result<usize> {
        self.set_all_paragraphs_bidi(false)
    }


    /// Set or clear word wrap on all paragraphs (`w:wordWrap`).
    ///
    /// When `enabled` is false, writes `w:wordWrap w:val="0"`; when true, removes the element
    /// (default is wrap-enabled).
    pub fn set_all_paragraphs_word_wrap(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "wordWrap");
                    if !enabled {
                        ppr.append_child(
                            OpenXmlElement::w("wordWrap").with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear explicit wordWrap from all paragraphs (restore default).
    pub fn clear_all_paragraphs_word_wrap(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "wordWrap");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }

    /// Set or clear mirror indents on all paragraphs (`w:mirrorIndents`).
    pub fn set_all_paragraphs_mirror_indents(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "mirrorIndents");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("mirrorIndents"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear mirrorIndents from all paragraphs.
    pub fn clear_all_paragraphs_mirror_indents(&mut self) -> Result<usize> {
        self.set_all_paragraphs_mirror_indents(false)
    }

    /// Set or clear snap-to-grid on all paragraphs (`w:snapToGrid`).
    pub fn set_all_paragraphs_snap_to_grid(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "snapToGrid");
                    if !enabled {
                        ppr.append_child(
                            OpenXmlElement::w("snapToGrid").with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear explicit snapToGrid from all paragraphs.
    pub fn clear_all_paragraphs_snap_to_grid(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "snapToGrid");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }


    /// Set text alignment on all paragraphs (`w:textAlignment w:val`). Pass `None` to clear.
    ///
    /// Values: `auto`, `baseline`, `bottom`, `center`, `top`.
    pub fn set_all_paragraphs_text_alignment(&mut self, val: Option<&str>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, val: Option<&str>, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "textAlignment");
                    if let Some(v) = val {
                        ppr.append_child(
                            OpenXmlElement::w("textAlignment")
                                .with_attribute_qname("w:val", v),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, val, n);
            }
        }
        visit(doc, val, &mut n);
        Ok(n)
    }

    /// Clear textAlignment from all paragraphs.
    pub fn clear_all_paragraphs_text_alignment(&mut self) -> Result<usize> {
        self.set_all_paragraphs_text_alignment(None)
    }


    /// Set or clear East Asian auto-spacing (`w:autoSpaceDE`) on all paragraphs.
    pub fn set_all_paragraphs_auto_space_de(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "autoSpaceDE");
                    if !enabled {
                        ppr.append_child(
                            OpenXmlElement::w("autoSpaceDE").with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear explicit autoSpaceDE from all paragraphs.
    pub fn clear_all_paragraphs_auto_space_de(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "autoSpaceDE");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }

    /// Set or clear number auto-spacing (`w:autoSpaceDN`) on all paragraphs.
    pub fn set_all_paragraphs_auto_space_dn(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "autoSpaceDN");
                    if !enabled {
                        ppr.append_child(
                            OpenXmlElement::w("autoSpaceDN").with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear explicit autoSpaceDN from all paragraphs.
    pub fn clear_all_paragraphs_auto_space_dn(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "autoSpaceDN");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }


    /// Set or clear overflow punctuation on all paragraphs (`w:overflowPunct`).
    pub fn set_all_paragraphs_overflow_punct(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "overflowPunct");
                    if !enabled {
                        ppr.append_child(
                            OpenXmlElement::w("overflowPunct")
                                .with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear explicit overflowPunct from all paragraphs.
    pub fn clear_all_paragraphs_overflow_punct(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "overflowPunct");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }

    /// Set or clear top-line punctuation on all paragraphs (`w:topLinePunct`).
    pub fn set_all_paragraphs_top_line_punct(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "topLinePunct");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("topLinePunct"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear topLinePunct from all paragraphs.
    pub fn clear_all_paragraphs_top_line_punct(&mut self) -> Result<usize> {
        self.set_all_paragraphs_top_line_punct(false)
    }

    /// Set or clear adjust-right-indent on all paragraphs (`w:adjustRightInd`).
    pub fn set_all_paragraphs_adjust_right_ind(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "adjustRightInd");
                    if !enabled {
                        ppr.append_child(
                            OpenXmlElement::w("adjustRightInd")
                                .with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear explicit adjustRightInd from all paragraphs.
    pub fn clear_all_paragraphs_adjust_right_ind(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "adjustRightInd");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }


    /// Set or clear kinsoku on all paragraphs (`w:kinsoku`). Default enabled when absent.
    pub fn set_all_paragraphs_kinsoku(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "kinsoku");
                    if !enabled {
                        ppr.append_child(
                            OpenXmlElement::w("kinsoku").with_attribute_qname("w:val", "0"),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear explicit kinsoku from all paragraphs.
    pub fn clear_all_paragraphs_kinsoku(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "kinsoku");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }


    /// Set or clear suppress auto hyphens on all paragraphs (`w:suppressAutoHyphens`).
    pub fn set_all_paragraphs_suppress_auto_hyphens(&mut self, enabled: bool) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, enabled: bool, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "suppressAutoHyphens");
                    if enabled {
                        ppr.append_child(OpenXmlElement::w("suppressAutoHyphens"));
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, enabled, n);
            }
        }
        visit(doc, enabled, &mut n);
        Ok(n)
    }

    /// Clear suppressAutoHyphens from all paragraphs.
    pub fn clear_all_paragraphs_suppress_auto_hyphens(&mut self) -> Result<usize> {
        self.set_all_paragraphs_suppress_auto_hyphens(false)
    }


    /// Set paragraph shading on all paragraphs (`w:shd`).
    ///
    /// `val` is the pattern (e.g. `"clear"`); `fill` is hex RGB without `#`.
    /// Pass both `None` to clear.
    pub fn set_all_paragraphs_shading(
        &mut self,
        val: Option<&str>,
        fill: Option<&str>,
    ) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(
            el: &mut OpenXmlElement,
            val: Option<&str>,
            fill: Option<&str>,
            n: &mut usize,
        ) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "shd");
                    if val.is_some() || fill.is_some() {
                        let mut shd = OpenXmlElement::w("shd");
                        if let Some(v) = val {
                            shd = shd.with_attribute_qname("w:val", v);
                        }
                        if let Some(f) = fill {
                            shd = shd.with_attribute_qname("w:fill", f);
                        }
                        ppr.append_child(shd);
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, val, fill, n);
            }
        }
        visit(doc, val, fill, &mut n);
        Ok(n)
    }

    /// Clear shading from all paragraphs.
    pub fn clear_all_paragraphs_shading(&mut self) -> Result<usize> {
        self.set_all_paragraphs_shading(None, None)
    }

    /// Set line spacing on all paragraphs (`w:spacing w:line` / `w:lineRule`).
    ///
    /// `line` is in twips (240 = single); `line_rule` e.g. `"auto"`, `"exact"`, `"atLeast"`.
    pub fn set_all_paragraphs_line_spacing(
        &mut self,
        line: Option<u32>,
        line_rule: Option<&str>,
    ) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(
            el: &mut OpenXmlElement,
            line: Option<u32>,
            line_rule: Option<&str>,
            n: &mut usize,
        ) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    // merge with existing spacing if present
                    if line.is_none() && line_rule.is_none() {
                        if let Some(sp) = ppr.child_mut("spacing") {
                            sp.attributes.retain(|a| {
                                !matches!(a.local_name.as_str(), "line" | "lineRule")
                            });
                            if sp.attributes.is_empty() {
                                ppr.children.retain(|c| c.local_name != "spacing");
                            }
                        }
                    } else {
                        if ppr.child("spacing").is_none() {
                            ppr.append_child(OpenXmlElement::w("spacing"));
                        }
                        if let Some(sp) = ppr.child_mut("spacing") {
                            if let Some(l) = line {
                                sp.set_attribute_qname("w:line", l.to_string());
                            }
                            if let Some(r) = line_rule {
                                sp.set_attribute_qname("w:lineRule", r);
                            }
                        }
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, line, line_rule, n);
            }
        }
        visit(doc, line, line_rule, &mut n);
        Ok(n)
    }

    /// Clear line spacing attrs from all paragraphs (keeps before/after if present).
    pub fn clear_all_paragraphs_line_spacing(&mut self) -> Result<usize> {
        self.set_all_paragraphs_line_spacing(None, None)
    }


    /// Set a bottom border on all paragraphs (`w:pBdr/w:bottom`).
    ///
    /// `val` e.g. `"single"`; `sz` is eighths of a point; `color` hex without `#`.
    pub fn set_all_paragraphs_bottom_border(
        &mut self,
        val: &str,
        sz: u32,
        color: &str,
    ) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, val: &str, sz: u32, color: &str, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    if ppr.child("pBdr").is_none() {
                        ppr.append_child(OpenXmlElement::w("pBdr"));
                    }
                    if let Some(pb) = ppr.child_mut("pBdr") {
                        pb.children.retain(|c| c.local_name != "bottom");
                        pb.append_child(
                            OpenXmlElement::w("bottom")
                                .with_attribute_qname("w:val", val)
                                .with_attribute_qname("w:sz", sz.to_string())
                                .with_attribute_qname("w:space", "1")
                                .with_attribute_qname("w:color", color),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, val, sz, color, n);
            }
        }
        visit(doc, val, sz, color, &mut n);
        Ok(n)
    }

    /// Clear all paragraph borders (`w:pBdr`) from all paragraphs.
    pub fn clear_all_paragraphs_borders(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "pBdr");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }


    /// Set a top border on all paragraphs (`w:pBdr/w:top`).
    pub fn set_all_paragraphs_top_border(
        &mut self,
        val: &str,
        sz: u32,
        color: &str,
    ) -> Result<usize> {
        self.set_all_paragraphs_side_border("top", val, sz, color)
    }

    /// Set a left border on all paragraphs (`w:pBdr/w:left`).
    pub fn set_all_paragraphs_left_border(
        &mut self,
        val: &str,
        sz: u32,
        color: &str,
    ) -> Result<usize> {
        self.set_all_paragraphs_side_border("left", val, sz, color)
    }

    /// Set a right border on all paragraphs (`w:pBdr/w:right`).
    pub fn set_all_paragraphs_right_border(
        &mut self,
        val: &str,
        sz: u32,
        color: &str,
    ) -> Result<usize> {
        self.set_all_paragraphs_side_border("right", val, sz, color)
    }

    fn set_all_paragraphs_side_border(
        &mut self,
        side: &str,
        val: &str,
        sz: u32,
        color: &str,
    ) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(
            el: &mut OpenXmlElement,
            side: &str,
            val: &str,
            sz: u32,
            color: &str,
            n: &mut usize,
        ) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    if ppr.child("pBdr").is_none() {
                        ppr.append_child(OpenXmlElement::w("pBdr"));
                    }
                    if let Some(pb) = ppr.child_mut("pBdr") {
                        pb.children.retain(|c| c.local_name != side);
                        pb.append_child(
                            OpenXmlElement::w(side)
                                .with_attribute_qname("w:val", val)
                                .with_attribute_qname("w:sz", sz.to_string())
                                .with_attribute_qname("w:space", "1")
                                .with_attribute_qname("w:color", color),
                        );
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, side, val, sz, color, n);
            }
        }
        visit(doc, side, val, sz, color, &mut n);
        Ok(n)
    }


    /// Set first-line indent (twips) on all paragraphs (`w:ind w:firstLine`).
    /// Pass `None` to clear firstLine/hanging.
    pub fn set_all_paragraphs_first_line_indent(&mut self, twips: Option<u32>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, twips: Option<u32>, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    if twips.is_none() {
                        if let Some(ind) = ppr.child_mut("ind") {
                            ind.attributes.retain(|a| {
                                !matches!(a.local_name.as_str(), "firstLine" | "hanging")
                            });
                            if ind.attributes.is_empty() {
                                ppr.children.retain(|c| c.local_name != "ind");
                            }
                        }
                    } else {
                        if ppr.child("ind").is_none() {
                            ppr.append_child(OpenXmlElement::w("ind"));
                        }
                        if let Some(ind) = ppr.child_mut("ind") {
                            ind.attributes.retain(|a| a.local_name != "hanging");
                            ind.set_attribute_qname("w:firstLine", twips.unwrap().to_string());
                        }
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, twips, n);
            }
        }
        visit(doc, twips, &mut n);
        Ok(n)
    }

    /// Clear first-line/hanging indent from all paragraphs.
    pub fn clear_all_paragraphs_first_line_indent(&mut self) -> Result<usize> {
        self.set_all_paragraphs_first_line_indent(None)
    }


    /// Set hanging indent (twips) on all paragraphs (`w:ind w:hanging`).
    pub fn set_all_paragraphs_hanging_indent(&mut self, twips: Option<u32>) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, twips: Option<u32>, n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    if twips.is_none() {
                        if let Some(ind) = ppr.child_mut("ind") {
                            ind.attributes.retain(|a| a.local_name != "hanging");
                            if ind.attributes.is_empty() {
                                ppr.children.retain(|c| c.local_name != "ind");
                            }
                        }
                    } else {
                        if ppr.child("ind").is_none() {
                            ppr.append_child(OpenXmlElement::w("ind"));
                        }
                        if let Some(ind) = ppr.child_mut("ind") {
                            ind.attributes.retain(|a| a.local_name != "firstLine");
                            ind.set_attribute_qname("w:hanging", twips.unwrap().to_string());
                        }
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, twips, n);
            }
        }
        visit(doc, twips, &mut n);
        Ok(n)
    }

    /// Clear hanging indent from all paragraphs.
    pub fn clear_all_paragraphs_hanging_indent(&mut self) -> Result<usize> {
        self.set_all_paragraphs_hanging_indent(None)
    }

    /// Set tab stops on all paragraphs (`w:tabs` under `w:pPr`).
    /// Each stop is `(val, pos_twips)` e.g. `("left", 720)`.
    /// Pass empty slice to clear tabs.
    pub fn set_all_paragraphs_tabs(
        &mut self,
        stops: &[(/* val */ &str, /* pos twips */ u32)],
    ) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        let owned: Vec<(String, u32)> = stops
            .iter()
            .map(|(v, p)| ((*v).to_string(), *p))
            .collect();
        fn visit(el: &mut OpenXmlElement, owned: &[(String, u32)], n: &mut usize) {
            if el.local_name == "p" {
                if el.child("pPr").is_none() {
                    el.children.insert(0, OpenXmlElement::w("pPr"));
                }
                if let Some(ppr) = el.child_mut("pPr") {
                    ppr.children.retain(|c| c.local_name != "tabs");
                    if !owned.is_empty() {
                        let mut tabs = OpenXmlElement::w("tabs");
                        for (val, pos) in owned {
                            let mut tab = OpenXmlElement::w("tab");
                            tab.set_attribute_qname("w:val", val.clone());
                            tab.set_attribute_qname("w:pos", pos.to_string());
                            tabs.append_child(tab);
                        }
                        ppr.append_child(tabs);
                    }
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, owned, n);
            }
        }
        visit(doc, &owned, &mut n);
        Ok(n)
    }

    /// Clear tab stops from all paragraphs.
    pub fn clear_all_paragraphs_tabs(&mut self) -> Result<usize> {
        self.set_all_paragraphs_tabs(&[])
    }


    pub fn clear_paragraph_styles(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn visit(el: &mut OpenXmlElement, n: &mut usize) {
            if el.local_name == "pPr" {
                let before = el.children.len();
                el.children.retain(|c| c.local_name != "pStyle");
                if el.children.len() < before {
                    *n += 1;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, n);
            }
        }
        visit(doc, &mut n);
        Ok(n)
    }


    pub fn clear_numbering(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/numbering.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::NUMBERING)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Append a paragraph element to the document body (before trailing `sectPr` if present).
    pub fn append_paragraph(&mut self, para: OpenXmlElement) -> Result<()> {
        let body = self.body_mut()?;
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, para);
        } else {
            body.append_child(para);
        }
        Ok(())
    }

    /// Remove body-level paragraphs by 0-based index among `w:p` children.
    ///
    /// Returns the number of paragraphs removed.
    pub fn remove_paragraphs_at(&mut self, indices: &[usize]) -> Result<usize> {
        let body = self.body_mut()?;
        let para_positions: Vec<usize> = body
            .children
            .iter()
            .enumerate()
            .filter(|(_, c)| c.local_name == "p")
            .map(|(i, _)| i)
            .collect();
        let mut to_remove: Vec<usize> = indices
            .iter()
            .filter_map(|i| para_positions.get(*i).copied())
            .collect();
        to_remove.sort_unstable();
        to_remove.dedup();
        let mut removed = 0usize;
        for pos in to_remove.into_iter().rev() {
            body.children.remove(pos);
            removed += 1;
        }
        Ok(removed)
    }

    /// Append a table (from a 2D string grid) to the document body.
    pub fn append_table(
        &mut self,
        rows: &[Vec<&str>],
        column_widths: Option<&[u32]>,
    ) -> Result<()> {
        use crate::wordprocessing::table_from_strings;
        let tbl = table_from_strings(rows, column_widths);
        let body = self.body_mut()?;
        if let Some(pos) = body.children.iter().position(|c| c.local_name == "sectPr") {
            body.children.insert(pos, tbl);
        } else {
            body.append_child(tbl);
        }
        Ok(())
    }

    /// Extract all body-level tables as 2D string grids.
    pub fn body_tables_as_strings(&mut self) -> Result<Vec<Vec<Vec<String>>>> {
        use crate::wordprocessing::table_to_strings;
        let body = self.body_mut()?;
        Ok(body
            .children
            .iter()
            .filter(|c| c.local_name == "tbl")
            .map(table_to_strings)
            .collect())
    }

    /// Append a row of cells to the first body-level table.
    ///
    /// Returns an error if no table exists.
    pub fn append_table_row(&mut self, cells: &[&str]) -> Result<()> {
        use crate::wordprocessing::{table_cell_with_text, table_row};
        let body = self.body_mut()?;
        let tbl = body
            .children
            .iter_mut()
            .find(|c| c.local_name == "tbl")
            .ok_or_else(|| Error::Package("no table in body".into()))?;
        let row_cells: Vec<_> = cells.iter().map(|c| table_cell_with_text(*c)).collect();
        tbl.append_child(table_row(row_cells));
        Ok(())
    }

    /// Number of body-level tables.
    pub fn table_count(&mut self) -> Result<usize> {
        let body = self.body_mut()?;
        Ok(body.children.iter().filter(|c| c.local_name == "tbl").count())
    }

    /// Remove a body-level table by 0-based index among tables.
    pub fn remove_table_at(&mut self, index: usize) -> Result<()> {
        let body = self.body_mut()?;
        let positions: Vec<usize> = body
            .children
            .iter()
            .enumerate()
            .filter(|(_, c)| c.local_name == "tbl")
            .map(|(i, _)| i)
            .collect();
        let pos = positions
            .get(index)
            .copied()
            .ok_or_else(|| Error::Package(format!("table index {index} out of range")))?;
        body.children.remove(pos);
        Ok(())
    }

    /// Whether the document body has no paragraphs or tables (sectPr ignored).
    pub fn is_body_empty(&mut self) -> Result<bool> {
        let body = self.body_mut()?;
        Ok(!body
            .children
            .iter()
            .any(|c| c.local_name == "p" || c.local_name == "tbl"))
    }

    /// Whether this package is a macro-enabled document type.
    pub fn is_macro_enabled(&self) -> bool {
        matches!(
            self.document_type,
            WordprocessingDocumentType::MacroEnabledDocument
                | WordprocessingDocumentType::MacroEnabledTemplate
        )
    }

    /// Number of parts in the underlying OPC package.
    pub fn part_count(&self) -> usize {
        self.package.opc().part_uris().len()
    }

    /// Alias for [`part_count`](Self::part_count) (shared naming with Excel/PPT).
    pub fn package_part_count(&self) -> usize {
        self.part_count()
    }

    /// True if a VBA project part is present.
    pub fn has_vba_project(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("vbaProject") || u.as_str().ends_with("vbaProject.bin"))
    }

    /// Remove VBA project and vbaData parts and their relationships.
    /// Read raw VBA project bytes if present.
    pub fn vba_project_bytes(&self) -> Option<Vec<u8>> {
        for uri in self.package.opc().part_uris() {
            let s = uri.as_str();
            if s.contains("vbaProject") || s.ends_with("vbaProject.bin") {
                return self.package.opc().get_part_cloned(&uri).ok().flatten();
            }
        }
        None
    }

    /// List URIs of VBA-related parts (project + data + signatures shells).
    pub fn list_vba_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str().to_ascii_lowercase();
                s.contains("vbaproject") || s.contains("vbadata") || s.contains("vbasignature")
            })
            .collect()
    }

    /// Count VBA-related parts.
    pub fn vba_part_count(&self) -> usize {
        self.list_vba_parts().len()
    }

    /// Parse `vbaProject.bin` CFB structure (streams/storages inventory; no macro execution).
    pub fn inspect_vba_project(&self) -> crate::Result<Option<crate::opc::CfbFile>> {
        let Some(bytes) = self.vba_project_bytes() else { return Ok(None); };
        Ok(Some(crate::opc::inspect_vba_project(&bytes)?))
    }


    pub fn clear_vba_project(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.contains("vbaProject") || s.contains("vbaData")
            })
            
            .collect();
        if uris.is_empty() {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::VBA_PROJECT
                                || r.relationship_type == rel::VBA_DATA
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        // Also drop vbaData related from vbaProject
        let vba_uri = PackUri::new("/word/vbaProject.bin");
        if self.package.opc().has_part(&vba_uri) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&vba_uri)
                .map(|rels| {
                    rels.iter().map(|r| r.id.clone()).collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(&vba_uri);
                for id in ids {
                    rels.remove(&id);
                }
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Whether any paragraph text contains `needle`.
    pub fn contains_text(&mut self, needle: &str) -> Result<bool> {
        if needle.is_empty() {
            return Ok(true);
        }
        Ok(self
            .paragraph_texts()?
            .iter()
            .any(|t| t.contains(needle)))
    }

    /// Count non-overlapping occurrences of `needle` across paragraph texts.
    pub fn count_text(&mut self, needle: &str) -> Result<usize> {
        if needle.is_empty() {
            return Ok(0);
        }
        let mut total = 0usize;
        for t in self.paragraph_texts()? {
            let mut rest = t.as_str();
            while let Some(pos) = rest.find(needle) {
                total += 1;
                rest = &rest[pos + needle.len()..];
            }
        }
        Ok(total)
    }

    /// First non-empty paragraph text, if any.
    pub fn first_paragraph_text(&mut self) -> Result<Option<String>> {
        Ok(self
            .paragraph_texts()?
            .into_iter()
            .find(|t| !t.is_empty()))
    }

    /// Last non-empty paragraph text, if any.
    pub fn last_paragraph_text(&mut self) -> Result<Option<String>> {
        Ok(self
            .paragraph_texts()?
            .into_iter()
            .rev()
            .find(|t| !t.is_empty()))
    }

    /// Character count across all paragraph texts (including spaces).
    pub fn character_count(&mut self) -> Result<usize> {
        Ok(self.paragraph_texts()?.iter().map(|t| t.chars().count()).sum())
    }

    /// Whether a styles part is present.
    pub fn has_styles(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/styles.xml"))
    }


    /// Count styles-related parts under `/word/`.
    pub fn styles_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                let s = u.as_str();
                s.contains("/word/styles") || s.ends_with("/styles.xml")
            })
            .count()
    }

    /// Remove the styles part and main-document relationship.
    pub fn clear_styles(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::STYLES)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Whether a settings part is present.
    pub fn has_settings(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/settings.xml"))
    }

    /// Remove the settings part and main-document relationship.
    pub fn clear_settings(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/settings.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::SETTINGS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Whether a theme part is present.
    pub fn has_theme(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/word/theme/"))
    }


    /// Count theme parts under `/word/theme/`.
    pub fn theme_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/theme/"))
            .count()
    }

    /// List theme part URIs.
    pub fn list_themes(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/theme/"))
            
            .collect()
    }

    /// Read theme names from theme parts (`a:theme/@name`).
    pub fn list_theme_names(&self) -> Result<Vec<(PackUri, String)>> {
        let mut out = Vec::new();
        for uri in self.list_themes() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            let name = root.get_attribute("name").unwrap_or("").to_string();
            out.push((uri, name));
        }
        Ok(out)
    }

    /// Set the theme name on the first theme part.
    pub fn set_theme_name(&mut self, name: &str) -> Result<bool> {
        let Some(uri) = self.list_themes().into_iter().next() else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("name", name);
        let ct = self
            .package
            .opc()
            .content_types()
            .content_type_for(uri.as_str())
            .unwrap_or(content_type::THEME)
            .to_string();
        self.package
            .opc_mut()
            .set_part(uri, &ct, crate::element::write_element(&root)?);
        Ok(true)
    }

    /// Read theme `@name` from the first theme part.
    pub fn theme_name(&self) -> Result<Option<String>> {
        let Some(uri) = self.list_themes().into_iter().next() else {
            return Ok(None);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("name").map(|s| s.to_string()))
    }

    /// Whether theme name is set.
    pub fn has_theme_name(&self) -> Result<bool> {
        Ok(self.theme_name()?.is_some())
    }

    /// Clear theme `@name` (does not remove the theme part).
    pub fn clear_theme_name(&mut self) -> Result<bool> {
        let Some(uri) = self.list_themes().into_iter().next() else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("name").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "name");
        let ct = self
            .package
            .opc()
            .content_types()
            .content_type_for(uri.as_str())
            .unwrap_or(content_type::THEME)
            .to_string();
        self.package
            .opc_mut()
            .set_part(uri, ct, crate::element::write_element(&root)?);
        Ok(true)
    }

    /// Remove theme parts and main-document theme relationships.
    pub fn clear_theme(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/theme/"))
            
            .collect();
        if uris.is_empty() {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::THEME)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Whether a font table part is present.
    pub fn has_font_table(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/fontTable.xml"))
    }

    /// List font names from `/word/fontTable.xml`.
    pub fn list_font_names(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/word/fontTable.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("font")
            .filter_map(|f| {
                f.get_attribute_qname("w:name")
                    .or_else(|| f.get_attribute("name"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Number of fonts in the font table.


    /// Whether the font table has any font entries.
    pub fn has_font_names(&self) -> Result<bool> {
        Ok(!self.list_font_names()?.is_empty())
    }

    /// Number of fonts in the font table.
    pub fn font_entry_count(&self) -> Result<usize> {
        Ok(self.list_font_names()?.len())
    }

    /// Whether a font with the given name exists in the font table.
    pub fn has_font_entry(&self, name: &str) -> Result<bool> {
        Ok(self.list_font_names()?.iter().any(|n| n == name))
    }

    /// List font table entries as `(name, charset?, family?, pitch?)`.
    pub fn list_font_entries(
        &self,
    ) -> Result<Vec<(String, Option<String>, Option<String>, Option<String>)>> {
        let uri = PackUri::new("/word/fontTable.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let mut out = Vec::new();
        for f in root.children_by_name("font") {
            let name = f
                .get_attribute_qname("w:name")
                .or_else(|| f.get_attribute("name"))
                .unwrap_or("")
                .to_string();
            if name.is_empty() {
                continue;
            }
            let child_val = |local: &str| -> Option<String> {
                f.child(local).and_then(|c| {
                    c.get_attribute_qname("w:val")
                        .or_else(|| c.get_attribute("val"))
                        .map(|s| s.to_string())
                })
            };
            out.push((
                name,
                child_val("charset"),
                child_val("family"),
                child_val("pitch"),
            ));
        }
        Ok(out)
    }

    /// Ensure a font table part exists (creates default if missing).
    pub fn ensure_font_table(&mut self) -> Result<()> {
        if !self.has_font_table() {
            let _ = self.add_default_font_table()?;
        }
        Ok(())
    }

    /// Add or replace a font entry in the font table.
    pub fn add_font_entry(
        &mut self,
        name: &str,
        charset: Option<&str>,
        family: Option<&str>,
        pitch: Option<&str>,
        alt_name: Option<&str>,
    ) -> Result<()> {
        self.ensure_font_table()?;
        let uri = PackUri::new("/word/fontTable.xml");
        let data = self
            .package
            .opc()
            .get_part(&uri)
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))?;
        let mut root = parse_element(data)?;
        root.children.retain(|c| {
            !(c.local_name == "font"
                && c.get_attribute_qname("w:name")
                    .or_else(|| c.get_attribute("name"))
                    == Some(name))
        });
        let mut font = OpenXmlElement::w("font").with_attribute_qname("w:name", name);
        if let Some(a) = alt_name {
            font.append_child(OpenXmlElement::w("altName").with_attribute_qname("w:val", a));
        }
        if let Some(c) = charset {
            font.append_child(OpenXmlElement::w("charset").with_attribute_qname("w:val", c));
        }
        if let Some(f) = family {
            font.append_child(OpenXmlElement::w("family").with_attribute_qname("w:val", f));
        }
        if let Some(p) = pitch {
            font.append_child(OpenXmlElement::w("pitch").with_attribute_qname("w:val", p));
        }
        root.append_child(font);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::WORD_FONT_TABLE, xml);
        Ok(())
    }

    /// Read font entry details as `(charset?, family?, pitch?, alt_name?)`.
    pub fn font_entry(
        &self,
        name: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<String>, Option<String>)>> {
        let uri = PackUri::new("/word/fontTable.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        for f in root.children_by_name("font") {
            let n = f
                .get_attribute_qname("w:name")
                .or_else(|| f.get_attribute("name"))
                .unwrap_or("");
            if n != name {
                continue;
            }
            let child_val = |local: &str| {
                f.child(local).and_then(|c| {
                    c.get_attribute_qname("w:val")
                        .or_else(|| c.get_attribute("val"))
                        .map(|s| s.to_string())
                })
            };
            return Ok(Some((
                child_val("charset"),
                child_val("family"),
                child_val("pitch"),
                child_val("altName"),
            )));
        }
        Ok(None)
    }

    /// Remove a font entry by name. Returns whether present.
    pub fn remove_font_entry(&mut self, name: &str) -> Result<bool> {
        let uri = PackUri::new("/word/fontTable.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| {
            !(c.local_name == "font"
                && c.get_attribute_qname("w:name")
                    .or_else(|| c.get_attribute("name"))
                    == Some(name))
        });
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_FONT_TABLE, xml);
        }
        Ok(removed)
    }

    /// Remove the font table part and relationship.
    pub fn clear_font_table(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/fontTable.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::FONT_TABLE)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Ensure default styles exist; returns the styles relationship id.
    pub fn ensure_styles(&mut self) -> Result<String> {
        if self.has_styles() {
            // Return existing relationship if possible
            if let Some(main) = self.main_document_part.as_ref() {
                let uri = main.part().uri.clone();
                if let Some(rid) = self
                    .package
                    .opc()
                    .part_relationships(&uri)
                    .and_then(|r| r.get_by_type(rel::STYLES).map(|x| x.id.clone()))
                {
                    return Ok(rid);
                }
            }
        }
        self.add_default_styles()
    }

    /// Ensure a default theme exists.
    pub fn ensure_theme(&mut self) -> Result<String> {
        if self.has_theme() {
            if let Some(main) = self.main_document_part.as_ref() {
                let uri = main.part().uri.clone();
                if let Some(rid) = self
                    .package
                    .opc()
                    .part_relationships(&uri)
                    .and_then(|r| r.get_by_type(rel::THEME).map(|x| x.id.clone()))
                {
                    return Ok(rid);
                }
            }
        }
        self.add_default_theme()
    }

    /// List header part URIs related from the main document.
    pub fn list_headers(&self) -> Vec<PackUri> {
        self.list_related_parts(rel::HEADER)
    }

    /// Whether any header parts exist.
    pub fn has_header(&self) -> bool {
        !self.list_headers().is_empty()
    }

    /// List header relationships as `(rId, uri)`.
    pub fn list_header_relationships(&self) -> Vec<(String, PackUri)> {
        self.list_related_part_relationships(rel::HEADER)
    }

    /// List footer part URIs related from the main document.
    pub fn list_footers(&self) -> Vec<PackUri> {
        self.list_related_parts(rel::FOOTER)
    }

    /// Whether any footer parts exist.
    pub fn has_footer(&self) -> bool {
        !self.list_footers().is_empty()
    }

    /// List footer relationships as `(rId, uri)`.
    pub fn list_footer_relationships(&self) -> Vec<(String, PackUri)> {
        self.list_related_part_relationships(rel::FOOTER)
    }

    /// Count binary media parts under `/word/media/`.
    pub fn media_count(&self) -> usize {
        self.list_images().len()
    }


    /// Alias for [`has_images`](Self::has_images).
    pub fn has_media(&self) -> bool {
        self.has_images()
    }

    /// Alias for [`list_images`](Self::list_images).
    pub fn list_media(&self) -> Vec<PackUri> {
        self.list_images()
    }

    /// Remove a single media/image part by URI and drop main-document relationships that target it.
    pub fn remove_media(&mut self, uri: &PackUri) -> Result<bool> {
        if !uri.as_str().starts_with("/word/media/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        let target = uri.as_str().to_string();
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            let t = r.target.as_str();
                            crate::opc::resolve_uri(&main_uri, t)
                                .map(|u| u.as_str() == target)
                                .unwrap_or(false)
                                || t == target
                                || t.ends_with(target.trim_start_matches('/'))
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Alias for [`clear_images`](Self::clear_images).
    pub fn clear_media(&mut self) -> Result<usize> {
        self.clear_images()
    }

    /// Whether any media/image parts exist under `/word/media/`.
    pub fn has_images(&self) -> bool {
        !self.list_images().is_empty()
    }


    /// List image part URIs under `/word/media/`.
    pub fn list_images(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/media/"))
            
            .collect()
    }

    /// Remove all media parts under `/word/media/` and image relationships from the main document.
    pub fn clear_images(&mut self) -> Result<usize> {
        let images = self.list_images();
        let n = images.len();
        if n == 0 {
            return Ok(0);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::IMAGE
                                || r.relationship_type.contains("image")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in images {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Whether any header parts are related from the main document.
    pub fn has_headers(&self) -> bool {
        !self.list_headers().is_empty()
    }

    /// Whether any footer parts are related from the main document.
    pub fn has_footers(&self) -> bool {
        !self.list_footers().is_empty()
    }

    /// Number of header parts related from the main document.
    pub fn header_count(&self) -> usize {
        self.list_headers().len()
    }

    /// Number of footer parts related from the main document.
    pub fn footer_count(&self) -> usize {
        self.list_footers().len()
    }

    /// Number of external hyperlink relationships from the main document.
    pub fn external_hyperlink_count(&self) -> usize {
        self.list_external_hyperlinks().len()
    }

    /// Whether any external hyperlinks are present.
    pub fn has_external_hyperlinks(&self) -> bool {
        self.external_hyperlink_count() > 0
    }

    /// Collect plain text from each header part (joined `w:t` runs).
    pub fn header_texts(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for uri in self.list_headers() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            out.push(
                root.descendants()
                    .filter(|e| e.local_name == "t")
                    .filter_map(|e| e.text.as_deref())
                    .collect::<Vec<_>>()
                    .join(""),
            );
        }
        Ok(out)
    }

    /// Collect plain text from each footer part (joined `w:t` runs).
    pub fn footer_texts(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for uri in self.list_footers() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            out.push(
                root.descendants()
                    .filter(|e| e.local_name == "t")
                    .filter_map(|e| e.text.as_deref())
                    .collect::<Vec<_>>()
                    .join(""),
            );
        }
        Ok(out)
    }

    /// Replace all text nodes in the n-th header with a single paragraph of `text`.
    pub fn set_header_text(&mut self, index: usize, content: &str) -> Result<bool> {
        let headers = self.list_headers();
        let Some(uri) = headers.get(index).cloned() else {
            return Ok(false);
        };
        let root = header(vec![paragraph(vec![run(vec![text(content)])])]);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::WORD_HEADER, xml);
        Ok(true)
    }

    /// Replace all text nodes in the n-th footer with a single paragraph of `text`.
    pub fn set_footer_text(&mut self, index: usize, content: &str) -> Result<bool> {
        let footers = self.list_footers();
        let Some(uri) = footers.get(index).cloned() else {
            return Ok(false);
        };
        let root = footer(vec![paragraph(vec![run(vec![text(content)])])]);
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::WORD_FOOTER, xml);
        Ok(true)
    }

    /// Remove a header part by 0-based index among header relationships.
    ///
    /// Also drops the main-document relationship and matching `sectPr` `headerReference`s.
    pub fn remove_header_at(&mut self, index: usize) -> Result<()> {
        self.remove_related_part_at(rel::HEADER, index)
    }

    /// Remove a footer part by 0-based index among footer relationships.
    ///
    /// Also drops the main-document relationship and matching `sectPr` `footerReference`s.
    pub fn remove_footer_at(&mut self, index: usize) -> Result<()> {
        self.remove_related_part_at(rel::FOOTER, index)
    }

    /// Remove all header parts related from the main document.
    pub fn clear_headers(&mut self) -> Result<usize> {
        let n = self.list_headers().len();
        for _ in 0..n {
            self.remove_header_at(0)?;
        }
        Ok(n)
    }

    /// Remove all footer parts related from the main document.
    pub fn clear_footers(&mut self) -> Result<usize> {
        let n = self.list_footers().len();
        for _ in 0..n {
            self.remove_footer_at(0)?;
        }
        Ok(n)
    }

    fn remove_related_part_at(&mut self, relationship_type: &str, index: usize) -> Result<()> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let rels: Vec<(String, String)> = self
            .package
            .opc()
            .part_relationships(&main_uri)
            .map(|r| {
                r.find_all_by_type(relationship_type)
                    .into_iter()
                    .map(|rel| (rel.id.clone(), rel.target.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let (rid, target) = rels.get(index).cloned().ok_or_else(|| {
            Error::Package(format!("{relationship_type} index {index} out of range"))
        })?;
        let uri = if target.starts_with('/') {
            PackUri::new(target)
        } else {
            PackUri::new(format!("/word/{}", target.trim_start_matches("./")))
        };
        let _ = self
            .package
            .opc_mut()
            .part_relationships_mut(&main_uri)
            .remove(&rid);
        self.package.opc_mut().remove_part(&uri);
        // Drop matching sectPr headerReference/footerReference entries for this rId.
        if relationship_type == rel::HEADER || relationship_type == rel::FOOTER {
            let _ = self.remove_sect_pr_reference_by_id(&rid);
        }
        Ok(())
    }

    /// Remove `w:headerReference` / `w:footerReference` (and similar) from all `sectPr`
    /// elements whose `r:id` matches `rid`. Returns count removed.
    pub fn remove_sect_pr_reference_by_id(&mut self, rid: &str) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn visit(el: &mut OpenXmlElement, rid: &str, count: &mut usize) {
            if el.local_name == "sectPr" {
                let before = el.children.len();
                el.children.retain(|c| {
                    let is_ref = matches!(
                        c.local_name.as_str(),
                        "headerReference" | "footerReference"
                    );
                    if is_ref {
                        let id = c
                            .get_attribute_qname("r:id")
                            .or_else(|| c.get_attribute("id"));
                        if id == Some(rid) {
                            return false;
                        }
                    }
                    true
                });
                *count += before - el.children.len();
            }
            for c in el.children.iter_mut() {
                visit(c, rid, count);
            }
        }
        let mut count = 0usize;
        visit(doc, rid, &mut count);
        Ok(count)
    }

    /// List sectPr header/footer references as `(kind, type, rId)` where kind is
    /// `"header"` or `"footer"`.
    pub fn list_sect_pr_references(&mut self) -> Result<Vec<(String, String, String)>> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let doc = main
            .part()
            .root
            .as_ref()
            .ok_or(Error::NoRootElement)?;
        let mut out = Vec::new();
        for e in doc.descendants() {
            if e.local_name != "headerReference" && e.local_name != "footerReference" {
                continue;
            }
            let kind = if e.local_name == "headerReference" {
                "header"
            } else {
                "footer"
            }
            .to_string();
            let ty = e
                .get_attribute_qname("w:type")
                .or_else(|| e.get_attribute("type"))
                .unwrap_or("default")
                .to_string();
            let rid = e
                .get_attribute_qname("r:id")
                .or_else(|| e.get_attribute("id"))
                .unwrap_or("")
                .to_string();
            out.push((kind, ty, rid));
        }
        Ok(out)
    }

    /// Whether any sectPr header/footer references exist.
    pub fn has_sect_pr_references(&mut self) -> Result<bool> {
        Ok(!self.list_sect_pr_references()?.is_empty())
    }

    /// Remove a header part by relationship id and strip matching sectPr references.
    pub fn remove_header_by_id(&mut self, rid: &str) -> Result<bool> {
        self.remove_header_or_footer_by_id(rel::HEADER, rid)
    }

    /// Remove a footer part by relationship id and strip matching sectPr references.
    pub fn remove_footer_by_id(&mut self, rid: &str) -> Result<bool> {
        self.remove_header_or_footer_by_id(rel::FOOTER, rid)
    }

    fn remove_header_or_footer_by_id(
        &mut self,
        relationship_type: &str,
        rid: &str,
    ) -> Result<bool> {
        let main = self
            .main_document_part
            .as_ref()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let main_uri = main.part().uri.clone();
        let target = {
            let Some(rels) = self.package.opc().part_relationships(&main_uri) else {
                return Ok(false);
            };
            let Some(rel) = rels.get(rid) else {
                return Ok(false);
            };
            if rel.relationship_type != relationship_type
                && !rel.relationship_type.contains(
                    if relationship_type == rel::HEADER {
                        "header"
                    } else {
                        "footer"
                    },
                )
            {
                return Ok(false);
            }
            rel.target.clone()
        };
        let uri = if target.starts_with('/') {
            PackUri::new(target)
        } else {
            PackUri::new(format!("/word/{}", target.trim_start_matches("./")))
        };
        let _ = self
            .package
            .opc_mut()
            .part_relationships_mut(&main_uri)
            .remove(rid);
        self.package.opc_mut().remove_part(&uri);
        let _ = self.remove_sect_pr_reference_by_id(rid)?;
        Ok(true)
    }

    fn clear_main_related_part(&mut self, uri_str: &str, relationship_type: &str) -> Result<bool> {
        let uri = PackUri::new(uri_str);
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(relationship_type)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    fn list_related_parts(&self, relationship_type: &str) -> Vec<PackUri> {
        self.list_related_part_relationships(relationship_type)
            .into_iter()
            .map(|(_, u)| u)
            .collect()
    }

    fn list_related_part_relationships(
        &self,
        relationship_type: &str,
    ) -> Vec<(String, PackUri)> {
        let Some(main) = self.main_document_part.as_ref() else {
            return Vec::new();
        };
        let main_uri = main.part().uri.clone();
        let Some(rels) = self.package.opc().part_relationships(&main_uri) else {
            return Vec::new();
        };
        rels.find_all_by_type(relationship_type)
            .into_iter()
            .map(|r| {
                let uri = if r.target.starts_with('/') {
                    PackUri::new(r.target.clone())
                } else {
                    let t = r.target.trim_start_matches("./");
                    PackUri::new(format!("/word/{t}"))
                };
                (r.id.clone(), uri)
            })
            .collect()
    }

    /// Whether a comments part is related from the main document.
    pub fn has_comments(&self) -> bool {
        !self.list_related_parts(rel::COMMENTS).is_empty()
            || self
                .package
                .opc()
                .has_part(&PackUri::new("/word/comments.xml"))
    }

    /// Count `w:comment` entries in the comments part (0 if missing).
    pub fn comment_count(&self) -> Result<usize> {
        let uri = PackUri::new("/word/comments.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root.children_by_name("comment").count())
    }

    /// List comments as `(id, author, text)` triples.
    pub fn list_comments(&self) -> Result<Vec<(String, String, String)>> {
        let uri = PackUri::new("/word/comments.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("comment")
            .map(|c| {
                let id = c
                    .get_attribute_qname("w:id")
                    .or_else(|| c.get_attribute("id"))
                    .unwrap_or("")
                    .to_string();
                let author = c
                    .get_attribute_qname("w:author")
                    .or_else(|| c.get_attribute("author"))
                    .unwrap_or("")
                    .to_string();
                let text = c
                    .descendants()
                    .filter(|e| e.local_name == "t")
                    .filter_map(|e| e.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("");
                (id, author, text)
            })
            .collect())
    }

    /// Update comment author / initials / date by id. Returns whether found.
    pub fn set_comment_attrs(
        &mut self,
        id: &str,
        author: Option<&str>,
        initials: Option<&str>,
        date: Option<&str>,
    ) -> Result<bool> {
        let uri = PackUri::new("/word/comments.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for c in root.children.iter_mut().filter(|c| c.local_name == "comment") {
            let cid = c
                .get_attribute_qname("w:id")
                .or_else(|| c.get_attribute("id"))
                .unwrap_or("");
            if cid != id {
                continue;
            }
            found = true;
            if let Some(a) = author {
                c.set_attribute_qname("w:author", a);
            }
            if let Some(i) = initials {
                c.set_attribute_qname("w:initials", i);
            }
            if let Some(d) = date {
                c.set_attribute_qname("w:date", d);
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_COMMENTS, xml);
        }
        Ok(found)
    }

    /// Clear author/initials/date attributes on a comment (keeps id).
    pub fn clear_comment_attrs(&mut self, id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/comments.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for c in root.children.iter_mut() {
            if c.local_name != "comment" {
                continue;
            }
            let cid = c
                .get_attribute_qname("w:id")
                .or_else(|| c.get_attribute("id"))
                .unwrap_or("");
            if cid != id {
                continue;
            }
            let before = c.attributes.len();
            c.attributes.retain(|a| {
                !matches!(a.local_name.as_str(), "author" | "initials" | "date")
            });
            found = c.attributes.len() < before;
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_COMMENTS, xml);
        }
        Ok(found)
    }

    /// Remove a single comment by id. Returns whether found.
    pub fn remove_comment(&mut self, id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/comments.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| {
            if c.local_name != "comment" {
                return true;
            }
            let cid = c
                .get_attribute_qname("w:id")
                .or_else(|| c.get_attribute("id"))
                .unwrap_or("");
            cid != id
        });
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_COMMENTS, xml);
        }
        Ok(removed)
    }

    /// Get a comment by id as `(author, text)`.
    pub fn comment_by_id(&self, id: &str) -> Result<Option<(String, String)>> {
        Ok(self
            .list_comments()?
            .into_iter()
            .find(|(cid, _, _)| cid == id)
            .map(|(_, a, t)| (a, t)))
    }

    /// Remove the comments part and its relationship. Returns whether it existed.
    pub fn clear_comments(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/comments.xml");
        let had = self.package.opc().has_part(&uri);
        if !had {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::COMMENTS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// List styles as `(style_id, name, type)` triples from `/word/styles.xml`.
    pub fn list_styles(&self) -> Result<Vec<(String, String, String)>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("style")
            .map(|s| {
                let id = s
                    .get_attribute_qname("w:styleId")
                    .or_else(|| s.get_attribute("styleId"))
                    .unwrap_or("")
                    .to_string();
                let ty = s
                    .get_attribute_qname("w:type")
                    .or_else(|| s.get_attribute("type"))
                    .unwrap_or("paragraph")
                    .to_string();
                let name = s
                    .child("name")
                    .and_then(|n| {
                        n.get_attribute_qname("w:val")
                            .or_else(|| n.get_attribute("val"))
                    })
                    .unwrap_or("")
                    .to_string();
                (id, name, ty)
            })
            .collect())
    }

    /// List style ids only (from `/word/styles.xml`).
    pub fn list_style_ids(&self) -> Result<Vec<String>> {
        Ok(self
            .list_styles()?
            .into_iter()
            .map(|(id, _, _)| id)
            .filter(|id| !id.is_empty())
            .collect())
    }

    /// List styles filtered by `w:type` (`paragraph`, `character`, `table`, `numbering`).
    /// Collect style ids actually referenced in the main document body (`w:pStyle` / `w:rStyle` / `w:tblStyle`).
    pub fn list_used_style_ids(&mut self) -> Result<Vec<String>> {
        let body = self.body_mut()?;
        let mut out = Vec::new();
        for e in body.descendants() {
            if matches!(e.local_name.as_str(), "pStyle" | "rStyle" | "tblStyle") {
                if let Some(v) = e
                    .get_attribute_qname("w:val")
                    .or_else(|| e.get_attribute("val"))
                {
                    if !v.is_empty() && !out.iter().any(|s| s == v) {
                        out.push(v.to_string());
                    }
                }
            }
        }
        Ok(out)
    }

    /// Style ids defined in styles.xml but not referenced in the body.
    pub fn list_unused_style_ids(&mut self) -> Result<Vec<String>> {
        let used = self.list_used_style_ids()?;
        let all = self.list_style_ids()?;
        Ok(all
            .into_iter()
            .filter(|id| !used.iter().any(|u| u == id))
            .collect())
    }

    /// Remove styles not referenced in the body. Returns how many styles were removed.
    ///
    /// Keeps the default style (`Normal` / styles marked default) when present.
    pub fn remove_unused_styles(&mut self) -> Result<usize> {
        let unused = self.list_unused_style_ids()?;
        let mut n = 0usize;
        for id in unused {
            if id == "Normal" || id.eq_ignore_ascii_case("normal") {
                continue;
            }
            if self.style_is_default(&id).unwrap_or(false) {
                continue;
            }
            if self.remove_style(&id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn list_styles_by_type(&self, style_type: &str) -> Result<Vec<(String, String)>> {
        Ok(self
            .list_styles()?
            .into_iter()
            .filter(|(_, ty, _)| ty == style_type)
            .map(|(id, _, name)| (id, name))
            .collect())
    }

    /// Count styles of a given type.
    pub fn style_count_by_type(&self, style_type: &str) -> Result<usize> {
        Ok(self.list_styles_by_type(style_type)?.len())
    }

    /// List paragraph styles as `(styleId, name)`.
    pub fn list_paragraph_styles(&self) -> Result<Vec<(String, String)>> {
        self.list_styles_by_type("paragraph")
    }

    /// List character styles as `(styleId, name)`.
    pub fn list_character_styles(&self) -> Result<Vec<(String, String)>> {
        self.list_styles_by_type("character")
    }

    /// List table styles as `(styleId, name)`.
    pub fn list_table_styles(&self) -> Result<Vec<(String, String)>> {
        self.list_styles_by_type("table")
    }

    /// List numbering styles as `(styleId, name)`.
    pub fn list_numbering_styles(&self) -> Result<Vec<(String, String)>> {
        self.list_styles_by_type("numbering")
    }

    /// Number of styles declared in `/word/styles.xml`.
    pub fn style_count(&self) -> Result<usize> {
        Ok(self.list_styles()?.len())
    }

    /// Remove a style by styleId. Returns whether found.
    pub fn remove_style(&mut self, style_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| {
            if c.local_name != "style" {
                return true;
            }
            let id = c
                .get_attribute_qname("w:styleId")
                .or_else(|| c.get_attribute("styleId"))
                .unwrap_or("");
            id != style_id
        });
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(removed)
    }

    /// Whether a style id exists.
    pub fn has_style(&self, style_id: &str) -> Result<bool> {
        Ok(self.list_style_ids()?.iter().any(|id| id == style_id))
    }

    /// List style ids marked as default (`w:default="1"`).
    pub fn list_default_style_ids(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("style")
            .filter(|c| {
                let d = c
                    .get_attribute_qname("w:default")
                    .or_else(|| c.get_attribute("default"));
                d == Some("1") || d.map(|s| s.eq_ignore_ascii_case("true")).unwrap_or(false)
            })
            .filter_map(|c| {
                c.get_attribute_qname("w:styleId")
                    .or_else(|| c.get_attribute("styleId"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Whether any default styles are marked.
    pub fn has_default_styles(&self) -> Result<bool> {
        Ok(!self.list_default_style_ids()?.is_empty())
    }

    /// Whether a style is marked default (`w:default="1"`).
    pub fn style_is_default(&self, style_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        for c in root.children_by_name("style") {
            let id = c
                .get_attribute_qname("w:styleId")
                .or_else(|| c.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            let d = c
                .get_attribute_qname("w:default")
                .or_else(|| c.get_attribute("default"));
            return Ok(d == Some("1") || d.map(|s| s.eq_ignore_ascii_case("true")).unwrap_or(false));
        }
        Ok(false)
    }

    /// Whether a style has `w:qFormat` (quick style).
    pub fn style_q_format(&self, style_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        for c in root.children_by_name("style") {
            let id = c
                .get_attribute_qname("w:styleId")
                .or_else(|| c.get_attribute("styleId"))
                .unwrap_or("");
            if id == style_id {
                return Ok(c.child("qFormat").is_some());
            }
        }
        Ok(false)
    }

    /// List style ids that have `w:qFormat`.
    pub fn list_qformat_styles(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("style")
            .filter(|c| c.child("qFormat").is_some())
            .filter_map(|c| {
                c.get_attribute_qname("w:styleId")
                    .or_else(|| c.get_attribute("styleId"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Whether any styles have `w:qFormat`.
    pub fn has_qformat_styles(&self) -> Result<bool> {
        Ok(!self.list_qformat_styles()?.is_empty())
    }

    /// Mark a style as default for its type (clears default on siblings of same type).
    pub fn set_style_default(&mut self, style_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        // Find type of target
        let mut target_type: Option<String> = None;
        for c in root.children_by_name("style") {
            let id = c
                .get_attribute_qname("w:styleId")
                .or_else(|| c.get_attribute("styleId"))
                .unwrap_or("");
            if id == style_id {
                target_type = Some(
                    c.get_attribute_qname("w:type")
                        .or_else(|| c.get_attribute("type"))
                        .unwrap_or("paragraph")
                        .to_string(),
                );
                break;
            }
        }
        let Some(ty) = target_type else {
            return Ok(false);
        };
        let mut found = false;
        for c in root.children.iter_mut() {
            if c.local_name != "style" {
                continue;
            }
            let id = c
                .get_attribute_qname("w:styleId")
                .or_else(|| c.get_attribute("styleId"))
                .unwrap_or("")
                .to_string();
            let ctype = c
                .get_attribute_qname("w:type")
                .or_else(|| c.get_attribute("type"))
                .unwrap_or("paragraph")
                .to_string();
            if ctype != ty {
                continue;
            }
            if id == style_id {
                c.set_attribute_qname("w:default", "1");
                found = true;
            } else {
                c.remove_attribute("default");
                // also remove qname form if present
                c.attributes.retain(|a| !(a.local_name == "default"));
            }
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(found)
    }

    /// Clear `w:default="1"` from a style (no longer the type default).
    pub fn clear_style_default(&mut self, style_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for s in root.children.iter_mut() {
            if s.local_name != "style" {
                continue;
            }
            let id = s
                .get_attribute_qname("w:styleId")
                .or_else(|| s.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            if s.get_attribute_qname("w:default").is_some()
                || s.get_attribute("default").is_some()
            {
                s.attributes.retain(|a| a.local_name != "default");
                found = true;
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(found)
    }

    /// Rename a style's `styleId` (and matching `name` val when it equals the old id).
    pub fn rename_style(&mut self, old_id: &str, new_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for c in root.children.iter_mut() {
            if c.local_name != "style" {
                continue;
            }
            let id = c
                .get_attribute_qname("w:styleId")
                .or_else(|| c.get_attribute("styleId"))
                .unwrap_or("");
            if id != old_id {
                continue;
            }
            c.set_attribute_qname("w:styleId", new_id);
            if let Some(name) = c.child_mut("name") {
                let val = name
                    .get_attribute_qname("w:val")
                    .or_else(|| name.get_attribute("val"));
                if val == Some(old_id) {
                    name.set_attribute_qname("w:val", new_id);
                }
            }
            found = true;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(found)
    }

    /// Read style linkage as `(based_on?, next?, link?)` for a style id.
    pub fn style_links(
        &self,
        style_id: &str,
    ) -> Result<Option<(Option<String>, Option<String>, Option<String>)>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        for s in root.children_by_name("style") {
            let id = s
                .get_attribute_qname("w:styleId")
                .or_else(|| s.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            let child_val = |local: &str| {
                s.child(local).and_then(|c| {
                    c.get_attribute_qname("w:val")
                        .or_else(|| c.get_attribute("val"))
                        .map(|x| x.to_string())
                })
            };
            return Ok(Some((
                child_val("basedOn"),
                child_val("next"),
                child_val("link"),
            )));
        }
        Ok(None)
    }

    /// Set style `basedOn` / `next` / `link` children on an existing style.
    /// List style ids that are based on `base_id` (`w:basedOn`).
    pub fn list_styles_based_on(&self, base_id: &str) -> Result<Vec<String>> {
        let all = self.list_styles()?;
        let mut out = Vec::new();
        for (id, _, _) in all {
            if let Some((based, _, _)) = self.style_links(&id)? {
                if based.as_deref() == Some(base_id) {
                    out.push(id);
                }
            }
        }
        Ok(out)
    }

    /// Whether any styles are based on `base_id`.
    pub fn has_styles_based_on(&self, base_id: &str) -> Result<bool> {
        Ok(!self.list_styles_based_on(base_id)?.is_empty())
    }

    /// List style ids whose `w:next` points to `next_id`.
    pub fn list_styles_with_next(&self, next_id: &str) -> Result<Vec<String>> {
        let all = self.list_styles()?;
        let mut out = Vec::new();
        for (id, _, _) in all {
            if let Some((_, next, _)) = self.style_links(&id)? {
                if next.as_deref() == Some(next_id) {
                    out.push(id);
                }
            }
        }
        Ok(out)
    }

    /// Whether any styles have `w:next` pointing to `next_id`.
    pub fn has_styles_with_next(&self, next_id: &str) -> Result<bool> {
        Ok(!self.list_styles_with_next(next_id)?.is_empty())
    }

    /// List `(styleId, linkId)` for styles that declare `w:link`.
    pub fn list_linked_styles(&self) -> Result<Vec<(String, String)>> {
        let all = self.list_styles()?;
        let mut out = Vec::new();
        for (id, _, _) in all {
            if let Some((_, _, link)) = self.style_links(&id)? {
                if let Some(l) = link {
                    out.push((id, l));
                }
            }
        }
        Ok(out)
    }

    /// Whether any styles declare `w:link`.
    pub fn has_linked_styles(&self) -> Result<bool> {
        Ok(!self.list_linked_styles()?.is_empty())
    }

    pub fn set_style_links(
        &mut self,
        style_id: &str,
        based_on: Option<&str>,
        next: Option<&str>,
        link: Option<&str>,
    ) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for s in root.children.iter_mut().filter(|c| c.local_name == "style") {
            let id = s
                .get_attribute_qname("w:styleId")
                .or_else(|| s.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            found = true;
            let upsert = |el: &mut OpenXmlElement, local: &str, val: &str| {
                if let Some(c) = el.child_mut(local) {
                    c.set_attribute_qname("w:val", val);
                } else {
                    el.append_child(
                        OpenXmlElement::w(local).with_attribute_qname("w:val", val),
                    );
                }
            };
            if let Some(v) = based_on {
                upsert(s, "basedOn", v);
            }
            if let Some(v) = next {
                upsert(s, "next", v);
            }
            if let Some(v) = link {
                upsert(s, "link", v);
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(found)
    }

    /// Clear style basedOn/next/link children on a style.
    pub fn clear_style_links(&mut self, style_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for s in root.children.iter_mut() {
            if s.local_name != "style" {
                continue;
            }
            let id = s
                .get_attribute_qname("w:styleId")
                .or_else(|| s.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            let before = s.children.len();
            s.children.retain(|c| {
                !matches!(c.local_name.as_str(), "basedOn" | "next" | "link")
            });
            found = s.children.len() < before;
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(found)
    }

    /// Set style OnOff flags and `uiPriority` on an existing style.
    ///
    /// Flags: `qFormat`, `semiHidden`, `unhideWhenUsed`, `locked`, `personal`, `personalCompose`,
    /// `personalReply`. Pass `None` to leave a flag unchanged; `Some(false)` removes the child.
    pub fn set_style_flags(
        &mut self,
        style_id: &str,
        q_format: Option<bool>,
        semi_hidden: Option<bool>,
        unhide_when_used: Option<bool>,
        locked: Option<bool>,
        ui_priority: Option<u32>,
    ) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for s in root.children.iter_mut().filter(|c| c.local_name == "style") {
            let id = s
                .get_attribute_qname("w:styleId")
                .or_else(|| s.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            found = true;
            let set_onoff = |el: &mut OpenXmlElement, local: &str, enabled: bool| {
                el.children.retain(|c| c.local_name != local);
                if enabled {
                    el.append_child(OpenXmlElement::w(local));
                }
            };
            if let Some(v) = q_format {
                set_onoff(s, "qFormat", v);
            }
            if let Some(v) = semi_hidden {
                set_onoff(s, "semiHidden", v);
            }
            if let Some(v) = unhide_when_used {
                set_onoff(s, "unhideWhenUsed", v);
            }
            if let Some(v) = locked {
                set_onoff(s, "locked", v);
            }
            if let Some(p) = ui_priority {
                if let Some(c) = s.child_mut("uiPriority") {
                    c.set_attribute_qname("w:val", p.to_string());
                } else {
                    s.append_child(
                        OpenXmlElement::w("uiPriority")
                            .with_attribute_qname("w:val", p.to_string()),
                    );
                }
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(found)
    }

    /// Clear style UI flags (qFormat/semiHidden/unhideWhenUsed/locked/uiPriority) on a style.
    pub fn clear_style_flags(&mut self, style_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for s in root.children.iter_mut() {
            if s.local_name != "style" {
                continue;
            }
            let id = s
                .get_attribute_qname("w:styleId")
                .or_else(|| s.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            let before = s.children.len();
            s.children.retain(|c| {
                !matches!(
                    c.local_name.as_str(),
                    "qFormat" | "semiHidden" | "unhideWhenUsed" | "locked" | "uiPriority"
                )
            });
            found = s.children.len() < before;
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_STYLES, xml);
        }
        Ok(found)
    }

    /// Read style flags as `(qFormat, semiHidden, unhideWhenUsed, locked, uiPriority?)`.
    pub fn style_flags(
        &self,
        style_id: &str,
    ) -> Result<Option<(bool, bool, bool, bool, Option<u32>)>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        for s in root.children_by_name("style") {
            let id = s
                .get_attribute_qname("w:styleId")
                .or_else(|| s.get_attribute("styleId"))
                .unwrap_or("");
            if id != style_id {
                continue;
            }
            let has = |local: &str| s.child(local).is_some();
            let prio = s.child("uiPriority").and_then(|c| {
                c.get_attribute_qname("w:val")
                    .or_else(|| c.get_attribute("val"))
                    .and_then(|v| v.parse().ok())
            });
            return Ok(Some((
                has("qFormat"),
                has("semiHidden"),
                has("unhideWhenUsed"),
                has("locked"),
                prio,
            )));
        }
        Ok(None)
    }

    /// Number of fonts declared in the font table.
    /// List style ids that are locked (`w:locked`).
    pub fn list_locked_styles(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("style")
            .filter(|c| c.child("locked").is_some())
            .filter_map(|c| {
                c.get_attribute_qname("w:styleId")
                    .or_else(|| c.get_attribute("styleId"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Whether any styles are locked.
    pub fn has_locked_styles(&self) -> Result<bool> {
        Ok(!self.list_locked_styles()?.is_empty())
    }

    /// List style ids that are semi-hidden (`w:semiHidden`).
    pub fn list_semi_hidden_styles(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("style")
            .filter(|c| c.child("semiHidden").is_some())
            .filter_map(|c| {
                c.get_attribute_qname("w:styleId")
                    .or_else(|| c.get_attribute("styleId"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Whether any styles are semi-hidden.
    pub fn has_semi_hidden_styles(&self) -> Result<bool> {
        Ok(!self.list_semi_hidden_styles()?.is_empty())
    }

    /// List style ids with `w:unhideWhenUsed`.
    pub fn list_unhide_when_used_styles(&self) -> Result<Vec<String>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("style")
            .filter(|c| c.child("unhideWhenUsed").is_some())
            .filter_map(|c| {
                c.get_attribute_qname("w:styleId")
                    .or_else(|| c.get_attribute("styleId"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Whether any styles have `w:unhideWhenUsed`.
    pub fn has_unhide_when_used_styles(&self) -> Result<bool> {
        Ok(!self.list_unhide_when_used_styles()?.is_empty())
    }

    /// List `(styleId, uiPriority)` for styles that declare `w:uiPriority`.
    pub fn list_styles_with_ui_priority(&self) -> Result<Vec<(String, u32)>> {
        let uri = PackUri::new("/word/styles.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("style")
            .filter_map(|c| {
                let id = c
                    .get_attribute_qname("w:styleId")
                    .or_else(|| c.get_attribute("styleId"))?
                    .to_string();
                let prio = c.child("uiPriority").and_then(|u| {
                    u.get_attribute_qname("w:val")
                        .or_else(|| u.get_attribute("val"))
                        .and_then(|s| s.parse().ok())
                })?;
                Some((id, prio))
            })
            .collect())
    }

    /// Whether any styles declare `w:uiPriority`.
    pub fn has_styles_with_ui_priority(&self) -> Result<bool> {
        Ok(!self.list_styles_with_ui_priority()?.is_empty())
    }

    pub fn font_count(&self) -> Result<usize> {
        Ok(self.list_font_names()?.len())
    }

    /// List user footnotes as `(id, text)` pairs (excludes separator ids -1/0).
    pub fn list_footnotes(&self) -> Result<Vec<(String, String)>> {
        let uri = PackUri::new("/word/footnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("footnote")
            .filter_map(|f| {
                let id = f
                    .get_attribute_qname("w:id")
                    .or_else(|| f.get_attribute("id"))
                    .unwrap_or("")
                    .to_string();
                if id == "-1" || id == "0" {
                    return None;
                }
                let text = f
                    .descendants()
                    .filter(|e| e.local_name == "t")
                    .filter_map(|e| e.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("");
                Some((id, text))
            })
            .collect())
    }

    /// List user endnotes as `(id, text)` pairs (excludes separator ids -1/0).
    pub fn list_endnotes(&self) -> Result<Vec<(String, String)>> {
        let uri = PackUri::new("/word/endnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("endnote")
            .filter_map(|f| {
                let id = f
                    .get_attribute_qname("w:id")
                    .or_else(|| f.get_attribute("id"))
                    .unwrap_or("")
                    .to_string();
                if id == "-1" || id == "0" {
                    return None;
                }
                let text = f
                    .descendants()
                    .filter(|e| e.local_name == "t")
                    .filter_map(|e| e.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("");
                Some((id, text))
            })
            .collect())
    }

    /// Whether a footnotes part is present.
    pub fn has_footnotes(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/footnotes.xml"))
    }

    /// Read footnote body text by id.
    pub fn footnote_text(&self, id: &str) -> Result<Option<String>> {
        Ok(self
            .list_footnotes()?
            .into_iter()
            .find(|(i, _)| i == id)
            .map(|(_, t)| t))
    }

    /// Whether a footnote with the given id exists.
    pub fn has_footnote(&self, id: &str) -> Result<bool> {
        Ok(self.list_footnotes()?.iter().any(|(i, _)| i == id))
    }

    /// Read endnote body text by id.
    pub fn endnote_text(&self, id: &str) -> Result<Option<String>> {
        Ok(self
            .list_endnotes()?
            .into_iter()
            .find(|(i, _)| i == id)
            .map(|(_, t)| t))
    }

    /// Whether an endnote with the given id exists.
    pub fn has_endnote(&self, id: &str) -> Result<bool> {
        Ok(self.list_endnotes()?.iter().any(|(i, _)| i == id))
    }

    /// Update the body text of a user footnote by id. Returns whether found.
    pub fn set_footnote_text(&mut self, id: &str, body_text: &str) -> Result<bool> {
        let uri = PackUri::new("/word/footnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for f in root.children.iter_mut().filter(|c| c.local_name == "footnote") {
            let fid = f
                .get_attribute_qname("w:id")
                .or_else(|| f.get_attribute("id"))
                .unwrap_or("");
            if fid != id {
                continue;
            }
            found = true;
            // Replace first paragraph text
            if let Some(p) = f.child_mut("p") {
                // clear runs and set single run text
                p.children.retain(|c| c.local_name != "r");
                p.append_child(run(vec![text(body_text)]));
            } else {
                f.children.clear();
                f.append_child(paragraph(vec![run(vec![text(body_text)])]));
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_FOOTNOTES, xml);
        }
        Ok(found)
    }

    /// Update the body text of a user endnote by id. Returns whether found.
    pub fn set_endnote_text(&mut self, id: &str, body_text: &str) -> Result<bool> {
        let uri = PackUri::new("/word/endnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for f in root.children.iter_mut().filter(|c| c.local_name == "endnote") {
            let eid = f
                .get_attribute_qname("w:id")
                .or_else(|| f.get_attribute("id"))
                .unwrap_or("");
            if eid != id {
                continue;
            }
            found = true;
            if let Some(p) = f.child_mut("p") {
                p.children.retain(|c| c.local_name != "r");
                p.append_child(run(vec![text(body_text)]));
            } else {
                f.children.clear();
                f.append_child(paragraph(vec![run(vec![text(body_text)])]));
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_ENDNOTES, xml);
        }
        Ok(found)
    }

    /// Remove a user footnote by id. Returns whether found.
    pub fn remove_footnote(&mut self, id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/footnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| {
            if c.local_name != "footnote" {
                return true;
            }
            let fid = c
                .get_attribute_qname("w:id")
                .or_else(|| c.get_attribute("id"))
                .unwrap_or("");
            fid != id
        });
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_FOOTNOTES, xml);
        }
        Ok(removed)
    }

    /// Remove a user endnote by id. Returns whether found.
    pub fn remove_endnote(&mut self, id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/endnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| {
            if c.local_name != "endnote" {
                return true;
            }
            let eid = c
                .get_attribute_qname("w:id")
                .or_else(|| c.get_attribute("id"))
                .unwrap_or("");
            eid != id
        });
        let removed = root.children.len() < before;
        if removed {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_ENDNOTES, xml);
        }
        Ok(removed)
    }

    /// Remove footnotes part and relationship.
    pub fn clear_footnotes(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/footnotes.xml", rel::FOOTNOTES)
    }

    /// Count user footnotes (excludes separator ids -1/0).
    pub fn footnote_count(&self) -> Result<usize> {
        let uri = PackUri::new("/word/footnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("footnote")
            .filter(|f| {
                let id = f
                    .get_attribute("id")
                    .or_else(|| {
                        f.attributes
                            .iter()
                            .find(|a| a.local_name == "id")
                            .map(|a| a.value.as_str())
                    })
                    .unwrap_or("");
                id != "-1" && id != "0"
            })
            .count())
    }

    /// Whether an endnotes part is present.
    pub fn has_endnotes(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/endnotes.xml"))
    }

    /// Remove endnotes part and relationship.

    pub fn clear_endnotes(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/endnotes.xml", rel::ENDNOTES)
    }

    /// Count user endnotes (excludes separator ids -1/0).
    /// Clear both footnotes and endnotes parts. Returns whether either was present.
    pub fn clear_all_notes(&mut self) -> Result<bool> {
        let a = self.clear_footnotes()?;
        let b = self.clear_endnotes()?;
        Ok(a || b)
    }

    /// Whether footnotes or endnotes parts exist.
    pub fn has_notes(&self) -> bool {
        self.has_footnotes() || self.has_endnotes()
    }

    pub fn endnote_count(&self) -> Result<usize> {
        let uri = PackUri::new("/word/endnotes.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("endnote")
            .filter(|f| {
                let id = f
                    .get_attribute("id")
                    .or_else(|| {
                        f.attributes
                            .iter()
                            .find(|a| a.local_name == "id")
                            .map(|a| a.value.as_str())
                    })
                    .unwrap_or("");
                id != "-1" && id != "0"
            })
            .count())
    }

    /// Whether a numbering part is present.
    pub fn has_numbering(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/numbering.xml"))
    }

    /// List abstract numbering definitions as `(abstract_num_id, multi_level_type?)`.
    pub fn list_abstract_nums(&self) -> Result<Vec<(u32, Option<String>)>> {
        let uri = PackUri::new("/word/numbering.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("abstractNum")
            .map(|a| {
                let id = a
                    .get_attribute_qname("w:abstractNumId")
                    .or_else(|| a.get_attribute("abstractNumId"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let mlt = a.child("multiLevelType").and_then(|c| {
                    c.get_attribute_qname("w:val")
                        .or_else(|| c.get_attribute("val"))
                        .map(|s| s.to_string())
                });
                (id, mlt)
            })
            .collect())
    }

    /// Number of abstract numbering definitions.
    pub fn abstract_num_count(&self) -> Result<usize> {
        Ok(self.list_abstract_nums()?.len())
    }

    /// Number of numbering instances.
    pub fn num_instance_count(&self) -> Result<usize> {
        Ok(self.list_num_instances()?.len())
    }


    /// Whether any abstract numbering definitions exist.
    pub fn has_abstract_nums(&self) -> Result<bool> {
        Ok(self.abstract_num_count()? > 0)
    }

    /// Whether any numbering instances exist.
    pub fn has_num_instances(&self) -> Result<bool> {
        Ok(self.num_instance_count()? > 0)
    }

    /// List numbering instances as `(num_id, abstract_num_id)`.
    pub fn list_num_instances(&self) -> Result<Vec<(u32, u32)>> {
        let uri = PackUri::new("/word/numbering.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .children_by_name("num")
            .map(|n| {
                let num_id = n
                    .get_attribute_qname("w:numId")
                    .or_else(|| n.get_attribute("numId"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let abstract_id = n
                    .child("abstractNumId")
                    .and_then(|c| {
                        c.get_attribute_qname("w:val")
                            .or_else(|| c.get_attribute("val"))
                    })
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                (num_id, abstract_id)
            })
            .collect())
    }

    /// Remove a numbering instance (`w:num`) by `numId`. Returns whether found.
    pub fn remove_num_instance(&mut self, num_id: u32) -> Result<bool> {
        let uri = PackUri::new("/word/numbering.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        let id_str = num_id.to_string();
        root.children.retain(|n| {
            if n.local_name != "num" {
                return true;
            }
            let nid = n
                .get_attribute_qname("w:numId")
                .or_else(|| n.get_attribute("numId"));
            nid != Some(id_str.as_str())
        });
        if root.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::WORD_NUMBERING, xml);
        Ok(true)
    }

    /// Remove an abstract numbering definition by `abstractNumId`. Returns whether found.
    ///
    /// Does not automatically remove `w:num` instances that reference it.
    pub fn remove_abstract_num(&mut self, abstract_num_id: u32) -> Result<bool> {
        let uri = PackUri::new("/word/numbering.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        let id_str = abstract_num_id.to_string();
        root.children.retain(|n| {
            if n.local_name != "abstractNum" {
                return true;
            }
            let aid = n
                .get_attribute_qname("w:abstractNumId")
                .or_else(|| n.get_attribute("abstractNumId"));
            aid != Some(id_str.as_str())
        });
        if root.children.len() == before {
            return Ok(false);
        }
        let xml = crate::element::write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::WORD_NUMBERING, xml);
        Ok(true)
    }

    /// Read level text for an abstract num level (`lvlText`).
    pub fn abstract_num_level_text(
        &self,
        abstract_num_id: u32,
        ilvl: u32,
    ) -> Result<Option<String>> {
        let uri = PackUri::new("/word/numbering.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        for a in root.children_by_name("abstractNum") {
            let id = a
                .get_attribute_qname("w:abstractNumId")
                .or_else(|| a.get_attribute("abstractNumId"))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            if id != abstract_num_id {
                continue;
            }
            for lvl in a.children_by_name("lvl") {
                let level = lvl
                    .get_attribute_qname("w:ilvl")
                    .or_else(|| lvl.get_attribute("ilvl"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                if level != ilvl {
                    continue;
                }
                return Ok(lvl.child("lvlText").and_then(|t| {
                    t.get_attribute_qname("w:val")
                        .or_else(|| t.get_attribute("val"))
                        .map(|s| s.to_string())
                }));
            }
        }
        Ok(None)
    }

    /// Set level text / numFmt on an abstract numbering level.
    pub fn set_abstract_num_level(
        &mut self,
        abstract_num_id: u32,
        ilvl: u32,
        lvl_text: Option<&str>,
        num_fmt: Option<&str>,
        start: Option<u32>,
    ) -> Result<bool> {
        let uri = PackUri::new("/word/numbering.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for a in root
            .children
            .iter_mut()
            .filter(|c| c.local_name == "abstractNum")
        {
            let id = a
                .get_attribute_qname("w:abstractNumId")
                .or_else(|| a.get_attribute("abstractNumId"))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            if id != abstract_num_id {
                continue;
            }
            for lvl in a.children.iter_mut().filter(|c| c.local_name == "lvl") {
                let level = lvl
                    .get_attribute_qname("w:ilvl")
                    .or_else(|| lvl.get_attribute("ilvl"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                if level != ilvl {
                    continue;
                }
                found = true;
                if let Some(t) = lvl_text {
                    if let Some(el) = lvl.child_mut("lvlText") {
                        el.set_attribute_qname("w:val", t);
                    } else {
                        lvl.append_child(
                            OpenXmlElement::w("lvlText").with_attribute_qname("w:val", t),
                        );
                    }
                }
                if let Some(f) = num_fmt {
                    if let Some(el) = lvl.child_mut("numFmt") {
                        el.set_attribute_qname("w:val", f);
                    } else {
                        lvl.append_child(
                            OpenXmlElement::w("numFmt").with_attribute_qname("w:val", f),
                        );
                    }
                }
                if let Some(s) = start {
                    if let Some(el) = lvl.child_mut("start") {
                        el.set_attribute_qname("w:val", s.to_string());
                    } else {
                        lvl.append_child(
                            OpenXmlElement::w("start")
                                .with_attribute_qname("w:val", s.to_string()),
                        );
                    }
                }
                break;
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_NUMBERING, xml);
        }
        Ok(found)
    }

    /// Remove a numbering level (`w:lvl`) from an abstractNum.
    pub fn clear_abstract_num_level(
        &mut self,
        abstract_num_id: u32,
        ilvl: u32,
    ) -> Result<bool> {
        let uri = PackUri::new("/word/numbering.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for a in root.children.iter_mut().filter(|c| c.local_name == "abstractNum") {
            let id = a
                .get_attribute_qname("w:abstractNumId")
                .or_else(|| a.get_attribute("abstractNumId"))
                .and_then(|s| s.parse().ok());
            if id != Some(abstract_num_id) {
                continue;
            }
            let before = a.children.len();
            a.children.retain(|c| {
                if c.local_name != "lvl" {
                    return true;
                }
                let lvl = c
                    .get_attribute_qname("w:ilvl")
                    .or_else(|| c.get_attribute("ilvl"))
                    .and_then(|s| s.parse().ok());
                lvl != Some(ilvl)
            });
            found = a.children.len() < before;
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_NUMBERING, xml);
        }
        Ok(found)
    }

    /// Whether a people part is present.
    pub fn has_people(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/people.xml"))
    }

    /// Whether a Custom UI / Ribbon part is present.
    pub fn has_custom_ui(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/customUI/customUI.xml"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::CUSTOM_UI_2007)
                .is_some()
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::CUSTOM_UI)
                .is_some()
    }

    /// Whether a VBA data companion part exists.
    pub fn has_vba_data(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/vbaData.xml"))
    }

    /// Whether stylesWithEffects part exists.
    pub fn has_styles_with_effects(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/stylesWithEffects.xml"))
    }

    /// Whether commentsIds part exists.
    pub fn has_comments_ids(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/commentsIds.xml"))
    }

    /// Whether commentsExtensible part exists.
    pub fn has_comments_extensible(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/commentsExtensible.xml"))
    }

    /// Whether commentsExtended part exists.
    pub fn has_comments_extended(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/commentsExtended.xml"))
            || self
                .package
                .opc()
                .part_uris().into_iter().any(|u| u.as_str().contains("commentsExtended"))
    }

    /// Whether document tasks part exists.
    pub fn has_document_tasks(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("/word/tasks") || s.contains("documenttasks") || s.contains("documentTasks")
        })
    }

    /// Remove document tasks part.
    pub fn clear_document_tasks(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/tasks/tasks.xml", rel::DOCUMENT_TASKS)
    }

    /// Whether any web extension parts exist.
    pub fn has_web_extensions(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/word/webextensions/"))
    }

    /// Count web extension parts under `/word/webextensions/`.
    pub fn web_extension_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/webextensions/"))
            .count()
    }

    /// Remove web extension + taskpanes parts.
    pub fn clear_web_extensions(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/webextensions/"))
            
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        // Drop package-level and main-document relationships that point at webextensions
        for ty in [rel::WEB_EXTENSION, rel::WEB_EXTENSION_TASKPANES] {
            if let Some(id) = self
                .package
                .opc()
                .package_relationships()
                .get_by_type(ty)
                .map(|r| r.id.clone())
            {
                self.package.opc_mut().package_relationships_mut().remove(&id);
            }
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::WEB_EXTENSION
                                || r.relationship_type == rel::WEB_EXTENSION_TASKPANES
                                || r.target.contains("webextension")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Whether key-map customization part exists.
    pub fn has_customization(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/customizations/customization.xml"))
    }

    /// Remove key-map customization part.
    pub fn clear_customization(&mut self) -> Result<bool> {
        self.clear_main_related_part(
            "/word/customizations/customization.xml",
            rel::CUSTOMIZATION,
        )
    }

    /// Whether QAT customizations part exists.
    pub fn has_quick_access_toolbar(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/customUI/qatCustomizations.xml"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::QAT)
                .is_some()
    }

    /// Remove QAT customizations part and package relationship.
    pub fn clear_quick_access_toolbar(&mut self) -> Result<bool> {
        let uri = PackUri::new("/customUI/qatCustomizations.xml");
        let had_part = self.package.opc().has_part(&uri);
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::QAT)
            .is_some();
        if !had_part && !had_rel {
            return Ok(false);
        }
        if let Some(id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::QAT)
            .map(|r| r.id.clone())
        {
            self.package.opc_mut().package_relationships_mut().remove(&id);
        }
        if had_part {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Whether a sensitivity label info part exists.
    pub fn has_label_info(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docMetadata/LabelInfo.xml"))
    }

    /// Remove label info part and package relationship.
    pub fn clear_label_info(&mut self) -> Result<bool> {
        let uri = PackUri::new("/docMetadata/LabelInfo.xml");
        let had_part = self.package.opc().has_part(&uri);
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::LABEL_INFO)
            .is_some();
        if !had_part && !had_rel {
            return Ok(false);
        }
        if let Some(id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::LABEL_INFO)
            .map(|r| r.id.clone())
        {
            self.package.opc_mut().package_relationships_mut().remove(&id);
        }
        if had_part {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Whether attached toolbars binary part exists.
    pub fn has_attached_toolbars(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/attachedToolbars.bin"))
    }

    /// Remove attached toolbars part.
    pub fn clear_attached_toolbars(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/attachedToolbars.bin", rel::ATTACHED_TOOLBARS)
    }

    /// Whether any SmartArt/diagram parts exist.
    pub fn has_diagrams(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/word/diagrams/"))
    }

    /// Count diagram parts under `/word/diagrams/`.
    pub fn diagram_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/diagrams/"))
            .count()
    }


    /// List diagram part URIs under `/word/diagrams/`.
    pub fn list_diagrams(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/diagrams/"))
            
            .collect()
    }

    /// Remove all diagram parts and main-document diagram relationships.
    pub fn clear_diagrams(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/diagrams/"))
            
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::DIAGRAM_DATA
                                || r.relationship_type == rel::DIAGRAM_LAYOUT
                                || r.relationship_type == rel::DIAGRAM_COLORS
                                || r.relationship_type == rel::DIAGRAM_STYLE
                                || r.relationship_type == rel::DIAGRAM_PERSIST_LAYOUT
                                || r.target.contains("diagrams/")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        // Also clear rels between diagram parts
        for parent in &uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(parent)
                .map(|rels| rels.iter().map(|r| r.id.clone()).collect())
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(parent);
                for id in ids {
                    rels.remove(&id);
                }
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Whether any embedded package/object parts exist under `/word/embeddings/`.
    pub fn has_embeddings(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().starts_with("/word/embeddings/"))
    }

    /// Count embedding parts under `/word/embeddings/`.
    pub fn embedding_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/embeddings/"))
            .count()
    }

    /// List embedding part URIs.
    pub fn list_embeddings(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().starts_with("/word/embeddings/"))
            
            .collect()
    }

    /// Remove a single embedding part by URI and drop main-document relationships that target it.
    pub fn remove_embedding(&mut self, uri: &PackUri) -> Result<bool> {
        if !uri.as_str().starts_with("/word/embeddings/") {
            return Ok(false);
        }
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        let target = uri.as_str().to_string();
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            let t = r.target.as_str();
                            crate::opc::resolve_uri(&main_uri, t)
                                .map(|u| u.as_str() == target)
                                .unwrap_or(false)
                                || t == target
                                || t.ends_with(target.trim_start_matches('/'))
                                || target.ends_with(t.trim_start_matches("./"))
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Remove all embedding parts and related main-document relationships.
    pub fn clear_embeddings(&mut self) -> Result<usize> {
        let uris = self.list_embeddings();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.target.contains("embeddings/")
                                || r.relationship_type.contains("package")
                                || r.relationship_type.contains("oleObject")
                                || r.relationship_type.contains("embedded")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Remove Custom UI part and package relationship.
    pub fn clear_custom_ui(&mut self) -> Result<bool> {
        let uri = PackUri::new("/customUI/customUI.xml");
        let had_part = self.package.opc().has_part(&uri);
        let had_rel = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::CUSTOM_UI_2007)
            .is_some()
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::CUSTOM_UI)
                .is_some();
        if !had_part && !had_rel {
            return Ok(false);
        }
        for ty in [rel::CUSTOM_UI_2007, rel::CUSTOM_UI] {
            if let Some(id) = self
                .package
                .opc()
                .package_relationships()
                .get_by_type(ty)
                .map(|r| r.id.clone())
            {
                self.package.opc_mut().package_relationships_mut().remove(&id);
            }
        }
        if had_part {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Remove VBA data companion part.
    pub fn clear_vba_data(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/vbaData.xml", rel::VBA_DATA)
    }

    /// Remove stylesWithEffects part.
    pub fn clear_styles_with_effects(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/stylesWithEffects.xml", rel::STYLES_WITH_EFFECTS)
    }

    /// Remove commentsIds part.
    pub fn clear_comments_ids(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/commentsIds.xml", rel::COMMENTS_IDS)
    }

    /// Remove commentsExtensible part.
    pub fn clear_comments_extensible(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/commentsExtensible.xml", rel::COMMENTS_EXTENSIBLE)
    }

    /// Remove commentsExtended part.
    pub fn clear_comments_extended(&mut self) -> Result<bool> {
        self.clear_main_related_part("/word/commentsExtended.xml", rel::COMMENTS_EXTENDED)
    }

    /// Count people entries in `/word/people.xml`.
    pub fn people_count(&self) -> Result<usize> {
        let uri = PackUri::new("/word/people.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "person")
            .count())
    }

    /// List people as `(author, providerId)`.
    pub fn list_people(&self) -> Result<Vec<(String, String)>> {
        let uri = PackUri::new("/word/people.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "person")
            .map(|e| {
                let author = e
                    .get_attribute_qname("w15:author")
                    .or_else(|| e.get_attribute("author"))
                    .unwrap_or("")
                    .to_string();
                let provider = e
                    .get_attribute_qname("w15:providerId")
                    .or_else(|| e.get_attribute("providerId"))
                    .unwrap_or("")
                    .to_string();
                (author, provider)
            })
            .collect())
    }

    /// Whether a person with the given author name exists.
    /// Add or replace a person entry in `/word/people.xml`.
    pub fn add_person(&mut self, author: &str, provider_id: &str) -> Result<()> {
        let uri = PackUri::new("/word/people.xml");
        let mut root = if let Some(data) = self.package.opc().get_part(&uri) {
            parse_element(data)?
        } else {
            OpenXmlElement::new(
                "w15",
                "http://schemas.microsoft.com/office/word/2012/wordml",
                "people",
            )
            .with_ns_decl("w15", "http://schemas.microsoft.com/office/word/2012/wordml")
        };
        // Remove existing same author
        root.children.retain(|c| {
            if c.local_name != "person" {
                return true;
            }
            let a = c
                .get_attribute_qname("w15:author")
                .or_else(|| c.get_attribute("author"));
            a != Some(author)
        });
        root.append_child(
            OpenXmlElement::new(
                "w15",
                "http://schemas.microsoft.com/office/word/2012/wordml",
                "person",
            )
            .with_attribute_qname("w15:author", author)
            .with_attribute_qname("w15:providerId", provider_id),
        );
        let xml = crate::element::write_element(&root)?;
        let ct = "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml";
        self.package.set_part(uri.clone(), ct, xml);
        // Ensure relationship from main
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let has_rel = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter().any(|r| {
                        r.target.contains("people")
                            || r.relationship_type.contains("people")
                    })
                })
                .unwrap_or(false);
            if !has_rel {
                let _ = self.package.add_part_relationship(
                    &main_uri,
                    "http://schemas.microsoft.com/office/2011/relationships/people",
                    &uri,
                    crate::opc::RelationshipTargetMode::Internal,
                );
            }
        }
        Ok(())
    }

    pub fn has_person(&self, author: &str) -> Result<bool> {
        Ok(self
            .list_people()?
            .iter()
            .any(|(id, _)| id == author))
    }

    /// Remove a person by author name. Returns whether found.
    pub fn remove_person(&mut self, author: &str) -> Result<bool> {
        let uri = PackUri::new("/word/people.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, author: &str, found: &mut bool) {
            let before = el.children.len();
            el.children.retain(|c| {
                if c.local_name == "person" {
                    let a = c
                        .get_attribute_qname("w15:author")
                        .or_else(|| c.get_attribute("author"));
                    if a == Some(author) {
                        return false;
                    }
                }
                true
            });
            if el.children.len() < before {
                *found = true;
            }
            for c in el.children.iter_mut() {
                visit(c, author, found);
            }
        }
        visit(&mut root, author, &mut found);
        if found {
            let xml = crate::element::write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(uri, content_type::WORD_PEOPLE, xml);
        }
        Ok(found)
    }

    /// Update providerId for a person by author. Returns whether found.
    pub fn set_person_provider(&mut self, author: &str, provider_id: &str) -> Result<bool> {
        let uri = PackUri::new("/word/people.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, author: &str, provider: &str, found: &mut bool) {
            if el.local_name == "person" {
                let a = el
                    .get_attribute_qname("w15:author")
                    .or_else(|| el.get_attribute("author"));
                if a == Some(author) {
                    el.set_attribute_qname("w15:providerId", provider);
                    *found = true;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, author, provider, found);
            }
        }
        visit(&mut root, author, provider_id, &mut found);
        if found {
            let xml = crate::element::write_element(&root)?;
            let ct = "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml";
            self.package.set_part(uri, ct, xml);
        }
        Ok(found)
    }

    /// Clear providerId attribute on a person author.
    pub fn clear_person_provider(&mut self, author: &str) -> Result<bool> {
        let uri = PackUri::new("/word/people.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let mut found = false;
        for el in root.children.iter_mut() {
            if el.local_name != "person" {
                continue;
            }
            let a = el
                .get_attribute_qname("w15:author")
                .or_else(|| el.get_attribute("author"));
            if a != Some(author) {
                continue;
            }
            let before = el.attributes.len();
            el.attributes.retain(|x| x.local_name != "providerId");
            if el.attributes.len() < before {
                found = true;
            }
            break;
        }
        if found {
            let xml = crate::element::write_element(&root)?;
            let ct = "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml";
            self.package.set_part(uri, ct, xml);
        }
        Ok(found)
    }

    /// Whether mail-merge recipient parts are present.
    pub fn has_mail_merge_recipients(&self) -> bool {
        self.package
            .opc()
            .part_uris().into_iter().any(|u| u.as_str().contains("/word/recipients"))
    }

    /// Count mail-merge recipient parts.
    pub fn mail_merge_recipient_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/recipients"))
            .count()
    }

    /// Whether web settings part is present.
    pub fn has_web_settings(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/word/webSettings.xml"))
    }

    /// Whether any printer settings parts are present.
    pub fn has_printer_settings(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
        })
    }

    /// Count printer settings parts.
    pub fn printer_settings_count(&self) -> usize {
        self.package
            .opc()
            .part_uris().into_iter().filter(|u| {
                u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
            })
            .count()
    }

    /// Remove the people part and relationship.
    pub fn clear_people(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/people.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::PEOPLE)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Remove mail-merge recipient parts and their relationships.
    pub fn clear_mail_merge_recipients(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| u.as_str().contains("/word/recipients"))
            
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        const RECIPIENT_DATA: &str =
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData";
        // Relationships may hang off settings or main
        let parents = [
            PackUri::new("/word/settings.xml"),
            self.main_document_part
                .as_ref()
                .map(|m| m.part().uri.clone())
                .unwrap_or_else(|| PackUri::new("/word/document.xml")),
        ];
        for parent in &parents {
            if !self.package.opc().has_part(parent)
                && self.package.opc().part_relationships(parent).is_none()
            {
                continue;
            }
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(parent)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type == RECIPIENT_DATA)
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(parent);
                for id in ids {
                    rels.remove(&id);
                }
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Remove web settings part and relationship.
    pub fn clear_web_settings(&mut self) -> Result<bool> {
        let uri = PackUri::new("/word/webSettings.xml");
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::WEB_SETTINGS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Remove printer settings parts and relationships.
    pub fn clear_printer_settings(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris().into_iter().filter(|u| {
                u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
            })
            
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Some(main) = self.main_document_part.as_ref() {
            let main_uri = main.part().uri.clone();
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::PRINTER_SETTINGS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&main_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// List all part URIs in the package.
    pub fn list_part_uris(&self) -> Vec<PackUri> {
        self.package.opc().part_uris()
    }

    /// Count package-level relationships.
    pub fn package_relationship_count(&self) -> usize {
        self.package.opc().package_relationships().len()
    }

    /// Count relationships from the main document part.
    pub fn main_relationship_count(&self) -> usize {
        let Some(main) = self.main_document_part.as_ref() else {
            return 0;
        };
        self.package
            .opc()
            .part_relationships(&main.part().uri)
            .map(|r| r.len())
            .unwrap_or(0)
    }

    /// Collect content-control tags as `(tag, alias, kind)` from the main document.
    pub fn content_control_tags(&mut self) -> Result<Vec<(String, String, String)>> {
        use crate::wordprocessing::collect_sdt_tags;
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(collect_sdt_tags(doc))
    }

    /// Whether the document contains any structured document tags (content controls).
    pub fn has_content_controls(&mut self) -> Result<bool> {
        Ok(!self.content_control_tags()?.is_empty())
    }

    /// Number of structured document tags (content controls) in the main document.
    pub fn content_control_count(&mut self) -> Result<usize> {
        Ok(self.content_control_tags()?.len())
    }

    /// Whether a content control with the given tag exists.
    pub fn has_content_control(&mut self, tag: &str) -> Result<bool> {
        Ok(self
            .content_control_tags()?
            .iter()
            .any(|(t, _, _)| t == tag))
    }

    /// Alias for [`content_control_tags`](Self::content_control_tags).
    pub fn list_content_control_tags(&mut self) -> Result<Vec<(String, String, String)>> {
        self.content_control_tags()
    }

    /// Read the alias of the first content control matching `tag`.
    pub fn content_control_alias(&mut self, tag: &str) -> Result<Option<String>> {
        Ok(self
            .content_control_tags()?
            .into_iter()
            .find(|(t, _, _)| t == tag)
            .map(|(_, alias, _)| alias)
            .filter(|a| !a.is_empty()))
    }

    /// Read the specialized kind of the first content control matching `tag`
    /// (e.g. `richText`, `date`, `comboBox`), or `"sdt"` when unspecified.
    pub fn content_control_kind(&mut self, tag: &str) -> Result<Option<String>> {
        Ok(self
            .content_control_infos()?
            .into_iter()
            .find(|(t, _, _, _)| t == tag)
            .map(|(_, _, kind, _)| kind))
    }

    /// Collect content controls as `(tag, alias, kind, text)`.
    pub fn content_control_infos(&mut self) -> Result<Vec<(String, String, String, String)>> {
        use crate::wordprocessing::collect_sdt_infos;
        let package = &self.package;
        let main = self
            .main_document_part
            .as_mut()
            .ok_or_else(|| Error::Package("no main document part".into()))?;
        let doc = main.document(package)?;
        Ok(collect_sdt_infos(doc))
    }

    /// Whether any content control uses the given alias.
    pub fn has_content_control_alias(&mut self, alias: &str) -> Result<bool> {
        Ok(self
            .content_control_tags()?
            .iter()
            .any(|(_, a, _)| a == alias))
    }

    /// Set the alias on the first content control matching `tag`.
    pub fn set_content_control_alias(&mut self, tag: &str, alias: &str) -> Result<bool> {
        self.set_content_control_tag(tag, None, Some(alias))
    }

    /// Clear the alias on the first content control matching `tag`.
    pub fn clear_content_control_alias(&mut self, tag: &str) -> Result<bool> {
        self.set_content_control_tag(tag, None, Some(""))
    }

    /// Read plain text content of the first content control matching `tag`.
    pub fn content_control_text(&mut self, tag: &str) -> Result<Option<String>> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let doc = main
            .part()
            .root
            .as_ref()
            .ok_or(Error::NoRootElement)?;
        for e in doc.descendants() {
            if e.local_name != "sdt" {
                continue;
            }
            let Some(pr) = e.child("sdtPr") else { continue };
            let tag_val = pr.child("tag").and_then(|t| {
                t.get_attribute_qname("w:val")
                    .or_else(|| t.get_attribute("val"))
            });
            if tag_val != Some(tag) {
                continue;
            }
            let text = e
                .child("sdtContent")
                .map(|c| c.inner_text())
                .unwrap_or_default();
            return Ok(Some(text));
        }
        Ok(None)
    }

    /// Replace plain text content of the first content control matching `tag`.
    ///
    /// Clears `sdtContent` and inserts a single paragraph with the text.
    pub fn set_content_control_text(&mut self, tag: &str, content: &str) -> Result<bool> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn visit(el: &mut OpenXmlElement, tag: &str, content: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sdt" {
                let tag_val = el.child("sdtPr").and_then(|pr| {
                    pr.child("tag").and_then(|t| {
                        t.get_attribute_qname("w:val")
                            .or_else(|| t.get_attribute("val"))
                    })
                });
                if tag_val == Some(tag) {
                    // Ensure sdtContent
                    if el.child("sdtContent").is_none() {
                        el.append_child(OpenXmlElement::w("sdtContent"));
                    }
                    if let Some(sc) = el.child_mut("sdtContent") {
                        sc.clear_children();
                        sc.append_child(paragraph(vec![run(vec![text(content)])]));
                    }
                    *found = true;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, tag, content, found);
            }
        }
        let mut found = false;
        visit(doc, tag, content, &mut found);
        Ok(found)
    }

    /// Set tag/alias on the first content control matching `old_tag`.
    pub fn set_content_control_tag(
        &mut self,
        old_tag: &str,
        new_tag: Option<&str>,
        new_alias: Option<&str>,
    ) -> Result<bool> {
        // Ensure root loaded
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            let _ = main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, old_tag: &str, new_tag: Option<&str>, new_alias: Option<&str>, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sdt" {
                if let Some(pr) = el.child_mut("sdtPr") {
                    let tag_val = pr.child("tag").and_then(|t| {
                        t.get_attribute_qname("w:val")
                            .or_else(|| t.get_attribute("val"))
                    });
                    if tag_val == Some(old_tag) {
                        *found = true;
                        if let Some(nt) = new_tag {
                            if let Some(t) = pr.child_mut("tag") {
                                t.set_attribute_qname("w:val", nt);
                            } else {
                                pr.append_child(
                                    OpenXmlElement::w("tag").with_attribute_qname("w:val", nt),
                                );
                            }
                        }
                        if let Some(na) = new_alias {
                            if let Some(a) = pr.child_mut("alias") {
                                a.set_attribute_qname("w:val", na);
                            } else {
                                pr.append_child(
                                    OpenXmlElement::w("alias").with_attribute_qname("w:val", na),
                                );
                            }
                        }
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, old_tag, new_tag, new_alias, found);
            }
        }
        visit(doc, old_tag, new_tag, new_alias, &mut found);
        if found {
            main.part_mut().dirty = true;
        }
        Ok(found)
    }

    /// Remove content controls with the given tag (unwraps content into parent).
    pub fn remove_content_control_by_tag(&mut self, tag: &str) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            let _ = main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        let mut n = 0usize;
        fn unwrap_sdt(el: &mut OpenXmlElement, tag: &str, n: &mut usize) {
            let mut i = 0;
            while i < el.children.len() {
                let is_match = {
                    let c = &el.children[i];
                    if c.local_name != "sdt" {
                        false
                    } else {
                        c.child("sdtPr")
                            .and_then(|pr| pr.child("tag"))
                            .and_then(|t| {
                                t.get_attribute_qname("w:val")
                                    .or_else(|| t.get_attribute("val"))
                            })
                            == Some(tag)
                    }
                };
                if is_match {
                    let sdt = el.children.remove(i);
                    let content_kids: Vec<OpenXmlElement> = sdt
                        .child("sdtContent")
                        .map(|sc| sc.children.clone())
                        .unwrap_or_default();
                    for (j, kid) in content_kids.into_iter().enumerate() {
                        el.children.insert(i + j, kid);
                    }
                    *n += 1;
                    // do not advance i; new content may contain more sdts
                } else {
                    unwrap_sdt(&mut el.children[i], tag, n);
                    i += 1;
                }
            }
        }
        unwrap_sdt(doc, tag, &mut n);
        if n > 0 {
            main.part_mut().dirty = true;
        }
        Ok(n)
    }

    /// Remove all content controls from the main document body.
    ///
    /// Returns the number of SDT elements removed.
    pub fn clear_content_controls(&mut self) -> Result<usize> {
        let tags: Vec<String> = self
            .content_control_tags()?
            .into_iter()
            .map(|(t, _, _)| t)
            .filter(|t| !t.is_empty())
            .collect();
        let mut n = 0usize;
        // Also remove untagged SDTs by walking DOM once if tags empty.
        if tags.is_empty() {
            return self.remove_all_content_controls();
        }
        let mut seen = std::collections::HashSet::new();
        for tag in tags {
            if seen.insert(tag.clone()) {
                n += self.remove_content_control_by_tag(&tag)?;
            }
        }
        // Catch remaining untagged
        n += self.remove_all_content_controls()?;
        Ok(n)
    }

    /// Remove every `w:sdt` under the main document (tagged or not), promoting children.
    fn remove_all_content_controls(&mut self) -> Result<usize> {
        {
            let package = &self.package;
            let main = self
                .main_document_part
                .as_mut()
                .ok_or_else(|| Error::Package("no main document part".into()))?;
            main.document(package)?;
        }
        let main = self.main_document_part.as_mut().unwrap();
        let part = main.part_mut();
        part.dirty = true;
        let doc = part.root.as_mut().ok_or(Error::NoRootElement)?;
        fn visit(el: &mut OpenXmlElement, count: &mut usize) {
            let mut i = 0;
            while i < el.children.len() {
                if el.children[i].local_name == "sdt" {
                    let removed = el.children.remove(i);
                    // Prefer promoting sdtContent children; else promote all.
                    let kids = if let Some(sc) = removed.children.iter().find(|c| c.local_name == "sdtContent") {
                        sc.children.clone()
                    } else {
                        removed.children
                    };
                    let n = kids.len();
                    for (offset, kid) in kids.into_iter().enumerate() {
                        el.children.insert(i + offset, kid);
                    }
                    *count += 1;
                    i += n;
                } else {
                    visit(&mut el.children[i], count);
                    i += 1;
                }
            }
        }
        let mut count = 0usize;
        visit(doc, &mut count);
        Ok(count)
    }

    pub fn bookmark_count(&mut self) -> Result<usize> {
        Ok(self.bookmarks()?.len())
    }

    /// Whether the document contains any bookmarks.
    pub fn has_bookmarks(&mut self) -> Result<bool> {
        Ok(self.bookmark_count()? > 0)
    }

    /// Bookmark names only (order matches [`Self::bookmarks`]).
    pub fn list_bookmark_names(&mut self) -> Result<Vec<String>> {
        Ok(self
            .bookmarks()?
            .into_iter()
            .map(|(_, name)| name)
            .collect())
    }

    /// Whether a bookmark with the given name exists.
    pub fn has_bookmark(&mut self, name: &str) -> Result<bool> {
        Ok(self.list_bookmark_names()?.iter().any(|n| n == name))
    }

    /// Count image/media parts under `/word/media/` (alias of [`Self::media_count`]).
    pub fn image_count(&self) -> usize {
        self.media_count()
    }
}

impl Drop for WordprocessingDocument {
    fn drop(&mut self) {
        if self.package.is_closed() {
            return;
        }
        if self.package.auto_save()
            && matches!(
                self.package.opc().mode(),
                PackageMode::Create | PackageMode::ReadWrite
            )
            && self.package.path().is_some()
        {
            let _ = self.flush_parts();
            let _ = self.package.save();
        }
    }
}
