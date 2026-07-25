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

/// Options for constructing a part reader (C# `OpenXmlPartReaderOptions`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenXmlPartReaderOptions {
    /// Report miscellaneous nodes (C# `ReadMiscellaneousNodes`).
    pub read_miscellaneous_nodes: bool,
    /// Maximum characters allowed in the part (0 = unlimited; C# `MaxCharactersInPart`).
    pub max_characters_in_part: u64,
    /// Skip insignificant whitespace text nodes (C# `IgnoreWhitespace`; default true).
    pub ignore_whitespace: bool,
}

impl Default for OpenXmlPartReaderOptions {
    fn default() -> Self {
        Self {
            read_miscellaneous_nodes: false,
            max_characters_in_part: 0,
            ignore_whitespace: true,
        }
    }
}

impl OpenXmlPartReaderOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_read_miscellaneous_nodes(mut self, yes: bool) -> Self {
        self.read_miscellaneous_nodes = yes;
        self
    }

    pub fn with_max_characters_in_part(mut self, max: u64) -> Self {
        self.max_characters_in_part = max;
        self
    }

    pub fn with_ignore_whitespace(mut self, yes: bool) -> Self {
        self.ignore_whitespace = yes;
        self
    }
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
    read_misc_nodes: bool,
    ignore_whitespace: bool,
    max_characters_in_part: u64,
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

    /// C# `OpenXmlReader.Create(partStream)` shell over bytes.
    pub fn create(data: &'a [u8]) -> Self {
        Self::from_bytes(data)
    }

    /// C# `OpenXmlReader.Create(..., readMiscNodes)`.
    pub fn create_with_misc(data: &'a [u8], read_misc_nodes: bool) -> Self {
        Self::from_bytes(data).with_read_misc_nodes(read_misc_nodes)
    }

    /// C# `OpenXmlPartReader` with [`OpenXmlPartReaderOptions`].
    pub fn create_with_options(data: &'a [u8], options: OpenXmlPartReaderOptions) -> Self {
        Self::from_reader_with_options(data, options)
    }
}

impl<R: BufRead> OpenXmlPartReader<R> {
    pub fn from_reader(reader: R) -> Self {
        Self::from_reader_with_options(reader, OpenXmlPartReaderOptions::default())
    }

    pub fn from_reader_with_options(reader: R, options: OpenXmlPartReaderOptions) -> Self {
        Self {
            inner: OpenXmlStreamReader::from_reader(reader),
            state: ElementState::Null,
            prefix: None,
            local_name: String::new(),
            attributes: Vec::new(),
            text: String::new(),
            depth: 0,
            read_misc_nodes: options.read_miscellaneous_nodes,
            ignore_whitespace: options.ignore_whitespace,
            max_characters_in_part: options.max_characters_in_part,
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

    /// Whether insignificant whitespace text is skipped (C# `IgnoreWhitespace`).
    pub fn with_ignore_whitespace(mut self, yes: bool) -> Self {
        self.ignore_whitespace = yes;
        self
    }

    /// Cap characters read from the part (0 = unlimited).
    pub fn with_max_characters_in_part(mut self, max: u64) -> Self {
        self.max_characters_in_part = max;
        self
    }

    pub fn ignore_whitespace(&self) -> bool {
        self.ignore_whitespace
    }

    pub fn max_characters_in_part(&self) -> u64 {
        self.max_characters_in_part
    }

    fn check_max_characters(&self) -> Result<()> {
        if self.max_characters_in_part > 0
            && self.inner.buffer_position() > self.max_characters_in_part
        {
            return Err(Error::Xml(format!(
                "part exceeds MaxCharactersInPart limit ({})",
                self.max_characters_in_part
            )));
        }
        Ok(())
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

    /// Whether the current node is a miscellaneous node (C# `IsMiscNode`).
    pub fn is_misc_node(&self) -> bool {
        self.state == ElementState::Misc
    }

    pub fn is_eof(&self) -> bool {
        self.eof || self.state == ElementState::EOF
    }

    /// Line/position of the current node (C# `GetLineInfo` / `IXmlLineInfo`).
    pub fn get_line_info(&self) -> super::xml_path::XmlLineInfo {
        self.inner.line_info()
    }

    /// Whether miscellaneous nodes are reported (C# `ReadMiscNodes`).
    pub fn read_misc_nodes(&self) -> bool {
        self.read_misc_nodes
    }

    /// XML declaration encoding when known (C# `Encoding`; always `None` shell).
    pub fn encoding(&self) -> Option<&str> {
        let _ = self;
        None
    }

    /// XML declaration standalone flag (C# `StandaloneXml`; always `None` shell).
    pub fn standalone_xml(&self) -> Option<bool> {
        let _ = self;
        None
    }

    /// Whether the current node carries a text value (C# `HasValue` shell).
    pub fn has_value(&self) -> bool {
        self.state == ElementState::LeafText && !self.text.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    /// Strongly-typed element type name shell (C# `ElementType`; returns local name).
    pub fn element_type_name(&self) -> &str {
        self.local_name.as_str()
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

    /// Number of non-xmlns attributes on the current start element.
    pub fn attribute_count(&self) -> usize {
        self.attributes
            .iter()
            .filter(|(k, _)| !k.starts_with("xmlns"))
            .count()
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
                    self.check_max_characters()?;
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
                    self.check_max_characters()?;
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
                    self.check_max_characters()?;
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
                    self.check_max_characters()?;
                    let is_ws = t.chars().all(|c| c.is_whitespace());
                    if is_ws && self.ignore_whitespace {
                        continue;
                    }
                    self.text = t;
                    self.local_name.clear();
                    self.prefix = None;
                    self.attributes.clear();
                    self.state = ElementState::LeafText;
                    return Ok(true);
                }
                Some(XmlEvent::CData(t)) => {
                    if !self.read_misc_nodes {
                        // Fold CDATA into text when misc nodes are not requested.
                        self.check_max_characters()?;
                        self.text = t;
                        self.local_name.clear();
                        self.prefix = None;
                        self.attributes.clear();
                        self.state = ElementState::LeafText;
                        return Ok(true);
                    }
                    self.check_max_characters()?;
                    self.text = t;
                    self.local_name = "#cdata-section".into();
                    self.prefix = None;
                    self.attributes.clear();
                    self.state = ElementState::Misc;
                    return Ok(true);
                }
                Some(XmlEvent::Comment(t)) => {
                    if !self.read_misc_nodes {
                        continue;
                    }
                    self.check_max_characters()?;
                    self.text = t;
                    self.local_name = "#comment".into();
                    self.prefix = None;
                    self.attributes.clear();
                    self.state = ElementState::Misc;
                    return Ok(true);
                }
                Some(XmlEvent::ProcessingInstruction { target, data }) => {
                    if !self.read_misc_nodes {
                        continue;
                    }
                    self.check_max_characters()?;
                    self.text = data;
                    self.local_name = target;
                    self.prefix = None;
                    self.attributes.clear();
                    self.state = ElementState::Misc;
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

    /// Namespace declarations on the current start element (C# `NamespaceDeclarations`).
    ///
    /// Returns `(prefix, uri)` pairs; the default namespace uses an empty prefix.
    pub fn namespace_declarations(&self) -> Vec<(String, String)> {
        self.attributes
            .iter()
            .filter_map(|(k, v)| {
                if k == "xmlns" {
                    Some((String::new(), v.clone()))
                } else {
                    k.strip_prefix("xmlns:")
                        .map(|p| (p.to_string(), v.clone()))
                }
            })
            .collect()
    }

    /// Attribute value by local name (optional prefix match via `prefix:local` key).
    pub fn get_attribute(&self, local_name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| {
                if k.starts_with("xmlns") {
                    return false;
                }
                k == local_name
                    || k.rsplit_once(':')
                        .map(|(_, l)| l == local_name)
                        .unwrap_or(false)
            })
            .map(|(_, v)| v.as_str())
    }

    /// Attribute value by 0-based index among non-xmlns attributes (C# `GetAttribute(int)`).
    pub fn get_attribute_at(&self, index: usize) -> Option<&str> {
        self.attributes
            .iter()
            .filter(|(k, _)| !k.starts_with("xmlns"))
            .nth(index)
            .map(|(_, v)| v.as_str())
    }

    /// Attribute value by local name + namespace URI when the xmlns for the prefix is on this element.
    pub fn get_attribute_ns(&self, local_name: &str, namespace_uri: &str) -> Option<&str> {
        // Resolve which prefixes map to namespace_uri on this start tag.
        let mut prefixes: Vec<&str> = Vec::new();
        for (k, v) in &self.attributes {
            if v != namespace_uri {
                continue;
            }
            if k == "xmlns" {
                prefixes.push("");
            } else if let Some(p) = k.strip_prefix("xmlns:") {
                prefixes.push(p);
            }
        }
        self.attributes.iter().find_map(|(k, v)| {
            if k.starts_with("xmlns") {
                return None;
            }
            let (pfx, local) = match k.split_once(':') {
                Some((p, l)) => (p, l),
                None => ("", k.as_str()),
            };
            if local == local_name && prefixes.iter().any(|p| *p == pfx) {
                Some(v.as_str())
            } else if local == local_name && namespace_uri.is_empty() && pfx.is_empty() {
                Some(v.as_str())
            } else {
                None
            }
        })
    }

    /// Close the reader (C# `Close` shell — marks EOF).
    pub fn close(&mut self) {
        self.eof = true;
        self.state = ElementState::EOF;
        self.local_name.clear();
        self.prefix = None;
        self.attributes.clear();
        self.text.clear();
        self.open_stack.clear();
        self.depth = 0;
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

    #[test]
    fn part_reader_namespace_declarations_and_get_attr() {
        let xml = br#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" w:rsidR="1"/>"#;
        let mut r = OpenXmlPartReader::from_bytes(xml);
        assert!(r.read().unwrap());
        let decls = r.namespace_declarations();
        assert!(
            decls.iter().any(|(p, u)| p == "w"
                && u == "http://schemas.openxmlformats.org/wordprocessingml/2006/main"),
            "{decls:?}"
        );
        assert!(decls.iter().any(|(p, _)| p == "r"), "{decls:?}");
        assert_eq!(r.get_attribute("rsidR"), Some("1"));
        assert!(r.has_attributes());
        assert_eq!(r.attribute_count(), 1);
        assert_eq!(r.get_attribute_at(0), Some("1"));
        assert_eq!(
            r.get_attribute_ns(
                "rsidR",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            ),
            Some("1")
        );
        r.close();
        assert!(r.is_eof());
    }

    #[test]
    fn part_reader_line_info() {
        let xml = b"<a>\n<b/>\n</a>";
        let mut r = OpenXmlPartReader::from_bytes(xml);
        assert!(r.read().unwrap());
        assert_eq!(r.local_name(), "a");
        assert!(r.get_line_info().has_line_info());
        assert_eq!(r.get_line_info().line_number, 1);
        assert!(r.read().unwrap());
        assert_eq!(r.local_name(), "b");
        assert_eq!(r.get_line_info().line_number, 2);
        assert!(!r.has_value());
        assert!(!r.read_misc_nodes());
        assert!(r.encoding().is_none());
        assert!(r.standalone_xml().is_none());
    }

    #[test]
    fn part_reader_options_preserve_whitespace() {
        let xml = b"<p>\n  <t>x</t>\n</p>";
        let mut r = OpenXmlPartReader::create_with_options(
            xml,
            OpenXmlPartReaderOptions::default().with_ignore_whitespace(false),
        );
        assert!(r.read().unwrap()); // p
        assert!(r.read().unwrap()); // whitespace text
        assert_eq!(r.element_state(), ElementState::LeafText);
        assert!(!r.ignore_whitespace());
    }

    #[test]
    fn part_reader_max_characters_limit() {
        let xml = br#"<root><a/><b/><c/><d/><e/></root>"#;
        let mut r = OpenXmlPartReader::create_with_options(
            xml,
            OpenXmlPartReaderOptions::default().with_max_characters_in_part(8),
        );
        let mut hit_limit = false;
        loop {
            match r.read() {
                Ok(false) => break,
                Ok(true) => continue,
                Err(e) => {
                    hit_limit = e.to_string().contains("MaxCharactersInPart");
                    break;
                }
            }
        }
        assert!(hit_limit, "expected MaxCharactersInPart error");
    }

    #[test]
    fn part_reader_misc_nodes_when_enabled() {
        let xml = b"<root><!--c--><?pi data?><![CDATA[x]]><t>y</t></root>";
        let mut r = OpenXmlPartReader::create_with_misc(xml, true);
        assert!(r.read().unwrap()); // root
        assert!(r.is_start_element());
        assert!(r.read().unwrap());
        assert!(r.is_misc_node());
        assert_eq!(r.local_name(), "#comment");
        assert_eq!(r.get_text(), "c");
        assert!(r.read().unwrap());
        assert!(r.is_misc_node());
        assert_eq!(r.local_name(), "pi");
        assert_eq!(r.get_text(), "data");
        assert!(r.read().unwrap());
        assert!(r.is_misc_node());
        assert_eq!(r.local_name(), "#cdata-section");
        assert_eq!(r.get_text(), "x");
        assert!(r.read().unwrap()); // t
        assert!(r.is_start_element());
    }

    #[test]
    fn part_reader_skips_misc_by_default() {
        let xml = b"<root><!--c--><t>y</t></root>";
        let mut r = OpenXmlPartReader::from_bytes(xml);
        assert!(r.read().unwrap()); // root
        assert!(r.read().unwrap()); // t (comment skipped)
        assert_eq!(r.local_name(), "t");
        assert!(!r.is_misc_node());
    }
}
