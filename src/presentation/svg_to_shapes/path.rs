//! SVG path `d` attribute parser and geometry helpers (MDN / SVG2).
//!
//! Supports M/L/H/V/C/S/Q/T/A/Z (absolute + relative) with implicit command
//! repetition. Arcs are flattened to cubic Béziers for DrawingML `custGeom`.

use super::matrix::{parse_numbers, Matrix};

/// Absolute path segment after normalization.
#[derive(Debug, Clone)]
pub enum Segment {
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    CubicTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    QuadTo {
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
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
                    let cubics =
                        arc_to_cubics(cursor.0, cursor.1, rx, ry, x_rot, large, sweep, x, y);
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

/// Scale all path points about the axis-aligned bbox center by `factor`.
/// Used to approximate CSS `stroke-alignment: inner|outer` on freeforms
/// (true parallel offset is expensive; uniform inset/outset of geometry is
/// the same approximation used for rect/circle/ellipse).
pub fn scale_segments_about_center(segs: &[Segment], factor: f64) -> Vec<Segment> {
    if (factor - 1.0).abs() < 1e-12 {
        return segs.to_vec();
    }
    let Some((min_x, min_y, max_x, max_y)) = bounds(segs) else {
        return segs.to_vec();
    };
    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;
    // then = other * self → apply translate(-c), then scale, then translate(+c).
    let m = Matrix::translate(-cx, -cy)
        .then(Matrix::scale(factor, factor))
        .then(Matrix::translate(cx, cy));
    transform_segments(segs, m)
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
        Segment::MoveTo { x: cx + rx, y: cy },
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
        Segment::MoveTo { x: x + rx, y },
        Segment::LineTo { x: x + w - rx, y },
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
        Segment::LineTo { x, y: y + ry },
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

/// Expand a stroked path into discrete dash segments using SVG `stroke-dasharray`
/// pattern (alternating on/off lengths in user units). Returns a list of open
/// polylines (as MoveTo/LineTo chains). Empty if the path has no length.
pub fn dash_segments(segs: &[Segment], dash: &[f64], dash_offset: f64) -> Vec<Vec<Segment>> {
    if segs.is_empty() || dash.is_empty() {
        return vec![segs.to_vec()];
    }
    let pattern: Vec<f64> = dash.iter().map(|v| (*v).abs().max(0.0)).collect();
    if pattern.iter().all(|&v| v <= 0.0) {
        return vec![segs.to_vec()];
    }
    // Dense polyline approximation of the path.
    let dens = densify(segs, 12);
    if dens.len() < 2 {
        return Vec::new();
    }
    // Cumulative lengths
    let mut cum = vec![0.0_f64; dens.len()];
    for i in 1..dens.len() {
        let dx = dens[i].0 - dens[i - 1].0;
        let dy = dens[i].1 - dens[i - 1].1;
        cum[i] = cum[i - 1] + (dx * dx + dy * dy).sqrt();
    }
    let total = *cum.last().unwrap();
    if total <= 1e-9 {
        return Vec::new();
    }
    let pat_sum: f64 = pattern.iter().sum();
    if pat_sum <= 1e-9 {
        return vec![segs.to_vec()];
    }

    // Walk along the path, emitting on-segments.
    let mut out: Vec<Vec<Segment>> = Vec::new();
    let mut pos = dash_offset.rem_euclid(pat_sum);
    // Find which pattern slot `pos` falls into and whether it's an "on" slot.
    let (mut slot, mut slot_pos) = {
        let mut acc = 0.0;
        let mut s = 0usize;
        loop {
            let plen = pattern[s % pattern.len()];
            if pos < acc + plen || plen <= 0.0 {
                break (s, pos - acc);
            }
            acc += plen;
            s += 1;
            if s > pattern.len() * 4 {
                break (0, 0.0);
            }
        }
    };
    let mut path_pos = 0.0_f64;
    while path_pos < total - 1e-9 {
        let plen = pattern[slot % pattern.len()].max(1e-6);
        let remain_in_slot = (plen - slot_pos).max(0.0);
        let remain_on_path = total - path_pos;
        let step = remain_in_slot.min(remain_on_path);
        let is_on = (slot % 2) == 0; // even indices are dashes (drawn)
        if is_on && step > 1e-6 {
            let a = path_pos;
            let b = path_pos + step;
            if let Some(poly) = extract_subpoly(&dens, &cum, a, b) {
                if poly.len() >= 2 {
                    let mut chain = Vec::with_capacity(poly.len());
                    chain.push(Segment::MoveTo {
                        x: poly[0].0,
                        y: poly[0].1,
                    });
                    for p in poly.iter().skip(1) {
                        chain.push(Segment::LineTo { x: p.0, y: p.1 });
                    }
                    out.push(chain);
                }
            }
        }
        path_pos += step;
        slot_pos += step;
        if slot_pos >= plen - 1e-12 {
            slot += 1;
            slot_pos = 0.0;
        }
    }
    out
}

fn densify(segs: &[Segment], samples_per_curve: usize) -> Vec<(f64, f64)> {
    // Like sample_points but also densifies long straight segments.
    let mut pts = Vec::new();
    let mut cur = (0.0_f64, 0.0_f64);
    let n = samples_per_curve.max(1);
    let max_seg = 4.0_f64; // user units
    let push_line = |pts: &mut Vec<(f64, f64)>, from: (f64, f64), to: (f64, f64)| {
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        let len = (dx * dx + dy * dy).sqrt();
        let steps = ((len / max_seg).ceil() as usize).max(1);
        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            pts.push((from.0 + dx * t, from.1 + dy * t));
        }
    };
    for s in segs {
        match *s {
            Segment::MoveTo { x, y } => {
                cur = (x, y);
                if pts.last().copied() != Some(cur) {
                    pts.push(cur);
                }
            }
            Segment::LineTo { x, y } => {
                let to = (x, y);
                push_line(&mut pts, cur, to);
                cur = to;
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
                // adaptive-ish fixed samples
                let approx_len = {
                    let a = ((x1 - p0.0).hypot(y1 - p0.1))
                        + ((x2 - x1).hypot(y2 - y1))
                        + ((x - x2).hypot(y - y2));
                    a
                };
                let steps = ((approx_len / max_seg).ceil() as usize).max(n);
                for i in 1..=steps {
                    let t = i as f64 / steps as f64;
                    pts.push(cubic_point(p0, (x1, y1), (x2, y2), (x, y), t));
                }
                cur = (x, y);
            }
            Segment::QuadTo { x1, y1, x, y } => {
                let p0 = cur;
                let approx_len = ((x1 - p0.0).hypot(y1 - p0.1)) + ((x - x1).hypot(y - y1));
                let steps = ((approx_len / max_seg).ceil() as usize).max(n);
                for i in 1..=steps {
                    let t = i as f64 / steps as f64;
                    pts.push(quad_point(p0, (x1, y1), (x, y), t));
                }
                cur = (x, y);
            }
            Segment::Close => {
                if let Some(&first) = pts.first() {
                    if (first.0 - cur.0).abs() > 1e-9 || (first.1 - cur.1).abs() > 1e-9 {
                        push_line(&mut pts, cur, first);
                        cur = first;
                    }
                }
            }
        }
    }
    pts
}

fn extract_subpoly(dens: &[(f64, f64)], cum: &[f64], a: f64, b: f64) -> Option<Vec<(f64, f64)>> {
    if b <= a + 1e-9 || dens.len() < 2 {
        return None;
    }
    let mut out = Vec::new();
    // start point
    out.push(point_at(dens, cum, a)?);
    for i in 1..dens.len() {
        if cum[i] <= a + 1e-12 {
            continue;
        }
        if cum[i] >= b - 1e-12 {
            break;
        }
        out.push(dens[i]);
    }
    out.push(point_at(dens, cum, b)?);
    // dedup consecutive
    out.dedup_by(|p, q| (p.0 - q.0).abs() < 1e-9 && (p.1 - q.1).abs() < 1e-9);
    Some(out)
}

fn point_at(dens: &[(f64, f64)], cum: &[f64], s: f64) -> Option<(f64, f64)> {
    if dens.is_empty() {
        return None;
    }
    if s <= 0.0 {
        return Some(dens[0]);
    }
    let total = *cum.last()?;
    if s >= total {
        return dens.last().copied();
    }
    // binary search
    let mut lo = 0usize;
    let mut hi = cum.len() - 1;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if cum[mid] <= s {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (s0, s1) = (cum[lo], cum[hi]);
    let t = if (s1 - s0).abs() < 1e-12 {
        0.0
    } else {
        ((s - s0) / (s1 - s0)).clamp(0.0, 1.0)
    };
    let (x0, y0) = dens[lo];
    let (x1, y1) = dens[hi];
    Some((x0 + (x1 - x0) * t, y0 + (y1 - y0) * t))
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
    // mid vertices: original LineTo/MoveTo/curve end corners only
    let mut corners = Vec::new();
    for s in segs {
        match *s {
            Segment::MoveTo { x, y } | Segment::LineTo { x, y } => corners.push((x, y)),
            Segment::CubicTo { x, y, .. } | Segment::QuadTo { x, y, .. } => corners.push((x, y)),
            Segment::Close => {}
        }
    }
    let mut mids = Vec::new();
    let mut mid_angles = Vec::new();
    if corners.len() > 2 {
        for i in 1..corners.len() - 1 {
            let prev = corners[i - 1];
            let cur = corners[i];
            let next = corners[i + 1];
            // Bisect incoming/outgoing tangents for a stable mid orientation.
            let a1 = (cur.1 - prev.1).atan2(cur.0 - prev.0);
            let a2 = (next.1 - cur.1).atan2(next.0 - cur.0);
            let mut d = a2 - a1;
            while d > std::f64::consts::PI {
                d -= std::f64::consts::TAU;
            }
            while d < -std::f64::consts::PI {
                d += std::f64::consts::TAU;
            }
            mids.push(cur);
            mid_angles.push(a1 + d * 0.5);
        }
    }
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
        mid_angles,
        end,
        start_angle: start_ang,
        end_angle: end_ang,
    })
}

#[derive(Debug, Clone)]
pub struct MarkerAnchors {
    pub start: (f64, f64),
    pub mids: Vec<(f64, f64)>,
    /// Tangent (or bisector) angle at each mid vertex, radians.
    pub mid_angles: Vec<f64>,
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

/// Clip path segments to an arbitrary polygon (clip-rule evenodd or nonzero).
pub fn clip_segments_to_polygon(segs: &[Segment], poly: &[(f64, f64)]) -> Vec<Segment> {
    clip_segments_to_polygon_rule(segs, poly, true)
}

/// Clip path segments using SVG clip-rule (`evenodd` vs `nonzero`).
pub fn clip_segments_to_polygon_rule(
    segs: &[Segment],
    poly: &[(f64, f64)],
    even_odd: bool,
) -> Vec<Segment> {
    clip_segments_to_polygons_rule(segs, &[poly.to_vec()], even_odd)
}

/// Clip against a union of polygons (SVG clipPath with multiple children).
pub fn clip_segments_to_polygons_rule(
    segs: &[Segment],
    polys: &[Vec<(f64, f64)>],
    even_odd: bool,
) -> Vec<Segment> {
    if segs.is_empty() || polys.is_empty() {
        return Vec::new();
    }
    let usable: Vec<&[(f64, f64)]> = polys
        .iter()
        .map(|p| p.as_slice())
        .filter(|p| p.len() >= 3)
        .collect();
    if usable.is_empty() {
        return Vec::new();
    }
    clip_segments_by_predicate(segs, |x, y| {
        usable
            .iter()
            .any(|poly| point_in_polygon_rule(x, y, poly, even_odd))
    })
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
    point_in_polygon_rule(x, y, poly, true)
}

/// Even-odd (SVG default for clip-rule historically in many UAs) vs nonzero winding.
pub fn point_in_polygon_rule(x: f64, y: f64, poly: &[(f64, f64)], even_odd: bool) -> bool {
    if poly.len() < 3 {
        return false;
    }
    if even_odd {
        let mut inside = false;
        let mut j = poly.len() - 1;
        for i in 0..poly.len() {
            let (xi, yi) = poly[i];
            let (xj, yj) = poly[j];
            let intersect =
                ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi + 1e-30) + xi);
            if intersect {
                inside = !inside;
            }
            j = i;
        }
        inside
    } else {
        // Nonzero winding number (Hormann/Agathos-style).
        let mut wn = 0_i32;
        let mut j = poly.len() - 1;
        for i in 0..poly.len() {
            let (xi, yi) = poly[i];
            let (xj, yj) = poly[j];
            // isLeft(j→i, p) = (xi-xj)*(y-yj) - (x-xj)*(yi-yj)
            let is_left = (xi - xj) * (y - yj) - (x - xj) * (yi - yj);
            if yj <= y {
                if yi > y && is_left > 0.0 {
                    wn += 1; // upward crossing
                }
            } else if yi <= y && is_left < 0.0 {
                wn -= 1; // downward crossing
            }
            j = i;
        }
        wn != 0
    }
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

    #[test]
    fn point_in_polygon_evenodd_vs_nonzero() {
        // Axis-aligned square — both rules agree.
        let square = [(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        assert!(point_in_polygon_rule(5.0, 5.0, &square, true));
        assert!(point_in_polygon_rule(5.0, 5.0, &square, false));
        assert!(!point_in_polygon_rule(15.0, 5.0, &square, true));
        assert!(!point_in_polygon_rule(15.0, 5.0, &square, false));

        // Self-overlapping pentagram-like star: evenodd empties the center, nonzero fills it.
        let star = [
            (0.0, 0.0),
            (5.0, 8.0),
            (10.0, 0.0),
            (2.0, 6.0),
            (8.0, 6.0),
        ];
        // Center of the star should be evenodd=false, nonzero=true.
        assert!(
            !point_in_polygon_rule(5.0, 5.0, &star, true),
            "evenodd should leave star center empty"
        );
        assert!(
            point_in_polygon_rule(5.0, 5.0, &star, false),
            "nonzero should fill star center"
        );
        // A tip of the star filled under both.
        assert!(point_in_polygon_rule(5.0, 7.0, &star, true));
        assert!(point_in_polygon_rule(5.0, 7.0, &star, false));
    }
}
