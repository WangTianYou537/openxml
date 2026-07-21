//! Minimal pivot table / pivot cache helpers.

use crate::element::OpenXmlElement;
use crate::namespace::ns;

const X: &str = ns::SPREADSHEETML.uri;
const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// `x:pivotCacheDefinition` referencing a worksheet range and optional records rel.
pub fn pivot_cache_definition(
    records_rel_id: &str,
    sheet_name: &str,
    source_ref: &str,
    field_names: &[&str],
    record_count: u32,
) -> OpenXmlElement {
    let fields: Vec<_> = field_names
        .iter()
        .map(|name| {
            OpenXmlElement::new("x", X, "cacheField")
                .with_attribute("name", *name)
                .with_attribute("numFmtId", "0")
                .with_child(
                    OpenXmlElement::new("x", X, "sharedItems")
                        .with_attribute("count", "0"),
                )
        })
        .collect();
    let field_count = field_names.len();

    OpenXmlElement::new("x", X, "pivotCacheDefinition")
        .with_ns_decl("x", X)
        .with_ns_decl("r", R)
        .with_attribute_ns("r", R, "id", records_rel_id)
        .with_attribute("refreshOnLoad", "1")
        .with_attribute("recordCount", record_count.to_string())
        .with_child(
            OpenXmlElement::new("x", X, "cacheSource")
                .with_attribute("type", "worksheet")
                .with_child(
                    OpenXmlElement::new("x", X, "worksheetSource")
                        .with_attribute("ref", source_ref)
                        .with_attribute("sheet", sheet_name),
                ),
        )
        .with_child(
            OpenXmlElement::new("x", X, "cacheFields")
                .with_attribute("count", field_count.to_string())
                .with_children(fields),
        )
}

/// Empty `x:pivotCacheRecords` with a count (Excel can refresh from source).
pub fn pivot_cache_records(count: u32) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "pivotCacheRecords")
        .with_ns_decl("x", X)
        .with_attribute("count", count.to_string())
}

/// Build `x:pivotCacheRecords` from row data (each inner slice is one record).
///
/// Values are written as `x:s` (string) or `x:n` (number) based on parse success.
pub fn pivot_cache_records_from_rows(rows: &[Vec<&str>]) -> OpenXmlElement {
    let mut records = Vec::new();
    for row in rows {
        let mut cells = Vec::new();
        for val in row {
            if let Ok(n) = val.parse::<f64>() {
                cells.push(
                    OpenXmlElement::new("x", X, "n").with_attribute("v", n.to_string()),
                );
            } else {
                cells.push(
                    OpenXmlElement::new("x", X, "s").with_attribute("v", *val),
                );
            }
        }
        records.push(OpenXmlElement::new("x", X, "r").with_children(cells));
    }
    OpenXmlElement::new("x", X, "pivotCacheRecords")
        .with_ns_decl("x", X)
        .with_attribute("count", records.len().to_string())
        .with_children(records)
}

/// Minimal `x:pivotTableDefinition`.
///
/// - `row_field`: 0-based field index used on rows
/// - `data_field`: 0-based field index used as data (sum)
pub fn pivot_table_definition(
    name: &str,
    cache_id: u32,
    location_ref: &str,
    field_names: &[&str],
    row_field: u32,
    data_field: u32,
) -> OpenXmlElement {
    let field_count = field_names.len() as u32;
    let pivot_fields: Vec<_> = (0..field_count)
        .map(|i| {
            let mut pf = OpenXmlElement::new("x", X, "pivotField")
                .with_attribute("showAll", "0");
            if i == row_field {
                pf.set_attribute("axis", "axisRow");
            }
            if i == data_field {
                pf.set_attribute("dataField", "1");
            }
            pf
        })
        .collect();

    OpenXmlElement::new("x", X, "pivotTableDefinition")
        .with_ns_decl("x", X)
        .with_attribute("name", name)
        .with_attribute("cacheId", cache_id.to_string())
        .with_attribute("dataOnRows", "0")
        .with_attribute("applyNumberFormats", "0")
        .with_attribute("applyBorderFormats", "0")
        .with_attribute("applyFontFormats", "0")
        .with_attribute("applyPatternFormats", "0")
        .with_attribute("applyAlignmentFormats", "0")
        .with_attribute("applyWidthHeightFormats", "1")
        .with_attribute("dataCaption", "Values")
        .with_attribute("updatedVersion", "8")
        .with_attribute("minRefreshableVersion", "3")
        .with_attribute("useAutoFormatting", "1")
        .with_attribute("itemPrintTitles", "1")
        .with_attribute("createdVersion", "8")
        .with_attribute("indent", "0")
        .with_attribute("outline", "1")
        .with_attribute("outlineData", "1")
        .with_attribute("multipleFieldFilters", "0")
        .with_child(
            OpenXmlElement::new("x", X, "location")
                .with_attribute("ref", location_ref)
                .with_attribute("firstHeaderRow", "1")
                .with_attribute("firstDataRow", "1")
                .with_attribute("firstDataCol", "1"),
        )
        .with_child(
            OpenXmlElement::new("x", X, "pivotFields")
                .with_attribute("count", field_count.to_string())
                .with_children(pivot_fields),
        )
        .with_child(
            OpenXmlElement::new("x", X, "rowFields")
                .with_attribute("count", "1")
                .with_child(
                    OpenXmlElement::new("x", X, "field")
                        .with_attribute("x", row_field.to_string()),
                ),
        )
        .with_child(
            OpenXmlElement::new("x", X, "dataFields")
                .with_attribute("count", "1")
                .with_child(
                    OpenXmlElement::new("x", X, "dataField")
                        .with_attribute(
                            "name",
                            format!("Sum of {}", field_names.get(data_field as usize).unwrap_or(&"Data")),
                        )
                        .with_attribute("fld", data_field.to_string())
                        .with_attribute("baseField", "0")
                        .with_attribute("baseItem", "0"),
                ),
        )
}

/// `x:pivotCaches` entry for the workbook.
pub fn workbook_pivot_cache(cache_id: u32, rel_id: &str) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "pivotCache")
        .with_attribute("cacheId", cache_id.to_string())
        .with_attribute_ns("r", R, "id", rel_id)
}

/// `x:pivotCaches` container.
pub fn workbook_pivot_caches(
    caches: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("x", X, "pivotCaches").with_children(caches)
}
