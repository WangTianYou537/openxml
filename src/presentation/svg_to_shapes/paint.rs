//! Paint server resolution: solid colors, linear/radial gradients, opacity.
//! Gradients map to DrawingML `a:gradFill` when possible.

use super::matrix::Matrix;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ColorStop {
    pub offset: f64,
    pub rgb: [u8; 3],
    pub alpha: f64,
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
    },
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub kind: GradientKind,
    pub stops: Vec<ColorStop>,
    pub transform: Matrix,
    pub units_object_bbox: bool,
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
    },
    RadialGradient {
        stops: Vec<ColorStop>,
    },
}

pub type GradientMap = HashMap<String, Gradient>;

pub fn parse_color(s: &str) -> Option<([u8; 3], f64)> {
    let s = s.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") || s.eq_ignore_ascii_case("transparent") {
        return None;
    }
    if let Some(hex) = s.strip_prefix('#') {
        return parse_hex(hex).map(|rgb| (rgb, 1.0));
    }
    if let Some(rest) = s.strip_prefix("rgb(").or_else(|| s.strip_prefix("rgba(")) {
        let rest = rest.trim_end_matches(')');
        let parts: Vec<&str> = rest.split(',').map(|p| p.trim()).collect();
        if parts.len() >= 3 {
            let r = parse_component(parts[0])?;
            let g = parse_component(parts[1])?;
            let b = parse_component(parts[2])?;
            let a = if parts.len() >= 4 {
                parts[3].parse().unwrap_or(1.0)
            } else {
                1.0
            };
            return Some(([r, g, b], a));
        }
    }
    named_color(s).map(|rgb| (rgb, 1.0))
}

fn parse_component(s: &str) -> Option<u8> {
    if let Some(p) = s.strip_suffix('%') {
        let v: f64 = p.parse().ok()?;
        Some((v / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8)
    } else {
        let v: f64 = s.parse().ok()?;
        Some(v.round().clamp(0.0, 255.0) as u8)
    }
}

fn parse_hex(hex: &str) -> Option<[u8; 3]> {
    match hex.len() {
        3 => {
            let b = hex.as_bytes();
            let n = |i: usize| {
                let c = b[i] as char;
                let v = c.to_digit(16)? as u8;
                Some(v * 16 + v)
            };
            Some([n(0)?, n(1)?, n(2)?])
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some([r, g, b])
        }
        8 => {
            // #RRGGBBAA — ignore AA here (caller uses opacity)
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some([r, g, b])
        }
        _ => None,
    }
}

fn named_color(s: &str) -> Option<[u8; 3]> {
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
    bbox: (f64, f64, f64, f64),
    default_black: bool,
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
        return Paint::Solid {
            rgb: [0, 0, 0],
            alpha: opacity.clamp(0.0, 1.0),
        };
    }
    if let Some(rest) = raw.strip_prefix("url(") {
        let id = rest
            .trim()
            .trim_start_matches('#')
            .trim_end_matches(')')
            .trim();
        // url(#id) form
        let id = id.trim_start_matches('#').trim_matches(|c| c == '"' || c == '\'');
        if let Some(g) = gradients.get(id) {
            return gradient_to_paint(g, opacity, bbox);
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

fn gradient_to_paint(g: &Gradient, opacity: f64, bbox: (f64, f64, f64, f64)) -> Paint {
    if g.stops.is_empty() {
        return Paint::None;
    }
    let mut stops: Vec<ColorStop> = g
        .stops
        .iter()
        .map(|s| ColorStop {
            offset: s.offset.clamp(0.0, 1.0),
            rgb: s.rgb,
            alpha: (s.alpha * opacity).clamp(0.0, 1.0),
        })
        .collect();
    stops.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());

    match g.kind {
        GradientKind::Linear { x1, y1, x2, y2 } => {
            // Map to user space if objectBoundingBox
            let (ux1, uy1, ux2, uy2) = if g.units_object_bbox {
                let (bx, by, bw, bh) = (bbox.0, bbox.1, (bbox.2 - bbox.0).max(1e-9), (bbox.3 - bbox.1).max(1e-9));
                (
                    bx + x1 * bw,
                    by + y1 * bh,
                    bx + x2 * bw,
                    by + y2 * bh,
                )
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
            Paint::LinearGradient { stops, angle }
        }
        GradientKind::Radial { .. } => Paint::RadialGradient { stops },
    }
}

pub fn rgb_hex(rgb: [u8; 3]) -> String {
    format!("{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

pub fn alpha_val(a: f64) -> i64 {
    ((a.clamp(0.0, 1.0) * 100_000.0).round() as i64).clamp(0, 100_000)
}
