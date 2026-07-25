//! OpenXmlPackage — base type for Word / Excel / PowerPoint packages.

use crate::error::{Error, Result};
use crate::features::FeatureCollection;
use crate::file_format::FileFormatVersions;
use crate::opc::OpcPackage;
use std::path::Path;

/// How Markup Compatibility content is processed when loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkupCompatibilityProcessMode {
    /// Leave MC markup untouched (default).
    #[default]
    NoProcess,
    /// Process AlternateContent / Ignorable on loaded part roots.
    ProcessLoadedPartsOnly,
}

/// Markup Compatibility processing settings (mirrors C# `MarkupCompatibilityProcessSettings`).
#[derive(Debug, Clone)]
pub struct MarkupCompatibilityProcessSettings {
    pub mode: MarkupCompatibilityProcessMode,
    /// Target Office version used when resolving ignorable prefixes.
    pub target_file_format_versions: FileFormatVersions,
}

impl Default for MarkupCompatibilityProcessSettings {
    fn default() -> Self {
        Self {
            mode: MarkupCompatibilityProcessMode::NoProcess,
            target_file_format_versions: FileFormatVersions::OFFICE2007,
        }
    }
}

/// Settings used when opening a package.
#[derive(Debug, Clone)]
pub struct OpenSettings {
    /// Automatically save on drop / close when the package was opened for write.
    pub auto_save: bool,
    /// Maximum characters allowed in a part (0 = unlimited).
    pub max_characters_in_part: u64,
    /// Markup Compatibility processing.
    pub markup_compatibility: MarkupCompatibilityProcessSettings,
    /// ZIP compression for ordinary parts (C# `OpenXmlPackage.CompressionOption`).
    pub compression: crate::opc::CompressionOption,
}

impl Default for OpenSettings {
    fn default() -> Self {
        Self {
            auto_save: true,
            max_characters_in_part: 0,
            markup_compatibility: MarkupCompatibilityProcessSettings::default(),
            compression: crate::opc::CompressionOption::Normal,
        }
    }
}

/// Base Open XML package.
///
/// Holds the underlying OPC package and open settings. Typed documents
/// (`WordprocessingDocument`, etc.) wrap this.
#[derive(Debug)]
pub struct OpenXmlPackage {
    opc: OpcPackage,
    settings: OpenSettings,
    closed: bool,
    features: FeatureCollection,
}

impl OpenXmlPackage {
    pub(crate) fn from_opc(mut opc: OpcPackage, settings: OpenSettings) -> Self {
        opc.set_compression_option(settings.compression);
        Self {
            opc,
            settings,
            closed: false,
            features: FeatureCollection::new(),
        }
    }

    /// Write package bytes to a stream (after callers flush typed dirty parts).
    pub fn write_to<W: std::io::Write>(&mut self, writer: W) -> Result<()> {
        self.ensure_open()?;
        self.opc.write_to(writer)
    }

    pub fn settings(&self) -> &OpenSettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut OpenSettings {
        &mut self.settings
    }

    pub fn auto_save(&self) -> bool {
        self.settings.auto_save
    }

    pub fn opc(&self) -> &OpcPackage {
        &self.opc
    }

    pub fn opc_mut(&mut self) -> &mut OpcPackage {
        &mut self.opc
    }

    /// Feature bag (typed services attached to this package).
    pub fn features(&self) -> &FeatureCollection {
        &self.features
    }

    pub fn features_mut(&mut self) -> &mut FeatureCollection {
        &mut self.features
    }

    /// Content-type → extension map (C# `PartExtensionProvider`).
    ///
    /// Stored in the feature bag; created on first access with known Office defaults.
    pub fn part_extension_provider(&mut self) -> &mut crate::opc::PartExtensionProvider {
        if !self.features.contains::<crate::opc::PartExtensionProvider>() {
            self.features
                .set(crate::opc::PartExtensionProvider::with_known_extensions());
        }
        self.features
            .get_mut::<crate::opc::PartExtensionProvider>()
            .expect("PartExtensionProvider just set")
    }

    /// Extension for `content_type` from the provider, or from the built-in table.
    pub fn extension_for_content_type(&mut self, content_type: &str) -> String {
        self.part_extension_provider()
            .extension_or_bin(content_type)
            .to_string()
    }


    /// Ensure a [`crate::features::PackageEvents`] feature exists.
    pub fn package_events(&mut self) -> &crate::features::PackageEvents {
        if !self.features.contains::<crate::features::PackageEvents>() {
            self.features.set(crate::features::PackageEvents::new());
        }
        self.features
            .get::<crate::features::PackageEvents>()
            .expect("PackageEvents just set")
    }

    /// Raise a package lifecycle event if a listener hub is registered (no-op otherwise).
    pub fn raise_package_event(&self, event_type: crate::features::PackageEventType) {
        if let Some(ev) = self.features.get::<crate::features::PackageEvents>() {
            ev.raise_type(event_type);
        }
    }

    pub fn raise_part_event(
        &self,
        event_type: crate::features::PackageEventType,
        part_uri: impl Into<String>,
    ) {
        if let Some(ev) = self.features.get::<crate::features::PackageEvents>() {
            ev.raise_part(event_type, part_uri);
        }
    }

    /// Ensure [`AnnotationsFeature`](crate::features::AnnotationsFeature) exists.
    fn annotations_feature_mut(&mut self) -> &mut crate::features::AnnotationsFeature {
        if !self.features.contains::<crate::features::AnnotationsFeature>() {
            self.features
                .set(crate::features::AnnotationsFeature::new());
        }
        self.features
            .get_mut::<crate::features::AnnotationsFeature>()
            .expect("AnnotationsFeature just set")
    }

    /// Add a package-level annotation (C# `OpenXmlPackage.AddAnnotation`).
    pub fn add_annotation<T: std::any::Any + Send + Sync>(&mut self, value: T) {
        self.annotations_feature_mut().add(value);
    }

    /// First package-level annotation of type `T`.
    pub fn annotation<T: std::any::Any + Send + Sync>(&self) -> Option<&T> {
        self.features
            .get::<crate::features::AnnotationsFeature>()
            .and_then(|a| a.get::<T>())
    }

    /// Remove package-level annotations of type `T`.
    pub fn remove_annotations<T: std::any::Any + Send + Sync>(&mut self) {
        if let Some(a) = self.features.get_mut::<crate::features::AnnotationsFeature>() {
            a.remove::<T>();
        }
    }

    /// Create an empty media data part (C# `CreateMediaDataPart`).
    pub fn create_media_data_part(
        &mut self,
        content_type: &str,
        extension: Option<&str>,
    ) -> crate::error::Result<crate::opc::DataPart> {
        self.opc_mut().create_media_data_part(content_type, extension)
    }

    /// Create a media data part pre-filled with bytes.
    pub fn create_media_data_part_with_data(
        &mut self,
        content_type: &str,
        extension: Option<&str>,
        data: impl Into<Vec<u8>>,
    ) -> crate::error::Result<crate::opc::DataPart> {
        self.opc_mut()
            .create_media_data_part_with_data(content_type, extension, data)
    }

    /// Registered data parts (C# `DataParts`).
    pub fn data_parts(&self) -> &[crate::opc::DataPart] {
        self.opc().data_parts()
    }

    /// Delete unused (unreferenced) data parts.
    pub fn delete_unused_data_parts(&mut self) -> usize {
        self.opc_mut().delete_unused_data_parts()
    }

    pub fn path(&self) -> Option<&Path> {
        self.opc.path()
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub(crate) fn ensure_open(&self) -> Result<()> {
        if self.closed {
            Err(Error::Closed)
        } else {
            Ok(())
        }
    }

    /// Save the package to its associated path.
    pub fn save(&mut self) -> Result<()> {
        self.ensure_open()?;
        self.raise_package_event(crate::features::PackageEventType::Saving);
        self.opc.save()?;
        self.raise_package_event(crate::features::PackageEventType::Saved);
        Ok(())
    }

    /// Save the package to a new path.
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.ensure_open()?;
        self.raise_package_event(crate::features::PackageEventType::Saving);
        self.opc.save_as(path)?;
        self.raise_package_event(crate::features::PackageEventType::Saved);
        Ok(())
    }

    /// Serialize to bytes without writing to disk.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.ensure_open()?;
        self.opc.to_bytes()
    }

    /// Mark closed without saving.
    pub fn close_without_save(&mut self) {
        self.closed = true;
    }

    pub(crate) fn mark_closed(&mut self) {
        self.raise_package_event(crate::features::PackageEventType::Closing);
        self.closed = true;
        self.raise_package_event(crate::features::PackageEventType::Closed);
    }
}
