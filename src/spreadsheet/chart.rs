//! Minimal DrawingML chart helpers for Excel.

use crate::element::OpenXmlElement;

const C: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";
const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// Build a minimal bar chart `c:chartSpace` with categories and values.
///
/// `categories` and `values` must be the same length. Values are embedded as
/// literals (no external workbook reference required for display in many hosts).
pub fn bar_chart_space(title: &str, categories: &[&str], values: &[f64]) -> OpenXmlElement {
    assert_eq!(categories.len(), values.len());

    let cat_pts: Vec<_> = categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(*cat))
        })
        .collect();
    let val_pts: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(v.to_string()))
        })
        .collect();
    let n = categories.len();

    let ser = OpenXmlElement::new("c", C, "ser")
        .with_child(
            OpenXmlElement::new("c", C, "idx").with_attribute("val", "0"),
        )
        .with_child(
            OpenXmlElement::new("c", C, "order").with_attribute("val", "0"),
        )
        .with_child(
            OpenXmlElement::new("c", C, "tx").with_child(
                OpenXmlElement::new("c", C, "v").with_text(title),
            ),
        )
        .with_child(
            OpenXmlElement::new("c", C, "cat").with_child(
                OpenXmlElement::new("c", C, "strLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(cat_pts),
            ),
        )
        .with_child(
            OpenXmlElement::new("c", C, "val").with_child(
                OpenXmlElement::new("c", C, "numLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(val_pts),
            ),
        );

    let bar_chart = OpenXmlElement::new("c", C, "barChart")
        .with_child(
            OpenXmlElement::new("c", C, "barDir").with_attribute("val", "col"),
        )
        .with_child(
            OpenXmlElement::new("c", C, "grouping").with_attribute("val", "clustered"),
        )
        .with_child(ser)
        .with_child(
            OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"),
        )
        .with_child(
            OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"),
        );

    let cat_ax = OpenXmlElement::new("c", C, "catAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(
            OpenXmlElement::new("c", C, "axPos").with_attribute("val", "b"),
        )
        .with_child(
            OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "2"),
        );

    let val_ax = OpenXmlElement::new("c", C, "valAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(
            OpenXmlElement::new("c", C, "axPos").with_attribute("val", "l"),
        )
        .with_child(
            OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "1"),
        );

    let plot = OpenXmlElement::new("c", C, "plotArea")
        .with_child(bar_chart)
        .with_child(cat_ax)
        .with_child(val_ax);

    let chart = OpenXmlElement::new("c", C, "chart")
        .with_child(
            OpenXmlElement::new("c", C, "title").with_child(
                OpenXmlElement::new("c", C, "tx").with_child(
                    OpenXmlElement::new("c", C, "rich")
                        .with_child(OpenXmlElement::new("a", A, "bodyPr"))
                        .with_child(OpenXmlElement::new("a", A, "lstStyle"))
                        .with_child(
                            OpenXmlElement::new("a", A, "p").with_child(
                                OpenXmlElement::new("a", A, "r")
                                    .with_child(OpenXmlElement::new("a", A, "t").with_text(title)),
                            ),
                        ),
                ),
            ),
        )
        .with_child(plot);

    OpenXmlElement::new("c", C, "chartSpace")
        .with_ns_decl("c", C)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(chart)
}

/// Build a minimal doughnut chart `c:chartSpace`.
pub fn doughnut_chart_space(title: &str, categories: &[&str], values: &[f64]) -> OpenXmlElement {
    assert_eq!(categories.len(), values.len());
    let ser = series_with_literals(title, categories, values);
    let doughnut = OpenXmlElement::new("c", C, "doughnutChart")
        .with_child(ser)
        .with_child(
            OpenXmlElement::new("c", C, "holeSize").with_attribute("val", "50"),
        )
        .with_child(
            OpenXmlElement::new("c", C, "firstSliceAng").with_attribute("val", "0"),
        );
    let plot = OpenXmlElement::new("c", C, "plotArea").with_child(doughnut);
    let chart = OpenXmlElement::new("c", C, "chart")
        .with_child(chart_title(title))
        .with_child(plot);
    OpenXmlElement::new("c", C, "chartSpace")
        .with_ns_decl("c", C)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(chart)
}

/// Build a minimal area chart `c:chartSpace`.
pub fn area_chart_space(title: &str, categories: &[&str], values: &[f64]) -> OpenXmlElement {
    assert_eq!(categories.len(), values.len());
    let ser = series_with_literals(title, categories, values);
    let area = OpenXmlElement::new("c", C, "areaChart")
        .with_child(
            OpenXmlElement::new("c", C, "grouping").with_attribute("val", "standard"),
        )
        .with_child(ser)
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"));
    chart_space_with_axes(title, area)
}

/// Build a minimal line chart `c:chartSpace` with embedded category/value literals.
pub fn line_chart_space(title: &str, categories: &[&str], values: &[f64]) -> OpenXmlElement {
    assert_eq!(categories.len(), values.len());
    let ser = series_with_literals(title, categories, values);
    let line_chart = OpenXmlElement::new("c", C, "lineChart")
        .with_child(
            OpenXmlElement::new("c", C, "grouping").with_attribute("val", "standard"),
        )
        .with_child(ser)
        .with_child(OpenXmlElement::new("c", C, "marker").with_attribute("val", "1"))
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"));
    chart_space_with_axes(title, line_chart)
}

/// Build a minimal scatter chart `c:chartSpace` with embedded x/y literals.
///
/// `x_values` and `y_values` must be the same length.
pub fn scatter_chart_space(title: &str, x_values: &[f64], y_values: &[f64]) -> OpenXmlElement {
    assert_eq!(x_values.len(), y_values.len());
    let n = x_values.len();
    let x_pts: Vec<_> = x_values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(v.to_string()))
        })
        .collect();
    let y_pts: Vec<_> = y_values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(v.to_string()))
        })
        .collect();
    let ser = OpenXmlElement::new("c", C, "ser")
        .with_child(OpenXmlElement::new("c", C, "idx").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "order").with_attribute("val", "0"))
        .with_child(
            OpenXmlElement::new("c", C, "tx")
                .with_child(OpenXmlElement::new("c", C, "v").with_text(title)),
        )
        .with_child(
            OpenXmlElement::new("c", C, "xVal").with_child(
                OpenXmlElement::new("c", C, "numLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(x_pts),
            ),
        )
        .with_child(
            OpenXmlElement::new("c", C, "yVal").with_child(
                OpenXmlElement::new("c", C, "numLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(y_pts),
            ),
        );
    let scatter = OpenXmlElement::new("c", C, "scatterChart")
        .with_child(
            OpenXmlElement::new("c", C, "scatterStyle").with_attribute("val", "lineMarker"),
        )
        .with_child(ser)
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"));
    // Reuse axis helper via val axes for both
    let cat_ax = OpenXmlElement::new("c", C, "valAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "axPos").with_attribute("val", "b"))
        .with_child(OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "2"));
    let val_ax = OpenXmlElement::new("c", C, "valAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "axPos").with_attribute("val", "l"))
        .with_child(OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "1"));
    let plot = OpenXmlElement::new("c", C, "plotArea")
        .with_child(scatter)
        .with_child(cat_ax)
        .with_child(val_ax);
    let chart = OpenXmlElement::new("c", C, "chart")
        .with_child(chart_title(title))
        .with_child(plot);
    OpenXmlElement::new("c", C, "chartSpace")
        .with_ns_decl("c", C)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(chart)
}

/// Build a minimal pie chart `c:chartSpace` with embedded category/value literals.
pub fn pie_chart_space(title: &str, categories: &[&str], values: &[f64]) -> OpenXmlElement {
    assert_eq!(categories.len(), values.len());
    let ser = series_with_literals(title, categories, values);
    let pie_chart = OpenXmlElement::new("c", C, "pieChart")
        .with_child(ser)
        .with_child(OpenXmlElement::new("c", C, "firstSliceAng").with_attribute("val", "0"));
    let plot = OpenXmlElement::new("c", C, "plotArea").with_child(pie_chart);
    let chart = OpenXmlElement::new("c", C, "chart")
        .with_child(chart_title(title))
        .with_child(plot);
    OpenXmlElement::new("c", C, "chartSpace")
        .with_ns_decl("c", C)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(chart)
}

fn series_with_literals(title: &str, categories: &[&str], values: &[f64]) -> OpenXmlElement {
    let n = categories.len();
    let cat_pts: Vec<_> = categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(*cat))
        })
        .collect();
    let val_pts: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(v.to_string()))
        })
        .collect();
    OpenXmlElement::new("c", C, "ser")
        .with_child(OpenXmlElement::new("c", C, "idx").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "order").with_attribute("val", "0"))
        .with_child(
            OpenXmlElement::new("c", C, "tx")
                .with_child(OpenXmlElement::new("c", C, "v").with_text(title)),
        )
        .with_child(
            OpenXmlElement::new("c", C, "cat").with_child(
                OpenXmlElement::new("c", C, "strLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(cat_pts),
            ),
        )
        .with_child(
            OpenXmlElement::new("c", C, "val").with_child(
                OpenXmlElement::new("c", C, "numLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(val_pts),
            ),
        )
}

/// Build a minimal radar chart `c:chartSpace` with embedded category/value literals.
pub fn radar_chart_space(title: &str, categories: &[&str], values: &[f64]) -> OpenXmlElement {
    assert_eq!(categories.len(), values.len());
    let ser = series_with_literals(title, categories, values);
    let radar = OpenXmlElement::new("c", C, "radarChart")
        .with_child(
            OpenXmlElement::new("c", C, "radarStyle").with_attribute("val", "marker"),
        )
        .with_child(ser)
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"));
    chart_space_with_axes(title, radar)
}

/// Build a minimal bubble chart `c:chartSpace` with embedded x/y/size literals.
///
/// All three slices must be the same length.
pub fn bubble_chart_space(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    sizes: &[f64],
) -> OpenXmlElement {
    assert_eq!(x_values.len(), y_values.len());
    assert_eq!(x_values.len(), sizes.len());
    let n = x_values.len();
    let x_pts: Vec<_> = x_values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(v.to_string()))
        })
        .collect();
    let y_pts: Vec<_> = y_values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(v.to_string()))
        })
        .collect();
    let size_pts: Vec<_> = sizes
        .iter()
        .enumerate()
        .map(|(i, v)| {
            OpenXmlElement::new("c", C, "pt")
                .with_attribute("idx", i.to_string())
                .with_child(OpenXmlElement::new("c", C, "v").with_text(v.to_string()))
        })
        .collect();
    let ser = OpenXmlElement::new("c", C, "ser")
        .with_child(OpenXmlElement::new("c", C, "idx").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "order").with_attribute("val", "0"))
        .with_child(
            OpenXmlElement::new("c", C, "tx")
                .with_child(OpenXmlElement::new("c", C, "v").with_text(title)),
        )
        .with_child(
            OpenXmlElement::new("c", C, "xVal").with_child(
                OpenXmlElement::new("c", C, "numLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(x_pts),
            ),
        )
        .with_child(
            OpenXmlElement::new("c", C, "yVal").with_child(
                OpenXmlElement::new("c", C, "numLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(y_pts),
            ),
        )
        .with_child(
            OpenXmlElement::new("c", C, "bubbleSize").with_child(
                OpenXmlElement::new("c", C, "numLit")
                    .with_child(
                        OpenXmlElement::new("c", C, "ptCount")
                            .with_attribute("val", n.to_string()),
                    )
                    .with_children(size_pts),
            ),
        );
    let bubble = OpenXmlElement::new("c", C, "bubbleChart")
        .with_child(ser)
        .with_child(OpenXmlElement::new("c", C, "bubble3D").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"));
    // Bubble uses two value axes
    let x_ax = OpenXmlElement::new("c", C, "valAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "axPos").with_attribute("val", "b"))
        .with_child(OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "2"));
    let y_ax = OpenXmlElement::new("c", C, "valAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "axPos").with_attribute("val", "l"))
        .with_child(OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "1"));
    let plot = OpenXmlElement::new("c", C, "plotArea")
        .with_child(bubble)
        .with_child(x_ax)
        .with_child(y_ax);
    let chart = OpenXmlElement::new("c", C, "chart")
        .with_child(chart_title(title))
        .with_child(plot);
    OpenXmlElement::new("c", C, "chartSpace")
        .with_ns_decl("c", C)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(chart)
}

fn chart_title(title: &str) -> OpenXmlElement {
    OpenXmlElement::new("c", C, "title").with_child(
        OpenXmlElement::new("c", C, "tx").with_child(
            OpenXmlElement::new("c", C, "rich")
                .with_child(OpenXmlElement::new("a", A, "bodyPr"))
                .with_child(OpenXmlElement::new("a", A, "lstStyle"))
                .with_child(
                    OpenXmlElement::new("a", A, "p").with_child(
                        OpenXmlElement::new("a", A, "r")
                            .with_child(OpenXmlElement::new("a", A, "t").with_text(title)),
                    ),
                ),
        ),
    )
}

fn chart_space_with_axes(title: &str, chart_kind: OpenXmlElement) -> OpenXmlElement {
    let cat_ax = OpenXmlElement::new("c", C, "catAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "1"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "axPos").with_attribute("val", "b"))
        .with_child(OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "2"));
    let val_ax = OpenXmlElement::new("c", C, "valAx")
        .with_child(OpenXmlElement::new("c", C, "axId").with_attribute("val", "2"))
        .with_child(
            OpenXmlElement::new("c", C, "scaling").with_child(
                OpenXmlElement::new("c", C, "orientation").with_attribute("val", "minMax"),
            ),
        )
        .with_child(OpenXmlElement::new("c", C, "delete").with_attribute("val", "0"))
        .with_child(OpenXmlElement::new("c", C, "axPos").with_attribute("val", "l"))
        .with_child(OpenXmlElement::new("c", C, "crossAx").with_attribute("val", "1"));
    let plot = OpenXmlElement::new("c", C, "plotArea")
        .with_child(chart_kind)
        .with_child(cat_ax)
        .with_child(val_ax);
    let chart = OpenXmlElement::new("c", C, "chart")
        .with_child(chart_title(title))
        .with_child(plot);
    OpenXmlElement::new("c", C, "chartSpace")
        .with_ns_decl("c", C)
        .with_ns_decl("a", A)
        .with_ns_decl("r", R)
        .with_child(chart)
}
