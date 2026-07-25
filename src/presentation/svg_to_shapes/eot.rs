//! Embedded OpenType (EOT) / PowerPoint `.fntdata` writer.
//!
//! Microsoft PowerPoint embeds fonts as EOT payloads in `ppt/fonts/fontN.fntdata`
//! with content type `application/x-fontdata` (see reference packages that open on
//! Windows). This is **not** ODTTF: no GUID XOR; the container is the EOT header
//! followed by font data.
//!
//! We emit EOT version `0x00020002` with **uncompressed** font data (`Flags=0`).
//! Reference PPTX files often set `TTEMBED_TTCOMPRESSED` (MicroType Express); that
//! compressor is proprietary. Uncompressed EOT is valid per the EOT spec.

/// UTF-16LE string with trailing NUL wchar (EOT name fields).
fn utf16_z(s: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len() * 2 + 2);
    for u in s.encode_utf16() {
        out.extend_from_slice(&u.to_le_bytes());
    }
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

fn name_block(s: &str) -> Vec<u8> {
    let b = utf16_z(s);
    let mut out = Vec::with_capacity(4 + b.len());
    out.extend_from_slice(&0u16.to_le_bytes()); // Padding
    out.extend_from_slice(&(b.len() as u16).to_le_bytes());
    out.extend_from_slice(&b);
    out
}

/// Minimal name/metrics fields needed for an EOT header.
#[derive(Clone, Debug)]
pub struct EotFontInfo {
    pub family: String,
    pub style: String,
    pub version: String,
    pub full_name: String,
    pub panose: [u8; 10],
    pub charset: u8,
    pub italic: u8,
    pub weight: u32,
    pub fs_type: u16,
    pub unicode_range: [u32; 4],
    pub code_page_range: [u32; 2],
    pub checksum_adjustment: u32,
}

impl Default for EotFontInfo {
    fn default() -> Self {
        Self {
            family: "Font".into(),
            style: "Regular".into(),
            version: "Version 1.0".into(),
            full_name: "Font".into(),
            panose: [2, 11, 6, 4, 2, 2, 2, 2, 2, 4],
            charset: 0,
            italic: 0,
            weight: 400,
            fs_type: 0,
            unicode_range: [0; 4],
            code_page_range: [0; 2],
            checksum_adjustment: 0,
        }
    }
}

fn name_by_id(face: &ttf_parser::Face<'_>, id: u16) -> Option<String> {
    for n in face.names() {
        if n.name_id == id {
            if let Some(s) = n.to_string() {
                return Some(s);
            }
        }
    }
    None
}

/// Parse basic name/OS/2 fields from a raw TTF/OTF.
pub fn font_info_from_sfnt(font: &[u8], fallback_family: &str) -> EotFontInfo {
    let mut info = EotFontInfo {
        family: fallback_family.to_string(),
        full_name: fallback_family.to_string(),
        ..EotFontInfo::default()
    };
    let Ok(face) = ttf_parser::Face::parse(font, 0) else {
        return info;
    };
    if let Some(s) = name_by_id(&face, ttf_parser::name_id::FAMILY) {
        info.family = s.clone();
        info.full_name = s;
    }
    if let Some(s) = name_by_id(&face, ttf_parser::name_id::SUBFAMILY) {
        info.style = s;
    }
    if let Some(s) = name_by_id(&face, ttf_parser::name_id::FULL_NAME) {
        info.full_name = s;
    }
    if let Some(s) = name_by_id(&face, ttf_parser::name_id::VERSION) {
        info.version = s;
    }
    info.italic = if face.is_italic() { 1 } else { 0 };
    if let Some(os2) = face.tables().os2 {
        info.weight = os2.weight().to_number() as u32;
        // OS/2 layout: version(2) avgCharWidth(2) weightClass(2) widthClass(2) fsType(2)
        // panose starts at offset 32 (10 bytes). Prefer installable fsType=0 for PPT.
        let raw = {
            // ttf-parser keeps OS/2 bytes private; re-parse from face tables via raw font
            // is done below when available. Use weight only here.
            None::<[u8; 10]>
        };
        let _ = raw;
    }
    // Prefer installable embedding for PPT packaging regardless of source fsType.
    info.fs_type = 0;
    // Best-effort panose from raw OS/2 table bytes if present in SFNT.
    if let Some(p) = read_os2_panose(font) {
        info.panose = p;
    }
    info
}

/// Read 10-byte panose from the OS/2 table in a SFNT font (offset 32).
fn read_os2_panose(font: &[u8]) -> Option<[u8; 10]> {
    if font.len() < 12 {
        return None;
    }
    let ntables = u16::from_be_bytes(font[4..6].try_into().ok()?) as usize;
    let mut off = 12usize;
    for _ in 0..ntables {
        if off + 16 > font.len() {
            break;
        }
        let tag = &font[off..off + 4];
        let to = u32::from_be_bytes(font[off + 8..off + 12].try_into().ok()?) as usize;
        let tl = u32::from_be_bytes(font[off + 12..off + 16].try_into().ok()?) as usize;
        if tag == b"OS/2" && to + 42 <= font.len() && tl >= 42 {
            let mut p = [0u8; 10];
            p.copy_from_slice(&font[to + 32..to + 42]);
            return Some(p);
        }
        off += 16;
    }
    None
}

/// Build an EOT 0x00020002 payload wrapping `font_data` (raw TTF/OTF bytes).
pub fn to_eot(font_data: &[u8], info: &EotFontInfo) -> Vec<u8> {
    let mut parts: Vec<u8> = Vec::with_capacity(256 + font_data.len());
    parts.extend_from_slice(&0u32.to_le_bytes()); // EotSize placeholder
    parts.extend_from_slice(&(font_data.len() as u32).to_le_bytes());
    parts.extend_from_slice(&0x0002_0002u32.to_le_bytes()); // Version
    parts.extend_from_slice(&0u32.to_le_bytes()); // Flags: uncompressed
    parts.extend_from_slice(&info.panose);
    parts.push(info.charset);
    parts.push(info.italic);
    parts.extend_from_slice(&info.weight.to_le_bytes());
    parts.extend_from_slice(&info.fs_type.to_le_bytes());
    parts.extend_from_slice(&0x504Cu16.to_le_bytes()); // Magic 'LP'
    for ur in info.unicode_range {
        parts.extend_from_slice(&ur.to_le_bytes());
    }
    for cpr in info.code_page_range {
        parts.extend_from_slice(&cpr.to_le_bytes());
    }
    parts.extend_from_slice(&info.checksum_adjustment.to_le_bytes());
    parts.extend_from_slice(&[0u8; 16]); // Reserved1..4
    parts.extend_from_slice(&name_block(&info.family));
    parts.extend_from_slice(&name_block(&info.style));
    parts.extend_from_slice(&name_block(&info.version));
    parts.extend_from_slice(&name_block(&info.full_name));
    parts.extend_from_slice(&0u16.to_le_bytes()); // RootStringSize
    // EOT 2.2 extras
    parts.extend_from_slice(&0u32.to_le_bytes()); // RootStringCheckSum
    parts.extend_from_slice(&0u32.to_le_bytes()); // EUDCCodePage
    parts.extend_from_slice(&0u16.to_le_bytes()); // Padding5
    parts.extend_from_slice(&0u16.to_le_bytes()); // SignatureSize
    parts.extend_from_slice(&0u32.to_le_bytes()); // EUDCFlags
    parts.extend_from_slice(&0u32.to_le_bytes()); // EUDCFontSize
    parts.extend_from_slice(font_data);

    let total = parts.len() as u32;
    parts[0..4].copy_from_slice(&total.to_le_bytes());
    parts
}

/// Wrap SFNT bytes as EOT, forcing `typeface` as the family name and `charset`.
pub fn sfnt_to_eot(font_data: &[u8], typeface: &str, charset: u8) -> Vec<u8> {
    let mut info = font_info_from_sfnt(font_data, typeface);
    info.family = typeface.to_string();
    if info.full_name.is_empty() {
        info.full_name = typeface.to_string();
    }
    info.charset = charset;
    to_eot(font_data, &info)
}

/// Hex panose string for `p:font/@panose`.
pub fn panose_hex(info: &EotFontInfo) -> String {
    info.panose.iter().map(|b| format!("{b:02X}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eot_header_magic_and_size() {
        let data = b"\x00\x01\x00\x00FAKE_FONT_BYTES______________";
        let eot = sfnt_to_eot(data, "TestFace", 0);
        assert!(eot.len() > data.len());
        let eot_size = u32::from_le_bytes(eot[0..4].try_into().unwrap());
        assert_eq!(eot_size as usize, eot.len());
        let font_data_size = u32::from_le_bytes(eot[4..8].try_into().unwrap());
        assert_eq!(font_data_size as usize, data.len());
        let ver = u32::from_le_bytes(eot[8..12].try_into().unwrap());
        assert_eq!(ver, 0x0002_0002);
        let magic = u16::from_le_bytes(eot[34..36].try_into().unwrap());
        assert_eq!(magic, 0x504C);
        assert_eq!(&eot[eot.len() - data.len()..], data);
    }
}
