//! Lightweight pull-style XML traversal over a part/document stream.
//!
//! This is **not** a full SAX `OpenXmlReader` port, but provides forward-only
//! event iteration so callers can scan large parts without building a full DOM.

use crate::error::{Error, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::BufRead;

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
    reader: Reader<R>,
    buf: Vec<u8>,
}

impl<R: BufRead> OpenXmlStreamReader<R> {
    pub fn from_reader(reader: R) -> Self {
        let mut reader = Reader::from_reader(reader);
        reader.config_mut().trim_text(false);
        Self {
            reader,
            buf: Vec::new(),
        }
    }

    /// Read the next event; `None` at EOF.
    pub fn read_event(&mut self) -> Result<Option<XmlEvent>> {
        loop {
            self.buf.clear();
            let event = self
                .reader
                .read_event_into(&mut self.buf)
                .map_err(|e| Error::Xml(e.to_string()))?;
            match event {
                Event::Start(e) => {
                    return Ok(Some(Self::start_event(&e, false)?));
                }
                Event::Empty(e) => {
                    return Ok(Some(Self::start_event(&e, true)?));
                }
                Event::End(e) => {
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
                    return Ok(Some(XmlEvent::Text(text)));
                }
                Event::CData(c) => {
                    let text = String::from_utf8_lossy(&c).into_owned();
                    if text.is_empty() {
                        continue;
                    }
                    return Ok(Some(XmlEvent::Text(text)));
                }
                Event::Eof => return Ok(None),
                // Skip declarations, comments, PIs, doc types
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
}
