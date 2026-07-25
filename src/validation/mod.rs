//! Schema validation helpers.
//!
//! - Lightweight allowed-child checks ([`validate_children`], [`validate_word_document`])
//! - Ordered particle matching ([`particle`]) for sequence/choice/group content models
//! - Package structure checks ([`validate_package`])
//! - Attribute simple-type checks ([`validate_attributes`])
//! - Semantic subset from C# `schematrons.json` ([`schematron_rules`])
//! - Attribute constraints (ranges / lengths / patterns / enums) ([`schematron_constraints`])

mod attributes;
mod digsig;
mod digsig_crypto;
mod package;
mod package_validator;
mod open_xml_validator;
mod particle;
mod schematron_constraints;
mod schematron_rules;
mod schematron_validate;
mod semantic;
mod validation_cache;
mod validation_context;
mod validation_stack;

pub use digsig::{add_digital_signature_part, clear_digital_signatures, digital_signature_parts, ensure_digital_signature_origin, has_digital_signature_origin, validate_digital_signatures};
pub use digsig_crypto::{
    build_signature_xml, build_signature_xml_with_value, build_signed_signature_xml, digest_hex,
    extract_signed_info_xml, rsa_sha256_sign, rsa_sha256_verify, signature_part_uris,
    simple_c14n_signed_info, validate_signature_digests, verify_signature_digests,
    verify_signature_value, DigestCheckResult,
};
pub use attributes::{
    validate_attribute_range, validate_attribute_value, validate_attributes,
    validate_spreadsheet_attribute_ranges, AttributeRule, AttributeType,
};
pub use package::validate_package;
pub use package_validator::{
    message_id as package_constraint_message_id, validate_package_constraints,
    validate_part_constraints, OpenXmlPackageValidationResult,
};
pub use open_xml_validator::{OpenXmlValidator, ValidationSettings};
pub use validation_cache::ValidationCache;
pub use validation_context::ValidationContext;
pub use validation_stack::{StateManager, ValidationElement, ValidationErrorEventArgs, ValidationStack};
pub use particle::{validate_particle, validate_word_particles, Occurs, Particle};
pub use schematron_constraints::{
    schematron_ancestor_unique_rules, schematron_attr_compare_rules,
    schematron_both_present_rules, schematron_conditional_attr_rules,
    schematron_cross_part_count_rules, schematron_cross_part_index_rules,
    schematron_enum_rules, schematron_finite_number_rules, schematron_fixed_bool_rules,
    schematron_fixed_ne_rules, schematron_fixed_value_rules, schematron_multi_ne_rules,
    schematron_nonzero_guid_rules, schematron_numeric_range_rules, schematron_pattern_rules,
    schematron_absent_when_not_rules, schematron_attr_and_enum_rules,
    schematron_bool_pair_impl_rules, schematron_enum_when_flag_rules,
    schematron_mutual_exclusive_rules, schematron_required_attr_rules,
    schematron_special_pattern_rules, schematron_string_length_rules, AncestorUniqueRule,
    AttrCompareRule, BothPresentRule, ConditionalAttrRule, CrossPartCountRule,
    CrossPartIndexRule, EnumRule, FiniteNumberRule, FixedBoolRule, FixedNeRule,
    FixedValueRule, MultiNeRule, NonZeroGuidRule, NumericRangeRule, PatternRule,
    AbsentWhenNotRule, AttrAndEnumRule, BoolPairImplRule, EnumWhenFlagRule, MutualExclusiveRule,
    RequiredAttrRule, SpecialPatternRule, StringLengthRule,
    SCHEMATRON_ABSENT_WHEN_NOT_COUNT, SCHEMATRON_ATTR_AND_ENUM_COUNT, SCHEMATRON_BOOL_PAIR_IMPL_COUNT,
    SCHEMATRON_ENUM_WHEN_FLAG_COUNT, SCHEMATRON_SPECIAL_PATTERN_COUNT,
    SCHEMATRON_ANCESTOR_UNIQUE_COUNT,
    SCHEMATRON_ATTR_COMPARE_COUNT, SCHEMATRON_BOTH_PRESENT_COUNT,
    SCHEMATRON_CONDITIONAL_ATTR_COUNT, SCHEMATRON_CROSS_PART_COUNT_COUNT,
    SCHEMATRON_CROSS_PART_INDEX_COUNT, SCHEMATRON_ENUM_COUNT, SCHEMATRON_FINITE_NUMBER_COUNT,
    SCHEMATRON_FIXED_BOOL_COUNT, SCHEMATRON_FIXED_NE_COUNT, SCHEMATRON_FIXED_VALUE_COUNT,
    SCHEMATRON_MULTI_NE_COUNT, SCHEMATRON_NONZERO_GUID_COUNT, SCHEMATRON_NUMERIC_RANGE_COUNT,
    SCHEMATRON_PATTERN_COUNT, SCHEMATRON_MUTUAL_EXCLUSIVE_COUNT, SCHEMATRON_REQUIRED_ATTR_COUNT, SCHEMATRON_STRING_LENGTH_COUNT,
};
pub use schematron_rules::{
    schematron_relationship_rules, schematron_unique_attribute_rules,
    SCHEMATRON_EXTRACTED_REL_COUNT, SCHEMATRON_EXTRACTED_UNIQUE_COUNT,
    SCHEMATRON_TOTAL_SOURCE_RULES,
};
pub use schematron_validate::{
    validate_schematron_ancestor_unique, validate_schematron_attr_compare,
    validate_schematron_both_present, validate_schematron_conditional_attrs,
    validate_schematron_constraints, validate_schematron_cross_part,
    validate_schematron_enums, validate_schematron_finite_numbers,
    validate_schematron_fixed_bools, validate_schematron_fixed_nes,
    validate_schematron_fixed_values, validate_schematron_multi_nes,
    validate_schematron_nonzero_guids, validate_schematron_numeric_ranges,
    validate_schematron_patterns, validate_schematron_absent_when_not, validate_schematron_attr_and_enum,
    validate_schematron_bool_pair_impl, validate_schematron_enum_when_flag,
    validate_schematron_mutual_exclusive, validate_schematron_required_attrs,
    validate_schematron_special_patterns,
    validate_schematron_string_lengths,
};
pub use semantic::{
    merged_relationship_rules, merged_unique_attribute_rules,
    presentation_relationship_rules, presentation_unique_attribute_rules,
    spreadsheet_relationship_rules, spreadsheet_unique_attribute_rules,
    validate_relationship_refs, validate_schematron_attributes, validate_schematron_subset,
    validate_semantic, validate_unique_attributes, word_relationship_rules,
    word_unique_attribute_rules, RelationshipExistRule, UniqueAttributeRule,
};

/// Re-export Markup Compatibility AlternateContent structural validation.
pub use crate::markup_compatibility::{
    validate_alternate_content, validate_mc_attributes,
};

use crate::element::OpenXmlElement;
use std::collections::HashMap;
use std::fmt;

/// Classification of a validation diagnostic (C# `ValidationErrorType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationErrorType {
    /// Schema / particle / attribute type error.
    Schema,
    /// Semantic / Schematron-style rule.
    Semantic,
    /// Package structure / part constraint.
    Package,
    /// Markup Compatibility rule.
    MarkupCompatibility,
}

/// A single validation diagnostic (C# `ValidationErrorInfo` subset).
///
/// `id` and `error_type` are derived from the conventional `MessageId: detail`
/// prefix used throughout this crate (mirrors C# resource ids).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    /// Invalid node location (C# `ValidationErrorInfo.Node` path shell).
    pub node_path: Option<String>,
    /// Related element location (C# `RelatedNode`).
    pub related_node_path: Option<String>,
    /// Related part URI (C# `RelatedPart`).
    pub related_part_uri: Option<String>,
}

impl ValidationError {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    /// Build from C#-style Id + Description (message becomes `Id: Description`).
    pub fn with_id(
        path: impl Into<String>,
        id: &str,
        description: impl AsRef<str>,
    ) -> Self {
        Self {
            path: path.into(),
            message: format!("{id}: {}", description.as_ref()),
            ..Default::default()
        }
    }

    pub fn with_node_path(mut self, path: impl Into<String>) -> Self {
        self.node_path = Some(path.into());
        self
    }

    pub fn with_related_node_path(mut self, path: impl Into<String>) -> Self {
        self.related_node_path = Some(path.into());
        self
    }

    pub fn with_related_part_uri(mut self, uri: impl Into<String>) -> Self {
        self.related_part_uri = Some(uri.into());
        self
    }

    /// Human-readable description (C# `ValidationErrorInfo.Description`).
    ///
    /// When `message` is `Id: detail`, returns the detail portion; otherwise the full message.
    pub fn description(&self) -> &str {
        match self.id() {
            Some(id) => self
                .message
                .get(id.len()..)
                .map(|rest| rest.trim_start_matches(':').trim())
                .filter(|s| !s.is_empty())
                .unwrap_or(self.message.as_str()),
            None => self.message.as_str(),
        }
    }

    /// XPath / part location as [`crate::element::XmlPath`] shell (C# `ValidationErrorInfo.Path`).
    ///
    /// Uses `path` as either an element XPath (when it starts with `/` and is not a package
    /// part URI) or a part URI.
    pub fn xml_path(&self) -> crate::element::XmlPath {
        let p = self.path.as_str();
        if p.starts_with('/') && p.contains(':') && !p.contains('.') {
            // Heuristic: element XPath like `/w:document[1]/w:body[1]`
            crate::element::XmlPath {
                xpath: p.to_string(),
                part_uri: None,
                namespaces: Vec::new(),
            }
        } else if p.starts_with('/') {
            crate::element::XmlPath::for_part(p)
        } else if p.is_empty() {
            crate::element::XmlPath {
                xpath: String::new(),
                part_uri: None,
                namespaces: Vec::new(),
            }
        } else {
            crate::element::XmlPath {
                xpath: p.to_string(),
                part_uri: None,
                namespaces: Vec::new(),
            }
        }
    }

    /// Stable error identifier when `message` begins with `Token:` (C# `ValidationErrorInfo.Id`).
    pub fn id(&self) -> Option<&str> {
        let head = self.message.split_once(':').map(|(h, _)| h.trim())?;
        if head.is_empty() {
            return None;
        }
        if !head
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return None;
        }
        // Prefer known SDK-style prefixes; also accept CamelCase ids without underscore
        // used by package constraints (PartIsNotAllowed, …).
        let ok = head.contains('_')
            || head.starts_with("Sch")
            || head.starts_with("MC")
            || head.starts_with("Sem")
            || head.starts_with("Part")
            || head.starts_with("Only")
            || head.starts_with("Required")
            || head.starts_with("Invalid")
            || head.starts_with("Data");
        ok.then_some(head)
    }

    /// Infer error category from [`Self::id`] / message (C# `ValidationErrorInfo.ErrorType`).
    pub fn error_type(&self) -> ValidationErrorType {
        let id = self.id().unwrap_or("");
        if id.starts_with("MC_") || id.starts_with("MC") {
            ValidationErrorType::MarkupCompatibility
        } else if id.starts_with("Sem_")
            || id.starts_with("Sem")
            || self.message.contains("relationship id")
            || self.message.contains("duplicate ")
        {
            ValidationErrorType::Semantic
        } else if id.starts_with("Part")
            || id.starts_with("Only")
            || id.starts_with("Required")
            || id.starts_with("InvalidContent")
            || id.starts_with("DataPart")
            || self.path.starts_with("/_rels")
            || self.path.contains("#")
                && (self.message.contains("part") || self.message.contains("relationship"))
        {
            ValidationErrorType::Package
        } else {
            ValidationErrorType::Schema
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id() {
            Some(id) => write!(
                f,
                "[{:?}/{id}] {}: {}",
                self.error_type(),
                self.path,
                self.message
            ),
            None => write!(f, "{}: {}", self.path, self.message),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Child constraint used by the lightweight validator.
#[derive(Debug, Clone, Copy)]
pub struct ChildRule {
    pub local_name: &'static str,
    pub max_one: bool,
    pub required: bool,
}

/// Extract the element local name from a schema child name like `w:CT_Body/w:body`.
pub fn local_name_from_schema_child(name: &str) -> &str {
    let elem = name.split('/').nth(1).unwrap_or(name);
    elem.rsplit(':').next().unwrap_or(elem)
}

/// Validate `element`'s direct children against `rules`.
pub fn validate_children(
    element: &OpenXmlElement,
    rules: &[ChildRule],
    path: &str,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let allowed: HashMap<&str, &ChildRule> = rules.iter().map(|r| (r.local_name, r)).collect();

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for child in &element.children {
        if child.local_name == "AlternateContent" {
            continue;
        }
        *counts.entry(child.local_name.as_str()).or_insert(0) += 1;
        if !allowed.is_empty() && !allowed.contains_key(child.local_name.as_str()) {
            errors.push(ValidationError {
                path: format!("{path}/{}", child.local_name),
                message: format!(
                    "unexpected child `<{}>` on `<{}>`",
                    child.local_name, element.local_name
                ),
                ..Default::default()
            });
        }
    }

    for rule in rules {
        let n = counts.get(rule.local_name).copied().unwrap_or(0);
        if rule.required && n == 0 {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!(
                    "missing required child `<{}>` on `<{}>`",
                    rule.local_name, element.local_name
                ),
                ..Default::default()
            });
        }
        if rule.max_one && n > 1 {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!(
                    "child `<{}>` occurs {n} times but at most one is allowed on `<{}>`",
                    rule.local_name, element.local_name
                ),
                ..Default::default()
            });
        }
    }

    errors
}

/// Hand-written rules for core WordprocessingML types.
pub mod word {
    use super::ChildRule;

    pub static DOCUMENT: &[ChildRule] = &[
        ChildRule {
            local_name: "background",
            max_one: true,
            required: false,
        },
        ChildRule {
            local_name: "body",
            max_one: true,
            required: true,
        },
    ];

    pub static BODY: &[ChildRule] = &[
        ChildRule {
            local_name: "sectPr",
            max_one: true,
            required: false,
        },
        ChildRule {
            local_name: "p",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "tbl",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "sdt",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "customXml",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "altChunk",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "bookmarkStart",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "bookmarkEnd",
            max_one: false,
            required: false,
        },
    ];

    pub static PARAGRAPH: &[ChildRule] = &[
        ChildRule {
            local_name: "pPr",
            max_one: true,
            required: false,
        },
        ChildRule {
            local_name: "r",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "hyperlink",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "bookmarkStart",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "bookmarkEnd",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "commentRangeStart",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "commentRangeEnd",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "fldSimple",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "sdt",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "customXml",
            max_one: false,
            required: false,
        },
    ];

    pub static RUN: &[ChildRule] = &[
        ChildRule {
            local_name: "rPr",
            max_one: true,
            required: false,
        },
        ChildRule {
            local_name: "t",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "br",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "tab",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "drawing",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "footnoteReference",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "endnoteReference",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "commentReference",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "lastRenderedPageBreak",
            max_one: false,
            required: false,
        },
        ChildRule {
            local_name: "sym",
            max_one: false,
            required: false,
        },
    ];
}

/// Recursively validate a Word document tree using lightweight rules.
pub fn validate_word_document(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if root.local_name == "document" {
        errors.extend(validate_children(root, word::DOCUMENT, "w:document"));
        if let Some(body) = root.child("body") {
            errors.extend(validate_children(body, word::BODY, "w:document/w:body"));
            for (i, p) in body.children_by_name("p").enumerate() {
                let path = format!("w:document/w:body/w:p[{i}]");
                errors.extend(validate_children(p, word::PARAGRAPH, &path));
                for (j, r) in p.children_by_name("r").enumerate() {
                    let rpath = format!("{path}/w:r[{j}]");
                    errors.extend(validate_children(r, word::RUN, &rpath));
                }
            }
        }
    }
    errors
}

/// Full Word validation: lightweight rules + ordered particles.
pub fn validate_word_document_full(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = validate_word_document(root);
    errors.extend(validate_word_particles(root));
    errors.sort_by(|a, b| (&a.path, &a.message).cmp(&(&b.path, &b.message)));
    errors.dedup();
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wordprocessing::{body, document, paragraph, run, text};

    #[test]
    fn document_requires_body() {
        let doc = document(vec![]);
        let errs = validate_word_document(&doc);
        assert!(errs.iter().any(|e| e.message.contains("missing required")));
    }

    #[test]
    fn valid_document_ok() {
        let doc = document(vec![body(vec![paragraph(vec![run(vec![text("hi")])])])]);
        let errs = validate_word_document(&doc);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn unexpected_child() {
        let mut doc = document(vec![body(vec![])]);
        doc.append_child(crate::element::OpenXmlElement::w("notARealChild"));
        let errs = validate_word_document(&doc);
        assert!(errs.iter().any(|e| e.message.contains("unexpected")));
    }

    #[test]
    fn max_one_body() {
        let doc = document(vec![body(vec![]), body(vec![])]);
        let errs = validate_word_document(&doc);
        assert!(errs.iter().any(|e| e.message.contains("at most one")));
    }

    #[test]
    fn full_validation_includes_particles() {
        let doc = document(vec![body(vec![paragraph(vec![run(vec![text("hi")])])])]);
        let errs = validate_word_document_full(&doc);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn error_id_from_mc_message() {
        let e = ValidationError {
            path: "x".into(),
            message: "MC_InvalidIgnorableAttribute: bad prefix".into(),
            ..Default::default()
        };
        assert_eq!(e.id(), Some("MC_InvalidIgnorableAttribute"));
        assert_eq!(e.error_type(), ValidationErrorType::MarkupCompatibility);
    }

    #[test]
    fn related_node_path_roundtrip() {
        let e = ValidationError::new("/a", "x")
            .with_node_path("/a/b")
            .with_related_node_path("/a/c")
            .with_related_part_uri("/word/document.xml");
        assert_eq!(e.node_path.as_deref(), Some("/a/b"));
        assert_eq!(e.related_node_path.as_deref(), Some("/a/c"));
        assert_eq!(e.related_part_uri.as_deref(), Some("/word/document.xml"));
    }
}
