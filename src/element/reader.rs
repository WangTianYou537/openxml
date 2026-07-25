//! XML → OpenXmlElement parser.

use super::element::{OpenXmlAttribute, OpenXmlElement, OpenXmlMiscKind};
use crate::error::{Error, Result};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

/// Parse an XML document (or fragment with a single root) into an `OpenXmlElement`.
///
/// Comments and processing instructions under the root are preserved as
/// [`OpenXmlMiscKind`] children (C# `OpenXmlMiscNode` parity). Whitespace-only
/// text between elements is still dropped unless the parent is a text element.
pub fn parse_element(xml: impl AsRef<[u8]>) -> Result<OpenXmlElement> {
    let mut reader = Reader::from_reader(xml.as_ref());
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut stack: Vec<OpenXmlElement> = Vec::new();
    let mut root: Option<OpenXmlElement> = None;
    // Accumulate text for the current element
    let mut text_buf = String::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                flush_text(&mut stack, &mut text_buf);
                let elem = element_from_start(&e, false)?;
                stack.push(elem);
            }
            Event::Empty(e) => {
                flush_text(&mut stack, &mut text_buf);
                let elem = element_from_start(&e, true)?;
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(elem);
                } else if root.is_none() {
                    root = Some(elem);
                } else {
                    return Err(Error::Xml("multiple root elements".into()));
                }
            }
            Event::End(_) => {
                flush_text(&mut stack, &mut text_buf);
                let elem = stack
                    .pop()
                    .ok_or_else(|| Error::Xml("unexpected end element".into()))?;
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(elem);
                } else if root.is_none() {
                    root = Some(elem);
                } else {
                    return Err(Error::Xml("multiple root elements".into()));
                }
            }
            Event::Text(t) => {
                let decoded = t.unescape().map_err(|e| Error::Xml(e.to_string()))?;
                text_buf.push_str(&decoded);
            }
            Event::CData(t) => {
                flush_text(&mut stack, &mut text_buf);
                let data = String::from_utf8_lossy(t.as_ref()).into_owned();
                // Prefer a dedicated CDATA misc child when nested; for leaf text
                // parents still append as element text if no siblings yet.
                if let Some(parent) = stack.last_mut() {
                    if parent.children.is_empty()
                        && parent.text.is_none()
                        && is_text_element(&parent.local_name)
                    {
                        parent.text = Some(data);
                    } else {
                        parent.children.push(OpenXmlElement::cdata(data));
                    }
                }
            }
            Event::Comment(c) => {
                flush_text(&mut stack, &mut text_buf);
                let data = String::from_utf8_lossy(c.as_ref()).into_owned();
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(OpenXmlElement::comment(data));
                }
                // Comments before/after root are dropped (no place to hang them).
            }
            Event::PI(p) => {
                flush_text(&mut stack, &mut text_buf);
                // quick-xml: PI target is before first space; rest is data
                let raw = String::from_utf8_lossy(p.as_ref());
                let (target, data) = match raw.split_once(|c: char| c.is_whitespace()) {
                    Some((t, d)) => (t.to_string(), d.trim_start().to_string()),
                    None => (raw.into_owned(), String::new()),
                };
                if let Some(parent) = stack.last_mut() {
                    parent
                        .children
                        .push(OpenXmlElement::processing_instruction(target, data));
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    root.ok_or_else(|| Error::Xml("no root element found".into()))
}

fn flush_text(stack: &mut [OpenXmlElement], text_buf: &mut String) {
    if text_buf.is_empty() {
        return;
    }
    // Only keep text if it has non-whitespace, OR if the element is a known text element
    // Always store; consumers can decide. For leaf text we need it.
    if let Some(elem) = stack.last_mut() {
        let t = text_buf.clone();
        // Merge with existing
        match &mut elem.text {
            Some(existing) => existing.push_str(&t),
            None => {
                // Skip pure whitespace text nodes between elements for non-text parents
                // when they look like indentation — but keep for elements that already
                // have no children and match common text element names.
                let is_indent = t.chars().all(|c| c.is_whitespace());
                if !is_indent || is_text_element(&elem.local_name) {
                    elem.text = Some(t);
                }
            }
        }
    }
    text_buf.clear();
}

fn is_text_element(local: &str) -> bool {
    matches!(
        local,
        "t" | "delText" | "instrText" | "delInstrText" | "v" | "f" | "text"
    )
}

fn element_from_start(e: &quick_xml::events::BytesStart<'_>, _empty: bool) -> Result<OpenXmlElement> {
    let (prefix, local) = split_qname(e.name());
    let mut elem = OpenXmlElement {
        prefix,
        namespace_uri: String::new(), // filled from xmlns if present
        local_name: local,
        attributes: Vec::new(),
        namespace_declarations: Vec::new(),
        children: Vec::new(),
        text: None,
        raw_outer_xml: None,
        misc_kind: OpenXmlMiscKind::None,
        annotations: Vec::new(),
    };

    for a in e.attributes().with_checks(false) {
        let a = a.map_err(|err| Error::Xml(err.to_string()))?;
        let key = String::from_utf8_lossy(a.key.as_ref()).into_owned();
        let value = a
            .unescape_value()
            .map_err(|err| Error::Xml(err.to_string()))?
            .into_owned();

        if key == "xmlns" {
            elem.namespace_uri = value.clone();
            elem.namespace_declarations.push((String::new(), value));
        } else if let Some(pfx) = key.strip_prefix("xmlns:") {
            if elem.prefix == pfx {
                elem.namespace_uri = value.clone();
            }
            elem.namespace_declarations
                .push((pfx.to_string(), value));
        } else {
            let (ap, al) = split_name_str(&key);
            elem.attributes.push(OpenXmlAttribute {
                prefix: if ap.is_empty() { None } else { Some(ap) },
                namespace_uri: None,
                local_name: al,
                value,
            });
        }
    }

    Ok(elem)
}

fn split_qname(name: QName<'_>) -> (String, String) {
    let s = String::from_utf8_lossy(name.as_ref());
    split_name_str(&s)
}

fn split_name_str(s: &str) -> (String, String) {
    if let Some((p, l)) = s.split_once(':') {
        (p.to_string(), l.to_string())
    } else {
        (String::new(), s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let xml = br#"<?xml version="1.0"?>
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
          <w:body>
            <w:p>
              <w:r>
                <w:t>Hello</w:t>
              </w:r>
            </w:p>
          </w:body>
        </w:document>"#;
        let root = parse_element(xml).unwrap();
        assert_eq!(root.local_name, "document");
        assert_eq!(root.inner_text().trim(), "Hello");
        let body = root.child("body").unwrap();
        let p = body.child("p").unwrap();
        assert!(p.child("r").is_some());
    }

    #[test]
    fn parse_preserves_comment_and_pi() {
        let xml = br#"<?xml version="1.0"?>
        <root>
          <!-- note -->
          <?mso-application progid="Word.Document"?>
          <child/>
        </root>"#;
        let root = parse_element(xml).unwrap();
        let kinds: Vec<_> = root.children.iter().map(|c| c.misc_kind()).collect();
        assert!(kinds.contains(&OpenXmlMiscKind::Comment));
        assert!(kinds.contains(&OpenXmlMiscKind::ProcessingInstruction));
        let comment = root
            .children
            .iter()
            .find(|c| c.misc_kind() == OpenXmlMiscKind::Comment)
            .unwrap();
        assert_eq!(comment.text_value().unwrap().trim(), "note");
        let pi = root
            .children
            .iter()
            .find(|c| c.misc_kind() == OpenXmlMiscKind::ProcessingInstruction)
            .unwrap();
        assert_eq!(pi.pi_target(), Some("mso-application"));
    }
}
