//! Office Open XML embedded font obfuscation (ODTTF).
//!
//! Per ECMA-376 / [MS-OFFCRYPTO]: obfuscate the first 32 bytes of a font file by
//! XORing with the 16-byte GUID of the font part, repeated twice
//! (`key[i % 16]` for `i in 0..32`). De-obfuscation is the same operation.
//!
//! See also: <https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/>

/// Generate a GUID for embedding. Uses OS randomness when available.
pub fn new_font_guid() -> [u8; 16] {
    let mut g = [0u8; 16];
    // Prefer getrandom via /dev/urandom for unique high bytes (older PRNG left
    // g[8..] as a fixed 8899-AABBCCDDEEFF pattern which is valid but suspicious).
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        use std::io::Read;
        let _ = f.read_exact(&mut g);
    } else {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let ptr = &nanos as *const _ as usize as u128;
        let mixed = nanos
            ^ (ptr << 17)
            ^ (ptr.rotate_left(33))
            ^ 0xA5A5_5A5A_C3C3_3C3C_1234_5678_9ABC_DEF0;
        for i in 0..16 {
            g[i] = (mixed >> (i * 8)) as u8;
        }
    }
    // RFC 4122 variant/version bits (version 4)
    g[6] = (g[6] & 0x0f) | 0x40;
    g[8] = (g[8] & 0x3f) | 0x80;
    g
}

/// Format GUID as `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` uppercase (Office style).
pub fn guid_string(g: &[u8; 16]) -> String {
    format!(
        "{{{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        g[3], g[2], g[1], g[0], // little-endian first three groups per MS GUID layout
        g[5], g[4],
        g[7], g[6],
        g[8], g[9],
        g[10], g[11], g[12], g[13], g[14], g[15]
    )
}

/// Obfuscate (or de-obfuscate) font bytes in place for the first 32 bytes using GUID key.
pub fn obfuscate_font(data: &mut [u8], guid: &[u8; 16]) {
    let n = data.len().min(32);
    for i in 0..n {
        data[i] ^= guid[i % 16];
    }
}

/// Produce an ODTTF byte buffer from raw TTF/OTF bytes.
pub fn to_odttf(font_bytes: &[u8], guid: &[u8; 16]) -> Vec<u8> {
    let mut out = font_bytes.to_vec();
    obfuscate_font(&mut out, guid);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_obfuscation() {
        let mut data = vec![0u8; 64];
        for (i, b) in data.iter_mut().enumerate() {
            *b = i as u8;
        }
        let original = data.clone();
        let g = new_font_guid();
        obfuscate_font(&mut data, &g);
        assert_ne!(&data[..32], &original[..32]);
        assert_eq!(&data[32..], &original[32..]);
        obfuscate_font(&mut data, &g);
        assert_eq!(data, original);
    }

    #[test]
    fn guid_string_braced() {
        let g = [0u8; 16];
        let s = guid_string(&g);
        assert!(s.starts_with('{') && s.ends_with('}'));
        assert_eq!(s.len(), 38);
    }

    #[test]
    fn guid_entropy_high_bytes_not_constant_pattern() {
        let a = new_font_guid();
        let b = new_font_guid();
        // With /dev/urandom, successive GUIDs must differ.
        assert_ne!(a, b);
        // High bytes must not be the old fixed 88 99 AA BB CC DD EE FF pattern.
        assert!(!(a[8] == 0x88 && a[9] == 0x99 && a[10] == 0xAA && a[11] == 0xBB));
    }
}
