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
}

impl<W: Write> OpenXmlPartWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            stack: Vec::new(),
            wrote_decl: false,
            write_declaration: true,
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

    /// Write a full element tree (start through end), nested under any open elements.
    pub fn write_element(&mut self, element: &OpenXmlElement) -> Result<()> {
        self.ensure_decl()?;
        write_element_to(&mut self.writer, element)
    }

    /// Write a start tag (C# `WriteStartElement`). Attributes are taken from `element`
    /// but children are not written — call [`write_element`] / [`write_string`] / [`write_end_element`].
    pub fn write_start_element(&mut self, element: &OpenXmlElement) -> Result<()> {
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
        self.writer.write_all(b">").map_err(Error::Io)?;
        self.stack.push(qname);
        Ok(())
    }

    /// Write a start tag from local name + attributes.
    pub fn write_start(
        &mut self,
        prefix: Option<&str>,
        local_name: &str,
        attributes: &[OpenXmlAttribute],
    ) -> Result<()> {
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
        self.writer.write_all(b">").map_err(Error::Io)?;
        self.stack.push(qname);
        Ok(())
    }

    pub fn write_end_element(&mut self) -> Result<()> {
        let qname = self
            .stack
            .pop()
            .ok_or_else(|| Error::Xml("WriteEndElement with empty stack".into()))?;
        self.writer.write_all(b"</").map_err(Error::Io)?;
        self.writer.write_all(qname.as_bytes()).map_err(Error::Io)?;
        self.writer.write_all(b">").map_err(Error::Io)?;
        Ok(())
    }

    /// Write character data (escaped).
    pub fn write_string(&mut self, text: &str) -> Result<()> {
        write_escaped_text(&mut self.writer, text)
    }

    /// Write raw XML without escaping (C# `WriteRaw`).
    pub fn write_raw(&mut self, xml: &str) -> Result<()> {
        self.ensure_decl()?;
        self.writer.write_all(xml.as_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
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
}
