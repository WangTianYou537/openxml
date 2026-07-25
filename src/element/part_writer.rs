//! Incremental SAX-style part writer (C# `OpenXmlWriter` / `OpenXmlPartWriter` shell).

use super::element::{OpenXmlAttribute, OpenXmlElement};
use crate::error::{Error, Result};
use std::io::Write;

/// Write Open XML elements to a stream without building a full intermediate string.
pub struct OpenXmlPartWriter<W: Write> {
    writer: W,
    stack: Vec<String>,
    wrote_decl: bool,
    write_declaration: bool,
    /// When true, a start tag is open (`<name` written, `>` deferred) so attributes can still be written.
    open_start: bool,
}

impl<W: Write> OpenXmlPartWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            stack: Vec::new(),
            wrote_decl: false,
            write_declaration: true,
            open_start: false,
        }
    }

    pub fn without_declaration(mut self) -> Self {
        self.write_declaration = false;
        self
    }

    fn ensure_decl(&mut self) -> Result<()> {
        if self.write_declaration && !self.wrote_decl {
            self.writer
                .write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#)
                .map_err(|e| Error::Io(e))?;
            self.wrote_decl = true;
        }
        Ok(())
    }

    /// Close a deferred start tag (`>`) if one is open.
    fn finish_open_start(&mut self) -> Result<()> {
        if self.open_start {
            self.writer.write_all(b">").map_err(Error::Io)?;
            self.open_start = false;
        }
        Ok(())
    }

    /// Write a full element tree (start through end), nested under any open elements.
    pub fn write_element(&mut self, element: &OpenXmlElement) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        write_element_to(&mut self.writer, element)
    }

    /// Write a start tag (C# `WriteStartElement`). Attributes are taken from `element`
    /// but children are not written — call [`write_element`] / [`write_string`] / [`write_end_element`].
    pub fn write_start_element(&mut self, element: &OpenXmlElement) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        let qname = element.qualified_name();
        self.writer.write_all(b"<").map_err(Error::Io)?;
        self.writer.write_all(qname.as_bytes()).map_err(Error::Io)?;
        for (prefix, uri) in &element.namespace_declarations {
            self.writer.write_all(b" xmlns").map_err(Error::Io)?;
            if !prefix.is_empty() {
                self.writer.write_all(b":").map_err(Error::Io)?;
                self.writer.write_all(prefix.as_bytes()).map_err(Error::Io)?;
            }
            self.writer.write_all(b"=\"").map_err(Error::Io)?;
            write_escaped_attr(&mut self.writer, uri)?;
            self.writer.write_all(b"\"").map_err(Error::Io)?;
        }
        for attr in &element.attributes {
            write_attr(&mut self.writer, attr)?;
        }
        self.open_start = true;
        self.stack.push(qname);
        Ok(())
    }

    /// Write a start tag using `element`'s name with overridden attributes and
    /// optional extra namespace declarations (C# `WriteStartElement` overloads).
    pub fn write_start_element_with(
        &mut self,
        element: &OpenXmlElement,
        attributes: &[OpenXmlAttribute],
        namespace_declarations: &[(String, String)],
    ) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        let qname = element.qualified_name();
        self.writer.write_all(b"<").map_err(Error::Io)?;
        self.writer.write_all(qname.as_bytes()).map_err(Error::Io)?;
        // Element's own ns decls first, then extras (extras win on duplicate prefix when serialized in order).
        let mut seen = std::collections::HashSet::new();
        for (prefix, uri) in element.namespace_declarations.iter().chain(namespace_declarations.iter()) {
            if !seen.insert(prefix.clone()) {
                continue;
            }
            self.writer.write_all(b" xmlns").map_err(Error::Io)?;
            if !prefix.is_empty() {
                self.writer.write_all(b":").map_err(Error::Io)?;
                self.writer.write_all(prefix.as_bytes()).map_err(Error::Io)?;
            }
            self.writer.write_all(b"=\"").map_err(Error::Io)?;
            write_escaped_attr(&mut self.writer, uri)?;
            self.writer.write_all(b"\"").map_err(Error::Io)?;
        }
        for attr in attributes {
            write_attr(&mut self.writer, attr)?;
        }
        self.open_start = true;
        self.stack.push(qname);
        Ok(())
    }

    /// Convenience: start element with explicit attributes only (keep element ns decls).
    pub fn write_start_element_attrs(
        &mut self,
        element: &OpenXmlElement,
        attributes: &[OpenXmlAttribute],
    ) -> Result<()> {
        self.write_start_element_with(element, attributes, &[])
    }

    /// Write a start element mirroring the current cursor of a part reader
    /// (C# `WriteStartElement(OpenXmlReader)`).
    pub fn write_start_from_part_reader<R: std::io::BufRead>(
        &mut self,
        reader: &super::part_reader::OpenXmlPartReader<R>,
    ) -> Result<()> {
        if !reader.is_start_element() {
            return Err(Error::Xml(
                "WriteStartElement(OpenXmlReader) requires a start element".into(),
            ));
        }
        let attrs = reader.open_xml_attributes();
        self.write_start(reader.prefix(), reader.local_name(), &attrs)
    }

    /// Write a start element from a DOM reader cursor (C# `WriteStartElement(OpenXmlReader)` subset).
    pub fn write_start_from_dom_reader(
        &mut self,
        reader: &super::dom_reader::OpenXmlDomReader<'_>,
    ) -> Result<()> {
        let Some(el) = reader.current() else {
            return Err(Error::Xml("DOM reader has no current element".into()));
        };
        if !reader.is_start_element() && !reader.is_misc_node() {
            return Err(Error::Xml("DOM reader is not on a start element".into()));
        }
        self.write_start_element(el)
    }

    /// Write start document declaration explicitly (C# `WriteStartDocument`).
    pub fn write_start_document(&mut self) -> Result<()> {
        self.write_declaration = true;
        self.ensure_decl()
    }

    /// Write start document with standalone flag (C# `WriteStartDocument(bool)`).
    pub fn write_start_document_standalone(&mut self, standalone: bool) -> Result<()> {
        if self.wrote_decl {
            return Ok(());
        }
        let decl = if standalone {
            br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_slice()
        } else {
            br#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#.as_slice()
        };
        self.writer.write_all(decl).map_err(Error::Io)?;
        self.wrote_decl = true;
        self.write_declaration = true;
        Ok(())
    }

    /// Write a start tag from local name + attributes.
    pub fn write_start(
        &mut self,
        prefix: Option<&str>,
        local_name: &str,
        attributes: &[OpenXmlAttribute],
    ) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        let qname = match prefix {
            Some(p) if !p.is_empty() => format!("{p}:{local_name}"),
            _ => local_name.to_string(),
        };
        self.writer.write_all(b"<").map_err(Error::Io)?;
        self.writer.write_all(qname.as_bytes()).map_err(Error::Io)?;
        for attr in attributes {
            write_attr(&mut self.writer, attr)?;
        }
        self.open_start = true;
        self.stack.push(qname);
        Ok(())
    }

    pub fn write_end_element(&mut self) -> Result<()> {
        self.finish_open_start()?;
        let qname = self
            .stack
            .pop()
            .ok_or_else(|| Error::Xml("WriteEndElement with empty stack".into()))?;
        self.writer.write_all(b"</").map_err(Error::Io)?;
        self.writer.write_all(qname.as_bytes()).map_err(Error::Io)?;
        self.writer.write_all(b">").map_err(Error::Io)?;
        Ok(())
    }

    /// Write an attribute on the currently open start tag (C# `WriteAttribute` / `WriteAttributeString`).
    ///
    /// Must be called after a start-element method and before any content or end tag.
    pub fn write_attribute(&mut self, attr: &OpenXmlAttribute) -> Result<()> {
        if !self.open_start {
            return Err(Error::Xml(
                "WriteAttribute requires an open start element".into(),
            ));
        }
        write_attr(&mut self.writer, attr)
    }

    /// Write `prefix:local="value"` on the open start tag (C# `WriteAttributeString`).
    pub fn write_attribute_string(
        &mut self,
        prefix: Option<&str>,
        local_name: &str,
        namespace_uri: Option<&str>,
        value: &str,
    ) -> Result<()> {
        let attr = match (prefix, namespace_uri) {
            (Some(p), Some(ns)) if !p.is_empty() => {
                OpenXmlAttribute::with_ns(p, ns, local_name, value)
            }
            (Some(p), _) if !p.is_empty() => OpenXmlAttribute {
                prefix: Some(p.to_string()),
                namespace_uri: None,
                local_name: local_name.to_string(),
                value: value.to_string(),
            },
            _ => OpenXmlAttribute::new(local_name, value),
        };
        self.write_attribute(&attr)
    }

    /// Write an `xmlns` / `xmlns:prefix` declaration on the open start tag
    /// (C# `WriteNamespaceDeclaration` shell).
    pub fn write_namespace_declaration(&mut self, prefix: &str, uri: &str) -> Result<()> {
        if !self.open_start {
            return Err(Error::Xml(
                "WriteNamespaceDeclaration requires an open start element".into(),
            ));
        }
        self.writer.write_all(b" xmlns").map_err(Error::Io)?;
        if !prefix.is_empty() {
            self.writer.write_all(b":").map_err(Error::Io)?;
            self.writer.write_all(prefix.as_bytes()).map_err(Error::Io)?;
        }
        self.writer.write_all(b"=\"").map_err(Error::Io)?;
        write_escaped_attr(&mut self.writer, uri)?;
        self.writer.write_all(b"\"").map_err(Error::Io)?;
        Ok(())
    }

    /// Write character data (escaped).
    pub fn write_string(&mut self, text: &str) -> Result<()> {
        self.finish_open_start()?;
        write_escaped_text(&mut self.writer, text)
    }

    /// Write a character slice (C# `WriteChars`).
    pub fn write_chars(&mut self, chars: &[char]) -> Result<()> {
        let s: String = chars.iter().collect();
        self.write_string(&s)
    }

    /// Write a CDATA section (C# `WriteCData`).
    pub fn write_cdata(&mut self, text: &str) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        self.writer.write_all(b"<![CDATA[").map_err(Error::Io)?;
        // Split `]]>` so the section stays well-formed.
        for (i, chunk) in text.split("]]>").enumerate() {
            if i > 0 {
                self.writer
                    .write_all(b"]]]]><![CDATA[>")
                    .map_err(Error::Io)?;
            }
            self.writer.write_all(chunk.as_bytes()).map_err(Error::Io)?;
        }
        self.writer.write_all(b"]]>").map_err(Error::Io)?;
        Ok(())
    }

    /// Write an XML comment (C# `WriteComment`).
    pub fn write_comment(&mut self, text: &str) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        self.writer.write_all(b"<!--").map_err(Error::Io)?;
        self.writer.write_all(text.as_bytes()).map_err(Error::Io)?;
        self.writer.write_all(b"-->").map_err(Error::Io)?;
        Ok(())
    }

    /// Write a processing instruction (C# `WriteProcessingInstruction`).
    pub fn write_processing_instruction(&mut self, target: &str, data: Option<&str>) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        self.writer.write_all(b"<?").map_err(Error::Io)?;
        self.writer.write_all(target.as_bytes()).map_err(Error::Io)?;
        if let Some(d) = data {
            if !d.is_empty() {
                self.writer.write_all(b" ").map_err(Error::Io)?;
                self.writer.write_all(d.as_bytes()).map_err(Error::Io)?;
            }
        }
        self.writer.write_all(b"?>").map_err(Error::Io)?;
        Ok(())
    }

    /// Write a character entity reference (C# `WriteCharEntity`), e.g. `&#xA0;`.
    pub fn write_char_entity(&mut self, ch: char) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        write!(self.writer, "&#x{:X};", ch as u32).map_err(Error::Io)?;
        Ok(())
    }

    /// Write a named entity reference (C# `WriteEntityRef`), e.g. `&nbsp;`.
    pub fn write_entity_ref(&mut self, name: &str) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        self.writer.write_all(b"&").map_err(Error::Io)?;
        self.writer.write_all(name.as_bytes()).map_err(Error::Io)?;
        self.writer.write_all(b";").map_err(Error::Io)?;
        Ok(())
    }

    /// Write raw XML without escaping (C# `WriteRaw`).
    pub fn write_raw(&mut self, xml: &str) -> Result<()> {
        self.finish_open_start()?;
        self.ensure_decl()?;
        self.writer.write_all(xml.as_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.finish_open_start()?;
        self.writer.flush().map_err(Error::Io)
    }

    /// Finish and return the underlying writer (errors if elements remain open).
    pub fn finish(mut self) -> Result<W> {
        if !self.stack.is_empty() {
            return Err(Error::Xml(format!(
                "{} unclosed element(s) in OpenXmlPartWriter",
                self.stack.len()
            )));
        }
        self.flush()?;
        Ok(self.writer)
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

fn write_element_to<W: Write>(w: &mut W, elem: &OpenXmlElement) -> Result<()> {
    use super::element::OpenXmlMiscKind;
    match elem.misc_kind {
        OpenXmlMiscKind::Comment => {
            w.write_all(b"<!--").map_err(Error::Io)?;
            w.write_all(elem.text.as_deref().unwrap_or("").as_bytes())
                .map_err(Error::Io)?;
            w.write_all(b"-->").map_err(Error::Io)?;
            return Ok(());
        }
        OpenXmlMiscKind::ProcessingInstruction => {
            w.write_all(b"<?").map_err(Error::Io)?;
            w.write_all(elem.pi_target().unwrap_or("").as_bytes())
                .map_err(Error::Io)?;
            if let Some(t) = elem.text.as_deref() {
                if !t.is_empty() {
                    w.write_all(b" ").map_err(Error::Io)?;
                    w.write_all(t.as_bytes()).map_err(Error::Io)?;
                }
            }
            w.write_all(b"?>").map_err(Error::Io)?;
            return Ok(());
        }
        OpenXmlMiscKind::CData => {
            w.write_all(b"<![CDATA[").map_err(Error::Io)?;
            w.write_all(elem.text.as_deref().unwrap_or("").as_bytes())
                .map_err(Error::Io)?;
            w.write_all(b"]]>").map_err(Error::Io)?;
            return Ok(());
        }
        OpenXmlMiscKind::None => {}
    }

    let qname = elem.qualified_name();
    let empty = elem.children.is_empty() && elem.text.is_none();
    w.write_all(b"<").map_err(Error::Io)?;
    w.write_all(qname.as_bytes()).map_err(Error::Io)?;
    for (prefix, uri) in &elem.namespace_declarations {
        w.write_all(b" xmlns").map_err(Error::Io)?;
        if !prefix.is_empty() {
            w.write_all(b":").map_err(Error::Io)?;
            w.write_all(prefix.as_bytes()).map_err(Error::Io)?;
        }
        w.write_all(b"=\"").map_err(Error::Io)?;
        write_escaped_attr(w, uri)?;
        w.write_all(b"\"").map_err(Error::Io)?;
    }
    for attr in &elem.attributes {
        write_attr(w, attr)?;
    }
    if empty {
        w.write_all(b"/>").map_err(Error::Io)?;
        return Ok(());
    }
    w.write_all(b">").map_err(Error::Io)?;
    if let Some(text) = &elem.text {
        write_escaped_text(w, text)?;
    }
    for child in &elem.children {
        write_element_to(w, child)?;
    }
    w.write_all(b"</").map_err(Error::Io)?;
    w.write_all(qname.as_bytes()).map_err(Error::Io)?;
    w.write_all(b">").map_err(Error::Io)?;
    Ok(())
}

fn write_attr<W: Write>(w: &mut W, attr: &OpenXmlAttribute) -> Result<()> {
    w.write_all(b" ").map_err(Error::Io)?;
    if let Some(p) = &attr.prefix {
        if !p.is_empty() {
            w.write_all(p.as_bytes()).map_err(Error::Io)?;
            w.write_all(b":").map_err(Error::Io)?;
        }
    }
    w.write_all(attr.local_name.as_bytes()).map_err(Error::Io)?;
    w.write_all(b"=\"").map_err(Error::Io)?;
    write_escaped_attr(w, &attr.value)?;
    w.write_all(b"\"").map_err(Error::Io)?;
    Ok(())
}

fn write_escaped_text<W: Write>(w: &mut W, s: &str) -> Result<()> {
    for ch in s.chars() {
        match ch {
            '&' => w.write_all(b"&amp;").map_err(Error::Io)?,
            '<' => w.write_all(b"&lt;").map_err(Error::Io)?,
            '>' => w.write_all(b"&gt;").map_err(Error::Io)?,
            _ => {
                let mut buf = [0u8; 4];
                let enc = ch.encode_utf8(&mut buf);
                w.write_all(enc.as_bytes()).map_err(Error::Io)?;
            }
        }
    }
    Ok(())
}

fn write_escaped_attr<W: Write>(w: &mut W, s: &str) -> Result<()> {
    for ch in s.chars() {
        match ch {
            '&' => w.write_all(b"&amp;").map_err(Error::Io)?,
            '<' => w.write_all(b"&lt;").map_err(Error::Io)?,
            '"' => w.write_all(b"&quot;").map_err(Error::Io)?,
            _ => {
                let mut buf = [0u8; 4];
                let enc = ch.encode_utf8(&mut buf);
                w.write_all(enc.as_bytes()).map_err(Error::Io)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;

    #[test]
    fn part_writer_incremental() {
        let mut w = OpenXmlPartWriter::new(Vec::new());
        let root = OpenXmlElement::w("document")
            .with_ns_decl("w", "http://schemas.openxmlformats.org/wordprocessingml/2006/main");
        w.write_start_element(&root).unwrap();
        w.write_element(&OpenXmlElement::w("body").with_child(
            OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_child(
                OpenXmlElement::w("t").with_text("Hi & Bye"),
            )),
        ))
        .unwrap();
        w.write_end_element().unwrap();
        let bytes = w.finish().unwrap();
        let s = String::from_utf8(bytes).unwrap();
        assert!(s.contains("<?xml"));
        assert!(s.contains("<w:document"));
        assert!(s.contains("Hi &amp; Bye"));
        assert!(s.ends_with("</w:document>") || s.contains("</w:document>"));
    }

    #[test]
    fn write_start_element_with_attrs() {
        let mut buf = Vec::new();
        let mut w = OpenXmlPartWriter::new(&mut buf).without_declaration();
        let el = OpenXmlElement::w("p");
        let attrs = vec![OpenXmlAttribute {
            prefix: Some("w".into()),
            namespace_uri: None,
            local_name: "rsidR".into(),
            value: "00AB".into(),
        }];
        w.write_start_element_attrs(&el, &attrs).unwrap();
        w.write_end_element().unwrap();
        w.finish().unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("rsidR"), "{s}");
        assert!(s.contains("<w:p") || s.contains(":p"), "{s}");
    }

    #[test]
    fn write_start_document_standalone_flag() {
        let mut buf = Vec::new();
        {
            let mut w = OpenXmlPartWriter::new(&mut buf).without_declaration();
            w.write_start_document_standalone(false).unwrap();
            w.write_start(None, "root", &[]).unwrap();
            w.write_end_element().unwrap();
            w.finish().unwrap();
        }
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("standalone=\"no\""), "{s}");
    }

    #[test]
    fn write_start_from_part_reader_roundtrip() {
        use super::super::part_reader::OpenXmlPartReader;
        let xml = br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:rsidR="1"><w:r/></w:p>"#;
        let mut r = OpenXmlPartReader::from_bytes(xml);
        assert!(r.read().unwrap());
        let mut buf = Vec::new();
        let mut w = OpenXmlPartWriter::new(&mut buf).without_declaration();
        w.write_start_from_part_reader(&r).unwrap();
        w.write_end_element().unwrap();
        w.finish().unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("p"), "{s}");
        assert!(s.contains("rsidR") || s.contains("w:p"), "{s}");
    }

    #[test]
    fn write_misc_nodes() {
        let mut buf = Vec::new();
        {
            let mut w = OpenXmlPartWriter::new(&mut buf).without_declaration();
            w.write_comment(" hi ").unwrap();
            w.write_processing_instruction("xml-stylesheet", Some("type=\"text/xsl\"")).unwrap();
            w.write_start(None, "root", &[]).unwrap();
            w.write_cdata("a]]>b").unwrap();
            w.write_chars(&['x', '&', 'y']).unwrap();
            w.write_char_entity('\u{A0}').unwrap();
            w.write_entity_ref("amp").unwrap();
            w.write_end_element().unwrap();
            w.finish().unwrap();
        }
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("<!-- hi -->"), "{s}");
        assert!(s.contains("<?xml-stylesheet"), "{s}");
        assert!(s.contains("<![CDATA["), "{s}");
        assert!(s.contains("x&amp;y"), "{s}");
        assert!(s.contains("&#xA0;"), "{s}");
        assert!(s.contains("&amp;"), "{s}");
    }

    #[test]
    fn write_attribute_on_open_start() {
        let mut buf = Vec::new();
        {
            let mut w = OpenXmlPartWriter::new(&mut buf).without_declaration();
            w.write_start(Some("w"), "p", &[]).unwrap();
            w.write_attribute_string(Some("w"), "rsidR", None, "00AB").unwrap();
            w.write_namespace_declaration(
                "r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            )
            .unwrap();
            w.write_string("x").unwrap();
            w.write_end_element().unwrap();
            w.finish().unwrap();
        }
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("rsidR=\"00AB\""), "{s}");
        assert!(s.contains("xmlns:r="), "{s}");
        assert!(s.contains(">x</"), "{s}");
    }
}
