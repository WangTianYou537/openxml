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

/// Runtime context for a single constraint evaluation (C# stack Current shell).
#[derive(Clone, Copy)]
pub struct SemanticConstraintContext<'a> {
    pub element: &'a OpenXmlElement,
    pub path: &'a str,
    pub parent: Option<&'a OpenXmlElement>,
    pub part_uri: Option<&'a str>,
    pub package: Option<&'a crate::opc::OpcPackage>,
    /// Root of the current part (for uniqueness / same-part references).
    pub part_root: Option<&'a OpenXmlElement>,
    /// Named part roots available for package-level reference constraints:
    /// `(part_key, root)` where `part_key` is a path (e.g. `/word/styles.xml`)
    /// or a simple name (e.g. `styles`).
    pub referenced_roots: &'a [(&'a str, &'a OpenXmlElement)],
}

impl<'a> SemanticConstraintContext<'a> {
    pub fn element_only(element: &'a OpenXmlElement, path: &'a str) -> Self {
        Self {
            element,
            path,
            parent: None,
            part_uri: None,
            package: None,
            part_root: None,
            referenced_roots: &[],
        }
    }

    fn relationship_type_for_id(&self, id: &str) -> Option<&'a str> {
        let package = self.package?;
        if let Some(part_uri) = self.part_uri {
            let uri = crate::opc::PackUri::new(part_uri);
            if let Some(rel) = package
                .part_relationships(&uri)
                .and_then(|rels| rels.get(id))
            {
                return Some(rel.relationship_type.as_str());
            }
        }
        package
            .package_relationships()
            .get(id)
            .map(|rel| rel.relationship_type.as_str())
    }

    fn has_relationship_id(&self, id: &str) -> bool {
        self.relationship_type_for_id(id).is_some()
    }

    fn find_referenced_root(&self, part_key: &str) -> Option<&'a OpenXmlElement> {
        if part_key == "." {
            return self.part_root;
        }
        self.referenced_roots.iter().find_map(|(key, root)| {
            if *key == part_key
                || key.ends_with(part_key)
                || key.rsplit('/').next() == Some(part_key)
                || key.contains(part_key)
            {
                Some(*root)
            } else {
                None
            }
        })
    }
}

/// Trait for semantic constraints (C# `SemanticConstraint` shell).
pub trait SemanticConstraint {
    fn gate(&self) -> SemanticConstraintGate;

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError>;

    /// C# `SemanticConstraint.Validate` — gate then `ValidateCore`.
    fn validate(
        &self,
        context: &ValidationContext,
        scx: &SemanticConstraintContext<'_>,
        application: ApplicationType,
    ) -> Option<ValidationError> {
        if !self.gate().applies(context, application) {
            return None;
        }
        self.validate_core(scx)
    }
}

/// Run constraints over `root` and descendants (with parent tracking).
pub fn validate_element_constraints(
    context: &ValidationContext,
    root: &OpenXmlElement,
    constraints: &[&dyn SemanticConstraint],
    application: ApplicationType,
) -> Vec<ValidationError> {
    validate_element_constraints_with_part(context, root, None, None, &[], constraints, application)
}

/// Part/package-aware constraint walk.
pub fn validate_element_constraints_with_part(
    context: &ValidationContext,
    root: &OpenXmlElement,
    part_uri: Option<&str>,
    package: Option<&crate::opc::OpcPackage>,
    referenced_roots: &[(&str, &OpenXmlElement)],
    constraints: &[&dyn SemanticConstraint],
    application: ApplicationType,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    fn walk(
        element: &OpenXmlElement,
        parent: Option<&OpenXmlElement>,
        path: &str,
        context: &ValidationContext,
        part_uri: Option<&str>,
        package: Option<&crate::opc::OpcPackage>,
        part_root: &OpenXmlElement,
        referenced_roots: &[(&str, &OpenXmlElement)],
        constraints: &[&dyn SemanticConstraint],
        application: ApplicationType,
        errors: &mut Vec<ValidationError>,
    ) {
        if !element.is_misc_node() {
            let scx = SemanticConstraintContext {
                element,
                path,
                parent,
                part_uri,
                package,
                part_root: Some(part_root),
                referenced_roots,
            };
            for constraint in constraints {
                if let Some(error) = constraint.validate(context, &scx, application) {
                    errors.push(error);
                }
            }
        }
        for (i, child) in element.children.iter().enumerate() {
            let child_path = format!("{path}/{}[{i}]", child.local_name);
            walk(
                child,
                Some(element),
                &child_path,
                context,
                part_uri,
                package,
                part_root,
                referenced_roots,
                constraints,
                application,
                errors,
            );
        }
    }
    walk(
        root,
        None,
        &root.qualified_name(),
        context,
        part_uri,
        package,
        root,
        referenced_roots,
        constraints,
        application,
        &mut errors,
    );
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        if find_attribute_value(scx.element, &self.attribute).is_some() {
            return None;
        }
        // C# only errors when the attribute is known on the element metadata
        // (`TryFindAttribute` succeeds). Without full metadata we treat any
        // missing value as a miss, matching the required-attribute Schematron
        // extractors used elsewhere.
        let qname = self.attribute.display();
        Some(semantic_error(
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let present: Vec<String> = self
            .attributes
            .iter()
            .filter(|attr| find_attribute_value(scx.element, attr).is_some())
            .map(|attr| attr.display())
            .collect();
        if present.len() < 2 {
            return None;
        }
        let all: Vec<String> = self.attributes.iter().map(|a| a.display()).collect();
        let last = present.last().cloned().unwrap_or_default();
        let earlier = present[..present.len() - 1].join(",");
        Some(semantic_error(
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let value = find_attribute_value(scx.element, &self.attribute)?;
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
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(scx.element, &self.attribute)?;
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
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(scx.element, &self.attribute)?;
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
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        if find_attribute_value(scx.element, &self.required_attribute).is_some() {
            return None;
        }
        let condition = find_attribute_value(scx.element, &self.condition_attribute)?;
        if !attribute_value_equals(condition, &self.value, false) {
            return None;
        }
        let req = self.required_attribute.display();
        let cond = self.condition_attribute.display();
        Some(semantic_error(
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let _absent = find_attribute_value(scx.element, &self.absent_attribute)?;
        let condition = find_attribute_value(scx.element, &self.condition_attribute)?;
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
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let min_text = find_attribute_value(scx.element, &self.min_attribute)?;
        let max_text = find_attribute_value(scx.element, &self.max_attribute)?;
        let min_value = parse_attr_num(min_text)?;
        let max_value = parse_attr_num(max_text)?;
        if min_value <= max_value {
            return None;
        }
        // C# leaves Id/Description empty (TODO); emit a stable semantic message.
        Some(semantic_error(
            scx.path,
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
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let a1 = find_attribute_value(scx.element, &self.attribute1).is_some();
        let a2 = find_attribute_value(scx.element, &self.attribute2).is_some();
        if a1 == a2 {
            return None;
        }
        // C# leaves Id/Description empty; emit a stable semantic message.
        Some(semantic_error(
            scx.path,
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

fn format_value_list(values: &[String], last_sep: &str) -> String {
    if values.is_empty() {
        return String::new();
    }
    if values.len() == 1 {
        return format!("'{}'", values[0]);
    }
    if values.len() == 2 {
        return format!("'{}' {last_sep} '{}'", values[0], values[1]);
    }
    let head = values[..values.len() - 1]
        .iter()
        .map(|v| format!("'{v}'"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{head} {last_sep} '{}'", values[values.len() - 1])
}

/// Very small regex subset for C# `AttributeValuePatternConstraint`:
/// anchors `^$`, literal chars, `.`, character classes `[...]`/`[^...]`,
/// and quantifiers `* + ? {n} {n,} {n,m}`. Unsupported patterns fail closed
/// (report as mismatch) only when the pattern itself is empty.
fn simple_pattern_is_match(pattern: &str, value: &str) -> bool {
    let mut pat = pattern;
    if let Some(rest) = pat.strip_prefix('^') {
        pat = rest;
    }
    if let Some(rest) = pat.strip_suffix('$') {
        pat = rest;
    }
    match_pattern(pat.as_bytes(), value.as_bytes())
}

fn match_pattern(pattern: &[u8], value: &[u8]) -> bool {
    // Recursive backtracking matcher for the supported subset.
    fn walk(pat: &[u8], val: &[u8]) -> bool {
        if pat.is_empty() {
            return val.is_empty();
        }
        let (atom, rest, quant) = match parse_atom(pat) {
            Some(v) => v,
            None => return false,
        };
        match quant {
            Quant::Exact(n) => {
                if val.len() < n {
                    return false;
                }
                for i in 0..n {
                    if !atom_matches(&atom, val[i]) {
                        return false;
                    }
                }
                walk(rest, &val[n..])
            }
            Quant::Optional => {
                if walk(rest, val) {
                    return true;
                }
                if !val.is_empty() && atom_matches(&atom, val[0]) {
                    return walk(rest, &val[1..]);
                }
                false
            }
            Quant::Star => {
                let mut i = 0;
                loop {
                    if walk(rest, &val[i..]) {
                        return true;
                    }
                    if i >= val.len() || !atom_matches(&atom, val[i]) {
                        return false;
                    }
                    i += 1;
                }
            }
            Quant::Plus => {
                if val.is_empty() || !atom_matches(&atom, val[0]) {
                    return false;
                }
                let mut i = 1;
                loop {
                    if walk(rest, &val[i..]) {
                        return true;
                    }
                    if i >= val.len() || !atom_matches(&atom, val[i]) {
                        return false;
                    }
                    i += 1;
                }
            }
            Quant::Range(min, max) => {
                let mut i = 0;
                while i < min {
                    if i >= val.len() || !atom_matches(&atom, val[i]) {
                        return false;
                    }
                    i += 1;
                }
                loop {
                    if walk(rest, &val[i..]) {
                        return true;
                    }
                    if i >= max || i >= val.len() || !atom_matches(&atom, val[i]) {
                        return false;
                    }
                    i += 1;
                }
            }
        }
    }
    walk(pattern, value)
}

#[derive(Clone)]
enum Atom {
    Any,
    Lit(u8),
    Class { negated: bool, chars: Vec<u8> },
}

enum Quant {
    Exact(usize),
    Optional,
    Star,
    Plus,
    Range(usize, usize),
}

fn parse_atom(pat: &[u8]) -> Option<(Atom, &[u8], Quant)> {
    if pat.is_empty() {
        return None;
    }
    let (atom, after_atom) = if pat[0] == b'.' {
        (Atom::Any, &pat[1..])
    } else if pat[0] == b'[' {
        let mut i = 1;
        let negated = if i < pat.len() && pat[i] == b'^' {
            i += 1;
            true
        } else {
            false
        };
        let mut chars = Vec::new();
        while i < pat.len() && pat[i] != b']' {
            if i + 2 < pat.len() && pat[i + 1] == b'-' && pat[i + 2] != b']' {
                let start = pat[i];
                let end = pat[i + 2];
                for c in start..=end {
                    chars.push(c);
                }
                i += 3;
            } else {
                chars.push(pat[i]);
                i += 1;
            }
        }
        if i >= pat.len() || pat[i] != b']' {
            return None;
        }
        (Atom::Class { negated, chars }, &pat[i + 1..])
    } else if pat[0] == b'\\' && pat.len() >= 2 {
        (Atom::Lit(pat[1]), &pat[2..])
    } else {
        (Atom::Lit(pat[0]), &pat[1..])
    };

    let (quant, rest) = if after_atom.first() == Some(&b'*') {
        (Quant::Star, &after_atom[1..])
    } else if after_atom.first() == Some(&b'+') {
        (Quant::Plus, &after_atom[1..])
    } else if after_atom.first() == Some(&b'?') {
        (Quant::Optional, &after_atom[1..])
    } else if after_atom.first() == Some(&b'{') {
        let close = after_atom.iter().position(|&b| b == b'}')?;
        let body = std::str::from_utf8(&after_atom[1..close]).ok()?;
        let quant = if let Some((a, b)) = body.split_once(',') {
            let min = a.parse::<usize>().ok()?;
            if b.is_empty() {
                Quant::Range(min, usize::MAX)
            } else {
                Quant::Range(min, b.parse::<usize>().ok()?)
            }
        } else {
            Quant::Exact(body.parse::<usize>().ok()?)
        };
        (quant, &after_atom[close + 1..])
    } else {
        (Quant::Exact(1), after_atom)
    };
    Some((atom, rest, quant))
}

fn atom_matches(atom: &Atom, byte: u8) -> bool {
    match atom {
        Atom::Any => true,
        Atom::Lit(c) => *c == byte,
        Atom::Class { negated, chars } => {
            let contains = chars.contains(&byte);
            if *negated {
                !contains
            } else {
                contains
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AttributeValuePatternConstraint
// ---------------------------------------------------------------------------

/// C# `AttributeValuePatternConstraint`.
#[derive(Debug, Clone)]
pub struct AttributeValuePatternConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    pattern: String,
}

impl AttributeValuePatternConstraint {
    pub fn new(attribute: AttributeName, pattern: impl Into<String>) -> Self {
        let mut pattern = pattern.into();
        if !(pattern.starts_with('^') && pattern.ends_with('$')) {
            pattern = format!("^{pattern}$");
        }
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute,
            pattern,
        }
    }
}

impl SemanticConstraint for AttributeValuePatternConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(scx.element, &self.attribute)?;
        if text.is_empty() {
            return None;
        }
        if simple_pattern_is_match(&self.pattern, text) {
            return None;
        }
        let sub = format_resource("Sch_PatternConstraintFailed", &[&self.pattern]);
        let qname = self.attribute.display();
        Some(semantic_error(
            scx.path,
            "Sem_AttributeValueDataTypeDetailed",
            format_resource("Sem_AttributeValueDataTypeDetailed", &[&qname, text, &sub]),
            ValidationErrorType::Schema,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeValueLessEqualToAnother
// ---------------------------------------------------------------------------

/// C# `AttributeValueLessEqualToAnother`.
#[derive(Debug, Clone)]
pub struct AttributeValueLessEqualToAnother {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    other_attribute: AttributeName,
    can_equal: bool,
}

impl AttributeValueLessEqualToAnother {
    pub fn new(
        attribute: AttributeName,
        other_attribute: AttributeName,
        can_equal: bool,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute,
            other_attribute,
            can_equal,
        }
    }
}

impl SemanticConstraint for AttributeValueLessEqualToAnother {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(scx.element, &self.attribute)?;
        let other_text = find_attribute_value(scx.element, &self.other_attribute)?;
        let val = parse_attr_num(text)?;
        let other_val = parse_attr_num(other_text)?;
        let ok = if self.can_equal {
            val <= other_val
        } else {
            val < other_val
        };
        if ok {
            return None;
        }
        let message_id = if self.can_equal {
            "Sem_AttributeValueLessEqualToAnother"
        } else {
            "Sem_AttributeValueLessEqualToAnotherEx"
        };
        let a = self.attribute.display();
        let b = self.other_attribute.display();
        Some(semantic_error(
            scx.path,
            "Sem_AttributeValueLessEqualToAnother",
            format_resource(message_id, &[&a, text, &b, other_text]),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeAbsentConditionToNonValue
// ---------------------------------------------------------------------------

/// C# `AttributeAbsentConditionToNonValue`.
#[derive(Debug, Clone)]
pub struct AttributeAbsentConditionToNonValue {
    gate: SemanticConstraintGate,
    absent_attribute: AttributeName,
    condition_attribute: AttributeName,
    values: Vec<String>,
}

impl AttributeAbsentConditionToNonValue {
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

impl SemanticConstraint for AttributeAbsentConditionToNonValue {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let _absent = find_attribute_value(scx.element, &self.absent_attribute)?;
        let condition = find_attribute_value(scx.element, &self.condition_attribute)?;
        if self
            .values
            .iter()
            .any(|v| attribute_value_equals(condition, v, false))
        {
            return None;
        }
        let value_string = format_value_list(&self.values, "and");
        let absent = self.absent_attribute.display();
        let cond = self.condition_attribute.display();
        Some(semantic_error(
            scx.path,
            "Sem_AttributeAbsentConditionToNonValue",
            format_resource(
                "Sem_AttributeAbsentConditionToNonValue",
                &[&absent, &cond, &value_string],
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// AttributeValueConditionToAnother
// ---------------------------------------------------------------------------

/// C# `AttributeValueConditionToAnother`.
#[derive(Debug, Clone)]
pub struct AttributeValueConditionToAnother {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    condition_attribute: AttributeName,
    values: Vec<String>,
    other_values: Vec<String>,
}

impl AttributeValueConditionToAnother {
    pub fn new(
        attribute: AttributeName,
        condition_attribute: AttributeName,
        values: Vec<String>,
        other_values: Vec<String>,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            attribute,
            condition_attribute,
            values,
            other_values,
        }
    }
}

impl SemanticConstraint for AttributeValueConditionToAnother {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(scx.element, &self.attribute)?;
        if self
            .values
            .iter()
            .any(|v| attribute_value_equals(text, v, false))
        {
            return None;
        }
        let condition = find_attribute_value(scx.element, &self.condition_attribute)?;
        if !self
            .other_values
            .iter()
            .any(|v| attribute_value_equals(condition, v, false))
        {
            return None;
        }
        let attr_values = format_value_list(&self.values, "or");
        let other_values = format_value_list(&self.other_values, "or");
        let a = self.attribute.display();
        let b = self.condition_attribute.display();
        Some(semantic_error(
            scx.path,
            "Sem_AttributeValueConditionToAnother",
            format_resource(
                "Sem_AttributeValueConditionToAnother",
                &[&a, &attr_values, &b, &other_values, &a, text],
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// RelationshipExistConstraint
// ---------------------------------------------------------------------------

/// C# `RelationshipExistConstraint` — rId attribute must resolve in the part.
#[derive(Debug, Clone)]
pub struct RelationshipExistConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
}

impl RelationshipExistConstraint {
    pub fn new(attribute: AttributeName) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::PART),
            attribute,
        }
    }
}

impl SemanticConstraint for RelationshipExistConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let id = find_attribute_value(scx.element, &self.attribute)?;
        if id.is_empty() {
            return None;
        }
        if scx.has_relationship_id(id) {
            return None;
        }
        let qname = self.attribute.display();
        Some(semantic_error(
            scx.path,
            "Sem_InvalidRelationshipId",
            format_resource("Sem_InvalidRelationshipId", &[id, &qname]),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// RelationshipTypeConstraint
// ---------------------------------------------------------------------------

/// C# `RelationshipTypeConstraint` — rId must resolve to the expected type.
#[derive(Debug, Clone)]
pub struct RelationshipTypeConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    expected_type: String,
}

impl RelationshipTypeConstraint {
    pub fn new(attribute: AttributeName, expected_type: impl Into<String>) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::PART),
            attribute,
            expected_type: expected_type.into(),
        }
    }
}

impl SemanticConstraint for RelationshipTypeConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let id = find_attribute_value(scx.element, &self.attribute)?;
        if id.is_empty() {
            return None;
        }
        let actual = scx.relationship_type_for_id(id)?;
        if relationship_type_matches(actual, &self.expected_type) {
            return None;
        }
        let qname = self.attribute.display();
        Some(semantic_error(
            scx.path,
            "Sem_IncorrectRelationshipType",
            format_resource(
                "Sem_IncorrectRelationshipType",
                &[actual, &qname, &self.expected_type],
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

fn relationship_type_matches(actual: &str, expected: &str) -> bool {
    if actual == expected {
        return true;
    }
    let exp_suffix = expected.rsplit('/').next().unwrap_or(expected);
    actual.ends_with(exp_suffix) || actual.contains(exp_suffix)
}

// ---------------------------------------------------------------------------
// UniqueAttributeValueConstraint
// ---------------------------------------------------------------------------

/// C# `UniqueAttributeValueConstraint` — attribute unique among same-local-name
/// elements under the part root (parent-scoped uniqueness when `parent_local_name`
/// is set).
#[derive(Debug, Clone)]
pub struct UniqueAttributeValueConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    case_sensitive: bool,
    /// Optional ancestor local name that scopes uniqueness (C# `_parent`).
    parent_local_name: Option<String>,
}

impl UniqueAttributeValueConstraint {
    pub fn new(attribute: AttributeName, case_sensitive: bool) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::PART),
            attribute,
            case_sensitive,
            parent_local_name: None,
        }
    }

    pub fn with_parent_local_name(mut self, parent_local_name: impl Into<String>) -> Self {
        self.parent_local_name = Some(parent_local_name.into());
        self
    }
}

impl SemanticConstraint for UniqueAttributeValueConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        // C# returns null when a parent scope is configured (parent-scoped
        // uniqueness path is currently unused in the SDK metadata).
        if self.parent_local_name.is_some() {
            return None;
        }
        let value = find_attribute_value(scx.element, &self.attribute)?;
        if value.is_empty() {
            return None;
        }
        let root = scx.part_root.unwrap_or(scx.element);
        let element_name = scx.element.local_name.as_str();
        let key = if self.case_sensitive {
            value.to_string()
        } else {
            value.to_ascii_lowercase()
        };

        let mut count = 0usize;
        for el in std::iter::once(root).chain(root.descendants()) {
            if el.local_name != element_name {
                continue;
            }
            let Some(other) = find_attribute_value(el, &self.attribute) else {
                continue;
            };
            let other_key = if self.case_sensitive {
                other.to_string()
            } else {
                other.to_ascii_lowercase()
            };
            if other_key == key {
                count += 1;
            }
        }
        if count < 2 {
            return None;
        }
        // Emit only when this is not the first occurrence so the error is raised
        // once per duplicate value (C# DuplicateFinder.IsDuplicate removes once).
        let mut seen = 0usize;
        for el in std::iter::once(root).chain(root.descendants()) {
            if el.local_name != element_name {
                continue;
            }
            let Some(other) = find_attribute_value(el, &self.attribute) else {
                continue;
            };
            let other_key = if self.case_sensitive {
                other.to_string()
            } else {
                other.to_ascii_lowercase()
            };
            if other_key != key {
                continue;
            }
            seen += 1;
            if std::ptr::eq(el, scx.element) {
                if seen == 1 {
                    return None;
                }
                break;
            }
        }
        let qname = self.attribute.display();
        Some(semantic_error(
            scx.path,
            "Sem_UniqueAttributeValue",
            format_resource("Sem_UniqueAttributeValue", &[&qname, value]),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// ParentTypeConstraint
// ---------------------------------------------------------------------------

/// C# `ParentTypeConstraint` — parent local name must (or must not) match.
#[derive(Debug, Clone)]
pub struct ParentTypeConstraint {
    gate: SemanticConstraintGate,
    parent_local_name: String,
    is_valid: bool,
}

impl ParentTypeConstraint {
    pub fn new(parent_local_name: impl Into<String>, is_valid: bool) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::ELEMENT),
            parent_local_name: parent_local_name.into(),
            is_valid,
        }
    }
}

impl SemanticConstraint for ParentTypeConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let parent = scx.parent?;
        let matches = parent.local_name == self.parent_local_name;
        // C# returns null when parent type matches OR when isValid is false
        // (the invalid-parent path is unfinished / empty error). Mirror that:
        // only report when is_valid is true and the parent does not match.
        if matches || !self.is_valid {
            return None;
        }
        Some(semantic_error(
            scx.path,
            "Sem_InvalidParentType",
            format!(
                "Element '{}' parent must be '{}', found '{}'.",
                scx.element.local_name, self.parent_local_name, parent.local_name
            ),
            ValidationErrorType::Semantic,
        ))
    }
}

// ---------------------------------------------------------------------------
// ReferenceExistConstraint
// ---------------------------------------------------------------------------

/// C# `ReferenceExistConstraint` — attribute value must exist as an attribute
/// on matching elements in a referenced part root.
#[derive(Debug, Clone)]
pub struct ReferenceExistConstraint {
    gate: SemanticConstraintGate,
    ref_attribute: AttributeName,
    part_path: String,
    element_local_name: String,
    element_display_name: String,
    attribute: AttributeName,
}

impl ReferenceExistConstraint {
    pub fn new(
        ref_attribute: AttributeName,
        part_path: impl Into<String>,
        element_local_name: impl Into<String>,
        element_display_name: impl Into<String>,
        attribute: AttributeName,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::PACKAGE),
            ref_attribute,
            part_path: part_path.into(),
            element_local_name: element_local_name.into(),
            element_display_name: element_display_name.into(),
            attribute,
        }
    }
}

impl SemanticConstraint for ReferenceExistConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let value = find_attribute_value(scx.element, &self.ref_attribute)?;
        if value.is_empty() {
            return None;
        }
        let root = scx.find_referenced_root(&self.part_path)?;
        let exists = std::iter::once(root)
            .chain(root.descendants())
            .filter(|el| el.local_name == self.element_local_name)
            .any(|el| find_attribute_value(el, &self.attribute) == Some(value));
        if exists {
            return None;
        }
        let part = scx
            .referenced_roots
            .iter()
            .find(|(k, _)| *k == self.part_path || k.contains(&self.part_path))
            .map(|(k, _)| *k)
            .unwrap_or(self.part_path.as_str());
        let qname = self.ref_attribute.display();
        Some(
            semantic_error(
                scx.path,
                "Sem_MissingReferenceElement",
                format_resource(
                    "Sem_MissingReferenceElement",
                    &[
                        &self.element_display_name,
                        &scx.element.local_name,
                        &qname,
                        part,
                        value,
                    ],
                ),
                ValidationErrorType::Semantic,
            )
            .with_related_part_uri(part),
        )
    }
}

// ---------------------------------------------------------------------------
// IndexReferenceConstraint
// ---------------------------------------------------------------------------

/// C# `IndexReferenceConstraint` — attribute is a 0/1-based index into elements
/// of a referenced part.
#[derive(Debug, Clone)]
pub struct IndexReferenceConstraint {
    gate: SemanticConstraintGate,
    attribute: AttributeName,
    ref_part_type: String,
    ref_element_local_name: String,
    ref_element_parent_local_name: Option<String>,
    index_base: i32,
}

impl IndexReferenceConstraint {
    pub fn new(
        attribute: AttributeName,
        ref_part_type: impl Into<String>,
        ref_element_local_name: impl Into<String>,
        index_base: i32,
    ) -> Self {
        Self {
            gate: SemanticConstraintGate::new(SemanticValidationLevel::PACKAGE),
            attribute,
            ref_part_type: ref_part_type.into(),
            ref_element_local_name: ref_element_local_name.into(),
            ref_element_parent_local_name: None,
            index_base,
        }
    }

    pub fn with_parent_local_name(mut self, parent: impl Into<String>) -> Self {
        self.ref_element_parent_local_name = Some(parent.into());
        self
    }
}

impl SemanticConstraint for IndexReferenceConstraint {
    fn gate(&self) -> SemanticConstraintGate {
        self.gate
    }

    fn validate_core(
        &self,
        scx: &SemanticConstraintContext<'_>,
    ) -> Option<ValidationError> {
        let text = find_attribute_value(scx.element, &self.attribute)?;
        if text.is_empty() {
            return None;
        }
        let index: i32 = text.parse().ok()?;
        let root = scx.find_referenced_root(&self.ref_part_type)?;
        let count = std::iter::once(root)
            .chain(root.descendants())
            .filter(|el| el.local_name == self.ref_element_local_name)
            .count() as i32;
        if index < count + self.index_base {
            return None;
        }
        let part = scx
            .referenced_roots
            .iter()
            .find(|(k, _)| *k == self.ref_part_type || k.contains(&self.ref_part_type))
            .map(|(k, _)| *k)
            .unwrap_or(self.ref_part_type.as_str());
        let qname = self.attribute.display();
        Some(
            semantic_error(
                scx.path,
                "Sem_MissingIndexedElement",
                format_resource(
                    "Sem_MissingIndexedElement",
                    &[
                        &self.ref_element_local_name,
                        &scx.element.local_name,
                        &qname,
                        part,
                        text,
                    ],
                ),
                ValidationErrorType::Semantic,
            )
            .with_related_part_uri(part),
        )
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
        assert!(required.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("id", "rId1");
        assert!(required.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());

        let exclusive = AttributeMutualExclusive::new(vec![
            AttributeName::local("auto"),
            AttributeName::local("rgb"),
            AttributeName::local("theme"),
        ]);
        let mut color = OpenXmlElement::w("tabColor");
        color.set_attribute("auto", "1");
        color.set_attribute("rgb", "FF0000");
        let path = color.qualified_name();
        let err = exclusive.validate_core(&SemanticConstraintContext::element_only(&color, &path)).unwrap();
        assert_eq!(err.id(), Some("Sem_AttributeMutualExclusive"));
        assert_eq!(err.error_type(), ValidationErrorType::Semantic);
    }

    #[test]
    fn length_range_set_and_minmax() {
        let length = AttributeValueLengthConstraint::new(AttributeName::local("name"), 1, 3);
        let mut el = OpenXmlElement::w("sheet");
        el.set_attribute("name", "abcd");
        let path = el.qualified_name();
        let err = length.validate_core(&SemanticConstraintContext::element_only(&el, &path)).unwrap();
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
        assert!(range.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("val", "5");
        assert!(range.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());

        let set = AttributeValueSetConstraint::new(
            AttributeName::local("val"),
            true,
            vec!["left".into(), "right".into()],
        );
        let mut el = OpenXmlElement::w("jc");
        el.set_attribute("val", "center");
        let path = el.qualified_name();
        assert!(set.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("val", "left");
        assert!(set.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());

        let minmax = AttributeMinMaxConstraint::new(
            AttributeName::local("min"),
            AttributeName::local("max"),
        );
        let mut el = OpenXmlElement::w("range");
        el.set_attribute("min", "10");
        el.set_attribute("max", "5");
        let path = el.qualified_name();
        assert!(minmax.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
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
        assert!(required.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("id", "rId1");
        assert!(required.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());

        let absent = AttributeAbsentConditionToValue::new(
            AttributeName::local("auto"),
            AttributeName::local("rgb"),
            vec!["FF0000".into()],
        );
        let mut el = OpenXmlElement::w("color");
        el.set_attribute("auto", "1");
        el.set_attribute("rgb", "FF0000");
        let path = el.qualified_name();
        assert!(absent.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());

        let pair = AttributePairConstraint::new(
            AttributeName::local("x"),
            AttributeName::local("y"),
        );
        let mut el = OpenXmlElement::w("pt");
        el.set_attribute("x", "1");
        let path = el.qualified_name();
        assert!(pair.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("y", "2");
        assert!(pair.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());
    }

    #[test]
    fn pattern_less_equal_absent_nonvalue_and_value_condition() {
        assert!(simple_pattern_is_match("^[0-9]{2}$", "12"));
        assert!(!simple_pattern_is_match("^[0-9]{2}$", "1a"));
        assert!(simple_pattern_is_match("^a.*z$", "abcz"));
        assert!(!simple_pattern_is_match("^a.*z$", "abc"));

        let pattern =
            AttributeValuePatternConstraint::new(AttributeName::local("id"), "[0-9a-fA-F]{4}");
        let mut el = OpenXmlElement::w("item");
        el.set_attribute("id", "12G4");
        let path = el.qualified_name();
        let err = pattern.validate_core(&SemanticConstraintContext::element_only(&el, &path)).unwrap();
        assert_eq!(err.id(), Some("Sem_AttributeValueDataTypeDetailed"));
        assert!(err.description().contains("Pattern"));
        el.set_attribute("id", "12aF");
        assert!(pattern.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());

        let le = AttributeValueLessEqualToAnother::new(
            AttributeName::local("min"),
            AttributeName::local("max"),
            true,
        );
        let mut el = OpenXmlElement::w("range");
        el.set_attribute("min", "9");
        el.set_attribute("max", "3");
        let path = el.qualified_name();
        assert!(le.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("min", "2");
        assert!(le.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());

        let absent_non = AttributeAbsentConditionToNonValue::new(
            AttributeName::local("auto"),
            AttributeName::local("mode"),
            vec!["manual".into()],
        );
        let mut el = OpenXmlElement::w("color");
        el.set_attribute("auto", "1");
        el.set_attribute("mode", "auto");
        let path = el.qualified_name();
        assert!(absent_non.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("mode", "manual");
        assert!(absent_non.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());

        let cond = AttributeValueConditionToAnother::new(
            AttributeName::local("val"),
            AttributeName::local("type"),
            vec!["left".into(), "right".into()],
            vec!["align".into()],
        );
        let mut el = OpenXmlElement::w("jc");
        el.set_attribute("val", "center");
        el.set_attribute("type", "align");
        let path = el.qualified_name();
        assert!(cond.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_some());
        el.set_attribute("val", "left");
        assert!(cond.validate_core(&SemanticConstraintContext::element_only(&el, &path)).is_none());
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
            .validate(&context, &SemanticConstraintContext::element_only(&el, "w:hyperlink"), ApplicationType::WORD)
            .is_none());
    }

    #[test]
    fn relationship_unique_parent_reference_and_index_constraints() {
        use crate::namespace::rel;
        use crate::opc::{OpcPackage, PackUri, RelationshipTargetMode};

        // UniqueAttributeValueConstraint needs a PART stack frame.
        let unique = UniqueAttributeValueConstraint::new(AttributeName::local("styleId"), true);
        let mut s1 = OpenXmlElement::w("style");
        s1.set_attribute("styleId", "Normal");
        let mut s2 = OpenXmlElement::w("style");
        s2.set_attribute("styleId", "Normal");
        let styles = OpenXmlElement::w("styles").with_children(vec![s1, s2]);
        let mut context = ctx();
        context.stack_mut().push_part("/word/styles.xml");
        let constraints: [&dyn SemanticConstraint; 1] = [&unique];
        let errors =
            validate_element_constraints(&context, &styles, &constraints, ApplicationType::WORD);
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert_eq!(errors[0].id(), Some("Sem_UniqueAttributeValue"));
        context.stack_mut().clear();

        // Parent-scoped uniqueness is intentionally a no-op (C# returns null).
        let with_parent = UniqueAttributeValueConstraint::new(AttributeName::local("styleId"), true)
            .with_parent_local_name("styles");
        context.stack_mut().push_part("/word/styles.xml");
        let constraints: [&dyn SemanticConstraint; 1] = [&with_parent];
        let skipped =
            validate_element_constraints(&context, &styles, &constraints, ApplicationType::WORD);
        assert!(
            skipped
                .iter()
                .all(|e| e.id() != Some("Sem_UniqueAttributeValue")),
            "{skipped:?}"
        );
        context.stack_mut().clear();

        // ParentTypeConstraint: w:t under w:r is ok; under w:p is not.
        let parent = ParentTypeConstraint::new("r", true);
        let r = OpenXmlElement::w("r").with_child(OpenXmlElement::w("t"));
        let p = OpenXmlElement::w("p").with_child(OpenXmlElement::w("t"));
        let constraints: [&dyn SemanticConstraint; 1] = [&parent];
        let ok = validate_element_constraints(&context, &r, &constraints, ApplicationType::WORD);
        assert!(
            ok.iter().all(|e| e.id() != Some("Sem_InvalidParentType")),
            "{ok:?}"
        );
        let bad = validate_element_constraints(&context, &p, &constraints, ApplicationType::WORD);
        assert!(
            bad.iter().any(|e| e.id() == Some("Sem_InvalidParentType")),
            "{bad:?}"
        );

        // RelationshipExist + RelationshipType with a real package.
        let mut package = OpcPackage::create();
        let part = PackUri::new("/word/document.xml");
        package.set_part(
            part.clone(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
            b"<w:document/>".to_vec(),
        );
        package.part_relationships_mut(&part).add_with_id(
            "rId1",
            rel::HYPERLINK,
            "https://example.com",
            RelationshipTargetMode::External,
        );

        let mut hyperlink = OpenXmlElement::w("hyperlink");
        hyperlink.set_attribute("id", "rId1");
        let mut bad_link = OpenXmlElement::w("hyperlink");
        bad_link.set_attribute("id", "rIdMissing");
        let root = OpenXmlElement::w("document").with_children(vec![hyperlink, bad_link]);

        let exist = RelationshipExistConstraint::new(AttributeName::local("id"));
        let ty = RelationshipTypeConstraint::new(AttributeName::local("id"), rel::HYPERLINK);
        let constraints: [&dyn SemanticConstraint; 2] = [&exist, &ty];
        context.stack_mut().push_part(part.as_str());
        let errors = validate_element_constraints_with_part(
            &context,
            &root,
            Some(part.as_str()),
            Some(&package),
            &[],
            &constraints,
            ApplicationType::WORD,
        );
        assert!(
            errors
                .iter()
                .any(|e| e.id() == Some("Sem_InvalidRelationshipId")),
            "{errors:?}"
        );
        assert!(
            !errors
                .iter()
                .any(|e| e.id() == Some("Sem_IncorrectRelationshipType")),
            "{errors:?}"
        );
        context.stack_mut().clear();

        // ReferenceExistConstraint (PACKAGE level) needs a package frame.
        let mut abstract_num = OpenXmlElement::w("abstractNum");
        abstract_num.set_attribute("abstractNumId", "1");
        let numbering = OpenXmlElement::w("numbering").with_child(abstract_num);
        let mut num_pr = OpenXmlElement::w("numPr");
        num_pr.set_attribute("val", "9");
        let mut good_num = OpenXmlElement::w("numPr");
        good_num.set_attribute("val", "1");
        let body = OpenXmlElement::w("body").with_children(vec![num_pr, good_num]);
        let ref_exist = ReferenceExistConstraint::new(
            AttributeName::local("val"),
            "numbering",
            "abstractNum",
            "abstractNum",
            AttributeName::local("abstractNumId"),
        );
        let constraints: [&dyn SemanticConstraint; 1] = [&ref_exist];
        context.stack_mut().push_package("/");
        let errors = validate_element_constraints_with_part(
            &context,
            &body,
            Some("/word/document.xml"),
            None,
            &[("numbering", &numbering), ("/word/numbering.xml", &numbering)],
            &constraints,
            ApplicationType::WORD,
        );
        assert!(
            errors
                .iter()
                .any(|e| e.id() == Some("Sem_MissingReferenceElement")
                    && e.description().contains("'9'")),
            "{errors:?}"
        );

        // IndexReferenceConstraint: index into abstractNum children.
        let mut n0 = OpenXmlElement::w("abstractNum");
        n0.set_attribute("abstractNumId", "0");
        let mut n1 = OpenXmlElement::w("abstractNum");
        n1.set_attribute("abstractNumId", "1");
        let numbering = OpenXmlElement::w("numbering").with_children(vec![n0, n1]);
        let mut ref_el = OpenXmlElement::w("num");
        ref_el.set_attribute("val", "5");
        let body = OpenXmlElement::w("body").with_child(ref_el);
        let index = IndexReferenceConstraint::new(
            AttributeName::local("val"),
            "numbering",
            "abstractNum",
            0,
        );
        let constraints: [&dyn SemanticConstraint; 1] = [&index];
        let errors = validate_element_constraints_with_part(
            &context,
            &body,
            Some("/word/document.xml"),
            None,
            &[("numbering", &numbering)],
            &constraints,
            ApplicationType::WORD,
        );
        assert!(
            errors
                .iter()
                .any(|e| e.id() == Some("Sem_MissingIndexedElement")),
            "{errors:?}"
        );
    }

}
