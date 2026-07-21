//! SpreadsheetML helpers (minimal core for `.xlsx`).

mod chart;
mod comments;
mod conditional_formatting;
mod drawing;
mod pivot;
mod sparkline;
mod vml;

pub use chart::{
    area_chart_space, bar_chart_space, bubble_chart_space, doughnut_chart_space, line_chart_space,
    pie_chart_space, radar_chart_space, scatter_chart_space,
};
pub use comments::{comment, comments_for_author, comments_root};
pub use sparkline::{sparkline, sparkline_ext, sparkline_group, sparkline_groups};
pub use vml::{cell_ref_to_row_col, vml_comments_drawing};
pub use conditional_formatting::{
    cf_rule_cell_is, cf_rule_color_scale, cf_rule_data_bar, cf_rule_expression, cf_rule_icon_set,
    conditional_formatting, dxf_fill, dxfs,
};
pub use drawing::{
    absolute_anchor_picture, chart_graphic_frame, one_cell_anchor_picture, picture,
    two_cell_anchor_chart, two_cell_anchor_picture, worksheet_drawing, worksheet_drawing_ref,
};
pub use pivot::{
    pivot_cache_definition, pivot_cache_records, pivot_cache_records_from_rows,
    pivot_table_definition, workbook_pivot_cache, workbook_pivot_caches,
};

use crate::element::OpenXmlElement;
use crate::namespace::ns;

const X: &str = ns::SPREADSHEETML.uri;

/// `x:workbook` root.
pub fn workbook(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "workbook")
        .with_ns_decl("x", X)
        .with_ns_decl(
            "r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        )
        .with_children(children)
}

/// `x:definedName` with a formula/reference body.
pub fn defined_name(name: &str, refers_to: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "definedName")
        .with_attribute("name", name)
        .with_text(refers_to)
}

/// `x:definedNames` container.
pub fn defined_names(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "definedNames").with_children(children)
}

/// Worksheet-level auto filter on a range (e.g. `"A1:C10"`).
pub fn auto_filter(reference: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "autoFilter").with_attribute("ref", reference)
}

/// Minimal Excel table definition (`x:table`) for a contiguous range with header row.
///
/// `columns` are header display names; `id` is the table id (unique in the workbook).
pub fn table_definition(
    id: u32,
    name: &str,
    display_name: &str,
    reference: &str,
    columns: &[&str],
) -> OpenXmlElement {
    let mut cols = Vec::new();
    for (i, col_name) in columns.iter().enumerate() {
        cols.push(
            OpenXmlElement::new("x", X, "tableColumn")
                .with_attribute("id", (i + 1).to_string())
                .with_attribute("name", *col_name),
        );
    }
    OpenXmlElement::new("x", X, "table")
        .with_ns_decl("x", X)
        .with_attribute("id", id.to_string())
        .with_attribute("name", name)
        .with_attribute("displayName", display_name)
        .with_attribute("ref", reference)
        .with_attribute("headerRowCount", "1")
        .with_child(auto_filter(reference))
        .with_child(
            OpenXmlElement::new("x", X, "tableColumns")
                .with_attribute("count", columns.len().to_string())
                .with_children(cols),
        )
        .with_child(
            OpenXmlElement::new("x", X, "tableStyleInfo")
                .with_attribute("name", "TableStyleMedium2")
                .with_attribute("showFirstColumn", "0")
                .with_attribute("showLastColumn", "0")
                .with_attribute("showRowStripes", "1")
                .with_attribute("showColumnStripes", "0"),
        )
}

/// Array formula cell (`f t="array"` with ref spanning the array).
pub fn cell_array_formula(
    reference: &str,
    formula: &str,
    array_ref: &str,
    cached_value: Option<&str>,
) -> OpenXmlElement {
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    let f = OpenXmlElement::new("x", X, "f")
        .with_attribute("t", "array")
        .with_attribute("ref", array_ref)
        .with_text(formula);
    c.append_child(f);
    if let Some(v) = cached_value {
        c.append_child(OpenXmlElement::new("x", X, "v").with_text(v));
    }
    c
}

/// Sheet-local defined name (name scoped to a sheet id).
pub fn defined_name_local(name: &str, refers_to: &str, local_sheet_id: u32) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "definedName")
        .with_attribute("name", name)
        .with_attribute("localSheetId", local_sheet_id.to_string())
        .with_text(refers_to)
}

/// A manual row page break at `id` (0-based row index before which to break).
pub fn row_break(id: u32, max: Option<u32>, manual: bool) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "brk").with_attribute("id", id.to_string());
    if let Some(m) = max {
        el.set_attribute("max", m.to_string());
    }
    if manual {
        el.set_attribute("man", "1");
    }
    el
}

/// `x:rowBreaks` container.
pub fn row_breaks(breaks: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let kids: Vec<_> = breaks.into_iter().collect();
    let count = kids.len();
    OpenXmlElement::new("x", X, "rowBreaks")
        .with_attribute("count", count.to_string())
        .with_attribute("manualBreakCount", count.to_string())
        .with_children(kids)
}

/// `x:colBreaks` container.
pub fn col_breaks(breaks: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let kids: Vec<_> = breaks.into_iter().collect();
    let count = kids.len();
    OpenXmlElement::new("x", X, "colBreaks")
        .with_attribute("count", count.to_string())
        .with_attribute("manualBreakCount", count.to_string())
        .with_children(kids)
}

/// Hyperlink relationship element for worksheets (`x:hyperlink`).
pub fn sheet_hyperlink(cell_ref: &str, relationship_id: &str, display: Option<&str>) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "hyperlink")
        .with_attribute("ref", cell_ref)
        .with_attribute_qname("r:id", relationship_id);
    if let Some(d) = display {
        el.set_attribute("display", d);
    }
    el
}

/// Hyperlink with optional tooltip and location (in-document target).
pub fn sheet_hyperlink_ex(
    cell_ref: &str,
    relationship_id: Option<&str>,
    display: Option<&str>,
    tooltip: Option<&str>,
    location: Option<&str>,
) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "hyperlink").with_attribute("ref", cell_ref);
    if let Some(rid) = relationship_id {
        el.set_attribute_qname("r:id", rid);
    }
    if let Some(d) = display {
        el.set_attribute("display", d);
    }
    if let Some(t) = tooltip {
        el.set_attribute("tooltip", t);
    }
    if let Some(loc) = location {
        el.set_attribute("location", loc);
    }
    el
}

/// `x:hyperlinks` container.
pub fn sheet_hyperlinks(
    links: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "hyperlinks").with_children(links)
}

/// Sort state for a range (`x:sortState`).
pub fn sort_state(reference: &str, column_ref: &str, descending: bool) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "sortState")
        .with_attribute("ref", reference)
        .with_child(
            OpenXmlElement::new("x", X, "sortCondition")
                .with_attribute("ref", column_ref)
                .with_attribute("descending", if descending { "1" } else { "0" }),
        )
}

/// Whole-number data validation.
pub fn data_validation_whole(
    sqref: &str,
    operator: &str,
    formula1: &str,
    formula2: Option<&str>,
    allow_blank: bool,
) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "dataValidation")
        .with_attribute("type", "whole")
        .with_attribute("operator", operator)
        .with_attribute("allowBlank", if allow_blank { "1" } else { "0" })
        .with_attribute("showInputMessage", "1")
        .with_attribute("showErrorMessage", "1")
        .with_attribute("sqref", sqref)
        .with_child(OpenXmlElement::new("x", X, "formula1").with_text(formula1));
    if let Some(f2) = formula2 {
        el.append_child(OpenXmlElement::new("x", X, "formula2").with_text(f2));
    }
    el
}

/// List-style data validation (`list` type) for a cell range.
pub fn data_validation_list(sqref: &str, formula: &str, allow_blank: bool) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "dataValidation")
        .with_attribute("type", "list")
        .with_attribute("allowBlank", if allow_blank { "1" } else { "0" })
        .with_attribute("showInputMessage", "1")
        .with_attribute("showErrorMessage", "1")
        .with_attribute("sqref", sqref)
        .with_child(OpenXmlElement::new("x", X, "formula1").with_text(formula))
}

/// `x:dataValidations` container.
pub fn data_validations(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let kids: Vec<_> = children.into_iter().collect();
    let count = kids.len();
    OpenXmlElement::new("x", X, "dataValidations")
        .with_attribute("count", count.to_string())
        .with_children(kids)
}

/// A single calculation-chain cell reference (`x:c` in calcChain).
///
/// `sheet_id` is the 1-based sheet index used by Excel's calc chain.
pub fn calc_chain_cell(cell_ref: &str, sheet_id: u32) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "c")
        .with_attribute("r", cell_ref)
        .with_attribute("i", sheet_id.to_string())
}

/// Minimal `x:calcChain` root.
pub fn calc_chain(cells: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "calcChain")
        .with_ns_decl("x", X)
        .with_children(cells)
}

/// Worksheet sheet protection (no password crypto).
pub fn sheet_protection(sheet: bool, objects: bool, scenarios: bool) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "sheetProtection")
        .with_attribute("sheet", if sheet { "1" } else { "0" })
        .with_attribute("objects", if objects { "1" } else { "0" })
        .with_attribute("scenarios", if scenarios { "1" } else { "0" })
}

/// Workbook-level protection element (structure/windows locks, no password).
pub fn workbook_protection(lock_structure: bool, lock_windows: bool) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "workbookProtection");
    if lock_structure {
        el.set_attribute("lockStructure", "1");
    }
    if lock_windows {
        el.set_attribute("lockWindows", "1");
    }
    el
}

/// Freeze panes via `x:sheetViews` / `x:sheetView` / `x:pane`.
///
/// `x_split`/`y_split` are the number of columns/rows frozen (0 = none).
/// `top_left_cell` is the first unfrozen cell (e.g. `"B2"`).
pub fn freeze_panes_views(x_split: f64, y_split: f64, top_left_cell: &str) -> OpenXmlElement {
    let mut pane = OpenXmlElement::new("x", X, "pane")
        .with_attribute("topLeftCell", top_left_cell)
        .with_attribute("activePane", "bottomRight")
        .with_attribute("state", "frozen");
    if x_split > 0.0 {
        pane.set_attribute("xSplit", x_split.to_string());
    }
    if y_split > 0.0 {
        pane.set_attribute("ySplit", y_split.to_string());
    }
    let view = OpenXmlElement::new("x", X, "sheetView")
        .with_attribute("workbookViewId", "0")
        .with_child(pane)
        .with_child(
            OpenXmlElement::new("x", X, "selection")
                .with_attribute("pane", "bottomRight")
                .with_attribute("activeCell", top_left_cell)
                .with_attribute("sqref", top_left_cell),
        );
    OpenXmlElement::new("x", X, "sheetViews").with_child(view)
}

/// Sheet views with a zoom scale percentage (e.g. `75`, `100`, `150`).
pub fn sheet_views_zoom(zoom_scale: u32) -> OpenXmlElement {
    let view = OpenXmlElement::new("x", X, "sheetView")
        .with_attribute("workbookViewId", "0")
        .with_attribute("zoomScale", zoom_scale.to_string())
        .with_attribute("zoomScaleNormal", zoom_scale.to_string());
    OpenXmlElement::new("x", X, "sheetViews").with_child(view)
}

/// Worksheet page margins in inches.
pub fn page_margins(
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    header: f64,
    footer: f64,
) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "pageMargins")
        .with_attribute("left", left.to_string())
        .with_attribute("right", right.to_string())
        .with_attribute("top", top.to_string())
        .with_attribute("bottom", bottom.to_string())
        .with_attribute("header", header.to_string())
        .with_attribute("footer", footer.to_string())
}

/// Worksheet page setup (paper size, orientation).
///
/// `paper_size` is OOXML paper size id (1 = Letter, 9 = A4).
/// `orientation` is `"portrait"` or `"landscape"`.
pub fn page_setup(paper_size: u32, orientation: &str, fit_to_page: bool) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "pageSetup")
        .with_attribute("paperSize", paper_size.to_string())
        .with_attribute("orientation", orientation);
    if fit_to_page {
        el.set_attribute("fitToWidth", "1");
        el.set_attribute("fitToHeight", "1");
    }
    el
}

/// `x:sheets` container.
pub fn sheets(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "sheets").with_children(children)
}

/// `x:sheet` entry in the workbook.
pub fn sheet(name: &str, sheet_id: u32, relationship_id: &str) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "sheet");
    el.set_attribute("name", name);
    el.set_attribute("sheetId", sheet_id.to_string());
    el.set_attribute_ns(
        "r",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        "id",
        relationship_id,
    );
    el
}

/// `x:sheet` entry with optional state (`visible`, `hidden`, `veryHidden`).
pub fn sheet_with_state(
    name: &str,
    sheet_id: u32,
    relationship_id: &str,
    state: Option<&str>,
) -> OpenXmlElement {
    let mut el = sheet(name, sheet_id, relationship_id);
    if let Some(state) = state {
        el.set_attribute("state", state);
    }
    el
}

/// Workbook calculation properties (`x:calcPr`).
pub fn calc_properties(full_calc_on_load: bool, calc_mode: &str) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "calcPr").with_attribute("calcMode", calc_mode);
    if full_calc_on_load {
        el.set_attribute("fullCalcOnLoad", "1");
    }
    el
}

/// Sheet outline properties (`x:outlinePr` under `sheetPr`).
pub fn outline_properties(summary_below: bool, summary_right: bool) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "outlinePr");
    el.set_attribute("summaryBelow", if summary_below { "1" } else { "0" });
    el.set_attribute("summaryRight", if summary_right { "1" } else { "0" });
    el
}

/// Set outline level on a row element (mutates).
pub fn row_set_outline_level(row: &mut OpenXmlElement, level: u8, hidden: bool) {
    row.set_attribute("outlineLevel", level.to_string());
    if hidden {
        row.set_attribute("hidden", "1");
    }
}

/// `x:worksheet` root.
pub fn worksheet(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "worksheet")
        .with_ns_decl("x", X)
        .with_children(children)
}

/// `x:sheetData`.
pub fn sheet_data(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "sheetData").with_children(children)
}

/// `x:cols` container for column width definitions.
pub fn columns(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "cols").with_children(children)
}

/// `x:col` — column width for columns `min`..=`max` (1-based).
///
/// `width` is in character units (Excel's default measure).
pub fn column(min: u32, max: u32, width: f64, custom_width: bool) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "col");
    el.set_attribute("min", min.to_string());
    el.set_attribute("max", max.to_string());
    el.set_attribute("width", width.to_string());
    if custom_width {
        el.set_attribute("customWidth", "1");
    }
    el
}

/// `x:col` with optional hidden flag.
pub fn column_with_hidden(
    min: u32,
    max: u32,
    width: f64,
    custom_width: bool,
    hidden: bool,
) -> OpenXmlElement {
    let mut el = column(min, max, width, custom_width);
    if hidden {
        el.set_attribute("hidden", "1");
    }
    el
}

/// `x:row` with a 1-based row index.
pub fn row(row_index: u32, children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "row");
    el.set_attribute("r", row_index.to_string());
    el.with_children(children)
}

/// `x:row` with optional height (points) and hidden flag.
pub fn row_with_height(
    row_index: u32,
    height_points: Option<f64>,
    hidden: bool,
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "row").with_attribute("r", row_index.to_string());
    if let Some(h) = height_points {
        el.set_attribute("ht", h.to_string());
        el.set_attribute("customHeight", "1");
    }
    if hidden {
        el.set_attribute("hidden", "1");
    }
    el.with_children(children)
}

/// Column definition with optional hidden flag.
pub fn column_ex(
    min: u32,
    max: u32,
    width: f64,
    custom_width: bool,
    hidden: bool,
) -> OpenXmlElement {
    let mut el = column(min, max, width, custom_width);
    if hidden {
        el.set_attribute("hidden", "1");
    }
    el
}

/// Sheet format properties (`x:sheetFormatPr`).
pub fn sheet_format_properties(
    default_row_height: f64,
    default_col_width: Option<f64>,
) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "sheetFormatPr")
        .with_attribute("defaultRowHeight", default_row_height.to_string());
    if let Some(w) = default_col_width {
        el.set_attribute("defaultColWidth", w.to_string());
        el.set_attribute("customHeight", "1");
    }
    el
}

/// Worksheet dimension (`x:dimension ref="A1:C10"`).
pub fn sheet_dimension(reference: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "dimension").with_attribute("ref", reference)
}

/// Shared formula cell (`t` omitted, `f` with `t="shared"`).
///
/// `si` is the shared formula index; only the master cell should include `formula`.
pub fn cell_shared_formula(
    reference: &str,
    si: u32,
    formula: Option<&str>,
    cached_value: Option<&str>,
) -> OpenXmlElement {
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    let mut f = OpenXmlElement::new("x", X, "f")
        .with_attribute("t", "shared")
        .with_attribute("si", si.to_string());
    if let Some(formula) = formula {
        f.set_attribute("ref", reference);
        f.set_text(formula);
    }
    c.append_child(f);
    if let Some(v) = cached_value {
        c.append_child(OpenXmlElement::new("x", X, "v").with_text(v));
    }
    c
}

/// Rich-text inline string cell with multiple text runs (bold/normal segments).
///
/// Each segment is `(text, bold)`.
pub fn cell_rich_text(reference: &str, segments: &[(&str, bool)]) -> OpenXmlElement {
    let mut is = OpenXmlElement::new("x", X, "is");
    for (text, bold) in segments {
        let mut r = OpenXmlElement::new("x", X, "r");
        if *bold {
            r.append_child(
                OpenXmlElement::new("x", X, "rPr")
                    .with_child(OpenXmlElement::new("x", X, "b")),
            );
        }
        let mut t = OpenXmlElement::new("x", X, "t").with_text(*text);
        if text.starts_with(char::is_whitespace) || text.ends_with(char::is_whitespace) {
            t.set_attribute_ns(
                "xml",
                "http://www.w3.org/XML/1998/namespace",
                "space",
                "preserve",
            );
        }
        r.append_child(t);
        is.append_child(r);
    }
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    c.set_attribute("t", "inlineStr");
    c.append_child(is);
    c
}

/// Minimal chartsheet root referencing a drawing relationship.
pub fn chartsheet(drawing_rel_id: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "chartsheet")
        .with_ns_decl("x", X)
        .with_ns_decl(
            "r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        )
        .with_child(
            OpenXmlElement::new("x", X, "drawing")
                .with_attribute_qname("r:id", drawing_rel_id),
        )
}

/// Inline-string cell (`t="inlineStr"`).
pub fn cell_inline_str(reference: &str, value: &str) -> OpenXmlElement {
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    c.set_attribute("t", "inlineStr");
    c.append_child(
        OpenXmlElement::new("x", X, "is")
            .with_child(OpenXmlElement::new("x", X, "t").with_text(value)),
    );
    c
}

/// Numeric cell.
pub fn cell_number(reference: &str, value: f64) -> OpenXmlElement {
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    c.append_child(OpenXmlElement::new("x", X, "v").with_text(value.to_string()));
    c
}

/// Formula cell (`x:f` + cached `x:v`).
///
/// `formula` should not include a leading `=`.
pub fn cell_formula(reference: &str, formula: &str, cached_value: Option<&str>) -> OpenXmlElement {
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    c.append_child(OpenXmlElement::new("x", X, "f").with_text(formula));
    if let Some(v) = cached_value {
        c.append_child(OpenXmlElement::new("x", X, "v").with_text(v));
    }
    c
}

/// Boolean cell (`t="b"`, value `0`/`1`).
pub fn cell_bool(reference: &str, value: bool) -> OpenXmlElement {
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    c.set_attribute("t", "b");
    c.append_child(
        OpenXmlElement::new("x", X, "v").with_text(if value { "1" } else { "0" }),
    );
    c
}

/// Shared-string cell (`t="s"`, value is the 0-based index into the SST).
pub fn cell_shared_string(reference: &str, sst_index: u32) -> OpenXmlElement {
    let mut c = OpenXmlElement::new("x", X, "c");
    c.set_attribute("r", reference);
    c.set_attribute("t", "s");
    c.append_child(OpenXmlElement::new("x", X, "v").with_text(sst_index.to_string()));
    c
}

/// Shared-string cell with a style index (`s` attribute).
pub fn cell_shared_string_styled(reference: &str, sst_index: u32, style_index: u32) -> OpenXmlElement {
    let mut c = cell_shared_string(reference, sst_index);
    c.set_attribute("s", style_index.to_string());
    c
}

/// Numeric cell with a style index.
pub fn cell_number_styled(reference: &str, value: f64, style_index: u32) -> OpenXmlElement {
    let mut c = cell_number(reference, value);
    c.set_attribute("s", style_index.to_string());
    c
}

/// `x:mergeCell` with a ref like `"A1:B2"`.
pub fn merge_cell(reference: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "mergeCell").with_attribute("ref", reference)
}

/// `x:mergeCells` container.
pub fn merge_cells(refs: impl IntoIterator<Item = impl AsRef<str>>) -> OpenXmlElement {
    let kids: Vec<_> = refs.into_iter().map(|r| merge_cell(r.as_ref())).collect();
    let count = kids.len();
    OpenXmlElement::new("x", X, "mergeCells")
        .with_attribute("count", count.to_string())
        .with_children(kids)
}

// ---------------------------------------------------------------------------
// Stylesheet (minimal)
// ---------------------------------------------------------------------------

/// Build a minimal `x:styleSheet` with optional bold font style.
///
/// Style indices:
/// - 0 — default (always present via cellStyleXfs/cellXfs)
/// - subsequent entries from `bold_styles` / custom builders
pub fn minimal_stylesheet(include_bold: bool) -> OpenXmlElement {
    let mut fonts = vec![
        // font 0: default
        OpenXmlElement::new("x", X, "font")
            .with_child(OpenXmlElement::new("x", X, "sz").with_attribute("val", "11"))
            .with_child(OpenXmlElement::new("x", X, "name").with_attribute("val", "Calibri")),
    ];
    if include_bold {
        fonts.push(
            OpenXmlElement::new("x", X, "font")
                .with_child(OpenXmlElement::new("x", X, "b"))
                .with_child(OpenXmlElement::new("x", X, "sz").with_attribute("val", "11"))
                .with_child(OpenXmlElement::new("x", X, "name").with_attribute("val", "Calibri")),
        );
    }
    let font_count = fonts.len();

    let fills = vec![
        // fill 0: none
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "none"),
        ),
        // fill 1: gray125 (required by Excel)
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "gray125"),
        ),
    ];

    let borders = vec![
        // border 0: empty
        OpenXmlElement::new("x", X, "border")
            .with_child(OpenXmlElement::new("x", X, "left"))
            .with_child(OpenXmlElement::new("x", X, "right"))
            .with_child(OpenXmlElement::new("x", X, "top"))
            .with_child(OpenXmlElement::new("x", X, "bottom"))
            .with_child(OpenXmlElement::new("x", X, "diagonal")),
    ];

    // cellStyleXfs: master formatting (xf 0)
    let cell_style_xfs = OpenXmlElement::new("x", X, "cellStyleXfs")
        .with_attribute("count", "1")
        .with_child(
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0"),
        );

    let mut cell_xfs_kids = vec![
        // xf 0: default
        OpenXmlElement::new("x", X, "xf")
            .with_attribute("numFmtId", "0")
            .with_attribute("fontId", "0")
            .with_attribute("fillId", "0")
            .with_attribute("borderId", "0")
            .with_attribute("xfId", "0"),
    ];
    if include_bold {
        // xf 1: bold
        cell_xfs_kids.push(
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "1")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "0")
                .with_attribute("applyFont", "1"),
        );
    }
    let xf_count = cell_xfs_kids.len();
    let cell_xfs = OpenXmlElement::new("x", X, "cellXfs")
        .with_attribute("count", xf_count.to_string())
        .with_children(cell_xfs_kids);

    OpenXmlElement::new("x", X, "styleSheet")
        .with_ns_decl("x", X)
        .with_child(
            OpenXmlElement::new("x", X, "fonts")
                .with_attribute("count", font_count.to_string())
                .with_children(fonts),
        )
        .with_child(
            OpenXmlElement::new("x", X, "fills")
                .with_attribute("count", "2")
                .with_children(fills),
        )
        .with_child(
            OpenXmlElement::new("x", X, "borders")
                .with_attribute("count", "1")
                .with_children(borders),
        )
        .with_child(cell_style_xfs)
        .with_child(cell_xfs)
}

/// Style index for bold cells when using [`minimal_stylesheet(true)`](minimal_stylesheet).
pub const STYLE_BOLD: u32 = 1;
pub const STYLE_DEFAULT: u32 = 0;
/// Style index for solid-fill cells when using [`stylesheet_with_fill`].
pub const STYLE_FILL: u32 = 1;
/// Named style "Title" cell xf index when using [`stylesheet_with_named_styles`].
pub const STYLE_NAMED_TITLE: u32 = 1;

/// Build a stylesheet with default + a named "Title" cell style (bold 18pt).
pub fn stylesheet_with_named_styles() -> OpenXmlElement {
    let fonts = vec![
        OpenXmlElement::new("x", X, "font")
            .with_child(OpenXmlElement::new("x", X, "sz").with_attribute("val", "11"))
            .with_child(OpenXmlElement::new("x", X, "name").with_attribute("val", "Calibri")),
        OpenXmlElement::new("x", X, "font")
            .with_child(OpenXmlElement::new("x", X, "b"))
            .with_child(OpenXmlElement::new("x", X, "sz").with_attribute("val", "18"))
            .with_child(OpenXmlElement::new("x", X, "name").with_attribute("val", "Calibri")),
    ];
    let fills = vec![
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "none"),
        ),
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "gray125"),
        ),
    ];
    let borders = vec![OpenXmlElement::new("x", X, "border")
        .with_child(OpenXmlElement::new("x", X, "left"))
        .with_child(OpenXmlElement::new("x", X, "right"))
        .with_child(OpenXmlElement::new("x", X, "top"))
        .with_child(OpenXmlElement::new("x", X, "bottom"))
        .with_child(OpenXmlElement::new("x", X, "diagonal"))];
    let cell_style_xfs = OpenXmlElement::new("x", X, "cellStyleXfs")
        .with_attribute("count", "2")
        .with_children(vec![
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0"),
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "1")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0"),
        ]);
    let cell_xfs = OpenXmlElement::new("x", X, "cellXfs")
        .with_attribute("count", "2")
        .with_children(vec![
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "0"),
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "1")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "1")
                .with_attribute("applyFont", "1"),
        ]);
    let cell_styles = OpenXmlElement::new("x", X, "cellStyles")
        .with_attribute("count", "2")
        .with_children(vec![
            OpenXmlElement::new("x", X, "cellStyle")
                .with_attribute("name", "Normal")
                .with_attribute("xfId", "0")
                .with_attribute("builtinId", "0"),
            OpenXmlElement::new("x", X, "cellStyle")
                .with_attribute("name", "Title")
                .with_attribute("xfId", "1"),
        ]);
    OpenXmlElement::new("x", X, "styleSheet")
        .with_ns_decl("x", X)
        .with_child(
            OpenXmlElement::new("x", X, "fonts")
                .with_attribute("count", "2")
                .with_children(fonts),
        )
        .with_child(
            OpenXmlElement::new("x", X, "fills")
                .with_attribute("count", "2")
                .with_children(fills),
        )
        .with_child(
            OpenXmlElement::new("x", X, "borders")
                .with_attribute("count", "1")
                .with_children(borders),
        )
        .with_child(cell_style_xfs)
        .with_child(cell_xfs)
        .with_child(cell_styles)
}

/// Build a stylesheet with default + solid RGB fill style (index [`STYLE_FILL`]).
///
/// `rgb` is an 8-hex ARGB string, e.g. `"FFFF00"`.
pub fn stylesheet_with_fill(rgb: &str) -> OpenXmlElement {
    let fonts = vec![OpenXmlElement::new("x", X, "font")
        .with_child(OpenXmlElement::new("x", X, "sz").with_attribute("val", "11"))
        .with_child(OpenXmlElement::new("x", X, "name").with_attribute("val", "Calibri"))];

    let fills = vec![
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "none"),
        ),
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "gray125"),
        ),
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill")
                .with_attribute("patternType", "solid")
                .with_child(
                    OpenXmlElement::new("x", X, "fgColor").with_attribute("rgb", rgb),
                ),
        ),
    ];

    let borders = vec![OpenXmlElement::new("x", X, "border")
        .with_child(OpenXmlElement::new("x", X, "left"))
        .with_child(OpenXmlElement::new("x", X, "right"))
        .with_child(OpenXmlElement::new("x", X, "top"))
        .with_child(OpenXmlElement::new("x", X, "bottom"))
        .with_child(OpenXmlElement::new("x", X, "diagonal"))];

    let cell_style_xfs = OpenXmlElement::new("x", X, "cellStyleXfs")
        .with_attribute("count", "1")
        .with_child(
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0"),
        );

    let cell_xfs = OpenXmlElement::new("x", X, "cellXfs")
        .with_attribute("count", "2")
        .with_children(vec![
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "0"),
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "2")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "0")
                .with_attribute("applyFill", "1"),
        ]);

    OpenXmlElement::new("x", X, "styleSheet")
        .with_ns_decl("x", X)
        .with_child(
            OpenXmlElement::new("x", X, "fonts")
                .with_attribute("count", "1")
                .with_children(fonts),
        )
        .with_child(
            OpenXmlElement::new("x", X, "fills")
                .with_attribute("count", "3")
                .with_children(fills),
        )
        .with_child(
            OpenXmlElement::new("x", X, "borders")
                .with_attribute("count", "1")
                .with_children(borders),
        )
        .with_child(cell_style_xfs)
        .with_child(cell_xfs)
}

/// Build a stylesheet with a custom number format and a cell xf that uses it.
///
/// Returns `(stylesheet, style_index)` where `style_index` is the cellXfs index
/// that applies `num_fmt_code` (e.g. `"0.00%"`, `"yyyy-mm-dd"`).
pub fn stylesheet_with_num_fmt(num_fmt_code: &str) -> (OpenXmlElement, u32) {
    // Custom numFmts use ids >= 164
    let num_fmt_id = 164u32;
    let num_fmts = OpenXmlElement::new("x", X, "numFmts")
        .with_attribute("count", "1")
        .with_child(
            OpenXmlElement::new("x", X, "numFmt")
                .with_attribute("numFmtId", num_fmt_id.to_string())
                .with_attribute("formatCode", num_fmt_code),
        );

    let fonts = vec![OpenXmlElement::new("x", X, "font")
        .with_child(OpenXmlElement::new("x", X, "sz").with_attribute("val", "11"))
        .with_child(OpenXmlElement::new("x", X, "name").with_attribute("val", "Calibri"))];
    let fills = vec![
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "none"),
        ),
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "gray125"),
        ),
    ];
    let borders = vec![OpenXmlElement::new("x", X, "border")
        .with_child(OpenXmlElement::new("x", X, "left"))
        .with_child(OpenXmlElement::new("x", X, "right"))
        .with_child(OpenXmlElement::new("x", X, "top"))
        .with_child(OpenXmlElement::new("x", X, "bottom"))
        .with_child(OpenXmlElement::new("x", X, "diagonal"))];
    let cell_style_xfs = OpenXmlElement::new("x", X, "cellStyleXfs")
        .with_attribute("count", "1")
        .with_child(
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0"),
        );
    let cell_xfs = OpenXmlElement::new("x", X, "cellXfs")
        .with_attribute("count", "2")
        .with_children(vec![
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "0"),
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", num_fmt_id.to_string())
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "0")
                .with_attribute("applyNumberFormat", "1"),
        ]);

    let sheet = OpenXmlElement::new("x", X, "styleSheet")
        .with_ns_decl("x", X)
        .with_child(num_fmts)
        .with_child(
            OpenXmlElement::new("x", X, "fonts")
                .with_attribute("count", "1")
                .with_children(fonts),
        )
        .with_child(
            OpenXmlElement::new("x", X, "fills")
                .with_attribute("count", "2")
                .with_children(fills),
        )
        .with_child(
            OpenXmlElement::new("x", X, "borders")
                .with_attribute("count", "1")
                .with_children(borders),
        )
        .with_child(cell_style_xfs)
        .with_child(cell_xfs);
    (sheet, 1)
}

/// Style index for the thin-border cell xf produced by [`stylesheet_with_border`].
pub const STYLE_BORDER: u32 = 1;

/// Build a stylesheet with a thin black border style at index [`STYLE_BORDER`].
pub fn stylesheet_with_border() -> OpenXmlElement {
    let fonts = vec![OpenXmlElement::new("x", X, "font")
        .with_child(OpenXmlElement::new("x", X, "sz").with_attribute("val", "11"))
        .with_child(OpenXmlElement::new("x", X, "name").with_attribute("val", "Calibri"))];
    let fills = vec![
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "none"),
        ),
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill").with_attribute("patternType", "gray125"),
        ),
    ];
    let side = |name: &str| {
        OpenXmlElement::new("x", X, name)
            .with_attribute("style", "thin")
            .with_child(
                OpenXmlElement::new("x", X, "color").with_attribute("rgb", "FF000000"),
            )
    };
    let borders = vec![
        // border 0: empty
        OpenXmlElement::new("x", X, "border")
            .with_child(OpenXmlElement::new("x", X, "left"))
            .with_child(OpenXmlElement::new("x", X, "right"))
            .with_child(OpenXmlElement::new("x", X, "top"))
            .with_child(OpenXmlElement::new("x", X, "bottom"))
            .with_child(OpenXmlElement::new("x", X, "diagonal")),
        // border 1: thin all sides
        OpenXmlElement::new("x", X, "border")
            .with_child(side("left"))
            .with_child(side("right"))
            .with_child(side("top"))
            .with_child(side("bottom"))
            .with_child(OpenXmlElement::new("x", X, "diagonal")),
    ];
    let cell_style_xfs = OpenXmlElement::new("x", X, "cellStyleXfs")
        .with_attribute("count", "1")
        .with_child(
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0"),
        );
    let cell_xfs = OpenXmlElement::new("x", X, "cellXfs")
        .with_attribute("count", "2")
        .with_children(vec![
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "0")
                .with_attribute("xfId", "0"),
            OpenXmlElement::new("x", X, "xf")
                .with_attribute("numFmtId", "0")
                .with_attribute("fontId", "0")
                .with_attribute("fillId", "0")
                .with_attribute("borderId", "1")
                .with_attribute("xfId", "0")
                .with_attribute("applyBorder", "1"),
        ]);
    OpenXmlElement::new("x", X, "styleSheet")
        .with_ns_decl("x", X)
        .with_child(
            OpenXmlElement::new("x", X, "fonts")
                .with_attribute("count", "1")
                .with_children(fonts),
        )
        .with_child(
            OpenXmlElement::new("x", X, "fills")
                .with_attribute("count", "2")
                .with_children(fills),
        )
        .with_child(
            OpenXmlElement::new("x", X, "borders")
                .with_attribute("count", "2")
                .with_children(borders),
        )
        .with_child(cell_style_xfs)
        .with_child(cell_xfs)
}

/// Minimal shared-strings table with the given strings.
pub fn shared_string_table(strings: impl IntoIterator<Item = impl AsRef<str>>) -> OpenXmlElement {
    let items: Vec<OpenXmlElement> = strings
        .into_iter()
        .map(|s| {
            let text = s.as_ref();
            let mut t = OpenXmlElement::new("x", X, "t").with_text(text);
            if text.starts_with(char::is_whitespace) || text.ends_with(char::is_whitespace) {
                t.set_attribute_ns("xml", "http://www.w3.org/XML/1998/namespace", "space", "preserve");
            }
            OpenXmlElement::new("x", X, "si").with_child(t)
        })
        .collect();
    let count = items.len();
    let mut sst = OpenXmlElement::new("x", X, "sst")
        .with_ns_decl("x", X)
        .with_children(items);
    sst.set_attribute("count", count.to_string());
    sst.set_attribute("uniqueCount", count.to_string());
    sst
}

/// In-memory shared string table builder (deduplicates strings).
#[derive(Debug, Default, Clone)]
pub struct SharedStringTableBuilder {
    strings: Vec<String>,
    index: std::collections::HashMap<String, u32>,
}

impl SharedStringTableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a string, returning its 0-based index.
    pub fn intern(&mut self, s: impl AsRef<str>) -> u32 {
        let s = s.as_ref();
        if let Some(&idx) = self.index.get(s) {
            return idx;
        }
        let idx = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.index.insert(s.to_string(), idx);
        idx
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    pub fn get(&self, index: u32) -> Option<&str> {
        self.strings.get(index as usize).map(|s| s.as_str())
    }

    pub fn strings(&self) -> &[String] {
        &self.strings
    }

    pub fn to_element(&self) -> OpenXmlElement {
        shared_string_table(self.strings.iter().map(|s| s.as_str()))
    }

    /// Replace all occurrences of `from` with `to` in every stored string.
    ///
    /// Rebuilds the internal index map. Returns the number of strings that changed.
    pub fn replace_all(&mut self, from: &str, to: &str) -> usize {
        if from.is_empty() {
            return 0;
        }
        let mut changed = 0usize;
        for s in &mut self.strings {
            if s.contains(from) {
                *s = s.replace(from, to);
                changed += 1;
            }
        }
        self.index.clear();
        for (i, s) in self.strings.iter().enumerate() {
            // Keep first occurrence index for duplicates after replace
            self.index.entry(s.clone()).or_insert(i as u32);
        }
        changed
    }

    /// Parse an existing `x:sst` element into a builder.
    pub fn from_element(root: &OpenXmlElement) -> Self {
        let mut b = Self::new();
        for si in root.children_by_name("si") {
            b.intern(si.inner_text());
        }
        b
    }
}
