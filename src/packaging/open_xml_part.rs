//! OpenXmlPart — a single part within an Open XML package.

use crate::element::{parse_element, write_element, write_element_fragment, OpenXmlElement};
use crate::error::{Error, Result};
use crate::opc::{PackUri, RelationshipTargetMode};
use crate::packaging::OpenXmlPackage;
use std::io::Write;

/// A part in an Open XML package (e.g. `/word/document.xml`).
#[derive(Debug)]
pub struct OpenXmlPart {
    pub(crate) uri: PackUri,
    pub(crate) content_type: String,
    pub(crate) relationship_type: String,
    /// Loaded root element (lazily populated).
    pub(crate) root: Option<OpenXmlElement>,
    pub(crate) dirty: bool,
    /// XML declaration standalone flag when known (C# `OpenXmlPartRootElement` shell).
    pub(crate) standalone_declaration: Option<bool>,
}

impl OpenXmlPart {
    pub fn new(
        uri: impl Into<PackUri>,
        content_type: impl Into<String>,
        relationship_type: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            content_type: content_type.into(),
            relationship_type: relationship_type.into(),
            root: None,
            dirty: false,
            standalone_declaration: Some(true),
        }
    }

    pub fn uri(&self) -> &PackUri {
        &self.uri
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn relationship_type(&self) -> &str {
        &self.relationship_type
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Access the root element, loading from the package if needed.
    pub fn root(&mut self, package: &OpenXmlPackage) -> Result<&OpenXmlElement> {
        if self.root.is_none() {
            self.load(package)?;
        }
        self.root
            .as_ref()
            .ok_or(Error::NoRootElement)
    }

    pub fn root_mut(&mut self, package: &OpenXmlPackage) -> Result<&mut OpenXmlElement> {
        if self.root.is_none() {
            self.load(package)?;
        }
        self.dirty = true;
        self.root
            .as_mut()
            .ok_or(Error::NoRootElement)
    }

    /// Set the root element directly (marks dirty).
    pub fn set_root(&mut self, element: OpenXmlElement) {
        self.root = Some(element);
        self.dirty = true;
    }

    pub fn take_root(&mut self) -> Option<OpenXmlElement> {
        self.root.take()
    }

    fn load(&mut self, package: &OpenXmlPackage) -> Result<()> {
        package.raise_part_root_event(
            crate::features::PackageEventType::Creating,
            self.uri.as_str(),
        );
        let data = package
            .opc()
            .get_part(&self.uri)
            .ok_or_else(|| Error::PartNotFound(self.uri.to_string()))?;
        let limit = package.settings().max_characters_in_part;
        if limit > 0 {
            // Approximate character count as UTF-8 bytes for DoS guard (matches C# spirit).
            if data.len() as u64 > limit {
                return Err(Error::PartTooLarge {
                    uri: self.uri.to_string(),
                    limit,
                });
            }
        }
        self.standalone_declaration = parse_xml_standalone(data);
        let mut element = parse_element(data)?;
        // Optional MC processing on load
        use crate::packaging::open_xml_package::MarkupCompatibilityProcessMode;
        if package.settings().markup_compatibility.mode
            == MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly
        {
            crate::markup_compatibility::process_markup_compatibility_for_version(
                &mut element,
                package
                    .settings()
                    .markup_compatibility
                    .target_file_format_versions,
            );
        }
        self.root = Some(element);
        self.dirty = false;
        package.raise_part_root_event(
            crate::features::PackageEventType::Created,
            self.uri.as_str(),
        );
        Ok(())
    }

    /// Write the root element back into the package if dirty.
    pub fn save_to_package(&mut self, package: &mut OpenXmlPackage) -> Result<()> {
        if !self.dirty {
            if let Some(root) = &self.root {
                // Still ensure part exists
                if !package.opc().has_part(&self.uri) {
                    let xml = write_element(root)?;
                    package.set_part(self.uri.clone(), self.content_type.clone(), xml);
                }
            }
            return Ok(());
        }
        let root = self.root.as_ref().ok_or(Error::NoRootElement)?;
        let xml = write_element(root)?;
        package.set_part(self.uri.clone(), self.content_type.clone(), xml);
        self.dirty = false;
        Ok(())
    }

    /// Get raw part bytes from the package (without parsing).
    pub fn get_stream(&self, package: &OpenXmlPackage) -> Result<Vec<u8>> {
        if self.dirty {
            if let Some(root) = &self.root {
                return write_element(root);
            }
        }
        package
            .opc()
            .get_part(&self.uri)
            .map(|b| b.to_vec())
            .ok_or_else(|| Error::PartNotFound(self.uri.to_string()))
    }

    /// Replace part content with raw bytes (clears loaded root).
    pub fn feed_data(&mut self, package: &mut OpenXmlPackage, data: impl Into<Vec<u8>>) {
        package.set_part(self.uri.clone(), self.content_type.clone(), data.into());
        self.root = None;
        self.dirty = false;
    }

    /// Whether the DOM root is currently loaded (C# `IsRootElementLoaded`).
    pub fn is_root_element_loaded(&self) -> bool {
        self.root.is_some()
    }

    /// Unload the DOM root, returning it if present (C# `UnloadRootElement`).
    ///
    /// Does **not** write dirty changes; call [`save_to_package`](Self::save_to_package) first
    /// if the in-memory tree must be persisted.
    pub fn unload_root_element(&mut self) -> Option<OpenXmlElement> {
        self.dirty = false;
        self.root.take()
    }

    /// Unload the DOM root and raise part-root Removing/Removed events when a hub is registered.
    pub fn unload_root_element_with_events(
        &mut self,
        package: &OpenXmlPackage,
    ) -> Option<OpenXmlElement> {
        package.raise_part_root_event(
            crate::features::PackageEventType::Removing,
            self.uri.as_str(),
        );
        let taken = self.unload_root_element();
        package.raise_part_root_event(
            crate::features::PackageEventType::Removed,
            self.uri.as_str(),
        );
        taken
    }

    /// Parent part URIs that reference this part (C# `GetParentParts`).
    pub fn get_parent_parts(&self, package: &OpenXmlPackage) -> Vec<PackUri> {
        package.opc().parent_parts(&self.uri)
    }

    /// Optional view of the loaded root without loading from the package.
    pub fn root_element(&self) -> Option<&OpenXmlElement> {
        self.root.as_ref()
    }

    /// Reload the DOM root from package bytes (C# `OpenXmlPartRootElement.Reload`).
    ///
    /// Discards any unsaved in-memory edits.
    pub fn reload(&mut self, package: &OpenXmlPackage) -> Result<()> {
        package.raise_part_root_event(
            crate::features::PackageEventType::Reloading,
            self.uri.as_str(),
        );
        self.root = None;
        self.dirty = false;
        self.load(package)?;
        package.raise_part_root_event(
            crate::features::PackageEventType::Reloaded,
            self.uri.as_str(),
        );
        Ok(())
    }

    /// Save the current root element XML into the package (C# `OpenXmlPartRootElement.Save`).
    pub fn save_root(&mut self, package: &mut OpenXmlPackage) -> Result<()> {
        package.raise_part_root_event(
            crate::features::PackageEventType::Saving,
            self.uri.as_str(),
        );
        self.dirty = true;
        self.save_to_package(package)?;
        package.raise_part_root_event(
            crate::features::PackageEventType::Saved,
            self.uri.as_str(),
        );
        Ok(())
    }

    /// Standalone flag used when writing the XML declaration (C# part-root `_standaloneDeclaration`).
    pub fn standalone_declaration(&self) -> Option<bool> {
        self.standalone_declaration
    }

    pub fn set_standalone_declaration(&mut self, standalone: Option<bool>) {
        self.standalone_declaration = standalone;
    }

    /// Serialize the loaded root to a stream with an XML declaration (C# `OpenXmlPartRootElement.Save(Stream)`).
    pub fn save_to_stream<W: std::io::Write>(&self, mut writer: W) -> Result<()> {
        let root = self.root.as_ref().ok_or(Error::NoRootElement)?;
        let standalone = match self.standalone_declaration {
            Some(true) => r#" standalone="yes""#,
            Some(false) => r#" standalone="no""#,
            None => "",
        };
        write!(
            writer,
            r#"<?xml version="1.0" encoding="UTF-8"{standalone}?>"#
        )
        .map_err(Error::Io)?;
        let body = write_element_fragment(root)?;
        writer.write_all(&body).map_err(Error::Io)?;
        Ok(())
    }

    /// Save root XML into the package part, forcing a write even when not dirty
    /// (C# `SaveToPart` / forced save).
    pub fn save_to_part(&mut self, package: &mut OpenXmlPackage) -> Result<()> {
        package.raise_part_root_event(
            crate::features::PackageEventType::Saving,
            self.uri.as_str(),
        );
        let root = self.root.as_ref().ok_or(Error::NoRootElement)?;
        let mut buf = Vec::new();
        {
            let standalone = match self.standalone_declaration {
                Some(true) => r#" standalone="yes""#,
                Some(false) => r#" standalone="no""#,
                None => "",
            };
            write!(
                buf,
                r#"<?xml version="1.0" encoding="UTF-8"{standalone}?>"#
            )
            .map_err(Error::Io)?;
            let body = write_element_fragment(root)?;
            buf.extend_from_slice(&body);
        }
        package.set_part(self.uri.clone(), self.content_type.clone(), buf);
        self.dirty = false;
        package.raise_part_root_event(
            crate::features::PackageEventType::Saved,
            self.uri.as_str(),
        );
        Ok(())
    }
}

/// Parse `standalone` from a leading XML declaration, if present.
fn parse_xml_standalone(xml: &[u8]) -> Option<bool> {
    let s = std::str::from_utf8(xml).ok()?;
    let trimmed = s.trim_start();
    if !trimmed.starts_with("<?xml") {
        return None;
    }
    let end = trimmed.find("?>")?;
    let decl = &trimmed[..end];
    // Match standalone="yes|no" (case-insensitive value)
    let lower = decl.to_ascii_lowercase();
    if let Some(idx) = lower.find("standalone") {
        let rest = &decl[idx + "standalone".len()..];
        let rest = rest.trim_start();
        let rest = rest.strip_prefix('=')?.trim_start();
        let quote = rest.chars().next()?;
        if quote != '"' && quote != '\'' {
            return None;
        }
        let val = rest[1..].split(quote).next()?.trim();
        return Some(val.eq_ignore_ascii_case("yes"));
    }
    None
}

/// Application-specific extended part (C# `ExtendedPart`).
///
/// Default target path is `udata/data*.dat` under the source part's directory.
#[derive(Debug)]
pub struct ExtendedPart {
    inner: OpenXmlPart,
}

impl ExtendedPart {
    pub const DEFAULT_EXTENSION: &'static str = ".dat";
    pub const DEFAULT_PATH: &'static str = "udata";
    pub const DEFAULT_NAME: &'static str = "data";

    pub fn new(
        uri: impl Into<PackUri>,
        content_type: impl Into<String>,
        relationship_type: impl Into<String>,
    ) -> Self {
        Self {
            inner: OpenXmlPart::new(uri, content_type, relationship_type),
        }
    }

    pub fn from_part(part: OpenXmlPart) -> Self {
        Self { inner: part }
    }

    pub fn part(&self) -> &OpenXmlPart {
        &self.inner
    }

    pub fn part_mut(&mut self) -> &mut OpenXmlPart {
        &mut self.inner
    }

    pub fn uri(&self) -> &PackUri {
        self.inner.uri()
    }

    pub fn content_type(&self) -> &str {
        self.inner.content_type()
    }

    pub fn relationship_type(&self) -> &str {
        self.inner.relationship_type()
    }

    pub fn feed_data(&mut self, package: &mut OpenXmlPackage, data: impl Into<Vec<u8>>) {
        self.inner.feed_data(package, data)
    }

    pub fn get_stream(&self, package: &OpenXmlPackage) -> Result<Vec<u8>> {
        self.inner.get_stream(package)
    }
}


/// The main document part of a WordprocessingDocument (`/word/document.xml`).
#[derive(Debug)]
pub struct MainDocumentPart {
    inner: OpenXmlPart,
}

impl MainDocumentPart {
    pub const RELATIONSHIP_TYPE: &'static str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
    pub const URI: &'static str = "/word/document.xml";

    pub fn new(content_type: impl Into<String>) -> Self {
        Self {
            inner: OpenXmlPart::new(
                Self::URI,
                content_type,
                Self::RELATIONSHIP_TYPE,
            ),
        }
    }

    pub fn from_part(part: OpenXmlPart) -> Self {
        Self { inner: part }
    }

    pub fn part(&self) -> &OpenXmlPart {
        &self.inner
    }

    pub fn part_mut(&mut self) -> &mut OpenXmlPart {
        &mut self.inner
    }

    pub fn uri(&self) -> &PackUri {
        self.inner.uri()
    }

    /// Get the `w:document` root element.
    pub fn document(&mut self, package: &OpenXmlPackage) -> Result<&OpenXmlElement> {
        self.inner.root(package)
    }

    pub fn document_mut(&mut self, package: &OpenXmlPackage) -> Result<&mut OpenXmlElement> {
        self.inner.root_mut(package)
    }

    pub fn set_document(&mut self, document: OpenXmlElement) {
        self.inner.set_root(document);
    }

    pub fn save_to_package(&mut self, package: &mut OpenXmlPackage) -> Result<()> {
        self.inner.save_to_package(package)
    }

    /// Add a relationship from this part to another part.
    pub fn add_part_relationship(
        &self,
        package: &mut OpenXmlPackage,
        relationship_type: &str,
        target: &PackUri,
    ) -> String {
        package.opc_mut().add_part_relationship(
            &self.inner.uri,
            relationship_type,
            target,
            RelationshipTargetMode::Internal,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::{content_type, rel};
    use crate::opc::{OpcPackage, RelationshipTargetMode};
    use crate::packaging::OpenXmlPackage;

    #[test]
    fn unload_root_and_parent_parts() {
        let mut opc = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        let styles = PackUri::new("/word/styles.xml");
        opc.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#.to_vec(),
        );
        opc.set_part(styles.clone(), content_type::WORD_STYLES, b"<w:styles/>".to_vec());
        opc.add_part_relationship(&doc, rel::STYLES, &styles, RelationshipTargetMode::Internal);
        let pkg = OpenXmlPackage::from_opc(opc, Default::default());

        let mut part = OpenXmlPart::new(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            rel::OFFICE_DOCUMENT,
        );
        assert!(!part.is_root_element_loaded());
        let _ = part.root(&pkg).unwrap();
        assert!(part.is_root_element_loaded());
        assert!(part.unload_root_element().is_some());
        assert!(!part.is_root_element_loaded());

        assert_eq!(pkg.opc().parent_parts(&styles), vec![doc.clone()]);
        assert!(pkg.opc().is_child_part(&doc, &styles));
        assert_eq!(part.get_parent_parts(&pkg), Vec::<PackUri>::new());
    }


    #[test]
    fn reload_discards_edits() {
        let mut opc = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        opc.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p/></w:body></w:document>"#.to_vec(),
        );
        let mut pkg = OpenXmlPackage::from_opc(opc, Default::default());
        let mut part = OpenXmlPart::new(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            rel::OFFICE_DOCUMENT,
        );
        {
            let root = part.root_mut(&pkg).unwrap();
            root.children.clear();
            assert!(root.children.is_empty());
        }
        part.reload(&pkg).unwrap();
        let root = part.root(&pkg).unwrap();
        assert!(root.child("body").is_some());
        assert!(pkg.can_save());
        assert!(!pkg.is_closed());
    }

    #[test]
    fn part_root_events_on_reload() {
        use crate::features::PackageEventType;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut opc = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        opc.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#.to_vec(),
        );
        let mut pkg = OpenXmlPackage::from_opc(opc, Default::default());
        let reloaded = Arc::new(AtomicUsize::new(0));
        let c = reloaded.clone();
        pkg.part_root_events().subscribe(move |e| {
            if e.event_type == PackageEventType::Reloaded {
                c.fetch_add(1, Ordering::SeqCst);
            }
        });
        let mut part = OpenXmlPart::new(
            doc,
            content_type::WORD_DOCUMENT,
            rel::OFFICE_DOCUMENT,
        );
        part.reload(&pkg).unwrap();
        assert_eq!(reloaded.load(Ordering::SeqCst), 1);
        let _ = part.unload_root_element_with_events(&pkg);
        assert!(!part.is_root_element_loaded());
    }

    #[test]
    fn save_to_stream_includes_standalone_declaration() {
        let mut part = OpenXmlPart::new(
            "/word/document.xml",
            content_type::WORD_DOCUMENT,
            rel::OFFICE_DOCUMENT,
        );
        part.set_root(OpenXmlElement::new(
            "w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "document",
        ));
        part.set_standalone_declaration(Some(false));
        let mut buf = Vec::new();
        part.save_to_stream(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.starts_with(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#));
        assert!(s.contains("document"));

        part.set_standalone_declaration(None);
        let mut buf = Vec::new();
        part.save_to_stream(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
        assert!(!s.contains("standalone="));
    }

    #[test]
    fn save_to_part_writes_even_when_not_dirty() {
        let mut opc = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        opc.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#.to_vec(),
        );
        let mut pkg = OpenXmlPackage::from_opc(opc, Default::default());
        let mut part = OpenXmlPart::new(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            rel::OFFICE_DOCUMENT,
        );
        let _ = part.root(&pkg).unwrap();
        assert!(!part.is_dirty());
        part.set_standalone_declaration(Some(true));
        part.save_to_part(&mut pkg).unwrap();
        assert!(!part.is_dirty());
        let bytes = pkg.opc().get_part(&doc).unwrap();
        let s = std::str::from_utf8(bytes).unwrap();
        assert!(s.contains(r#"standalone="yes""#));
        assert!(s.contains("document"));
    }

    #[test]
    fn load_captures_standalone_declaration() {
        let mut opc = OpcPackage::create();
        let doc = PackUri::new("/word/document.xml");
        opc.set_part(
            doc.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#.to_vec(),
        );
        let pkg = OpenXmlPackage::from_opc(opc, Default::default());
        let mut part = OpenXmlPart::new(
            doc,
            content_type::WORD_DOCUMENT,
            rel::OFFICE_DOCUMENT,
        );
        assert_eq!(part.standalone_declaration(), Some(true)); // default before load
        let _ = part.root(&pkg).unwrap();
        assert_eq!(part.standalone_declaration(), Some(false));
    }
}
