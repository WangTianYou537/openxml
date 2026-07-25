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
    /// Process Markup Compatibility on every XML part in the package
    /// (C# `ProcessAllParts`).
    ProcessAllParts,
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

    /// Validate part relationship constraints (C# `PackageValidator`).
    pub fn validate_package_constraints(&self) -> Vec<crate::validation::ValidationError> {
        crate::validation::validate_package_constraints(&self.opc)
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

    /// All parts reachable from package relationships, BFS order
    /// (C# `OpenXmlPackageExtensions.GetAllParts`).
    pub fn get_all_parts(&self) -> Vec<(crate::opc::PackUri, String)> {
        self.opc.get_all_parts()
    }

    /// C# `StrictRelationshipFound`.
    pub fn strict_relationship_found(&self) -> bool {
        self.opc.strict_relationship_found()
    }

    /// Whether any part still embeds a Strict OOXML namespace URI.
    pub fn strict_namespace_found(&self) -> bool {
        self.opc.strict_namespace_found()
    }

    /// Normalize Strict → Transitional namespaces and relationship types.
    pub fn rewrite_strict_to_transitional(&mut self) -> crate::error::Result<(usize, usize)> {
        crate::namespace_rewrite::rewrite_package_to_transitional(&mut self.opc)
    }

    /// Delete multiple parts by URI (C# `DeleteParts`), raising part Removing/Removed events.
    pub fn delete_parts(&mut self, uris: &[crate::opc::PackUri]) -> usize {
        let mut n = 0;
        for uri in uris {
            if self.delete_part(uri).is_some() {
                n += 1;
            }
        }
        n
    }

    /// Remove a part and raise part Removing/Removed events (C# `DeletePart` + `IPartEventsFeature`).
    pub fn delete_part(&mut self, uri: &crate::opc::PackUri) -> Option<Vec<u8>> {
        let uri_str = uri.to_string();
        self.raise_part_event(crate::features::PackageEventType::Removing, &uri_str);
        self.raise_part_event(crate::features::PackageEventType::Deleting, &uri_str);
        let data = self.opc.remove_part(uri);
        if data.is_some() {
            self.raise_part_event(crate::features::PackageEventType::Removed, &uri_str);
            self.raise_part_event(crate::features::PackageEventType::Deleted, &uri_str);
        }
        data
    }

    /// Insert or replace part bytes and raise Adding/Added (or Creating/Created) part events.
    pub fn set_part(
        &mut self,
        uri: impl Into<crate::opc::PackUri>,
        content_type: impl Into<String>,
        data: impl Into<Vec<u8>>,
    ) {
        let uri = uri.into();
        let uri_str = uri.to_string();
        let existed = self.opc.has_part(&uri);
        if existed {
            self.raise_part_event(crate::features::PackageEventType::Adding, &uri_str);
        } else {
            self.raise_part_event(crate::features::PackageEventType::Creating, &uri_str);
            self.raise_part_event(crate::features::PackageEventType::Adding, &uri_str);
        }
        self.opc.set_part(uri, content_type, data);
        if existed {
            self.raise_part_event(crate::features::PackageEventType::Added, &uri_str);
        } else {
            self.raise_part_event(crate::features::PackageEventType::Created, &uri_str);
            self.raise_part_event(crate::features::PackageEventType::Added, &uri_str);
        }
    }

    /// Apply Markup Compatibility processing to all XML parts when mode is
    /// [`ProcessAllParts`](MarkupCompatibilityProcessMode::ProcessAllParts).
    pub fn process_markup_compatibility_all_parts(&mut self) -> crate::error::Result<usize> {
        if self.settings.markup_compatibility.mode != MarkupCompatibilityProcessMode::ProcessAllParts
        {
            return Ok(0);
        }
        let version = self.settings.markup_compatibility.target_file_format_versions;
        let uris = self.opc.part_uris();
        let mut n = 0usize;
        for uri in uris {
            let Some(data) = self.opc.get_part(&uri).map(|b| b.to_vec()) else {
                continue;
            };
            let trimmed: Vec<u8> = data
                .iter()
                .skip_while(|b| b.is_ascii_whitespace())
                .copied()
                .take(5)
                .collect();
            if !(trimmed.starts_with(b"<?xml") || trimmed.starts_with(b"<")) {
                continue;
            }
            let mut root = match crate::element::parse_element(&data) {
                Ok(r) => r,
                Err(_) => continue,
            };
            crate::markup_compatibility::process_markup_compatibility_for_version(
                &mut root, version,
            );
            let ct = self
                .opc
                .content_types()
                .content_type_for(uri.as_str())
                .unwrap_or("application/xml")
                .to_string();
            let xml = crate::element::write_element(&root)?;
            self.opc.set_part(uri, ct, xml);
            n += 1;
        }
        Ok(n)
    }




    /// Ensure a [`crate::features::PackageEvents`] feature exists.
    /// Ensure an [`OpenXmlElementContext`](crate::element::OpenXmlElementContext) feature exists.
    pub fn element_context(&mut self) -> &crate::element::OpenXmlElementContext {
        if !self
            .features()
            .contains::<crate::element::OpenXmlElementContext>()
        {
            self.features_mut()
                .set(crate::element::OpenXmlElementContext::new());
        }
        self.features()
            .get::<crate::element::OpenXmlElementContext>()
            .expect("element context just set")
    }

    pub fn element_context_mut(&mut self) -> &mut crate::element::OpenXmlElementContext {
        if !self
            .features()
            .contains::<crate::element::OpenXmlElementContext>()
        {
            self.features_mut()
                .set(crate::element::OpenXmlElementContext::new());
        }
        self.features_mut()
            .get_mut::<crate::element::OpenXmlElementContext>()
            .expect("element context just set")
    }

    pub fn package_events(&mut self) -> &crate::features::PackageEvents {
        if !self.features.contains::<crate::features::PackageEvents>() {
            self.features.set(crate::features::PackageEvents::new());
        }
        self.features
            .get::<crate::features::PackageEvents>()
            .expect("PackageEvents just set")
    }

    /// Ensure a [`PartEvents`](crate::features::PartEvents) feature exists
    /// (C# `AddPartEventsFeature` / `IPartEventsFeature`).
    pub fn part_events(&mut self) -> &crate::features::PartEvents {
        if !self.features.contains::<crate::features::PartEvents>() {
            self.features.set(crate::features::PartEvents::new());
        }
        self.features
            .get::<crate::features::PartEvents>()
            .expect("PartEvents just set")
    }

    /// Raise a package lifecycle event if a listener hub is registered (no-op otherwise).
    pub fn raise_package_event(&self, event_type: crate::features::PackageEventType) {
        if let Some(ev) = self.features.get::<crate::features::PackageEvents>() {
            ev.raise_type(event_type);
        }
    }

    /// Raise a part-container event on [`PartEvents`] (and mirror on [`PackageEvents`] if present).
    pub fn raise_part_event(
        &self,
        event_type: crate::features::PackageEventType,
        part_uri: impl Into<String>,
    ) {
        let uri = part_uri.into();
        if let Some(ev) = self.features.get::<crate::features::PartEvents>() {
            ev.raise(event_type, uri.clone());
        }
        if let Some(ev) = self.features.get::<crate::features::PackageEvents>() {
            ev.raise_part(event_type, uri);
        }
    }

    /// Ensure a [`PartRootEvents`](crate::features::PartRootEvents) feature exists
    /// (C# `AddPartRootEventsFeature`).
    pub fn part_root_events(&mut self) -> &crate::features::PartRootEvents {
        if !self
            .features
            .contains::<crate::features::PartRootEvents>()
        {
            self.features
                .set(crate::features::PartRootEvents::new());
        }
        self.features
            .get::<crate::features::PartRootEvents>()
            .expect("PartRootEvents just set")
    }

    /// Raise a part-root lifecycle event if the hub is registered (no-op otherwise).
    pub fn raise_part_root_event(
        &self,
        event_type: crate::features::PackageEventType,
        part_uri: impl Into<String>,
    ) {
        if let Some(ev) = self.features.get::<crate::features::PartRootEvents>() {
            ev.raise(event_type, part_uri);
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

    /// Delete a data part if unreferenced (C# `DeletePart(DataPart)`).
    pub fn delete_data_part(&mut self, uri: &crate::opc::PackUri) -> Result<bool> {
        self.ensure_open()?;
        self.opc.delete_data_part(uri)
    }

    pub fn path(&self) -> Option<&Path> {
        self.opc.path()
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Whether the package can be saved (C# `CanSave`).
    ///
    /// False when closed or opened read-only.
    pub fn can_save(&self) -> bool {
        if self.closed {
            return false;
        }
        matches!(
            self.opc.mode(),
            crate::opc::PackageMode::Create | crate::opc::PackageMode::ReadWrite
        )
    }

    /// File access mode shell (C# `FileOpenAccess`).
    pub fn file_open_access(&self) -> crate::opc::PackageMode {
        self.opc.mode()
    }

    /// Max characters per part from open settings (C# `MaxCharactersInPart`).
    pub fn max_characters_in_part(&self) -> u64 {
        self.settings.max_characters_in_part
    }

    /// Close the package: optionally save, delete unused data parts, raise events
    /// (C# `Dispose` / close path simplified).
    pub fn close(&mut self, save: bool) -> Result<()> {
        self.ensure_open()?;
        if save && self.can_save() {
            self.save()?;
        }
        // C# DeleteUnusedDataPartOnClose
        self.opc.delete_unused_data_parts();
        self.mark_closed();
        Ok(())
    }

    /// Import a part (optionally recursive) from another package (C# `AddPart` cross-package).
    pub fn copy_part_from(
        &mut self,
        source: &OpenXmlPackage,
        source_uri: &crate::opc::PackUri,
        dest_uri: &crate::opc::PackUri,
        opts: crate::opc::CopyPartOptions,
    ) -> Result<std::collections::HashMap<crate::opc::PackUri, crate::opc::PackUri>> {
        self.ensure_open()?;
        self.opc
            .copy_part_from(source.opc(), source_uri, dest_uri, opts)
    }

    /// Structural package comparison (part set, relationships, XML DOM).
    pub fn compare_to(
        &self,
        other: &OpenXmlPackage,
        opts: crate::opc::PackageEqualityOptions,
    ) -> crate::opc::PackageDiff {
        crate::opc::compare_packages(self.opc(), other.opc(), &opts)
    }

    /// Whether packages are structurally equal under default options.
    pub fn packages_equal(&self, other: &OpenXmlPackage) -> bool {
        crate::opc::packages_equal(self.opc(), other.opc())
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


#[cfg(test)]
mod part_events_tests {
    use super::*;
    use crate::features::PackageEventType;
    use crate::opc::PackUri;
    use crate::namespace::content_type;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn part_events_on_set_and_delete() {
        let mut pkg = OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        let added = Arc::new(AtomicUsize::new(0));
        let removed = Arc::new(AtomicUsize::new(0));
        let a = added.clone();
        let r = removed.clone();
        pkg.part_events().subscribe(move |e| {
            match e.event_type {
                PackageEventType::Added => {
                    a.fetch_add(1, Ordering::SeqCst);
                }
                PackageEventType::Removed => {
                    r.fetch_add(1, Ordering::SeqCst);
                }
                _ => {}
            }
        });
        let uri = PackUri::new("/word/styles.xml");
        pkg.set_part(uri.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        assert_eq!(added.load(Ordering::SeqCst), 1);
        assert!(pkg.delete_part(&uri).is_some());
        assert_eq!(removed.load(Ordering::SeqCst), 1);
    }
}
