//! Stateful Open XML part reader (C# `OpenXmlPartReader` / `OpenXmlReader` shell).
//!
//! Built on the same pull model as [`super::streaming::OpenXmlStreamReader`],
//! but exposes a cursor-style API: `Read` / `GetText` / `LoadCurrentElement`.

use super::element::{OpenXmlAttribute, OpenXmlElement};
use super::reader::parse_element;
use super::streaming::{OpenXmlStreamReader, XmlEvent};
use crate::error::{Error, Result};
use std::io::BufRead;

/// Reader position relative to the current node (C# `ElementState` subset).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ElementState {
    #[default]
    Null,
    Start,
    End,
    LeafText,
    Misc,
    EOF,
}

/// Cursor over an XML part stream (C# `OpenXmlReader` shell).
pub struct OpenXmlPartReader<R: BufRead> {
    inner: OpenXmlStreamReader<R>,
    state: ElementState,
    prefix: Option<String>,
    local_name: String,
    attributes: Vec<(String, String)>,
    text: String,
    depth: usize,
    #[allow(dead_code)]
    read_misc_nodes: bool,
    /// Stack of open element qnames for LoadCurrentElement reconstruction.
    open_stack: Vec<StackFrame>,
    /// Buffered events for LoadCurrentElement of the current start node.
    #[allow(dead_code)]
    pending_load: Option<Vec<XmlEvent>>,
    eof: bool,
}

#[derive(Debug, Clone)]
struct StackFrame {
    prefix: Option<String>,
    local_name: String,
    attributes: Vec<(String, String)>,
}

impl<'a> OpenXmlPartReader<&'a [u8]> {
    pub fn from_bytes(data: &'a [u8]) -> Self {
        Self::from_reader(data)
    }
}

impl<R: BufRead> OpenXmlPartReader<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            inner: OpenXmlStreamReader::from_reader(reader),
            state: ElementState::Null,
            prefix: None,
            local_name: String::new(),
            attributes: Vec::new(),
            text: String::new(),
            depth: 0,
            read_misc_nodes: false,
            open_stack: Vec::new(),
            pending_load: None,
            eof: false,
        }
    }

    /// When true, comment/PI events surface as [`ElementState::Misc`] (C# `ReadMiscellaneousNodes`).
    pub fn with_read_misc_nodes(mut self, yes: bool) -> Self {
        self.read_misc_nodes = yes;
        self
    }

    pub fn element_state(&self) -> ElementState {
        self.state
    }

    pub fn is_start_element(&self) -> bool {
        self.state == ElementState::Start
    }

    pub fn is_end_element(&self) -> bool {
        self.state == ElementState::End
    }

    pub fn is_eof(&self) -> bool {
        self.eof || self.state == ElementState::EOF
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    pub fn namespace_uri(&self) -> Option<&str> {
        // Best-effort: look for xmlns on current attributes
        let pfx = self.prefix.as_deref().unwrap_or("");
        for (k, v) in &self.attributes {
            if pfx.is_empty() && k == "xmlns" {
                return Some(v.as_str());
            }
            if let Some(rest) = k.strip_prefix("xmlns:") {
                if rest == pfx {
                    return Some(v.as_str());
                }
            }
        }
        None
    }

    pub fn attributes(&self) -> &[(String, String)] {
        &self.attributes
    }

    /// Move to the next node (C# `Read`).
    pub fn read(&mut self) -> Result<bool> {
        if self.eof {
            self.state = ElementState::EOF;
            return Ok(false);
        }
        loop {
            match self.inner.read_event()? {
                None => {
                    self.eof = true;
                    self.state = ElementState::EOF;
                    self.local_name.clear();
                    self.prefix = None;
                    self.attributes.clear();
                    self.text.clear();
                    return Ok(false);
                }
                Some(XmlEvent::Start {
                    prefix,
                    local_name,
                    attributes,
                }) => {
                    self.prefix = prefix.clone();
                    self.local_name = local_name.clone();
                    self.attributes = attributes.clone();
                    self.text.clear();
                    self.state = ElementState::Start;
                    self.open_stack.push(StackFrame {
                        prefix,
                        local_name,
                        attributes,
                    });
                    self.depth = self.open_stack.len();
                    return Ok(true);
                }
                Some(XmlEvent::Empty {
                    prefix,
                    local_name,
                    attributes,
                }) => {
                    self.prefix = prefix;
                    self.local_name = local_name;
                    self.attributes = attributes;
                    self.text.clear();
                    // Empty is both start and end; report as Start (leaf).
                    self.state = ElementState::Start;
                    self.depth = self.open_stack.len() + 1;
                    // Immediately completeable as leaf — depth after empty doesn't stay open.
                    return Ok(true);
                }
                Some(XmlEvent::End {
                    prefix,
                    local_name,
                }) => {
                    if !self.open_stack.is_empty() {
                        self.open_stack.pop();
                    }
                    self.prefix = prefix;
                    self.local_name = local_name;
                    self.attributes.clear();
                    self.text.clear();
                    self.state = ElementState::End;
                    self.depth = self.open_stack.len();
                    return Ok(true);
                }
                Some(XmlEvent::Text(t)) => {
                    let is_ws = t.chars().all(|c| c.is_whitespace());
                    if is_ws {
                        continue;
                    }
                    self.text = t;
                    self.local_name.clear();
                    self.prefix = None;
                    self.attributes.clear();
                    self.state = ElementState::LeafText;
                    return Ok(true);
                }
            }
        }
    }

    /// Text of the current leaf (C# `GetText`).
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Skip the content of the current start element until its matching end
    /// (C# `Skip`).
    pub fn skip(&mut self) -> Result<()> {
        if self.state != ElementState::Start {
            return Ok(());
        }
        let target_depth = self.depth;
        // For empty-reported start with no stack push, just return.
        if self.open_stack.len() < target_depth {
            // was empty element
            return Ok(());
        }
        while self.read()? {
            if self.state == ElementState::End && self.depth < target_depth {
                break;
            }
            if self.state == ElementState::End && self.depth + 1 == target_depth {
                // ended the element we started
                break;
            }
            if self.state == ElementState::EOF {
                break;
            }
        }
        Ok(())
    }

    /// Move to the first child of the current start element (C# `ReadFirstChild`).
    ///
    /// Returns `false` and positions on the matching end when there is no child.
    pub fn read_first_child(&mut self) -> Result<bool> {
        if self.state != ElementState::Start {
            return Ok(false);
        }
        let parent_depth = self.depth;
        // Empty element (start not pushed to stack): no children.
        if self.open_stack.len() < parent_depth {
            self.state = ElementState::End;
            return Ok(false);
        }
        while self.read()? {
            match self.state {
                ElementState::Start if self.depth == parent_depth + 1 => {
                    return Ok(true);
                }
                ElementState::End if self.depth < parent_depth => {
                    return Ok(false);
                }
                ElementState::End if self.depth + 1 == parent_depth => {
                    // closed parent with no element child (maybe only text)
                    return Ok(false);
                }
                ElementState::EOF => return Ok(false),
                ElementState::LeafText | ElementState::Misc => continue,
                _ => continue,
            }
        }
        Ok(false)
    }

    /// Move to the next sibling element (C# `ReadNextSibling`).
    ///
    /// Skips the rest of the current element and advances to the next start at
    /// the same depth. If none, positions on the parent end and returns `false`.
    pub fn read_next_sibling(&mut self) -> Result<bool> {
        if self.is_eof() {
            return Ok(false);
        }

        let target_depth = match self.state {
            ElementState::Start => {
                let d = self.depth;
                self.skip()?;
                d
            }
            ElementState::End => {
                // Finished current element; siblings start at depth + 1.
                self.depth + 1
            }
            _ => return Ok(false),
        };

        while self.read()? {
            match self.state {
                ElementState::Start if self.depth == target_depth => return Ok(true),
                ElementState::End if self.depth + 1 == target_depth => return Ok(false),
                ElementState::End if self.depth < target_depth.saturating_sub(1) => {
                    return Ok(false);
                }
                ElementState::EOF => return Ok(false),
                _ => continue,
            }
        }
        Ok(false)
    }

    /// Whether the current start element has attributes (C# `HasAttributes`) (C# `HasAttributes`).
    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    /// Load the current start element and all its descendants into a DOM tree
    /// (C# `LoadCurrentElement`). Advances the reader to the matching end.
    pub fn load_current_element(&mut self) -> Result<Option<OpenXmlElement>> {
        if self.state != ElementState::Start {
            return Ok(None);
        }
        // Collect events from current start through matching end.
        let mut events: Vec<XmlEvent> = Vec::new();
        let start_depth = self.depth;
        events.push(XmlEvent::Start {
            prefix: self.prefix.clone(),
            local_name: self.local_name.clone(),
            attributes: self.attributes.clone(),
        });
        // If this was an empty element (not on stack), synthesize end.
        let on_stack = self
            .open_stack
            .last()
            .map(|f| f.local_name == self.local_name)
            .unwrap_or(false);
        if !on_stack {
            // empty: already complete
            events.push(XmlEvent::End {
                prefix: self.prefix.clone(),
                local_name: self.local_name.clone(),
            });
        } else {
            loop {
                match self.inner.read_event()? {
                    None => {
                        self.eof = true;
                        break;
                    }
                    Some(ev) => {
                        let is_end = matches!(&ev, XmlEvent::End { .. });
                        // track stack for depth
                        match &ev {
                            XmlEvent::Start {
                                prefix,
                                local_name,
                                attributes,
                            } => {
                                self.open_stack.push(StackFrame {
                                    prefix: prefix.clone(),
                                    local_name: local_name.clone(),
                                    attributes: attributes.clone(),
                                });
                            }
                            XmlEvent::End { .. } => {
                                self.open_stack.pop();
                            }
                            _ => {}
                        }
                        events.push(ev);
                        if is_end && self.open_stack.len() < start_depth {
                            break;
                        }
                    }
                }
            }
            self.depth = self.open_stack.len();
            self.state = ElementState::End;
        }

        let xml = super::streaming::write_xml_events(&events)?;
        let root = parse_element(&xml)?;
        Ok(Some(root))
    }

    /// Convenience: read until EOF collecting all text under elements named `local`.
    pub fn collect_text_under(&mut self, local: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();
        let mut capture = false;
        while self.read()? {
            match self.state {
                ElementState::Start if self.local_name == local => capture = true,
                ElementState::LeafText if capture => {
                    out.push(self.text.clone());
                    capture = false;
                }
                ElementState::End if self.local_name == local => capture = false,
                _ => {}
            }
        }
        Ok(out)
    }

    /// Build `OpenXmlAttribute` list for the current start element.
    pub fn open_xml_attributes(&self) -> Vec<OpenXmlAttribute> {
        self.attributes
            .iter()
            .filter(|(k, _)| !k.starts_with("xmlns"))
            .map(|(k, v)| {
                let (prefix, local) = if let Some((p, l)) = k.split_once(':') {
                    (Some(p.to_string()), l.to_string())
                } else {
                    (None, k.clone())
                };
                OpenXmlAttribute {
                    prefix,
                    namespace_uri: None,
                    local_name: local,
                    value: v.clone(),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_reader_walk_and_load() {
        let xml = br#"<?xml version="1.0"?>
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
          <w:body>
            <w:p><w:r><w:t>Hello</w:t></w:r></w:p>
          </w:body>
        </w:document>"#;
        let mut r = OpenXmlPartReader::from_bytes(xml);
        assert!(r.read().unwrap());
        assert_eq!(r.element_state(), ElementState::Start);
        assert_eq!(r.local_name(), "document");
        // load whole document
        let doc = r.load_current_element().unwrap().unwrap();
        assert_eq!(doc.local_name, "document");
        assert!(doc.inner_text().contains("Hello"));
    }

    #[test]
    fn part_reader_get_text() {
        let xml = br#"<w:t xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">Hi</w:t>"#;
        let mut r = OpenXmlPartReader::from_bytes(xml);
        assert!(r.read().unwrap()); // start t
        assert!(r.read().unwrap()); // text
        assert_eq!(r.element_state(), ElementState::LeafText);
        assert_eq!(r.get_text(), "Hi");
    }

    #[test]
    fn part_reader_first_child_siblings() {
        let xml = br#"<body><p>a</p><p>b</p><p>c</p></body>"#;
        let mut r = OpenXmlPartReader::from_bytes(xml);
        assert!(r.read().unwrap());
        assert_eq!(r.local_name(), "body");
        assert!(r.read_first_child().unwrap());
        assert_eq!(r.local_name(), "p");
        assert!(r.read_next_sibling().unwrap());
        assert_eq!(r.local_name(), "p");
        assert!(r.read_next_sibling().unwrap());
        assert_eq!(r.local_name(), "p");
        assert!(!r.read_next_sibling().unwrap());
        assert!(r.element_state() == ElementState::End || r.local_name() == "body");
    }
}
