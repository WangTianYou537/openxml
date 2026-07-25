//! Typed reference relationships (C# `ReferenceRelationship` hierarchy).

use super::{Relationship, RelationshipTargetMode, Relationships};
use crate::namespace::rel;
use crate::opc::media::media_rel;
use crate::opc::{OpcPackage, PackUri};

/// A reference relationship (internal or external). Mirrors C# `ReferenceRelationship`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceRelationship {
    pub id: String,
    pub relationship_type: String,
    pub target: String,
    pub is_external: bool,
}

impl ReferenceRelationship {
    pub fn from_relationship(rel: &Relationship) -> Self {
        Self {
            id: rel.id.clone(),
            relationship_type: rel.relationship_type.clone(),
            target: rel.target.clone(),
            is_external: rel.target_mode == RelationshipTargetMode::External,
        }
    }

    pub fn is_hyperlink(&self) -> bool {
        self.relationship_type == rel::HYPERLINK
    }

    pub fn is_audio(&self) -> bool {
        self.relationship_type == media_rel::AUDIO
    }

    pub fn is_video(&self) -> bool {
        self.relationship_type == media_rel::VIDEO
    }

    pub fn is_media(&self) -> bool {
        self.relationship_type == media_rel::MEDIA
    }
}

/// Hyperlink relationship (C# `HyperlinkRelationship`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperlinkRelationship {
    pub inner: ReferenceRelationship,
}

impl HyperlinkRelationship {
    pub const RELATIONSHIP_TYPE: &'static str = rel::HYPERLINK;

    pub fn new(target: impl Into<String>, is_external: bool, id: impl Into<String>) -> Self {
        Self {
            inner: ReferenceRelationship {
                id: id.into(),
                relationship_type: Self::RELATIONSHIP_TYPE.to_string(),
                target: target.into(),
                is_external,
            },
        }
    }

    pub fn from_relationship(rel: &Relationship) -> Option<Self> {
        if rel.relationship_type != Self::RELATIONSHIP_TYPE {
            return None;
        }
        Some(Self {
            inner: ReferenceRelationship::from_relationship(rel),
        })
    }

    pub fn id(&self) -> &str {
        &self.inner.id
    }

    pub fn target(&self) -> &str {
        &self.inner.target
    }

    pub fn is_external(&self) -> bool {
        self.inner.is_external
    }
}

/// Audio reference to a media data part (C# `AudioReferenceRelationship`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioReferenceRelationship {
    pub inner: ReferenceRelationship,
}

impl AudioReferenceRelationship {
    pub const RELATIONSHIP_TYPE: &'static str = media_rel::AUDIO;

    pub fn from_relationship(rel: &Relationship) -> Option<Self> {
        if rel.relationship_type != Self::RELATIONSHIP_TYPE {
            return None;
        }
        Some(Self {
            inner: ReferenceRelationship::from_relationship(rel),
        })
    }
}

/// Video reference to a media data part (C# `VideoReferenceRelationship`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoReferenceRelationship {
    pub inner: ReferenceRelationship,
}

impl VideoReferenceRelationship {
    pub const RELATIONSHIP_TYPE: &'static str = media_rel::VIDEO;

    pub fn from_relationship(rel: &Relationship) -> Option<Self> {
        if rel.relationship_type != Self::RELATIONSHIP_TYPE {
            return None;
        }
        Some(Self {
            inner: ReferenceRelationship::from_relationship(rel),
        })
    }
}

/// Media reference (C# `MediaReferenceRelationship`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaReferenceRelationship {
    pub inner: ReferenceRelationship,
}

impl MediaReferenceRelationship {
    pub const RELATIONSHIP_TYPE: &'static str = media_rel::MEDIA;

    pub fn from_relationship(rel: &Relationship) -> Option<Self> {
        if rel.relationship_type != Self::RELATIONSHIP_TYPE {
            return None;
        }
        Some(Self {
            inner: ReferenceRelationship::from_relationship(rel),
        })
    }
}

fn map_rels<'a, T>(
    rels: Option<&'a Relationships>,
    f: impl Fn(&Relationship) -> Option<T>,
) -> Vec<T> {
    match rels {
        Some(r) => r.iter().filter_map(f).collect(),
        None => Vec::new(),
    }
}

impl OpcPackage {
    /// All reference relationships from `source` (package-level if `None`).
    pub fn reference_relationships(
        &self,
        source: Option<&PackUri>,
    ) -> Vec<ReferenceRelationship> {
        let rels = match source {
            Some(s) => self.part_relationships(s),
            None => Some(self.package_relationships()),
        };
        map_rels(rels, |r| Some(ReferenceRelationship::from_relationship(r)))
    }

    /// Hyperlink relationships from `source`.
    pub fn hyperlink_relationships(
        &self,
        source: Option<&PackUri>,
    ) -> Vec<HyperlinkRelationship> {
        let rels = match source {
            Some(s) => self.part_relationships(s),
            None => Some(self.package_relationships()),
        };
        map_rels(rels, HyperlinkRelationship::from_relationship)
    }

    /// Audio reference relationships from `source`.
    pub fn audio_reference_relationships(
        &self,
        source: Option<&PackUri>,
    ) -> Vec<AudioReferenceRelationship> {
        let rels = match source {
            Some(s) => self.part_relationships(s),
            None => Some(self.package_relationships()),
        };
        map_rels(rels, AudioReferenceRelationship::from_relationship)
    }

    /// Video reference relationships from `source`.
    pub fn video_reference_relationships(
        &self,
        source: Option<&PackUri>,
    ) -> Vec<VideoReferenceRelationship> {
        let rels = match source {
            Some(s) => self.part_relationships(s),
            None => Some(self.package_relationships()),
        };
        map_rels(rels, VideoReferenceRelationship::from_relationship)
    }

    /// Media reference relationships from `source`.
    pub fn media_reference_relationships(
        &self,
        source: Option<&PackUri>,
    ) -> Vec<MediaReferenceRelationship> {
        let rels = match source {
            Some(s) => self.part_relationships(s),
            None => Some(self.package_relationships()),
        };
        map_rels(rels, MediaReferenceRelationship::from_relationship)
    }

    /// Add a hyperlink relationship (external by default when target looks like a URL).
    pub fn add_hyperlink_relationship(
        &mut self,
        source: &PackUri,
        target: &str,
        is_external: bool,
    ) -> String {
        let mode = if is_external {
            RelationshipTargetMode::External
        } else {
            RelationshipTargetMode::Internal
        };
        let id = self
            .part_relationships_mut(source)
            .add(rel::HYPERLINK, target, mode)
            .id
            .clone();
        // part_relationships_mut already sets dirty
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::content_type;

    #[test]
    fn hyperlink_relationship_roundtrip() {
        let mut pkg = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        let id = pkg.add_hyperlink_relationship(&doc, "https://example.com", true);
        let hls = pkg.hyperlink_relationships(Some(&doc));
        assert_eq!(hls.len(), 1);
        assert_eq!(hls[0].id(), id);
        assert!(hls[0].is_external());
        assert_eq!(hls[0].target(), "https://example.com");
    }
}
