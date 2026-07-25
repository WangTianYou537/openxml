//! Common Open XML parts (styles, settings, images, …).

use crate::element::{write_element, OpenXmlElement};
use crate::error::{Error, Result};
use crate::namespace::{content_type, ns, rel};
use crate::opc::{PackUri, RelationshipTargetMode};
use crate::packaging::{MainDocumentPart, OpenXmlPackage, OpenXmlPart};

/// Image format / content type for binary image parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Bmp,
    Tiff,
    Emf,
    Wmf,
    Svg,
}

impl ImageFormat {
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Png => content_type::IMAGE_PNG,
            Self::Jpeg => content_type::IMAGE_JPEG,
            Self::Gif => content_type::IMAGE_GIF,
            Self::Bmp => content_type::IMAGE_BMP,
            Self::Tiff => content_type::IMAGE_TIFF,
            Self::Emf => content_type::IMAGE_EMF,
            Self::Wmf => content_type::IMAGE_WMF,
            Self::Svg => content_type::IMAGE_SVG,
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
            Self::Emf => "emf",
            Self::Wmf => "wmf",
            Self::Svg => "svg",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.trim_start_matches('.').to_ascii_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "gif" => Some(Self::Gif),
            "bmp" => Some(Self::Bmp),
            "tif" | "tiff" => Some(Self::Tiff),
            "emf" => Some(Self::Emf),
            "wmf" => Some(Self::Wmf),
            "svg" => Some(Self::Svg),
            _ => None,
        }
    }
}

/// Style definitions part (`/word/styles.xml`).
#[derive(Debug)]
pub struct StyleDefinitionsPart {
    inner: OpenXmlPart,
}

impl StyleDefinitionsPart {
    pub const URI: &'static str = "/word/styles.xml";
    pub const RELATIONSHIP_TYPE: &'static str = rel::STYLES;
    pub const CONTENT_TYPE: &'static str = content_type::WORD_STYLES;

    pub fn new() -> Self {
        Self {
            inner: OpenXmlPart::new(Self::URI, Self::CONTENT_TYPE, Self::RELATIONSHIP_TYPE),
        }
    }

    pub fn part(&self) -> &OpenXmlPart {
        &self.inner
    }

    pub fn part_mut(&mut self) -> &mut OpenXmlPart {
        &mut self.inner
    }

    pub fn set_styles(&mut self, styles: OpenXmlElement) {
        self.inner.set_root(styles);
    }

    pub fn save_to_package(&mut self, package: &mut OpenXmlPackage) -> Result<()> {
        self.inner.save_to_package(package)
    }
}

impl Default for StyleDefinitionsPart {
    fn default() -> Self {
        Self::new()
    }
}

/// Document settings part (`/word/settings.xml`).
#[derive(Debug)]
pub struct DocumentSettingsPart {
    inner: OpenXmlPart,
}

impl DocumentSettingsPart {
    pub const URI: &'static str = "/word/settings.xml";
    pub const RELATIONSHIP_TYPE: &'static str = rel::SETTINGS;
    pub const CONTENT_TYPE: &'static str = content_type::WORD_SETTINGS;

    pub fn new() -> Self {
        Self {
            inner: OpenXmlPart::new(Self::URI, Self::CONTENT_TYPE, Self::RELATIONSHIP_TYPE),
        }
    }

    pub fn part(&self) -> &OpenXmlPart {
        &self.inner
    }

    pub fn part_mut(&mut self) -> &mut OpenXmlPart {
        &mut self.inner
    }

    pub fn set_settings(&mut self, settings: OpenXmlElement) {
        self.inner.set_root(settings);
    }

    pub fn save_to_package(&mut self, package: &mut OpenXmlPackage) -> Result<()> {
        self.inner.save_to_package(package)
    }
}

impl Default for DocumentSettingsPart {
    fn default() -> Self {
        Self::new()
    }
}

/// A binary image part under `/word/media/`.
#[derive(Debug)]
pub struct ImagePart {
    uri: PackUri,
    content_type: String,
    relationship_id: String,
}

impl ImagePart {
    pub fn uri(&self) -> &PackUri {
        &self.uri
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    /// Relationship id from the main document part (`rIdN`).
    pub fn relationship_id(&self) -> &str {
        &self.relationship_id
    }
}

/// Minimal default `w:styles` document (Normal style only).
pub fn default_styles() -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    OpenXmlElement::new("w", w, "styles")
        .with_ns_decl("w", w)
        .with_child(
            OpenXmlElement::new("w", w, "docDefaults")
                .with_child(
                    OpenXmlElement::new("w", w, "rPrDefault").with_child(
                        OpenXmlElement::new("w", w, "rPr").with_child(
                            OpenXmlElement::new("w", w, "lang")
                                .with_attribute_qname("w:val", "en-US")
                                .with_attribute_qname("w:eastAsia", "en-US"),
                        ),
                    ),
                )
                .with_child(
                    OpenXmlElement::new("w", w, "pPrDefault")
                        .with_child(OpenXmlElement::new("w", w, "pPr")),
                ),
        )
        .with_child(
            OpenXmlElement::new("w", w, "style")
                .with_attribute_qname("w:type", "paragraph")
                .with_attribute_qname("w:default", "1")
                .with_attribute_qname("w:styleId", "Normal")
                .with_child(
                    OpenXmlElement::new("w", w, "name").with_attribute_qname("w:val", "Normal"),
                )
                .with_child(OpenXmlElement::new("w", w, "qFormat")),
        )
}

/// Minimal empty `w:settings` document.
pub fn default_settings() -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    OpenXmlElement::new("w", w, "settings")
        .with_ns_decl("w", w)
        .with_child(
            OpenXmlElement::new("w", w, "defaultTabStop").with_attribute_qname("w:val", "720"),
        )
}

impl MainDocumentPart {
    /// Add (or replace) the style definitions part with the given styles root.
    ///
    /// Returns the relationship id from the main document part.
    pub fn add_styles_part(
        &self,
        package: &mut OpenXmlPackage,
        styles: OpenXmlElement,
    ) -> Result<String> {
        let xml = write_element(&styles)?;
        let uri = PackUri::new(StyleDefinitionsPart::URI);
        package
            .opc_mut()
            .set_part(uri.clone(), StyleDefinitionsPart::CONTENT_TYPE, xml);
        // Avoid duplicate relationships of the same type when re-adding.
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| {
                rels.get_by_type(StyleDefinitionsPart::RELATIONSHIP_TYPE)
                    .map(|r| r.id.clone())
            })
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(
            package,
            StyleDefinitionsPart::RELATIONSHIP_TYPE,
            &uri,
        ))
    }

    /// Add a minimal default styles part.
    pub fn add_default_styles_part(&self, package: &mut OpenXmlPackage) -> Result<String> {
        self.add_styles_part(package, default_styles())
    }

    /// Add (or replace) the document settings part.
    pub fn add_settings_part(
        &self,
        package: &mut OpenXmlPackage,
        settings: OpenXmlElement,
    ) -> Result<String> {
        let xml = write_element(&settings)?;
        let uri = PackUri::new(DocumentSettingsPart::URI);
        package
            .opc_mut()
            .set_part(uri.clone(), DocumentSettingsPart::CONTENT_TYPE, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| {
                rels.get_by_type(DocumentSettingsPart::RELATIONSHIP_TYPE)
                    .map(|r| r.id.clone())
            })
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, DocumentSettingsPart::RELATIONSHIP_TYPE, &uri))
    }

    /// Add a minimal default settings part.
    pub fn add_default_settings_part(&self, package: &mut OpenXmlPackage) -> Result<String> {
        self.add_settings_part(package, default_settings())
    }

    /// Add a binary image part under `/word/media/` and relate it from the main document.
    pub fn add_image_part(
        &self,
        package: &mut OpenXmlPackage,
        format: ImageFormat,
        data: impl Into<Vec<u8>>,
    ) -> Result<ImagePart> {
        // Pick next free imageN.ext
        let mut index = 1u32;
        let uri = loop {
            let candidate =
                PackUri::new(format!("/word/media/image{index}.{}", format.extension()));
            if !package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
            if index > 10_000 {
                return Err(Error::Package("too many image parts".into()));
            }
        };

        let content_type = format.content_type().to_string();
        package
            .opc_mut()
            .set_part(uri.clone(), content_type.clone(), data.into());

        // Content types for images are usually defaults by extension
        package
            .opc_mut()
            .content_types_mut()
            .set_default(format.extension(), content_type.clone());

        let relationship_id =
            self.add_part_relationship(package, rel::IMAGE, &uri);

        Ok(ImagePart {
            uri,
            content_type,
            relationship_id,
        })
    }

    /// Add a header part (`/word/headerN.xml`) and return its relationship id.
    pub fn add_header_part(
        &self,
        package: &mut OpenXmlPackage,
        content: OpenXmlElement,
    ) -> Result<(String, PackUri)> {
        let mut index = 1u32;
        let uri = loop {
            let candidate = PackUri::new(format!("/word/header{index}.xml"));
            if !package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let xml = write_element(&content)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_HEADER, xml);
        let rid = self.add_part_relationship(package, rel::HEADER, &uri);
        Ok((rid, uri))
    }

    /// Add a footer part (`/word/footerN.xml`) and return its relationship id.
    pub fn add_footer_part(
        &self,
        package: &mut OpenXmlPackage,
        content: OpenXmlElement,
    ) -> Result<(String, PackUri)> {
        let mut index = 1u32;
        let uri = loop {
            let candidate = PackUri::new(format!("/word/footer{index}.xml"));
            if !package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        let xml = write_element(&content)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_FOOTER, xml);
        let rid = self.add_part_relationship(package, rel::FOOTER, &uri);
        Ok((rid, uri))
    }

    /// Add an external hyperlink relationship from the main document part.
    pub fn add_hyperlink(
        &self,
        package: &mut OpenXmlPackage,
        target_url: &str,
    ) -> String {
        package.add_hyperlink_relationship(self.uri(), target_url, true)
    }

    /// Add or replace the comments part (`/word/comments.xml`).
    pub fn add_comments_part(
        &self,
        package: &mut OpenXmlPackage,
        comments_root: OpenXmlElement,
    ) -> Result<String> {
        let uri = PackUri::new("/word/comments.xml");
        let xml = write_element(&comments_root)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_COMMENTS, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| rels.get_by_type(rel::COMMENTS).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, rel::COMMENTS, &uri))
    }

    /// Add a numbering definitions part.
    pub fn add_numbering_part(
        &self,
        package: &mut OpenXmlPackage,
        numbering_root: OpenXmlElement,
    ) -> Result<String> {
        let uri = PackUri::new("/word/numbering.xml");
        let xml = write_element(&numbering_root)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_NUMBERING, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| rels.get_by_type(rel::NUMBERING).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, rel::NUMBERING, &uri))
    }

    /// Add a theme part at `/word/theme/theme1.xml`.
    pub fn add_theme_part(
        &self,
        package: &mut OpenXmlPackage,
        theme_root: OpenXmlElement,
    ) -> Result<String> {
        let uri = PackUri::new("/word/theme/theme1.xml");
        let xml = write_element(&theme_root)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::THEME, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| rels.get_by_type(rel::THEME).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, rel::THEME, &uri))
    }

    /// Add or replace the footnotes part (`/word/footnotes.xml`).
    pub fn add_footnotes_part(
        &self,
        package: &mut OpenXmlPackage,
        footnotes_root: OpenXmlElement,
    ) -> Result<String> {
        let uri = PackUri::new("/word/footnotes.xml");
        let xml = write_element(&footnotes_root)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_FOOTNOTES, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| rels.get_by_type(rel::FOOTNOTES).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, rel::FOOTNOTES, &uri))
    }

    /// Add or replace the endnotes part (`/word/endnotes.xml`).
    pub fn add_endnotes_part(
        &self,
        package: &mut OpenXmlPackage,
        endnotes_root: OpenXmlElement,
    ) -> Result<String> {
        let uri = PackUri::new("/word/endnotes.xml");
        let xml = write_element(&endnotes_root)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_ENDNOTES, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| rels.get_by_type(rel::ENDNOTES).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, rel::ENDNOTES, &uri))
    }

    /// Add a font table part (`/word/fontTable.xml`).
    pub fn add_font_table_part(
        &self,
        package: &mut OpenXmlPackage,
        fonts_root: OpenXmlElement,
    ) -> Result<String> {
        let uri = PackUri::new("/word/fontTable.xml");
        let xml = write_element(&fonts_root)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_FONT_TABLE, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| rels.get_by_type(rel::FONT_TABLE).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, rel::FONT_TABLE, &uri))
    }

    /// Add a web settings part (`/word/webSettings.xml`).
    pub fn add_web_settings_part(
        &self,
        package: &mut OpenXmlPackage,
        web_settings_root: OpenXmlElement,
    ) -> Result<String> {
        let uri = PackUri::new("/word/webSettings.xml");
        let xml = write_element(&web_settings_root)?;
        package
            .opc_mut()
            .set_part(uri.clone(), content_type::WORD_WEB_SETTINGS, xml);
        if let Some(existing) = package
            .opc()
            .part_relationships(self.uri())
            .and_then(|rels| rels.get_by_type(rel::WEB_SETTINGS).map(|r| r.id.clone()))
        {
            return Ok(existing);
        }
        Ok(self.add_part_relationship(package, rel::WEB_SETTINGS, &uri))
    }

    /// Add an Alternative Format Import part (altChunk) and return its relationship id.
    pub fn add_alt_chunk_part(
        &self,
        package: &mut OpenXmlPackage,
        format: AlternativeFormatImportType,
        data: impl Into<Vec<u8>>,
    ) -> Result<(String, PackUri)> {
        let mut index = 1u32;
        let uri = loop {
            let candidate =
                PackUri::new(format!("/word/afchunk{index}.{}", format.extension()));
            if !package.opc().has_part(&candidate) {
                break candidate;
            }
            index += 1;
        };
        package
            .opc_mut()
            .set_part(uri.clone(), format.content_type(), data.into());
        // Prefer default by extension for non-xml types
        package
            .opc_mut()
            .content_types_mut()
            .set_default(format.extension(), format.content_type());
        let rid = self.add_part_relationship(package, rel::AF_CHUNK, &uri);
        Ok((rid, uri))
    }
}

/// Content types for Alternative Format Import (altChunk) parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlternativeFormatImportType {
    Html,
    Xhtml,
    TextPlain,
    Xml,
    Rtf,
    Mht,
}

impl AlternativeFormatImportType {
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Html => "text/html",
            Self::Xhtml => "application/xhtml+xml",
            Self::TextPlain => "text/plain",
            Self::Xml => "application/xml",
            Self::Rtf => "application/rtf",
            Self::Mht => "message/rfc822",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Html => "htm",
            Self::Xhtml => "xhtml",
            Self::TextPlain => "txt",
            Self::Xml => "xml",
            Self::Rtf => "rtf",
            Self::Mht => "mht",
        }
    }
}

/// Build a `w:hdr` root with the given block-level children (paragraphs, …).
pub fn header(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    OpenXmlElement::new("w", w, "hdr")
        .with_ns_decl("w", w)
        .with_children(children)
}

/// Build a `w:ftr` root with the given block-level children.
pub fn footer(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    OpenXmlElement::new("w", w, "ftr")
        .with_ns_decl("w", w)
        .with_children(children)
}

/// `w:headerReference` for use inside `w:sectPr`.
pub fn header_reference(relationship_id: &str, header_type: &str) -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    let r = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    OpenXmlElement::new("w", w, "headerReference")
        .with_attribute_qname("w:type", header_type)
        .with_attribute_ns("r", r, "id", relationship_id)
}

/// `w:footerReference` for use inside `w:sectPr`.
pub fn footer_reference(relationship_id: &str, footer_type: &str) -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    let r = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    OpenXmlElement::new("w", w, "footerReference")
        .with_attribute_qname("w:type", footer_type)
        .with_attribute_ns("r", r, "id", relationship_id)
}

/// `w:hyperlink` run container referencing an external relationship.
pub fn hyperlink(relationship_id: &str, children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    let r = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    let mut el = OpenXmlElement::new("w", w, "hyperlink")
        .with_attribute_ns("r", r, "id", relationship_id);
    el.append_children(children);
    el
}

/// Internal hyperlink targeting a bookmark name (`w:anchor`).
pub fn hyperlink_anchor(
    bookmark_name: &str,
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    let w = ns::WORDPROCESSINGML.uri;
    let mut el = OpenXmlElement::new("w", w, "hyperlink")
        .with_attribute_qname("w:anchor", bookmark_name);
    el.append_children(children);
    el
}
