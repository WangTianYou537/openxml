//! WordprocessingML elements.
//!
//! Hand-written convenience helpers for the common create/read path.
//! Full schema coverage (700+ types) lives in
//! [`crate::generated::wordprocessingml_2006_main`] and is produced by
//! `openxml-codegen` from the C# SDK's `data/schemas/*.json`.

use crate::element::{OpenXmlAttribute, OpenXmlElement};
use crate::namespace::ns;

// Re-export the full generated element factory surface under this module for
// callers who want schema-complete constructors without going through `generated::`.
pub use crate::generated::wordprocessingml_2006_main::{
    create as create_by_class, info_by_class, info_by_local_name, ELEMENT_COUNT, ELEMENTS,
    NAMESPACE_PREFIX, NAMESPACE_URI, TYPE_COUNT,
};

const W: &str = ns::WORDPROCESSINGML.uri;

/// Create a `w:document` root element with the standard namespace declaration.
pub fn document(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "document")
        .with_ns_decl("w", W)
        .with_children(children)
}

/// Create a `w:body` element.
pub fn body(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "body").with_children(children)
}

/// Create a `w:p` (paragraph) element.
pub fn paragraph(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "p").with_children(children)
}

/// Create a `w:r` (run) element.
pub fn run(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "r").with_children(children)
}

/// Create a `w:t` (text) element. Sets `xml:space="preserve"` when the text
/// has leading/trailing whitespace.
pub fn text(value: impl Into<String>) -> OpenXmlElement {
    let value = value.into();
    let mut t = OpenXmlElement::new("w", W, "t").with_text(value.clone());
    if value.starts_with(char::is_whitespace) || value.ends_with(char::is_whitespace) {
        t.attributes.push(OpenXmlAttribute::with_ns(
            "xml",
            "http://www.w3.org/XML/1998/namespace",
            "space",
            "preserve",
        ));
    }
    t
}

/// Create a `w:rPr` (run properties) element.
pub fn run_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "rPr").with_children(children)
}

/// Create a `w:pPr` (paragraph properties) element.
pub fn paragraph_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "pPr").with_children(children)
}

/// Bold run property (`w:b`).
pub fn bold() -> OpenXmlElement {
    OpenXmlElement::new("w", W, "b")
}

/// Italic run property (`w:i`).
pub fn italic() -> OpenXmlElement {
    OpenXmlElement::new("w", W, "i")
}

/// Create a paragraph containing a single run of text.
pub fn paragraph_with_text(value: impl Into<String>) -> OpenXmlElement {
    paragraph(vec![run(vec![text(value)])])
}

/// Paragraph style reference (`w:pStyle` inside `w:pPr`).
pub fn paragraph_style_ref(style_id: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "pStyle").with_attribute_qname("w:val", style_id)
}

/// Create a paragraph with a paragraph-style reference and text content.
pub fn paragraph_with_style(style_id: &str, value: impl Into<String>) -> OpenXmlElement {
    paragraph(vec![
        paragraph_properties(vec![paragraph_style_ref(style_id)]),
        run(vec![text(value)]),
    ])
}

/// Apply (or replace) a `w:pStyle` on an existing paragraph element.
pub fn apply_paragraph_style(para: &mut OpenXmlElement, style_id: &str) {
    if let Some(ppr) = para.child_mut("pPr") {
        if let Some(ps) = ppr.child_mut("pStyle") {
            ps.set_attribute_qname("w:val", style_id);
        } else {
            ppr.children.insert(0, paragraph_style_ref(style_id));
        }
    } else {
        para.children
            .insert(0, paragraph_properties(vec![paragraph_style_ref(style_id)]));
    }
}

/// Create a bold paragraph.
pub fn paragraph_with_bold_text(value: impl Into<String>) -> OpenXmlElement {
    paragraph(vec![run(vec![
        run_properties(vec![bold()]),
        text(value),
    ])])
}

/// Build a run with optional formatting flags.
///
/// `color_hex` is 6-digit RGB without `#`; `size_half_points` is Word half-points
/// (24 = 12pt); `highlight` is a Word highlight color name (e.g. `"yellow"`).
pub fn run_with_formatting(
    value: impl Into<String>,
    bold_flag: bool,
    italic_flag: bool,
    color_hex: Option<&str>,
    size_half_points: Option<u32>,
    underline_val: Option<&str>,
    highlight: Option<&str>,
) -> OpenXmlElement {
    let mut props = Vec::new();
    if bold_flag {
        props.push(bold());
    }
    if italic_flag {
        props.push(italic());
    }
    if let Some(c) = color_hex {
        props.push(run_color(c));
    }
    if let Some(sz) = size_half_points {
        props.push(run_size(sz));
    }
    if let Some(u) = underline_val {
        props.push(underline(u));
    }
    if let Some(h) = highlight {
        props.push(run_highlight(h));
    }
    if props.is_empty() {
        run(vec![text(value)])
    } else {
        run(vec![run_properties(props), text(value)])
    }
}

/// Paragraph containing a single formatted run.
pub fn paragraph_with_formatted_text(
    value: impl Into<String>,
    bold_flag: bool,
    italic_flag: bool,
    color_hex: Option<&str>,
    size_half_points: Option<u32>,
) -> OpenXmlElement {
    paragraph(vec![run_with_formatting(
        value,
        bold_flag,
        italic_flag,
        color_hex,
        size_half_points,
        None,
        None,
    )])
}

/// Section properties with an empty sectPr (often required for Word round-trip).
pub fn section_properties() -> OpenXmlElement {
    OpenXmlElement::new("w", W, "sectPr")
}

/// Build `w:sectPr` with page size (twips) and margins.
///
/// US Letter defaults: `w=12240`, `h=15840`, margins 1440 (1").
pub fn section_properties_with_page(
    page_w: u32,
    page_h: u32,
    margin_top: u32,
    margin_right: u32,
    margin_bottom: u32,
    margin_left: u32,
) -> OpenXmlElement {
    OpenXmlElement::w("sectPr")
        .with_child(
            OpenXmlElement::w("pgSz")
                .with_attribute_qname("w:w", page_w.to_string())
                .with_attribute_qname("w:h", page_h.to_string()),
        )
        .with_child(
            OpenXmlElement::w("pgMar")
                .with_attribute_qname("w:top", margin_top.to_string())
                .with_attribute_qname("w:right", margin_right.to_string())
                .with_attribute_qname("w:bottom", margin_bottom.to_string())
                .with_attribute_qname("w:left", margin_left.to_string())
                .with_attribute_qname("w:header", "720")
                .with_attribute_qname("w:footer", "720")
                .with_attribute_qname("w:gutter", "0"),
        )
}

/// Page borders inside `w:sectPr` (`w:pgBorders` with top/left/bottom/right).
///
/// `color` is hex RGB; `sz` is border size in eighths of a point.
pub fn page_borders(color: &str, sz: u32) -> OpenXmlElement {
    fn border(name: &str, color: &str, sz: u32) -> OpenXmlElement {
        OpenXmlElement::w(name)
            .with_attribute_qname("w:val", "single")
            .with_attribute_qname("w:sz", sz.to_string())
            .with_attribute_qname("w:space", "24")
            .with_attribute_qname("w:color", color)
    }
    OpenXmlElement::w("pgBorders")
        .with_child(border("top", color, sz))
        .with_child(border("left", color, sz))
        .with_child(border("bottom", color, sz))
        .with_child(border("right", color, sz))
}

/// Section properties with page size, margins, and page borders.
pub fn section_properties_with_borders(
    page_w: u32,
    page_h: u32,
    margins: (u32, u32, u32, u32),
    border_color: &str,
    border_sz: u32,
) -> OpenXmlElement {
    let mut sect = section_properties_with_page(
        page_w, page_h, margins.0, margins.1, margins.2, margins.3,
    );
    sect.append_child(page_borders(border_color, border_sz));
    sect
}

/// Simple field (`w:fldSimple`) with instruction and result text.
pub fn simple_field(instruction: &str, result: impl Into<String>) -> OpenXmlElement {
    OpenXmlElement::w("fldSimple")
        .with_attribute_qname("w:instr", instruction)
        .with_child(run(vec![text(result)]))
}


/// Field character (`w:fldChar`) with type `begin` / `separate` / `end`.
pub fn field_char(kind: &str) -> OpenXmlElement {
    OpenXmlElement::w("fldChar").with_attribute_qname("w:fldCharType", kind)
}

/// Instruction text run for a complex field (`w:instrText`).
pub fn field_instruction(instruction: &str) -> OpenXmlElement {
    OpenXmlElement::w("r").with_child(
        OpenXmlElement::w("instrText")
            .with_attribute_qname("xml:space", "preserve")
            .with_text(instruction),
    )
}

/// Complex field as a sequence of runs: begin, instruction, separate, result, end.
///
/// Suitable as paragraph children. `result` is the cached display text.
pub fn complex_field(instruction: &str, result: &str) -> Vec<OpenXmlElement> {
    vec![
        OpenXmlElement::w("r").with_child(field_char("begin")),
        field_instruction(instruction),
        OpenXmlElement::w("r").with_child(field_char("separate")),
        run(vec![text(result)]),
        OpenXmlElement::w("r").with_child(field_char("end")),
    ]
}

/// Complex field wrapped in a paragraph.
pub fn complex_field_paragraph(instruction: &str, result: &str) -> OpenXmlElement {
    paragraph(complex_field(instruction, result))
}

/// SEQ caption field (e.g. figure/table numbering).
///
/// `seq_name` is typically `"Figure"` or `"Table"`.
pub fn caption_field(seq_name: &str, label: &str, number: &str) -> OpenXmlElement {
    paragraph(vec![
        run(vec![text(format!("{label} "))]),
        simple_field(&format!(" SEQ {seq_name} \\* ARABIC "), number),
        run(vec![text(format!(": {label}"))]),
    ])
}

/// Phonetic guide / ruby text (`w:ruby`).
pub fn ruby(base_text: &str, ruby_text: &str) -> OpenXmlElement {
    OpenXmlElement::w("ruby")
        .with_child(
            OpenXmlElement::w("rubyPr")
                .with_child(OpenXmlElement::w("rubyAlign").with_attribute_qname("w:val", "center"))
                .with_child(OpenXmlElement::w("hps").with_attribute_qname("w:val", "12"))
                .with_child(OpenXmlElement::w("hpsRaise").with_attribute_qname("w:val", "12"))
                .with_child(OpenXmlElement::w("hpsBaseText").with_attribute_qname("w:val", "24"))
                .with_child(OpenXmlElement::w("lid").with_attribute_qname("w:val", "ja-JP")),
        )
        .with_child(
            OpenXmlElement::w("rt").with_child(run(vec![text(ruby_text)])),
        )
        .with_child(
            OpenXmlElement::w("rubyBase").with_child(run(vec![text(base_text)])),
        )
}

const M: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";

/// OMML math run (`m:r` with `m:t`).
pub fn omml_run(text_value: &str) -> OpenXmlElement {
    OpenXmlElement::new("m", M, "r")
        .with_child(OpenXmlElement::new("m", M, "t").with_text(text_value))
}

/// OMML fraction (`m:f`).
pub fn omml_fraction(numerator: &str, denominator: &str) -> OpenXmlElement {
    OpenXmlElement::new("m", M, "f")
        .with_child(
            OpenXmlElement::new("m", M, "num").with_child(omml_run(numerator)),
        )
        .with_child(
            OpenXmlElement::new("m", M, "den").with_child(omml_run(denominator)),
        )
}

/// Inline Office Math (`m:oMath`) containing a single fraction or runs.
pub fn omml_math(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("m", M, "oMath")
        .with_ns_decl("m", M)
        .with_children(children)
}

/// Paragraph containing an OMML math expression.
pub fn math_paragraph(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    paragraph(vec![omml_math(children)])
}

/// PAGE field result (current page number placeholder).
pub fn page_number_field() -> OpenXmlElement {
    simple_field(" PAGE ", "1")
}

/// NUMPAGES field result (total pages placeholder).
pub fn num_pages_field() -> OpenXmlElement {
    simple_field(" NUMPAGES ", "1")
}

/// Table of contents field (TOC).
///
/// `switches` defaults to a common TOC instruction when empty, e.g. `TOC \\o "1-3" \\h \\z \\u`.
pub fn toc_field(switches: &str) -> OpenXmlElement {
    let instr = if switches.is_empty() {
        " TOC \\o \"1-3\" \\h \\z \\u ".to_string()
    } else {
        format!(" {switches} ")
    };
    simple_field(&instr, "Table of Contents")
}

/// Document background color (`w:background` under `w:document`).
///
/// `color` is a 6-digit hex RGB string without `#` (e.g. `"FFFF00"`).
pub fn document_background(color: &str) -> OpenXmlElement {
    OpenXmlElement::w("background").with_attribute_qname("w:color", color)
}

/// Text watermark via VML shape in a header (simplified).
///
/// Returns a `w:hdr` root containing a watermark shape with the given text.
pub fn watermark_header(text_value: &str) -> OpenXmlElement {
    let v = "urn:schemas-microsoft-com:vml";
    let o = "urn:schemas-microsoft-com:office:office";
    let w10 = "urn:schemas-microsoft-com:office:word";
    let shape = OpenXmlElement::new("v", v, "shape")
        .with_attribute("id", "PowerPlusWaterMarkObject")
        .with_attribute(
            "style",
            "position:absolute;margin-left:0;margin-top:0;width:527.85pt;height:131.95pt;rotation:315;z-index:-251657216;mso-position-horizontal:center;mso-position-horizontal-relative:margin;mso-position-vertical:center;mso-position-vertical-relative:margin",
        )
        .with_attribute("fillcolor", "silver")
        .with_attribute("stroked", "f")
        .with_child(
            OpenXmlElement::new("v", v, "fill")
                .with_attribute("opacity", ".5"),
        )
        .with_child(
            OpenXmlElement::new("v", v, "textpath")
                .with_attribute("style", "font-family:\"Calibri\";font-size:1pt")
                .with_attribute("string", text_value),
        )
        .with_child(
            OpenXmlElement::new("w10", w10, "wrap").with_attribute("type", "none"),
        );

    let pict = OpenXmlElement::w("pict")
        .with_ns_decl("v", v)
        .with_ns_decl("o", o)
        .with_ns_decl("w10", w10)
        .with_child(
            OpenXmlElement::new("v", v, "shapetype")
                .with_attribute("id", "_x0000_t136")
                .with_attribute("coordsize", "21600,21600")
                .with_attribute_qname("o:spt", "136")
                .with_attribute("path", "m@7,l@8,m@5,21600l@6,21600e")
                .with_child(OpenXmlElement::new("v", v, "path")
                    .with_attribute("textpathok", "t")),
        )
        .with_child(shape);

    OpenXmlElement::w("hdr")
        .with_ns_decl("w", crate::namespace::ns::WORDPROCESSINGML.uri)
        .with_child(paragraph(vec![run(vec![pict])]))
}

/// Drop-cap paragraph properties (`w:dropCap` inside `w:framePr` / `w:pPr`).
///
/// `drop_cap` is typically `"drop"` or `"margin"`; `lines` is the number of lines to drop.
pub fn drop_cap_paragraph_properties(drop_cap: &str, lines: u32) -> OpenXmlElement {
    paragraph_properties(vec![OpenXmlElement::w("framePr")
        .with_attribute_qname("w:dropCap", drop_cap)
        .with_attribute_qname("w:lines", lines.to_string())])
}

/// Build a paragraph whose first letter is styled as a drop cap (framePr on pPr).
pub fn drop_cap_paragraph(text_value: impl Into<String>, lines: u32) -> OpenXmlElement {
    paragraph(vec![
        drop_cap_paragraph_properties("drop", lines),
        run(vec![text(text_value)]),
    ])
}

/// Document variable entry (`w:docVar`) for settings.
pub fn document_variable(name: &str, value: &str) -> OpenXmlElement {
    OpenXmlElement::w("docVar")
        .with_attribute_qname("w:name", name)
        .with_attribute_qname("w:val", value)
}

/// `w:docVars` container.
pub fn document_variables(
    vars: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
) -> OpenXmlElement {
    let kids: Vec<_> = vars
        .into_iter()
        .map(|(n, v)| document_variable(n.as_ref(), v.as_ref()))
        .collect();
    OpenXmlElement::w("docVars").with_children(kids)
}

/// Build a complete minimal document with the given paragraphs and a trailing sectPr.
pub fn simple_document(paragraphs: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let mut kids: Vec<OpenXmlElement> = paragraphs.into_iter().collect();
    kids.push(section_properties());
    document(vec![body(kids)])
}

/// Replace all occurrences of `from` with `to` in every `w:t` text node under `root`.
///
/// Returns the number of individual string replacements performed (not nodes touched).
pub fn replace_text(root: &mut OpenXmlElement, from: &str, to: &str) -> usize {
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

/// Collect all `w:t` text values under `root` (in document order).
pub fn collect_texts(root: &OpenXmlElement) -> Vec<String> {
    let mut out = Vec::new();
    collect_texts_recursive(root, &mut out);
    out
}

fn collect_texts_recursive(elem: &OpenXmlElement, out: &mut Vec<String>) {
    if elem.local_name == "t" {
        if let Some(t) = &elem.text {
            out.push(t.clone());
        }
    }
    for child in &elem.children {
        collect_texts_recursive(child, out);
    }
}

/// `w:commentRangeStart` marker.
pub fn comment_range_start(id: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "commentRangeStart").with_attribute_qname("w:id", id)
}

/// `w:commentRangeEnd` marker.
pub fn comment_range_end(id: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "commentRangeEnd").with_attribute_qname("w:id", id)
}

/// `w:commentReference` (typically inside a run after the range end).
pub fn comment_reference(id: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "commentReference").with_attribute_qname("w:id", id)
}

/// A single `w:comment` entry for the comments part.
pub fn comment(id: &str, author: &str, initials: &str, body_text: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "comment")
        .with_attribute_qname("w:id", id)
        .with_attribute_qname("w:author", author)
        .with_attribute_qname("w:initials", initials)
        .with_child(paragraph(vec![run(vec![text(body_text)])]))
}

/// `w:comments` root element.
pub fn comments(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "comments")
        .with_ns_decl("w", W)
        .with_children(children)
}

/// Minimal numbering definitions with one abstract bullet list (abstractNumId=0)
/// and one numbering instance (numId=1).
pub fn default_numbering() -> OpenXmlElement {
    let lvl = |ilvl: u32| {
        OpenXmlElement::new("w", W, "lvl")
            .with_attribute_qname("w:ilvl", ilvl.to_string())
            .with_child(
                OpenXmlElement::new("w", W, "start").with_attribute_qname("w:val", "1"),
            )
            .with_child(
                OpenXmlElement::new("w", W, "numFmt").with_attribute_qname("w:val", "bullet"),
            )
            .with_child(
                OpenXmlElement::new("w", W, "lvlText").with_attribute_qname("w:val", "·"),
            )
            .with_child(
                OpenXmlElement::new("w", W, "lvlJc").with_attribute_qname("w:val", "left"),
            )
    };
    OpenXmlElement::new("w", W, "numbering")
        .with_ns_decl("w", W)
        .with_child(
            OpenXmlElement::new("w", W, "abstractNum")
                .with_attribute_qname("w:abstractNumId", "0")
                .with_child(lvl(0)),
        )
        .with_child(
            OpenXmlElement::new("w", W, "num")
                .with_attribute_qname("w:numId", "1")
                .with_child(
                    OpenXmlElement::new("w", W, "abstractNumId")
                        .with_attribute_qname("w:val", "0"),
                ),
        )
}

/// Paragraph properties that attach numbering (`numId` / `ilvl`).
pub fn numbered_paragraph_properties(num_id: u32, ilvl: u32) -> OpenXmlElement {
    paragraph_properties(vec![OpenXmlElement::new("w", W, "numPr")
        .with_child(
            OpenXmlElement::new("w", W, "ilvl").with_attribute_qname("w:val", ilvl.to_string()),
        )
        .with_child(
            OpenXmlElement::new("w", W, "numId").with_attribute_qname("w:val", num_id.to_string()),
        )])
}

/// A numbered/bulleted paragraph using the given numId (default list uses 1).
pub fn numbered_paragraph(num_id: u32, text_value: impl Into<String>) -> OpenXmlElement {
    paragraph(vec![
        numbered_paragraph_properties(num_id, 0),
        run(vec![text(text_value)]),
    ])
}

// ---------------------------------------------------------------------------
// Tables
// ---------------------------------------------------------------------------

/// `w:tbl` table element.
pub fn table(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "tbl").with_children(children)
}

/// `w:tblPr` table properties.
pub fn table_properties(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "tblPr").with_children(children)
}

/// `w:tblStyle` with a style id.
pub fn table_style(style_id: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "tblStyle").with_attribute_qname("w:val", style_id)
}

/// `w:tblW` width (`type` is usually `"auto"` or `"dxa"`).
pub fn table_width(width_type: &str, width: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "tblW")
        .with_attribute_qname("w:type", width_type)
        .with_attribute_qname("w:w", width)
}

/// `w:tblGrid` with column widths (dxa units).
pub fn table_grid(column_widths_dxa: &[u32]) -> OpenXmlElement {
    let cols: Vec<_> = column_widths_dxa
        .iter()
        .map(|w| {
            OpenXmlElement::new("w", W, "gridCol").with_attribute_qname("w:w", w.to_string())
        })
        .collect();
    OpenXmlElement::new("w", W, "tblGrid").with_children(cols)
}

/// `w:tr` table row.
pub fn table_row(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "tr").with_children(children)
}

/// `w:tc` table cell containing the given block-level children.
pub fn table_cell(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "tc").with_children(children)
}

/// Table cell containing a single paragraph of text.
pub fn table_cell_with_text(value: impl Into<String>) -> OpenXmlElement {
    table_cell(vec![paragraph_with_text(value)])
}

/// Build a simple table from a 2D grid of strings.
///
/// The first row is treated like any other row (no special header styling).
/// Column widths default to equal shares of 5000 dxa if `column_widths` is `None`.
pub fn table_from_strings(
    rows: &[Vec<&str>],
    column_widths: Option<&[u32]>,
) -> OpenXmlElement {
    let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let widths: Vec<u32> = if let Some(w) = column_widths {
        w.to_vec()
    } else if col_count == 0 {
        Vec::new()
    } else {
        let each = 5000u32 / col_count as u32;
        vec![each; col_count]
    };

    let mut kids = vec![
        table_properties(vec![table_width("auto", "0")]),
        table_grid(&widths),
    ];
    for row in rows {
        let cells: Vec<_> = row.iter().map(|c| table_cell_with_text(*c)).collect();
        kids.push(table_row(cells));
    }
    table(kids)
}

/// Extract a 2D grid of cell texts from a `w:tbl` element.
pub fn table_to_strings(tbl: &OpenXmlElement) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for tr in tbl.children_by_name("tr") {
        let mut row = Vec::new();
        for tc in tr.children_by_name("tc") {
            row.push(tc.inner_text());
        }
        out.push(row);
    }
    out
}

/// Minimal Office theme (DrawingML) sufficient for Word round-trip.
pub fn default_theme(name: &str) -> OpenXmlElement {
    let a = "http://schemas.openxmlformats.org/drawingml/2006/main";
    let scheme_clr = |val: &str| {
        OpenXmlElement::new("a", a, "srgbClr").with_attribute("val", val)
    };
    let color = |name: &str, hex: &str| {
        OpenXmlElement::new("a", a, name).with_child(scheme_clr(hex))
    };
    OpenXmlElement::new("a", a, "theme")
        .with_ns_decl("a", a)
        .with_attribute("name", name)
        .with_child(
            OpenXmlElement::new("a", a, "themeElements")
                .with_child(
                    OpenXmlElement::new("a", a, "clrScheme")
                        .with_attribute("name", "Office")
                        .with_child(color("dk1", "000000"))
                        .with_child(color("lt1", "FFFFFF"))
                        .with_child(color("dk2", "44546A"))
                        .with_child(color("lt2", "E7E6E6"))
                        .with_child(color("accent1", "4472C4"))
                        .with_child(color("accent2", "ED7D31"))
                        .with_child(color("accent3", "A5A5A5"))
                        .with_child(color("accent4", "FFC000"))
                        .with_child(color("accent5", "5B9BD5"))
                        .with_child(color("accent6", "70AD47"))
                        .with_child(color("hlink", "0563C1"))
                        .with_child(color("folHlink", "954F72")),
                )
                .with_child(
                    OpenXmlElement::new("a", a, "fontScheme")
                        .with_attribute("name", "Office")
                        .with_child(
                            OpenXmlElement::new("a", a, "majorFont").with_child(
                                OpenXmlElement::new("a", a, "latin")
                                    .with_attribute("typeface", "Calibri Light"),
                            ),
                        )
                        .with_child(
                            OpenXmlElement::new("a", a, "minorFont").with_child(
                                OpenXmlElement::new("a", a, "latin")
                                    .with_attribute("typeface", "Calibri"),
                            ),
                        ),
                )
                .with_child(
                    {
                        // Minimal Office-compatible format scheme (3 fills, 3 lines, 3 effects, 3 bg fills)
                        let solid_ph = || {
                            OpenXmlElement::new("a", a, "solidFill").with_child(
                                OpenXmlElement::new("a", a, "schemeClr").with_attribute("val", "phClr"),
                            )
                        };
                        let ln = |w: &str| {
                            OpenXmlElement::new("a", a, "ln")
                                .with_attribute("w", w)
                                .with_attribute("cap", "flat")
                                .with_attribute("cmpd", "sng")
                                .with_attribute("algn", "ctr")
                                .with_child(solid_ph())
                                .with_child(
                                    OpenXmlElement::new("a", a, "prstDash").with_attribute("val", "solid"),
                                )
                        };
                        OpenXmlElement::new("a", a, "fmtScheme")
                            .with_attribute("name", "Office")
                            .with_child(
                                OpenXmlElement::new("a", a, "fillStyleLst")
                                    .with_child(solid_ph())
                                    .with_child(solid_ph())
                                    .with_child(solid_ph()),
                            )
                            .with_child(
                                OpenXmlElement::new("a", a, "lnStyleLst")
                                    .with_child(ln("6350"))
                                    .with_child(ln("12700"))
                                    .with_child(ln("19050")),
                            )
                            .with_child(
                                OpenXmlElement::new("a", a, "effectStyleLst")
                                    .with_child(
                                        OpenXmlElement::new("a", a, "effectStyle")
                                            .with_child(OpenXmlElement::new("a", a, "effectLst")),
                                    )
                                    .with_child(
                                        OpenXmlElement::new("a", a, "effectStyle")
                                            .with_child(OpenXmlElement::new("a", a, "effectLst")),
                                    )
                                    .with_child(
                                        OpenXmlElement::new("a", a, "effectStyle")
                                            .with_child(OpenXmlElement::new("a", a, "effectLst")),
                                    ),
                            )
                            .with_child(
                                OpenXmlElement::new("a", a, "bgFillStyleLst")
                                    .with_child(solid_ph())
                                    .with_child(solid_ph())
                                    .with_child(solid_ph()),
                            )
                    },
                ),
        )
}

/// Wrap a run of content with comment range markers and a reference.
///
/// Returns `(range_start, content..., range_end, reference_run)` as a flat list
/// suitable for inserting into a paragraph.
pub fn with_comment(
    id: &str,
    content: impl IntoIterator<Item = OpenXmlElement>,
) -> Vec<OpenXmlElement> {
    let mut out = vec![comment_range_start(id)];
    out.extend(content);
    out.push(comment_range_end(id));
    out.push(run(vec![comment_reference(id)]));
    out
}

/// `w:altChunk` element referencing an AlternativeFormatImportPart by relationship id.
pub fn alt_chunk(relationship_id: &str) -> OpenXmlElement {
    let r = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    OpenXmlElement::new("w", W, "altChunk").with_attribute_ns("r", r, "id", relationship_id)
}

// ---------------------------------------------------------------------------
// Footnotes / endnotes
// ---------------------------------------------------------------------------

/// `w:footnotes` root.
pub fn footnotes(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "footnotes")
        .with_ns_decl("w", W)
        .with_children(children)
}

/// `w:endnotes` root.
pub fn endnotes(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "endnotes")
        .with_ns_decl("w", W)
        .with_children(children)
}

/// A single `w:footnote` entry.
///
/// `note_type` is typically omitted for normal notes; use `"separator"` /
/// `"continuationSeparator"` for the special system notes Word expects.
pub fn footnote(id: &str, note_type: Option<&str>, body_text: &str) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("w", W, "footnote").with_attribute_qname("w:id", id);
    if let Some(t) = note_type {
        el.set_attribute_qname("w:type", t);
    }
    el.append_child(paragraph(vec![run(vec![text(body_text)])]));
    el
}

/// A single `w:endnote` entry.
pub fn endnote(id: &str, note_type: Option<&str>, body_text: &str) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("w", W, "endnote").with_attribute_qname("w:id", id);
    if let Some(t) = note_type {
        el.set_attribute_qname("w:type", t);
    }
    el.append_child(paragraph(vec![run(vec![text(body_text)])]));
    el
}

/// `w:footnoteReference` (placed inside a run).
pub fn footnote_reference(id: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "footnoteReference").with_attribute_qname("w:id", id)
}

/// `w:endnoteReference` (placed inside a run).
pub fn endnote_reference(id: &str) -> OpenXmlElement {
    OpenXmlElement::new("w", W, "endnoteReference").with_attribute_qname("w:id", id)
}

/// Build a minimal footnotes part body with separator + continuation + one note.
pub fn default_footnotes_with(id: &str, body_text: &str) -> OpenXmlElement {
    footnotes(vec![
        footnote("-1", Some("separator"), ""),
        footnote("0", Some("continuationSeparator"), ""),
        footnote(id, None, body_text),
    ])
}

/// Build a minimal endnotes part body with separator + continuation + one note.
pub fn default_endnotes_with(id: &str, body_text: &str) -> OpenXmlElement {
    endnotes(vec![
        endnote("-1", Some("separator"), ""),
        endnote("0", Some("continuationSeparator"), ""),
        endnote(id, None, body_text),
    ])
}

/// Run containing a footnote reference.
pub fn footnote_ref_run(id: &str) -> OpenXmlElement {
    run(vec![footnote_reference(id)])
}

/// Run containing an endnote reference.
pub fn endnote_ref_run(id: &str) -> OpenXmlElement {
    run(vec![endnote_reference(id)])
}

/// Paragraph justification (`w:jc`).
///
/// `val` is typically `"left"`, `"center"`, `"right"`, `"both"`, `"distribute"`.
pub fn justification(val: &str) -> OpenXmlElement {
    OpenXmlElement::w("jc").with_attribute_qname("w:val", val)
}

/// Paragraph border element (`w:top`/`w:left`/… inside `w:pBdr`).
pub fn paragraph_border_side(name: &str, color: &str, sz: u32) -> OpenXmlElement {
    OpenXmlElement::w(name)
        .with_attribute_qname("w:val", "single")
        .with_attribute_qname("w:sz", sz.to_string())
        .with_attribute_qname("w:space", "4")
        .with_attribute_qname("w:color", color)
}

/// Full paragraph borders (`w:pBdr`).
pub fn paragraph_borders(color: &str, sz: u32) -> OpenXmlElement {
    OpenXmlElement::w("pBdr")
        .with_child(paragraph_border_side("top", color, sz))
        .with_child(paragraph_border_side("left", color, sz))
        .with_child(paragraph_border_side("bottom", color, sz))
        .with_child(paragraph_border_side("right", color, sz))
}

/// Bidirectional paragraph mark (`w:bidi`).
pub fn bidi() -> OpenXmlElement {
    OpenXmlElement::w("bidi")
}

/// Right-to-left run (`w:rtl` in rPr).
pub fn run_rtl() -> OpenXmlElement {
    OpenXmlElement::w("rtl")
}

/// Small caps (`w:smallCaps`).
pub fn small_caps() -> OpenXmlElement {
    OpenXmlElement::w("smallCaps")
}

/// All caps (`w:caps`).
pub fn caps() -> OpenXmlElement {
    OpenXmlElement::w("caps")
}

/// Strike (`w:strike`).
pub fn strike() -> OpenXmlElement {
    OpenXmlElement::w("strike")
}

/// Double strike (`w:dstrike`).
pub fn double_strike() -> OpenXmlElement {
    OpenXmlElement::w("dstrike")
}

/// Underline (`w:u`), val e.g. `"single"`, `"double"`, `"dash"`.
pub fn underline(val: &str) -> OpenXmlElement {
    OpenXmlElement::w("u").with_attribute_qname("w:val", val)
}

/// Font color (`w:color`).
pub fn run_color(hex: &str) -> OpenXmlElement {
    OpenXmlElement::w("color").with_attribute_qname("w:val", hex)
}

/// Font size in half-points (`w:sz`).
pub fn run_size(half_points: u32) -> OpenXmlElement {
    OpenXmlElement::w("sz").with_attribute_qname("w:val", half_points.to_string())
}

/// Character spacing on a run (`w:spacing` in `w:rPr`, value in twips).
pub fn run_spacing(twips: i32) -> OpenXmlElement {
    OpenXmlElement::w("spacing").with_attribute_qname("w:val", twips.to_string())
}

/// Character scale percentage (`w:w` in rPr, 1–600).
pub fn run_scale(percent: u32) -> OpenXmlElement {
    OpenXmlElement::w("w").with_attribute_qname("w:val", percent.to_string())
}

/// Run language (`w:lang`).
pub fn run_language(val: &str) -> OpenXmlElement {
    OpenXmlElement::w("lang").with_attribute_qname("w:val", val)
}

/// Position (raise/lower) in half-points (`w:position`).
pub fn run_position(half_points: i32) -> OpenXmlElement {
    OpenXmlElement::w("position").with_attribute_qname("w:val", half_points.to_string())
}

/// DATE field.
pub fn date_field() -> OpenXmlElement {
    simple_field(" DATE \\@ \"yyyy-MM-dd\" ", "2020-01-01")
}

/// TIME field.
pub fn time_field() -> OpenXmlElement {
    simple_field(" TIME \\@ \"HH:mm\" ", "12:00")
}

/// AUTHOR field.
pub fn author_field() -> OpenXmlElement {
    simple_field(" AUTHOR ", "Author")
}

/// FILENAME field.
pub fn filename_field() -> OpenXmlElement {
    simple_field(" FILENAME ", "document.docx")
}

/// Vertical alignment for table cell (`w:vAlign`).
pub fn cell_vertical_align(val: &str) -> OpenXmlElement {
    OpenXmlElement::w("vAlign").with_attribute_qname("w:val", val)
}

/// Table cell properties with vertical alignment.
pub fn table_cell_properties_valign(val: &str) -> OpenXmlElement {
    OpenXmlElement::w("tcPr").with_child(cell_vertical_align(val))
}

/// Page break (`w:br w:type="page"`).
pub fn page_break() -> OpenXmlElement {
    OpenXmlElement::w("br").with_attribute_qname("w:type", "page")
}

/// Column break (`w:br w:type="column"`).
pub fn column_break() -> OpenXmlElement {
    OpenXmlElement::w("br").with_attribute_qname("w:type", "column")
}

/// Text wrapping break (`w:br` without type, or `w:type="textWrapping"`).
pub fn text_wrapping_break() -> OpenXmlElement {
    OpenXmlElement::w("br").with_attribute_qname("w:type", "textWrapping")
}

/// Run containing a page break.
pub fn page_break_run() -> OpenXmlElement {
    run(vec![page_break()])
}

/// Paragraph indent (`w:ind` inside `w:pPr`). Values in twips.
pub fn paragraph_indent(
    left: Option<u32>,
    right: Option<u32>,
    first_line: Option<u32>,
    hanging: Option<u32>,
) -> OpenXmlElement {
    let mut el = OpenXmlElement::w("ind");
    if let Some(v) = left {
        el.set_attribute_qname("w:left", v.to_string());
    }
    if let Some(v) = right {
        el.set_attribute_qname("w:right", v.to_string());
    }
    if let Some(v) = first_line {
        el.set_attribute_qname("w:firstLine", v.to_string());
    }
    if let Some(v) = hanging {
        el.set_attribute_qname("w:hanging", v.to_string());
    }
    el
}

/// Vertical text alignment in a paragraph (`w:textAlignment`).
pub fn text_alignment(val: &str) -> OpenXmlElement {
    OpenXmlElement::w("textAlignment").with_attribute_qname("w:val", val)
}

/// Keep paragraph with next (`w:keepNext`).
pub fn keep_next() -> OpenXmlElement {
    OpenXmlElement::w("keepNext")
}

/// Keep lines together (`w:keepLines`).
pub fn keep_lines() -> OpenXmlElement {
    OpenXmlElement::w("keepLines")
}

/// Page break before paragraph (`w:pageBreakBefore`).
pub fn page_break_before() -> OpenXmlElement {
    OpenXmlElement::w("pageBreakBefore")
}

/// Paragraph spacing (`w:spacing` inside `w:pPr`).
///
/// Values are in twips for before/after; `line` is in twips (240 = single).
pub fn paragraph_spacing(
    before: Option<u32>,
    after: Option<u32>,
    line: Option<u32>,
) -> OpenXmlElement {
    let mut el = OpenXmlElement::w("spacing");
    if let Some(b) = before {
        el.set_attribute_qname("w:before", b.to_string());
    }
    if let Some(a) = after {
        el.set_attribute_qname("w:after", a.to_string());
    }
    if let Some(l) = line {
        el.set_attribute_qname("w:line", l.to_string());
        el.set_attribute_qname("w:lineRule", "auto");
    }
    el
}

/// Paragraph shading (`w:shd`).
pub fn paragraph_shading(fill_color: &str) -> OpenXmlElement {
    OpenXmlElement::w("shd")
        .with_attribute_qname("w:val", "clear")
        .with_attribute_qname("w:color", "auto")
        .with_attribute_qname("w:fill", fill_color)
}

/// Run shading / highlight (`w:highlight` or `w:shd` on rPr).
pub fn run_highlight(color: &str) -> OpenXmlElement {
    OpenXmlElement::w("highlight").with_attribute_qname("w:val", color)
}

/// Paragraph with spacing and optional shading.
pub fn paragraph_with_spacing(
    before: Option<u32>,
    after: Option<u32>,
    line: Option<u32>,
    fill: Option<&str>,
    text_value: impl Into<String>,
) -> OpenXmlElement {
    let mut ppr_kids = vec![paragraph_spacing(before, after, line)];
    if let Some(f) = fill {
        ppr_kids.push(paragraph_shading(f));
    }
    paragraph(vec![
        paragraph_properties(ppr_kids),
        run(vec![text(text_value)]),
    ])
}

/// A tab stop (`w:tab`) for paragraph properties.
///
/// `val` is typically `"left"`, `"center"`, `"right"`, `"decimal"`.
/// `pos` is in twips.
pub fn tab_stop(val: &str, pos: u32) -> OpenXmlElement {
    OpenXmlElement::w("tab")
        .with_attribute_qname("w:val", val)
        .with_attribute_qname("w:pos", pos.to_string())
}

/// `w:tabs` container for paragraph properties.
pub fn tabs(stops: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::w("tabs").with_children(stops)
}

/// Paragraph with tab stops applied.
pub fn paragraph_with_tabs(
    stops: &[(/* val */ &str, /* pos twips */ u32)],
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    let tab_els: Vec<_> = stops.iter().map(|(v, p)| tab_stop(v, *p)).collect();
    let mut kids = vec![paragraph_properties(vec![tabs(tab_els)])];
    kids.extend(children);
    paragraph(kids)
}

/// Symbol character (`w:sym`) in a run.
pub fn symbol(font: &str, hex_char: &str) -> OpenXmlElement {
    OpenXmlElement::w("sym")
        .with_attribute_qname("w:font", font)
        .with_attribute_qname("w:char", hex_char)
}

/// Table with a table style applied (`w:tblStyle`).
pub fn table_with_style(
    style_id: &str,
    rows: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    let mut tbl = table(vec![table_properties(vec![table_style(style_id)])]);
    for row in rows {
        tbl.append_child(row);
    }
    tbl
}

/// Paragraph style definition fragment for `w:styles`.
pub fn paragraph_style(
    style_id: &str,
    name: &str,
    based_on: Option<&str>,
    is_default: bool,
) -> OpenXmlElement {
    let mut el = OpenXmlElement::w("style")
        .with_attribute_qname("w:type", "paragraph")
        .with_attribute_qname("w:styleId", style_id);
    if is_default {
        el.set_attribute_qname("w:default", "1");
    }
    el.append_child(OpenXmlElement::w("name").with_attribute_qname("w:val", name));
    if let Some(base) = based_on {
        el.append_child(OpenXmlElement::w("basedOn").with_attribute_qname("w:val", base));
    }
    el.append_child(OpenXmlElement::w("qFormat"));
    el
}

/// Document defaults for styles (`w:docDefaults` with run/paragraph defaults).
pub fn doc_defaults(ascii_font: &str, font_size_half_points: u32) -> OpenXmlElement {
    OpenXmlElement::w("docDefaults")
        .with_child(
            OpenXmlElement::w("rPrDefault").with_child(
                OpenXmlElement::w("rPr")
                    .with_child(
                        OpenXmlElement::w("rFonts")
                            .with_attribute_qname("w:ascii", ascii_font)
                            .with_attribute_qname("w:hAnsi", ascii_font)
                            .with_attribute_qname("w:eastAsia", ascii_font)
                            .with_attribute_qname("w:cs", ascii_font),
                    )
                    .with_child(
                        OpenXmlElement::w("sz")
                            .with_attribute_qname("w:val", font_size_half_points.to_string()),
                    )
                    .with_child(
                        OpenXmlElement::w("szCs")
                            .with_attribute_qname("w:val", font_size_half_points.to_string()),
                    ),
            ),
        )
        .with_child(
            OpenXmlElement::w("pPrDefault").with_child(OpenXmlElement::w("pPr")),
        )
}

/// Minimal bibliography sources custom XML (ODF-ish Office bibliography schema).
pub fn bibliography_sources(
    sources: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
) -> OpenXmlElement {
    // Simplified: list of Source with Tag and Title
    let b =
        "http://schemas.openxmlformats.org/officeDocument/2006/bibliography";
    let mut root = OpenXmlElement::new("b", b, "Sources")
        .with_ns_decl("b", b)
        .with_attribute("SelectedStyle", "\\APASixthEditionOfficeOnline.xsl")
        .with_attribute("StyleName", "APA")
        .with_attribute("Version", "6");
    for (tag, title) in sources {
        root.append_child(
            OpenXmlElement::new("b", b, "Source")
                .with_child(
                    OpenXmlElement::new("b", b, "Tag").with_text(tag.as_ref()),
                )
                .with_child(
                    OpenXmlElement::new("b", b, "Title").with_text(title.as_ref()),
                )
                .with_child(
                    OpenXmlElement::new("b", b, "SourceType").with_text("Book"),
                ),
        );
    }
    root
}

/// Minimal `w:fonts` font table with Calibri + Times New Roman.
pub fn default_font_table() -> OpenXmlElement {
    fn font(name: &str, charset: &str, family: &str, pitch: &str) -> OpenXmlElement {
        OpenXmlElement::w("font")
            .with_attribute_qname("w:name", name)
            .with_child(OpenXmlElement::w("charset").with_attribute_qname("w:val", charset))
            .with_child(OpenXmlElement::w("family").with_attribute_qname("w:val", family))
            .with_child(OpenXmlElement::w("pitch").with_attribute_qname("w:val", pitch))
    }
    OpenXmlElement::w("fonts")
        .with_ns_decl("w", crate::namespace::ns::WORDPROCESSINGML.uri)
        .with_child(font("Calibri", "00", "swiss", "variable"))
        .with_child(font("Times New Roman", "00", "roman", "variable"))
}

/// Minimal `w:webSettings` part root.
pub fn default_web_settings() -> OpenXmlElement {
    OpenXmlElement::w("webSettings")
        .with_ns_decl("w", crate::namespace::ns::WORDPROCESSINGML.uri)
        .with_child(OpenXmlElement::w("optimizeForBrowser"))
        .with_child(OpenXmlElement::w("allowPNG"))
}

/// `w:documentProtection` element for settings (edit restriction, no password crypto).
///
/// `edit` is typically `"readOnly"`, `"forms"`, `"comments"`, or `"trackedChanges"`.
pub fn document_protection(edit: &str, enforcement: bool) -> OpenXmlElement {
    OpenXmlElement::w("documentProtection")
        .with_attribute_qname("w:edit", edit)
        .with_attribute_qname("w:enforcement", if enforcement { "1" } else { "0" })
}

/// Minimal glossary document root with one doc part entry.
pub fn glossary_document(
    doc_part_name: &str,
    body_children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    let mut doc_part_pr = OpenXmlElement::w("docPartPr");
    doc_part_pr.append_child(
        OpenXmlElement::w("name").with_attribute_qname("w:val", doc_part_name),
    );
    doc_part_pr.append_child(
        OpenXmlElement::w("category")
            .with_child(OpenXmlElement::w("name").with_attribute_qname("w:val", "General"))
            .with_child(OpenXmlElement::w("gallery").with_attribute_qname("w:val", "placeholder")),
    );
    doc_part_pr.append_child(OpenXmlElement::w("types").with_child(
        OpenXmlElement::w("type").with_attribute_qname("w:val", "bbPlcHdr"),
    ));

    let mut doc_part_body = OpenXmlElement::w("docPartBody");
    for child in body_children {
        doc_part_body.append_child(child);
    }

    let doc_part = OpenXmlElement::w("docPart")
        .with_child(doc_part_pr)
        .with_child(doc_part_body);

    OpenXmlElement::w("glossaryDocument")
        .with_ns_decl("w", crate::namespace::ns::WORDPROCESSINGML.uri)
        .with_child(OpenXmlElement::w("docParts").with_child(doc_part))
}

/// Block-level structured document tag (content control) with a tag/alias and body paragraphs.
///
/// Produces a `w:sdt` suitable for placement under `w:body`.
pub fn sdt_block(tag: &str, alias: &str, body_children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let mut sdt_pr = OpenXmlElement::w("sdtPr");
    sdt_pr.append_child(
        OpenXmlElement::w("alias").with_attribute_qname("w:val", alias),
    );
    sdt_pr.append_child(OpenXmlElement::w("tag").with_attribute_qname("w:val", tag));
    sdt_pr.append_child(OpenXmlElement::w("id").with_attribute_qname("w:val", "1"));

    let mut content = OpenXmlElement::w("sdtContent");
    for child in body_children {
        content.append_child(child);
    }

    OpenXmlElement::w("sdt")
        .with_child(sdt_pr)
        .with_child(content)
}

/// Inline structured document tag wrapping run-level children.
pub fn sdt_run(tag: &str, alias: &str, run_children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let mut sdt_pr = OpenXmlElement::w("sdtPr");
    sdt_pr.append_child(
        OpenXmlElement::w("alias").with_attribute_qname("w:val", alias),
    );
    sdt_pr.append_child(OpenXmlElement::w("tag").with_attribute_qname("w:val", tag));

    let mut content = OpenXmlElement::w("sdtContent");
    for child in run_children {
        content.append_child(child);
    }

    OpenXmlElement::w("sdt")
        .with_child(sdt_pr)
        .with_child(content)
}


/// Block-level SDT with an explicit kind marker under `sdtPr` (e.g. `"richText"`, `"date"`).
pub fn sdt_block_with_kind(
    tag: &str,
    alias: &str,
    kind: &str,
    body_children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    let mut sdt = sdt_block(tag, alias, body_children);
    if let Some(pr) = sdt.child_mut("sdtPr") {
        if !kind.is_empty() {
            pr.append_child(OpenXmlElement::w(kind));
        }
    }
    sdt
}

/// Inline SDT with an explicit kind marker under `sdtPr`.
pub fn sdt_run_with_kind(
    tag: &str,
    alias: &str,
    kind: &str,
    run_children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    let mut sdt = sdt_run(tag, alias, run_children);
    if let Some(pr) = sdt.child_mut("sdtPr") {
        if !kind.is_empty() {
            pr.append_child(OpenXmlElement::w(kind));
        }
    }
    sdt
}

/// `w:bookmarkStart` marker.
pub fn bookmark_start(id: &str, name: &str) -> OpenXmlElement {
    OpenXmlElement::w("bookmarkStart")
        .with_attribute_qname("w:id", id)
        .with_attribute_qname("w:name", name)
}

/// `w:bookmarkEnd` marker.
pub fn bookmark_end(id: &str) -> OpenXmlElement {
    OpenXmlElement::w("bookmarkEnd").with_attribute_qname("w:id", id)
}

/// Wrap `inner` paragraph children between bookmark start/end with the given name/id.
pub fn with_bookmark(
    id: &str,
    name: &str,
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> Vec<OpenXmlElement> {
    let mut out = vec![bookmark_start(id, name)];
    out.extend(children);
    out.push(bookmark_end(id));
    out
}

/// Collect `(id, name)` for every `w:bookmarkStart` descendant.
pub fn collect_bookmarks(root: &OpenXmlElement) -> Vec<(String, String)> {
    root.descendants()
        .filter(|e| e.local_name == "bookmarkStart")
        .filter_map(|e| {
            let id = e
                .get_attribute_qname("w:id")
                .or_else(|| e.get_attribute("id"))?
                .to_string();
            let name = e
                .get_attribute_qname("w:name")
                .or_else(|| e.get_attribute("name"))?
                .to_string();
            Some((id, name))
        })
        .collect()
}

/// Collect `(tag, alias, inner_text)` for every `w:sdt` descendant.
pub fn collect_sdt_tags(root: &OpenXmlElement) -> Vec<(String, String, String)> {
    collect_sdt_infos(root)
        .into_iter()
        .map(|(tag, alias, _kind, text)| (tag, alias, text))
        .collect()
}

/// Collect content controls as `(tag, alias, kind, text)`.
///
/// `kind` is the first specialized `sdtPr` child local name when present
/// (e.g. `richText`, `comboBox`, `date`, `docPartObj`), otherwise `"sdt"`.
pub fn collect_sdt_infos(root: &OpenXmlElement) -> Vec<(String, String, String, String)> {
    const KIND_MARKERS: &[&str] = &[
        "citation",
        "comboBox",
        "date",
        "docPartList",
        "docPartObj",
        "dropDownList",
        "equation",
        "group",
        "picture",
        "repeatingSection",
        "richText",
        "text",
        "checkbox",
        "entityPicker",
    ];
    root.descendants()
        .filter(|e| e.local_name == "sdt")
        .map(|sdt| {
            let pr = sdt.child("sdtPr");
            let tag = pr
                .and_then(|p| p.child("tag"))
                .and_then(|t| t.get_attribute_qname("w:val").or_else(|| t.get_attribute("val")))
                .unwrap_or("")
                .to_string();
            let alias = pr
                .and_then(|p| p.child("alias"))
                .and_then(|t| t.get_attribute_qname("w:val").or_else(|| t.get_attribute("val")))
                .unwrap_or("")
                .to_string();
            let kind = pr
                .map(|p| {
                    p.children
                        .iter()
                        .find(|c| KIND_MARKERS.iter().any(|k| c.local_name == *k))
                        .map(|c| c.local_name.clone())
                        .unwrap_or_else(|| "sdt".to_string())
                })
                .unwrap_or_else(|| "sdt".to_string());
            let text = sdt
                .child("sdtContent")
                .map(|c| c.inner_text())
                .unwrap_or_default();
            (tag, alias, kind, text)
        })
        .collect()
}

/// Tracked insertion (`w:ins`) wrapping run-level children.
pub fn inserted_run(
    id: &str,
    author: &str,
    date: &str,
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::w("ins")
        .with_attribute_qname("w:id", id)
        .with_attribute_qname("w:author", author)
        .with_attribute_qname("w:date", date)
        .with_children(children)
}

/// Tracked deletion (`w:del`) wrapping runs that use `w:delText`.
pub fn deleted_run(
    id: &str,
    author: &str,
    date: &str,
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::w("del")
        .with_attribute_qname("w:id", id)
        .with_attribute_qname("w:author", author)
        .with_attribute_qname("w:date", date)
        .with_children(children)
}

/// `w:delText` leaf (used inside deleted runs).
pub fn del_text(value: impl Into<String>) -> OpenXmlElement {
    let mut el = OpenXmlElement::w("delText").with_text(value);
    el.set_attribute_ns("xml", "http://www.w3.org/XML/1998/namespace", "space", "preserve");
    el
}

/// Convenience: inserted paragraph content as a single run of text.
pub fn inserted_text_run(id: &str, author: &str, date: &str, value: impl Into<String>) -> OpenXmlElement {
    inserted_run(id, author, date, vec![run(vec![text(value)])])
}

/// Convenience: deleted paragraph content as a single run of delText.
pub fn deleted_text_run(id: &str, author: &str, date: &str, value: impl Into<String>) -> OpenXmlElement {
    deleted_run(id, author, date, vec![run(vec![del_text(value)])])
}

/// Accept all tracked insertions/deletions under `root` (mutates in place).
///
/// - `w:ins` → unwrap children (keep inserted content)
/// - `w:del` → remove entirely
///
/// Returns the number of revision markers processed.
pub fn accept_revisions(root: &mut OpenXmlElement) -> usize {
    apply_revisions(root, true)
}

/// Reject all tracked insertions/deletions under `root` (mutates in place).
///
/// - `w:ins` → remove entirely
/// - `w:del` → unwrap children, converting `w:delText` → `w:t`
///
/// Returns the number of revision markers processed.
pub fn reject_revisions(root: &mut OpenXmlElement) -> usize {
    apply_revisions(root, false)
}

fn apply_revisions(root: &mut OpenXmlElement, accept: bool) -> usize {
    let mut count = 0;
    // Process children first (post-order) so nested revisions resolve
    let mut i = 0;
    while i < root.children.len() {
        count += apply_revisions(&mut root.children[i], accept);
        let name = root.children[i].local_name.clone();
        match (name.as_str(), accept) {
            ("ins", true) | ("del", false) => {
                // Unwrap: promote children into parent
                let mut child = root.children.remove(i);
                if !accept {
                    // rejecting deletion: convert delText → t
                    convert_del_text_to_text(&mut child);
                }
                let kids = std::mem::take(&mut child.children);
                for (offset, k) in kids.into_iter().enumerate() {
                    root.children.insert(i + offset, k);
                }
                count += 1;
                // do not advance i — re-process inserted kids
            }
            ("ins", false) | ("del", true) => {
                root.children.remove(i);
                count += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    count
}

fn convert_del_text_to_text(el: &mut OpenXmlElement) {
    if el.local_name == "delText" {
        el.local_name = "t".into();
    }
    for child in &mut el.children {
        convert_del_text_to_text(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{parse_element, write_element};

    #[test]
    fn build_and_parse() {
        let doc = document(vec![body(vec![paragraph(vec![run(vec![text("Hi")])])])]);
        let xml = write_element(&doc).unwrap();
        let parsed = parse_element(&xml).unwrap();
        assert_eq!(parsed.inner_text(), "Hi");
    }

    #[test]
    fn replace_text_works() {
        let mut doc = document(vec![body(vec![
            paragraph(vec![run(vec![text("Hello world")])]),
            paragraph(vec![run(vec![text("world peace")])]),
        ])]);
        let n = replace_text(&mut doc, "world", "Rust");
        assert_eq!(n, 2);
        assert_eq!(doc.inner_text(), "Hello RustRust peace");
    }
}
