//! PresentationML helpers (minimal core for `.pptx`).


pub mod svg_to_shapes;
pub use svg_to_shapes::{
    shape_tree_from_svg, svg_to_shapes, svg_to_shapes_ex, svg_to_shapes_with_options,
    SvgShapeConversion, SvgToShapesOptions, UsedFont,
};
use crate::element::OpenXmlElement;
use crate::namespace::ns;

const P: &str = ns::PRESENTATIONML.uri;
const A: &str = ns::DRAWINGML.uri;
const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// `p:presentation` root.
pub fn presentation(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "presentation")
        .with_ns_decl("p", P)
        .with_ns_decl("r", R)
        .with_ns_decl("a", A)
        .with_attribute("saveSubsetFonts", "1")
        .with_children(children)
}

/// Standard 16:9 slide size in EMUs (13.333" × 7.5").
pub const SLIDE_SIZE_16_9: (i64, i64) = (12_192_000, 6_858_000);
/// Standard 4:3 slide size in EMUs (10" × 7.5").
pub const SLIDE_SIZE_4_3: (i64, i64) = (9_144_000, 6_858_000);

/// `p:sldSz` element (cx/cy in EMUs).
pub fn slide_size(cx: i64, cy: i64) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sldSz")
        .with_attribute("cx", cx.to_string())
        .with_attribute("cy", cy.to_string())
}

/// `p:notesSz` element.
pub fn notes_size(cx: i64, cy: i64) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "notesSz")
        .with_attribute("cx", cx.to_string())
        .with_attribute("cy", cy.to_string())
}

/// `p:sldIdLst`.
pub fn slide_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sldIdLst").with_children(children)
}

/// `p:sldId`.
pub fn slide_id(id: u32, relationship_id: &str) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("p", P, "sldId");
    el.set_attribute("id", id.to_string());
    el.set_attribute_ns("r", R, "id", relationship_id);
    el
}

/// `p:sld` (slide) root.
pub fn slide(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sld")
        .with_ns_decl("p", P)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_children(children)
}

/// `p:cSld`.
pub fn common_slide_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "cSld").with_children(children)
}

/// `p:spTree`.
pub fn shape_tree(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "spTree").with_children(children)
}

/// Non-visual group shape properties required at the start of `p:spTree`.
pub fn group_shape_properties() -> OpenXmlElement {
    OpenXmlElement::new("p", P, "nvGrpSpPr")
        .with_child(
            OpenXmlElement::new("p", P, "cNvPr")
                .with_attribute("id", "1")
                .with_attribute("name", ""),
        )
        .with_child(OpenXmlElement::new("p", P, "cNvGrpSpPr"))
        .with_child(OpenXmlElement::new("p", P, "nvPr"))
}

/// Group shape properties (`p:grpSpPr`) with identity transform (required by PowerPoint).
///
/// Root `p:spTree` extents must match the slide size. Zero `cx`/`cy` is tolerated by
/// LibreOffice but Microsoft PowerPoint rejects the package as unreadable.
pub fn group_shape_pr() -> OpenXmlElement {
    group_shape_pr_sized(SLIDE_SIZE_16_9.0, SLIDE_SIZE_16_9.1)
}

/// `p:grpSpPr` with explicit child/parent extents (EMUs).
pub fn group_shape_pr_sized(cx: i64, cy: i64) -> OpenXmlElement {
    let cx_s = cx.max(0).to_string();
    let cy_s = cy.max(0).to_string();
    OpenXmlElement::new("p", P, "grpSpPr").with_child(
        OpenXmlElement::new("a", A, "xfrm")
            .with_child(
                OpenXmlElement::new("a", A, "off")
                    .with_attribute("x", "0")
                    .with_attribute("y", "0"),
            )
            .with_child(
                OpenXmlElement::new("a", A, "ext")
                    .with_attribute("cx", &cx_s)
                    .with_attribute("cy", &cy_s),
            )
            .with_child(
                OpenXmlElement::new("a", A, "chOff")
                    .with_attribute("x", "0")
                    .with_attribute("y", "0"),
            )
            .with_child(
                OpenXmlElement::new("a", A, "chExt")
                    .with_attribute("cx", &cx_s)
                    .with_attribute("cy", &cy_s),
            ),
    )
}

/// `a:off` + `a:ext` transform for a shape.
pub fn transform_2d(x: i64, y: i64, cx: i64, cy: i64) -> OpenXmlElement {
    OpenXmlElement::new("a", A, "xfrm")
        .with_child(
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

/// DrawingML paragraph containing a single text run.
pub fn drawing_paragraph(text: &str) -> OpenXmlElement {
    OpenXmlElement::new("a", A, "p").with_child(
        OpenXmlElement::new("a", A, "r")
            .with_child(OpenXmlElement::new("a", A, "rPr").with_attribute("lang", "en-US"))
            .with_child(OpenXmlElement::new("a", A, "t").with_text(text)),
    )
}

/// `p:txBody` with body properties and one or more paragraphs of text.
pub fn text_body(paragraphs: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let mut kids = vec![
        OpenXmlElement::new("a", A, "bodyPr"),
        OpenXmlElement::new("a", A, "lstStyle"),
    ];
    kids.extend(paragraphs);
    OpenXmlElement::new("p", P, "txBody").with_children(kids)
}

/// Build a DrawingML table (`a:tbl`) with string cells.
///
/// Column widths are equal divisions of `total_width` EMUs.
pub fn drawing_table(rows: &[Vec<&str>], total_width: i64) -> OpenXmlElement {
    let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(0).max(1);
    let col_w = total_width / col_count as i64;
    let mut grid_cols = Vec::new();
    for _ in 0..col_count {
        grid_cols.push(
            OpenXmlElement::new("a", A, "gridCol").with_attribute("w", col_w.to_string()),
        );
    }
    let mut tbl_rows = Vec::new();
    for row in rows {
        let mut cells = Vec::new();
        for i in 0..col_count {
            let text = row.get(i).copied().unwrap_or("");
            cells.push(
                OpenXmlElement::new("a", A, "tc")
                    .with_child(
                        OpenXmlElement::new("a", A, "txBody")
                            .with_child(OpenXmlElement::new("a", A, "bodyPr"))
                            .with_child(OpenXmlElement::new("a", A, "lstStyle"))
                            .with_child(drawing_paragraph(text)),
                    )
                    .with_child(OpenXmlElement::new("a", A, "tcPr")),
            );
        }
        tbl_rows.push(
            OpenXmlElement::new("a", A, "tr")
                .with_attribute("h", "370840")
                .with_children(cells),
        );
    }
    OpenXmlElement::new("a", A, "tbl")
        .with_child(
            OpenXmlElement::new("a", A, "tblPr")
                .with_attribute("firstRow", "1")
                .with_attribute("bandRow", "1")
                .with_child(
                    OpenXmlElement::new("a", A, "tableStyleId")
                        .with_text("{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}"),
                ),
        )
        .with_child(
            OpenXmlElement::new("a", A, "tblGrid").with_children(grid_cols),
        )
        .with_children(tbl_rows)
}

/// A graphic frame containing a DrawingML table, positioned on the slide.
pub fn table_graphic_frame(
    id: u32,
    name: &str,
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
    rows: &[Vec<&str>],
) -> OpenXmlElement {
    let tbl = drawing_table(rows, cx);
    OpenXmlElement::new("p", P, "graphicFrame")
        .with_child(
            OpenXmlElement::new("p", P, "nvGraphicFramePr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", id.to_string())
                        .with_attribute("name", name),
                )
                .with_child(OpenXmlElement::new("p", P, "cNvGraphicFramePr").with_child(
                    OpenXmlElement::new("a", A, "graphicFrameLocks")
                        .with_attribute("noGrp", "1"),
                ))
                .with_child(OpenXmlElement::new("p", P, "nvPr")),
        )
        .with_child(
            // graphicFrame uses p:xfrm (same shape as a:xfrm)
            OpenXmlElement::new("p", P, "xfrm")
                .with_child(
                    OpenXmlElement::new("a", A, "off")
                        .with_attribute("x", x.to_string())
                        .with_attribute("y", y.to_string()),
                )
                .with_child(
                    OpenXmlElement::new("a", A, "ext")
                        .with_attribute("cx", cx.to_string())
                        .with_attribute("cy", cy.to_string()),
                ),
        )
        .with_child(
            OpenXmlElement::new("a", A, "graphic").with_child(
                OpenXmlElement::new("a", A, "graphicData")
                    .with_attribute(
                        "uri",
                        "http://schemas.openxmlformats.org/drawingml/2006/table",
                    )
                    .with_child(tbl),
            ),
        )
}

/// A picture shape referencing an image relationship (`r:embed`).
pub fn picture_shape(
    id: u32,
    name: &str,
    image_rel_id: &str,
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "pic")
        .with_child(
            OpenXmlElement::new("p", P, "nvPicPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", id.to_string())
                        .with_attribute("name", name),
                )
                .with_child(OpenXmlElement::new("p", P, "cNvPicPr").with_child(
                    OpenXmlElement::new("a", A, "picLocks").with_attribute("noChangeAspect", "1"),
                ))
                .with_child(OpenXmlElement::new("p", P, "nvPr")),
        )
        .with_child(
            OpenXmlElement::new("p", P, "blipFill")
                .with_child(
                    OpenXmlElement::new("a", A, "blip").with_attribute_qname("r:embed", image_rel_id),
                )
                .with_child(
                    OpenXmlElement::new("a", A, "stretch")
                        .with_child(OpenXmlElement::new("a", A, "fillRect")),
                ),
        )
        .with_child(
            OpenXmlElement::new("p", P, "spPr")
                .with_child(transform_2d(x, y, cx, cy))
                .with_child(
                    OpenXmlElement::new("a", A, "prstGeom")
                        .with_attribute("prst", "rect")
                        .with_child(OpenXmlElement::new("a", A, "avLst")),
                ),
        )
}

/// Picture with PNG base blip + Office SVG extension (`asvg:svgBlip`), matching C# SvgExample.
///
/// `png_rel_id` is the fallback raster embed on `a:blip`; `svg_rel_id` is referenced from
/// `asvg:svgBlip` so PowerPoint 2016+ can render the vector.
pub fn picture_shape_with_svg(
    id: u32,
    name: &str,
    png_rel_id: &str,
    svg_rel_id: &str,
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
) -> OpenXmlElement {
    const ASVG: &str = "http://schemas.microsoft.com/office/drawing/2016/SVG/main";
    let svg_blip = OpenXmlElement::new("asvg", ASVG, "svgBlip")
        .with_ns_decl("asvg", ASVG)
        .with_attribute_qname("r:embed", svg_rel_id);
    let ext = OpenXmlElement::new("a", A, "ext")
        .with_attribute("uri", "{96DAC541-7B7A-43D3-8B79-37D633B846F1}")
        .with_child(svg_blip);
    let blip = OpenXmlElement::new("a", A, "blip")
        .with_attribute_qname("r:embed", png_rel_id)
        .with_child(OpenXmlElement::new("a", A, "extLst").with_child(ext));
    OpenXmlElement::new("p", P, "pic")
        .with_child(
            OpenXmlElement::new("p", P, "nvPicPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", id.to_string())
                        .with_attribute("name", name),
                )
                .with_child(OpenXmlElement::new("p", P, "cNvPicPr").with_child(
                    OpenXmlElement::new("a", A, "picLocks").with_attribute("noChangeAspect", "1"),
                ))
                .with_child(OpenXmlElement::new("p", P, "nvPr")),
        )
        .with_child(
            OpenXmlElement::new("p", P, "blipFill")
                .with_child(blip)
                .with_child(
                    OpenXmlElement::new("a", A, "stretch")
                        .with_child(OpenXmlElement::new("a", A, "fillRect")),
                ),
        )
        .with_child(
            OpenXmlElement::new("p", P, "spPr")
                .with_child(transform_2d(x, y, cx, cy))
                .with_child(
                    OpenXmlElement::new("a", A, "prstGeom")
                        .with_attribute("prst", "rect")
                        .with_child(OpenXmlElement::new("a", A, "avLst")),
                ),
        )
}


/// SVG-only picture matching PowerPoint's native insert (no PNG fallback).
///
/// `a:blip` has **no** `r:embed`; the SVG is referenced only via
/// `a:extLst/a:ext/asvg:svgBlip/@r:embed` (uri `{96DAC541-7B7A-43D3-8B79-37D633B846F1}`).
pub fn picture_shape_svg(
    id: u32,
    name: &str,
    svg_rel_id: &str,
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
) -> OpenXmlElement {
    const ASVG: &str = "http://schemas.microsoft.com/office/drawing/2016/SVG/main";
    let svg_blip = OpenXmlElement::new("asvg", ASVG, "svgBlip")
        .with_ns_decl("asvg", ASVG)
        .with_attribute_qname("r:embed", svg_rel_id);
    let ext = OpenXmlElement::new("a", A, "ext")
        .with_attribute("uri", "{96DAC541-7B7A-43D3-8B79-37D633B846F1}")
        .with_child(svg_blip);
    let blip = OpenXmlElement::new("a", A, "blip")
        .with_child(OpenXmlElement::new("a", A, "extLst").with_child(ext));
    OpenXmlElement::new("p", P, "pic")
        .with_child(
            OpenXmlElement::new("p", P, "nvPicPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", id.to_string())
                        .with_attribute("name", name),
                )
                .with_child(OpenXmlElement::new("p", P, "cNvPicPr").with_child(
                    OpenXmlElement::new("a", A, "picLocks").with_attribute("noChangeAspect", "1"),
                ))
                .with_child(OpenXmlElement::new("p", P, "nvPr")),
        )
        .with_child(
            OpenXmlElement::new("p", P, "blipFill")
                .with_child(blip)
                .with_child(
                    OpenXmlElement::new("a", A, "stretch")
                        .with_child(OpenXmlElement::new("a", A, "fillRect")),
                ),
        )
        .with_child(
            OpenXmlElement::new("p", P, "spPr")
                .with_child(transform_2d(x, y, cx, cy))
                .with_child(
                    OpenXmlElement::new("a", A, "prstGeom")
                        .with_attribute("prst", "rect")
                        .with_child(OpenXmlElement::new("a", A, "avLst")),
                ),
        )
}

/// A simple text box shape positioned on the slide.
pub fn text_shape(id: u32, name: &str, x: i64, y: i64, cx: i64, cy: i64, text: &str) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sp")
        .with_child(
            OpenXmlElement::new("p", P, "nvSpPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", id.to_string())
                        .with_attribute("name", name),
                )
                .with_child(
                    OpenXmlElement::new("p", P, "cNvSpPr").with_attribute("txBox", "1"),
                )
                .with_child(OpenXmlElement::new("p", P, "nvPr")),
        )
        .with_child(
            OpenXmlElement::new("p", P, "spPr")
                .with_child(transform_2d(x, y, cx, cy))
                .with_child(
                    OpenXmlElement::new("a", A, "prstGeom")
                        .with_attribute("prst", "rect")
                        .with_child(OpenXmlElement::new("a", A, "avLst")),
                ),
        )
        .with_child(text_body(vec![drawing_paragraph(text)]))
}

/// A preset auto-shape (rectangle, ellipse, roundRect, …) with optional solid fill.
///
/// `preset` is a DrawingML preset geometry name (`rect`, `ellipse`, `roundRect`,
/// `triangle`, `rtTriangle`, `diamond`, `pentagon`, `hexagon`, `star5`, …).
/// `fill_rgb` is an optional 6-digit hex color without `#` (e.g. `"4472C4"`).
pub fn auto_shape(
    id: u32,
    name: &str,
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
    preset: &str,
    fill_rgb: Option<&str>,
) -> OpenXmlElement {
    let mut sp_pr = OpenXmlElement::new("p", P, "spPr")
        .with_child(transform_2d(x, y, cx, cy))
        .with_child(
            OpenXmlElement::new("a", A, "prstGeom")
                .with_attribute("prst", preset)
                .with_child(OpenXmlElement::new("a", A, "avLst")),
        );
    if let Some(rgb) = fill_rgb {
        sp_pr = sp_pr.with_child(
            OpenXmlElement::new("a", A, "solidFill").with_child(
                OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb),
            ),
        );
    }
    OpenXmlElement::new("p", P, "sp")
        .with_child(
            OpenXmlElement::new("p", P, "nvSpPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", id.to_string())
                        .with_attribute("name", name),
                )
                .with_child(OpenXmlElement::new("p", P, "cNvSpPr"))
                .with_child(OpenXmlElement::new("p", P, "nvPr")),
        )
        .with_child(sp_pr)
}

/// Build a complete slide containing a single centered-ish text box.
pub fn slide_with_text(text: &str) -> OpenXmlElement {
    // EMUs: 914400 per inch. Standard slide ~10" x 7.5".
    let shape = text_shape(
        2,
        "Title 1",
        457200,  // 0.5"
        1371600, // 1.5"
        8229600, // 9"
        1143000, // 1.25"
        text,
    );
    slide(vec![common_slide_data(vec![shape_tree(vec![
        group_shape_properties(),
        group_shape_pr(),
        shape,
    ])])])
}

/// Collect all DrawingML text (`a:t`) from a slide element.
pub fn slide_texts(root: &OpenXmlElement) -> Vec<String> {
    root.descendants()
        .filter(|e| e.prefix == "a" && e.local_name == "t" || e.local_name == "t" && e.namespace_uri == A)
        .filter_map(|e| e.text_value().map(|s| s.to_string()))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Replace all occurrences of `from` with `to` in every `a:t` text node under `root`.
///
/// Returns the number of individual string replacements performed.
pub fn replace_slide_text(root: &mut OpenXmlElement, from: &str, to: &str) -> usize {
    if from.is_empty() {
        return 0;
    }
    let mut count = 0;
    replace_text_recursive(root, from, to, &mut count);
    count
}

fn replace_text_recursive(elem: &mut OpenXmlElement, from: &str, to: &str, count: &mut usize) {
    if elem.local_name == "t" {
        if let Some(text) = &mut elem.text {
            if text.contains(from) {
                let occurrences = text.matches(from).count();
                *count += occurrences;
                *text = text.replace(from, to);
            }
        }
    }
    for child in &mut elem.children {
        replace_text_recursive(child, from, to, count);
    }
}

/// `p:sldMasterIdLst`.
pub fn slide_master_id_list(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sldMasterIdLst").with_children(children)
}

/// `p:sldMasterId`.
pub fn slide_master_id(id: u32, relationship_id: &str) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sldMasterId")
        .with_attribute("id", id.to_string())
        .with_attribute_ns("r", R, "id", relationship_id)
}

/// Minimal empty shape tree with required nvGrpSpPr/grpSpPr.
pub fn empty_shape_tree() -> OpenXmlElement {
    shape_tree(vec![group_shape_properties(), group_shape_pr()])
}

/// Empty title + subtitle placeholders matching Office `type="title"` layout (layout1).
pub fn title_subtitle_placeholders() -> Vec<OpenXmlElement> {
    let title = OpenXmlElement::new("p", P, "sp")
        .with_child(
            OpenXmlElement::new("p", P, "nvSpPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", "2")
                        .with_attribute("name", "Title 1"),
                )
                .with_child(
                    OpenXmlElement::new("p", P, "cNvSpPr").with_child(
                        OpenXmlElement::new("a", A, "spLocks").with_attribute("noGrp", "1"),
                    ),
                )
                .with_child(
                    OpenXmlElement::new("p", P, "nvPr").with_child(
                        OpenXmlElement::new("p", P, "ph").with_attribute("type", "ctrTitle"),
                    ),
                ),
        )
        .with_child(OpenXmlElement::new("p", P, "spPr"))
        .with_child(
            OpenXmlElement::new("p", P, "txBody")
                .with_child(OpenXmlElement::new("a", A, "bodyPr"))
                .with_child(OpenXmlElement::new("a", A, "lstStyle"))
                .with_child(
                    OpenXmlElement::new("a", A, "p").with_child(
                        OpenXmlElement::new("a", A, "endParaRPr")
                            .with_attribute("lang", "zh-CN")
                            .with_attribute("altLang", "en-US"),
                    ),
                ),
        );
    let subtitle = OpenXmlElement::new("p", P, "sp")
        .with_child(
            OpenXmlElement::new("p", P, "nvSpPr")
                .with_child(
                    OpenXmlElement::new("p", P, "cNvPr")
                        .with_attribute("id", "3")
                        .with_attribute("name", "Subtitle 2"),
                )
                .with_child(
                    OpenXmlElement::new("p", P, "cNvSpPr").with_child(
                        OpenXmlElement::new("a", A, "spLocks").with_attribute("noGrp", "1"),
                    ),
                )
                .with_child(
                    OpenXmlElement::new("p", P, "nvPr").with_child(
                        OpenXmlElement::new("p", P, "ph")
                            .with_attribute("type", "subTitle")
                            .with_attribute("idx", "1"),
                    ),
                ),
        )
        .with_child(OpenXmlElement::new("p", P, "spPr"))
        .with_child(
            OpenXmlElement::new("p", P, "txBody")
                .with_child(OpenXmlElement::new("a", A, "bodyPr"))
                .with_child(OpenXmlElement::new("a", A, "lstStyle"))
                .with_child(
                    OpenXmlElement::new("a", A, "p").with_child(
                        OpenXmlElement::new("a", A, "endParaRPr")
                            .with_attribute("lang", "zh-CN")
                            .with_attribute("altLang", "en-US"),
                    ),
                ),
        );
    vec![title, subtitle]
}

/// Color mapping required by slide masters/layouts.
pub fn color_map() -> OpenXmlElement {
    OpenXmlElement::new("p", P, "clrMap")
        .with_attribute("bg1", "lt1")
        .with_attribute("tx1", "dk1")
        .with_attribute("bg2", "lt2")
        .with_attribute("tx2", "dk2")
        .with_attribute("accent1", "accent1")
        .with_attribute("accent2", "accent2")
        .with_attribute("accent3", "accent3")
        .with_attribute("accent4", "accent4")
        .with_attribute("accent5", "accent5")
        .with_attribute("accent6", "accent6")
        .with_attribute("hlink", "hlink")
        .with_attribute("folHlink", "folHlink")
}

/// `p:sldLayoutIdLst` entry.
pub fn slide_layout_id(id: u32, relationship_id: &str) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sldLayoutId")
        .with_attribute("id", id.to_string())
        .with_attribute_ns("r", R, "id", relationship_id)
}

/// Minimal `p:sldLayout` (blank layout).
///
/// Uses `clrMapOvr` with master color mapping (Office rejects a full `clrMap` on layouts).
pub fn slide_layout(name: &str) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sldLayout")
        .with_ns_decl("p", P)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_attribute("type", "blank")
        .with_attribute("preserve", "1")
        .with_child(common_slide_data(vec![empty_shape_tree()]).with_attribute("name", name))
        .with_child(
            OpenXmlElement::new("p", P, "clrMapOvr")
                .with_child(OpenXmlElement::new("a", A, "masterClrMapping")),
        )
}

/// Minimal text styles required on slide masters (title/body/other).
pub fn master_text_styles() -> OpenXmlElement {
    let lvl = |sz: &str| {
        OpenXmlElement::new("a", A, "lvl1pPr")
            .with_attribute("algn", "l")
            .with_attribute("defTabSz", "914400")
            .with_attribute("rtl", "0")
            .with_attribute("eaLnBrk", "1")
            .with_attribute("latinLnBrk", "0")
            .with_attribute("hangingPunct", "1")
            .with_child(
                OpenXmlElement::new("a", A, "defRPr")
                    .with_attribute("sz", sz)
                    .with_attribute("kern", "1200")
                    .with_child(
                        OpenXmlElement::new("a", A, "solidFill").with_child(
                            OpenXmlElement::new("a", A, "schemeClr").with_attribute("val", "tx1"),
                        ),
                    )
                    .with_child(
                        OpenXmlElement::new("a", A, "latin").with_attribute("typeface", "+mn-lt"),
                    ),
            )
    };
    OpenXmlElement::new("p", P, "txStyles")
        .with_child(OpenXmlElement::new("p", P, "titleStyle").with_child(lvl("4400")))
        .with_child(OpenXmlElement::new("p", P, "bodyStyle").with_child(lvl("2800")))
        .with_child(OpenXmlElement::new("p", P, "otherStyle").with_child(lvl("1800")))
}

/// Minimal `p:sldMaster` with a layout id list child (caller supplies layout rels in list).
pub fn slide_master(layout_ids: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "sldMaster")
        .with_ns_decl("p", P)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(common_slide_data(vec![empty_shape_tree()]))
        .with_child(color_map())
        .with_child(OpenXmlElement::new("p", P, "sldLayoutIdLst").with_children(layout_ids))
        .with_child(master_text_styles())
}

/// `p:cSld` with an optional name attribute.
pub fn common_slide_data_named(
    name: &str,
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "cSld")
        .with_attribute("name", name)
        .with_children(children)
}

/// Minimal `p:notes` (notes slide) with a text body.
pub fn notes_slide(text: &str) -> OpenXmlElement {
    let shape = text_shape(
        2,
        "Notes Placeholder",
        685_800,
        685_800,
        5_486_400,
        3_657_600,
        text,
    );
    OpenXmlElement::new("p", P, "notes")
        .with_ns_decl("p", P)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(common_slide_data(vec![shape_tree(vec![
            group_shape_properties(),
            group_shape_pr(),
            shape,
        ])]))
}

/// Minimal blank `p:notesMaster`.
pub fn notes_master() -> OpenXmlElement {
    OpenXmlElement::new("p", P, "notesMaster")
        .with_ns_decl("p", P)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(common_slide_data(vec![empty_shape_tree()]))
        .with_child(color_map())
}

/// Minimal blank `p:handoutMaster`.
pub fn handout_master() -> OpenXmlElement {
    OpenXmlElement::new("p", P, "handoutMaster")
        .with_ns_decl("p", P)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(common_slide_data(vec![empty_shape_tree()]))
        .with_child(color_map())
}

const P14: &str = "http://schemas.microsoft.com/office/powerpoint/2010/main";

/// A PPT section entry (`p14:section`) with a name and optional slide id list.
///
/// `slide_ids` are the presentation slide id values (`p:sldId/@id`).
pub fn section(name: &str, section_id: &str, slide_ids: &[u32]) -> OpenXmlElement {
    let mut sld_id_lst = OpenXmlElement::new("p14", P14, "sldIdLst");
    for id in slide_ids {
        sld_id_lst.append_child(
            OpenXmlElement::new("p14", P14, "sldId").with_attribute("id", id.to_string()),
        );
    }
    OpenXmlElement::new("p14", P14, "section")
        .with_attribute("name", name)
        .with_attribute("id", section_id)
        .with_child(sld_id_lst)
}

/// `p14:sectionLst` container for presentation sections.
pub fn section_list(sections: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("p14", P14, "sectionLst")
        .with_ns_decl("p14", P14)
        .with_children(sections)
}

/// Presentation extension wrapping a section list (for `p:extLst`).
pub fn section_list_ext(sections: OpenXmlElement) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "ext")
        .with_attribute("uri", "{521415D9-36F7-43E2-AB2F-B90AF26B5E84}")
        .with_ns_decl("p14", P14)
        .with_child(sections)
}

/// A slide comment (`p:cm`) at position (x,y) EMUs.
pub fn slide_comment(
    author_id: u32,
    index: u32,
    datetime: &str,
    x: i64,
    y: i64,
    text: &str,
) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "cm")
        .with_attribute("authorId", author_id.to_string())
        .with_attribute("dt", datetime)
        .with_attribute("idx", index.to_string())
        .with_child(
            OpenXmlElement::new("p", P, "pos")
                .with_attribute("x", x.to_string())
                .with_attribute("y", y.to_string()),
        )
        .with_child(
            OpenXmlElement::new("p", P, "text").with_text(text),
        )
}

/// Slide comments list root (`p:cmLst`).
pub fn slide_comments(
    comments: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "cmLst")
        .with_ns_decl("p", P)
        .with_children(comments)
}

/// Slide transition (`p:transition`) with a named effect child.
///
/// `effect` is a local name such as `"fade"`, `"dissolve"`, `"push"`, `"wipe"`, `"split"`.
/// `speed` is `"slow"`, `"med"`, or `"fast"`. `advance_after_ms` sets auto-advance.
pub fn slide_transition(
    effect: &str,
    speed: &str,
    advance_on_click: bool,
    advance_after_ms: Option<u32>,
) -> OpenXmlElement {
    let mut tr = OpenXmlElement::new("p", P, "transition").with_attribute("spd", speed);
    if advance_on_click {
        tr.set_attribute("advClick", "1");
    } else {
        tr.set_attribute("advClick", "0");
    }
    if let Some(ms) = advance_after_ms {
        tr.set_attribute("advTm", ms.to_string());
    }
    // Effect elements are empty leaves under transition
    tr.append_child(OpenXmlElement::new("p", P, effect));
    tr
}

/// Fade transition helper.
pub fn fade_transition(speed: &str) -> OpenXmlElement {
    slide_transition("fade", speed, true, None)
}

/// Dissolve transition helper.
pub fn dissolve_transition(speed: &str) -> OpenXmlElement {
    slide_transition("dissolve", speed, true, None)
}

/// Push transition helper.
pub fn push_transition(speed: &str) -> OpenXmlElement {
    slide_transition("push", speed, true, None)
}

/// Wipe transition helper.
pub fn wipe_transition(speed: &str) -> OpenXmlElement {
    slide_transition("wipe", speed, true, None)
}

/// Split transition helper.
pub fn split_transition(speed: &str) -> OpenXmlElement {
    slide_transition("split", speed, true, None)
}

/// Cover transition helper.
pub fn cover_transition(speed: &str) -> OpenXmlElement {
    slide_transition("cover", speed, true, None)
}

/// Wheel transition helper.
pub fn wheel_transition(speed: &str) -> OpenXmlElement {
    slide_transition("wheel", speed, true, None)
}

/// Random transition helper.
pub fn random_transition(speed: &str) -> OpenXmlElement {
    slide_transition("random", speed, true, None)
}


/// Blinds transition helper.
pub fn blinds_transition(speed: &str) -> OpenXmlElement {
    slide_transition("blinds", speed, true, None)
}

/// Checker transition helper.
pub fn checker_transition(speed: &str) -> OpenXmlElement {
    slide_transition("checker", speed, true, None)
}

/// Circle transition helper.
pub fn circle_transition(speed: &str) -> OpenXmlElement {
    slide_transition("circle", speed, true, None)
}

/// Diamond transition helper.
pub fn diamond_transition(speed: &str) -> OpenXmlElement {
    slide_transition("diamond", speed, true, None)
}

/// Plus transition helper.
pub fn plus_transition(speed: &str) -> OpenXmlElement {
    slide_transition("plus", speed, true, None)
}

/// Newsflash transition helper.
pub fn newsflash_transition(speed: &str) -> OpenXmlElement {
    slide_transition("newsflash", speed, true, None)
}

/// Strips transition helper.
pub fn strips_transition(speed: &str) -> OpenXmlElement {
    slide_transition("strips", speed, true, None)
}

/// Wedge transition helper.
pub fn wedge_transition(speed: &str) -> OpenXmlElement {
    slide_transition("wedge", speed, true, None)
}

/// Zoom transition helper.
pub fn zoom_transition(speed: &str) -> OpenXmlElement {
    slide_transition("zoom", speed, true, None)
}

/// Solid color slide background (`p:bg` / `p:bgPr` / `a:solidFill`).
pub fn solid_slide_background(rgb: &str) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "bg").with_child(
        OpenXmlElement::new("p", P, "bgPr")
            .with_child(
                OpenXmlElement::new("a", A, "solidFill").with_child(
                    OpenXmlElement::new("a", A, "srgbClr").with_attribute("val", rgb),
                ),
            )
            .with_child(OpenXmlElement::new("a", A, "effectLst")),
    )
}

/// Header/footer placeholders on a slide (`p:hf`).
///
/// Controls visibility of date, footer text, and slide number.
pub fn header_footer(
    show_date: bool,
    show_footer: bool,
    show_slide_number: bool,
) -> OpenXmlElement {
    OpenXmlElement::new("p", P, "hf")
        .with_attribute("sldNum", if show_slide_number { "1" } else { "0" })
        .with_attribute("hdr", "0")
        .with_attribute("ftr", if show_footer { "1" } else { "0" })
        .with_attribute("dt", if show_date { "1" } else { "0" })
}

/// Timing container for a simple appear animation on a shape.
///
/// Minimal `p:timing` tree that targets shape id `shape_id`.
pub fn simple_appear_timing(shape_id: u32) -> OpenXmlElement {
    // Build a minimal timing/tnLst/par/cTn structure
    let sp_tgt = OpenXmlElement::new("p", P, "spTgt").with_attribute("spid", shape_id.to_string());
    let tgt_el = OpenXmlElement::new("p", P, "tgtEl").with_child(sp_tgt);
    let c_bhvr = OpenXmlElement::new("p", P, "cBhvr").with_child(
        OpenXmlElement::new("p", P, "cTn")
            .with_attribute("id", "2")
            .with_attribute("dur", "1")
            .with_attribute("fill", "hold")
            .with_child(tgt_el),
    );
    let anim = OpenXmlElement::new("p", P, "animEffect")
        .with_attribute("transition", "in")
        .with_attribute("filter", "fade")
        .with_child(c_bhvr);
    let child_tn = OpenXmlElement::new("p", P, "cTn")
        .with_attribute("id", "1")
        .with_attribute("dur", "indefinite")
        .with_attribute("restart", "never")
        .with_attribute("nodeType", "clickEffect")
        .with_child(
            OpenXmlElement::new("p", P, "childTnLst").with_child(anim),
        );
    let par = OpenXmlElement::new("p", P, "par").with_child(child_tn);
    OpenXmlElement::new("p", P, "timing").with_child(
        OpenXmlElement::new("p", P, "tnLst").with_child(par),
    )
}
