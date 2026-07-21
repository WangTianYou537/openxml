//! Spreadsheet drawing (`xdr:wsDr`) helpers for anchoring charts on sheets.

use crate::element::OpenXmlElement;

const XDR: &str = "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing";
const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const C: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";

/// Cell marker (`xdr:from` / `xdr:to`).
///
/// Columns and rows are 0-based (as in the OOXML drawing schema).
pub fn marker(col: u32, col_off: i64, row: u32, row_off: i64) -> OpenXmlElement {
    // local name set by caller via wrapper
    OpenXmlElement::new("xdr", XDR, "from") // placeholder name
        .with_child(OpenXmlElement::new("xdr", XDR, "col").with_text(col.to_string()))
        .with_child(OpenXmlElement::new("xdr", XDR, "colOff").with_text(col_off.to_string()))
        .with_child(OpenXmlElement::new("xdr", XDR, "row").with_text(row.to_string()))
        .with_child(OpenXmlElement::new("xdr", XDR, "rowOff").with_text(row_off.to_string()))
}

fn marker_named(
    local: &str,
    col: u32,
    col_off: i64,
    row: u32,
    row_off: i64,
) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, local)
        .with_child(OpenXmlElement::new("xdr", XDR, "col").with_text(col.to_string()))
        .with_child(OpenXmlElement::new("xdr", XDR, "colOff").with_text(col_off.to_string()))
        .with_child(OpenXmlElement::new("xdr", XDR, "row").with_text(row.to_string()))
        .with_child(OpenXmlElement::new("xdr", XDR, "rowOff").with_text(row_off.to_string()))
}

/// Non-visual graphic frame properties.
fn nv_graphic_frame_pr(id: u32, name: &str) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "nvGraphicFramePr")
        .with_child(
            OpenXmlElement::new("xdr", XDR, "cNvPr")
                .with_attribute("id", id.to_string())
                .with_attribute("name", name),
        )
        .with_child(OpenXmlElement::new("xdr", XDR, "cNvGraphicFramePr"))
}

/// Transform for the graphic frame (often identity at origin).
fn graphic_frame_xfrm() -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "xfrm")
        .with_child(
            OpenXmlElement::new("a", A, "off")
                .with_attribute("x", "0")
                .with_attribute("y", "0"),
        )
        .with_child(
            OpenXmlElement::new("a", A, "ext")
                .with_attribute("cx", "0")
                .with_attribute("cy", "0"),
        )
}

/// `xdr:graphicFrame` that references a chart part via `r:id`.
pub fn chart_graphic_frame(id: u32, name: &str, chart_rel_id: &str) -> OpenXmlElement {
    let graphic_data = OpenXmlElement::new("a", A, "graphicData")
        .with_attribute("uri", C)
        .with_child(
            OpenXmlElement::new("c", C, "chart").with_attribute_ns("r", R, "id", chart_rel_id),
        );

    OpenXmlElement::new("xdr", XDR, "graphicFrame")
        .with_child(nv_graphic_frame_pr(id, name))
        .with_child(graphic_frame_xfrm())
        .with_child(
            OpenXmlElement::new("a", A, "graphic").with_child(graphic_data),
        )
}

/// `xdr:twoCellAnchor` placing a chart between two cells.
///
/// Columns/rows are **0-based**. Offsets are in EMUs.
pub fn two_cell_anchor_chart(
    from_col: u32,
    from_row: u32,
    to_col: u32,
    to_row: u32,
    chart_rel_id: &str,
    name: &str,
) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "twoCellAnchor")
        .with_attribute("editAs", "oneCell")
        .with_child(marker_named("from", from_col, 0, from_row, 0))
        .with_child(marker_named("to", to_col, 0, to_row, 0))
        .with_child(chart_graphic_frame(2, name, chart_rel_id))
        .with_child(OpenXmlElement::new("xdr", XDR, "clientData"))
}

/// Picture non-visual properties.
fn nv_pic_pr(id: u32, name: &str) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "nvPicPr")
        .with_child(
            OpenXmlElement::new("xdr", XDR, "cNvPr")
                .with_attribute("id", id.to_string())
                .with_attribute("name", name),
        )
        .with_child(
            OpenXmlElement::new("xdr", XDR, "cNvPicPr").with_child(
                OpenXmlElement::new("a", A, "picLocks").with_attribute("noChangeAspect", "1"),
            ),
        )
}

/// `xdr:pic` referencing an image part via `r:id` (blip embed).
pub fn picture(id: u32, name: &str, image_rel_id: &str, cx: i64, cy: i64) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "pic")
        .with_child(nv_pic_pr(id, name))
        .with_child(
            OpenXmlElement::new("xdr", XDR, "blipFill")
                .with_child(
                    OpenXmlElement::new("a", A, "blip")
                        .with_attribute_ns("r", R, "embed", image_rel_id)
                        .with_attribute("cstate", "print"),
                )
                .with_child(
                    OpenXmlElement::new("a", A, "stretch")
                        .with_child(OpenXmlElement::new("a", A, "fillRect")),
                ),
        )
        .with_child(
            OpenXmlElement::new("xdr", XDR, "spPr")
                .with_child(
                    OpenXmlElement::new("a", A, "xfrm")
                        .with_child(
                            OpenXmlElement::new("a", A, "off")
                                .with_attribute("x", "0")
                                .with_attribute("y", "0"),
                        )
                        .with_child(
                            OpenXmlElement::new("a", A, "ext")
                                .with_attribute("cx", cx.to_string())
                                .with_attribute("cy", cy.to_string()),
                        ),
                )
                .with_child(
                    OpenXmlElement::new("a", A, "prstGeom")
                        .with_attribute("prst", "rect")
                        .with_child(OpenXmlElement::new("a", A, "avLst")),
                ),
        )
}

/// `xdr:oneCellAnchor` placing a picture from a cell with a fixed extent (EMU).
pub fn one_cell_anchor_picture(
    from_col: u32,
    from_row: u32,
    cx: i64,
    cy: i64,
    image_rel_id: &str,
    name: &str,
) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "oneCellAnchor")
        .with_child(marker_named("from", from_col, 0, from_row, 0))
        .with_child(
            OpenXmlElement::new("xdr", XDR, "ext")
                .with_attribute("cx", cx.to_string())
                .with_attribute("cy", cy.to_string()),
        )
        .with_child(picture(3, name, image_rel_id, cx, cy))
        .with_child(OpenXmlElement::new("xdr", XDR, "clientData"))
}

/// `xdr:absoluteAnchor` placing a picture at an absolute position (EMU).
pub fn absolute_anchor_picture(
    x: i64,
    y: i64,
    cx: i64,
    cy: i64,
    image_rel_id: &str,
    name: &str,
) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "absoluteAnchor")
        .with_child(
            OpenXmlElement::new("xdr", XDR, "pos")
                .with_attribute("x", x.to_string())
                .with_attribute("y", y.to_string()),
        )
        .with_child(
            OpenXmlElement::new("xdr", XDR, "ext")
                .with_attribute("cx", cx.to_string())
                .with_attribute("cy", cy.to_string()),
        )
        .with_child(picture(4, name, image_rel_id, cx, cy))
        .with_child(OpenXmlElement::new("xdr", XDR, "clientData"))
}

/// `xdr:twoCellAnchor` placing a picture between two cells.
pub fn two_cell_anchor_picture(
    from_col: u32,
    from_row: u32,
    to_col: u32,
    to_row: u32,
    image_rel_id: &str,
    name: &str,
) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "twoCellAnchor")
        .with_attribute("editAs", "oneCell")
        .with_child(marker_named("from", from_col, 0, from_row, 0))
        .with_child(marker_named("to", to_col, 0, to_row, 0))
        .with_child(picture(5, name, image_rel_id, 0, 0))
        .with_child(OpenXmlElement::new("xdr", XDR, "clientData"))
}

/// `xdr:wsDr` worksheet drawing root containing the given anchors.
pub fn worksheet_drawing(
    anchors: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("xdr", XDR, "wsDr")
        .with_ns_decl("xdr", XDR)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_ns_decl("c", C)
        .with_children(anchors)
}

/// `x:drawing` element for the worksheet pointing at a relationship id.
pub fn worksheet_drawing_ref(relationship_id: &str) -> OpenXmlElement {
    let x = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
    OpenXmlElement::new("x", x, "drawing").with_attribute_ns("r", R, "id", relationship_id)
}
