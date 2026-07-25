//! Paint server resolution: solid colors, linear/radial gradients, opacity.
//! Gradients map to DrawingML `a:gradFill` when possible.

use super::matrix::Matrix;
use std::cell::Cell;
use std::collections::HashMap;

thread_local! {
    /// Active CSS `color-scheme` for `light-dark()` resolution during paint.
    static COLOR_SCHEME: Cell<&'static str> = Cell::new("light");
    /// Active CSS `accent-color` used when resolving system `AccentColor`.
    static ACCENT_COLOR: Cell<Option<[u8; 3]>> = Cell::new(None);
}


/// Run `f` with CSS `color-scheme` in effect for `light-dark()`.
pub fn with_color_scheme<R>(scheme: &str, f: impl FnOnce() -> R) -> R {
    with_paint_color_env(scheme, None, f)
}

/// Run `f` with CSS `color-scheme` and optional `accent-color` override for system AccentColor.
pub fn with_paint_color_env<R>(
    scheme: &str,
    accent: Option<&str>,
    f: impl FnOnce() -> R,
) -> R {
    let owned: &'static str = color_scheme_preference(scheme);
    let accent_rgb = accent.and_then(parse_color).map(|(rgb, _)| rgb);
    COLOR_SCHEME.with(|c| {
        let prev = c.replace(owned);
        let out = ACCENT_COLOR.with(|a| {
            let prev_a = a.replace(accent_rgb);
            let out = f();
            a.set(prev_a);
            out
        });
        c.set(prev);
        out
    })
}


fn color_scheme_preference(scheme: &str) -> &'static str {
    let lower = scheme.trim().to_ascii_lowercase();
    if lower.is_empty() || lower == "normal" {
        return "light";
    }
    let tokens: Vec<&str> = lower
        .split_whitespace()
        .filter(|t| *t == "light" || *t == "dark")
        .collect();
    if tokens.is_empty() {
        return "light";
    }
    // `only dark` / `only light`
    if lower.contains("only") {
        if tokens.contains(&"dark") && !tokens.contains(&"light") {
            return "dark";
        }
        if tokens.contains(&"light") && !tokens.contains(&"dark") {
            return "light";
        }
    }
    // Prefer first of light|dark in the list (CSS used order for dual schemes
    // normally follows OS; without OS signal use first token).
    if tokens[0] == "dark" {
        "dark"
    } else {
        "light"
    }
}

fn active_color_scheme() -> &'static str {
    COLOR_SCHEME.with(|c| c.get())
}

/// Public read of active color-scheme for `@media (prefers-color-scheme: …)`.
pub fn active_color_scheme_for_media() -> &'static str {
    active_color_scheme()
}


#[derive(Debug, Clone)]
pub struct ColorStop {
    pub offset: f64,
    pub rgb: [u8; 3],
    pub alpha: f64,
    /// When true, `rgb` is a placeholder; resolve from `currentColor` at paint time.
    pub current_color: bool,
}

#[derive(Debug, Clone)]
pub enum GradientKind {
    Linear {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
    Radial {
        cx: f64,
        cy: f64,
        r: f64,
        fx: f64,
        fy: f64,
        /// SVG 2 focus radius (`fr`). DrawingML has no exact analogue; we
        /// approximate by inserting a solid-pad stop at `fr/r`.
        fr: f64,
    },
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub kind: GradientKind,
    pub stops: Vec<ColorStop>,
    pub transform: Matrix,
    pub units_object_bbox: bool,
    /// SVG `spreadMethod`: pad | reflect | repeat (default pad).
    pub spread_method: String,
    /// SVG `color-interpolation`: sRGB (default) | linearRGB.
    pub color_interpolation: String,
}

#[derive(Debug, Clone)]
pub enum Paint {
    None,
    Solid {
        rgb: [u8; 3],
        alpha: f64,
    },
    LinearGradient {
        stops: Vec<ColorStop>,
        /// Angle in DrawingML 1/60000 deg, clockwise from left-to-right.
        angle: i32,
        /// DrawingML `a:gradFill/@flip`: none | x | y | xy (from SVG spreadMethod).
        flip: String,
    },
    RadialGradient {
        stops: Vec<ColorStop>,
        /// Focus center as fraction of shape bbox (0–1). DrawingML `fillToRect`.
        cx: f64,
        cy: f64,
        /// Approximate radius as fraction of min(bbox w,h). Informational; DML
        /// circle fills always reach the shape edge.
        r: f64,
        flip: String,
    },
    /// DrawingML `a:pattFill` mapped from SVG `<hatch>` / simple diagonal patterns.
    PatternFill {
        /// DrawingML preset: horz | vert | dnDiag | upDiag | cross | diagCross | …
        prst: String,
        fg: [u8; 3],
        fg_alpha: f64,
        bg: [u8; 3],
        bg_alpha: f64,
    },
}

pub type GradientMap = HashMap<String, Gradient>;

/// SVG `<solidColor>` paint server (SVG Tiny 1.2 / SVG 2).
#[derive(Debug, Clone)]
pub struct SolidColor {
    pub rgb: [u8; 3],
    pub alpha: f64,
    /// When true, resolve from `currentColor` at paint time.
    pub current_color: bool,
}

pub type SolidColorMap = HashMap<String, SolidColor>;

/// SVG 2 `<hatch>` paint server → DrawingML pattern fill.
#[derive(Debug, Clone)]
pub struct HatchDef {
    pub prst: String,
    pub fg: [u8; 3],
    pub fg_alpha: f64,
    pub bg: [u8; 3],
    pub bg_alpha: f64,
}

pub type HatchMap = HashMap<String, HatchDef>;

/// Map SVG hatch angle (degrees) + relative pitch to a DrawingML preset name.
pub fn hatch_angle_to_prst(angle_deg: f64, pitch: f64) -> String {
    // Normalize to [0, 180).
    let mut a = angle_deg % 180.0;
    if a < 0.0 {
        a += 180.0;
    }
    // Coarse density: narrow presets for small pitch.
    let dense = pitch > 0.0 && pitch < 6.0;
    if (a - 0.0).abs() < 22.5 || (a - 180.0).abs() < 22.5 {
        return if dense { "narHorz" } else { "horz" }.into();
    }
    if (a - 90.0).abs() < 22.5 {
        return if dense { "narVert" } else { "vert" }.into();
    }
    if (a - 45.0).abs() < 22.5 {
        return "dnDiag".into();
    }
    if (a - 135.0).abs() < 22.5 {
        return "upDiag".into();
    }
    // Default diagonals for odd angles.
    if a < 90.0 {
        "dnDiag".into()
    } else {
        "upDiag".into()
    }
}


fn spread_to_flip(spread: &str) -> String {
    match spread.trim().to_ascii_lowercase().as_str() {
        // DrawingML flip reflects the gradient when it overflows the shape.
        // SVG reflect ≈ flip along both axes; repeat has no exact DML match —
        // use xy as a best-effort approximation (repeat would need tiled fill).
        "reflect" | "repeat" => "xy".into(),
        _ => "none".into(),
    }
}


pub fn parse_color(s: &str) -> Option<([u8; 3], f64)> {
    let s = s.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") {
        return None;
    }
    if s.eq_ignore_ascii_case("transparent") {
        return Some(([0, 0, 0], 0.0));
    }
    let lower = s.to_ascii_lowercase();
    // CSS system colors (used values; approximate common desktop defaults).
    if let Some(rgb) = system_color(&lower) {
        return Some((rgb, 1.0));
    }
    if let Some(hex) = s.strip_prefix('#') {
        return parse_hex(hex);
    }
    if let Some((name, rest)) = lower.split_once('(') {
        let body = rest.strip_suffix(')')?.trim();
        if name == "color-mix" {
            // color-mix(in <space>, c1 p1%, c2 p2%). Any interpolation space
            // is accepted; components are resolved then mixed in sRGB (approx).
            return parse_color_mix(body);
        }
        if name == "light-dark" {
            // CSS Color L5: light-dark(light, dark). Prefer light (no color-scheme).
            return parse_light_dark(body);
        }
        if name == "contrast-color" || name == "color-contrast" {
            // CSS Color L5: contrast-color(color) → black or white maximizing contrast.
            // `color-contrast()` is the older draft name still seen in docs/tests.
            return parse_contrast_color(body);
        }
        if name == "color" {
            // CSS Color L4 color(srgb r g b [/ a]) and relatives → sRGB.
            return parse_color_function(body);
        }
        if name == "device-cmyk" {
            // SVG 1.1 device-cmyk(c m y k [/ a]) → rough sRGB via (1-c)*(1-k).
            return parse_device_cmyk(body);
        }
        if name == "rgb" || name == "rgba" {
            let tokens = split_css_color_args(body);
            // CSS Color L5 relative: rgb(from <color> r g b [/ a])
            if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false)
                && tokens.len() >= 5
            {
                return parse_relative_rgb(&tokens[1], &tokens[2..]);
            }
            if tokens.len() >= 3 {
                let r = parse_component(&tokens[0])?;
                let g = parse_component(&tokens[1])?;
                let b = parse_component(&tokens[2])?;
                let alpha = tokens
                    .get(3)
                    .map(|value| parse_alpha(value))
                    .unwrap_or(Some(1.0))?;
                return Some(([r, g, b], alpha));
            }
        } else if name == "hsl" || name == "hsla" {
            let tokens = split_css_color_args(body);
            if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false)
                && tokens.len() >= 5
            {
                // Relative hsl(from color h s l) → resolve origin then apply channel refs.
                return parse_relative_hsl(&tokens[1], &tokens[2..]);
            }
            if tokens.len() >= 3 {
                let hue = parse_hue_fraction(&tokens[0])?;
                let saturation = parse_percentage_or_number(&tokens[1])?;
                let lightness = parse_percentage_or_number(&tokens[2])?;
                let alpha = tokens
                    .get(3)
                    .map(|value| parse_alpha(value))
                    .unwrap_or(Some(1.0))?;
                return Some((hsl_to_rgb(hue, saturation, lightness), alpha));
            }
        } else if name == "hwb" {
            // CSS Color Module Level 4: hwb(H W B [/ A])
            let tokens = split_css_color_args(body);
            // CSS Color L5 relative: hwb(from <color> h w b [/ a])
            if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false)
                && tokens.len() >= 5
            {
                return parse_relative_hwb(&tokens[1], &tokens[2..]);
            }
            if tokens.len() >= 3 {
                let hue = parse_hue_fraction(&tokens[0])?;
                let white = parse_percentage_or_number(&tokens[1])?;
                let black = parse_percentage_or_number(&tokens[2])?;
                let alpha = tokens
                    .get(3)
                    .map(|value| parse_alpha(value))
                    .unwrap_or(Some(1.0))?;
                return Some((hwb_to_rgb(hue, white, black), alpha));
            }
        } else if name == "oklab" {
            // CSS Color L4: oklab(L a b [/ A]); L is 0..1 or %.
            let tokens = split_css_color_args(body);
            if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false)
                && tokens.len() >= 5
            {
                return parse_relative_oklab(&tokens[1], &tokens[2..]);
            }
            if tokens.len() >= 3 {
                let l = parse_oklab_l(&tokens[0])?;
                let a = parse_optional_number(&tokens[1])?;
                let b = parse_optional_number(&tokens[2])?;
                let alpha = tokens
                    .get(3)
                    .map(|value| parse_alpha(value))
                    .unwrap_or(Some(1.0))?;
                return Some((oklab_to_srgb(l, a, b), alpha));
            }
        } else if name == "oklch" {
            // CSS Color L4: oklch(L C H [/ A]); H in degrees.
            let tokens = split_css_color_args(body);
            if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false)
                && tokens.len() >= 5
            {
                return parse_relative_oklch(&tokens[1], &tokens[2..]);
            }
            if tokens.len() >= 3 {
                let l = parse_oklab_l(&tokens[0])?;
                let c = parse_optional_number(&tokens[1])?;
                let h_frac = parse_hue_fraction(&tokens[2])?;
                let alpha = tokens
                    .get(3)
                    .map(|value| parse_alpha(value))
                    .unwrap_or(Some(1.0))?;
                let h = h_frac * 2.0 * std::f64::consts::PI;
                let a = c * h.cos();
                let b = c * h.sin();
                return Some((oklab_to_srgb(l, a, b), alpha));
            }
        } else if name == "lab" {
            // CSS Color L4: lab(L a b [/ A]); L is 0..100 or %.
            let tokens = split_css_color_args(body);
            if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false)
                && tokens.len() >= 5
            {
                return parse_relative_lab(&tokens[1], &tokens[2..]);
            }
            if tokens.len() >= 3 {
                let l = parse_cie_lab_l(&tokens[0])?;
                let a = parse_optional_number(&tokens[1])?;
                let b = parse_optional_number(&tokens[2])?;
                let alpha = tokens
                    .get(3)
                    .map(|value| parse_alpha(value))
                    .unwrap_or(Some(1.0))?;
                return Some((cie_lab_to_srgb(l, a, b), alpha));
            }
        } else if name == "lch" {
            // CSS Color L4: lch(L C H [/ A]).
            let tokens = split_css_color_args(body);
            if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false)
                && tokens.len() >= 5
            {
                return parse_relative_lch(&tokens[1], &tokens[2..]);
            }
            if tokens.len() >= 3 {
                let l = parse_cie_lab_l(&tokens[0])?;
                let c = parse_optional_number(&tokens[1])?;
                let h_frac = parse_hue_fraction(&tokens[2])?;
                let alpha = tokens
                    .get(3)
                    .map(|value| parse_alpha(value))
                    .unwrap_or(Some(1.0))?;
                let h = h_frac * 2.0 * std::f64::consts::PI;
                let a = c * h.cos();
                let b = c * h.sin();
                return Some((cie_lab_to_srgb(l, a, b), alpha));
            }
        }
    }
    named_color(&lower).map(|rgb| (rgb, 1.0))
}

fn parse_cie_lab_l(s: &str) -> Option<f64> {
    if s.trim().eq_ignore_ascii_case("none") {
        return Some(0.0);
    }
    if let Some(p) = s.strip_suffix('%') {
        Some((p.parse::<f64>().ok()? / 100.0 * 100.0).clamp(0.0, 100.0))
    } else {
        Some(s.parse::<f64>().ok()?.clamp(0.0, 100.0))
    }
}

/// CIE Lab (D65) → sRGB via XYZ.
fn cie_lab_to_srgb(l: f64, a: f64, b: f64) -> [u8; 3] {
    // Lab → XYZ (D65 white)
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;
    let f_inv = |t: f64| {
        let t3 = t * t * t;
        if t3 > 0.008856 {
            t3
        } else {
            (t - 16.0 / 116.0) / 7.787
        }
    };
    let xn = 0.95047;
    let yn = 1.0;
    let zn = 1.08883;
    let x = xn * f_inv(fx);
    let y = yn * f_inv(fy);
    let z = zn * f_inv(fz);
    // XYZ → linear sRGB
    let r_lin = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let g_lin = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let b_lin = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
    let to_srgb = |c: f64| -> u8 {
        let c = c.clamp(0.0, 1.0);
        let encoded = if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        };
        (encoded * 255.0).round().clamp(0.0, 255.0) as u8
    };
    [to_srgb(r_lin), to_srgb(g_lin), to_srgb(b_lin)]
}

fn parse_oklab_l(s: &str) -> Option<f64> {
    if s.trim().eq_ignore_ascii_case("none") {
        return Some(0.0);
    }
    if let Some(p) = s.strip_suffix('%') {
        Some((p.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
    } else {
        Some(s.parse::<f64>().ok()?.clamp(0.0, 1.0))
    }
}

/// OKLab → sRGB (CSS Color Module Level 4).
fn oklab_to_srgb(l: f64, a: f64, b: f64) -> [u8; 3] {
    // OKLab → LMS'
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
    let l_c = l_ * l_ * l_;
    let m_c = m_ * m_ * m_;
    let s_c = s_ * s_ * s_;
    // LMS → linear sRGB
    let r_lin = 4.0767416621 * l_c - 3.3077115913 * m_c + 0.2309699292 * s_c;
    let g_lin = -1.2684380046 * l_c + 2.6097574011 * m_c - 0.3413193965 * s_c;
    let b_lin = -0.0041960863 * l_c - 0.7034186147 * m_c + 1.7076147010 * s_c;
    let to_srgb = |c: f64| -> u8 {
        let c = c.clamp(0.0, 1.0);
        let encoded = if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        };
        (encoded * 255.0).round().clamp(0.0, 255.0) as u8
    };
    [to_srgb(r_lin), to_srgb(g_lin), to_srgb(b_lin)]
}

fn hwb_to_rgb(h: f64, w: f64, b: f64) -> [u8; 3] {
    // https://www.w3.org/TR/css-color-4/#hwb-to-rgb
    let sum = w + b;
    let (w, b) = if sum > 1.0 {
        (w / sum, b / sum)
    } else {
        (w, b)
    };
    let [r, g, bl] = hsl_to_rgb(h, 1.0, 0.5);
    let rf = r as f64 / 255.0;
    let gf = g as f64 / 255.0;
    let bf = bl as f64 / 255.0;
    let scale = 1.0 - w - b;
    [
        ((rf * scale + w) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((gf * scale + w) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((bf * scale + w) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

fn split_css_color_args(body: &str) -> Vec<String> {
    // Split on whitespace/commas/slash at top level only so `calc(h + 120)` stays one token.
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    for ch in body.chars() {
        match ch {
            '(' => {
                depth += 1;
                cur.push(ch);
            }
            ')' => {
                depth -= 1;
                cur.push(ch);
            }
            ',' | '/' if depth == 0 => {
                if !cur.trim().is_empty() {
                    out.push(cur.trim().to_string());
                }
                cur.clear();
            }
            c if c.is_whitespace() && depth == 0 => {
                if !cur.trim().is_empty() {
                    out.push(cur.trim().to_string());
                }
                cur.clear();
            }
            c => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

fn parse_alpha(s: &str) -> Option<f64> {
    if s.trim().eq_ignore_ascii_case("none") {
        return Some(0.0);
    }
    if let Some(value) = s.strip_suffix('%') {
        Some((value.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
    } else {
        Some(s.parse::<f64>().ok()?.clamp(0.0, 1.0))
    }
}

fn parse_percentage(s: &str) -> Option<f64> {
    Some((s.strip_suffix('%')?.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
}

/// Number channel that accepts CSS Color L4 `none` as 0.
fn parse_optional_number(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.eq_ignore_ascii_case("none") {
        return Some(0.0);
    }
    s.parse::<f64>().ok()
}

/// CSS Color L4 percentage-or-number for S/L/W/B channels: `50%` → 0.5, bare `0.5` → 0.5.
fn parse_percentage_or_number(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.eq_ignore_ascii_case("none") {
        return Some(0.0);
    }
    if let Some(p) = s.strip_suffix('%') {
        Some((p.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
    } else {
        Some(s.parse::<f64>().ok()?.clamp(0.0, 1.0))
    }
}

/// CSS Color hue: unitless degrees, or `deg` / `grad` / `rad` / `turn` → fraction of circle [0,1).
fn parse_hue_fraction(s: &str) -> Option<f64> {
    let s = s.trim().to_ascii_lowercase();
    if s.is_empty() || s == "none" {
        return Some(0.0);
    }
    if let Some(v) = s.strip_suffix("turn") {
        return Some(v.trim().parse::<f64>().ok()?.rem_euclid(1.0));
    }
    if let Some(v) = s.strip_suffix("grad") {
        return Some((v.trim().parse::<f64>().ok()? / 400.0).rem_euclid(1.0));
    }
    if let Some(v) = s.strip_suffix("rad") {
        let rad = v.trim().parse::<f64>().ok()?;
        return Some((rad / (2.0 * std::f64::consts::PI)).rem_euclid(1.0));
    }
    // deg or unitless (CSS default degrees for hsl/hwb hue).
    let v = s.strip_suffix("deg").unwrap_or(&s).trim();
    Some((v.parse::<f64>().ok()? / 360.0).rem_euclid(1.0))
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    if s <= 1e-12 {
        let gray = (l * 255.0).round() as u8;
        return [gray, gray, gray];
    }
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let channel = |mut t: f64| {
        t = t.rem_euclid(1.0);
        let value = if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 1.0 / 2.0 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        };
        (value * 255.0).round().clamp(0.0, 255.0) as u8
    };
    [channel(h + 1.0 / 3.0), channel(h), channel(h - 1.0 / 3.0)]
}

fn parse_component(s: &str) -> Option<u8> {
    if s.trim().eq_ignore_ascii_case("none") {
        return Some(0);
    }
    if let Some(p) = s.strip_suffix('%') {
        let v: f64 = p.parse().ok()?;
        Some((v / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8)
    } else {
        let v: f64 = s.parse().ok()?;
        Some(v.round().clamp(0.0, 255.0) as u8)
    }
}

fn parse_hex(hex: &str) -> Option<([u8; 3], f64)> {
    match hex.len() {
        3 | 4 => {
            let digits: Vec<u8> = hex
                .chars()
                .map(|c| c.to_digit(16).map(|v| (v * 17) as u8))
                .collect::<Option<_>>()?;
            let alpha = if digits.len() == 4 {
                digits[3] as f64 / 255.0
            } else {
                1.0
            };
            Some(([digits[0], digits[1], digits[2]], alpha))
        }
        6 | 8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let alpha = if hex.len() == 8 {
                u8::from_str_radix(&hex[6..8], 16).ok()? as f64 / 255.0
            } else {
                1.0
            };
            Some(([r, g, b], alpha))
        }
        _ => None,
    }
}


/// CSS system colors (CSS Color Module Level 4) — approximate used values.
fn system_color(s: &str) -> Option<[u8; 3]> {
    match s {
        "canvas" | "background" => Some(if active_color_scheme() == "dark" {
            [18, 18, 18]
        } else {
            [255, 255, 255]
        }),
        "canvastext" | "windowtext" | "text" => Some(if active_color_scheme() == "dark" {
            [255, 255, 255]
        } else {
            [0, 0, 0]
        }),
        "linktext" | "link" => Some([0, 0, 238]),
        "visitedtext" | "visited" => Some([85, 26, 139]),
        "activetext" | "activeborder" => Some([255, 0, 0]),
        "buttonface" | "button" | "threedface" => Some([240, 240, 240]),
        "buttontext" => Some([0, 0, 0]),
        "buttonborder" | "threeddarkshadow" => Some([118, 118, 118]),
        "field" => Some([255, 255, 255]),
        "fieldtext" => Some([0, 0, 0]),
        "highlight" => Some([0, 120, 215]),
        "highlighttext" => Some([255, 255, 255]),
        "selecteditem" => Some([0, 120, 215]),
        "selecteditemtext" => Some([255, 255, 255]),
        "mark" => Some([255, 255, 0]),
        "marktext" => Some([0, 0, 0]),
        "graytext" | "inactivecaptiontext" => Some([109, 109, 109]),
        "accentcolor" => ACCENT_COLOR
            .with(|a| a.get())
            .or(Some([0, 120, 215])),
        "accentcolortext" => Some([255, 255, 255]),
        "menutext" => Some([0, 0, 0]),
        "menu" => Some([240, 240, 240]),
        _ => None,
    }
}

/// CSS Color L5 relative `rgb(from ORIGIN r g b [/ a])`.
/// Channel tokens may be `r`/`g`/`b`/`alpha`, numbers, or percentages.
fn parse_relative_rgb(origin: &str, channels: &[String]) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            alpha = resolve_rel_channel(&ch[pos + 1], base, base_a, 'a')?;
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let r = resolve_rel_channel(&ch[0], base, base_a, 'r')?;
    let g = resolve_rel_channel(&ch[1], base, base_a, 'g')?;
    let b = resolve_rel_channel(&ch[2], base, base_a, 'b')?;
    Some((
        [
            r.round().clamp(0.0, 255.0) as u8,
            g.round().clamp(0.0, 255.0) as u8,
            b.round().clamp(0.0, 255.0) as u8,
        ],
        alpha.clamp(0.0, 1.0),
    ))
}

/// Relative `hsl(from ORIGIN h s l [/ a])` — origin converted to HSL, channels remixed.
fn parse_relative_hsl(origin: &str, channels: &[String]) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let (h0, s0, l0) = rgb_to_hsl(base);
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            let t = ch[pos + 1].as_str();
            alpha = if t.eq_ignore_ascii_case("alpha") {
                base_a
            } else {
                parse_alpha(t)?
            };
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let h = resolve_hsl_channel(&ch[0], h0, s0, l0, 'h')?;
    let s = resolve_hsl_channel(&ch[1], h0, s0, l0, 's')?;
    let l = resolve_hsl_channel(&ch[2], h0, s0, l0, 'l')?;
    Some((hsl_to_rgb(h / 360.0, s, l), alpha.clamp(0.0, 1.0)))
}

/// sRGB → CIE Lab (D65), inverse of cie_lab_to_srgb.
fn srgb_to_cie_lab(rgb: [u8; 3]) -> (f64, f64, f64) {
    let to_lin = |c: u8| {
        let c = c as f64 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let r = to_lin(rgb[0]);
    let g = to_lin(rgb[1]);
    let b = to_lin(rgb[2]);
    // linear sRGB → XYZ (D65)
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;
    let xn = 0.95047;
    let yn = 1.0;
    let zn = 1.08883;
    let f = |t: f64| {
        if t > 0.008856 {
            t.cbrt()
        } else {
            7.787 * t + 16.0 / 116.0
        }
    };
    let fx = f(x / xn);
    let fy = f(y / yn);
    let fz = f(z / zn);
    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let bb = 200.0 * (fy - fz);
    (l, a, bb)
}

fn cie_lab_to_lch(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
    let c = (a * a + b * b).sqrt();
    let h = if c < 1e-12 {
        0.0
    } else {
        b.atan2(a).to_degrees().rem_euclid(360.0)
    };
    (l, c, h)
}

/// Relative `lab(from ORIGIN l a b [/ alpha])`.
fn parse_relative_lab(origin: &str, channels: &[String]) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let (l0, a0, b0) = srgb_to_cie_lab(base);
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            let t = ch[pos + 1].as_str();
            alpha = if t.eq_ignore_ascii_case("alpha") {
                base_a
            } else {
                parse_alpha(t)?
            };
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let l = resolve_cie_lab_channel(&ch[0], l0, a0, b0, 'l')?;
    let a = resolve_cie_lab_channel(&ch[1], l0, a0, b0, 'a')?;
    let b = resolve_cie_lab_channel(&ch[2], l0, a0, b0, 'b')?;
    Some((cie_lab_to_srgb(l, a, b), alpha.clamp(0.0, 1.0)))
}

/// Relative `lch(from ORIGIN l c h [/ alpha])`.
fn parse_relative_lch(origin: &str, channels: &[String]) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let (l0, a0, b0) = srgb_to_cie_lab(base);
    let (l0, c0, h0) = cie_lab_to_lch(l0, a0, b0);
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            let t = ch[pos + 1].as_str();
            alpha = if t.eq_ignore_ascii_case("alpha") {
                base_a
            } else {
                parse_alpha(t)?
            };
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let l = resolve_cie_lch_channel(&ch[0], l0, c0, h0, 'l')?;
    let c = resolve_cie_lch_channel(&ch[1], l0, c0, h0, 'c')?;
    let h_deg = resolve_cie_lch_channel(&ch[2], l0, c0, h0, 'h')?;
    let h = h_deg * std::f64::consts::PI / 180.0;
    let a = c * h.cos();
    let b = c * h.sin();
    Some((cie_lab_to_srgb(l, a, b), alpha.clamp(0.0, 1.0)))
}

fn resolve_cie_lab_channel(tok: &str, l: f64, a: f64, b: f64, which: char) -> Option<f64> {
    let t = tok.trim().to_ascii_lowercase();
    if t == "none" {
        return Some(0.0);
    }
    match (which, t.as_str()) {
        ('l', "l") => Some(l),
        ('a', "a") => Some(a),
        ('b', "b") => Some(b),
        _ => {
            if let Some(v) = eval_rel_channel_expr(tok, &[("l", l), ("a", a), ("b", b)]) {
                return Some(v);
            }
            match which {
                'l' => parse_cie_lab_l(tok),
                'a' | 'b' => tok.parse::<f64>().ok(),
                _ => None,
            }
        }
    }
}

fn resolve_cie_lch_channel(tok: &str, l: f64, c: f64, h: f64, which: char) -> Option<f64> {
    let t = tok.trim().to_ascii_lowercase();
    if t == "none" {
        return Some(0.0);
    }
    match (which, t.as_str()) {
        ('l', "l") => Some(l),
        ('c', "c") => Some(c),
        ('h', "h") => Some(h),
        _ => {
            if let Some(v) = eval_rel_channel_expr(tok, &[("l", l), ("c", c), ("h", h)]) {
                return Some(v);
            }
            match which {
                'l' => parse_cie_lab_l(tok),
                'c' => tok.parse::<f64>().ok(),
                'h' => {
                    let v = t.trim_end_matches("deg").trim().parse::<f64>().ok()?;
                    Some(v)
                }
                _ => None,
            }
        }
    }
}

/// Relative `hwb(from ORIGIN h w b [/ a])`.
fn parse_relative_hwb(origin: &str, channels: &[String]) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let (h0, s0, l0) = rgb_to_hsl(base);
    // Approximate HWB from HSL: pure hue at s=1 l=0.5; W≈(1-s)*l-ish is imperfect.
    // Convert via RGB→HWB: W = min(r,g,b), B = 1-max(r,g,b), H from hsl.
    let r = base[0] as f64 / 255.0;
    let g = base[1] as f64 / 255.0;
    let b = base[2] as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let w0 = min;
    let bk0 = 1.0 - max;
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            let t = ch[pos + 1].as_str();
            alpha = if t.eq_ignore_ascii_case("alpha") {
                base_a
            } else {
                parse_alpha(t)?
            };
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let h = resolve_hwb_channel(&ch[0], h0, w0, bk0, 'h')?;
    let w = resolve_hwb_channel(&ch[1], h0, w0, bk0, 'w')?;
    let bk = resolve_hwb_channel(&ch[2], h0, w0, bk0, 'b')?;
    let _ = (s0, l0);
    Some((hwb_to_rgb(h / 360.0, w, bk), alpha.clamp(0.0, 1.0)))
}

fn resolve_hwb_channel(tok: &str, h: f64, w: f64, b: f64, which: char) -> Option<f64> {
    let t = tok.trim().to_ascii_lowercase();
    if t == "none" {
        return Some(0.0);
    }
    match (which, t.as_str()) {
        ('h', "h") => Some(h),
        ('w', "w") => Some(w),
        ('b', "b") => Some(b),
        _ => {
            if let Some(v) = eval_rel_channel_expr(
                tok,
                &[("h", h), ("w", w * 100.0), ("b", b * 100.0)],
            ) {
                return Some(if which == 'h' {
                    v
                } else {
                    (v / 100.0).clamp(0.0, 1.0)
                });
            }
            match which {
                'h' => {
                    let v = t.trim_end_matches("deg").trim().parse::<f64>().ok()?;
                    Some(v)
                }
                'w' | 'b' => {
                    if let Some(p) = t.strip_suffix('%') {
                        Some((p.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
                    } else {
                        Some(t.parse::<f64>().ok()?.clamp(0.0, 1.0))
                    }
                }
                _ => None,
            }
        }
    }
}

/// sRGB → OKLab (CSS Color L4 inverse of oklab_to_srgb).
fn srgb_to_oklab(rgb: [u8; 3]) -> (f64, f64, f64) {
    let to_lin = |c: u8| {
        let c = c as f64 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let r = to_lin(rgb[0]);
    let g = to_lin(rgb[1]);
    let b = to_lin(rgb[2]);
    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();
    let ll = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let bch = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;
    (ll, a, bch)
}

fn oklab_to_oklch(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
    let c = (a * a + b * b).sqrt();
    let h = if c < 1e-12 {
        0.0
    } else {
        b.atan2(a).to_degrees().rem_euclid(360.0)
    };
    (l, c, h)
}

/// Relative `oklab(from ORIGIN l a b [/ alpha])`.
fn parse_relative_oklab(origin: &str, channels: &[String]) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let (l0, a0, b0) = srgb_to_oklab(base);
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            let t = ch[pos + 1].as_str();
            alpha = if t.eq_ignore_ascii_case("alpha") {
                base_a
            } else {
                parse_alpha(t)?
            };
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let l = resolve_oklab_channel(&ch[0], l0, a0, b0, 'l')?;
    let a = resolve_oklab_channel(&ch[1], l0, a0, b0, 'a')?;
    let b = resolve_oklab_channel(&ch[2], l0, a0, b0, 'b')?;
    Some((oklab_to_srgb(l, a, b), alpha.clamp(0.0, 1.0)))
}

/// Relative `oklch(from ORIGIN l c h [/ alpha])`.
fn parse_relative_oklch(origin: &str, channels: &[String]) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let (l0, a0, b0) = srgb_to_oklab(base);
    let (l0, c0, h0) = oklab_to_oklch(l0, a0, b0);
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            let t = ch[pos + 1].as_str();
            alpha = if t.eq_ignore_ascii_case("alpha") {
                base_a
            } else {
                parse_alpha(t)?
            };
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let l = resolve_oklch_channel(&ch[0], l0, c0, h0, 'l')?;
    let c = resolve_oklch_channel(&ch[1], l0, c0, h0, 'c')?;
    let h_deg = resolve_oklch_channel(&ch[2], l0, c0, h0, 'h')?;
    let h = h_deg * std::f64::consts::PI / 180.0;
    let a = c * h.cos();
    let b = c * h.sin();
    Some((oklab_to_srgb(l, a, b), alpha.clamp(0.0, 1.0)))
}

fn resolve_oklab_channel(tok: &str, l: f64, a: f64, b: f64, which: char) -> Option<f64> {
    let t = tok.trim().to_ascii_lowercase();
    if t == "none" {
        return Some(0.0);
    }
    match (which, t.as_str()) {
        ('l', "l") => Some(l),
        ('a', "a") => Some(a),
        ('b', "b") => Some(b),
        _ => {
            if let Some(v) = eval_rel_channel_expr(tok, &[("l", l), ("a", a), ("b", b)]) {
                return Some(v);
            }
            match which {
                'l' => parse_oklab_l(tok),
                'a' | 'b' => tok.parse::<f64>().ok(),
                _ => None,
            }
        }
    }
}

fn resolve_oklch_channel(tok: &str, l: f64, c: f64, h: f64, which: char) -> Option<f64> {
    let t = tok.trim().to_ascii_lowercase();
    if t == "none" {
        return Some(0.0);
    }
    match (which, t.as_str()) {
        ('l', "l") => Some(l),
        ('c', "c") => Some(c),
        ('h', "h") => Some(h),
        _ => {
            if let Some(v) = eval_rel_channel_expr(tok, &[("l", l), ("c", c), ("h", h)]) {
                return Some(v);
            }
            match which {
                'l' => parse_oklab_l(tok),
                'c' => tok.parse::<f64>().ok(),
                'h' => {
                    let v = t.trim_end_matches("deg").trim().parse::<f64>().ok()?;
                    Some(v)
                }
                _ => None,
            }
        }
    }
}

/// Evaluate a relative-color channel token that may be `calc(...)` with channel vars.
/// `vars` maps lowercase channel names to numeric values (e.g. "h"→120.0).
fn eval_rel_channel_expr(tok: &str, vars: &[(&str, f64)]) -> Option<f64> {
    let t = tok.trim();
    let lower = t.to_ascii_lowercase();
    // Direct channel keyword.
    for (name, val) in vars {
        if lower == *name {
            return Some(*val);
        }
    }
    // calc(...) with channel names substituted as numbers.
    if lower.starts_with("calc(") && t.ends_with(')') {
        let body = &t[5..t.len() - 1];
        let mut expr = body.to_string();
        // Replace longer names first (alpha before a).
        let mut pairs: Vec<(&str, f64)> = vars.to_vec();
        pairs.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        for (name, val) in pairs {
            // Word-boundary-ish replace of channel idents.
            let mut out = String::new();
            let bytes = expr.as_bytes();
            let mut i = 0usize;
            let nl = name.len();
            let name_l = name.to_ascii_lowercase();
            while i < bytes.len() {
                let rest = &expr[i..];
                let rest_l = rest.to_ascii_lowercase();
                if rest_l.starts_with(&name_l) {
                    let before_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
                    let after_i = i + nl;
                    let after_ok = after_i >= bytes.len() || !bytes[after_i].is_ascii_alphanumeric();
                    if before_ok && after_ok {
                        out.push_str(&format!("{val:.12}"));
                        i = after_i;
                        continue;
                    }
                }
                out.push(expr[i..].chars().next().unwrap());
                i += expr[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            }
            expr = out;
        }
        // Lightweight arithmetic: + - * / and parentheses, numbers only.
        return eval_rel_arith(&expr);
    }
    // Bare number / percent / deg handled by caller.
    None
}

fn eval_rel_arith(expr: &str) -> Option<f64> {
    let expr = expr.trim();
    if expr.is_empty() {
        return None;
    }
    // Parentheses.
    if let Some(open) = expr.find('(') {
        let mut depth = 0i32;
        let mut end = None;
        for (i, ch) in expr[open..].char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(open + i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let end = end?;
        let inner = eval_rel_arith(expr[open + 1..end].trim())?;
        let mut rewritten = String::new();
        rewritten.push_str(&expr[..open]);
        rewritten.push_str(&format!("{inner:.12}"));
        rewritten.push_str(&expr[end + 1..]);
        return eval_rel_arith(&rewritten);
    }
    // + - at top level (binary).
    let bytes = expr.as_bytes();
    let mut depth = 0i32;
    for i in (0..bytes.len()).rev() {
        match bytes[i] {
            b')' => depth += 1,
            b'(' => depth -= 1,
            b'+' | b'-' if depth == 0 && i > 0 => {
                let left = expr[..i].trim();
                let right = expr[i + 1..].trim();
                if left.is_empty() || right.is_empty() {
                    continue;
                }
                let lb = left.as_bytes()[left.len() - 1];
                if !(lb.is_ascii_digit() || lb == b'.' || lb == b')') {
                    continue;
                }
                let lv = eval_rel_arith(left)?;
                let rv = eval_rel_arith(right)?;
                return Some(if bytes[i] == b'+' { lv + rv } else { lv - rv });
            }
            _ => {}
        }
    }
    depth = 0;
    for i in (0..bytes.len()).rev() {
        match bytes[i] {
            b')' => depth += 1,
            b'(' => depth -= 1,
            b'*' | b'/' if depth == 0 && i > 0 => {
                let left = expr[..i].trim();
                let right = expr[i + 1..].trim();
                let lv = eval_rel_arith(left)?;
                let rv = eval_rel_arith(right)?;
                if bytes[i] == b'*' {
                    return Some(lv * rv);
                } else if rv.abs() < 1e-12 {
                    return None;
                } else {
                    return Some(lv / rv);
                }
            }
            _ => {}
        }
    }
    // Leaf number (optional deg/% stripped).
    let leaf = expr
        .trim()
        .trim_end_matches("deg")
        .trim_end_matches('%')
        .trim();
    leaf.parse::<f64>().ok()
}

fn resolve_rel_channel(tok: &str, base: [u8; 3], base_a: f64, which: char) -> Option<f64> {
    let t = tok.trim();
    match t.to_ascii_lowercase().as_str() {
        "none" => Some(0.0),
        "r" => Some(base[0] as f64),
        "g" => Some(base[1] as f64),
        "b" => Some(base[2] as f64),
        "alpha" | "a" if which == 'a' => Some(base_a),
        "alpha" => Some(base_a * 255.0), // rare
        _ => {
            if let Some(v) = eval_rel_channel_expr(
                tok,
                &[
                    ("r", base[0] as f64),
                    ("g", base[1] as f64),
                    ("b", base[2] as f64),
                    ("alpha", base_a),
                ],
            ) {
                return Some(v);
            }
            if which == 'a' {
                parse_alpha(t)
            } else if let Some(p) = t.strip_suffix('%') {
                Some((p.parse::<f64>().ok()? / 100.0) * 255.0)
            } else {
                t.parse::<f64>().ok()
            }
        }
    }
}

fn resolve_hsl_channel(tok: &str, h: f64, s: f64, l: f64, which: char) -> Option<f64> {
    let t = tok.trim().to_ascii_lowercase();
    if t == "none" {
        return Some(0.0);
    }
    match (which, t.as_str()) {
        ('h', "h") => Some(h),
        ('s', "s") => Some(s),
        ('l', "l") => Some(l),
        _ => {
            // calc with s/l as 0..100 percentages for CSS-like arithmetic.
            if let Some(v) = eval_rel_channel_expr(
                tok,
                &[("h", h), ("s", s * 100.0), ("l", l * 100.0)],
            ) {
                return Some(if which == 'h' {
                    v
                } else {
                    (v / 100.0).clamp(0.0, 1.0)
                });
            }
            match which {
                'h' => {
                    let v = t.trim_end_matches("deg").trim().parse::<f64>().ok()?;
                    Some(v)
                }
                's' | 'l' => {
                    if let Some(p) = t.strip_suffix('%') {
                        Some((p.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
                    } else {
                        Some(t.parse::<f64>().ok()?.clamp(0.0, 1.0))
                    }
                }
                _ => None,
            }
        }
    }
}

fn rgb_to_hsl(rgb: [u8; 3]) -> (f64, f64, f64) {
    let r = rgb[0] as f64 / 255.0;
    let g = rgb[1] as f64 / 255.0;
    let b = rgb[2] as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-12 {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if (max - r).abs() < 1e-12 {
        let mut v = (g - b) / d + if g < b { 6.0 } else { 0.0 };
        v
    } else if (max - g).abs() < 1e-12 {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } / 6.0;
    (h * 360.0, s, l)
}

/// CSS Color L4 `color(<colorspace> c1 c2 c3 [/ a])`.
/// Supports `srgb`, `srgb-linear`, `display-p3` (treated as sRGB), and
/// `xyz` / `xyz-d65` (rough XYZ→sRGB). Components may be numbers or %.
/// CSS Color L5 `color(from ORIGIN <space> c0 c1 c2 [/ a])` with channel refs r/g/b/alpha
/// (srgb-family) or x/y/z (xyz). Soft: all spaces resolve origin to sRGB first.
fn parse_relative_color_fn(
    space: &str,
    origin: &str,
    channels: &[String],
) -> Option<([u8; 3], f64)> {
    let (base, base_a) = parse_color(origin)?;
    let mut ch = channels.to_vec();
    let mut alpha = base_a;
    if let Some(pos) = ch.iter().position(|t| t == "/") {
        if pos + 1 < ch.len() {
            let t = ch[pos + 1].as_str();
            alpha = if t.eq_ignore_ascii_case("alpha") {
                base_a
            } else {
                parse_alpha_component(t)?
            };
        }
        ch.truncate(pos);
    }
    if ch.len() < 3 {
        return None;
    }
    let space = space.to_ascii_lowercase();
    match space.as_str() {
        "srgb" | "display-p3" | "a98-rgb" | "prophoto-rgb" | "rec2020" => {
            let r0 = base[0] as f64 / 255.0;
            let g0 = base[1] as f64 / 255.0;
            let b0 = base[2] as f64 / 255.0;
            let c0 = resolve_color_fn_channel(&ch[0], r0, g0, b0, 'r')?;
            let c1 = resolve_color_fn_channel(&ch[1], r0, g0, b0, 'g')?;
            let c2 = resolve_color_fn_channel(&ch[2], r0, g0, b0, 'b')?;
            Some((
                [
                    (c0 * 255.0).round().clamp(0.0, 255.0) as u8,
                    (c1 * 255.0).round().clamp(0.0, 255.0) as u8,
                    (c2 * 255.0).round().clamp(0.0, 255.0) as u8,
                ],
                alpha.clamp(0.0, 1.0),
            ))
        }
        "srgb-linear" => {
            let to_lin = |c: u8| {
                let c = c as f64 / 255.0;
                if c <= 0.04045 {
                    c / 12.92
                } else {
                    ((c + 0.055) / 1.055).powf(2.4)
                }
            };
            let r0 = to_lin(base[0]);
            let g0 = to_lin(base[1]);
            let b0 = to_lin(base[2]);
            let c0 = resolve_color_fn_channel(&ch[0], r0, g0, b0, 'r')?;
            let c1 = resolve_color_fn_channel(&ch[1], r0, g0, b0, 'g')?;
            let c2 = resolve_color_fn_channel(&ch[2], r0, g0, b0, 'b')?;
            let to_srgb = |c: f64| -> f64 {
                let c = c.clamp(0.0, 1.0);
                if c <= 0.0031308 {
                    12.92 * c
                } else {
                    1.055 * c.powf(1.0 / 2.4) - 0.055
                }
            };
            Some((
                [
                    (to_srgb(c0) * 255.0).round().clamp(0.0, 255.0) as u8,
                    (to_srgb(c1) * 255.0).round().clamp(0.0, 255.0) as u8,
                    (to_srgb(c2) * 255.0).round().clamp(0.0, 255.0) as u8,
                ],
                alpha.clamp(0.0, 1.0),
            ))
        }
        _ => {
            // Unknown space: fall back to sRGB channel remix of origin.
            let r0 = base[0] as f64 / 255.0;
            let g0 = base[1] as f64 / 255.0;
            let b0 = base[2] as f64 / 255.0;
            let c0 = resolve_color_fn_channel(&ch[0], r0, g0, b0, 'r')?;
            let c1 = resolve_color_fn_channel(&ch[1], r0, g0, b0, 'g')?;
            let c2 = resolve_color_fn_channel(&ch[2], r0, g0, b0, 'b')?;
            Some((
                [
                    (c0 * 255.0).round().clamp(0.0, 255.0) as u8,
                    (c1 * 255.0).round().clamp(0.0, 255.0) as u8,
                    (c2 * 255.0).round().clamp(0.0, 255.0) as u8,
                ],
                alpha.clamp(0.0, 1.0),
            ))
        }
    }
}

fn resolve_color_fn_channel(tok: &str, r: f64, g: f64, b: f64, which: char) -> Option<f64> {
    let t = tok.trim().to_ascii_lowercase();
    match t.as_str() {
        "r" | "red" => Some(r),
        "g" | "green" => Some(g),
        "b" | "blue" => Some(b),
        "x" if which == 'r' => Some(r),
        "y" if which == 'g' => Some(g),
        "z" if which == 'b' => Some(b),
        _ => parse_color_component(tok, true),
    }
}

fn parse_color_function(body: &str) -> Option<([u8; 3], f64)> {
    let mut tokens = split_css_color_args(body);
    if tokens.is_empty() {
        return None;
    }
    let space = tokens.remove(0).to_ascii_lowercase();
    // CSS Color L5: color(from <color> srgb r g b [/ a]) — origin then space.
    // Also accept color(srgb from <color> r g b) soft order.
    let mut rel_origin: Option<String> = None;
    if space == "from" {
        if tokens.len() < 5 {
            return None;
        }
        rel_origin = Some(tokens.remove(0));
        let space2 = tokens.remove(0).to_ascii_lowercase();
        return parse_relative_color_fn(&space2, rel_origin.as_deref().unwrap(), &tokens);
    }
    if tokens.first().map(|t| t.eq_ignore_ascii_case("from")).unwrap_or(false) && tokens.len() >= 5
    {
        tokens.remove(0); // from
        let origin = tokens.remove(0);
        return parse_relative_color_fn(&space, &origin, &tokens);
    }
    // Optional alpha after /
    let mut alpha = 1.0_f64;
    if let Some(pos) = tokens.iter().position(|t| t == "/") {
        if pos + 1 < tokens.len() {
            alpha = parse_alpha_component(&tokens[pos + 1])?;
        }
        tokens.truncate(pos);
    } else if tokens.len() >= 4 {
        // space-separated 4th as alpha
        if let Some(a) = parse_alpha_component(&tokens[3]) {
            alpha = a;
            tokens.truncate(3);
        }
    }
    if tokens.len() < 3 {
        return None;
    }
    let c0 = parse_color_component(&tokens[0], false)?;
    let c1 = parse_color_component(&tokens[1], true)?;
    let c2 = parse_color_component(&tokens[2], true)?;
    let rgb = match space.as_str() {
        "srgb" | "display-p3" | "a98-rgb" | "prophoto-rgb" | "rec2020" => {
            // Treat as sRGB 0..1 (display-p3 etc are approx).
            [
                (c0 * 255.0).round().clamp(0.0, 255.0) as u8,
                (c1 * 255.0).round().clamp(0.0, 255.0) as u8,
                (c2 * 255.0).round().clamp(0.0, 255.0) as u8,
            ]
        }
        "srgb-linear" => {
            let to_srgb = |c: f64| -> f64 {
                let c = c.clamp(0.0, 1.0);
                if c <= 0.0031308 {
                    12.92 * c
                } else {
                    1.055 * c.powf(1.0 / 2.4) - 0.055
                }
            };
            [
                (to_srgb(c0) * 255.0).round().clamp(0.0, 255.0) as u8,
                (to_srgb(c1) * 255.0).round().clamp(0.0, 255.0) as u8,
                (to_srgb(c2) * 255.0).round().clamp(0.0, 255.0) as u8,
            ]
        }
        "xyz" | "xyz-d65" | "xyz-d50" => {
            // Rough XYZ D65 → sRGB linear → sRGB.
            let (x, y, z) = (c0, c1, c2);
            let r = 3.2406 * x - 1.5372 * y - 0.4986 * z;
            let g = -0.9689 * x + 1.8758 * y + 0.0415 * z;
            let b = 0.0557 * x - 0.2040 * y + 1.0570 * z;
            let to_srgb = |c: f64| -> f64 {
                let c = c.clamp(0.0, 1.0);
                if c <= 0.0031308 {
                    12.92 * c
                } else {
                    1.055 * c.powf(1.0 / 2.4) - 0.055
                }
            };
            [
                (to_srgb(r) * 255.0).round().clamp(0.0, 255.0) as u8,
                (to_srgb(g) * 255.0).round().clamp(0.0, 255.0) as u8,
                (to_srgb(b) * 255.0).round().clamp(0.0, 255.0) as u8,
            ]
        }
        _ => return None,
    };
    Some((rgb, alpha.clamp(0.0, 1.0)))
}

fn parse_color_component(s: &str, signed: bool) -> Option<f64> {
    let s = s.trim();
    if let Some(p) = s.strip_suffix('%') {
        let n: f64 = p.trim().parse().ok()?;
        return Some((n / 100.0).clamp(if signed { -2.0 } else { 0.0 }, 2.0));
    }
    s.parse::<f64>().ok().map(|n| {
        if signed {
            n
        } else {
            n.clamp(0.0, 1.0)
        }
    })
}

fn parse_alpha_component(s: &str) -> Option<f64> {
    let s = s.trim();
    if let Some(p) = s.strip_suffix('%') {
        return Some((p.trim().parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0));
    }
    Some(s.parse::<f64>().ok()?.clamp(0.0, 1.0))
}

/// SVG 1.1 `device-cmyk(c, m, y, k [, a])` → approximate sRGB.
fn parse_device_cmyk(body: &str) -> Option<([u8; 3], f64)> {
    let tokens = split_css_color_args(body);
    if tokens.len() < 4 {
        return None;
    }
    let parse01 = |s: &str| -> Option<f64> {
        let s = s.trim();
        if let Some(p) = s.strip_suffix('%') {
            Some((p.trim().parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
        } else {
            Some(s.parse::<f64>().ok()?.clamp(0.0, 1.0))
        }
    };
    let c = parse01(&tokens[0])?;
    let m = parse01(&tokens[1])?;
    let y = parse01(&tokens[2])?;
    let k = parse01(&tokens[3])?;
    let alpha = if tokens.len() >= 5 {
        parse_alpha_component(&tokens[4])?
    } else {
        1.0
    };
    let r = (1.0 - c) * (1.0 - k);
    let g = (1.0 - m) * (1.0 - k);
    let b = (1.0 - y) * (1.0 - k);
    Some((
        [
            (r * 255.0).round().clamp(0.0, 255.0) as u8,
            (g * 255.0).round().clamp(0.0, 255.0) as u8,
            (b * 255.0).round().clamp(0.0, 255.0) as u8,
        ],
        alpha,
    ))
}

/// Parse `color-mix(in <space>, COLOR p%, COLOR q%)` body (without outer parens).
/// Interpolation space is ignored; resolved colours are mixed in sRGB.
fn parse_color_mix(body: &str) -> Option<([u8; 3], f64)> {
    // Strip optional "in <colorspace>," prefix.
    let rest = if let Some(r) = body.strip_prefix("in ") {
        let comma = r.find(',')?;
        r[comma + 1..].trim()
    } else {
        body
    };
    // Split on top-level commas into two color+percent parts.
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, ch) in rest.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(rest[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(rest[start..].trim());
    if parts.len() < 2 {
        return None;
    }
    let (c1, p1) = parse_color_mix_component(parts[0])?;
    let (c2, p2_opt) = parse_color_mix_component(parts[1])?;
    let p1 = p1.unwrap_or(0.5);
    let p2 = p2_opt.unwrap_or(1.0 - p1);
    let sum = p1 + p2;
    if sum <= 1e-12 {
        return None;
    }
    let w1 = p1 / sum;
    let w2 = p2 / sum;
    let rgb = [
        (c1.0[0] as f64 * w1 + c2.0[0] as f64 * w2)
            .round()
            .clamp(0.0, 255.0) as u8,
        (c1.0[1] as f64 * w1 + c2.0[1] as f64 * w2)
            .round()
            .clamp(0.0, 255.0) as u8,
        (c1.0[2] as f64 * w1 + c2.0[2] as f64 * w2)
            .round()
            .clamp(0.0, 255.0) as u8,
    ];
    let alpha = c1.1 * w1 + c2.1 * w2;
    Some((rgb, alpha.clamp(0.0, 1.0)))
}

fn parse_color_mix_component(s: &str) -> Option<(([u8; 3], f64), Option<f64>)> {
    let s = s.trim();
    // COLOR percentage
    let tokens: Vec<&str> = s.rsplitn(2, char::is_whitespace).collect();
    if tokens.len() == 2 {
        let maybe_pct = tokens[0].trim();
        let color_part = tokens[1].trim();
        if let Some(p) = maybe_pct.strip_suffix('%') {
            if let Ok(v) = p.parse::<f64>() {
                let c = parse_color(color_part)?;
                return Some((c, Some((v / 100.0).clamp(0.0, 1.0))));
            }
        }
    }
    // percentage COLOR
    let tokens_f: Vec<&str> = s.splitn(2, char::is_whitespace).collect();
    if tokens_f.len() == 2 {
        let maybe_pct = tokens_f[0].trim();
        let color_part = tokens_f[1].trim();
        if let Some(p) = maybe_pct.strip_suffix('%') {
            if let Ok(v) = p.parse::<f64>() {
                let c = parse_color(color_part)?;
                return Some((c, Some((v / 100.0).clamp(0.0, 1.0))));
            }
        }
    }
    let c = parse_color(s)?;
    Some((c, None))
}

/// CSS Color Level 5 `light-dark(lightColor, darkColor)` — prefer light.
fn parse_light_dark(body: &str) -> Option<([u8; 3], f64)> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, ch) in body.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(body[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(body[start..].trim());
    if parts.is_empty() {
        return None;
    }
    // CSS Color L5: pick light or dark arg from active color-scheme.
    let prefer_dark = active_color_scheme() == "dark";
    if prefer_dark {
        parts
            .get(1)
            .and_then(|p| parse_color(p))
            .or_else(|| parse_color(parts[0]))
    } else {
        parse_color(parts[0]).or_else(|| parts.get(1).and_then(|p| parse_color(p)))
    }
}

/// CSS Color L5 `contrast-color(<color>)` — return black or white with higher contrast
/// against the argument (WCAG relative luminance, soft).
fn parse_contrast_color(body: &str) -> Option<([u8; 3], f64)> {
    let (rgb, a) = parse_color(body.trim())?;
    // If base is nearly transparent, prefer black.
    if a < 0.08 {
        return Some(([0, 0, 0], 1.0));
    }
    let lum = relative_luminance(rgb);
    // WCAG: contrast vs white = (1.05)/(L+0.05); vs black = (L+0.05)/0.05.
    // Prefer white when base is dark (L < ~0.179 → white wins).
    if lum < 0.179 {
        Some(([255, 255, 255], 1.0))
    } else {
        Some(([0, 0, 0], 1.0))
    }
}

fn relative_luminance(rgb: [u8; 3]) -> f64 {
    let lin = |c: u8| {
        let s = c as f64 / 255.0;
        if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(rgb[0]) + 0.7152 * lin(rgb[1]) + 0.0722 * lin(rgb[2])
}



fn named_color(s: &str) -> Option<[u8; 3]> {
    // CSS Color Module Level 4 extended keywords (SVG/CSS shared set).
    Some(match s.to_ascii_lowercase().as_str() {
        "black" => [0, 0, 0],
        "silver" => [192, 192, 192],
        "gray" | "grey" => [128, 128, 128],
        "white" => [255, 255, 255],
        "maroon" => [128, 0, 0],
        "red" => [255, 0, 0],
        "purple" => [128, 0, 128],
        "fuchsia" | "magenta" => [255, 0, 255],
        "green" => [0, 128, 0],
        "lime" => [0, 255, 0],
        "olive" => [128, 128, 0],
        "yellow" => [255, 255, 0],
        "navy" => [0, 0, 128],
        "blue" => [0, 0, 255],
        "teal" => [0, 128, 128],
        "aqua" | "cyan" => [0, 255, 255],
        "orange" => [255, 165, 0],
        "pink" => [255, 192, 203],
        "brown" => [165, 42, 42],
        "aliceblue" => [240, 248, 255],
        "antiquewhite" => [250, 235, 215],
        "aquamarine" => [127, 255, 212],
        "azure" => [240, 255, 255],
        "beige" => [245, 245, 220],
        "bisque" => [255, 228, 196],
        "blanchedalmond" => [255, 235, 205],
        "blueviolet" => [138, 43, 226],
        "burlywood" => [222, 184, 135],
        "cadetblue" => [95, 158, 160],
        "chartreuse" => [127, 255, 0],
        "chocolate" => [210, 105, 30],
        "coral" => [255, 127, 80],
        "cornflowerblue" => [100, 149, 237],
        "cornsilk" => [255, 248, 220],
        "crimson" => [220, 20, 60],
        "darkblue" => [0, 0, 139],
        "darkcyan" => [0, 139, 139],
        "darkgoldenrod" => [184, 134, 11],
        "darkgray" | "darkgrey" => [169, 169, 169],
        "darkgreen" => [0, 100, 0],
        "darkkhaki" => [189, 183, 107],
        "darkmagenta" => [139, 0, 139],
        "darkolivegreen" => [85, 107, 47],
        "darkorange" => [255, 140, 0],
        "darkorchid" => [153, 50, 204],
        "darkred" => [139, 0, 0],
        "darksalmon" => [233, 150, 122],
        "darkseagreen" => [143, 188, 143],
        "darkslateblue" => [72, 61, 139],
        "darkslategray" | "darkslategrey" => [47, 79, 79],
        "darkturquoise" => [0, 206, 209],
        "darkviolet" => [148, 0, 211],
        "deeppink" => [255, 20, 147],
        "deepskyblue" => [0, 191, 255],
        "dimgray" | "dimgrey" => [105, 105, 105],
        "dodgerblue" => [30, 144, 255],
        "firebrick" => [178, 34, 34],
        "floralwhite" => [255, 250, 240],
        "forestgreen" => [34, 139, 34],
        "gainsboro" => [220, 220, 220],
        "ghostwhite" => [248, 248, 255],
        "gold" => [255, 215, 0],
        "goldenrod" => [218, 165, 32],
        "greenyellow" => [173, 255, 47],
        "honeydew" => [240, 255, 240],
        "hotpink" => [255, 105, 180],
        "indianred" => [205, 92, 92],
        "indigo" => [75, 0, 130],
        "ivory" => [255, 255, 240],
        "khaki" => [240, 230, 140],
        "lavender" => [230, 230, 250],
        "lavenderblush" => [255, 240, 245],
        "lawngreen" => [124, 252, 0],
        "lemonchiffon" => [255, 250, 205],
        "lightblue" => [173, 216, 230],
        "lightcoral" => [240, 128, 128],
        "lightcyan" => [224, 255, 255],
        "lightgoldenrodyellow" => [250, 250, 210],
        "lightgray" | "lightgrey" => [211, 211, 211],
        "lightgreen" => [144, 238, 144],
        "lightpink" => [255, 182, 193],
        "lightsalmon" => [255, 160, 122],
        "lightseagreen" => [32, 178, 170],
        "lightskyblue" => [135, 206, 250],
        "lightslategray" | "lightslategrey" => [119, 136, 153],
        "lightsteelblue" => [176, 196, 222],
        "lightyellow" => [255, 255, 224],
        "limegreen" => [50, 205, 50],
        "linen" => [250, 240, 230],
        "mediumaquamarine" => [102, 205, 170],
        "mediumblue" => [0, 0, 205],
        "mediumorchid" => [186, 85, 211],
        "mediumpurple" => [147, 112, 219],
        "mediumseagreen" => [60, 179, 113],
        "mediumslateblue" => [123, 104, 238],
        "mediumspringgreen" => [0, 250, 154],
        "mediumturquoise" => [72, 209, 204],
        "mediumvioletred" => [199, 21, 133],
        "midnightblue" => [25, 25, 112],
        "mintcream" => [245, 255, 250],
        "mistyrose" => [255, 228, 225],
        "moccasin" => [255, 228, 181],
        "navajowhite" => [255, 222, 173],
        "oldlace" => [253, 245, 230],
        "olivedrab" => [107, 142, 35],
        "orangered" => [255, 69, 0],
        "orchid" => [218, 112, 214],
        "palegoldenrod" => [238, 232, 170],
        "palegreen" => [152, 251, 152],
        "paleturquoise" => [175, 238, 238],
        "palevioletred" => [219, 112, 147],
        "papayawhip" => [255, 239, 213],
        "peachpuff" => [255, 218, 185],
        "peru" => [205, 133, 63],
        "plum" => [221, 160, 221],
        "powderblue" => [176, 224, 230],
        "rebeccapurple" => [102, 51, 153],
        "rosybrown" => [188, 143, 143],
        "royalblue" => [65, 105, 225],
        "saddlebrown" => [139, 69, 19],
        "salmon" => [250, 128, 114],
        "sandybrown" => [244, 164, 96],
        "seagreen" => [46, 139, 87],
        "seashell" => [255, 245, 238],
        "sienna" => [160, 82, 45],
        "skyblue" => [135, 206, 235],
        "slateblue" => [106, 90, 205],
        "slategray" | "slategrey" => [112, 128, 144],
        "snow" => [255, 250, 250],
        "springgreen" => [0, 255, 127],
        "steelblue" => [70, 130, 180],
        "tan" => [210, 180, 140],
        "thistle" => [216, 191, 216],
        "tomato" => [255, 99, 71],
        "turquoise" => [64, 224, 208],
        "violet" => [238, 130, 238],
        "wheat" => [245, 222, 179],
        "whitesmoke" => [245, 245, 245],
        "yellowgreen" => [154, 205, 50],
        "transparent" => return None,
        _ => return None,
    })
}

pub fn parse_percent_or_number(s: &str, default: f64) -> f64 {
    let s = s.trim();
    if let Some(p) = s.strip_suffix('%') {
        p.parse::<f64>().unwrap_or(default * 100.0) / 100.0
    } else {
        s.parse().unwrap_or(default)
    }
}

/// Resolve a paint value (`fill` / `stroke`) against the gradient table.
///
/// `bbox` is the element bounding box in user space for objectBoundingBox units.
/// Default fill when attribute is missing is black (SVG); use `default_black`.
pub fn resolve_paint(
    value: Option<&str>,
    opacity: f64,
    gradients: &GradientMap,
    solids: &SolidColorMap,
    hatches: &HatchMap,
    bbox: (f64, f64, f64, f64),
    default_black: bool,
    current_color: Option<&str>,
) -> Paint {
    resolve_paint_ex(
        value,
        opacity,
        gradients,
        solids,
        hatches,
        bbox,
        default_black,
        current_color,
        None,
    )
}

/// Like [`resolve_paint`], with optional inherited `color-interpolation` (linearRGB).
pub fn resolve_paint_ex(
    value: Option<&str>,
    opacity: f64,
    gradients: &GradientMap,
    solids: &SolidColorMap,
    hatches: &HatchMap,
    bbox: (f64, f64, f64, f64),
    default_black: bool,
    current_color: Option<&str>,
    color_interpolation: Option<&str>,
) -> Paint {
    resolve_paint_ex2(
        value,
        opacity,
        gradients,
        solids,
        hatches,
        bbox,
        default_black,
        current_color,
        color_interpolation,
        None,
    )
}

/// densify_samples: Some(n) overrides linearRGB densify count; Some(0) disables.
pub fn resolve_paint_ex2(
    value: Option<&str>,
    opacity: f64,
    gradients: &GradientMap,
    solids: &SolidColorMap,
    hatches: &HatchMap,
    bbox: (f64, f64, f64, f64),
    default_black: bool,
    current_color: Option<&str>,
    color_interpolation: Option<&str>,
    densify_samples: Option<usize>,
) -> Paint {
    let Some(raw) = value.map(str::trim).filter(|s| !s.is_empty()) else {
        return if default_black {
            Paint::Solid {
                rgb: [0, 0, 0],
                alpha: opacity.clamp(0.0, 1.0),
            }
        } else {
            Paint::None
        };
    };
    if raw.eq_ignore_ascii_case("none") {
        return Paint::None;
    }
    if raw.eq_ignore_ascii_case("currentcolor") {
        return current_color
            .and_then(parse_color)
            .map(|(rgb, alpha)| Paint::Solid {
                rgb,
                alpha: (alpha * opacity).clamp(0.0, 1.0),
            })
            .unwrap_or(Paint::None);
    }
    // Marker/SVG2 context paints are rewritten by the walker before resolve.
    // Fall through to none if they leak here unresolved.
    if raw.eq_ignore_ascii_case("context-fill") || raw.eq_ignore_ascii_case("context-stroke") {
        return Paint::None;
    }
    if let Some(rest) = raw.strip_prefix("url(") {
        let id = rest
            .trim()
            .trim_start_matches('#')
            .trim_end_matches(')')
            .trim();
        // url(#id) form
        let id = id
            .trim_start_matches('#')
            .trim_matches(|c| c == '"' || c == '\'');
        if let Some(g) = gradients.get(id) {
            return gradient_to_paint(
                g,
                opacity,
                bbox,
                current_color,
                color_interpolation,
                densify_samples,
            );
        }
        if let Some(s) = solids.get(id) {
            if s.current_color {
                return current_color
                    .and_then(parse_color)
                    .map(|(rgb, a)| Paint::Solid {
                        rgb,
                        alpha: (s.alpha * a * opacity).clamp(0.0, 1.0),
                    })
                    .unwrap_or(Paint::None);
            }
            return Paint::Solid {
                rgb: s.rgb,
                alpha: (s.alpha * opacity).clamp(0.0, 1.0),
            };
        }
        if let Some(h) = hatches.get(id) {
            return Paint::PatternFill {
                prst: h.prst.clone(),
                fg: h.fg,
                fg_alpha: (h.fg_alpha * opacity).clamp(0.0, 1.0),
                bg: h.bg,
                bg_alpha: (h.bg_alpha * opacity).clamp(0.0, 1.0),
            };
        }
        return Paint::None;
    }
    if let Some((rgb, a)) = parse_color(raw) {
        return Paint::Solid {
            rgb,
            alpha: (a * opacity).clamp(0.0, 1.0),
        };
    }
    Paint::None
}

fn gradient_to_paint(
    g: &Gradient,
    opacity: f64,
    bbox: (f64, f64, f64, f64),
    current_color: Option<&str>,
    color_interpolation_override: Option<&str>,
    densify_samples: Option<usize>,
) -> Paint {
    if g.stops.is_empty() {
        return Paint::None;
    }
    // SVG: with gradientUnits="objectBoundingBox", a zero-width or zero-height
    // object bbox makes the gradient paint nothing (transparent). Chrome skips
    // painting e.g. vertical <line> strokes with diagonal objectBoundingBox
    // gradients; emitting a solid/angle gradient here was a major residual.
    if g.units_object_bbox {
        let bw = (bbox.2 - bbox.0).abs();
        let bh = (bbox.3 - bbox.1).abs();
        if bw < 1e-6 || bh < 1e-6 {
            return Paint::None;
        }
    }
    let cc = current_color.and_then(parse_color);
    let mut stops: Vec<ColorStop> = g
        .stops
        .iter()
        .map(|s| {
            let (rgb, a) = if s.current_color {
                cc.unwrap_or((s.rgb, 1.0))
            } else {
                (s.rgb, 1.0)
            };
            ColorStop {
                offset: s.offset.clamp(0.0, 1.0),
                rgb,
                alpha: (s.alpha * a * opacity).clamp(0.0, 1.0),
                current_color: false,
            }
        })
        .collect();
    stops.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());

    // SVG `color-interpolation="linearRGB"`: DrawingML stop lerp is sRGB-ish;
    // densify intermediate stops so piecewise-sRGB approximates linear-light blend.
    let ci = color_interpolation_override
        .unwrap_or(g.color_interpolation.as_str());
    // Gradient element's own non-default wins; otherwise inherited override.
    let ci = if !g.color_interpolation.eq_ignore_ascii_case("srgb")
        && !g.color_interpolation.eq_ignore_ascii_case("auto")
        && !g.color_interpolation.is_empty()
    {
        g.color_interpolation.as_str()
    } else {
        ci
    };
    if ci.eq_ignore_ascii_case("linearrgb") {
        let n = densify_samples.unwrap_or(6);
        if n > 0 {
            stops = densify_stops_linear_rgb(stops, n);
        }
    }

    // SVG 2 radial `fr`: focus circle radius. Approximate by holding the first
    // stop solid until fr/r of the gradient radius, then interpolating.
    if let GradientKind::Radial { r, fr, .. } = &g.kind {
        if *fr > 1e-9 && *r > 1e-9 && !stops.is_empty() {
            let t = (*fr / *r).clamp(0.0, 0.999);
            if t > 1e-6 {
                let first = stops[0].clone();
                if first.offset > t + 1e-9 {
                    stops.insert(
                        0,
                        ColorStop {
                            offset: t,
                            rgb: first.rgb,
                            alpha: first.alpha,
                            current_color: false,
                        },
                    );
                } else if (first.offset - 0.0).abs() < 1e-9 {
                    // Duplicate first stop at fr so DrawingML holds color to that radius.
                    stops.insert(
                        1,
                        ColorStop {
                            offset: t,
                            rgb: first.rgb,
                            alpha: first.alpha,
                            current_color: false,
                        },
                    );
                }
            }
        }
    }

    // LibreOffice often ignores per-stop alpha on a:gradFill (paints opaque).
    // When the whole gradient is uniformly translucent and low-opacity, collapse
    // to solidFill with alpha — correct compositing over dark fills (e.g. the
    // node inner r=45 primaryGradient @ opacity 0.1).
    let max_a = stops.iter().map(|s| s.alpha).fold(0.0_f64, f64::max);
    let min_a = stops.iter().map(|s| s.alpha).fold(1.0_f64, f64::min);
    let uniform_alpha = (max_a - min_a) < 0.02;
    if uniform_alpha && max_a > 0.001 && max_a < 0.35 {
        // Weighted mid colour by stop spacing
        let mut acc = [0.0_f64; 3];
        let mut wsum = 0.0_f64;
        for i in 0..stops.len() {
            let w = if stops.len() == 1 {
                1.0
            } else if i == 0 {
                (stops[1].offset - stops[0].offset).max(0.01)
            } else if i + 1 == stops.len() {
                (stops[i].offset - stops[i - 1].offset).max(0.01)
            } else {
                ((stops[i + 1].offset - stops[i - 1].offset) * 0.5).max(0.01)
            };
            acc[0] += stops[i].rgb[0] as f64 * w;
            acc[1] += stops[i].rgb[1] as f64 * w;
            acc[2] += stops[i].rgb[2] as f64 * w;
            wsum += w;
        }
        let rgb = [
            (acc[0] / wsum).round().clamp(0.0, 255.0) as u8,
            (acc[1] / wsum).round().clamp(0.0, 255.0) as u8,
            (acc[2] / wsum).round().clamp(0.0, 255.0) as u8,
        ];
        // LibreOffice often ignores a:alpha on solidFill/gradFill and paints
        // opaque. Bake low alpha over a dark slate (#0B0D13) — correct for
        // this presentation's dark backgrounds; general hosts still get a
        // reasonable translucent-looking solid.
        let bg = [11.0_f64, 13.0, 19.0];
        let a = max_a;
        let baked = [
            (bg[0] * (1.0 - a) + rgb[0] as f64 * a)
                .round()
                .clamp(0.0, 255.0) as u8,
            (bg[1] * (1.0 - a) + rgb[1] as f64 * a)
                .round()
                .clamp(0.0, 255.0) as u8,
            (bg[2] * (1.0 - a) + rgb[2] as f64 * a)
                .round()
                .clamp(0.0, 255.0) as u8,
        ];
        return Paint::Solid {
            rgb: baked,
            alpha: 1.0,
        };
    }

    match g.kind {
        GradientKind::Linear { x1, y1, x2, y2 } => {
            // Map to user space if objectBoundingBox
            let (ux1, uy1, ux2, uy2) = if g.units_object_bbox {
                let (bx, by, bw, bh) = (
                    bbox.0,
                    bbox.1,
                    (bbox.2 - bbox.0).max(1e-9),
                    (bbox.3 - bbox.1).max(1e-9),
                );
                (bx + x1 * bw, by + y1 * bh, bx + x2 * bw, by + y2 * bh)
            } else {
                (x1, y1, x2, y2)
            };
            let (ux1, uy1) = g.transform.map_point(ux1, uy1);
            let (ux2, uy2) = g.transform.map_point(ux2, uy2);
            // DrawingML linear angle: 0 = left→right, increases clockwise, unit 1/60000 deg
            let dx = ux2 - ux1;
            let dy = uy2 - uy1;
            // SVG y-down; DrawingML y-down in shapes too for fills in OOXML presentation
            // Angle from positive x-axis, clockwise: atan2(dy, dx)
            let deg = dy.atan2(dx).to_degrees();
            // OOXML: 0 at left-to-right, clockwise positive
            let mut ang = deg;
            if ang < 0.0 {
                ang += 360.0;
            }
            let angle = (ang * 60_000.0).round() as i32;
            {
                let flip = spread_to_flip(&g.spread_method);
                Paint::LinearGradient { stops, angle, flip }
            }
        }
        GradientKind::Radial {
            cx,
            cy,
            r,
            fx: _,
            fy: _,
            fr: _,
        } => {
            // Map focus into bbox-normalized 0..1 for DrawingML fillToRect.
            let (ncx, ncy, nr) = if g.units_object_bbox {
                // already in 0..1 (or slightly outside)
                (cx, cy, r)
            } else {
                let (bx, by, bx1, by1) = bbox;
                let bw = (bx1 - bx).max(1e-9);
                let bh = (by1 - by).max(1e-9);
                let (ux, uy) = g.transform.map_point(cx, cy);
                (
                    ((ux - bx) / bw).clamp(-1.0, 2.0),
                    ((uy - by) / bh).clamp(-1.0, 2.0),
                    (r / bw.min(bh)).clamp(0.01, 10.0),
                )
            };
            Paint::RadialGradient {
                stops,
                cx: ncx,
                cy: ncy,
                r: nr,
                flip: spread_to_flip(&g.spread_method),
            }
        }
    }
}


/// sRGB channel 0..1 → linear-light.
fn srgb_to_linear(c: f64) -> f64 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// linear-light → sRGB channel 0..1.
fn linear_to_srgb(c: f64) -> f64 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn rgb_u8_to_linear(rgb: [u8; 3]) -> [f64; 3] {
    [
        srgb_to_linear(rgb[0] as f64 / 255.0),
        srgb_to_linear(rgb[1] as f64 / 255.0),
        srgb_to_linear(rgb[2] as f64 / 255.0),
    ]
}

fn linear_to_rgb_u8(lin: [f64; 3]) -> [u8; 3] {
    [
        (linear_to_srgb(lin[0]) * 255.0).round().clamp(0.0, 255.0) as u8,
        (linear_to_srgb(lin[1]) * 255.0).round().clamp(0.0, 255.0) as u8,
        (linear_to_srgb(lin[2]) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

/// Insert intermediate stops so sRGB piecewise lerp ≈ linearRGB blend.
fn densify_stops_linear_rgb(stops: Vec<ColorStop>, samples_per_span: usize) -> Vec<ColorStop> {
    if stops.len() < 2 || samples_per_span == 0 {
        return stops;
    }
    let mut out = Vec::with_capacity(stops.len() + (stops.len() - 1) * samples_per_span);
    out.push(stops[0].clone());
    for i in 0..stops.len() - 1 {
        let a = &stops[i];
        let b = &stops[i + 1];
        let la = rgb_u8_to_linear(a.rgb);
        let lb = rgb_u8_to_linear(b.rgb);
        for k in 1..=samples_per_span {
            let t = k as f64 / (samples_per_span as f64 + 1.0);
            let off = a.offset + (b.offset - a.offset) * t;
            let lin = [
                la[0] + (lb[0] - la[0]) * t,
                la[1] + (lb[1] - la[1]) * t,
                la[2] + (lb[2] - la[2]) * t,
            ];
            let alpha = a.alpha + (b.alpha - a.alpha) * t;
            out.push(ColorStop {
                offset: off,
                rgb: linear_to_rgb_u8(lin),
                alpha,
                current_color: false,
            });
        }
        out.push(b.clone());
    }
    out
}

pub fn rgb_hex(rgb: [u8; 3]) -> String {
    format!("{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

pub fn alpha_val(a: f64) -> i64 {
    ((a.clamp(0.0, 1.0) * 100_000.0).round() as i64).clamp(0, 100_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::svg_to_shapes::matrix::Matrix;

    #[test]
    fn parses_modern_css_colors_and_alpha_hex() {
        assert_eq!(
            parse_color("#12345678"),
            Some(([0x12, 0x34, 0x56], 0x78 as f64 / 255.0))
        );
        assert_eq!(
            parse_color("#abcd"),
            Some(([0xaa, 0xbb, 0xcc], 0xdd as f64 / 255.0))
        );
        assert_eq!(
            parse_color("RGB(100% 0% 50% / 25%)"),
            Some(([255, 0, 128], 0.25))
        );
        assert_eq!(
            parse_color("hsl(0 100% 50% / 50%)"),
            Some(([255, 0, 0], 0.5))
        );
        assert_eq!(parse_color("transparent"), Some(([0, 0, 0], 0.0)));
        assert_eq!(parse_color("rebeccapurple"), Some(([102, 51, 153], 1.0)));
        assert_eq!(parse_color("cornflowerblue"), Some(([100, 149, 237], 1.0)));
        // CSS Color L4 `none` missing components → 0.
        assert_eq!(parse_color("rgb(none none none)").unwrap().0, [0, 0, 0]);
        assert_eq!(parse_color("hsl(none 100% 50%)").unwrap().0, [255, 0, 0]); // hue 0 = red
        assert_eq!(parse_color("oklab(none 0 0)").unwrap().0, [0, 0, 0]);
        let rel = parse_color("rgb(from #ff0000 none g b)").unwrap();
        assert_eq!(rel.0[0], 0);
        assert!(rel.0[1] < 10 && rel.0[2] < 10, "g/b from red near 0, got {:?}", rel.0);

        // Hue units + modern number form for S/L (CSS Color L4).
        assert_eq!(parse_color("hsl(120deg 100% 50%)").unwrap().0, [0, 255, 0]);
        assert_eq!(parse_color("hsl(0.5turn 100% 50%)").unwrap().0, [0, 255, 255]);
        assert_eq!(parse_color("hsl(120 1 0.5)").unwrap().0, [0, 255, 0]); // number S/L
        assert_eq!(parse_color("hwb(120deg 0% 0%)").unwrap().0, [0, 255, 0]);
        assert_eq!(parse_color("hwb(none 0% 0%)").unwrap().0, [255, 0, 0]);
        // none hue treated as 0
        let _ = parse_color("oklch(0.628 0.258 29.2deg)");

        // hwb(0 0% 0%) ≡ pure red
        assert_eq!(parse_color("hwb(0 0% 0%)"), Some(([255, 0, 0], 1.0)));
        // hwb with alpha
        let hwb_a = parse_color("hwb(120 0% 0% / 50%)").unwrap();
        assert_eq!(hwb_a.0, [0, 255, 0]);
        assert!((hwb_a.1 - 0.5).abs() < 1e-6);
        // oklab pure white-ish: L=1 a=0 b=0
        let ok = parse_color("oklab(1 0 0)").unwrap();
        assert!(ok.0[0] > 250 && ok.0[1] > 250 && ok.0[2] > 250, "oklab white {:?}", ok.0);
        // oklch red-ish: L=0.628 C=0.258 H=29.2 ≈ sRGB red
        let okl = parse_color("oklch(0.628 0.258 29.2)").unwrap();
        assert!(okl.0[0] > 200 && okl.0[1] < 80 && okl.0[2] < 80, "oklch red {:?}", okl.0);
        let okl_a = parse_color("oklch(0.5 0 0 / 40%)").unwrap();
        assert!((okl_a.1 - 0.4).abs() < 1e-6);
        // system colors
        assert_eq!(parse_color("CanvasText"), Some(([0, 0, 0], 1.0)));
        assert_eq!(parse_color("Highlight"), Some(([0, 120, 215], 1.0)));
        // color-mix in srgb 50/50 red+blue ≈ purple
        let mix = parse_color("color-mix(in srgb, red 50%, blue)").unwrap();
        assert!((mix.0[0] as i32 - 128).abs() <= 1, "mix r {:?}", mix);
        assert_eq!(mix.0[1], 0);
        assert!((mix.0[2] as i32 - 128).abs() <= 1, "mix b {:?}", mix);
        // color-mix 100% red
        let mix2 = parse_color("color-mix(in srgb, #ff0000 100%, #0000ff)").unwrap();
        assert_eq!(mix2.0, [255, 0, 0]);
        // light-dark prefers light
        assert_eq!(parse_color("light-dark(#ff0000, #0000ff)").unwrap().0, [255, 0, 0]);
        // contrast-color: dark base → white; light base → black
        assert_eq!(parse_color("contrast-color(#000000)").unwrap().0, [255, 255, 255]);
        assert_eq!(parse_color("contrast-color(#ffffff)").unwrap().0, [0, 0, 0]);
        assert_eq!(parse_color("contrast-color(black)").unwrap().0, [255, 255, 255]);
        assert_eq!(parse_color("color-contrast(#000)").unwrap().0, [255, 255, 255]);
    }

    #[test]
    fn object_bbox_gradient_on_zero_width_is_none() {
        let g = Gradient {
            kind: GradientKind::Linear {
                x1: 0.0,
                y1: 0.0,
                x2: 1.0,
                y2: 1.0,
            },
            stops: vec![
                ColorStop {
                    offset: 0.0,
                    rgb: [37, 99, 235],
                    alpha: 1.0,
                    current_color: false,
                },
                ColorStop {
                    offset: 1.0,
                    rgb: [96, 165, 250],
                    alpha: 1.0,
                    current_color: false,
                },
            ],
            transform: Matrix::identity(),
            units_object_bbox: true,
            spread_method: "pad".into(),
            color_interpolation: "sRGB".into(),
        };
        // Vertical line: width 0, height 60 — Chrome paints nothing.
        let paint = gradient_to_paint(&g, 1.0, (90.0, 340.0, 90.0, 400.0), None, None, None);
        assert!(matches!(paint, Paint::None));
        // Non-degenerate bbox still resolves.
        let paint2 = gradient_to_paint(&g, 1.0, (90.0, 340.0, 100.0, 400.0), None, None, None);
        assert!(matches!(paint2, Paint::LinearGradient { .. }));
    }

    #[test]
    fn color_function_srgb_and_device_cmyk() {
        let c = parse_color("color(srgb 1 0 0)").unwrap();
        assert_eq!(c.0, [255, 0, 0]);
        let c2 = parse_color("color(srgb 0% 50% 100% / 50%)").unwrap();
        assert_eq!(c2.0, [0, 128, 255]);
        assert!((c2.1 - 0.5).abs() < 1e-6);
        let k = parse_color("device-cmyk(0, 1, 1, 0)").unwrap();
        // full M+Y → red
        assert_eq!(k.0, [255, 0, 0]);
        // color-mix in hsl still works (sRGB mix approx)
        let mix = parse_color("color-mix(in hsl, red 50%, blue)").unwrap();
        assert!(mix.0[0] > 50 && mix.0[2] > 50);
    }

    #[test]
    fn relative_color_rgb_from() {
        let c = parse_color("rgb(from #336699 r g b)").unwrap();
        assert_eq!(c.0, [0x33, 0x66, 0x99]);
        let c2 = parse_color("rgb(from red g r b)").unwrap();
        // swap r/g of red → green
        assert_eq!(c2.0, [0, 255, 0]);
        let c3 = parse_color("hsl(from #ff0000 h s l)").unwrap();
        assert_eq!(c3.0, [255, 0, 0]);
    }

    #[test]
    fn relative_color_oklch_hwb_from() {
        // Identity oklch(from red l c h) ≈ red
        let c = parse_color("oklch(from red l c h)").unwrap();
        assert!(c.0[0] > 200 && c.0[1] < 40 && c.0[2] < 40, "oklch from red {:?}", c.0);
        // Fixed hue channel → green-ish
        let c2 = parse_color("oklch(from #ff0000 l c 120)").unwrap();
        assert!(c2.0[1] > c2.0[0] && c2.0[1] > c2.0[2], "hue 120 greenish {:?}", c2.0);
        // calc() channel math: red hue + 120 → green-ish
        let c2b = parse_color("oklch(from #ff0000 l c calc(h + 120))").unwrap();
        assert!(
            c2b.0[1] > c2b.0[0] && c2b.0[1] > c2b.0[2],
            "calc(h+120) greenish {:?}",
            c2b.0
        );
        // hwb identity
        let c3 = parse_color("hwb(from red h w b)").unwrap();
        assert_eq!(c3.0, [255, 0, 0]);
        // oklab identity
        let c4 = parse_color("oklab(from #336699 l a b)").unwrap();
        assert!(
            (c4.0[0] as i16 - 0x33).abs() < 3
                && (c4.0[1] as i16 - 0x66).abs() < 3
                && (c4.0[2] as i16 - 0x99).abs() < 3,
            "oklab roundtrip {:?}",
            c4.0
        );
        let c5 = parse_color("lab(from #336699 l a b)").unwrap();
        assert!(
            (c5.0[0] as i16 - 0x33).abs() < 4
                && (c5.0[1] as i16 - 0x66).abs() < 4
                && (c5.0[2] as i16 - 0x99).abs() < 4,
            "lab roundtrip {:?}",
            c5.0
        );
        let c6 = parse_color("lch(from red l c h)").unwrap();
        assert!(c6.0[0] > 200 && c6.0[1] < 40 && c6.0[2] < 40, "lch from red {:?}", c6.0);
        // color(from … srgb …) and color(srgb from …)
        let c7 = parse_color("color(from #336699 srgb r g b)").unwrap();
        assert_eq!(c7.0, [0x33, 0x66, 0x99]);
        let c8 = parse_color("color(srgb from red g r b)").unwrap();
        assert_eq!(c8.0, [0, 255, 0]);
    }

    #[test]
    fn linear_rgb_densify_darkens_mid_red_green() {
        // sRGB mid of red→green is bright yellow-ish; linearRGB mid is darker olive.
        let g = Gradient {
            kind: GradientKind::Linear {
                x1: 0.0,
                y1: 0.0,
                x2: 1.0,
                y2: 0.0,
            },
            stops: vec![
                ColorStop {
                    offset: 0.0,
                    rgb: [255, 0, 0],
                    alpha: 1.0,
                    current_color: false,
                },
                ColorStop {
                    offset: 1.0,
                    rgb: [0, 255, 0],
                    alpha: 1.0,
                    current_color: false,
                },
            ],
            transform: Matrix::identity(),
            units_object_bbox: true,
            spread_method: "pad".into(),
            color_interpolation: "linearRGB".into(),
        };
        let paint = gradient_to_paint(&g, 1.0, (0.0, 0.0, 100.0, 100.0), None, None, None);
        match paint {
            Paint::LinearGradient { stops, .. } => {
                assert!(stops.len() > 2, "densified len={}", stops.len());
                // Find stop nearest 0.5
                let mid = stops
                    .iter()
                    .min_by(|a, b| {
                        (a.offset - 0.5)
                            .abs()
                            .partial_cmp(&(b.offset - 0.5).abs())
                            .unwrap()
                    })
                    .unwrap();
                // Linear-light mid of (1,0,0) and (0,1,0) → sRGB ~ (188, 188, 0) not (128,128,0)
                // and channel values should be below 200 (not full bright yellow).
                assert!(
                    mid.rgb[0] < 220 && mid.rgb[1] < 220,
                    "mid={:?}",
                    mid.rgb
                );
                assert!(mid.rgb[2] < 40, "mid blue {:?}", mid.rgb);
            }
            other => panic!("expected linear grad, got {other:?}"),
        }

    #[test]
    fn light_dark_respects_color_scheme() {
        let light = with_color_scheme("light", || parse_color("light-dark(#ff0000, #0000ff)"));
        let dark = with_color_scheme("dark", || parse_color("light-dark(#ff0000, #0000ff)"));
        assert_eq!(light.unwrap().0, [255, 0, 0]);
        assert_eq!(dark.unwrap().0, [0, 0, 255]);
    }

    #[test]
    fn accent_color_env_overrides_system() {
        let c = with_paint_color_env("light", Some("#112233"), || parse_color("AccentColor"));
        assert_eq!(c.unwrap().0, [0x11, 0x22, 0x33]);
        let d = with_paint_color_env("light", None, || parse_color("AccentColor"));
        assert_eq!(d.unwrap().0, [0, 120, 215]);
    }
    }
}
