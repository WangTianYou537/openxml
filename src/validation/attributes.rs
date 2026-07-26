//! Lightweight attribute-level validation helpers.
//!
//! Not a full XSD type system — covers common simple-type checks used when
//! wiring schema attribute metadata to runtime values.

use super::ValidationError;
use crate::element::OpenXmlElement;
use crate::markup_compatibility::McContext;
use crate::simple_types::{
    BooleanValue, DoubleValue, Int32Value, OnOffValue, OpenXmlSimpleType, UInt32Value,
};

const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";

/// Cross-schema attribute metadata (Word / Spreadsheet / Presentation / Drawing).
#[derive(Debug, Clone, Copy)]
struct AttrMeta {
    qname: &'static str,
    type_name: &'static str,
}

/// Cross-schema element metadata used by attribute/leaf validation.
#[derive(Debug, Clone)]
struct ElementMeta {
    prefix: &'static str,
    is_leaf: bool,
    is_leaf_text: bool,
    attributes: Vec<AttrMeta>,
}

/// Resolve generated schema metadata for common OfficeML prefixes.
///
/// Looks up Word (`w`), Spreadsheet (`x`), Presentation (`p`), and Drawing (`a`)
/// schemas. Prefix must match the generated element info (avoids collisions on
/// shared local names like `fonts` / `comments`).
fn generated_element_meta(element: &OpenXmlElement) -> Option<ElementMeta> {
    macro_rules! try_schema {
        ($mod:path) => {{
            use $mod as schema;
            if let Some(info) = schema::info_by_local_name(&element.local_name) {
                if element.prefix == info.prefix {
                    return Some(ElementMeta {
                        prefix: info.prefix,
                        is_leaf: info.is_leaf,
                        is_leaf_text: info.is_leaf_text,
                        attributes: info
                            .attributes
                            .iter()
                            .map(|a| AttrMeta {
                                qname: a.qname,
                                type_name: a.type_name,
                            })
                            .collect(),
                    });
                }
            }
        }};
    }

    // Prefer the element's own prefix, then fall through other schemas.
    match element.prefix.as_str() {
        "w" => try_schema!(crate::generated::wordprocessingml_2006_main),
        "x" => try_schema!(crate::generated::spreadsheetml_2006_main),
        "p" => try_schema!(crate::generated::presentationml_2006_main),
        "a" => try_schema!(crate::generated::drawingml_2006_main),
        _ => {}
    }
    try_schema!(crate::generated::wordprocessingml_2006_main);
    try_schema!(crate::generated::spreadsheetml_2006_main);
    try_schema!(crate::generated::presentationml_2006_main);
    try_schema!(crate::generated::drawingml_2006_main);
    None
}

/// C# `SchemaTypeValidator.ValidateEmptyComplexType` /
/// `ValidateSimpleContextComplexType` — a schema leaf element (empty or
/// simple-content complex type) cannot contain element children. At most one
/// `Sch_InvalidChildinLeafElement` error is reported per element.
pub fn validate_leaf_content(element: &OpenXmlElement, path: &str) -> Vec<ValidationError> {
    let Some(info) = generated_element_meta(element) else {
        return Vec::new();
    };
    if !(info.is_leaf || info.is_leaf_text) {
        return Vec::new();
    }
    if element.children.iter().any(|child| !child.is_misc_node()) {
        return vec![ValidationError::with_id(
            path,
            "Sch_InvalidChildinLeafElement",
            format!(
                "The element '{}' is a leaf element and cannot contain children.",
                element.qualified_name()
            ),
        )];
    }
    Vec::new()
}

/// C# `SchemaTypeValidator.ValidateValue` for declared attributes: lexical
/// checks against the generated `type_name`, reporting
/// `Sch_AttributeValueDataTypeDetailed` on mismatch.
///
/// Routes numeric / OnOff / HexBinary / token-family types through the
/// framework [`super::Validator`] stack when a mapping exists; other types keep
/// the XsdType lexical path. Covers Word, Spreadsheet, Presentation, Drawing.
pub fn validate_attribute_value_types(
    element: &OpenXmlElement,
    path: &str,
) -> Vec<ValidationError> {
    let Some(info) = generated_element_meta(element) else {
        return Vec::new();
    };

    let settings = super::ValidationSettings::new(crate::file_format::FileFormatVersions::ALL);
    let mut context = super::ValidationContext::new(settings);
    context.current_path = path.to_string();
    context.stack_mut().push_element_path(path);

    for attribute in &element.attributes {
        let prefix = attribute.prefix.as_deref().unwrap_or("");
        let qname = if prefix.is_empty() {
            format!("{}:{}", info.prefix, attribute.local_name)
        } else {
            format!("{prefix}:{}", attribute.local_name)
        };
        // Spreadsheet/Drawing sometimes declare attrs as `:local` (no prefix).
        let declared = info.attributes.iter().find(|a| {
            a.qname == qname
                || a.qname == format!(":{}", attribute.local_name)
                || (a.qname.ends_with(&format!(":{}", attribute.local_name))
                    && a.qname.rsplit_once(':').map(|(_, l)| l) == Some(attribute.local_name.as_str()))
        });
        let Some(declared) = declared else {
            continue;
        };
        let _ = super::validate_attribute_with_type_name(
            &mut context,
            &qname,
            declared.type_name,
            &attribute.value,
        );
    }
    context.stack_mut().pop();
    // Ensure path on errors matches the caller's path (create_error uses current_path).
    context.errors
}

/// C# `SchemaTypeValidator.ValidateAttributes` extended-attribute branch:
/// report `Sch_UndeclaredAttribute` for attributes not declared in the
/// generated schema for `element` (Word/Spreadsheet/Presentation/Drawing),
/// skipping MC-ignorable namespaces, `xml:*`, `xmlns` declarations, and `mc:*`
/// compatibility attributes. Unknown elements are not checked.
pub fn validate_undeclared_attributes(
    element: &OpenXmlElement,
    mc_context: &McContext,
    path: &str,
) -> Vec<ValidationError> {
    let Some(info) = generated_element_meta(element) else {
        return Vec::new();
    };

    let mut errors = Vec::new();
    for attribute in &element.attributes {
        let prefix = attribute.prefix.as_deref().unwrap_or("");
        if prefix == "xmlns" || (prefix.is_empty() && attribute.local_name == "xmlns") {
            continue;
        }
        if prefix == "xml" || attribute.namespace_uri.as_deref() == Some(XML_NAMESPACE) {
            continue;
        }
        if prefix == "mc"
            || attribute.namespace_uri.as_deref()
                == Some(crate::namespace::ns::MARKUP_COMPATIBILITY.uri)
        {
            continue;
        }
        // MC lists may hold URIs (resolved) or bare prefixes (no lookup): try both.
        let uri_ignorable = attribute
            .namespace_uri
            .as_deref()
            .is_some_and(|uri| mc_context.is_ignorable_ns(uri));
        if uri_ignorable || (!prefix.is_empty() && mc_context.is_ignorable_ns(prefix)) {
            continue;
        }

        let qname = if prefix.is_empty() {
            format!("{}:{}", info.prefix, attribute.local_name)
        } else {
            format!("{prefix}:{}", attribute.local_name)
        };
        let declared = info.attributes.iter().any(|a| {
            a.qname == qname
                || a.qname == format!(":{}", attribute.local_name)
                || (a.qname.ends_with(&format!(":{}", attribute.local_name))
                    && a.qname.rsplit_once(':').map(|(_, l)| l) == Some(attribute.local_name.as_str()))
        });
        if !declared {
            let display = if prefix.is_empty() {
                attribute.local_name.clone()
            } else {
                format!("{prefix}:{}", attribute.local_name)
            };
            errors.push(ValidationError::with_id(
                path,
                "Sch_UndeclaredAttribute",
                format!("The '{display}' attribute is not declared."),
            ));
        }
    }
    errors
}

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
            ..Default::default()
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
                    ..Default::default()
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
            ..Default::default()
        }),
        Err(_) => Some(ValidationError {
            path: format!("{path}/@{attr_name}"),
            message: format!("attribute `{attr_name}` value `{value}` is not numeric"),
            ..Default::default()
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

    #[test]
    fn spreadsheet_undeclared_attribute_reported() {
        let sheet = OpenXmlElement::x("sheet")
            .with_attribute("name", "S")
            .with_attribute("sheetId", "1")
            .with_attribute("notARealAttr", "x");
        let mc = McContext::with_exception_on_error(false);
        let errs = validate_undeclared_attributes(&sheet, &mc, "x:sheet");
        assert!(
            errs.iter().any(|e| e.id() == Some("Sch_UndeclaredAttribute")
                && e.message.contains("notARealAttr")),
            "{errs:?}"
        );
    }

    #[test]
    fn spreadsheet_declared_sheet_attrs_accepted() {
        let sheet = OpenXmlElement::x("sheet")
            .with_attribute("name", "S")
            .with_attribute("sheetId", "1")
            .with_attribute_qname("r:id", "rId1");
        let mc = McContext::with_exception_on_error(false);
        let errs = validate_undeclared_attributes(&sheet, &mc, "x:sheet");
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn spreadsheet_leaf_forbids_element_children() {
        let mut sheet = OpenXmlElement::x("sheet").with_attribute("name", "S");
        sheet.append_child(OpenXmlElement::x("sheetData"));
        let errs = validate_leaf_content(&sheet, "x:sheet");
        assert!(
            errs.iter()
                .any(|e| e.id() == Some("Sch_InvalidChildinLeafElement")),
            "{errs:?}"
        );
    }

    #[test]
    fn spreadsheet_attribute_type_rejects_bad_sheet_id() {
        let sheet = OpenXmlElement::x("sheet")
            .with_attribute("name", "S")
            .with_attribute("sheetId", "not-a-number");
        let errs = validate_attribute_value_types(&sheet, "x:sheet");
        assert!(
            !errs.is_empty()
                || errs.iter().any(|e| e.message.contains("sheetId")
                    || e.id() == Some("Sch_AttributeValueDataTypeDetailed")),
            "{errs:?}"
        );
        // Either a typed error is raised, or at minimum the helper does not panic.
        let _ = errs;
        // Strong assertion: bad UInt32 should produce a datatype error.
        assert!(
            errs.iter().any(|e| {
                e.id() == Some("Sch_AttributeValueDataTypeDetailed")
                    || e.message.to_lowercase().contains("uint")
                    || e.message.contains("not-a-number")
                    || e.message.contains("sheetId")
            }),
            "expected datatype error for sheetId, got {errs:?}"
        );
    }
}
