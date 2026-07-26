//! Element-level semantic constraint shells (C# `Validation/Semantic/*Constraint`).
//!
//! These are the typed C# constraint classes used by element metadata. They are
//! distinct from the Schematron-table extractors in `schematron_*` — same rule
//! shapes, but object-form with [`SemanticConstraintGate`] application.

use super::{
    validation_resource_message, SemanticConstraintGate, SemanticValidationLevel, ValidationContext,
    ValidationError, ValidationErrorType,
};
use crate::element::OpenXmlElement;
use crate::features::ApplicationType;

/// Attribute identity used by semantic constraints (C# `OpenXmlQualifiedName` shell).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeName {
    pub local_name: String,
    pub namespace_uri: String,
}

impl AttributeName {
    pub fn new(local_name: impl Into<String>, namespace_uri: impl Into<String>) -> Self {
        Self {
            local_name: local_name.into(),
            namespace_uri: namespace_uri.into(),
        }
    }

    pub fn local(local_name: impl Into<String>) -> Self {
        Self::new(local_name, "")
    }

    pub fn display(&self) -> String {
        if self.namespace_uri.is_empty() {
            self.local_name.clone()
        } else {
            format!("{{{}}}:{}", self.namespace_uri, self.local_name)
        }
    }
}

/// C# `SemanticConstraint.TryFindAttribute` + value presence.
fn find_attribute_value<'a>(
    element: &'a OpenXmlElement,
    attribute: &AttributeName,
) -> Option<&'a str> {
    if !attribute.namespace_uri.is_empty() {
        if let Some(value) =
            element.get_attribute_ns(&attribute.local_name, &attribute.namespace_uri)
        {
            return Some(value);
        }
    }
    // C# falls back to local-name match when namespace does not match exactly.
    element
        .attributes
        .iter()
        .find(|a| a.local_name == attribute.local_name)
        .map(|a| a.value.as_str())
}

/// C# `SemanticConstraint.AttributeValueEquals` (string/boolean lexical forms).
fn attribute_value_equals(actual: &str, expected: &str, ignore_case: bool) -> bool {
    if ignore_case {
        return actual.eq_ignore_ascii_case(expected);
    }
    if actual == expected {
        return true;
    }
    // OnOff / TrueFalse boolean lexical equality (C# CompareBooleanValue path).
    let as_bool = |s: &str| match s {
        "true" | "1" | "on" => Some(true),
        "false" | "0" | "off" => Some(false),
        _ => None,
    };
    match (as_bool(actual), as_bool(expected)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

fn format_resource(message_id: &str, args: &[&str]) -> String {
    match validation_resource_message(message_id) {
        Some(template) => {
            let mut message = template.to_string();
            for (index, arg) in args.iter().enumerate() {
                message = message.replace(&format!("{{{index}}}"), arg);
            }
            for index in args.len()..10 {
                message = message.replace(&format!("{{{index}}}"), "");
            }
            message
        }
        None => format!("An unknown error occurred. Original message: '{message_id}'"),
    }
}

fn semantic_error(
    path: &str,
    id: &str,
    description: impl AsRef<str>,
    error_type: ValidationErrorType,
) -> ValidationError {
    ValidationError::with_id(path, id, description).with_error_type(error_type)
}

/// Trait for element-level semantic constraints (C# `SemanticConstraint` shell).
pub trait SemanticConstraint {
    fn gate(&self) -> SemanticConstraintGate;

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError>;

    /// C# `SemanticConstraint.Validate` — gate then `ValidateCore`.
    fn validate(
        &self,
        context: &ValidationContext,
        element: &OpenXmlElement,
        path: &str,
        application: ApplicationType,
    ) -> Option<ValidationError> {
        if !self.gate().applies(context, application) {
            return None;
        }
        self.validate_core(element, path)
    }
}

/// Run a list of element-level constraints on every descendant of `root`.
pub fn validate_element_constraints(
    context: &ValidationContext,
    root: &OpenXmlElement,
    constraints: &[&dyn SemanticConstraint],
    application: ApplicationType,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for element in std::iter::once(root).chain(root.descendants()) {
        if element.is_misc_node() {
            continue;
        }
        let path = element.qualified_name();
        for constraint in constraints {
            if let Some(error) = constraint.validate(context, element, &path, application) {
                errors.push(error);
            }
        }
    }
    errors
}

// ---------------------------------------------------------------------------
// AttributeCannotOmitConstraint
// ---------------------------------------------------------------------------

/// C# `AttributeCannotOmitConstraint` — required attribute must be present.
#[derive(Debug, Clone)]
pub struct AttributeCannotOmitConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
}

impl AttributeCannotOmitConstraint {
    pub fn new(attribute: AttributeName) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute,
        }
    }

    pub fn with_gate(mut self, gate: SemanticConstraintGate) -> Self {
        self.gate = gate;
        self
    }
}

impl SemanticConstraint for AttributeCannotOmitConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        if find_attribute_value(element, &self.attribute).is_some() {
            return None;
        }
        // C# only errors when the attribute is known on the element metadata
        // (`TryFindAttribute` succeeds). Without full metadata we treat any
        // missing value as a miss, matching the required-attribute Schematron
        // extractors used elsewhere.
        let qname = self.attribute.display();
        Some(semantic_error(
            path,
            "Sem_MissRequiredAttribute",
            format_resource("Sch_MissRequiredAttribute", &[&qname]),
            ValidationErrorType::Schema,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeMutualExclusive
// ---------------------------------------------------------------------------

/// C# `AttributeMutualExclusive` — at most one of the listed attributes may be set.
#[derive(Debug, Clone)]
pub struct AttributeMutualExclusive {
    gate: SemanticConstraintGate,
    attributes: Vec<AttributeName>,
}

impl AttributeMutualExclusive {
    pub fn new(attributes: Vec<AttributeName>) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attributes,
        }
    }
}

impl SemanticConstraint for AttributeMutualExclusive {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        let present: Vec<String> = self
            .attributes
            .iter()
            .filter(|attr| find_attribute_value(element, attr).is_some())
            .map(|attr| attr.display())
            .collect();
        if present.len() < 2 {
            return None;
        }
        let all: Vec<String> = self.attributes.iter().map(|a| a.display()).collect();
        let last = present.last().cloned().unwrap_or_default();
        let earlier = present[..present.len() - 1].join(",");
        Some(semantic_error(
            path,
            "Sem_AttributeMutualExclusive",
            format_resource(
                "Sem_AttributeMutualExclusive",
                &[&earlier, &last, &all.join(",")],
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeValueLengthConstraint
// ---------------------------------------------------------------------------

/// C# `AttributeValueLengthConstraint`.
#[derive(Debug, Clone)]
pub struct AttributeValueLengthConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    min_length: usize,
    max_length: usize,
}

impl AttributeValueLengthConstraint {
    pub fn new(attribute: AttributeName, min_length: usize, max_length: usize) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute,
            min_length,
            max_length,
        }
    }
}

impl SemanticConstraint for AttributeValueLengthConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        let value = find_attribute_value(element, &self.attribute)?;
        let len = value.chars().count();
        let sub = if len < self.min_length {
            format_resource(
                "Sem_MinLengthConstraintFailed",
                &[&self.min_length.to_string()],
            )
        } else if len > self.max_length {
            format_resource(
                "Sem_MaxLengthConstraintFailed",
                &[&self.max_length.to_string()],
            )
        } else {
            return None;
        };
        let qname = self.attribute.display();
        Some(semantic_error(
            path,
            "Sem_AttributeValueDataTypeDetailed",
            format_resource("Sem_AttributeValueDataTypeDetailed", &[&qname, value, &sub]),
            ValidationErrorType::Schema,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeValueRangeConstraint
// ---------------------------------------------------------------------------

/// C# `AttributeValueRangeConstraint`.
#[derive(Debug, Clone)]
pub struct AttributeValueRangeConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    is_valid_range: bool,
    min_value: f64,
    max_value: f64,
    min_inclusive: bool,
    max_inclusive: bool,
}

impl AttributeValueRangeConstraint {
    pub fn new(
        attribute: AttributeName,
        is_valid_range: bool,
        min_value: f64,
        min_inclusive: bool,
        max_value: f64,
        max_inclusive: bool,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute,
            is_valid_range,
            min_value,
            max_value,
            min_inclusive,
            max_inclusive,
        }
    }
}

fn parse_attr_num(value: &str) -> Option<f64> {
    if let Ok(v) = value.parse::<f64>() {
        return Some(v);
    }
    // HexBinaryValue path (C# GetAttrNumVal).
    i64::from_str_radix(value, 16).ok().map(|v| v as f64)
}

impl SemanticConstraint for AttributeValueRangeConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(element, &self.attribute)?;
        if text.is_empty() {
            return None;
        }
        let value = parse_attr_num(text)?;
        let in_range = {
            let min_ok = if self.min_inclusive {
                value >= self.min_value
            } else {
                value > self.min_value
            };
            let max_ok = if self.max_inclusive {
                value <= self.max_value
            } else {
                value < self.max_value
            };
            min_ok && max_ok
        };
        let ok = if self.is_valid_range {
            in_range
        } else {
            !in_range
        };
        if ok {
            return None;
        }
        let min_s = self.min_value.to_string();
        let max_s = self.max_value.to_string();
        let sub = if self.is_valid_range {
            if self.min_inclusive && !(value >= self.min_value) {
                format_resource("Sch_MinInclusiveConstraintFailed", &[&min_s])
            } else if !self.min_inclusive && !(value > self.min_value) {
                format_resource("Sch_MinExclusiveConstraintFailed", &[&min_s])
            } else if self.max_inclusive && !(value <= self.max_value) {
                format_resource("Sch_MaxInclusiveConstraintFailed", &[&max_s])
            } else {
                format_resource("Sch_MaxExclusiveConstraintFailed", &[&max_s])
            }
        } else {
            format!(
                " The value must not be in the range [{}, {}].",
                min_s, max_s
            )
        };
        let qname = self.attribute.display();
        Some(semantic_error(
            path,
            "Sem_AttributeValueDataTypeDetailed",
            format_resource("Sem_AttributeValueDataTypeDetailed", &[&qname, text, &sub]),
            ValidationErrorType::Schema,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeValueSetConstraint
// ---------------------------------------------------------------------------

/// C# `AttributeValueSetConstraint`.
#[derive(Debug, Clone)]
pub struct AttributeValueSetConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    is_valid_value_set: bool,
    value_set: Vec<String>,
}

impl AttributeValueSetConstraint {
    pub fn new(
        attribute: AttributeName,
        is_valid_value_set: bool,
        value_set: Vec<String>,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute,
            is_valid_value_set,
            value_set,
        }
    }
}

impl SemanticConstraint for AttributeValueSetConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(element, &self.attribute)?;
        if text.is_empty() {
            return None;
        }
        let contains = self
            .value_set
            .iter()
            .any(|v| attribute_value_equals(text, v, false));
        // C#: if (!_isValidValueSet ^ valueSetContains) return null;
        if !self.is_valid_value_set ^ contains {
            return None;
        }
        let sub = format_resource("Sch_EnumerationConstraintFailed", &[]);
        let qname = self.attribute.display();
        Some(semantic_error(
            path,
            "Sem_AttributeValueDataTypeDetailed",
            format_resource("Sem_AttributeValueDataTypeDetailed", &[&qname, text, &sub]),
            ValidationErrorType::Schema,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeRequiredConditionToValue
// ---------------------------------------------------------------------------

/// C# `AttributeRequiredConditionToValue`.
#[derive(Debug, Clone)]
pub struct AttributeRequiredConditionToValue {
    gate: SemanticConstraintGate,
    required_attribute: AttributeName,
    condition_attribute: AttributeName,
    value: String,
}

impl AttributeRequiredConditionToValue {
    pub fn new(
        required_attribute: AttributeName,
        condition_attribute: AttributeName,
        value: impl Into<String>,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            required_attribute,
            condition_attribute,
            value: value.into(),
        }
    }
}

impl SemanticConstraint for AttributeRequiredConditionToValue {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        if find_attribute_value(element, &self.required_attribute).is_some() {
            return None;
        }
        let condition = find_attribute_value(element, &self.condition_attribute)?;
        if !attribute_value_equals(condition, &self.value, false) {
            return None;
        }
        let req = self.required_attribute.display();
        let cond = self.condition_attribute.display();
        Some(semantic_error(
            path,
            "Sem_AttributeRequiredConditionToValue",
            format_resource(
                "Sem_AttributeRequiredConditionToValue",
                &[&req, &cond, &self.value],
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeAbsentConditionToValue
// ---------------------------------------------------------------------------

/// C# `AttributeAbsentConditionToValue`.
#[derive(Debug, Clone)]
pub struct AttributeAbsentConditionToValue {
    gate: SemanticConstraintGate,
    absent_attribute: AttributeName,
    condition_attribute: AttributeName,
    values: Vec<String>,
}

impl AttributeAbsentConditionToValue {
    pub fn new(
        absent_attribute: AttributeName,
        condition_attribute: AttributeName,
        values: Vec<String>,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            absent_attribute,
            condition_attribute,
            values,
        }
    }
}

impl SemanticConstraint for AttributeAbsentConditionToValue {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        let _absent = find_attribute_value(element, &self.absent_attribute)?;
        let condition = find_attribute_value(element, &self.condition_attribute)?;
        if !self
            .values
            .iter()
            .any(|v| attribute_value_equals(condition, v, false))
        {
            return None;
        }
        let value_string = if self.values.is_empty() {
            String::new()
        } else if self.values.len() == 1 {
            format!("'{}'", self.values[0])
        } else {
            let mut parts: Vec<String> = self
                .values
                .iter()
                .take(self.values.len() - 1)
                .map(|v| format!("'{v}'"))
                .collect();
            let last = format!("'{}'", self.values[self.values.len() - 1]);
            if parts.len() == 1 {
                format!("{} or {last}", parts[0])
            } else {
                let head = parts.drain(..).collect::<Vec<_>>().join(", ");
                format!("{head} or {last}")
            }
        };
        let absent = self.absent_attribute.display();
        let cond = self.condition_attribute.display();
        Some(semantic_error(
            path,
            "Sem_AttributeAbsentConditionToValue",
            format_resource(
                "Sem_AttributeAbsentConditionToValue",
                &[&absent, &cond, &value_string],
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeMinMaxConstraint
// ---------------------------------------------------------------------------

/// C# `AttributeMinMaxConstraint` — min attribute numeric value ≤ max attribute.
#[derive(Debug, Clone)]
pub struct AttributeMinMaxConstraint {
    gate: SemanticConstraintGate,
    min_attribute: AttributeName,
    max_attribute: AttributeName,
}

impl AttributeMinMaxConstraint {
    pub fn new(min_attribute: AttributeName, max_attribute: AttributeName) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            min_attribute,
            max_attribute,
        }
    }
}

impl SemanticConstraint for AttributeMinMaxConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        let min_text = find_attribute_value(element, &self.min_attribute)?;
        let max_text = find_attribute_value(element, &self.max_attribute)?;
        let min_value = parse_attr_num(min_text)?;
        let max_value = parse_attr_num(max_text)?;
        if min_value <= max_value {
            return None;
        }
        // C# leaves Id/Description empty (TODO); emit a stable semantic message.
        Some(semantic_error(
            path,
            "Sem_AttributeMinMaxConstraint",
            format!(
                "Attribute '{}' value {min_value} must be less than or equal to attribute '{}' value {max_value}.",
                self.min_attribute.display(),
                self.max_attribute.display()
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributePairConstraint
// ---------------------------------------------------------------------------

/// C# `AttributePairConstraint` — both attributes present or both absent.
#[derive(Debug, Clone)]
pub struct AttributePairConstraint {
    gate: SemanticConstraintGate,
    attribute1: AttributeName,
    attribute2: AttributeName,
}

impl AttributePairConstraint {
    pub fn new(attribute1: AttributeName, attribute2: AttributeName) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute1,
            attribute2,
        }
    }
}

impl SemanticConstraint for AttributePairConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        element: &OpenXmlElement,
        path: &str,
    ) -> Option<ValidationError> {
        let a1 = find_attribute_value(element, &self.attribute1).is_some();
        let a2 = find_attribute_value(element, &self.attribute2).is_some();
        if a1 == a2 {
            return None;
        }
        // C# leaves Id/Description empty; emit a stable semantic message.
        Some(semantic_error(
            path,
            "Sem_AttributePairConstraint",
            format!(
                "Attributes '{}' and '{}' must appear as a pair.",
                self.attribute1.display(),
                self.attribute2.display()
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_format::FileFormatVersions;
    use crate::validation::ValidationSettings;

    fn ctx() -> ValidationContext {
        ValidationContext::with_file_format(FileFormatVersions::OFFICE2007)
    }

    #[test]
    fn cannot_omit_and_mutual_exclusive() {
        let required = AttributeCannotOmitConstraint::new(AttributeName::local("id"));
        let mut el = OpenXmlElement::w("hyperlink");
        let path = el.qualified_name();
        assert!(required.validate_core(&el, &path).is_some());
        el.set_attribute("id", "rId1");
        assert!(required.validate_core(&el, &path).is_none());

        let exclusive = AttributeMutualExclusive::new(vec![
            AttributeName::local("auto"),
            AttributeName::local("rgb"),
            AttributeName::local("theme"),
        ]);
        let mut color = OpenXmlElement::w("tabColor");
        color.set_attribute("auto", "1");
        color.set_attribute("rgb", "FF0000");
        let path = color.qualified_name();
        let err = exclusive.validate_core(&color, &path).unwrap();
        assert_eq!(err.id(), Some("Sem_AttributeMutualExclusive"));
        assert_eq!(err.error_type(), ValidationErrorType::Semantic);
    }

    #[test]
    fn length_range_set_and_minmax() {
        let length = AttributeValueLengthConstraint::new(AttributeName::local("name"), 1, 3);
        let mut el = OpenXmlElement::w("sheet");
        el.set_attribute("name", "abcd");
        let path = el.qualified_name();
        let err = length.validate_core(&el, &path).unwrap();
        assert_eq!(err.id(), Some("Sem_AttributeValueDataTypeDetailed"));
        assert!(err.description().contains("MaxLength"));

        let range = AttributeValueRangeConstraint::new(
            AttributeName::local("val"),
            true,
            1.0,
            true,
            10.0,
            true,
        );
        let mut el = OpenXmlElement::w("num");
        el.set_attribute("val", "0");
        let path = el.qualified_name();
        assert!(range.validate_core(&el, &path).is_some());
        el.set_attribute("val", "5");
        assert!(range.validate_core(&el, &path).is_none());

        let set = AttributeValueSetConstraint::new(
            AttributeName::local("val"),
            true,
            vec!["left".into(), "right".into()],
        );
        let mut el = OpenXmlElement::w("jc");
        el.set_attribute("val", "center");
        let path = el.qualified_name();
        assert!(set.validate_core(&el, &path).is_some());
        el.set_attribute("val", "left");
        assert!(set.validate_core(&el, &path).is_none());

        let minmax = AttributeMinMaxConstraint::new(
            AttributeName::local("min"),
            AttributeName::local("max"),
        );
        let mut el = OpenXmlElement::w("range");
        el.set_attribute("min", "10");
        el.set_attribute("max", "5");
        let path = el.qualified_name();
        assert!(minmax.validate_core(&el, &path).is_some());
    }

    #[test]
    fn conditional_required_absent_and_pair() {
        let required = AttributeRequiredConditionToValue::new(
            AttributeName::local("id"),
            AttributeName::local("type"),
            "external",
        );
        let mut el = OpenXmlElement::w("link");
        el.set_attribute("type", "external");
        let path = el.qualified_name();
        assert!(required.validate_core(&el, &path).is_some());
        el.set_attribute("id", "rId1");
        assert!(required.validate_core(&el, &path).is_none());

        let absent = AttributeAbsentConditionToValue::new(
            AttributeName::local("auto"),
            AttributeName::local("rgb"),
            vec!["FF0000".into()],
        );
        let mut el = OpenXmlElement::w("color");
        el.set_attribute("auto", "1");
        el.set_attribute("rgb", "FF0000");
        let path = el.qualified_name();
        assert!(absent.validate_core(&el, &path).is_some());

        let pair = AttributePairConstraint::new(
            AttributeName::local("x"),
            AttributeName::local("y"),
        );
        let mut el = OpenXmlElement::w("pt");
        el.set_attribute("x", "1");
        let path = el.qualified_name();
        assert!(pair.validate_core(&el, &path).is_some());
        el.set_attribute("y", "2");
        assert!(pair.validate_core(&el, &path).is_none());
    }

    #[test]
    fn validate_element_constraints_walks_descendants_and_honors_gate() {
        let exclusive = AttributeMutualExclusive::new(vec![
            AttributeName::local("auto"),
            AttributeName::local("rgb"),
        ]);
        let mut child = OpenXmlElement::w("tabColor");
        child.set_attribute("auto", "1");
        child.set_attribute("rgb", "00FF00");
        let root = OpenXmlElement::w("document").with_child(child);
        let context = ctx();
        let constraints: [&dyn SemanticConstraint; 1] = [&exclusive];
        let errors =
            validate_element_constraints(&context, &root, &constraints, ApplicationType::WORD);
        assert_eq!(errors.len(), 1);

        // Version gate: constraint introduced in Office2013 does not apply for 2007.
        let gated = AttributeCannotOmitConstraint::new(AttributeName::local("id")).with_gate(
            SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT)
                .with_version(FileFormatVersions::OFFICE2013),
        );
        let el = OpenXmlElement::w("hyperlink");
        let context = ValidationContext::new(ValidationSettings::new(
            FileFormatVersions::OFFICE2007,
        ));
        assert!(gated
            .validate(&context, &el, "w:hyperlink", ApplicationType::WORD)
            .is_none());
    }
}
