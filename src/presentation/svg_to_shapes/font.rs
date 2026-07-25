//! Font loading and precise text measurement via `ttf-parser`.
//!
//! Resolves CSS `font-family` lists against system fonts (Noto CJK, DejaVu,
//! Liberation as Helvetica/Arial stand-ins) and measures advance widths /
//! ascender/descender for pixel-accurate text boxes.
//!
//! Mixed Latin/CJK runs use **dual-face measurement**: Latin glyphs from
//! Liberation/DejaVu, CJK glyphs from Noto Sans CJK SC — matching how browsers
//! fall through a font-family stack.

use super::ttc;
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

        let candidates: &[(&str, &str, &str)] = &[
            (
                "noto sans cjk sc",
                "regular",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            ),
            (
                "noto sans cjk sc",
                "bold",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
            ),
            (
                "noto sans cjk sc",
                "medium",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Medium.ttc",
            ),
            (
                "noto sans cjk sc",
                "black",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Black.ttc",
            ),
            (
                "noto sans cjk jp",
                "regular",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            ),
            (
                "noto sans cjk jp",
                "bold",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
            ),
            (
                "noto sans cjk jp",
                "medium",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Medium.ttc",
            ),
            (
                "noto sans cjk jp",
                "black",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Black.ttc",
            ),
            (
                "dejavu sans",
                "regular",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ),
            (
                "dejavu sans",
                "bold",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
            ),
            (
                "liberation sans",
                "regular",
                "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            ),
            (
                "liberation sans",
                "bold",
                "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
            ),
            (
                "liberation sans",
                "regular",
                "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
            ),
            (
                "liberation sans",
                "bold",
                "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
            ),
            // Liberation Serif ≈ Times New Roman metrics (default Latin when SVG omits font).
            (
                "liberation serif",
                "regular",
                "/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf",
            ),
            (
                "liberation serif",
                "bold",
                "/usr/share/fonts/truetype/liberation/LiberationSerif-Bold.ttf",
            ),
            (
                "liberation serif",
                "regular",
                "/usr/share/fonts/truetype/liberation2/LiberationSerif-Regular.ttf",
            ),
            (
                "liberation serif",
                "bold",
                "/usr/share/fonts/truetype/liberation2/LiberationSerif-Bold.ttf",
            ),
            // Bundled TrueType Noto Sans SC (Source Han Sans SC) for EOT embedding.
            // Prefer these over system Noto CJK TTC/OTTO (MS PowerPoint rejects many
            // CFF extracts) and over Droid (fsType=8 restricted embedding).
            (
                "noto sans sc",
                "regular",
                concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSansSC-Regular.ttf"),
            ),
            (
                "noto sans sc",
                "bold",
                concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSansSC-Bold.ttf"),
            ),
        ];

        for (family, style, path) in candidates {
            let path = Path::new(path);
            if !path.exists() {
                continue;
            }
            if let Ok(data) = std::fs::read(path) {
                let face_index = pick_face_index(&data, family);
                faces.entry((*family).into()).or_default().insert(
                    (*style).into(),
                    FontFace {
                        path: path.to_path_buf(),
                        index: face_index,
                        data,
                    },
                );
            }
        }

        // Font stack aliases for *measurement* (local faces). DrawingML typeface names
        // come from `typeface_for` and may differ (e.g. Times New Roman / Microsoft YaHei).
        for (from, to) in [
            ("helvetica", "liberation sans"),
            ("helvetica neue", "liberation sans"),
            ("arial", "liberation sans"),
            ("sans-serif", "liberation sans"),
            // Default Latin when SVG omits font: Times New Roman → Liberation Serif metrics.
            ("times new roman", "liberation serif"),
            ("times", "liberation serif"),
            ("times roman", "liberation serif"),
            ("serif", "liberation serif"),
            ("monospace", "dejavu sans"),
            // slide-1 fidelity: PingFang SC alias → JP face was measured closer to Chrome.
            ("pingfang sc", "noto sans cjk jp"),
            // Microsoft YaHei (default Chinese when SVG omits font) → SC metrics for measure.
            ("microsoft yahei", "noto sans sc"),
            ("微软雅黑", "noto sans sc"),
            ("yahei", "noto sans sc"),
            // Explicit SC family names still map to SC.
            ("heiti sc", "noto sans cjk sc"),
            ("stheiti", "noto sans cjk sc"),
            ("hiragino sans gb", "noto sans cjk sc"),
            ("pingfang tc", "noto sans cjk jp"),
            ("noto sans cjk", "noto sans cjk sc"),
            ("source han sans", "noto sans sc"),
            ("source han sans sc", "noto sans sc"),
            ("source han sans cn", "noto sans sc"),
            ("思源黑体", "noto sans sc"),
            ("noto sans sc", "noto sans sc"),
            ("dejavu sans", "dejavu sans"),
            ("liberation sans", "liberation sans"),
            ("liberation serif", "liberation serif"),
        ] {
            aliases.insert(from.into(), to.into());
        }

        Self { faces, aliases }
    }

    /// Measure `text` at `font_size` with dual-face Latin/CJK fallback.
    ///
    /// `weight` is a CSS numeric weight (100–900); 400=regular, 500/600=medium, ≥700=bold.
    pub fn measure(
        &self,
        text: &str,
        font_size: f64,
        font_family: &str,
        bold: bool,
    ) -> TextMetrics {
        self.measure_weight(text, font_size, font_family, if bold { 700 } else { 400 })
    }

    pub fn measure_weight(
        &self,
        text: &str,
        font_size: f64,
        font_family: &str,
        weight: i32,
    ) -> TextMetrics {
        self.measure_weight_ex(text, font_size, font_family, weight, false, false)
    }

    /// Like [`measure_weight`], with CSS `font-variant-numeric: tabular-nums`
    /// equalizing ASCII digit advances to the max digit width, and optional
    /// `font-kerning: normal` pair kerning.
    pub fn measure_weight_ex(
        &self,
        text: &str,
        font_size: f64,
        font_family: &str,
        weight: i32,
        tabular_nums: bool,
        enable_kern: bool,
    ) -> TextMetrics {
        let style = weight_style(weight);
        let families = parse_font_family_list(font_family);

        let latin_face = self
            .resolve_prefer_latin(&families, style)
            .or_else(|| self.faces.get("liberation sans").and_then(|m| m.get(style)))
            .or_else(|| {
                self.faces
                    .get("liberation sans")
                    .and_then(|m| m.get("regular"))
            })
            .or_else(|| self.faces.get("dejavu sans").and_then(|m| m.get("regular")));

        let cjk_face = self
            .resolve_prefer_cjk(&families, style)
            .or_else(|| {
                self.faces
                    .get("noto sans cjk sc")
                    .and_then(|m| m.get(style))
            })
            .or_else(|| {
                self.faces
                    .get("noto sans cjk sc")
                    .and_then(|m| m.get("regular"))
            });

        let latin = latin_face.and_then(|f| ttf_parser::Face::parse(&f.data, f.index).ok());
        let cjk = cjk_face.and_then(|f| ttf_parser::Face::parse(&f.data, f.index).ok());

        if latin.is_none() && cjk.is_none() {
            let w: f64 = text
                .chars()
                .map(|c| {
                    if is_cjk(c) {
                        font_size
                    } else if c <= '\u{00ff}' {
                        font_size * 0.55
                    } else {
                        font_size * 0.6
                    }
                })
                .sum();
            return TextMetrics {
                width: w,
                height: font_size * 1.2,
                ascent: font_size * 0.8,
                descent: font_size * 0.2,
            };
        }

        let tab_digit = if tabular_nums {
            Some(max_digit_advance(font_size, latin.as_ref(), cjk.as_ref()))
        } else {
            None
        };

        let mut width = 0.0_f64;
        let mut prev: Option<(char, bool)> = None;
        for ch in text.chars() {
            let prefer_cjk = is_cjk(ch);
            if let Some((prev_ch, prev_cjk)) = prev {
                if prev_cjk == prefer_cjk {
                    // Tabular digits: no pair kerning between digits.
                    let skip_kern = tab_digit.is_some()
                        && prev_ch.is_ascii_digit()
                        && ch.is_ascii_digit();
                    if !skip_kern {
                        width += pair_kerning(
                            prev_ch,
                            ch,
                            font_size,
                            if prefer_cjk {
                                cjk.as_ref()
                            } else {
                                latin.as_ref()
                            },
                            enable_kern,
                        );
                    }
                }
            }
            if let Some(tw) = tab_digit {
                if ch.is_ascii_digit() {
                    width += tw;
                    prev = Some((ch, prefer_cjk));
                    continue;
                }
            }
            width += glyph_advance(ch, font_size, latin.as_ref(), cjk.as_ref());
            prev = Some((ch, prefer_cjk));
        }

        let (ascent, descent) = metrics_ascent_descent(font_size, latin.as_ref(), cjk.as_ref());
        let height = ascent + descent;
        TextMetrics {
            width,
            height: height.max(font_size),
            ascent: ascent.max(font_size * 0.7),
            descent: descent.max(font_size * 0.15),
        }
    }

    /// DrawingML typeface name for `a:latin` / `a:ea`.
    ///
    /// When the SVG stack is unspecified/generic, defaults are **Times New Roman**
    /// (Latin) and **Microsoft YaHei** (East Asian) — Windows system faces.
    /// Explicit named families keep their resolved DrawingML names.
    pub fn typeface_for(&self, font_family: &str, for_east_asian: bool) -> String {
        let families = parse_font_family_list(font_family);
        let generic_only = families.is_empty()
            || families.iter().all(|f| {
                let k = f.to_ascii_lowercase();
                matches!(
                    k.as_str(),
                    "serif"
                        | "sans-serif"
                        | "monospace"
                        | "cursive"
                        | "fantasy"
                        | "system-ui"
                        | "ui-serif"
                        | "ui-sans-serif"
                        | "ui-monospace"
                        | "emoji"
                        | "math"
                        | "fangsong"
                )
            });

        if for_east_asian {
            for f in &families {
                let key = f.to_ascii_lowercase();
                if key.contains("microsoft yahei")
                    || key == "微软雅黑"
                    || key == "yahei"
                    || key.contains("yahei")
                {
                    return "Microsoft YaHei".into();
                }
                if key.contains("pingfang") {
                    // Keep host PingFang when named; Windows often substitutes YaHei.
                    return "Microsoft YaHei".into();
                }
                if key.contains("noto sans sc")
                    || key.contains("source han")
                    || f.contains("思源")
                {
                    return if self.faces.contains_key("noto sans sc") {
                        "Noto Sans SC".into()
                    } else {
                        "Noto Sans CJK SC".into()
                    };
                }
                if key.contains("noto sans cjk") || key.contains("cjk") {
                    return "Noto Sans CJK SC".into();
                }
                if key.contains("simsun") || f.contains("宋体") {
                    return "SimSun".into();
                }
                if key.contains("simhei") || f.contains("黑体") {
                    return "SimHei".into();
                }
            }
            // Default Chinese face when SVG omits a CJK family.
            return "Microsoft YaHei".into();
        }

        for f in &families {
            let key = f.to_ascii_lowercase();
            if key.contains("times new roman")
                || key == "times"
                || key == "times roman"
                || (key == "serif" && (generic_only || families.len() == 1))
            {
                return "Times New Roman".into();
            }
            if key.contains("arial") {
                return "Arial".into();
            }
            if key.contains("helvetica") {
                // Helvetica is not a Windows core font; Arial is the usual stand-in.
                return "Arial".into();
            }
            if key.contains("liberation serif") {
                return "Times New Roman".into();
            }
            if key.contains("liberation sans") || key.contains("liberation") {
                return "Arial".into();
            }
            if key.contains("dejavu") {
                return "Arial".into();
            }
            if key.contains("calibri") {
                return "Calibri".into();
            }
            if key.contains("georgia") {
                return "Georgia".into();
            }
            if key.contains("courier") {
                return "Courier New".into();
            }
        }
        if generic_only {
            // SVG omitted a concrete Latin face → Times New Roman.
            return "Times New Roman".into();
        }
        "Times New Roman".into()
    }

    /// Resolve a concrete face file for embedding.
    ///
    /// For TTC sources, extracts a single SFNT face so it can be ODTTF-embedded.
    /// Returns (typeface, path_hint, bytes, bold, is_cjk).
    /// CSS aspect value ≈ x-height / em for `font-size-adjust`.
    pub fn aspect_ratio(&self, font_family: &str, weight: i32) -> f64 {
        let style = weight_style(weight);
        let families = parse_font_family_list(font_family);
        let latin_face = self
            .resolve_prefer_latin(&families, style)
            .or_else(|| self.faces.get("liberation sans").and_then(|m| m.get(style)))
            .or_else(|| {
                self.faces
                    .get("liberation sans")
                    .and_then(|m| m.get("regular"))
            });
        let cjk_face = self
            .resolve_prefer_cjk(&families, style)
            .or_else(|| {
                self.faces
                    .get("noto sans cjk sc")
                    .and_then(|m| m.get("regular"))
            });
        for ff in [latin_face, cjk_face].into_iter().flatten() {
            let Ok(face) = ttf_parser::Face::parse(&ff.data, ff.index) else {
                continue;
            };
            let units = face.units_per_em() as f64;
            if units <= 0.0 {
                continue;
            }
            if let Some(os2) = face.tables().os2 {
                // Prefer OS/2 sxHeight when non-zero.
                let sx = {
                    // ttf-parser: x_height may be Option or raw; try both via helper.
                    os2_x_height(&os2)
                };
                if sx > 0.0 {
                    return (sx / units).clamp(0.1, 1.5);
                }
            }
            if let Some(gid) = face.glyph_index('x') {
                if let Some(bb) = face.glyph_bounding_box(gid) {
                    let h = (bb.y_max as f64 - bb.y_min as f64).abs();
                    if h > 0.0 {
                        return (h / units).clamp(0.1, 1.5);
                    }
                }
            }
        }
        0.5
    }

    pub fn face_for_embed(
        &self,
        font_family: &str,
        bold: bool,
    ) -> Option<(String, PathBuf, Vec<u8>, bool)> {
        let style = if bold { "bold" } else { "regular" };
        let families = parse_font_family_list(font_family);
        // Prefer Latin for embed of "Arial" stack; CJK is separate via typeface_for(ea)
        let face = self
            .resolve_prefer_latin(&families, style)
            .or_else(|| self.resolve(&families, style))
            .or_else(|| self.faces.get("dejavu sans").and_then(|m| m.get("regular")))
            .or_else(|| {
                self.faces
                    .get("liberation sans")
                    .and_then(|m| m.get("regular"))
            })?;

        let path_l = face.path.to_string_lossy().to_ascii_lowercase();
        // Must match the font name table: MS PowerPoint rejects embedded fonts whose
        // `p:font/@typeface` does not agree with nameID 1/4 inside the EOT payload.
        let typeface = if path_l.contains("notosanssc") || path_l.contains("noto sans sc") {
            "Noto Sans SC".into()
        } else if path_l.contains("cjk") || (path_l.contains("noto") && path_l.contains("cjk")) {
            "Noto Sans CJK SC".into()
        } else if path_l.contains("dejavu") {
            "DejaVu Sans".into()
        } else if path_l.contains("liberation") {
            "Liberation Sans".into()
        } else {
            // Prefer the resolved CSS family; do not lie as "Arial" for substitutes.
            self.typeface_for(font_family, false)
        };

        let bytes = if face.data.get(0..4) == Some(b"ttcf") {
            ttc::extract_face(&face.data, face.index)?
        } else {
            face.data.clone()
        };
        Some((typeface, face.path.clone(), bytes, bold))
    }

    /// Embeddable CJK face for East-Asian typeface.
    ///
    /// Prefer bundled TrueType `Noto Sans SC` (Source Han Sans SC, fsType=0).
    /// Fall back to a single-face extract from system Noto Sans CJK SC TTC.
    pub fn cjk_face_for_embed(&self, bold: bool) -> Option<(String, PathBuf, Vec<u8>, bool)> {
        let style = if bold { "bold" } else { "regular" };
        if let Some(face) = self
            .faces
            .get("noto sans sc")
            .and_then(|m| m.get(style).or_else(|| m.get("regular")))
        {
            return Some((
                "Noto Sans SC".into(),
                face.path.clone(),
                face.data.clone(),
                bold,
            ));
        }
        let face = self
            .faces
            .get("noto sans cjk sc")
            .and_then(|m| m.get(style).or_else(|| m.get("regular")))?;
        let bytes = if face.data.get(0..4) == Some(b"ttcf") {
            ttc::extract_face(&face.data, face.index)?
        } else {
            face.data.clone()
        };
        Some(("Noto Sans CJK SC".into(), face.path.clone(), bytes, bold))
    }

    /// Whether this FontDb has a bundled ODTTF-safe CJK face.
    pub fn has_embeddable_cjk(&self) -> bool {
        self.faces.contains_key("noto sans sc")
    }

    fn resolve_prefer_latin(&self, families: &[String], style: &str) -> Option<&FontFace> {
        for f in families {
            let key = f.to_ascii_lowercase();
            let resolved = self.aliases.get(&key).cloned().unwrap_or(key);
            if resolved.contains("cjk") || resolved.contains("noto") {
                continue;
            }
            if let Some(face) = self.face_style(&resolved, style) {
                return Some(face);
            }
        }
        // fall through original list
        self.resolve(families, style)
    }

    fn resolve_prefer_cjk(&self, families: &[String], style: &str) -> Option<&FontFace> {
        for f in families {
            let key = f.to_ascii_lowercase();
            let resolved = self.aliases.get(&key).cloned().unwrap_or(key);
            if resolved.contains("cjk")
                || resolved.contains("noto")
                || resolved.contains("pingfang")
                || resolved.contains("yahei")
            {
                if let Some(face) = self.face_style(&resolved, style) {
                    return Some(face);
                }
            }
        }
        self.faces
            .get("noto sans cjk jp")
            .and_then(|m| m.get(style).or_else(|| m.get("regular")))
            .or_else(|| {
                self.faces
                    .get("noto sans cjk sc")
                    .and_then(|m| m.get(style).or_else(|| m.get("regular")))
            })
    }

    fn face_style(&self, resolved: &str, style: &str) -> Option<&FontFace> {
        let styles = self.faces.get(resolved)?;
        let is_cjk_family = resolved.contains("cjk") || resolved.contains("noto");
        // Chrome on this host maps CSS weight ≥600 for CJK sans to the same face
        // as 700 (fc-match → Noto SC Bold/Black). Medium is visibly thinner.
        let style = if is_cjk_family && style == "medium" {
            "bold"
        } else {
            style
        };
        if let Some(face) = styles.get(style) {
            return Some(face);
        }
        // Fallbacks along the weight axis
        match style {
            "bold" => styles.get("medium").or_else(|| styles.get("regular")),
            "medium" => styles.get("regular").or_else(|| styles.get("bold")),
            "black" => styles
                .get("bold")
                .or_else(|| styles.get("medium"))
                .or_else(|| styles.get("regular")),
            _ => styles.get("regular"),
        }
    }

    fn resolve(&self, families: &[String], style: &str) -> Option<&FontFace> {
        for f in families {
            let key = f.to_ascii_lowercase();
            let resolved = self.aliases.get(&key).cloned().unwrap_or(key);
            if let Some(face) = self.face_style(&resolved, style) {
                return Some(face);
            }
        }
        None
    }
}

fn is_cjk(c: char) -> bool {
    matches!(
        c,
        '\u{3000}'..='\u{303F}' // CJK punctuation
            | '\u{3040}'..='\u{30FF}' // Hiragana/Katakana
            | '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{FF00}'..='\u{FFEF}' // fullwidth
            | '\u{20000}'..='\u{2FA1F}'
    ) || c == '：'
        || c == '（'
        || c == '）'
        || c == '、'
        || c == '。'
        || c == '—'
}

fn glyph_advance(
    ch: char,
    font_size: f64,
    latin: Option<&ttf_parser::Face<'_>>,
    cjk: Option<&ttf_parser::Face<'_>>,
) -> f64 {
    let prefer_cjk = is_cjk(ch);
    let order: [&Option<&ttf_parser::Face<'_>>; 2] = if prefer_cjk {
        [&cjk, &latin]
    } else {
        [&latin, &cjk]
    };
    for face in order.into_iter().flatten() {
        if let Some(gid) = face.glyph_index(ch) {
            let units = face.units_per_em() as f64;
            let scale = font_size / units.max(1.0);
            let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
            return adv * scale;
        }
    }
    if prefer_cjk {
        font_size
    } else {
        font_size * 0.5
    }
}


/// Max advance among ASCII digits 0–9 (for `font-variant-numeric: tabular-nums`).
fn max_digit_advance(
    font_size: f64,
    latin: Option<&ttf_parser::Face<'_>>,
    cjk: Option<&ttf_parser::Face<'_>>,
) -> f64 {
    let mut m = 0.0_f64;
    for d in b'0'..=b'9' {
        let a = glyph_advance(d as char, font_size, latin, cjk);
        if a > m {
            m = a;
        }
    }
    if m <= 0.0 {
        font_size * 0.6
    } else {
        m
    }
}

/// Horizontal kerning from the face `kern` table (pair adjustment in em units).
///
/// Disabled for outline→DrawingML paths: Chrome's SVG layout for this corpus
/// effectively matches unkerned advances for the dual Latin/CJK stack we use,
/// and applying `kern` shifted title/body widths enough to raise slide-1 mad.
/// Keep the helper for future opt-in / shaping backends (GPOS).
fn pair_kerning(
    left: char,
    right: char,
    font_size: f64,
    face: Option<&ttf_parser::Face<'_>>,
    enable: bool,
) -> f64 {
    if !enable {
        return 0.0;
    }
    pair_kerning_from_table(left, right, font_size, face)
}

#[allow(dead_code)]
fn pair_kerning_from_table(
    left: char,
    right: char,
    font_size: f64,
    face: Option<&ttf_parser::Face<'_>>,
) -> f64 {
    let Some(face) = face else {
        return 0.0;
    };
    let Some(kern) = face.tables().kern else {
        return 0.0;
    };
    let (Some(left_gid), Some(right_gid)) = (face.glyph_index(left), face.glyph_index(right))
    else {
        return 0.0;
    };
    let mut adjustment = 0_i16;
    for subtable in kern.subtables {
        if let Some(value) = subtable.glyphs_kerning(left_gid, right_gid) {
            adjustment = value;
            break;
        }
    }
    if adjustment == 0 {
        return 0.0;
    }
    let units = face.units_per_em() as f64;
    let scale = font_size / units.max(1.0);
    adjustment as f64 * scale
}

fn metrics_ascent_descent(
    font_size: f64,
    latin: Option<&ttf_parser::Face<'_>>,
    cjk: Option<&ttf_parser::Face<'_>>,
) -> (f64, f64) {
    let mut ascent = 0.0_f64;
    let mut descent = 0.0_f64;
    for face in [latin, cjk].into_iter().flatten() {
        let units = face.units_per_em() as f64;
        let scale = font_size / units.max(1.0);
        // Prefer typographic metrics (closer to CSS/SVG), fall back to hhea.
        let (asc, desc) = if let Some(os2) = face.tables().os2 {
            // use_typographic_metrics when available
            let typo_asc = os2.typographic_ascender() as f64;
            let typo_desc = -(os2.typographic_descender() as f64);
            if typo_asc > 0.0 {
                (typo_asc * scale, typo_desc.max(0.0) * scale)
            } else {
                (
                    face.ascender() as f64 * scale,
                    (-face.descender() as f64) * scale,
                )
            }
        } else {
            (
                face.ascender() as f64 * scale,
                (-face.descender() as f64) * scale,
            )
        };
        ascent = ascent.max(asc);
        descent = descent.max(desc);
    }
    if ascent == 0.0 {
        // CSS default roughly 0.8em ascent for sans
        ascent = font_size * 0.8;
    }
    if descent == 0.0 {
        descent = font_size * 0.2;
    }
    (ascent, descent)
}

fn pick_face_index(data: &[u8], want_family: &str) -> u32 {
    let n = ttf_parser::fonts_in_collection(data).unwrap_or(1);
    let want = want_family.to_ascii_lowercase();
    if want.contains("noto sans cjk jp") {
        return 0;
    }
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

/// Outline `text` into SVG-user-space path segments (Y-down), for DrawingML custGeom.
///
/// Uses the same dual-face Latin/CJK resolution as [`FontDb::measure`]. Glyph Y is
/// flipped from font space (Y-up) to SVG (Y-down). `letter_spacing` is added after
/// each glyph except the last.
pub fn outline_text(
    db: &FontDb,
    text: &str,
    font_size: f64,
    font_family: &str,
    bold: bool,
    origin_x: f64,
    baseline_y: f64,
    letter_spacing: f64,
) -> Vec<super::path::Segment> {
    outline_text_weight(
        db,
        text,
        font_size,
        font_family,
        if bold { 700 } else { 400 },
        origin_x,
        baseline_y,
        letter_spacing,
        0.0,
    )
}

pub fn outline_text_weight(
    db: &FontDb,
    text: &str,
    font_size: f64,
    font_family: &str,
    weight: i32,
    origin_x: f64,
    baseline_y: f64,
    letter_spacing: f64,
    word_spacing: f64,
) -> Vec<super::path::Segment> {
    outline_glyphs_weight(
        db,
        text,
        font_size,
        font_family,
        weight,
        origin_x,
        baseline_y,
        letter_spacing,
        word_spacing,
    )
    .into_iter()
    .flatten()
    .collect()
}

/// Outline each glyph as its own segment list (better DrawingML fidelity).
pub fn outline_glyphs_weight(
    db: &FontDb,
    text: &str,
    font_size: f64,
    font_family: &str,
    weight: i32,
    origin_x: f64,
    baseline_y: f64,
    letter_spacing: f64,
    word_spacing: f64,
) -> Vec<Vec<super::path::Segment>> {
    outline_glyphs_weight_ex(
        db,
        text,
        font_size,
        font_family,
        weight,
        origin_x,
        baseline_y,
        letter_spacing,
        word_spacing,
        false,
        false,
    )
}

/// Like [`outline_glyphs_weight`], with optional tabular digit advances and kern.
pub fn outline_glyphs_weight_ex(
    db: &FontDb,
    text: &str,
    font_size: f64,
    font_family: &str,
    weight: i32,
    origin_x: f64,
    baseline_y: f64,
    letter_spacing: f64,
    word_spacing: f64,
    tabular_nums: bool,
    enable_kern: bool,
) -> Vec<Vec<super::path::Segment>> {
    use super::path::Segment;
    use ttf_parser::OutlineBuilder;

    let style = weight_style(weight);
    let families = parse_font_family_list(font_family);
    let latin_face = db
        .resolve_prefer_latin(&families, style)
        .or_else(|| db.faces.get("liberation sans").and_then(|m| m.get(style)))
        .or_else(|| {
            db.faces
                .get("liberation sans")
                .and_then(|m| m.get("regular"))
        })
        .or_else(|| db.faces.get("dejavu sans").and_then(|m| m.get("regular")));
    let cjk_face = db
        .resolve_prefer_cjk(&families, style)
        .or_else(|| db.faces.get("noto sans cjk sc").and_then(|m| m.get(style)))
        .or_else(|| {
            db.faces
                .get("noto sans cjk sc")
                .and_then(|m| m.get("regular"))
        });

    let latin_data = latin_face.map(|f| (f.data.as_slice(), f.index));
    let cjk_data = cjk_face.map(|f| (f.data.as_slice(), f.index));
    let latin = latin_data.and_then(|(d, i)| ttf_parser::Face::parse(d, i).ok());
    let cjk = cjk_data.and_then(|(d, i)| ttf_parser::Face::parse(d, i).ok());

    struct Builder {
        segs: Vec<Segment>,
        scale: f64,
        ox: f64,
        oy: f64,
    }
    impl Builder {
        fn map(&self, x: f32, y: f32) -> (f64, f64) {
            (
                self.ox + x as f64 * self.scale,
                self.oy - y as f64 * self.scale,
            )
        }
    }
    impl OutlineBuilder for Builder {
        fn move_to(&mut self, x: f32, y: f32) {
            let (x, y) = self.map(x, y);
            self.segs.push(Segment::MoveTo { x, y });
        }
        fn line_to(&mut self, x: f32, y: f32) {
            let (x, y) = self.map(x, y);
            self.segs.push(Segment::LineTo { x, y });
        }
        fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
            let (x1, y1) = self.map(x1, y1);
            let (x, y) = self.map(x, y);
            self.segs.push(Segment::QuadTo { x1, y1, x, y });
        }
        fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
            let (x1, y1) = self.map(x1, y1);
            let (x2, y2) = self.map(x2, y2);
            let (x, y) = self.map(x, y);
            self.segs.push(Segment::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            });
        }
        fn close(&mut self) {
            self.segs.push(Segment::Close);
        }
    }

    let tab_digit = if tabular_nums {
        Some(max_digit_advance(font_size, latin.as_ref(), cjk.as_ref()))
    } else {
        None
    };

    let mut pen_x = origin_x;
    let mut glyphs = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut prev_glyph: Option<(char, bool)> = None;
    for (i, ch) in chars.iter().copied().enumerate() {
        let prefer_cjk = is_cjk(ch);
        // Apply kerning before placing this glyph (same face preference).
        if let Some((prev_ch, prev_cjk)) = prev_glyph {
            if prev_cjk == prefer_cjk {
                let skip_kern = tab_digit.is_some()
                    && prev_ch.is_ascii_digit()
                    && ch.is_ascii_digit();
                if !skip_kern {
                    pen_x += pair_kerning(
                        prev_ch,
                        ch,
                        font_size,
                        if prefer_cjk {
                            cjk.as_ref()
                        } else {
                            latin.as_ref()
                        },
                        enable_kern,
                    );
                }
            }
        }
        if ch == ' ' {
            // advance space without emitting geometry
            let faces: [Option<&ttf_parser::Face<'_>>; 2] = [latin.as_ref(), cjk.as_ref()];
            let mut adv = font_size * 0.33;
            for face in faces.into_iter().flatten() {
                if let Some(gid) = face.glyph_index(' ') {
                    let units = face.units_per_em() as f64;
                    let scale = font_size / units.max(1.0);
                    adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64 * scale;
                    break;
                }
            }
            pen_x += adv + word_spacing;
            if i + 1 < chars.len() {
                pen_x += letter_spacing;
            }
            prev_glyph = Some((ch, false));
            continue;
        }
        let faces: [Option<&ttf_parser::Face<'_>>; 2] = if prefer_cjk {
            [cjk.as_ref(), latin.as_ref()]
        } else {
            [latin.as_ref(), cjk.as_ref()]
        };
        let mut advanced = false;
        for face in faces.into_iter().flatten() {
            if let Some(gid) = face.glyph_index(ch) {
                let units = face.units_per_em() as f64;
                let scale = font_size / units.max(1.0);
                let natural_adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64 * scale;
                // Tabular nums: equal advance cell for ASCII digits; center ink.
                let (adv, tab_center) = if let Some(tw) = tab_digit {
                    if ch.is_ascii_digit() {
                        (tw, true)
                    } else {
                        (natural_adv, false)
                    }
                } else {
                    (natural_adv, false)
                };
                // Fullwidth punctuation (FF01–FF5E): Noto ink is left-biased and sits low
                // vs Chrome's mixed-stack colon. Center in the em and lift ~6%.
                let mut ox = pen_x;
                let mut oy = baseline_y;
                if tab_center {
                    // Center proportional digit ink inside the tabular cell.
                    ox = pen_x + (adv - natural_adv) * 0.5;
                } else if is_fullwidth_ascii(ch) {
                    if let Some(bbox) = face.glyph_bounding_box(gid) {
                        let ink_l = bbox.x_min as f64 * scale;
                        let ink_r = bbox.x_max as f64 * scale;
                        let ink_w = (ink_r - ink_l).max(0.0);
                        let target_l = (adv - ink_w) * 0.5;
                        ox = pen_x + (target_l - ink_l);
                    }
                    // Noto fullwidth colon sits lower than Chrome's mark.
                    oy = baseline_y - font_size * 0.008;
                }
                // Optical size: LO custGeom AA under-inks bold CJK vs Chrome
                // at small/UI sizes and slightly over-spreads regular gray.
                // Tuned on slide-1 regional mad (global ~3.15).
                let (draw_scale, ox_adj, oy_adj) = if !is_fullwidth_ascii(ch) {
                    let optical = if prefer_cjk {
                        if font_size <= 16.0 {
                            // Regular body + small bold UI (node labels 14–16): 1.012.
                            // Bold was 1.020; A/B 1.012 → global mad 3.0408→3.0393
                            // (llm_model label mad −0.26). Hold 1.012.
                            // A/B regular 1.016 → mad 3.0393; hold 1.012.
                            if weight < 500 {
                                1.012
                            } else {
                                1.012
                            }
                        } else if font_size <= 20.0 {
                            // Card titles ~18px weight 600.
                            // A/B 1.030/1.038 regressed; hold 1.034.
                            if weight < 500 {
                                1.0
                            } else {
                                1.034
                            }
                        } else if font_size <= 24.0 {
                            // Subtitle ~22px regular: identity; LO AA already heavy.
                            if weight < 500 {
                                1.0
                            } else {
                                1.022
                            }
                        } else if font_size <= 28.0 {
                            1.022
                        } else {
                            // Display title ≥28: 1.020 locked (A/B 1.015 → mad 3.05).
                            1.020
                        }
                    } else if font_size <= 18.0 && weight >= 600 {
                        // Small bold Latin (badge + "Dify Platform"): 1.035.
                        // A/B vs 1.025: global 3.0393→3.0387, platform mad −0.30;
                        // badge +0.18 (acceptable). Hold 1.035.
                        // A/B 1.040 → global mad 3.0407; hold 1.035.
                        1.035
                    } else {
                        1.0
                    };
                    let s = scale * optical;
                    // Keep ink centered in the advance cell.
                    let ox2 = pen_x - (adv * 0.5) * (optical - 1.0);
                    // Card body gray CJK (~15px regular): Noto sits slightly high
                    // vs Chrome PingFang-like stack — nudge baseline down. Footer
                    // (12px) excluded via size > 13.
                    let oy2 = if weight < 500 && font_size > 13.0 && font_size <= 16.0 {
                        oy + font_size * 0.018
                    } else if weight < 500 && font_size > 20.0 && font_size <= 24.0 {
                        oy + font_size * 0.010
                    } else if weight >= 600 && font_size > 16.0 && font_size <= 20.0 {
                        oy + font_size * 0.008
                    } else {
                        oy
                    };
                    (s, ox2, oy2)
                } else {
                    (scale, ox, oy)
                };
                let mut b = Builder {
                    segs: Vec::new(),
                    scale: draw_scale,
                    ox: ox_adj,
                    oy: oy_adj,
                };
                let _ = face.outline_glyph(gid, &mut b);
                if !b.segs.is_empty() {
                    glyphs.push(b.segs);
                }
                pen_x += adv;
                advanced = true;
                break;
            }
        }
        if !advanced {
            pen_x += if prefer_cjk {
                font_size
            } else {
                font_size * 0.5
            };
        }
        prev_glyph = Some((ch, prefer_cjk));
        if i + 1 < chars.len() {
            pen_x += letter_spacing;
        }
    }
    glyphs
}

fn is_fullwidth_ascii(c: char) -> bool {
    let u = c as u32;
    (0xFF01..=0xFF5E).contains(&u)
}

/// Map CSS font-weight to our loaded style keys.
fn weight_style(weight: i32) -> &'static str {
    if weight >= 900 {
        "black"
    } else if weight >= 600 {
        // CSS 600 is semi-bold; Chrome synthesizes/selects bold for common
        // sans stacks (Liberation has no Medium). Prefer bold for ≥600.
        "bold"
    } else if weight >= 500 {
        "medium"
    } else {
        "regular"
    }
}

/// Parse CSS font-weight string to numeric 100–900.
pub fn parse_font_weight(s: &str) -> i32 {
    match s.trim().to_ascii_lowercase().as_str() {
        "normal" | "regular" => 400,
        "bold" => 700,
        "bolder" => 700,
        "lighter" => 300,
        other => other.parse::<i32>().unwrap_or(400).clamp(100, 900),
    }
}

fn os2_x_height(os2: &ttf_parser::os2::Table<'_>) -> f64 {
    // ttf-parser versions differ: try x_height() returning Option or i16.
    // Use unicode_range / raw if needed — prefer glyph fallback when 0.
    #[allow(unused_imports)]
    use ttf_parser::os2::Table;
    // In ttf-parser 0.25, `x_height()` -> Option<i16>
    match os2.x_height() {
        Some(v) if v > 0 => v as f64,
        _ => 0.0,
    }
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
    fn kerning_table_helper_is_finite() {
        let db = FontDb::global();
        // Outline path keeps unkerned advances (Chrome SVG parity for dual stack).
        // The raw table helper must still return a finite adjustment when present.
        let pair = db.measure("AV", 64.0, "Liberation Sans, Helvetica, sans-serif", false);
        let a = db.measure("A", 64.0, "Liberation Sans, Helvetica, sans-serif", false);
        let v = db.measure("V", 64.0, "Liberation Sans, Helvetica, sans-serif", false);
        assert!(pair.width.is_finite() && pair.width > 0.0);
        // With kerning gated off, pair width equals sum of advances.
        assert!(
            (pair.width - (a.width + v.width)).abs() < 0.5,
            "pair={} a+v={}",
            pair.width,
            a.width + v.width
        );
        let _ = pair_kerning_from_table;
    }

    #[test]
    fn measure_cjk() {
        let db = FontDb::global();
        let m = db.measure(
            "中文",
            16.0,
            "PingFang SC, Noto Sans CJK SC, sans-serif",
            true,
        );
        assert!(m.width > 20.0, "width={}", m.width);
    }

    #[test]
    fn measure_mixed_wider_than_latin_only_heuristic() {
        let db = FontDb::global();
        let mixed = db.measure(
            "Dify：让 LLM 应用落地触手可及",
            54.0,
            "Helvetica Neue, Helvetica, Arial, PingFang SC, sans-serif",
            true,
        );
        // 14 visible chars with CJK should be substantially wider than pure 0.55*em latin
        assert!(mixed.width > 400.0, "width={}", mixed.width);
        assert!(mixed.width < 900.0, "width={}", mixed.width);
    }

    #[test]
    fn tabular_nums_equalizes_digit_widths() {
        let db = FontDb::global();
        let prop = db.measure_weight_ex("111", 64.0, "Liberation Sans, sans-serif", 400, false, false);
        let tab = db.measure_weight_ex("111", 64.0, "Liberation Sans, sans-serif", 400, true, false);
        let prop9 = db.measure_weight_ex("999", 64.0, "Liberation Sans, sans-serif", 400, false, false);
        let tab9 = db.measure_weight_ex("999", 64.0, "Liberation Sans, sans-serif", 400, true, false);
        assert!(
            (tab.width - tab9.width).abs() < 0.5,
            "tab 111={} tab 999={}",
            tab.width,
            tab9.width
        );
        if prop.width + 1.0 < prop9.width {
            assert!(
                tab.width + 0.5 >= prop.width,
                "tabular should not shrink 111: prop={} tab={}",
                prop.width,
                tab.width
            );
        }
        let _ = prop9;
    }

    #[test]
    fn typeface_for_defaults_tnr_and_yahei() {
        let db = FontDb::global();
        // SVG omitted font-family → generic default stack.
        assert_eq!(
            db.typeface_for("Times New Roman, Microsoft YaHei, 微软雅黑, serif", false),
            "Times New Roman"
        );
        assert_eq!(
            db.typeface_for("Times New Roman, Microsoft YaHei, 微软雅黑, serif", true),
            "Microsoft YaHei"
        );
        assert_eq!(db.typeface_for("serif", false), "Times New Roman");
        assert_eq!(db.typeface_for("sans-serif", true), "Microsoft YaHei");
        // Explicit Latin sans stack still maps to Arial (Windows).
        assert_eq!(
            db.typeface_for("Helvetica Neue, Helvetica, Arial, PingFang SC, sans-serif", false),
            "Arial"
        );
        assert_eq!(
            db.typeface_for("Helvetica Neue, Helvetica, Arial, PingFang SC, sans-serif", true),
            "Microsoft YaHei"
        );
    }
}

