//! Data parts and data-part reference relationships (C# `DataPart` / `MediaDataPart` /
//! `DataPartReferenceRelationship`).

use super::media::{media_rel, MediaKind};
use super::reference_relationship::ReferenceRelationship;
use super::uri::PackUri;
use super::{OpcPackage, Relationship, RelationshipTargetMode};
use crate::error::{Error, Result};

/// Handle for a package-level data/media part (C# `DataPart` / `MediaDataPart`).
///
/// Unlike ordinary Open XML parts, data parts are not part of the part-relationship
/// tree until a [`DataPartReferenceRelationship`] points at them. They live under
/// `/media/` (or a custom URI) and are tracked in the package data-part set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPart {
    pub uri: PackUri,
    pub content_type: String,
    /// Optional media kind when created via media helpers.
    pub kind: Option<MediaKind>,
}

impl DataPart {
    pub fn new(uri: impl Into<PackUri>, content_type: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            content_type: content_type.into(),
            kind: None,
        }
    }

    pub fn with_kind(mut self, kind: MediaKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn uri(&self) -> &PackUri {
        &self.uri
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn kind(&self) -> Option<MediaKind> {
        self.kind
    }

    /// Whether this is a media (audio/video/media) data part (C# `MediaDataPart` role).
    pub fn is_media_data_part(&self) -> bool {
        matches!(
            self.kind,
            Some(MediaKind::Audio | MediaKind::Video | MediaKind::Media)
        )
    }

    /// Default target path segment for media data parts (C# `MediaDataPart.TargetPath`).
    pub const DEFAULT_MEDIA_TARGET_PATH: &'static str = "media";
    /// Default target name (C# `MediaDataPart.TargetName`).
    pub const DEFAULT_MEDIA_TARGET_NAME: &'static str = "mediadata";
    /// Default target extension (C# `MediaDataPart.TargetFileExtension`).
    pub const DEFAULT_MEDIA_TARGET_EXT: &'static str = ".bin";
    /// Default target path for generic data parts (C# `DataPart.TargetPath`).
    pub const DEFAULT_DATA_TARGET_PATH: &'static str = "data";
    pub const DEFAULT_DATA_TARGET_NAME: &'static str = "data";
    pub const DEFAULT_DATA_TARGET_EXT: &'static str = ".bin";
}

/// Type alias for media-focused data parts (C# `MediaDataPart`).
pub type MediaDataPart = DataPart;

/// Internal reference to a [`DataPart`] (C# `DataPartReferenceRelationship`).
///
/// Covers audio / video / media relationship types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPartReferenceRelationship {
    pub inner: ReferenceRelationship,
    pub data_part_uri: PackUri,
}

impl DataPartReferenceRelationship {
    pub fn is_data_part_relationship_type(relationship_type: &str) -> bool {
        matches!(
            relationship_type,
            media_rel::AUDIO | media_rel::VIDEO | media_rel::MEDIA
        )
    }

    pub fn from_relationship(rel: &Relationship, data_part_uri: PackUri) -> Option<Self> {
        if !Self::is_data_part_relationship_type(&rel.relationship_type) {
            return None;
        }
        Some(Self {
            inner: ReferenceRelationship::from_relationship(rel),
            data_part_uri,
        })
    }

    pub fn id(&self) -> &str {
        &self.inner.id
    }

    pub fn relationship_type(&self) -> &str {
        &self.inner.relationship_type
    }

    pub fn data_part_uri(&self) -> &PackUri {
        &self.data_part_uri
    }

    pub fn is_audio(&self) -> bool {
        self.inner.relationship_type == media_rel::AUDIO
    }

    pub fn is_video(&self) -> bool {
        self.inner.relationship_type == media_rel::VIDEO
    }

    pub fn is_media(&self) -> bool {
        self.inner.relationship_type == media_rel::MEDIA
    }
}

/// (RelationshipId, part URI) pair — C# `IdPartPair`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdPartPair {
    pub relationship_id: String,
    pub part_uri: PackUri,
}

impl IdPartPair {
    pub fn new(relationship_id: impl Into<String>, part_uri: PackUri) -> Self {
        Self {
            relationship_id: relationship_id.into(),
            part_uri,
        }
    }

    /// C# `IdPartPair.RelationshipId`.
    pub fn relationship_id(&self) -> &str {
        &self.relationship_id
    }

    /// C# `IdPartPair.OpenXmlPart` URI shell (Rust stores the part URI).
    pub fn part_uri(&self) -> &PackUri {
        &self.part_uri
    }

    /// Alias matching C# property name when only the URI is available.
    pub fn open_xml_part_uri(&self) -> &PackUri {
        &self.part_uri
    }
}

impl From<&super::part_uri::RelatedPart> for IdPartPair {
    fn from(p: &super::part_uri::RelatedPart) -> Self {
        Self {
            relationship_id: p.id.clone(),
            part_uri: p.uri.clone(),
        }
    }
}

fn extension_for_content_type(content_type: &str) -> String {
    let p = super::part_extension::PartExtensionProvider::with_known_extensions();
    // strip leading dot for URI construction
    p.extension_or_bin(content_type)
        .trim_start_matches('.')
        .to_string()
}

fn kind_for_content_type(content_type: &str) -> MediaKind {
    if content_type.starts_with("audio/") {
        MediaKind::Audio
    } else if content_type.starts_with("video/") {
        MediaKind::Video
    } else if content_type.starts_with("image/") {
        MediaKind::Image
    } else {
        MediaKind::Media
    }
}

impl OpcPackage {
    /// Create an empty media data part (C# `CreateMediaDataPart(contentType)`).
    ///
    /// The part is registered in the data-part set but has no inbound relationship
    /// until [`add_data_part_reference_relationship`](Self::add_data_part_reference_relationship).
    pub fn create_media_data_part(
        &mut self,
        content_type: &str,
        extension: Option<&str>,
    ) -> Result<DataPart> {
        let ext = extension
            .map(|e| e.trim_start_matches('.'))
            .map(|e| e.to_string())
            .unwrap_or_else(|| extension_for_content_type(content_type));
        let kind = kind_for_content_type(content_type);
        let mut index = 1u32;
        let uri = loop {
            let candidate = PackUri::new(format!("/media/data{index}.{ext}"));
            if !self.has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        self.set_part(uri.clone(), content_type, Vec::new());
        let part = DataPart::new(uri, content_type).with_kind(kind);
        self.register_data_part(part.clone());
        Ok(part)
    }

    /// Create a media data part and feed bytes immediately.
    pub fn create_media_data_part_with_data(
        &mut self,
        content_type: &str,
        extension: Option<&str>,
        data: impl Into<Vec<u8>>,
    ) -> Result<DataPart> {
        let mut part = self.create_media_data_part(content_type, extension)?;
        self.feed_data_part(&part.uri, data)?;
        // refresh content type from package (unchanged) — keep kind
        if let Some(ct) = self
            .content_types()
            .content_type_for(part.uri.as_str())
            .map(|s| s.to_string())
        {
            part.content_type = ct;
        }
        Ok(part)
    }

    /// Register an existing part URI as a data part (used after open/load).
    pub fn register_data_part(&mut self, part: DataPart) {
        if !self.data_parts.iter().any(|p| p.uri == part.uri) {
            self.data_parts.push(part);
        }
    }

    /// All registered data parts (C# `OpenXmlPackage.DataParts`).
    pub fn data_parts(&self) -> &[DataPart] {
        &self.data_parts
    }

    /// Look up a registered data part by URI.
    pub fn get_data_part(&self, uri: &PackUri) -> Option<&DataPart> {
        self.data_parts.iter().find(|p| &p.uri == uri)
    }

    /// Feed / replace the byte content of a data part (C# `DataPart.FeedData`).
    pub fn feed_data_part(&mut self, uri: &PackUri, data: impl Into<Vec<u8>>) -> Result<()> {
        if !self.has_part(uri) {
            return Err(Error::Package(format!(
                "data part `{}` not found",
                uri.as_str()
            )));
        }
        let ct = self
            .content_types()
            .content_type_for(uri.as_str())
            .unwrap_or("application/octet-stream")
            .to_string();
        self.set_part(uri.clone(), ct, data.into());
        Ok(())
    }

    /// Read data-part bytes (C# `DataPart.GetStream` read).
    pub fn data_part_bytes(&self, uri: &PackUri) -> Option<&[u8]> {
        self.get_part(uri)
    }

    /// Owned copy of data-part bytes (C# `DataPart.GetStream` read shell).
    pub fn get_data_part_stream(&self, uri: &PackUri) -> Result<Vec<u8>> {
        self.data_part_bytes(uri)
            .map(|b| b.to_vec())
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))
    }

    /// Replace data-part content (C# `DataPart.FeedData` shell).
    pub fn feed_data_part_stream(
        &mut self,
        uri: &PackUri,
        data: impl Into<Vec<u8>>,
    ) -> Result<()> {
        self.feed_data_part(uri, data)
    }

    /// Add a data-part reference relationship from `source` to an existing data part
    /// (C# `AddDataPartReferenceRelationship`).
    pub fn add_data_part_reference_relationship(
        &mut self,
        source: &PackUri,
        data_part: &DataPart,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<DataPartReferenceRelationship> {
        if !DataPartReferenceRelationship::is_data_part_relationship_type(relationship_type) {
            return Err(Error::Package(format!(
                "relationship type `{relationship_type}` is not a data-part reference"
            )));
        }
        if !self.has_part(&data_part.uri) {
            return Err(Error::Package(format!(
                "data part `{}` not in package",
                data_part.uri.as_str()
            )));
        }
        // Ensure registered.
        if self.get_data_part(&data_part.uri).is_none() {
            self.register_data_part(data_part.clone());
        }
        let rid = if let Some(id) = id {
            self.part_relationships_mut(source)
                .add_with_id(
                    id,
                    relationship_type,
                    super::uri::relativize(source, &data_part.uri),
                    RelationshipTargetMode::Internal,
                )
                .id
                .clone()
        } else {
            self.add_part_relationship(
                source,
                relationship_type,
                &data_part.uri,
                RelationshipTargetMode::Internal,
            )
        };
        Ok(DataPartReferenceRelationship {
            inner: ReferenceRelationship {
                id: rid,
                relationship_type: relationship_type.to_string(),
                target: super::uri::relativize(source, &data_part.uri),
                is_external: false,
            },
            data_part_uri: data_part.uri.clone(),
        })
    }

    /// Data-part reference relationships from `source` (package-level if `None`).
    pub fn data_part_reference_relationships(
        &self,
        source: Option<&PackUri>,
    ) -> Vec<DataPartReferenceRelationship> {
        let rels: Vec<&Relationship> = match source {
            Some(s) => self
                .part_relationships(s)
                .map(|r| r.iter().collect())
                .unwrap_or_default(),
            None => self.package_relationships().iter().collect(),
        };
        let mut out = Vec::new();
        for rel in rels {
            if !DataPartReferenceRelationship::is_data_part_relationship_type(
                &rel.relationship_type,
            ) {
                continue;
            }
            if rel.target_mode != RelationshipTargetMode::Internal {
                continue;
            }
            let Ok(uri) = self.resolve_relationship(source, rel) else {
                continue;
            };
            if let Some(dpr) = DataPartReferenceRelationship::from_relationship(rel, uri) {
                out.push(dpr);
            }
        }
        out
    }

    /// All data-part reference relationships across the whole package that point at
    /// `data_part_uri` (C# `DataPart.GetDataPartReferenceRelationships`).
    pub fn find_data_part_references(
        &self,
        data_part_uri: &PackUri,
    ) -> Vec<(Option<PackUri>, DataPartReferenceRelationship)> {
        let mut out = Vec::new();
        for dpr in self.data_part_reference_relationships(None) {
            if &dpr.data_part_uri == data_part_uri {
                out.push((None, dpr));
            }
        }
        // Every part that has relationships.
        let sources: Vec<PackUri> = self.part_relationship_sources();
        for src in sources {
            for dpr in self.data_part_reference_relationships(Some(&src)) {
                if &dpr.data_part_uri == data_part_uri {
                    out.push((Some(src.clone()), dpr));
                }
            }
        }
        out
    }

    /// Delete a data part only if no references remain (C# `DeletePart(DataPart)`).
    ///
    /// Returns `true` when removed. Errors if still referenced.
    pub fn delete_data_part(&mut self, uri: &PackUri) -> Result<bool> {
        if self.get_data_part(uri).is_none() && !self.has_part(uri) {
            return Ok(false);
        }
        let refs = self.find_data_part_references(uri);
        if !refs.is_empty() {
            return Err(Error::Package(format!(
                "data part `{}` is still referenced ({} refs)",
                uri.as_str(),
                refs.len()
            )));
        }
        self.data_parts.retain(|p| &p.uri != uri);
        self.remove_part(uri);
        Ok(true)
    }

    /// Delete all data parts that have no remaining references
    /// (C# `DeleteUnusedDataPartOnClose` shell).
    pub fn delete_unused_data_parts(&mut self) -> usize {
        let uris: Vec<PackUri> = self.data_parts.iter().map(|p| p.uri.clone()).collect();
        let mut n = 0;
        for uri in uris {
            if self.find_data_part_references(&uri).is_empty() {
                self.data_parts.retain(|p| p.uri != uri);
                if self.remove_part(&uri).is_some() {
                    n += 1;
                }
            }
        }
        n
    }

    /// Child parts as [`IdPartPair`] (C# `OpenXmlPartContainer.Parts`).
    pub fn id_part_pairs(&self, source: Option<&PackUri>) -> Vec<IdPartPair> {
        self.related_parts(source, None)
            .iter()
            .map(IdPartPair::from)
            .collect()
    }

    /// Get a typed reference relationship by id (C# `GetReferenceRelationship`).
    pub fn get_reference_relationship(
        &self,
        source: Option<&PackUri>,
        id: &str,
    ) -> Option<ReferenceRelationship> {
        let rel = match source {
            Some(s) => self.part_relationships(s)?.get(id)?,
            None => self.package_relationships().get(id)?,
        };
        Some(ReferenceRelationship::from_relationship(rel))
    }

    /// Delete a reference relationship by id (C# `DeleteReferenceRelationship`).
    ///
    /// Alias of [`delete_relationship`](Self::delete_relationship) with a clearer name.
    pub fn delete_reference_relationship(
        &mut self,
        source: Option<&PackUri>,
        id: &str,
    ) -> Option<Relationship> {
        self.delete_relationship(source, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::content_type;
    use crate::opc::media::media_rel;

    #[test]
    fn create_feed_reference_delete_data_part() {
        let mut pkg = OpcPackage::create();
        let slide = PackUri::new("/ppt/slides/slide1.xml");
        pkg.set_part(
            slide.clone(),
            content_type::PRESENTATION_SLIDE,
            b"<p:sld/>".to_vec(),
        );

        let part = pkg
            .create_media_data_part("audio/mpeg", Some("mp3"))
            .unwrap();
        assert!(pkg.has_part(&part.uri));
        assert_eq!(pkg.data_parts().len(), 1);
        pkg.feed_data_part(&part.uri, b"ID3xxxx").unwrap();
        assert_eq!(pkg.data_part_bytes(&part.uri), Some(b"ID3xxxx".as_slice()));

        // Still orphaned — delete_unused should remove it.
        assert_eq!(pkg.delete_unused_data_parts(), 1);
        assert!(pkg.data_parts().is_empty());

        let part = pkg
            .create_media_data_part_with_data("video/mp4", Some("mp4"), b"ftyp")
            .unwrap();
        let dpr = pkg
            .add_data_part_reference_relationship(
                &slide,
                &part,
                media_rel::VIDEO,
                Some("rIdVid1"),
            )
            .unwrap();
        assert_eq!(dpr.id(), "rIdVid1");
        assert!(dpr.is_video());
        let listed = pkg.data_part_reference_relationships(Some(&slide));
        assert_eq!(listed.len(), 1);

        // In use — cannot delete.
        assert!(pkg.delete_data_part(&part.uri).is_err());

        // Drop the reference, then delete.
        assert!(pkg
            .delete_reference_relationship(Some(&slide), "rIdVid1")
            .is_some());
        assert!(pkg.delete_data_part(&part.uri).unwrap());
        assert!(pkg.get_data_part(&part.uri).is_none());
    }

    #[test]
    fn id_part_pairs_and_get_reference() {
        let mut pkg = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        pkg.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        let id = pkg.add_part_relationship(
            &doc,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
            &styles,
            RelationshipTargetMode::Internal,
        );
        let pairs = pkg.id_part_pairs(Some(&doc));
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].relationship_id, id);
        assert_eq!(pairs[0].part_uri, styles);

        let rr = pkg.get_reference_relationship(Some(&doc), &id).unwrap();
        assert_eq!(rr.id, id);
        assert!(!rr.is_external);
    }
}
