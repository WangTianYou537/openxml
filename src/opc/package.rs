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

        Ok(Self {
            mode: PackageMode::Read,
            path: None,
            parts: RefCell::new(parts),
            zip_bytes: RefCell::new(if lazy { Some(zip_bytes) } else { None }),
            content_types,
            package_rels,
            part_rels,
            dirty: false,
        })
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

    pub fn remove_part(&mut self, uri: &PackUri) -> Option<Vec<u8>> {
        self.content_types.overrides.shift_remove(uri.as_str());
        self.part_rels.shift_remove(uri);
        self.dirty = true;
        match self.parts.borrow_mut().shift_remove(uri) {
            Some(PartData::Loaded(d)) => Some(d),
            Some(PartData::Lazy { .. }) => None,
            None => None,
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

    /// Serialize the package to ZIP bytes.
    ///
    /// Lazy parts are decompressed on the fly for the write (the package itself is not mutated).
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let cursor = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(cursor);
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644);
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
