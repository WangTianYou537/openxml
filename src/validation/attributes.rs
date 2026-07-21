//! Lightweight attribute-level validation helpers.
//!
//! Not a full XSD type system — covers common simple-type checks used when
//! wiring schema attribute metadata to runtime values.

use super::ValidationError;
use crate::element::OpenXmlElement;
use crate::simple_types::{
    BooleanValue, DoubleValue, Int32Value, OnOffValue, OpenXmlSimpleType, UInt32Value,
};

/// Expected simple type for an attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeType {
    String,
    Boolean,
    OnOff,
    Int32,
    UInt32,
    Double,
    /// Non-empty hex string (rsid-style).
    HexBinary,
}

/// Validate a single attribute value against a declared type.
pub fn validate_attribute_value(
    path: &str,
    attr_name: &str,
    value: &str,
    ty: AttributeType,
) -> Option<ValidationError> {
    let ok = match ty {
        AttributeType::String => true,
        AttributeType::Boolean => BooleanValue::from_inner_text(value).is_some(),
        AttributeType::OnOff => OnOffValue::from_inner_text(value).is_some(),
        AttributeType::Int32 => Int32Value::from_inner_text(value).is_some(),
        AttributeType::UInt32 => UInt32Value::from_inner_text(value).is_some(),
        AttributeType::Double => DoubleValue::from_inner_text(value).is_some(),
        AttributeType::HexBinary => {
            !value.is_empty()
                && value.bytes().all(|b| b.is_ascii_hexdigit())
                && value.len() % 2 == 0
        }
    };
    if ok {
        None
    } else {
        Some(ValidationError {
            path: format!("{path}/@{attr_name}"),
            message: format!(
                "attribute `{attr_name}` value `{value}` is not a valid {ty:?}"
            ),
        })
    }
}

/// Rule for validating one attribute on an element.
#[derive(Debug, Clone, Copy)]
pub struct AttributeRule {
    pub local_name: &'static str,
    pub required: bool,
    pub ty: AttributeType,
}

/// Validate attributes on `element` against `rules`.
pub fn validate_attributes(
    element: &OpenXmlElement,
    rules: &[AttributeRule],
    path: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in rules {
        let value = element
            .get_attribute(rule.local_name)
            .or_else(|| {
                // Also try qname-less match against full attribute local names
                element
                    .attributes
                    .iter()
                    .find(|a| a.local_name == rule.local_name)
                    .map(|a| a.value.as_str())
            });
        match value {
            None if rule.required => {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!(
                        "missing required attribute `{}` on `<{}>`",
                        rule.local_name, element.local_name
                    ),
                });
            }
            Some(v) => {
                if let Some(err) = validate_attribute_value(path, rule.local_name, v, rule.ty) {
                    errors.push(err);
                }
            }
            None => {}
        }
    }
    errors
}

/// Built-in attribute rules for common WordprocessingML leaves.
pub mod word {
    use super::*;

    pub fn on_off_val() -> &'static [AttributeRule] {
        &[AttributeRule {
            local_name: "val",
            required: false,
            ty: AttributeType::OnOff,
        }]
    }

    /// Validate a `w:b` / `w:i` style on/off element.
    pub fn validate_on_off_element(el: &OpenXmlElement, path: &str) -> Vec<ValidationError> {
        validate_attributes(el, on_off_val(), path)
    }
}

/// Validate that a numeric attribute is within `[min, max]` (inclusive).
pub fn validate_attribute_range(
    path: &str,
    attr_name: &str,
    value: &str,
    min: f64,
    max: f64,
) -> Option<ValidationError> {
    match value.parse::<f64>() {
        Ok(n) if n >= min && n <= max => None,
        Ok(n) => Some(ValidationError {
            path: format!("{path}/@{attr_name}"),
            message: format!(
                "attribute `{attr_name}` value `{n}` is outside range [{min}, {max}]"
            ),
        }),
        Err(_) => Some(ValidationError {
            path: format!("{path}/@{attr_name}"),
            message: format!("attribute `{attr_name}` value `{value}` is not numeric"),
        }),
    }
}

/// Built-in numeric range rules for common SpreadsheetML attributes
/// (subset of Schematron numeric constraints).
pub fn validate_spreadsheet_attribute_ranges(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for el in root.descendants() {
        match el.local_name.as_str() {
            "sheet" => {
                if let Some(v) = attr_value(el, "sheetId") {
                    if let Some(e) =
                        validate_attribute_range("x:sheet", "sheetId", v, 1.0, 65534.0)
                    {
                        errors.push(e);
                    }
                }
            }
            "customWorkbookView" => {
                if let Some(v) = attr_value(el, "tabRatio") {
                    if let Some(e) = validate_attribute_range(
                        "x:customWorkbookView",
                        "tabRatio",
                        v,
                        0.0,
                        1000.0,
                    ) {
                        errors.push(e);
                    }
                }
                if let Some(v) = attr_value(el, "activeSheetId") {
                    if let Some(e) = validate_attribute_range(
                        "x:customWorkbookView",
                        "activeSheetId",
                        v,
                        1.0,
                        65534.0,
                    ) {
                        errors.push(e);
                    }
                }
            }
            "functionGroups" => {
                if let Some(v) = attr_value(el, "builtInGroupCount") {
                    if let Some(e) = validate_attribute_range(
                        "x:functionGroups",
                        "builtInGroupCount",
                        v,
                        0.0,
                        255.0,
                    ) {
                        errors.push(e);
                    }
                }
            }
            _ => {}
        }
    }
    errors
}

fn attr_value<'a>(el: &'a OpenXmlElement, name: &str) -> Option<&'a str> {
    el.get_attribute(name)
        .or_else(|| el.get_attribute_qname(&format!("x:{name}")))
        .or_else(|| {
            el.attributes
                .iter()
                .find(|a| a.local_name == name)
                .map(|a| a.value.as_str())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;

    #[test]
    fn int_attr_ok() {
        assert!(validate_attribute_value("/x", "id", "42", AttributeType::Int32).is_none());
        assert!(validate_attribute_value("/x", "id", "nope", AttributeType::Int32).is_some());
    }

    #[test]
    fn required_attr() {
        let el = OpenXmlElement::w("x");
        let rules = [AttributeRule {
            local_name: "id",
            required: true,
            ty: AttributeType::String,
        }];
        let errs = validate_attributes(&el, &rules, "/x");
        assert_eq!(errs.len(), 1);
    }

    #[test]
    fn sheet_id_range() {
        let sheet = OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "sheet",
        )
        .with_attribute("name", "S")
        .with_attribute("sheetId", "70000");
        let root = OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "workbook",
        )
        .with_child(sheet);
        let errs = validate_spreadsheet_attribute_ranges(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("outside range")),
            "{errs:?}"
        );
    }
}
