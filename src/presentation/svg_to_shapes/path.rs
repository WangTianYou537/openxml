//! SVG path `d` attribute parser and geometry helpers (MDN / SVG2).
//!
//! Supports M/L/H/V/C/S/Q/T/A/Z (absolute + relative) with implicit command
//! repetition. Arcs are flattened to cubic Béziers for DrawingML `custGeom`.

use super::matrix::{parse_numbers, Matrix};

/// Absolute path segment after normalization.
#[derive(Debug, Clone)]
pub enum Segment {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CubicTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    QuadTo { x1: f64, y1: f64, x: f64, y: f64 },
    Close,
}

/// Parse SVG path data into absolute segments. Arcs are expanded to cubics.
pub fn parse_path(d: &str) -> Vec<Segment> {
    let tokens = tokenize(d);
    let mut segs = Vec::new();
    let mut i = 0;
    let mut cursor = (0.0_f64, 0.0_f64);
    let mut subpath_start = (0.0_f64, 0.0_f64);
    let mut last_cubic_ctrl: Option<(f64, f64)> = None;
    let mut last_quad_ctrl: Option<(f64, f64)> = None;
    let mut cmd: Option<char> = None;

    while i < tokens.len() {
        if let Token::Cmd(c) = tokens[i] {
            cmd = Some(c);
            i += 1;
            if c == 'Z' || c == 'z' {
                segs.push(Segment::Close);
                cursor = subpath_start;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                cmd = None;
            }
            continue;
        }
        let Some(c) = cmd else {
            i += 1;
            continue;
        };

        match c {
            'M' | 'm' => {
                let rel = c == 'm';
                if let Some((x, y)) = read_pair(&tokens, &mut i) {
                    if rel {
                        cursor.0 += x;
                        cursor.1 += y;
                    } else {
                        cursor = (x, y);
                    }
                    subpath_start = cursor;
                    segs.push(Segment::MoveTo {
                        x: cursor.0,
                        y: cursor.1,
                    });
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                    // subsequent pairs are LineTo
                    cmd = Some(if rel { 'l' } else { 'L' });
                }
            }
            'L' | 'l' => {
                let rel = c == 'l';
                if let Some((x, y)) = read_pair(&tokens, &mut i) {
                    if rel {
                        cursor.0 += x;
                        cursor.1 += y;
                    } else {
                        cursor = (x, y);
                    }
                    segs.push(Segment::LineTo {
                        x: cursor.0,
                        y: cursor.1,
                    });
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
            }
            'H' | 'h' => {
                let rel = c == 'h';
                if let Some(x) = read_one(&tokens, &mut i) {
                    if rel {
                        cursor.0 += x;
                    } else {
                        cursor.0 = x;
                    }
                    segs.push(Segment::LineTo {
                        x: cursor.0,
                        y: cursor.1,
                    });
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
            }
            'V' | 'v' => {
                let rel = c == 'v';
                if let Some(y) = read_one(&tokens, &mut i) {
                    if rel {
                        cursor.1 += y;
                    } else {
                        cursor.1 = y;
                    }
                    segs.push(Segment::LineTo {
                        x: cursor.0,
                        y: cursor.1,
                    });
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                }
            }
            'C' | 'c' => {
                let rel = c == 'c';
                if let Some(vals) = read_n(&tokens, &mut i, 6) {
                    let (x1, y1, x2, y2, x, y) = if rel {
                        (
                            cursor.0 + vals[0],
                            cursor.1 + vals[1],
                            cursor.0 + vals[2],
                            cursor.1 + vals[3],
                            cursor.0 + vals[4],
                            cursor.1 + vals[5],
                        )
                    } else {
                        (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5])
                    };
                    segs.push(Segment::CubicTo {
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    });
                    last_cubic_ctrl = Some((x2, y2));
                    last_quad_ctrl = None;
                    cursor = (x, y);
                }
            }
            'S' | 's' => {
                let rel = c == 's';
                if let Some(vals) = read_n(&tokens, &mut i, 4) {
                    let (x1, y1) = if let Some((lx, ly)) = last_cubic_ctrl {
                        (2.0 * cursor.0 - lx, 2.0 * cursor.1 - ly)
                    } else {
                        cursor
                    };
                    let (x2, y2, x, y) = if rel {
                        (
                            cursor.0 + vals[0],
                            cursor.1 + vals[1],
                            cursor.0 + vals[2],
                            cursor.1 + vals[3],
                        )
                    } else {
                        (vals[0], vals[1], vals[2], vals[3])
                    };
                    segs.push(Segment::CubicTo {
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    });
                    last_cubic_ctrl = Some((x2, y2));
                    last_quad_ctrl = None;
                    cursor = (x, y);
                }
            }
            'Q' | 'q' => {
                let rel = c == 'q';
                if let Some(vals) = read_n(&tokens, &mut i, 4) {
                    let (x1, y1, x, y) = if rel {
                        (
                            cursor.0 + vals[0],
                            cursor.1 + vals[1],
                            cursor.0 + vals[2],
                            cursor.1 + vals[3],
                        )
                    } else {
                        (vals[0], vals[1], vals[2], vals[3])
                    };
                    segs.push(Segment::QuadTo { x1, y1, x, y });
                    last_quad_ctrl = Some((x1, y1));
                    last_cubic_ctrl = None;
                    cursor = (x, y);
                }
            }
            'T' | 't' => {
                let rel = c == 't';
                if let Some(vals) = read_n(&tokens, &mut i, 2) {
                    let (x1, y1) = if let Some((lx, ly)) = last_quad_ctrl {
                        (2.0 * cursor.0 - lx, 2.0 * cursor.1 - ly)
                    } else {
                        cursor
                    };
                    let (x, y) = if rel {
                        (cursor.0 + vals[0], cursor.1 + vals[1])
                    } else {
                        (vals[0], vals[1])
                    };
                    segs.push(Segment::QuadTo { x1, y1, x, y });
                    last_quad_ctrl = Some((x1, y1));
                    last_cubic_ctrl = None;
                    cursor = (x, y);
                }
            }
            'A' | 'a' => {
                let rel = c == 'a';
                if let Some(vals) = read_n(&tokens, &mut i, 7) {
                    let rx = vals[0].abs();
                    let ry = vals[1].abs();
                    let x_rot = vals[2];
                    let large = vals[3] != 0.0;
                    let sweep = vals[4] != 0.0;
                    let (x, y) = if rel {
                        (cursor.0 + vals[5], cursor.1 + vals[6])
                    } else {
                        (vals[5], vals[6])
                    };
                    let cubics = arc_to_cubics(cursor.0, cursor.1, rx, ry, x_rot, large, sweep, x, y);
                    for (x1, y1, x2, y2, ex, ey) in cubics {
                        segs.push(Segment::CubicTo {
                            x1,
                            y1,
                            x2,
                            y2,
                            x: ex,
                            y: ey,
                        });
                    }
                    last_cubic_ctrl = None;
                    last_quad_ctrl = None;
                    cursor = (x, y);
                }
            }
            'Z' | 'z' => {
                segs.push(Segment::Close);
                cursor = subpath_start;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
                cmd = None;
            }
            _ => {
                // skip unknown
                i += 1;
            }
        }
    }
    segs
}

/// Apply affine transform to all points of segments.
pub fn transform_segments(segs: &[Segment], m: Matrix) -> Vec<Segment> {
    segs.iter()
        .map(|s| match *s {
            Segment::MoveTo { x, y } => {
                let (x, y) = m.map_point(x, y);
                Segment::MoveTo { x, y }
            }
            Segment::LineTo { x, y } => {
                let (x, y) = m.map_point(x, y);
                Segment::LineTo { x, y }
            }
            Segment::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                let (x1, y1) = m.map_point(x1, y1);
                let (x2, y2) = m.map_point(x2, y2);
                let (x, y) = m.map_point(x, y);
                Segment::CubicTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                }
            }
            Segment::QuadTo { x1, y1, x, y } => {
                let (x1, y1) = m.map_point(x1, y1);
                let (x, y) = m.map_point(x, y);
                Segment::QuadTo { x1, y1, x, y }
            }
            Segment::Close => Segment::Close,
        })
        .collect()
}

/// Bounding box of path segments.
pub fn bounds(segs: &[Segment]) -> Option<(f64, f64, f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut n = 0;
    let mut add = |x: f64, y: f64| {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        n += 1;
    };
    for s in segs {
        match *s {
            Segment::MoveTo { x, y } | Segment::LineTo { x, y } => add(x, y),
            Segment::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                add(x1, y1);
                add(x2, y2);
                add(x, y);
            }
            Segment::QuadTo { x1, y1, x, y } => {
                add(x1, y1);
                add(x, y);
            }
            Segment::Close => {}
        }
    }
    if n == 0 {
        None
    } else {
        Some((min_x, min_y, max_x, max_y))
    }
}

/// Convert ellipse / circle to path segments (4 cubics).
pub fn ellipse_path(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<Segment> {
    // κ ≈ 0.5522847498 for circle quarter
    let k = 0.552_284_749_8;
    let ox = rx * k;
    let oy = ry * k;
    vec![
        Segment::MoveTo {
            x: cx + rx,
            y: cy,
        },
        Segment::CubicTo {
            x1: cx + rx,
            y1: cy + oy,
            x2: cx + ox,
            y2: cy + ry,
            x: cx,
            y: cy + ry,
        },
        Segment::CubicTo {
            x1: cx - ox,
            y1: cy + ry,
            x2: cx - rx,
            y2: cy + oy,
            x: cx - rx,
            y: cy,
        },
        Segment::CubicTo {
            x1: cx - rx,
            y1: cy - oy,
            x2: cx - ox,
            y2: cy - ry,
            x: cx,
            y: cy - ry,
        },
        Segment::CubicTo {
            x1: cx + ox,
            y1: cy - ry,
            x2: cx + rx,
            y2: cy - oy,
            x: cx + rx,
            y: cy,
        },
        Segment::Close,
    ]
}

/// Rectangle (optionally rounded) as path segments.
pub fn rect_path(x: f64, y: f64, w: f64, h: f64, mut rx: f64, mut ry: f64) -> Vec<Segment> {
    if w <= 0.0 || h <= 0.0 {
        return Vec::new();
    }
    rx = rx.abs().min(w / 2.0);
    ry = ry.abs().min(h / 2.0);
    if rx <= 1e-9 && ry <= 1e-9 {
        return vec![
            Segment::MoveTo { x, y },
            Segment::LineTo { x: x + w, y },
            Segment::LineTo { x: x + w, y: y + h },
            Segment::LineTo { x, y: y + h },
            Segment::Close,
        ];
    }
    let k = 0.552_284_749_8;
    let ox = rx * k;
    let oy = ry * k;
    vec![
        Segment::MoveTo {
            x: x + rx,
            y,
        },
        Segment::LineTo {
            x: x + w - rx,
            y,
        },
        Segment::CubicTo {
            x1: x + w - rx + ox,
            y1: y,
            x2: x + w,
            y2: y + ry - oy,
            x: x + w,
            y: y + ry,
        },
        Segment::LineTo {
            x: x + w,
            y: y + h - ry,
        },
        Segment::CubicTo {
            x1: x + w,
            y1: y + h - ry + oy,
            x2: x + w - rx + ox,
            y2: y + h,
            x: x + w - rx,
            y: y + h,
        },
        Segment::LineTo {
            x: x + rx,
            y: y + h,
        },
        Segment::CubicTo {
            x1: x + rx - ox,
            y1: y + h,
            x2: x,
            y2: y + h - ry + oy,
            x,
            y: y + h - ry,
        },
        Segment::LineTo {
            x,
            y: y + ry,
        },
        Segment::CubicTo {
            x1: x,
            y1: y + ry - oy,
            x2: x + rx - ox,
            y2: y,
            x: x + rx,
            y,
        },
        Segment::Close,
    ]
}

pub fn line_path(x1: f64, y1: f64, x2: f64, y2: f64) -> Vec<Segment> {
    vec![
        Segment::MoveTo { x: x1, y: y1 },
        Segment::LineTo { x: x2, y: y2 },
    ]
}

pub fn polygon_path(pts: &[(f64, f64)], close: bool) -> Vec<Segment> {
    if pts.is_empty() {
        return Vec::new();
    }
    let mut segs = vec![Segment::MoveTo {
        x: pts[0].0,
        y: pts[0].1,
    }];
    for p in &pts[1..] {
        segs.push(Segment::LineTo { x: p.0, y: p.1 });
    }
    if close {
        segs.push(Segment::Close);
    }
    segs
}

// ── Arc → cubics (SVG endpoint parameterization) ────────────────────────────

fn arc_to_cubics(
    x1: f64,
    y1: f64,
    mut rx: f64,
    mut ry: f64,
    phi_deg: f64,
    large: bool,
    sweep: bool,
    x2: f64,
    y2: f64,
) -> Vec<(f64, f64, f64, f64, f64, f64)> {
    if rx < 1e-12 || ry < 1e-12 {
        return vec![(x1, y1, x2, y2, x2, y2)]; // degenerate → line-ish
    }
    let phi = phi_deg.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();

    // Step 1: compute (x1', y1')
    let dx = (x1 - x2) / 2.0;
    let dy = (y1 - y2) / 2.0;
    let x1p = cos_phi * dx + sin_phi * dy;
    let y1p = -sin_phi * dx + cos_phi * dy;

    // Correct out-of-range radii
    let mut lam = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if lam > 1.0 {
        lam = lam.sqrt();
        rx *= lam;
        ry *= lam;
    }

    // Step 2: compute center (cx', cy')
    let rx2 = rx * rx;
    let ry2 = ry * ry;
    let x1p2 = x1p * x1p;
    let y1p2 = y1p * y1p;
    let mut sq = ((rx2 * ry2 - rx2 * y1p2 - ry2 * x1p2) / (rx2 * y1p2 + ry2 * x1p2)).max(0.0);
    sq = sq.sqrt();
    if large == sweep {
        sq = -sq;
    }
    let cxp = sq * (rx * y1p) / ry;
    let cyp = sq * -(ry * x1p) / rx;

    // Step 3: center in original coords
    let cx = cos_phi * cxp - sin_phi * cyp + (x1 + x2) / 2.0;
    let cy = sin_phi * cxp + cos_phi * cyp + (y1 + y2) / 2.0;

    // Angles
    let ux = (x1p - cxp) / rx;
    let uy = (y1p - cyp) / ry;
    let vx = (-x1p - cxp) / rx;
    let vy = (-y1p - cyp) / ry;
    let theta1 = vector_angle(1.0, 0.0, ux, uy);
    let mut dtheta = vector_angle(ux, uy, vx, vy);
    if !sweep && dtheta > 0.0 {
        dtheta -= 2.0 * std::f64::consts::PI;
    } else if sweep && dtheta < 0.0 {
        dtheta += 2.0 * std::f64::consts::PI;
    }

    // Split into ≤90° segments
    let segments = ((dtheta.abs() / (std::f64::consts::PI / 2.0)).ceil() as usize).max(1);
    let delta = dtheta / segments as f64;
    let t = (4.0 / 3.0) * (delta / 4.0).tan();

    let mut out = Vec::with_capacity(segments);
    for i in 0..segments {
        let th1 = theta1 + i as f64 * delta;
        let th2 = th1 + delta;
        let (s1, c1) = th1.sin_cos();
        let (s2, c2) = th2.sin_cos();
        // ellipse point
        let e = |cos_t: f64, sin_t: f64| -> (f64, f64) {
            let x = cos_phi * rx * cos_t - sin_phi * ry * sin_t + cx;
            let y = sin_phi * rx * cos_t + cos_phi * ry * sin_t + cy;
            (x, y)
        };
        // derivative / scaled
        let ep = |cos_t: f64, sin_t: f64| -> (f64, f64) {
            let dx = -cos_phi * rx * sin_t - sin_phi * ry * cos_t;
            let dy = -sin_phi * rx * sin_t + cos_phi * ry * cos_t;
            (dx, dy)
        };
        let (p1x, p1y) = e(c1, s1);
        let (p2x, p2y) = e(c2, s2);
        let (d1x, d1y) = ep(c1, s1);
        let (d2x, d2y) = ep(c2, s2);
        let q1x = p1x + t * d1x;
        let q1y = p1y + t * d1y;
        let q2x = p2x - t * d2x;
        let q2y = p2y - t * d2y;
        // first point of first segment should connect from (x1,y1) — use computed
        let _ = (p1x, p1y);
        out.push((q1x, q1y, q2x, q2y, p2x, p2y));
    }
    out
}

fn vector_angle(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    let sign = if ux * vy - uy * vx < 0.0 { -1.0 } else { 1.0 };
    let dot = ux * vx + uy * vy;
    let u_len = (ux * ux + uy * uy).sqrt();
    let v_len = (vx * vx + vy * vy).sqrt();
    let cos = (dot / (u_len * v_len)).clamp(-1.0, 1.0);
    sign * cos.acos()
}

// ── Tokenizer ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Token {
    Cmd(char),
    Num(f64),
}

fn tokenize(d: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let chars: Vec<char> = d.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() || c == ',' {
            i += 1;
            continue;
        }
        if c.is_ascii_alphabetic() {
            out.push(Token::Cmd(c));
            i += 1;
            continue;
        }
        // number — reuse parse_numbers on a slice is awkward; inline
        let start = i;
        if chars[i] == '+' || chars[i] == '-' {
            i += 1;
        }
        let mut seen_dot = false;
        let mut seen_exp = false;
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_ascii_digit() {
                i += 1;
            } else if ch == '.' && !seen_dot && !seen_exp {
                seen_dot = true;
                i += 1;
            } else if (ch == 'e' || ch == 'E') && !seen_exp {
                seen_exp = true;
                i += 1;
                if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                    i += 1;
                }
            } else {
                break;
            }
        }
        if i > start {
            let s: String = chars[start..i].iter().collect();
            if let Ok(n) = s.parse::<f64>() {
                out.push(Token::Num(n));
            }
        } else {
            i += 1;
        }
    }
    out
}

fn read_one(tokens: &[Token], i: &mut usize) -> Option<f64> {
    if *i < tokens.len() {
        if let Token::Num(n) = tokens[*i] {
            *i += 1;
            return Some(n);
        }
    }
    None
}

fn read_pair(tokens: &[Token], i: &mut usize) -> Option<(f64, f64)> {
    let x = read_one(tokens, i)?;
    let y = read_one(tokens, i)?;
    Some((x, y))
}

fn read_n(tokens: &[Token], i: &mut usize, n: usize) -> Option<Vec<f64>> {
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(read_one(tokens, i)?);
    }
    Some(v)
}

/// Parse `points` attribute for polygon/polyline.
pub fn parse_points(s: &str) -> Vec<(f64, f64)> {
    let nums = parse_numbers(s);
    nums.chunks(2)
        .filter(|c| c.len() == 2)
        .map(|c| (c[0], c[1]))
        .collect()
}

/// Flatten path segments into a polyline of sample points (for clip tests / markers).
pub fn sample_points(segs: &[Segment], samples_per_curve: usize) -> Vec<(f64, f64)> {
    let mut pts = Vec::new();
    let mut cur = (0.0_f64, 0.0_f64);
    let n = samples_per_curve.max(1);
    for s in segs {
        match *s {
            Segment::MoveTo { x, y } => {
                cur = (x, y);
                pts.push(cur);
            }
            Segment::LineTo { x, y } => {
                cur = (x, y);
                pts.push(cur);
            }
            Segment::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                let p0 = cur;
                for i in 1..=n {
                    let t = i as f64 / n as f64;
                    pts.push(cubic_point(p0, (x1, y1), (x2, y2), (x, y), t));
                }
                cur = (x, y);
            }
            Segment::QuadTo { x1, y1, x, y } => {
                let p0 = cur;
                for i in 1..=n {
                    let t = i as f64 / n as f64;
                    pts.push(quad_point(p0, (x1, y1), (x, y), t));
                }
                cur = (x, y);
            }
            Segment::Close => {}
        }
    }
    pts
}

fn cubic_point(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    t: f64,
) -> (f64, f64) {
    let u = 1.0 - t;
    let uu = u * u;
    let tt = t * t;
    let x = uu * u * p0.0 + 3.0 * uu * t * p1.0 + 3.0 * u * tt * p2.0 + tt * t * p3.0;
    let y = uu * u * p0.1 + 3.0 * uu * t * p1.1 + 3.0 * u * tt * p2.1 + tt * t * p3.1;
    (x, y)
}

fn quad_point(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    (
        u * u * p0.0 + 2.0 * u * t * p1.0 + t * t * p2.0,
        u * u * p0.1 + 2.0 * u * t * p1.1 + t * t * p2.1,
    )
}

/// Open subpath endpoints and mid-vertex list for marker placement.
/// Returns (start, mids, end, start_tangent_angle, end_tangent_angle) in user space.
pub fn marker_anchors(segs: &[Segment]) -> Option<MarkerAnchors> {
    let pts = sample_points(segs, 8);
    if pts.len() < 2 {
        return None;
    }
    let start = pts[0];
    let end = *pts.last().unwrap();
    // mid vertices: original LineTo/MoveTo corners only
    let mut corners = Vec::new();
    for s in segs {
        match *s {
            Segment::MoveTo { x, y } | Segment::LineTo { x, y } => corners.push((x, y)),
            Segment::CubicTo { x, y, .. } | Segment::QuadTo { x, y, .. } => corners.push((x, y)),
            Segment::Close => {}
        }
    }
    let mids = if corners.len() > 2 {
        corners[1..corners.len() - 1].to_vec()
    } else {
        Vec::new()
    };
    let start_ang = {
        let (dx, dy) = (pts[1].0 - pts[0].0, pts[1].1 - pts[0].1);
        dy.atan2(dx)
    };
    let end_ang = {
        let n = pts.len();
        let (dx, dy) = (pts[n - 1].0 - pts[n - 2].0, pts[n - 1].1 - pts[n - 2].1);
        dy.atan2(dx)
    };
    Some(MarkerAnchors {
        start,
        mids,
        end,
        start_angle: start_ang,
        end_angle: end_ang,
    })
}

#[derive(Debug, Clone)]
pub struct MarkerAnchors {
    pub start: (f64, f64),
    pub mids: Vec<(f64, f64)>,
    pub end: (f64, f64),
    pub start_angle: f64,
    pub end_angle: f64,
}

/// Axis-aligned rect clip of path segments (Cohen–Sutherland style on samples + rebuild).
/// Returns segments restricted to `rect = (min_x, min_y, max_x, max_y)`.
pub fn clip_segments_to_rect(segs: &[Segment], rect: (f64, f64, f64, f64)) -> Vec<Segment> {
    if segs.is_empty() {
        return Vec::new();
    }
    clip_segments_by_predicate(segs, |x, y| point_in_rect(x, y, rect))
}

/// Clip path segments to an arbitrary polygon (even-odd fill rule).
pub fn clip_segments_to_polygon(segs: &[Segment], poly: &[(f64, f64)]) -> Vec<Segment> {
    if segs.is_empty() || poly.len() < 3 {
        return Vec::new();
    }
    clip_segments_by_predicate(segs, |x, y| point_in_polygon(x, y, poly))
}

fn clip_segments_by_predicate<F>(segs: &[Segment], inside: F) -> Vec<Segment>
where
    F: Fn(f64, f64) -> bool,
{
    // Sample densely, keep points inside, rebuild as polyline (lossy but robust).
    let pts = sample_points(segs, 16);
    let mut out = Vec::new();
    let mut pen_down = false;
    for (x, y) in pts {
        if inside(x, y) {
            if !pen_down {
                out.push(Segment::MoveTo { x, y });
                pen_down = true;
            } else {
                out.push(Segment::LineTo { x, y });
            }
        } else {
            pen_down = false;
        }
    }
    if !out.is_empty() && matches!(segs.last(), Some(Segment::Close)) {
        out.push(Segment::Close);
    }
    out
}

/// Whether a point is inside an axis-aligned rect.
pub fn point_in_rect(x: f64, y: f64, rect: (f64, f64, f64, f64)) -> bool {
    x >= rect.0 && x <= rect.2 && y >= rect.1 && y <= rect.3
}

/// Even-odd point-in-polygon test.
pub fn point_in_polygon(x: f64, y: f64, poly: &[(f64, f64)]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        let intersect = ((yi > y) != (yj > y))
            && (x < (xj - xi) * (y - yi) / (yj - yi + 1e-30) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Approximate polygon from path (corner points).
pub fn path_polygon(segs: &[Segment]) -> Vec<(f64, f64)> {
    let mut poly = Vec::new();
    for s in segs {
        match *s {
            Segment::MoveTo { x, y } | Segment::LineTo { x, y } => poly.push((x, y)),
            Segment::CubicTo { x, y, .. } | Segment::QuadTo { x, y, .. } => poly.push((x, y)),
            Segment::Close => {}
        }
    }
    poly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_move_line_close() {
        let s = parse_path("M10 10 L20 20 Z");
        assert!(matches!(s[0], Segment::MoveTo { .. }));
        assert!(matches!(s[1], Segment::LineTo { .. }));
        assert!(matches!(s[2], Segment::Close));
    }

    #[test]
    fn parse_arc() {
        let s = parse_path("M10,30 A20,20 0,0,1 50,30");
        assert!(s.len() >= 2);
        assert!(matches!(s[0], Segment::MoveTo { .. }));
        assert!(matches!(s[1], Segment::CubicTo { .. }));
    }

    #[test]
    fn smooth_cubic_reflection() {
        // C then S — S should reflect
        let s = parse_path("M0 0 C10 0 20 10 30 10 S50 0 60 0");
        assert_eq!(s.len(), 3);
    }
}
