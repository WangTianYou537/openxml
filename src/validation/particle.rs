//! Ordered particle matching (sequence / choice / group / all / element).
//!
//! Particles follow the XSD-inspired content models used in the C# Open XML SDK.
//! Both hand-authored core models and schema-generated particles use the same
//! owned [`Particle`] tree so codegen can emit them freely.

use crate::element::OpenXmlElement;
use crate::file_format::FileFormatVersions;
use crate::markup_compatibility::McContext;
use crate::validation::{
    ValidationContext, ValidationError, ValidationSettings,
};
use std::fmt;

/// Occurrence constraints for a particle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occurs {
    pub min: u32,
    /// `None` means unbounded.
    pub max: Option<u32>,
}

impl Occurs {
    pub const ONE: Occurs = Occurs {
        min: 1,
        max: Some(1),
    };
    pub const OPTIONAL: Occurs = Occurs {
        min: 0,
        max: Some(1),
    };
    pub const STAR: Occurs = Occurs { min: 0, max: None };
    pub const PLUS: Occurs = Occurs { min: 1, max: None };

    pub fn new(min: u32, max: Option<u32>) -> Self {
        Self { min, max }
    }

    /// Parse from schema JSON `Occurs: [{Min?, Max?}]` (first entry).
    pub fn from_schema(occurs: Option<&serde_json::Value>) -> Self {
        // Default when missing: unbounded (min 0)
        let Some(arr) = occurs.and_then(|v| v.as_array()) else {
            return Occurs::STAR;
        };
        let Some(first) = arr.first() else {
            return Occurs::STAR;
        };
        let min = first.get("Min").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let max = first.get("Max").and_then(|v| v.as_u64()).map(|v| v as u32);
        Occurs { min, max }
    }
}

/// A content-model particle (owned tree — suitable for codegen).
#[derive(Debug, Clone)]
pub enum Particle {
    /// Named element matched by local name (prefix ignored).
    Element { local_name: String, occurs: Occurs },
    /// Ordered sequence of particles.
    Sequence { items: Vec<Particle>, occurs: Occurs },
    /// Exactly one of the alternatives (per occurrence).
    Choice { items: Vec<Particle>, occurs: Occurs },
    /// Transparent grouping (same as Sequence for matching).
    Group { items: Vec<Particle>, occurs: Occurs },
    /// XSD `xs:all` — treat as unordered choice-star of each item once (simplified: choice of items each optional, then require each min).
    All { items: Vec<Particle>, occurs: Occurs },
    /// Any element (wildcard) with a namespace mode (C# `AnyParticle` + `XsdAny`).
    /// When `uri` is `Some`, matches only that namespace (`ParticleType::AnyWithUri`).
    Any {
        occurs: Occurs,
        namespace: XsdAnyNamespace,
        uri: Option<String>,
    },
    /// Version-gated particle (C# `ParticleConstraint.Version` + `Build`).
    Versioned {
        version: FileFormatVersions,
        inner: Box<Particle>,
    },
}

/// C# `XsdAny` — namespace constraint modes for `xs:any` wildcards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XsdAnyNamespace {
    /// `##any` — elements from any namespace.
    #[default]
    Any,
    /// `##other` — any namespace other than the parent's target namespace.
    Other,
    /// `##local` — unqualified elements only.
    Local,
    /// `##targetNamespace` — the parent's target namespace only.
    TargetNamespace,
}

/// C# `ParticleType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleType {
    Element = 0,
    All,
    Any,
    Choice,
    Group,
    Sequence,
    AnyWithUri,
    Invalid,
}

impl Particle {
    /// C# `ParticleConstraint.ParticleType`.
    pub fn particle_type(&self) -> ParticleType {
        match self {
            Particle::Element { .. } => ParticleType::Element,
            Particle::All { .. } => ParticleType::All,
            Particle::Any { namespace, uri, .. } => {
                if uri.is_some() {
                    ParticleType::AnyWithUri
                } else {
                    match namespace {
                        XsdAnyNamespace::Any
                        | XsdAnyNamespace::Other
                        | XsdAnyNamespace::Local
                        | XsdAnyNamespace::TargetNamespace => ParticleType::Any,
                    }
                }
            }
            Particle::Choice { .. } => ParticleType::Choice,
            Particle::Group { .. } => ParticleType::Group,
            Particle::Sequence { .. } => ParticleType::Sequence,
            Particle::Versioned { inner, .. } => inner.particle_type(),
        }
    }
}

impl XsdAnyNamespace {
    /// Wildcard token used in expected-children messages.
    pub fn token(self) -> &'static str {
        match self {
            XsdAnyNamespace::Any => "##any",
            XsdAnyNamespace::Other => "##other",
            XsdAnyNamespace::Local => "##local",
            XsdAnyNamespace::TargetNamespace => "##targetNamespace",
        }
    }

    fn matches(self, element_ns: &str, target_ns: &str) -> bool {
        match self {
            XsdAnyNamespace::Any => true,
            XsdAnyNamespace::Local => element_ns.is_empty(),
            XsdAnyNamespace::Other => !element_ns.is_empty() && element_ns != target_ns,
            XsdAnyNamespace::TargetNamespace => !element_ns.is_empty() && element_ns == target_ns,
        }
    }

    /// C# `XsdAnyExtensions.GetNamespaceString`.
    pub fn namespace_string(self) -> &'static str {
        self.token()
    }
}

impl Particle {
    pub fn element(local_name: impl Into<String>, occurs: Occurs) -> Self {
        Particle::Element {
            local_name: local_name.into(),
            occurs,
        }
    }

    pub fn sequence(items: Vec<Particle>, occurs: Occurs) -> Self {
        Particle::Sequence { items, occurs }
    }

    pub fn choice(items: Vec<Particle>, occurs: Occurs) -> Self {
        Particle::Choice { items, occurs }
    }

    pub fn group(items: Vec<Particle>, occurs: Occurs) -> Self {
        Particle::Group { items, occurs }
    }

    pub fn all(items: Vec<Particle>, occurs: Occurs) -> Self {
        Particle::All { items, occurs }
    }

    pub fn any(occurs: Occurs) -> Self {
        Particle::Any {
            occurs,
            namespace: XsdAnyNamespace::Any,
            uri: None,
        }
    }

    pub fn any_with_namespace(occurs: Occurs, namespace: XsdAnyNamespace) -> Self {
        Particle::Any {
            occurs,
            namespace,
            uri: None,
        }
    }

    /// C# `AnyParticle` with an explicit namespace URI (`ParticleType::AnyWithUri`).
    pub fn any_with_uri(occurs: Occurs, uri: impl Into<String>) -> Self {
        Particle::Any {
            occurs,
            namespace: XsdAnyNamespace::Any,
            uri: Some(uri.into()),
        }
    }

    /// Gate `inner` on `version` (C# particle `Version` property).
    pub fn versioned(version: FileFormatVersions, inner: Particle) -> Self {
        Particle::Versioned {
            version,
            inner: Box::new(inner),
        }
    }

    /// C# `ParticleConstraint.Build(version)` — prune subtrees whose version is
    /// not satisfied by the target. Returns `None` when this particle itself is
    /// filtered out.
    pub fn build_for(&self, version: FileFormatVersions) -> Option<Particle> {
        match self {
            Particle::Versioned {
                version: required,
                inner,
            } => {
                if version.at_least(*required) {
                    inner.build_for(version)
                } else {
                    None
                }
            }
            Particle::Element { .. } | Particle::Any { .. } => Some(self.clone()),
            Particle::Sequence { items, occurs } => Some(Particle::Sequence {
                items: items.iter().filter_map(|i| i.build_for(version)).collect(),
                occurs: *occurs,
            }),
            Particle::Choice { items, occurs } => Some(Particle::Choice {
                items: items.iter().filter_map(|i| i.build_for(version)).collect(),
                occurs: *occurs,
            }),
            Particle::Group { items, occurs } => Some(Particle::Group {
                items: items.iter().filter_map(|i| i.build_for(version)).collect(),
                occurs: *occurs,
            }),
            Particle::All { items, occurs } => Some(Particle::All {
                items: items.iter().filter_map(|i| i.build_for(version)).collect(),
                occurs: *occurs,
            }),
        }
    }

    fn occurs(&self) -> Occurs {
        match self {
            Particle::Element { occurs, .. }
            | Particle::Sequence { occurs, .. }
            | Particle::Choice { occurs, .. }
            | Particle::Group { occurs, .. }
            | Particle::All { occurs, .. }
            | Particle::Any { occurs, .. } => *occurs,
            Particle::Versioned { inner, .. } => inner.occurs(),
        }
    }

    /// Build a particle tree from schema JSON (as stored under `"Particle"`).
    pub fn from_schema_json(value: &serde_json::Value) -> Option<Self> {
        from_schema_value(value)
    }
}

fn local_from_name(name: &str) -> String {
    let elem = name.split('/').nth(1).unwrap_or(name);
    elem.rsplit(':').next().unwrap_or(elem).to_string()
}

fn from_schema_value(value: &serde_json::Value) -> Option<Particle> {
    // Leaf element form: { "Name": "w:CT_Body/w:body", "Occurs": [...] }
    if let Some(name) = value.get("Name").and_then(|v| v.as_str()) {
        let occurs = Occurs::from_schema(value.get("Occurs"));
        return Some(Particle::element(local_from_name(name), occurs));
    }

    let kind = value.get("Kind").and_then(|v| v.as_str())?;
    let occurs = Occurs::from_schema(value.get("Occurs"));
    let items: Vec<Particle> = value
        .get("Items")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .filter_map(from_schema_value)
        .collect();

    match kind {
        "Sequence" => Some(Particle::sequence(items, occurs)),
        "Choice" => Some(Particle::choice(items, occurs)),
        "Group" => Some(Particle::group(items, occurs)),
        "All" => Some(Particle::all(items, occurs)),
        "Any" | "any" => Some(Particle::any(occurs)),
        _ => {
            // Unknown kind with Name already handled; treat as group
            if items.is_empty() {
                None
            } else {
                Some(Particle::group(items, occurs))
            }
        }
    }
}

struct MatchResult {
    consumed: usize,
    errors: Vec<String>,
}

/// Particle match result (C# `ParticleMatch`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParticleMatch {
    #[default]
    Nomatch,
    Partial,
    Matched,
}

/// Expected children collected for error reporting (C# `ExpectedChildren`).
#[derive(Debug, Clone, Default)]
pub struct ExpectedChildren {
    elements: Vec<String>,
    xsd_any_namespaces: Vec<String>,
}

impl ExpectedChildren {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a known child element name (C# `Add(OpenXmlSchemaType)`).
    pub fn add_element(&mut self, name: impl Into<String>) {
        self.elements.push(name.into());
    }

    /// Add the namespace of an `xsd:any` child (C# `Add(string)`).
    pub fn add_any_namespace(&mut self, namespace_uri: impl Into<String>) {
        self.xsd_any_namespaces.push(namespace_uri.into());
    }

    /// Merge all entries from another set (C# `Add(ExpectedChildren)`).
    pub fn add_all(&mut self, other: &ExpectedChildren) {
        self.elements.extend(other.elements.iter().cloned());
        self.xsd_any_namespaces
            .extend(other.xsd_any_namespaces.iter().cloned());
    }

    pub fn count(&self) -> usize {
        self.elements.len() + self.xsd_any_namespaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.xsd_any_namespaces.clear();
    }

    pub fn elements(&self) -> &[String] {
        &self.elements
    }

    pub fn any_namespaces(&self) -> &[String] {
        &self.xsd_any_namespaces
    }

    /// C# `GetExpectedChildrenMessage` — `" List of possible elements expected: <a>,<b>."`.
    pub fn expected_children_message(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut names: Vec<String> = self
            .elements
            .iter()
            .map(|name| format!("<{name}>"))
            .collect();
        names.extend(
            self.xsd_any_namespaces
                .iter()
                .map(|ns| format!("any element in namespace '{ns}'")),
        );
        format!(" List of possible elements expected: {}.", names.join(","))
    }
}

/// Match bookkeeping for one particle-match attempt (C# `ParticleMatchInfo`).
#[derive(Debug, Clone, Default)]
pub struct ParticleMatchInfo {
    pub match_result: ParticleMatch,
    pub start_element: Option<String>,
    pub last_matched_element: Option<String>,
    pub error_message: Option<String>,
    expected_children: ExpectedChildren,
}

impl ParticleMatchInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_start_element(start_element: impl Into<String>) -> Self {
        Self {
            start_element: Some(start_element.into()),
            ..Self::default()
        }
    }

    pub fn expected_children(&self) -> &ExpectedChildren {
        &self.expected_children
    }

    pub fn expected_children_mut(&mut self) -> &mut ExpectedChildren {
        &mut self.expected_children
    }

    /// Replace the expected-children set (C# `SetExpectedChildren`).
    pub fn set_expected_children(&mut self, expected: &ExpectedChildren) {
        self.expected_children.clear();
        if !expected.is_empty() {
            self.expected_children.add_all(expected);
        }
    }

    /// C# `Reset(startElement)` — keep the allocation, clear the state.
    pub fn reset(&mut self, start_element: Option<&str>) {
        self.start_element = start_element.map(str::to_string);
        self.match_result = ParticleMatch::Nomatch;
        self.last_matched_element = None;
        self.error_message = None;
        self.expected_children.clear();
    }
}

/// Elements with `minOccurs > 0` in this particle (C# `GetRequiredElements`).
pub fn get_required_elements(particle: &Particle, result: &mut ExpectedChildren) -> bool {
    match particle {
        Particle::Element { local_name, occurs } => {
            if occurs.min > 0 {
                result.add_element(local_name.clone());
                true
            } else {
                false
            }
        }
        Particle::Any { occurs, namespace, uri } => {
            if occurs.min > 0 {
                if let Some(uri) = uri {
                    result.add_any_namespace(uri.clone());
                } else {
                    result.add_any_namespace(namespace.token());
                }
                true
            } else {
                false
            }
        }
        Particle::Choice { items, occurs } => {
            // C# ChoiceParticleValidator: required only when every alternative is required.
            if occurs.min == 0 {
                return false;
            }
            let mut choice_children = ExpectedChildren::new();
            let mut required = !items.is_empty();
            for item in items {
                if !get_required_elements(item, &mut choice_children) {
                    required = false;
                }
            }
            if required {
                result.add_all(&choice_children);
            }
            required
        }
        Particle::Sequence { items, occurs }
        | Particle::Group { items, occurs }
        | Particle::All { items, occurs } => {
            if occurs.min == 0 {
                return false;
            }
            let mut required = false;
            for item in items {
                if get_required_elements(item, result) {
                    required = true;
                }
            }
            required
        }
        Particle::Versioned { inner, .. } => get_required_elements(inner, result),
    }
}

/// All elements this particle may start with (C# `GetExpectedElements`).
pub fn get_expected_elements(particle: &Particle, result: &mut ExpectedChildren) -> bool {
    match particle {
        Particle::Element { local_name, .. } => {
            result.add_element(local_name.clone());
            true
        }
        Particle::Any { namespace, uri, .. } => {
            if let Some(uri) = uri {
                result.add_any_namespace(uri.clone());
            } else {
                result.add_any_namespace(namespace.token());
            }
            true
        }
        Particle::Sequence { items, .. }
        | Particle::Choice { items, .. }
        | Particle::Group { items, .. }
        | Particle::All { items, .. } => {
            for item in items {
                get_expected_elements(item, result);
            }
            true
        }
        Particle::Versioned { inner, .. } => get_expected_elements(inner, result),
    }
}

/// Validate that `element`'s children match `particle` in order.
pub fn validate_particle(
    element: &OpenXmlElement,
    particle: &Particle,
    path: &str,
) -> Vec<ValidationError> {
    validate_particle_for_version(
        element,
        particle,
        path,
        FileFormatVersions::OFFICE2007,
    )
}

/// Version-aware particle validation over logical MC children.
pub fn validate_particle_for_version(
    element: &OpenXmlElement,
    particle: &Particle,
    path: &str,
    version: FileFormatVersions,
) -> Vec<ValidationError> {
    let mut context = ValidationContext::new(ValidationSettings::new(version));
    // Always collect expected children for Sch_* mismatch messages (C# re-validates
    // with CollectExpectedChildren when emitting invalid/incomplete content errors).
    context.set_collect_expected_children(true);
    validate_particle_with_context(element, particle, path, &context, &McContext::new())
}

/// Particle validation using an existing [`ValidationContext`] and MC context.
pub fn validate_particle_with_context(
    element: &OpenXmlElement,
    particle: &Particle,
    path: &str,
    context: &ValidationContext,
    mc_context: &McContext,
) -> Vec<ValidationError> {
    let validation_children = context.validation_children_with_context(element, mc_context);
    let children: Vec<&OpenXmlElement> = validation_children
        .iter()
        .map(|child| child.element)
        .collect();

    // C# ValidationCache.GetConstraint: build the version-pruned particle first.
    let built = particle.build_for(context.file_format());
    let Some(particle) = built.as_ref() else {
        return Vec::new();
    };

    let result = match_particle(particle, &children, 0, element.namespace_uri.as_str());
    let mut errors = Vec::new();

    if result.consumed < children.len() {
        let extra = children[result.consumed];
        let mut expected_suffix = String::new();
        if context.collect_expected_children {
            let mut expected = ExpectedChildren::new();
            get_expected_elements(particle, &mut expected);
            expected_suffix = expected.expected_children_message();
        }
        let child_name = if extra.prefix.is_empty() {
            extra.local_name.clone()
        } else {
            format!("{}:{}", extra.prefix, extra.local_name)
        };
        let description = format!(
            "The element has invalid child element '{child_name}'.{expected_suffix}"
        );
        errors.push(
            ValidationError::with_id(
                format!("{path}/{}", extra.local_name),
                "Sch_InvalidElementContentExpectingComplex",
                description,
            )
            .with_error_type(crate::validation::ValidationErrorType::Schema),
        );
    }
    for e in result.errors {
        let incomplete = e.contains("requires at least") || e.contains("required particle");
        let (id, mut description) = if incomplete {
            (
                "Sch_IncompleteContentExpectingComplex",
                format!("The element has incomplete content. {e}"),
            )
        } else {
            (
                "Sch_InvalidElementContentExpectingComplex",
                e,
            )
        };
        if context.collect_expected_children {
            let mut expected = ExpectedChildren::new();
            if incomplete {
                get_required_elements(particle, &mut expected);
            }
            if expected.is_empty() {
                get_expected_elements(particle, &mut expected);
            }
            description.push_str(&expected.expected_children_message());
        }
        errors.push(
            ValidationError::with_id(path, id, description)
                .with_error_type(crate::validation::ValidationErrorType::Schema),
        );
    }
    errors
}

fn match_particle(
    particle: &Particle,
    children: &[&OpenXmlElement],
    start: usize,
    target_ns: &str,
) -> MatchResult {
    let occurs = particle.occurs();
    let mut total_consumed = 0usize;
    let mut count = 0u32;
    let mut errors = Vec::new();

    loop {
        if let Some(max) = occurs.max {
            if count >= max {
                break;
            }
        }
        let pos = start + total_consumed;
        if pos > children.len() {
            break;
        }
        let one = match_once(particle, children, pos, target_ns);
        if one.consumed == 0 {
            errors.extend(one.errors);
            break;
        }
        total_consumed += one.consumed;
        count += 1;
    }

    if count < occurs.min {
        errors.push(format!(
            "particle requires at least {} occurrence(s), found {count}",
            occurs.min
        ));
    }

    MatchResult {
        consumed: total_consumed,
        errors,
    }
}

fn match_once(
    particle: &Particle,
    children: &[&OpenXmlElement],
    start: usize,
    target_ns: &str,
) -> MatchResult {
    match particle {
        Particle::Element { local_name, .. } => {
            if let Some(child) = children.get(start) {
                if child.local_name == local_name.as_str() {
                    return MatchResult {
                        consumed: 1,
                        errors: Vec::new(),
                    };
                }
            }
            MatchResult {
                consumed: 0,
                errors: Vec::new(),
            }
        }
        Particle::Any { namespace, uri, .. } => {
            let matches = children.get(start).is_some_and(|child| {
                if let Some(uri) = uri {
                    child.namespace_uri.as_str() == uri.as_str()
                } else {
                    namespace.matches(child.namespace_uri.as_str(), target_ns)
                }
            });
            if matches {
                MatchResult {
                    consumed: 1,
                    errors: Vec::new(),
                }
            } else {
                MatchResult {
                    consumed: 0,
                    errors: Vec::new(),
                }
            }
        }
        Particle::Sequence { items, .. } | Particle::Group { items, .. } => {
            let mut total = 0usize;
            let mut errors = Vec::new();
            for item in items {
                let r = match_particle(item, children, start + total, target_ns);
                if r.consumed == 0 && item.occurs().min > 0 {
                    errors.extend(r.errors);
                    errors.push(format!(
                        "sequence: required particle did not match at position {}",
                        start + total
                    ));
                    return MatchResult {
                        consumed: 0,
                        errors,
                    };
                }
                total += r.consumed;
                errors.extend(r.errors);
            }
            MatchResult {
                consumed: total,
                errors,
            }
        }
        Particle::Choice { items, .. } => {
            let mut best: Option<MatchResult> = None;
            for item in items {
                let r = match_particle(item, children, start, target_ns);
                if r.consumed > 0 && r.errors.is_empty() {
                    match &best {
                        Some(b) if b.consumed >= r.consumed => {}
                        _ => best = Some(r),
                    }
                } else if r.consumed > 0 && best.is_none() {
                    best = Some(r);
                }
            }
            best.unwrap_or(MatchResult {
                consumed: 0,
                errors: Vec::new(),
            })
        }
        Particle::All { items, .. } => {
            // Simplified xs:all: each item may appear at most once, order free.
            // Greedy: repeatedly pick any unmatched item that matches next child.
            let mut used = vec![false; items.len()];
            let mut total = 0usize;
            let mut errors = Vec::new();
            loop {
                let pos = start + total;
                if pos >= children.len() {
                    break;
                }
                let mut matched = false;
                for (i, item) in items.iter().enumerate() {
                    if used[i] {
                        continue;
                    }
                    let r = match_particle(item, children, pos, target_ns);
                    if r.consumed > 0 && r.errors.is_empty() {
                        used[i] = true;
                        total += r.consumed;
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    break;
                }
            }
            for (i, item) in items.iter().enumerate() {
                if !used[i] && item.occurs().min > 0 {
                    errors.push(format!(
                        "xs:all: required particle #{} not matched",
                        i
                    ));
                }
            }
            if !errors.is_empty() {
                MatchResult {
                    consumed: 0,
                    errors,
                }
            } else {
                MatchResult {
                    consumed: total,
                    errors,
                }
            }
        }
        // build_for strips Versioned wrappers before matching; treat a direct
        // call as transparent.
        Particle::Versioned { inner, .. } => match_once(inner, children, start, target_ns),
    }
}

// ---------------------------------------------------------------------------
// Hand-authored particles for core WordprocessingML types
// ---------------------------------------------------------------------------

pub mod word {
    use super::{Occurs, Particle};

    pub fn document() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("background", Occurs::OPTIONAL),
                Particle::element("body", Occurs::ONE),
            ],
            Occurs::ONE,
        )
    }

    pub fn body() -> Particle {
        let block = Particle::choice(
            vec![
                Particle::element("p", Occurs::ONE),
                Particle::element("tbl", Occurs::ONE),
                Particle::element("sdt", Occurs::ONE),
                Particle::element("customXml", Occurs::ONE),
                Particle::element("altChunk", Occurs::ONE),
                Particle::element("bookmarkStart", Occurs::ONE),
                Particle::element("bookmarkEnd", Occurs::ONE),
                Particle::element("commentRangeStart", Occurs::ONE),
                Particle::element("commentRangeEnd", Occurs::ONE),
            ],
            Occurs::STAR,
        );
        Particle::sequence(
            vec![block, Particle::element("sectPr", Occurs::OPTIONAL)],
            Occurs::ONE,
        )
    }

    pub fn paragraph() -> Particle {
        let content = Particle::choice(
            vec![
                Particle::element("r", Occurs::ONE),
                Particle::element("hyperlink", Occurs::ONE),
                Particle::element("bookmarkStart", Occurs::ONE),
                Particle::element("bookmarkEnd", Occurs::ONE),
                Particle::element("commentRangeStart", Occurs::ONE),
                Particle::element("commentRangeEnd", Occurs::ONE),
                Particle::element("fldSimple", Occurs::ONE),
                Particle::element("sdt", Occurs::ONE),
                Particle::element("customXml", Occurs::ONE),
            ],
            Occurs::STAR,
        );
        Particle::sequence(
            vec![Particle::element("pPr", Occurs::OPTIONAL), content],
            Occurs::ONE,
        )
    }

    pub fn run() -> Particle {
        let content = Particle::choice(
            vec![
                Particle::element("t", Occurs::ONE),
                Particle::element("br", Occurs::ONE),
                Particle::element("tab", Occurs::ONE),
                Particle::element("drawing", Occurs::ONE),
                Particle::element("footnoteReference", Occurs::ONE),
                Particle::element("endnoteReference", Occurs::ONE),
                Particle::element("commentReference", Occurs::ONE),
                Particle::element("lastRenderedPageBreak", Occurs::ONE),
                Particle::element("sym", Occurs::ONE),
                Particle::element("softHyphen", Occurs::ONE),
                Particle::element("noBreakHyphen", Occurs::ONE),
            ],
            Occurs::STAR,
        );
        Particle::sequence(
            vec![Particle::element("rPr", Occurs::OPTIONAL), content],
            Occurs::ONE,
        )
    }

    pub fn table() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("tblPr", Occurs::OPTIONAL),
                Particle::element("tblGrid", Occurs::OPTIONAL),
                Particle::element("tr", Occurs::PLUS),
            ],
            Occurs::ONE,
        )
    }

    pub fn table_row() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("tblPrEx", Occurs::OPTIONAL),
                Particle::element("trPr", Occurs::OPTIONAL),
                Particle::element("tc", Occurs::PLUS),
            ],
            Occurs::ONE,
        )
    }

    pub fn table_cell() -> Particle {
        let block = Particle::choice(
            vec![
                Particle::element("p", Occurs::ONE),
                Particle::element("tbl", Occurs::ONE),
                Particle::element("sdt", Occurs::ONE),
                Particle::element("altChunk", Occurs::ONE),
            ],
            Occurs::PLUS,
        );
        Particle::sequence(
            vec![Particle::element("tcPr", Occurs::OPTIONAL), block],
            Occurs::ONE,
        )
    }

    /// Particle registry lookup (C# `ValidationCache.GetParticleConstraint` shell).
    pub fn particle_for(local_name: &str) -> Option<Particle> {
        Some(match local_name {
            "document" => document(),
            "body" => body(),
            "p" => paragraph(),
            "r" => run(),
            "tbl" => table(),
            "tr" => table_row(),
            "tc" => table_cell(),
            _ => return None,
        })
    }
}

/// Recursively validate a Word document using ordered particles.
pub fn validate_word_particles(root: &OpenXmlElement) -> Vec<ValidationError> {
    validate_word_particles_for_version(root, FileFormatVersions::OFFICE2007)
}

/// Recursively validate Word particles against target-version logical MC children.
pub fn validate_word_particles_for_version(
    root: &OpenXmlElement,
    version: FileFormatVersions,
) -> Vec<ValidationError> {
    let mut context = ValidationContext::new(ValidationSettings::new(version));
    context.set_collect_expected_children(true);
    let root_mc_context = McContext::new();
    let mut errors = Vec::new();
    if root.local_name != "document" {
        return errors;
    }
    errors.extend(validate_particle_with_context(
        root,
        &word::document(),
        "w:document",
        &context,
        &root_mc_context,
    ));
    let root_children = context.validation_children_with_context(root, &root_mc_context);
    if let Some(body) = root_children
        .iter()
        .find(|child| child.element.local_name == "body")
    {
        errors.extend(validate_particle_with_context(
            body.element,
            &word::body(),
            "w:document/w:body",
            &context,
            &body.mc_context,
        ));
        let body_children =
            context.validation_children_with_context(body.element, &body.mc_context);
        for (i, paragraph) in body_children
            .iter()
            .filter(|child| child.element.local_name == "p")
            .enumerate()
        {
            let path = format!("w:document/w:body/w:p[{i}]");
            errors.extend(validate_particle_with_context(
                paragraph.element,
                &word::paragraph(),
                &path,
                &context,
                &paragraph.mc_context,
            ));
            let paragraph_children = context
                .validation_children_with_context(paragraph.element, &paragraph.mc_context);
            for (j, run) in paragraph_children
                .iter()
                .filter(|child| child.element.local_name == "r")
                .enumerate()
            {
                let run_path = format!("{path}/w:r[{j}]");
                errors.extend(validate_particle_with_context(
                    run.element,
                    &word::run(),
                    &run_path,
                    &context,
                    &run.mc_context,
                ));
            }
        }
        for (i, table) in body_children
            .iter()
            .filter(|child| child.element.local_name == "tbl")
            .enumerate()
        {
            let path = format!("w:document/w:body/w:tbl[{i}]");
            errors.extend(validate_particle_with_context(
                table.element,
                &word::table(),
                &path,
                &context,
                &table.mc_context,
            ));
            let table_children =
                context.validation_children_with_context(table.element, &table.mc_context);
            for (row_index, row) in table_children
                .iter()
                .filter(|child| child.element.local_name == "tr")
                .enumerate()
            {
                let row_path = format!("{path}/w:tr[{row_index}]");
                errors.extend(validate_particle_with_context(
                    row.element,
                    &word::table_row(),
                    &row_path,
                    &context,
                    &row.mc_context,
                ));
                let row_children =
                    context.validation_children_with_context(row.element, &row.mc_context);
                for (cell_index, cell) in row_children
                    .iter()
                    .filter(|child| child.element.local_name == "tc")
                    .enumerate()
                {
                    let cell_path = format!("{row_path}/w:tc[{cell_index}]");
                    errors.extend(validate_particle_with_context(
                        cell.element,
                        &word::table_cell(),
                        &cell_path,
                        &context,
                        &cell.mc_context,
                    ));
                }
            }
        }
    }
    errors
}

impl fmt::Display for Occurs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.max {
            Some(max) => write!(f, "{{{},{}}}", self.min, max),
            None => write!(f, "{{{},∞}}", self.min),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wordprocessing::{
        body, document, paragraph, paragraph_properties, paragraph_with_text, run,
        table_from_strings, text,
    };

    #[test]
    fn document_particle_ok() {
        let doc = document(vec![body(vec![paragraph_with_text("hi")])]);
        let errs = validate_word_particles(&doc);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn document_missing_body() {
        let doc = document(vec![]);
        let errs = validate_word_particles(&doc);
        assert!(!errs.is_empty());
        assert!(
            errs.iter().any(|e| {
                e.id() == Some("Sch_IncompleteContentExpectingComplex")
                    && e.description().contains("List of possible elements expected")
            }),
            "{errs:?}"
        );
    }

    #[test]
    fn run_particle() {
        let r = run(vec![text("x")]);
        let errs = validate_particle(&r, &word::run(), "w:r");
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn table_particle() {
        let tbl = table_from_strings(&[vec!["a", "b"], vec!["c", "d"]], None);
        let errs = validate_particle(&tbl, &word::table(), "w:tbl");
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn unexpected_in_document() {
        let mut doc = document(vec![body(vec![])]);
        doc.append_child(crate::element::OpenXmlElement::w("bogus"));
        let errs = validate_word_particles(&doc);
        assert!(errs.iter().any(|e| {
            e.id() == Some("Sch_InvalidElementContentExpectingComplex")
                || e.message.contains("invalid child")
                || e.message.contains("unexpected")
        }));
    }

    #[test]
    fn paragraph_ppr_then_runs() {
        let p = paragraph(vec![
            paragraph_properties(vec![]),
            run(vec![text("a")]),
            run(vec![text("b")]),
        ]);
        let errs = validate_particle(&p, &word::paragraph(), "w:p");
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn alternate_content_supplies_required_body_for_version() {
        use crate::markup_compatibility::alternate_content_with;
        use crate::wordprocessing::body;

        let document = crate::wordprocessing::document(vec![alternate_content_with(
            "w14",
            vec![body(vec![])],
            vec![body(vec![])],
        )]);
        let office_2007 = validate_particle_for_version(
            &document,
            &word::document(),
            "w:document",
            FileFormatVersions::OFFICE2007,
        );
        assert!(office_2007.is_empty(), "{office_2007:?}");
        let office_2010 = validate_particle_for_version(
            &document,
            &word::document(),
            "w:document",
            FileFormatVersions::OFFICE2010,
        );
        assert!(office_2010.is_empty(), "{office_2010:?}");
    }

    #[test]
    fn process_content_supplies_required_body() {
        use crate::namespace::ns;
        use crate::wordprocessing::body;

        let mut document = crate::wordprocessing::document(vec![]);
        document.set_attribute_ns("mc", ns::MARKUP_COMPATIBILITY.uri, "Ignorable", "w14");
        document.set_attribute_ns(
            "mc",
            ns::MARKUP_COMPATIBILITY.uri,
            "ProcessContent",
            "w14:wrapper",
        );
        document.append_child(
            OpenXmlElement::new("w14", "urn:w14", "wrapper")
                .with_children(vec![body(vec![])]),
        );
        let errors = validate_particle_for_version(
            &document,
            &word::document(),
            "w:document",
            FileFormatVersions::OFFICE2007,
        );
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn inherited_process_content_applies_inside_body() {
        use crate::namespace::ns;
        use crate::wordprocessing::{body, paragraph_with_text};

        let wrapper = OpenXmlElement::new("w14", "urn:w14", "wrapper")
            .with_children(vec![paragraph_with_text("promoted")]);
        let mut document = crate::wordprocessing::document(vec![body(vec![wrapper])]);
        document.set_attribute_ns("mc", ns::MARKUP_COMPATIBILITY.uri, "Ignorable", "w14");
        document.set_attribute_ns(
            "mc",
            ns::MARKUP_COMPATIBILITY.uri,
            "ProcessContent",
            "w14:wrapper",
        );

        let errors = validate_word_particles_for_version(
            &document,
            FileFormatVersions::OFFICE2007,
        );
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn from_schema_json_document() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"Kind":"Sequence","Items":[
                {"Kind":"Sequence","Items":[{"Name":"w:CT_Background/w:background","Occurs":[{"Max":1}]}]},
                {"Kind":"Sequence","Items":[{"Name":"w:CT_Body/w:body","Occurs":[{"Max":1}]}]}
            ]}"#,
        )
        .unwrap();
        let p = Particle::from_schema_json(&json).unwrap();
        let doc = document(vec![body(vec![])]);
        let errs = validate_particle(&doc, &p, "w:document");
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn expected_children_message_and_merge() {
        let mut expected = ExpectedChildren::new();
        assert!(expected.is_empty());
        assert_eq!(expected.expected_children_message(), "");

        expected.add_element("w:background");
        expected.add_element("w:body");
        expected.add_any_namespace("urn:custom");
        assert_eq!(expected.count(), 3);
        assert_eq!(
            expected.expected_children_message(),
            " List of possible elements expected: <w:background>,<w:body>,any element in namespace 'urn:custom'."
        );

        let mut merged = ExpectedChildren::new();
        merged.add_all(&expected);
        assert_eq!(merged.count(), 3);
        merged.clear();
        assert!(merged.is_empty());
    }

    #[test]
    fn required_and_expected_elements_from_particles() {
        let particle = word::document();
        let mut required = ExpectedChildren::new();
        assert!(get_required_elements(&particle, &mut required));
        assert_eq!(required.elements(), &[String::from("body")]);

        let mut expected = ExpectedChildren::new();
        assert!(get_expected_elements(&particle, &mut expected));
        assert_eq!(
            expected.elements(),
            &[String::from("background"), String::from("body")]
        );

        // Choice is required only when every alternative is required.
        let optional_choice = Particle::choice(
            vec![
                Particle::element("a", Occurs::ONE),
                Particle::element("b", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        );
        let mut required = ExpectedChildren::new();
        assert!(!get_required_elements(&optional_choice, &mut required));
        assert!(required.is_empty());

        let required_choice = Particle::choice(
            vec![
                Particle::element("a", Occurs::ONE),
                Particle::element("b", Occurs::ONE),
            ],
            Occurs::ONE,
        );
        let mut required = ExpectedChildren::new();
        assert!(get_required_elements(&required_choice, &mut required));
        assert_eq!(required.elements(), &["a", "b"]);
    }

    #[test]
    fn particle_match_info_reset_and_expected_children() {
        let mut info = ParticleMatchInfo::with_start_element("w:p");
        assert_eq!(info.match_result, ParticleMatch::Nomatch);
        info.match_result = ParticleMatch::Partial;
        info.last_matched_element = Some("w:r".into());
        info.error_message = Some("bad".into());

        let mut expected = ExpectedChildren::new();
        expected.add_element("w:t");
        info.set_expected_children(&expected);
        assert_eq!(info.expected_children().count(), 1);

        info.reset(Some("w:tbl"));
        assert_eq!(info.start_element.as_deref(), Some("w:tbl"));
        assert_eq!(info.match_result, ParticleMatch::Nomatch);
        assert!(info.last_matched_element.is_none());
        assert!(info.error_message.is_none());
        assert!(info.expected_children().is_empty());
    }

    #[test]
    fn xsd_any_namespace_modes_gate_matching() {
        use crate::element::OpenXmlElement;

        let w_ns = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
        let parent_with = |child: OpenXmlElement| {
            OpenXmlElement::new("w", w_ns, "container").with_child(child)
        };
        let same_ns_child = OpenXmlElement::new("w", w_ns, "inner");
        let other_ns_child = OpenXmlElement::new("x", "urn:other", "inner");
        let local_child = OpenXmlElement::new("", "", "inner");

        let check = |mode: XsdAnyNamespace, child: &OpenXmlElement| {
            let particle = Particle::any_with_namespace(Occurs::ONE, mode);
            validate_particle(&parent_with(child.clone()), &particle, "container").is_empty()
        };

        assert!(check(XsdAnyNamespace::Any, &same_ns_child));
        assert!(check(XsdAnyNamespace::Any, &other_ns_child));

        assert!(check(XsdAnyNamespace::Local, &local_child));
        assert!(!check(XsdAnyNamespace::Local, &same_ns_child));

        assert!(check(XsdAnyNamespace::Other, &other_ns_child));
        assert!(!check(XsdAnyNamespace::Other, &same_ns_child));
        assert!(!check(XsdAnyNamespace::Other, &local_child));

        assert!(check(XsdAnyNamespace::TargetNamespace, &same_ns_child));
        assert!(!check(XsdAnyNamespace::TargetNamespace, &other_ns_child));

        let mut expected = ExpectedChildren::new();
        get_expected_elements(
            &Particle::any_with_namespace(Occurs::ONE, XsdAnyNamespace::Other),
            &mut expected,
        );
        assert_eq!(expected.any_namespaces(), &[String::from("##other")]);

        // AnyWithUri matches only the specified namespace URI.
        let uri = "urn:custom";
        let particle = Particle::any_with_uri(Occurs::ONE, uri);
        assert_eq!(particle.particle_type(), ParticleType::AnyWithUri);
        let parent = OpenXmlElement::new("w", w_ns, "container")
            .with_child(OpenXmlElement::new("x", uri, "inner"));
        assert!(validate_particle(&parent, &particle, "container").is_empty());
        let parent_wrong = OpenXmlElement::new("w", w_ns, "container")
            .with_child(OpenXmlElement::new("x", "urn:other", "inner"));
        assert!(!validate_particle(&parent_wrong, &particle, "container").is_empty());
        let mut expected = ExpectedChildren::new();
        get_expected_elements(&particle, &mut expected);
        assert_eq!(expected.any_namespaces(), &[String::from("urn:custom")]);
        assert_eq!(XsdAnyNamespace::Other.namespace_string(), "##other");
    }

    #[test]
    fn versioned_particles_prune_by_target_version() {
        use crate::element::OpenXmlElement;

        let particle = Particle::sequence(
            vec![
                Particle::element("body", Occurs::ONE),
                Particle::versioned(
                    FileFormatVersions::OFFICE2010,
                    Particle::element("glow", Occurs::ONE),
                ),
            ],
            Occurs::ONE,
        );

        let built_2007 = particle.build_for(FileFormatVersions::OFFICE2007).unwrap();
        let mut expected = ExpectedChildren::new();
        get_expected_elements(&built_2007, &mut expected);
        assert_eq!(expected.elements(), &[String::from("body")]);

        let built_2010 = particle.build_for(FileFormatVersions::OFFICE2010).unwrap();
        let mut expected = ExpectedChildren::new();
        get_expected_elements(&built_2010, &mut expected);
        assert_eq!(
            expected.elements(),
            &[String::from("body"), String::from("glow")]
        );

        // A 2010-only child is rejected under a 2007 build but accepted for 2010.
        let doc = OpenXmlElement::w("document")
            .with_children(vec![OpenXmlElement::w("body"), OpenXmlElement::w("glow")]);
        let errors_2007 = validate_particle_for_version(
            &doc,
            &particle,
            "w:document",
            FileFormatVersions::OFFICE2007,
        );
        assert!(
            errors_2007.iter().any(|e| {
                e.id() == Some("Sch_InvalidElementContentExpectingComplex")
                    || e.message.contains("invalid child")
                    || e.message.contains("unexpected")
            }),
            "{errors_2007:?}"
        );
        let errors_2010 = validate_particle_for_version(
            &doc,
            &particle,
            "w:document",
            FileFormatVersions::OFFICE2010,
        );
        assert!(errors_2010.is_empty(), "{errors_2010:?}");

        assert!(Particle::versioned(
            FileFormatVersions::OFFICE2013,
            Particle::element("x", Occurs::ONE)
        )
        .build_for(FileFormatVersions::OFFICE2007)
        .is_none());
    }

    #[test]
    fn mismatch_error_appends_expected_children_when_collected() {
        let mut context = ValidationContext::new(ValidationSettings::new(
            FileFormatVersions::OFFICE2007,
        ));
        let doc = document(vec![body(vec![])]);
        let mut bad = doc.clone();
        bad.append_child(crate::element::OpenXmlElement::w("unexpected"));

        let plain = validate_particle_with_context(
            &bad,
            &word::document(),
            "w:document",
            &context,
            &McContext::new(),
        );
        assert_eq!(
            plain[0].id(),
            Some("Sch_InvalidElementContentExpectingComplex"),
            "{plain:?}"
        );

        context.set_collect_expected_children(true);
        let collected = validate_particle_with_context(
            &bad,
            &word::document(),
            "w:document",
            &context,
            &McContext::new(),
        );
        assert!(
            collected[0]
                .message
                .contains(" List of possible elements expected: <background>,<body>."),
            "{collected:?}"
        );
    }
}
