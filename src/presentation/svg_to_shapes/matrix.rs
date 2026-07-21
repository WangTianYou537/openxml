//! 2D affine transforms matching SVG / CSS matrix conventions.
//!
//! SVG `transform` list is applied **right-to-left** (MDN): the rightmost
//! function is applied first. Matrices use column vectors:
//!
//! ```text
//! | a  c  e |   | x |
//! | b  d  f | × | y |
//! | 0  0  1 |   | 1 |
//! ```

use std::f64::consts::PI;

/// Affine 2D transform (a,b,c,d,e,f).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Matrix {
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn translate(tx: f64, ty: f64) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: tx,
            f: ty,
        }
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn rotate_deg(angle: f64) -> Self {
        let r = angle * PI / 180.0;
        let (s, c) = r.sin_cos();
        Self {
            a: c,
            b: s,
            c: -s,
            d: c,
            e: 0.0,
            f: 0.0,
        }
    }

    /// `rotate(a cx cy)` ≡ `T(cx,cy) · R(a) · T(-cx,-cy)`.
    pub fn rotate_around(angle: f64, cx: f64, cy: f64) -> Self {
        Self::translate(cx, cy)
            .then(Self::rotate_deg(angle))
            .then(Self::translate(-cx, -cy))
    }

    pub fn skew_x_deg(angle: f64) -> Self {
        let t = (angle * PI / 180.0).tan();
        Self {
            a: 1.0,
            b: 0.0,
            c: t,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn skew_y_deg(angle: f64) -> Self {
        let t = (angle * PI / 180.0).tan();
        Self {
            a: 1.0,
            b: t,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn from_svg_matrix(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Self { a, b, c, d, e, f }
    }

    /// Apply `other` after `self` (self then other): `p' = other · self · p`.
    pub fn then(self, other: Self) -> Self {
        // other * self
        Self {
            a: other.a * self.a + other.c * self.b,
            b: other.b * self.a + other.d * self.b,
            c: other.a * self.c + other.c * self.d,
            d: other.b * self.c + other.d * self.d,
            e: other.a * self.e + other.c * self.f + other.e,
            f: other.b * self.e + other.d * self.f + other.f,
        }
    }

    /// Apply this transform to a point.
    pub fn map_point(self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.c * y + self.e,
            self.b * x + self.d * y + self.f,
        )
    }

    /// Transform a free vector (ignore translation) — useful for radii.
    pub fn map_vector(self, x: f64, y: f64) -> (f64, f64) {
        (self.a * x + self.c * y, self.b * x + self.d * y)
    }

    /// Approximate uniform scale factor (average singular value).
    pub fn avg_scale(self) -> f64 {
        let sx = (self.a * self.a + self.b * self.b).sqrt();
        let sy = (self.c * self.c + self.d * self.d).sqrt();
        (sx + sy) / 2.0
    }

    pub fn determinant(self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// Parse an SVG `transform` attribute value into a single matrix.
    ///
    /// Functions are composed right-to-left per MDN.
    pub fn parse_transform_list(s: &str) -> Self {
        let mut funcs: Vec<Matrix> = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
                i += 1;
            }
            if i >= bytes.len() {
                break;
            }
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
                i += 1;
            }
            let name = s[start..i].to_ascii_lowercase();
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] != b'(' {
                break;
            }
            i += 1; // (
            let args_start = i;
            while i < bytes.len() && bytes[i] != b')' {
                i += 1;
            }
            let args_str = &s[args_start..i.min(s.len())];
            if i < bytes.len() {
                i += 1; // )
            }
            let nums = parse_numbers(args_str);
            let m = match name.as_str() {
                "matrix" if nums.len() >= 6 => {
                    Self::from_svg_matrix(nums[0], nums[1], nums[2], nums[3], nums[4], nums[5])
                }
                "translate" if !nums.is_empty() => {
                    Self::translate(nums[0], nums.get(1).copied().unwrap_or(0.0))
                }
                "scale" if !nums.is_empty() => {
                    let sx = nums[0];
                    let sy = nums.get(1).copied().unwrap_or(sx);
                    Self::scale(sx, sy)
                }
                "rotate" if !nums.is_empty() => {
                    if nums.len() >= 3 {
                        Self::rotate_around(nums[0], nums[1], nums[2])
                    } else {
                        Self::rotate_deg(nums[0])
                    }
                }
                "skewx" if !nums.is_empty() => Self::skew_x_deg(nums[0]),
                "skewy" if !nums.is_empty() => Self::skew_y_deg(nums[0]),
                _ => Self::identity(),
            };
            funcs.push(m);
        }
        // MDN: rightmost function applied first.
        // transform="A B C" → p' = A · B · C · p  (C first).
        // self.then(other) = other · self (self first). So f.then(acc) = acc · f.
        // fold left: I, then A → A; then B → A·B; then C → A·B·C. Correct.
        funcs
            .into_iter()
            .fold(Self::identity(), |acc, f| f.then(acc))
    }
}

/// Parse a list of SVG numbers (whitespace / comma separated, optional signs, exponents).
pub fn parse_numbers(s: &str) -> Vec<f64> {
    let mut out = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        while i < chars.len() && (chars[i].is_whitespace() || chars[i] == ',') {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }
        // flags 0/1 for arcs may appear as bare digits
        if !matches!(chars[i], '+' | '-' | '.' | '0'..='9') {
            i += 1;
            continue;
        }
        let start = i;
        if chars[i] == '+' || chars[i] == '-' {
            i += 1;
        }
        let mut seen_dot = false;
        let mut seen_exp = false;
        while i < chars.len() {
            let c = chars[i];
            if c.is_ascii_digit() {
                i += 1;
            } else if c == '.' && !seen_dot && !seen_exp {
                seen_dot = true;
                i += 1;
            } else if (c == 'e' || c == 'E') && !seen_exp {
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
            let tok: String = chars[start..i].iter().collect();
            if let Ok(n) = tok.parse::<f64>() {
                out.push(n);
            }
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_then_scale() {
        // transform="scale(2) translate(10 0)" → translate first, then scale
        let m = Matrix::parse_transform_list("scale(2) translate(10 0)");
        let (x, y) = m.map_point(1.0, 1.0);
        // translate → (11,1), scale → (22,2)
        assert!((x - 22.0).abs() < 1e-9);
        assert!((y - 2.0).abs() < 1e-9);
    }

    #[test]
    fn rotate_90() {
        let m = Matrix::rotate_deg(90.0);
        let (x, y) = m.map_point(1.0, 0.0);
        assert!(x.abs() < 1e-9);
        assert!((y - 1.0).abs() < 1e-9);
    }
}
