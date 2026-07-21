//! Minimal VML drawing helpers for Excel cell comments.

use crate::element::OpenXmlElement;

const V: &str = "urn:schemas-microsoft-com:vml";
const O: &str = "urn:schemas-microsoft-com:office:office";
const X: &str = "urn:schemas-microsoft-com:office:excel";

/// Build a minimal `xml` VML drawing root with one note shape per cell.
///
/// Shape ids are `#_x0000_s{1025+i}`; `ClientData` links to `row`/`col` (0-based).
pub fn vml_comments_drawing(notes: &[(u32, u32)]) -> OpenXmlElement {
    let mut root = OpenXmlElement::new("", "", "xml")
        .with_ns_decl("v", V)
        .with_ns_decl("o", O)
        .with_ns_decl("x", X);

    root.append_child(
        OpenXmlElement::new("o", O, "shapelayout")
            .with_attribute_qname("v:ext", "edit")
            .with_child(
                OpenXmlElement::new("o", O, "idmap")
                    .with_attribute_qname("v:ext", "edit")
                    .with_attribute("data", "1"),
            ),
    );

    root.append_child(
        OpenXmlElement::new("v", V, "shapetype")
            .with_attribute("id", "_x0000_t202")
            .with_attribute("coordsize", "21600,21600")
            .with_attribute_qname("o:spt", "202")
            .with_attribute("path", "m,l,21600r21600,l21600,xe")
            .with_child(OpenXmlElement::new("v", V, "stroke").with_attribute("joinstyle", "miter"))
            .with_child(
                OpenXmlElement::new("v", V, "path")
                    .with_attribute("gradientshapeok", "t")
                    .with_attribute_qname("o:connecttype", "rect"),
            ),
    );

    for (i, (row, col)) in notes.iter().enumerate() {
        let shape_id = format!("_x0000_s{}", 1025 + i);
        let shape = OpenXmlElement::new("v", V, "shape")
            .with_attribute("id", &shape_id)
            .with_attribute("type", "#_x0000_t202")
            .with_attribute(
                "style",
                "position:absolute;margin-left:59.25pt;margin-top:1.5pt;width:108pt;height:59.25pt;z-index:1;visibility:hidden",
            )
            .with_attribute("fillcolor", "#ffffe1")
            .with_attribute_qname("o:insetmode", "auto")
            .with_child(OpenXmlElement::new("v", V, "fill").with_attribute("color2", "#ffffe1"))
            .with_child(OpenXmlElement::new("v", V, "shadow")
                .with_attribute("color", "black")
                .with_attribute("obscured", "t"))
            .with_child(OpenXmlElement::new("v", V, "path").with_attribute_qname("o:connecttype", "none"))
            .with_child(
                OpenXmlElement::new("v", V, "textbox")
                    .with_attribute("style", "mso-direction-alt:auto")
                    .with_child(
                        OpenXmlElement::new("", "", "div")
                            .with_attribute("style", "text-align:left"),
                    ),
            )
            .with_child(
                OpenXmlElement::new("x", X, "ClientData")
                    .with_attribute("ObjectType", "Note")
                    .with_child(OpenXmlElement::new("x", X, "MoveWithCells"))
                    .with_child(OpenXmlElement::new("x", X, "SizeWithCells"))
                    .with_child(OpenXmlElement::new("x", X, "AutoFill").with_text("False"))
                    .with_child(OpenXmlElement::new("x", X, "Row").with_text(row.to_string()))
                    .with_child(OpenXmlElement::new("x", X, "Column").with_text(col.to_string())),
            );
        root.append_child(shape);
    }
    root
}

/// Parse an A1-style cell reference into 0-based `(row, col)`.
pub fn cell_ref_to_row_col(cell_ref: &str) -> Option<(u32, u32)> {
    let bytes = cell_ref.as_bytes();
    let mut i = 0;
    let mut col = 0u32;
    while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
        col = col * 26 + (bytes[i].to_ascii_uppercase() - b'A' + 1) as u32;
        i += 1;
    }
    if i == 0 || i >= bytes.len() {
        return None;
    }
    let row: u32 = std::str::from_utf8(&bytes[i..]).ok()?.parse().ok()?;
    if row == 0 {
        return None;
    }
    Some((row - 1, col.saturating_sub(1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cell_ref() {
        assert_eq!(cell_ref_to_row_col("A1"), Some((0, 0)));
        assert_eq!(cell_ref_to_row_col("B2"), Some((1, 1)));
        assert_eq!(cell_ref_to_row_col("AA10"), Some((9, 26)));
    }
}
