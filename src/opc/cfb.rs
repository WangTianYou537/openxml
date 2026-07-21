//! Compound File Binary (CFB / OLE2) structure reader for `vbaProject.bin`.
//!
//! Parses the CFB header and directory to list streams/storages. This is **not**
//! a VBA bytecode interpreter — it only inventories the binary container so
//! callers can inspect `vbaProject.bin` layout (e.g. `VBA/`, `PROJECT`,
//! `dir`, module streams).

use std::io::{self, Cursor, Read, Seek, SeekFrom};

/// A directory entry in a CFB file.
#[derive(Debug, Clone)]
pub struct CfbEntry {
    pub name: String,
    pub object_type: CfbObjectType,
    pub start_sector: u32,
    pub stream_size: u64,
    pub child_id: u32,
    pub left_id: u32,
    pub right_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfbObjectType {
    Unknown = 0,
    Storage = 1,
    Stream = 2,
    RootStorage = 5,
}

impl CfbObjectType {
    fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Storage,
            2 => Self::Stream,
            5 => Self::RootStorage,
            _ => Self::Unknown,
        }
    }
}

/// Parsed CFB container.
#[derive(Debug, Clone)]
pub struct CfbFile {
    pub sector_size: usize,
    pub mini_sector_size: usize,
    pub entries: Vec<CfbEntry>,
}

const HEADER_SIG: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
const NOSTREAM: u32 = 0xFFFFFFFF;

impl CfbFile {
    /// Parse a CFB container from bytes.
    pub fn parse(data: &[u8]) -> io::Result<Self> {
        if data.len() < 512 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "CFB too small for header",
            ));
        }
        if data[0..8] != HEADER_SIG {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "not a CFB/OLE compound file (bad signature)",
            ));
        }
        let sector_shift = u16::from_le_bytes([data[30], data[31]]) as usize;
        let mini_sector_shift = u16::from_le_bytes([data[32], data[33]]) as usize;
        let sector_size = 1usize << sector_shift;
        let mini_sector_size = 1usize << mini_sector_shift;
        let dir_first_sector = u32::from_le_bytes(data[48..52].try_into().unwrap());
        let fat_sectors = u32::from_le_bytes(data[44..48].try_into().unwrap()) as usize;

        // Build FAT from first 109 DIFAT entries in header (enough for typical vbaProject)
        let mut fat: Vec<u32> = Vec::new();
        let difat_count = fat_sectors.min(109);
        for i in 0..difat_count {
            let off = 76 + i * 4;
            if off + 4 > data.len() {
                break;
            }
            let sec = u32::from_le_bytes(data[off..off + 4].try_into().unwrap());
            if sec == NOSTREAM {
                break;
            }
            let start = 512 + (sec as usize) * sector_size;
            let end = start + sector_size;
            if end > data.len() {
                break;
            }
            for j in 0..(sector_size / 4) {
                let o = start + j * 4;
                fat.push(u32::from_le_bytes(data[o..o + 4].try_into().unwrap()));
            }
        }

        // Read directory chain
        let mut dir_bytes = Vec::new();
        let mut sector = dir_first_sector;
        let mut guard = 0;
        while sector != NOSTREAM && sector < 0xFFFF_FFFA && guard < 4096 {
            guard += 1;
            let start = 512 + (sector as usize) * sector_size;
            let end = start + sector_size;
            if end > data.len() {
                break;
            }
            dir_bytes.extend_from_slice(&data[start..end]);
            sector = fat.get(sector as usize).copied().unwrap_or(NOSTREAM);
        }

        let mut entries = Vec::new();
        let mut offset = 0;
        while offset + 128 <= dir_bytes.len() {
            let entry = &dir_bytes[offset..offset + 128];
            let name_len = u16::from_le_bytes([entry[64], entry[65]]) as usize;
            let obj_type = CfbObjectType::from_u8(entry[66]);
            // name is UTF-16LE, name_len includes null terminator bytes
            let name_byte_len = name_len.saturating_sub(2).min(64);
            let mut name = String::new();
            if name_byte_len >= 2 {
                let raw = &entry[0..name_byte_len];
                for chunk in raw.chunks_exact(2) {
                    let cu = u16::from_le_bytes([chunk[0], chunk[1]]);
                    if cu == 0 {
                        break;
                    }
                    if let Some(ch) = char::from_u32(cu as u32) {
                        name.push(ch);
                    }
                }
            }
            let left_id = u32::from_le_bytes(entry[68..72].try_into().unwrap());
            let right_id = u32::from_le_bytes(entry[72..76].try_into().unwrap());
            let child_id = u32::from_le_bytes(entry[76..80].try_into().unwrap());
            let start_sector = u32::from_le_bytes(entry[116..120].try_into().unwrap());
            let stream_size = u64::from_le_bytes(entry[120..128].try_into().unwrap());

            // Skip unused entries (type 0 and empty name)
            if obj_type != CfbObjectType::Unknown || !name.is_empty() {
                entries.push(CfbEntry {
                    name,
                    object_type: obj_type,
                    start_sector,
                    stream_size,
                    child_id,
                    left_id,
                    right_id,
                });
            }
            offset += 128;
        }

        Ok(Self {
            sector_size,
            mini_sector_size,
            entries,
        })
    }

    /// Stream/storage entry names (non-empty).
    pub fn entry_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| !e.name.is_empty())
            .map(|e| e.name.as_str())
            .collect()
    }

    /// Stream entries only.
    pub fn streams(&self) -> impl Iterator<Item = &CfbEntry> {
        self.entries
            .iter()
            .filter(|e| e.object_type == CfbObjectType::Stream)
    }

    /// Whether this looks like a VBA project CFB (has VBA storage or PROJECT stream).
    pub fn is_vba_project(&self) -> bool {
        self.entries.iter().any(|e| {
            let n = e.name.to_ascii_uppercase();
            n == "VBA" || n == "PROJECT" || n.starts_with("VBA/")
        })
    }
}

/// Parse `vbaProject.bin` bytes as CFB and return entry inventory.
pub fn inspect_vba_project(data: &[u8]) -> io::Result<CfbFile> {
    CfbFile::parse(data)
}

/// Minimal CFB builder for tests (root + one stream).
#[cfg(test)]
fn build_minimal_cfb_for_test() -> Vec<u8> {
    // Not a full writer — tests use a handcrafted minimal header+dir when needed.
    // For unit tests we craft a valid-enough header with empty directory chain.
    let mut data = vec![0u8; 512];
    data[0..8].copy_from_slice(&HEADER_SIG);
    // little-endian version 3, sector shift 9 (512), mini shift 6 (64)
    data[24] = 0x3E;
    data[25] = 0x00;
    data[26] = 0x03;
    data[27] = 0x00;
    data[28] = 0xFE;
    data[29] = 0xFF;
    data[30] = 9; // sector shift
    data[31] = 0;
    data[32] = 6; // mini sector shift
    data[33] = 0;
    // first dir sector = 0, fat sectors = 1, first fat = 0 in difat[0]
    data[44] = 1; // number of FAT sectors
    // first directory sector = 1 (sector 0 is FAT)
    data[48..52].copy_from_slice(&1u32.to_le_bytes());
    data[76] = 0; // difat[0] = sector 0 (FAT)
    // sector 0 = FAT, sector 1 = directory
    data.resize(512 + 512 * 2, 0);
    // FAT at sector 0: entry0 = FATSECT (0xFFFFFFFD), entry1 = ENDOFCHAIN (0xFFFFFFFE)
    let fat_off = 512;
    data[fat_off..fat_off + 4].copy_from_slice(&0xFFFFFFFDu32.to_le_bytes());
    data[fat_off + 4..fat_off + 8].copy_from_slice(&0xFFFFFFFEu32.to_le_bytes());
    // directory at sector 1: one root entry named "Root Entry"
    let dir_off = 512 + 512;
    // name UTF-16LE "Root Entry\0"
    let name = "Root Entry\0"
        .encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect::<Vec<_>>();
    data[dir_off..dir_off + name.len()].copy_from_slice(&name);
    let name_len = name.len() as u16;
    data[dir_off + 64..dir_off + 66].copy_from_slice(&name_len.to_le_bytes());
    data[dir_off + 66] = 5; // root storage
    // left/right/child = NOSTREAM
    data[dir_off + 68..dir_off + 72].copy_from_slice(&NOSTREAM.to_le_bytes());
    data[dir_off + 72..dir_off + 76].copy_from_slice(&NOSTREAM.to_le_bytes());
    data[dir_off + 76..dir_off + 80].copy_from_slice(&NOSTREAM.to_le_bytes());
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_non_cfb() {
        assert!(CfbFile::parse(b"not a cfb file!!!!").is_err());
    }

    #[test]
    fn parse_minimal_header() {
        let data = build_minimal_cfb_for_test();
        let cfb = CfbFile::parse(&data).expect("parse");
        assert_eq!(cfb.sector_size, 512);
        assert!(
            cfb.entry_names().iter().any(|n| n.contains("Root")),
            "entries={:?}",
            cfb.entry_names()
        );
    }

    #[test]
    fn inspect_vba_detects_project_stream() {
        // Build minimal CFB with a stream named PROJECT as second directory entry
        let mut data = build_minimal_cfb_for_test();
        let dir_off = 512 + 512 + 128;
        if data.len() < dir_off + 128 {
            data.resize(dir_off + 128, 0);
        }
        let name = "PROJECT\0"
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<_>>();
        data[dir_off..dir_off + name.len()].copy_from_slice(&name);
        let name_len = name.len() as u16;
        data[dir_off + 64..dir_off + 66].copy_from_slice(&name_len.to_le_bytes());
        data[dir_off + 66] = 2; // stream
        data[dir_off + 68..dir_off + 72].copy_from_slice(&NOSTREAM.to_le_bytes());
        data[dir_off + 72..dir_off + 76].copy_from_slice(&NOSTREAM.to_le_bytes());
        data[dir_off + 76..dir_off + 80].copy_from_slice(&NOSTREAM.to_le_bytes());
        let cfb = inspect_vba_project(&data).unwrap();
        assert!(
            cfb.is_vba_project(),
            "names={:?}",
            cfb.entry_names()
        );
        assert!(cfb.streams().any(|s| s.name == "PROJECT"));
    }
}
