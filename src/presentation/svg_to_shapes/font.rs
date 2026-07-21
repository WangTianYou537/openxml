//! Font loading and precise text measurement via `ttf-parser`.
//!
//! Resolves CSS `font-family` lists against system fonts (Noto CJK, DejaVu,
//! Liberation as Helvetica/Arial stand-ins) and measures advance widths /
//! ascender/descender for pixel-accurate text boxes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Measured text layout in SVG user units (at the requested `font_size`).
#[derive(Debug, Clone)]
pub struct TextMetrics {
    pub width: f64,
    pub height: f64,
    pub ascent: f64,
    pub descent: f64, // positive downward distance below baseline
}

struct FontFace {
    path: PathBuf,
    /// Face index inside TTC.
    index: u32,
    /// Cached file bytes (TTC/TTF).
    data: Vec<u8>,
}

pub struct FontDb {
    /// family-lower → style-key → face
    faces: HashMap<String, HashMap<String, FontFace>>,
    /// family aliases (helvetica → liberation sans, etc.)
    aliases: HashMap<String, String>,
}

impl FontDb {
    pub fn global() -> &'static FontDb {
        static DB: OnceLock<FontDb> = OnceLock::new();
        DB.get_or_init(FontDb::load_system)
    }

    fn load_system() -> Self {
        let mut faces: HashMap<String, HashMap<String, FontFace>> = HashMap::new();
        let mut aliases = HashMap::new();

        // Known good paths on this environment + common Linux locations.
        let candidates: &[(&str, &str, u32, &str)] = &[
            // family, style, ttc_index, path
            (
                "noto sans cjk sc",
                "regular",
                0, // SC is often face 0 or 2 depending on build — we'll probe
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            ),
            (
                "noto sans cjk sc",
                "bold",
                0,
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
            ),
            (
                "noto sans cjk sc",
                "medium",
                0,
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Medium.ttc",
            ),
            (
                "dejavu sans",
                "regular",
                0,
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ),
            (
                "dejavu sans",
                "bold",
                0,
                "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
            ),
            (
                "liberation sans",
                "regular",
                0,
                "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            ),
            (
                "liberation sans",
                "bold",
                0,
                "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
            ),
            (
                "liberation sans",
                "regular",
                0,
                "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
            ),
        ];

        for (family, style, _idx, path) in candidates {
            let path = Path::new(path);
            if !path.exists() {
                continue;
            }
            if let Ok(data) = std::fs::read(path) {
                // For TTC, pick the face whose family name matches best.
                let face_index = pick_face_index(&data, family);
                faces
                    .entry((*family).into())
                    .or_default()
                    .insert(
                        (*style).into(),
                        FontFace {
                            path: path.to_path_buf(),
                            index: face_index,
                            data,
                        },
                    );
            }
        }

        // CSS generic / common aliases → real families we have
        for (from, to) in [
            ("helvetica", "liberation sans"),
            ("helvetica neue", "liberation sans"),
            ("arial", "liberation sans"),
            ("sans-serif", "liberation sans"),
            ("serif", "dejavu sans"),
            ("monospace", "dejavu sans"),
            ("pingfang sc", "noto sans cjk sc"),
            ("microsoft yahei", "noto sans cjk sc"),
            ("微软雅黑", "noto sans cjk sc"),
            ("noto sans cjk", "noto sans cjk sc"),
            ("source han sans", "noto sans cjk sc"),
            ("source han sans sc", "noto sans cjk sc"),
        ] {
            aliases.insert(from.into(), to.into());
        }

        Self { faces, aliases }
    }

    /// Measure `text` at `font_size` (SVG user units / px) with the given CSS font-family list.
    pub fn measure(&self, text: &str, font_size: f64, font_family: &str, bold: bool) -> TextMetrics {
        let style = if bold { "bold" } else { "regular" };
        let families = parse_font_family_list(font_family);
        let face = self
            .resolve(&families, style)
            .or_else(|| self.resolve(&families, "regular"))
            .or_else(|| self.faces.get("liberation sans").and_then(|m| m.get("regular")))
            .or_else(|| self.faces.get("dejavu sans").and_then(|m| m.get("regular")))
            .or_else(|| self.faces.get("noto sans cjk sc").and_then(|m| m.get("regular")));

        let Some(face) = face else {
            // Fallback heuristic: CJK ~1em, Latin ~0.55em
            let w: f64 = text
                .chars()
                .map(|c| {
                    if c <= '\u{00ff}' {
                        font_size * 0.55
                    } else {
                        font_size
                    }
                })
                .sum();
            return TextMetrics {
                width: w,
                height: font_size * 1.2,
                ascent: font_size * 0.8,
                descent: font_size * 0.2,
            };
        };

        let face_ref = ttf_parser::Face::parse(&face.data, face.index).ok();
        let Some(face_ref) = face_ref else {
            return TextMetrics {
                width: font_size * text.chars().count() as f64 * 0.6,
                height: font_size * 1.2,
                ascent: font_size * 0.8,
                descent: font_size * 0.2,
            };
        };

        let units = face_ref.units_per_em() as f64;
        let scale = font_size / units;
        let mut width = 0.0_f64;
        for ch in text.chars() {
            if let Some(gid) = face_ref.glyph_index(ch) {
                let adv = face_ref.glyph_hor_advance(gid).unwrap_or(0) as f64;
                width += adv * scale;
            } else if let Some(gid) = face_ref.glyph_index('\u{fffd}') {
                width += face_ref.glyph_hor_advance(gid).unwrap_or(0) as f64 * scale;
            } else {
                width += font_size * 0.5;
            }
        }

        let ascent = face_ref.ascender() as f64 * scale;
        let descent = (-face_ref.descender() as f64) * scale; // positive
        let height = ascent + descent;
        TextMetrics {
            width,
            height: height.max(font_size),
            ascent: ascent.max(font_size * 0.7),
            descent: descent.max(font_size * 0.15),
        }
    }

    /// DrawingML typeface name to embed in `a:latin` / `a:ea`.
    pub fn typeface_for(&self, font_family: &str, for_east_asian: bool) -> String {
        let families = parse_font_family_list(font_family);
        for f in &families {
            let key = f.to_ascii_lowercase();
            let resolved = self.aliases.get(&key).cloned().unwrap_or(key);
            if for_east_asian
                && (resolved.contains("cjk")
                    || resolved.contains("noto")
                    || resolved.contains("yahei")
                    || resolved.contains("pingfang"))
            {
                return "Noto Sans CJK SC".into();
            }
            if resolved.contains("liberation") || resolved.contains("dejavu") {
                return if for_east_asian {
                    "Noto Sans CJK SC".into()
                } else {
                    "Arial".into() // PowerPoint maps Arial well; Liberation is metric-compatible
                };
            }
        }
        if for_east_asian {
            "Noto Sans CJK SC".into()
        } else {
            "Arial".into()
        }
    }

    /// Resolve a concrete face file for embedding. Returns (typeface label, path, bytes, bold).
    pub fn face_for_embed(
        &self,
        font_family: &str,
        bold: bool,
    ) -> Option<(String, PathBuf, Vec<u8>, bool)> {
        let style = if bold { "bold" } else { "regular" };
        let families = parse_font_family_list(font_family);
        let face = self
            .resolve(&families, style)
            .or_else(|| self.resolve(&families, "regular"))
            .or_else(|| self.faces.get("liberation sans").and_then(|m| m.get("regular")))
            .or_else(|| self.faces.get("noto sans cjk sc").and_then(|m| m.get("regular")))?;
        let typeface = self.typeface_for(font_family, false);
        // Prefer CJK typeface name when the resolved face is CJK
        let typeface = if face.path.to_string_lossy().to_ascii_lowercase().contains("noto")
            || face.path.to_string_lossy().to_ascii_lowercase().contains("cjk")
        {
            "Noto Sans CJK SC".into()
        } else {
            typeface
        };
        Some((typeface, face.path.clone(), face.data.clone(), bold))
    }

    fn resolve(&self, families: &[String], style: &str) -> Option<&FontFace> {
        for f in families {
            let key = f.to_ascii_lowercase();
            let resolved = self.aliases.get(&key).cloned().unwrap_or(key);
            if let Some(styles) = self.faces.get(&resolved) {
                if let Some(face) = styles.get(style) {
                    return Some(face);
                }
                // medium as bold-ish fallback
                if style == "bold" {
                    if let Some(face) = styles.get("medium") {
                        return Some(face);
                    }
                }
                if let Some(face) = styles.get("regular") {
                    return Some(face);
                }
            }
        }
        None
    }
}

fn pick_face_index(data: &[u8], want_family: &str) -> u32 {
    let n = ttf_parser::fonts_in_collection(data).unwrap_or(1);
    let want = want_family.to_ascii_lowercase();
    for i in 0..n {
        if let Ok(face) = ttf_parser::Face::parse(data, i) {
            if let Some(name) = face_family_name(&face) {
                let n = name.to_ascii_lowercase();
                if n.contains("sc") && want.contains("sc") {
                    return i;
                }
                if n.contains(&want) || want.contains(&n) {
                    return i;
                }
            }
        }
    }
    // NotoSansCJK TTC face order is often JP=0, KR=1, SC=2, TC=3
    if want.contains("sc") && n > 2 {
        return 2;
    }
    0
}

fn face_family_name(face: &ttf_parser::Face<'_>) -> Option<String> {
    // Prefer Typographic Family, then Font Family
    use ttf_parser::name_id;
    let mut best = None;
    for name in face.names() {
        if !name.is_unicode() {
            continue;
        }
        if name.name_id == name_id::TYPOGRAPHIC_FAMILY || name.name_id == name_id::FAMILY {
            if let Some(s) = name.to_string() {
                best = Some(s);
                if name.name_id == name_id::TYPOGRAPHIC_FAMILY {
                    break;
                }
            }
        }
    }
    best
}

fn parse_font_family_list(s: &str) -> Vec<String> {
    s.split(',')
        .map(|p| p.trim().trim_matches(|c| c == '"' || c == '\'').to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_ascii_positive() {
        let db = FontDb::global();
        let m = db.measure("Hello", 16.0, "Helvetica, Arial, sans-serif", false);
        assert!(m.width > 20.0 && m.width < 80.0, "width={}", m.width);
        assert!(m.ascent > 0.0);
    }

    #[test]
    fn measure_cjk() {
        let db = FontDb::global();
        let m = db.measure("中文", 16.0, "PingFang SC, Noto Sans CJK SC, sans-serif", true);
        // Two full-width glyphs ≈ 2 * font_size
        assert!(m.width > 20.0, "width={}", m.width);
    }
}
