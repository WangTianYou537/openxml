//! Conditional formatting helpers for worksheets.

use crate::element::OpenXmlElement;
use crate::namespace::ns;

const X: &str = ns::SPREADSHEETML.uri;

/// `x:conditionalFormatting` for a set of cell references (`sqref`, e.g. `"A1:A10"`).
pub fn conditional_formatting(
    sqref: &str,
    rules: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "conditionalFormatting")
        .with_attribute("sqref", sqref)
        .with_children(rules)
}

/// Cell-value rule: `type="cellIs"` with an operator and formula(s).
///
/// `operator` is one of: `equal`, `notEqual`, `greaterThan`, `lessThan`,
/// `greaterThanOrEqual`, `lessThanOrEqual`, `between`, `notBetween`.
/// `formulas` holds one or two formula strings (no leading `=`).
pub fn cf_rule_cell_is(
    operator: &str,
    priority: u32,
    dxf_id: Option<u32>,
    formulas: &[&str],
) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "cfRule")
        .with_attribute("type", "cellIs")
        .with_attribute("operator", operator)
        .with_attribute("priority", priority.to_string());
    if let Some(id) = dxf_id {
        el.set_attribute("dxfId", id.to_string());
    }
    for f in formulas {
        el.append_child(OpenXmlElement::new("x", X, "formula").with_text(*f));
    }
    el
}

/// Expression rule: `type="expression"` with a single formula.
pub fn cf_rule_expression(priority: u32, dxf_id: Option<u32>, formula: &str) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "cfRule")
        .with_attribute("type", "expression")
        .with_attribute("priority", priority.to_string());
    if let Some(id) = dxf_id {
        el.set_attribute("dxfId", id.to_string());
    }
    el.append_child(OpenXmlElement::new("x", X, "formula").with_text(formula));
    el
}

/// `x:cfvo` (conditional formatting value object).
pub fn cfvo(kind: &str, value: Option<&str>) -> OpenXmlElement {
    let mut el = OpenXmlElement::new("x", X, "cfvo").with_attribute("type", kind);
    if let Some(v) = value {
        el.set_attribute("val", v);
    }
    el
}

/// `x:color` with RGB (e.g. `"FFFF0000"`).
pub fn cf_color_rgb(rgb: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "color").with_attribute("rgb", rgb)
}

/// 2- or 3-stop color scale rule.
///
/// `stops` is a list of `(cfvo_type, cfvo_val, rgb)` e.g.
/// `[("min", None, "FFF8696B"), ("max", None, "FF63BE7B")]`.
pub fn cf_rule_color_scale(
    priority: u32,
    stops: &[(&str, Option<&str>, &str)],
) -> OpenXmlElement {
    let mut scale = OpenXmlElement::new("x", X, "colorScale");
    for (kind, val, _) in stops {
        scale.append_child(cfvo(kind, *val));
    }
    for (_, _, rgb) in stops {
        scale.append_child(cf_color_rgb(rgb));
    }
    OpenXmlElement::new("x", X, "cfRule")
        .with_attribute("type", "colorScale")
        .with_attribute("priority", priority.to_string())
        .with_child(scale)
}

/// Data bar rule with min/max cfvo and a fill color.
/// Icon set conditional formatting rule (`iconSet` type).
///
/// `icon_set` is e.g. `"3TrafficLights1"`, `"3Arrows"`, `"4Rating"`, `"5Quarters"`.
pub fn cf_rule_icon_set(priority: u32, icon_set: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "cfRule")
        .with_attribute("type", "iconSet")
        .with_attribute("priority", priority.to_string())
        .with_child(
            OpenXmlElement::new("x", X, "iconSet")
                .with_attribute("iconSet", icon_set)
                .with_child(cfvo("percent", Some("0")))
                .with_child(cfvo("percent", Some("33")))
                .with_child(cfvo("percent", Some("67"))),
        )
}

pub fn cf_rule_data_bar(priority: u32, rgb: &str) -> OpenXmlElement {
    let bar = OpenXmlElement::new("x", X, "dataBar")
        .with_child(cfvo("min", None))
        .with_child(cfvo("max", None))
        .with_child(cf_color_rgb(rgb));
    OpenXmlElement::new("x", X, "cfRule")
        .with_attribute("type", "dataBar")
        .with_attribute("priority", priority.to_string())
        .with_child(bar)
}

/// Differential format (`x:dxf`) with solid fill color — for use in stylesheet.
pub fn dxf_fill(rgb: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "dxf").with_child(
        OpenXmlElement::new("x", X, "fill").with_child(
            OpenXmlElement::new("x", X, "patternFill")
                .with_attribute("patternType", "solid")
                .with_child(OpenXmlElement::new("x", X, "fgColor").with_attribute("rgb", rgb)),
        ),
    )
}

/// `x:dxfs` container for differential formats.
pub fn dxfs(items: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    let kids: Vec<_> = items.into_iter().collect();
    let count = kids.len();
    OpenXmlElement::new("x", X, "dxfs")
        .with_attribute("count", count.to_string())
        .with_children(kids)
}
