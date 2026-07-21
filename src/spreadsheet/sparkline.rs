//! Minimal Excel sparkline helpers (Office 2010+ `x14` namespace).

use crate::element::OpenXmlElement;

const X14: &str = "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";
const XM: &str = "http://schemas.microsoft.com/office/excel/2006/main";

/// A single sparkline linking a data range to a display cell.
pub fn sparkline(data_ref: &str, cell_ref: &str) -> OpenXmlElement {
    OpenXmlElement::new("x14", X14, "sparkline")
        .with_child(
            OpenXmlElement::new("xm", XM, "f").with_text(data_ref),
        )
        .with_child(
            OpenXmlElement::new("xm", XM, "sqref").with_text(cell_ref),
        )
}

/// Sparkline group of the given type (`line`, `column`, or `stacked`).
pub fn sparkline_group(
    sparkline_type: &str,
    data_ref: &str,
    cell_ref: &str,
) -> OpenXmlElement {
    OpenXmlElement::new("x14", X14, "sparklineGroup")
        .with_ns_decl("x14", X14)
        .with_ns_decl("xm", XM)
        .with_attribute("type", sparkline_type)
        .with_attribute("displayEmptyCellsAs", "gap")
        .with_child(
            OpenXmlElement::new("x14", X14, "colorSeries")
                .with_attribute("theme", "1")
                .with_attribute("lastClr", "000000"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "colorNegative")
                .with_attribute("theme", "1"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "colorAxis")
                .with_attribute("theme", "1"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "colorMarkers")
                .with_attribute("theme", "1"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "colorFirst")
                .with_attribute("theme", "1"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "colorLast")
                .with_attribute("theme", "1"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "colorHigh")
                .with_attribute("theme", "1"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "colorLow")
                .with_attribute("theme", "1"),
        )
        .with_child(
            OpenXmlElement::new("x14", X14, "sparklines").with_child(sparkline(data_ref, cell_ref)),
        )
}

/// `x14:sparklineGroups` container.
pub fn sparkline_groups(
    groups: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("x14", X14, "sparklineGroups")
        .with_ns_decl("x14", X14)
        .with_ns_decl("xm", XM)
        .with_children(groups)
}

/// Worksheet extension wrapping sparkline groups (for `x:extLst`).
pub fn sparkline_ext(groups: OpenXmlElement) -> OpenXmlElement {
    let x = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
    OpenXmlElement::new("x", x, "ext")
        .with_attribute("uri", "{05C60535-1F16-4fd2-B633-F4F36F0B64E0}")
        .with_ns_decl("x14", X14)
        .with_child(groups)
}
