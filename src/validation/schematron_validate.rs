//! Runtime enforcement of extractable Schematron constraint tables.

use super::attributes::validate_attribute_range;
use super::schematron_constraints::{
    schematron_ancestor_unique_rules, schematron_attr_compare_rules,
    schematron_both_present_rules, schematron_conditional_attr_rules,
    schematron_cross_part_count_rules, schematron_cross_part_index_rules,
    schematron_enum_rules, schematron_finite_number_rules, schematron_fixed_bool_rules,
    schematron_fixed_ne_rules, schematron_fixed_value_rules, schematron_multi_ne_rules,
    schematron_nonzero_guid_rules, schematron_numeric_range_rules, schematron_pattern_rules,
    schematron_absent_when_not_rules, schematron_attr_and_enum_rules,
    schematron_bool_pair_impl_rules, schematron_enum_when_flag_rules,
    schematron_mutual_exclusive_rules, schematron_required_attr_rules,
    schematron_special_pattern_rules, schematron_string_length_rules,
};
use std::collections::{HashMap, HashSet};
use super::ValidationError;
use crate::element::{parse_element, OpenXmlElement};
use crate::opc::OpcPackage;

fn attr_value<'a>(el: &'a OpenXmlElement, name: &str) -> Option<&'a str> {
    el.get_attribute(name)
        .or_else(|| el.get_attribute_qname(&format!("w:{name}")))
        .or_else(|| el.get_attribute_qname(&format!("x:{name}")))
        .or_else(|| el.get_attribute_qname(&format!("p:{name}")))
        .or_else(|| el.get_attribute_qname(&format!("r:{name}")))
        .or_else(|| {
            el.attributes
                .iter()
                .find(|a| a.local_name == name)
                .map(|a| a.value.as_str())
        })
}

/// Validate numeric attribute ranges from Schematron-extracted rules.
pub fn validate_schematron_numeric_ranges(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_numeric_range_rules();
    let mut errors = Vec::new();
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            // Skip non-finite sentinels issues: parse and compare
            let min = if rule.min.is_finite() {
                rule.min
            } else {
                f64::MIN
            };
            let max = if rule.max.is_finite() {
                rule.max
            } else {
                f64::MAX
            };
            if let Some(e) = validate_attribute_range(
                &format!("{}:{}", "el", rule.element),
                rule.attribute,
                v,
                min,
                max,
            ) {
                // Prefer cleaner path
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: e.message,
                });
            }
        }
    }
    errors
}

/// Validate string-length constraints from Schematron-extracted rules.
pub fn validate_schematron_string_lengths(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_string_length_rules();
    let mut errors = Vec::new();
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            let len = v.chars().count();
            if len < rule.min || len > rule.max {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` length {len} is outside range [{}, {}]",
                        rule.attribute, rule.min, rule.max
                    ),
                });
            }
        }
    }
    errors
}

/// Very small Schematron `matches()` subset:
/// - exact length dots like `.{1}`
/// - character class + quantifier patterns handled via simplified checks
/// - UUID-ish and hex patterns via dedicated fast paths
///
/// Unsupported patterns are skipped (not reported as errors).
pub fn validate_schematron_patterns(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_pattern_rules();
    let mut errors = Vec::new();
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            match match_schematron_pattern(v, rule.pattern) {
                PatternResult::Ok => {}
                PatternResult::Fail => {
                    errors.push(ValidationError {
                        path: format!("{}/@{}", rule.element, rule.attribute),
                        message: format!(
                            "attribute `{}` value `{v}` does not match pattern `{}`",
                            rule.attribute, rule.pattern
                        ),
                    });
                }
                PatternResult::Unsupported => {}
            }
        }
    }
    errors
}

enum PatternResult {
    Ok,
    Fail,
    Unsupported,
}

fn match_schematron_pattern(value: &str, pattern: &str) -> PatternResult {
    // `.{N}` exact length
    if let Some(n) = pattern
        .strip_prefix(".{")
        .and_then(|s| s.strip_suffix('}'))
        .and_then(|s| s.parse::<usize>().ok())
    {
        return if value.chars().count() == n {
            PatternResult::Ok
        } else {
            PatternResult::Fail
        };
    }
    // hex of fixed length: [0-9a-fA-F]{8}
    if let Some(inner) = pattern
        .strip_prefix("[0-9a-fA-F]{")
        .or_else(|| pattern.strip_prefix("[0-9a-fA-F]{"))
    {
        if let Some(n) = inner.strip_suffix('}').and_then(|s| s.parse::<usize>().ok()) {
            let ok = value.len() == n && value.bytes().all(|b| b.is_ascii_hexdigit());
            return if ok {
                PatternResult::Ok
            } else {
                PatternResult::Fail
            };
        }
    }
    // UUID
    if pattern
        == "[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}"
    {
        let parts: Vec<_> = value.split('-').collect();
        let ok = parts.len() == 5
            && parts[0].len() == 8
            && parts[1].len() == 4
            && parts[2].len() == 4
            && parts[3].len() == 4
            && parts[4].len() == 12
            && value.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
        return if ok {
            PatternResult::Ok
        } else {
            PatternResult::Fail
        };
    }
    // `[^,]*` — no commas
    if pattern == "[^,]*" {
        return if value.contains(',') {
            PatternResult::Fail
        } else {
            PatternResult::Ok
        };
    }
    // `[a-zA-Z_\\][a-zA-Z0-9_.]*` identifier-ish
    if pattern == r"[a-zA-Z_\\][a-zA-Z0-9_.]*" || pattern == r"[a-zA-Z_\\][a-zA-Z0-9_.]*" {
        let mut chars = value.chars();
        let Some(first) = chars.next() else {
            return PatternResult::Fail;
        };
        if !(first.is_ascii_alphabetic() || first == '_' || first == '\\') {
            return PatternResult::Fail;
        }
        if chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '\\') {
            return PatternResult::Ok;
        }
        return PatternResult::Fail;
    }
    // `[^\\d].*` — does not start with digit
    if pattern == r"[^\d].*" || pattern == "[^\\d].*" {
        let mut chars = value.chars();
        return match chars.next() {
            Some(c) if !c.is_ascii_digit() => PatternResult::Ok,
            Some(_) => PatternResult::Fail,
            None => PatternResult::Fail,
        };
    }
    // sheet name forbidden chars subset
    if pattern.contains("[^*") || pattern.contains(r"[^\'*") {
        // Excel sheet name: no * [ ] : ? \ / and not empty
        if value.is_empty() {
            return PatternResult::Fail;
        }
        let forbidden = ['*', '[', ']', ':', '?', '\\', '/'];
        if value.chars().any(|c| forbidden.contains(&c)) {
            return PatternResult::Fail;
        }
        return PatternResult::Ok;
    }
    PatternResult::Unsupported
}

/// Validate enumeration constraints (`@attr = a or @attr = b …`).
pub fn validate_schematron_enums(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_enum_rules();
    let mut errors = Vec::new();
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            if !rule.values.iter().any(|allowed| *allowed == v) {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` value `{v}` is not one of {:?}",
                        rule.attribute, rule.values
                    ),
                });
            }
        }
    }
    errors
}

/// Within each matching ancestor, ensure child element/@attr values are unique.
pub fn validate_schematron_ancestor_unique(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_ancestor_unique_rules();
    let mut errors = Vec::new();
    for rule in &rules {
        // Find all ancestor nodes
        let ancestors: Vec<&OpenXmlElement> = std::iter::once(root)
            .chain(root.descendants())
            .filter(|e| e.local_name == rule.ancestor)
            .collect();
        for anc in ancestors {
            let mut seen: HashMap<String, usize> = HashMap::new();
            for el in anc.descendants() {
                if el.local_name != rule.element {
                    continue;
                }
                let Some(raw) = attr_value(el, rule.attribute) else {
                    continue;
                };
                let key = if rule.case_insensitive {
                    raw.to_ascii_lowercase()
                } else {
                    raw.to_string()
                };
                let count = seen.entry(key).or_insert(0);
                *count += 1;
                if *count == 2 {
                    errors.push(ValidationError {
                        path: format!(
                            "{}/{}/@{}",
                            rule.ancestor, rule.element, rule.attribute
                        ),
                        message: format!(
                            "duplicate {} `@{}` value `{}` under `<{}>`",
                            rule.element, rule.attribute, raw, rule.ancestor
                        ),
                    });
                }
            }
        }
    }
    errors
}

/// When flag attribute equals flag_value, required attribute must be present.
pub fn validate_schematron_conditional_attrs(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_conditional_attr_rules();
    let mut errors = Vec::new();
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let Some(flag_val) = attr_value(el, rule.flag_attribute) else {
                continue;
            };
            if flag_val != rule.flag_value {
                continue;
            }
            if attr_value(el, rule.required_attribute).is_none() {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.required_attribute),
                    message: format!(
                        "attribute `{}` is required on `<{}>` when `@{}` is `{}`",
                        rule.required_attribute,
                        rule.element,
                        rule.flag_attribute,
                        rule.flag_value
                    ),
                });
            }
        }
    }
    errors
}

/// GUID attributes must not be the nil UUID.
pub fn validate_schematron_nonzero_guids(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_nonzero_guid_rules();
    let mut errors = Vec::new();
    const NIL: &str = "00000000-0000-0000-0000-000000000000";
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            if v.eq_ignore_ascii_case(NIL) {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` must not be the nil UUID",
                        rule.attribute
                    ),
                });
            }
        }
    }
    errors
}

/// Same-element attribute comparison (`@left OP @right`).
pub fn validate_schematron_attr_compare(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_attr_compare_rules();
    let mut errors = Vec::new();
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let (Some(left), Some(right)) = (attr_value(el, rule.left), attr_value(el, rule.right))
            else {
                continue;
            };
            let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) else {
                continue;
            };
            let ok = match rule.op {
                "<=" => l <= r,
                ">=" => l >= r,
                "<" => l < r,
                ">" => l > r,
                _ => true,
            };
            if !ok {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.left),
                    message: format!(
                        "attribute `{}` ({left}) is not {} `{}` ({right})",
                        rule.left, rule.op, rule.right
                    ),
                });
            }
        }
    }
    errors
}

/// Fixed boolean attribute values (`@attr = true|false`).
pub fn validate_schematron_fixed_bools(root: &OpenXmlElement) -> Vec<ValidationError> {
    let rules = schematron_fixed_bool_rules();
    let mut errors = Vec::new();
    for el in root.descendants() {
        for rule in &rules {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            let actual = matches!(v, "1" | "true" | "on" | "True");
            if actual != rule.expected {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` must be `{}` (got `{v}`)",
                        rule.attribute, rule.expected
                    ),
                });
            }
        }
    }
    errors
}

fn parse_part_root(package: &OpcPackage, uri: &crate::opc::PackUri) -> Option<OpenXmlElement> {
    let bytes = package.get_part(uri)?;
    let s = std::str::from_utf8(bytes).ok()?;
    parse_element(s).ok()
}

/// Resolve a Schematron `Part:` hint to package part roots.
///
/// Hints are either relative (`.`, `..`) — meaning the current part tree —
/// or named part types / paths. We match by URI path suffix heuristics.
fn resolve_part_roots<'a>(
    package: &'a OpcPackage,
    current_root: &'a OpenXmlElement,
    part_hint: &str,
) -> Vec<OpenXmlElement> {
    if part_hint == "." || part_hint == ".." {
        return vec![current_root.clone()];
    }
    // Map common Part: names / paths to URI substrings
    let needle = part_hint
        .rsplit('/')
        .next()
        .unwrap_or(part_hint)
        .trim_start_matches('/');
    let needle_lower = needle.to_ascii_lowercase();
    let mut out = Vec::new();
    for uri in package.part_uris() {
        let path = uri.as_str().to_ascii_lowercase();
        let matched = match needle_lower.as_str() {
            "footnotepart" | "footnotespart" => path.contains("footnote"),
            "endnotespart" => path.contains("endnote"),
            "wordprocessingcommentspart" => path.contains("comment"),
            "workbookpart" => path.ends_with("/workbook.xml") || path == "/xl/workbook.xml",
            "workbookstylespart" => path.contains("styles"),
            "cellmetadatapart" => path.contains("metadata"),
            "connectionspart" => path.contains("connection"),
            "customxmlmappingspart" => path.contains("xmlmaps") || path.contains("customxml"),
            "pivottablecachedefinitionpart" => path.contains("pivotcache"),
            other => {
                // strip "Part" suffix and match fragment
                let base = other.strip_suffix("part").unwrap_or(other);
                path.contains(base)
            }
        };
        if matched {
            if let Some(root) = parse_part_root(package, &uri) {
                out.push(root);
            }
        }
    }
    out
}

fn collect_attr_values(roots: &[OpenXmlElement], element: &str, attribute: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    for root in roots {
        for el in std::iter::once(root).chain(root.descendants()) {
            if el.local_name != element {
                continue;
            }
            if let Some(v) = attr_value(el, attribute) {
                set.insert(v.to_string());
            }
        }
    }
    set
}

fn count_elements(roots: &[OpenXmlElement], element: &str) -> usize {
    let mut n = 0;
    for root in roots {
        for el in std::iter::once(root).chain(root.descendants()) {
            if el.local_name == element {
                n += 1;
            }
        }
    }
    n
}

/// Cross-part Index-of and count bounds that require package context.
///
/// When the referenced part is missing, the rule is skipped (cannot evaluate).
pub fn validate_schematron_cross_part(
    package: &OpcPackage,
    root: &OpenXmlElement,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Index-of rules
    for rule in schematron_cross_part_index_rules() {
        let targets = resolve_part_roots(package, root, rule.part_hint);
        if targets.is_empty() {
            continue;
        }
        let allowed = collect_attr_values(&targets, rule.target_element, rule.target_attribute);
        if allowed.is_empty() {
            continue;
        }
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            if !allowed.contains(v) {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` value `{v}` not found among `{}/@{}` (part `{}`)",
                        rule.attribute, rule.target_element, rule.target_attribute, rule.part_hint
                    ),
                });
            }
        }
    }

    // Count bounds: @attr < count(target) + offset
    for rule in schematron_cross_part_count_rules() {
        let targets = resolve_part_roots(package, root, rule.part_hint);
        if targets.is_empty() {
            continue;
        }
        let count = count_elements(&targets, rule.target_element) as i64;
        let limit = count + rule.offset;
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            let Ok(n) = v.parse::<i64>() else {
                continue;
            };
            if n >= limit {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` value {n} must be < {limit} (count of `{}` + {})",
                        rule.attribute, rule.target_element, rule.offset
                    ),
                });
            }
        }
    }

    errors
}


/// Fixed literal equality (`@attr = value`).
pub fn validate_schematron_fixed_values(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_fixed_value_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            if v != rule.value {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` must be `{}` (got `{v}`)",
                        rule.attribute, rule.value
                    ),
                });
            }
        }
    }
    errors
}

/// Fixed literal inequality (`@attr != value`).
pub fn validate_schematron_fixed_nes(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_fixed_ne_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            if v == rule.forbidden {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` must not be `{}`",
                        rule.attribute, rule.forbidden
                    ),
                });
            }
        }
    }
    errors
}

/// Multi-value inequality (`@attr != a and @attr != b …`).
pub fn validate_schematron_multi_nes(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_multi_ne_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            if rule.forbidden.iter().any(|f| *f == v) {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` value `{v}` is forbidden ({:?})",
                        rule.attribute, rule.forbidden
                    ),
                });
            }
        }
    }
    errors
}

/// Both attributes must be present together.
pub fn validate_schematron_both_present(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_both_present_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let has_l = attr_value(el, rule.left).is_some();
            let has_r = attr_value(el, rule.right).is_some();
            if has_l != has_r {
                errors.push(ValidationError {
                    path: format!("{}", rule.element),
                    message: format!(
                        "attributes `{}` and `{}` must both be present or both absent",
                        rule.left, rule.right
                    ),
                });
            }
        }
    }
    errors
}

/// Attribute must not be NaN / INF / -INF.
pub fn validate_schematron_finite_numbers(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_finite_number_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            let bad = matches!(
                v,
                "NaN" | "nan" | "INF" | "Inf" | "inf" | "-INF" | "-Inf" | "-inf"
            );
            if bad {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` must be a finite number (got `{v}`)",
                        rule.attribute
                    ),
                });
            }
        }
    }
    errors
}

/// Attribute must be present when element exists.
pub fn validate_schematron_required_attrs(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_required_attr_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            if attr_value(el, rule.attribute).is_none() {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` is required on `<{}>`",
                        rule.attribute, rule.element
                    ),
                });
            }
        }
    }
    errors
}


/// Attribute must be absent when condition attribute is not one of the allowed values (1.15).
pub fn validate_schematron_absent_when_not(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_absent_when_not_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(_) = attr_value(el, rule.absent_attribute) else {
                continue;
            };
            let Some(cond) = attr_value(el, rule.condition_attribute) else {
                // condition missing — absent should not be present? C#: if condition missing, still error when absent present
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.absent_attribute),
                    message: format!(
                        "attribute `{}` must be absent unless `@{}` is one of {:?}",
                        rule.absent_attribute, rule.condition_attribute, rule.allowed_values
                    ),
                });
                continue;
            };
            if !rule.allowed_values.iter().any(|v| *v == cond) {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.absent_attribute),
                    message: format!(
                        "attribute `{}` must be absent when `@{}` is `{cond}` (allowed: {:?})",
                        rule.absent_attribute, rule.condition_attribute, rule.allowed_values
                    ),
                });
            }
        }
    }
    errors
}

/// At most one of the listed attributes may be present (1.16 mutual exclusive).
pub fn validate_schematron_mutual_exclusive(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_mutual_exclusive_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let present: Vec<&str> = rule
                .attributes
                .iter()
                .filter(|a| attr_value(el, a).is_some())
                .copied()
                .collect();
            if present.len() > 1 {
                errors.push(ValidationError {
                    path: rule.element.to_string(),
                    message: format!(
                        "attributes {:?} are mutually exclusive; found {:?}",
                        rule.attributes, present
                    ),
                });
            }
        }
    }
    errors
}


/// When flag equals flag_value, other_attribute must equal other_value.
pub fn validate_schematron_bool_pair_impl(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_bool_pair_impl_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(flag) = attr_value(el, rule.flag_attribute) else {
                continue;
            };
            if flag != rule.flag_value {
                continue;
            }
            let actual = attr_value(el, rule.other_attribute);
            if actual != Some(rule.other_value) {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.other_attribute),
                    message: format!(
                        "attribute `{}` must be `{}` when `@{}` is `{}` (got `{}`)",
                        rule.other_attribute,
                        rule.other_value,
                        rule.flag_attribute,
                        rule.flag_value,
                        actual.unwrap_or("<missing>")
                    ),
                });
            }
        }
    }
    errors
}

/// When required_attribute is present, flag_attribute must be one of flag_values.
pub fn validate_schematron_attr_and_enum(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_attr_and_enum_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            if attr_value(el, rule.required_attribute).is_none() {
                continue;
            }
            let Some(flag) = attr_value(el, rule.flag_attribute) else {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.flag_attribute),
                    message: format!(
                        "attribute `{}` is required when `{}` is present (allowed: {:?})",
                        rule.flag_attribute, rule.required_attribute, rule.flag_values
                    ),
                });
                continue;
            };
            if !rule.flag_values.iter().any(|v| *v == flag) {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.flag_attribute),
                    message: format!(
                        "attribute `{}` value `{flag}` not in {:?} when `{}` present",
                        rule.flag_attribute, rule.flag_values, rule.required_attribute
                    ),
                });
            }
        }
    }
    errors
}


/// When flag is one of flag_values, other must be one of other_values.
pub fn validate_schematron_enum_when_flag(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_enum_when_flag_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(flag) = attr_value(el, rule.flag_attribute) else {
                continue;
            };
            if !rule.flag_values.iter().any(|v| *v == flag) {
                continue;
            }
            let actual = attr_value(el, rule.other_attribute);
            let ok = actual
                .map(|a| rule.other_values.iter().any(|v| *v == a))
                .unwrap_or(false);
            if !ok {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.other_attribute),
                    message: format!(
                        "attribute `{}` must be one of {:?} when `@{}` is `{}` (got `{}`)",
                        rule.other_attribute,
                        rule.other_values,
                        rule.flag_attribute,
                        flag,
                        actual.unwrap_or("<missing>")
                    ),
                });
            }
        }
    }
    errors
}

/// Special pattern kinds: excel_sheet_name / excel_codename.
pub fn validate_schematron_special_patterns(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for rule in schematron_special_pattern_rules() {
        for el in root.descendants() {
            if el.local_name != rule.element {
                continue;
            }
            let Some(v) = attr_value(el, rule.attribute) else {
                continue;
            };
            let ok = match rule.kind {
                "excel_sheet_name" => {
                    !v.is_empty()
                        && !v.chars().any(|c| matches!(c, '*' | '[' | ']' | ':' | '?' | '\\' | '/'))
                        && !v.starts_with('\'')
                }
                "excel_codename" => {
                    let mut chars = v.chars();
                    match chars.next() {
                        Some(c) if c.is_alphabetic() || c == '_' || !c.is_ascii() => {
                            chars.all(|c| c.is_alphanumeric() || c == '_' || !c.is_ascii())
                        }
                        _ => false,
                    }
                }
                _ => true,
            };
            if !ok {
                errors.push(ValidationError {
                    path: format!("{}/@{}", rule.element, rule.attribute),
                    message: format!(
                        "attribute `{}` value `{v}` fails special pattern `{}`",
                        rule.attribute, rule.kind
                    ),
                });
            }
        }
    }
    errors
}

/// Run all extractable Schematron attribute constraints (single-tree, no package).
pub fn validate_schematron_constraints(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = validate_schematron_numeric_ranges(root);
    errors.extend(validate_schematron_string_lengths(root));
    errors.extend(validate_schematron_patterns(root));
    errors.extend(validate_schematron_enums(root));
    errors.extend(validate_schematron_ancestor_unique(root));
    errors.extend(validate_schematron_conditional_attrs(root));
    errors.extend(validate_schematron_nonzero_guids(root));
    errors.extend(validate_schematron_attr_compare(root));
    errors.extend(validate_schematron_fixed_bools(root));
    errors.extend(validate_schematron_fixed_values(root));
    errors.extend(validate_schematron_fixed_nes(root));
    errors.extend(validate_schematron_multi_nes(root));
    errors.extend(validate_schematron_both_present(root));
    errors.extend(validate_schematron_finite_numbers(root));
    errors.extend(validate_schematron_required_attrs(root));
    errors.extend(validate_schematron_absent_when_not(root));
    errors.extend(validate_schematron_mutual_exclusive(root));
    errors.extend(validate_schematron_bool_pair_impl(root));
    errors.extend(validate_schematron_attr_and_enum(root));
    errors.extend(validate_schematron_enum_when_flag(root));
    errors.extend(validate_schematron_special_patterns(root));
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;

    #[test]
    fn sheet_id_out_of_range() {
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
            "sheets",
        )
        .with_child(sheet);
        let errs = validate_schematron_numeric_ranges(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("outside range")),
            "{errs:?}"
        );
    }

    #[test]
    fn sheet_name_too_long() {
        let sheet = OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "sheet",
        )
        .with_attribute("name", "abcdefghijklmnopqrstuvwxyz012345"); // 32 chars
        let root = OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "sheets",
        )
        .with_child(sheet);
        let errs = validate_schematron_string_lengths(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("length")),
            "{errs:?}"
        );
    }

    #[test]
    fn decimal_symbol_length() {
        let el = OpenXmlElement::w("decimalSymbol").with_attribute_qname("w:val", "ab");
        let root = OpenXmlElement::w("settings").with_child(el);
        let errs = validate_schematron_patterns(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("does not match")),
            "{errs:?}"
        );
    }

    #[test]
    fn enum_rejects_unknown_value() {
        // VML arc dgmlayout must be 0..3
        let el = OpenXmlElement::new(
            "v",
            "urn:schemas-microsoft-com:vml",
            "arc",
        )
        .with_attribute("dgmlayout", "9");
        let root = OpenXmlElement::new(
            "v",
            "urn:schemas-microsoft-com:vml",
            "shape",
        )
        .with_child(el);
        let errs = validate_schematron_enums(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("not one of")),
            "{errs:?}"
        );
    }

    #[test]
    fn ancestor_unique_detects_dup() {
        // protectedRange/@name unique under protectedRanges
        let mut ranges = OpenXmlElement::x("protectedRanges");
        ranges.append_child(
            OpenXmlElement::x("protectedRange").with_attribute("name", "A"),
        );
        ranges.append_child(
            OpenXmlElement::x("protectedRange").with_attribute("name", "A"),
        );
        let root = OpenXmlElement::x("worksheet").with_child(ranges);
        let errs = validate_schematron_ancestor_unique(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("duplicate")),
            "{errs:?}"
        );
    }

    #[test]
    fn conditional_attr_requires_operator_for_cells() {
        let rule = OpenXmlElement::x("cfRule")
            .with_attribute("type", "cells");
        // missing operator
        let root = OpenXmlElement::x("conditionalFormatting").with_child(rule);
        let errs = validate_schematron_conditional_attrs(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("required")),
            "{errs:?}"
        );
    }

    #[test]
    fn nonzero_guid_rejects_nil() {
        let el = OpenXmlElement::x("customSheetView").with_attribute(
            "guid",
            "00000000-0000-0000-0000-000000000000",
        );
        let root = OpenXmlElement::x("worksheet").with_child(el);
        let errs = validate_schematron_nonzero_guids(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("nil UUID")),
            "{errs:?}"
        );
    }

    #[test]
    fn attr_compare_detects_inverted_range() {
        let col = OpenXmlElement::x("col")
            .with_attribute("min", "5")
            .with_attribute("max", "2");
        let root = OpenXmlElement::x("cols").with_child(col);
        let errs = validate_schematron_attr_compare(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("is not")),
            "{errs:?}"
        );
    }

    #[test]
    fn fixed_bool_rejects_true_when_false_required() {
        let f = OpenXmlElement::x("f").with_attribute("bx", "true");
        let root = OpenXmlElement::x("c").with_child(f);
        let errs = validate_schematron_fixed_bools(&root);
        assert!(
            errs.iter().any(|e| e.message.contains("must be")),
            "{errs:?}"
        );
    }

    #[test]
    fn cross_part_index_and_count() {
        use crate::namespace::content_type;
        use crate::opc::{OpcPackage, PackUri};

        let mut pkg = OpcPackage::create();
        // styles with one xf
        let styles = r#"<?xml version="1.0"?><styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><cellXfs><xf/></cellXfs></styleSheet>"#;
        pkg.set_part(
            "/xl/styles.xml",
            content_type::SPREADSHEET_STYLES,
            styles.as_bytes().to_vec(),
        );
        // worksheet cell with style index out of range
        let sheet = OpenXmlElement::x("worksheet").with_child(
            OpenXmlElement::x("sheetData").with_child(
                OpenXmlElement::x("row").with_child(OpenXmlElement::x("c").with_attribute("s", "5")),
            ),
        );
        let errs = validate_schematron_cross_part(&pkg, &sheet);
        assert!(
            errs.iter().any(|e| e.message.contains("must be <")),
            "{errs:?}"
        );

        // index-of: commentReference without matching comment
        let comments = r#"<?xml version="1.0"?><comments xmlns="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><comment w:id="1" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/></comments>"#;
        pkg.set_part(
            "/word/comments.xml",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
            comments.as_bytes().to_vec(),
        );
        let para = OpenXmlElement::w("p").with_child(
            OpenXmlElement::w("commentReference").with_attribute_qname("w:id", "99"),
        );
        let errs = validate_schematron_cross_part(&pkg, &para);
        assert!(
            errs.iter().any(|e| e.message.contains("not found")),
            "{errs:?}"
        );
        let _ = PackUri::new("/word/comments.xml");
    }
}
