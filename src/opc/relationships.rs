//! OPC relationships (`.rels` files).

use crate::error::{Error, Result};
use indexmap::IndexMap;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::{Cursor, Write};

/// Whether a relationship target is internal to the package or an external URI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelationshipTargetMode {
    #[default]
    Internal,
    External,
}

impl RelationshipTargetMode {
    fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Internal => None,
            Self::External => Some("External"),
        }
    }

    fn parse(s: Option<&str>) -> Self {
        match s {
            Some("External") => Self::External,
            _ => Self::Internal,
        }
    }
}

/// A single OPC relationship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    pub id: String,
    pub relationship_type: String,
    pub target: String,
    pub target_mode: RelationshipTargetMode,
}

/// Collection of relationships for a package or part.
#[derive(Debug, Clone)]
pub struct Relationships {
    /// Ordered map of relationship id → relationship.
    items: IndexMap<String, Relationship>,
    next_id: u32,
}

impl Default for Relationships {
    fn default() -> Self {
        Self::new()
    }
}

impl Relationships {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
            next_id: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Relationship> {
        self.items.values()
    }

    pub fn get(&self, id: &str) -> Option<&Relationship> {
        self.items.get(id)
    }

    pub fn get_by_type(&self, relationship_type: &str) -> Option<&Relationship> {
        self.items
            .values()
            .find(|r| r.relationship_type == relationship_type)
    }

    pub fn find_all_by_type(&self, relationship_type: &str) -> Vec<&Relationship> {
        self.items
            .values()
            .filter(|r| r.relationship_type == relationship_type)
            .collect()
    }

    /// Add a relationship, auto-generating an id like `rId1`.
    pub fn add(
        &mut self,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
        target_mode: RelationshipTargetMode,
    ) -> &Relationship {
        let id = self.allocate_id();
        let rel = Relationship {
            id: id.clone(),
            relationship_type: relationship_type.into(),
            target: target.into(),
            target_mode,
        };
        self.items.insert(id.clone(), rel);
        self.items.get(&id).unwrap()
    }

    /// Add with an explicit id.
    pub fn add_with_id(
        &mut self,
        id: impl Into<String>,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
        target_mode: RelationshipTargetMode,
    ) -> &Relationship {
        let id = id.into();
        if let Some(n) = parse_rid(&id) {
            if n >= self.next_id {
                self.next_id = n + 1;
            }
        }
        let rel = Relationship {
            id: id.clone(),
            relationship_type: relationship_type.into(),
            target: target.into(),
            target_mode,
        };
        self.items.insert(id.clone(), rel);
        self.items.get(&id).unwrap()
    }

    pub fn remove(&mut self, id: &str) -> Option<Relationship> {
        self.items.shift_remove(id)
    }

    fn allocate_id(&mut self) -> String {
        loop {
            let id = format!("rId{}", self.next_id);
            self.next_id += 1;
            if !self.items.contains_key(&id) {
                return id;
            }
        }
    }

    pub fn parse(xml: &[u8]) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        reader.config_mut().trim_text(true);
        let mut rels = Relationships::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    let local = local_name(e.name().as_ref());
                    if local == "Relationship" {
                        let id = attr(&e, "Id")?;
                        let rtype = attr(&e, "Type")?;
                        let target = attr(&e, "Target")?;
                        let mode = RelationshipTargetMode::parse(
                            attr_opt(&e, "TargetMode").as_deref(),
                        );
                        rels.add_with_id(id, rtype, target, mode);
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(rels)
    }

    pub fn to_xml(&self) -> Result<Vec<u8>> {
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes"))))?;

        let ns = "http://schemas.openxmlformats.org/package/2006/relationships";
        let mut start = BytesStart::new("Relationships");
        start.push_attribute(("xmlns", ns));
        writer.write_event(Event::Start(start))?;

        for rel in self.items.values() {
            let mut e = BytesStart::new("Relationship");
            e.push_attribute(("Id", rel.id.as_str()));
            e.push_attribute(("Type", rel.relationship_type.as_str()));
            e.push_attribute(("Target", rel.target.as_str()));
            if let Some(mode) = rel.target_mode.as_str() {
                e.push_attribute(("TargetMode", mode));
            }
            writer.write_event(Event::Empty(e))?;
        }

        writer.write_event(Event::End(BytesEnd::new("Relationships")))?;
        let mut out = writer.into_inner().into_inner();
        out.write_all(b"\n")?;
        Ok(out)
    }
}

fn parse_rid(id: &str) -> Option<u32> {
    id.strip_prefix("rId")?.parse().ok()
}

fn local_name(name: &[u8]) -> String {
    let s = String::from_utf8_lossy(name);
    s.rsplit(':').next().unwrap_or(&s).to_string()
}

fn attr(e: &BytesStart<'_>, key: &str) -> Result<String> {
    attr_opt(e, key).ok_or_else(|| Error::Xml(format!("missing attribute `{key}`")))
}

fn attr_opt(e: &BytesStart<'_>, key: &str) -> Option<String> {
    for a in e.attributes().with_checks(false).flatten() {
        if a.key.as_ref() == key.as_bytes() {
            return Some(String::from_utf8_lossy(&a.value).into_owned());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let mut rels = Relationships::new();
        rels.add(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
            "word/document.xml",
            RelationshipTargetMode::Internal,
        );
        let xml = rels.to_xml().unwrap();
        let parsed = Relationships::parse(&xml).unwrap();
        assert_eq!(parsed.len(), 1);
        let r = parsed.get("rId1").unwrap();
        assert_eq!(r.target, "word/document.xml");
    }
}
