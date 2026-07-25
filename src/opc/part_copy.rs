//! Copy / import parts between packages (C# `AddPart` cross-package shell).

use super::uri::{relativize, PackUri};
use super::{OpcPackage, RelationshipTargetMode};
use crate::error::{Error, Result};
use std::collections::{HashMap, HashSet, VecDeque};

/// Options for [`OpcPackage::copy_part_from`].
#[derive(Debug, Clone, Copy)]
pub struct CopyPartOptions {
    /// Also copy the part's outbound relationship graph (BFS of internal targets).
    pub recursive: bool,
    /// When recursive, also copy external relationship entries (target strings only).
    pub include_external_rels: bool,
}

impl Default for CopyPartOptions {
    fn default() -> Self {
        Self {
            recursive: true,
            include_external_rels: true,
        }
    }
}

impl OpcPackage {
    /// Copy a single part's bytes + content type from `source` into this package at
    /// `dest_uri` (C# `AddPart` when the part lives in another package — non-shared).
    pub fn import_part(
        &mut self,
        source: &OpcPackage,
        source_uri: &PackUri,
        dest_uri: &PackUri,
    ) -> Result<()> {
        let data = source
            .get_part(source_uri)
            .ok_or_else(|| Error::PartNotFound(source_uri.to_string()))?
            .to_vec();
        let ct = source
            .content_types()
            .content_type_for(source_uri.as_str())
            .unwrap_or("application/octet-stream")
            .to_string();
        self.set_part(dest_uri.clone(), ct, data);
        Ok(())
    }

    /// Copy `source_uri` (and optionally its relationship subtree) from `source`
    /// into this package, preserving relative relationship structure.
    ///
    /// Returns a map of `source_uri → dest_uri` for every part copied.
    pub fn copy_part_from(
        &mut self,
        source: &OpcPackage,
        source_uri: &PackUri,
        dest_uri: &PackUri,
        opts: CopyPartOptions,
    ) -> Result<HashMap<PackUri, PackUri>> {
        if !source.has_part(source_uri) {
            return Err(Error::PartNotFound(source_uri.to_string()));
        }

        // Plan: BFS from source_uri collecting parts to copy.
        let mut order: Vec<PackUri> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<PackUri> = VecDeque::new();
        queue.push_back(source_uri.clone());
        seen.insert(source_uri.as_str().to_string());

        while let Some(uri) = queue.pop_front() {
            order.push(uri.clone());
            if !opts.recursive {
                continue;
            }
            if let Some(rels) = source.part_relationships(&uri) {
                for rel in rels.iter() {
                    if rel.target_mode != RelationshipTargetMode::Internal {
                        continue;
                    }
                    if let Ok(child) = source.resolve_relationship(Some(&uri), rel) {
                        if source.has_part(&child) && seen.insert(child.as_str().to_string()) {
                            queue.push_back(child);
                        }
                    }
                }
            }
        }

        // Map source → dest URIs. Root uses dest_uri; children keep path relative to root.
        let mut map: HashMap<PackUri, PackUri> = HashMap::new();
        map.insert(source_uri.clone(), dest_uri.clone());
        for uri in order.iter().skip(1) {
            // Keep the same absolute path if free; otherwise prefix with a unique folder.
            let candidate = if self.has_part(uri) && uri != source_uri {
                // Collision — place under /_imported/<n>/...
                let mut n = 1u32;
                loop {
                    let c = PackUri::new(format!("/_imported/{n}{}", uri.as_str()));
                    if !self.has_part(&c) {
                        break c;
                    }
                    n += 1;
                }
            } else {
                uri.clone()
            };
            map.insert(uri.clone(), candidate);
        }

        // Copy bytes
        for src in &order {
            let dst = map.get(src).expect("mapped").clone();
            self.import_part(source, src, &dst)?;
        }

        // Copy relationships with remapped internal targets
        for src in &order {
            let Some(rels) = source.part_relationships(src) else {
                continue;
            };
            let dst = map.get(src).expect("mapped").clone();
            for rel in rels.iter() {
                match rel.target_mode {
                    RelationshipTargetMode::External => {
                        if opts.include_external_rels {
                            self.part_relationships_mut(&dst).add_with_id(
                                &rel.id,
                                rel.relationship_type.clone(),
                                rel.target.clone(),
                                RelationshipTargetMode::External,
                            );
                        }
                    }
                    RelationshipTargetMode::Internal => {
                        let Ok(abs) = source.resolve_relationship(Some(src), rel) else {
                            continue;
                        };
                        let target_dest = match map.get(&abs) {
                            Some(u) => u.clone(),
                            None => {
                                // Not in copied subgraph — keep original absolute as relative
                                abs
                            }
                        };
                        let rel_target = relativize(&dst, &target_dest);
                        self.part_relationships_mut(&dst).add_with_id(
                            &rel.id,
                            rel.relationship_type.clone(),
                            rel_target,
                            RelationshipTargetMode::Internal,
                        );
                    }
                }
            }
        }

        Ok(map)
    }

    /// Add a relationship from `source` to an existing part in this package and
    /// return the relationship id (same-package `CreateRelationshipToPart` / shared AddPart).
    pub fn add_part_relationship_to_existing(
        &mut self,
        source: &PackUri,
        target: &PackUri,
        relationship_type: &str,
        id: Option<&str>,
    ) -> Result<String> {
        if !self.has_part(target) {
            return Err(Error::PartNotFound(target.to_string()));
        }
        // Already related?
        if let Some(existing) = self.get_id_of_part(Some(source), target) {
            if let Some(want) = id {
                if existing != want {
                    return Err(Error::Package(format!(
                        "part already related as `{existing}`, not `{want}`"
                    )));
                }
            }
            return Ok(existing);
        }
        Ok(self.create_relationship_to_part(source, target, relationship_type, id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::{content_type, rel};

    #[test]
    fn copy_part_recursive_roundtrip() {
        let mut src = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        src.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            b"<w:document/>".to_vec(),
        );
        src.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        src.add_part_relationship(&doc, rel::STYLES, &styles, RelationshipTargetMode::Internal);

        let mut dst = OpcPackage::create();
        let dest_doc = PackUri::new("/word/document.xml");
        let map = dst
            .copy_part_from(&src, &doc, &dest_doc, CopyPartOptions::default())
            .unwrap();
        assert!(dst.has_part(&dest_doc));
        assert!(dst.has_part(&styles));
        assert_eq!(map.get(&doc), Some(&dest_doc));
        assert_eq!(
            dst.get_id_of_part(Some(&dest_doc), &styles).is_some(),
            true
        );
    }

    #[test]
    fn import_part_non_recursive() {
        let mut src = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        src.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        src.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        src.add_part_relationship(&doc, rel::STYLES, &styles, RelationshipTargetMode::Internal);

        let mut dst = OpcPackage::create();
        let map = dst
            .copy_part_from(
                &src,
                &doc,
                &PackUri::new("/word/document.xml"),
                CopyPartOptions {
                    recursive: false,
                    include_external_rels: false,
                },
            )
            .unwrap();
        assert_eq!(map.len(), 1);
        assert!(dst.has_part(&PackUri::new("/word/document.xml")));
        assert!(!dst.has_part(&styles));
    }
}
