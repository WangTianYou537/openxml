//! `[Content_Types].xml` reader/writer.

use crate::error::{Error, Result};
use crate::namespace::content_type as ct;
use indexmap::IndexMap;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::{Cursor, Write};

/// A content type override for a specific part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentTypeOverride {
    pub part_name: String,
    pub content_type: String,
}

/// Package content types map.
#[derive(Debug, Clone, Default)]
pub struct ContentTypes {
    /// Extension (without leading dot) → content type.
    pub defaults: IndexMap<String, String>,
    /// Part name (with leading `/`) → content type.
    pub overrides: IndexMap<String, String>,
}

impl ContentTypes {
    pub fn new() -> Self {
        let mut ct = Self::default();
        ct.defaults
            .insert("rels".into(), ct::RELATIONSHIPS.into());
        ct.defaults.insert("xml".into(), "application/xml".into());
        ct
    }

    pub fn set_default(&mut self, extension: impl Into<String>, content_type: impl Into<String>) {
        self.defaults.insert(extension.into(), content_type.into());
    }

    pub fn set_override(&mut self, part_name: impl Into<String>, content_type: impl Into<String>) {
        let mut name = part_name.into();
        if !name.starts_with('/') {
            name.insert(0, '/');
        }
        self.overrides.insert(name, content_type.into());
    }

    pub fn content_type_for(&self, part_name: &str) -> Option<&str> {
        let name = if part_name.starts_with('/') {
            part_name.to_string()
        } else {
            format!("/{part_name}")
        };
        if let Some(ct) = self.overrides.get(&name) {
            return Some(ct.as_str());
        }
        let ext = name.rsplit('.').next()?;
        self.defaults.get(ext).map(|s| s.as_str())
    }

    pub fn parse(xml: &[u8]) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        reader.config_mut().trim_text(true);
        let mut defaults = IndexMap::new();
        let mut overrides = IndexMap::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    let local = local_name(e.name().as_ref());
                    match local.as_str() {
                        "Default" => {
                            let ext = attr(&e, "Extension")?;
                            let ct = attr(&e, "ContentType")?;
                            defaults.insert(ext, ct);
                        }
                        "Override" => {
                            let part = attr(&e, "PartName")?;
                            let ct = attr(&e, "ContentType")?;
                            overrides.insert(part, ct);
                        }
                        _ => {}
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(Self { defaults, overrides })
    }

    pub fn to_xml(&self) -> Result<Vec<u8>> {
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes"))))?;

        let ns = "http://schemas.openxmlformats.org/package/2006/content-types";
        let mut start = BytesStart::new("Types");
        start.push_attribute(("xmlns", ns));
        writer.write_event(Event::Start(start))?;

        for (ext, content_type) in &self.defaults {
            let mut e = BytesStart::new("Default");
            e.push_attribute(("Extension", ext.as_str()));
            e.push_attribute(("ContentType", content_type.as_str()));
            writer.write_event(Event::Empty(e))?;
        }
        for (part, content_type) in &self.overrides {
            let mut e = BytesStart::new("Override");
            e.push_attribute(("PartName", part.as_str()));
            e.push_attribute(("ContentType", content_type.as_str()));
            writer.write_event(Event::Empty(e))?;
        }

        writer.write_event(Event::End(BytesEnd::new("Types")))?;
        let mut out = writer.into_inner().into_inner();
        out.write_all(b"\n")?;
        Ok(out)
    }
}

fn local_name(name: &[u8]) -> String {
    let s = String::from_utf8_lossy(name);
    s.rsplit(':').next().unwrap_or(&s).to_string()
}

fn attr(e: &BytesStart<'_>, key: &str) -> Result<String> {
    for a in e.attributes().with_checks(false) {
        let a = a.map_err(|err| Error::Xml(err.to_string()))?;
        if a.key.as_ref() == key.as_bytes() {
            return Ok(String::from_utf8_lossy(&a.value).into_owned());
        }
    }
    Err(Error::Xml(format!("missing attribute `{key}`")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let mut ct = ContentTypes::new();
        ct.set_override(
            "/word/document.xml",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
        );
        let xml = ct.to_xml().unwrap();
        let parsed = ContentTypes::parse(&xml).unwrap();
        assert_eq!(
            parsed.content_type_for("/word/document.xml"),
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml")
        );
    }
}
