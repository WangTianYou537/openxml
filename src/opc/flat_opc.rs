//! Flat OPC — single-XML representation of an Open XML package.
//!
//! Format (namespace `http://schemas.microsoft.com/office/2006/xmlPackage`):
//! ```xml
//! <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
//! <?mso-application progid="Word.Document"?>
//! <pkg:package xmlns:pkg="...">
//!   <pkg:part pkg:name="/_rels/.rels" pkg:contentType="..." pkg:padding="512">
//!     <pkg:xmlData>...</pkg:xmlData>
//!   </pkg:part>
//!   <pkg:part pkg:name="/word/media/image1.png" pkg:contentType="image/png">
//!     <pkg:binaryData>base64...</pkg:binaryData>
//!   </pkg:part>
//! </pkg:package>
//! ```

use crate::error::{Error, Result};
use crate::namespace::content_type as ct;
use crate::opc::{ContentTypes, OpcPackage, PackUri, Relationships};
use quick_xml::events::{BytesDecl, BytesEnd, BytesPI, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

const PKG_NS: &str = "http://schemas.microsoft.com/office/2006/xmlPackage";

/// Convert an OPC package to a Flat OPC XML document (UTF-8 bytes).
pub fn to_flat_opc(package: &OpcPackage, progid: Option<&str>) -> Result<Vec<u8>> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes"))))?;

    if let Some(id) = progid {
        // Processing instruction: <?mso-application progid="Word.Document"?>
        let pi = format!(r#"mso-application progid="{id}""#);
        writer.write_event(Event::PI(BytesPI::new(pi)))?;
    }

    let mut root = BytesStart::new("pkg:package");
    root.push_attribute(("xmlns:pkg", PKG_NS));
    writer.write_event(Event::Start(root))?;

    // Content types
    write_xml_part(
        &mut writer,
        "/[Content_Types].xml",
        "application/xml",
        &package.content_types().to_xml()?,
    )?;

    // Package relationships
    write_xml_part(
        &mut writer,
        "/_rels/.rels",
        ct::RELATIONSHIPS,
        &package.package_relationships().to_xml()?,
    )?;

    // Parts
    for uri in package.part_uris() {
        let data = package
            .get_part(&uri)
            .ok_or_else(|| Error::PartNotFound(uri.to_string()))?;
        let content_type = package
            .content_types()
            .content_type_for(uri.as_str())
            .unwrap_or("application/octet-stream");

        if looks_like_xml(content_type, data) {
            write_xml_part(&mut writer, uri.as_str(), content_type, data)?;
        } else {
            write_binary_part(&mut writer, uri.as_str(), content_type, data)?;
        }
    }

    // Part relationships
    for uri in package.part_uris() {
        if let Some(rels) = package.part_relationships(&uri) {
            if rels.is_empty() {
                continue;
            }
            let rel_uri = uri.relationship_part_uri();
            write_xml_part(
                &mut writer,
                rel_uri.as_str(),
                ct::RELATIONSHIPS,
                &rels.to_xml()?,
            )?;
        }
    }

    writer.write_event(Event::End(BytesEnd::new("pkg:package")))?;
    Ok(writer.into_inner().into_inner())
}

/// Parse a Flat OPC XML document into an in-memory OPC package.
pub fn from_flat_opc(xml: impl AsRef<[u8]>) -> Result<OpcPackage> {
    let mut reader = Reader::from_reader(xml.as_ref());
    reader.config_mut().trim_text(false);

    let mut package = OpcPackage::create();
    let mut buf = Vec::new();

    // State for current part
    let mut part_name: Option<String> = None;
    let mut part_ct: Option<String> = None;
    let mut in_xml_data = false;
    let mut in_binary_data = false;
    let mut xml_data = Vec::new();
    let mut binary_text = String::new();
    // Depth tracking for nested XML inside xmlData
    let mut xml_depth: i32 = 0;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let local = local_name(e.name().as_ref());
                if local == "part" {
                    part_name = attr_pkg(&e, "name");
                    part_ct = attr_pkg(&e, "contentType");
                    xml_data.clear();
                    binary_text.clear();
                    in_xml_data = false;
                    in_binary_data = false;
                } else if local == "xmlData" {
                    in_xml_data = true;
                    xml_depth = 0;
                    xml_data.clear();
                } else if local == "binaryData" {
                    in_binary_data = true;
                    binary_text.clear();
                } else if in_xml_data {
                    // Capture nested XML as raw bytes
                    write_start_raw(&mut xml_data, &e)?;
                    xml_depth += 1;
                }
            }
            Event::Empty(e) => {
                if in_xml_data {
                    write_empty_raw(&mut xml_data, &e)?;
                } else {
                    let local = local_name(e.name().as_ref());
                    if local == "part" {
                        // empty part — ignore
                        part_name = None;
                        part_ct = None;
                    }
                }
            }
            Event::End(e) => {
                let local = local_name(e.name().as_ref());
                if in_xml_data && local != "xmlData" {
                    write_end_raw(&mut xml_data, &e)?;
                    xml_depth -= 1;
                } else if local == "xmlData" {
                    in_xml_data = false;
                } else if local == "binaryData" {
                    in_binary_data = false;
                } else if local == "part" {
                    if let (Some(name), Some(ct_val)) = (part_name.take(), part_ct.take()) {
                        store_part(&mut package, &name, &ct_val, &xml_data, &binary_text)?;
                    }
                    xml_data.clear();
                    binary_text.clear();
                }
            }
            Event::Text(t) => {
                if in_binary_data {
                    let decoded = t.unescape().map_err(|e| Error::Xml(e.to_string()))?;
                    binary_text.push_str(&decoded);
                } else if in_xml_data {
                    let raw = t.as_ref();
                    xml_data.extend_from_slice(raw);
                }
            }
            Event::CData(t) => {
                if in_xml_data {
                    xml_data.extend_from_slice(b"<![CDATA[");
                    xml_data.extend_from_slice(t.as_ref());
                    xml_data.extend_from_slice(b"]]>");
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    let _ = xml_depth;
    Ok(package)
}

fn store_part(
    package: &mut OpcPackage,
    name: &str,
    content_type: &str,
    xml_data: &[u8],
    binary_text: &str,
) -> Result<()> {
    let uri = PackUri::new(name);

    // Special handling for content types and relationships
    if uri.as_str() == "/[Content_Types].xml" {
        let ct = ContentTypes::parse(xml_data)?;
        *package.content_types_mut() = ct;
        return Ok(());
    }

    if uri.as_str() == "/_rels/.rels" {
        let rels = Relationships::parse(xml_data)?;
        *package.package_relationships_mut() = rels;
        return Ok(());
    }

    if uri.as_str().contains("/_rels/") && uri.as_str().ends_with(".rels") {
        let source = part_uri_from_rels_uri(&uri);
        let rels = Relationships::parse(xml_data)?;
        *package.part_relationships_mut(&source) = rels;
        return Ok(());
    }

    let data = if !xml_data.is_empty() {
        // Ensure XML declaration for consistency
        let mut out = Vec::new();
        if !xml_data.starts_with(b"<?xml") {
            out.extend_from_slice(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        }
        out.extend_from_slice(xml_data);
        out
    } else {
        // base64 binary
        let cleaned: String = binary_text.chars().filter(|c| !c.is_whitespace()).collect();
        base64_decode(&cleaned)?
    };

    package.set_part(uri, content_type, data);
    Ok(())
}

fn write_xml_part<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    content_type: &str,
    data: &[u8],
) -> Result<()> {
    let mut part = BytesStart::new("pkg:part");
    part.push_attribute(("pkg:name", name));
    part.push_attribute(("pkg:contentType", content_type));
    writer.write_event(Event::Start(part))?;
    writer.write_event(Event::Start(BytesStart::new("pkg:xmlData")))?;

    // Strip XML declaration from payload if present
    let payload = strip_xml_decl(data);
    // Write as raw text (unescaped) — Flat OPC embeds XML as child content.
    // quick-xml BytesText escapes, so write via a workaround: parse and re-emit
    // as nested events would be ideal; for simplicity write escaped then note
    // that for round-trip we also accept the raw form. Better: write raw bytes
    // using a custom approach.
    write_raw_xml_fragment(writer, payload)?;

    writer.write_event(Event::End(BytesEnd::new("pkg:xmlData")))?;
    writer.write_event(Event::End(BytesEnd::new("pkg:part")))?;
    Ok(())
}

fn write_binary_part<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    content_type: &str,
    data: &[u8],
) -> Result<()> {
    let mut part = BytesStart::new("pkg:part");
    part.push_attribute(("pkg:name", name));
    part.push_attribute(("pkg:contentType", content_type));
    writer.write_event(Event::Start(part))?;
    writer.write_event(Event::Start(BytesStart::new("pkg:binaryData")))?;
    let b64 = base64_encode(data);
    writer.write_event(Event::Text(BytesText::new(&b64)))?;
    writer.write_event(Event::End(BytesEnd::new("pkg:binaryData")))?;
    writer.write_event(Event::End(BytesEnd::new("pkg:part")))?;
    Ok(())
}

/// Write an XML fragment as raw nested content (re-parse and re-emit events).
fn write_raw_xml_fragment<W: std::io::Write>(writer: &mut Writer<W>, data: &[u8]) -> Result<()> {
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                writer.write_event(Event::Start(e.into_owned()))?;
            }
            Event::Empty(e) => {
                writer.write_event(Event::Empty(e.into_owned()))?;
            }
            Event::End(e) => {
                writer.write_event(Event::End(e.into_owned()))?;
            }
            Event::Text(t) => {
                writer.write_event(Event::Text(t.into_owned()))?;
            }
            Event::CData(t) => {
                writer.write_event(Event::CData(t.into_owned()))?;
            }
            Event::Comment(t) => {
                writer.write_event(Event::Comment(t.into_owned()))?;
            }
            Event::Decl(_) => {} // skip
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}

fn looks_like_xml(content_type: &str, data: &[u8]) -> bool {
    if content_type.contains("xml") || content_type.ends_with("+xml") {
        return true;
    }
    let trimmed = data
        .iter()
        .skip_while(|b| b.is_ascii_whitespace())
        .copied()
        .take(5)
        .collect::<Vec<_>>();
    trimmed.starts_with(b"<?xml") || trimmed.starts_with(b"<")
}

fn strip_xml_decl(data: &[u8]) -> &[u8] {
    let s = std::str::from_utf8(data).unwrap_or("");
    let trimmed = s.trim_start();
    if let Some(rest) = trimmed.strip_prefix("<?xml") {
        if let Some(idx) = rest.find("?>") {
            return rest[idx + 2..].trim_start().as_bytes();
        }
    }
    data
}

fn part_uri_from_rels_uri(rels_uri: &PackUri) -> PackUri {
    let s = rels_uri.as_str();
    if let Some(idx) = s.find("/_rels/") {
        let dir = &s[..idx];
        let rest = &s[idx + "/_rels/".len()..];
        let name = rest.strip_suffix(".rels").unwrap_or(rest);
        if dir.is_empty() {
            PackUri::new(format!("/{name}"))
        } else {
            PackUri::new(format!("{dir}/{name}"))
        }
    } else {
        PackUri::new("/")
    }
}

fn local_name(name: &[u8]) -> String {
    let s = String::from_utf8_lossy(name);
    s.rsplit(':').next().unwrap_or(&s).to_string()
}

fn attr_pkg(e: &BytesStart<'_>, key: &str) -> Option<String> {
    for a in e.attributes().with_checks(false).flatten() {
        let k = String::from_utf8_lossy(a.key.as_ref());
        let local = k.rsplit(':').next().unwrap_or(&k);
        if local == key {
            return Some(String::from_utf8_lossy(&a.value).into_owned());
        }
    }
    None
}

fn write_start_raw(out: &mut Vec<u8>, e: &BytesStart<'_>) -> Result<()> {
    out.push(b'<');
    out.extend_from_slice(e.name().as_ref());
    for a in e.attributes().with_checks(false).flatten() {
        out.push(b' ');
        out.extend_from_slice(a.key.as_ref());
        out.extend_from_slice(b"=\"");
        out.extend_from_slice(&a.value);
        out.push(b'"');
    }
    out.push(b'>');
    Ok(())
}

fn write_empty_raw(out: &mut Vec<u8>, e: &BytesStart<'_>) -> Result<()> {
    out.push(b'<');
    out.extend_from_slice(e.name().as_ref());
    for a in e.attributes().with_checks(false).flatten() {
        out.push(b' ');
        out.extend_from_slice(a.key.as_ref());
        out.extend_from_slice(b"=\"");
        out.extend_from_slice(&a.value);
        out.push(b'"');
    }
    out.extend_from_slice(b"/>");
    Ok(())
}

fn write_end_raw(out: &mut Vec<u8>, e: &quick_xml::events::BytesEnd<'_>) -> Result<()> {
    out.extend_from_slice(b"</");
    out.extend_from_slice(e.name().as_ref());
    out.push(b'>');
    Ok(())
}

// Minimal base64 (no external dep)
fn base64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(T[((n >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(T[(n & 63) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn base64_decode(s: &str) -> Result<Vec<u8>> {
    fn val(c: u8) -> Result<u8> {
        match c {
            b'A'..=b'Z' => Ok(c - b'A'),
            b'a'..=b'z' => Ok(c - b'a' + 26),
            b'0'..=b'9' => Ok(c - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            _ => Err(Error::Package(format!("invalid base64 char: {}", c as char))),
        }
    }
    let pad = s.chars().rev().take_while(|&c| c == '=').count();
    let bytes: Vec<u8> = s.bytes().filter(|&b| b != b'=').collect();
    let mut out = Vec::new();
    for chunk in bytes.chunks(4) {
        let v0 = val(chunk[0])? as u32;
        let v1 = if chunk.len() > 1 {
            val(chunk[1])? as u32
        } else {
            0
        };
        let v2 = if chunk.len() > 2 {
            val(chunk[2])? as u32
        } else {
            0
        };
        let v3 = if chunk.len() > 3 {
            val(chunk[3])? as u32
        } else {
            0
        };
        let n = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3;
        out.push(((n >> 16) & 0xFF) as u8);
        if chunk.len() > 2 {
            out.push(((n >> 8) & 0xFF) as u8);
        }
        if chunk.len() > 3 {
            out.push((n & 0xFF) as u8);
        }
    }
    // With padding, a final 3-char group (one `=`) yields 2 bytes already via
    // chunk.len()==3; a final 2-char group (two `=`) yields 1 byte via
    // chunk.len()==2. No further truncation needed. Keep `pad` for validation.
    let _ = pad;
    Ok(out)
}

/// ProgIDs used by Microsoft Office for Flat OPC processing instructions.
pub mod progid {
    pub const WORD: &str = "Word.Document";
    pub const EXCEL: &str = "Excel.Sheet";
    pub const POWERPOINT: &str = "PowerPoint.Show";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::{content_type, rel};
    use crate::opc::RelationshipTargetMode;

    #[test]
    fn base64_roundtrip() {
        let data = b"Hello, Flat OPC!";
        let enc = base64_encode(data);
        let dec = base64_decode(&enc).unwrap();
        assert_eq!(dec, data);
    }

    #[test]
    fn flat_opc_roundtrip() {
        let mut pkg = OpcPackage::create();
        pkg.set_part(
            "/word/document.xml",
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>Hi</w:t></w:r></w:p></w:body></w:document>"#.to_vec(),
        );
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &PackUri::new("/word/document.xml"),
            RelationshipTargetMode::Internal,
        );

        let flat = to_flat_opc(&pkg, Some(progid::WORD)).unwrap();
        let flat_str = String::from_utf8_lossy(&flat);
        assert!(flat_str.contains("pkg:package"));
        assert!(flat_str.contains("/word/document.xml"));

        let opened = from_flat_opc(&flat).unwrap();
        assert!(opened.has_part(&PackUri::new("/word/document.xml")));
        let main = opened.main_part_uri(rel::OFFICE_DOCUMENT).unwrap();
        assert_eq!(main.as_str(), "/word/document.xml");
        let doc = opened.get_part_str(&main).unwrap().unwrap();
        assert!(doc.contains("Hi"));
    }
}
