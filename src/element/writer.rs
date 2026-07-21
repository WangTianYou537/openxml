//! OpenXmlElement → XML writer.

use super::element::OpenXmlElement;
use crate::error::Result;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
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
}
