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

/// SDK behavioral compatibility level (C# `CompatibilityLevel`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompatibilityLevel {
    /// Latest behavior (C# `Default` / resolves to Version_3_0).
    #[default]
    Default,
    /// Maintain compatibility with Open XML SDK v2.20 where possible.
    Version2_20,
    /// Maintain compatibility with Open XML SDK v3.0 where possible.
    Version3_0,
}

impl CompatibilityLevel {
    /// Effective level (C# getter maps `Default` → `Version_3_0`).
    pub fn effective(self) -> Self {
        match self {
            Self::Default => Self::Version3_0,
            other => other,
        }
    }

    /// Whether this level is at least the given version (Default counts as 3.0).
    pub fn at_least(self, min: CompatibilityLevel) -> bool {
        fn rank(c: CompatibilityLevel) -> u8 {
            match c.effective() {
                CompatibilityLevel::Version2_20 => 1,
                CompatibilityLevel::Version3_0 | CompatibilityLevel::Default => 2,
            }
        }
        rank(self) >= rank(min)
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
    /// Behavioral compatibility with prior SDK major versions (C# `CompatibilityLevel`).
    pub compatibility_level: CompatibilityLevel,
}

impl Default for OpenSettings {
    fn default() -> Self {
        Self {
            auto_save: true,
            max_characters_in_part: 0,
            markup_compatibility: MarkupCompatibilityProcessSettings::default(),
            compression: crate::opc::CompressionOption::Normal,
            compatibility_level: CompatibilityLevel::Default,
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
        let mut pkg = Self {
            opc,
            settings,
            closed: false,
            features: FeatureCollection::new(),
        };
        // Seed PartsFeature + PartUriFeature + DataPartsFeature from existing package parts.
        let uris: Vec<_> = pkg.opc.part_uris();
        {
            let parts = pkg.parts_feature_mut();
            for uri in &uris {
                parts.add(uri.as_str());
            }
        }
        pkg.features.set(crate::features::PartUriFeature::from_helper(
            crate::opc::PartUriHelper::from_package(pkg.opc()),
        ));
        {
            let data_uris: Vec<String> = pkg
                .opc
                .data_parts()
                .iter()
                .map(|p| p.uri.as_str().to_string())
                .collect();
            let dp = pkg.data_parts_feature_mut();
            for u in data_uris {
                dp.add(u);
            }
        }
        // Seed part/reference relationship feature shells from existing .rels.
        {
            let mut part_entries: Vec<(String, String)> = Vec::new();
            let mut ref_entries: Vec<(String, String, String, bool)> = Vec::new();
            for r in pkg.opc.package_relationships().iter() {
                let external =
                    r.target_mode == crate::opc::RelationshipTargetMode::External;
                if external {
                    ref_entries.push((
                        r.id.clone(),
                        r.relationship_type.clone(),
                        r.target.clone(),
                        true,
                    ));
                } else {
                    part_entries.push((r.id.clone(), r.target.clone()));
                }
            }
            let sources = pkg.opc.part_relationship_sources();
            for src in sources {
                if let Some(rels) = pkg.opc.part_relationships(&src) {
                    for r in rels.iter() {
                        let external =
                            r.target_mode == crate::opc::RelationshipTargetMode::External;
                        if external {
                            ref_entries.push((
                                r.id.clone(),
                                r.relationship_type.clone(),
                                r.target.clone(),
                                true,
                            ));
                        } else {
                            part_entries.push((r.id.clone(), r.target.clone()));
                            ref_entries.push((
                                r.id.clone(),
                                r.relationship_type.clone(),
                                r.target.clone(),
                                false,
                            ));
                        }
                    }
                }
            }
            {
                let pr = pkg.part_relationships_feature();
                for (id, target) in part_entries {
                    pr.add(id, target);
                }
            }
            {
                let rr = pkg.reference_relationships_feature();
                for (id, ty, target, ext) in ref_entries {
                    rr.add(id, ty, target, ext);
                }
            }
        }
        pkg
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

    /// Normalize Transitional → Strict namespaces and relationship types.
    pub fn rewrite_transitional_to_strict(&mut self) -> crate::error::Result<(usize, usize)> {
        crate::namespace_rewrite::rewrite_package_to_strict(&mut self.opc)
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

    /// Recursively delete parts of a relationship type (C# `DeletePartsRecursivelyOfType`).
    pub fn delete_parts_recursively_of_relationship_type(
        &mut self,
        relationship_type: &str,
    ) -> usize {
        self.opc
            .delete_parts_recursively_of_relationship_type_all(relationship_type)
    }

    /// Remove a part and raise part Removing/Removed events (C# `DeletePart` + `IPartEventsFeature`).
    pub fn delete_part(&mut self, uri: &crate::opc::PackUri) -> Option<Vec<u8>> {
        let uri_str = uri.to_string();
        self.raise_part_event(crate::features::PackageEventType::Removing, &uri_str);
        self.raise_part_event(crate::features::PackageEventType::Deleting, &uri_str);
        let data = self.opc.remove_part(uri);
        if data.is_some() {
            self.parts_feature_mut().remove(&uri_str);
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
        self.opc.set_part(uri.clone(), content_type, data);
        self.parts_feature_mut().add(&uri_str);
        self.part_uri_feature().reserve(&uri);
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
        let part = self.opc_mut().create_media_data_part(content_type, extension)?;
        self.data_parts_feature_mut().add(part.uri.as_str());
        Ok(part)
    }

    /// Create a media data part pre-filled with bytes.
    pub fn create_media_data_part_with_data(
        &mut self,
        content_type: &str,
        extension: Option<&str>,
        data: impl Into<Vec<u8>>,
    ) -> crate::error::Result<crate::opc::DataPart> {
        let part = self
            .opc_mut()
            .create_media_data_part_with_data(content_type, extension, data)?;
        self.data_parts_feature_mut().add(part.uri.as_str());
        Ok(part)
    }

    /// Registered data parts (C# `DataParts`).
    pub fn data_parts(&self) -> &[crate::opc::DataPart] {
        self.opc().data_parts()
    }

    /// Delete unused (unreferenced) data parts.
    pub fn delete_unused_data_parts(&mut self) -> usize {
        let before: Vec<String> = self
            .opc
            .data_parts()
            .iter()
            .map(|p| p.uri.as_str().to_string())
            .collect();
        let n = self.opc_mut().delete_unused_data_parts();
        if n > 0 {
            let after: std::collections::HashSet<String> = self
                .opc
                .data_parts()
                .iter()
                .map(|p| p.uri.as_str().to_string())
                .collect();
            let feat = self.data_parts_feature_mut();
            for u in before {
                if !after.contains(&u) {
                    feat.remove(&u);
                }
            }
        }
        n
    }

    /// Delete a data part if unreferenced (C# `DeletePart(DataPart)`).
    pub fn delete_data_part(&mut self, uri: &crate::opc::PackUri) -> Result<bool> {
        self.ensure_open()?;
        let ok = self.opc.delete_data_part(uri)?;
        if ok {
            self.data_parts_feature_mut().remove(uri.as_str());
        }
        Ok(ok)
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

    /// Package capability flags (C# `IPackageFeature.Capabilities` shell).
    pub fn package_capabilities(&self) -> crate::features::PackageCapabilities {
        use crate::features::PackageCapabilities;
        let mut c = PackageCapabilities::CACHED;
        if self.can_save() {
            c |= PackageCapabilities::SAVE;
        }
        // In-memory/lazy open always allows reload from bytes/path when path known.
        c |= PackageCapabilities::RELOAD;
        c
    }

    /// Application host type when registered (C# `IApplicationTypeFeature`).
    pub fn application_type(&self) -> crate::features::ApplicationType {
        self.features
            .get::<crate::features::ApplicationType>()
            .copied()
            .unwrap_or(crate::features::ApplicationType::NONE)
    }

    /// Set application host type on the feature bag.
    pub fn set_application_type(&mut self, app: crate::features::ApplicationType) {
        self.features.set(app);
    }

    /// Main part feature metadata when registered (C# `IMainPartFeature`).
    pub fn main_part_feature(&self) -> Option<&crate::features::MainPartFeature> {
        self.features.get::<crate::features::MainPartFeature>()
    }

    /// Register / replace main part feature metadata.
    pub fn set_main_part_feature(&mut self, feature: crate::features::MainPartFeature) {
        self.features.set(feature);
    }

    /// Document type feature when registered (C# `IDocumentTypeFeature` shell).
    pub fn document_type_feature(&self) -> Option<&crate::features::DocumentTypeFeature> {
        self.features.get::<crate::features::DocumentTypeFeature>()
    }

    /// Register document type feature.
    pub fn set_document_type_feature(&mut self, feature: crate::features::DocumentTypeFeature) {
        self.features.set(feature);
    }

    /// Ensure a disposable feature exists (C# `IDisposableFeature`).
    pub fn disposable_feature(&mut self) -> &mut crate::features::DisposableFeature {
        if !self.features.contains::<crate::features::DisposableFeature>() {
            self.features
                .set(crate::features::DisposableFeature::new());
        }
        self.features
            .get_mut::<crate::features::DisposableFeature>()
            .expect("just inserted")
    }

    /// Register a dispose callback run on [`close`](Self::close) (C# `IDisposableFeature.Register`).
    pub fn register_dispose<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.disposable_feature().register(f);
    }

    /// Schema tracking feature (C# `ISchemaTrackingFeature` shell).
    pub fn schema_tracking(&mut self) -> &mut crate::features::SchemaTrackingFeature {
        if !self
            .features
            .contains::<crate::features::SchemaTrackingFeature>()
        {
            self.features
                .set(crate::features::SchemaTrackingFeature::new());
        }
        self.features
            .get_mut::<crate::features::SchemaTrackingFeature>()
            .expect("just inserted")
    }

    /// Strict-namespace observation feature (C# `IStrictNamespaceFeature`).
    pub fn strict_namespace_feature(&self) -> crate::features::StrictNamespaceFeature {
        self.features
            .get::<crate::features::StrictNamespaceFeature>()
            .copied()
            .unwrap_or_else(|| {
                crate::features::StrictNamespaceFeature::new(self.strict_namespace_found())
            })
    }

    /// Record strict-namespace observation on the feature bag.
    pub fn set_strict_namespace_feature(&mut self, found: bool) {
        self.features
            .set(crate::features::StrictNamespaceFeature::new(found));
    }

    /// Relationship filter pipeline (C# `IRelationshipFilterFeature`).
    pub fn relationship_filter(&mut self) -> &crate::features::RelationshipFilterFeature {
        if !self
            .features
            .contains::<crate::features::RelationshipFilterFeature>()
        {
            self.features
                .set(crate::features::RelationshipFilterFeature::new());
        }
        self.features
            .get::<crate::features::RelationshipFilterFeature>()
            .expect("just inserted")
    }

    /// Package factory marker (C# `IPackageFactoryFeature` shell).
    pub fn package_factory_feature(&self) -> Option<&crate::features::PackageFactoryFeature> {
        self.features
            .get::<crate::features::PackageFactoryFeature>()
    }

    pub fn set_package_factory_feature(&mut self, kind: impl Into<String>) {
        self.features
            .set(crate::features::PackageFactoryFeature::new(kind));
    }

    /// Programmatic id generator (C# `IProgrammaticIdentifierFeature`).
    pub fn programmatic_identifier(&mut self) -> &crate::features::ProgrammaticIdentifierFeature {
        if !self
            .features
            .contains::<crate::features::ProgrammaticIdentifierFeature>()
        {
            self.features
                .set(crate::features::ProgrammaticIdentifierFeature::default());
        }
        self.features
            .get::<crate::features::ProgrammaticIdentifierFeature>()
            .expect("just inserted")
    }

    /// Content-type constancy flag (C# `IContentTypeFeature`).
    pub fn content_type_feature(&self) -> crate::features::ContentTypeFeature {
        self.features
            .get::<crate::features::ContentTypeFeature>()
            .copied()
            .unwrap_or_default()
    }

    pub fn set_content_type_feature(&mut self, feature: crate::features::ContentTypeFeature) {
        self.features.set(feature);
    }

    /// Package sync lock (C# `ILockFeature`).
    pub fn lock_feature(&mut self) -> &crate::features::LockFeature {
        if !self.features.contains::<crate::features::LockFeature>() {
            self.features.set(crate::features::LockFeature::new());
        }
        self.features
            .get::<crate::features::LockFeature>()
            .expect("just inserted")
    }

    /// Loaded-parts registry (C# `IPartsFeature` shell).
    pub fn parts_feature(&mut self) -> &crate::features::PartsFeature {
        self.parts_feature_mut()
    }

    fn parts_feature_mut(&mut self) -> &mut crate::features::PartsFeature {
        if !self.features.contains::<crate::features::PartsFeature>() {
            self.features.set(crate::features::PartsFeature::new());
        }
        self.features
            .get_mut::<crate::features::PartsFeature>()
            .expect("just inserted")
    }

    /// Part factory by relationship type (C# `IPartFactoryFeature`).
    pub fn part_factory(&mut self) -> &mut crate::features::PartFactoryFeature {
        if !self
            .features
            .contains::<crate::features::PartFactoryFeature>()
        {
            self.features
                .set(crate::features::PartFactoryFeature::new());
        }
        self.features
            .get_mut::<crate::features::PartFactoryFeature>()
            .expect("just inserted")
    }

    /// Known data-part relationship types (C# `IKnownDataPartFeature`).
    pub fn known_data_part_feature(&mut self) -> &crate::features::KnownDataPartFeature {
        if !self
            .features
            .contains::<crate::features::KnownDataPartFeature>()
        {
            self.features
                .set(crate::features::KnownDataPartFeature::with_defaults());
        }
        self.features
            .get::<crate::features::KnownDataPartFeature>()
            .expect("just inserted")
    }

    /// Whether `relationship_type` is a known data-part relationship.
    pub fn is_known_data_part_relationship(&mut self, relationship_type: &str) -> bool {
        self.known_data_part_feature().is_known(relationship_type)
    }

    /// Package stream bytes when opened from memory (C# `IPackageStreamFeature`).
    pub fn package_stream_feature(&mut self) -> &mut crate::features::PackageStreamFeature {
        if !self
            .features
            .contains::<crate::features::PackageStreamFeature>()
        {
            self.features
                .set(crate::features::PackageStreamFeature::new());
        }
        self.features
            .get_mut::<crate::features::PackageStreamFeature>()
            .expect("just inserted")
    }

    /// Current part URI context (C# `IPackagePartFeature`).
    pub fn package_part_feature(&mut self) -> &mut crate::features::PackagePartFeature {
        if !self
            .features
            .contains::<crate::features::PackagePartFeature>()
        {
            self.features
                .set(crate::features::PackagePartFeature::new());
        }
        self.features
            .get_mut::<crate::features::PackagePartFeature>()
            .expect("just inserted")
    }

    /// Package initializer hooks (C# `IPackageInitializer`).
    pub fn package_initializer(&mut self) -> &crate::features::PackageInitializerFeature {
        if !self
            .features
            .contains::<crate::features::PackageInitializerFeature>()
        {
            self.features
                .set(crate::features::PackageInitializerFeature::new());
        }
        self.features
            .get::<crate::features::PackageInitializerFeature>()
            .expect("just inserted")
    }

    /// Run all registered package initializers and clear them.
    pub fn run_package_initializers(&mut self) {
        if let Some(init) = self
            .features
            .get::<crate::features::PackageInitializerFeature>()
        {
            init.run_all();
        }
    }

    /// Part URI allocator (C# `IPartUriFeature`).
    pub fn part_uri_feature(&mut self) -> &mut crate::features::PartUriFeature {
        if !self.features.contains::<crate::features::PartUriFeature>() {
            let helper = crate::opc::PartUriHelper::from_package(&self.opc);
            self.features
                .set(crate::features::PartUriFeature::from_helper(helper));
        }
        self.features
            .get_mut::<crate::features::PartUriFeature>()
            .expect("just inserted")
    }

    /// Data-parts URI registry (C# `IDataPartsFeature`).
    pub fn data_parts_feature(&mut self) -> &crate::features::DataPartsFeature {
        self.data_parts_feature_mut()
    }

    fn data_parts_feature_mut(&mut self) -> &mut crate::features::DataPartsFeature {
        if !self
            .features
            .contains::<crate::features::DataPartsFeature>()
        {
            self.features
                .set(crate::features::DataPartsFeature::new());
        }
        self.features
            .get_mut::<crate::features::DataPartsFeature>()
            .expect("just inserted")
    }

    /// Part relationship id map shell (C# `IPartRelationshipsFeature`).
    pub fn part_relationships_feature(
        &mut self,
    ) -> &mut crate::features::PartRelationshipsFeature {
        if !self
            .features
            .contains::<crate::features::PartRelationshipsFeature>()
        {
            self.features
                .set(crate::features::PartRelationshipsFeature::new());
        }
        self.features
            .get_mut::<crate::features::PartRelationshipsFeature>()
            .expect("just inserted")
    }

    /// Reference relationship registry shell (C# `IReferenceRelationshipsFeature`).
    pub fn reference_relationships_feature(
        &mut self,
    ) -> &mut crate::features::ReferenceRelationshipsFeature {
        if !self
            .features
            .contains::<crate::features::ReferenceRelationshipsFeature>()
        {
            self.features
                .set(crate::features::ReferenceRelationshipsFeature::new());
        }
        self.features
            .get_mut::<crate::features::ReferenceRelationshipsFeature>()
            .expect("just inserted")
    }

    /// Typed part factory by type name (C# `ITypedPartFactoryFeature`).
    pub fn typed_part_factory_feature(
        &mut self,
    ) -> &mut crate::features::TypedPartFactoryFeature {
        if !self
            .features
            .contains::<crate::features::TypedPartFactoryFeature>()
        {
            self.features
                .set(crate::features::TypedPartFactoryFeature::new());
        }
        self.features
            .get_mut::<crate::features::TypedPartFactoryFeature>()
            .expect("just inserted")
    }

    /// Target path metadata (C# `ITargetFeature`).
    pub fn target_feature(&mut self) -> &mut crate::features::TargetFeature {
        if !self.features.contains::<crate::features::TargetFeature>() {
            self.features.set(crate::features::TargetFeature::default());
        }
        self.features
            .get_mut::<crate::features::TargetFeature>()
            .expect("just inserted")
    }

    pub fn set_target_feature(&mut self, feature: crate::features::TargetFeature) {
        self.features.set(feature);
    }

    /// Root element factory (C# `IRootElementFeature`).
    pub fn root_element_feature(&mut self) -> &mut crate::features::RootElementFeature {
        if !self
            .features
            .contains::<crate::features::RootElementFeature>()
        {
            self.features
                .set(crate::features::RootElementFeature::new());
        }
        self.features
            .get_mut::<crate::features::RootElementFeature>()
            .expect("just inserted")
    }

    /// Save callbacks (C# `ISaveFeature`).
    pub fn save_feature(&mut self) -> &crate::features::SaveFeature {
        if !self.features.contains::<crate::features::SaveFeature>() {
            self.features.set(crate::features::SaveFeature::new());
        }
        self.features
            .get::<crate::features::SaveFeature>()
            .expect("just inserted")
    }

    /// Run registered save hooks for a container URI (empty = package).
    pub fn run_save_hooks(&mut self, container_uri: &str) {
        if let Some(f) = self.features.get::<crate::features::SaveFeature>() {
            f.save(container_uri);
        }
    }

    /// Package feature shell (C# `IPackageFeature`).
    pub fn package_feature(&mut self) -> &mut crate::features::PackageFeature {
        if !self.features.contains::<crate::features::PackageFeature>() {
            let mut f = crate::features::PackageFeature::with_capabilities(self.package_capabilities());
            if let Some(p) = self.opc.path() {
                f.path = Some(p.display().to_string());
            }
            self.features.set(f);
        }
        self.features
            .get_mut::<crate::features::PackageFeature>()
            .expect("just inserted")
    }

    /// Namespace resolver (C# `IOpenXmlNamespaceResolver`).
    pub fn namespace_resolver_feature(
        &mut self,
    ) -> &mut crate::features::OpenXmlNamespaceResolverFeature {
        if !self
            .features
            .contains::<crate::features::OpenXmlNamespaceResolverFeature>()
        {
            self.features
                .set(crate::features::OpenXmlNamespaceResolverFeature::with_defaults());
        }
        self.features
            .get_mut::<crate::features::OpenXmlNamespaceResolverFeature>()
            .expect("just inserted")
    }

    /// Random number generator (C# `IRandomNumberGeneratorFeature`).
    pub fn random_number_generator_feature(
        &mut self,
    ) -> &crate::features::RandomNumberGeneratorFeature {
        if !self
            .features
            .contains::<crate::features::RandomNumberGeneratorFeature>()
        {
            self.features
                .set(crate::features::RandomNumberGeneratorFeature::new());
        }
        self.features
            .get::<crate::features::RandomNumberGeneratorFeature>()
            .expect("just inserted")
    }

    /// Container dispose hooks (C# `IContainerDisposableFeature`).
    pub fn container_disposable_feature(
        &mut self,
    ) -> &crate::features::ContainerDisposableFeature {
        if !self
            .features
            .contains::<crate::features::ContainerDisposableFeature>()
        {
            self.features
                .set(crate::features::ContainerDisposableFeature::new());
        }
        self.features
            .get::<crate::features::ContainerDisposableFeature>()
            .expect("just inserted")
    }

    /// Part element events (C# `IElementEventFeature`).
    pub fn element_events_feature(&mut self) -> &crate::features::ElementEventsFeature {
        if !self
            .features
            .contains::<crate::features::ElementEventsFeature>()
        {
            self.features
                .set(crate::features::ElementEventsFeature::new());
        }
        self.features
            .get::<crate::features::ElementEventsFeature>()
            .expect("just inserted")
    }

    /// Paragraph id generator (C# `IParagraphIdGeneratorFeature`).
    pub fn paragraph_id_generator(&mut self) -> &mut crate::features::ParagraphIdGenerator {
        if !self
            .features
            .contains::<crate::features::ParagraphIdGenerator>()
        {
            self.features
                .set(crate::features::ParagraphIdGenerator::new());
        }
        self.features
            .get_mut::<crate::features::ParagraphIdGenerator>()
            .expect("just inserted")
    }

    /// Paragraph id collection (C# `IParagraphIdCollectionFeature`).
    pub fn paragraph_id_collection(
        &mut self,
    ) -> &mut crate::features::ParagraphIdCollectionFeature {
        if !self
            .features
            .contains::<crate::features::ParagraphIdCollectionFeature>()
        {
            self.features
                .set(crate::features::ParagraphIdCollectionFeature::new());
        }
        self.features
            .get_mut::<crate::features::ParagraphIdCollectionFeature>()
            .expect("just inserted")
    }

    /// Shared feature registry (C# `ISharedFeature` shell).
    pub fn shared_feature_registry(&mut self) -> &mut crate::features::SharedFeatureRegistry {
        if !self
            .features
            .contains::<crate::features::SharedFeatureRegistry>()
        {
            self.features
                .set(crate::features::SharedFeatureRegistry::new());
        }
        self.features
            .get_mut::<crate::features::SharedFeatureRegistry>()
            .expect("just inserted")
    }

    /// Seed generator uniqueness from the paragraph-id collection.
    pub fn sync_paragraph_id_generator_from_collection(&mut self) {
        let ids: Vec<String> = self
            .paragraph_id_collection()
            .iter()
            .map(|s| s.to_string())
            .collect();
        let gen = self.paragraph_id_generator();
        for id in ids {
            gen.register_existing(id);
        }
    }

    /// Record package source bytes on the stream feature (C# open-from-stream path).
    pub fn set_package_stream_bytes(&mut self, bytes: impl Into<Vec<u8>>) {
        self.package_stream_feature().set_bytes(bytes);
    }

    /// Add a package-level relationship after running any registered relationship filters
    /// (C# relationship create + `IRelationshipFilterFeature`).
    pub fn add_package_relationship(
        &mut self,
        relationship_type: &str,
        target: &crate::opc::PackUri,
        target_mode: crate::opc::RelationshipTargetMode,
    ) -> String {
        let mut builder = crate::features::PackageRelationshipBuilder::new(
            "",
            relationship_type,
            target.as_str(),
        )
        .with_target_mode(match target_mode {
            crate::opc::RelationshipTargetMode::Internal => "Internal",
            crate::opc::RelationshipTargetMode::External => "External",
        });
        if let Some(f) = self
            .features
            .get::<crate::features::RelationshipFilterFeature>()
        {
            f.apply(&mut builder);
        }
        let mode = if builder.target_mode.eq_ignore_ascii_case("External") {
            crate::opc::RelationshipTargetMode::External
        } else {
            crate::opc::RelationshipTargetMode::Internal
        };
        let target_uri = crate::opc::PackUri::new(&builder.target);
        let id = self.opc
            .add_package_relationship(&builder.relationship_type, &target_uri, mode);
        if mode == crate::opc::RelationshipTargetMode::Internal {
            self.part_relationships_feature()
                .add(&id, target_uri.as_str());
        } else {
            self.reference_relationships_feature().add(
                &id,
                &builder.relationship_type,
                target_uri.as_str(),
                true,
            );
        }
        id
    }

    /// Add a part→part relationship after relationship filters.
    pub fn add_part_relationship(
        &mut self,
        source: &crate::opc::PackUri,
        relationship_type: &str,
        target: &crate::opc::PackUri,
        target_mode: crate::opc::RelationshipTargetMode,
    ) -> String {
        let mut builder = crate::features::PackageRelationshipBuilder::new(
            "",
            relationship_type,
            target.as_str(),
        )
        .with_target_mode(match target_mode {
            crate::opc::RelationshipTargetMode::Internal => "Internal",
            crate::opc::RelationshipTargetMode::External => "External",
        })
        .with_source_uri(source.as_str());
        if let Some(f) = self
            .features
            .get::<crate::features::RelationshipFilterFeature>()
        {
            f.apply(&mut builder);
        }
        let mode = if builder.target_mode.eq_ignore_ascii_case("External") {
            crate::opc::RelationshipTargetMode::External
        } else {
            crate::opc::RelationshipTargetMode::Internal
        };
        let target_uri = crate::opc::PackUri::new(&builder.target);
        let id = self.opc.add_part_relationship(
            source,
            &builder.relationship_type,
            &target_uri,
            mode,
        );
        if mode == crate::opc::RelationshipTargetMode::Internal {
            self.part_relationships_feature()
                .add(&id, target_uri.as_str());
        } else {
            self.reference_relationships_feature().add(
                &id,
                &builder.relationship_type,
                target_uri.as_str(),
                true,
            );
        }
        id
    }

    /// Add an external relationship after relationship filters
    /// (C# external/hyperlink create + `IReferenceRelationshipsFeature` shell).
    ///
    /// `source` is the part that owns the relationship, or `None` for package-level.
    pub fn add_external_relationship(
        &mut self,
        source: Option<&crate::opc::PackUri>,
        relationship_type: &str,
        target: &str,
    ) -> String {
        let mut builder = crate::features::PackageRelationshipBuilder::new(
            "",
            relationship_type,
            target,
        )
        .with_target_mode("External");
        if let Some(s) = source {
            builder = builder.with_source_uri(s.as_str());
        }
        if let Some(f) = self
            .features
            .get::<crate::features::RelationshipFilterFeature>()
        {
            f.apply(&mut builder);
        }
        let id = self.opc.add_external_relationship(
            source,
            &builder.relationship_type,
            &builder.target,
        );
        self.reference_relationships_feature().add(
            &id,
            &builder.relationship_type,
            &builder.target,
            true,
        );
        id
    }

    /// Delete a reference relationship by id and drop it from feature shells
    /// (C# `DeleteReferenceRelationship`).
    pub fn delete_reference_relationship(
        &mut self,
        source: Option<&crate::opc::PackUri>,
        id: &str,
    ) -> Option<crate::opc::Relationship> {
        let removed = self.opc.delete_reference_relationship(source, id)?;
        self.part_relationships_feature().remove(id);
        self.reference_relationships_feature().remove(id);
        Some(removed)
    }

    /// Get a reference relationship by id (C# `GetReferenceRelationship`).
    pub fn get_reference_relationship(
        &self,
        source: Option<&crate::opc::PackUri>,
        id: &str,
    ) -> Option<crate::opc::ReferenceRelationship> {
        self.opc.get_reference_relationship(source, id)
    }

    /// Child parts as [`IdPartPair`] under `source` (package-level when `None`).
    pub fn id_part_pairs(
        &self,
        source: Option<&crate::opc::PackUri>,
    ) -> Vec<crate::opc::IdPartPair> {
        self.opc.id_part_pairs(source)
    }

    /// Part URI for relationship id (C# `GetPartById`).
    pub fn get_part_by_id(
        &self,
        source: Option<&crate::opc::PackUri>,
        id: &str,
    ) -> Option<crate::opc::PackUri> {
        self.opc.get_part_by_id(source, id)
    }

    /// Relationship id of a related part (C# `GetIdOfPart`).
    pub fn get_id_of_part(
        &self,
        source: Option<&crate::opc::PackUri>,
        part_uri: &crate::opc::PackUri,
    ) -> Option<String> {
        self.opc.get_id_of_part(source, part_uri)
    }

    /// Data-part reference relationships under `source` (package-level when `None`).
    pub fn data_part_reference_relationships(
        &self,
        source: Option<&crate::opc::PackUri>,
    ) -> Vec<crate::opc::DataPartReferenceRelationship> {
        self.opc.data_part_reference_relationships(source)
    }

    /// Hyperlink relationships under `source` (package-level when `None`).
    pub fn hyperlink_relationships(
        &self,
        source: Option<&crate::opc::PackUri>,
    ) -> Vec<crate::opc::HyperlinkRelationship> {
        self.opc.hyperlink_relationships(source)
    }

    /// Create a relationship from `source` to an existing internal part (C#
    /// `CreateRelationshipToPart`), applying relationship filters and tracking
    /// feature bags.
    pub fn create_relationship_to_part(
        &mut self,
        source: &crate::opc::PackUri,
        target: &crate::opc::PackUri,
        relationship_type: &str,
        id: Option<&str>,
    ) -> crate::error::Result<String> {
        if !self.opc.has_part(target) {
            return Err(crate::error::Error::PartNotFound(target.to_string()));
        }
        if let Some(existing) = self.opc.get_id_of_part(Some(source), target) {
            if let Some(want) = id {
                if existing != want {
                    return Err(crate::error::Error::Package(format!(
                        "part already related as `{existing}`, not `{want}`"
                    )));
                }
            }
            return Ok(existing);
        }
        // Filtered create with optional fixed id via opc helper when id is Some.
        let mut builder = crate::features::PackageRelationshipBuilder::new(
            id.unwrap_or(""),
            relationship_type,
            target.as_str(),
        )
        .with_target_mode("Internal")
        .with_source_uri(source.as_str());
        if let Some(f) = self
            .features
            .get::<crate::features::RelationshipFilterFeature>()
        {
            f.apply(&mut builder);
        }
        let target_uri = crate::opc::PackUri::new(&builder.target);
        let rid = if builder.id.is_empty() {
            self.opc.add_part_relationship(
                source,
                &builder.relationship_type,
                &target_uri,
                crate::opc::RelationshipTargetMode::Internal,
            )
        } else {
            self.opc.create_relationship_to_part(
                source,
                &target_uri,
                &builder.relationship_type,
                Some(builder.id.as_str()),
            )
        };
        self.part_relationships_feature()
            .add(&rid, target_uri.as_str());
        Ok(rid)
    }

    /// Change the relationship id of an existing relationship (C# `ChangeIdOfPart`)
    /// and keep feature shells in sync.
    pub fn change_id_of_part(
        &mut self,
        source: Option<&crate::opc::PackUri>,
        part_uri: &crate::opc::PackUri,
        new_id: &str,
    ) -> crate::error::Result<String> {
        let old_id = self.opc.change_id_of_part(source, part_uri, new_id)?;
        if old_id != new_id {
            if self.part_relationships_feature().remove(&old_id) {
                self.part_relationships_feature()
                    .add(new_id, part_uri.as_str());
            }
            if let Some((rel_type, target, external)) = self
                .reference_relationships_feature()
                .try_get(&old_id)
                .map(|(t, tgt, e)| (t.to_string(), tgt.to_string(), e))
            {
                self.reference_relationships_feature().remove(&old_id);
                self.reference_relationships_feature().add(
                    new_id,
                    rel_type,
                    target,
                    external,
                );
            }
        }
        Ok(old_id)
    }

    /// Add a data-part reference relationship (C# `AddDataPartReferenceRelationship`) and
    /// track it on part/reference relationship feature shells.
    pub fn add_data_part_reference_relationship(
        &mut self,
        source: &crate::opc::PackUri,
        data_part: &crate::opc::DataPart,
        relationship_type: &str,
        id: Option<&str>,
    ) -> crate::error::Result<crate::opc::DataPartReferenceRelationship> {
        let mut builder = crate::features::PackageRelationshipBuilder::new(
            id.unwrap_or(""),
            relationship_type,
            data_part.uri.as_str(),
        )
        .with_target_mode("Internal")
        .with_source_uri(source.as_str());
        if let Some(f) = self
            .features
            .get::<crate::features::RelationshipFilterFeature>()
        {
            f.apply(&mut builder);
        }
        let filtered_type = builder.relationship_type.clone();
        let r = self.opc.add_data_part_reference_relationship(
            source,
            data_part,
            &filtered_type,
            if builder.id.is_empty() {
                None
            } else {
                Some(builder.id.as_str())
            },
        )?;
        let rid = r.id().to_string();
        self.part_relationships_feature()
            .add(&rid, data_part.uri.as_str());
        self.reference_relationships_feature().add(
            &rid,
            &filtered_type,
            data_part.uri.as_str(),
            false,
        );
        self.data_parts_feature_mut().add(data_part.uri.as_str());
        Ok(r)
    }

    /// Add a hyperlink relationship (C# `AddHyperlinkRelationship`) via filters +
    /// `IReferenceRelationshipsFeature`.
    pub fn add_hyperlink_relationship(
        &mut self,
        source: &crate::opc::PackUri,
        target: &str,
        is_external: bool,
    ) -> String {
        let mode = if is_external {
            crate::opc::RelationshipTargetMode::External
        } else {
            crate::opc::RelationshipTargetMode::Internal
        };
        let mut builder = crate::features::PackageRelationshipBuilder::new(
            "",
            crate::namespace::rel::HYPERLINK,
            target,
        )
        .with_target_mode(match mode {
            crate::opc::RelationshipTargetMode::Internal => "Internal",
            crate::opc::RelationshipTargetMode::External => "External",
        })
        .with_source_uri(source.as_str());
        if let Some(f) = self
            .features
            .get::<crate::features::RelationshipFilterFeature>()
        {
            f.apply(&mut builder);
        }
        let mode = if builder.target_mode.eq_ignore_ascii_case("External") {
            crate::opc::RelationshipTargetMode::External
        } else {
            crate::opc::RelationshipTargetMode::Internal
        };
        let id = self.opc.add_hyperlink_relationship(
            source,
            &builder.target,
            mode == crate::opc::RelationshipTargetMode::External,
        );
        // Re-apply relationship type if a filter rewrote it (opc helper always uses HYPERLINK).
        if builder.relationship_type != crate::namespace::rel::HYPERLINK {
            // Best-effort: record filtered type on the reference feature only.
        }
        if mode == crate::opc::RelationshipTargetMode::External {
            self.reference_relationships_feature().add(
                &id,
                &builder.relationship_type,
                &builder.target,
                true,
            );
        } else {
            self.part_relationships_feature()
                .add(&id, builder.target.as_str());
            self.reference_relationships_feature().add(
                &id,
                &builder.relationship_type,
                &builder.target,
                false,
            );
        }
        id
    }

    /// File access mode (C# `OpenXmlPackage.FileOpenAccess`).
    pub fn file_open_access(&self) -> crate::opc::FileOpenAccess {
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
        if let Some(d) = self.features.get_mut::<crate::features::DisposableFeature>() {
            d.dispose_all();
        }
        if let Some(d) = self
            .features
            .get::<crate::features::ContainerDisposableFeature>()
        {
            d.dispose();
        }
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
        self.run_save_hooks("");
        self.raise_package_event(crate::features::PackageEventType::Saving);
        self.opc.save()?;
        self.raise_package_event(crate::features::PackageEventType::Saved);
        Ok(())
    }

    /// Save the package to a new path.
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.ensure_open()?;
        self.run_save_hooks("");
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

    #[test]
    fn feed_data_and_save_to_package_raise_part_events() {
        use crate::packaging::OpenXmlPart;
        use crate::namespace::rel;

        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        let added = Arc::new(AtomicUsize::new(0));
        let a = added.clone();
        pkg.part_events().subscribe(move |e| {
            if e.event_type == PackageEventType::Added {
                a.fetch_add(1, Ordering::SeqCst);
            }
        });
        let uri = PackUri::new("/word/document.xml");
        let mut part = OpenXmlPart::new(
            uri.clone(),
            content_type::WORD_DOCUMENT,
            rel::OFFICE_DOCUMENT,
        );
        part.feed_data(&mut pkg, b"<w:document/>".to_vec());
        assert_eq!(added.load(Ordering::SeqCst), 1);
        part.set_root(crate::element::OpenXmlElement::new(
            "w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "document",
        ));
        part.save_to_package(&mut pkg).unwrap();
        assert!(added.load(Ordering::SeqCst) >= 2);
    }

    #[test]
    fn compatibility_level_effective_and_at_least() {
        assert_eq!(
            CompatibilityLevel::Default.effective(),
            CompatibilityLevel::Version3_0
        );
        assert!(CompatibilityLevel::Default.at_least(CompatibilityLevel::Version3_0));
        assert!(CompatibilityLevel::Version3_0.at_least(CompatibilityLevel::Version2_20));
        assert!(!CompatibilityLevel::Version2_20.at_least(CompatibilityLevel::Version3_0));
        let s = OpenSettings::default();
        assert_eq!(s.compatibility_level, CompatibilityLevel::Default);
    }

    #[test]
    fn application_type_and_dispose_on_close() {
        use crate::features::ApplicationType;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        assert_eq!(pkg.application_type(), ApplicationType::NONE);
        pkg.set_application_type(ApplicationType::WORD);
        assert_eq!(pkg.application_type(), ApplicationType::WORD);
        pkg.set_main_part_feature(crate::features::MainPartFeature::new(
            "rel",
            "ct",
            Some("/word/document.xml".into()),
        ));
        assert_eq!(
            pkg.main_part_feature().and_then(|m| m.part_uri.clone()).as_deref(),
            Some("/word/document.xml")
        );

        let fired = Arc::new(AtomicBool::new(false));
        let f = fired.clone();
        pkg.register_dispose(move || f.store(true, Ordering::SeqCst));
        pkg.close(false).unwrap();
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn relationship_filter_and_factory_feature() {
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        pkg.set_package_factory_feature("WordprocessingDocument");
        assert_eq!(
            pkg.package_factory_feature()
                .map(|f| f.package_kind.as_str()),
            Some("WordprocessingDocument")
        );
        pkg.relationship_filter().add_filter(|b| {
            b.target = b.target.replace("styles", "styles2");
        });
        let mut b = crate::features::PackageRelationshipBuilder::new(
            "rId1",
            "rel",
            "/word/styles.xml",
        );
        pkg.relationship_filter().apply(&mut b);
        assert_eq!(b.target, "/word/styles2.xml");
        let id1 = pkg.programmatic_identifier().next_id();
        let id2 = pkg.programmatic_identifier().next_id();
        assert_ne!(id1, id2);
        assert!(id1.starts_with('R'));
    }

    #[test]
    fn add_package_relationship_runs_filters() {
        use crate::namespace::rel;
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        pkg.relationship_filter().add_filter(|b| {
            if b.relationship_type.contains("officeDocument") {
                b.relationship_type = "http://filtered/officeDocument".into();
            }
        });
        let uri = PackUri::new("/word/document.xml");
        pkg.set_part(uri.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        let id = pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &uri,
            crate::opc::RelationshipTargetMode::Internal,
        );
        assert!(!id.is_empty());
        let rels: Vec<_> = pkg.opc().package_relationships().iter().collect();
        assert!(rels
            .iter()
            .any(|r| r.relationship_type == "http://filtered/officeDocument"));
    }

    #[test]
    fn parts_feature_tracks_set_and_delete() {
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        let uri = PackUri::new("/word/styles.xml");
        pkg.set_part(uri.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        assert!(pkg.parts_feature().contains(uri.as_str()));
        assert!(pkg.delete_part(&uri).is_some());
        assert!(!pkg.parts_feature().contains(uri.as_str()));
        pkg.set_content_type_feature(crate::features::ContentTypeFeature::constant());
        assert!(pkg.content_type_feature().is_constant);
        let ran = Arc::new(AtomicUsize::new(0));
        let r = ran.clone();
        pkg.lock_feature().with_lock(|| {
            r.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(ran.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn package_stream_part_initializer_accessors() {
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        pkg.package_stream_feature().set_bytes(b"abc");
        assert_eq!(
            pkg.package_stream_feature().bytes.as_deref(),
            Some(&b"abc"[..])
        );
        pkg.package_part_feature()
            .set_uri("/word/document.xml");
        assert_eq!(
            pkg.package_part_feature().part_uri.as_deref(),
            Some("/word/document.xml")
        );
        let ran = Arc::new(AtomicUsize::new(0));
        let r = ran.clone();
        pkg.package_initializer().register(move || {
            r.fetch_add(1, Ordering::SeqCst);
        });
        pkg.run_package_initializers();
        assert_eq!(ran.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn from_opc_seeds_parts_and_part_uri_features() {
        use crate::namespace::rel;
        use crate::opc::RelationshipTargetMode;
        let mut opc = crate::opc::OpcPackage::create();
        let uri = PackUri::new("/word/document.xml");
        opc.set_part(
            uri.clone(),
            content_type::WORD_DOCUMENT,
            b"<w:document/>".to_vec(),
        );
        opc.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let mut pkg = OpenXmlPackage::from_opc(opc, OpenSettings::default());
        assert!(pkg.parts_feature().contains(uri.as_str()));
        assert!(pkg.part_uri_feature().is_reserved(&uri));
        assert!(!pkg.part_relationships_feature().is_empty());
    }

    #[test]
    fn relationship_and_data_part_features_track_adds() {
        use crate::namespace::rel;
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        pkg.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        let id = pkg.add_part_relationship(
            &doc,
            rel::STYLES,
            &styles,
            crate::opc::RelationshipTargetMode::Internal,
        );
        assert!(pkg.part_relationships_feature().contains_id(&id));
        assert_eq!(
            pkg.part_relationships_feature().try_get(&id),
            Some(styles.as_str())
        );

        let ext_id = pkg.add_external_relationship(
            Some(&doc),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
            "https://example.com",
        );
        assert!(pkg.reference_relationships_feature().contains(&ext_id));
        assert_eq!(
            pkg.reference_relationships_feature().try_get(&ext_id).map(|t| t.2),
            Some(true)
        );

        pkg.typed_part_factory_feature()
            .register("StylesPart", rel::STYLES);
        assert_eq!(
            pkg.typed_part_factory_feature().create("StylesPart"),
            Some(rel::STYLES)
        );

        let media = pkg
            .create_media_data_part("image/png", Some("png"))
            .expect("media");
        assert!(pkg.data_parts_feature().contains(media.uri.as_str()));
        assert!(pkg.delete_data_part(&media.uri).unwrap());
        assert!(!pkg.data_parts_feature().contains(media.uri.as_str()));
    }

    #[test]
    fn target_root_save_package_feature_accessors() {
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        pkg.set_target_feature(crate::features::TargetFeature::new("/word", "xml", "document"));
        assert_eq!(pkg.target_feature().name, "document");
        pkg.root_element_feature().register(
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "document",
            "Document",
        );
        assert_eq!(
            pkg.root_element_feature().try_create(
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "document"
            ),
            Some("Document")
        );
        let n = Arc::new(AtomicUsize::new(0));
        let c = n.clone();
        pkg.save_feature().register(move |_uri| {
            c.fetch_add(1, Ordering::SeqCst);
        });
        pkg.run_save_hooks("/word/document.xml");
        assert_eq!(n.load(Ordering::SeqCst), 1);
        assert!(pkg
            .package_feature()
            .capabilities
            .contains(crate::features::PackageCapabilities::CACHED));
        pkg.package_feature().reload();
        assert_eq!(pkg.package_feature().reload_count, 1);
    }

    #[test]
    fn namespace_random_element_feature_accessors() {
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        assert_eq!(
            pkg.namespace_resolver_feature().get_version(
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            ),
            crate::file_format::FileFormatVersions::OFFICE2007
        );
        let mut buf = [0u8; 8];
        pkg.random_number_generator_feature().get_bytes(&mut buf);
        assert_ne!(buf, [0u8; 8]);
        let n = Arc::new(AtomicUsize::new(0));
        let c = n.clone();
        pkg.element_events_feature().subscribe(move |e| {
            if e.part_uri == "/word/document.xml" {
                c.fetch_add(1, Ordering::SeqCst);
            }
        });
        pkg.element_events_feature().raise_kind(
            PackageEventType::Added,
            "/word/document.xml",
            "w:r",
            None,
        );
        assert_eq!(n.load(Ordering::SeqCst), 1);
        let d = Arc::new(AtomicUsize::new(0));
        let dd = d.clone();
        pkg.container_disposable_feature().register(move || {
            dd.fetch_add(1, Ordering::SeqCst);
        });
        pkg.container_disposable_feature().dispose();
        assert_eq!(d.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn paragraph_id_and_shared_feature_accessors() {
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        pkg.paragraph_id_collection().add("00000001");
        pkg.sync_paragraph_id_generator_from_collection();
        let id = pkg.paragraph_id_generator().create_unique_paragraph_id();
        assert_ne!(id, "00000001");
        assert!(pkg.paragraph_id_generator().contains(&id));
        pkg.shared_feature_registry().add("ParagraphId");
        assert_eq!(pkg.shared_feature_registry().count(), 1);
    }

    #[test]
    fn add_hyperlink_relationship_tracks_reference_feature() {
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        let doc = PackUri::new("/word/document.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        pkg.relationship_filter().add_filter(|b| {
            if b.relationship_type.contains("hyperlink") {
                b.target = format!("{}?filtered=1", b.target);
            }
        });
        let id = pkg.add_hyperlink_relationship(&doc, "https://example.com", true);
        assert!(!id.is_empty());
        let got = pkg.reference_relationships_feature().try_get(&id);
        assert!(got.is_some());
        assert_eq!(got.unwrap().1, "https://example.com?filtered=1");
        let hls = pkg.hyperlink_relationships(Some(&doc));
        assert_eq!(hls.len(), 1);
        assert_eq!(hls[0].target(), "https://example.com?filtered=1");
    }

    #[test]
    fn add_data_part_reference_tracks_features() {
        use crate::opc::media_rel;
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        let doc = PackUri::new("/word/document.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        let media = pkg
            .create_media_data_part("audio/mpeg", Some("mp3"))
            .expect("media");
        let r = pkg
            .add_data_part_reference_relationship(&doc, &media, media_rel::AUDIO, None)
            .expect("ref");
        assert!(pkg.part_relationships_feature().contains_id(r.id()));
        assert!(pkg.reference_relationships_feature().contains(r.id()));
        assert!(pkg.data_parts_feature().contains(media.uri.as_str()));

        assert!(pkg
            .delete_reference_relationship(Some(&doc), r.id())
            .is_some());
        assert!(!pkg.part_relationships_feature().contains_id(r.id()));
        assert!(!pkg.reference_relationships_feature().contains(r.id()));
    }

    #[test]
    fn create_relationship_to_part_and_change_id_track_features() {
        use crate::namespace::rel;
        let mut pkg =
            OpenXmlPackage::from_opc(crate::opc::OpcPackage::create(), OpenSettings::default());
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        pkg.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            b"<w:document/>".to_vec(),
        );
        pkg.set_part(
            styles.clone(),
            content_type::WORD_STYLES,
            b"<w:styles/>".to_vec(),
        );
        let id = pkg
            .create_relationship_to_part(&doc, &styles, rel::STYLES, None)
            .expect("create rel");
        assert!(pkg.part_relationships_feature().contains_id(&id));
        assert_eq!(
            pkg.part_relationships_feature().try_get(&id),
            Some(styles.as_str())
        );
        assert_eq!(pkg.get_id_of_part(Some(&doc), &styles).as_deref(), Some(id.as_str()));
        assert_eq!(
            pkg.get_part_by_id(Some(&doc), &id).as_ref(),
            Some(&styles)
        );
        let pairs = pkg.id_part_pairs(Some(&doc));
        assert!(pairs.iter().any(|p| p.relationship_id == id && p.part_uri == styles));

        let old = pkg
            .change_id_of_part(Some(&doc), &styles, "rIdStyles")
            .expect("change id");
        assert_eq!(old, id);
        assert!(!pkg.part_relationships_feature().contains_id(&id));
        assert!(pkg.part_relationships_feature().contains_id("rIdStyles"));
        assert_eq!(
            pkg.part_relationships_feature().try_get("rIdStyles"),
            Some(styles.as_str())
        );
        let got = pkg
            .get_reference_relationship(Some(&doc), "rIdStyles")
            .expect("ref");
        assert_eq!(got.id, "rIdStyles");
    }
}
