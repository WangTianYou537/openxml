//! OPC package backed by a ZIP archive.

use super::content_types::ContentTypes;
use super::relationships::Relationships;
use super::uri::{relativize, resolve_uri, PackUri};
use super::{Relationship, RelationshipTargetMode};
use crate::error::{Error, Result};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

/// How the package was opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageMode {
    Read,
    Create,
    ReadWrite,
}

/// ZIP compression level for ordinary (non-media, non-font) parts.
///
/// Mirrors C# `System.IO.Packaging.CompressionOption` used by `OpenXmlPackage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionOption {
    /// No compression (`Stored`) for all parts.
    NotCompressed,
    /// Fast deflate (zip crate default level).
    Fast,
    /// Normal deflate (default; C# `CompressionOption.Normal`).
    #[default]
    Normal,
    /// Maximum deflate effort.
    Maximum,
    /// Alias of [`Fast`] (C# `SuperFast`).
    SuperFast,
}

/// How part bytes are stored.
#[derive(Debug, Clone)]
enum PartData {
    /// Fully loaded bytes.
    Loaded(Vec<u8>),
    /// Not yet decompressed; index into the backing ZIP archive in `zip_bytes`.
    Lazy { zip_index: usize },
}

/// In-memory representation of an OPC package.
///
/// By default parts are fully loaded. [`OpcPackage::open_lazy`] / [`OpcPackage::open_bytes_lazy`]
/// keep compressed ZIP data and decompress individual parts on first access — useful for large
/// packages when only a few parts are needed.
#[derive(Debug)]
pub struct OpcPackage {
    mode: PackageMode,
    path: Option<PathBuf>,
    /// Part URI → raw or lazy bytes (RefCell so lazy materialize works with `&self`).
    parts: RefCell<IndexMap<PackUri, PartData>>,
    /// Backing ZIP bytes when any part is still lazy.
    zip_bytes: RefCell<Option<Vec<u8>>>,
    content_types: ContentTypes,
    /// Package-level relationships (`/_rels/.rels`).
    package_rels: Relationships,
    /// Part URI → that part's relationships.
    part_rels: IndexMap<PackUri, Relationships>,
    dirty: bool,
    /// Compression for non-media/non-font parts when serializing.
    compression: CompressionOption,
    /// Package-level data/media parts (C# `IDataPartsFeature` / `DataParts`).
    pub(crate) data_parts: Vec<super::data_part::DataPart>,
}

impl OpcPackage {
    /// Create a new empty package (in-memory).
    pub fn create() -> Self {
        Self {
            mode: PackageMode::Create,
            path: None,
            parts: RefCell::new(IndexMap::new()),
            zip_bytes: RefCell::new(None),
            content_types: ContentTypes::new(),
            package_rels: Relationships::new(),
            part_rels: IndexMap::new(),
            dirty: true,
            compression: CompressionOption::Normal,
            data_parts: Vec::new(),
        }
    }

    /// Create a new package that will be written to `path` on save.
    pub fn create_file(path: impl Into<PathBuf>) -> Self {
        let mut pkg = Self::create();
        pkg.path = Some(path.into());
        pkg
    }

    /// Open an existing package from a file path (read-write, loads into memory).
    /// Detect encrypted Office files (OLE compound CFB signature or encrypted-package part).
    ///
    /// Mirrors C# `OpenXmlPackage.IsEncryptedOfficeFile`. This crate does **not**
    /// decrypt; callers should refuse to open when this returns `true`.
    pub fn is_encrypted_office_file(path: impl AsRef<Path>) -> Result<bool> {
        let mut file = File::open(path)?;
        Self::is_encrypted_office_stream(&mut file)
    }

    /// Detect encrypted Office content from a seekable byte stream.
    pub fn is_encrypted_office_stream<R: Read + Seek>(reader: &mut R) -> Result<bool> {
        if Self::has_ole_cfb_signature(reader)? {
            return Ok(true);
        }
        // Not OLE — try as ZIP/OPC and look for encrypted-package part
        let start = reader.stream_position()?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        reader.seek(SeekFrom::Start(start))?;
        match Self::open_reader(Cursor::new(buf)) {
            Ok(pkg) => Ok(pkg.has_encrypted_package_part()),
            Err(_) => Ok(false),
        }
    }

    fn has_ole_cfb_signature<R: Read + Seek>(reader: &mut R) -> Result<bool> {
        let start = reader.stream_position()?;
        let mut header = [0u8; 8];
        let n = reader.read(&mut header)?;
        reader.seek(SeekFrom::Start(start))?;
        // OLE Compound File signature (encrypted legacy / IRM packages)
        const OLE_CFB: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
        Ok(n == 8 && header == OLE_CFB)
    }

    fn has_encrypted_package_part(&self) -> bool {
        self.parts.borrow().keys().any(|u| {
            self.content_types
                .content_type_for(u.as_str())
                .map(|ct| {
                    ct.eq_ignore_ascii_case(
                        "application/vnd.openxmlformats-officedocument.encrypted-package",
                    )
                })
                .unwrap_or(false)
                || u.as_str().eq_ignore_ascii_case("/encryptedpackage")
                || u.as_str().to_ascii_lowercase().contains("encryptedpackage")
        })
    }

    /// Open an existing package from a file path (read-write, loads into memory).
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut file = File::open(path)?;
        // Cheap reject of OLE/IRM encrypted compound files before ZIP parse
        if Self::has_ole_cfb_signature(&mut file)? {
            return Err(Error::EncryptedPackage);
        }
        let mut pkg = Self::open_reader(file)?;
        if pkg.has_encrypted_package_part() {
            return Err(Error::EncryptedPackage);
        }
        pkg.path = Some(path.to_path_buf());
        pkg.mode = PackageMode::ReadWrite;
        Ok(pkg)
    }

    /// Open an existing package from any `Read + Seek` source (eager: all parts decompressed).
    pub fn open_reader<R: Read + std::io::Seek>(reader: R) -> Result<Self> {
        Self::open_reader_impl(reader, false)
    }

    /// Open a package, deferring part decompression until first access.
    pub fn open_reader_lazy<R: Read + std::io::Seek>(reader: R) -> Result<Self> {
        Self::open_reader_impl(reader, true)
    }

    fn open_reader_impl<R: Read + std::io::Seek>(mut reader: R, lazy: bool) -> Result<Self> {
        // Keep raw ZIP bytes for lazy mode (and for re-open of individual entries).
        let start = reader.stream_position().unwrap_or(0);
        let mut zip_bytes = Vec::new();
        reader.read_to_end(&mut zip_bytes)?;
        reader.seek(SeekFrom::Start(start)).ok();

        let mut archive = ZipArchive::new(Cursor::new(zip_bytes.as_slice()))?;
        let mut parts: IndexMap<PackUri, PartData> = IndexMap::new();

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let name = entry.name().to_string();
            if name.ends_with('/') {
                continue;
            }
            let uri = PackUri::new(format!("/{name}"));
            if lazy {
                parts.insert(uri, PartData::Lazy { zip_index: i });
            } else {
                let mut data = Vec::new();
                entry.read_to_end(&mut data)?;
                parts.insert(uri, PartData::Loaded(data));
            }
        }

        // Content types + package rels always need to be loaded.
        let ct_uri = PackUri::new("/[Content_Types].xml");
        let ct_data = Self::materialize_part_data(&mut parts, &zip_bytes, &ct_uri)?
            .ok_or_else(|| Error::Package("missing [Content_Types].xml in package".into()))?;
        let content_types = ContentTypes::parse(&ct_data)?;
        parts.shift_remove(&ct_uri);

        let rels_uri = PackUri::new("/_rels/.rels");
        let package_rels = if let Some(data) =
            Self::materialize_part_data(&mut parts, &zip_bytes, &rels_uri)?
        {
            parts.shift_remove(&rels_uri);
            Relationships::parse(&data)?
        } else {
            Relationships::new()
        };

        // Extract part relationship parts into part_rels (always materialize).
        let mut part_rels = IndexMap::new();
        let rel_keys: Vec<PackUri> = parts
            .keys()
            .filter(|u| u.as_str().contains("/_rels/") && u.as_str().ends_with(".rels"))
            .cloned()
            .collect();

        for rel_uri in rel_keys {
            if let Some(data) = Self::materialize_part_data(&mut parts, &zip_bytes, &rel_uri)? {
                parts.shift_remove(&rel_uri);
                let source = part_uri_from_rels_uri(&rel_uri);
                let rels = Relationships::parse(&data)?;
                part_rels.insert(source, rels);
            }
        }

        let mut pkg = Self {
            mode: PackageMode::Read,
            path: None,
            parts: RefCell::new(parts),
            zip_bytes: RefCell::new(if lazy { Some(zip_bytes) } else { None }),
            content_types,
            package_rels,
            part_rels,
            dirty: false,
            compression: CompressionOption::Normal,
            data_parts: Vec::new(),
        };
        // Seed data-part registry from audio/video/media relationships present in the package.
        pkg.discover_data_parts();
        Ok(pkg)
    }

    /// Scan relationships and register targets of data-part reference types.
    pub fn discover_data_parts(&mut self) {
        use super::data_part::DataPartReferenceRelationship;
        let mut uris = Vec::new();
        for rel in self.package_rels.iter() {
            if DataPartReferenceRelationship::is_data_part_relationship_type(&rel.relationship_type)
                && rel.target_mode == RelationshipTargetMode::Internal
            {
                if let Ok(u) = self.resolve_relationship(None, rel) {
                    uris.push(u);
                }
            }
        }
        let sources: Vec<PackUri> = self.part_rels.keys().cloned().collect();
        for src in sources {
            if let Some(rels) = self.part_rels.get(&src) {
                for rel in rels.iter() {
                    if DataPartReferenceRelationship::is_data_part_relationship_type(
                        &rel.relationship_type,
                    ) && rel.target_mode == RelationshipTargetMode::Internal
                    {
                        if let Ok(u) = self.resolve_relationship(Some(&src), rel) {
                            uris.push(u);
                        }
                    }
                }
            }
        }
        for uri in uris {
            if !self.has_part(&uri) {
                continue;
            }
            if self.data_parts.iter().any(|p| p.uri == uri) {
                continue;
            }
            let ct = self
                .content_types
                .content_type_for(uri.as_str())
                .unwrap_or("application/octet-stream")
                .to_string();
            self.data_parts
                .push(super::data_part::DataPart::new(uri, ct));
        }
    }

    /// Materialize a part into `Loaded` and return a clone of its bytes.
    fn materialize_part_data(
        parts: &mut IndexMap<PackUri, PartData>,
        zip_bytes: &[u8],
        uri: &PackUri,
    ) -> Result<Option<Vec<u8>>> {
        match parts.get(uri) {
            None => Ok(None),
            Some(PartData::Loaded(data)) => Ok(Some(data.clone())),
            Some(PartData::Lazy { zip_index }) => {
                let idx = *zip_index;
                let mut archive = ZipArchive::new(Cursor::new(zip_bytes))?;
                let mut entry = archive.by_index(idx)?;
                let mut data = Vec::new();
                entry.read_to_end(&mut data)?;
                parts.insert(uri.clone(), PartData::Loaded(data.clone()));
                Ok(Some(data))
            }
        }
    }

    fn ensure_loaded(&self, uri: &PackUri) -> Result<()> {
        let needs = matches!(self.parts.borrow().get(uri), Some(PartData::Lazy { .. }));
        if !needs {
            return Ok(());
        }
        let zip_index = match self.parts.borrow().get(uri) {
            Some(PartData::Lazy { zip_index }) => *zip_index,
            _ => return Ok(()),
        };
        let zip_bytes = self.zip_bytes.borrow();
        let zip_bytes = zip_bytes
            .as_ref()
            .ok_or_else(|| Error::Package("lazy part without zip backing store".into()))?;
        let mut archive = ZipArchive::new(Cursor::new(zip_bytes.as_slice()))?;
        let mut entry = archive.by_index(zip_index)?;
        let mut data = Vec::new();
        entry.read_to_end(&mut data)?;
        drop(zip_bytes);
        self.parts
            .borrow_mut()
            .insert(uri.clone(), PartData::Loaded(data));
        Ok(())
    }

    /// Force-load every lazy part (e.g. before write).
    pub fn materialize_all(&self) -> Result<()> {
        let uris: Vec<PackUri> = self
            .parts
            .borrow()
            .iter()
            .filter_map(|(u, p)| match p {
                PartData::Lazy { .. } => Some(u.clone()),
                _ => None,
            })
            .collect();
        for u in uris {
            self.ensure_loaded(&u)?;
        }
        if self
            .parts
            .borrow()
            .values()
            .all(|p| matches!(p, PartData::Loaded(_)))
        {
            *self.zip_bytes.borrow_mut() = None;
        }
        Ok(())
    }

    /// Whether this package still has unloaded lazy parts.
    pub fn has_lazy_parts(&self) -> bool {
        self.parts
            .borrow()
            .values()
            .any(|p| matches!(p, PartData::Lazy { .. }))
    }

    /// Open from bytes (e.g. an in-memory docx), eager.
    pub fn open_bytes(data: impl AsRef<[u8]>) -> Result<Self> {
        Self::open_bytes_impl(data.as_ref(), false)
    }

    /// Open from bytes with lazy part loading.
    pub fn open_bytes_lazy(data: impl AsRef<[u8]>) -> Result<Self> {
        Self::open_bytes_impl(data.as_ref(), true)
    }

    fn open_bytes_impl(bytes: &[u8], lazy: bool) -> Result<Self> {
        if bytes.len() >= 8 {
            const OLE_CFB: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
            if bytes[..8] == OLE_CFB {
                return Err(Error::EncryptedPackage);
            }
        }
        let pkg = Self::open_reader_impl(Cursor::new(bytes.to_vec()), lazy)?;
        if pkg.has_encrypted_package_part() {
            return Err(Error::EncryptedPackage);
        }
        Ok(pkg)
    }

    /// Open a file path with lazy part loading.
    pub fn open_lazy(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut file = File::open(path)?;
        if Self::has_ole_cfb_signature(&mut file)? {
            return Err(Error::EncryptedPackage);
        }
        let mut pkg = Self::open_reader_lazy(file)?;
        if pkg.has_encrypted_package_part() {
            return Err(Error::EncryptedPackage);
        }
        pkg.path = Some(path.to_path_buf());
        pkg.mode = PackageMode::ReadWrite;
        Ok(pkg)
    }

    pub fn mode(&self) -> PackageMode {
        self.mode
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn content_types(&self) -> &ContentTypes {
        &self.content_types
    }

    pub fn content_types_mut(&mut self) -> &mut ContentTypes {
        self.dirty = true;
        &mut self.content_types
    }

    pub fn package_relationships(&self) -> &Relationships {
        &self.package_rels
    }

    pub fn package_relationships_mut(&mut self) -> &mut Relationships {
        self.dirty = true;
        &mut self.package_rels
    }

    /// List all part URIs in the package (excluding content types / rels).
    pub fn part_uris(&self) -> Vec<PackUri> {
        self.parts.borrow().keys().cloned().collect()
    }

    pub fn has_part(&self, uri: &PackUri) -> bool {
        self.parts.borrow().contains_key(uri)
    }

    /// Get part bytes, materializing a lazy part if needed.
    pub fn get_part(&self, uri: &PackUri) -> Option<&[u8]> {
        if self.ensure_loaded(uri).is_err() {
            return None;
        }
        // SAFETY: we leak nothing; return via transmute of extended lifetime is unsafe.
        // Instead return None for API that needs long-lived ref — use load_part for owned.
        // For compatibility with existing code that expects Option<&[u8]>, we store loaded
        // data in the map and use a lifetime tied to self via Ref::map — but that can't
        // return & from Ref. So we use a thread-local-free approach: after ensure_loaded,
        // the entry is Loaded; we get a raw pointer from the RefCell temporarily.
        // Practical approach used by many crates: document that get_part returns None for
        // still-failed loads; for success we need owned or unsafe.
        //
        // We use load into a side channel is wrong. Best fix: change callers to load_part.
        // To keep API: use `unsafe` with the knowledge RefCell is not mutably borrowed.
        let parts = self.parts.borrow();
        match parts.get(uri) {
            Some(PartData::Loaded(d)) => {
                // Extend lifetime: data lives in RefCell as long as self lives and is not replaced.
                // Safe if no concurrent borrow_mut; single-threaded package use is assumed.
                let ptr = d.as_ptr();
                let len = d.len();
                // Drop borrow before returning raw slice
                drop(parts);
                Some(unsafe { std::slice::from_raw_parts(ptr, len) })
            }
            _ => None,
        }
    }

    /// Load a part (materializing if lazy) and return owned bytes.
    pub fn load_part(&self, uri: &PackUri) -> Result<Option<Vec<u8>>> {
        self.ensure_loaded(uri)?;
        Ok(match self.parts.borrow().get(uri) {
            Some(PartData::Loaded(d)) => Some(d.clone()),
            _ => None,
        })
    }

    /// Clone part bytes, materializing if needed.
    pub fn get_part_cloned(&self, uri: &PackUri) -> Result<Option<Vec<u8>>> {
        self.load_part(&uri)
    }

    pub fn get_part_str(&self, uri: &PackUri) -> Result<Option<&str>> {
        let Some(bytes) = self.get_part(&uri) else {
            return Ok(None);
        };
        Ok(Some(
            std::str::from_utf8(bytes)
                .map_err(|e| Error::Xml(format!("part `{uri}` is not UTF-8: {e}")))?,
        ))
    }

    /// Insert or replace a part's content and content type.
    pub fn set_part(
        &mut self,
        uri: impl Into<PackUri>,
        content_type: impl Into<String>,
        data: impl Into<Vec<u8>>,
    ) {
        let uri = uri.into();
        let content_type = content_type.into();
        self.content_types
            .set_override(uri.as_str(), content_type);
        self.parts
            .borrow_mut()
            .insert(uri, PartData::Loaded(data.into()));
        self.dirty = true;
    }

    /// Remove a part's bytes, content-type override, and its own `.rels`.
    ///
    /// Also strips **inbound** relationships (package-level and other parts) that
    /// target this URI — closer to C# `OpenXmlPartContainer.DeletePart` than a bare
    /// map remove. Does **not** cascade into children of the deleted part; use
    /// [`delete_part_and_orphans`](Self::delete_part_and_orphans) for that.
    pub fn remove_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.remove_inbound_relationships(uri);
        self.content_types.overrides.shift_remove(uri.as_str());
        self.part_rels.shift_remove(uri);
        self.data_parts.retain(|p| &p.uri != uri);
        self.dirty = true;
        match self.parts.borrow_mut().shift_remove(uri) {
            Some(PartData::Loaded(d)) => Some(d),
            Some(PartData::Lazy { .. }) => {
                // Part existed (lazy) but bytes were never materialised.
                Some(Vec::new())
            }
            None => None,
        }
    }

    /// URIs that currently own a relationships part (for whole-package scans).
    pub fn part_relationship_sources(&self) -> Vec<PackUri> {
        self.part_rels.keys().cloned().collect()
    }

    /// Remove every relationship (package or part) whose resolved internal target is `uri`.
    pub fn remove_inbound_relationships(&mut self, uri: &PackUri) {
        let target = uri.as_str();
        self.package_rels.remove_where(|r| {
            r.target_mode == RelationshipTargetMode::Internal
                && relationship_targets_uri(None, r, target)
        });
        // Collect source URIs first to avoid borrow issues while mutating.
        let sources: Vec<PackUri> = self.part_rels.keys().cloned().collect();
        for source in sources {
            if let Some(rels) = self.part_rels.get_mut(&source) {
                rels.remove_where(|r| {
                    r.target_mode == RelationshipTargetMode::Internal
                        && relationship_targets_uri(Some(&source), r, target)
                });
            }
        }
        self.dirty = true;
    }

    /// Collect absolute URIs of all parts reachable from package relationships
    /// (and optionally only from `roots` if provided).
    pub fn reachable_parts(&self, roots: Option<&[PackUri]>) -> indexmap::IndexSet<PackUri> {
        use indexmap::IndexSet;
        let mut seen: IndexSet<PackUri> = IndexSet::new();
        let mut stack: Vec<PackUri> = Vec::new();
        if let Some(roots) = roots {
            for r in roots {
                if self.has_part(r) {
                    stack.push(r.clone());
                }
            }
        } else {
            for rel in self.package_rels.iter() {
                if rel.target_mode != RelationshipTargetMode::Internal {
                    continue;
                }
                if let Ok(u) = self.resolve_relationship(None, rel) {
                    if self.has_part(&u) {
                        stack.push(u);
                    }
                }
            }
        }
        while let Some(uri) = stack.pop() {
            if !seen.insert(uri.clone()) {
                continue;
            }
            if let Some(rels) = self.part_relationships(&uri) {
                for rel in rels.iter() {
                    if rel.target_mode != RelationshipTargetMode::Internal {
                        continue;
                    }
                    if let Ok(child) = self.resolve_relationship(Some(&uri), rel) {
                        if self.has_part(&child) && !seen.contains(&child) {
                            stack.push(child);
                        }
                    }
                }
            }
        }
        seen
    }

    /// Delete `uri` and any parts that become unreachable from the package root
    /// (C# `DeletePartCore` orphan cascade, simplified).
    ///
    /// Returns the removed bytes of `uri` if it existed.
    pub fn delete_part_and_orphans(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        if !self.has_part(uri) {
            return None;
        }
        // Parts reachable *only* via the subtree starting at `uri`.
        let from_target = self.reachable_parts(Some(std::slice::from_ref(uri)));
        // Live parts if we pretend `uri` is already gone: walk from package root but
        // skip entering `uri`.
        let live = self.reachable_parts_excluding(uri);
        let mut to_delete: Vec<PackUri> = from_target
            .into_iter()
            .filter(|p| !live.contains(p))
            .collect();
        // Ensure the primary target is deleted even if still referenced (caller asked).
        if !to_delete.iter().any(|p| p == uri) {
            to_delete.push(uri.clone());
        }
        // Delete deepest paths first so inbound cleanup is predictable.
        to_delete.sort_by(|a, b| b.as_str().len().cmp(&a.as_str().len()));
        let mut primary = None;
        for p in &to_delete {
            let data = self.remove_part(p);
            if p == uri {
                primary = data;
            }
        }
        primary
    }

    /// Like [`reachable_parts`](Self::reachable_parts) from package root, but never
    /// descends into `exclude` (treats it as already deleted).
    pub fn reachable_parts_excluding(&self, exclude: &PackUri) -> indexmap::IndexSet<PackUri> {
        use indexmap::IndexSet;
        let mut seen: IndexSet<PackUri> = IndexSet::new();
        let mut stack: Vec<PackUri> = Vec::new();
        for rel in self.package_rels.iter() {
            if rel.target_mode != RelationshipTargetMode::Internal {
                continue;
            }
            if let Ok(u) = self.resolve_relationship(None, rel) {
                if &u != exclude && self.has_part(&u) {
                    stack.push(u);
                }
            }
        }
        while let Some(uri) = stack.pop() {
            if &uri == exclude || !seen.insert(uri.clone()) {
                continue;
            }
            if let Some(rels) = self.part_relationships(&uri) {
                for rel in rels.iter() {
                    if rel.target_mode != RelationshipTargetMode::Internal {
                        continue;
                    }
                    if let Ok(child) = self.resolve_relationship(Some(&uri), rel) {
                        if &child != exclude && self.has_part(&child) && !seen.contains(&child) {
                            stack.push(child);
                        }
                    }
                }
            }
        }
        seen
    }

    /// Delete the part identified by relationship `id` on `source` (or package-level
    /// when `source` is `None`), cascading orphans. Mirrors C# `DeletePart(string id)`.
    pub fn delete_part_by_id(&mut self, source: Option<&PackUri>, id: &str) -> bool {
        let rel = match source {
            Some(s) => self.part_relationships(s).and_then(|r| r.get(id)).cloned(),
            None => self.package_rels.get(id).cloned(),
        };
        let Some(rel) = rel else {
            return false;
        };
        if rel.target_mode == RelationshipTargetMode::External {
            match source {
                Some(s) => {
                    self.part_relationships_mut(s).remove(id);
                }
                None => {
                    self.package_rels.remove(id);
                }
            }
            self.dirty = true;
            return true;
        }
        let Ok(target) = self.resolve_relationship(source, &rel) else {
            return false;
        };
        // Drop the relationship first so orphan detection sees it gone.
        match source {
            Some(s) => {
                self.part_relationships_mut(s).remove(id);
            }
            None => {
                self.package_rels.remove(id);
            }
        }
        self.dirty = true;
        if self.has_part(&target) {
            // Orphans relative to remaining graph (relationship already removed).
            let live = self.reachable_parts(None);
            let from_target = self.reachable_parts(Some(std::slice::from_ref(&target)));
            let mut to_delete: Vec<PackUri> = from_target
                .into_iter()
                .filter(|p| !live.contains(p))
                .collect();
            if to_delete.is_empty() && !live.contains(&target) {
                to_delete.push(target);
            } else if !to_delete.iter().any(|p| p == &target) && !live.contains(&target) {
                to_delete.push(target);
            }
            to_delete.sort_by(|a, b| b.as_str().len().cmp(&a.as_str().len()));
            for p in to_delete {
                // remove_part also strips any remaining inbound refs
                let _ = self.remove_part(&p);
            }
        }
        true
    }

    /// Delete every part whose content type equals `content_type`, cascading orphans.
    ///
    /// Approximate stand-in for C# `DeletePartsRecursivelyOfType<T>` when T is known
    /// only by content type (Rust has no generic OpenXmlPart hierarchy).
    /// Delete multiple parts by URI (C# `DeleteParts`). Each URI is removed with
    /// inbound relationship cleanup; does not cascade orphans unless you also call
    /// [`delete_part_and_orphans`](Self::delete_part_and_orphans) per part.
    pub fn delete_parts(&mut self, uris: &[PackUri]) -> usize {
        let mut n = 0;
        for uri in uris {
            if self.remove_part(uri).is_some() {
                n += 1;
            }
        }
        n
    }

    /// Delete parts by relationship ids under `source` (C# `DeleteParts` via ids).
    pub fn delete_parts_by_ids(&mut self, source: Option<&PackUri>, ids: &[&str]) -> usize {
        let mut n = 0;
        for id in ids {
            if self.delete_part_by_id(source, id) {
                n += 1;
            }
        }
        n
    }

    pub fn delete_parts_of_content_type(&mut self, content_type: &str) -> usize {
        let mut uris: Vec<PackUri> = self
            .part_uris()
            .into_iter()
            .filter(|u| {
                self.content_types
                    .content_type_for(u.as_str())
                    .map(|ct| ct == content_type)
                    .unwrap_or(false)
            })
            .collect();
        uris.sort_by(|a, b| b.as_str().len().cmp(&a.as_str().len()));
        let mut n = 0;
        for u in uris {
            if self.has_part(&u) {
                let _ = self.delete_part_and_orphans(&u);
                n += 1;
            }
        }
        n
    }

    /// Delete every internal relationship of `relationship_type` under `source`
    /// (package-level if `source` is `None`), cascading part deletion.
    pub fn delete_parts_of_relationship_type(
        &mut self,
        source: Option<&PackUri>,
        relationship_type: &str,
    ) -> usize {
        let ids: Vec<String> = match source {
            Some(s) => self
                .part_relationships(s)
                .map(|rels| {
                    rels.find_all_by_type(relationship_type)
                        .into_iter()
                        .map(|r| r.id.clone())
                        .collect()
                })
                .unwrap_or_default(),
            None => self
                .package_rels
                .find_all_by_type(relationship_type)
                .into_iter()
                .map(|r| r.id.clone())
                .collect(),
        };
        let mut n = 0;
        for id in ids {
            if self.delete_part_by_id(source, &id) {
                n += 1;
            }
        }
        n
    }

    /// Add an external relationship from `source` (or package-level if `None`).
    ///
    /// Mirrors C# `OpenXmlPartContainer.AddExternalRelationship`.
    pub fn add_external_relationship(
        &mut self,
        source: Option<&PackUri>,
        relationship_type: &str,
        external_uri: &str,
    ) -> String {
        let id = match source {
            Some(s) => self
                .part_relationships_mut(s)
                .add(
                    relationship_type,
                    external_uri,
                    RelationshipTargetMode::External,
                )
                .id
                .clone(),
            None => self
                .package_rels
                .add(
                    relationship_type,
                    external_uri,
                    RelationshipTargetMode::External,
                )
                .id
                .clone(),
        };
        self.dirty = true;
        id
    }

    /// Add an external relationship with an explicit id.
    pub fn add_external_relationship_with_id(
        &mut self,
        source: Option<&PackUri>,
        id: &str,
        relationship_type: &str,
        external_uri: &str,
    ) -> String {
        let id = match source {
            Some(s) => self
                .part_relationships_mut(s)
                .add_with_id(
                    id,
                    relationship_type,
                    external_uri,
                    RelationshipTargetMode::External,
                )
                .id
                .clone(),
            None => self
                .package_rels
                .add_with_id(
                    id,
                    relationship_type,
                    external_uri,
                    RelationshipTargetMode::External,
                )
                .id
                .clone(),
        };
        self.dirty = true;
        id
    }

    /// List external relationships on `source` (package-level if `None`).
    pub fn external_relationships(&self, source: Option<&PackUri>) -> Vec<&Relationship> {
        match source {
            Some(s) => self
                .part_relationships(s)
                .map(|r| r.external())
                .unwrap_or_default(),
            None => self.package_rels.external(),
        }
    }

    /// Delete a relationship by id from `source` (or package-level). Does not delete
    /// the target part — use [`delete_part_by_id`](Self::delete_part_by_id) for that.
    pub fn delete_relationship(
        &mut self,
        source: Option<&PackUri>,
        id: &str,
    ) -> Option<Relationship> {
        let removed = match source {
            Some(s) => self.part_relationships_mut(s).remove(id),
            None => self.package_rels.remove(id),
        };
        if removed.is_some() {
            self.dirty = true;
        }
        removed
    }

    /// Resolve the part targeted by relationship `id` on `source` (C# `GetPartById` /
    /// `TryGetPartById`). Returns `None` for missing ids or external targets.
    pub fn get_part_by_id(
        &self,
        source: Option<&PackUri>,
        id: &str,
    ) -> Option<PackUri> {
        let rel = match source {
            Some(s) => self.part_relationships(s)?.get(id)?,
            None => self.package_rels.get(id)?,
        };
        if rel.target_mode == RelationshipTargetMode::External {
            return None;
        }
        self.resolve_relationship(source, rel).ok()
    }

    /// Relationship id of the first internal relationship from `source` whose target
    /// resolves to `part_uri` (C# `GetIdOfPart`).
    pub fn get_id_of_part(
        &self,
        source: Option<&PackUri>,
        part_uri: &PackUri,
    ) -> Option<String> {
        let rels: Box<dyn Iterator<Item = &Relationship> + '_> = match source {
            Some(s) => Box::new(self.part_relationships(s)?.iter()),
            None => Box::new(self.package_rels.iter()),
        };
        for rel in rels {
            if rel.target_mode != RelationshipTargetMode::Internal {
                continue;
            }
            if let Ok(u) = self.resolve_relationship(source, rel) {
                if &u == part_uri {
                    return Some(rel.id.clone());
                }
            }
        }
        None
    }

    /// Change the relationship id of an existing relationship from `source` to
    /// `part_uri` (C# `ChangeIdOfPart`). Returns the previous id.
    pub fn change_id_of_part(
        &mut self,
        source: Option<&PackUri>,
        part_uri: &PackUri,
        new_id: &str,
    ) -> Result<String> {
        if new_id.is_empty() {
            return Err(Error::Package(
                "relationship id must be non-empty".into(),
            ));
        }
        // Conflict check + find old id.
        let existing = match source {
            Some(s) => self.part_relationships(s).and_then(|r| r.get(new_id)).is_some(),
            None => self.package_rels.get(new_id).is_some(),
        };
        if existing {
            return Err(Error::Package(format!(
                "relationship id `{new_id}` already in use"
            )));
        }
        let old_id = self
            .get_id_of_part(source, part_uri)
            .ok_or_else(|| {
                Error::Package(format!(
                    "part `{}` is not related from the given source",
                    part_uri.as_str()
                ))
            })?;
        if old_id == new_id {
            return Ok(old_id);
        }
        let removed = match source {
            Some(s) => self.part_relationships_mut(s).remove(&old_id),
            None => self.package_rels.remove(&old_id),
        }
        .ok_or_else(|| Error::Package(format!("relationship `{old_id}` vanished")))?;
        match source {
            Some(s) => {
                self.part_relationships_mut(s).add_with_id(
                    new_id,
                    removed.relationship_type,
                    removed.target,
                    removed.target_mode,
                );
            }
            None => {
                self.package_rels.add_with_id(
                    new_id,
                    removed.relationship_type,
                    removed.target,
                    removed.target_mode,
                );
            }
        }
        self.dirty = true;
        Ok(old_id)
    }

    /// Create a relationship from `source` to an existing internal part (C#
    /// `CreateRelationshipToPart`).
    pub fn create_relationship_to_part(
        &mut self,
        source: &PackUri,
        target: &PackUri,
        relationship_type: &str,
        id: Option<&str>,
    ) -> String {
        if let Some(id) = id {
            self.part_relationships_mut(source)
                .add_with_id(
                    id,
                    relationship_type,
                    super::uri::relativize(source, target),
                    RelationshipTargetMode::Internal,
                )
                .id
                .clone()
        } else {
            self.add_part_relationship(
                source,
                relationship_type,
                target,
                RelationshipTargetMode::Internal,
            )
        }
    }

    pub fn part_relationships(&self, part: &PackUri) -> Option<&Relationships> {
        self.part_rels.get(part)
    }

    pub fn part_relationships_mut(&mut self, part: &PackUri) -> &mut Relationships {
        self.dirty = true;
        self.part_rels.entry(part.clone()).or_default()
    }

    /// Add a package-level relationship and return its id.
    pub fn add_package_relationship(
        &mut self,
        relationship_type: &str,
        target: &PackUri,
        target_mode: RelationshipTargetMode,
    ) -> String {
        let target_str = match target_mode {
            RelationshipTargetMode::Internal => target.as_str().trim_start_matches('/').to_string(),
            RelationshipTargetMode::External => target.as_str().to_string(),
        };
        let rel = self
            .package_rels
            .add(relationship_type, target_str, target_mode);
        self.dirty = true;
        rel.id.clone()
    }

    /// Add a relationship from `source` to `target`.
    pub fn add_part_relationship(
        &mut self,
        source: &PackUri,
        relationship_type: &str,
        target: &PackUri,
        target_mode: RelationshipTargetMode,
    ) -> String {
        let target_str = match target_mode {
            RelationshipTargetMode::Internal => relativize(source, target),
            RelationshipTargetMode::External => target.as_str().to_string(),
        };
        let rel = self
            .part_relationships_mut(source)
            .add(relationship_type, target_str, target_mode);
        rel.id.clone()
    }

    /// Resolve a relationship target to an absolute pack URI.
    pub fn resolve_relationship(
        &self,
        source: Option<&PackUri>,
        rel: &Relationship,
    ) -> Result<PackUri> {
        if rel.target_mode == RelationshipTargetMode::External {
            return Ok(PackUri::new(&rel.target));
        }
        let root = PackUri::new("/");
        let source = source.unwrap_or(&root);
        // Package-level targets are relative to package root
        if source.as_str() == "/" {
            Ok(PackUri::new(format!("/{}", rel.target.trim_start_matches('/'))))
        } else {
            resolve_uri(source, &rel.target)
        }
    }

    /// Find the target of the first package relationship of the given type.
    pub fn main_part_uri(&self, relationship_type: &str) -> Result<PackUri> {
        let rel = self
            .package_rels
            .get_by_type(relationship_type)
            .ok_or_else(|| Error::RelationshipNotFound(relationship_type.into()))?;
        self.resolve_relationship(None, rel)
    }

    /// Save the package to its path, or to `path` if provided.
    pub fn save(&mut self) -> Result<()> {
        let path = self
            .path
            .clone()
            .ok_or_else(|| Error::Package("no path associated with package; use save_as".into()))?;
        self.save_as(path)
    }

    /// Save the package to a file path.
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let data = self.to_bytes()?;
        // Write via temp file for safety
        let tmp = path.with_extension("tmp-openxml");
        {
            let mut f = File::create(&tmp)?;
            f.write_all(&data)?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, path)?;
        self.path = Some(path.to_path_buf());
        self.mode = PackageMode::ReadWrite;
        self.dirty = false;
        Ok(())
    }

    /// ZIP compression option for ordinary parts (C# `OpenXmlPackage.CompressionOption`).
    pub fn compression_option(&self) -> CompressionOption {
        self.compression
    }

    pub fn set_compression_option(&mut self, option: CompressionOption) {
        self.compression = option;
        self.dirty = true;
    }

    /// Write the package ZIP to any `Write` sink (C# stream save surface).
    pub fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        let bytes = self.to_bytes()?;
        writer.write_all(&bytes)?;
        Ok(())
    }

    fn deflate_options(&self) -> SimpleFileOptions {
        let method = match self.compression {
            CompressionOption::NotCompressed => CompressionMethod::Stored,
            CompressionOption::Fast
            | CompressionOption::Normal
            | CompressionOption::Maximum
            | CompressionOption::SuperFast => CompressionMethod::Deflated,
        };
        SimpleFileOptions::default()
            .compression_method(method)
            .unix_permissions(0o644)
    }

    /// Serialize the package to ZIP bytes.
    ///
    /// Lazy parts are decompressed on the fly for the write (the package itself is not mutated).
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let cursor = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(cursor);
        let options = self.deflate_options();
        // Office stores media (images, etc.) uncompressed (Stored).
        let media_options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o644);
        // PowerPoint also stores embedded fonts (`.fntdata` / `.odttf`) uncompressed.
        let font_options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o644);

        // [Content_Types].xml
        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(&self.content_types.to_xml()?)?;

        // Package relationships
        zip.start_file("_rels/.rels", options)?;
        zip.write_all(&self.package_rels.to_xml()?)?;

        // Parts
        let part_uris: Vec<PackUri> = self.parts.borrow().keys().cloned().collect();
        for uri in part_uris {
            let bytes = self
                .load_part(&uri)?
                .ok_or_else(|| Error::Package(format!("missing part `{uri}`")))?;
            let path = uri.as_str();
            let opts = if path.contains("/media/") {
                media_options
            } else if path.contains("/fonts/")
                || path.ends_with(".fntdata")
                || path.ends_with(".odttf")
            {
                font_options
            } else {
                options
            };
            zip.start_file(uri.zip_name(), opts)?;
            zip.write_all(&bytes)?;
        }

        // Part relationships
        for (part_uri, rels) in &self.part_rels {
            if rels.is_empty() {
                continue;
            }
            let rel_uri = part_uri.relationship_part_uri();
            zip.start_file(rel_uri.zip_name(), options)?;
            zip.write_all(&rels.to_xml()?)?;
        }

        let cursor = zip.finish()?;
        Ok(cursor.into_inner())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

/// Whether an internal relationship resolves to absolute pack URI `target`.
fn relationship_targets_uri(
    source: Option<&PackUri>,
    rel: &Relationship,
    target: &str,
) -> bool {
    if rel.target_mode != RelationshipTargetMode::Internal {
        return false;
    }
    // Fast path: absolute or same string forms.
    let t = rel.target.trim_start_matches('/');
    let want = target.trim_start_matches('/');
    if t == want || rel.target == target {
        return true;
    }
    match source {
        None => {
            // Package-level targets are relative to package root.
            format!("/{t}") == target || rel.target == target
        }
        Some(src) => resolve_uri(src, &rel.target)
            .map(|u| u.as_str() == target)
            .unwrap_or(false),
    }
}

/// Convert `/word/_rels/document.xml.rels` → `/word/document.xml`.
fn part_uri_from_rels_uri(rels_uri: &PackUri) -> PackUri {
    let s = rels_uri.as_str();
    // Pattern: {dir}/_rels/{name}.rels
    if let Some(idx) = s.find("/_rels/") {
        let dir = &s[..idx];
        let rest = &s[idx + "/_rels/".len()..];
        let name = rest.strip_suffix(".rels").unwrap_or(rest);
        if dir.is_empty() {
            PackUri::new(format!("/{name}"))
        } else {
            PackUri::new(format!("{dir}/{name}"))
        }
    } else {
        PackUri::new("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::{content_type, rel};

    #[test]
    fn create_and_roundtrip() {
        let mut pkg = OpcPackage::create();
        pkg.set_part(
            "/word/document.xml",
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#.to_vec(),
        );
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &PackUri::new("/word/document.xml"),
            RelationshipTargetMode::Internal,
        );

        let bytes = pkg.to_bytes().unwrap();
        let opened = OpcPackage::open_bytes(&bytes).unwrap();
        assert!(opened.has_part(&PackUri::new("/word/document.xml")));
        let main = opened.main_part_uri(rel::OFFICE_DOCUMENT).unwrap();
        assert_eq!(main.as_str(), "/word/document.xml");
    }

    #[test]
    fn remove_part_strips_inbound_rels() {
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
        assert!(pkg
            .part_relationships(&doc)
            .unwrap()
            .get_by_type(rel::STYLES)
            .is_some());
        let _ = pkg.remove_part(&styles);
        assert!(!pkg.has_part(&styles));
        assert!(pkg
            .part_relationships(&doc)
            .unwrap()
            .get_by_type(rel::STYLES)
            .is_none());
    }

    #[test]
    fn delete_part_and_orphans_cascades() {
        let mut pkg = OpcPackage::create();
        let slide = PackUri::new("/ppt/slides/slide1.xml");
        let chart = PackUri::new("/ppt/charts/chart1.xml");
        let drawing = PackUri::new("/ppt/charts/_rels/unused-child.xml");
        // Use a real child under chart
        let colors = PackUri::new("/ppt/charts/colors1.xml");
        pkg.set_part(
            PackUri::new("/ppt/presentation.xml"),
            content_type::PRESENTATION,
            b"<p:presentation/>".to_vec(),
        );
        pkg.set_part(slide.clone(), content_type::PRESENTATION_SLIDE, b"<p:sld/>".to_vec());
        pkg.set_part(
            chart.clone(),
            content_type::DRAWINGML_CHART,
            b"<c:chartSpace/>".to_vec(),
        );
        pkg.set_part(
            colors.clone(),
            content_type::CHART_COLOR_STYLE,
            b"<c:colorStyle/>".to_vec(),
        );
        let _ = drawing;
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &PackUri::new("/ppt/presentation.xml"),
            RelationshipTargetMode::Internal,
        );
        pkg.add_part_relationship(
            &PackUri::new("/ppt/presentation.xml"),
            rel::SLIDE,
            &slide,
            RelationshipTargetMode::Internal,
        );
        pkg.add_part_relationship(
            &slide,
            rel::CHART,
            &chart,
            RelationshipTargetMode::Internal,
        );
        // Private child only reachable from chart
        pkg.add_part_relationship(
            &chart,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartColorStyle",
            &colors,
            RelationshipTargetMode::Internal,
        );
        let _ = pkg.delete_part_and_orphans(&chart);
        assert!(!pkg.has_part(&chart));
        assert!(!pkg.has_part(&colors));
        assert!(pkg.has_part(&slide));
        assert!(pkg
            .part_relationships(&slide)
            .unwrap()
            .get_by_type(rel::CHART)
            .is_none());
    }

    #[test]
    fn external_relationship_roundtrip() {
        let mut pkg = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        let id = pkg.add_external_relationship(
            Some(&doc),
            rel::HYPERLINK,
            "https://example.com/a",
        );
        let ext = pkg.external_relationships(Some(&doc));
        assert_eq!(ext.len(), 1);
        assert_eq!(ext[0].id, id);
        assert_eq!(ext[0].target, "https://example.com/a");
        assert_eq!(ext[0].target_mode, RelationshipTargetMode::External);
        assert!(pkg.delete_relationship(Some(&doc), &id).is_some());
        assert!(pkg.external_relationships(Some(&doc)).is_empty());
    }

    #[test]
    fn change_id_of_part_roundtrip() {
        let mut pkg = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        pkg.set_part(doc.clone(), content_type::WORD_DOCUMENT, b"<w:document/>".to_vec());
        pkg.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        let old = pkg.add_part_relationship(
            &doc,
            rel::STYLES,
            &styles,
            RelationshipTargetMode::Internal,
        );
        assert_eq!(pkg.get_id_of_part(Some(&doc), &styles).as_deref(), Some(old.as_str()));
        assert_eq!(pkg.get_part_by_id(Some(&doc), &old), Some(styles.clone()));
        let prev = pkg.change_id_of_part(Some(&doc), &styles, "rIdStyles").unwrap();
        assert_eq!(prev, old);
        assert_eq!(pkg.get_id_of_part(Some(&doc), &styles).as_deref(), Some("rIdStyles"));
        assert!(pkg.get_part_by_id(Some(&doc), &old).is_none());
        assert_eq!(pkg.get_part_by_id(Some(&doc), "rIdStyles"), Some(styles));
    }


    #[test]

    #[test]
    fn delete_parts_batch() {
        let mut pkg = OpcPackage::create();
        let a = PackUri::new("/word/a.xml");
        let b = PackUri::new("/word/b.xml");
        pkg.set_part(a.clone(), "application/xml", b"<a/>".to_vec());
        pkg.set_part(b.clone(), "application/xml", b"<b/>".to_vec());
        assert_eq!(pkg.delete_parts(&[a.clone(), b.clone()]), 2);
        assert!(!pkg.has_part(&a));
        assert!(!pkg.has_part(&b));
    }

    fn lazy_open_defers_parts() {
        let mut pkg = OpcPackage::create();
        pkg.set_part(
            "/word/document.xml",
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#.to_vec(),
        );
        pkg.set_part(
            "/word/styles.xml",
            content_type::WORD_STYLES,
            br#"<?xml version="1.0"?><w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#.to_vec(),
        );
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &PackUri::new("/word/document.xml"),
            RelationshipTargetMode::Internal,
        );
        let bytes = pkg.to_bytes().unwrap();
        let lazy = OpcPackage::open_bytes_lazy(&bytes).unwrap();
        assert!(lazy.has_lazy_parts());
        assert!(lazy.has_part(&PackUri::new("/word/document.xml")));
        // get_part materializes on access
        let data = lazy
            .get_part(&PackUri::new("/word/document.xml"))
            .expect("document");
        assert!(data.windows(7).any(|w| w == b"<w:body"));
        assert!(!lazy.has_lazy_parts() || lazy.get_part(&PackUri::new("/word/styles.xml")).is_some());
        // Round-trip still works with remaining lazy parts
        let again = lazy.to_bytes().unwrap();
        let opened = OpcPackage::open_bytes(&again).unwrap();
        assert!(opened.has_part(&PackUri::new("/word/styles.xml")));
    }
}
