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
        self.opc.save()
    }

    /// Save the package to a new path.
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.ensure_open()?;
        self.opc.save_as(path)
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
        self.closed = true;
    }
}
