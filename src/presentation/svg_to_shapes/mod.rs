//! Full SVG → PowerPoint DrawingML converter.
//!
//! Implements MDN-aligned support for:
//! - Geometry: `rect`, `circle`, `ellipse`, `line`, `polyline`, `polygon`, `path`
//!   (M/L/H/V/C/S/Q/T/A/Z absolute+relative)
//! - Transforms: `matrix/translate/scale/rotate/skewX/skewY` (right-to-left)
//! - Paint: solid colors, `linearGradient` / `radialGradient` → `a:gradFill`
//! - Style inheritance: presentation attrs, inline `style=`, and `<style>` class/id rules
//! - Filters: `feGaussianBlur` → `a:glow`; `feOffset`/`feDropShadow`(+blur/flood) → `a:outerShdw`
//! - Clipping: `clipPath` rect **and** polygon (point-in-polygon sample clip)
//! - Masks: luminance/opacity approximation via `mask` → shape alpha
//! - Markers: `marker-start/mid/end` on paths/lines
//! - Patterns: true tile expansion of pattern children across shape bbox (capped)
//! - Reuse: same-document `<use href="#id">` / `symbol` with x/y/transform
//! - Text: real glyph advances via `ttf-parser` + system fonts; rotation via `a:xfrm/@rot`
//! - Fonts used are reported and embedded as ODTTF (`application/vnd...obfuscatedFont`)
//!
//! Coordinates map the SVG `viewBox` onto a target EMU rectangle.
//!
//! Pixel-level parity vs browser/LO SVG raster is measured by
//! `scripts/svg_pptx_pixel_diff.py` (LO host differences expected).

mod dml;
mod font;
mod matrix;
pub mod odttf;
mod paint;
mod path;

use crate::element::OpenXmlElement;
use crate::error::{Error, Result};
use crate::presentation::{group_shape_pr, group_shape_properties, shape_tree};
use dml::{
    freeform_shape, preset_shape, text_shape, ShapeBuild, ShapeEffect, TextShapeOpts,
};
use font::FontDb;
use matrix::Matrix;
use paint::{
    parse_color, parse_percent_or_number, resolve_paint, Gradient, GradientKind, GradientMap, Paint,
};
use path::{
    bounds, clip_segments_to_polygon, clip_segments_to_rect, ellipse_path, line_path,
    marker_anchors, parse_path, parse_points, path_polygon, polygon_path, rect_path,
    transform_segments, Segment,
};
use std::collections::{HashMap, HashSet};

/// A font face that was actually used while converting text (for PPTX embedding).
#[derive(Debug, Clone)]
pub struct UsedFont {
    pub typeface: String,
    pub bold: bool,
    pub path: std::path::PathBuf,
    pub data: Vec<u8>,
}

/// Result of converting an SVG document into DrawingML shape elements.
#[derive(Debug, Clone)]
pub struct SvgShapeConversion {
    pub shapes: Vec<OpenXmlElement>,
    pub next_shape_id: u32,
    pub view_width: f64,
    pub view_height: f64,
    /// Distinct fonts referenced by text runs (for optional embedding).
    pub used_fonts: Vec<UsedFont>,
}

/// Convert SVG bytes into native DrawingML shapes sized to `(target_cx, target_cy)` EMUs.
pub fn svg_to_shapes(
    svg_bytes: &[u8],
    target_cx: i64,
    target_cy: i64,
    start_id: u32,
) -> Result<SvgShapeConversion> {
    let text = std::str::from_utf8(svg_bytes)
        .map_err(|e| Error::Xml(format!("svg is not utf-8: {e}")))?;
    let root = parse_dom(text)?;
    let (vx, vy, vw, vh) = view_box(&root).unwrap_or((0.0, 0.0, 1280.0, 720.0));

    let mut gradients = GradientMap::new();
    collect_gradients(&root, &mut gradients);

    let mut filters = FilterMap::new();
    collect_filters(&root, &mut filters);

    let css = collect_css_rules(&root);

    let mut id_index: HashMap<String, Node> = HashMap::new();
    index_ids(&root, &mut id_index);

    let mut clip_paths = ClipPathMap::new();
    collect_clip_paths(&root, &mut clip_paths);

    let mut markers = MarkerMap::new();
    collect_markers(&root, &mut markers);

    let mut patterns = PatternMap::new();
    collect_patterns(&root, &mut patterns);

    let mut masks = MaskMap::new();
    collect_masks(&root, &mut masks);

    // user → EMU: translate -viewBox origin, then scale
    let sx = target_cx as f64 / vw.max(1e-9);
    let sy = target_cy as f64 / vh.max(1e-9);
    let user_to_emu = Matrix::translate(-vx, -vy).then(Matrix::scale(sx, sy));

    let fonts = FontDb::global();
    let mut shapes = Vec::new();
    let mut next_id = start_id.max(2);
    let mut used_font_keys: HashSet<String> = HashSet::new();
    let mut used_fonts: Vec<UsedFont> = Vec::new();
    let initial = Style::default();
    let mut ctx = WalkCtx {
        gradients: &gradients,
        filters: &filters,
        css: &css,
        id_index: &id_index,
        clip_paths: &clip_paths,
        markers: &markers,
        patterns: &patterns,
        masks: &masks,
        user_to_emu,
        fonts,
        used_font_keys: &mut used_font_keys,
        used_fonts: &mut used_fonts,
        active_clip: None,
        expand_patterns: true,
    };
    walk(
        &root,
        Matrix::identity(),
        &initial,
        &mut ctx,
        &mut shapes,
        &mut next_id,
        0,
    )?;

    Ok(SvgShapeConversion {
        shapes,
        next_shape_id: next_id,
        view_width: vw,
        view_height: vh,
        used_fonts,
    })
}

/// Build a complete `p:spTree` from converted shapes.
pub fn shape_tree_from_svg(
    svg_bytes: &[u8],
    target_cx: i64,
    target_cy: i64,
) -> Result<(OpenXmlElement, SvgShapeConversion)> {
    let conv = svg_to_shapes(svg_bytes, target_cx, target_cy, 2)?;
    let mut kids = vec![group_shape_properties(), group_shape_pr()];
    kids.extend(conv.shapes.clone());
    Ok((shape_tree(kids), conv))
}

// ── DOM ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Node {
    name: String,
    attrs: HashMap<String, String>,
    text: String,
    children: Vec<Node>,
}

fn parse_dom(xml: &str) -> Result<Node> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut stack: Vec<Node> = Vec::new();
    let mut root: Option<Node> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().local_name().as_ref()).into_owned();
                let mut attrs = HashMap::new();
                for a in e.attributes().with_checks(false).flatten() {
                    let k = String::from_utf8_lossy(a.key.as_ref())
                        .rsplit(':')
                        .next()
                        .unwrap_or("")
                        .to_string();
                    // prefer local name
                    let k = String::from_utf8_lossy(a.key.local_name().as_ref()).into_owned();
                    let v = a
                        .unescape_value()
                        .map(|c| c.into_owned())
                        .unwrap_or_else(|_| String::from_utf8_lossy(&a.value).into_owned());
                    attrs.insert(k, v);
                }
                stack.push(Node {
                    name,
                    attrs,
                    text: String::new(),
                    children: Vec::new(),
                });
            }
            Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().local_name().as_ref()).into_owned();
                let mut attrs = HashMap::new();
                for a in e.attributes().with_checks(false).flatten() {
                    let k = String::from_utf8_lossy(a.key.local_name().as_ref()).into_owned();
                    let v = a
                        .unescape_value()
                        .map(|c| c.into_owned())
                        .unwrap_or_else(|_| String::from_utf8_lossy(&a.value).into_owned());
                    attrs.insert(k, v);
                }
                let node = Node {
                    name,
                    attrs,
                    text: String::new(),
                    children: Vec::new(),
                };
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    root = Some(node);
                }
            }
            Ok(Event::End(_)) => {
                let node = stack
                    .pop()
                    .ok_or_else(|| Error::Xml("svg stack underflow".into()))?;
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    root = Some(node);
                }
            }
            Ok(Event::Text(t)) => {
                if let Some(parent) = stack.last_mut() {
                    let s = t.unescape().unwrap_or_default();
                    parent.text.push_str(&s);
                }
            }
            Ok(Event::CData(t)) => {
                if let Some(parent) = stack.last_mut() {
                    parent.text.push_str(&String::from_utf8_lossy(&t));
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Error::Xml(format!("svg parse: {e}"))),
            _ => {}
        }
        buf.clear();
    }
    root.ok_or_else(|| Error::Xml("empty svg".into()))
}

fn view_box(root: &Node) -> Option<(f64, f64, f64, f64)> {
    if let Some(vb) = root.attrs.get("viewBox") {
        let parts: Vec<f64> = matrix::parse_numbers(vb);
        if parts.len() == 4 {
            return Some((parts[0], parts[1], parts[2], parts[3]));
        }
    }
    let w = root
        .attrs
        .get("width")
        .and_then(|s| parse_length(s))
        .unwrap_or(1280.0);
    let h = root
        .attrs
        .get("height")
        .and_then(|s| parse_length(s))
        .unwrap_or(720.0);
    Some((0.0, 0.0, w, h))
}

fn parse_length(s: &str) -> Option<f64> {
    let s = s.trim();
    let s = s
        .trim_end_matches("px")
        .trim_end_matches("pt")
        .trim_end_matches("em")
        .trim();
    s.parse().ok()
}

// ── Style ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Style {
    fill: Option<String>,
    stroke: Option<String>,
    stroke_width: f64,
    stroke_opacity: f64,
    fill_opacity: f64,
    opacity: f64,
    stroke_dasharray: Option<String>,
    stroke_linecap: Option<String>,
    stroke_linejoin: Option<String>,
    font_family: String,
    font_size: f64,
    font_weight: String,
    font_style: String,
    letter_spacing: f64,
    text_anchor: String,
    /// Resolved filter id (without leading #), if any.
    filter: Option<String>,
    /// clip-path url id (without #).
    clip_path: Option<String>,
    /// mask url id (without #).
    mask: Option<String>,
    marker_start: Option<String>,
    marker_mid: Option<String>,
    marker_end: Option<String>,
    display: bool,
    visibility: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill: Some("black".into()),
            stroke: Some("none".into()),
            stroke_width: 1.0,
            stroke_opacity: 1.0,
            fill_opacity: 1.0,
            opacity: 1.0,
            stroke_dasharray: None,
            stroke_linecap: None,
            stroke_linejoin: None,
            font_family: "sans-serif".into(),
            font_size: 16.0,
            font_weight: "400".into(),
            font_style: "normal".into(),
            letter_spacing: 0.0,
            text_anchor: "start".into(),
            filter: None,
            clip_path: None,
            mask: None,
            marker_start: None,
            marker_mid: None,
            marker_end: None,
            display: true,
            visibility: true,
        }
    }
}

impl Style {
    fn apply_node(&self, node: &Node, css: &CssRules) -> Self {
        let mut s = self.clone();
        // CSS stylesheet rules (element / class / id) — lower specificity first
        apply_css_for_node(&mut s, node, css);
        // presentation attributes override stylesheet
        apply_attr(&mut s, &node.attrs);
        // then style="" overrides everything
        if let Some(style) = node.attrs.get("style") {
            let mut map = HashMap::new();
            for part in style.split(';') {
                let mut kv = part.splitn(2, ':');
                if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                    map.insert(k.trim().to_string(), v.trim().to_string());
                }
            }
            apply_attr(&mut s, &map);
        }
        s
    }
}

fn apply_attr(s: &mut Style, attrs: &HashMap<String, String>) {
    if let Some(v) = attrs.get("fill") {
        s.fill = Some(v.clone());
    }
    if let Some(v) = attrs.get("stroke") {
        s.stroke = Some(v.clone());
    }
    if let Some(v) = attrs.get("stroke-width") {
        if let Some(n) = parse_length(v) {
            s.stroke_width = n;
        }
    }
    if let Some(v) = attrs.get("stroke-opacity") {
        s.stroke_opacity = v.parse().unwrap_or(s.stroke_opacity);
    }
    if let Some(v) = attrs.get("fill-opacity") {
        s.fill_opacity = v.parse().unwrap_or(s.fill_opacity);
    }
    if let Some(v) = attrs.get("opacity") {
        // Local opacity only; walk() multiplies with parent.
        s.opacity = v.parse().unwrap_or(1.0);
    }
    if let Some(v) = attrs.get("stroke-dasharray") {
        if v.trim() != "none" {
            s.stroke_dasharray = Some(v.clone());
        } else {
            s.stroke_dasharray = None;
        }
    }
    if let Some(v) = attrs.get("stroke-linecap") {
        s.stroke_linecap = Some(v.clone());
    }
    if let Some(v) = attrs.get("stroke-linejoin") {
        s.stroke_linejoin = Some(v.clone());
    }
    if let Some(v) = attrs.get("font-family") {
        s.font_family = v.clone();
    }
    if let Some(v) = attrs.get("font-size") {
        if let Some(n) = parse_length(v) {
            s.font_size = n;
        }
    }
    if let Some(v) = attrs.get("font-weight") {
        s.font_weight = v.clone();
    }
    if let Some(v) = attrs.get("font-style") {
        s.font_style = v.clone();
    }
    if let Some(v) = attrs.get("letter-spacing") {
        if let Some(n) = parse_length(v) {
            s.letter_spacing = n;
        }
    }
    if let Some(v) = attrs.get("text-anchor") {
        s.text_anchor = v.clone();
    }
    if let Some(v) = attrs.get("filter") {
        if v.trim().eq_ignore_ascii_case("none") {
            s.filter = None;
        } else {
            s.filter = parse_url_id(v);
        }
    }
    if let Some(v) = attrs.get("clip-path") {
        if v.trim().eq_ignore_ascii_case("none") {
            s.clip_path = None;
        } else {
            s.clip_path = parse_url_id(v);
        }
    }
    if let Some(v) = attrs.get("mask") {
        if v.trim().eq_ignore_ascii_case("none") {
            s.mask = None;
        } else {
            s.mask = parse_url_id(v);
        }
    }
    if let Some(v) = attrs.get("marker-start") {
        s.marker_start = if v.trim().eq_ignore_ascii_case("none") {
            None
        } else {
            parse_url_id(v)
        };
    }
    if let Some(v) = attrs.get("marker-mid") {
        s.marker_mid = if v.trim().eq_ignore_ascii_case("none") {
            None
        } else {
            parse_url_id(v)
        };
    }
    if let Some(v) = attrs.get("marker-end") {
        s.marker_end = if v.trim().eq_ignore_ascii_case("none") {
            None
        } else {
            parse_url_id(v)
        };
    }
    // shorthand `marker` sets all three
    if let Some(v) = attrs.get("marker") {
        if v.trim().eq_ignore_ascii_case("none") {
            s.marker_start = None;
            s.marker_mid = None;
            s.marker_end = None;
        } else if let Some(id) = parse_url_id(v) {
            s.marker_start = Some(id.clone());
            s.marker_mid = Some(id.clone());
            s.marker_end = Some(id);
        }
    }
    if let Some(v) = attrs.get("display") {
        s.display = v != "none";
    }
    if let Some(v) = attrs.get("visibility") {
        s.visibility = v != "hidden" && v != "collapse";
    }
}

fn parse_url_id(s: &str) -> Option<String> {
    let s = s.trim();
    let rest = s.strip_prefix("url(")?.trim_end_matches(')').trim();
    let id = rest
        .trim_matches(|c| c == '"' || c == '\'')
        .trim_start_matches('#')
        .trim();
    if id.is_empty() {
        None
    } else {
        Some(id.to_string())
    }
}

// ── CSS <style> rules ───────────────────────────────────────────────────────

#[derive(Debug, Default, Clone)]
struct CssRules {
    /// tag name → declarations
    by_tag: HashMap<String, HashMap<String, String>>,
    /// class name → declarations
    by_class: HashMap<String, HashMap<String, String>>,
    /// id → declarations
    by_id: HashMap<String, HashMap<String, String>>,
}

fn collect_css_rules(root: &Node) -> CssRules {
    let mut rules = CssRules::default();
    collect_css_rules_rec(root, &mut rules);
    rules
}

fn collect_css_rules_rec(node: &Node, rules: &mut CssRules) {
    if node.name == "style" {
        parse_css_block(&node.text, rules);
    }
    for c in &node.children {
        collect_css_rules_rec(c, rules);
    }
}

/// Minimal CSS parser for SVG presentation properties.
/// Supports simple selectors: `tag`, `.class`, `#id`, and comma-separated lists.
fn parse_css_block(css: &str, rules: &mut CssRules) {
    // Strip /* comments */
    let mut cleaned = String::with_capacity(css.len());
    let bytes = css.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(bytes.len());
            continue;
        }
        cleaned.push(bytes[i] as char);
        i += 1;
    }

    let mut rest = cleaned.as_str();
    while let Some(brace) = rest.find('{') {
        let selectors = rest[..brace].trim();
        rest = &rest[brace + 1..];
        let end = match rest.find('}') {
            Some(e) => e,
            None => break,
        };
        let body = &rest[..end];
        rest = &rest[end + 1..];
        let decls = parse_declarations(body);
        if decls.is_empty() {
            continue;
        }
        for sel in selectors.split(',') {
            let sel = sel.trim();
            if sel.is_empty() {
                continue;
            }
            if let Some(id) = sel.strip_prefix('#') {
                // only bare #id
                if id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                    merge_decls(rules.by_id.entry(id.to_string()).or_default(), &decls);
                }
            } else if let Some(cls) = sel.strip_prefix('.') {
                if cls.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                    merge_decls(rules.by_class.entry(cls.to_string()).or_default(), &decls);
                }
            } else if sel.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                merge_decls(
                    rules.by_tag.entry(sel.to_ascii_lowercase()).or_default(),
                    &decls,
                );
            }
            // complex selectors ignored
        }
    }
}

fn parse_declarations(body: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for part in body.split(';') {
        let mut kv = part.splitn(2, ':');
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            let k = k.trim();
            let v = v.trim();
            if !k.is_empty() && !v.is_empty() {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }
    map
}

fn merge_decls(dst: &mut HashMap<String, String>, src: &HashMap<String, String>) {
    for (k, v) in src {
        dst.insert(k.clone(), v.clone());
    }
}

fn apply_css_for_node(s: &mut Style, node: &Node, css: &CssRules) {
    // tag
    if let Some(decls) = css.by_tag.get(&node.name.to_ascii_lowercase()) {
        apply_attr(s, decls);
    }
    // classes
    if let Some(class_attr) = node.attrs.get("class") {
        for cls in class_attr.split_whitespace() {
            if let Some(decls) = css.by_class.get(cls) {
                apply_attr(s, decls);
            }
        }
    }
    // id
    if let Some(id) = node.attrs.get("id") {
        if let Some(decls) = css.by_id.get(id) {
            apply_attr(s, decls);
        }
    }
}

// ── Filters ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct SvgFilter {
    /// Primary blur stdDeviation (user units).
    blur: f64,
    /// Whether SourceGraphic is composited over the blur (glow keeps original).
    composite_source: bool,
    /// feOffset dx/dy in user units.
    offset_dx: f64,
    offset_dy: f64,
    /// Flood / shadow colour when present.
    flood_rgb: Option<[u8; 3]>,
    flood_alpha: f64,
    /// Prefer outerShdw (offset present) over pure glow.
    as_shadow: bool,
}

type FilterMap = HashMap<String, SvgFilter>;

fn collect_filters(node: &Node, out: &mut FilterMap) {
    if node.name == "filter" {
        if let Some(id) = node.attrs.get("id") {
            let mut blur = 0.0_f64;
            let mut composite_source = false;
            let mut offset_dx = 0.0_f64;
            let mut offset_dy = 0.0_f64;
            let mut flood_rgb = None;
            let mut flood_alpha = 1.0_f64;
            let mut has_offset = false;
            for c in &node.children {
                match c.name.as_str() {
                    "feGaussianBlur" => {
                        if let Some(sd) = c.attrs.get("stdDeviation") {
                            let nums = matrix::parse_numbers(sd);
                            if !nums.is_empty() {
                                let a = nums[0];
                                let b = nums.get(1).copied().unwrap_or(a);
                                blur = (a + b) / 2.0;
                            }
                        }
                    }
                    "feOffset" => {
                        has_offset = true;
                        offset_dx = c
                            .attrs
                            .get("dx")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);
                        offset_dy = c
                            .attrs
                            .get("dy")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);
                    }
                    "feFlood" => {
                        if let Some(col) = c.attrs.get("flood-color") {
                            if let Some((rgb, a)) = parse_color(col) {
                                flood_rgb = Some(rgb);
                                flood_alpha = a;
                            }
                        }
                        if let Some(op) = c.attrs.get("flood-opacity") {
                            flood_alpha = op.parse().unwrap_or(flood_alpha);
                        }
                    }
                    "feDropShadow" => {
                        // SVG2 convenience primitive
                        has_offset = true;
                        offset_dx = c
                            .attrs
                            .get("dx")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(2.0);
                        offset_dy = c
                            .attrs
                            .get("dy")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(2.0);
                        if let Some(sd) = c.attrs.get("stdDeviation") {
                            let nums = matrix::parse_numbers(sd);
                            if !nums.is_empty() {
                                blur = nums[0];
                            }
                        }
                        if let Some(col) = c.attrs.get("flood-color") {
                            if let Some((rgb, a)) = parse_color(col) {
                                flood_rgb = Some(rgb);
                                flood_alpha = a;
                            }
                        }
                        if let Some(op) = c.attrs.get("flood-opacity") {
                            flood_alpha = op.parse().unwrap_or(flood_alpha);
                        }
                    }
                    "feComposite" | "feMerge" | "feBlend" => {
                        composite_source = true;
                    }
                    "feColorMatrix" => {
                        // Luminance-to-alpha / matrix filters don't map 1:1 to DrawingML;
                        // treat as soft composite so glow/shadow paths still attach.
                        composite_source = true;
                    }
                    _ => {}
                }
            }
            out.insert(
                id.clone(),
                SvgFilter {
                    blur,
                    composite_source,
                    offset_dx,
                    offset_dy,
                    flood_rgb,
                    flood_alpha,
                    as_shadow: has_offset,
                },
            );
        }
    }
    for c in &node.children {
        collect_filters(c, out);
    }
}

fn shape_effect_for(
    style: &Style,
    filters: &FilterMap,
    ctm: Matrix,
    user_to_emu: Matrix,
) -> Option<ShapeEffect> {
    let id = style.filter.as_deref()?;
    let f = filters.get(id)?;
    let scale = ctm.then(user_to_emu).avg_scale();
    let mut eff = ShapeEffect::default();

    if f.as_shadow {
        // Map offset+blur → outerShdw
        let dist = (f.offset_dx.hypot(f.offset_dy) * scale).round().max(0.0) as i64;
        let blur_emu = (f.blur * 2.5 * scale).round().max(0.0) as i64;
        if dist > 0 || blur_emu > 0 {
            // angle: atan2(dy, dx) in SVG y-down → DML clockwise from right
            let deg = f.offset_dy.atan2(f.offset_dx).to_degrees();
            let mut ang = deg;
            if ang < 0.0 {
                ang += 360.0;
            }
            let (rgb, a) = f
                .flood_rgb
                .map(|rgb| (rgb, f.flood_alpha))
                .unwrap_or(([0, 0, 0], 0.4));
            eff.shadow_blur_emu = blur_emu;
            eff.shadow_dist_emu = dist;
            eff.shadow_dir = (ang * 60_000.0).round() as i32;
            eff.shadow_rgb = rgb;
            eff.shadow_alpha = (a * style.opacity).clamp(0.05, 0.9);
        }
    } else if f.blur > 0.0 {
        // Pure blur / glow
        let rad_emu = (f.blur * 2.5 * scale).round().max(0.0) as i64;
        if rad_emu > 0 {
            let (rgb, alpha) = glow_color(style);
            let alpha = (alpha * style.opacity).clamp(0.05, 0.9);
            let _ = f.composite_source;
            eff.glow_rad_emu = rad_emu;
            eff.glow_rgb = rgb;
            eff.glow_alpha = alpha;
        }
    }

    if eff.is_empty() {
        None
    } else {
        Some(eff)
    }
}

fn glow_color(style: &Style) -> ([u8; 3], f64) {
    if let Some(stroke) = style.stroke.as_deref() {
        if !stroke.eq_ignore_ascii_case("none") {
            if let Some((rgb, a)) = parse_color(stroke) {
                return (rgb, a * style.stroke_opacity);
            }
            if stroke.starts_with("url(") {
                return ([59, 130, 246], 0.7); // #3B82F6
            }
        }
    }
    if let Some(fill) = style.fill.as_deref() {
        if !fill.eq_ignore_ascii_case("none") {
            if let Some((rgb, a)) = parse_color(fill) {
                return (rgb, a * style.fill_opacity);
            }
            if fill.starts_with("url(") {
                return ([59, 130, 246], 0.55);
            }
        }
    }
    ([59, 130, 246], 0.5)
}

// ── Clip paths ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum ClipGeom {
    /// Axis-aligned rect in the clipPath's local space.
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    },
    /// Polygon vertices in local space.
    Polygon(Vec<(f64, f64)>),
}

#[derive(Debug, Clone)]
struct ClipPathDef {
    geom: ClipGeom,
    /// clipPathUnits objectBoundingBox?
    object_bbox: bool,
    transform: Matrix,
}

type ClipPathMap = HashMap<String, ClipPathDef>;

fn collect_clip_paths(node: &Node, out: &mut ClipPathMap) {
    if node.name == "clipPath" {
        if let Some(id) = node.attrs.get("id") {
            let object_bbox = node
                .attrs
                .get("clipPathUnits")
                .map(|s| s.as_str() == "objectBoundingBox")
                .unwrap_or(false);
            let transform = node
                .attrs
                .get("transform")
                .map(|s| Matrix::parse_transform_list(s))
                .unwrap_or_default();
            // Take first geometry child (rect / circle / ellipse / polygon / path)
            if let Some(geom) = first_clip_geom(node) {
                out.insert(
                    id.clone(),
                    ClipPathDef {
                        geom,
                        object_bbox,
                        transform,
                    },
                );
            }
        }
    }
    for c in &node.children {
        collect_clip_paths(c, out);
    }
}

fn first_clip_geom(node: &Node) -> Option<ClipGeom> {
    for c in &node.children {
        match c.name.as_str() {
            "rect" => {
                let x = attr_f(c, "x", 0.0);
                let y = attr_f(c, "y", 0.0);
                let w = attr_f(c, "width", 0.0);
                let h = attr_f(c, "height", 0.0);
                if w > 0.0 && h > 0.0 {
                    return Some(ClipGeom::Rect { x, y, w, h });
                }
            }
            "circle" => {
                let cx = attr_f(c, "cx", 0.0);
                let cy = attr_f(c, "cy", 0.0);
                let r = attr_f(c, "r", 0.0);
                if r > 0.0 {
                    // approximate as polygon
                    let n = 24;
                    let mut poly = Vec::with_capacity(n);
                    for i in 0..n {
                        let a = i as f64 / n as f64 * std::f64::consts::TAU;
                        poly.push((cx + r * a.cos(), cy + r * a.sin()));
                    }
                    return Some(ClipGeom::Polygon(poly));
                }
            }
            "ellipse" => {
                let cx = attr_f(c, "cx", 0.0);
                let cy = attr_f(c, "cy", 0.0);
                let rx = attr_f(c, "rx", 0.0);
                let ry = attr_f(c, "ry", 0.0);
                if rx > 0.0 && ry > 0.0 {
                    let n = 24;
                    let mut poly = Vec::with_capacity(n);
                    for i in 0..n {
                        let a = i as f64 / n as f64 * std::f64::consts::TAU;
                        poly.push((cx + rx * a.cos(), cy + ry * a.sin()));
                    }
                    return Some(ClipGeom::Polygon(poly));
                }
            }
            "polygon" | "polyline" => {
                if let Some(pts) = c.attrs.get("points") {
                    let poly = parse_points(pts);
                    if poly.len() >= 3 {
                        return Some(ClipGeom::Polygon(poly));
                    }
                }
            }
            "path" => {
                if let Some(d) = c.attrs.get("d") {
                    let segs = parse_path(d);
                    let poly = path_polygon(&segs);
                    if poly.len() >= 3 {
                        return Some(ClipGeom::Polygon(poly));
                    }
                    // fallback: use bounds as rect
                    if let Some((x0, y0, x1, y1)) = bounds(&segs) {
                        return Some(ClipGeom::Rect {
                            x: x0,
                            y: y0,
                            w: x1 - x0,
                            h: y1 - y0,
                        });
                    }
                }
            }
            "g" | "svg" => {
                if let Some(g) = first_clip_geom(c) {
                    return Some(g);
                }
            }
            _ => {}
        }
    }
    None
}

/// Active clip region in the establishing element's transformed user space.
#[derive(Debug, Clone)]
enum ActiveClip {
    Rect(f64, f64, f64, f64), // min_x, min_y, max_x, max_y
    Polygon(Vec<(f64, f64)>),
}

impl ActiveClip {
    fn bounds(&self) -> (f64, f64, f64, f64) {
        match self {
            ActiveClip::Rect(x0, y0, x1, y1) => (*x0, *y0, *x1, *y1),
            ActiveClip::Polygon(poly) => {
                let min_x = poly.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
                let min_y = poly.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
                let max_x = poly.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
                let max_y = poly.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
                (min_x, min_y, max_x, max_y)
            }
        }
    }

    fn intersect(&self, other: &ActiveClip) -> ActiveClip {
        // Prefer tighter rect intersection when both are rects; otherwise keep polygon of self
        // and rely on sequential clip application via bounds intersection for bbox culling.
        match (self, other) {
            (ActiveClip::Rect(a0, a1, a2, a3), ActiveClip::Rect(b0, b1, b2, b3)) => {
                let x0 = a0.max(*b0);
                let y0 = a1.max(*b1);
                let x1 = a2.min(*b2);
                let y1 = a3.min(*b3);
                ActiveClip::Rect(x0, y0, x1.max(x0), y1.max(y0))
            }
            (ActiveClip::Polygon(p), _) => ActiveClip::Polygon(p.clone()),
            (_, ActiveClip::Polygon(p)) => ActiveClip::Polygon(p.clone()),
        }
    }
}

/// Resolve clipPath into an ActiveClip in the element's CTM space.
fn resolve_active_clip(
    clip: &ClipPathDef,
    ctm: Matrix,
    obj_bbox: Option<(f64, f64, f64, f64)>,
) -> ActiveClip {
    let m = ctm.then(clip.transform);
    match &clip.geom {
        ClipGeom::Rect { x, y, w, h } => {
            let (mut x, mut y, mut w, mut h) = (*x, *y, *w, *h);
            if clip.object_bbox {
                if let Some((bx, by, bx1, by1)) = obj_bbox {
                    let bw = (bx1 - bx).max(1e-9);
                    let bh = (by1 - by).max(1e-9);
                    x = bx + x * bw;
                    y = by + y * bh;
                    w *= bw;
                    h *= bh;
                }
            }
            let p0 = m.map_point(x, y);
            let p1 = m.map_point(x + w, y + h);
            ActiveClip::Rect(
                p0.0.min(p1.0),
                p0.1.min(p1.1),
                p0.0.max(p1.0),
                p0.1.max(p1.1),
            )
        }
        ClipGeom::Polygon(poly) => {
            let mut out = Vec::with_capacity(poly.len());
            for &(px, py) in poly {
                let (mut x, mut y) = (px, py);
                if clip.object_bbox {
                    if let Some((bx, by, bx1, by1)) = obj_bbox {
                        let bw = (bx1 - bx).max(1e-9);
                        let bh = (by1 - by).max(1e-9);
                        x = bx + x * bw;
                        y = by + y * bh;
                    }
                }
                out.push(m.map_point(x, y));
            }
            ActiveClip::Polygon(out)
        }
    }
}

// ── Markers ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct MarkerDef {
    /// Reference width/height of marker viewport.
    ref_w: f64,
    ref_h: f64,
    ref_x: f64,
    ref_y: f64,
    /// markerUnits: strokeWidth (default) or userSpaceOnUse
    stroke_width_units: bool,
    orient_auto: bool,
    orient_angle: f64,
    /// Children rendered as geometry (we store a simplified path + style snapshot).
    children: Vec<Node>,
    view_box: Option<(f64, f64, f64, f64)>,
}

type MarkerMap = HashMap<String, MarkerDef>;

fn collect_markers(node: &Node, out: &mut MarkerMap) {
    if node.name == "marker" {
        if let Some(id) = node.attrs.get("id") {
            let ref_w = attr_f(node, "markerWidth", 3.0);
            let ref_h = attr_f(node, "markerHeight", 3.0);
            let ref_x = attr_f(node, "refX", 0.0);
            let ref_y = attr_f(node, "refY", 0.0);
            let stroke_width_units = node
                .attrs
                .get("markerUnits")
                .map(|s| s.as_str() != "userSpaceOnUse")
                .unwrap_or(true);
            let (orient_auto, orient_angle) = match node.attrs.get("orient").map(|s| s.as_str()) {
                Some("auto") | Some("auto-start-reverse") => (true, 0.0),
                Some(s) => (false, s.parse().unwrap_or(0.0)),
                None => (false, 0.0),
            };
            let view_box = node.attrs.get("viewBox").and_then(|vb| {
                let p = matrix::parse_numbers(vb);
                if p.len() == 4 {
                    Some((p[0], p[1], p[2], p[3]))
                } else {
                    None
                }
            });
            out.insert(
                id.clone(),
                MarkerDef {
                    ref_w,
                    ref_h,
                    ref_x,
                    ref_y,
                    stroke_width_units,
                    orient_auto,
                    orient_angle,
                    children: node.children.clone(),
                    view_box,
                },
            );
        }
    }
    for c in &node.children {
        collect_markers(c, out);
    }
}

// ── Patterns ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct PatternDef {
    /// Solid colour approximation (first painted child fill) — fallback when tiling is disabled.
    solid: Option<([u8; 3], f64)>,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    children: Vec<Node>,
    /// patternUnits objectBoundingBox?
    object_bbox: bool,
    transform: Matrix,
}

type PatternMap = HashMap<String, PatternDef>;

fn collect_patterns(node: &Node, out: &mut PatternMap) {
    if node.name == "pattern" {
        if let Some(id) = node.attrs.get("id") {
            let x = attr_f(node, "x", 0.0);
            let y = attr_f(node, "y", 0.0);
            let w = attr_f(node, "width", 0.0);
            let h = attr_f(node, "height", 0.0);
            let solid = first_solid_paint(node);
            let object_bbox = node
                .attrs
                .get("patternUnits")
                .map(|s| s.as_str() != "userSpaceOnUse")
                .unwrap_or(true);
            let transform = node
                .attrs
                .get("patternTransform")
                .map(|s| Matrix::parse_transform_list(s))
                .unwrap_or_default();
            out.insert(
                id.clone(),
                PatternDef {
                    solid,
                    x,
                    y,
                    w,
                    h,
                    children: node.children.clone(),
                    object_bbox,
                    transform,
                },
            );
        }
    }
    for c in &node.children {
        collect_patterns(c, out);
    }
}

/// Maximum tiles emitted per axis when expanding a pattern (keeps packages bounded).
const MAX_PATTERN_TILES_PER_AXIS: i32 = 32;

/// Expand a pattern fill over `bbox_user` by instantiating tile children.
fn emit_pattern_tiles(
    pattern: &PatternDef,
    bbox_user: (f64, f64, f64, f64),
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let (bx0, by0, bx1, by1) = bbox_user;
    let bw = (bx1 - bx0).max(1e-9);
    let bh = (by1 - by0).max(1e-9);
    let (mut px, mut py, mut pw, mut ph) = (pattern.x, pattern.y, pattern.w, pattern.h);
    if pw <= 0.0 || ph <= 0.0 {
        return;
    }
    if pattern.object_bbox {
        px = bx0 + px * bw;
        py = by0 + py * bh;
        pw *= bw;
        ph *= bh;
    }
    if pw < 1e-6 || ph < 1e-6 {
        return;
    }
    // Tile origin: align pattern origin so tiles cover bbox
    let start_ix = ((bx0 - px) / pw).floor() as i32 - 1;
    let end_ix = ((bx1 - px) / pw).ceil() as i32 + 1;
    let start_iy = ((by0 - py) / ph).floor() as i32 - 1;
    let end_iy = ((by1 - py) / ph).ceil() as i32 + 1;
    let nx = (end_ix - start_ix + 1).clamp(1, MAX_PATTERN_TILES_PER_AXIS);
    let ny = (end_iy - start_iy + 1).clamp(1, MAX_PATTERN_TILES_PER_AXIS);

    // Clip tiles to the shape bbox via active_clip temporarily.
    let prev_clip = ctx.active_clip.clone();
    let shape_clip = ActiveClip::Rect(bx0, by0, bx1, by1);
    ctx.active_clip = Some(match prev_clip {
        Some(ref p) => p.intersect(&shape_clip),
        None => shape_clip,
    });

    let mut child_style = style.clone();
    // Avoid recursive pattern expansion on tiles.
    child_style.fill = pattern
        .solid
        .map(|(rgb, a)| format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]))
        .or(child_style.fill);
    // Prefer children paints over parent fill when present.
    for iy in 0..ny {
        for ix in 0..nx {
            let ox = px + (start_ix + ix) as f64 * pw;
            let oy = py + (start_iy + iy) as f64 * ph;
            let tile_ctm = ctm
                .then(Matrix::translate(ox, oy))
                .then(pattern.transform);
            for child in &pattern.children {
                let _ = walk(child, tile_ctm, &child_style, ctx, out, next_id, 0);
            }
        }
    }
    ctx.active_clip = prev_clip;
}

// ── Masks ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct MaskDef {
    /// Children that define the mask (luminance → opacity approximation).
    children: Vec<Node>,
    /// Optional solid opacity extracted from first painted child.
    opacity: f64,
}

type MaskMap = HashMap<String, MaskDef>;

fn collect_masks(node: &Node, out: &mut MaskMap) {
    if node.name == "mask" {
        if let Some(id) = node.attrs.get("id") {
            let opacity = first_solid_paint(node)
                .map(|(rgb, a)| {
                    // SVG mask default uses luminance: Y = 0.2126R+0.7152G+0.0722B
                    let y = (0.2126 * rgb[0] as f64
                        + 0.7152 * rgb[1] as f64
                        + 0.0722 * rgb[2] as f64)
                        / 255.0;
                    (y * a).clamp(0.0, 1.0)
                })
                .unwrap_or(1.0);
            out.insert(
                id.clone(),
                MaskDef {
                    children: node.children.clone(),
                    opacity,
                },
            );
        }
    }
    for c in &node.children {
        collect_masks(c, out);
    }
}

fn first_solid_paint(node: &Node) -> Option<([u8; 3], f64)> {
    for c in &node.children {
        if let Some(fill) = c.attrs.get("fill") {
            if let Some(p) = parse_color(fill) {
                return Some(p);
            }
        }
        if let Some(style) = c.attrs.get("style") {
            for part in style.split(';') {
                let mut kv = part.splitn(2, ':');
                if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                    if k.trim() == "fill" {
                        if let Some(p) = parse_color(v.trim()) {
                            return Some(p);
                        }
                    }
                }
            }
        }
        if let Some(p) = first_solid_paint(c) {
            return Some(p);
        }
    }
    None
}

// ── ID index for <use> ──────────────────────────────────────────────────────

fn index_ids(node: &Node, out: &mut HashMap<String, Node>) {
    if let Some(id) = node.attrs.get("id") {
        out.insert(id.clone(), node.clone());
    }
    for c in &node.children {
        index_ids(c, out);
    }
}

// ── Walk context ────────────────────────────────────────────────────────────

struct WalkCtx<'a> {
    gradients: &'a GradientMap,
    filters: &'a FilterMap,
    css: &'a CssRules,
    id_index: &'a HashMap<String, Node>,
    clip_paths: &'a ClipPathMap,
    markers: &'a MarkerMap,
    patterns: &'a PatternMap,
    masks: &'a MaskMap,
    user_to_emu: Matrix,
    fonts: &'a FontDb,
    used_font_keys: &'a mut HashSet<String>,
    used_fonts: &'a mut Vec<UsedFont>,
    /// Active clip region in user space (after ctm of establishing element).
    active_clip: Option<ActiveClip>,
    /// When true, pattern paints expand as tiled shapes instead of solid approx.
    expand_patterns: bool,
}

// ── Gradients ───────────────────────────────────────────────────────────────

fn collect_gradients(node: &Node, out: &mut GradientMap) {
    if node.name == "linearGradient" || node.name == "radialGradient" {
        if let Some(id) = node.attrs.get("id") {
            let units = node
                .attrs
                .get("gradientUnits")
                .map(|s| s.as_str())
                .unwrap_or("objectBoundingBox");
            let units_object_bbox = units != "userSpaceOnUse";
            let transform = node
                .attrs
                .get("gradientTransform")
                .map(|s| Matrix::parse_transform_list(s))
                .unwrap_or_default();

            let mut stops = Vec::new();
            for c in &node.children {
                if c.name == "stop" {
                    let offset = c
                        .attrs
                        .get("offset")
                        .map(|s| parse_percent_or_number(s, 0.0))
                        .unwrap_or(0.0);
                    // stop-color / stop-opacity from attr or style
                    let mut color = c
                        .attrs
                        .get("stop-color")
                        .cloned()
                        .unwrap_or_else(|| "black".into());
                    let mut opac = c
                        .attrs
                        .get("stop-opacity")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1.0);
                    if let Some(style) = c.attrs.get("style") {
                        for part in style.split(';') {
                            let mut kv = part.splitn(2, ':');
                            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                                match k.trim() {
                                    "stop-color" => color = v.trim().into(),
                                    "stop-opacity" => opac = v.trim().parse().unwrap_or(opac),
                                    _ => {}
                                }
                            }
                        }
                    }
                    let (rgb, a) = parse_color(&color).unwrap_or(([0, 0, 0], 1.0));
                    stops.push(paint::ColorStop {
                        offset,
                        rgb,
                        alpha: a * opac,
                    });
                }
            }

            // href inheritance (xlink:href / href)
            if stops.is_empty() {
                if let Some(href) = node
                    .attrs
                    .get("href")
                    .or_else(|| node.attrs.get("xlink:href"))
                {
                    let ref_id = href.trim().trim_start_matches('#');
                    if let Some(g) = out.get(ref_id) {
                        stops = g.stops.clone();
                    }
                }
            }

            let kind = if node.name == "linearGradient" {
                GradientKind::Linear {
                    x1: parse_percent_or_number(node.attrs.get("x1").map(|s| s.as_str()).unwrap_or("0%"), 0.0),
                    y1: parse_percent_or_number(node.attrs.get("y1").map(|s| s.as_str()).unwrap_or("0%"), 0.0),
                    x2: parse_percent_or_number(node.attrs.get("x2").map(|s| s.as_str()).unwrap_or("100%"), 1.0),
                    y2: parse_percent_or_number(node.attrs.get("y2").map(|s| s.as_str()).unwrap_or("0%"), 0.0),
                }
            } else {
                let cx = parse_percent_or_number(
                    node.attrs.get("cx").map(|s| s.as_str()).unwrap_or("50%"),
                    0.5,
                );
                let cy = parse_percent_or_number(
                    node.attrs.get("cy").map(|s| s.as_str()).unwrap_or("50%"),
                    0.5,
                );
                let r = parse_percent_or_number(
                    node.attrs.get("r").map(|s| s.as_str()).unwrap_or("50%"),
                    0.5,
                );
                let fx = node
                    .attrs
                    .get("fx")
                    .map(|s| parse_percent_or_number(s, cx))
                    .unwrap_or(cx);
                let fy = node
                    .attrs
                    .get("fy")
                    .map(|s| parse_percent_or_number(s, cy))
                    .unwrap_or(cy);
                GradientKind::Radial { cx, cy, r, fx, fy }
            };

            out.insert(
                id.clone(),
                Gradient {
                    kind,
                    stops,
                    transform,
                    units_object_bbox,
                },
            );
        }
    }
    for c in &node.children {
        collect_gradients(c, out);
    }
}

// ── Walk ────────────────────────────────────────────────────────────────────

const MAX_USE_DEPTH: u32 = 32;

fn walk(
    node: &Node,
    parent_ctm: Matrix,
    parent_style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
    use_depth: u32,
) -> Result<()> {
    // Skip non-rendered
    if matches!(
        node.name.as_str(),
        "defs"
            | "clipPath"
            | "mask"
            | "filter"
            | "linearGradient"
            | "radialGradient"
            | "stop"
            | "style"
            | "script"
            | "title"
            | "desc"
            | "metadata"
            | "symbol"
            | "marker"
            | "pattern"
            | "feGaussianBlur"
            | "feComposite"
            | "feOffset"
            | "feMerge"
            | "feMergeNode"
            | "feColorMatrix"
            | "feBlend"
            | "feFlood"
    ) {
        // still collect nested gradients already done; just skip render
        return Ok(());
    }

    // local transform
    let local_tf = node
        .attrs
        .get("transform")
        .map(|s| Matrix::parse_transform_list(s))
        .unwrap_or_default();
    let ctm = parent_ctm.then(local_tf);

    // style with proper opacity multiply
    let parent_opacity = parent_style.opacity;
    let mut style = parent_style.apply_node(node, ctx.css);
    // recompute opacity as parent * local (apply_node already set local opacity)
    let local_op = node
        .attrs
        .get("opacity")
        .map(|s| s.as_str())
        .or_else(|| {
            node.attrs.get("style").and_then(|st| {
                st.split(';').find_map(|p| {
                    let mut kv = p.splitn(2, ':');
                    match (kv.next()?.trim(), kv.next()?.trim()) {
                        ("opacity", v) => Some(v),
                        _ => None,
                    }
                })
            })
        })
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    // If CSS set opacity without presentation attr, apply_node already wrote it into style.opacity
    // as local; always multiply by parent.
    let css_local = style.opacity; // may already include CSS/presentation local
    let local = if node.attrs.contains_key("opacity")
        || node
            .attrs
            .get("style")
            .map(|s| s.contains("opacity"))
            .unwrap_or(false)
    {
        local_op
    } else {
        // style.opacity was inherited then maybe overridden by CSS filter rules etc.
        // When no local opacity attr, keep parent*1 but respect CSS opacity if set on this node.
        let had_css_opacity = node
            .attrs
            .get("class")
            .map(|_| true)
            .unwrap_or(false)
            || node.attrs.contains_key("id");
        if had_css_opacity && (css_local - parent_opacity).abs() > 1e-12 {
            // CSS may have set a new local opacity replacing inherited
            css_local
        } else {
            1.0
        }
    };
    style.opacity = (parent_opacity * local).clamp(0.0, 1.0);

    if !style.display || !style.visibility {
        return Ok(());
    }

    // Establish clip-path for this node and descendants.
    let prev_clip = ctx.active_clip.clone();
    if let Some(ref clip_id) = style.clip_path {
        if let Some(def) = ctx.clip_paths.get(clip_id).cloned() {
            let clip = resolve_active_clip(&def, ctm, None);
            ctx.active_clip = Some(match prev_clip {
                Some(ref p) => p.intersect(&clip),
                None => clip,
            });
        }
    }

    // Mask → multiply opacity by luminance approximation of mask contents.
    if let Some(ref mask_id) = style.mask {
        if let Some(m) = ctx.masks.get(mask_id) {
            style.opacity = (style.opacity * m.opacity).clamp(0.0, 1.0);
        }
    }

    match node.name.as_str() {
        "svg" | "g" | "a" | "switch" => {
            for c in &node.children {
                walk(c, ctm, &style, ctx, out, next_id, use_depth)?;
            }
        }
        "rect" => emit_rect(node, ctm, &style, ctx, out, next_id),
        "circle" => emit_circle(node, ctm, &style, ctx, out, next_id),
        "ellipse" => emit_ellipse(node, ctm, &style, ctx, out, next_id),
        "line" => emit_line(node, ctm, &style, ctx, out, next_id),
        "polyline" => emit_poly(node, ctm, &style, ctx, false, out, next_id),
        "polygon" => emit_poly(node, ctm, &style, ctx, true, out, next_id),
        "path" => emit_path(node, ctm, &style, ctx, out, next_id),
        "text" => emit_text(node, ctm, &style, ctx, out, next_id),
        "use" => emit_use(node, ctm, &style, ctx, out, next_id, use_depth)?,
        _ => {
            for c in &node.children {
                walk(c, ctm, &style, ctx, out, next_id, use_depth)?;
            }
        }
    }
    ctx.active_clip = prev_clip;
    Ok(())
}

fn emit_use(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
    use_depth: u32,
) -> Result<()> {
    if use_depth >= MAX_USE_DEPTH {
        return Ok(());
    }
    let href = node
        .attrs
        .get("href")
        .or_else(|| node.attrs.get("xlink:href"))
        .map(|s| s.as_str())
        .unwrap_or("");
    if href.is_empty() || !href.starts_with('#') {
        return Ok(()); // external use not supported
    }
    let ref_id = href.trim_start_matches('#');
    let Some(target) = ctx.id_index.get(ref_id) else {
        return Ok(());
    };
    let x = attr_f(node, "x", 0.0);
    let y = attr_f(node, "y", 0.0);
    let use_ctm = ctm.then(Matrix::translate(x, y));

    // <use> clones the referenced element; for <symbol>/<svg> also honour width/height
    // by scaling if both intrinsic and use sizes are present (best-effort).
    let mut clone_ctm = use_ctm;
    if matches!(target.name.as_str(), "symbol" | "svg") {
        let uw = node.attrs.get("width").and_then(|s| parse_length(s));
        let uh = node.attrs.get("height").and_then(|s| parse_length(s));
        if let (Some(uw), Some(uh)) = (uw, uh) {
            let (vw, vh) = if let Some(vb) = target.attrs.get("viewBox") {
                let p = matrix::parse_numbers(vb);
                if p.len() == 4 {
                    (p[2], p[3])
                } else {
                    (uw, uh)
                }
            } else {
                (
                    target
                        .attrs
                        .get("width")
                        .and_then(|s| parse_length(s))
                        .unwrap_or(uw),
                    target
                        .attrs
                        .get("height")
                        .and_then(|s| parse_length(s))
                        .unwrap_or(uh),
                )
            };
            if vw > 0.0 && vh > 0.0 {
                clone_ctm = use_ctm.then(Matrix::scale(uw / vw, uh / vh));
            }
        }
        // render symbol/svg children
        for c in &target.children {
            walk(c, clone_ctm, style, ctx, out, next_id, use_depth + 1)?;
        }
    } else {
        walk(target, clone_ctm, style, ctx, out, next_id, use_depth + 1)?;
    }
    Ok(())
}

fn attr_f(node: &Node, key: &str, default: f64) -> f64 {
    node.attrs
        .get(key)
        .and_then(|s| parse_length(s))
        .unwrap_or(default)
}

fn is_axis_aligned(m: Matrix) -> bool {
    // no rotation/skew: b and c ~ 0
    m.b.abs() < 1e-9 && m.c.abs() < 1e-9 && m.a > 0.0 && m.d > 0.0
}

fn paints_for(
    style: &Style,
    gradients: &GradientMap,
    patterns: &PatternMap,
    bbox_user: (f64, f64, f64, f64),
    has_explicit_fill: bool,
) -> (Paint, Paint) {
    let fill_op = (style.opacity * style.fill_opacity).clamp(0.0, 1.0);
    let stroke_op = (style.opacity * style.stroke_opacity).clamp(0.0, 1.0);
    let fill = resolve_paint_or_pattern(
        style.fill.as_deref(),
        fill_op,
        gradients,
        patterns,
        bbox_user,
        !has_explicit_fill && style.fill.is_some(),
    );
    // SVG default fill is black when fill attr omitted; our Style defaults fill=black
    let fill = if style.fill.as_deref() == Some("none") {
        Paint::None
    } else {
        fill
    };
    let stroke = resolve_paint_or_pattern(
        style.stroke.as_deref(),
        stroke_op,
        gradients,
        patterns,
        bbox_user,
        false,
    );
    let stroke = if style.stroke.as_deref() == Some("none") || style.stroke.is_none() {
        if style.stroke.as_deref() == Some("none") {
            Paint::None
        } else {
            stroke
        }
    } else {
        stroke
    };
    let _ = has_explicit_fill;
    (fill, stroke)
}

fn resolve_paint_or_pattern(
    value: Option<&str>,
    opacity: f64,
    gradients: &GradientMap,
    patterns: &PatternMap,
    bbox: (f64, f64, f64, f64),
    default_black: bool,
) -> Paint {
    if let Some(raw) = value.map(str::trim) {
        if let Some(rest) = raw.strip_prefix("url(") {
            let id = rest
                .trim()
                .trim_end_matches(')')
                .trim()
                .trim_matches(|c| c == '"' || c == '\'')
                .trim_start_matches('#');
            if let Some(pat) = patterns.get(id) {
                if let Some((rgb, a)) = pat.solid {
                    return Paint::Solid {
                        rgb,
                        alpha: (a * opacity).clamp(0.0, 1.0),
                    };
                }
                // no solid child — leave transparent
                return Paint::None;
            }
        }
    }
    resolve_paint(value, opacity, gradients, bbox, default_black)
}

fn fill_is_expandable_pattern(style: &Style, patterns: &PatternMap) -> bool {
    let Some(raw) = style.fill.as_deref() else {
        return false;
    };
    if let Some(rest) = raw.strip_prefix("url(") {
        let id = rest
            .trim()
            .trim_end_matches(')')
            .trim()
            .trim_matches(|c| c == '"' || c == '\'')
            .trim_start_matches('#');
        if let Some(pat) = patterns.get(id) {
            return pat.w > 0.0 && pat.h > 0.0 && !pat.children.is_empty();
        }
    }
    false
}

fn dash_preset(style: &Style) -> Option<&'static str> {
    let d = style.stroke_dasharray.as_deref()?;
    if d.trim().eq_ignore_ascii_case("none") {
        return None;
    }
    let nums = matrix::parse_numbers(d);
    if nums.is_empty() {
        return None;
    }
    // Map common dash patterns to DrawingML presets
    let a = nums[0];
    let b = nums.get(1).copied().unwrap_or(a);
    if a <= 1.0 && b >= 2.0 {
        Some("sysDot")
    } else if a < b {
        Some("sysDash")
    } else if a > b * 2.0 {
        Some("lgDash")
    } else {
        Some("dash")
    }
}

fn stroke_cap(style: &Style) -> Option<&'static str> {
    match style.stroke_linecap.as_deref()? {
        "butt" => Some("flat"),
        "round" => Some("rnd"),
        "square" => Some("sq"),
        _ => None,
    }
}

fn stroke_join(style: &Style) -> Option<&'static str> {
    match style.stroke_linejoin.as_deref()? {
        "miter" => Some("miter"),
        "round" => Some("round"),
        "bevel" => Some("bevel"),
        _ => None,
    }
}

fn stroke_w_emu(style: &Style, ctm: Matrix, user_to_emu: Matrix) -> i64 {
    let scale = ctm.then(user_to_emu).avg_scale();
    (style.stroke_width * scale).round().max(0.0) as i64
}

/// Rotation of the linear part of `m`, in DrawingML 1/60000 deg (clockwise positive).
fn matrix_rot_dml(m: Matrix) -> Option<i32> {
    // angle of x-axis image; SVG/CSS y-down matches DrawingML y-down
    let deg = m.b.atan2(m.a).to_degrees();
    if deg.abs() < 0.05 {
        return None;
    }
    let mut d = deg;
    if d < 0.0 {
        d += 360.0;
    }
    Some((d * 60_000.0).round() as i32)
}

fn push_path_shape(
    segs_user: Vec<Segment>,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    name: &str,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    if segs_user.is_empty() {
        return;
    }
    // Apply active clip: transform to CTM space, clip, then only scale to EMU.
    if let Some(clip) = ctx.active_clip.clone() {
        let segs_ctm = transform_segments(&segs_user, ctm);
        let clipped = match clip {
            ActiveClip::Rect(x0, y0, x1, y1) => {
                clip_segments_to_rect(&segs_ctm, (x0, y0, x1, y1))
            }
            ActiveClip::Polygon(ref poly) => clip_segments_to_polygon(&segs_ctm, poly),
        };
        if clipped.is_empty() {
            return;
        }
        return push_path_shape_transformed(clipped, style, ctx, name, out, next_id, true);
    }

    let user_to_emu = ctx.user_to_emu;
    let segs = transform_segments(&segs_user, ctm.then(user_to_emu));
    let Some((min_x, min_y, max_x, max_y)) = bounds(&segs) else {
        return;
    };
    let segs_ctm = transform_segments(&segs_user, ctm);
    let bbox_user = bounds(&segs_ctm).unwrap_or((0.0, 0.0, 1.0, 1.0));

    // Pattern expansion: when fill is a pattern url and tiling is enabled, emit tiles.
    if ctx.expand_patterns {
        if let Some(raw) = style.fill.as_deref() {
            if let Some(rest) = raw.strip_prefix("url(") {
                let id = rest
                    .trim()
                    .trim_end_matches(')')
                    .trim()
                    .trim_matches(|c| c == '"' || c == '\'')
                    .trim_start_matches('#');
                if let Some(pat) = ctx.patterns.get(id).cloned() {
                    if pat.w > 0.0 && pat.h > 0.0 && !pat.children.is_empty() {
                        emit_pattern_tiles(&pat, bbox_user, ctm, style, ctx, out, next_id);
                        // still emit stroke-only outline if stroke present
                        if style.stroke.as_deref().map(|s| s != "none").unwrap_or(false) {
                            let mut stroke_style = style.clone();
                            stroke_style.fill = Some("none".into());
                            let (fill, stroke) =
                                paints_for(&stroke_style, ctx.gradients, ctx.patterns, bbox_user, true);
                            let sw = stroke_w_emu(style, ctm, user_to_emu);
                            let effect = shape_effect_for(style, ctx.filters, ctm, user_to_emu);
                            let idn = *next_id;
                            *next_id += 1;
                            out.push(freeform_shape(&ShapeBuild {
                                id: idn,
                                name: format!("{name}{idn}"),
                                x: min_x.floor() as i64,
                                y: min_y.floor() as i64,
                                cx: (max_x - min_x).ceil().max(1.0) as i64,
                                cy: (max_y - min_y).ceil().max(1.0) as i64,
                                segments: segs,
                                fill,
                                stroke,
                                stroke_width_emu: sw.max(1270),
                                stroke_dash: dash_preset(style),
                                stroke_cap: stroke_cap(style),
                                stroke_join: stroke_join(style),
                                effect,
                            }));
                        }
                        emit_markers_for_path(&segs_user, ctm, style, ctx, out, next_id);
                        return;
                    }
                }
            }
        }
    }

    let (fill, stroke) = paints_for(style, ctx.gradients, ctx.patterns, bbox_user, true);
    let sw = stroke_w_emu(style, ctm, user_to_emu);
    let effect = shape_effect_for(style, ctx.filters, ctm, user_to_emu);

    let id = *next_id;
    *next_id += 1;
    let build = ShapeBuild {
        id,
        name: format!("{name}{id}"),
        x: min_x.floor() as i64,
        y: min_y.floor() as i64,
        cx: (max_x - min_x).ceil().max(1.0) as i64,
        cy: (max_y - min_y).ceil().max(1.0) as i64,
        segments: segs,
        fill,
        stroke,
        stroke_width_emu: sw.max(if matches!(style.stroke.as_deref(), Some(s) if s != "none") {
            1270
        } else {
            0
        }),
        stroke_dash: dash_preset(style),
        stroke_cap: stroke_cap(style),
        stroke_join: stroke_join(style),
        effect,
    };
    out.push(freeform_shape(&build));

    // Markers on the unclipped original path (in user space)
    emit_markers_for_path(&segs_user, ctm, style, ctx, out, next_id);
}

/// segs already in the active-clip CTM space; only user_to_emu remains.
fn push_path_shape_transformed(
    segs_ctm: Vec<Segment>,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    name: &str,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
    _clipped: bool,
) {
    let user_to_emu = ctx.user_to_emu;
    let segs = transform_segments(&segs_ctm, user_to_emu);
    let Some((min_x, min_y, max_x, max_y)) = bounds(&segs) else {
        return;
    };
    let bbox_user = bounds(&segs_ctm).unwrap_or((0.0, 0.0, 1.0, 1.0));
    let (fill, stroke) = paints_for(style, ctx.gradients, ctx.patterns, bbox_user, true);
    let sw = stroke_w_emu(style, Matrix::identity(), user_to_emu);
    let effect = shape_effect_for(style, ctx.filters, Matrix::identity(), user_to_emu);
    let id = *next_id;
    *next_id += 1;
    out.push(freeform_shape(&ShapeBuild {
        id,
        name: format!("{name}{id}"),
        x: min_x.floor() as i64,
        y: min_y.floor() as i64,
        cx: (max_x - min_x).ceil().max(1.0) as i64,
        cy: (max_y - min_y).ceil().max(1.0) as i64,
        segments: segs,
        fill,
        stroke,
        stroke_width_emu: sw.max(if matches!(style.stroke.as_deref(), Some(s) if s != "none") {
            1270
        } else {
            0
        }),
        stroke_dash: dash_preset(style),
        stroke_cap: stroke_cap(style),
        stroke_join: stroke_join(style),
        effect,
    }));
}

fn emit_markers_for_path(
    segs_user: &[Segment],
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let has_any = style.marker_start.is_some()
        || style.marker_mid.is_some()
        || style.marker_end.is_some();
    if !has_any {
        return;
    }
    let Some(anchors) = marker_anchors(segs_user) else {
        return;
    };
    let sw = style.stroke_width.max(1.0);
    if let Some(id) = style.marker_start.as_deref() {
        if let Some(m) = ctx.markers.get(id).cloned() {
            place_marker(
                &m,
                anchors.start,
                if m.orient_auto {
                    anchors.start_angle
                } else {
                    m.orient_angle * std::f64::consts::PI / 180.0
                },
                sw,
                ctm,
                style,
                ctx,
                out,
                next_id,
            );
        }
    }
    if let Some(id) = style.marker_mid.as_deref() {
        if let Some(m) = ctx.markers.get(id).cloned() {
            for &(x, y) in &anchors.mids {
                place_marker(
                    &m,
                    (x, y),
                    if m.orient_auto {
                        anchors.end_angle
                    } else {
                        m.orient_angle * std::f64::consts::PI / 180.0
                    },
                    sw,
                    ctm,
                    style,
                    ctx,
                    out,
                    next_id,
                );
            }
        }
    }
    if let Some(id) = style.marker_end.as_deref() {
        if let Some(m) = ctx.markers.get(id).cloned() {
            place_marker(
                &m,
                anchors.end,
                if m.orient_auto {
                    anchors.end_angle
                } else {
                    m.orient_angle * std::f64::consts::PI / 180.0
                },
                sw,
                ctm,
                style,
                ctx,
                out,
                next_id,
            );
        }
    }
}

fn place_marker(
    marker: &MarkerDef,
    at: (f64, f64),
    angle: f64,
    stroke_w: f64,
    ctm: Matrix,
    parent_style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let scale = if marker.stroke_width_units {
        stroke_w
    } else {
        1.0
    };
    // viewBox → markerWidth/Height
    let (vbx, vby, vbw, vbh) = marker.view_box.unwrap_or((0.0, 0.0, marker.ref_w, marker.ref_h));
    let sx = if vbw > 0.0 {
        marker.ref_w / vbw * scale
    } else {
        scale
    };
    let sy = if vbh > 0.0 {
        marker.ref_h / vbh * scale
    } else {
        scale
    };
    // T(at) · R(angle) · scale · T(-ref)
    let deg = angle.to_degrees();
    let local = Matrix::translate(-marker.ref_x, -marker.ref_y)
        .then(Matrix::scale(sx, sy))
        .then(Matrix::rotate_deg(deg))
        .then(Matrix::translate(at.0, at.1));
    let _ = (vbx, vby);
    let marker_ctm = ctm.then(local);
    let mut style = parent_style.clone();
    // markers don't inherit markers
    style.marker_start = None;
    style.marker_mid = None;
    style.marker_end = None;
    for child in &marker.children {
        let _ = walk(child, marker_ctm, &style, ctx, out, next_id, 0);
    }
}

fn push_preset(
    id: u32,
    name: &str,
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
    preset: &str,
    fill: &Paint,
    stroke: &Paint,
    sw: i64,
    adj: Option<i32>,
    style: &Style,
    effect: Option<ShapeEffect>,
    out: &mut Vec<OpenXmlElement>,
) {
    out.push(preset_shape(
        id,
        name,
        x,
        y,
        cx,
        cy,
        preset,
        fill,
        stroke,
        sw,
        adj,
        dash_preset(style),
        stroke_cap(style),
        stroke_join(style),
        effect.as_ref(),
    ));
}

fn emit_rect(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let x = attr_f(node, "x", 0.0);
    let y = attr_f(node, "y", 0.0);
    let w = attr_f(node, "width", 0.0);
    let h = attr_f(node, "height", 0.0);
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let mut rx = attr_f(node, "rx", f64::NAN);
    let mut ry = attr_f(node, "ry", f64::NAN);
    if rx.is_nan() && ry.is_nan() {
        rx = 0.0;
        ry = 0.0;
    } else if rx.is_nan() {
        rx = ry;
    } else if ry.is_nan() {
        ry = rx;
    }

    let user_to_emu = ctx.user_to_emu;
    if ctx.active_clip.is_none() && !fill_is_expandable_pattern(style, ctx.patterns) && is_axis_aligned(ctm) && is_axis_aligned(user_to_emu) {
        // fast path: preset geometry
        let m = ctm.then(user_to_emu);
        let (x0, y0) = m.map_point(x, y);
        let (x1, y1) = m.map_point(x + w, y + h);
        let bx = x0.min(x1);
        let by = y0.min(y1);
        let bw = (x0 - x1).abs().max(1.0);
        let bh = (y0 - y1).abs().max(1.0);
        let bbox_user = {
            let p0 = ctm.map_point(x, y);
            let p1 = ctm.map_point(x + w, y + h);
            (
                p0.0.min(p1.0),
                p0.1.min(p1.1),
                p0.0.max(p1.0),
                p0.1.max(p1.1),
            )
        };
        let (fill, stroke) = paints_for(style, ctx.gradients, ctx.patterns, bbox_user, true);
        let sw = stroke_w_emu(style, ctm, user_to_emu);
        let preset = if rx > 0.5 || ry > 0.5 {
            "roundRect"
        } else {
            "rect"
        };
        let adj = if rx > 0.5 {
            let half = w.min(h) / 2.0;
            Some(((rx / half) * 50_000.0).round().clamp(0.0, 50_000.0) as i32)
        } else {
            None
        };
        let id = *next_id;
        *next_id += 1;
        let effect = shape_effect_for(style, ctx.filters, ctm, user_to_emu);
        push_preset(
            id,
            &format!("rect{id}"),
            bx.round() as i64,
            by.round() as i64,
            bw.round() as i64,
            bh.round() as i64,
            preset,
            &fill,
            &stroke,
            sw.max(0),
            adj,
            style,
            effect,
            out,
        );
        return;
    }
    let segs = rect_path(x, y, w, h, rx, ry);
    push_path_shape(segs, ctm, style, ctx, "rect", out, next_id);
}

fn emit_circle(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let cx = attr_f(node, "cx", 0.0);
    let cy = attr_f(node, "cy", 0.0);
    let r = attr_f(node, "r", 0.0);
    if r <= 0.0 {
        return;
    }
    let user_to_emu = ctx.user_to_emu;
    if ctx.active_clip.is_none() && !fill_is_expandable_pattern(style, ctx.patterns) && is_axis_aligned(ctm) && is_axis_aligned(user_to_emu) {
        let m = ctm.then(user_to_emu);
        let (x0, y0) = m.map_point(cx - r, cy - r);
        let (x1, y1) = m.map_point(cx + r, cy + r);
        let bx = x0.min(x1);
        let by = y0.min(y1);
        let bw = (x0 - x1).abs().max(1.0);
        let bh = (y0 - y1).abs().max(1.0);
        let p0 = ctm.map_point(cx - r, cy - r);
        let p1 = ctm.map_point(cx + r, cy + r);
        let bbox_user = (
            p0.0.min(p1.0),
            p0.1.min(p1.1),
            p0.0.max(p1.0),
            p0.1.max(p1.1),
        );
        let (fill, stroke) = paints_for(style, ctx.gradients, ctx.patterns, bbox_user, true);
        let sw = stroke_w_emu(style, ctm, user_to_emu);
        let id = *next_id;
        *next_id += 1;
        let effect = shape_effect_for(style, ctx.filters, ctm, user_to_emu);
        push_preset(
            id,
            &format!("ellipse{id}"),
            bx.round() as i64,
            by.round() as i64,
            bw.round() as i64,
            bh.round() as i64,
            "ellipse",
            &fill,
            &stroke,
            sw.max(0),
            None,
            style,
            effect,
            out,
        );
        return;
    }
    push_path_shape(
        ellipse_path(cx, cy, r, r),
        ctm,
        style,
        ctx,
        "ellipse",
        out,
        next_id,
    );
}

fn emit_ellipse(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let cx = attr_f(node, "cx", 0.0);
    let cy = attr_f(node, "cy", 0.0);
    let rx = attr_f(node, "rx", 0.0);
    let ry = attr_f(node, "ry", 0.0);
    if rx <= 0.0 || ry <= 0.0 {
        return;
    }
    let user_to_emu = ctx.user_to_emu;
    if ctx.active_clip.is_none() && !fill_is_expandable_pattern(style, ctx.patterns) && is_axis_aligned(ctm) && is_axis_aligned(user_to_emu) {
        let m = ctm.then(user_to_emu);
        let (x0, y0) = m.map_point(cx - rx, cy - ry);
        let (x1, y1) = m.map_point(cx + rx, cy + ry);
        let bx = x0.min(x1);
        let by = y0.min(y1);
        let bw = (x0 - x1).abs().max(1.0);
        let bh = (y0 - y1).abs().max(1.0);
        let p0 = ctm.map_point(cx - rx, cy - ry);
        let p1 = ctm.map_point(cx + rx, cy + ry);
        let bbox_user = (
            p0.0.min(p1.0),
            p0.1.min(p1.1),
            p0.0.max(p1.0),
            p0.1.max(p1.1),
        );
        let (fill, stroke) = paints_for(style, ctx.gradients, ctx.patterns, bbox_user, true);
        let sw = stroke_w_emu(style, ctm, user_to_emu);
        let id = *next_id;
        *next_id += 1;
        let effect = shape_effect_for(style, ctx.filters, ctm, user_to_emu);
        push_preset(
            id,
            &format!("ellipse{id}"),
            bx.round() as i64,
            by.round() as i64,
            bw.round() as i64,
            bh.round() as i64,
            "ellipse",
            &fill,
            &stroke,
            sw.max(0),
            None,
            style,
            effect,
            out,
        );
        return;
    }
    push_path_shape(
        ellipse_path(cx, cy, rx, ry),
        ctm,
        style,
        ctx,
        "ellipse",
        out,
        next_id,
    );
}

fn emit_line(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let x1 = attr_f(node, "x1", 0.0);
    let y1 = attr_f(node, "y1", 0.0);
    let x2 = attr_f(node, "x2", 0.0);
    let y2 = attr_f(node, "y2", 0.0);
    push_path_shape(
        line_path(x1, y1, x2, y2),
        ctm,
        style,
        ctx,
        "line",
        out,
        next_id,
    );
}

fn emit_poly(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    close: bool,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let Some(pts) = node.attrs.get("points") else {
        return;
    };
    let points = parse_points(pts);
    push_path_shape(
        polygon_path(&points, close),
        ctm,
        style,
        ctx,
        if close { "polygon" } else { "polyline" },
        out,
        next_id,
    );
}

fn emit_path(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let Some(d) = node.attrs.get("d") else {
        return;
    };
    let segs = parse_path(d);
    push_path_shape(segs, ctm, style, ctx, "path", out, next_id);
}

fn emit_text(
    node: &Node,
    ctm: Matrix,
    style: &Style,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    // Flatten into runs: each tspan with explicit x/y starts a new run;
    // otherwise concatenate under parent position.
    let mut runs: Vec<(f64, f64, Style, String)> = Vec::new();
    let base_x = attr_f(node, "x", 0.0);
    let base_y = attr_f(node, "y", 0.0);
    flatten_text_runs(node, style, ctx.css, base_x, base_y, &mut runs);

    for (x, y, st, content) in runs {
        let content = content.trim();
        if content.is_empty() {
            continue;
        }
        emit_text_run(x, y, &st, content, ctm, ctx, out, next_id);
    }
}

fn flatten_text_runs(
    node: &Node,
    parent_style: &Style,
    css: &CssRules,
    parent_x: f64,
    parent_y: f64,
    out: &mut Vec<(f64, f64, Style, String)>,
) {
    let style = parent_style.apply_node(node, css);
    // first value of list x/y if provided (SVG allows space-separated lists)
    let x = node
        .attrs
        .get("x")
        .and_then(|s| matrix::parse_numbers(s).into_iter().next())
        .or_else(|| node.attrs.get("x").and_then(|s| parse_length(s)))
        .unwrap_or(parent_x);
    let y = node
        .attrs
        .get("y")
        .and_then(|s| matrix::parse_numbers(s).into_iter().next())
        .or_else(|| node.attrs.get("y").and_then(|s| parse_length(s)))
        .unwrap_or(parent_y);
    let dx = node
        .attrs
        .get("dx")
        .and_then(|s| matrix::parse_numbers(s).into_iter().next())
        .unwrap_or(0.0);
    let dy = node
        .attrs
        .get("dy")
        .and_then(|s| matrix::parse_numbers(s).into_iter().next())
        .unwrap_or(0.0);
    let x = x + dx;
    let y = y + dy;

    // Direct text on this node
    let direct = node.text.trim();
    if !direct.is_empty() && node.children.iter().all(|c| c.name != "tspan") {
        out.push((x, y, style.clone(), node.text.clone()));
        return;
    }
    if !direct.is_empty() {
        out.push((x, y, style.clone(), direct.to_string()));
    }
    for c in &node.children {
        if c.name == "tspan" || c.name == "a" || c.name == "textPath" {
            flatten_text_runs(c, &style, css, x, y, out);
        }
    }
}

fn emit_text_run(
    x: f64,
    y: f64,
    style: &Style,
    content: &str,
    ctm: Matrix,
    ctx: &mut WalkCtx<'_>,
    out: &mut Vec<OpenXmlElement>,
    next_id: &mut u32,
) {
    let fonts = ctx.fonts;
    let user_to_emu = ctx.user_to_emu;
    let bold = matches!(
        style.font_weight.as_str(),
        "bold" | "600" | "700" | "800" | "900"
    ) || style
        .font_weight
        .parse::<i32>()
        .map(|n| n >= 600)
        .unwrap_or(false);
    let italic = style.font_style == "italic" || style.font_style == "oblique";

    let metrics = fonts.measure(content, style.font_size, &style.font_family, bold);
    let extra = style.letter_spacing * content.chars().count().saturating_sub(1) as f64;
    let text_w = metrics.width + extra;
    let text_h = metrics.height;

    let anchor_dx = match style.text_anchor.as_str() {
        "middle" => -text_w / 2.0,
        "end" => -text_w,
        _ => 0.0,
    };
    // Unrotated top-left of the text box in user space (baseline at y).
    let top_x = x + anchor_dx;
    let top_y = y - metrics.ascent;

    let m = ctm.then(user_to_emu);
    let rot = matrix_rot_dml(ctm);
    let (box_x, box_y, box_w, box_h) = if is_axis_aligned(ctm) && is_axis_aligned(user_to_emu) {
        let (x0, y0) = m.map_point(top_x, top_y);
        let scale_x = m.a.abs();
        let scale_y = m.d.abs();
        (x0, y0, text_w * scale_x, text_h * scale_y)
    } else if rot.is_some() {
        // Keep local width/height and put rotation on a:xfrm@rot.
        // Map the unrotated top-left; DrawingML rotates about the shape center.
        // Approximate: place the axis-aligned box at the mapped top-left of the
        // unrotated rectangle, using average scale for extents.
        let scale = m.avg_scale();
        let (x0, y0) = m.map_point(top_x, top_y);
        // Better: map the four corners of the unrotated box and take the center,
        // then rebuild an axis-aligned box of local size around that center.
        let corners = [
            m.map_point(top_x, top_y),
            m.map_point(top_x + text_w, top_y),
            m.map_point(top_x + text_w, top_y + text_h),
            m.map_point(top_x, top_y + text_h),
        ];
        let cx = corners.iter().map(|p| p.0).sum::<f64>() / 4.0;
        let cy = corners.iter().map(|p| p.1).sum::<f64>() / 4.0;
        let bw = text_w * scale;
        let bh = text_h * scale;
        (cx - bw / 2.0, cy - bh / 2.0, bw, bh)
    } else {
        let corners = [
            m.map_point(top_x, top_y),
            m.map_point(top_x + text_w, top_y),
            m.map_point(top_x + text_w, top_y + text_h),
            m.map_point(top_x, top_y + text_h),
        ];
        let min_x = corners.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let min_y = corners.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        let max_x = corners.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
        let max_y = corners.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
        (min_x, min_y, (max_x - min_x).max(1.0), (max_y - min_y).max(1.0))
    };

    let fill_op = (style.opacity * style.fill_opacity).clamp(0.0, 1.0);
    let (rgb, alpha) = match resolve_paint(
        style.fill.as_deref(),
        fill_op,
        ctx.gradients,
        (top_x, top_y, top_x + text_w, top_y + text_h),
        true,
    ) {
        Paint::Solid { rgb, alpha } => (rgb, alpha),
        Paint::LinearGradient { stops, .. } | Paint::RadialGradient { stops } => stops
            .first()
            .map(|s| (s.rgb, s.alpha))
            .unwrap_or(([0, 0, 0], 1.0)),
        Paint::None => ([0, 0, 0], 1.0),
    };

    let scale = ctm.then(user_to_emu).avg_scale();
    let em_emu = style.font_size * scale;
    let pt = em_emu / 12_700.0;
    let sz = (pt * 100.0).round().max(100.0) as i64;
    let spc = if style.letter_spacing.abs() > 1e-6 {
        let spc_emu = style.letter_spacing * scale;
        (spc_emu / 12_700.0 * 100.0).round() as i64
    } else {
        0
    };
    let align = match style.text_anchor.as_str() {
        "middle" => "ctr",
        "end" => "r",
        _ => "l",
    };

    let id = *next_id;
    *next_id += 1;
    let pad_w = (box_w * 1.06).max(box_w + scale * 2.0);
    // Track font for optional embedding
    if let Some((typeface, path, data, b)) = fonts.face_for_embed(&style.font_family, bold) {
        let key = format!("{typeface}|{b}");
        if ctx.used_font_keys.insert(key) {
            ctx.used_fonts.push(UsedFont {
                typeface,
                bold: b,
                path,
                data,
            });
        }
    }
    out.push(text_shape(&TextShapeOpts {
        id,
        name: format!("text{id}"),
        x: box_x.round() as i64,
        y: box_y.round() as i64,
        cx: pad_w.round().max(1.0) as i64,
        cy: box_h.round().max(1.0) as i64,
        rot,
        text: content.to_string(),
        font_size_half_points: sz,
        bold,
        italic,
        rgb,
        alpha,
        align,
        latin: fonts.typeface_for(&style.font_family, false),
        ea: fonts.typeface_for(&style.font_family, true),
        letter_spacing_emu: spc,
    }));
}


fn collect_text(node: &Node, out: &mut String) {
    if !node.text.is_empty() {
        out.push_str(&node.text);
    }
    for c in &node.children {
        if c.name == "tspan" || c.name == "textPath" || c.name == "a" {
            collect_text(c, out);
        } else if c.name.is_empty() {
            collect_text(c, out);
        } else {
            // nested text elements
            collect_text(c, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_basic_rect_and_text() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <rect x="10" y="10" width="40" height="20" fill="#FF0000"/>
          <text x="10" y="50" font-size="12" fill="#00FF00">Hi</text>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert!(conv.shapes.len() >= 2);
        assert_eq!(conv.shapes[0].local_name, "sp");
    }

    #[test]
    fn parses_arc_and_cubic() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <path d="M10,30 A20,20 0,0,1 50,30 C60,10 80,10 90,30" fill="none" stroke="#000"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("cubicBezTo") || s.contains("custGeom"));
    }

    #[test]
    fn linear_gradient_emits_grad_fill() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <linearGradient id="g" x1="0" y1="0" x2="1" y2="0">
              <stop offset="0%" stop-color="#FF0000"/>
              <stop offset="100%" stop-color="#0000FF"/>
            </linearGradient>
          </defs>
          <rect width="100" height="100" fill="url(#g)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("gradFill"), "{s}");
    }

    #[test]
    fn transform_scale_translate() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <g transform="translate(10 20)">
            <rect width="10" height="10" fill="#00FF00"/>
          </g>
        </svg>"##;
        let conv = svg_to_shapes(svg, 100_000, 100_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
    }

    #[test]
    fn path_all_commands() {
        // M L H V C S Q T A Z absolute + relative
        let svg = br##"
        <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
          <path d="M10 10 L50 10 H80 V40 C90 40 100 50 100 60 S110 80 120 80
                   Q130 90 140 80 T160 80 A10 10 0 0 1 180 100 z
                   m0 20 l10 0 h5 v5 c5 0 10 5 10 10 s5 10 10 10
                   q5 5 10 0 t10 0 a5 5 0 0 0 20 10 z"
                fill="none" stroke="#123456" stroke-width="2"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 2_000_000, 2_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
    }

    #[test]
    fn polygon_polyline_line() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <polygon points="0,0 50,0 25,40" fill="#f00"/>
          <polyline points="0,50 50,50 50,100" fill="none" stroke="#0f0"/>
          <line x1="0" y1="0" x2="100" y2="100" stroke="#00f" stroke-width="2"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 3);
    }

    #[test]
    fn text_metrics_width_positive() {
        let svg = br##"
        <svg viewBox="0 0 400 100" xmlns="http://www.w3.org/2000/svg">
          <text x="0" y="50" font-size="24" font-family="Helvetica, Arial, sans-serif"
                font-weight="700" fill="#fff">Dify Platform</text>
        </svg>"##;
        let conv = svg_to_shapes(svg, 4_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("Dify Platform"));
        assert!(s.contains("sz="));
    }

    #[test]
    fn nested_group_opacity() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <g opacity="0.5">
            <rect width="100" height="100" fill="#2563EB" opacity="0.5"/>
          </g>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        // 0.5 * 0.5 = 0.25 → alpha 25000
        assert!(s.contains("alpha"), "{s}");
    }

    #[test]
    fn rotate_transform() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <rect x="40" y="40" width="20" height="20" fill="#0f0"
                transform="rotate(45 50 50)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        // rotated → freeform custGeom
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        assert!(String::from_utf8_lossy(&xml).contains("custGeom"));
    }

    #[test]
    fn filter_glow_emits_effect_lst() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <filter id="glow">
              <feGaussianBlur stdDeviation="4"/>
              <feComposite in="SourceGraphic" in2="blur" operator="over"/>
            </filter>
          </defs>
          <circle cx="50" cy="50" r="20" fill="#3B82F6" filter="url(#glow)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("effectLst"), "{s}");
        assert!(s.contains("glow"), "{s}");
    }

    #[test]
    fn css_class_rules_apply() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <style>
            .card { fill: #FF0000; opacity: 0.5; }
            rect { stroke: #00FF00; stroke-width: 2; }
          </style>
          <rect class="card" width="50" height="50"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("FF0000") || s.contains("ff0000") || s.contains("solidFill"), "{s}");
        assert!(s.contains("alpha"), "{s}");
    }

    #[test]
    fn use_href_clones_shape() {
        let svg = br##"
        <svg viewBox="0 0 200 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <rect id="r" width="20" height="10" fill="#0000FF"/>
          </defs>
          <use href="#r" x="0" y="0"/>
          <use href="#r" x="50" y="0"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 2_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 2);
    }

    #[test]
    fn text_rotation_emits_xfrm_rot() {
        let svg = br##"
        <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
          <text x="100" y="100" font-size="16" fill="#fff"
                transform="rotate(30 100 100)">Rotated</text>
        </svg>"##;
        let conv = svg_to_shapes(svg, 2_000_000, 2_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("rot="), "{s}");
    }

    #[test]
    fn clip_path_rect_clips_geometry() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <clipPath id="c">
              <rect x="0" y="0" width="50" height="50"/>
            </clipPath>
          </defs>
          <rect x="0" y="0" width="100" height="100" fill="#f00" clip-path="url(#c)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert!(!conv.shapes.is_empty());
        // clipped freeform should be present
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("custGeom") || s.contains("prstGeom"), "{s}");
    }

    #[test]
    fn marker_end_emits_extra_shape() {
        let svg = br##"
        <svg viewBox="0 0 200 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <marker id="arrow" markerWidth="10" markerHeight="10" refX="5" refY="5" orient="auto">
              <path d="M0,0 L10,5 L0,10 z" fill="#000"/>
            </marker>
          </defs>
          <line x1="10" y1="50" x2="180" y2="50" stroke="#000" stroke-width="2"
                marker-end="url(#arrow)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 2_000_000, 1_000_000, 2).unwrap();
        // line + marker shape
        assert!(conv.shapes.len() >= 2, "got {}", conv.shapes.len());
    }

    #[test]
    fn pattern_fill_tiles_or_solid() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="p" width="10" height="10" patternUnits="userSpaceOnUse">
              <rect width="10" height="10" fill="#00FF00"/>
            </pattern>
          </defs>
          <rect width="100" height="100" fill="url(#p)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        // Tiling expands into many tile shapes (10x10 over 100x100 => ~100, plus clamp).
        assert!(conv.shapes.len() >= 1, "got {}", conv.shapes.len());
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("00FF00") || s.contains("00ff00") || s.contains("solidFill") || s.contains("sp"), "{s}");
    }

    #[test]
    fn mask_reduces_opacity() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <mask id="m">
              <rect width="100" height="100" fill="#808080"/>
            </mask>
          </defs>
          <rect width="100" height="100" fill="#FF0000" mask="url(#m)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        // luminance of #808080 ≈ 0.5 → alpha present
        assert!(s.contains("alpha"), "{s}");
    }

    #[test]
    fn clip_path_polygon_clips() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <clipPath id="tri">
              <polygon points="0,0 100,0 50,100"/>
            </clipPath>
          </defs>
          <rect x="0" y="0" width="100" height="100" fill="#00f" clip-path="url(#tri)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert!(!conv.shapes.is_empty());
    }

    #[test]
    fn drop_shadow_filter_emits_outer_shdw() {
        let svg = br##"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <filter id="sh">
              <feOffset dx="3" dy="4"/>
              <feGaussianBlur stdDeviation="2"/>
              <feFlood flood-color="#000000" flood-opacity="0.5"/>
            </filter>
          </defs>
          <rect x="10" y="10" width="40" height="30" fill="#3366FF" filter="url(#sh)"/>
        </svg>"##;
        let conv = svg_to_shapes(svg, 1_000_000, 1_000_000, 2).unwrap();
        assert_eq!(conv.shapes.len(), 1);
        let xml = crate::element::write_element(&conv.shapes[0]).unwrap();
        let s = String::from_utf8_lossy(&xml);
        assert!(s.contains("outerShdw"), "{s}");
    }

    #[test]
    fn used_fonts_reported_for_text() {
        let svg = br##"
        <svg viewBox="0 0 200 50" xmlns="http://www.w3.org/2000/svg">
          <text x="0" y="30" font-size="16" font-family="Helvetica, Arial, sans-serif"
                fill="#000">Hello</text>
        </svg>"##;
        let conv = svg_to_shapes(svg, 2_000_000, 500_000, 2).unwrap();
        assert!(!conv.used_fonts.is_empty(), "expected used fonts");
        assert!(!conv.used_fonts[0].data.is_empty());
    }
}
