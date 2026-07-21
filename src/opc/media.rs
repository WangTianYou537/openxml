//! Media / data parts (audio, video, generic binary) related from Open XML parts.
//!
//! Mirrors C# `DataPart` / `MediaDataPart` at a practical level: store bytes, create
//! package relationships. No playback or codec handling.

use crate::error::Result;
use crate::opc::{OpcPackage, PackUri, RelationshipTargetMode};

/// Common media relationship type URIs.
pub mod media_rel {
    pub const AUDIO: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio";
    pub const VIDEO: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video";
    pub const MEDIA: &str =
        "http://schemas.microsoft.com/office/2007/relationships/media";
    pub const IMAGE: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
}

/// Kind of media data part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Audio,
    Video,
    Media,
    Image,
}

impl MediaKind {
    pub fn relationship_type(self) -> &'static str {
        match self {
            Self::Audio => media_rel::AUDIO,
            Self::Video => media_rel::VIDEO,
            Self::Media => media_rel::MEDIA,
            Self::Image => media_rel::IMAGE,
        }
    }
}

/// Handle returned when a media part is added.
#[derive(Debug, Clone)]
pub struct MediaPartInfo {
    pub uri: PackUri,
    pub relationship_id: String,
    pub content_type: String,
    pub kind: MediaKind,
}

/// Add a media/data part related from `source_part`.
///
/// `extension` should not include a leading dot (e.g. `"mp3"`, `"mp4"`).
/// Parts are stored under `/media/` by default.
pub fn add_media_part(
    package: &mut OpcPackage,
    source_part: &PackUri,
    kind: MediaKind,
    data: impl Into<Vec<u8>>,
    content_type: &str,
    extension: &str,
) -> Result<MediaPartInfo> {
    let mut index = 1u32;
    let uri = loop {
        let candidate = PackUri::new(format!("/media/media{index}.{extension}"));
        if !package.has_part(&candidate) {
            break candidate;
        }
        index += 1;
    };
    package.set_part(uri.clone(), content_type, data.into());
    let rid = package.add_part_relationship(
        source_part,
        kind.relationship_type(),
        &uri,
        RelationshipTargetMode::Internal,
    );
    Ok(MediaPartInfo {
        uri,
        relationship_id: rid,
        content_type: content_type.to_string(),
        kind,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::content_type;
    use crate::namespace::rel;
    use crate::opc::PackUri;

    #[test]
    fn add_audio_part() {
        let mut pkg = OpcPackage::create();
        let slide = PackUri::new("/ppt/slides/slide1.xml");
        pkg.set_part(
            slide.clone(),
            content_type::PRESENTATION_SLIDE,
            b"<p:sld/>".to_vec(),
        );
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &PackUri::new("/ppt/presentation.xml"),
            RelationshipTargetMode::Internal,
        );
        let info = add_media_part(
            &mut pkg,
            &slide,
            MediaKind::Audio,
            b"ID3fake",
            "audio/mpeg",
            "mp3",
        )
        .unwrap();
        assert!(pkg.has_part(&info.uri));
        assert!(info.relationship_id.starts_with('r'));
    }
}
