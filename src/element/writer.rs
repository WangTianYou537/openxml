//! OpenXmlElement → XML writer.

use super::element::{OpenXmlElement, OpenXmlMiscKind};
use crate::error::Result;
use quick_xml::events::{BytesCData, BytesDecl, BytesEnd, BytesPI, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;

/// Serialize an element tree to UTF-8 XML bytes (with XML declaration).
pub fn write_element(root: &OpenXmlElement) -> Result<Vec<u8>> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes"))))?;
    // newline after decl
    write_node(&mut writer, root, true)?;
    Ok(writer.into_inner().into_inner())
}

/// Serialize without XML declaration (for fragments).
#[allow(dead_code)]
pub fn write_element_fragment(root: &OpenXmlElement) -> Result<Vec<u8>> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    write_node(&mut writer, root, true)?;
    Ok(writer.into_inner().into_inner())
}

fn write_node<W: std::io::Write>(
    writer: &mut Writer<W>,
    elem: &OpenXmlElement,
    is_root: bool,
) -> Result<()> {
    match elem.misc_kind {
        OpenXmlMiscKind::Comment => {
            let body = elem.text.as_deref().unwrap_or("");
            writer.write_event(Event::Comment(BytesText::new(body)))?;
            return Ok(());
        }
        OpenXmlMiscKind::ProcessingInstruction => {
            let target = elem.pi_target().unwrap_or("");
            let data = elem.text.as_deref().unwrap_or("");
            let raw = if data.is_empty() {
                target.to_string()
            } else {
                format!("{target} {data}")
            };
            writer.write_event(Event::PI(BytesPI::new(raw.as_str())))?;
            return Ok(());
        }
        OpenXmlMiscKind::CData => {
            let body = elem.text.as_deref().unwrap_or("");
            writer.write_event(Event::CData(BytesCData::new(body)))?;
            return Ok(());
        }
        OpenXmlMiscKind::None => {}
    }

    let qname = elem.qualified_name();
    let mut start = BytesStart::new(qname.clone());

    // Namespace declarations
    for (prefix, uri) in &elem.namespace_declarations {
        if prefix.is_empty() {
            start.push_attribute(("xmlns", uri.as_str()));
        } else {
            start.push_attribute((format!("xmlns:{prefix}").as_str(), uri.as_str()));
        }
    }

    // If root has a prefix/uri but no matching xmlns, emit it
    if is_root && !elem.prefix.is_empty() && !elem.namespace_uri.is_empty() {
        let has = elem
            .namespace_declarations
            .iter()
            .any(|(p, _)| p == &elem.prefix);
        if !has {
            start.push_attribute((
                format!("xmlns:{}", elem.prefix).as_str(),
                elem.namespace_uri.as_str(),
            ));
        }
    }

    for attr in &elem.attributes {
        let key = attr.qualified_name();
        start.push_attribute((key.as_str(), attr.value.as_str()));
    }

    let has_body = elem.text.is_some() || !elem.children.is_empty();
    if !has_body {
        writer.write_event(Event::Empty(start))?;
        return Ok(());
    }

    writer.write_event(Event::Start(start))?;

    if let Some(text) = &elem.text {
        // Use xml:space=preserve semantics: write text as-is (escaped)
        writer.write_event(Event::Text(BytesText::new(text)))?;
    }

    for child in &elem.children {
        write_node(writer, child, false)?;
    }

    writer.write_event(Event::End(BytesEnd::new(qname)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::parse_element;

    #[test]
    fn roundtrip_text() {
        let elem = OpenXmlElement::w("document")
            .with_ns_decl(
                "w",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            )
            .with_child(
                OpenXmlElement::w("body").with_child(
                    OpenXmlElement::w("p").with_child(
                        OpenXmlElement::w("r")
                            .with_child(OpenXmlElement::w("t").with_text("Hello & world")),
                    ),
                ),
            );
        let xml = write_element(&elem).unwrap();
        let parsed = parse_element(&xml).unwrap();
        assert_eq!(parsed.inner_text(), "Hello & world");
    }

    #[test]
    fn roundtrip_comment() {
        let elem = OpenXmlElement::new("", "", "root")
            .with_child(OpenXmlElement::comment(" hello "))
            .with_child(OpenXmlElement::new("", "", "child"));
        let xml = write_element(&elem).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("<!-- hello -->"), "{s}");
        let parsed = parse_element(&xml).unwrap();
        assert!(parsed
            .children
            .iter()
            .any(|c| c.misc_kind() == OpenXmlMiscKind::Comment));
    }
}
