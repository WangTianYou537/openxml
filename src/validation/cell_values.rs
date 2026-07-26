//! Spreadsheet cell value semantic validation (C# `CellType` `IValidator`).

use super::{ValidationError, ValidationErrorType};
use crate::element::OpenXmlElement;
use crate::generated::spreadsheetml_2006_main::CellValues;

/// C# `CellType.IValidator.Validate` — when `c@t` is boolean/date/number, the
/// child `v` text must parse as that type (`Sem_CellValue` on failure).
pub fn validate_spreadsheet_cell_values(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    walk(root, "x:worksheet", &mut errors);
    errors
}

fn walk(element: &OpenXmlElement, path: &str, errors: &mut Vec<ValidationError>) {
    if is_cell_element(element) {
        if let Some(error) = validate_one_cell(element, path) {
            errors.push(error);
        }
    }
    for (i, child) in element.children.iter().enumerate() {
        if child.is_misc_node() {
            continue;
        }
        let child_path = format!("{path}/{}[{}]", child.qualified_name(), i + 1);
        walk(child, &child_path, errors);
    }
}

fn is_cell_element(element: &OpenXmlElement) -> bool {
    element.local_name == "c"
        && (element.prefix == "x"
            || element.namespace_uri
                == "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
            || element.prefix.is_empty())
}

fn cell_value_text(cell: &OpenXmlElement) -> Option<&str> {
    cell.children
        .iter()
        .find(|c| c.local_name == "v" && !c.is_misc_node())
        .and_then(|v| v.text.as_deref())
}

fn data_type(cell: &OpenXmlElement) -> Option<CellValues> {
    let t = cell
        .get_attribute("t")
        .or_else(|| cell.get_attribute_qname("x:t"))?;
    CellValues::from_str(t)
}

fn try_boolean(text: &str) -> bool {
    matches!(
        text.trim(),
        "0" | "1" | "true" | "false" | "TRUE" | "FALSE"
    )
}

fn try_number(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() {
        return false;
    }
    // Integer or floating (including scientific) — reject NaN/Inf.
    if let Ok(v) = t.parse::<f64>() {
        return v.is_finite();
    }
    false
}

fn try_date(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() {
        return false;
    }
    // ISO-8601-ish: YYYY-MM-DD or YYYY-MM-DDThh:mm:ss(.fff)(Z|±hh:mm)?
    // Also accept pure serial numbers (Excel date serials as numbers).
    if try_number(t) {
        return true;
    }
    // Minimal ISO date / date-time check.
    let bytes = t.as_bytes();
    if bytes.len() < 10 {
        return false;
    }
    // YYYY-MM-DD
    let date_ok = bytes[0..4].iter().all(|b| b.is_ascii_digit())
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(|b| b.is_ascii_digit())
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(|b| b.is_ascii_digit());
    if !date_ok {
        return false;
    }
    if bytes.len() == 10 {
        return true;
    }
    if bytes[10] != b'T' && bytes[10] != b' ' {
        return false;
    }
    // Require at least hh:mm
    t.len() >= 16
}

fn validate_one_cell(cell: &OpenXmlElement, path: &str) -> Option<ValidationError> {
    let data_type = data_type(cell)?;
    let value_text = cell_value_text(cell)?;
    let ok = match data_type {
        CellValues::Boolean => try_boolean(value_text),
        CellValues::Number => try_number(value_text),
        CellValues::Date => try_date(value_text),
        // Error / SharedString / String / InlineString: no lexical check here.
        _ => true,
    };
    if ok {
        return None;
    }
    Some(
        ValidationError::with_id(
            path,
            "Sem_CellValue",
            format!(
                "Cell contents have invalid value '{value_text}' for type '{}'.",
                data_type.as_str()
            ),
        )
        .with_error_type(ValidationErrorType::Semantic),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spreadsheet::{cell_bool, cell_number};

    fn cell_with(t: &str, v: &str) -> OpenXmlElement {
        let mut c = OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "c",
        );
        c.set_attribute("r", "A1");
        c.set_attribute("t", t);
        c.append_child(
            OpenXmlElement::new(
                "x",
                "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                "v",
            )
            .with_text(v),
        );
        c
    }

    #[test]
    fn boolean_cell_accepts_0_1() {
        let c = cell_bool("A1", true);
        assert!(validate_spreadsheet_cell_values(&c).is_empty());
    }

    #[test]
    fn boolean_cell_rejects_text() {
        let c = cell_with("b", "maybe");
        let errs = validate_spreadsheet_cell_values(&c);
        assert!(
            errs.iter().any(|e| e.id() == Some("Sem_CellValue")),
            "{errs:?}"
        );
    }

    #[test]
    fn number_cell_rejects_text() {
        let c = cell_with("n", "not-a-number");
        let errs = validate_spreadsheet_cell_values(&c);
        assert!(
            errs.iter().any(|e| e.id() == Some("Sem_CellValue")),
            "{errs:?}"
        );
    }

    #[test]
    fn number_cell_accepts_float() {
        let c = cell_number("B2", 3.14);
        // cell_number may omit t="n" (default number) — then validator skips.
        // Force t="n".
        let mut c = c;
        c.set_attribute("t", "n");
        assert!(validate_spreadsheet_cell_values(&c).is_empty(), "{:?}", validate_spreadsheet_cell_values(&c));
    }

    #[test]
    fn date_cell_accepts_iso() {
        let c = cell_with("d", "2020-01-02T00:00:00");
        assert!(validate_spreadsheet_cell_values(&c).is_empty());
    }

    #[test]
    fn date_cell_rejects_garbage() {
        let c = cell_with("d", "not-a-date");
        let errs = validate_spreadsheet_cell_values(&c);
        assert!(
            errs.iter().any(|e| e.id() == Some("Sem_CellValue")),
            "{errs:?}"
        );
    }

    #[test]
    fn shared_string_not_checked() {
        let c = cell_with("s", "0");
        assert!(validate_spreadsheet_cell_values(&c).is_empty());
    }
}
