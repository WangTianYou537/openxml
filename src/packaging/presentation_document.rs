//! PresentationDocument — PowerPoint (`.pptx`) package.

use super::open_xml_package::{OpenSettings, OpenXmlPackage};
use crate::element::{parse_element, write_element, OpenXmlElement};
use crate::error::{Error, Result};
use crate::namespace::{content_type, rel};
use crate::opc::{
    CustomProperties, ExtendedProperties, OpcPackage, PackUri, PackageMode, PackageProperties,
    RelationshipTargetMode, Relationships,
};
use crate::presentation::{
    auto_shape, common_slide_data, dissolve_transition, fade_transition, group_shape_pr,
    group_shape_properties, handout_master, header_footer, notes_master, notes_size, notes_slide,
    picture_shape, presentation, replace_slide_text, section, section_list, section_list_ext,
    shape_tree, slide, slide_comment, slide_comments, slide_id, slide_id_list, slide_layout,
    slide_layout_id, slide_master, slide_master_id, slide_master_id_list, slide_size, slide_texts,
    slide_transition, slide_with_text, solid_slide_background, table_graphic_frame, text_shape,
    SLIDE_SIZE_16_9,
};
use std::path::Path;

/// Type of PresentationML package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PresentationDocumentType {
    #[default]
    Presentation,
    Template,
    Slideshow,
    MacroEnabledPresentation,
    MacroEnabledTemplate,
    MacroEnabledSlideshow,
}

impl PresentationDocumentType {
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Presentation => content_type::PRESENTATION,
            Self::Template => {
                "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml"
            }
            Self::Slideshow => {
                "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml"
            }
            Self::MacroEnabledPresentation => {
                "application/vnd.ms-powerpoint.presentation.macroEnabled.main+xml"
            }
            Self::MacroEnabledTemplate => {
                "application/vnd.ms-powerpoint.template.macroEnabled.main+xml"
            }
            Self::MacroEnabledSlideshow => {
                "application/vnd.ms-powerpoint.slideshow.macroEnabled.main+xml"
            }
        }
    }

    pub fn from_content_type(ct: &str) -> Option<Self> {
        match ct {
            content_type::PRESENTATION => Some(Self::Presentation),
            "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml" => {
                Some(Self::Template)
            }
            "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml" => {
                Some(Self::Slideshow)
            }
            "application/vnd.ms-powerpoint.presentation.macroEnabled.main+xml" => {
                Some(Self::MacroEnabledPresentation)
            }
            "application/vnd.ms-powerpoint.template.macroEnabled.main+xml" => {
                Some(Self::MacroEnabledTemplate)
            }
            "application/vnd.ms-powerpoint.slideshow.macroEnabled.main+xml" => {
                Some(Self::MacroEnabledSlideshow)
            }
            _ => None,
        }
    }
}

const PRESENTATION_URI: &str = "/ppt/presentation.xml";

/// Info about a slide in the presentation.
#[derive(Debug, Clone)]
pub struct SlideInfo {
    pub relationship_id: String,
    pub uri: PackUri,
    pub id: u32,
}

/// Info about a slide master.
#[derive(Debug, Clone)]
pub struct SlideMasterInfo {
    pub relationship_id: String,
    pub uri: PackUri,
    pub id: u32,
}

/// Info about a slide layout.
#[derive(Debug, Clone)]
pub struct SlideLayoutInfo {
    pub relationship_id: String,
    pub uri: PackUri,
    pub id: u32,
    /// Relationship id from the parent slide master.
    pub master_relationship_id: String,
}

/// An Open XML presentation document (`.pptx`).
#[derive(Debug)]
pub struct PresentationDocument {
    package: OpenXmlPackage,
    document_type: PresentationDocumentType,
    slides: Vec<SlideInfo>,
    masters: Vec<SlideMasterInfo>,
    layouts: Vec<SlideLayoutInfo>,
    next_slide_index: u32,
    next_slide_id: u32,
    next_master_index: u32,
    next_layout_index: u32,
    next_master_id: u32,
    next_layout_id: u32,
}

/// How fonts are packaged after SVG → shapes conversion.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SvgFontEmbedMode {
    /// No font parts. Host substitutes (Windows: TNR / YaHei / Arial).
    #[default]
    None,
    /// Editable text + subset EOT of used faces (`--embed-font`).
    Subset,
    /// Editable text + full EOT of used faces (`--embed-font-fully`).
    Full,
}

/// Options for [`PresentationDocument::add_svg_shapes_on_slide_ex`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SvgShapesOnSlideOptions {
    /// `false` = outline glyphs as shapes (`--font-shape`); `true` = text boxes.
    pub editable_text: bool,
    /// Font packaging mode.
    pub font_embed: SvgFontEmbedMode,
}

impl Default for SvgShapesOnSlideOptions {
    fn default() -> Self {
        // Default: editable text boxes, no embed (Windows system faces).
        Self {
            editable_text: true,
            font_embed: SvgFontEmbedMode::None,
        }
    }
}

impl PresentationDocument {
    /// Create a new presentation at `path`.
    pub fn create(path: impl AsRef<Path>, document_type: PresentationDocumentType) -> Result<Self> {
        Self::create_with_settings(path, document_type, OpenSettings::default())
    }

    /// Create a new presentation at `path` with custom open settings.
    pub fn create_with_settings(
        path: impl AsRef<Path>,
        document_type: PresentationDocumentType,
        settings: OpenSettings,
    ) -> Result<Self> {
        let opc = OpcPackage::create_file(path.as_ref());
        let mut package = OpenXmlPackage::from_opc(opc, settings);
        package.set_application_type(crate::features::ApplicationType::POWERPOINT);
        package.set_package_factory_feature("PresentationDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "PresentationDocument",
        ));
        Ok(Self {
            package,
            document_type,
            slides: Vec::new(),
            masters: Vec::new(),
            layouts: Vec::new(),
            next_slide_index: 1,
            next_slide_id: 256,
            next_master_index: 1,
            next_layout_index: 1,
            next_master_id: 2_147_483_648,
            next_layout_id: 2_147_483_649,
        })
    }

    pub fn create_in_memory(document_type: PresentationDocumentType) -> Result<Self> {
        let opc = OpcPackage::create();
        let mut package = OpenXmlPackage::from_opc(opc, OpenSettings::default());
        package.set_application_type(crate::features::ApplicationType::POWERPOINT);
        package.set_package_factory_feature("PresentationDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "PresentationDocument",
        ));
        Ok(Self {
            package,
            document_type,
            slides: Vec::new(),
            masters: Vec::new(),
            layouts: Vec::new(),
            next_slide_index: 1,
            next_slide_id: 256,
            next_master_index: 1,
            next_layout_index: 1,
            next_master_id: 2_147_483_648,
            next_layout_id: 2_147_483_649,
        })
    }

    /// Quick-create a presentation at `path` with a single title slide.
    pub fn create_simple(path: impl AsRef<Path>, slide_text: &str) -> Result<Self> {
        let mut ppt = Self::create(path, PresentationDocumentType::Presentation)?;
        ppt.add_slide_with_text(slide_text)?;
        Ok(ppt)
    }

    pub fn open(path: impl AsRef<Path>, is_editable: bool) -> Result<Self> {
        Self::open_with_settings(path, is_editable, OpenSettings::default())
    }

    /// Open an existing presentation with custom open settings.
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

    /// Create a presentation by cloning an existing package (template).
    pub fn create_from_template(template_path: impl AsRef<Path>) -> Result<Self> {
        Self::create_from_template_as(template_path, None)
    }

    /// Create a presentation by cloning a template, optionally changing document type.
    pub fn create_from_template_as(
        template_path: impl AsRef<Path>,
        document_type: Option<PresentationDocumentType>,
    ) -> Result<Self> {
        let src = Self::open(template_path, false)?;
        let bytes = src.to_bytes()?;
        let mut cloned = Self::open_bytes(bytes)?;
        if let Some(dt) = document_type {
            cloned.change_document_type(dt)?;
        }
        Ok(cloned)
    }

    pub fn open_bytes(data: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = data.as_ref().to_vec();
        let opc = OpcPackage::open_bytes(&bytes)?;
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        let mut doc = Self::from_opc(opc, settings)?;
        doc.package_mut().set_package_stream_bytes(bytes);
        Ok(doc)
    }

    /// Open a Presentation package from any `Read + Seek` stream (C# `Open(Stream, …)`).
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
        self.package.write_to(writer)
    }


    fn from_opc(opc: OpcPackage, settings: OpenSettings) -> Result<Self> {
        let mut package = OpenXmlPackage::from_opc(opc, settings);
        package.set_application_type(crate::features::ApplicationType::POWERPOINT);
        package.set_package_factory_feature("PresentationDocument");
        package.set_document_type_feature(crate::features::DocumentTypeFeature::new(
            "PresentationDocument",
        ));
        if let Ok(uri) = package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ct = package
                .opc()
                .content_types()
                .content_type_for(uri.as_str())
                .unwrap_or("")
                .to_string();
            package.set_main_part_feature(crate::features::MainPartFeature::new(
                rel::OFFICE_DOCUMENT,
                ct,
                Some(uri.as_str().to_string()),
            ));
        }
        let mut doc = Self {
            package,
            document_type: PresentationDocumentType::Presentation,
            slides: Vec::new(),
            masters: Vec::new(),
            layouts: Vec::new(),
            next_slide_index: 1,
            next_slide_id: 256,
            next_master_index: 1,
            next_layout_index: 1,
            next_master_id: 2_147_483_648,
            next_layout_id: 2_147_483_649,
        };
        doc.reload_slides()?;
        Ok(doc)
    }

    fn reload_slides(&mut self) -> Result<()> {
        self.slides.clear();
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(()),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(());
        };
        let root = parse_element(data)?;
        let Some(list) = root.child("sldIdLst") else {
            return Ok(());
        };
        for sld in list.children_by_name("sldId") {
            let id = sld
                .get_attribute("id")
                .and_then(|s| s.parse().ok())
                .unwrap_or(256);
            let rid = sld
                .get_attribute_qname("r:id")
                .or_else(|| sld.get_attribute("id"))
                .unwrap_or("")
                .to_string();
            if rid.is_empty() {
                continue;
            }
            let uri = if let Some(rel_obj) = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .and_then(|rels| rels.get(&rid))
            {
                self.package
                    .opc()
                    .resolve_relationship(Some(&pres_uri), rel_obj)?
            } else {
                continue;
            };
            if let Some(name) = uri.as_str().rsplit('/').next() {
                if let Some(n) = name
                    .strip_prefix("slide")
                    .and_then(|s| s.strip_suffix(".xml"))
                    .and_then(|s| s.parse::<u32>().ok())
                {
                    if n >= self.next_slide_index {
                        self.next_slide_index = n + 1;
                    }
                }
            }
            if id >= self.next_slide_id {
                self.next_slide_id = id + 1;
            }
            self.slides.push(SlideInfo {
                relationship_id: rid,
                uri,
                id,
            });
        }
        Ok(())
    }

    pub fn document_type(&self) -> PresentationDocumentType {
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

    /// Read package core properties.
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

    /// Write extended properties.
    pub fn set_extended_properties(&mut self, props: &ExtendedProperties) -> Result<()> {
        props.save_to(self.package.opc_mut())
    }

    /// Read custom properties.
    pub fn custom_properties(&self) -> Result<CustomProperties> {
        CustomProperties::load_from(self.package.opc())
    }

    /// Write custom properties.
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

    /// Whether core language is set.
    pub fn has_language(&self) -> Result<bool> {
        Ok(self.language()?.is_some())
    }

    /// Clear core language.
    pub fn clear_language(&mut self) -> Result<bool> {
        let had = self.language()?.is_some();
        if had {
            let mut props = self.package_properties()?;
            props.language = None;
            self.set_package_properties(&props)?;
        }
        Ok(had)
    }

    /// Convenience: set core version.
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
            .map(|r| (r.id.clone(), r.relationship_type.clone(), r.target.clone()))
            .collect()
    }

    /// List presentation relationships as `(id, type, target)`.
    pub fn list_presentation_relationships(&self) -> Vec<(String, String, String)> {
        let pres = PackUri::new(PRESENTATION_URI);
        self.package
            .opc()
            .part_relationships(&pres)
            .map(|rels| {
                rels.iter()
                    .map(|r| (r.id.clone(), r.relationship_type.clone(), r.target.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Count relationships from the presentation part.
    pub fn presentation_relationship_count(&self) -> usize {
        self.list_presentation_relationships().len()
    }

    /// Alias for [`list_presentation_relationships`](Self::list_presentation_relationships).
    pub fn list_main_relationships(&self) -> Vec<(String, String, String)> {
        self.list_presentation_relationships()
    }

    /// Alias for [`presentation_relationship_count`](Self::presentation_relationship_count).
    pub fn main_relationship_count(&self) -> usize {
        self.presentation_relationship_count()
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

    /// Convenience: set extended Slides count.
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

    pub fn set_app_slides(&mut self, slides: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.slides = Some(slides);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Slides count.
    pub fn app_slides(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.slides)
    }

    /// Convenience: set extended Notes count.
    pub fn set_app_notes(&mut self, notes: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.notes = Some(notes);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended Notes count.
    pub fn app_notes(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.notes)
    }

    /// Convenience: set extended PresentationFormat.
    pub fn set_presentation_format(&mut self, format: &str) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.presentation_format = Some(format.to_string());
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended PresentationFormat.
    pub fn presentation_format(&self) -> Result<Option<String>> {
        Ok(self.extended_properties()?.presentation_format)
    }

    /// Convenience: set extended HiddenSlides.
    pub fn set_app_hidden_slides(&mut self, n: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.hidden_slides = Some(n);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended HiddenSlides.
    pub fn app_hidden_slides(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.hidden_slides)
    }

    /// Convenience: set extended MMClips.
    pub fn set_mm_clips(&mut self, n: i32) -> Result<()> {
        let mut props = self.extended_properties()?;
        props.mm_clips = Some(n);
        self.set_extended_properties(&props)
    }

    /// Convenience: read extended MMClips.
    pub fn mm_clips(&self) -> Result<Option<i32>> {
        Ok(self.extended_properties()?.mm_clips)
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

    /// Whether PresentationFormat is set.
    pub fn has_presentation_format(&self) -> Result<bool> {
        Ok(self.presentation_format()?.is_some())
    }

    /// Clear PresentationFormat.
    pub fn clear_presentation_format(&mut self) -> Result<bool> {
        let had = self.presentation_format()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.presentation_format = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// Whether MMClips is set.
    pub fn has_mm_clips(&self) -> Result<bool> {
        Ok(self.mm_clips()?.is_some())
    }

    /// Clear MMClips.
    pub fn clear_mm_clips(&mut self) -> Result<bool> {
        let had = self.mm_clips()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.mm_clips = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
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

    /// Whether HiddenSlides is set.
    pub fn has_app_hidden_slides(&self) -> Result<bool> {
        Ok(self.app_hidden_slides()?.is_some())
    }

    /// Clear HiddenSlides.
    pub fn clear_app_hidden_slides(&mut self) -> Result<bool> {
        let had = self.app_hidden_slides()?.is_some();
        if had {
            let mut props = self.extended_properties()?;
            props.hidden_slides = None;
            self.set_extended_properties(&props)?;
        }
        Ok(had)
    }

    /// List relationships from a slide as `(id, type, target)`.
    pub fn list_slide_relationships(
        &self,
        slide_index: usize,
    ) -> Result<Vec<(String, String, String)>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        Ok(self
            .package
            .opc()
            .part_relationships(&slide_info.uri)
            .map(|rels| {
                rels.iter()
                    .map(|r| (r.id.clone(), r.relationship_type.clone(), r.target.clone()))
                    .collect()
            })
            .unwrap_or_default())
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
    pub fn set_part_bytes(&mut self, uri: &str, content_type: &str, data: impl Into<Vec<u8>>) {
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

    /// Number of parts in the package.
    pub fn package_part_count(&self) -> usize {
        self.package.opc().part_uris().len()
    }

    /// Add a package thumbnail image (`docProps/thumbnail.{ext}`).
    pub fn add_thumbnail(
        &mut self,
        image_bytes: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<String> {
        let uri = PackUri::new(format!("/docProps/thumbnail.{extension}"));
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type_str, image_bytes.into());
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

    /// Whether a package thumbnail relationship or part exists.
    pub fn has_thumbnail(&self) -> bool {
        self.package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .is_some()
            || self
                .package
                .opc()
                .part_uris()
                .into_iter()
                .any(|u| u.as_str().starts_with("/docProps/thumbnail."))
    }

    /// Remove the package thumbnail part and relationship.
    pub fn clear_thumbnail(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/docProps/thumbnail."))
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
        if let Some(rel_id) = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::THUMBNAIL)
            .map(|r| r.id.clone())
        {
            self.package
                .opc_mut()
                .package_relationships_mut()
                .remove(&rel_id);
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Add a digital signature origin part shell (no crypto).
    pub fn add_digital_signature_origin(&mut self) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/_xmlsignatures/origin.sigs");
        if !self.package.opc().has_part(&uri) {
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
    pub fn add_xml_signature_part(
        &mut self,
        signature_xml: impl AsRef<[u8]>,
    ) -> Result<(String, PackUri)> {
        let (_origin_rid, origin_uri) = self.add_digital_signature_origin()?;
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
            .part_uris()
            .into_iter()
            .filter(|u| {
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
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/_xmlsignatures/"))
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

    /// List custom XML parts related from the presentation as `(id, uri, bytes)`.
    pub fn custom_xml_parts(&self) -> Result<Vec<(String, PackUri, Vec<u8>)>> {
        let pres_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let Some(rels) = self.package.opc().part_relationships(&pres_uri) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for r in rels.find_all_by_type(rel::CUSTOM_XML) {
            let target = crate::opc::resolve_uri(&pres_uri, &r.target)?;
            if let Some(data) = self.package.opc().get_part(&target) {
                out.push((r.id.clone(), target, data.to_vec()));
            }
        }
        Ok(out)
    }

    /// Number of custom XML parts.
    pub fn custom_xml_part_count(&self) -> Result<usize> {
        Ok(self.custom_xml_parts()?.len())
    }

    /// Whether any custom XML parts are present.
    pub fn has_custom_xml_parts(&self) -> Result<bool> {
        Ok(!self.custom_xml_parts()?.is_empty())
    }

    /// Add a custom XML part related from the presentation.
    pub fn add_custom_xml_part(
        &mut self,
        xml_bytes: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let pres_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let part_uri = loop {
            let candidate = PackUri::new(format!("/customXml/item{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package.set_part(
            part_uri.clone(),
            content_type::CUSTOM_XML,
            xml_bytes.into(),
        );
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::CUSTOM_XML,
            &part_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, part_uri))
    }

    /// Attach a custom XML properties part to an existing custom XML item part.
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
        let ds = "http://schemas.openxmlformats.org/officeDocument/2006/customXml";
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

    /// Remove a custom XML part by relationship id.
    pub fn remove_custom_xml_part(&mut self, relationship_id: &str) -> Result<bool> {
        let pres_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let target = {
            let Some(rels) = self.package.opc().part_relationships(&pres_uri) else {
                return Ok(false);
            };
            let Some(rel_entry) = rels.get(relationship_id) else {
                return Ok(false);
            };
            if rel_entry.relationship_type != rel::CUSTOM_XML {
                return Ok(false);
            }
            crate::opc::resolve_uri(&pres_uri, &rel_entry.target)?
        };
        self.package
            .opc_mut()
            .part_relationships_mut(&pres_uri)
            .remove(relationship_id);
        self.package.opc_mut().remove_part(&target);
        Ok(true)
    }

    /// Remove all custom XML parts related from the presentation.
    pub fn clear_custom_xml_parts(&mut self) -> Result<usize> {
        let parts = self.custom_xml_parts()?;
        let n = parts.len();
        for (rid, _, _) in parts {
            let _ = self.remove_custom_xml_part(&rid)?;
        }
        Ok(n)
    }

    /// Embed an arbitrary package/object under `/ppt/embeddings/`.
    pub fn add_embedded_package(
        &mut self,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<(String, PackUri)> {
        let pres_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let uri = loop {
            let candidate = PackUri::new(format!(
                "/ppt/embeddings/Microsoft_Object{index}.{extension}"
            ));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::PACKAGE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Alias for [`add_embedded_package`](Self::add_embedded_package) using package content type.
    pub fn add_embedded_package_part(
        &mut self,
        data: impl Into<Vec<u8>>,
        extension: &str,
    ) -> Result<(String, PackUri)> {
        self.add_embedded_package(data, content_type::PACKAGE_EMBEDDED, extension)
    }

    /// Embed an OLE object binary part shell under `/ppt/embeddings/`.
    ///
    /// `prog_id` is accepted for API compatibility and currently unused.
    pub fn add_embedded_object(
        &mut self,
        data: impl Into<Vec<u8>>,
        prog_id: &str,
    ) -> Result<(String, PackUri)> {
        let _ = prog_id;
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/embeddings/oleObject{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
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

    /// Whether any embedded package/object parts exist under `/ppt/embeddings/`.
    pub fn has_embeddings(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().starts_with("/ppt/embeddings/"))
    }

    /// Count embedding parts under `/ppt/embeddings/`.
    pub fn embedding_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/embeddings/"))
            .count()
    }

    /// List embedding part URIs.
    pub fn list_embeddings(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/embeddings/"))
            .collect()
    }

    /// Remove all embedding parts and related presentation relationships.
    pub fn clear_embeddings(&mut self) -> Result<usize> {
        let uris = self.list_embeddings();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
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
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Add a VBA project binary part shell (no macro execution).
    pub fn add_vba_project(&mut self, data: impl Into<Vec<u8>>) -> Result<(String, PackUri)> {
        let pres_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let uri = PackUri::new("/ppt/vbaProject.bin");
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type::VBA_PROJECT, data.into());
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::VBA_PROJECT).map(|r| r.id.clone()))
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::VBA_PROJECT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// True if a VBA project part is present.
    pub fn has_vba_project(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("vbaProject") || u.as_str().ends_with("vbaProject.bin"))
    }

    /// Remove VBA project parts and their relationships.
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

    pub fn vba_part_count(&self) -> usize {
        self.list_vba_parts().len()
    }

    /// Parse `vbaProject.bin` CFB structure (streams/storages inventory; no macro execution).
    pub fn inspect_vba_project(&self) -> crate::Result<Option<crate::opc::CfbFile>> {
        let Some(bytes) = self.vba_project_bytes() else {
            return Ok(None);
        };
        Ok(Some(crate::opc::inspect_vba_project(&bytes)?))
    }

    pub fn clear_vba_project(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str();
                s.contains("vbaProject") || s.contains("vbaData")
            })
            .collect();
        if uris.is_empty() {
            return Ok(false);
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type == rel::VBA_PROJECT)
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Add a Custom UI part (`/customUI/customUI.xml`) at package level.
    pub fn add_custom_ui(&mut self, custom_ui_xml: impl AsRef<[u8]>) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/customUI/customUI.xml");
        self.package.set_part(
            uri.clone(),
            content_type::CUSTOM_UI,
            custom_ui_xml.as_ref().to_vec(),
        );
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

    /// Whether a Custom UI part/relationship exists.
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
                self.package
                    .opc_mut()
                    .package_relationships_mut()
                    .remove(&id);
            }
        }
        if had_part {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Add a printer settings binary part shell related from the presentation.
    pub fn add_printer_settings(&mut self, data: impl Into<Vec<u8>>) -> Result<(String, PackUri)> {
        let pres_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let uri = loop {
            let candidate =
                PackUri::new(format!("/ppt/printerSettings/printerSettings{index}.bin"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        // Reuse spreadsheet printer content type (binary printer blob).
        self.package.set_part(
            uri.clone(),
            content_type::SPREADSHEET_PRINTER_SETTINGS,
            data.into(),
        );
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::PRINTER_SETTINGS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether any printer settings parts exist.
    pub fn has_printer_settings(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
        })
    }

    /// Count printer settings parts.
    pub fn printer_settings_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
            })
            .count()
    }

    /// Remove all printer settings parts and related presentation relationships.
    pub fn clear_printer_settings(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                u.as_str().contains("printerSettings") || u.as_str().contains("PrinterSettings")
            })
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::PRINTER_SETTINGS)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Add attached toolbars binary part shell related from the presentation.
    pub fn add_attached_toolbars(&mut self, data: impl Into<Vec<u8>>) -> Result<(String, PackUri)> {
        let pres_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let uri = PackUri::new("/ppt/attachedToolbars.bin");
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type::ATTACHED_TOOLBARS, data.into());
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| {
                rels.get_by_type(rel::ATTACHED_TOOLBARS)
                    .map(|r| r.id.clone())
            })
        {
            return Ok((existing, uri));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::ATTACHED_TOOLBARS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether attached toolbars part exists.
    pub fn has_attached_toolbars(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/ppt/attachedToolbars.bin"))
            || self
                .package
                .opc()
                .part_uris()
                .into_iter()
                .any(|u| u.as_str().contains("attachedToolbars"))
    }

    /// Remove attached toolbars part and relationship.
    pub fn clear_attached_toolbars(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/attachedToolbars.bin");
        let had_part = self.package.opc().has_part(&uri);
        let mut had_rel = false;
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            if let Some(id) = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .and_then(|rels| {
                    rels.get_by_type(rel::ATTACHED_TOOLBARS)
                        .map(|r| r.id.clone())
                })
            {
                had_rel = true;
                self.package
                    .opc_mut()
                    .part_relationships_mut(&pres_uri)
                    .remove(&id);
            }
        }
        if had_part {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(had_part || had_rel)
    }

    /// Add a Quick Access Toolbar customizations part (package-level).
    pub fn add_quick_access_toolbar(&mut self) -> Result<(String, PackUri)> {
        let uri = PackUri::new("/customUI/qatCustomizations.xml");
        let mso = "http://schemas.microsoft.com/office/2006/01/customui";
        let root = OpenXmlElement::new("mso", mso, "customUI")
            .with_ns_decl("mso", mso)
            .with_child(OpenXmlElement::new("mso", mso, "ribbon").with_child(
                OpenXmlElement::new("mso", mso, "qat").with_child(OpenXmlElement::new(
                    "mso",
                    mso,
                    "sharedControls",
                )),
            ));
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

    /// Whether a QAT customizations part exists.
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
            self.package
                .opc_mut()
                .package_relationships_mut()
                .remove(&id);
        }
        if had_part {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
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

    /// Whether a sensitivity label info part exists.
    pub fn has_label_info(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/docMetadata/LabelInfo.xml"))
            || self
                .package
                .opc()
                .package_relationships()
                .get_by_type(rel::LABEL_INFO)
                .is_some()
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
            self.package
                .opc_mut()
                .package_relationships_mut()
                .remove(&id);
        }
        if had_part {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Add Office web extension + taskpanes shells under `/ppt/webextensions/`.
    pub fn add_web_extension_shell(
        &mut self,
        store_id: &str,
        version: &str,
    ) -> Result<(PackUri, PackUri)> {
        let we_uri = PackUri::new("/ppt/webextensions/webextension1.xml");
        let tp_uri = PackUri::new("/ppt/webextensions/taskpanes.xml");
        let we = "http://schemas.microsoft.com/office/webextensions/webextension/2010/11";
        let wetp = "http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11";
        let ext = OpenXmlElement::new("we", we, "webextension")
            .with_ns_decl("we", we)
            .with_attribute(
                "id",
                format!("{{{}-0000-0000-0000-000000000000}}", store_id),
            )
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
            .with_ns_decl(
                "r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            )
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
        self.package.add_part_relationship(
            &tp_uri,
            rel::WEB_EXTENSION,
            &we_uri,
            RelationshipTargetMode::Internal,
        );
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

    /// Whether any web extension parts exist under `/ppt/webextensions/`.
    pub fn has_web_extensions(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("/ppt/webextensions/"))
    }

    /// Count web extension parts under `/ppt/webextensions/`.
    pub fn web_extension_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/webextensions/"))
            .count()
    }

    /// Remove web extension + taskpanes parts and package relationships.
    pub fn clear_web_extensions(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/webextensions/"))
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        for ty in [rel::WEB_EXTENSION, rel::WEB_EXTENSION_TASKPANES] {
            if let Some(id) = self
                .package
                .opc()
                .package_relationships()
                .get_by_type(ty)
                .map(|r| r.id.clone())
            {
                self.package
                    .opc_mut()
                    .package_relationships_mut()
                    .remove(&id);
            }
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
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
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Whether any SmartArt/diagram parts exist under `/ppt/diagrams/`.
    pub fn has_diagrams(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("/ppt/diagrams/"))
    }

    /// Count diagram parts under `/ppt/diagrams/`.
    pub fn diagram_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/diagrams/"))
            .count()
    }

    /// List diagram part URIs.
    pub fn list_diagrams(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/diagrams/"))
            .collect()
    }

    /// Remove diagram parts and related main-part diagram relationships.
    pub fn clear_diagrams(&mut self) -> Result<usize> {
        let uris = self.list_diagrams();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(main_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
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
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Add a SmartArt / diagram parts shell (data, layout, colors, style, persist layout).
    ///
    /// Creates minimal diagram parts under `/ppt/diagrams/` related from the main
    /// document (persist layout is related from the data part). Returns the data part URI.
    pub fn add_diagram_shell(&mut self, unique_id: &str) -> Result<PackUri> {
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let data_uri = loop {
            let c = PackUri::new(format!("/ppt/diagrams/data{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let layout_uri = PackUri::new(format!("/ppt/diagrams/layout{index}.xml"));
        let colors_uri = PackUri::new(format!("/ppt/diagrams/colors{index}.xml"));
        let style_uri = PackUri::new(format!("/ppt/diagrams/quickStyle{index}.xml"));
        let drawing_uri = PackUri::new(format!("/ppt/diagrams/drawing{index}.xml"));
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
                            .with_child(OpenXmlElement::new("a", a, "t").with_child(
                                OpenXmlElement::new("a", a, "p").with_child(
                                    OpenXmlElement::new("a", a, "r").with_child(
                                        OpenXmlElement::new("a", a, "t").with_text("Node"),
                                    ),
                                ),
                            )),
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
            self.package
                .opc_mut()
                .set_part(uri.clone(), ct, crate::element::write_element(&el)?);
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

    /// Add legacy diagram text parts shell (VML diagram text).
    pub fn add_legacy_diagram_text(
        &mut self,
        text_data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, PackUri)> {
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let mut index = 1u32;
        let text_uri = loop {
            let c = PackUri::new(format!("/ppt/diagrams/legacy/text{index}.bin"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let info_uri = PackUri::new(format!("/ppt/diagrams/legacy/textInfo{index}.xml"));
        self.package.set_part(
            text_uri.clone(),
            content_type::LEGACY_DIAGRAM_TEXT,
            text_data.into(),
        );
        let dgm = "http://schemas.microsoft.com/office/drawing/2008/diagram";
        let info = OpenXmlElement::new("dgm", dgm, "textInfo").with_ns_decl("dgm", dgm);
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

    /// Store an image media part under `/ppt/media/` without anchoring it.
    ///
    /// Returns `(relationship_id, part_uri)` related from the main document part.
    /// Prefer [`add_image_on_sheet`] / [`add_image_on_slide`] when a drawing anchor is needed.
    pub fn add_image(
        &mut self,
        format: crate::packaging::ImageFormat,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let main_uri = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT)?;
        let ext = match format {
            crate::packaging::ImageFormat::Png => "png",
            crate::packaging::ImageFormat::Jpeg => "jpeg",
            crate::packaging::ImageFormat::Gif => "gif",
            crate::packaging::ImageFormat::Bmp => "bmp",
            crate::packaging::ImageFormat::Tiff => "tiff",
            crate::packaging::ImageFormat::Emf => "emf",
            crate::packaging::ImageFormat::Wmf => "wmf",
            crate::packaging::ImageFormat::Svg => "svg",
        };
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/media/image{index}.{ext}"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let ct = format.content_type();
        self.package
            .opc_mut()
            .content_types_mut()
            .set_default(ext, ct);
        self.package
            .opc_mut()
            .set_part(uri.clone(), ct, data.into());
        let rid = self.package.add_part_relationship(
            &main_uri,
            rel::IMAGE,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((rid, uri))
    }

    /// Whether any media/image parts exist under `/ppt/media/`.
    pub fn has_images(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().starts_with("/ppt/media/"))
    }

    /// Count media/image parts under `/ppt/media/`.
    pub fn image_count(&self) -> usize {
        self.list_images().len()
    }

    /// List media/image part URIs under `/ppt/media/`.
    pub fn list_images(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/media/"))
            .collect()
    }

    /// Remove media/image parts under `/ppt/media/` and related image relationships.
    pub fn clear_images(&mut self) -> Result<usize> {
        let images = self.list_images();
        let n = images.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(main_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&main_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::IMAGE
                                || r.relationship_type.contains("image")
                                || r.target.contains("media/")
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
        // also drop sheet/slide-level image rels pointing at media
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        for part_uri in part_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&part_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type == rel::IMAGE || r.target.contains("media/")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if ids.is_empty() {
                continue;
            }
            let rels = self.package.opc_mut().part_relationships_mut(&part_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in images {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Flat OPC XML string for this presentation.
    pub fn to_flat_opc_string(&self) -> Result<String> {
        use crate::opc::{progid, to_flat_opc};
        let bytes = to_flat_opc(self.package.opc(), Some(progid::POWERPOINT))?;
        String::from_utf8(bytes).map_err(|e| Error::Xml(e.to_string()))
    }

    /// Open a presentation from Flat OPC XML.
    pub fn from_flat_opc(xml: impl AsRef<[u8]>) -> Result<Self> {
        use crate::opc::from_flat_opc;
        let opc = from_flat_opc(xml)?;
        let mut settings = OpenSettings::default();
        settings.auto_save = false;
        Self::from_opc(opc, settings)
    }

    /// Normalize Strict OOXML namespaces/relationships to Transitional.
    ///
    /// Returns `(xml_replacements, relationship_replacements)`.
    pub fn rewrite_strict_to_transitional(&mut self) -> Result<(usize, usize)> {
        crate::namespace_rewrite::rewrite_package_to_transitional(self.package.opc_mut())
    }

    /// Normalize Transitional OOXML namespaces/relationships to Strict.
    pub fn rewrite_transitional_to_strict(&mut self) -> Result<(usize, usize)> {
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

    /// Set an integer custom property by name.
    pub fn set_custom_property_i4(&mut self, name: &str, value: i32) -> Result<()> {
        let mut props = self.custom_properties()?;
        props.set_i4(name, value);
        self.set_custom_properties(&props)
    }

    /// Read an integer custom property by name.
    pub fn get_custom_property_i4(&self, name: &str) -> Result<Option<i32>> {
        Ok(self
            .custom_properties()?
            .get(name)
            .and_then(|p| match &p.value {
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
        Ok(self
            .custom_properties()?
            .get(name)
            .and_then(|p| match &p.value {
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

    /// Convenience: package structure + presentation relationship/uniqueness checks.
    pub fn validate(&self) -> Result<Vec<crate::validation::ValidationError>> {
        let mut errs = self.validate_package()?;
        errs.extend(self.validate_relationships()?);
        Ok(errs)
    }

    /// Same as [`validate`](Self::validate) for PowerPoint (no separate particle pass).
    pub fn validate_full(&self) -> Result<Vec<crate::validation::ValidationError>> {
        self.validate()
    }

    /// Validate OPC package structure.
    pub fn validate_package(&self) -> Result<Vec<crate::validation::ValidationError>> {
        Ok(crate::validation::validate_package(
            self.package.opc(),
            true,
        ))
    }

    /// Validate part relationship constraints (C# `PackageValidator`).
    pub fn validate_package_constraints(&self) -> Result<Vec<crate::validation::ValidationError>> {
        Ok(crate::validation::validate_package_constraints(
            self.package.opc(),
        ))
    }

    /// Validate relationship-id attributes and unique-attribute rules in the presentation.
    pub fn validate_relationships(&self) -> Result<Vec<crate::validation::ValidationError>> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        if !self.package.opc().has_part(&pres_uri) {
            return Ok(Vec::new());
        }
        let xml = self
            .package
            .opc()
            .get_part(&pres_uri)
            .ok_or_else(|| Error::Package("presentation missing".into()))?;
        let root = parse_element(xml)?;
        let rel_rules = crate::validation::merged_relationship_rules(
            crate::validation::presentation_relationship_rules(),
        );
        let unique_rules = crate::validation::merged_unique_attribute_rules(
            crate::validation::presentation_unique_attribute_rules(),
        );
        Ok(crate::validation::validate_semantic(
            self.package.opc(),
            &pres_uri,
            &root,
            &rel_rules,
            &unique_rules,
        ))
    }

    /// Validate the presentation with the full extractable Schematron subset.
    pub fn validate_schematron(&self) -> Result<Vec<crate::validation::ValidationError>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        if !self.package.opc().has_part(&pres_uri) {
            return Ok(Vec::new());
        }
        let xml = self
            .package
            .opc()
            .get_part(&pres_uri)
            .ok_or_else(|| Error::Package("presentation missing".into()))?;
        let root = parse_element(xml)?;
        Ok(crate::validation::validate_schematron_subset(
            self.package.opc(),
            &pres_uri,
            &root,
        ))
    }

    /// Validate Schematron attribute constraints on the main document part root.
    pub fn validate_schematron_attributes(
        &self,
    ) -> Result<Vec<crate::validation::ValidationError>> {
        let main_uri = match self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let Some(data) = self.package.opc().get_part(&main_uri) else {
            return Ok(Vec::new());
        };
        let root = crate::element::parse_element(data)?;
        Ok(crate::validation::validate_schematron_attributes(&root))
    }

    /// Remove a part from the package (content-type, child rels, inbound rels).
    pub fn delete_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.package.delete_part(uri)
    }

    /// Alias for [`delete_part`](Self::delete_part).
    pub fn remove_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.delete_part(uri)
    }

    /// Delete a part and cascade to parts that become unreachable.
    pub fn delete_part_and_orphans(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.package.delete_part_and_orphans(uri)
    }

    /// Delete the part identified by relationship id on the presentation part.
    pub fn delete_part_by_id(&mut self, id: &str) -> bool {
        let source = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok();
        self.package
            .delete_part_by_id(source.as_ref(), id)
    }

    /// Delete every part with the given content type, cascading orphans.
    pub fn delete_parts_of_content_type(&mut self, content_type: &str) -> usize {
        self.package
            .delete_parts_of_content_type(content_type)
    }

    /// Recursively delete parts of a relationship type (C# `DeletePartsRecursivelyOfType` stand-in).
    pub fn delete_parts_recursively_of_relationship_type(
        &mut self,
        relationship_type: &str,
    ) -> usize {
        self.package
            .delete_parts_recursively_of_relationship_type(relationship_type)
    }

    /// Add an external relationship from the presentation part.
    pub fn add_external_relationship(
        &mut self,
        relationship_type: &str,
        external_uri: &str,
    ) -> Result<String> {
        let pres = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no presentation part".into()))?;
        Ok(self.package.add_external_relationship(
            Some(&pres),
            relationship_type,
            external_uri,
        ))
    }

    /// External relationships on the presentation part.
    pub fn external_relationships(&self) -> Vec<&crate::opc::Relationship> {
        let Ok(pres) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package.opc().external_relationships(Some(&pres))
    }

    /// Ensure [`PackageEvents`](crate::features::PackageEvents) is registered.
    pub fn package_events(&mut self) -> &crate::features::PackageEvents {
        self.package.package_events()
    }

    /// Part-container events (C# `IPartEventsFeature`).
    pub fn part_events(&mut self) -> &crate::features::PartEvents {
        self.package.part_events()
    }

    /// Child parts related from the main part (C# GetPartsOfType / Parts).
    pub fn related_parts(
        &self,
        relationship_type: Option<&str>,
    ) -> Vec<crate::opc::RelatedPart> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package
            .opc()
            .related_parts(Some(&main), relationship_type)
    }

    /// Allocate a unique part URI under the main part.
    pub fn create_unique_part_uri(
        &self,
        content_type: &str,
        target_path: &str,
        target_name: &str,
        target_ext: &str,
    ) -> Result<PackUri> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package.opc().create_unique_part_uri(
            content_type,
            &main,
            target_path,
            target_name,
            target_ext,
        )
    }



    /// Delete multiple parts by URI (C# `DeleteParts`).
    pub fn delete_parts(&mut self, uris: &[PackUri]) -> usize {
        self.package.delete_parts(uris)
    }

    /// C# `StrictRelationshipFound`.
    pub fn strict_relationship_found(&self) -> bool {
        self.package.strict_relationship_found()
    }

    /// Hyperlink relationships on the main part (C# `HyperlinkRelationships`).
    pub fn hyperlink_relationships(&self) -> Vec<crate::opc::HyperlinkRelationship> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package
            .hyperlink_relationships(Some(&main))
    }

    /// Relationship id of a part under the main part (C# `GetIdOfPart`).
    pub fn get_id_of_part(&self, part_uri: &PackUri) -> Option<String> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package.get_id_of_part(Some(&main), part_uri)
    }

    /// Part URI for relationship id on the main part (C# `GetPartById`).
    pub fn get_part_by_id(&self, id: &str) -> Option<PackUri> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package.get_part_by_id(Some(&main), id)
    }

    /// Change the relationship id of a child part (C# `ChangeIdOfPart`).
    pub fn change_id_of_part(&mut self, part_uri: &PackUri, new_id: &str) -> Result<String> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package
            .change_id_of_part(Some(&main), part_uri, new_id)
    }

    /// Child parts as IdPartPair under the main part (C# `Parts`).
    pub fn id_part_pairs(&self) -> Vec<crate::opc::IdPartPair> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package.id_part_pairs(Some(&main))
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


    /// Add a data-part reference from the main part (C# `AddDataPartReferenceRelationship`).
    pub fn add_data_part_reference_relationship(
        &mut self,
        data_part: &crate::opc::DataPart,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<crate::opc::DataPartReferenceRelationship> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package.add_data_part_reference_relationship(
            &main,
            data_part,
            relationship_type,
            id,
        )
    }

    /// Data-part references on the main part.
    pub fn data_part_reference_relationships(
        &self,
    ) -> Vec<crate::opc::DataPartReferenceRelationship> {
        let Ok(main) = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
        else {
            return Vec::new();
        };
        self.package
            .data_part_reference_relationships(Some(&main))
    }

    /// Delete a reference relationship by id on the main part
    /// (C# `DeleteReferenceRelationship`).
    pub fn delete_reference_relationship(&mut self, id: &str) -> Option<crate::opc::Relationship> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package
            .delete_reference_relationship(Some(&main), id)
    }

    /// Get a reference relationship by id on the main part.
    pub fn get_reference_relationship(&self, id: &str) -> Option<crate::opc::ReferenceRelationship> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .ok()?;
        self.package
            .get_reference_relationship(Some(&main), id)
    }

    /// Create a relationship from the main part to an existing part
    /// (C# `CreateRelationshipToPart` same-package).
    pub fn create_relationship_to_part(
        &mut self,
        target: &PackUri,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<String> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        self.package
            .create_relationship_to_part(&main, target, relationship_type, id)
    }

    /// Create an [`ExtendedPart`] under `ppt/udata/` with auto URI.
    pub fn create_extended_part(
        &mut self,
        content_type_str: &str,
        relationship_type: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, crate::packaging::ExtendedPart)> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        let mut index = 1u32;
        let part_uri = loop {
            let candidate = PackUri::new(format!("/ppt/udata/data{index}.dat"));
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

    /// Add a new typed child part under the presentation via generated PartInfo
    /// (C# `AddNewPart<T>` shell).
    pub fn add_typed_child_part(
        &mut self,
        part_name: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<crate::packaging::TypedPart> {
        let main = self
            .package
            .opc()
            .main_part_uri(crate::namespace::rel::OFFICE_DOCUMENT)
            .map_err(|_| Error::Package("no main part".into()))?;
        crate::packaging::add_typed_part(
            &mut self.package,
            &main,
            Some("PresentationPart"),
            part_name,
            data,
        )
    }

    /// Clone this presentation into a new in-memory package (deep copy of all parts).
    ///
    /// C# `CloneableExtensions.Clone()` (MemoryStream).
    pub fn clone_document(&self) -> Result<Self> {
        let bytes = self.to_bytes()?;
        Self::open_bytes(bytes)
    }

    /// Clone to a new file path (C# `Clone(string path)`).
    pub fn clone_to_path(&self, path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = self.to_bytes()?;
        let mut cloned = Self::open_bytes(bytes)?;
        *cloned.settings_mut() = self.settings().clone();
        cloned.save_as(path)?;
        let settings = cloned.settings().clone();
        drop(cloned);
        Self::open_with_settings(path, true, settings)
    }

    /// Clone package ZIP bytes.
    pub fn clone_to_bytes(&self) -> Result<Vec<u8>> {
        self.to_bytes()
    }

    /// Clone and write ZIP bytes to a writer.
    pub fn clone_to_writer<W: std::io::Write>(&self, mut writer: W) -> Result<()> {
        let bytes = self.clone_to_bytes()?;
        writer.write_all(&bytes)?;
        Ok(())
    }

    /// Add an arbitrary extended part related from the presentation.
    ///
    /// Corresponds to C# `ExtendedPart`. Returns `(uri, relationship_id)`.
    pub fn add_extended_part(
        &mut self,
        uri: &str,
        content_type_str: &str,
        relationship_type: &str,
        data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, String)> {
        let pres_uri = self.ensure_presentation()?;
        let part_uri = PackUri::new(if uri.starts_with('/') {
            uri.to_string()
        } else {
            format!("/{uri}")
        });
        self.package
            .opc_mut()
            .set_part(part_uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &pres_uri,
            relationship_type,
            &part_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((part_uri, rid))
    }

    fn ensure_presentation(&mut self) -> Result<PackUri> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        if self.package.opc().has_part(&pres_uri) {
            return Ok(pres_uri);
        }
        self.package.set_part(
            pres_uri.clone(),
            self.document_type.content_type(),
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><p:presentation xmlns:p=\"http://schemas.openxmlformats.org/presentationml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><p:sldIdLst/></p:presentation>".to_vec(),
        );
        self.package.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &pres_uri,
            RelationshipTargetMode::Internal,
        );
        Ok(pres_uri)
    }

    fn rewrite_presentation(&mut self) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let ids: Vec<_> = self
            .slides
            .iter()
            .map(|s| slide_id(s.id, &s.relationship_id))
            .collect();
        let master_ids: Vec<_> = self
            .masters
            .iter()
            .map(|m| slide_master_id(m.id, &m.relationship_id))
            .collect();
        let (cx, cy) = SLIDE_SIZE_16_9;
        // Office expects sldMasterIdLst / sldIdLst before sldSz (see ECMA-376 + EmptySlide.pptx).
        let mut kids = Vec::new();
        if !master_ids.is_empty() {
            kids.push(slide_master_id_list(master_ids));
        }
        kids.push(slide_id_list(ids));
        kids.push(slide_size(cx, cy));
        kids.push(notes_size(6_858_000, 9_144_000));
        // defaultTextStyle from a real Office PPTX (required for robust open)
        if let Ok(dts) = crate::element::parse_element(
            include_str!("ppt_templates/defaultTextStyle.xml").as_bytes(),
        ) {
            kids.push(dts);
        }
        let pres = presentation(kids);
        let xml = write_element(&pres)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    pub fn slides(&self) -> &[SlideInfo] {
        &self.slides
    }

    pub fn masters(&self) -> &[SlideMasterInfo] {
        &self.masters
    }

    pub fn layouts(&self) -> &[SlideLayoutInfo] {
        &self.layouts
    }

    /// Master part URIs.
    pub fn list_masters(&self) -> Vec<PackUri> {
        self.masters.iter().map(|m| m.uri.clone()).collect()
    }

    /// Layout part URIs.
    pub fn list_layouts(&self) -> Vec<PackUri> {
        self.layouts.iter().map(|l| l.uri.clone()).collect()
    }

    /// Set the layout type attribute (`p:sldLayout/@type`), e.g. `"blank"`, `"title"`, `"obj"`.
    pub fn set_slide_layout_type(&mut self, layout_index: usize, layout_type: &str) -> Result<()> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let mut root = if let Some(data) = self.package.opc().get_part(&layout.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(layout.uri.to_string()));
        };
        root.set_attribute("type", layout_type);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(layout.uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
        Ok(())
    }

    /// Read `p:sldLayout/@type`.
    pub fn slide_layout_type(&self, layout_index: usize) -> Result<Option<String>> {
        let layout = self
            .layouts
            .get(layout_index)
            .ok_or_else(|| Error::Package(format!("layout index {layout_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("type").map(|s| s.to_string()))
    }

    /// Set whether a layout is preserved (`p:sldLayout/@preserve`).
    pub fn set_slide_layout_preserve(&mut self, layout_index: usize, preserve: bool) -> Result<()> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let mut root = if let Some(data) = self.package.opc().get_part(&layout.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(layout.uri.to_string()));
        };
        root.set_attribute("preserve", if preserve { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(layout.uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
        Ok(())
    }

    /// Whether a layout is preserved (defaults false when absent).
    pub fn slide_layout_preserve(&self, layout_index: usize) -> Result<bool> {
        let layout = self
            .layouts
            .get(layout_index)
            .ok_or_else(|| Error::Package(format!("layout index {layout_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("preserve")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set layout matching name (`p:sldLayout/@matchingName`).
    pub fn set_slide_layout_matching_name(
        &mut self,
        layout_index: usize,
        name: &str,
    ) -> Result<()> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let mut root = if let Some(data) = self.package.opc().get_part(&layout.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(layout.uri.to_string()));
        };
        root.set_attribute("matchingName", name);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(layout.uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
        Ok(())
    }

    /// Read layout matching name.
    pub fn slide_layout_matching_name(&self, layout_index: usize) -> Result<Option<String>> {
        let layout = self
            .layouts
            .get(layout_index)
            .ok_or_else(|| Error::Package(format!("layout index {layout_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("matchingName").map(|s| s.to_string()))
    }

    /// Whether layout matchingName is set.
    pub fn has_slide_layout_matching_name(&self, layout_index: usize) -> Result<bool> {
        Ok(self.slide_layout_matching_name(layout_index)?.is_some())
    }

    /// Clear layout `@matchingName`.
    pub fn clear_slide_layout_matching_name(&mut self, layout_index: usize) -> Result<bool> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("matchingName").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "matchingName");
        self.package.set_part(
            layout.uri,
            content_type::PRESENTATION_SLIDE_LAYOUT,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear layout `@preserve` attribute.
    pub fn clear_slide_layout_preserve(&mut self, layout_index: usize) -> Result<bool> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("preserve").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "preserve");
        self.package.set_part(
            layout.uri,
            content_type::PRESENTATION_SLIDE_LAYOUT,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear layout `@userDrawn` attribute.
    pub fn clear_slide_layout_user_drawn(&mut self, layout_index: usize) -> Result<bool> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("userDrawn").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "userDrawn");
        self.package.set_part(
            layout.uri,
            content_type::PRESENTATION_SLIDE_LAYOUT,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear layout `@type` attribute.
    pub fn clear_slide_layout_type(&mut self, layout_index: usize) -> Result<bool> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("type").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "type");
        self.package.set_part(
            layout.uri,
            content_type::PRESENTATION_SLIDE_LAYOUT,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear layout `@showMasterSp` attribute.
    pub fn clear_slide_layout_show_master_shapes(&mut self, layout_index: usize) -> Result<bool> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("showMasterSp").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "showMasterSp");
        self.package.set_part(
            layout.uri,
            content_type::PRESENTATION_SLIDE_LAYOUT,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear layout `@showMasterPhAnim` attribute.
    pub fn clear_slide_layout_show_master_ph_anim(&mut self, layout_index: usize) -> Result<bool> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("showMasterPhAnim").is_none() {
            return Ok(false);
        }
        root.attributes
            .retain(|a| a.local_name != "showMasterPhAnim");
        self.package.set_part(
            layout.uri,
            content_type::PRESENTATION_SLIDE_LAYOUT,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Set whether a layout is user-drawn (`p:sldLayout/@userDrawn`).
    pub fn set_slide_layout_user_drawn(
        &mut self,
        layout_index: usize,
        user_drawn: bool,
    ) -> Result<()> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let mut root = if let Some(data) = self.package.opc().get_part(&layout.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(layout.uri.to_string()));
        };
        root.set_attribute("userDrawn", if user_drawn { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(layout.uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
        Ok(())
    }

    /// Whether a layout is user-drawn (defaults false when absent).
    pub fn slide_layout_user_drawn(&self, layout_index: usize) -> Result<bool> {
        let layout = self
            .layouts
            .get(layout_index)
            .ok_or_else(|| Error::Package(format!("layout index {layout_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("userDrawn")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set whether a slide master is preserved (`p:sldMaster/@preserve`).
    pub fn set_slide_master_preserve(&mut self, master_index: usize, preserve: bool) -> Result<()> {
        let master =
            self.masters.get(master_index).cloned().ok_or_else(|| {
                Error::Package(format!("master index {master_index} out of range"))
            })?;
        let mut root = if let Some(data) = self.package.opc().get_part(&master.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(master.uri.to_string()));
        };
        root.set_attribute("preserve", if preserve { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(master.uri, content_type::PRESENTATION_SLIDE_MASTER, xml);
        Ok(())
    }

    /// Whether a slide master is preserved (defaults false when absent).
    pub fn slide_master_preserve(&self, master_index: usize) -> Result<bool> {
        let master = self
            .masters
            .get(master_index)
            .ok_or_else(|| Error::Package(format!("master index {master_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&master.uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("preserve")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Clear master `@preserve` attribute.
    pub fn clear_slide_master_preserve(&mut self, master_index: usize) -> Result<bool> {
        let master =
            self.masters.get(master_index).cloned().ok_or_else(|| {
                Error::Package(format!("master index {master_index} out of range"))
            })?;
        let Some(data) = self.package.opc().get_part(&master.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("preserve").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "preserve");
        self.package.set_part(
            master.uri,
            content_type::PRESENTATION_SLIDE_MASTER,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Install PowerPoint-required companion parts: presProps, viewProps, tableStyles.
    fn install_office_presentation_scaffold(&mut self) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        // Install in Office order around theme: presProps, viewProps, (theme by caller), tableStyles.
        // This helper installs props + tableStyles; theme relationship is separate.
        let early = [
            (
                "/ppt/presProps.xml",
                content_type::PRESENTATION_PROPS,
                rel::PRES_PROPS,
                include_str!("ppt_templates/presProps.xml"),
            ),
            (
                "/ppt/viewProps.xml",
                content_type::PRESENTATION_VIEW_PROPS,
                rel::VIEW_PROPS,
                include_str!("ppt_templates/viewProps.xml"),
            ),
        ];
        for (path, ct, rel_ty, xml) in early {
            let uri = PackUri::new(path);
            if !self.package.opc().has_part(&uri) {
                self.package
                    .opc_mut()
                    .set_part(uri.clone(), ct, xml.as_bytes().to_vec());
            }
            if self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .and_then(|r| r.get_by_type(rel_ty))
                .is_none()
            {
                let _ = self.package.add_part_relationship(
                    &pres_uri,
                    rel_ty,
                    &uri,
                    RelationshipTargetMode::Internal,
                );
            }
        }
        // Theme relationship (part already created with master)
        let theme_uri = PackUri::new("/ppt/theme/theme1.xml");
        if self.package.opc().has_part(&theme_uri)
            && self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .and_then(|r| r.get_by_type(rel::THEME))
                .is_none()
        {
            let _ = self.package.add_part_relationship(
                &pres_uri,
                rel::THEME,
                &theme_uri,
                RelationshipTargetMode::Internal,
            );
        }
        let table_uri = PackUri::new("/ppt/tableStyles.xml");
        if !self.package.opc().has_part(&table_uri) {
            self.package.set_part(
                table_uri.clone(),
                content_type::PRESENTATION_TABLE_STYLES,
                include_str!("ppt_templates/tableStyles.xml")
                    .as_bytes()
                    .to_vec(),
            );
        }
        if self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|r| r.get_by_type(rel::TABLE_STYLES))
            .is_none()
        {
            let _ = self.package.add_part_relationship(
                &pres_uri,
                rel::TABLE_STYLES,
                &table_uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok(())
    }

    /// Ensure a blank slide master + layout exist. Returns `(master, layout)`.
    pub fn ensure_default_master_layout(&mut self) -> Result<(SlideMasterInfo, SlideLayoutInfo)> {
        if let Some(m) = self.masters.first().cloned() {
            if let Some(l) = self
                .layouts
                .iter()
                .find(|l| l.uri.as_str().ends_with("slideLayout7.xml"))
                .cloned()
                .or_else(|| self.layouts.first().cloned())
            {
                return Ok((m, l));
            }
        }
        self.add_blank_master_with_layout()
    }

    /// Add a blank slide master with one blank layout.
    pub fn add_blank_master_with_layout(&mut self) -> Result<(SlideMasterInfo, SlideLayoutInfo)> {
        let pres_uri = self.ensure_presentation()?;

        // Full Office slide master + 11 layouts (binary-identical to a PowerPoint-created PPTX).
        // Install master/theme before scaffold props so presentation rIds are closer to Office
        // (rId1=master, then later slide, then props/theme/tableStyles).
        let master_uri = PackUri::new("/ppt/slideMasters/slideMaster1.xml");
        let theme_uri = PackUri::new("/ppt/theme/theme1.xml");

        if !self.package.opc().has_part(&theme_uri) {
            self.package.set_part(
                theme_uri.clone(),
                content_type::THEME,
                include_str!("ppt_templates/theme1.xml").as_bytes().to_vec(),
            );
        }
        // Theme presentation relationship is added after master (Office rId order).

        // Layouts 1..11
        for i in 1u32..=11 {
            let layout_uri = PackUri::new(format!("/ppt/slideLayouts/slideLayout{i}.xml"));
            if !self.package.opc().has_part(&layout_uri) {
                let xml = match i {
                    1 => include_str!("ppt_templates/slideLayout1.xml"),
                    2 => include_str!("ppt_templates/slideLayout2.xml"),
                    3 => include_str!("ppt_templates/slideLayout3.xml"),
                    4 => include_str!("ppt_templates/slideLayout4.xml"),
                    5 => include_str!("ppt_templates/slideLayout5.xml"),
                    6 => include_str!("ppt_templates/slideLayout6.xml"),
                    7 => include_str!("ppt_templates/slideLayout7.xml"),
                    8 => include_str!("ppt_templates/slideLayout8.xml"),
                    9 => include_str!("ppt_templates/slideLayout9.xml"),
                    10 => include_str!("ppt_templates/slideLayout10.xml"),
                    11 => include_str!("ppt_templates/slideLayout11.xml"),
                    _ => unreachable!(),
                };
                self.package.set_part(
                    layout_uri.clone(),
                    content_type::PRESENTATION_SLIDE_LAYOUT,
                    xml.as_bytes().to_vec(),
                );
            }
            // layout → master (Office template: rId1 → slideMaster1)
            if self
                .package
                .opc()
                .part_relationships(&layout_uri)
                .map(|r| r.is_empty())
                .unwrap_or(true)
            {
                let parsed = Relationships::parse(
                    include_str!("ppt_templates/slideLayout1.xml.rels").as_bytes(),
                )?;
                *self.package.opc_mut().part_relationships_mut(&layout_uri) = parsed;
            }
        }

        // Master XML
        if !self.package.opc().has_part(&master_uri) {
            self.package.set_part(
                master_uri.clone(),
                content_type::PRESENTATION_SLIDE_MASTER,
                include_str!("ppt_templates/slideMaster1.xml")
                    .as_bytes()
                    .to_vec(),
            );
        }
        // Master relationships: load Office template (rId1..rId11 layouts, rId12 theme)
        {
            let master_rels = self.package.opc_mut().part_relationships_mut(&master_uri);
            if master_rels.is_empty() {
                let parsed = Relationships::parse(
                    include_str!("ppt_templates/slideMaster1.xml.rels").as_bytes(),
                )?;
                *master_rels = parsed;
            }
        }

        // Presentation → master
        let master_rel = if let Some(r) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::SLIDE_MASTER).map(|r| r.id.clone()))
        {
            r
        } else {
            self.package.add_part_relationship(
                &pres_uri,
                rel::SLIDE_MASTER,
                &master_uri,
                RelationshipTargetMode::Internal,
            )
        };

        // presentation→theme is deferred until after the first slide (Office rId order).

        // Register all 11 layouts; default for blank slides is layout7 (type="blank").
        let master_info = SlideMasterInfo {
            relationship_id: master_rel,
            uri: master_uri,
            id: 2_147_483_648, // Office default
        };
        if self.masters.is_empty() {
            self.masters.push(master_info.clone());
        }
        // layout ids from Office master sldLayoutIdLst: 2147483649 + (i-1)
        if self.layouts.is_empty() {
            for i in 1u32..=11 {
                self.layouts.push(SlideLayoutInfo {
                    relationship_id: format!("rId{i}"),
                    uri: PackUri::new(format!("/ppt/slideLayouts/slideLayout{i}.xml")),
                    id: 2_147_483_648 + i,
                    master_relationship_id: master_info.relationship_id.clone(),
                });
            }
        }
        // Prefer blank layout (slideLayout7) as the returned default for free-form content.
        let layout_info = self
            .layouts
            .iter()
            .find(|l| l.uri.as_str().ends_with("slideLayout7.xml"))
            .cloned()
            .unwrap_or_else(|| self.layouts[0].clone());
        self.next_master_index = self.next_master_index.max(2);
        self.next_layout_index = self.next_layout_index.max(12);
        self.next_master_id = self.next_master_id.max(2_147_483_649);
        self.next_layout_id = self.next_layout_id.max(2_147_483_660);
        // Scaffold (presProps/viewProps/tableStyles/theme rel) is installed after the first
        // slide so presentation.xml.rels order matches Office: master, slide, props...
        self.rewrite_presentation()?;
        Ok((master_info, layout_info))
    }

    /// Add a slide linked to the default blank layout (creates master/layout if needed).
    pub fn add_slide_with_layout(&mut self, slide_root: OpenXmlElement) -> Result<SlideInfo> {
        let (_, layout) = self.ensure_default_master_layout()?;
        let info = self.add_slide(slide_root)?;
        // Relate slide → layout
        let _ = self.package.add_part_relationship(
            &info.uri,
            rel::SLIDE_LAYOUT,
            &layout.uri,
            RelationshipTargetMode::Internal,
        );
        // After first slide exists, install Office scaffold so rIds look like:
        // rId1=master, rId2=slide, rId3=presProps, rId4=viewProps, rId5=theme, rId6=tableStyles
        self.install_office_presentation_scaffold()?;
        Ok(info)
    }

    /// Attach a notes slide with the given text to an existing slide (by index).
    pub fn add_notes_to_slide(&mut self, slide_index: usize, text: &str) -> Result<PackUri> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;

        let mut index = 1u32;
        let notes_uri = loop {
            let candidate = PackUri::new(format!("/ppt/notesSlides/notesSlide{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };

        let xml = write_element(&notes_slide(text))?;
        self.package.set_part(
            notes_uri.clone(),
            content_type::PRESENTATION_NOTES_SLIDE,
            xml,
        );

        // slide → notesSlide
        self.package.add_part_relationship(
            &slide_info.uri,
            rel::NOTES_SLIDE,
            &notes_uri,
            RelationshipTargetMode::Internal,
        );
        // notesSlide → slide
        self.package.add_part_relationship(
            &notes_uri,
            rel::SLIDE,
            &slide_info.uri,
            RelationshipTargetMode::Internal,
        );

        Ok(notes_uri)
    }

    /// Replace notes text for a slide (clears existing notes then re-adds).
    pub fn set_notes_text(&mut self, slide_index: usize, text: &str) -> Result<PackUri> {
        let _ = self.clear_notes(slide_index)?;
        self.add_notes_to_slide(slide_index, text)
    }

    /// Number of slides in the presentation.
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Collect texts from every slide (one `Vec<String>` per slide).
    /// Whether the presentation has no slides.
    pub fn is_presentation_empty(&self) -> bool {
        self.slides.is_empty()
    }

    pub fn all_slide_texts(&self) -> Result<Vec<Vec<String>>> {
        let mut out = Vec::with_capacity(self.slides.len());
        for i in 0..self.slides.len() {
            out.push(self.slide_texts(i)?);
        }
        Ok(out)
    }

    /// Count shapes (`p:sp` / `p:pic` / `p:graphicFrame`) on a slide.
    pub fn shape_count(&self, slide_index: usize) -> Result<usize> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| {
                e.local_name == "sp" || e.local_name == "pic" || e.local_name == "graphicFrame"
            })
            .count())
    }

    /// List shape ids and names from `p:cNvPr` on a slide as `(id, name)`.
    pub fn list_shape_ids(&self, slide_index: usize) -> Result<Vec<(u32, String)>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "cNvPr")
            .filter_map(|e| {
                let id = e.get_attribute("id")?.parse().ok()?;
                let name = e.get_attribute("name").unwrap_or("").to_string();
                Some((id, name))
            })
            .collect())
    }

    /// List shape names on a slide (order of `cNvPr` descendants).
    pub fn list_shape_names(&self, slide_index: usize) -> Result<Vec<String>> {
        Ok(self
            .list_shape_ids(slide_index)?
            .into_iter()
            .map(|(_, n)| n)
            .collect())
    }

    /// Whether a slide has any shapes with `cNvPr` ids.
    pub fn has_shape_ids(&self, slide_index: usize) -> Result<bool> {
        Ok(!self.list_shape_ids(slide_index)?.is_empty())
    }

    /// Count shapes with `cNvPr` on a slide.
    pub fn shape_id_count(&self, slide_index: usize) -> Result<usize> {
        Ok(self.list_shape_ids(slide_index)?.len())
    }

    /// Rename a shape by id (`cNvPr/@name`). Returns whether found.
    pub fn set_shape_name(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        name: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, name: &str, found: &mut bool) {
            if el.local_name == "cNvPr" {
                if el.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id) {
                    el.set_attribute("name", name);
                    *found = true;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, name, found);
            }
        }
        visit(&mut root, shape_id, name, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Remove a shape (or pic/cxnSp/grpSp/graphicFrame) by `cNvPr/@id`.
    ///
    /// Returns whether a matching shape was removed from the slide.
    pub fn remove_shape_by_id(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            if el.local_name == "cNvPr" {
                if el.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id) {
                    return true;
                }
            }
            el.children.iter().any(|c| contains_id(c, shape_id))
        }
        fn remove_from(el: &mut OpenXmlElement, shape_id: u32) -> bool {
            let before = el.children.len();
            el.children.retain(|c| {
                // Shape containers that carry cNvPr
                let is_shape_like = matches!(
                    c.local_name.as_str(),
                    "sp" | "pic" | "cxnSp" | "grpSp" | "graphicFrame" | "contentPart"
                );
                if is_shape_like && contains_id(c, shape_id) {
                    return false;
                }
                true
            });
            if el.children.len() < before {
                return true;
            }
            for c in el.children.iter_mut() {
                if remove_from(c, shape_id) {
                    return true;
                }
            }
            false
        }
        let found = remove_from(&mut root, shape_id);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether a shape with the given id exists on the slide.
    pub fn has_shape(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self
            .list_shape_ids(slide_index)?
            .iter()
            .any(|(id, _)| *id == shape_id))
    }

    /// Look up a shape name by id on a slide.
    pub fn shape_name(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        Ok(self
            .list_shape_ids(slide_index)?
            .into_iter()
            .find(|(id, _)| *id == shape_id)
            .map(|(_, n)| n))
    }

    /// Count shapes on a slide whose `cNvPr/@name` equals `name`.
    pub fn shape_count_by_name(&self, slide_index: usize, name: &str) -> Result<usize> {
        Ok(self
            .list_shape_ids(slide_index)?
            .into_iter()
            .filter(|(_, n)| n == name)
            .count())
    }

    /// Whether any shape on the slide has the given name.
    pub fn has_shape_named(&self, slide_index: usize, name: &str) -> Result<bool> {
        Ok(self.shape_count_by_name(slide_index, name)? > 0)
    }

    /// List all shapes across slides as `(slide_index, shape_id, name)`.
    pub fn list_all_shape_ids(&self) -> Result<Vec<(usize, u32, String)>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            for (id, name) in self.list_shape_ids(i)? {
                out.push((i, id, name));
            }
        }
        Ok(out)
    }

    /// Whether any slide has shapes (via list_all_shape_ids).
    pub fn has_any_shapes(&self) -> Result<bool> {
        Ok(!self.list_all_shape_ids()?.is_empty())
    }

    /// Collect text under the shape with `cNvPr/@id == shape_id` on a slide.
    pub fn shape_text(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn find_shape<'a>(el: &'a OpenXmlElement, shape_id: u32) -> Option<&'a OpenXmlElement> {
            let is_shape_like = matches!(
                el.local_name.as_str(),
                "sp" | "pic" | "cxnSp" | "grpSp" | "graphicFrame" | "contentPart"
            );
            if is_shape_like {
                let has = el.descendants().any(|e| {
                    e.local_name == "cNvPr"
                        && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
                });
                if has {
                    return Some(el);
                }
            }
            for c in &el.children {
                if let Some(f) = find_shape(c, shape_id) {
                    return Some(f);
                }
            }
            None
        }
        Ok(find_shape(&root, shape_id).map(|s| {
            s.descendants()
                .filter(|e| e.local_name == "t")
                .filter_map(|e| e.text_value().map(|s| s.to_string()))
                .collect::<Vec<_>>()
                .join("")
        }))
    }

    /// Whether a shape has non-empty text content.
    pub fn has_shape_text(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self
            .shape_text(slide_index, shape_id)?
            .map(|t| !t.is_empty())
            .unwrap_or(false))
    }

    /// Set `cNvPr/@hidden` on a shape. Returns whether the shape was found.
    /// Clear text under a shape (sets empty string). Returns whether the shape had text.
    pub fn clear_shape_text(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let had = self.has_shape_text(slide_index, shape_id)?;
        if had {
            self.set_shape_text(slide_index, shape_id, "")?;
        }
        Ok(had)
    }

    /// Clear text on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_text(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_text(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// List shape texts as `(shape_id, text)` for shapes that have text.
    pub fn list_shape_texts(&self, slide_index: usize) -> Result<Vec<(u32, String)>> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut out = Vec::new();
        for (id, _) in ids {
            if let Some(t) = self.shape_text(slide_index, id)? {
                if !t.is_empty() {
                    out.push((id, t));
                }
            }
        }
        Ok(out)
    }

    /// Read first run font size (hundredths of a point) under a shape text body.
    pub fn shape_font_size(&self, slide_index: usize, shape_id: u32) -> Result<Option<u32>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                for d in e.descendants() {
                    if d.local_name == "rPr" {
                        if let Some(sz) = d.get_attribute("sz").and_then(|s| s.parse().ok()) {
                            return Ok(Some(sz));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether the shape has an explicit run font size.
    pub fn has_shape_font_size(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_font_size(slide_index, shape_id)?.is_some())
    }

    /// Set font size (hundredths of a point, e.g. 1800 = 18pt) on all `a:rPr` under the shape.
    pub fn set_shape_font_size(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        size_hundredths: u32,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, sz: u32, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn set_rpr(el: &mut OpenXmlElement, sz: u32, found: &mut bool) {
                    if el.local_name == "rPr" {
                        el.set_attribute("sz", sz.to_string());
                        *found = true;
                    }
                    for c in el.children.iter_mut() {
                        set_rpr(c, sz, found);
                    }
                }
                set_rpr(el, sz, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, sz, found);
            }
        }
        visit(&mut root, shape_id, size_hundredths, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear explicit `sz` on all `a:rPr` under the shape.
    pub fn clear_shape_font_size(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn clear_rpr(el: &mut OpenXmlElement, found: &mut bool) {
                    if el.local_name == "rPr" {
                        let before = el.attributes.len();
                        el.attributes.retain(|a| a.local_name != "sz");
                        if el.attributes.len() < before {
                            *found = true;
                        }
                    }
                    for c in el.children.iter_mut() {
                        clear_rpr(c, found);
                    }
                }
                clear_rpr(el, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether any run under the shape is bold (`a:rPr/@b`).
    /// Clear font size on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_font_size(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_font_size(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shape_bold(&self, slide_index: usize, shape_id: u32) -> Result<Option<bool>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                for d in e.descendants() {
                    if d.local_name == "rPr" {
                        if let Some(b) = d.get_attribute("b") {
                            return Ok(Some(b == "1" || b.eq_ignore_ascii_case("true")));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether bold is explicitly set on the shape text.
    pub fn has_shape_bold(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_bold(slide_index, shape_id)?.is_some())
    }

    /// Set bold on all `a:rPr` under the shape.
    pub fn set_shape_bold(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        bold: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, bold: bool, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn set_b(el: &mut OpenXmlElement, bold: bool, found: &mut bool) {
                    if el.local_name == "rPr" {
                        if bold {
                            el.set_attribute("b", "1");
                        } else {
                            el.attributes.retain(|a| a.local_name != "b");
                        }
                        *found = true;
                    }
                    for c in el.children.iter_mut() {
                        set_b(c, bold, found);
                    }
                }
                set_b(el, bold, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, bold, found);
            }
        }
        visit(&mut root, shape_id, bold, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear bold attribute on shape text runs.
    pub fn clear_shape_bold(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_bold(slide_index, shape_id, false)
    }

    /// Whether italic is explicitly set on the shape text (`a:rPr/@i`).
    /// Clear bold on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_bold(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_bold(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shape_italic(&self, slide_index: usize, shape_id: u32) -> Result<Option<bool>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                for d in e.descendants() {
                    if d.local_name == "rPr" {
                        if let Some(i) = d.get_attribute("i") {
                            return Ok(Some(i == "1" || i.eq_ignore_ascii_case("true")));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether italic is explicitly set.
    pub fn has_shape_italic(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_italic(slide_index, shape_id)?.is_some())
    }

    /// Set italic on all `a:rPr` under the shape.
    pub fn set_shape_italic(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        italic: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, italic: bool, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn set_i(el: &mut OpenXmlElement, italic: bool, found: &mut bool) {
                    if el.local_name == "rPr" {
                        if italic {
                            el.set_attribute("i", "1");
                        } else {
                            el.attributes.retain(|a| a.local_name != "i");
                        }
                        *found = true;
                    }
                    for c in el.children.iter_mut() {
                        set_i(c, italic, found);
                    }
                }
                set_i(el, italic, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, italic, found);
            }
        }
        visit(&mut root, shape_id, italic, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear italic attribute on shape text runs.
    pub fn clear_shape_italic(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_italic(slide_index, shape_id, false)
    }

    /// Read first run solid font color RGB under a shape.
    /// Clear italic on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_italic(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_italic(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shape_font_color(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                for d in e.descendants() {
                    if d.local_name == "rPr" {
                        if let Some(sf) = d.child("solidFill") {
                            if let Some(srgb) = sf.child("srgbClr") {
                                return Ok(srgb.get_attribute("val").map(|s| s.to_string()));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether the shape has an explicit run font color.
    pub fn has_shape_font_color(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_font_color(slide_index, shape_id)?.is_some())
    }

    /// Set solid font color (6-hex) on all `a:rPr` under the shape.
    pub fn set_shape_font_color(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        rgb: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, rgb: &str, a: &str, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn set_color(el: &mut OpenXmlElement, rgb: &str, a: &str, found: &mut bool) {
                    if el.local_name == "rPr" {
                        el.children.retain(|c| c.local_name != "solidFill");
                        el.append_child(OpenXmlElement::new("a", a, "solidFill").with_child(
                            OpenXmlElement::new("a", a, "srgbClr").with_attribute("val", rgb),
                        ));
                        *found = true;
                    }
                    for c in el.children.iter_mut() {
                        set_color(c, rgb, a, found);
                    }
                }
                set_color(el, rgb, a, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, rgb, a, found);
            }
        }
        visit(&mut root, shape_id, rgb, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear solid font color from shape runs.
    pub fn clear_shape_font_color(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn clear_c(el: &mut OpenXmlElement, found: &mut bool) {
                    if el.local_name == "rPr" {
                        let before = el.children.len();
                        el.children.retain(|c| c.local_name != "solidFill");
                        if el.children.len() < before {
                            *found = true;
                        }
                    }
                    for c in el.children.iter_mut() {
                        clear_c(c, found);
                    }
                }
                clear_c(el, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether underline is set on shape text (`a:rPr/@u`).
    /// Clear font color on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_font_color(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_font_color(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shape_underline(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                for d in e.descendants() {
                    if d.local_name == "rPr" {
                        if let Some(u) = d.get_attribute("u") {
                            return Ok(Some(u.to_string()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether underline is explicitly set.
    pub fn has_shape_underline(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_underline(slide_index, shape_id)?.is_some())
    }

    /// Set underline on all `a:rPr` (e.g. `"sng"`, `"dbl"`). Pass `None` to clear.
    pub fn set_shape_underline(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        underline: Option<&str>,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            underline: Option<&str>,
            found: &mut bool,
        ) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn set_u(el: &mut OpenXmlElement, underline: Option<&str>, found: &mut bool) {
                    if el.local_name == "rPr" {
                        if let Some(u) = underline {
                            el.set_attribute("u", u);
                        } else {
                            el.attributes.retain(|a| a.local_name != "u");
                        }
                        *found = true;
                    }
                    for c in el.children.iter_mut() {
                        set_u(c, underline, found);
                    }
                }
                set_u(el, underline, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, underline, found);
            }
        }
        visit(&mut root, shape_id, underline, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear underline on shape text runs.
    pub fn clear_shape_underline(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_underline(slide_index, shape_id, None)
    }

    /// Read first latin typeface under a shape's run properties.
    /// Clear underline on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_underline(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_underline(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shape_font_name(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                for d in e.descendants() {
                    if d.local_name == "latin" {
                        if let Some(tf) = d.get_attribute("typeface") {
                            return Ok(Some(tf.to_string()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether the shape has an explicit latin typeface.
    pub fn has_shape_font_name(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_font_name(slide_index, shape_id)?.is_some())
    }

    /// Set latin typeface on all `a:rPr` under the shape.
    pub fn set_shape_font_name(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        typeface: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            typeface: &str,
            a: &str,
            found: &mut bool,
        ) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn set_tf(el: &mut OpenXmlElement, typeface: &str, a: &str, found: &mut bool) {
                    if el.local_name == "rPr" {
                        el.children.retain(|c| c.local_name != "latin");
                        el.append_child(
                            OpenXmlElement::new("a", a, "latin")
                                .with_attribute("typeface", typeface),
                        );
                        *found = true;
                    }
                    for c in el.children.iter_mut() {
                        set_tf(c, typeface, a, found);
                    }
                }
                set_tf(el, typeface, a, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, typeface, a, found);
            }
        }
        visit(&mut root, shape_id, typeface, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear latin typeface from shape runs.
    pub fn clear_shape_font_name(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn clear_tf(el: &mut OpenXmlElement, found: &mut bool) {
                    if el.local_name == "rPr" {
                        let before = el.children.len();
                        el.children.retain(|c| c.local_name != "latin");
                        if el.children.len() < before {
                            *found = true;
                        }
                    }
                    for c in el.children.iter_mut() {
                        clear_tf(c, found);
                    }
                }
                clear_tf(el, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether strike is set on shape text (`a:rPr/@strike`).
    /// Clear font name on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_font_name(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_font_name(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Set bodyPr anchor on a shape text body (`a:bodyPr/@anchor`: t/ctr/b).
    pub fn set_shape_text_anchor(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        anchor: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, anchor: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.set_attribute("anchor", anchor);
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, anchor, found);
            }
        }
        visit(&mut root, shape_id, anchor, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read bodyPr anchor for a shape.
    pub fn shape_text_anchor(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("anchor").map(|s| s.to_string()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether bodyPr anchor is set.
    pub fn has_shape_text_anchor(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_anchor(slide_index, shape_id)?.is_some())
    }

    /// Clear bodyPr anchor from a shape.
    pub fn clear_shape_text_anchor(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        let before = bp.attributes.len();
                        bp.attributes.retain(|a| a.local_name != "anchor");
                        *found = bp.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Set bodyPr wrap on a shape (`a:bodyPr/@wrap`: none/square).
    pub fn set_shape_text_wrap(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        wrap: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, wrap: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.set_attribute("wrap", wrap);
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, wrap, found);
            }
        }
        visit(&mut root, shape_id, wrap, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read bodyPr wrap for a shape.
    pub fn shape_text_wrap(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("wrap").map(|s| s.to_string()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether bodyPr wrap is set.
    pub fn has_shape_text_wrap(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_wrap(slide_index, shape_id)?.is_some())
    }

    /// Clear bodyPr wrap from a shape.
    pub fn clear_shape_text_wrap(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        let before = bp.attributes.len();
                        bp.attributes.retain(|a| a.local_name != "wrap");
                        *found = bp.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Set bodyPr upright on a shape (`a:bodyPr/@upright`).
    pub fn set_shape_text_upright(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        upright: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, upright: bool, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if upright {
                            bp.set_attribute("upright", "1");
                        } else {
                            bp.attributes.retain(|a| a.local_name != "upright");
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, upright, found);
            }
        }
        visit(&mut root, shape_id, upright, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr upright is set.
    pub fn has_shape_text_upright(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("upright").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear bodyPr upright from a shape.
    pub fn clear_shape_text_upright(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_text_upright(slide_index, shape_id, false)
    }

    /// Set bodyPr vert on a shape (`a:bodyPr/@vert`: horz/vert/vert270/wordArtVert/…).
    pub fn set_shape_text_vert(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        vert: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, vert: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.set_attribute("vert", vert);
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, vert, found);
            }
        }
        visit(&mut root, shape_id, vert, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read bodyPr vert for a shape.
    pub fn shape_text_vert(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("vert").map(|s| s.to_string()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether bodyPr vert is set.
    pub fn has_shape_text_vert(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_vert(slide_index, shape_id)?.is_some())
    }

    /// Clear bodyPr vert from a shape.
    pub fn clear_shape_text_vert(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        let before = bp.attributes.len();
                        bp.attributes.retain(|a| a.local_name != "vert");
                        *found = bp.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Set bodyPr insets on a shape (EMU). Pass `None` for any side to leave unchanged.
    pub fn set_shape_text_insets(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        l_ins: Option<i64>,
        t_ins: Option<i64>,
        r_ins: Option<i64>,
        b_ins: Option<i64>,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            l_ins: Option<i64>,
            t_ins: Option<i64>,
            r_ins: Option<i64>,
            b_ins: Option<i64>,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if let Some(v) = l_ins {
                            bp.set_attribute("lIns", v.to_string());
                        }
                        if let Some(v) = t_ins {
                            bp.set_attribute("tIns", v.to_string());
                        }
                        if let Some(v) = r_ins {
                            bp.set_attribute("rIns", v.to_string());
                        }
                        if let Some(v) = b_ins {
                            bp.set_attribute("bIns", v.to_string());
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, l_ins, t_ins, r_ins, b_ins, found);
            }
        }
        visit(&mut root, shape_id, l_ins, t_ins, r_ins, b_ins, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read bodyPr insets as `(l, t, r, b)` EMUs when any is present.
    pub fn shape_text_insets(
        &self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<Option<(Option<i64>, Option<i64>, Option<i64>, Option<i64>)>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        let parse = |n: &str| bp.get_attribute(n).and_then(|s| s.parse().ok());
                        let l = parse("lIns");
                        let t = parse("tIns");
                        let r = parse("rIns");
                        let b = parse("bIns");
                        if l.is_some() || t.is_some() || r.is_some() || b.is_some() {
                            return Ok(Some((l, t, r, b)));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether any bodyPr inset is set.
    pub fn has_shape_text_insets(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_insets(slide_index, shape_id)?.is_some())
    }

    /// Clear all bodyPr insets from a shape.
    pub fn clear_shape_text_insets(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        let before = bp.attributes.len();
                        bp.attributes.retain(|a| {
                            !matches!(a.local_name.as_str(), "lIns" | "tIns" | "rIns" | "bIns")
                        });
                        *found = bp.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Set bodyPr column count (`a:bodyPr/@numCol`).
    pub fn set_shape_text_num_col(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        num_col: u32,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, num_col: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.set_attribute("numCol", num_col.to_string());
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, num_col, found);
            }
        }
        visit(&mut root, shape_id, num_col, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read bodyPr numCol.
    pub fn shape_text_num_col(&self, slide_index: usize, shape_id: u32) -> Result<Option<u32>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("numCol").and_then(|s| s.parse().ok()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether bodyPr numCol is set.
    pub fn has_shape_text_num_col(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_num_col(slide_index, shape_id)?.is_some())
    }

    /// Clear bodyPr numCol from a shape.
    pub fn clear_shape_text_num_col(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        let before = bp.attributes.len();
                        bp.attributes.retain(|a| a.local_name != "numCol");
                        *found = bp.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Set bodyPr column spacing (`a:bodyPr/@spcCol` EMUs).
    pub fn set_shape_text_spc_col(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        spc_col: i64,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, spc_col: i64, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.set_attribute("spcCol", spc_col.to_string());
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, spc_col, found);
            }
        }
        visit(&mut root, shape_id, spc_col, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read bodyPr spcCol.
    pub fn shape_text_spc_col(&self, slide_index: usize, shape_id: u32) -> Result<Option<i64>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("spcCol").and_then(|s| s.parse().ok()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether bodyPr spcCol is set.
    pub fn has_shape_text_spc_col(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_spc_col(slide_index, shape_id)?.is_some())
    }

    /// Clear bodyPr spcCol from a shape.
    pub fn clear_shape_text_spc_col(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        let before = bp.attributes.len();
                        bp.attributes.retain(|a| a.local_name != "spcCol");
                        *found = bp.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Set bodyPr fromWordArt flag.
    pub fn set_shape_text_from_word_art(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if enabled {
                            bp.set_attribute("fromWordArt", "1");
                        } else {
                            bp.attributes.retain(|a| a.local_name != "fromWordArt");
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, found);
            }
        }
        visit(&mut root, shape_id, enabled, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr fromWordArt is set.
    pub fn has_shape_text_from_word_art(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("fromWordArt").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear bodyPr fromWordArt from a shape.
    pub fn clear_shape_text_from_word_art(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        self.set_shape_text_from_word_art(slide_index, shape_id, false)
    }

    /// Set bodyPr anchorCtr (center horizontally when anchoring).
    pub fn set_shape_text_anchor_ctr(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if enabled {
                            bp.set_attribute("anchorCtr", "1");
                        } else {
                            bp.attributes.retain(|a| a.local_name != "anchorCtr");
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, found);
            }
        }
        visit(&mut root, shape_id, enabled, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr anchorCtr is set.
    pub fn has_shape_text_anchor_ctr(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("anchorCtr").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear bodyPr anchorCtr from a shape.
    pub fn clear_shape_text_anchor_ctr(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        self.set_shape_text_anchor_ctr(slide_index, shape_id, false)
    }

    /// Set bodyPr rtlCol (right-to-left columns).
    pub fn set_shape_text_rtl_col(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if enabled {
                            bp.set_attribute("rtlCol", "1");
                        } else {
                            bp.attributes.retain(|a| a.local_name != "rtlCol");
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, found);
            }
        }
        visit(&mut root, shape_id, enabled, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr rtlCol is set.
    pub fn has_shape_text_rtl_col(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("rtlCol").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear bodyPr rtlCol from a shape.
    pub fn clear_shape_text_rtl_col(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_text_rtl_col(slide_index, shape_id, false)
    }

    /// Set bodyPr forceAA (force anti-alias).
    pub fn set_shape_text_force_aa(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if enabled {
                            bp.set_attribute("forceAA", "1");
                        } else {
                            bp.attributes.retain(|a| a.local_name != "forceAA");
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, found);
            }
        }
        visit(&mut root, shape_id, enabled, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr forceAA is set.
    pub fn has_shape_text_force_aa(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("forceAA").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear bodyPr forceAA from a shape.
    pub fn clear_shape_text_force_aa(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_text_force_aa(slide_index, shape_id, false)
    }

    /// Set bodyPr compatLnSpc (compatible line spacing).
    pub fn set_shape_text_compat_ln_spc(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if enabled {
                            bp.set_attribute("compatLnSpc", "1");
                        } else {
                            bp.attributes.retain(|a| a.local_name != "compatLnSpc");
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, found);
            }
        }
        visit(&mut root, shape_id, enabled, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr compatLnSpc is set.
    pub fn has_shape_text_compat_ln_spc(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute("compatLnSpc").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear bodyPr compatLnSpc from a shape.
    pub fn clear_shape_text_compat_ln_spc(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        self.set_shape_text_compat_ln_spc(slide_index, shape_id, false)
    }

    /// Set bodyPr spcFirst (space before first paragraph, EMUs or percent-ish depending on font).
    pub fn set_shape_text_spc_first(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        spc: i64,
    ) -> Result<bool> {
        self.set_shape_body_pr_i64(slide_index, shape_id, "spcFirst", Some(spc))
    }

    /// Read bodyPr spcFirst.
    pub fn shape_text_spc_first(&self, slide_index: usize, shape_id: u32) -> Result<Option<i64>> {
        self.shape_body_pr_i64(slide_index, shape_id, "spcFirst")
    }

    /// Whether bodyPr spcFirst is set.
    pub fn has_shape_text_spc_first(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_spc_first(slide_index, shape_id)?.is_some())
    }

    /// Clear bodyPr spcFirst.
    pub fn clear_shape_text_spc_first(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        self.set_shape_body_pr_i64(slide_index, shape_id, "spcFirst", None)
    }

    /// Set bodyPr spcLast.
    pub fn set_shape_text_spc_last(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        spc: i64,
    ) -> Result<bool> {
        self.set_shape_body_pr_i64(slide_index, shape_id, "spcLast", Some(spc))
    }

    /// Read bodyPr spcLast.
    pub fn shape_text_spc_last(&self, slide_index: usize, shape_id: u32) -> Result<Option<i64>> {
        self.shape_body_pr_i64(slide_index, shape_id, "spcLast")
    }

    /// Whether bodyPr spcLast is set.
    pub fn has_shape_text_spc_last(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_spc_last(slide_index, shape_id)?.is_some())
    }

    /// Clear bodyPr spcLast.
    pub fn clear_shape_text_spc_last(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_body_pr_i64(slide_index, shape_id, "spcLast", None)
    }

    fn set_shape_body_pr_i64(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        attr: &str,
        value: Option<i64>,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            attr: &str,
            value: Option<i64>,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        match value {
                            Some(v) => bp.set_attribute(attr, v.to_string()),
                            None => {
                                bp.attributes.retain(|a| a.local_name != attr);
                            }
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, attr, value, found);
            }
        }
        visit(&mut root, shape_id, attr, value, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    fn shape_body_pr_i64(
        &self,
        slide_index: usize,
        shape_id: u32,
        attr: &str,
    ) -> Result<Option<i64>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.get_attribute(attr).and_then(|s| s.parse().ok()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Enable or clear bodyPr normAutofit child.
    pub fn set_shape_text_norm_autofit(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, a: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.children.retain(|c| {
                            !matches!(
                                c.local_name.as_str(),
                                "normAutofit" | "noAutofit" | "spAutoFit"
                            )
                        });
                        if enabled {
                            bp.append_child(OpenXmlElement::new("a", a, "normAutofit"));
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, a, found);
            }
        }
        visit(&mut root, shape_id, enabled, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr has normAutofit.
    pub fn has_shape_text_norm_autofit(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.child("normAutofit").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear autofit children from bodyPr.
    pub fn clear_shape_text_norm_autofit(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        self.set_shape_text_norm_autofit(slide_index, shape_id, false)
    }

    /// Enable or clear bodyPr spAutoFit child (shape autofit).
    pub fn set_shape_text_sp_autofit(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, a: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.children.retain(|c| {
                            !matches!(
                                c.local_name.as_str(),
                                "normAutofit" | "noAutofit" | "spAutoFit"
                            )
                        });
                        if enabled {
                            bp.append_child(OpenXmlElement::new("a", a, "spAutoFit"));
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, a, found);
            }
        }
        visit(&mut root, shape_id, enabled, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr has spAutoFit.
    pub fn has_shape_text_sp_autofit(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.child("spAutoFit").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear spAutoFit from bodyPr.
    pub fn clear_shape_text_sp_autofit(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        self.set_shape_text_sp_autofit(slide_index, shape_id, false)
    }

    /// Enable or clear bodyPr noAutofit child.
    pub fn set_shape_text_no_autofit(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        enabled: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, enabled: bool, a: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        bp.children.retain(|c| {
                            !matches!(
                                c.local_name.as_str(),
                                "normAutofit" | "noAutofit" | "spAutoFit"
                            )
                        });
                        if enabled {
                            bp.append_child(OpenXmlElement::new("a", a, "noAutofit"));
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, enabled, a, found);
            }
        }
        visit(&mut root, shape_id, enabled, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether bodyPr has noAutofit.
    pub fn has_shape_text_no_autofit(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        return Ok(bp.child("noAutofit").is_some());
                    }
                }
            }
        }
        Ok(false)
    }

    /// Clear noAutofit from bodyPr.
    pub fn clear_shape_text_no_autofit(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        self.set_shape_text_no_autofit(slide_index, shape_id, false)
    }

    /// Set normAutofit fontScale (percent * 1000, e.g. 90000 = 90%). Enables normAutofit.
    pub fn set_shape_text_font_scale(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        font_scale: Option<u32>,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            font_scale: Option<u32>,
            a: &str,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if font_scale.is_none() {
                            if let Some(na) = bp.child_mut("normAutofit") {
                                na.attributes.retain(|x| x.local_name != "fontScale");
                            }
                        } else {
                            bp.children.retain(|c| {
                                !matches!(c.local_name.as_str(), "noAutofit" | "spAutoFit")
                            });
                            if bp.child("normAutofit").is_none() {
                                bp.append_child(OpenXmlElement::new("a", a, "normAutofit"));
                            }
                            if let Some(na) = bp.child_mut("normAutofit") {
                                na.set_attribute("fontScale", font_scale.unwrap().to_string());
                            }
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, font_scale, a, found);
            }
        }
        visit(&mut root, shape_id, font_scale, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read normAutofit fontScale when present.
    pub fn shape_text_font_scale(&self, slide_index: usize, shape_id: u32) -> Result<Option<u32>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        if let Some(na) = bp.child("normAutofit") {
                            return Ok(na.get_attribute("fontScale").and_then(|s| s.parse().ok()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether normAutofit has fontScale.
    pub fn has_shape_text_font_scale(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_text_font_scale(slide_index, shape_id)?.is_some())
    }

    /// Clear fontScale from normAutofit.
    pub fn clear_shape_text_font_scale(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        let had = self.has_shape_text_font_scale(slide_index, shape_id)?;
        if had {
            self.set_shape_text_font_scale(slide_index, shape_id, None)?;
        }
        Ok(had)
    }

    /// Set normAutofit lnSpcReduction (percent * 1000). Enables normAutofit.
    pub fn set_shape_text_ln_spc_reduction(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        reduction: Option<u32>,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            reduction: Option<u32>,
            a: &str,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(tx) = el.child_mut("txBody") {
                    if let Some(bp) = tx.child_mut("bodyPr") {
                        if reduction.is_none() {
                            if let Some(na) = bp.child_mut("normAutofit") {
                                na.attributes.retain(|x| x.local_name != "lnSpcReduction");
                            }
                        } else {
                            bp.children.retain(|c| {
                                !matches!(c.local_name.as_str(), "noAutofit" | "spAutoFit")
                            });
                            if bp.child("normAutofit").is_none() {
                                bp.append_child(OpenXmlElement::new("a", a, "normAutofit"));
                            }
                            if let Some(na) = bp.child_mut("normAutofit") {
                                na.set_attribute("lnSpcReduction", reduction.unwrap().to_string());
                            }
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, reduction, a, found);
            }
        }
        visit(&mut root, shape_id, reduction, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read normAutofit lnSpcReduction when present.
    pub fn shape_text_ln_spc_reduction(
        &self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<Option<u32>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(tx) = e.child("txBody") {
                    if let Some(bp) = tx.child("bodyPr") {
                        if let Some(na) = bp.child("normAutofit") {
                            return Ok(na
                                .get_attribute("lnSpcReduction")
                                .and_then(|s| s.parse().ok()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether normAutofit has lnSpcReduction.
    pub fn has_shape_text_ln_spc_reduction(
        &self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        Ok(self
            .shape_text_ln_spc_reduction(slide_index, shape_id)?
            .is_some())
    }

    /// Clear lnSpcReduction from normAutofit.
    pub fn clear_shape_text_ln_spc_reduction(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<bool> {
        let had = self.has_shape_text_ln_spc_reduction(slide_index, shape_id)?;
        if had {
            self.set_shape_text_ln_spc_reduction(slide_index, shape_id, None)?;
        }
        Ok(had)
    }

    pub fn shape_strike(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                for d in e.descendants() {
                    if d.local_name == "rPr" {
                        if let Some(s) = d.get_attribute("strike") {
                            return Ok(Some(s.to_string()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether strike is explicitly set.
    pub fn has_shape_strike(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_strike(slide_index, shape_id)?.is_some())
    }

    /// Set strike on all `a:rPr` (e.g. `"sngStrike"`, `"dblStrike"`). Pass `None` to clear.
    pub fn set_shape_strike(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        strike: Option<&str>,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, strike: Option<&str>, found: &mut bool) {
            if el.local_name == "sp" && contains_id(el, shape_id) {
                fn set_s(el: &mut OpenXmlElement, strike: Option<&str>, found: &mut bool) {
                    if el.local_name == "rPr" {
                        if let Some(s) = strike {
                            el.set_attribute("strike", s);
                        } else {
                            el.attributes.retain(|a| a.local_name != "strike");
                        }
                        *found = true;
                    }
                    for c in el.children.iter_mut() {
                        set_s(c, strike, found);
                    }
                }
                set_s(el, strike, found);
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, strike, found);
            }
        }
        visit(&mut root, shape_id, strike, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear strike on shape text runs.
    pub fn clear_shape_strike(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_strike(slide_index, shape_id, None)
    }

    /// Clear strike on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_strike(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_strike(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn set_shape_hidden(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        hidden: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, hidden: bool, found: &mut bool) {
            if el.local_name == "cNvPr" {
                if el.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id) {
                    if hidden {
                        el.set_attribute("hidden", "1");
                    } else {
                        el.remove_attribute("hidden");
                    }
                    *found = true;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, hidden, found);
            }
        }
        visit(&mut root, shape_id, hidden, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Whether a shape has `cNvPr/@hidden` set.
    pub fn is_shape_hidden(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        for e in root.descendants() {
            if e.local_name == "cNvPr"
                && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            {
                return Ok(e
                    .get_attribute("hidden")
                    .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                    .unwrap_or(false));
            }
        }
        Ok(false)
    }

    /// Replace text under the shape with `cNvPr/@id == shape_id`.
    ///
    /// Sets the first `a:t` node; creates none if the shape has no text body.
    /// Returns whether a text node was updated.
    /// Remove the `hidden` attribute from a shape's cNvPr (defaults to visible).
    pub fn clear_shape_hidden(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if el.local_name == "cNvPr" {
                if el.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id) {
                    if el.get_attribute("hidden").is_some() {
                        el.attributes.retain(|a| a.local_name != "hidden");
                        *found = true;
                    }
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            self.package.set_part(
                info.uri,
                content_type::PRESENTATION_SLIDE,
                write_element(&root)?,
            );
        }
        Ok(found)
    }

    pub fn set_shape_text(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        content: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        fn set_text(el: &mut OpenXmlElement, shape_id: u32, content: &str, found: &mut bool) {
            if *found {
                return;
            }
            let is_shape_like = matches!(
                el.local_name.as_str(),
                "sp" | "pic" | "cxnSp" | "grpSp" | "graphicFrame"
            );
            if is_shape_like && contains_id(el, shape_id) {
                fn set_first_t(el: &mut OpenXmlElement, content: &str, done: &mut bool) {
                    if *done {
                        return;
                    }
                    if el.local_name == "t" {
                        el.set_text(content);
                        *done = true;
                        return;
                    }
                    for c in el.children.iter_mut() {
                        set_first_t(c, content, done);
                    }
                }
                let mut done = false;
                set_first_t(el, content, &mut done);
                *found = done;
                return;
            }
            for c in el.children.iter_mut() {
                set_text(c, shape_id, content, found);
            }
        }
        let mut found = false;
        set_text(&mut root, shape_id, content, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read solid fill RGB (6-hex) from the shape with `cNvPr/@id == shape_id`.
    pub fn shape_fill_rgb(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(sp_pr) = e.child("spPr") {
                    if let Some(sf) = sp_pr.child("solidFill") {
                        if let Some(srgb) = sf.child("srgbClr") {
                            return Ok(srgb.get_attribute("val").map(|s| s.to_string()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether the shape has a solid fill color.
    pub fn has_shape_fill(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_fill_rgb(slide_index, shape_id)?.is_some())
    }

    /// Set solid fill RGB (6-hex, no `#`) on the shape with `cNvPr/@id == shape_id`.
    pub fn set_shape_fill(&mut self, slide_index: usize, shape_id: u32, rgb: &str) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, rgb: &str, a: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if el.child("spPr").is_none() {
                    let p = crate::namespace::ns::PRESENTATIONML.uri;
                    el.append_child(OpenXmlElement::new("p", p, "spPr"));
                }
                if let Some(sp_pr) = el.child_mut("spPr") {
                    // drop existing fill-like children
                    sp_pr.children.retain(|c| {
                        !matches!(
                            c.local_name.as_str(),
                            "solidFill"
                                | "noFill"
                                | "gradFill"
                                | "blipFill"
                                | "pattFill"
                                | "grpFill"
                        )
                    });
                    sp_pr.append_child(OpenXmlElement::new("a", a, "solidFill").with_child(
                        OpenXmlElement::new("a", a, "srgbClr").with_attribute("val", rgb),
                    ));
                    *found = true;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, rgb, a, found);
            }
        }
        visit(&mut root, shape_id, rgb, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear solid/gradient/pattern fill on a shape (sets `a:noFill`).
    pub fn clear_shape_fill(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, a: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    let before = sp_pr.children.len();
                    sp_pr.children.retain(|c| {
                        !matches!(
                            c.local_name.as_str(),
                            "solidFill"
                                | "noFill"
                                | "gradFill"
                                | "blipFill"
                                | "pattFill"
                                | "grpFill"
                        )
                    });
                    sp_pr.append_child(OpenXmlElement::new("a", a, "noFill"));
                    *found = true;
                    let _ = before;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, a, found);
            }
        }
        visit(&mut root, shape_id, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read shape flip flags as `(flipH, flipV)` from `spPr/xfrm`.
    /// Clear fill on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_fill(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_fill(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Set solid fill alpha on a shape (0–100000 thousandths of a percent).
    ///
    /// Requires an existing solid fill; returns false if the shape has no solidFill.
    pub fn set_shape_fill_alpha(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        alpha: u32,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, alpha: u32, a: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    if let Some(sf) = sp_pr.child_mut("solidFill") {
                        if let Some(srgb) = sf.child_mut("srgbClr") {
                            srgb.children.retain(|c| c.local_name != "alpha");
                            srgb.append_child(
                                OpenXmlElement::new("a", a, "alpha")
                                    .with_attribute("val", alpha.to_string()),
                            );
                            *found = true;
                            return;
                        }
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, alpha, a, found);
            }
        }
        visit(&mut root, shape_id, alpha, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read solid fill alpha if present.
    pub fn shape_fill_alpha(&self, slide_index: usize, shape_id: u32) -> Result<Option<u32>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(sp_pr) = e.child("spPr") {
                    if let Some(sf) = sp_pr.child("solidFill") {
                        if let Some(srgb) = sf.child("srgbClr") {
                            if let Some(al) = srgb.child("alpha") {
                                return Ok(al.get_attribute("val").and_then(|s| s.parse().ok()));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether solid fill alpha is set.
    pub fn has_shape_fill_alpha(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_fill_alpha(slide_index, shape_id)?.is_some())
    }

    /// Clear solid fill alpha from a shape.
    pub fn clear_shape_fill_alpha(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    if let Some(sf) = sp_pr.child_mut("solidFill") {
                        if let Some(srgb) = sf.child_mut("srgbClr") {
                            let before = srgb.children.len();
                            srgb.children.retain(|c| c.local_name != "alpha");
                            *found = srgb.children.len() < before;
                            return;
                        }
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    pub fn shape_flip(&self, slide_index: usize, shape_id: u32) -> Result<Option<(bool, bool)>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(sp_pr) = e.child("spPr") {
                    if let Some(xfrm) = sp_pr.child("xfrm") {
                        let fh = xfrm
                            .get_attribute("flipH")
                            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                            .unwrap_or(false);
                        let fv = xfrm
                            .get_attribute("flipV")
                            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                            .unwrap_or(false);
                        return Ok(Some((fh, fv)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether xfrm is present (flip readable).
    pub fn has_shape_flip(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_flip(slide_index, shape_id)?.is_some())
    }

    /// Set shape flip flags on `spPr/xfrm` (`flipH` / `flipV`).
    pub fn set_shape_flip(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        flip_h: bool,
        flip_v: bool,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            flip_h: bool,
            flip_v: bool,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    if let Some(xfrm) = sp_pr.child_mut("xfrm") {
                        if flip_h {
                            xfrm.set_attribute("flipH", "1");
                        } else {
                            xfrm.attributes.retain(|a| a.local_name != "flipH");
                        }
                        if flip_v {
                            xfrm.set_attribute("flipV", "1");
                        } else {
                            xfrm.attributes.retain(|a| a.local_name != "flipV");
                        }
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, flip_h, flip_v, found);
            }
        }
        visit(&mut root, shape_id, flip_h, flip_v, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear both flip flags on a shape.
    pub fn clear_shape_flip(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        self.set_shape_flip(slide_index, shape_id, false, false)
    }

    /// Read shape rotation in 1/60000 degrees from `spPr/xfrm/@rot` (0 if absent).
    /// Clear flip flags on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_flip(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_flip(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shape_rotation(&self, slide_index: usize, shape_id: u32) -> Result<Option<i32>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(sp_pr) = e.child("spPr") {
                    if let Some(xfrm) = sp_pr.child("xfrm") {
                        return Ok(Some(
                            xfrm.get_attribute("rot")
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(0),
                        ));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether xfrm is present (rotation readable).
    pub fn has_shape_rotation(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_rotation(slide_index, shape_id)?.is_some())
    }

    /// Set shape rotation in 1/60000 degrees on `spPr/xfrm/@rot`.
    pub fn set_shape_rotation(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        rot: i32,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, rot: i32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    if let Some(xfrm) = sp_pr.child_mut("xfrm") {
                        xfrm.set_attribute("rot", rot.to_string());
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, rot, found);
            }
        }
        visit(&mut root, shape_id, rot, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear `xfrm/@rot` (rotation becomes default 0).
    pub fn clear_shape_rotation(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    if let Some(xfrm) = sp_pr.child_mut("xfrm") {
                        let before = xfrm.attributes.len();
                        xfrm.attributes.retain(|a| a.local_name != "rot");
                        *found = xfrm.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read preset geometry name (`spPr/prstGeom/@prst`), e.g. `"rect"`, `"ellipse"`.
    /// Clear rotation on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_rotation(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_rotation(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn shape_preset_geom(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(sp_pr) = e.child("spPr") {
                    if let Some(pg) = sp_pr.child("prstGeom") {
                        return Ok(pg.get_attribute("prst").map(|s| s.to_string()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether the shape has a preset geometry.
    pub fn has_shape_preset_geom(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_preset_geom(slide_index, shape_id)?.is_some())
    }

    /// Set preset geometry (`spPr/prstGeom/@prst`). Creates prstGeom if missing.
    pub fn set_shape_preset_geom(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        preset: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, preset: &str, a: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    if let Some(pg) = sp_pr.child_mut("prstGeom") {
                        pg.set_attribute("prst", preset);
                    } else {
                        sp_pr.append_child(
                            OpenXmlElement::new("a", a, "prstGeom")
                                .with_attribute("prst", preset)
                                .with_child(OpenXmlElement::new("a", a, "avLst")),
                        );
                    }
                    *found = true;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, preset, a, found);
            }
        }
        visit(&mut root, shape_id, preset, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Remove `prstGeom` from a shape. Returns whether present.
    pub fn clear_shape_preset_geom(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    let before = sp_pr.children.len();
                    sp_pr.children.retain(|c| c.local_name != "prstGeom");
                    *found = sp_pr.children.len() < before;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read shape transform as `(x, y, cx, cy)` EMUs from `spPr/xfrm`.
    pub fn shape_transform(
        &self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<Option<(i64, i64, i64, i64)>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(sp_pr) = e.child("spPr") {
                    if let Some(xfrm) = sp_pr.child("xfrm") {
                        let off = xfrm.child("off");
                        let ext = xfrm.child("ext");
                        let x = off
                            .and_then(|o| o.get_attribute("x"))
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                        let y = off
                            .and_then(|o| o.get_attribute("y"))
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                        let cx = ext
                            .and_then(|o| o.get_attribute("cx"))
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                        let cy = ext
                            .and_then(|o| o.get_attribute("cy"))
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                        return Ok(Some((x, y, cx, cy)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether the shape has an `xfrm` transform.
    pub fn has_shape_transform(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_transform(slide_index, shape_id)?.is_some())
    }

    /// Set shape transform `(x, y, cx, cy)` EMUs on `spPr/xfrm`.
    pub fn set_shape_transform(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            x: i64,
            y: i64,
            cx: i64,
            cy: i64,
            a: &str,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if el.child("spPr").is_none() {
                    let p = crate::namespace::ns::PRESENTATIONML.uri;
                    el.append_child(OpenXmlElement::new("p", p, "spPr"));
                }
                if let Some(sp_pr) = el.child_mut("spPr") {
                    sp_pr.children.retain(|c| c.local_name != "xfrm");
                    // insert xfrm first among children for validity
                    sp_pr.children.insert(
                        0,
                        OpenXmlElement::new("a", a, "xfrm")
                            .with_child(
                                OpenXmlElement::new("a", a, "off")
                                    .with_attribute("x", x.to_string())
                                    .with_attribute("y", y.to_string()),
                            )
                            .with_child(
                                OpenXmlElement::new("a", a, "ext")
                                    .with_attribute("cx", cx.to_string())
                                    .with_attribute("cy", cy.to_string()),
                            ),
                    );
                    *found = true;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, x, y, cx, cy, a, found);
            }
        }
        visit(&mut root, shape_id, x, y, cx, cy, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Read outline/line solid color RGB from a shape (`spPr/ln/solidFill/srgbClr`).
    pub fn shape_line_rgb(&self, slide_index: usize, shape_id: u32) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        for e in root.descendants() {
            if e.local_name == "sp" && contains_id(e, shape_id) {
                if let Some(sp_pr) = e.child("spPr") {
                    if let Some(ln) = sp_pr.child("ln") {
                        if let Some(sf) = ln.child("solidFill") {
                            if let Some(srgb) = sf.child("srgbClr") {
                                return Ok(srgb.get_attribute("val").map(|s| s.to_string()));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Whether the shape has a solid line color.
    pub fn has_shape_line(&self, slide_index: usize, shape_id: u32) -> Result<bool> {
        Ok(self.shape_line_rgb(slide_index, shape_id)?.is_some())
    }

    /// Set outline line solid RGB and optional width EMUs on a shape.
    pub fn set_shape_line(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        rgb: &str,
        width_emu: Option<i64>,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let a = crate::namespace::ns::DRAWINGML.uri;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            shape_id: u32,
            rgb: &str,
            width_emu: Option<i64>,
            a: &str,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if el.child("spPr").is_none() {
                    let p = crate::namespace::ns::PRESENTATIONML.uri;
                    el.append_child(OpenXmlElement::new("p", p, "spPr"));
                }
                if let Some(sp_pr) = el.child_mut("spPr") {
                    sp_pr.children.retain(|c| c.local_name != "ln");
                    let mut ln = OpenXmlElement::new("a", a, "ln");
                    if let Some(w) = width_emu {
                        ln.set_attribute("w", w.to_string());
                    }
                    ln.append_child(OpenXmlElement::new("a", a, "solidFill").with_child(
                        OpenXmlElement::new("a", a, "srgbClr").with_attribute("val", rgb),
                    ));
                    sp_pr.append_child(ln);
                    *found = true;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, rgb, width_emu, a, found);
            }
        }
        visit(&mut root, shape_id, rgb, width_emu, a, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear outline/line on a shape (`spPr/ln`).
    pub fn clear_shape_line(&mut self, slide_index: usize, shape_id: u32) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        fn contains_id(el: &OpenXmlElement, shape_id: u32) -> bool {
            el.descendants().any(|e| {
                e.local_name == "cNvPr"
                    && e.get_attribute("id").and_then(|s| s.parse().ok()) == Some(shape_id)
            })
        }
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, shape_id: u32, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "sp" && contains_id(el, shape_id) {
                if let Some(sp_pr) = el.child_mut("spPr") {
                    let before = sp_pr.children.len();
                    sp_pr.children.retain(|c| c.local_name != "ln");
                    *found = sp_pr.children.len() < before;
                    return;
                }
            }
            for c in el.children.iter_mut() {
                visit(c, shape_id, found);
            }
        }
        visit(&mut root, shape_id, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Remove all shapes whose `cNvPr/@name` matches `name`. Returns count removed.
    /// Clear outline/line on every shape of a slide. Returns shapes modified.
    pub fn clear_all_shape_line(&mut self, slide_index: usize) -> Result<usize> {
        let ids = self.list_shape_ids(slide_index)?;
        let mut n = 0usize;
        for (id, _) in ids {
            if self.clear_shape_line(slide_index, id)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn remove_shapes_by_name(&mut self, slide_index: usize, name: &str) -> Result<usize> {
        let ids: Vec<u32> = self
            .list_shape_ids(slide_index)?
            .into_iter()
            .filter(|(_, n)| n == name)
            .map(|(id, _)| id)
            .collect();
        let mut count = 0usize;
        for id in ids {
            if self.remove_shape_by_id(slide_index, id)? {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Remove every shape-like element from a slide (`sp`/`pic`/`cxnSp`/`grpSp`/`graphicFrame`).
    ///
    /// Returns the number of top-level shape-like children removed from `spTree`.
    pub fn clear_shapes(&mut self, slide_index: usize) -> Result<usize> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&info.uri)
                .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?,
        )?;
        let mut count = 0usize;
        if let Some(csld) = root.child_mut("cSld") {
            if let Some(tree) = csld.child_mut("spTree") {
                let before = tree.children.len();
                tree.children.retain(|c| {
                    !matches!(
                        c.local_name.as_str(),
                        "sp" | "pic" | "cxnSp" | "grpSp" | "graphicFrame" | "contentPart"
                    )
                });
                count = before - tree.children.len();
            }
        }
        if count > 0 {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(count)
    }

    /// Whether this package is a macro-enabled presentation type.
    pub fn is_macro_enabled(&self) -> bool {
        matches!(
            self.document_type,
            PresentationDocumentType::MacroEnabledPresentation
                | PresentationDocumentType::MacroEnabledTemplate
                | PresentationDocumentType::MacroEnabledSlideshow
        )
    }

    /// Number of parts in the underlying OPC package.
    pub fn part_count(&self) -> usize {
        self.package.opc().part_uris().len()
    }

    /// Whether any slide text contains `needle`.
    pub fn contains_text(&self, needle: &str) -> Result<bool> {
        if needle.is_empty() {
            return Ok(true);
        }
        for texts in self.all_slide_texts()? {
            if texts.iter().any(|t| t.contains(needle)) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Read notes text for a slide (if a notes slide relationship exists).
    pub fn notes_text(&self, slide_index: usize) -> Result<Option<String>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let Some(rels) = self.package.opc().part_relationships(&slide_info.uri) else {
            return Ok(None);
        };
        let Some(rel) = rels.get_by_type(rel::NOTES_SLIDE) else {
            return Ok(None);
        };
        let target = if rel.target.starts_with('/') {
            PackUri::new(rel.target.clone())
        } else {
            // Resolve relative to slide directory
            let base = slide_info.uri.as_str();
            let parent = base.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
            let joined = format!("{}/{}", parent, rel.target.trim_start_matches("./"));
            // Normalize .. segments simply
            let notes_uri = if joined.contains("..") {
                PackUri::new(format!(
                    "/ppt/notesSlides/{}",
                    rel.target.rsplit('/').next().unwrap_or(&rel.target)
                ))
            } else {
                PackUri::new(joined)
            };
            notes_uri
        };
        let data = match self.package.opc().get_part(&target) {
            Some(d) => d,
            None => {
                // Try common absolute path
                let alt = PackUri::new(format!(
                    "/ppt/notesSlides/{}",
                    rel.target.rsplit('/').next().unwrap_or(&rel.target)
                ));
                match self.package.opc().get_part(&alt) {
                    Some(d) => d,
                    None => return Ok(None),
                }
            }
        };
        let root = parse_element(data)?;
        let texts = slide_texts(&root);
        if texts.is_empty() {
            Ok(Some(String::new()))
        } else {
            Ok(Some(texts.join("\n")))
        }
    }

    /// Whether the slide has an associated notes slide relationship.
    /// Whether a slide has non-empty notes text.
    pub fn has_notes_text(&self, slide_index: usize) -> Result<bool> {
        Ok(self
            .notes_text(slide_index)?
            .map(|t| !t.trim().is_empty())
            .unwrap_or(false))
    }

    /// Clear notes text on a slide (removes notes or empties). Returns whether notes text was present.
    pub fn clear_notes_text(&mut self, slide_index: usize) -> Result<bool> {
        let had = self.has_notes_text(slide_index)?;
        if had {
            // Prefer clearing notes part entirely
            let _ = self.clear_notes(slide_index)?;
        }
        Ok(had)
    }

    /// Clear notes text on every notes slide. Returns slides modified.
    pub fn clear_all_notes_text(&mut self) -> Result<usize> {
        let n = self.slides.len();
        let mut count = 0usize;
        for i in 0..n {
            if self.clear_notes_text(i)? {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn has_notes(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        Ok(self
            .package
            .opc()
            .part_relationships(&slide_info.uri)
            .and_then(|rels| rels.get_by_type(rel::NOTES_SLIDE))
            .is_some())
    }

    fn notes_uri_for_slide(&self, slide_index: usize) -> Result<Option<PackUri>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let Some(rels) = self.package.opc().part_relationships(&slide_info.uri) else {
            return Ok(None);
        };
        let Some(rel) = rels.get_by_type(rel::NOTES_SLIDE) else {
            return Ok(None);
        };
        let target = if rel.target.starts_with('/') {
            PackUri::new(rel.target.clone())
        } else if rel.target.contains("..") {
            PackUri::new(format!(
                "/ppt/notesSlides/{}",
                rel.target.rsplit('/').next().unwrap_or(&rel.target)
            ))
        } else {
            let base = slide_info.uri.as_str();
            let parent = base.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
            PackUri::new(format!(
                "{}/{}",
                parent,
                rel.target.trim_start_matches("./")
            ))
        };
        if self.package.opc().has_part(&target) {
            Ok(Some(target))
        } else {
            let alt = PackUri::new(format!(
                "/ppt/notesSlides/{}",
                rel.target.rsplit('/').next().unwrap_or(&rel.target)
            ));
            if self.package.opc().has_part(&alt) {
                Ok(Some(alt))
            } else {
                Ok(None)
            }
        }
    }

    /// Set the notes slide common data name (`p:cSld/@name` on notes).
    pub fn set_notes_name(&mut self, slide_index: usize, name: &str) -> Result<()> {
        let Some(notes_uri) = self.notes_uri_for_slide(slide_index)? else {
            return Err(Error::Package(format!(
                "no notes slide for index {slide_index}"
            )));
        };
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&notes_uri)
                .ok_or_else(|| Error::PartNotFound(notes_uri.to_string()))?,
        )?;
        if let Some(csld) = root.child_mut("cSld") {
            csld.set_attribute("name", name);
        } else {
            return Err(Error::Package("notes cSld missing".into()));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(notes_uri, content_type::PRESENTATION_NOTES_SLIDE, xml);
        Ok(())
    }

    /// Read notes slide common data name.
    pub fn notes_name(&self, slide_index: usize) -> Result<Option<String>> {
        let Some(notes_uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(None);
        };
        let data = match self.package.opc().get_part(&notes_uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let root = parse_element(data)?;
        Ok(root
            .child("cSld")
            .and_then(|c| c.get_attribute("name").map(|s| s.to_string())))
    }

    /// Configure header/footer visibility on a notes slide (`p:hf`).
    /// Whether notes cSld name is set.
    pub fn has_notes_name(&self, slide_index: usize) -> Result<bool> {
        Ok(self.notes_name(slide_index)?.is_some())
    }

    /// Clear notes cSld `@name`.
    pub fn clear_notes_name(&mut self, slide_index: usize) -> Result<bool> {
        let Some(notes_uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&notes_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(csld) = root.child_mut("cSld") else {
            return Ok(false);
        };
        if csld.get_attribute("name").is_none() {
            return Ok(false);
        }
        csld.attributes.retain(|a| a.local_name != "name");
        self.package.set_part(
            notes_uri,
            content_type::PRESENTATION_NOTES_SLIDE,
            write_element(&root)?,
        );
        Ok(true)
    }

    pub fn set_notes_header_footer(
        &mut self,
        slide_index: usize,
        show_date: bool,
        show_header: bool,
        show_footer: bool,
        show_slide_number: bool,
    ) -> Result<()> {
        let Some(notes_uri) = self.notes_uri_for_slide(slide_index)? else {
            return Err(Error::Package(format!(
                "no notes slide for index {slide_index}"
            )));
        };
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&notes_uri)
                .ok_or_else(|| Error::PartNotFound(notes_uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "hf");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.append_child(
            OpenXmlElement::new("p", p, "hf")
                .with_attribute("sldNum", if show_slide_number { "1" } else { "0" })
                .with_attribute("hdr", if show_header { "1" } else { "0" })
                .with_attribute("ftr", if show_footer { "1" } else { "0" })
                .with_attribute("dt", if show_date { "1" } else { "0" }),
        );
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(notes_uri, content_type::PRESENTATION_NOTES_SLIDE, xml);
        Ok(())
    }

    /// Whether notes slide has header/footer flags.
    pub fn has_notes_header_footer(&self, slide_index: usize) -> Result<bool> {
        let Some(notes_uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(false);
        };
        let data = match self.package.opc().get_part(&notes_uri) {
            Some(d) => d,
            None => return Ok(false),
        };
        let root = parse_element(data)?;
        Ok(root.child("hf").is_some())
    }

    /// Clear notes header/footer. Returns whether present.
    pub fn clear_notes_header_footer(&mut self, slide_index: usize) -> Result<bool> {
        let Some(notes_uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(false);
        };
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&notes_uri)
                .ok_or_else(|| Error::PartNotFound(notes_uri.to_string()))?,
        )?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "hf");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(notes_uri, content_type::PRESENTATION_NOTES_SLIDE, xml);
        }
        Ok(removed)
    }

    /// Remove the notes slide related from a slide. Returns whether notes existed.
    pub fn clear_notes(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let rel_info = {
            let Some(rels) = self.package.opc().part_relationships(&slide_info.uri) else {
                return Ok(false);
            };
            rels.get_by_type(rel::NOTES_SLIDE)
                .map(|r| (r.id.clone(), r.target.clone()))
        };
        let Some((rid, target)) = rel_info else {
            return Ok(false);
        };
        let notes_uri = if target.starts_with('/') {
            PackUri::new(target)
        } else if target.contains("..") {
            PackUri::new(format!(
                "/ppt/notesSlides/{}",
                target.rsplit('/').next().unwrap_or(&target)
            ))
        } else {
            let base = slide_info.uri.as_str();
            let parent = base.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
            PackUri::new(format!("{}/{}", parent, target.trim_start_matches("./")))
        };
        self.package.opc_mut().remove_part(&notes_uri);
        self.package
            .opc_mut()
            .part_relationships_mut(&slide_info.uri)
            .remove(&rid);
        Ok(true)
    }

    /// Whether a theme part is present.
    pub fn has_theme(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("/ppt/theme/"))
    }

    /// Count theme parts under `/ppt/theme/`.
    pub fn theme_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/theme/"))
            .count()
    }

    /// List theme part URIs.
    pub fn list_themes(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/theme/"))
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
            .set_part(uri, &ct, write_element(&root)?);
        Ok(true)
    }

    /// Remove theme parts and presentation theme relationships.
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

    /// Clear theme `@name` attribute (does not remove the theme part).
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
            .set_part(uri, ct, write_element(&root)?);
        Ok(true)
    }

    pub fn clear_theme(&mut self) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/theme/"))
            .collect();
        if uris.is_empty() {
            return Ok(false);
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::THEME)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Ensure a default theme exists. Returns `(uri, relationship_id)`.
    pub fn ensure_theme(&mut self) -> Result<(PackUri, String)> {
        if self.has_theme() {
            let pres_uri = PackUri::new(PRESENTATION_URI);
            if let Some(rel) = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .and_then(|r| r.get_by_type(rel::THEME))
            {
                let uri = if rel.target.starts_with('/') {
                    PackUri::new(rel.target.clone())
                } else {
                    PackUri::new(format!("/ppt/{}", rel.target.trim_start_matches("../")))
                };
                // Prefer known path
                let theme_uri = if self
                    .package
                    .opc()
                    .has_part(&PackUri::new("/ppt/theme/theme1.xml"))
                {
                    PackUri::new("/ppt/theme/theme1.xml")
                } else {
                    uri
                };
                return Ok((theme_uri, rel.id.clone()));
            }
        }
        self.add_default_theme()
    }

    /// Count media parts under `/ppt/media/`.
    pub fn media_count(&self) -> usize {
        self.list_media().len()
    }

    /// List media part URIs under `/ppt/media/`.
    pub fn list_media(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/media/"))
            .collect()
    }

    /// Remove a single media/image part by URI and drop relationships that target it.
    pub fn remove_media(&mut self, uri: &PackUri) -> Result<bool> {
        if !(uri.as_str().starts_with("/ppt/media/") || uri.as_str().contains("/media/")) {
            return Ok(false);
        }
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        let target = uri.as_str().to_string();
        let part_uris: Vec<PackUri> = self.package.opc().part_uris();
        for src in part_uris {
            let Some(rels) = self.package.opc().part_relationships(&src) else {
                continue;
            };
            let ids: Vec<String> = rels
                .iter()
                .filter(|r| {
                    let t = r.target.as_str();
                    crate::opc::resolve_uri(&src, t)
                        .map(|u| u.as_str() == target)
                        .unwrap_or(false)
                        || t == target
                        || t.ends_with(target.trim_start_matches('/'))
                })
                .map(|r| r.id.clone())
                .collect();
            if ids.is_empty() {
                continue;
            }
            let rels_mut = self.package.opc_mut().part_relationships_mut(&src);
            for id in ids {
                rels_mut.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Remove all media parts under `/ppt/media/` and image/media relationships from slides.
    pub fn clear_media(&mut self) -> Result<usize> {
        let media = self.list_media();
        let n = media.len();
        if n == 0 {
            return Ok(0);
        }
        let slide_uris: Vec<PackUri> = self.slides.iter().map(|s| s.uri.clone()).collect();
        for slide_uri in slide_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&slide_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type.contains("image")
                                || r.relationship_type.contains("media")
                                || r.relationship_type.contains("audio")
                                || r.relationship_type.contains("video")
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(&slide_uri);
                for id in ids {
                    rels.remove(&id);
                }
            }
        }
        for uri in media {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Count chart parts under `/ppt/charts/`.
    pub fn chart_count(&self) -> usize {
        self.list_charts().len()
    }

    /// Whether any drawing parts exist under `/ppt/drawings/`.
    pub fn has_drawings(&self) -> bool {
        self.drawing_count() > 0
    }

    /// Count drawing parts under `/ppt/drawings/`.
    pub fn drawing_count(&self) -> usize {
        self.list_drawings().len()
    }

    /// List drawing part URIs under `/ppt/drawings/`.
    pub fn list_drawings(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/drawings/"))
            .collect()
    }

    /// Remove drawing parts under `/ppt/drawings/` and related relationships from slides/charts.
    pub fn clear_drawings(&mut self) -> Result<usize> {
        let uris = self.list_drawings();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        let parents: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str();
                s.starts_with("/ppt/slides/") || s.starts_with("/ppt/charts/")
            })
            .collect();
        for parent in parents {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&parent)
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
            if ids.is_empty() {
                continue;
            }
            let rels = self.package.opc_mut().part_relationships_mut(&parent);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Whether any chart parts exist under `/ppt/charts/`.
    pub fn has_charts(&self) -> bool {
        self.chart_count() > 0
    }

    /// List chart part URIs under `/ppt/charts/`.
    pub fn list_charts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/charts/"))
            .collect()
    }

    /// Number of slides that currently have comments.
    pub fn slide_comments_count(&self) -> Result<usize> {
        Ok(self.slides_with_comments()?.len())
    }

    /// Number of slides that currently have a solid/custom background.
    pub fn background_count(&self) -> Result<usize> {
        Ok(self.slides_with_background()?.len())
    }

    /// Number of slides that currently have header/footer flags.
    pub fn header_footer_count(&self) -> Result<usize> {
        Ok(self.slides_with_header_footer()?.len())
    }

    /// Remove all chart parts under `/ppt/charts/` and chart relationships from slides.
    pub fn clear_charts(&mut self) -> Result<usize> {
        let charts = self.list_charts();
        let n = charts.len();
        if n == 0 {
            return Ok(0);
        }
        let slide_uris: Vec<PackUri> = self.slides.iter().map(|s| s.uri.clone()).collect();
        for slide_uri in slide_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&slide_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type.contains("chart"))
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(&slide_uri);
                for id in ids {
                    rels.remove(&id);
                }
            }
        }
        for uri in charts {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Number of slide masters.
    pub fn master_count(&self) -> usize {
        self.masters.len()
    }

    /// Whether any slide masters exist.
    pub fn has_slide_masters(&self) -> bool {
        self.master_count() > 0
    }

    /// Alias for [`list_masters`](Self::list_masters).
    pub fn list_slide_masters(&self) -> Vec<PackUri> {
        self.list_masters()
    }

    /// Count slide master parts (URI inventory).
    pub fn slide_master_part_count(&self) -> usize {
        self.list_slide_masters().len()
    }

    /// Alias for [`slide_master_part_count`](Self::slide_master_part_count).
    pub fn slide_master_count(&self) -> usize {
        self.slide_master_part_count()
    }

    /// Whether any slide layouts exist.
    pub fn has_slide_layouts(&self) -> bool {
        self.layout_count() > 0
    }

    /// Alias for [`list_layouts`](Self::list_layouts).
    pub fn list_slide_layouts(&self) -> Vec<PackUri> {
        self.list_layouts()
    }

    /// Number of slide layouts.
    pub fn layout_count(&self) -> usize {
        self.layouts.len()
    }

    /// Whether any notes masters exist.
    pub fn has_notes_master(&self) -> bool {
        self.notes_master_count() > 0
    }

    /// Count notes master parts.
    pub fn notes_master_count(&self) -> usize {
        self.list_notes_masters().len()
    }

    /// List notes master part URIs.
    pub fn list_notes_masters(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/notesMasters/"))
            .collect()
    }

    /// Whether a handout master exists.
    pub fn has_handout_master(&self) -> bool {
        self.handout_master_count() > 0
    }

    /// Count handout master parts.
    pub fn handout_master_count(&self) -> usize {
        self.list_handout_masters().len()
    }

    /// List handout master part URIs.
    pub fn list_handout_masters(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/handoutMasters/"))
            .collect()
    }

    /// Remove notes master parts and presentation relationships.
    pub fn clear_notes_master(&mut self) -> Result<bool> {
        self.clear_master_parts("/ppt/notesMasters/", rel::NOTES_MASTER)
    }

    /// Remove handout master parts and presentation relationships.
    pub fn clear_handout_master(&mut self) -> Result<bool> {
        self.clear_master_parts("/ppt/handoutMasters/", rel::HANDOUT_MASTER)
    }

    fn clear_master_parts(&mut self, path_prefix: &str, rel_type: &str) -> Result<bool> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains(path_prefix))
            .collect();
        if uris.is_empty() {
            return Ok(false);
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel_type)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
            // Drop notesMasterIdLst / handoutMasterIdLst entries if present
            if let Some(data) = self.package.opc().get_part(&pres_uri).map(|d| d.to_vec()) {
                if let Ok(mut root) = parse_element(&data) {
                    let tag = if path_prefix.contains("notes") {
                        "notesMasterIdLst"
                    } else {
                        "handoutMasterIdLst"
                    };
                    let before = root.children.len();
                    root.children.retain(|c| c.local_name != tag);
                    if root.children.len() < before {
                        let xml = write_element(&root)?;
                        self.package.set_part(
                            pres_uri.clone(),
                            self.document_type.content_type(),
                            xml,
                        );
                    }
                }
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(true)
    }

    /// Whether any user-defined tag parts exist.
    pub fn has_user_defined_tags(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("/ppt/tags/"))
    }

    /// Count user-defined tag parts under `/ppt/tags/`.
    pub fn user_defined_tag_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/tags/"))
            .count()
    }

    /// Whether any slide sync data parts exist.
    pub fn has_slide_sync_data(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("slideUpdateInfo") || u.as_str().contains("slideSync"))
    }

    /// Count slide sync data parts.
    pub fn slide_sync_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("slideUpdateInfo") || u.as_str().contains("slideSync"))
            .count()
    }

    /// Whether a comment authors part exists.
    pub fn has_comment_authors(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/ppt/commentAuthors.xml"))
    }

    /// Count comment author entries.
    pub fn comment_author_count(&self) -> Result<usize> {
        Ok(self.list_comment_authors()?.len())
    }

    /// List comment authors as `(id, name, initials)`.
    pub fn list_comment_authors(&self) -> Result<Vec<(u32, String, String)>> {
        let uri = PackUri::new("/ppt/commentAuthors.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "cmAuthor")
            .map(|a| {
                let id = a
                    .get_attribute("id")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let name = a.get_attribute("name").unwrap_or("").to_string();
                let initials = a.get_attribute("initials").unwrap_or("").to_string();
                (id, name, initials)
            })
            .collect())
    }

    /// Look up a comment author by id as `(name, initials)`.
    pub fn comment_author_by_id(&self, id: u32) -> Result<Option<(String, String)>> {
        Ok(self
            .list_comment_authors()?
            .into_iter()
            .find(|(i, _, _)| *i == id)
            .map(|(_, name, initials)| (name, initials)))
    }

    /// Append a single comment author entry, creating the part if needed.
    ///
    /// Returns the author id that was written.
    pub fn append_comment_author(&mut self, id: u32, name: &str, initials: &str) -> Result<u32> {
        let uri = PackUri::new("/ppt/commentAuthors.xml");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let mut authors = self.list_comment_authors()?;
        authors.retain(|(i, _, _)| *i != id);
        authors.push((id, name.to_string(), initials.to_string()));
        let pairs: Vec<(u32, &str, &str)> = authors
            .iter()
            .map(|(i, n, init)| (*i, n.as_str(), init.as_str()))
            .collect();
        if self.package.opc().has_part(&uri) {
            let mut root = OpenXmlElement::new("p", p, "cmAuthorLst").with_ns_decl("p", p);
            for (i, n, init) in &pairs {
                root.append_child(
                    OpenXmlElement::new("p", p, "cmAuthor")
                        .with_attribute("id", i.to_string())
                        .with_attribute("name", *n)
                        .with_attribute("initials", *init)
                        .with_attribute("lastIdx", "0")
                        .with_attribute("clrIdx", "0"),
                );
            }
            self.package.set_part(
                uri,
                content_type::PRESENTATION_COMMENT_AUTHORS,
                write_element(&root)?,
            );
        } else {
            self.add_comment_authors(&pairs)?;
        }
        Ok(id)
    }

    /// Remove a comment author by id. Returns whether it was present.
    pub fn remove_comment_author(&mut self, id: u32) -> Result<bool> {
        let uri = PackUri::new("/ppt/commentAuthors.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| {
            !(c.local_name == "cmAuthor"
                && c.get_attribute("id").and_then(|s| s.parse::<u32>().ok()) == Some(id))
        });
        let removed = root.children.len() < before;
        if removed {
            if root.children.is_empty() {
                let _ = self.clear_comment_authors()?;
            } else {
                self.package.set_part(
                    uri,
                    content_type::PRESENTATION_COMMENT_AUTHORS,
                    write_element(&root)?,
                );
            }
        }
        Ok(removed)
    }

    /// List user-defined tag parts as URIs.
    pub fn list_user_defined_tags(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/tags/"))
            .collect()
    }

    /// Collect all user-defined tags as `(name, value)` across tag parts.
    pub fn list_user_defined_tag_entries(&self) -> Result<Vec<(String, String)>> {
        let mut out = Vec::new();
        for uri in self.list_user_defined_tags() {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let root = parse_element(data)?;
            for t in root.descendants().filter(|e| e.local_name == "tag") {
                let name = t.get_attribute("name").unwrap_or("").to_string();
                let val = t.get_attribute("val").unwrap_or("").to_string();
                out.push((name, val));
            }
        }
        Ok(out)
    }

    /// List slide sync part URIs.
    /// Whether any user-defined tag entries exist.
    pub fn has_user_defined_tag_entries(&self) -> Result<bool> {
        Ok(!self.list_user_defined_tag_entries()?.is_empty())
    }

    /// Count user-defined tag entries.
    pub fn user_defined_tag_entry_count(&self) -> Result<usize> {
        Ok(self.list_user_defined_tag_entries()?.len())
    }

    pub fn list_slide_sync_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("slideUpdateInfo") || u.as_str().contains("slideSync"))
            .collect()
    }

    /// Whether any slide sync / update-info parts exist.
    pub fn has_slide_sync_parts(&self) -> bool {
        !self.list_slide_sync_parts().is_empty()
    }

    /// Count slide sync / update-info parts.
    pub fn slide_sync_part_count(&self) -> usize {
        self.list_slide_sync_parts().len()
    }

    /// Whether any modern comment parts exist.
    pub fn has_modern_comments(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("modernComment")
                || s.contains("/ppt/comments/modern")
                || s.contains("p188")
                || (s.contains("/ppt/comments/") && s.contains("modern"))
        })
    }

    /// Count modern comment parts.
    pub fn modern_comment_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str();
                s.contains("modernComment")
                    || s.contains("/ppt/comments/modern")
                    || (s.contains("/ppt/comments/") && s.contains("modern"))
            })
            .count()
    }

    /// Remove modern comment parts and related relationships.
    pub fn clear_modern_comments(&mut self) -> Result<usize> {
        // Path contains "modernComment" (e.g. /ppt/comments/modernComment1.xml)
        self.clear_slide_related_parts("modernComment", rel::PPT_MODERN_COMMENTS)
    }

    /// Whether modern authors part exists.
    pub fn has_modern_authors(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/ppt/authors.xml"))
    }

    /// Remove modern authors part.
    pub fn clear_modern_authors(&mut self) -> Result<bool> {
        self.clear_pres_related_part("/ppt/authors.xml", rel::PPT_AUTHORS)
    }

    /// Whether any chart drawing parts exist for slides.
    pub fn has_chart_drawings(&self) -> bool {
        if self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.contains("chartshapes") || s.contains("chartDrawing")
        }) {
            return true;
        }
        // Detect via CHART_DRAWING relationships on slides or charts
        let parents: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str();
                s.starts_with("/ppt/slides/") || s.starts_with("/ppt/charts/")
            })
            .collect();
        parents.iter().any(|p| {
            self.package
                .opc()
                .part_relationships(p)
                .map(|rels| {
                    rels.iter()
                        .any(|r| r.relationship_type == rel::CHART_DRAWING)
                })
                .unwrap_or(false)
        })
    }

    /// Remove chart drawing parts related to slides.
    pub fn clear_chart_drawings(&mut self) -> Result<usize> {
        let mut uris: Vec<PackUri> = Vec::new();
        let parents: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str();
                s.starts_with("/ppt/slides/")
                    || s.starts_with("/ppt/charts/")
                    || s.starts_with("/ppt/drawings/")
            })
            .collect();
        for parent in &parents {
            if let Some(rels) = self.package.opc().part_relationships(parent) {
                for r in rels.iter() {
                    if r.relationship_type == rel::CHART_DRAWING {
                        if let Ok(uri) = self.package.opc().resolve_relationship(Some(parent), r) {
                            if !uris.iter().any(|x| x == &uri) {
                                uris.push(uri);
                            }
                        }
                    }
                }
            }
        }
        for u in self.package.opc().part_uris() {
            let s = u.as_str();
            if s.contains("chartshapes") || s.contains("chartDrawing") {
                if !uris.iter().any(|x| x == &u) {
                    uris.push(u.clone());
                }
            }
        }
        // Also drawings that are targets under /ppt/drawings/ with chartDrawing content
        // (add_chart_drawing_for_slide creates /ppt/drawings/drawingN.xml)
        // If they were collected via rels, fine; if orphaned, leave them.
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        for parent in parents {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&parent)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type == rel::CHART_DRAWING)
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(&parent);
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

    /// Whether any 3D model media parts exist.
    pub fn has_model_3d(&self) -> bool {
        self.package.opc().part_uris().into_iter().any(|u| {
            let s = u.as_str();
            s.ends_with(".glb") || s.ends_with(".gltf") || s.contains("model3d")
        })
    }

    /// Remove comment authors part.
    pub fn clear_comment_authors(&mut self) -> Result<bool> {
        self.clear_pres_related_part("/ppt/commentAuthors.xml", rel::COMMENT_AUTHORS)
    }

    /// Remove 3D model media parts and slide relationships.
    pub fn clear_model_3d(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str();
                s.ends_with(".glb") || s.ends_with(".gltf") || s.contains("model3d")
            })
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        let slide_uris: Vec<PackUri> = self.slides.iter().map(|s| s.uri.clone()).collect();
        for slide_uri in slide_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&slide_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type == rel::MODEL_3D)
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(&slide_uri);
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

    /// Remove all user-defined tag parts and their slide relationships.
    pub fn clear_user_defined_tags(&mut self) -> Result<usize> {
        self.clear_slide_related_parts("/ppt/tags/", rel::USER_DEFINED_TAGS)
    }

    /// Remove all slide sync data parts and their slide relationships.
    pub fn clear_slide_sync_data(&mut self) -> Result<usize> {
        self.clear_slide_related_parts("slideUpdateInfo", rel::SLIDE_SYNC)
    }

    fn clear_slide_related_parts(&mut self, path_hint: &str, rel_type: &str) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains(path_hint))
            .collect();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        let slide_uris: Vec<PackUri> = self.slides.iter().map(|s| s.uri.clone()).collect();
        for slide_uri in slide_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&slide_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel_type)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(&slide_uri);
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

    /// Whether any audio/video media relationships exist on any slide.
    pub fn has_media(&self) -> bool {
        self.media_count() > 0
    }

    /// List all part URIs in the package.
    pub fn list_part_uris(&self) -> Vec<PackUri> {
        self.package.opc().part_uris()
    }

    /// Count package-level relationships.
    pub fn package_relationship_count(&self) -> usize {
        self.package.opc().package_relationships().len()
    }

    /// Add a blank notes master and link it from the presentation.
    ///
    /// Returns the notes master URI.
    pub fn add_notes_master(&mut self) -> Result<PackUri> {
        let pres_uri = self.ensure_presentation()?;
        let mut index = 1u32;
        let master_uri = loop {
            let candidate = PackUri::new(format!("/ppt/notesMasters/notesMaster{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let xml = write_element(&notes_master())?;
        self.package.set_part(
            master_uri.clone(),
            content_type::PRESENTATION_NOTES_MASTER,
            xml,
        );
        if self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::NOTES_MASTER))
            .is_none()
        {
            self.package.add_part_relationship(
                &pres_uri,
                rel::NOTES_MASTER,
                &master_uri,
                RelationshipTargetMode::Internal,
            );
        }
        // Also add notesMasterIdLst entry if missing
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        if root.child("notesMasterIdLst").is_none() {
            // Find the relationship id we just created
            let rid = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .and_then(|rels| rels.get_by_type(rel::NOTES_MASTER).map(|r| r.id.clone()))
                .unwrap_or_else(|| "rIdNotesMaster".into());
            let entry = OpenXmlElement::new(
                "p",
                crate::namespace::ns::PRESENTATIONML.uri,
                "notesMasterId",
            )
            .with_attribute_qname("r:id", &rid);
            let list = OpenXmlElement::new(
                "p",
                crate::namespace::ns::PRESENTATIONML.uri,
                "notesMasterIdLst",
            )
            .with_child(entry);
            // Insert after sldMasterIdLst if present
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sldMasterIdLst")
                .map(|i| i + 1)
                .unwrap_or(0);
            root.children.insert(insert_at, list);
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(master_uri)
    }

    /// Set header/footer flags on the notes master part (`p:hf`).
    pub fn set_notes_master_header_footer(
        &mut self,
        show_date: bool,
        show_header: bool,
        show_footer: bool,
        show_slide_number: bool,
    ) -> Result<()> {
        if !self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("/notesMasters/"))
        {
            self.add_notes_master()?;
        }
        // find first notes master uri
        let master_uri = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .find(|u| u.as_str().contains("/notesMasters/"))
            .ok_or_else(|| Error::Package("notes master missing".into()))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&master_uri)
                .ok_or_else(|| Error::PartNotFound(master_uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "hf");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.append_child(
            OpenXmlElement::new("p", p, "hf")
                .with_attribute("sldNum", if show_slide_number { "1" } else { "0" })
                .with_attribute("hdr", if show_header { "1" } else { "0" })
                .with_attribute("ftr", if show_footer { "1" } else { "0" })
                .with_attribute("dt", if show_date { "1" } else { "0" }),
        );
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(master_uri, content_type::PRESENTATION_NOTES_MASTER, xml);
        Ok(())
    }

    /// Whether notes master has header/footer flags.
    pub fn has_notes_master_header_footer(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/notesMasters/"))
            .any(|u| {
                self.package
                    .opc()
                    .get_part(&u)
                    .and_then(|d| parse_element(d).ok())
                    .map(|r| r.child("hf").is_some())
                    .unwrap_or(false)
            })
    }

    /// Clear `p:hf` from notes masters. Returns how many masters were updated.
    pub fn clear_notes_master_header_footer(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/notesMasters/"))
            .collect();
        let mut n = 0;
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let mut root = parse_element(data)?;
            let before = root.children.len();
            root.children.retain(|c| c.local_name != "hf");
            if root.children.len() < before {
                let xml = write_element(&root)?;
                self.package
                    .opc_mut()
                    .set_part(uri, content_type::PRESENTATION_NOTES_MASTER, xml);
                n += 1;
            }
        }
        Ok(n)
    }

    /// Set header/footer flags on the handout master part (`p:hf`).
    pub fn set_handout_master_header_footer(
        &mut self,
        show_date: bool,
        show_header: bool,
        show_footer: bool,
        show_slide_number: bool,
    ) -> Result<()> {
        if !self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().contains("/handoutMasters/"))
        {
            self.add_handout_master()?;
        }
        let master_uri = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .find(|u| u.as_str().contains("/handoutMasters/"))
            .ok_or_else(|| Error::Package("handout master missing".into()))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&master_uri)
                .ok_or_else(|| Error::PartNotFound(master_uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "hf");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.append_child(
            OpenXmlElement::new("p", p, "hf")
                .with_attribute("sldNum", if show_slide_number { "1" } else { "0" })
                .with_attribute("hdr", if show_header { "1" } else { "0" })
                .with_attribute("ftr", if show_footer { "1" } else { "0" })
                .with_attribute("dt", if show_date { "1" } else { "0" }),
        );
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(master_uri, content_type::PRESENTATION_HANDOUT_MASTER, xml);
        Ok(())
    }

    /// Whether handout master has header/footer flags.
    pub fn has_handout_master_header_footer(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/handoutMasters/"))
            .any(|u| {
                self.package
                    .opc()
                    .get_part(&u)
                    .and_then(|d| parse_element(d).ok())
                    .map(|r| r.child("hf").is_some())
                    .unwrap_or(false)
            })
    }

    /// Clear `p:hf` from handout masters. Returns how many masters were updated.
    pub fn clear_handout_master_header_footer(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/handoutMasters/"))
            .collect();
        let mut n = 0;
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let mut root = parse_element(data)?;
            let before = root.children.len();
            root.children.retain(|c| c.local_name != "hf");
            if root.children.len() < before {
                let xml = write_element(&root)?;
                self.package.set_part(
                    uri,
                    content_type::PRESENTATION_HANDOUT_MASTER,
                    xml,
                );
                n += 1;
            }
        }
        Ok(n)
    }

    /// Set header/footer flags on the first slide master (`p:hf`).
    pub fn set_slide_master_header_footer(
        &mut self,
        show_date: bool,
        show_header: bool,
        show_footer: bool,
        show_slide_number: bool,
    ) -> Result<()> {
        let master_uri = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .find(|u| u.as_str().contains("/slideMasters/"))
            .ok_or_else(|| Error::Package("slide master missing".into()))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&master_uri)
                .ok_or_else(|| Error::PartNotFound(master_uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "hf");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.append_child(
            OpenXmlElement::new("p", p, "hf")
                .with_attribute("sldNum", if show_slide_number { "1" } else { "0" })
                .with_attribute("hdr", if show_header { "1" } else { "0" })
                .with_attribute("ftr", if show_footer { "1" } else { "0" })
                .with_attribute("dt", if show_date { "1" } else { "0" }),
        );
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(master_uri, content_type::PRESENTATION_SLIDE_MASTER, xml);
        Ok(())
    }

    /// Whether any slide master has header/footer flags.
    pub fn has_slide_master_header_footer(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/slideMasters/"))
            .any(|u| {
                self.package
                    .opc()
                    .get_part(&u)
                    .and_then(|d| parse_element(d).ok())
                    .map(|r| r.child("hf").is_some())
                    .unwrap_or(false)
            })
    }

    /// Clear header/footer from all slide masters. Returns how many masters were updated.
    pub fn clear_slide_master_header_footers(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/slideMasters/"))
            .collect();
        let mut n = 0;
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let mut root = parse_element(data)?;
            let before = root.children.len();
            root.children.retain(|c| c.local_name != "hf");
            if root.children.len() < before {
                let xml = write_element(&root)?;
                self.package
                    .opc_mut()
                    .set_part(uri, content_type::PRESENTATION_SLIDE_MASTER, xml);
                n += 1;
            }
        }
        Ok(n)
    }

    /// Set header/footer flags on the first slide layout (`p:hf`).
    pub fn set_slide_layout_header_footer(
        &mut self,
        show_date: bool,
        show_header: bool,
        show_footer: bool,
        show_slide_number: bool,
    ) -> Result<()> {
        let layout_uri = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .find(|u| u.as_str().contains("/slideLayouts/"))
            .ok_or_else(|| Error::Package("slide layout missing".into()))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&layout_uri)
                .ok_or_else(|| Error::PartNotFound(layout_uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "hf");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.append_child(
            OpenXmlElement::new("p", p, "hf")
                .with_attribute("sldNum", if show_slide_number { "1" } else { "0" })
                .with_attribute("hdr", if show_header { "1" } else { "0" })
                .with_attribute("ftr", if show_footer { "1" } else { "0" })
                .with_attribute("dt", if show_date { "1" } else { "0" }),
        );
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(layout_uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
        Ok(())
    }

    /// Whether any slide layout has header/footer flags.
    pub fn has_slide_layout_header_footer(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/slideLayouts/"))
            .any(|u| {
                self.package
                    .opc()
                    .get_part(&u)
                    .and_then(|d| parse_element(d).ok())
                    .map(|r| r.child("hf").is_some())
                    .unwrap_or(false)
            })
    }

    /// Clear header/footer from all slide layouts. Returns how many were updated.
    pub fn clear_slide_layout_header_footers(&mut self) -> Result<usize> {
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/slideLayouts/"))
            .collect();
        let mut n = 0;
        for uri in uris {
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let mut root = parse_element(data)?;
            let before = root.children.len();
            root.children.retain(|c| c.local_name != "hf");
            if root.children.len() < before {
                let xml = write_element(&root)?;
                self.package
                    .opc_mut()
                    .set_part(uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
                n += 1;
            }
        }
        Ok(n)
    }

    /// Add a blank handout master and link it from the presentation.
    pub fn add_handout_master(&mut self) -> Result<PackUri> {
        let pres_uri = self.ensure_presentation()?;
        let mut index = 1u32;
        let master_uri = loop {
            let candidate = PackUri::new(format!("/ppt/handoutMasters/handoutMaster{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let xml = write_element(&handout_master())?;
        self.package.set_part(
            master_uri.clone(),
            content_type::PRESENTATION_HANDOUT_MASTER,
            xml,
        );
        if self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::HANDOUT_MASTER))
            .is_none()
        {
            self.package.add_part_relationship(
                &pres_uri,
                rel::HANDOUT_MASTER,
                &master_uri,
                RelationshipTargetMode::Internal,
            );
        }
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        if root.child("handoutMasterIdLst").is_none() {
            let rid = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .and_then(|rels| rels.get_by_type(rel::HANDOUT_MASTER).map(|r| r.id.clone()))
                .unwrap_or_else(|| "rIdHandoutMaster".into());
            let entry = OpenXmlElement::new(
                "p",
                crate::namespace::ns::PRESENTATIONML.uri,
                "handoutMasterId",
            )
            .with_attribute_qname("r:id", &rid);
            let list = OpenXmlElement::new(
                "p",
                crate::namespace::ns::PRESENTATIONML.uri,
                "handoutMasterIdLst",
            )
            .with_child(entry);
            let insert_at = root
                .children
                .iter()
                .position(|c| {
                    matches!(c.local_name.as_str(), "sldMasterIdLst" | "notesMasterIdLst")
                })
                .map(|i| i + 1)
                .unwrap_or(0);
            root.children.insert(insert_at, list);
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(master_uri)
    }

    /// Add comments to a slide.
    ///
    /// Each entry is `(author_id, datetime, x_emu, y_emu, text)`.
    /// Ensure authors exist via [`add_comment_authors`] first.
    pub fn add_slide_comments(
        &mut self,
        slide_index: usize,
        comments: &[(u32, &str, i64, i64, &str)],
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut index = 1u32;
        let uri = loop {
            let candidate = PackUri::new(format!("/ppt/comments/comment{index}.xml"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let cms: Vec<_> = comments
            .iter()
            .enumerate()
            .map(|(i, (aid, dt, x, y, text))| slide_comment(*aid, i as u32 + 1, dt, *x, *y, text))
            .collect();
        let root = slide_comments(cms);
        self.package.set_part(
            uri.clone(),
            content_type::PRESENTATION_COMMENTS,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &slide_info.uri,
            rel::COMMENTS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add user-defined tags part for a slide.
    pub fn add_user_defined_tags(
        &mut self,
        slide_index: usize,
        tags: &[(&str, &str)],
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/tags/tag{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let mut root = OpenXmlElement::new("p", p, "tagLst").with_ns_decl("p", p);
        for (name, val) in tags {
            root.append_child(
                OpenXmlElement::new("p", p, "tag")
                    .with_attribute("name", *name)
                    .with_attribute("val", *val),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::USER_DEFINED_TAGS,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &slide_info.uri,
            rel::USER_DEFINED_TAGS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a 3D model reference part shell.
    pub fn add_model_3d(
        &mut self,
        slide_index: usize,
        glb_data: impl Into<Vec<u8>>,
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/media/model{index}.glb"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type::MODEL_3D, glb_data.into());
        let rid = self.package.add_part_relationship(
            &slide_info.uri,
            rel::MODEL_3D,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add slide sync data part shell.
    pub fn add_slide_sync_data(
        &mut self,
        slide_index: usize,
        server_sld_id: &str,
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/slideUpdateInfo/slideUpdateInfo{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let root = OpenXmlElement::new("p", p, "sldSyncPr")
            .with_ns_decl("p", p)
            .with_attribute("serverSldId", server_sld_id)
            .with_attribute("serverSldModifiedTime", "2020-01-01T00:00:00");
        self.package.set_part(
            uri.clone(),
            content_type::SLIDE_SYNC,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &slide_info.uri,
            rel::SLIDE_SYNC,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a DrawingML chart part related from a slide.

    /// Convenience: add a bar chart on slide 0 (creates a blank slide if none).
    pub fn add_chart(
        &mut self,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        if self.slides.is_empty() {
            self.add_slide_with_text(title)?;
        }
        self.add_chart_on_slide(0, title, categories, values)
    }

    ///
    /// Creates a minimal bar chart under `/ppt/charts/`. Returns `(chart_uri, relationship_id)`.
    pub fn add_chart_on_slide(
        &mut self,
        slide_index: usize,
        title: &str,
        categories: &[&str],
        values: &[f64],
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/charts/chart{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let chart = crate::spreadsheet::bar_chart_space(title, categories, values);
        self.package.set_part(
            uri.clone(),
            content_type::DRAWINGML_CHART,
            write_element(&chart)?,
        );
        let rid = self.package.add_part_relationship(
            &slide_info.uri,
            rel::CHART,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add a chart drawing user-shapes part related from a chart on a slide.
    ///
    /// If `chart_rel_target` is a package-absolute chart URI (e.g. `/ppt/charts/chart1.xml`)
    /// the drawing is related from that chart; otherwise it is related from the slide.
    pub fn add_chart_drawing_for_slide(
        &mut self,
        slide_index: usize,
        chart_rel_target: &str,
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/drawings/drawing{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let cdr = "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing";
        let a = crate::namespace::ns::DRAWINGML.uri;
        let root = OpenXmlElement::new("cdr", cdr, "userShapes")
            .with_ns_decl("cdr", cdr)
            .with_ns_decl("a", a);
        self.package.set_part(
            uri.clone(),
            content_type::CHART_DRAWING,
            write_element(&root)?,
        );
        let parent = if chart_rel_target.starts_with('/') {
            PackUri::new(chart_rel_target)
        } else {
            slide_info.uri.clone()
        };
        let rid = self.package.add_part_relationship(
            &parent,
            rel::CHART_DRAWING,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add an embedded font part related from the presentation.
    ///
    /// When `preferred_stem` is `Some`, the part is stored as
    /// `/ppt/fonts/{stem}.{ext}` (Office ODTTF parts use a braced GUID stem that
    /// matches the XOR key). Otherwise `font{N}.{ext}` is used.
    pub fn add_font_part(
        &mut self,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<(PackUri, String)> {
        self.add_font_part_named(data, content_type_str, extension, None)
    }

    /// Like [`add_font_part`], but allow a preferred part stem (no extension).
    pub fn add_font_part_named(
        &mut self,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
        preferred_stem: Option<&str>,
    ) -> Result<(PackUri, String)> {
        let pres_uri = self
            .package
            .opc()
            .package_relationships()
            .get_by_type(rel::OFFICE_DOCUMENT)
            .map(|r| {
                PackUri::new(if r.target.starts_with('/') {
                    r.target.clone()
                } else {
                    format!("/{}", r.target.trim_start_matches('/'))
                })
            })
            .unwrap_or_else(|| PackUri::new("/ppt/presentation.xml"));
        let ext = extension.trim_start_matches('.');
        let uri = if let Some(stem) = preferred_stem {
            let stem = stem.trim();
            let candidate = PackUri::new(format!("/ppt/fonts/{stem}.{ext}"));
            if self.package.opc().has_part(&candidate) {
                let mut index = 1u32;
                loop {
                    let c = PackUri::new(format!("/ppt/fonts/{stem}_{index}.{ext}"));
                    if !self.package.opc().has_part(&c) {
                        break c;
                    }
                    index += 1;
                }
            } else {
                candidate
            }
        } else {
            let mut index = 1u32;
            loop {
                let c = PackUri::new(format!("/ppt/fonts/font{index}.{ext}"));
                if !self.package.opc().has_part(&c) {
                    break c;
                }
                index += 1;
            }
        };
        self.package
            .opc_mut()
            .set_part(uri.clone(), content_type_str, data.into());
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::FONT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Whether any embedded font parts exist under `/ppt/fonts/`.
    pub fn has_font_parts(&self) -> bool {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .any(|u| u.as_str().starts_with("/ppt/fonts/"))
    }

    /// Count embedded font parts under `/ppt/fonts/`.
    pub fn font_part_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/fonts/"))
            .count()
    }

    /// List embedded font part URIs.
    pub fn list_font_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/fonts/"))
            .collect()
    }

    /// Remove all embedded font parts and related main-part font relationships.
    pub fn clear_font_parts(&mut self) -> Result<usize> {
        let uris = self.list_font_parts();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel::FONT)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
        }
        Ok(n)
    }

    /// Whether the slide has a comments relationship / part.
    pub fn has_slide_comments(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        Ok(self
            .package
            .opc()
            .part_relationships(&slide_info.uri)
            .map(|rels| {
                rels.iter()
                    .any(|r| r.relationship_type.contains("/comments"))
            })
            .unwrap_or(false))
    }

    /// Count classic PPT comment parts under `/ppt/comments/`.
    pub fn slide_comments_part_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/comments/"))
            .count()
    }

    /// Whether any classic slide comment parts exist under `/ppt/comments/`.
    pub fn has_comments(&self) -> bool {
        self.slide_comments_part_count() > 0
    }

    /// Alias for [`slide_comments_part_count`](Self::slide_comments_part_count).
    pub fn comment_count(&self) -> usize {
        self.slide_comments_part_count()
    }

    /// List classic slide comment part URIs.
    pub fn list_comment_parts(&self) -> Vec<PackUri> {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().starts_with("/ppt/comments/"))
            .collect()
    }

    /// Whether any classic comment parts exist under `/ppt/comments/`.
    pub fn has_comment_parts(&self) -> bool {
        !self.list_comment_parts().is_empty()
    }

    /// Count classic comment parts.
    pub fn comment_part_count(&self) -> usize {
        self.list_comment_parts().len()
    }

    /// Remove all classic slide comments parts and relationships.
    pub fn clear_comments(&mut self) -> Result<usize> {
        let uris = self.list_comment_parts();
        let n = uris.len();
        if n == 0 {
            return Ok(0);
        }
        let slide_uris: Vec<PackUri> = self.slides.iter().map(|s| s.uri.clone()).collect();
        for slide_uri in slide_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&slide_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| r.relationship_type.contains("/comments"))
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if !ids.is_empty() {
                let rels = self.package.opc_mut().part_relationships_mut(&slide_uri);
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

    /// Remove classic slide comments for a slide. Returns whether comments existed.
    pub fn clear_slide_comments(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let targets: Vec<(String, PackUri)> = {
            let Some(rels) = self.package.opc().part_relationships(&slide_info.uri) else {
                return Ok(false);
            };
            let mut out = Vec::new();
            for r in rels.iter() {
                if r.relationship_type.contains("/comments") {
                    if let Ok(uri) = self
                        .package
                        .opc()
                        .resolve_relationship(Some(&slide_info.uri), r)
                    {
                        out.push((r.id.clone(), uri));
                    }
                }
            }
            out
        };
        if targets.is_empty() {
            return Ok(false);
        }
        for (id, uri) in targets {
            self.package.opc_mut().remove_part(&uri);
            self.package
                .opc_mut()
                .part_relationships_mut(&slide_info.uri)
                .remove(&id);
        }
        Ok(true)
    }

    /// List comments on a slide as `(author_id, idx, datetime, text)`.
    pub fn list_slide_comments(
        &self,
        slide_index: usize,
    ) -> Result<Vec<(u32, u32, String, String)>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let Some(rels) = self.package.opc().part_relationships(&slide_info.uri) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for r in rels.iter() {
            if !r.relationship_type.contains("/comments") {
                continue;
            }
            let Ok(uri) = self
                .package
                .opc()
                .resolve_relationship(Some(&slide_info.uri), r)
            else {
                continue;
            };
            let Some(data) = self.package.opc().get_part(&uri) else {
                continue;
            };
            let Ok(root) = parse_element(data) else {
                continue;
            };
            for cm in root.descendants().filter(|e| e.local_name == "cm") {
                let author_id = cm
                    .get_attribute("authorId")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let idx = cm
                    .get_attribute("idx")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let dt = cm.get_attribute("dt").unwrap_or("").to_string();
                let text = cm.child("text").map(|t| t.inner_text()).unwrap_or_default();
                out.push((author_id, idx, dt, text));
            }
        }
        Ok(out)
    }

    /// Add modern PowerPoint comments part shell (2018+ format).
    ///
    /// Each entry is `(author_id, text)`.
    pub fn add_modern_comments(
        &mut self,
        slide_index: usize,
        comments: &[(&str, &str)],
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut index = 1u32;
        let uri = loop {
            let c = PackUri::new(format!("/ppt/comments/modernComment{index}.xml"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        let p188 = "http://schemas.microsoft.com/office/powerpoint/2018/8/main";
        let mut root = OpenXmlElement::new("p188", p188, "cmLst").with_ns_decl("p188", p188);
        for (i, (author_id, text)) in comments.iter().enumerate() {
            root.append_child(
                OpenXmlElement::new("p188", p188, "cm")
                    .with_attribute("id", format!("{{{i}}}"))
                    .with_attribute("authorId", *author_id)
                    .with_child(
                        OpenXmlElement::new("p188", p188, "txBody").with_child(
                            OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, "p")
                                .with_child(
                                    OpenXmlElement::new(
                                        "a",
                                        crate::namespace::ns::DRAWINGML.uri,
                                        "r",
                                    )
                                    .with_child(
                                        OpenXmlElement::new(
                                            "a",
                                            crate::namespace::ns::DRAWINGML.uri,
                                            "t",
                                        )
                                        .with_text(*text),
                                    ),
                                ),
                        ),
                    ),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::PPT_MODERN_COMMENTS,
            write_element(&root)?,
        );
        let rid = self.package.add_part_relationship(
            &slide_info.uri,
            rel::PPT_MODERN_COMMENTS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add modern PowerPoint authors part shell.
    pub fn add_modern_authors(&mut self, authors: &[(&str, &str)]) -> Result<(PackUri, String)> {
        let pres_uri = self.ensure_presentation()?;
        let uri = PackUri::new("/ppt/authors.xml");
        let p188 = "http://schemas.microsoft.com/office/powerpoint/2018/8/main";
        let mut root = OpenXmlElement::new("p188", p188, "authorLst").with_ns_decl("p188", p188);
        for (id, name) in authors {
            root.append_child(
                OpenXmlElement::new("p188", p188, "author")
                    .with_attribute("id", *id)
                    .with_attribute("name", *name)
                    .with_attribute("initials", &name.chars().next().unwrap_or('?').to_string())
                    .with_attribute("userId", *id)
                    .with_attribute("providerId", "None"),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::PPT_AUTHORS,
            write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::PPT_AUTHORS).map(|r| r.id.clone()))
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::PPT_AUTHORS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add comment authors part (`ppt/commentAuthors.xml`).
    ///
    /// Each entry is `(id, name, initials)`.
    pub fn add_comment_authors(
        &mut self,
        authors: &[(u32, &str, &str)],
    ) -> Result<(PackUri, String)> {
        let pres_uri = self.ensure_presentation()?;
        let uri = PackUri::new("/ppt/commentAuthors.xml");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let mut root = OpenXmlElement::new("p", p, "cmAuthorLst").with_ns_decl("p", p);
        for (id, name, initials) in authors {
            root.append_child(
                OpenXmlElement::new("p", p, "cmAuthor")
                    .with_attribute("id", id.to_string())
                    .with_attribute("name", *name)
                    .with_attribute("initials", *initials)
                    .with_attribute("lastIdx", "0")
                    .with_attribute("clrIdx", "0"),
            );
        }
        self.package.set_part(
            uri.clone(),
            content_type::PRESENTATION_COMMENT_AUTHORS,
            write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::COMMENT_AUTHORS).map(|r| r.id.clone()))
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::COMMENT_AUTHORS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Add presentation properties part (`ppt/presProps.xml`).
    pub fn add_presentation_properties(&mut self) -> Result<(PackUri, String)> {
        let pres_uri = self.ensure_presentation()?;
        let uri = PackUri::new("/ppt/presProps.xml");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let root = OpenXmlElement::new("p", p, "presentationPr")
            .with_ns_decl("p", p)
            .with_ns_decl("a", crate::namespace::ns::DRAWINGML.uri)
            .with_child(
                OpenXmlElement::new("p", p, "clrMru").with_child(
                    OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, "srgbClr")
                        .with_attribute("val", "000000"),
                ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::PRESENTATION_PROPS,
            write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::PRES_PROPS).map(|r| r.id.clone()))
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::PRES_PROPS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    fn ensure_presentation_properties_root(&mut self) -> Result<(PackUri, OpenXmlElement)> {
        let uri = PackUri::new("/ppt/presProps.xml");
        if let Some(data) = self.package.opc().get_part(&uri) {
            return Ok((uri, parse_element(data)?));
        }
        self.add_presentation_properties()?;
        let data = self
            .package
            .opc()
            .get_part(&uri)
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))?;
        Ok((uri, parse_element(data)?))
    }

    fn save_presentation_properties(&mut self, uri: PackUri, root: &OpenXmlElement) -> Result<()> {
        self.package.set_part(
            uri,
            content_type::PRESENTATION_PROPS,
            write_element(root)?,
        );
        Ok(())
    }

    fn ensure_show_pr_mut<'a>(&self, root: &'a mut OpenXmlElement) -> &'a mut OpenXmlElement {
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if root.child("showPr").is_none() {
            root.append_child(OpenXmlElement::new("p", p, "showPr"));
        }
        root.child_mut("showPr").expect("showPr ensured")
    }

    /// Set slideshow properties (`p:showPr` attrs).
    pub fn set_show_loop(&mut self, loop_: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let show = self.ensure_show_pr_mut(&mut root);
        show.set_attribute("loop", if loop_ { "1" } else { "0" });
        self.save_presentation_properties(uri, &root)
    }

    /// Whether slideshow loops.
    pub fn show_loop(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.get_attribute("loop"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `showPr/@showNarration`.
    /// Disable `show loop`. Returns whether it was enabled.
    pub fn clear_show_loop(&mut self) -> Result<bool> {
        let had = self.show_loop()?;
        if had {
            self.set_show_loop(false)?;
        }
        Ok(had)
    }

    pub fn set_show_narration(&mut self, show: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr.set_attribute("showNarration", if show { "1" } else { "0" });
        self.save_presentation_properties(uri, &root)
    }

    /// Whether narration is shown during slideshow.
    pub fn show_narration(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.get_attribute("showNarration"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    /// Set `showPr/@showAnimation`.
    /// Disable `show narration`. Returns whether it was enabled.
    pub fn clear_show_narration(&mut self) -> Result<bool> {
        let had = self.show_narration()?;
        if had {
            self.set_show_narration(false)?;
        }
        Ok(had)
    }

    pub fn set_show_animation(&mut self, show: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr.set_attribute("showAnimation", if show { "1" } else { "0" });
        self.save_presentation_properties(uri, &root)
    }

    /// Whether animation is shown during slideshow (default true).
    pub fn show_animation(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.get_attribute("showAnimation"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set `showPr/@useTimings`.
    /// Disable `show animation`. Returns whether it was enabled.
    pub fn clear_show_animation(&mut self) -> Result<bool> {
        let had = self.show_animation()?;
        if had {
            self.set_show_animation(false)?;
        }
        Ok(had)
    }

    pub fn set_use_timings(&mut self, use_timings: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr.set_attribute("useTimings", if use_timings { "1" } else { "0" });
        self.save_presentation_properties(uri, &root)
    }

    /// Whether timings are used during slideshow (default true).
    pub fn use_timings(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.get_attribute("useTimings"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Whether useTimings is explicitly set on showPr.
    pub fn has_use_timings(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.get_attribute("useTimings"))
            .is_some())
    }

    /// Remove explicit `useTimings` from showPr. Returns whether it was present.
    pub fn clear_use_timings(&mut self) -> Result<bool> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let Some(show_pr) = root.child_mut("showPr") else {
            return Ok(false);
        };
        let before = show_pr.attributes.len();
        show_pr.attributes.retain(|a| a.local_name != "useTimings");
        if show_pr.attributes.len() == before {
            return Ok(false);
        }
        self.save_presentation_properties(uri, &root)?;
        Ok(true)
    }

    pub fn set_show_mode_presented(&mut self) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr
            .children
            .retain(|c| !matches!(c.local_name.as_str(), "present" | "browse" | "kiosk"));
        show_pr.append_child(OpenXmlElement::new("p", p, "present"));
        self.save_presentation_properties(uri, &root)
    }

    /// Set slideshow mode to browse (`p:browse`, optional `showScrollbar`).
    pub fn set_show_mode_browse(&mut self, show_scrollbar: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr
            .children
            .retain(|c| !matches!(c.local_name.as_str(), "present" | "browse" | "kiosk"));
        show_pr.append_child(
            OpenXmlElement::new("p", p, "browse")
                .with_attribute("showScrollbar", if show_scrollbar { "1" } else { "0" }),
        );
        self.save_presentation_properties(uri, &root)
    }

    /// Update browse scrollbar without changing mode (no-op if not browse).
    pub fn set_show_browse_scrollbar(&mut self, show_scrollbar: bool) -> Result<bool> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let Some(show_pr) = root.child_mut("showPr") else {
            return Ok(false);
        };
        let Some(browse) = show_pr.child_mut("browse") else {
            return Ok(false);
        };
        browse.set_attribute("showScrollbar", if show_scrollbar { "1" } else { "0" });
        self.save_presentation_properties(uri, &root)?;
        Ok(true)
    }

    /// Set slideshow mode to kiosk with restart interval in ms (`p:kiosk/@restart`).
    pub fn set_show_mode_kiosk(&mut self, restart_ms: u32) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr
            .children
            .retain(|c| !matches!(c.local_name.as_str(), "present" | "browse" | "kiosk"));
        show_pr.append_child(
            OpenXmlElement::new("p", p, "kiosk").with_attribute("restart", restart_ms.to_string()),
        );
        self.save_presentation_properties(uri, &root)
    }

    /// Update kiosk restart interval without changing mode (no-op if not kiosk).
    pub fn set_show_kiosk_restart_ms(&mut self, restart_ms: u32) -> Result<bool> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let Some(show_pr) = root.child_mut("showPr") else {
            return Ok(false);
        };
        let Some(kiosk) = show_pr.child_mut("kiosk") else {
            return Ok(false);
        };
        kiosk.set_attribute("restart", restart_ms.to_string());
        self.save_presentation_properties(uri, &root)?;
        Ok(true)
    }

    /// Read slideshow mode: `"presented"`, `"browse"`, `"kiosk"`, or `None`.
    pub fn show_mode(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(show) = root.child("showPr") else {
            return Ok(None);
        };
        if show.child("present").is_some() {
            return Ok(Some("presented".into()));
        }
        if show.child("browse").is_some() {
            return Ok(Some("browse".into()));
        }
        if show.child("kiosk").is_some() {
            return Ok(Some("kiosk".into()));
        }
        Ok(None)
    }

    /// Set slideshow slide range (`p:showPr/p:sldRg` with start/end, 0-based indices).
    ///
    /// Removes `sldAll` / `custShow` when setting a range.
    pub fn set_show_slide_range(&mut self, start: u32, end: u32) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr
            .children
            .retain(|c| !matches!(c.local_name.as_str(), "sldAll" | "sldRg" | "custShow"));
        show_pr.append_child(
            OpenXmlElement::new("p", p, "sldRg")
                .with_attribute("st", start.to_string())
                .with_attribute("end", end.to_string()),
        );
        self.save_presentation_properties(uri, &root)
    }

    /// Show all slides during slideshow (`p:sldAll`).
    pub fn set_show_all_slides(&mut self) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr
            .children
            .retain(|c| !matches!(c.local_name.as_str(), "sldAll" | "sldRg" | "custShow"));
        show_pr.append_child(OpenXmlElement::new("p", p, "sldAll"));
        self.save_presentation_properties(uri, &root)
    }

    /// Use a custom show during slideshow (`p:custShow/@id`).
    pub fn set_show_custom_show(&mut self, custom_show_id: u32) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr
            .children
            .retain(|c| !matches!(c.local_name.as_str(), "sldAll" | "sldRg" | "custShow"));
        show_pr.append_child(
            OpenXmlElement::new("p", p, "custShow")
                .with_attribute("id", custom_show_id.to_string()),
        );
        self.save_presentation_properties(uri, &root)
    }

    /// Read show slide range as `(start, end)` when `sldRg` is present.
    pub fn show_slide_range(&self) -> Result<Option<(u32, u32)>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(show) = root.child("showPr") else {
            return Ok(None);
        };
        let Some(rg) = show.child("sldRg") else {
            return Ok(None);
        };
        let st = rg
            .get_attribute("st")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let end = rg
            .get_attribute("end")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        Ok(Some((st, end)))
    }

    /// Whether show is set to all slides.
    pub fn show_all_slides(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .map(|s| s.child("sldAll").is_some())
            .unwrap_or(false))
    }

    /// Read custom show id used for slideshow, if any.
    pub fn show_custom_show_id(&self) -> Result<Option<u32>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.child("custShow"))
            .and_then(|c| c.get_attribute("id"))
            .and_then(|s| s.parse().ok()))
    }

    /// Read browse mode showScrollbar flag when mode is browse.
    pub fn show_browse_scrollbar(&self) -> Result<Option<bool>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(browse) = root.child("showPr").and_then(|s| s.child("browse")) else {
            return Ok(None);
        };
        Ok(Some(
            browse
                .get_attribute("showScrollbar")
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(true),
        ))
    }

    /// Read kiosk restart interval in ms when mode is kiosk.
    pub fn show_kiosk_restart_ms(&self) -> Result<Option<u32>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.child("kiosk"))
            .and_then(|k| k.get_attribute("restart"))
            .and_then(|s| s.parse().ok()))
    }

    /// Whether a slideshow mode child is present.
    pub fn has_show_mode(&self) -> Result<bool> {
        Ok(self.show_mode()?.is_some())
    }

    /// Clear present/browse/kiosk mode children from showPr.
    pub fn clear_show_mode(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(show_pr) = root.child_mut("showPr") else {
            return Ok(false);
        };
        let before = show_pr.children.len();
        show_pr
            .children
            .retain(|c| !matches!(c.local_name.as_str(), "present" | "browse" | "kiosk"));
        let removed = show_pr.children.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Whether a slide range is configured for show.
    pub fn has_show_slide_range(&self) -> Result<bool> {
        Ok(self.show_slide_range()?.is_some())
    }

    /// Clear `sldRg` from showPr.
    pub fn clear_show_slide_range(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(show_pr) = root.child_mut("showPr") else {
            return Ok(false);
        };
        let before = show_pr.children.len();
        show_pr.children.retain(|c| c.local_name != "sldRg");
        let removed = show_pr.children.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear `sldAll` from showPr.
    pub fn clear_show_all_slides(&mut self) -> Result<bool> {
        let had = self.show_all_slides()?;
        if !had {
            return Ok(false);
        }
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(show_pr) = root.child_mut("showPr") else {
            return Ok(false);
        };
        show_pr.children.retain(|c| c.local_name != "sldAll");
        self.save_presentation_properties(uri, &root)?;
        Ok(true)
    }

    /// Whether custom show is selected for slideshow.
    pub fn has_show_custom_show(&self) -> Result<bool> {
        Ok(self.show_custom_show_id()?.is_some())
    }

    /// Clear `custShow` from showPr.
    pub fn clear_show_custom_show(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(show_pr) = root.child_mut("showPr") else {
            return Ok(false);
        };
        let before = show_pr.children.len();
        show_pr.children.retain(|c| c.local_name != "custShow");
        let removed = show_pr.children.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Set ink pen color for slideshow (`p:showPr/p:penClr` with srgb).
    pub fn set_show_pen_color(&mut self, rgb: &str) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let a = crate::namespace::ns::DRAWINGML.uri;
        let show_pr = self.ensure_show_pr_mut(&mut root);
        show_pr.children.retain(|c| c.local_name != "penClr");
        show_pr.append_child(
            OpenXmlElement::new("p", p, "penClr")
                .with_child(OpenXmlElement::new("a", a, "srgbClr").with_attribute("val", rgb)),
        );
        self.save_presentation_properties(uri, &root)
    }

    /// Read pen color RGB when present.
    pub fn show_pen_color(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("showPr")
            .and_then(|s| s.child("penClr"))
            .and_then(|p| p.child("srgbClr"))
            .and_then(|c| c.get_attribute("val").map(|s| s.to_string())))
    }

    /// Clear pen color. Returns whether present.
    pub fn clear_show_pen_color(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(show) = root.child_mut("showPr") else {
            return Ok(false);
        };
        let before = show.children.len();
        show.children.retain(|c| c.local_name != "penClr");
        let removed = show.children.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Whether a `showPr` element exists.
    pub fn has_show_properties(&self) -> bool {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return false;
        };
        parse_element(data)
            .map(|r| r.child("showPr").is_some())
            .unwrap_or(false)
    }

    /// Clear showPr. Returns whether it was present.
    pub fn clear_show_properties(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "showPr");
        let removed = root.children.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Add a custom show (`p:custShowLst/p:custShow`) selecting slides by 0-based index.
    ///
    /// Returns the custom show id.
    pub fn add_custom_show(&mut self, name: &str, slide_indices: &[usize]) -> Result<u32> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let r_ns = crate::namespace::ns::RELATIONSHIPS.uri;

        // Next custom show id
        let next_id = root
            .child("custShowLst")
            .map(|lst| {
                lst.children_by_name("custShow")
                    .filter_map(|c| c.get_attribute("id").and_then(|s| s.parse::<u32>().ok()))
                    .max()
                    .unwrap_or(0)
                    + 1
            })
            .unwrap_or(1);

        let mut sld_lst = OpenXmlElement::new("p", p, "sldLst");
        for &idx in slide_indices {
            let info = self
                .slides
                .get(idx)
                .ok_or_else(|| Error::Package(format!("slide index {idx} out of range")))?;
            sld_lst.append_child(
                OpenXmlElement::new("p", p, "sld")
                    .with_attribute_qname("r:id", &info.relationship_id)
                    .with_ns_decl("r", r_ns),
            );
        }
        let show = OpenXmlElement::new("p", p, "custShow")
            .with_attribute("name", name)
            .with_attribute("id", next_id.to_string())
            .with_child(sld_lst);

        if let Some(lst) = root.child_mut("custShowLst") {
            lst.append_child(show);
        } else {
            let lst = OpenXmlElement::new("p", p, "custShowLst").with_child(show);
            // After sldIdLst typically
            let insert_at = root
                .children
                .iter()
                .position(|c| c.local_name == "sldIdLst")
                .map(|i| i + 1)
                .unwrap_or(root.children.len());
            root.children.insert(insert_at, lst);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(next_id)
    }

    /// List custom shows as `(id, name, slide_rids)`.
    pub fn list_custom_shows(&self) -> Result<Vec<(u32, String, Vec<String>)>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(lst) = root.child("custShowLst") else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for show in lst.children_by_name("custShow") {
            let id = show
                .get_attribute("id")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let name = show.get_attribute("name").unwrap_or("").to_string();
            let rids = show
                .child("sldLst")
                .map(|sl| {
                    sl.children_by_name("sld")
                        .filter_map(|s| {
                            s.get_attribute_qname("r:id")
                                .or_else(|| s.get_attribute("id"))
                                .map(|x| x.to_string())
                        })
                        .collect()
                })
                .unwrap_or_default();
            out.push((id, name, rids));
        }
        Ok(out)
    }

    /// Number of custom shows.
    pub fn custom_show_count(&self) -> Result<usize> {
        Ok(self.list_custom_shows()?.len())
    }

    /// Whether any custom shows exist.
    pub fn has_custom_shows(&self) -> Result<bool> {
        Ok(self.custom_show_count()? > 0)
    }

    /// List custom show names in order as `(id, name)`.
    pub fn list_custom_show_names(&self) -> Result<Vec<(u32, String)>> {
        Ok(self
            .list_custom_shows()?
            .into_iter()
            .map(|(id, name, _)| (id, name))
            .collect())
    }

    /// Whether a custom show with the given name exists.
    pub fn has_custom_show_named(&self, name: &str) -> Result<bool> {
        Ok(self
            .list_custom_show_names()?
            .iter()
            .any(|(_, n)| n == name))
    }

    /// List custom show names only.
    pub fn custom_show_names(&self) -> Result<Vec<String>> {
        Ok(self
            .list_custom_show_names()?
            .into_iter()
            .map(|(_, n)| n)
            .collect())
    }

    /// Rename a custom show by id. Returns whether found.
    pub fn rename_custom_show(&mut self, id: u32, new_name: &str) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let Some(lst) = root.child_mut("custShowLst") else {
            return Ok(false);
        };
        let mut found = false;
        for show in lst.children.iter_mut() {
            if show.local_name != "custShow" {
                continue;
            }
            let sid = show.get_attribute("id").and_then(|s| s.parse::<u32>().ok());
            if sid == Some(id) {
                show.set_attribute("name", new_name);
                found = true;
                break;
            }
        }
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(found)
    }

    /// Look up a custom show by id as `(name, slide_rids)`.
    pub fn custom_show_by_id(&self, id: u32) -> Result<Option<(String, Vec<String>)>> {
        Ok(self
            .list_custom_shows()?
            .into_iter()
            .find(|(i, _, _)| *i == id)
            .map(|(_, name, rids)| (name, rids)))
    }

    /// Replace the slide list of a custom show by id with the given slide indices.
    pub fn set_custom_show_slides(&mut self, id: u32, slide_indices: &[usize]) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let Some(lst) = root.child_mut("custShowLst") else {
            return Ok(false);
        };
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let r_ns = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
        let mut found = false;
        for show in lst.children.iter_mut() {
            if show.local_name != "custShow" {
                continue;
            }
            let sid = show.get_attribute("id").and_then(|s| s.parse::<u32>().ok());
            if sid != Some(id) {
                continue;
            }
            show.children.retain(|c| c.local_name != "sldLst");
            let mut sld_lst = OpenXmlElement::new("p", p, "sldLst");
            for idx in slide_indices {
                let info = self
                    .slides
                    .get(*idx)
                    .ok_or_else(|| Error::Package(format!("slide index {idx} out of range")))?;
                sld_lst.append_child(
                    OpenXmlElement::new("p", p, "sld")
                        .with_attribute_qname("r:id", &info.relationship_id)
                        .with_ns_decl("r", r_ns),
                );
            }
            show.append_child(sld_lst);
            found = true;
            break;
        }
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(found)
    }

    /// Clear all slides from a custom show (keeps the show entry).
    pub fn clear_custom_show_slides(&mut self, id: u32) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(csl) = root.child_mut("custShowLst") else {
            return Ok(false);
        };
        let mut found = false;
        for show in csl.children.iter_mut() {
            if show.local_name != "custShow" {
                continue;
            }
            if show.get_attribute("id").and_then(|s| s.parse().ok()) != Some(id) {
                continue;
            }
            if let Some(sld_lst) = show.child_mut("sldLst") {
                if !sld_lst.children.is_empty() {
                    sld_lst.children.clear();
                    found = true;
                }
            }
            break;
        }
        if found {
            self.package.set_part(
                pres_uri,
                content_type::PRESENTATION,
                write_element(&root)?,
            );
        }
        Ok(found)
    }

    /// Remove a custom show by id. Returns whether it was present.
    pub fn remove_custom_show(&mut self, id: u32) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(lst) = root.child_mut("custShowLst") else {
            return Ok(false);
        };
        let before = lst.children.len();
        lst.children
            .retain(|c| c.get_attribute("id").and_then(|s| s.parse::<u32>().ok()) != Some(id));
        let removed = lst.children.len() < before;
        if lst.children.is_empty() {
            root.children.retain(|c| c.local_name != "custShowLst");
        }
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Clear all custom shows. Returns how many were removed.
    pub fn clear_custom_shows(&mut self) -> Result<usize> {
        let shows = self.list_custom_shows()?;
        let n = shows.len();
        if n == 0 {
            return Ok(0);
        }
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        root.children.retain(|c| c.local_name != "custShowLst");
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(n)
    }

    /// Set print properties `prnPr` attributes (frameSlides, hiddenSlides, etc.).
    pub fn set_print_frame_slides(&mut self, enabled: bool) -> Result<()> {
        self.set_prn_pr_attr("frameSlides", if enabled { "1" } else { "0" })
    }

    /// Whether frameSlides is set on print properties.
    pub fn print_frame_slides(&self) -> Result<bool> {
        self.prn_pr_bool("frameSlides", false)
    }

    /// Set print properties `hiddenSlides`.
    /// Disable `print frame slides`. Returns whether it was enabled.
    pub fn clear_print_frame_slides(&mut self) -> Result<bool> {
        let had = self.print_frame_slides()?;
        if had {
            self.set_print_frame_slides(false)?;
        }
        Ok(had)
    }

    pub fn set_print_hidden_slides(&mut self, enabled: bool) -> Result<()> {
        self.set_prn_pr_attr("hiddenSlides", if enabled { "1" } else { "0" })
    }

    /// Whether hiddenSlides is printed.
    pub fn print_hidden_slides(&self) -> Result<bool> {
        self.prn_pr_bool("hiddenSlides", false)
    }

    /// Set print properties `scaleToFitPaper`.
    /// Disable `print hidden slides`. Returns whether it was enabled.
    pub fn clear_print_hidden_slides(&mut self) -> Result<bool> {
        let had = self.print_hidden_slides()?;
        if had {
            self.set_print_hidden_slides(false)?;
        }
        Ok(had)
    }

    pub fn set_print_scale_to_fit_paper(&mut self, enabled: bool) -> Result<()> {
        self.set_prn_pr_attr("scaleToFitPaper", if enabled { "1" } else { "0" })
    }

    /// Whether scaleToFitPaper is enabled.
    pub fn print_scale_to_fit_paper(&self) -> Result<bool> {
        self.prn_pr_bool("scaleToFitPaper", false)
    }

    /// Set print color mode (`prnPr/@clrMode`): `"bw"`, `"gray"`, `"clr"`.
    /// Disable `print scale to fit paper`. Returns whether it was enabled.
    pub fn clear_print_scale_to_fit_paper(&mut self) -> Result<bool> {
        let had = self.print_scale_to_fit_paper()?;
        if had {
            self.set_print_scale_to_fit_paper(false)?;
        }
        Ok(had)
    }

    pub fn set_print_color_mode(&mut self, mode: &str) -> Result<()> {
        self.set_prn_pr_attr("clrMode", mode)
    }

    /// Read print color mode.
    pub fn print_color_mode(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("prnPr")
            .and_then(|p| p.get_attribute("clrMode").map(|s| s.to_string())))
    }

    /// Set what to print (`prnPr/@prnWhat`): `"slides"`, `"handouts"`, `"notes"`, `"outline"`.
    pub fn set_print_what(&mut self, what: &str) -> Result<()> {
        self.set_prn_pr_attr("prnWhat", what)
    }

    /// Read print-what setting.
    pub fn print_what(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("prnPr")
            .and_then(|p| p.get_attribute("prnWhat").map(|s| s.to_string())))
    }

    /// Whether print color mode is set.
    pub fn has_print_color_mode(&self) -> Result<bool> {
        Ok(self.print_color_mode()?.is_some())
    }

    /// Clear `prnPr/@clrMode`.
    pub fn clear_print_color_mode(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(prn) = root.child_mut("prnPr") else {
            return Ok(false);
        };
        let before = prn.attributes.len();
        prn.attributes.retain(|a| a.local_name != "clrMode");
        let removed = prn.attributes.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Whether print-what is set.
    pub fn has_print_what(&self) -> Result<bool> {
        Ok(self.print_what()?.is_some())
    }

    /// Clear `prnPr/@prnWhat`.
    pub fn clear_print_what(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(prn) = root.child_mut("prnPr") else {
            return Ok(false);
        };
        let before = prn.attributes.len();
        prn.attributes.retain(|a| a.local_name != "prnWhat");
        let removed = prn.attributes.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Clear print properties (`prnPr`). Returns whether present.
    pub fn clear_print_properties(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "prnPr");
        let removed = root.children.len() < before;
        if removed {
            self.save_presentation_properties(uri, &root)?;
        }
        Ok(removed)
    }

    fn ensure_prn_pr_mut<'a>(&self, root: &'a mut OpenXmlElement) -> &'a mut OpenXmlElement {
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if root.child("prnPr").is_none() {
            root.append_child(OpenXmlElement::new("p", p, "prnPr"));
        }
        root.child_mut("prnPr").expect("prnPr ensured")
    }

    fn set_prn_pr_attr(&mut self, attr: &str, value: &str) -> Result<()> {
        let (uri, mut root) = self.ensure_presentation_properties_root()?;
        let prn = self.ensure_prn_pr_mut(&mut root);
        prn.set_attribute(attr, value);
        self.save_presentation_properties(uri, &root)
    }

    fn prn_pr_bool(&self, attr: &str, default: bool) -> Result<bool> {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(default);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("prnPr")
            .and_then(|p| p.get_attribute(attr))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Whether print properties exist.
    pub fn has_print_properties(&self) -> bool {
        let uri = PackUri::new("/ppt/presProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return false;
        };
        parse_element(data)
            .map(|r| r.child("prnPr").is_some())
            .unwrap_or(false)
    }

    /// Add a table styles part shell for presentation tables.
    pub fn add_table_styles(&mut self) -> Result<(PackUri, String)> {
        let pres_uri = self.ensure_presentation()?;
        let uri = PackUri::new("/ppt/tableStyles.xml");
        let a = crate::namespace::ns::DRAWINGML.uri;
        let root = OpenXmlElement::new("a", a, "tblStyleLst")
            .with_ns_decl("a", a)
            .with_attribute("def", "{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}");
        self.package.set_part(
            uri.clone(),
            content_type::PRESENTATION_TABLE_STYLES,
            write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::TABLE_STYLES).map(|r| r.id.clone()))
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::TABLE_STYLES,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    /// Whether a table styles part exists.
    pub fn has_table_styles(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/ppt/tableStyles.xml"))
    }

    /// Whether any styles part exists (table styles or `/ppt/styles`).
    pub fn has_styles(&self) -> bool {
        self.has_table_styles()
            || self
                .package
                .opc()
                .part_uris()
                .into_iter()
                .any(|u| u.as_str().contains("/ppt/styles"))
    }

    /// Count styles-related parts.
    pub fn styles_count(&self) -> usize {
        self.package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| {
                let s = u.as_str();
                s.contains("tableStyles") || s.contains("/ppt/styles")
            })
            .count()
    }

    /// Remove table styles (and any `/ppt/styles` parts).
    pub fn clear_styles(&mut self) -> Result<bool> {
        let mut removed = self.clear_table_styles()?;
        let uris: Vec<PackUri> = self
            .package
            .opc()
            .part_uris()
            .into_iter()
            .filter(|u| u.as_str().contains("/ppt/styles"))
            .collect();
        for uri in uris {
            self.package.opc_mut().remove_part(&uri);
            removed = true;
        }
        Ok(removed)
    }

    /// Remove the table styles part and presentation relationship.
    pub fn clear_table_styles(&mut self) -> Result<bool> {
        self.clear_pres_related_part("/ppt/tableStyles.xml", rel::TABLE_STYLES)
    }

    /// Whether a presentation properties part exists.
    pub fn has_presentation_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/ppt/presProps.xml"))
    }

    /// Whether a view properties part exists.
    pub fn has_view_properties(&self) -> bool {
        self.package
            .opc()
            .has_part(&PackUri::new("/ppt/viewProps.xml"))
    }

    /// Convenience: whether presentation or view properties exist.
    pub fn has_any_properties(&self) -> bool {
        self.has_presentation_properties() || self.has_view_properties()
    }

    /// Whether any of notes/handout masters exist.
    pub fn has_any_master_extras(&self) -> bool {
        self.has_notes_master() || self.has_handout_master()
    }

    /// Total notes + handout master parts.
    pub fn extra_master_count(&self) -> usize {
        self.notes_master_count() + self.handout_master_count()
    }

    /// Remove presentation properties part.
    pub fn clear_presentation_properties(&mut self) -> Result<bool> {
        self.clear_pres_related_part("/ppt/presProps.xml", rel::PRES_PROPS)
    }

    /// Remove view properties part.
    pub fn clear_view_properties(&mut self) -> Result<bool> {
        self.clear_pres_related_part("/ppt/viewProps.xml", rel::VIEW_PROPS)
    }

    fn clear_pres_related_part(&mut self, uri_str: &str, rel_type: &str) -> Result<bool> {
        let uri = PackUri::new(uri_str);
        if !self.package.opc().has_part(&uri) {
            return Ok(false);
        }
        if let Ok(pres_uri) = self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&pres_uri)
                .map(|rels| {
                    rels.find_all_by_type(rel_type)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            let rels = self.package.opc_mut().part_relationships_mut(&pres_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        self.package.opc_mut().remove_part(&uri);
        Ok(true)
    }

    /// Add view properties part (`ppt/viewProps.xml`).
    pub fn add_view_properties(&mut self) -> Result<(PackUri, String)> {
        let pres_uri = self.ensure_presentation()?;
        let uri = PackUri::new("/ppt/viewProps.xml");
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let root = OpenXmlElement::new("p", p, "viewPr")
            .with_ns_decl("p", p)
            .with_ns_decl("a", crate::namespace::ns::DRAWINGML.uri)
            .with_attribute("lastView", "sldView")
            .with_child(
                OpenXmlElement::new("p", p, "normalViewPr")
                    .with_child(
                        OpenXmlElement::new("p", p, "restoredLeft")
                            .with_attribute("sz", "15620")
                            .with_attribute("autoAdjust", "0"),
                    )
                    .with_child(
                        OpenXmlElement::new("p", p, "restoredTop")
                            .with_attribute("sz", "94660")
                            .with_attribute("autoAdjust", "0"),
                    ),
            );
        self.package.set_part(
            uri.clone(),
            content_type::PRESENTATION_VIEW_PROPS,
            write_element(&root)?,
        );
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::VIEW_PROPS).map(|r| r.id.clone()))
        {
            return Ok((uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::VIEW_PROPS,
            &uri,
            RelationshipTargetMode::Internal,
        );
        Ok((uri, rid))
    }

    fn ensure_view_properties_root(&mut self) -> Result<(PackUri, OpenXmlElement)> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        if let Some(data) = self.package.opc().get_part(&uri) {
            return Ok((uri, parse_element(data)?));
        }
        self.add_view_properties()?;
        let data = self
            .package
            .opc()
            .get_part(&uri)
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))?;
        Ok((uri, parse_element(data)?))
    }

    fn save_view_properties(&mut self, uri: PackUri, root: &OpenXmlElement) -> Result<()> {
        self.package.set_part(
            uri,
            content_type::PRESENTATION_VIEW_PROPS,
            write_element(root)?,
        );
        Ok(())
    }

    /// Set view properties `lastView` (e.g. `"sldView"`, `"sldThumbnailView"`, `"notesView"`).
    pub fn set_last_view(&mut self, view: &str) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        root.set_attribute("lastView", view);
        self.save_view_properties(uri, &root)
    }

    /// Read `lastView` from view properties.
    pub fn last_view(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute("lastView").map(|s| s.to_string()))
    }

    /// Clear `lastView` attribute. Returns whether present.
    pub fn clear_last_view(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.attributes.len();
        root.attributes.retain(|a| a.local_name != "lastView");
        let removed = root.attributes.len() < before;
        if removed {
            self.save_view_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Set view properties `showComments`.
    pub fn set_show_comments(&mut self, show: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        root.set_attribute("showComments", if show { "1" } else { "0" });
        self.save_view_properties(uri, &root)
    }

    /// Whether showComments is enabled (default true when unset).
    pub fn show_comments(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("showComments")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set grid spacing in view properties (`p:gridSpacing` cx/cy EMUs).
    /// Disable `show comments`. Returns whether it was enabled.
    pub fn clear_show_comments(&mut self) -> Result<bool> {
        let had = self.show_comments()?;
        if had {
            self.set_show_comments(false)?;
        }
        Ok(had)
    }

    pub fn set_grid_spacing(&mut self, cx: i64, cy: i64) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        root.children.retain(|c| c.local_name != "gridSpacing");
        let a = crate::namespace::ns::DRAWINGML.uri;
        // gridSpacing uses p: prefix in presentation view props but CT is a:CT_PositiveSize2D
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.append_child(
            OpenXmlElement::new("p", p, "gridSpacing")
                .with_attribute("cx", cx.to_string())
                .with_attribute("cy", cy.to_string())
                .with_ns_decl("a", a),
        );
        self.save_view_properties(uri, &root)
    }

    /// Read grid spacing `(cx, cy)` when present.
    pub fn grid_spacing(&self) -> Result<Option<(i64, i64)>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(gs) = root.child("gridSpacing") else {
            return Ok(None);
        };
        let cx = gs
            .get_attribute("cx")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let cy = gs
            .get_attribute("cy")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        Ok(Some((cx, cy)))
    }

    /// Whether grid spacing is set.
    pub fn has_grid_spacing(&self) -> Result<bool> {
        Ok(self.grid_spacing()?.is_some())
    }

    /// Clear grid spacing. Returns whether it was present.
    pub fn clear_grid_spacing(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "gridSpacing");
        let removed = root.children.len() < before;
        if removed {
            self.save_view_properties(uri, &root)?;
        }
        Ok(removed)
    }

    fn ensure_normal_view_pr_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if root.child("normalViewPr").is_none() {
            root.append_child(
                OpenXmlElement::new("p", p, "normalViewPr")
                    .with_child(
                        OpenXmlElement::new("p", p, "restoredLeft")
                            .with_attribute("sz", "15620")
                            .with_attribute("autoAdjust", "0"),
                    )
                    .with_child(
                        OpenXmlElement::new("p", p, "restoredTop")
                            .with_attribute("sz", "94660")
                            .with_attribute("autoAdjust", "0"),
                    ),
            );
        }
        root.child_mut("normalViewPr")
            .expect("normalViewPr ensured")
    }

    fn set_normal_view_bool_attr(&mut self, attr: &str, value: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let nvp = self.ensure_normal_view_pr_mut(&mut root);
        nvp.set_attribute(attr, if value { "1" } else { "0" });
        self.save_view_properties(uri, &root)
    }

    fn normal_view_bool_attr(&self, attr: &str, default: bool) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(default);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("normalViewPr")
            .and_then(|n| n.get_attribute(attr))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Set normal view `showOutlineIcons`.
    pub fn set_show_outline_icons(&mut self, show: bool) -> Result<()> {
        self.set_normal_view_bool_attr("showOutlineIcons", show)
    }

    /// Whether outline icons are shown in normal view (default true when unset).
    pub fn show_outline_icons(&self) -> Result<bool> {
        self.normal_view_bool_attr("showOutlineIcons", true)
    }

    /// Set normal view `preferSingleView`.
    /// Disable `show outline icons`. Returns whether it was enabled.
    pub fn clear_show_outline_icons(&mut self) -> Result<bool> {
        let had = self.show_outline_icons()?;
        if had {
            self.set_show_outline_icons(false)?;
        }
        Ok(had)
    }

    pub fn set_prefer_single_view(&mut self, prefer: bool) -> Result<()> {
        self.set_normal_view_bool_attr("preferSingleView", prefer)
    }

    /// Whether preferSingleView is set (default false when unset).
    pub fn prefer_single_view(&self) -> Result<bool> {
        self.normal_view_bool_attr("preferSingleView", false)
    }

    /// Set normal view `snapVertSplitter`.
    /// Disable `prefer single view`. Returns whether it was enabled.
    pub fn clear_prefer_single_view(&mut self) -> Result<bool> {
        let had = self.prefer_single_view()?;
        if had {
            self.set_prefer_single_view(false)?;
        }
        Ok(had)
    }

    pub fn set_snap_vert_splitter(&mut self, snap: bool) -> Result<()> {
        self.set_normal_view_bool_attr("snapVertSplitter", snap)
    }

    /// Whether snapVertSplitter is set (default false when unset).
    pub fn snap_vert_splitter(&self) -> Result<bool> {
        self.normal_view_bool_attr("snapVertSplitter", false)
    }

    /// Set normal view vertical bar state (`vertBarState`: `"minimized"`, `"restored"`, `"maximized"`).
    /// Disable `snap vert splitter`. Returns whether it was enabled.
    pub fn clear_snap_vert_splitter(&mut self) -> Result<bool> {
        let had = self.snap_vert_splitter()?;
        if had {
            self.set_snap_vert_splitter(false)?;
        }
        Ok(had)
    }

    pub fn set_vert_bar_state(&mut self, state: &str) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let nvp = self.ensure_normal_view_pr_mut(&mut root);
        nvp.set_attribute("vertBarState", state);
        self.save_view_properties(uri, &root)
    }

    /// Read `vertBarState` when present.
    pub fn vert_bar_state(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("normalViewPr")
            .and_then(|n| n.get_attribute("vertBarState").map(|s| s.to_string())))
    }

    /// Set normal view horizontal bar state (`horzBarState`).
    pub fn set_horz_bar_state(&mut self, state: &str) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let nvp = self.ensure_normal_view_pr_mut(&mut root);
        nvp.set_attribute("horzBarState", state);
        self.save_view_properties(uri, &root)
    }

    /// Read `horzBarState` when present.
    pub fn horz_bar_state(&self) -> Result<Option<String>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("normalViewPr")
            .and_then(|n| n.get_attribute("horzBarState").map(|s| s.to_string())))
    }

    /// Set normal view restored left portion size (`restoredLeft/@sz`, 0–100000).
    pub fn set_restored_left(&mut self, sz: u32, auto_adjust: Option<bool>) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let nvp = self.ensure_normal_view_pr_mut(&mut root);
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if let Some(el) = nvp.child_mut("restoredLeft") {
            el.set_attribute("sz", sz.to_string());
            if let Some(aa) = auto_adjust {
                el.set_attribute("autoAdjust", if aa { "1" } else { "0" });
            }
        } else {
            let mut el =
                OpenXmlElement::new("p", p, "restoredLeft").with_attribute("sz", sz.to_string());
            if let Some(aa) = auto_adjust {
                el.set_attribute("autoAdjust", if aa { "1" } else { "0" });
            }
            nvp.append_child(el);
        }
        self.save_view_properties(uri, &root)
    }

    /// Read restored left `(sz, auto_adjust)`.
    pub fn restored_left(&self) -> Result<Option<(u32, bool)>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(el) = root
            .child("normalViewPr")
            .and_then(|n| n.child("restoredLeft"))
        else {
            return Ok(None);
        };
        let sz = el
            .get_attribute("sz")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let aa = el
            .get_attribute("autoAdjust")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        Ok(Some((sz, aa)))
    }

    /// Set normal view restored top portion size (`restoredTop/@sz`, 0–100000).
    pub fn set_restored_top(&mut self, sz: u32, auto_adjust: Option<bool>) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let nvp = self.ensure_normal_view_pr_mut(&mut root);
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if let Some(el) = nvp.child_mut("restoredTop") {
            el.set_attribute("sz", sz.to_string());
            if let Some(aa) = auto_adjust {
                el.set_attribute("autoAdjust", if aa { "1" } else { "0" });
            }
        } else {
            let mut el =
                OpenXmlElement::new("p", p, "restoredTop").with_attribute("sz", sz.to_string());
            if let Some(aa) = auto_adjust {
                el.set_attribute("autoAdjust", if aa { "1" } else { "0" });
            }
            nvp.append_child(el);
        }
        self.save_view_properties(uri, &root)
    }

    /// Read restored top `(sz, auto_adjust)`.
    pub fn restored_top(&self) -> Result<Option<(u32, bool)>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(el) = root
            .child("normalViewPr")
            .and_then(|n| n.child("restoredTop"))
        else {
            return Ok(None);
        };
        let sz = el
            .get_attribute("sz")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let aa = el
            .get_attribute("autoAdjust")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        Ok(Some((sz, aa)))
    }

    /// Whether `horzBarState` is set.
    pub fn has_horz_bar_state(&self) -> Result<bool> {
        Ok(self.horz_bar_state()?.is_some())
    }

    /// Clear `horzBarState`.
    pub fn clear_horz_bar_state(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(nvp) = root.child_mut("normalViewPr") else {
            return Ok(false);
        };
        let before = nvp.attributes.len();
        nvp.attributes.retain(|a| a.local_name != "horzBarState");
        let removed = nvp.attributes.len() < before;
        if removed {
            self.save_view_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Whether `vertBarState` is set.
    pub fn has_vert_bar_state(&self) -> Result<bool> {
        Ok(self.vert_bar_state()?.is_some())
    }

    /// Clear `vertBarState`.
    pub fn clear_vert_bar_state(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(nvp) = root.child_mut("normalViewPr") else {
            return Ok(false);
        };
        let before = nvp.attributes.len();
        nvp.attributes.retain(|a| a.local_name != "vertBarState");
        let removed = nvp.attributes.len() < before;
        if removed {
            self.save_view_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Whether restoredLeft is present.
    pub fn has_restored_left(&self) -> Result<bool> {
        Ok(self.restored_left()?.is_some())
    }

    /// Clear restoredLeft.
    pub fn clear_restored_left(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(nvp) = root.child_mut("normalViewPr") else {
            return Ok(false);
        };
        let before = nvp.children.len();
        nvp.children.retain(|c| c.local_name != "restoredLeft");
        let removed = nvp.children.len() < before;
        if removed {
            self.save_view_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Whether restoredTop is present.
    pub fn has_restored_top(&self) -> Result<bool> {
        Ok(self.restored_top()?.is_some())
    }

    /// Clear restoredTop.
    pub fn clear_restored_top(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(nvp) = root.child_mut("normalViewPr") else {
            return Ok(false);
        };
        let before = nvp.children.len();
        nvp.children.retain(|c| c.local_name != "restoredTop");
        let removed = nvp.children.len() < before;
        if removed {
            self.save_view_properties(uri, &root)?;
        }
        Ok(removed)
    }

    /// Ensure `slideViewPr/cSldViewPr` exists under view properties.
    fn ensure_slide_c_sld_view_pr_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if root.child("slideViewPr").is_none() {
            root.append_child(
                OpenXmlElement::new("p", p, "slideViewPr").with_child(
                    OpenXmlElement::new("p", p, "cSldViewPr").with_child(
                        OpenXmlElement::new("p", p, "cViewPr").with_child(
                            OpenXmlElement::new("p", p, "scale").with_child(
                                OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, "sx")
                                    .with_attribute("n", "100")
                                    .with_attribute("d", "100"),
                            ),
                        ),
                    ),
                ),
            );
        }
        let svp = root.child_mut("slideViewPr").expect("slideViewPr");
        if svp.child("cSldViewPr").is_none() {
            svp.append_child(OpenXmlElement::new("p", p, "cSldViewPr"));
        }
        svp.child_mut("cSldViewPr").expect("cSldViewPr")
    }

    fn set_slide_view_bool_attr(&mut self, attr: &str, value: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let c = self.ensure_slide_c_sld_view_pr_mut(&mut root);
        c.set_attribute(attr, if value { "1" } else { "0" });
        self.save_view_properties(uri, &root)
    }

    fn slide_view_bool_attr(&self, attr: &str, default: bool) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(default);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("slideViewPr")
            .and_then(|s| s.child("cSldViewPr"))
            .and_then(|c| c.get_attribute(attr))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Set slide view `snapToGrid` (`slideViewPr/cSldViewPr/@snapToGrid`).
    pub fn set_snap_to_grid(&mut self, snap: bool) -> Result<()> {
        self.set_slide_view_bool_attr("snapToGrid", snap)
    }

    /// Whether snapToGrid is enabled (default true when unset).
    pub fn snap_to_grid(&self) -> Result<bool> {
        self.slide_view_bool_attr("snapToGrid", true)
    }

    /// Set slide view `snapToObjects`.
    /// Disable `snap to grid`. Returns whether it was enabled.
    pub fn clear_snap_to_grid(&mut self) -> Result<bool> {
        let had = self.snap_to_grid()?;
        if had {
            self.set_snap_to_grid(false)?;
        }
        Ok(had)
    }

    pub fn set_snap_to_objects(&mut self, snap: bool) -> Result<()> {
        self.set_slide_view_bool_attr("snapToObjects", snap)
    }

    /// Whether snapToObjects is enabled (default false when unset).
    pub fn snap_to_objects(&self) -> Result<bool> {
        self.slide_view_bool_attr("snapToObjects", false)
    }

    /// Set slide view `showGuides`.
    /// Disable `snap to objects`. Returns whether it was enabled.
    pub fn clear_snap_to_objects(&mut self) -> Result<bool> {
        let had = self.snap_to_objects()?;
        if had {
            self.set_snap_to_objects(false)?;
        }
        Ok(had)
    }

    pub fn set_show_guides(&mut self, show: bool) -> Result<()> {
        self.set_slide_view_bool_attr("showGuides", show)
    }

    /// Whether showGuides is enabled (default false when unset).
    pub fn show_guides(&self) -> Result<bool> {
        self.slide_view_bool_attr("showGuides", false)
    }

    /// Set sorter view `showFormatting`.
    /// Disable `show guides`. Returns whether it was enabled.
    pub fn clear_show_guides(&mut self) -> Result<bool> {
        let had = self.show_guides()?;
        if had {
            self.set_show_guides(false)?;
        }
        Ok(had)
    }

    pub fn set_sorter_show_formatting(&mut self, show: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if root.child("sorterViewPr").is_none() {
            root.append_child(
                OpenXmlElement::new("p", p, "sorterViewPr").with_child(
                    OpenXmlElement::new("p", p, "cViewPr").with_child(
                        OpenXmlElement::new("p", p, "scale").with_child(
                            OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, "sx")
                                .with_attribute("n", "100")
                                .with_attribute("d", "100"),
                        ),
                    ),
                ),
            );
        }
        if let Some(sv) = root.child_mut("sorterViewPr") {
            sv.set_attribute("showFormatting", if show { "1" } else { "0" });
        }
        self.save_view_properties(uri, &root)
    }

    /// Whether sorter view shows formatting (default true when unset).
    pub fn sorter_show_formatting(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sorterViewPr")
            .and_then(|s| s.get_attribute("showFormatting"))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set sorter show-formatting to false. Returns the previous value.
    pub fn clear_sorter_show_formatting(&mut self) -> Result<bool> {
        let had = self.sorter_show_formatting()?;
        self.set_sorter_show_formatting(false)?;
        Ok(had)
    }

    /// Ensure `notesViewPr/cSldViewPr` exists under view properties.
    fn ensure_notes_c_sld_view_pr_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if root.child("notesViewPr").is_none() {
            root.append_child(
                OpenXmlElement::new("p", p, "notesViewPr").with_child(
                    OpenXmlElement::new("p", p, "cSldViewPr").with_child(
                        OpenXmlElement::new("p", p, "cViewPr").with_child(
                            OpenXmlElement::new("p", p, "scale").with_child(
                                OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, "sx")
                                    .with_attribute("n", "100")
                                    .with_attribute("d", "100"),
                            ),
                        ),
                    ),
                ),
            );
        }
        let nvp = root.child_mut("notesViewPr").expect("notesViewPr");
        if nvp.child("cSldViewPr").is_none() {
            nvp.append_child(OpenXmlElement::new("p", p, "cSldViewPr"));
        }
        nvp.child_mut("cSldViewPr").expect("cSldViewPr")
    }

    fn set_notes_view_bool_attr(&mut self, attr: &str, value: bool) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let c = self.ensure_notes_c_sld_view_pr_mut(&mut root);
        c.set_attribute(attr, if value { "1" } else { "0" });
        self.save_view_properties(uri, &root)
    }

    fn notes_view_bool_attr(&self, attr: &str, default: bool) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(default);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("notesViewPr")
            .and_then(|s| s.child("cSldViewPr"))
            .and_then(|c| c.get_attribute(attr))
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    /// Set notes view `snapToGrid`.
    pub fn set_notes_snap_to_grid(&mut self, snap: bool) -> Result<()> {
        self.set_notes_view_bool_attr("snapToGrid", snap)
    }

    /// Whether notes view snapToGrid is enabled (default true when unset).
    pub fn notes_snap_to_grid(&self) -> Result<bool> {
        self.notes_view_bool_attr("snapToGrid", true)
    }

    /// Set notes view `snapToObjects`.
    /// Disable `notes snap to grid`. Returns whether it was enabled.
    pub fn clear_notes_snap_to_grid(&mut self) -> Result<bool> {
        let had = self.notes_snap_to_grid()?;
        if had {
            self.set_notes_snap_to_grid(false)?;
        }
        Ok(had)
    }

    pub fn set_notes_snap_to_objects(&mut self, snap: bool) -> Result<()> {
        self.set_notes_view_bool_attr("snapToObjects", snap)
    }

    /// Whether notes view snapToObjects is enabled (default false when unset).
    pub fn notes_snap_to_objects(&self) -> Result<bool> {
        self.notes_view_bool_attr("snapToObjects", false)
    }

    /// Set notes view `showGuides`.
    /// Disable `notes snap to objects`. Returns whether it was enabled.
    pub fn clear_notes_snap_to_objects(&mut self) -> Result<bool> {
        let had = self.notes_snap_to_objects()?;
        if had {
            self.set_notes_snap_to_objects(false)?;
        }
        Ok(had)
    }

    pub fn set_notes_show_guides(&mut self, show: bool) -> Result<()> {
        self.set_notes_view_bool_attr("showGuides", show)
    }

    /// Whether notes view showGuides is enabled (default false when unset).
    pub fn notes_show_guides(&self) -> Result<bool> {
        self.notes_view_bool_attr("showGuides", false)
    }

    /// Add a guide to the slide view guide list (`slideViewPr/cSldViewPr/guideLst/guide`).
    ///
    /// `orient` is typically `"horz"` or `"vert"`; `pos` is in EMUs from the origin.
    /// Disable `notes show guides`. Returns whether it was enabled.
    pub fn clear_notes_show_guides(&mut self) -> Result<bool> {
        let had = self.notes_show_guides()?;
        if had {
            self.set_notes_show_guides(false)?;
        }
        Ok(had)
    }

    pub fn add_slide_guide(&mut self, orient: &str, pos: i32) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let c = self.ensure_slide_c_sld_view_pr_mut(&mut root);
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if c.child("guideLst").is_none() {
            c.append_child(OpenXmlElement::new("p", p, "guideLst"));
        }
        let lst = c.child_mut("guideLst").expect("guideLst");
        lst.append_child(
            OpenXmlElement::new("p", p, "guide")
                .with_attribute("orient", orient)
                .with_attribute("pos", pos.to_string()),
        );
        self.save_view_properties(uri, &root)
    }

    /// Number of slide view guides.
    pub fn slide_guide_count(&self) -> Result<usize> {
        Ok(self.list_slide_guides()?.len())
    }

    /// Whether any slide view guides exist.
    pub fn has_slide_guides(&self) -> Result<bool> {
        Ok(self.slide_guide_count()? > 0)
    }

    /// List slide view guides as `(orient, pos)`.
    pub fn list_slide_guides(&self) -> Result<Vec<(String, i32)>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(lst) = root
            .child("slideViewPr")
            .and_then(|s| s.child("cSldViewPr"))
            .and_then(|c| c.child("guideLst"))
        else {
            return Ok(Vec::new());
        };
        Ok(lst
            .children
            .iter()
            .filter(|g| g.local_name == "guide")
            .map(|g| {
                (
                    g.get_attribute("orient").unwrap_or("horz").to_string(),
                    g.get_attribute("pos")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                )
            })
            .collect())
    }

    /// Clear slide view guides. Returns how many were removed.
    pub fn clear_slide_guides(&mut self) -> Result<usize> {
        let n = self.list_slide_guides()?.len();
        if n == 0 {
            return Ok(0);
        }
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        if let Some(c) = root
            .child_mut("slideViewPr")
            .and_then(|s| s.child_mut("cSldViewPr"))
        {
            c.children.retain(|ch| ch.local_name != "guideLst");
            self.save_view_properties(uri, &root)?;
        }
        Ok(n)
    }

    /// Remove slide guides matching orient and pos. Returns count removed.
    pub fn remove_slide_guide(&mut self, orient: &str, pos: i32) -> Result<usize> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        let Some(lst) = root
            .child_mut("slideViewPr")
            .and_then(|s| s.child_mut("cSldViewPr"))
            .and_then(|c| c.child_mut("guideLst"))
        else {
            return Ok(0);
        };
        let before = lst.children.len();
        let pos_s = pos.to_string();
        lst.children.retain(|g| {
            if g.local_name != "guide" {
                return true;
            }
            !(g.get_attribute("orient") == Some(orient)
                && g.get_attribute("pos") == Some(pos_s.as_str()))
        });
        let n = before - lst.children.len();
        if n > 0 {
            self.save_view_properties(uri, &root)?;
        }
        Ok(n)
    }

    /// Ensure outline view properties exist under viewProps.
    fn ensure_outline_view_pr_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
    ) -> &'a mut OpenXmlElement {
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        if root.child("outlineViewPr").is_none() {
            root.append_child(
                OpenXmlElement::new("p", p, "outlineViewPr").with_child(
                    OpenXmlElement::new("p", p, "cViewPr").with_child(
                        OpenXmlElement::new("p", p, "scale")
                            .with_child(
                                OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, "sx")
                                    .with_attribute("n", "100")
                                    .with_attribute("d", "100"),
                            )
                            .with_child(
                                OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, "sy")
                                    .with_attribute("n", "100")
                                    .with_attribute("d", "100"),
                            ),
                    ),
                ),
            );
        }
        root.child_mut("outlineViewPr").expect("outlineViewPr")
    }

    /// Set outline view scale as `(sx_n, sx_d, sy_n, sy_d)`.
    pub fn set_outline_view_scale(
        &mut self,
        sx_n: i32,
        sx_d: i32,
        sy_n: i32,
        sy_d: i32,
    ) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let ovp = self.ensure_outline_view_pr_mut(&mut root);
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let a = crate::namespace::ns::DRAWINGML.uri;
        if let Some(cv) = ovp.child_mut("cViewPr") {
            cv.children.retain(|c| c.local_name != "scale");
            cv.append_child(
                OpenXmlElement::new("p", p, "scale")
                    .with_child(
                        OpenXmlElement::new("a", a, "sx")
                            .with_attribute("n", sx_n.to_string())
                            .with_attribute("d", sx_d.to_string()),
                    )
                    .with_child(
                        OpenXmlElement::new("a", a, "sy")
                            .with_attribute("n", sy_n.to_string())
                            .with_attribute("d", sy_d.to_string()),
                    ),
            );
        }
        self.save_view_properties(uri, &root)
    }

    /// Read outline view scale as `(sx_n, sx_d, sy_n, sy_d)`.
    pub fn outline_view_scale(&self) -> Result<Option<(i32, i32, i32, i32)>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(scale) = root
            .child("outlineViewPr")
            .and_then(|o| o.child("cViewPr"))
            .and_then(|c| c.child("scale"))
        else {
            return Ok(None);
        };
        let read = |name: &str| -> (i32, i32) {
            let el = scale.child(name);
            (
                el.and_then(|e| e.get_attribute("n"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
                el.and_then(|e| e.get_attribute("d"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            )
        };
        let (sx_n, sx_d) = read("sx");
        let (sy_n, sy_d) = read("sy");
        Ok(Some((sx_n, sx_d, sy_n, sy_d)))
    }

    /// Whether outlineViewPr is present.
    pub fn has_outline_view_pr(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("outlineViewPr").is_some())
    }

    /// Clear `p:outlineViewPr` from viewProps. Returns whether present.
    pub fn clear_outline_view_pr(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "outlineViewPr");
        if root.children.len() == before {
            return Ok(false);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::PRESENTATION_VIEW_PROPS, xml);
        Ok(true)
    }

    /// Alias for [`clear_outline_view_pr`](Self::clear_outline_view_pr).
    pub fn clear_outline_view_scale(&mut self) -> Result<bool> {
        self.clear_outline_view_pr()
    }

    fn ensure_named_c_view_pr_mut<'a>(
        &self,
        root: &'a mut OpenXmlElement,
        local: &str,
    ) -> &'a mut OpenXmlElement {
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let a = crate::namespace::ns::DRAWINGML.uri;
        if root.child(local).is_none() {
            root.append_child(
                OpenXmlElement::new("p", p, local).with_child(
                    OpenXmlElement::new("p", p, "cViewPr").with_child(
                        OpenXmlElement::new("p", p, "scale")
                            .with_child(
                                OpenXmlElement::new("a", a, "sx")
                                    .with_attribute("n", "100")
                                    .with_attribute("d", "100"),
                            )
                            .with_child(
                                OpenXmlElement::new("a", a, "sy")
                                    .with_attribute("n", "100")
                                    .with_attribute("d", "100"),
                            ),
                    ),
                ),
            );
        }
        let view = root.child_mut(local).expect("view pr");
        if view.child("cViewPr").is_none() {
            view.append_child(
                OpenXmlElement::new("p", p, "cViewPr").with_child(
                    OpenXmlElement::new("p", p, "scale")
                        .with_child(
                            OpenXmlElement::new("a", a, "sx")
                                .with_attribute("n", "100")
                                .with_attribute("d", "100"),
                        )
                        .with_child(
                            OpenXmlElement::new("a", a, "sy")
                                .with_attribute("n", "100")
                                .with_attribute("d", "100"),
                        ),
                ),
            );
        }
        view.child_mut("cViewPr").expect("cViewPr")
    }

    fn set_c_view_scale(
        &mut self,
        view_local: &str,
        sx_n: i32,
        sx_d: i32,
        sy_n: i32,
        sy_d: i32,
    ) -> Result<()> {
        let (uri, mut root) = self.ensure_view_properties_root()?;
        let cv = self.ensure_named_c_view_pr_mut(&mut root, view_local);
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let a = crate::namespace::ns::DRAWINGML.uri;
        cv.children.retain(|c| c.local_name != "scale");
        cv.append_child(
            OpenXmlElement::new("p", p, "scale")
                .with_child(
                    OpenXmlElement::new("a", a, "sx")
                        .with_attribute("n", sx_n.to_string())
                        .with_attribute("d", sx_d.to_string()),
                )
                .with_child(
                    OpenXmlElement::new("a", a, "sy")
                        .with_attribute("n", sy_n.to_string())
                        .with_attribute("d", sy_d.to_string()),
                ),
        );
        self.save_view_properties(uri, &root)
    }

    fn c_view_scale(&self, view_local: &str) -> Result<Option<(i32, i32, i32, i32)>> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(scale) = root
            .child(view_local)
            .and_then(|o| o.child("cViewPr"))
            .and_then(|c| c.child("scale"))
        else {
            return Ok(None);
        };
        let read = |name: &str| -> (i32, i32) {
            let el = scale.child(name);
            (
                el.and_then(|e| e.get_attribute("n"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
                el.and_then(|e| e.get_attribute("d"))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            )
        };
        let (sx_n, sx_d) = read("sx");
        let (sy_n, sy_d) = read("sy");
        Ok(Some((sx_n, sx_d, sy_n, sy_d)))
    }

    /// Set notes text view scale.
    pub fn set_notes_text_view_scale(
        &mut self,
        sx_n: i32,
        sx_d: i32,
        sy_n: i32,
        sy_d: i32,
    ) -> Result<()> {
        self.set_c_view_scale("notesTextViewPr", sx_n, sx_d, sy_n, sy_d)
    }

    /// Read notes text view scale.
    pub fn notes_text_view_scale(&self) -> Result<Option<(i32, i32, i32, i32)>> {
        self.c_view_scale("notesTextViewPr")
    }

    /// Set sorter view scale.
    pub fn set_sorter_view_scale(
        &mut self,
        sx_n: i32,
        sx_d: i32,
        sy_n: i32,
        sy_d: i32,
    ) -> Result<()> {
        self.set_c_view_scale("sorterViewPr", sx_n, sx_d, sy_n, sy_d)
    }

    /// Read sorter view scale.
    pub fn sorter_view_scale(&self) -> Result<Option<(i32, i32, i32, i32)>> {
        self.c_view_scale("sorterViewPr")
    }

    /// Whether notesTextViewPr is present.
    pub fn has_notes_text_view_pr(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("notesTextViewPr").is_some())
    }

    /// Clear notesTextViewPr.
    pub fn clear_notes_text_view_pr(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "notesTextViewPr");
        if root.children.len() == before {
            return Ok(false);
        }
        self.save_view_properties(uri, &root)?;
        Ok(true)
    }

    /// Alias for [`clear_notes_text_view_pr`](Self::clear_notes_text_view_pr).
    pub fn clear_notes_text_view_scale(&mut self) -> Result<bool> {
        self.clear_notes_text_view_pr()
    }

    /// Whether sorterViewPr is present.
    pub fn has_sorter_view_pr(&self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("sorterViewPr").is_some())
    }

    /// Clear sorterViewPr.
    pub fn clear_sorter_view_pr(&mut self) -> Result<bool> {
        let uri = PackUri::new("/ppt/viewProps.xml");
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "sorterViewPr");
        if root.children.len() == before {
            return Ok(false);
        }
        self.save_view_properties(uri, &root)?;
        Ok(true)
    }

    /// Alias for [`clear_sorter_view_pr`](Self::clear_sorter_view_pr).
    pub fn clear_sorter_view_scale(&mut self) -> Result<bool> {
        self.clear_sorter_view_pr()
    }

    /// Add a theme part to the presentation (reuses Word default theme structure).
    pub fn add_default_theme(&mut self) -> Result<(PackUri, String)> {
        let pres_uri = self.ensure_presentation()?;
        let theme_uri = PackUri::new("/ppt/theme/theme1.xml");
        let theme = crate::wordprocessing::default_theme("Office Theme");
        // Rewrite theme root namespace is already DrawingML
        let xml = write_element(&theme)?;
        self.package
            .opc_mut()
            .set_part(theme_uri.clone(), content_type::THEME, xml);
        if let Some(existing) = self
            .package
            .opc()
            .part_relationships(&pres_uri)
            .and_then(|rels| rels.get_by_type(rel::THEME).map(|r| r.id.clone()))
        {
            return Ok((theme_uri, existing));
        }
        let rid = self.package.add_part_relationship(
            &pres_uri,
            rel::THEME,
            &theme_uri,
            RelationshipTargetMode::Internal,
        );
        Ok((theme_uri, rid))
    }

    /// Remove a slide by index.
    ///
    /// Deletes the slide part and rewrites the presentation slide list. Does not
    /// clean up orphaned notes/media that only that slide referenced.
    pub fn remove_slide(&mut self, slide_index: usize) -> Result<()> {
        if slide_index >= self.slides.len() {
            return Err(Error::Package(format!(
                "slide index {slide_index} out of range"
            )));
        }
        let info = self.slides.remove(slide_index);
        let pres_uri = self.ensure_presentation()?;
        let _ = self
            .package
            .opc_mut()
            .part_relationships_mut(&pres_uri)
            .remove(&info.relationship_id);
        self.package.opc_mut().remove_part(&info.uri);
        self.rewrite_presentation()
    }

    /// Move a slide from `from` index to `to` index (0-based).
    pub fn move_slide(&mut self, from: usize, to: usize) -> Result<()> {
        if from >= self.slides.len() {
            return Err(Error::Package(format!("slide index {from} out of range")));
        }
        let item = self.slides.remove(from);
        let insert_at = to.min(self.slides.len());
        self.slides.insert(insert_at, item);
        self.rewrite_presentation()
    }

    /// Show or hide a slide (`p:cSld` sibling `p:nvGrpSpPr` / `p:nvPr` show attribute).
    ///
    /// Sets `p:cSld`/`p:nvGrpSpPr` is not used; instead sets `p:sld/@show` to `"0"` when hidden.
    pub fn set_slide_hidden(&mut self, slide_index: usize, hidden: bool) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = if let Some(data) = self.package.opc().get_part(&slide_info.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_info.uri.to_string()));
        };
        if hidden {
            root.set_attribute("show", "0");
        } else {
            root.attributes.retain(|a| a.local_name != "show");
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Clear hidden flag on a slide (unhide). Returns whether it was hidden.
    pub fn clear_slide_hidden(&mut self, slide_index: usize) -> Result<bool> {
        let had = self.is_slide_hidden(slide_index)?;
        if had {
            self.unhide_slide(slide_index)?;
        }
        Ok(had)
    }

    /// Whether a slide is hidden (`p:sld/@show="0"`).
    /// Hide a single slide.
    pub fn hide_slide(&mut self, slide_index: usize) -> Result<()> {
        self.set_slide_hidden(slide_index, true)
    }

    /// Unhide a single slide.
    pub fn unhide_slide(&mut self, slide_index: usize) -> Result<()> {
        self.set_slide_hidden(slide_index, false)
    }

    pub fn is_slide_hidden(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root.get_attribute("show") == Some("0"))
    }

    /// Set whether master shapes are shown on a slide (`p:sld/@showMasterSp`).
    pub fn set_show_master_shapes(&mut self, slide_index: usize, show: bool) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = if let Some(data) = self.package.opc().get_part(&slide_info.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_info.uri.to_string()));
        };
        root.set_attribute("showMasterSp", if show { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Whether master shapes are shown (defaults true when absent).
    pub fn show_master_shapes(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&slide_info.uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("showMasterSp")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set whether master placeholder animations are shown (`p:sld/@showMasterPhAnim`).
    pub fn set_show_master_ph_anim(&mut self, slide_index: usize, show: bool) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = if let Some(data) = self.package.opc().get_part(&slide_info.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_info.uri.to_string()));
        };
        root.set_attribute("showMasterPhAnim", if show { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Whether master placeholder animations are shown (defaults true when absent).
    pub fn show_master_ph_anim(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&slide_info.uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("showMasterPhAnim")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Clear slide `@showMasterSp` attribute.
    pub fn clear_show_master_shapes(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&slide_info.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("showMasterSp").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "showMasterSp");
        self.package.set_part(
            slide_info.uri,
            content_type::PRESENTATION_SLIDE,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear slide `@showMasterPhAnim` attribute.
    pub fn clear_show_master_ph_anim(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&slide_info.uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("showMasterPhAnim").is_none() {
            return Ok(false);
        }
        root.attributes
            .retain(|a| a.local_name != "showMasterPhAnim");
        self.package.set_part(
            slide_info.uri,
            content_type::PRESENTATION_SLIDE,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Set whether master shapes are shown on a notes slide (`p:notes/@showMasterSp`).
    pub fn set_notes_show_master_shapes(&mut self, slide_index: usize, show: bool) -> Result<bool> {
        let Some(uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("showMasterSp", if show { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::PRESENTATION_NOTES_SLIDE, xml);
        Ok(true)
    }

    /// Whether notes show master shapes (defaults true when absent).
    pub fn notes_show_master_shapes(&self, slide_index: usize) -> Result<bool> {
        let Some(uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(true);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("showMasterSp")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set whether master shapes are shown on a layout (`p:sldLayout/@showMasterSp`).
    pub fn set_slide_layout_show_master_shapes(
        &mut self,
        layout_index: usize,
        show: bool,
    ) -> Result<()> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let mut root = if let Some(data) = self.package.opc().get_part(&layout.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(layout.uri.to_string()));
        };
        root.set_attribute("showMasterSp", if show { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(layout.uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
        Ok(())
    }

    /// Whether layout shows master shapes (defaults true when absent).
    pub fn slide_layout_show_master_shapes(&self, layout_index: usize) -> Result<bool> {
        let layout = self
            .layouts
            .get(layout_index)
            .ok_or_else(|| Error::Package(format!("layout index {layout_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("showMasterSp")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set whether master placeholder animations are shown on a layout.
    pub fn set_slide_layout_show_master_ph_anim(
        &mut self,
        layout_index: usize,
        show: bool,
    ) -> Result<()> {
        let layout =
            self.layouts.get(layout_index).cloned().ok_or_else(|| {
                Error::Package(format!("layout index {layout_index} out of range"))
            })?;
        let mut root = if let Some(data) = self.package.opc().get_part(&layout.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(layout.uri.to_string()));
        };
        root.set_attribute("showMasterPhAnim", if show { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(layout.uri, content_type::PRESENTATION_SLIDE_LAYOUT, xml);
        Ok(())
    }

    /// Whether layout shows master placeholder animations (defaults true when absent).
    pub fn slide_layout_show_master_ph_anim(&self, layout_index: usize) -> Result<bool> {
        let layout = self
            .layouts
            .get(layout_index)
            .ok_or_else(|| Error::Package(format!("layout index {layout_index} out of range")))?;
        let Some(data) = self.package.opc().get_part(&layout.uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("showMasterPhAnim")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Set whether master placeholder animations are shown on a notes slide.
    pub fn set_notes_show_master_ph_anim(
        &mut self,
        slide_index: usize,
        show: bool,
    ) -> Result<bool> {
        let Some(uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        root.set_attribute("showMasterPhAnim", if show { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(uri, content_type::PRESENTATION_NOTES_SLIDE, xml);
        Ok(true)
    }

    /// Whether notes show master placeholder animations (defaults true when absent).
    pub fn notes_show_master_ph_anim(&self, slide_index: usize) -> Result<bool> {
        let Some(uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(true);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(true);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("showMasterPhAnim")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true))
    }

    /// Clear notes `@showMasterSp` attribute.
    pub fn clear_notes_show_master_shapes(&mut self, slide_index: usize) -> Result<bool> {
        let Some(uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("showMasterSp").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "showMasterSp");
        self.package.set_part(
            uri,
            content_type::PRESENTATION_NOTES_SLIDE,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear notes `@showMasterPhAnim` attribute.
    pub fn clear_notes_show_master_ph_anim(&mut self, slide_index: usize) -> Result<bool> {
        let Some(uri) = self.notes_uri_for_slide(slide_index)? else {
            return Ok(false);
        };
        let Some(data) = self.package.opc().get_part(&uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        if root.get_attribute("showMasterPhAnim").is_none() {
            return Ok(false);
        }
        root.attributes
            .retain(|a| a.local_name != "showMasterPhAnim");
        self.package.set_part(
            uri,
            content_type::PRESENTATION_NOTES_SLIDE,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Count hidden slides.
    pub fn hidden_slide_count(&self) -> Result<usize> {
        Ok(self.list_hidden_slides()?.len())
    }

    /// Indices of hidden slides.
    pub fn list_hidden_slides(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.is_slide_hidden(i)? {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Indices of slides that have notes.
    /// Whether any slides are hidden.
    /// Unhide every hidden slide. Returns how many slides were unhidden.
    pub fn unhide_all_slides(&mut self) -> Result<usize> {
        let hidden = self.list_hidden_slides()?;
        for i in &hidden {
            self.set_slide_hidden(*i, false)?;
        }
        Ok(hidden.len())
    }

    /// Hide slides at the given indices. Returns how many were updated.
    pub fn hide_slides(&mut self, indices: &[usize]) -> Result<usize> {
        let mut n = 0usize;
        for &i in indices {
            if i < self.slides.len() {
                self.set_slide_hidden(i, true)?;
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn has_hidden_slides(&self) -> Result<bool> {
        Ok(!self.list_hidden_slides()?.is_empty())
    }

    /// Indices of slides that are not hidden.
    pub fn list_visible_slides(&self) -> Result<Vec<usize>> {
        let hidden: std::collections::HashSet<usize> =
            self.list_hidden_slides()?.into_iter().collect();
        Ok((0..self.slides.len())
            .filter(|i| !hidden.contains(i))
            .collect())
    }

    /// Whether any slides are visible.
    pub fn has_visible_slides(&self) -> Result<bool> {
        Ok(!self.list_visible_slides()?.is_empty())
    }

    pub fn slides_with_notes(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.has_notes(i)? {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Number of slides that currently have notes.
    /// Whether any slide has notes.
    pub fn has_any_notes(&self) -> Result<bool> {
        Ok(!self.slides_with_notes()?.is_empty())
    }

    /// Alias for [`has_any_notes`](Self::has_any_notes).
    pub fn has_slides_with_notes(&self) -> Result<bool> {
        self.has_any_notes()
    }

    pub fn notes_count(&self) -> Result<usize> {
        Ok(self.slides_with_notes()?.len())
    }

    /// List notes texts as `(slide_index, text)` for slides that have notes.
    pub fn list_notes_texts(&self) -> Result<Vec<(usize, String)>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if let Some(t) = self.notes_text(i)? {
                out.push((i, t));
            }
        }
        Ok(out)
    }

    /// Whether any slides currently have notes.
    pub fn has_notes_slides(&self) -> Result<bool> {
        Ok(self.notes_count()? > 0)
    }

    /// Remove notes from every slide that has them.
    pub fn clear_all_notes(&mut self) -> Result<usize> {
        let idxs = self.slides_with_notes()?;
        let mut n = 0;
        for i in idxs {
            if self.clear_notes(i)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Total shape count across all slides.
    pub fn total_shape_count(&self) -> Result<usize> {
        let mut n = 0;
        for i in 0..self.slides.len() {
            n += self.shape_count(i)?;
        }
        Ok(n)
    }

    /// Whether any slide has at least one shape.
    pub fn has_any_shape(&self) -> Result<bool> {
        Ok(self.total_shape_count()? > 0)
    }

    /// Indices of slides that have at least one shape.
    pub fn slides_with_shapes(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.shape_count(i)? > 0 {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Whether any slide has shapes.
    pub fn has_slides_with_shapes(&self) -> Result<bool> {
        Ok(!self.slides_with_shapes()?.is_empty())
    }

    /// Indices of slides that have no shapes.
    pub fn list_empty_slides(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.shape_count(i)? == 0 {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Whether any slide has no shapes.
    /// Remove all empty slides (no shapes). Returns how many were removed.
    ///
    /// Will not remove the last remaining slide.
    pub fn remove_empty_slides(&mut self) -> Result<usize> {
        let mut removed = 0usize;
        loop {
            if self.slides.len() <= 1 {
                break;
            }
            let empties = self.list_empty_slides()?;
            let Some(&idx) = empties.iter().rev().next() else {
                break;
            };
            // Prefer removing from the end to keep indices stable for remaining empties
            self.remove_slide(idx)?;
            removed += 1;
        }
        Ok(removed)
    }

    pub fn has_empty_slides(&self) -> Result<bool> {
        Ok(!self.list_empty_slides()?.is_empty())
    }

    /// Define presentation sections grouping slides by index ranges.
    ///
    /// Each entry is `(section_name, start_slide_index, end_slide_index_inclusive)`.
    pub fn set_sections(&mut self, sections_spec: &[(&str, usize, usize)]) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut section_els = Vec::new();
        for (i, (name, start, end)) in sections_spec.iter().enumerate() {
            let ids: Vec<u32> = self
                .slides
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx >= *start && *idx <= *end)
                .map(|(_, s)| s.id)
                .collect();
            let guid = format!(
                "{{{:08X}-0000-0000-0000-{:012X}}}",
                i as u32 + 1,
                i as u32 + 1
            );
            section_els.push(section(name, &guid, &ids));
        }
        let lst = section_list(section_els);
        let ext = section_list_ext(lst);

        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        if let Some(ext_lst) = root.child_mut("extLst") {
            ext_lst.children.retain(|c| {
                c.get_attribute("uri") != Some("{521415D9-36F7-43E2-AB2F-B90AF26B5E84}")
            });
            ext_lst.append_child(ext);
        } else {
            let lst = OpenXmlElement::new("p", crate::namespace::ns::PRESENTATIONML.uri, "extLst")
                .with_child(ext);
            root.append_child(lst);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Whether the presentation has section definitions (`p14:sectionLst`).
    pub fn has_sections(&self) -> Result<bool> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.descendants().any(|e| e.local_name == "sectionLst"))
    }

    /// List presentation sections as `(name, id)` pairs.
    pub fn list_sections(&self) -> Result<Vec<(String, String)>> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "section")
            .map(|s| {
                let name = s.get_attribute("name").unwrap_or("").to_string();
                let id = s.get_attribute("id").unwrap_or("").to_string();
                (name, id)
            })
            .collect())
    }

    /// Number of presentation sections.
    /// List section names only.
    pub fn list_section_names(&self) -> Result<Vec<String>> {
        Ok(self
            .list_sections()?
            .into_iter()
            .map(|(name, _)| name)
            .collect())
    }

    /// Whether a section with the given name exists.
    pub fn has_section_named(&self, name: &str) -> Result<bool> {
        Ok(self.list_section_names()?.iter().any(|n| n == name))
    }

    pub fn section_count(&self) -> Result<usize> {
        Ok(self.list_sections()?.len())
    }

    /// Rename a presentation section by current name. Returns whether found.
    pub fn rename_section(&mut self, old_name: &str, new_name: &str) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        fn rename(el: &mut OpenXmlElement, old: &str, new: &str) -> bool {
            if el.local_name == "section" && el.get_attribute("name") == Some(old) {
                el.set_attribute("name", new);
                return true;
            }
            for child in el.children.iter_mut() {
                if rename(child, old, new) {
                    return true;
                }
            }
            false
        }
        let found = rename(&mut root, old_name, new_name);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(found)
    }

    /// Remove a presentation section by name. Returns whether found.
    pub fn remove_section(&mut self, name: &str) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        fn remove(el: &mut OpenXmlElement, name: &str) -> bool {
            let before = el.children.len();
            el.children
                .retain(|c| !(c.local_name == "section" && c.get_attribute("name") == Some(name)));
            let mut found = el.children.len() < before;
            for child in el.children.iter_mut() {
                if remove(child, name) {
                    found = true;
                }
            }
            found
        }
        let found = remove(&mut root, name);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(found)
    }

    /// Remove all section definitions. Returns whether any were present.
    pub fn clear_sections(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let mut removed = false;
        if let Some(ext_lst) = root.child_mut("extLst") {
            let before = ext_lst.children.len();
            ext_lst.children.retain(|c| {
                c.get_attribute("uri") != Some("{521415D9-36F7-43E2-AB2F-B90AF26B5E84}")
            });
            removed = ext_lst.children.len() < before;
            if ext_lst.children.is_empty() {
                root.children.retain(|c| c.local_name != "extLst");
            }
        }
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Whether a slide has a transition element.
    pub fn has_transition(&self, slide_index: usize) -> Result<bool> {
        Ok(self.get_slide_transition(slide_index)?.is_some())
    }

    /// Indices of slides that currently have a transition.
    pub fn slides_with_transition(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.has_transition(i)? {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Indices of slides that currently have animation timing.
    pub fn slides_with_animation(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.has_animation(i)? {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Indices of slides that have a solid/custom background.
    /// Number of slides that have animation timing.
    pub fn slides_with_animation_count(&self) -> Result<usize> {
        Ok(self.slides_with_animation()?.len())
    }

    /// Whether any slide has animation timing.
    pub fn has_slides_with_animation(&self) -> Result<bool> {
        Ok(self.slides_with_animation_count()? > 0)
    }

    pub fn slides_with_background(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.has_slide_background(i)? {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Indices of slides that have classic comments.
    pub fn slides_with_comments(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.has_slide_comments(i)? {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Indices of slides with header/footer flags.
    /// Whether any slide has comments.
    pub fn has_slides_with_comments(&self) -> Result<bool> {
        Ok(!self.slides_with_comments()?.is_empty())
    }

    pub fn slides_with_header_footer(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if self.has_slide_header_footer(i)? {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// List transitions for all slides as `(index, effect, speed)` for slides that have one.
    pub fn list_slide_transitions(&self) -> Result<Vec<(usize, String, String)>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if let Some((effect, speed)) = self.get_slide_transition(i)? {
                out.push((i, effect, speed));
            }
        }
        Ok(out)
    }

    /// Whether any slides have transitions configured.
    pub fn has_slide_transitions(&self) -> Result<bool> {
        Ok(!self.list_slide_transitions()?.is_empty())
    }

    /// Read slide transition as `(effect, speed)` when present.
    pub fn get_slide_transition(&self, slide_index: usize) -> Result<Option<(String, String)>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        let Some(tr) = root.child("transition") else {
            return Ok(None);
        };
        let speed = tr.get_attribute("spd").unwrap_or("med").to_string();
        let effect = tr
            .children
            .first()
            .map(|c| c.local_name.clone())
            .unwrap_or_default();
        Ok(Some((effect, speed)))
    }

    /// Read full transition details as `(effect, speed, adv_click, adv_tm_ms?)`.
    pub fn transition_details(
        &self,
        slide_index: usize,
    ) -> Result<Option<(String, String, bool, Option<u32>)>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        let Some(tr) = root.child("transition") else {
            return Ok(None);
        };
        let speed = tr.get_attribute("spd").unwrap_or("med").to_string();
        let effect = tr
            .children
            .first()
            .map(|c| c.local_name.clone())
            .unwrap_or_default();
        let adv_click = tr
            .get_attribute("advClick")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let adv_tm = tr.get_attribute("advTm").and_then(|s| s.parse().ok());
        Ok(Some((effect, speed, adv_click, adv_tm)))
    }

    /// Update transition attributes without replacing the effect child.
    pub fn set_transition_attrs(
        &mut self,
        slide_index: usize,
        speed: Option<&str>,
        adv_click: Option<bool>,
        adv_tm_ms: Option<Option<u32>>,
    ) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let Some(tr) = root.child_mut("transition") else {
            return Ok(false);
        };
        if let Some(s) = speed {
            tr.set_attribute("spd", s);
        }
        if let Some(c) = adv_click {
            tr.set_attribute("advClick", if c { "1" } else { "0" });
        }
        if let Some(tm) = adv_tm_ms {
            match tm {
                Some(ms) => tr.set_attribute("advTm", ms.to_string()),
                None => {
                    tr.attributes.retain(|a| a.local_name != "advTm");
                }
            }
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(true)
    }

    /// Remove transition from a slide. Returns whether one was present.
    pub fn clear_transition(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "transition");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(removed)
    }

    /// Set notes size in EMUs on the presentation.
    pub fn set_notes_size(&mut self, cx: i64, cy: i64) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "notesSz");
        let sz = notes_size(cx, cy);
        if let Some(pos) = root.children.iter().position(|c| c.local_name == "sldSz") {
            root.children.insert(pos + 1, sz);
        } else {
            root.children.insert(0, sz);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Set presentation `firstSlideNum` attribute.
    pub fn set_first_slide_num(&mut self, num: u32) -> Result<()> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        root.set_attribute("firstSlideNum", num.to_string());
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Read `firstSlideNum` when present.
    pub fn first_slide_num(&self) -> Result<Option<u32>> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("firstSlideNum")
            .and_then(|s| s.parse().ok()))
    }

    /// Whether `firstSlideNum` is set on the presentation.
    pub fn has_first_slide_num(&self) -> Result<bool> {
        Ok(self.first_slide_num()?.is_some())
    }

    /// Remove `firstSlideNum` from the presentation. Returns whether it was present.
    pub fn clear_first_slide_num(&mut self) -> Result<bool> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri).map(|d| d.to_vec()) else {
            return Ok(false);
        };
        let mut root = parse_element(&data)?;
        if root.get_attribute("firstSlideNum").is_none() {
            return Ok(false);
        }
        root.attributes.retain(|a| a.local_name != "firstSlideNum");
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    /// Set presentation-level RTL (`rtl` attribute).
    pub fn set_rtl(&mut self, rtl: bool) -> Result<()> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        root.set_attribute("rtl", if rtl { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Whether presentation is RTL.
    pub fn rtl(&self) -> Result<bool> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute("rtl")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false))
    }

    fn set_presentation_bool_attr(&mut self, attr: &str, value: bool) -> Result<()> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        root.set_attribute(attr, if value { "1" } else { "0" });
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    fn presentation_bool_attr(&self, attr: &str, default: bool) -> Result<bool> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(default);
        };
        let root = parse_element(data)?;
        Ok(root
            .get_attribute(attr)
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default))
    }

    fn set_presentation_str_attr(&mut self, attr: &str, value: &str) -> Result<()> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        root.set_attribute(attr, value);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    fn presentation_str_attr(&self, attr: &str) -> Result<Option<String>> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root.get_attribute(attr).map(|s| s.to_string()))
    }

    fn clear_presentation_attr(&mut self, attr: &str) -> Result<bool> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.attributes.len();
        root.attributes.retain(|a| a.local_name != attr);
        let removed = root.attributes.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Set presentation `serverZoom` attribute.
    /// Alias for [`rtl`](Self::rtl).
    pub fn has_rtl(&self) -> Result<bool> {
        self.rtl()
    }

    /// Disable presentation-level RTL. Returns whether it was enabled.
    pub fn clear_rtl(&mut self) -> Result<bool> {
        let had = self.rtl()?;
        if had {
            self.set_rtl(false)?;
        }
        Ok(had)
    }

    pub fn set_server_zoom(&mut self, zoom: i32) -> Result<()> {
        self.set_presentation_str_attr("serverZoom", &zoom.to_string())
    }

    /// Read `serverZoom` when present.
    pub fn server_zoom(&self) -> Result<Option<i32>> {
        Ok(self
            .presentation_str_attr("serverZoom")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set `showSpecialPlsOnTitleSld` (show special placeholders on title slide).
    /// Whether server zoom is set.
    pub fn has_server_zoom(&self) -> Result<bool> {
        Ok(self.server_zoom()?.is_some())
    }

    pub fn set_show_special_pls_on_title_sld(&mut self, enabled: bool) -> Result<()> {
        self.set_presentation_bool_attr("showSpecialPlsOnTitleSld", enabled)
    }

    /// Whether special placeholders are shown on the title slide (default true).
    pub fn show_special_pls_on_title_sld(&self) -> Result<bool> {
        self.presentation_bool_attr("showSpecialPlsOnTitleSld", true)
    }

    /// Set `removePersonalInfoOnSave`.
    /// Disable showing special placeholders on the title slide. Returns prior effective value.
    pub fn clear_show_special_pls_on_title_sld(&mut self) -> Result<bool> {
        let had = self.show_special_pls_on_title_sld()?;
        self.set_show_special_pls_on_title_sld(false)?;
        Ok(had)
    }

    pub fn set_remove_personal_info_on_save(&mut self, enabled: bool) -> Result<()> {
        self.set_presentation_bool_attr("removePersonalInfoOnSave", enabled)
    }

    /// Whether removePersonalInfoOnSave is enabled.
    pub fn remove_personal_info_on_save(&self) -> Result<bool> {
        self.presentation_bool_attr("removePersonalInfoOnSave", false)
    }

    /// Set presentation `compatMode`.
    /// Disable `remove personal info on save`. Returns whether it was enabled.
    pub fn clear_remove_personal_info_on_save(&mut self) -> Result<bool> {
        let had = self.remove_personal_info_on_save()?;
        if had {
            self.set_remove_personal_info_on_save(false)?;
        }
        Ok(had)
    }

    pub fn set_compat_mode(&mut self, enabled: bool) -> Result<()> {
        self.set_presentation_bool_attr("compatMode", enabled)
    }

    /// Whether presentation compatMode is enabled.
    pub fn compat_mode(&self) -> Result<bool> {
        self.presentation_bool_attr("compatMode", false)
    }

    /// Set `strictFirstAndLastChars`.
    /// Disable `compat mode`. Returns whether it was enabled.
    pub fn clear_compat_mode(&mut self) -> Result<bool> {
        let had = self.compat_mode()?;
        if had {
            self.set_compat_mode(false)?;
        }
        Ok(had)
    }

    pub fn set_strict_first_and_last_chars(&mut self, enabled: bool) -> Result<()> {
        self.set_presentation_bool_attr("strictFirstAndLastChars", enabled)
    }

    /// Whether strictFirstAndLastChars is enabled.
    pub fn strict_first_and_last_chars(&self) -> Result<bool> {
        self.presentation_bool_attr("strictFirstAndLastChars", false)
    }

    /// Set `embedTrueTypeFonts`.
    /// Disable `strict first and last chars`. Returns whether it was enabled.
    pub fn clear_strict_first_and_last_chars(&mut self) -> Result<bool> {
        let had = self.strict_first_and_last_chars()?;
        if had {
            self.set_strict_first_and_last_chars(false)?;
        }
        Ok(had)
    }

    pub fn set_embed_true_type_fonts(&mut self, enabled: bool) -> Result<()> {
        self.set_presentation_bool_attr("embedTrueTypeFonts", enabled)
    }

    /// Whether embedTrueTypeFonts is enabled.
    pub fn embed_true_type_fonts(&self) -> Result<bool> {
        self.presentation_bool_attr("embedTrueTypeFonts", false)
    }

    /// Set `saveSubsetFonts`.
    /// Disable `embed true type fonts`. Returns whether it was enabled.
    pub fn clear_embed_true_type_fonts(&mut self) -> Result<bool> {
        let had = self.embed_true_type_fonts()?;
        if had {
            self.set_embed_true_type_fonts(false)?;
        }
        Ok(had)
    }

    pub fn set_save_subset_fonts(&mut self, enabled: bool) -> Result<()> {
        self.set_presentation_bool_attr("saveSubsetFonts", enabled)
    }

    /// Whether saveSubsetFonts is enabled.
    pub fn save_subset_fonts(&self) -> Result<bool> {
        self.presentation_bool_attr("saveSubsetFonts", false)
    }

    /// Set `autoCompressPictures`.
    /// Disable `save subset fonts`. Returns whether it was enabled.
    pub fn clear_save_subset_fonts(&mut self) -> Result<bool> {
        let had = self.save_subset_fonts()?;
        if had {
            self.set_save_subset_fonts(false)?;
        }
        Ok(had)
    }

    pub fn set_auto_compress_pictures(&mut self, enabled: bool) -> Result<()> {
        self.set_presentation_bool_attr("autoCompressPictures", enabled)
    }

    /// Whether autoCompressPictures is enabled (default true when unset).
    pub fn auto_compress_pictures(&self) -> Result<bool> {
        self.presentation_bool_attr("autoCompressPictures", true)
    }

    /// Set `bookmarkIdSeed`.
    /// Disable `auto compress pictures`. Returns whether it was enabled.
    pub fn clear_auto_compress_pictures(&mut self) -> Result<bool> {
        let had = self.auto_compress_pictures()?;
        if had {
            self.set_auto_compress_pictures(false)?;
        }
        Ok(had)
    }

    pub fn set_bookmark_id_seed(&mut self, seed: u32) -> Result<()> {
        self.set_presentation_str_attr("bookmarkIdSeed", &seed.to_string())
    }

    /// Read `bookmarkIdSeed` when present.
    pub fn bookmark_id_seed(&self) -> Result<Option<u32>> {
        Ok(self
            .presentation_str_attr("bookmarkIdSeed")?
            .and_then(|s| s.parse().ok()))
    }

    /// Set `conformance` (e.g. `"strict"` or `"transitional"`).
    /// Whether a bookmark id seed is set on the presentation.
    pub fn has_bookmark_id_seed(&self) -> Result<bool> {
        Ok(self.bookmark_id_seed()?.is_some())
    }

    pub fn set_conformance(&mut self, value: &str) -> Result<()> {
        self.set_presentation_str_attr("conformance", value)
    }

    /// Read `conformance` attribute.
    pub fn conformance(&self) -> Result<Option<String>> {
        self.presentation_str_attr("conformance")
    }

    /// Clear `serverZoom`. Returns whether present.
    pub fn clear_server_zoom(&mut self) -> Result<bool> {
        self.clear_presentation_attr("serverZoom")
    }

    /// Clear `bookmarkIdSeed`. Returns whether present.
    pub fn clear_bookmark_id_seed(&mut self) -> Result<bool> {
        self.clear_presentation_attr("bookmarkIdSeed")
    }

    /// Clear `conformance`. Returns whether present.
    pub fn clear_conformance(&mut self) -> Result<bool> {
        self.clear_presentation_attr("conformance")
    }

    /// Remove presentation `notesSz`. Returns whether it was present.
    pub fn clear_notes_size(&mut self) -> Result<bool> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "notesSz");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Set the presentation slide size in EMUs (e.g. [`SLIDE_SIZE_16_9`]).
    pub fn set_slide_size(&mut self, cx: i64, cy: i64) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&pres_uri) {
            parse_element(data)?
        } else {
            presentation(Vec::<OpenXmlElement>::new())
        };
        root.children.retain(|c| c.local_name != "sldSz");
        // sldSz typically comes before sldIdLst
        let sz = slide_size(cx, cy);
        if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "sldIdLst")
        {
            root.children.insert(pos, sz);
        } else {
            root.children.insert(0, sz);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Set slide size with optional type attribute (e.g. `"screen16x9"`, `"letter"`, `"a4"`).
    pub fn set_slide_size_ex(&mut self, cx: i64, cy: i64, size_type: Option<&str>) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = if let Some(data) = self.package.opc().get_part(&pres_uri) {
            parse_element(data)?
        } else {
            presentation(Vec::<OpenXmlElement>::new())
        };
        root.children.retain(|c| c.local_name != "sldSz");
        let mut sz = slide_size(cx, cy);
        if let Some(t) = size_type {
            sz.set_attribute("type", t);
        }
        if let Some(pos) = root
            .children
            .iter()
            .position(|c| c.local_name == "sldIdLst")
        {
            root.children.insert(pos, sz);
        } else {
            root.children.insert(0, sz);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Clear `sldSz/@type` when present (keeps cx/cy).
    pub fn clear_slide_size_type(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(sz) = root.child_mut("sldSz") else {
            return Ok(false);
        };
        if sz.get_attribute("type").is_none() {
            return Ok(false);
        }
        sz.attributes.retain(|a| a.local_name != "type");
        self.package.set_part(
            pres_uri,
            content_type::PRESENTATION,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Clear slide size type attribute (alias for [`clear_slide_size_type`](Self::clear_slide_size_type)).
    pub fn clear_slide_size_ex(&mut self) -> Result<bool> {
        self.clear_slide_size_type()
    }

    /// Read slide size type attribute when present.
    pub fn slide_size_type(&self) -> Result<Option<String>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("sldSz")
            .and_then(|s| s.get_attribute("type").map(|t| t.to_string())))
    }

    /// Ensure a minimal `defaultTextStyle` exists on the presentation.
    pub fn ensure_default_text_style(&mut self) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        if root.child("defaultTextStyle").is_some() {
            return Ok(());
        }
        let a = crate::namespace::ns::DRAWINGML.uri;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        // Minimal empty default text style list
        let dts = OpenXmlElement::new("p", p, "defaultTextStyle")
            .with_ns_decl("a", a)
            .with_child(OpenXmlElement::new("a", a, "defPPr"));
        root.children.push(dts);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Whether defaultTextStyle is present.
    pub fn has_default_text_style(&self) -> Result<bool> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(false),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("defaultTextStyle").is_some())
    }

    /// Clear defaultTextStyle. Returns whether present.
    pub fn clear_default_text_style(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "defaultTextStyle");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Read slide size `(cx, cy)` in EMUs, if present.
    pub fn slide_size(&self) -> Result<Option<(i64, i64)>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(sz) = root.child("sldSz") else {
            return Ok(None);
        };
        let cx = sz
            .get_attribute("cx")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let cy = sz
            .get_attribute("cy")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        Ok(Some((cx, cy)))
    }

    /// Read notes size in EMUs from the presentation, if set.
    pub fn notes_size(&self) -> Result<Option<(i64, i64)>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(sz) = root.child("notesSz") else {
            return Ok(None);
        };
        let cx = sz
            .get_attribute("cx")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let cy = sz
            .get_attribute("cy")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        Ok(Some((cx, cy)))
    }

    /// Whether slide size is set on the presentation.
    pub fn has_slide_size(&self) -> Result<bool> {
        Ok(self.slide_size()?.is_some())
    }

    /// Remove presentation `sldSz`. Returns whether it was present.
    pub fn clear_slide_size(&mut self) -> Result<bool> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "sldSz");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Whether notes size is set on the presentation.
    pub fn has_notes_size(&self) -> Result<bool> {
        Ok(self.notes_size()?.is_some())
    }

    /// Whether a slide has a solid background fill (`p:bg`).
    pub fn has_slide_background(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root
            .child("cSld")
            .map(|c| c.child("bg").is_some())
            .unwrap_or(false))
    }

    /// Read solid slide background RGB if present (from `a:srgbClr`).
    pub fn slide_background_rgb(&self, slide_index: usize) -> Result<Option<String>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        let Some(csld) = root.child("cSld") else {
            return Ok(None);
        };
        let Some(bg) = csld.child("bg") else {
            return Ok(None);
        };
        Ok(bg
            .descendants()
            .find(|e| e.local_name == "srgbClr")
            .and_then(|e| e.get_attribute("val").map(|s| s.to_string())))
    }

    /// Set a solid RGB background color on a slide.
    pub fn set_slide_background(&mut self, slide_index: usize, rgb: &str) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        if let Some(csld) = root.child_mut("cSld") {
            csld.children.retain(|c| c.local_name != "bg");
            // bg is first child of cSld
            csld.children.insert(0, solid_slide_background(rgb));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Remove a slide's background. Returns whether one was present.
    pub fn clear_slide_background(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let mut removed = false;
        if let Some(csld) = root.child_mut("cSld") {
            let before = csld.children.len();
            csld.children.retain(|c| c.local_name != "bg");
            removed = csld.children.len() < before;
        }
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(removed)
    }

    /// Set the common slide data name (`p:cSld/@name`).
    pub fn set_slide_name(&mut self, slide_index: usize, name: &str) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        if let Some(csld) = root.child_mut("cSld") {
            csld.set_attribute("name", name);
        } else {
            return Err(Error::Package("cSld missing".into()));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Read slide common data name.
    pub fn slide_name(&self, slide_index: usize) -> Result<Option<String>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root
            .child("cSld")
            .and_then(|c| c.get_attribute("name").map(|s| s.to_string())))
    }

    /// List slide names as `(index, name)` for slides that have `cSld/@name`.
    pub fn list_slide_names(&self) -> Result<Vec<(usize, String)>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if let Some(n) = self.slide_name(i)? {
                if !n.is_empty() {
                    out.push((i, n));
                }
            }
        }
        Ok(out)
    }

    /// Whether any slides have non-empty names.
    pub fn has_slide_names(&self) -> Result<bool> {
        Ok(!self.list_slide_names()?.is_empty())
    }

    /// Clear a slide name. Returns whether one was present.
    pub fn clear_slide_name(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let Some(csld) = root.child_mut("cSld") else {
            return Ok(false);
        };
        let before = csld.attributes.len();
        csld.attributes.retain(|a| a.local_name != "name");
        let removed = csld.attributes.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(removed)
    }

    /// Whether any slide has the given name (`cSld/@name`).
    /// Clear names on every slide. Returns how many names were cleared.
    pub fn clear_all_slide_names(&mut self) -> Result<usize> {
        let mut n = 0usize;
        for i in 0..self.slides.len() {
            if self.clear_slide_name(i)? {
                n += 1;
            }
        }
        Ok(n)
    }

    pub fn has_slide_named(&self, name: &str) -> Result<bool> {
        Ok(self.list_slide_names()?.iter().any(|(_, n)| n == name))
    }

    /// Whether any slide currently has a background.
    pub fn has_any_background(&self) -> Result<bool> {
        Ok(self.background_count()? > 0)
    }

    /// Remove backgrounds from all slides that have them.
    pub fn clear_all_backgrounds(&mut self) -> Result<usize> {
        let idxs = self.slides_with_background()?;
        let mut n = 0;
        for i in idxs {
            if self.clear_slide_background(i)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Alias for [`clear_slide_background`](Self::clear_slide_background).
    pub fn clear_background(&mut self, slide_index: usize) -> Result<bool> {
        self.clear_slide_background(slide_index)
    }

    /// Configure header/footer visibility on a slide (`p:hf`).
    pub fn set_slide_header_footer(
        &mut self,
        slide_index: usize,
        show_date: bool,
        show_footer: bool,
        show_slide_number: bool,
    ) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        // cSld sibling - hf is a direct child of sld
        root.children.retain(|c| c.local_name != "hf");
        root.append_child(header_footer(show_date, show_footer, show_slide_number));
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Whether a slide has header/footer flags (`p:hf`).
    pub fn has_slide_header_footer(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root.child("hf").is_some())
    }

    /// Read slide header/footer flags as `(show_date, show_footer, show_slide_number, show_header)`.
    pub fn slide_header_footer(
        &self,
        slide_index: usize,
    ) -> Result<Option<(bool, bool, bool, bool)>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        let Some(hf) = root.child("hf") else {
            return Ok(None);
        };
        let on = |name: &str, default: bool| {
            hf.get_attribute(name)
                .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
                .unwrap_or(default)
        };
        Ok(Some((
            on("dt", true),
            on("ftr", true),
            on("sldNum", true),
            on("hdr", true),
        )))
    }

    /// Set photo album properties on the presentation (`p:photoAlbum`).
    pub fn set_photo_album(
        &mut self,
        black_and_white: bool,
        show_captions: bool,
        layout: &str,
        frame: &str,
    ) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.children.retain(|c| c.local_name != "photoAlbum");
        let album = OpenXmlElement::new("p", p, "photoAlbum")
            .with_attribute("bw", if black_and_white { "1" } else { "0" })
            .with_attribute("showCaptions", if show_captions { "1" } else { "0" })
            .with_attribute("layout", layout)
            .with_attribute("frame", frame);
        // typically after sldIdLst
        let insert_at = root
            .children
            .iter()
            .position(|c| c.local_name == "sldIdLst")
            .map(|i| i + 1)
            .unwrap_or(root.children.len());
        root.children.insert(insert_at, album);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Read photo album as `(bw, show_captions, layout, frame)`.
    pub fn photo_album(&self) -> Result<Option<(bool, bool, String, String)>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(pa) = root.child("photoAlbum") else {
            return Ok(None);
        };
        let bw = pa
            .get_attribute("bw")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let caps = pa
            .get_attribute("showCaptions")
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let layout = pa
            .get_attribute("layout")
            .unwrap_or("fitToSlide")
            .to_string();
        let frame = pa.get_attribute("frame").unwrap_or("rectangle").to_string();
        Ok(Some((bw, caps, layout, frame)))
    }

    /// Whether photoAlbum is present.
    pub fn has_photo_album(&self) -> Result<bool> {
        Ok(self.photo_album()?.is_some())
    }

    /// Clear photoAlbum. Returns whether present.
    pub fn clear_photo_album(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "photoAlbum");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Update individual photo album attributes without replacing the element.
    pub fn set_photo_album_attrs(
        &mut self,
        black_and_white: Option<bool>,
        show_captions: Option<bool>,
        layout: Option<&str>,
        frame: Option<&str>,
    ) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let Some(pa) = root.child_mut("photoAlbum") else {
            return Ok(false);
        };
        if let Some(bw) = black_and_white {
            pa.set_attribute("bw", if bw { "1" } else { "0" });
        }
        if let Some(sc) = show_captions {
            pa.set_attribute("showCaptions", if sc { "1" } else { "0" });
        }
        if let Some(l) = layout {
            pa.set_attribute("layout", l);
        }
        if let Some(f) = frame {
            pa.set_attribute("frame", f);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    /// Clear optional photoAlbum attributes (bw/showCaptions/layout/frame).
    pub fn clear_photo_album_attrs(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(pa) = root.child_mut("photoAlbum") else {
            return Ok(false);
        };
        let before = pa.attributes.len();
        pa.attributes.retain(|a| {
            !matches!(
                a.local_name.as_str(),
                "bw" | "showCaptions" | "layout" | "frame"
            )
        });
        if pa.attributes.len() == before {
            return Ok(false);
        }
        self.package.set_part(
            pres_uri,
            content_type::PRESENTATION,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Update kinsoku attributes in place.
    pub fn set_kinsoku_attrs(
        &mut self,
        lang: Option<&str>,
        inval_start_chars: Option<&str>,
        inval_end_chars: Option<&str>,
    ) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let Some(k) = root.child_mut("kinsoku") else {
            return Ok(false);
        };
        if let Some(l) = lang {
            k.set_attribute("lang", l);
        }
        if let Some(s) = inval_start_chars {
            k.set_attribute("invalStChars", s);
        }
        if let Some(e) = inval_end_chars {
            k.set_attribute("invalEndChars", e);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    /// Clear kinsoku attributes (lang/invalStChars/invalEndChars).
    pub fn clear_kinsoku_attrs(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(k) = root.child_mut("kinsoku") else {
            return Ok(false);
        };
        let before = k.attributes.len();
        k.attributes.retain(|a| {
            !matches!(
                a.local_name.as_str(),
                "lang" | "invalStChars" | "invalEndChars"
            )
        });
        if k.attributes.len() == before {
            return Ok(false);
        }
        self.package.set_part(
            pres_uri,
            content_type::PRESENTATION,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Remove kinsoku element entirely.
    pub fn clear_kinsoku(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "kinsoku");
        if root.children.len() == before {
            return Ok(false);
        }
        self.package.set_part(
            pres_uri,
            content_type::PRESENTATION,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Set kinsoku (East Asian line-break) settings on the presentation.
    pub fn set_kinsoku(
        &mut self,
        lang: &str,
        inval_start_chars: &str,
        inval_end_chars: &str,
    ) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.children.retain(|c| c.local_name != "kinsoku");
        let k = OpenXmlElement::new("p", p, "kinsoku")
            .with_attribute("lang", lang)
            .with_attribute("invalStChars", inval_start_chars)
            .with_attribute("invalEndChars", inval_end_chars);
        root.children.push(k);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Read kinsoku as `(lang, inval_start, inval_end)`.
    pub fn kinsoku(&self) -> Result<Option<(String, String, String)>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        let Some(k) = root.child("kinsoku") else {
            return Ok(None);
        };
        Ok(Some((
            k.get_attribute("lang").unwrap_or("").to_string(),
            k.get_attribute("invalStChars").unwrap_or("").to_string(),
            k.get_attribute("invalEndChars").unwrap_or("").to_string(),
        )))
    }

    /// Whether kinsoku is present.
    pub fn has_kinsoku(&self) -> Result<bool> {
        Ok(self.kinsoku()?.is_some())
    }

    /// Set modification verifier shell (`p:modifyVerifier`) without real crypto hashing.
    ///
    /// Stores placeholder algorithm metadata for package shape compatibility.
    pub fn set_modify_verifier(&mut self, algorithm_name: &str, spin_count: u32) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        root.children.retain(|c| c.local_name != "modifyVerifier");
        let mv = OpenXmlElement::new("p", p, "modifyVerifier")
            .with_attribute("algorithmName", algorithm_name)
            .with_attribute("spinCount", spin_count.to_string())
            .with_attribute("hashValue", "")
            .with_attribute("saltValue", "");
        root.children.push(mv);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// Update modifyVerifier attributes in place.
    pub fn set_modify_verifier_attrs(
        &mut self,
        algorithm_name: Option<&str>,
        spin_count: Option<u32>,
    ) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let Some(mv) = root.child_mut("modifyVerifier") else {
            return Ok(false);
        };
        if let Some(a) = algorithm_name {
            mv.set_attribute("algorithmName", a);
        }
        if let Some(s) = spin_count {
            mv.set_attribute("spinCount", s.to_string());
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(true)
    }

    /// Clear modifyVerifier algorithmName/spinCount attributes.
    pub fn clear_modify_verifier_attrs(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(mv) = root.child_mut("modifyVerifier") else {
            return Ok(false);
        };
        let before = mv.attributes.len();
        mv.attributes
            .retain(|a| a.local_name != "algorithmName" && a.local_name != "spinCount");
        if mv.attributes.len() == before {
            return Ok(false);
        }
        self.package.set_part(
            pres_uri,
            content_type::PRESENTATION,
            write_element(&root)?,
        );
        Ok(true)
    }

    /// Whether modifyVerifier is present.
    pub fn has_modify_verifier(&self) -> Result<bool> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(false),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let root = parse_element(data)?;
        Ok(root.child("modifyVerifier").is_some())
    }

    /// Read modifyVerifier algorithm name when present.
    pub fn modify_verifier_algorithm(&self) -> Result<Option<String>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("modifyVerifier")
            .and_then(|m| m.get_attribute("algorithmName").map(|s| s.to_string())))
    }

    /// Read modifyVerifier spinCount when present.
    pub fn modify_verifier_spin_count(&self) -> Result<Option<u32>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(None);
        };
        let root = parse_element(data)?;
        Ok(root
            .child("modifyVerifier")
            .and_then(|m| m.get_attribute("spinCount"))
            .and_then(|s| s.parse().ok()))
    }

    /// Clear modifyVerifier. Returns whether present.
    pub fn clear_modify_verifier(&mut self) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "modifyVerifier");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Add an embedded font list entry shell (`p:embeddedFontLst/p:embeddedFont`).
    ///
    /// `typeface` is the font name; relationship ids for font data parts are optional.
    /// When an entry for the same typeface already exists, the provided face slots are merged.
    pub fn add_embedded_font(&mut self, typeface: &str, regular_rid: Option<&str>) -> Result<()> {
        self.add_embedded_font_faces(typeface, regular_rid, None)
    }

    /// Add/merge an embedded font entry with optional regular and bold relationship ids.
    pub fn add_embedded_font_faces(
        &mut self,
        typeface: &str,
        regular_rid: Option<&str>,
        bold_rid: Option<&str>,
    ) -> Result<()> {
        self.add_embedded_font_faces_ex(typeface, regular_rid, bold_rid, None, None)
    }

    /// Add/merge embedded font entry with optional `charset` / `pitchFamily` on `p:font`.
    pub fn add_embedded_font_faces_ex(
        &mut self,
        typeface: &str,
        regular_rid: Option<&str>,
        bold_rid: Option<&str>,
        charset: Option<i32>,
        pitch_family: Option<i32>,
    ) -> Result<()> {
        self.add_embedded_font_faces_full(
            typeface,
            regular_rid,
            bold_rid,
            charset,
            pitch_family,
            None,
        )
    }

    /// Like [`add_embedded_font_faces_ex`], with optional panose hex (10 bytes → 20 hex chars).
    pub fn add_embedded_font_faces_full(
        &mut self,
        typeface: &str,
        regular_rid: Option<&str>,
        bold_rid: Option<&str>,
        charset: Option<i32>,
        pitch_family: Option<i32>,
        panose: Option<&str>,
    ) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let a = crate::namespace::ns::DRAWINGML.uri;
        let panose_val = panose
            .filter(|s| s.len() == 20)
            .unwrap_or("020B0604020202020204")
            .to_string();

        let make_font_el = |typeface: &str| {
            let mut font = OpenXmlElement::new("p", p, "font")
                .with_attribute("typeface", typeface)
                .with_ns_decl("a", a);
            // DrawingML CT_TextFont attrs (optional)
            if let Some(cs) = charset {
                font = font.with_attribute("charset", cs.to_string());
            }
            if let Some(pf) = pitch_family {
                font = font.with_attribute("pitchFamily", pf.to_string());
            }
            font = font.with_attribute("panose", &panose_val);
            font
        };

        // Find existing entry for typeface
        let mut found = false;
        if let Some(lst) = root.child_mut("embeddedFontLst") {
            for entry in lst.children.iter_mut() {
                if entry.local_name != "embeddedFont" {
                    continue;
                }
                let same = entry
                    .child("font")
                    .and_then(|f| f.get_attribute("typeface"))
                    .map(|t| t == typeface)
                    .unwrap_or(false);
                if !same {
                    continue;
                }
                found = true;
                if let Some(rid) = regular_rid {
                    if entry.child("regular").is_none() {
                        entry.append_child(
                            OpenXmlElement::new("p", p, "regular")
                                .with_attribute_qname("r:id", rid),
                        );
                    }
                }
                if let Some(rid) = bold_rid {
                    if entry.child("bold").is_none() {
                        entry.append_child(
                            OpenXmlElement::new("p", p, "bold").with_attribute_qname("r:id", rid),
                        );
                    }
                }
                break;
            }
            if !found {
                let mut entry =
                    OpenXmlElement::new("p", p, "embeddedFont").with_child(make_font_el(typeface));
                if let Some(rid) = regular_rid {
                    entry.append_child(
                        OpenXmlElement::new("p", p, "regular").with_attribute_qname("r:id", rid),
                    );
                }
                if let Some(rid) = bold_rid {
                    entry.append_child(
                        OpenXmlElement::new("p", p, "bold").with_attribute_qname("r:id", rid),
                    );
                }
                lst.append_child(entry);
            }
        } else {
            let mut entry =
                OpenXmlElement::new("p", p, "embeddedFont").with_child(make_font_el(typeface));
            if let Some(rid) = regular_rid {
                entry.append_child(
                    OpenXmlElement::new("p", p, "regular").with_attribute_qname("r:id", rid),
                );
            }
            if let Some(rid) = bold_rid {
                entry.append_child(
                    OpenXmlElement::new("p", p, "bold").with_attribute_qname("r:id", rid),
                );
            }
            let lst = OpenXmlElement::new("p", p, "embeddedFontLst").with_child(entry);
            // ECMA-376 CT_Presentation order: ... notesSz, embeddedFontLst, ...
            // defaultTextStyle. Appending after defaultTextStyle makes MS PowerPoint
            // refuse to open the package ("cannot read").
            let insert_at = presentation_embedded_font_lst_insert_at(&root);
            root.children.insert(insert_at, lst);
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// List embedded font typefaces.
    pub fn list_embedded_fonts(&self) -> Result<Vec<String>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(lst) = root.child("embeddedFontLst") else {
            return Ok(Vec::new());
        };
        Ok(lst
            .children_by_name("embeddedFont")
            .filter_map(|e| {
                e.child("font")
                    .and_then(|f| f.get_attribute("typeface").map(|s| s.to_string()))
            })
            .collect())
    }

    /// Number of embedded font entries.
    pub fn embedded_font_count(&self) -> Result<usize> {
        Ok(self.list_embedded_fonts()?.len())
    }

    /// Whether embedded fonts are listed.
    pub fn has_embedded_fonts(&self) -> Result<bool> {
        Ok(self.embedded_font_count()? > 0)
    }

    /// Clear embedded font list. Returns how many were removed.
    pub fn clear_embedded_fonts(&mut self) -> Result<usize> {
        let n = self.embedded_font_count()?;
        if n == 0 {
            return Ok(0);
        }
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        root.children.retain(|c| c.local_name != "embeddedFontLst");
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(n)
    }

    /// Remove an embedded font entry by typeface. Returns whether found.
    pub fn remove_embedded_font(&mut self, typeface: &str) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(lst) = root.child_mut("embeddedFontLst") else {
            return Ok(false);
        };
        let before = lst.children.len();
        lst.children.retain(|e| {
            !(e.local_name == "embeddedFont"
                && e.child("font").and_then(|f| f.get_attribute("typeface")) == Some(typeface))
        });
        let removed = lst.children.len() < before;
        if lst.children.is_empty() {
            root.children.retain(|c| c.local_name != "embeddedFontLst");
        }
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Add a customer data reference shell on the presentation (`p:custDataLst/p:custData`).
    pub fn add_customer_data(&mut self, relationship_id: &str) -> Result<()> {
        let pres_uri = self.ensure_presentation()?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&pres_uri)
                .ok_or_else(|| Error::PartNotFound(pres_uri.to_string()))?,
        )?;
        let p = crate::namespace::ns::PRESENTATIONML.uri;
        let entry =
            OpenXmlElement::new("p", p, "custData").with_attribute_qname("r:id", relationship_id);
        if let Some(lst) = root.child_mut("custDataLst") {
            lst.append_child(entry);
        } else {
            root.children
                .push(OpenXmlElement::new("p", p, "custDataLst").with_child(entry));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(())
    }

    /// List customer data relationship ids.
    pub fn list_customer_data(&self) -> Result<Vec<String>> {
        let pres_uri = match self.package.opc().main_part_uri(rel::OFFICE_DOCUMENT) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(Vec::new());
        };
        let root = parse_element(data)?;
        let Some(lst) = root.child("custDataLst") else {
            return Ok(Vec::new());
        };
        Ok(lst
            .children_by_name("custData")
            .filter_map(|c| {
                c.get_attribute_qname("r:id")
                    .or_else(|| c.get_attribute("id"))
                    .map(|s| s.to_string())
            })
            .collect())
    }

    /// Number of customer data entries.
    pub fn customer_data_count(&self) -> Result<usize> {
        Ok(self.list_customer_data()?.len())
    }

    /// Whether customer data exists.
    pub fn has_customer_data(&self) -> Result<bool> {
        Ok(self.customer_data_count()? > 0)
    }

    /// Clear customer data list. Returns how many were removed.
    pub fn clear_customer_data(&mut self) -> Result<usize> {
        let n = self.customer_data_count()?;
        if n == 0 {
            return Ok(0);
        }
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(0);
        };
        let mut root = parse_element(data)?;
        root.children.retain(|c| c.local_name != "custDataLst");
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(pres_uri, self.document_type.content_type(), xml);
        Ok(n)
    }

    /// Remove a customer data entry by relationship id. Returns whether found.
    pub fn remove_customer_data(&mut self, relationship_id: &str) -> Result<bool> {
        let pres_uri = self.ensure_presentation()?;
        let Some(data) = self.package.opc().get_part(&pres_uri) else {
            return Ok(false);
        };
        let mut root = parse_element(data)?;
        let Some(lst) = root.child_mut("custDataLst") else {
            return Ok(false);
        };
        let before = lst.children.len();
        lst.children.retain(|c| {
            let rid = c
                .get_attribute_qname("r:id")
                .or_else(|| c.get_attribute("id"));
            rid != Some(relationship_id)
        });
        let removed = lst.children.len() < before;
        if lst.children.is_empty() {
            root.children.retain(|c| c.local_name != "custDataLst");
        }
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(pres_uri, self.document_type.content_type(), xml);
        }
        Ok(removed)
    }

    /// Whether any slide has header/footer flags.
    pub fn has_any_header_footer(&self) -> Result<bool> {
        Ok(self.header_footer_count()? > 0)
    }

    /// Remove `p:hf` from all slides that have it.
    pub fn clear_all_header_footers(&mut self) -> Result<usize> {
        let idxs = self.slides_with_header_footer()?;
        let mut n = 0;
        for i in idxs {
            if self.clear_slide_header_footer(i)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Remove slide header/footer flags (`p:hf`). Returns whether one was present.
    pub fn clear_slide_header_footer(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "hf");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(removed)
    }

    /// Whether a slide has a timing/animation tree (`p:timing`).
    pub fn has_animation(&self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root.child("timing").is_some())
    }

    /// Number of slides that have a transition.
    pub fn transition_count(&self) -> Result<usize> {
        Ok(self.slides_with_transition()?.len())
    }

    /// Whether any slide has a transition.
    pub fn has_any_transition(&self) -> Result<bool> {
        Ok(self.transition_count()? > 0)
    }

    /// Alias for [`has_any_transition`](Self::has_any_transition).
    pub fn has_slides_with_transition(&self) -> Result<bool> {
        self.has_any_transition()
    }

    /// Whether any slide has an animation timeline.
    pub fn has_any_animation(&self) -> Result<bool> {
        Ok(self.animation_count()? > 0)
    }

    /// Remove transitions from all slides that have them.
    pub fn clear_all_transitions(&mut self) -> Result<usize> {
        let idxs = self.slides_with_transition()?;
        let mut n = 0;
        for i in idxs {
            if self.clear_transition(i)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Remove animations from all slides that have them.
    pub fn clear_all_animations(&mut self) -> Result<usize> {
        let idxs = self.slides_with_animation()?;
        let mut n = 0;
        for i in idxs {
            if self.clear_animation(i)? {
                n += 1;
            }
        }
        Ok(n)
    }

    /// Number of slides that have animation timing.
    pub fn animation_count(&self) -> Result<usize> {
        Ok(self.slides_with_animation()?.len())
    }

    /// Remove animation timing from a slide. Returns whether it was present.
    pub fn clear_animation(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let before = root.children.len();
        root.children.retain(|c| c.local_name != "timing");
        let removed = root.children.len() < before;
        if removed {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(removed)
    }

    /// List animation target shape ids on a slide (`spTgt/@spid`).
    pub fn list_animation_shape_ids(&self, slide_index: usize) -> Result<Vec<u32>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root
            .descendants()
            .filter(|e| e.local_name == "spTgt")
            .filter_map(|e| e.get_attribute("spid").and_then(|s| s.parse().ok()))
            .collect())
    }

    /// Attach a simple appear/fade animation timing targeting a shape id.
    /// Whether a slide's timing targets any shapes.
    pub fn has_animation_shape_ids(&self, slide_index: usize) -> Result<bool> {
        Ok(!self.list_animation_shape_ids(slide_index)?.is_empty())
    }

    /// Count animation target shape ids on a slide.
    pub fn animation_shape_id_count(&self, slide_index: usize) -> Result<usize> {
        Ok(self.list_animation_shape_ids(slide_index)?.len())
    }

    pub fn set_simple_appear_animation(&mut self, slide_index: usize, shape_id: u32) -> Result<()> {
        self.set_animation_effect(slide_index, shape_id, "fade", "in")
    }

    /// Attach an `animEffect` timing targeting a shape id.
    ///
    /// `filter` e.g. `"fade"`, `"blinds(horizontal)"`, `"box(in)"`, `"checkerboard(across)"`.
    /// `transition` is typically `"in"` or `"out"`.
    pub fn set_animation_effect(
        &mut self,
        slide_index: usize,
        shape_id: u32,
        filter: &str,
        transition: &str,
    ) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "timing");
        // Build timing with custom filter/transition (same structure as simple_appear_timing).
        let p_ns = "http://schemas.openxmlformats.org/presentationml/2006/main";
        let sp_tgt =
            OpenXmlElement::new("p", p_ns, "spTgt").with_attribute("spid", shape_id.to_string());
        let tgt_el = OpenXmlElement::new("p", p_ns, "tgtEl").with_child(sp_tgt);
        let c_bhvr = OpenXmlElement::new("p", p_ns, "cBhvr").with_child(
            OpenXmlElement::new("p", p_ns, "cTn")
                .with_attribute("id", "2")
                .with_attribute("dur", "1")
                .with_attribute("fill", "hold")
                .with_child(tgt_el),
        );
        let anim = OpenXmlElement::new("p", p_ns, "animEffect")
            .with_attribute("transition", transition)
            .with_attribute("filter", filter)
            .with_child(c_bhvr);
        let child_tn = OpenXmlElement::new("p", p_ns, "cTn")
            .with_attribute("id", "1")
            .with_attribute("dur", "indefinite")
            .with_attribute("restart", "never")
            .with_attribute("nodeType", "clickEffect")
            .with_child(OpenXmlElement::new("p", p_ns, "childTnLst").with_child(anim));
        let par = OpenXmlElement::new("p", p_ns, "par").with_child(child_tn);
        let timing = OpenXmlElement::new("p", p_ns, "timing")
            .with_child(OpenXmlElement::new("p", p_ns, "tnLst").with_child(par));
        root.append_child(timing);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// List animation effects across all slides as `(slide_index, filter, transition)`.
    pub fn list_animation_effects(&self) -> Result<Vec<(usize, String, String)>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if let Some((f, t)) = self.animation_effect(i)? {
                out.push((i, f, t));
            }
        }
        Ok(out)
    }

    /// Whether any slides have animation effects configured.
    pub fn has_animation_effects(&self) -> Result<bool> {
        Ok(!self.list_animation_effects()?.is_empty())
    }

    /// Indices of slides that have at least one animation effect.
    pub fn slides_with_animation_effects(&self) -> Result<Vec<usize>> {
        Ok(self
            .list_animation_effects()?
            .into_iter()
            .map(|(i, _, _)| i)
            .collect())
    }

    /// Read `(filter, transition)` from the first `animEffect` under slide timing, if any.
    pub fn animation_effect(&self, slide_index: usize) -> Result<Option<(String, String)>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        for e in root.descendants() {
            if e.local_name == "animEffect" {
                let filter = e.get_attribute("filter").unwrap_or("fade").to_string();
                let transition = e.get_attribute("transition").unwrap_or("in").to_string();
                return Ok(Some((filter, transition)));
            }
        }
        Ok(None)
    }

    /// Whether the slide has an animEffect timing entry.
    pub fn has_animation_effect(&self, slide_index: usize) -> Result<bool> {
        Ok(self.animation_effect(slide_index)?.is_some())
    }

    /// Read duration (ms string or `"indefinite"`) from the first animEffect `cBhvr/cTn@dur`.
    pub fn animation_duration(&self, slide_index: usize) -> Result<Option<String>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        for e in root.descendants() {
            if e.local_name != "animEffect" {
                continue;
            }
            if let Some(cb) = e.child("cBhvr") {
                if let Some(ctn) = cb.child("cTn") {
                    return Ok(ctn.get_attribute("dur").map(|s| s.to_string()));
                }
            }
        }
        Ok(None)
    }

    /// Whether animation duration is set on the slide's first animEffect.
    pub fn has_animation_duration(&self, slide_index: usize) -> Result<bool> {
        Ok(self.animation_duration(slide_index)?.is_some())
    }

    /// Set duration on the first animEffect `cBhvr/cTn@dur` (e.g. `"500"` ms or `"indefinite"`).
    pub fn set_animation_duration(&mut self, slide_index: usize, dur: &str) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, dur: &str, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "animEffect" {
                if let Some(cb) = el.child_mut("cBhvr") {
                    if let Some(ctn) = cb.child_mut("cTn") {
                        ctn.set_attribute("dur", dur);
                        *found = true;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, dur, found);
            }
        }
        visit(&mut root, dur, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear duration attribute on the first animEffect (removes `@dur`).
    pub fn clear_animation_duration(&mut self, slide_index: usize) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let mut found = false;
        fn visit(el: &mut OpenXmlElement, found: &mut bool) {
            if *found {
                return;
            }
            if el.local_name == "animEffect" {
                if let Some(cb) = el.child_mut("cBhvr") {
                    if let Some(ctn) = cb.child_mut("cTn") {
                        let before = ctn.attributes.len();
                        ctn.attributes.retain(|a| a.local_name != "dur");
                        *found = ctn.attributes.len() < before;
                        return;
                    }
                }
            }
            for c in el.children.iter_mut() {
                visit(c, found);
            }
        }
        visit(&mut root, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Update filter/transition on the first animEffect without rebuilding timing.
    pub fn set_animation_filter(
        &mut self,
        slide_index: usize,
        filter: Option<&str>,
        transition: Option<&str>,
    ) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let mut found = false;
        fn visit(
            el: &mut OpenXmlElement,
            filter: Option<&str>,
            transition: Option<&str>,
            found: &mut bool,
        ) {
            if *found {
                return;
            }
            if el.local_name == "animEffect" {
                if let Some(f) = filter {
                    el.set_attribute("filter", f);
                }
                if let Some(t) = transition {
                    el.set_attribute("transition", t);
                }
                *found = true;
                return;
            }
            for c in el.children.iter_mut() {
                visit(c, filter, transition, found);
            }
        }
        visit(&mut root, filter, transition, &mut found);
        if found {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(found)
    }

    /// Clear animation filter/transition attributes on the first animEffect.
    pub fn clear_animation_filter(&mut self, slide_index: usize) -> Result<bool> {
        self.set_animation_filter(slide_index, None, None)
    }

    /// List animation effects on a slide as `(shape_id, filter, transition)`.
    pub fn list_slide_animation_effects(
        &self,
        slide_index: usize,
    ) -> Result<Vec<(u32, String, String)>> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        let mut out = Vec::new();
        for e in root.descendants() {
            if e.local_name != "animEffect" {
                continue;
            }
            let filter = e.get_attribute("filter").unwrap_or("fade").to_string();
            let transition = e.get_attribute("transition").unwrap_or("in").to_string();
            let mut shape_id = 0u32;
            for d in e.descendants() {
                if d.local_name == "spTgt" {
                    if let Some(s) = d.get_attribute("spid").and_then(|s| s.parse().ok()) {
                        shape_id = s;
                        break;
                    }
                }
            }
            out.push((shape_id, filter, transition));
        }
        Ok(out)
    }

    /// Remove animation timing entries that target `shape_id` on a slide.
    ///
    /// When no animation entries remain, the `timing` element is dropped.
    /// Returns how many `animEffect` nodes were removed.
    pub fn remove_animation_for_shape(
        &mut self,
        slide_index: usize,
        shape_id: u32,
    ) -> Result<usize> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        let mut removed = 0usize;
        fn prune(el: &mut OpenXmlElement, shape_id: u32, removed: &mut usize) {
            let mut i = 0;
            while i < el.children.len() {
                let is_match = el.children[i].local_name == "animEffect"
                    && el.children[i].descendants().any(|d| {
                        d.local_name == "spTgt"
                            && d.get_attribute("spid").and_then(|s| s.parse().ok())
                                == Some(shape_id)
                    });
                if is_match {
                    el.children.remove(i);
                    *removed += 1;
                } else {
                    prune(&mut el.children[i], shape_id, removed);
                    i += 1;
                }
            }
        }
        prune(&mut root, shape_id, &mut removed);
        // Drop empty timing if no animEffect left
        if removed > 0 {
            let still = root.descendants().any(|e| e.local_name == "animEffect");
            if !still {
                root.children.retain(|c| c.local_name != "timing");
            }
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(removed)
    }

    /// Set a transition effect on a slide.
    ///
    /// `effect` e.g. `"fade"`, `"dissolve"`, `"push"`, `"wipe"`.
    /// `speed` is `"slow"`, `"med"`, or `"fast"`.
    pub fn set_slide_transition(
        &mut self,
        slide_index: usize,
        effect: &str,
        speed: &str,
        advance_after_ms: Option<u32>,
    ) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = parse_element(
            self.package
                .opc()
                .get_part(&slide_info.uri)
                .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?,
        )?;
        root.children.retain(|c| c.local_name != "transition");
        root.append_child(slide_transition(effect, speed, true, advance_after_ms));
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Apply a fade transition to a slide.
    pub fn set_fade_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        let _ = fade_transition(speed);
        self.set_slide_transition(slide_index, "fade", speed, None)
    }

    /// Apply a dissolve transition to a slide.
    pub fn set_dissolve_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        let _ = dissolve_transition(speed);
        self.set_slide_transition(slide_index, "dissolve", speed, None)
    }

    /// Apply a push transition to a slide.
    pub fn set_push_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "push", speed, None)
    }

    /// Apply a wipe transition to a slide.
    pub fn set_wipe_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "wipe", speed, None)
    }

    /// Apply a split transition to a slide.
    pub fn set_split_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "split", speed, None)
    }

    /// Apply a cover transition to a slide.
    pub fn set_cover_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "cover", speed, None)
    }

    /// Apply a wheel transition to a slide.
    pub fn set_wheel_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "wheel", speed, None)
    }

    /// Apply a random transition to a slide.
    pub fn set_random_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "random", speed, None)
    }

    /// Apply a blinds transition to a slide.
    pub fn set_blinds_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "blinds", speed, None)
    }

    /// Apply a checker transition to a slide.
    pub fn set_checker_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "checker", speed, None)
    }

    /// Apply a circle transition to a slide.
    pub fn set_circle_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "circle", speed, None)
    }

    /// Apply a diamond transition to a slide.
    pub fn set_diamond_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "diamond", speed, None)
    }

    /// Apply a plus transition to a slide.
    pub fn set_plus_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "plus", speed, None)
    }

    /// Apply a newsflash transition to a slide.
    pub fn set_newsflash_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "newsflash", speed, None)
    }

    /// Apply a strips transition to a slide.
    pub fn set_strips_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "strips", speed, None)
    }

    /// Apply a wedge transition to a slide.
    pub fn set_wedge_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "wedge", speed, None)
    }

    /// Apply a zoom transition to a slide.
    pub fn set_zoom_transition(&mut self, slide_index: usize, speed: &str) -> Result<()> {
        self.set_slide_transition(slide_index, "zoom", speed, None)
    }

    /// Duplicate an existing slide (deep-copies slide XML and media relationships).
    ///
    /// Binary media parts are shared (same target); only the slide part is cloned.
    pub fn clone_slide(&mut self, slide_index: usize) -> Result<SlideInfo> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&slide_info.uri)
            .ok_or_else(|| Error::PartNotFound(slide_info.uri.to_string()))?
            .to_vec();
        let root = parse_element(&data)?;
        let new_info = self.add_slide(root)?;
        // Copy relationships from source slide (images, notes, etc.)
        if let Some(rels) = self
            .package
            .opc()
            .part_relationships(&slide_info.uri)
            .map(|r| {
                r.iter()
                    .map(|rel| {
                        (
                            rel.relationship_type.clone(),
                            rel.target.clone(),
                            rel.target_mode,
                        )
                    })
                    .collect::<Vec<_>>()
            })
        {
            for (ty, target, mode) in rels {
                // Skip notes slide auto-rel to avoid double notes; user can re-add
                if ty.contains("notesSlide") {
                    continue;
                }
                if mode == RelationshipTargetMode::External {
                    self.package
                        .opc_mut()
                        .part_relationships_mut(&new_info.uri)
                        .add(&ty, &target, mode);
                } else {
                    // Resolve target relative to source, re-relativize to new slide
                    if let Ok(abs) = crate::opc::resolve_uri(&slide_info.uri, &target) {
                        let _ = self.package.add_part_relationship(
                            &new_info.uri,
                            &ty,
                            &abs,
                            RelationshipTargetMode::Internal,
                        );
                    }
                }
            }
        }
        Ok(new_info)
    }

    /// Add a slide with the given root element. Returns slide info.
    pub fn add_slide(&mut self, slide_root: OpenXmlElement) -> Result<SlideInfo> {
        let pres_uri = self.ensure_presentation()?;
        let index = self.next_slide_index;
        self.next_slide_index += 1;
        let id = self.next_slide_id;
        self.next_slide_id += 1;
        let slide_uri = PackUri::new(format!("/ppt/slides/slide{index}.xml"));

        let xml = write_element(&slide_root)?;
        self.package
            .opc_mut()
            .set_part(slide_uri.clone(), content_type::PRESENTATION_SLIDE, xml);

        let slide_rel = self.package.add_part_relationship(
            &pres_uri,
            rel::SLIDE,
            &slide_uri,
            RelationshipTargetMode::Internal,
        );

        let info = SlideInfo {
            relationship_id: slide_rel,
            uri: slide_uri,
            id,
        };
        self.slides.push(info.clone());
        self.rewrite_presentation()?;
        Ok(info)
    }

    /// Add a blank slide (empty shape tree) linked to the default blank layout.
    ///
    /// Ensures slide master, layout, and theme exist so PowerPoint can open the file.
    pub fn add_blank_slide(&mut self) -> Result<SlideInfo> {
        use crate::presentation::empty_shape_tree;
        let sld = slide(vec![common_slide_data(vec![empty_shape_tree()])]);
        self.add_slide_with_layout(sld)
    }

    /// Embed an image on a slide as a picture shape.
    ///
    /// Coordinates and extents are in EMUs. Returns `(image_uri, image_rel_id)`.
    pub fn add_image_on_slide(
        &mut self,
        slide_index: usize,
        image_bytes: &[u8],
        content_type_str: &str,
        extension: &str,
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
        name: &str,
    ) -> Result<(PackUri, String)> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let slide_uri = slide_info.uri;

        let mut img_index = 1u32;
        let image_uri = loop {
            let candidate = PackUri::new(format!("/ppt/media/image{img_index}.{extension}"));
            if !self.package.opc().has_part(&candidate) {
                break candidate;
            }
            img_index += 1;
        };
        self.package
            .opc_mut()
            .set_part(image_uri.clone(), content_type_str, image_bytes.to_vec());
        let img_rel = self.package.add_part_relationship(
            &slide_uri,
            rel::IMAGE,
            &image_uri,
            RelationshipTargetMode::Internal,
        );

        // Load slide, append picture into shape tree
        let mut root = if let Some(data) = self.package.opc().get_part(&slide_uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_uri.to_string()));
        };
        let shape_id = root
            .descendants()
            .filter(|e| e.local_name == "cNvPr")
            .filter_map(|e| e.get_attribute("id").and_then(|s| s.parse::<u32>().ok()))
            .max()
            .unwrap_or(1)
            + 1;
        let pic = picture_shape(shape_id, name, &img_rel, x, y, cx, cy);

        // Find spTree under cSld
        if let Some(csld) = root.child_mut("cSld") {
            if let Some(tree) = csld.child_mut("spTree") {
                tree.append_child(pic);
            } else {
                csld.append_child(shape_tree(vec![pic]));
            }
        } else {
            root.append_child(common_slide_data(vec![shape_tree(vec![pic])]));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_uri, content_type::PRESENTATION_SLIDE, xml);
        Ok((image_uri, img_rel))
    }

    /// Embed an SVG on a slide exactly as PowerPoint does (no PNG fallback).
    ///
    /// Matches a real Office PPTX: `a:blip` has **no** `r:embed`; the SVG part is
    /// referenced only from `asvg:svgBlip/@r:embed` inside blip extension
    /// `{96DAC541-7B7A-43D3-8B79-37D633B846F1}`. The slide is also given
    /// `p:clrMapOvr` like Office-created slides.

    /// Convert an SVG into native DrawingML shapes and append them on a slide.
    ///
    /// Unlike [`add_svg_on_slide`], this does **not** embed the SVG media part —
    /// geometry is expanded to `p:sp` / `a:custGeom` / text boxes so PowerPoint
    /// edits them as ordinary shapes. Coordinates map the SVG viewBox onto the
    /// given EMU rectangle `(x, y, cx, cy)`.
    ///
    /// Default options: editable text boxes, no font embed. See
    /// [`add_svg_shapes_on_slide_ex`] for `--font-shape` / `--embed-font` /
    /// `--embed-font-fully`.
    ///
    /// Returns the number of shapes added.
    pub fn add_svg_shapes_on_slide(
        &mut self,
        slide_index: usize,
        svg_bytes: &[u8],
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
    ) -> Result<usize> {
        self.add_svg_shapes_on_slide_ex(
            slide_index,
            svg_bytes,
            x,
            y,
            cx,
            cy,
            SvgShapesOnSlideOptions::default(),
        )
    }

    /// Like [`add_svg_shapes_on_slide`], with text / font-embed options.
    pub fn add_svg_shapes_on_slide_ex(
        &mut self,
        slide_index: usize,
        svg_bytes: &[u8],
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
        options: SvgShapesOnSlideOptions,
    ) -> Result<usize> {
        use crate::presentation::svg_to_shapes;

        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let slide_uri = slide_info.uri;

        let mut root = if let Some(data) = self.package.opc().get_part(&slide_uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_uri.to_string()));
        };
        let start_id = root
            .descendants()
            .filter(|e| e.local_name == "cNvPr")
            .filter_map(|e| e.get_attribute("id").and_then(|s| s.parse::<u32>().ok()))
            .max()
            .unwrap_or(1)
            + 1;

        let prefer_embed = matches!(
            options.font_embed,
            SvgFontEmbedMode::Subset | SvgFontEmbedMode::Full
        ) && options.editable_text;
        let conv = svg_to_shapes::svg_to_shapes_with_options(
            svg_bytes,
            cx,
            cy,
            start_id,
            svg_to_shapes::SvgToShapesOptions {
                editable_text: options.editable_text,
                prefer_embeddable_faces: prefer_embed,
            },
        )?;

        // Font embed only for editable text boxes (outlined glyphs need no fonts).
        if options.editable_text
            && !matches!(options.font_embed, SvgFontEmbedMode::None)
        {
            let mut referenced = std::collections::HashSet::new();
            for sp in &conv.shapes {
                collect_referenced_typefaces(sp, &mut referenced);
            }
            if !referenced.is_empty() {
                let mut codepoints = std::collections::HashSet::new();
                for sp in &conv.shapes {
                    collect_text_codepoints(sp, &mut codepoints);
                }
                // Keep a small ASCII baseline for post-edit typing (subset mode).
                for c in 0x20u32..0x7Fu32 {
                    codepoints.insert(c);
                }
                self.package
                    .opc_mut()
                    .content_types_mut()
                    .set_default("fntdata", content_type::FONT_DATA);
                let mut embedded_any = false;
                let full = matches!(options.font_embed, SvgFontEmbedMode::Full);
                for uf in &conv.used_fonts {
                    if !referenced
                        .iter()
                        .any(|t| t.eq_ignore_ascii_case(&uf.typeface))
                    {
                        continue;
                    }
                    // Skip Windows/macOS system faces and Liberation/DejaVu (MS PPT
                    // refuses to install Liberation embeds).
                    let tf_l = uf.typeface.to_ascii_lowercase();
                    if matches!(
                        tf_l.as_str(),
                        "arial"
                            | "microsoft yahei"
                            | "微软雅黑"
                            | "simsun"
                            | "simhei"
                            | "nsimsun"
                            | "dengxian"
                            | "等线"
                            | "calibri"
                            | "segoe ui"
                            | "tahoma"
                            | "times new roman"
                            | "times"
                            | "courier new"
                            | "georgia"
                            | "liberation sans"
                            | "liberation serif"
                            | "liberation mono"
                            | "dejavu sans"
                            | "dejavu serif"
                    ) || tf_l.contains("yahei")
                        || tf_l.starts_with("liberation")
                        || tf_l.starts_with("dejavu")
                    {
                        continue;
                    }
                    if uf.data.len() > 24_000_000 {
                        continue;
                    }
                    if uf.data.get(0..4) == Some(b"ttcf") {
                        continue;
                    }
                    let font_bytes = if full {
                        // Full face; still force installable fsType via subset script
                        // only when huge (>2MB) CJK would bloat the package too much?
                        // User asked fully embed — keep original bytes.
                        uf.data.clone()
                    } else {
                        subset_ttf_for_embed(&uf.data, &codepoints)
                    };
                    let is_cjk = tf_l.contains("cjk")
                        || tf_l.contains("noto sans sc")
                        || uf.typeface.contains("思源")
                        || tf_l.contains("source han");
                    let charset_u8: u8 = if is_cjk { 134 } else { 0 };
                    let charset_xml: i32 = if is_cjk { -122 } else { 0 };
                    let pitch_family = if is_cjk { 2 } else { 34 };
                    let eot_info = {
                        let mut info = crate::presentation::svg_to_shapes::eot::font_info_from_sfnt(
                            &font_bytes,
                            &uf.typeface,
                        );
                        info.family = uf.typeface.clone();
                        info.charset = charset_u8;
                        info
                    };
                    let eot =
                        crate::presentation::svg_to_shapes::eot::to_eot(&font_bytes, &eot_info);
                    let panose = crate::presentation::svg_to_shapes::eot::panose_hex(&eot_info);
                    if let Ok((_uri, rid)) =
                        self.add_font_part_named(eot, content_type::FONT_DATA, "fntdata", None)
                    {
                        let _ = self.add_embedded_font_faces_full(
                            &uf.typeface,
                            if uf.bold { None } else { Some(&rid) },
                            if uf.bold { Some(&rid) } else { None },
                            Some(charset_xml),
                            Some(pitch_family),
                            Some(&panose),
                        );
                        embedded_any = true;
                    }
                }
                if embedded_any {
                    let _ = self.set_embed_true_type_fonts(true);
                    // saveSubsetFonts only when we actually subset.
                    let _ = self.set_save_subset_fonts(!full);
                }
            }
        }
        // Offset all shapes by (x, y) if non-zero
        let mut shapes = conv.shapes;
        if x != 0 || y != 0 {
            for sp in &mut shapes {
                offset_shape(sp, x, y);
            }
        }
        let count = shapes.len();
        if let Some(csld) = root.child_mut("cSld") {
            if let Some(tree) = csld.child_mut("spTree") {
                ensure_sp_tree_group_extents(tree, cx, cy);
                for sp in shapes {
                    tree.append_child(sp);
                }
            } else {
                let mut kids = vec![
                    crate::presentation::group_shape_properties(),
                    crate::presentation::group_shape_pr_sized(cx, cy),
                ];
                kids.extend(shapes);
                csld.append_child(shape_tree(kids));
            }
        } else {
            let mut kids = vec![
                crate::presentation::group_shape_properties(),
                crate::presentation::group_shape_pr_sized(cx, cy),
            ];
            kids.extend(shapes);
            root.append_child(common_slide_data(vec![shape_tree(kids)]));
        }
        if root.child("clrMapOvr").is_none() {
            root.append_child(
                OpenXmlElement::new(
                    "p",
                    "http://schemas.openxmlformats.org/presentationml/2006/main",
                    "clrMapOvr",
                )
                .with_child(OpenXmlElement::new(
                    "a",
                    "http://schemas.openxmlformats.org/drawingml/2006/main",
                    "masterClrMapping",
                )),
            );
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(count)
    }

    pub fn add_svg_on_slide(
        &mut self,
        slide_index: usize,
        svg_bytes: &[u8],
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
        name: &str,
    ) -> Result<PackUri> {
        use crate::presentation::picture_shape_svg;

        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let slide_uri = slide_info.uri;

        let mut index = 1u32;
        let svg_uri = loop {
            let c = PackUri::new(format!("/ppt/media/image{index}.svg"));
            if !self.package.opc().has_part(&c) {
                break c;
            }
            index += 1;
        };
        // Office registers SVG via Default Extension only (no Override for media).
        self.package
            .opc_mut()
            .content_types_mut()
            .set_default("svg", content_type::IMAGE_SVG);
        {
            // Insert media part without creating a content-type Override.
            let opc = self.package.opc_mut();
            // set_part always overrides; strip the override afterward so Default applies.
            opc.set_part(svg_uri.clone(), content_type::IMAGE_SVG, svg_bytes.to_vec());
            opc.content_types_mut()
                .overrides
                .shift_remove(svg_uri.as_str());
        }
        let svg_rel = self.package.add_part_relationship(
            &slide_uri,
            rel::IMAGE,
            &svg_uri,
            RelationshipTargetMode::Internal,
        );

        let mut root = if let Some(data) = self.package.opc().get_part(&slide_uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_uri.to_string()));
        };
        let shape_id = root
            .descendants()
            .filter(|e| e.local_name == "cNvPr")
            .filter_map(|e| e.get_attribute("id").and_then(|s| s.parse::<u32>().ok()))
            .max()
            .unwrap_or(1)
            + 1;
        let pic = picture_shape_svg(shape_id, name, &svg_rel, x, y, cx, cy);
        if let Some(csld) = root.child_mut("cSld") {
            if let Some(tree) = csld.child_mut("spTree") {
                tree.append_child(pic);
            } else {
                csld.append_child(shape_tree(vec![pic]));
            }
        } else {
            root.append_child(common_slide_data(vec![shape_tree(vec![pic])]));
        }
        // Office slides include clrMapOvr
        if root.child("clrMapOvr").is_none() {
            root.append_child(
                OpenXmlElement::new(
                    "p",
                    "http://schemas.openxmlformats.org/presentationml/2006/main",
                    "clrMapOvr",
                )
                .with_child(OpenXmlElement::new(
                    "a",
                    "http://schemas.openxmlformats.org/drawingml/2006/main",
                    "masterClrMapping",
                )),
            );
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(svg_uri)
    }

    /// Add a preset auto-shape on a slide (rectangle, ellipse, …).
    ///
    /// Coordinates are EMUs. `fill_rgb` is optional 6-digit hex without `#`.
    /// Returns the shape id assigned on the slide.
    pub fn add_auto_shape_on_slide(
        &mut self,
        slide_index: usize,
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
        preset: &str,
        fill_rgb: Option<&str>,
        name: &str,
    ) -> Result<u32> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let slide_uri = slide_info.uri;
        let mut root = if let Some(data) = self.package.opc().get_part(&slide_uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_uri.to_string()));
        };
        let shape_id = root
            .descendants()
            .filter(|e| e.local_name == "cNvPr")
            .filter_map(|e| e.get_attribute("id").and_then(|s| s.parse::<u32>().ok()))
            .max()
            .unwrap_or(1)
            + 1;
        let shape = auto_shape(shape_id, name, x, y, cx, cy, preset, fill_rgb);
        if let Some(csld) = root.child_mut("cSld") {
            if let Some(tree) = csld.child_mut("spTree") {
                tree.append_child(shape);
            } else {
                csld.append_child(shape_tree(vec![shape]));
            }
        } else {
            root.append_child(common_slide_data(vec![shape_tree(vec![shape])]));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(shape_id)
    }

    /// Add a text box shape on a slide.
    ///
    /// Returns the shape id assigned on the slide.
    pub fn add_text_box_on_slide(
        &mut self,
        slide_index: usize,
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
        text: &str,
        name: &str,
    ) -> Result<u32> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let slide_uri = slide_info.uri;
        let mut root = if let Some(data) = self.package.opc().get_part(&slide_uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_uri.to_string()));
        };
        let shape_id = root
            .descendants()
            .filter(|e| e.local_name == "cNvPr")
            .filter_map(|e| e.get_attribute("id").and_then(|s| s.parse::<u32>().ok()))
            .max()
            .unwrap_or(1)
            + 1;
        let shape = text_shape(shape_id, name, x, y, cx, cy, text);
        if let Some(csld) = root.child_mut("cSld") {
            if let Some(tree) = csld.child_mut("spTree") {
                tree.append_child(shape);
            } else {
                csld.append_child(shape_tree(vec![shape]));
            }
        } else {
            root.append_child(common_slide_data(vec![shape_tree(vec![shape])]));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(shape_id)
    }

    /// Attach an audio media part to a slide (relationship only — no timeline/animation).
    pub fn add_audio_on_slide(
        &mut self,
        slide_index: usize,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<crate::opc::MediaPartInfo> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        crate::opc::add_media_part(
            self.package.opc_mut(),
            &slide_info.uri,
            crate::opc::MediaKind::Audio,
            data,
            content_type_str,
            extension,
        )
    }

    /// Attach a video media part to a slide (relationship only).
    pub fn add_video_on_slide(
        &mut self,
        slide_index: usize,
        data: impl Into<Vec<u8>>,
        content_type_str: &str,
        extension: &str,
    ) -> Result<crate::opc::MediaPartInfo> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        crate::opc::add_media_part(
            self.package.opc_mut(),
            &slide_info.uri,
            crate::opc::MediaKind::Video,
            data,
            content_type_str,
            extension,
        )
    }

    /// Add a table graphic frame to an existing slide.
    ///
    /// Coordinates/extents are EMUs. `rows` is a 2D string grid.
    pub fn add_table_on_slide(
        &mut self,
        slide_index: usize,
        rows: &[Vec<&str>],
        x: i64,
        y: i64,
        cx: i64,
        cy: i64,
        name: &str,
    ) -> Result<()> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let slide_uri = slide_info.uri;
        let mut root = if let Some(data) = self.package.opc().get_part(&slide_uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(slide_uri.to_string()));
        };
        let shape_id = root
            .descendants()
            .filter(|e| e.local_name == "cNvPr")
            .filter_map(|e| e.get_attribute("id").and_then(|s| s.parse::<u32>().ok()))
            .max()
            .unwrap_or(1)
            + 1;
        let frame = table_graphic_frame(shape_id, name, x, y, cx, cy, rows);
        if let Some(csld) = root.child_mut("cSld") {
            if let Some(tree) = csld.child_mut("spTree") {
                tree.append_child(frame);
            } else {
                csld.append_child(shape_tree(vec![frame]));
            }
        } else {
            root.append_child(common_slide_data(vec![shape_tree(vec![frame])]));
        }
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(slide_uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(())
    }

    /// Add presentation part with a single empty slide.
    pub fn add_presentation_with_slide(&mut self) -> Result<String> {
        let sld = slide(vec![common_slide_data(vec![shape_tree(Vec::<
            OpenXmlElement,
        >::new())])]);
        let info = self.add_slide(sld)?;
        Ok(info.relationship_id)
    }

    /// Add a slide containing a text box with the given text.
    pub fn add_slide_with_text(&mut self, text: &str) -> Result<SlideInfo> {
        self.add_slide_with_layout(slide_with_text(text))
    }

    /// Read all DrawingML text runs from a slide (0-based index).
    pub fn slide_texts(&self, index: usize) -> Result<Vec<String>> {
        let info = self
            .slides
            .get(index)
            .ok_or_else(|| Error::Package(format!("slide index {index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(slide_texts(&root))
    }

    /// Convenience: texts from the first slide.
    pub fn first_slide_texts(&self) -> Result<Vec<String>> {
        self.slide_texts(0)
    }

    /// First non-empty text run on a slide (often the title placeholder).
    pub fn slide_title(&self, index: usize) -> Result<Option<String>> {
        Ok(self
            .slide_texts(index)?
            .into_iter()
            .find(|t| !t.trim().is_empty()))
    }

    /// Titles (first non-empty text) for every slide.
    pub fn slide_titles(&self) -> Result<Vec<Option<String>>> {
        let mut out = Vec::with_capacity(self.slides.len());
        for i in 0..self.slides.len() {
            out.push(self.slide_title(i)?);
        }
        Ok(out)
    }

    /// Replace text on a slide (all `a:t` nodes). Returns number of replacements.
    pub fn replace_slide_text(
        &mut self,
        slide_index: usize,
        from: &str,
        to: &str,
    ) -> Result<usize> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = if let Some(data) = self.package.opc().get_part(&info.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(info.uri.to_string()));
        };
        let count = replace_slide_text(&mut root, from, to);
        let xml = write_element(&root)?;
        self.package
            .opc_mut()
            .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        Ok(count)
    }

    /// Set the text of the first text-bearing shape on a slide.
    ///
    /// Replaces the entire content of the first `a:t` node found. Returns `true`
    /// if a text node was updated.
    pub fn set_slide_text(&mut self, slide_index: usize, text: &str) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = if let Some(data) = self.package.opc().get_part(&info.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(info.uri.to_string()));
        };
        let mut updated = false;
        set_first_t(&mut root, text, &mut updated);
        if updated {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(updated)
    }

    /// Set the text of the n-th text node (`a:t`) on a slide (0-based among all `a:t`).
    pub fn set_slide_text_at(
        &mut self,
        slide_index: usize,
        text_index: usize,
        text: &str,
    ) -> Result<bool> {
        let info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let mut root = if let Some(data) = self.package.opc().get_part(&info.uri) {
            parse_element(data)?
        } else {
            return Err(Error::PartNotFound(info.uri.to_string()));
        };
        let mut idx = 0usize;
        let mut updated = false;
        fn visit(
            el: &mut OpenXmlElement,
            target: usize,
            text: &str,
            idx: &mut usize,
            updated: &mut bool,
        ) {
            if *updated {
                return;
            }
            if el.local_name == "t" {
                if *idx == target {
                    el.set_text(text);
                    *updated = true;
                    return;
                }
                *idx += 1;
            }
            for c in el.children.iter_mut() {
                visit(c, target, text, idx, updated);
            }
        }
        visit(&mut root, text_index, text, &mut idx, &mut updated);
        if updated {
            let xml = write_element(&root)?;
            self.package
                .opc_mut()
                .set_part(info.uri, content_type::PRESENTATION_SLIDE, xml);
        }
        Ok(updated)
    }

    /// Count text nodes (`a:t`) on a slide.
    pub fn slide_text_node_count(&self, slide_index: usize) -> Result<usize> {
        let info = self
            .slides
            .get(slide_index)
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let data = self
            .package
            .opc()
            .get_part(&info.uri)
            .ok_or_else(|| Error::PartNotFound(info.uri.to_string()))?;
        let root = parse_element(data)?;
        Ok(root.descendants().filter(|e| e.local_name == "t").count())
    }

    /// Add an external hyperlink relationship from a slide.
    ///
    /// Returns the relationship id. Callers can wire it into a shape click via
    /// DrawingML `hlinkClick`; this method only creates the package relationship.
    pub fn add_slide_hyperlink(&mut self, slide_index: usize, url: &str) -> Result<String> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let rid = self
            .package
            .add_hyperlink_relationship(&slide_info.uri, url, true);
        Ok(rid)
    }

    /// List external hyperlink relationships on a slide as `(id, target)`.
    pub fn list_slide_hyperlinks(&self, slide_index: usize) -> Result<Vec<(String, String)>> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        Ok(self
            .package
            .opc()
            .part_relationships(&slide_info.uri)
            .map(|rels| {
                rels.iter()
                    .filter(|r| {
                        r.relationship_type.contains("hyperlink")
                            || r.relationship_type == rel::HYPERLINK
                    })
                    .map(|r| (r.id.clone(), r.target.clone()))
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Remove a hyperlink relationship from a slide by rId. Returns whether found.
    /// Indices of slides that have hyperlinks.
    /// Whether a slide has any hyperlink relationships.
    pub fn has_slide_hyperlinks(&self, slide_index: usize) -> Result<bool> {
        Ok(!self.list_slide_hyperlinks(slide_index)?.is_empty())
    }

    pub fn slides_with_hyperlinks(&self) -> Result<Vec<usize>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            if !self.list_slide_hyperlinks(i)?.is_empty() {
                out.push(i);
            }
        }
        Ok(out)
    }

    /// Whether any slide has hyperlinks.
    pub fn has_slides_with_hyperlinks(&self) -> Result<bool> {
        Ok(!self.slides_with_hyperlinks()?.is_empty())
    }

    pub fn remove_slide_hyperlink(&mut self, slide_index: usize, rid: &str) -> Result<bool> {
        let slide_info = self
            .slides
            .get(slide_index)
            .cloned()
            .ok_or_else(|| Error::Package(format!("slide index {slide_index} out of range")))?;
        let rels = self
            .package
            .opc_mut()
            .part_relationships_mut(&slide_info.uri);
        let removed = rels.remove(rid).is_some();
        Ok(removed)
    }

    /// Whether any slide has hyperlink relationships.
    pub fn has_hyperlinks(&self) -> bool {
        (0..self.slides.len()).any(|i| {
            self.list_slide_hyperlinks(i)
                .map(|v| !v.is_empty())
                .unwrap_or(false)
        })
    }

    /// Total external hyperlink relationships across all slides.
    pub fn hyperlink_count(&self) -> Result<usize> {
        let mut n = 0;
        for i in 0..self.slides.len() {
            n += self.list_slide_hyperlinks(i)?.len();
        }
        Ok(n)
    }

    /// List hyperlinks across slides as `(slide_index, id, target)`.
    pub fn list_hyperlinks(&self) -> Result<Vec<(usize, String, String)>> {
        let mut out = Vec::new();
        for i in 0..self.slides.len() {
            for (id, target) in self.list_slide_hyperlinks(i)? {
                out.push((i, id, target));
            }
        }
        Ok(out)
    }

    /// Alias for [`list_hyperlinks`](Self::list_hyperlinks) (Word-compatible name).
    pub fn list_external_hyperlinks(&self) -> Result<Vec<(usize, String, String)>> {
        self.list_hyperlinks()
    }

    /// Alias for [`has_hyperlinks`](Self::has_hyperlinks).
    pub fn has_external_hyperlinks(&self) -> bool {
        self.has_hyperlinks()
    }

    /// Alias for [`hyperlink_count`](Self::hyperlink_count).
    pub fn external_hyperlink_count(&self) -> Result<usize> {
        self.hyperlink_count()
    }

    /// Alias for [`clear_hyperlinks`](Self::clear_hyperlinks).
    pub fn clear_external_hyperlinks(&mut self) -> Result<usize> {
        self.clear_hyperlinks()
    }

    /// Remove all external hyperlink relationships from all slides.
    pub fn clear_hyperlinks(&mut self) -> Result<usize> {
        let mut n = 0;
        let slide_uris: Vec<PackUri> = self.slides.iter().map(|s| s.uri.clone()).collect();
        for slide_uri in slide_uris {
            let ids: Vec<String> = self
                .package
                .opc()
                .part_relationships(&slide_uri)
                .map(|rels| {
                    rels.iter()
                        .filter(|r| {
                            r.relationship_type.contains("hyperlink")
                                || r.relationship_type == rel::HYPERLINK
                        })
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default();
            if ids.is_empty() {
                continue;
            }
            n += ids.len();
            let rels = self.package.opc_mut().part_relationships_mut(&slide_uri);
            for id in ids {
                rels.remove(&id);
            }
        }
        Ok(n)
    }

    pub fn save(&mut self) -> Result<()> {
        self.package.save()
    }

    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.package.save_as(path)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.package.to_bytes()
    }

    /// Change the presentation content type (e.g. template → presentation).
    pub fn change_document_type(&mut self, new_type: PresentationDocumentType) -> Result<()> {
        let pres_uri = PackUri::new(PRESENTATION_URI);
        let ct = new_type.content_type();
        let data = self
            .package
            .opc()
            .get_part(&pres_uri)
            .map(|b| b.to_vec())
            .unwrap_or_default();
        self.package.set_part(pres_uri, ct, data);
        self.document_type = new_type;
        Ok(())
    }

    /// Close the presentation, saving if `auto_save` is enabled and a path is set.
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
        self.package.close_without_save();
        Ok(())
    }
}

fn set_first_t(elem: &mut OpenXmlElement, text: &str, updated: &mut bool) {
    if *updated {
        return;
    }
    if elem.local_name == "t" {
        elem.set_text(text);
        *updated = true;
        return;
    }
    for child in &mut elem.children {
        set_first_t(child, text, updated);
        if *updated {
            return;
        }
    }
}

impl Drop for PresentationDocument {
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
            let _ = self.package.save();
        }
    }
}

fn offset_shape(elem: &mut OpenXmlElement, dx: i64, dy: i64) {
    if elem.local_name == "off" {
        if let Some(x) = elem.get_attribute("x").and_then(|s| s.parse::<i64>().ok()) {
            elem.set_attribute("x", (x + dx).to_string());
        }
        if let Some(y) = elem.get_attribute("y").and_then(|s| s.parse::<i64>().ok()) {
            elem.set_attribute("y", (y + dy).to_string());
        }
    }
    for child in &mut elem.children {
        offset_shape(child, dx, dy);
    }
}

/// True when a shape tree fragment still references a font typeface (text runs).
/// Outlined glyph shapes (`custGeom` only) do not need embedded fonts.

fn collect_text_codepoints(elem: &OpenXmlElement, out: &mut std::collections::HashSet<u32>) {
    if elem.local_name == "t" {
        if let Some(t) = elem.text.as_deref() {
            for c in t.chars() {
                out.insert(c as u32);
            }
        }
    }
    for child in &elem.children {
        collect_text_codepoints(child, out);
    }
}

/// Glyph-subset a TrueType face to `codepoints` (on-demand embed).
///
/// Uses `scripts/subset_ttf.py` + fontTools. Returns original bytes when the
/// face is not TrueType SFNT, subsetting is unavailable, or the subset is not smaller.
fn subset_ttf_for_embed(font_bytes: &[u8], codepoints: &std::collections::HashSet<u32>) -> Vec<u8> {
    // TrueType only (`\0\x01\0\0`); skip CFF/OTTO and collections.
    if font_bytes.get(0..4) != Some(b"\x00\x01\x00\x00") {
        return font_bytes.to_vec();
    }
    if codepoints.is_empty() {
        return font_bytes.to_vec();
    }
    // Tiny faces already smaller than a useful subset payload — still try when
    // larger than ~32KB so ASCII-only packs shrink consistently.
    if font_bytes.len() < 32_768 {
        return font_bytes.to_vec();
    }
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/subset_ttf.py");
    if !script.exists() {
        return font_bytes.to_vec();
    }
    let tmp = std::env::temp_dir();
    let pid = std::process::id();
    // Unique paths per call so regular/bold subsets in one process do not race.
    let nonce = codepoints.len().wrapping_mul(font_bytes.len());
    let in_path = tmp.join(format!("openxml-embed-in-{pid}-{nonce}.ttf"));
    let out_path = tmp.join(format!("openxml-embed-out-{pid}-{nonce}.ttf"));
    if std::fs::write(&in_path, font_bytes).is_err() {
        return font_bytes.to_vec();
    }
    let mut args: Vec<String> = vec![
        script.to_string_lossy().into_owned(),
        in_path.to_string_lossy().into_owned(),
        out_path.to_string_lossy().into_owned(),
    ];
    // Cap codepoint list to keep argv reasonable; include all for typical slides.
    for cp in codepoints.iter().copied().take(4000) {
        args.push(format!("{cp:X}"));
    }
    let status = std::process::Command::new("python3").args(&args).status();
    let result = match status {
        Ok(s) if s.success() => std::fs::read(&out_path).ok(),
        _ => None,
    };
    let _ = std::fs::remove_file(&in_path);
    let _ = std::fs::remove_file(&out_path);
    match result {
        Some(b) if !b.is_empty() && b.len() <= font_bytes.len() => b,
        _ => font_bytes.to_vec(),
    }
}

/// Collect `a:latin` / `a:ea` / `a:cs` / `a:sym` typeface names from a shape tree.
fn collect_referenced_typefaces(elem: &OpenXmlElement, out: &mut std::collections::HashSet<String>) {
    if matches!(elem.local_name.as_str(), "latin" | "ea" | "cs" | "sym") {
        if let Some(tf) = elem.get_attribute("typeface") {
            if !tf.is_empty() && tf != "+mn-lt" && tf != "+mj-lt" && tf != "+mn-ea" && tf != "+mj-ea" {
                out.insert(tf.to_string());
            }
        }
    }
    for child in &elem.children {
        collect_referenced_typefaces(child, out);
    }
}

fn shape_references_typeface(elem: &OpenXmlElement) -> bool {
    if matches!(elem.local_name.as_str(), "latin" | "ea" | "cs" | "sym")
        && elem.get_attribute("typeface").is_some()
    {
        return true;
    }
    if elem.local_name == "txBody" {
        return true;
    }
    elem.children.iter().any(shape_references_typeface)
}

/// Insert index for `p:embeddedFontLst` under `p:presentation` per ECMA-376 CT_Presentation:
/// after `notesSz` (or `sldSz`), before `custShowLst` / `defaultTextStyle` / later siblings.
fn presentation_embedded_font_lst_insert_at(root: &OpenXmlElement) -> usize {
    const AFTER: &[&str] = &[
        "sldMasterIdLst",
        "notesMasterIdLst",
        "handoutMasterIdLst",
        "sldIdLst",
        "sldSz",
        "notesSz",
    ];
    const BEFORE: &[&str] = &[
        "custShowLst",
        "photoAlbum",
        "custDataLst",
        "kinsoku",
        "defaultTextStyle",
        "modifyVerifier",
        "extLst",
    ];
    if let Some(i) = root
        .children
        .iter()
        .position(|c| BEFORE.contains(&c.local_name.as_str()))
    {
        return i;
    }
    if let Some(i) = root
        .children
        .iter()
        .rposition(|c| AFTER.contains(&c.local_name.as_str()))
    {
        return i + 1;
    }
    root.children.len()
}

/// Ensure root `p:spTree/p:grpSpPr` extents match the slide target (non-zero).
fn ensure_sp_tree_group_extents(tree: &mut OpenXmlElement, cx: i64, cy: i64) {
    let cx = cx.max(0);
    let cy = cy.max(0);
    let Some(grp) = tree.child_mut("grpSpPr") else {
        tree.children.insert(
            if tree.child("nvGrpSpPr").is_some() { 1 } else { 0 },
            crate::presentation::group_shape_pr_sized(cx, cy),
        );
        return;
    };
    let Some(xfrm) = grp.child_mut("xfrm") else {
        *grp = crate::presentation::group_shape_pr_sized(cx, cy);
        return;
    };
    for (tag, xk, yk, xv, yv) in [
        ("ext", "cx", "cy", cx, cy),
        ("chExt", "cx", "cy", cx, cy),
        ("off", "x", "y", 0i64, 0i64),
        ("chOff", "x", "y", 0i64, 0i64),
    ] {
        if let Some(el) = xfrm.child_mut(tag) {
            el.set_attribute(xk, xv.to_string());
            el.set_attribute(yk, yv.to_string());
        } else {
            xfrm.append_child(
                OpenXmlElement::new("a", crate::namespace::ns::DRAWINGML.uri, tag)
                    .with_attribute(xk, xv.to_string())
                    .with_attribute(yk, yv.to_string()),
            );
        }
    }
}
