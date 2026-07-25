//! Lightweight pull-style XML traversal over a part/document stream.
//!
//! This is **not** a full SAX `OpenXmlReader` port, but provides forward-only
//! event iteration so callers can scan large parts without building a full DOM.

use crate::error::{Error, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::{self, BufRead, Read};

/// Counts newlines as the underlying reader is consumed (for `IXmlLineInfo`).
struct LineCountingReader<R> {
    inner: R,
    line: u64,
    column: u64,
    /// Snapshot taken before each `read_event_into` call.
    mark_line: u64,
    mark_column: u64,
}

impl<R> LineCountingReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            line: 1,
            column: 1,
            mark_line: 1,
            mark_column: 1,
        }
    }

    fn mark(&mut self) {
        self.mark_line = self.line;
        self.mark_column = self.column;
    }

    fn note_bytes(&mut self, data: &[u8]) {
        for &b in data {
            if b == b'\n' {
                self.line = self.line.saturating_add(1);
                self.column = 1;
            } else {
                self.column = self.column.saturating_add(1);
            }
        }
    }
}

impl<R: Read> Read for LineCountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.note_bytes(&buf[..n]);
        Ok(n)
    }
}

impl<R: BufRead> BufRead for LineCountingReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        // Count bytes being consumed from the current buffer view.
        // `fill_buf` may have been called; we must count only the slice consumed.
        // Safe approach: peek then consume.
        if amt == 0 {
            return;
        }
        // Re-borrow: get the bytes about to be consumed.
        // `fill_buf` is already valid for at least `amt` after a successful read path.
        if let Ok(buf) = self.inner.fill_buf() {
            let n = amt.min(buf.len());
            // Copy bytes to count (cannot hold borrow across mutate of line counters via note
            // while also calling consume on inner — copy first).
            let chunk: Vec<u8> = buf[..n].to_vec();
            self.inner.consume(n);
            self.note_bytes(&chunk);
            if amt > n {
                // Should not happen with correct BufRead users; ignore remainder.
            }
        } else {
            self.inner.consume(amt);
        }
    }
}

/// A streaming XML event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlEvent {
    /// Start of an element (`local_name`, optional prefix, attributes as `(qname, value)`).
    Start {
        prefix: Option<String>,
        local_name: String,
        attributes: Vec<(String, String)>,
    },
    /// Empty element (start+end combined).
    Empty {
        prefix: Option<String>,
        local_name: String,
        attributes: Vec<(String, String)>,
    },
    /// End of an element.
    End {
        prefix: Option<String>,
        local_name: String,
    },
    /// Character data (decoded, unescaped).
    Text(String),
}

/// Forward-only reader over an XML byte stream.
pub struct OpenXmlStreamReader<R: BufRead> {
    reader: Reader<LineCountingReader<R>>,
    buf: Vec<u8>,
    /// Line/column of the most recently *returned* event start.
    event_line: u64,
    event_column: u64,
}

impl<R: BufRead> OpenXmlStreamReader<R> {
    pub fn from_reader(reader: R) -> Self {
        let mut reader = Reader::from_reader(LineCountingReader::new(reader));
        reader.config_mut().trim_text(false);
        Self {
            reader,
            buf: Vec::new(),
            event_line: 0,
            event_column: 0,
        }
    }

    /// Line/position of the last returned event (C# `IXmlLineInfo` subset).
    pub fn line_info(&self) -> super::xml_path::XmlLineInfo {
        if self.event_line == 0 {
            super::xml_path::XmlLineInfo::EMPTY
        } else {
            super::xml_path::XmlLineInfo::new(self.event_line, self.event_column)
        }
    }

    /// Absolute byte offset in the input (quick-xml `buffer_position`).
    pub fn buffer_position(&self) -> u64 {
        self.reader.buffer_position()
    }

    /// Read the next event; `None` at EOF.
    pub fn read_event(&mut self) -> Result<Option<XmlEvent>> {
        loop {
            self.buf.clear();
            // Mark line/col before consuming the next event's bytes.
            self.reader.get_mut().mark();
            let mark_line = self.reader.get_mut().mark_line;
            let mark_col = self.reader.get_mut().mark_column;
            let event = self
                .reader
                .read_event_into(&mut self.buf)
                .map_err(|e| Error::Xml(e.to_string()))?;
            match event {
                Event::Start(e) => {
                    self.event_line = mark_line;
                    self.event_column = mark_col;
                    return Ok(Some(Self::start_event(&e, false)?));
                }
                Event::Empty(e) => {
                    self.event_line = mark_line;
                    self.event_column = mark_col;
                    return Ok(Some(Self::start_event(&e, true)?));
                }
                Event::End(e) => {
                    self.event_line = mark_line;
                    self.event_column = mark_col;
                    let name = e.name();
                    let (prefix, local) = split_name(name.as_ref());
                    return Ok(Some(XmlEvent::End {
                        prefix,
                        local_name: local,
                    }));
                }
                Event::Text(t) => {
                    let text = t
                        .unescape()
                        .map_err(|e| Error::Xml(e.to_string()))?
                        .into_owned();
                    if text.is_empty() {
                        continue;
                    }
                    self.event_line = mark_line;
                    self.event_column = mark_col;
                    return Ok(Some(XmlEvent::Text(text)));
                }
                Event::CData(c) => {
                    let text = String::from_utf8_lossy(&c).into_owned();
                    if text.is_empty() {
                        continue;
                    }
                    self.event_line = mark_line;
                    self.event_column = mark_col;
                    return Ok(Some(XmlEvent::Text(text)));
                }
                Event::Eof => {
                    self.event_line = 0;
                    self.event_column = 0;
                    return Ok(None);
                }
                // Skip declarations, comments, PIs, doc types (bytes already counted).
                _ => continue,
            }
        }
    }

    /// Collect all local-name text under elements matching `local_name` (flat scan).
    pub fn collect_text_under(&mut self, local_name: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();
        let mut depth = 0i32;
        let mut capturing = false;
        let mut buf = String::new();
        while let Some(ev) = self.read_event()? {
            match ev {
                XmlEvent::Start {
                    local_name: ref n, ..
                }
                | XmlEvent::Empty {
                    local_name: ref n, ..
                } => {
                    if n == local_name {
                        capturing = true;
                        depth = 1;
                        buf.clear();
                        if matches!(ev, XmlEvent::Empty { .. }) {
                            out.push(std::mem::take(&mut buf));
                            capturing = false;
                            depth = 0;
                        }
                    } else if capturing {
                        depth += 1;
                    }
                }
                XmlEvent::End {
                    local_name: ref n, ..
                } => {
                    if capturing {
                        depth -= 1;
                        if depth == 0 && n == local_name {
                            out.push(std::mem::take(&mut buf));
                            capturing = false;
                        }
                    }
                }
                XmlEvent::Text(t) if capturing => buf.push_str(&t),
                _ => {}
            }
        }
        Ok(out)
    }

    fn start_event(
        e: &quick_xml::events::BytesStart<'_>,
        empty: bool,
    ) -> Result<XmlEvent> {
        let name = e.name();
        let (prefix, local) = split_name(name.as_ref());
        let mut attributes = Vec::new();
        for attr in e.attributes().with_checks(false) {
            let attr = attr.map_err(|err| Error::Xml(err.to_string()))?;
            let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
            let val = attr
                .unescape_value()
                .map_err(|err| Error::Xml(err.to_string()))?
                .into_owned();
            attributes.push((key, val));
        }
        if empty {
            Ok(XmlEvent::Empty {
                prefix,
                local_name: local,
                attributes,
            })
        } else {
            Ok(XmlEvent::Start {
                prefix,
                local_name: local,
                attributes,
            })
        }
    }
}

impl OpenXmlStreamReader<&[u8]> {
    pub fn from_bytes(data: &[u8]) -> OpenXmlStreamReader<&[u8]> {
        OpenXmlStreamReader::from_reader(data)
    }
}

fn split_name(raw: &[u8]) -> (Option<String>, String) {
    let s = String::from_utf8_lossy(raw);
    if let Some((p, l)) = s.split_once(':') {
        (Some(p.to_string()), l.to_string())
    } else {
        (None, s.into_owned())
    }
}

/// Write a sequence of streaming events to XML bytes (fragment, no declaration).
pub fn write_xml_events(events: &[XmlEvent]) -> Result<Vec<u8>> {
    use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event as QEvent};
    use quick_xml::Writer;
    use std::io::Cursor;

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    for ev in events {
        match ev {
            XmlEvent::Start {
                prefix,
                local_name,
                attributes,
            } => {
                let name = qname(prefix.as_deref(), local_name);
                let mut start = BytesStart::new(name);
                for (k, v) in attributes {
                    start.push_attribute((k.as_str(), v.as_str()));
                }
                writer
                    .write_event(QEvent::Start(start))
                    .map_err(|e| Error::Xml(e.to_string()))?;
            }
            XmlEvent::Empty {
                prefix,
                local_name,
                attributes,
            } => {
                let name = qname(prefix.as_deref(), local_name);
                let mut start = BytesStart::new(name);
                for (k, v) in attributes {
                    start.push_attribute((k.as_str(), v.as_str()));
                }
                writer
                    .write_event(QEvent::Empty(start))
                    .map_err(|e| Error::Xml(e.to_string()))?;
            }
            XmlEvent::End {
                prefix,
                local_name,
            } => {
                let name = qname(prefix.as_deref(), local_name);
                writer
                    .write_event(QEvent::End(BytesEnd::new(name)))
                    .map_err(|e| Error::Xml(e.to_string()))?;
            }
            XmlEvent::Text(t) => {
                writer
                    .write_event(QEvent::Text(BytesText::new(t)))
                    .map_err(|e| Error::Xml(e.to_string()))?;
            }
        }
    }
    Ok(writer.into_inner().into_inner())
}

fn qname(prefix: Option<&str>, local: &str) -> String {
    match prefix {
        Some(p) if !p.is_empty() => format!("{p}:{local}"),
        _ => local.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_read_paragraphs() {
        let xml = br#"<?xml version="1.0"?>
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
          <w:body>
            <w:p><w:r><w:t>Hello</w:t></w:r></w:p>
            <w:p><w:r><w:t>World</w:t></w:r></w:p>
          </w:body>
        </w:document>"#;
        let mut r = OpenXmlStreamReader::from_bytes(xml);
        let texts = r.collect_text_under("t").unwrap();
        assert_eq!(texts, vec!["Hello".to_string(), "World".to_string()]);
    }

    #[test]
    fn stream_write_roundtrip_events() {
        let events = vec![
            XmlEvent::Start {
                prefix: Some("w".into()),
                local_name: "t".into(),
                attributes: vec![],
            },
            XmlEvent::Text("hi".into()),
            XmlEvent::End {
                prefix: Some("w".into()),
                local_name: "t".into(),
            },
        ];
        let xml = write_xml_events(&events).unwrap();
        let s = String::from_utf8(xml).unwrap();
        assert!(s.contains("hi"));
    }

    #[test]
    fn stream_line_info_tracks_newlines() {
        let xml = b"<root>\n  <child/>\n</root>";
        let mut r = OpenXmlStreamReader::from_bytes(xml);
        let ev = r.read_event().unwrap().unwrap();
        assert!(matches!(ev, XmlEvent::Start { local_name: ref n, .. } if n == "root"));
        let li = r.line_info();
        assert!(li.has_line_info(), "{li:?}");
        assert_eq!(li.line_number, 1, "{li:?}");

        // Skip insignificant whitespace Text events between elements.
        let mut child_li = None;
        while let Some(ev) = r.read_event().unwrap() {
            if matches!(ev, XmlEvent::Empty { local_name: ref n, .. } if n == "child") {
                child_li = Some(r.line_info());
                break;
            }
        }
        let li = child_li.expect("child empty element");
        assert_eq!(li.line_number, 2, "{li:?}");
    }
}
