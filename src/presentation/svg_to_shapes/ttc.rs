//! Extract a single face from a TrueType/OpenType Collection (`.ttc` / `.otc`).
//!
//! Builds a standalone SFNT font (TTF or CFF/`OTTO`) by copying the face's
//! table directory and table blobs. Enough for ODTTF embedding of Noto CJK SC.

/// Extract face `index` from a TTC/OTC byte buffer into a single-font SFNT file.
pub fn extract_face(data: &[u8], index: u32) -> Option<Vec<u8>> {
    if data.len() < 12 {
        return None;
    }
    // Single font already
    if &data[0..4] == b"\x00\x01\x00\x00" || &data[0..4] == b"OTTO" || &data[0..4] == b"true" {
        return Some(data.to_vec());
    }
    if &data[0..4] != b"ttcf" {
        return None;
    }
    let num_fonts = u32::from_be_bytes(data[8..12].try_into().ok()?);
    if index >= num_fonts {
        return None;
    }
    // Offset table positions start at byte 12 (for header version 1.0/2.0 both)
    // version is at 4..8; for 2.0 there are DSIG fields after offsets, but offsets
    // still begin at 12.
    let off_pos = 12 + (index as usize) * 4;
    if off_pos + 4 > data.len() {
        return None;
    }
    let face_offset = u32::from_be_bytes(data[off_pos..off_pos + 4].try_into().ok()?) as usize;
    if face_offset + 12 > data.len() {
        return None;
    }
    let sfnt_tag = &data[face_offset..face_offset + 4];
    let num_tables =
        u16::from_be_bytes(data[face_offset + 4..face_offset + 6].try_into().ok()?) as usize;
    let search_range = &data[face_offset + 6..face_offset + 8];
    let entry_selector = &data[face_offset + 8..face_offset + 10];
    let range_shift = &data[face_offset + 10..face_offset + 12];

    // Table records: 16 bytes each
    let records_start = face_offset + 12;
    let records_end = records_start + num_tables * 16;
    if records_end > data.len() {
        return None;
    }

    #[derive(Clone)]
    struct Rec {
        tag: [u8; 4],
        checksum: [u8; 4],
        offset: u32,
        length: u32,
    }
    let mut recs = Vec::with_capacity(num_tables);
    for i in 0..num_tables {
        let o = records_start + i * 16;
        let tag = data[o..o + 4].try_into().ok()?;
        let checksum = data[o + 4..o + 8].try_into().ok()?;
        let offset = u32::from_be_bytes(data[o + 8..o + 12].try_into().ok()?);
        let length = u32::from_be_bytes(data[o + 12..o + 16].try_into().ok()?);
        if offset as usize + length as usize > data.len() {
            return None;
        }
        recs.push(Rec {
            tag,
            checksum,
            offset,
            length,
        });
    }

    // Layout: header (12) + records (16*n) + tables (4-byte aligned)
    let header_size = 12 + num_tables * 16;
    let mut out = Vec::with_capacity(data.len() / num_fonts.max(1) as usize);
    out.extend_from_slice(sfnt_tag);
    out.extend_from_slice(&(num_tables as u16).to_be_bytes());
    out.extend_from_slice(search_range);
    out.extend_from_slice(entry_selector);
    out.extend_from_slice(range_shift);
    // placeholder records
    out.resize(header_size, 0);

    let mut table_offset = header_size as u32;
    for (i, rec) in recs.iter().enumerate() {
        // align to 4 bytes
        while table_offset % 4 != 0 {
            out.push(0);
            table_offset += 1;
        }
        let start = rec.offset as usize;
        let end = start + rec.length as usize;
        out.extend_from_slice(&data[start..end]);
        // pad
        let pad = (4 - (rec.length % 4)) % 4;
        for _ in 0..pad {
            out.push(0);
        }

        let rec_pos = 12 + i * 16;
        out[rec_pos..rec_pos + 4].copy_from_slice(&rec.tag);
        out[rec_pos + 4..rec_pos + 8].copy_from_slice(&rec.checksum);
        out[rec_pos + 8..rec_pos + 12].copy_from_slice(&table_offset.to_be_bytes());
        out[rec_pos + 12..rec_pos + 16].copy_from_slice(&rec.length.to_be_bytes());

        table_offset += rec.length + pad;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_noto_sc_if_present() {
        let path = "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc";
        let Ok(data) = std::fs::read(path) else {
            return;
        };
        // face 2 is often SC
        let face = extract_face(&data, 2).expect("extract");
        assert!(face.len() > 1000);
        assert!(&face[0..4] == b"OTTO" || &face[0..4] == b"\x00\x01\x00\x00");
        // parseable by ttf-parser
        ttf_parser::Face::parse(&face, 0).expect("parse extracted");
    }
}
