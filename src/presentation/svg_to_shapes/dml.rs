//! Emit DrawingML `p:sp` elements from geometry + paint.

use super::paint::{alpha_val, rgb_hex, ColorStop, Paint};
use super::path::Segment;
use crate::element::OpenXmlElement;

const P: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";

/// Path coordinate space resolution for custGeom (higher = smoother glyph outlines).
/// 2e6 gives sub-pixel fidelity at 1280×720 while staying well under DrawingML int limits.
pub const PATH_SPACE: i64 = 2_000_000;

/// Optional DrawingML shape effect approximating an SVG filter.
#[derive(Debug, Clone, Default)]
pub struct ShapeEffect {
    /// Glow radius in EMUs (`a:glow/@rad`). Zero disables.
    pub glow_rad_emu: i64,
    pub glow_rgb: [u8; 3],
    pub glow_alpha: f64,
    /// Soft edge radius in EMUs (`a:softEdge/@rad`) — blurs the shape itself
    /// (maps SVG blur-only filters that replace SourceGraphic).
    pub soft_edge_emu: i64,
    /// Outer shadow blur radius (EMU). Zero disables.
    pub shadow_blur_emu: i64,
    pub shadow_dist_emu: i64,
    /// Shadow direction in DrawingML 1/60000 deg (0 = right, clockwise).
    pub shadow_dir: i32,
    pub shadow_rgb: [u8; 3],
    pub shadow_alpha: f64,
    /// When true, emit `a:innerShdw` instead of `a:outerShdw` (CSS inset box-shadow).
    pub shadow_inset: bool,
}

impl ShapeEffect {
    pub fn is_empty(&self) -> bool {
        self.glow_rad_emu <= 0
            && self.soft_edge_emu <= 0
            && self.shadow_blur_emu <= 0
            && self.shadow_dist_emu <= 0
    }
}

#[derive(Debug, Clone)]
pub struct ShapeBuild {
    pub id: u32,
    pub name: String,
    /// EMU bounding box.
    pub x: i64,
    pub y: i64,
    pub cx: i64,
    pub cy: i64,
    pub segments: Vec<Segment>,
    pub fill: Paint,
    pub stroke: Paint,
    pub stroke_width_emu: i64,
    pub stroke_dash: Option<&'static str>,
    pub stroke_cap: Option<&'static str>,
    pub stroke_join: Option<&'static str>,
    /// DrawingML miter lim (100000ths); ignored unless join is miter.
    pub stroke_miter_lim: Option<i64>,
    pub effect: Option<ShapeEffect>,
    /// DrawingML `a:path/@fill` ([`ST_PathFillMode`](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.pathfillmodevalues)):
    /// `none|norm|lighten|lightenLess|darken|darkenLess`. There is **no** `evenOdd` —
    /// MS PowerPoint rejects packages that use it. SVG even-odd holes must be expressed
    /// via opposite subpath winding under `norm` (TrueType glyph outlines already do).
    pub path_fill_mode: Option<&'static str>,
}

/// Map converter fill-mode tags to a legal ST_PathFillMode, or None for default `norm`.
fn sanitize_path_fill_mode(mode: Option<&str>) -> Option<&'static str> {
    match mode {
        None => None,
        Some("none") => Some("none"),
        Some("norm") => Some("norm"),
        Some("lighten") => Some("lighten"),
        Some("lightenLess") => Some("lightenLess"),
        Some("darken") => Some("darken"),
        Some("darkenLess") => Some("darkenLess"),
        // Legacy invalid token from SVG fill-rule=evenodd — drop so default norm applies.
        Some("evenOdd") | Some("evenodd") | Some("even-odd") => None,
        Some(_) => None,
    }
}

pub fn freeform_shape(b: &ShapeBuild) -> OpenXmlElement {
    let path_w = PATH_SPACE;
    let path_h = PATH_SPACE;
    let min_x = b.x as f64;
    let min_y = b.y as f64;
    let w = (b.cx as f64).max(1.0);
    let h = (b.cy as f64).max(1.0);

    let to_path = |x: f64, y: f64| -> (i64, i64) {
        (
            (((x - min_x) / w) * path_w as f64).round() as i64,
            (((y - min_y) / h) * path_h as f64).round() as i64,
        )
    };

    let mut path = OpenXmlElement::new("a", A, "path")
        .with_attribute("w", path_w.to_string())
        .with_attribute("h", path_h.to_string());
    if matches!(b.fill, Paint::None) {
        path = path.with_attribute("fill", "none");
    } else if let Some(mode) = sanitize_path_fill_mode(b.path_fill_mode) {
        path = path.with_attribute("fill", mode);
    }

    // Track pen in user space so QuadTo can be promoted to cubicBezTo.
    let mut pen = (min_x, min_y);
    for s in &b.segments {
        match *s {
            Segment::MoveTo { x, y } => {
                let (px, py) = to_path(x, y);
                path = path.with_child(pt_cmd("moveTo", px, py));
                pen = (x, y);
            }
            Segment::LineTo { x, y } => {
                let (px, py) = to_path(x, y);
                path = path.with_child(pt_cmd("lnTo", px, py));
                pen = (x, y);
            }
            Segment::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                let (p1x, p1y) = to_path(x1, y1);
                let (p2x, p2y) = to_path(x2, y2);
                let (px, py) = to_path(x, y);
                path = path.with_child(
                    OpenXmlElement::new("a", A, "cubicBezTo")
                        .with_child(pt(p1x, p1y))
                        .with_child(pt(p2x, p2y))
                        .with_child(pt(px, py)),
                );
                pen = (x, y);
            }
            Segment::QuadTo { x1, y1, x, y } => {
                // LO rasterizes quadBezTo poorly for TrueType glyph outlines.
                // Exact degree-elevation: CP1 = P0 + 2/3 (Q-P0), CP2 = P2 + 2/3 (Q-P2).
                let (p0x, p0y) = pen;
                let c1x = p0x + 2.0 / 3.0 * (x1 - p0x);
                let c1y = p0y + 2.0 / 3.0 * (y1 - p0y);
                let c2x = x + 2.0 / 3.0 * (x1 - x);
                let c2y = y + 2.0 / 3.0 * (y1 - y);
                let (p1x, p1y) = to_path(c1x, c1y);
                let (p2x, p2y) = to_path(c2x, c2y);
                let (px, py) = to_path(x, y);
                path = path.with_child(
                    OpenXmlElement::new("a", A, "cubicBezTo")
                        .with_child(pt(p1x, p1y))
                        .with_child(pt(p2x, p2y))
                        .with_child(pt(px, py)),
                );
                pen = (x, y);
            }
            Segment::Close => {
                path = path.with_child(OpenXmlElement::new("a", A, "close"));
            }
        }
    }

    let mut sp_pr = OpenXmlElement::new("p", P, "spPr")
        .with_child(xfrm(b.x, b.y, b.cx.max(1), b.cy.max(1), None))
        .with_child(
            OpenXmlElement::new("a", A, "custGeom")
                .with_child(OpenXmlElement::new("a", A, "avLst"))
                .with_child(OpenXmlElement::new("a", A, "gdLst"))
                .with_child(OpenXmlElement::new("a", A, "ahLst"))
                .with_child(OpenXmlElement::new("a", A, "cxnLst"))
                .with_child(
                    OpenXmlElement::new("a", A, "rect")
                        .with_attribute("l", "l")
                        .with_attribute("t", "t")
                        .with_attribute("r", "r")
                        .with_attribute("b", "b"),
                )
                .with_child(OpenXmlElement::new("a", A, "pathLst").with_child(path)),
        );

    sp_pr = append_fill(sp_pr, &b.fill);
    sp_pr = append_stroke(
        sp_pr,
        &b.stroke,
        b.stroke_width_emu,
        b.stroke_dash,
        b.stroke_cap,
        b.stroke_join,
        b.stroke_miter_lim,
    );
    if let Some(ref e) = b.effect {
        sp_pr = sp_pr.with_child(effect_lst(e));
    }

    shape_shell(b.id, &b.name, sp_pr, None)
}

/// Preset geometry (rect/ellipse/roundRect) when transform is axis-aligned.
pub fn preset_shape(
    id: u32,
    name: &str,
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
    preset: &str,
    fill: &Paint,
    stroke: &Paint,
    stroke_w: i64,
    round_adj: Option<i32>,
    dash: Option<&'static str>,
    cap: Option<&'static str>,
    join: Option<&'static str>,
    miter_lim: Option<i64>,
    effect: Option<&ShapeEffect>,
) -> OpenXmlElement {
    let mut av = OpenXmlElement::new("a", A, "avLst");
    if let Some(adj) = round_adj {
        av = av.with_child(
            OpenXmlElement::new("a", A, "gd")
                .with_attribute("name", "adj")
                .with_attribute("fmla", format!("val {adj}")),
        );
    }
    let mut sp_pr = OpenXmlElement::new("p", P, "spPr")
        .with_child(xfrm(x, y, cx.max(1), cy.max(1), None))
        .with_child(
            OpenXmlElement::new("a", A, "prstGeom")
                .with_attribute("prst", preset)
                .with_child(av),
        );
    sp_pr = append_fill(sp_pr, fill);
    sp_pr = append_stroke(sp_pr, stroke, stroke_w, dash, cap, join, miter_lim);
    if let Some(e) = effect {
        sp_pr = sp_pr.with_child(effect_lst(e));
    }
    shape_shell(id, name, sp_pr, None)
}

pub struct TextShapeOpts {
    pub id: u32,
    pub name: String,
    pub x: i64,
    pub y: i64,
    pub cx: i64,
    pub cy: i64,
    /// DrawingML rotation in 1/60000 deg (clockwise positive), if any.
    pub rot: Option<i32>,
    pub text: String,
    pub font_size_half_points: i64, // DrawingML sz is hundredths of a point
    pub bold: bool,
    pub italic: bool,
    pub rgb: [u8; 3],
    pub alpha: f64,
    pub align: &'static str, // l | ctr | r
    pub latin: String,
    pub ea: String,
    pub letter_spacing_emu: i64,
}

pub fn text_shape(o: &TextShapeOpts) -> OpenXmlElement {
    let mut r_pr = OpenXmlElement::new("a", A, "rPr")
        .with_attribute("lang", "zh-CN")
        .with_attribute("altLang", "en-US")
        .with_attribute("sz", o.font_size_half_points.to_string())
        .with_attribute("dirty", "0");
    if o.bold {
        r_pr = r_pr.with_attribute("b", "1");
    }
    if o.italic {
        r_pr = r_pr.with_attribute("i", "1");
    }
    if o.letter_spacing_emu != 0 {
        r_pr = r_pr.with_attribute("spc", o.letter_spacing_emu.to_string());
    }
    r_pr = r_pr.with_child(solid_fill(o.rgb, o.alpha));
    r_pr = r_pr.with_child(
        OpenXmlElement::new("a", A, "latin").with_attribute("typeface", o.latin.as_str()),
    );
    r_pr = r_pr
        .with_child(OpenXmlElement::new("a", A, "ea").with_attribute("typeface", o.ea.as_str()));
    r_pr = r_pr
        .with_child(OpenXmlElement::new("a", A, "cs").with_attribute("typeface", o.latin.as_str()));

    let run = OpenXmlElement::new("a", A, "r")
        .with_child(r_pr)
        .with_child(OpenXmlElement::new("a", A, "t").with_text(&o.text));

    let para = OpenXmlElement::new("a", A, "p")
        .with_child(OpenXmlElement::new("a", A, "pPr").with_attribute("algn", o.align))
        .with_child(run)
        .with_child(
            OpenXmlElement::new("a", A, "endParaRPr")
                .with_attribute("lang", "zh-CN")
                .with_attribute("sz", o.font_size_half_points.to_string()),
        );

    let tx_body = OpenXmlElement::new("p", P, "txBody")
        .with_child(
            OpenXmlElement::new("a", A, "bodyPr")
                .with_attribute("wrap", "none")
                .with_attribute("lIns", "0")
                .with_attribute("tIns", "0")
                .with_attribute("rIns", "0")
                .with_attribute("bIns", "0")
                .with_attribute("rtlCol", "0")
                .with_attribute("anchor", "t"),
        )
        .with_child(OpenXmlElement::new("a", A, "lstStyle"))
        .with_child(para);

    let sp_pr = OpenXmlElement::new("p", P, "spPr")
        .with_child(xfrm(o.x, o.y, o.cx.max(1), o.cy.max(1), o.rot))
        .with_child(
            OpenXmlElement::new("a", A, "prstGeom")
                .with_attribute("prst", "rect")
                .with_child(OpenXmlElement::new("a", A, "avLst")),
        )
        .with_child(OpenXmlElement::new("a", A, "noFill"));

    OpenXmlElement::new("p", P, "sp")
        .with_child(
            OpenXmlElement::new("p", P, "nvSpPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", o.id.to_string())
                        .with_attribute("name", &o.name),
                )
                .with_child(OpenXmlElement::new("p", P, "cNvSpPr").with_attribute("txBox", "1"))
                .with_child(OpenXmlElement::new("p", P, "nvPr")),
        )
        .with_child(sp_pr)
        .with_child(tx_body)
}

// ── helpers ─────────────────────────────────────────────────────────────────

fn shape_shell(
    id: u32,
    name: &str,
    sp_pr: OpenXmlElement,
    tx: Option<OpenXmlElement>,
) -> OpenXmlElement {
    let mut sp = OpenXmlElement::new("p", P, "sp").with_child(
        OpenXmlElement::new("p", P, "nvSpPr")
            .with_child(
                OpenXmlElement::new("p", P, "cNvPr")
                    .with_attribute("id", id.to_string())
                    .with_attribute("name", name),
            )
            .with_child(OpenXmlElement::new("p", P, "cNvSpPr"))
            .with_child(OpenXmlElement::new("p", P, "nvPr")),
    );
    sp = sp.with_child(sp_pr);
    if let Some(t) = tx {
        sp = sp.with_child(t);
    }
    sp
}

fn xfrm(x: i64, y: i64, cx: i64, cy: i64, rot: Option<i32>) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("a", A, "xfrm");
    if let Some(r) = rot {
        if r != 0 {
            el = el.with_attribute("rot", r.to_string());
        }
    }
    el.with_child(
        OpenXmlElement::new("a", A, "off")
            .with_attribute("x", x.to_string())
            .with_attribute("y", y.to_string()),
    )
    .with_child(
        OpenXmlElement::new("a", A, "ext")
            .with_attribute("cx", cx.to_string())
            .with_attribute("cy", cy.to_string()),
    )
}

fn effect_lst(e: &ShapeEffect) -> OpenXmlElement {
    let mut lst = OpenXmlElement::new("a", A, "effectLst");
    if e.glow_rad_emu > 0 {
        let mut clr =
            OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb_hex(e.glow_rgb));
        if (e.glow_alpha - 1.0).abs() > 0.001 {
            clr = clr.with_child(
                OpenXmlElement::new("a", A, "alpha")
                    .with_attribute("val", alpha_val(e.glow_alpha).to_string()),
            );
        }
        lst = lst.with_child(
            OpenXmlElement::new("a", A, "glow")
                .with_attribute("rad", e.glow_rad_emu.to_string())
                .with_child(clr),
        );
    }
    if e.soft_edge_emu > 0 {
        lst = lst.with_child(
            OpenXmlElement::new("a", A, "softEdge")
                .with_attribute("rad", e.soft_edge_emu.to_string()),
        );
    }
    if e.shadow_blur_emu > 0 || e.shadow_dist_emu > 0 {
        let mut clr =
            OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb_hex(e.shadow_rgb));
        if (e.shadow_alpha - 1.0).abs() > 0.001 {
            clr = clr.with_child(
                OpenXmlElement::new("a", A, "alpha")
                    .with_attribute("val", alpha_val(e.shadow_alpha).to_string()),
            );
        }
        let tag = if e.shadow_inset { "innerShdw" } else { "outerShdw" };
        let mut shdw = OpenXmlElement::new("a", A, tag)
            .with_attribute("blurRad", e.shadow_blur_emu.max(0).to_string())
            .with_attribute("dist", e.shadow_dist_emu.max(0).to_string())
            .with_attribute("dir", e.shadow_dir.to_string());
        if !e.shadow_inset {
            shdw = shdw
                .with_attribute("algn", "ctr")
                .with_attribute("rotWithShape", "0");
        }
        lst = lst.with_child(shdw.with_child(clr));
    }
    lst
}

fn pt(x: i64, y: i64) -> OpenXmlElement {
    OpenXmlElement::new("a", A, "pt")
        .with_attribute("x", x.to_string())
        .with_attribute("y", y.to_string())
}

fn pt_cmd(name: &str, x: i64, y: i64) -> OpenXmlElement {
    OpenXmlElement::new("a", A, name).with_child(pt(x, y))
}

fn solid_fill(rgb: [u8; 3], alpha: f64) -> OpenXmlElement {
    let mut clr = OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb_hex(rgb));
    if (alpha - 1.0).abs() > 0.001 {
        clr = clr.with_child(
            OpenXmlElement::new("a", A, "alpha")
                .with_attribute("val", alpha_val(alpha).to_string()),
        );
    }
    OpenXmlElement::new("a", A, "solidFill").with_child(clr)
}

fn append_fill(mut sp_pr: OpenXmlElement, paint: &Paint) -> OpenXmlElement {
    match paint {
        Paint::None => sp_pr.with_child(OpenXmlElement::new("a", A, "noFill")),
        Paint::Solid { rgb, alpha } => sp_pr.with_child(solid_fill(*rgb, *alpha)),
        Paint::LinearGradient { stops, angle, flip } => {
            sp_pr.with_child(grad_fill_linear(stops, *angle, flip.as_str()))
        }
        Paint::RadialGradient {
            stops,
            cx,
            cy,
            r: _,
            flip,
        } => sp_pr.with_child(grad_fill_radial(stops, *cx, *cy, flip.as_str())),
        Paint::PatternFill {
            prst,
            fg,
            fg_alpha,
            bg,
            bg_alpha,
        } => sp_pr.with_child(patt_fill(prst, *fg, *fg_alpha, *bg, *bg_alpha)),
    }
}

fn patt_fill(
    prst: &str,
    fg: [u8; 3],
    fg_alpha: f64,
    bg: [u8; 3],
    bg_alpha: f64,
) -> OpenXmlElement {
    let mut fg_clr = OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb_hex(fg));
    if (fg_alpha - 1.0).abs() > 0.001 {
        fg_clr = fg_clr.with_child(
            OpenXmlElement::new("a", A, "alpha")
                .with_attribute("val", alpha_val(fg_alpha).to_string()),
        );
    }
    let mut bg_clr = OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb_hex(bg));
    if (bg_alpha - 1.0).abs() > 0.001 {
        bg_clr = bg_clr.with_child(
            OpenXmlElement::new("a", A, "alpha")
                .with_attribute("val", alpha_val(bg_alpha).to_string()),
        );
    }
    OpenXmlElement::new("a", A, "pattFill")
        .with_attribute("prst", prst)
        .with_child(OpenXmlElement::new("a", A, "fgClr").with_child(fg_clr))
        .with_child(OpenXmlElement::new("a", A, "bgClr").with_child(bg_clr))
}

fn append_stroke(
    sp_pr: OpenXmlElement,
    paint: &Paint,
    width: i64,
    dash: Option<&'static str>,
    cap: Option<&'static str>,
    join: Option<&'static str>,
    miter_lim: Option<i64>,
) -> OpenXmlElement {
    let cap_val = cap.unwrap_or("rnd");
    let join_el = match join.unwrap_or("round") {
        "bevel" => OpenXmlElement::new("a", A, "bevel"),
        "miter" => OpenXmlElement::new("a", A, "miter").with_attribute(
            "lim",
            miter_lim.unwrap_or(400_000).to_string(),
        ),
        _ => OpenXmlElement::new("a", A, "round"),
    };
    match paint {
        Paint::None => sp_pr.with_child(
            OpenXmlElement::new("a", A, "ln").with_child(OpenXmlElement::new("a", A, "noFill")),
        ),
        Paint::Solid { rgb, alpha } => {
            let mut ln = OpenXmlElement::new("a", A, "ln")
                .with_attribute("w", width.max(1270).to_string())
                .with_attribute("cap", cap_val)
                .with_attribute("cmpd", "sng")
                .with_attribute("algn", "ctr")
                .with_child(solid_fill(*rgb, *alpha));
            let dash_val = dash.unwrap_or("solid");
            ln = ln.with_child(
                OpenXmlElement::new("a", A, "prstDash").with_attribute("val", dash_val),
            );
            ln = ln.with_child(join_el);
            sp_pr.with_child(ln)
        }
        Paint::LinearGradient { stops, angle, flip } => {
            let mut ln = OpenXmlElement::new("a", A, "ln")
                .with_attribute("w", width.max(1270).to_string())
                .with_attribute("cap", cap_val)
                .with_child(grad_fill_linear(stops, *angle, flip.as_str()));
            ln = ln
                .with_child(OpenXmlElement::new("a", A, "prstDash").with_attribute("val", "solid"));
            ln = ln.with_child(join_el);
            sp_pr.with_child(ln)
        }
        Paint::RadialGradient { stops, cx, cy, flip, .. } => {
            let mut ln = OpenXmlElement::new("a", A, "ln")
                .with_attribute("w", width.max(1270).to_string())
                .with_attribute("cap", cap_val)
                .with_child(grad_fill_radial(stops, *cx, *cy, flip.as_str()));
            ln = ln
                .with_child(OpenXmlElement::new("a", A, "prstDash").with_attribute("val", "solid"));
            ln = ln.with_child(join_el);
            sp_pr.with_child(ln)
        }
        Paint::PatternFill {
            prst,
            fg,
            fg_alpha,
            bg,
            bg_alpha,
        } => {
            let mut ln = OpenXmlElement::new("a", A, "ln")
                .with_attribute("w", width.max(1270).to_string())
                .with_attribute("cap", cap_val)
                .with_child(patt_fill(prst, *fg, *fg_alpha, *bg, *bg_alpha));
            ln = ln
                .with_child(OpenXmlElement::new("a", A, "prstDash").with_attribute("val", "solid"));
            ln = ln.with_child(join_el);
            sp_pr.with_child(ln)
        }
    }
}

fn gs_list(stops: &[ColorStop]) -> OpenXmlElement {
    let mut lst = OpenXmlElement::new("a", A, "gsLst");
    for s in stops {
        let pos = ((s.offset.clamp(0.0, 1.0) * 100_000.0).round() as i64).clamp(0, 100_000);
        let mut clr = OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb_hex(s.rgb));
        if (s.alpha - 1.0).abs() > 0.001 {
            clr = clr.with_child(
                OpenXmlElement::new("a", A, "alpha")
                    .with_attribute("val", alpha_val(s.alpha).to_string()),
            );
        }
        lst = lst.with_child(
            OpenXmlElement::new("a", A, "gs")
                .with_attribute("pos", pos.to_string())
                .with_child(clr),
        );
    }
    lst
}

fn grad_fill_linear(stops: &[ColorStop], angle: i32, flip: &str) -> OpenXmlElement {
    OpenXmlElement::new("a", A, "gradFill")
        .with_attribute("flip", flip)
        .with_attribute("rotWithShape", "1")
        .with_child(gs_list(stops))
        .with_child(
            OpenXmlElement::new("a", A, "lin")
                .with_attribute("ang", angle.to_string())
                .with_attribute("scaled", "0"),
        )
}

fn grad_fill_radial(stops: &[ColorStop], cx: f64, cy: f64, flip: &str) -> OpenXmlElement {
    // DrawingML path="circle" with fillToRect defines the focus rectangle.
    // SVG radial focus (cx,cy) in 0..1 maps to:
    //   l = cx*100000, t = cy*100000, r = (1-cx)*100000, b = (1-cy)*100000
    // so the focus point sits at (cx,cy) of the shape.
    let to_pct = |v: f64| ((v.clamp(0.0, 1.0) * 100_000.0).round() as i64).clamp(0, 100_000);
    let l = to_pct(cx);
    let t = to_pct(cy);
    let r = to_pct(1.0 - cx);
    let b = to_pct(1.0 - cy);
    OpenXmlElement::new("a", A, "gradFill")
        .with_attribute("flip", flip)
        .with_attribute("rotWithShape", "1")
        .with_child(gs_list(stops))
        .with_child(
            OpenXmlElement::new("a", A, "path")
                .with_attribute("path", "circle")
                .with_child(
                    OpenXmlElement::new("a", A, "fillToRect")
                        .with_attribute("l", l.to_string())
                        .with_attribute("t", t.to_string())
                        .with_attribute("r", r.to_string())
                        .with_attribute("b", b.to_string()),
                ),
        )
}
