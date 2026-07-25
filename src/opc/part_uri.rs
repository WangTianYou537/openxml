//! Unique part URI allocation (C# `PartUriHelper` / `IPartUriFeature`).

use super::uri::{resolve_uri, PackUri};
use super::OpcPackage;
use crate::error::Result;
use std::collections::{HashMap, HashSet};

/// Content types that number the first instance as `1` (ISO/IEC 29500).
fn numbered_content_types() -> HashSet<&'static str> {
    HashSet::from([
        // WordprocessingML
        "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml",
        // SpreadsheetML
        "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
        "application/vnd.openxmlformats-officedocument.drawing+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
        // PresentationML
        "application/vnd.openxmlformats-officedocument.presentationml.comments+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
        "application/vnd.openxmlformats-officedocument.presentationml.tags+xml",
        // DrawingML
        "application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
        "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
        "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml",
        "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml",
        "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
        "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml",
        "application/vnd.openxmlformats-officedocument.theme+xml",
        "application/vnd.openxmlformats-officedocument.themeOverride+xml",
        // Shared
        "application/vnd.openxmlformats-officedocument.customXmlProperties+xml",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings",
        "application/vnd.openxmlformats-officedocument.presentationml.printerSettings",
    ])
}

/// Allocates unique part URIs within a package, mirroring C# `PartUriHelper`.
#[derive(Debug, Default)]
pub struct PartUriHelper {
    sequence_numbers: HashMap<String, u32>,
    reserved: HashSet<String>,
}

impl PartUriHelper {
    pub fn new() -> Self {
        Self::default()
    }

    /// Seed reserved URIs from parts already in the package.
    pub fn from_package(package: &OpcPackage) -> Self {
        let mut h = Self::new();
        for uri in package.part_uris() {
            h.reserve(&uri);
        }
        h
    }

    pub fn reserve(&mut self, uri: &PackUri) {
        self.reserved.insert(normalize(uri.as_str()));
    }

    /// Reserve a URI and advance the content-type sequence counter
    /// (C# `IPartUriFeature.ReserveUri`).
    pub fn reserve_uri(&mut self, content_type: &str, uri: &PackUri) {
        let _ = self.next_sequence_number(content_type);
        self.reserve(uri);
    }

    pub fn is_reserved(&self, uri: &PackUri) -> bool {
        self.reserved.contains(&normalize(uri.as_str()))
    }

    /// Number of reserved URIs.
    pub fn reserved_count(&self) -> usize {
        self.reserved.len()
    }

    /// Clear reserved URIs and sequence counters.
    pub fn clear(&mut self) {
        self.reserved.clear();
        self.sequence_numbers.clear();
    }

    /// Create a unique part URI under `parent`.
    ///
    /// `target_path` is relative to the parent part (e.g. `"media"`, `"../charts"`, `"."`).
    /// `target_name` is the base file name without extension; `target_ext` includes the dot
    /// (e.g. `".xml"`, `".png"`).
    pub fn create_part_uri(
        &mut self,
        content_type: &str,
        parent: &PackUri,
        target_path: &str,
        target_name: &str,
        target_ext: &str,
        force_unique: bool,
    ) -> Result<PackUri> {
        let ext = if target_ext.is_empty() || target_ext.starts_with('.') {
            target_ext.to_string()
        } else {
            format!(".{target_ext}")
        };

        let part_uri = if force_unique {
            loop {
                let seq = self.next_sequence_number(content_type);
                let file = format!("{target_name}{seq}{ext}");
                let rel = join_path(target_path, &file);
                let candidate = resolve_uri(parent, &rel)?;
                if !self.reserved.contains(&normalize(candidate.as_str())) {
                    break candidate;
                }
            }
        } else {
            let file = format!("{target_name}{ext}");
            let rel = join_path(target_path, &file);
            resolve_uri(parent, &rel)?
        };

        self.reserve(&part_uri);
        Ok(part_uri)
    }

    /// Ensure `target` (relative to `parent`) is unique by appending a sequence number.
    pub fn ensure_unique_part_uri(
        &mut self,
        content_type: &str,
        parent: &PackUri,
        target: &str,
    ) -> Result<PackUri> {
        let resolved = if target.starts_with('/') {
            PackUri::new(target)
        } else {
            resolve_uri(parent, target)?
        };
        let path = resolved.as_str();
        let (dir, file) = match path.rfind('/') {
            Some(i) => (&path[..=i], &path[i + 1..]),
            None => ("/", path),
        };
        let (name, ext) = match file.rfind('.') {
            Some(i) => (&file[..i], &file[i..]),
            None => (file, ""),
        };
        // Recreate relative to parent using "." path general + unique name.
        // Use absolute dir as target_path via parent = dir's dummy.
        let parent_for_path = if dir == "/" {
            PackUri::new("/")
        } else {
            // parent of the file is the directory as a fake part uri
            PackUri::new(format!("{}dummy", dir))
        };
        self.create_part_uri(
            content_type,
            &parent_for_path,
            ".",
            name,
            ext,
            true,
        )
    }

    fn next_sequence_number(&mut self, content_type: &str) -> String {
        let entry = self
            .sequence_numbers
            .entry(content_type.to_string())
            .or_insert(0);
        *entry += 1;
        let count = *entry;
        if count == 1 && !numbered_content_types().contains(content_type) {
            String::new()
        } else {
            count.to_string()
        }
    }
}

fn normalize(uri: &str) -> String {
    uri.trim_end_matches('/').to_ascii_lowercase()
}

fn join_path(target_path: &str, file: &str) -> String {
    let tp = target_path.trim_end_matches('/');
    if tp.is_empty() || tp == "." {
        file.to_string()
    } else {
        format!("{tp}/{file}")
    }
}

/// A part related from a source, with its relationship id (C# `IdPartPair` shell).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedPart {
    pub id: String,
    pub uri: PackUri,
    pub relationship_type: String,
    pub content_type: Option<String>,
}

impl RelatedPart {
    pub fn new(
        id: impl Into<String>,
        uri: PackUri,
        relationship_type: impl Into<String>,
        content_type: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            uri,
            relationship_type: relationship_type.into(),
            content_type,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn uri(&self) -> &PackUri {
        &self.uri
    }

    pub fn relationship_type(&self) -> &str {
        &self.relationship_type
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Convert to [`super::IdPartPair`] (drops relationship/content type).
    pub fn to_id_part_pair(&self) -> crate::opc::IdPartPair {
        crate::opc::IdPartPair::new(self.id.clone(), self.uri.clone())
    }
}

impl OpcPackage {
    /// List internal child parts related from `source` (package-level if `None`).
    ///
    /// Approximate C# `OpenXmlPartContainer.Parts` / `GetPartsOfType` by relationship type
    /// filter when `relationship_type` is `Some`.
    pub fn related_parts(
        &self,
        source: Option<&PackUri>,
        relationship_type: Option<&str>,
    ) -> Vec<RelatedPart> {
        let rels: Vec<&super::Relationship> = match source {
            Some(s) => self
                .part_relationships(s)
                .map(|r| r.iter().collect())
                .unwrap_or_default(),
            None => self.package_relationships().iter().collect(),
        };
        let mut out = Vec::new();
        for rel in rels {
            if rel.target_mode != super::RelationshipTargetMode::Internal {
                continue;
            }
            if let Some(want) = relationship_type {
                if rel.relationship_type != want {
                    continue;
                }
            }
            let Ok(uri) = self.resolve_relationship(source, rel) else {
                continue;
            };
            if !self.has_part(&uri) {
                continue;
            }
            let ct = self
                .content_types()
                .content_type_for(uri.as_str())
                .map(|s| s.to_string());
            out.push(RelatedPart {
                id: rel.id.clone(),
                uri,
                relationship_type: rel.relationship_type.clone(),
                content_type: ct,
            });
        }
        out
    }

    /// Parts of a given relationship type under `source` (C# `GetPartsOfType` by rel).
    pub fn parts_of_relationship_type(
        &self,
        source: Option<&PackUri>,
        relationship_type: &str,
    ) -> Vec<RelatedPart> {
        self.related_parts(source, Some(relationship_type))
    }

    /// Parts whose content type equals `content_type` under `source` (any depth-1 child).
    pub fn parts_of_content_type(
        &self,
        source: Option<&PackUri>,
        content_type: &str,
    ) -> Vec<RelatedPart> {
        self.related_parts(source, None)
            .into_iter()
            .filter(|p| p.content_type.as_deref() == Some(content_type))
            .collect()
    }

    /// Allocate a unique part URI using a temporary [`PartUriHelper`] seeded from this package.
    pub fn create_unique_part_uri(
        &self,
        content_type: &str,
        parent: &PackUri,
        target_path: &str,
        target_name: &str,
        target_ext: &str,
    ) -> Result<PackUri> {
        let mut helper = PartUriHelper::from_package(self);
        helper.create_part_uri(
            content_type,
            parent,
            target_path,
            target_name,
            target_ext,
            true,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::{content_type, rel};
    use crate::opc::RelationshipTargetMode;

    #[test]
    fn unique_header_uris_start_at_one() {
        let mut h = PartUriHelper::new();
        let parent = PackUri::new("/word/document.xml");
        let u1 = h
            .create_part_uri(
                content_type::WORD_HEADER,
                &parent,
                "header",
                "header",
                ".xml",
                true,
            )
            .unwrap();
        let u2 = h
            .create_part_uri(
                content_type::WORD_HEADER,
                &parent,
                "header",
                "header",
                ".xml",
                true,
            )
            .unwrap();
        assert_eq!(u1.as_str(), "/word/header/header1.xml");
        assert_eq!(u2.as_str(), "/word/header/header2.xml");
    }

    #[test]
    fn unique_skips_existing_package_parts() {
        let mut pkg = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        pkg.set_part(
            PackUri::new("/word/media/image1.png"),
            content_type::IMAGE_PNG,
            b"png",
        );
        let next = pkg
            .create_unique_part_uri(
                content_type::IMAGE_PNG,
                &doc,
                "media",
                "image",
                ".png",
            )
            .unwrap();
        // First non-numbered attempt may collide with image1; helper reserves existing.
        assert_ne!(next.as_str(), "/word/media/image1.png");
        assert!(next.as_str().starts_with("/word/media/image"));
    }

    #[test]
    fn related_parts_lists_children() {
        let mut pkg = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        pkg.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &doc,
            RelationshipTargetMode::Internal,
        );
        pkg.add_part_relationship(
            &doc,
            rel::STYLES,
            &styles,
            RelationshipTargetMode::Internal,
        );
        let kids = pkg.parts_of_relationship_type(Some(&doc), rel::STYLES);
        assert_eq!(kids.len(), 1);
        assert_eq!(kids[0].uri.as_str(), "/word/styles.xml");
        assert_eq!(
            kids[0].content_type.as_deref(),
            Some(content_type::WORD_STYLES)
        );
    }
}


impl OpcPackage {
    /// Parts that have an internal relationship targeting `child`
    /// (C# `OpenXmlPart.GetParentParts` shell — returns source URIs).
    pub fn parent_parts(&self, child: &PackUri) -> Vec<PackUri> {
        let mut out = Vec::new();
        // Package-level relationships
        for rel in self.package_relationships().iter() {
            if rel.target_mode != super::RelationshipTargetMode::Internal {
                continue;
            }
            if let Ok(u) = self.resolve_relationship(None, rel) {
                if &u == child {
                    // Package itself is not a part URI; skip — C# yields OpenXmlPart only.
                }
            }
        }
        for src in self.part_relationship_sources() {
            if let Some(rels) = self.part_relationships(&src) {
                for rel in rels.iter() {
                    if rel.target_mode != super::RelationshipTargetMode::Internal {
                        continue;
                    }
                    if let Ok(u) = self.resolve_relationship(Some(&src), rel) {
                        if &u == child {
                            out.push(src.clone());
                            break;
                        }
                    }
                }
            }
        }
        out
    }

    /// Whether `parent` has an internal child relationship to `child`.
    pub fn is_child_part(&self, parent: &PackUri, child: &PackUri) -> bool {
        let Some(rels) = self.part_relationships(parent) else {
            return false;
        };
        for rel in rels.iter() {
            if rel.target_mode != super::RelationshipTargetMode::Internal {
                continue;
            }
            if let Ok(u) = self.resolve_relationship(Some(parent), rel) {
                if &u == child {
                    return true;
                }
            }
        }
        false
    }
}


impl OpcPackage {
    /// Breadth-first traversal of all parts reachable from package relationships
    /// (C# `OpenXmlPackageExtensions.GetAllParts`).
    ///
    /// Returns `(uri, content_type)` pairs; content type may be empty if missing.
    pub fn get_all_parts(&self) -> Vec<(PackUri, String)> {
        use std::collections::{HashSet, VecDeque};
        let mut visited: HashSet<String> = HashSet::new();
        let mut out = Vec::new();
        let mut queue: VecDeque<PackUri> = VecDeque::new();

        for rel in self.package_relationships().iter() {
            if rel.target_mode != super::RelationshipTargetMode::Internal {
                continue;
            }
            if let Ok(u) = self.resolve_relationship(None, rel) {
                if self.has_part(&u) && visited.insert(u.as_str().to_string()) {
                    queue.push_back(u);
                }
            }
        }

        while let Some(uri) = queue.pop_front() {
            let ct = self
                .content_types()
                .content_type_for(uri.as_str())
                .unwrap_or("")
                .to_string();
            out.push((uri.clone(), ct));
            if let Some(rels) = self.part_relationships(&uri) {
                for rel in rels.iter() {
                    if rel.target_mode != super::RelationshipTargetMode::Internal {
                        continue;
                    }
                    if let Ok(child) = self.resolve_relationship(Some(&uri), rel) {
                        if self.has_part(&child) && visited.insert(child.as_str().to_string()) {
                            queue.push_back(child);
                        }
                    }
                }
            }
        }
        out
    }

    /// BFS part URIs only.
    pub fn get_all_part_uris(&self) -> Vec<PackUri> {
        self.get_all_parts().into_iter().map(|(u, _)| u).collect()
    }
}

#[cfg(test)]
mod get_all_parts_tests {
    use super::*;
    use crate::namespace::{content_type, rel};
    use crate::opc::RelationshipTargetMode;

    #[test]
    fn get_all_parts_bfs() {
        let mut pkg = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        let theme = PackUri::new("/word/theme/theme1.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        pkg.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        pkg.set_part(theme.clone(), content_type::THEME, b"<a:theme/>".to_vec());
        pkg.add_package_relationship(rel::OFFICE_DOCUMENT, &doc, RelationshipTargetMode::Internal);
        pkg.add_part_relationship(&doc, rel::STYLES, &styles, RelationshipTargetMode::Internal);
        pkg.add_part_relationship(&doc, rel::THEME, &theme, RelationshipTargetMode::Internal);
        let all = pkg.get_all_part_uris();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], doc);
        assert!(all.contains(&styles));
        assert!(all.contains(&theme));
    }
}

#[cfg(test)]
mod reserve_uri_tests {
    use super::*;
    use crate::opc::OpcPackage;

    #[test]
    fn reserve_uri_advances_sequence() {
        let mut h = PartUriHelper::new();
        let u = PackUri::new("/word/media/image1.png");
        h.reserve_uri("image/png", &u);
        assert!(h.is_reserved(&u));
        assert_eq!(h.reserved_count(), 1);
        let parent = PackUri::new("/word/document.xml");
        let next = h
            .create_part_uri("image/png", &parent, "media", "image", ".png", true)
            .unwrap();
        // sequence advanced once by reserve_uri, so next unique should not collide
        assert!(h.is_reserved(&next));
        assert_ne!(next.as_str(), u.as_str());
        h.clear();
        assert_eq!(h.reserved_count(), 0);
        let _ = OpcPackage::create();
    }

    #[test]
    fn related_part_accessors_and_id_part_pair() {
        let uri = PackUri::new("/word/styles.xml");
        let r = RelatedPart::new(
            "rId1",
            uri.clone(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml".into()),
        );
        assert_eq!(r.id(), "rId1");
        assert_eq!(r.uri(), &uri);
        assert!(r.relationship_type().ends_with("/styles"));
        assert!(r.content_type().unwrap().contains("styles"));
        let pair = r.to_id_part_pair();
        assert_eq!(pair.relationship_id(), "rId1");
        assert_eq!(pair.part_uri(), &uri);
        assert_eq!(pair.open_xml_part_uri(), &uri);
    }
}
