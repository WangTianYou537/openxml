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
        // C# uses ValidationResources Fmt_ElementName / Fmt_ElementNameSeparator /
        // Fmt_AnyElementInNamespace / Fmt_ListOfPossibleElements.
        let elem_fmt = crate::validation::validation_resource_message("Fmt_ElementName")
            .unwrap_or("<{0}>");
        let sep = crate::validation::validation_resource_message("Fmt_ElementNameSeparator")
            .unwrap_or(",");
        let any_fmt =
            crate::validation::validation_resource_message("Fmt_AnyElementInNamespace")
                .unwrap_or("any element in namespace '{0}'");
        let list_fmt =
            crate::validation::validation_resource_message("Fmt_ListOfPossibleElements")
                .unwrap_or(" List of possible elements expected: {0}.");

        let mut names: Vec<String> = self
            .elements
            .iter()
            .map(|name| elem_fmt.replace("{0}", name))
            .collect();
        names.extend(
            self.xsd_any_namespaces
                .iter()
                .map(|ns| any_fmt.replace("{0}", ns)),
        );
        list_fmt.replace("{0}", &names.join(sep))
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

        // C# AllParticleValidator: duplicate allowed child under xs:all → Sch_AllElement.
        if matches!(particle.particle_type(), ParticleType::All)
            && is_all_duplicate_child(particle, &children[..result.consumed], extra)
        {
            errors.push(
                ValidationError::with_id(
                    format!("{path}/{}", extra.local_name),
                    "Sch_AllElement",
                    format!(
                        "Element '{child_name}' cannot appear more than once if content model type is \"all\"."
                    ),
                )
                .with_error_type(crate::validation::ValidationErrorType::Schema),
            );
        } else {
            // C# CompositeParticleValidator: CanContainChild → Unexpected; else
            // TryCreateValidChild → WrongType or InvalidElementContent.
            let id = if element.can_contain_child(extra) {
                "Sch_UnexpectedElementContentExpectingComplex"
            } else if element
                .try_create_valid_child(
                    context.file_format(),
                    &extra.prefix,
                    &extra.local_name,
                )
                .is_some()
            {
                // Parent allows a different type with the same local name.
                "Sch_InvalidElementContentWrongType"
            } else {
                "Sch_InvalidElementContentExpectingComplex"
            };
            let description = match id {
                "Sch_UnexpectedElementContentExpectingComplex" => format!(
                    "The element has unexpected child element '{child_name}'.{expected_suffix}"
                ),
                "Sch_InvalidElementContentWrongType" => format!(
                    "The element has child element '{child_name}' of invalid type '{}'.",
                    extra.local_name
                ),
                _ => format!(
                    "The element has invalid child element '{child_name}'.{expected_suffix}"
                ),
            };
            errors.push(
                ValidationError::with_id(
                    format!("{path}/{}", extra.local_name),
                    id,
                    description,
                )
                .with_error_type(crate::validation::ValidationErrorType::Schema),
            );
        }
    }
    for e in result.errors {
        // xs:all duplicate reported from match_particle as structured message.
        if e.starts_with("Sch_AllElement:") {
            let child_name = e.trim_start_matches("Sch_AllElement:").trim();
            errors.push(
                ValidationError::with_id(
                    path,
                    "Sch_AllElement",
                    format!(
                        "Element '{child_name}' cannot appear more than once if content model type is \"all\"."
                    ),
                )
                .with_error_type(crate::validation::ValidationErrorType::Schema),
            );
            continue;
        }
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

/// Whether `extra` is a second occurrence of an already-matched xs:all child.
fn is_all_duplicate_child(
    particle: &Particle,
    matched: &[&OpenXmlElement],
    extra: &OpenXmlElement,
) -> bool {
    let Particle::All { items, .. } = particle else {
        return false;
    };
    // Extra matches some item that was already consumed among `matched`.
    let matches_item = |item: &Particle, child: &OpenXmlElement| -> bool {
        match item {
            Particle::Element { local_name, .. } => child.local_name == *local_name,
            _ => false,
        }
    };
    let mut used = vec![false; items.len()];
    for child in matched {
        for (i, item) in items.iter().enumerate() {
            if !used[i] && matches_item(item, child) {
                used[i] = true;
                break;
            }
        }
    }
    for (i, item) in items.iter().enumerate() {
        if used[i] && matches_item(item, extra) {
            return true;
        }
    }
    false
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

    /// `<w:sectPr>` — common section properties (simplified ordered optionals).
    pub fn section_properties() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("footnotePr", Occurs::OPTIONAL),
                Particle::element("endnotePr", Occurs::OPTIONAL),
                Particle::element("type", Occurs::OPTIONAL),
                Particle::element("pgSz", Occurs::OPTIONAL),
                Particle::element("pgMar", Occurs::OPTIONAL),
                Particle::element("pgBorders", Occurs::OPTIONAL),
                Particle::element("lnNumType", Occurs::OPTIONAL),
                Particle::element("pgNumType", Occurs::OPTIONAL),
                Particle::element("cols", Occurs::OPTIONAL),
                Particle::element("formProt", Occurs::OPTIONAL),
                Particle::element("vAlign", Occurs::OPTIONAL),
                Particle::element("titlePg", Occurs::OPTIONAL),
                Particle::element("textDirection", Occurs::OPTIONAL),
                Particle::element("bidi", Occurs::OPTIONAL),
                Particle::element("docGrid", Occurs::OPTIONAL),
                Particle::element("headerReference", Occurs::STAR),
                Particle::element("footerReference", Occurs::STAR),
            ],
            Occurs::ONE,
        )
    }

    /// `<w:sdt>` structured document tag (block/run-level simplified).
    pub fn sdt() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("sdtPr", Occurs::OPTIONAL),
                Particle::element("sdtEndPr", Occurs::OPTIONAL),
                Particle::element("sdtContent", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    /// `<w:sdtContent>` — block-level content choice.
    pub fn sdt_content() -> Particle {
        Particle::choice(
            vec![
                Particle::element("p", Occurs::ONE),
                Particle::element("tbl", Occurs::ONE),
                Particle::element("sdt", Occurs::ONE),
                Particle::element("customXml", Occurs::ONE),
                Particle::element("r", Occurs::ONE),
                Particle::element("hyperlink", Occurs::ONE),
                Particle::element("bookmarkStart", Occurs::ONE),
                Particle::element("bookmarkEnd", Occurs::ONE),
            ],
            Occurs::STAR,
        )
    }

    /// `<w:hyperlink>` — run-level content.
    pub fn hyperlink() -> Particle {
        Particle::choice(
            vec![
                Particle::element("r", Occurs::ONE),
                Particle::element("sdt", Occurs::ONE),
                Particle::element("hyperlink", Occurs::ONE),
                Particle::element("fldSimple", Occurs::ONE),
                Particle::element("bookmarkStart", Occurs::ONE),
                Particle::element("bookmarkEnd", Occurs::ONE),
            ],
            Occurs::STAR,
        )
    }

    /// `<w:drawing>` — DrawingML anchor/inline.
    pub fn drawing() -> Particle {
        Particle::choice(
            vec![
                Particle::element("anchor", Occurs::ONE),
                Particle::element("inline", Occurs::ONE),
            ],
            Occurs::PLUS,
        )
    }

    /// `<w:styles>` part root.
    pub fn styles() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("docDefaults", Occurs::OPTIONAL),
                Particle::element("latentStyles", Occurs::OPTIONAL),
                Particle::element("style", Occurs::STAR),
            ],
            Occurs::ONE,
        )
    }

    /// `<w:numbering>` part root.
    pub fn numbering() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("numPicBullet", Occurs::STAR),
                Particle::element("abstractNum", Occurs::STAR),
                Particle::element("num", Occurs::STAR),
                Particle::element("numIdMacAtCleanup", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    /// `<w:fonts>` part root.
    pub fn fonts_table() -> Particle {
        Particle::sequence(vec![Particle::element("font", Occurs::STAR)], Occurs::ONE)
    }

    /// `<w:comments>` part root.
    pub fn comments() -> Particle {
        Particle::sequence(vec![Particle::element("comment", Occurs::STAR)], Occurs::ONE)
    }

    /// `<w:footnotes>` / `<w:endnotes>` part roots.
    pub fn footnotes() -> Particle {
        Particle::sequence(
            vec![Particle::element("footnote", Occurs::STAR)],
            Occurs::ONE,
        )
    }

    pub fn endnotes() -> Particle {
        Particle::sequence(
            vec![Particle::element("endnote", Occurs::STAR)],
            Occurs::ONE,
        )
    }

    /// `<w:hdr>` / `<w:ftr>` — block-level content like body (no sectPr).
    pub fn header_footer() -> Particle {
        Particle::choice(
            vec![
                Particle::element("p", Occurs::ONE),
                Particle::element("tbl", Occurs::ONE),
                Particle::element("sdt", Occurs::ONE),
                Particle::element("customXml", Occurs::ONE),
                Particle::element("altChunk", Occurs::ONE),
                Particle::element("bookmarkStart", Occurs::ONE),
                Particle::element("bookmarkEnd", Occurs::ONE),
            ],
            Occurs::STAR,
        )
    }

    /// `<w:abstractNum>` numbering definition.
    pub fn abstract_num() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("nsid", Occurs::OPTIONAL),
                Particle::element("multiLevelType", Occurs::OPTIONAL),
                Particle::element("tmpl", Occurs::OPTIONAL),
                Particle::element("name", Occurs::OPTIONAL),
                Particle::element("styleLink", Occurs::OPTIONAL),
                Particle::element("numStyleLink", Occurs::OPTIONAL),
                Particle::element("lvl", Occurs::STAR),
            ],
            Occurs::ONE,
        )
    }

    /// `<w:settings>` document settings part root (generated content model).
    pub fn settings() -> Particle {
        crate::generated::wordprocessingml_2006_main::particle_settings()
    }

    /// `<w:webSettings>` web settings part root (generated content model).
    pub fn web_settings() -> Particle {
        crate::generated::wordprocessingml_2006_main::particle_web_settings()
    }

    /// `<w:glossaryDocument>` glossary part root (generated content model).
    pub fn glossary_document() -> Particle {
        crate::generated::wordprocessingml_2006_main::particle_glossary_document()
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
            "sectPr" => section_properties(),
            "sdt" => sdt(),
            "sdtContent" => sdt_content(),
            "hyperlink" => hyperlink(),
            "drawing" => drawing(),
            "styles" => styles(),
            "numbering" => numbering(),
            "fonts" => fonts_table(),
            "comments" => comments(),
            "footnotes" => footnotes(),
            "endnotes" => endnotes(),
            "hdr" | "ftr" => header_footer(),
            "abstractNum" => abstract_num(),
            "settings" => settings(),
            "webSettings" => web_settings(),
            "glossaryDocument" => glossary_document(),
            _ => return None,
        })
    }
}

/// Hand-authored particles for core SpreadsheetML types.
pub mod spreadsheet {
    use super::{Occurs, Particle};

    /// `<x:workbook>` — simplified: optional props then required `sheets`.
    pub fn workbook() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("fileVersion", Occurs::OPTIONAL),
                Particle::element("fileSharing", Occurs::OPTIONAL),
                Particle::element("workbookPr", Occurs::OPTIONAL),
                Particle::element("workbookProtection", Occurs::OPTIONAL),
                Particle::element("bookViews", Occurs::OPTIONAL),
                Particle::element("sheets", Occurs::ONE),
                Particle::element("functionGroups", Occurs::OPTIONAL),
                Particle::element("externalReferences", Occurs::OPTIONAL),
                Particle::element("definedNames", Occurs::OPTIONAL),
                Particle::element("calcPr", Occurs::OPTIONAL),
                Particle::element("oleSize", Occurs::OPTIONAL),
                Particle::element("customWorkbookViews", Occurs::OPTIONAL),
                Particle::element("pivotCaches", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn sheets() -> Particle {
        Particle::sequence(
            vec![Particle::element("sheet", Occurs::PLUS)],
            Occurs::ONE,
        )
    }

    /// `<x:worksheet>` — ordered optional children with required `sheetData`.
    pub fn worksheet() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("sheetPr", Occurs::OPTIONAL),
                Particle::element("dimension", Occurs::OPTIONAL),
                Particle::element("sheetViews", Occurs::OPTIONAL),
                Particle::element("sheetFormatPr", Occurs::OPTIONAL),
                Particle::element("cols", Occurs::STAR),
                Particle::element("sheetData", Occurs::ONE),
                Particle::element("sheetCalcPr", Occurs::OPTIONAL),
                Particle::element("sheetProtection", Occurs::OPTIONAL),
                Particle::element("protectedRanges", Occurs::OPTIONAL),
                Particle::element("autoFilter", Occurs::OPTIONAL),
                Particle::element("sortState", Occurs::OPTIONAL),
                Particle::element("dataConsolidate", Occurs::OPTIONAL),
                Particle::element("mergeCells", Occurs::OPTIONAL),
                Particle::element("conditionalFormatting", Occurs::STAR),
                Particle::element("dataValidations", Occurs::OPTIONAL),
                Particle::element("hyperlinks", Occurs::OPTIONAL),
                Particle::element("printOptions", Occurs::OPTIONAL),
                Particle::element("pageMargins", Occurs::OPTIONAL),
                Particle::element("pageSetup", Occurs::OPTIONAL),
                Particle::element("headerFooter", Occurs::OPTIONAL),
                Particle::element("rowBreaks", Occurs::OPTIONAL),
                Particle::element("colBreaks", Occurs::OPTIONAL),
                Particle::element("drawing", Occurs::OPTIONAL),
                Particle::element("legacyDrawing", Occurs::OPTIONAL),
                Particle::element("picture", Occurs::OPTIONAL),
                Particle::element("oleObjects", Occurs::OPTIONAL),
                Particle::element("tableParts", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn sheet_data() -> Particle {
        Particle::sequence(
            vec![Particle::element("row", Occurs::STAR)],
            Occurs::ONE,
        )
    }

    pub fn row() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("c", Occurs::STAR),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn cell() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("f", Occurs::OPTIONAL),
                Particle::choice(
                    vec![
                        Particle::element("v", Occurs::ONE),
                        Particle::element("is", Occurs::ONE),
                    ],
                    Occurs::OPTIONAL,
                ),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    /// `<x:sst>` shared string table.
    pub fn shared_string_table() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("si", Occurs::STAR),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    /// `<x:styleSheet>` — ordered optional collections.
    pub fn stylesheet() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("numFmts", Occurs::OPTIONAL),
                Particle::element("fonts", Occurs::OPTIONAL),
                Particle::element("fills", Occurs::OPTIONAL),
                Particle::element("borders", Occurs::OPTIONAL),
                Particle::element("cellStyleXfs", Occurs::OPTIONAL),
                Particle::element("cellXfs", Occurs::OPTIONAL),
                Particle::element("cellStyles", Occurs::OPTIONAL),
                Particle::element("dxfs", Occurs::OPTIONAL),
                Particle::element("tableStyles", Occurs::OPTIONAL),
                Particle::element("colors", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn fonts() -> Particle {
        Particle::sequence(vec![Particle::element("font", Occurs::STAR)], Occurs::ONE)
    }

    pub fn fills() -> Particle {
        Particle::sequence(vec![Particle::element("fill", Occurs::STAR)], Occurs::ONE)
    }

    pub fn borders() -> Particle {
        Particle::sequence(vec![Particle::element("border", Occurs::STAR)], Occurs::ONE)
    }

    pub fn cell_xfs() -> Particle {
        Particle::sequence(vec![Particle::element("xf", Occurs::STAR)], Occurs::ONE)
    }

    pub fn particle_for(local_name: &str) -> Option<Particle> {
        Some(match local_name {
            "workbook" => workbook(),
            "sheets" => sheets(),
            "worksheet" => worksheet(),
            "sheetData" => sheet_data(),
            "row" => row(),
            "c" => cell(),
            "sst" => shared_string_table(),
            "styleSheet" => stylesheet(),
            "fonts" => fonts(),
            "fills" => fills(),
            "borders" => borders(),
            "cellXfs" => cell_xfs(),
            _ => return None,
        })
    }
}

/// Hand-authored particles for core PresentationML types.
pub mod presentation {
    use super::{Occurs, Particle};

    /// `<p:presentation>` — simplified ordered optional children.
    pub fn presentation() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("sldMasterIdLst", Occurs::OPTIONAL),
                Particle::element("notesMasterIdLst", Occurs::OPTIONAL),
                Particle::element("handoutMasterIdLst", Occurs::OPTIONAL),
                Particle::element("sldIdLst", Occurs::OPTIONAL),
                Particle::element("sldSz", Occurs::OPTIONAL),
                Particle::element("notesSz", Occurs::OPTIONAL),
                Particle::element("embeddedFontLst", Occurs::OPTIONAL),
                Particle::element("custShowLst", Occurs::OPTIONAL),
                Particle::element("defaultTextStyle", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn slide() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("cSld", Occurs::ONE),
                Particle::element("clrMapOvr", Occurs::OPTIONAL),
                Particle::element("transition", Occurs::OPTIONAL),
                Particle::element("timing", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn slide_layout() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("cSld", Occurs::ONE),
                Particle::element("clrMapOvr", Occurs::OPTIONAL),
                Particle::element("transition", Occurs::OPTIONAL),
                Particle::element("timing", Occurs::OPTIONAL),
                Particle::element("hf", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn slide_master() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("cSld", Occurs::ONE),
                Particle::element("clrMap", Occurs::ONE),
                Particle::element("sldLayoutIdLst", Occurs::OPTIONAL),
                Particle::element("transition", Occurs::OPTIONAL),
                Particle::element("timing", Occurs::OPTIONAL),
                Particle::element("hf", Occurs::OPTIONAL),
                Particle::element("txStyles", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn common_slide_data() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("bg", Occurs::OPTIONAL),
                Particle::element("spTree", Occurs::ONE),
                Particle::element("custDataLst", Occurs::OPTIONAL),
                Particle::element("controls", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn shape_tree() -> Particle {
        let shapes = Particle::choice(
            vec![
                Particle::element("sp", Occurs::ONE),
                Particle::element("grpSp", Occurs::ONE),
                Particle::element("graphicFrame", Occurs::ONE),
                Particle::element("cxnSp", Occurs::ONE),
                Particle::element("pic", Occurs::ONE),
                Particle::element("contentPart", Occurs::ONE),
            ],
            Occurs::STAR,
        );
        Particle::sequence(
            vec![
                Particle::element("nvGrpSpPr", Occurs::ONE),
                Particle::element("grpSpPr", Occurs::ONE),
                shapes,
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn shape() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("nvSpPr", Occurs::ONE),
                Particle::element("spPr", Occurs::ONE),
                Particle::element("style", Occurs::OPTIONAL),
                Particle::element("txBody", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn picture() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("nvPicPr", Occurs::ONE),
                Particle::element("blipFill", Occurs::ONE),
                Particle::element("spPr", Occurs::ONE),
                Particle::element("style", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn particle_for(local_name: &str) -> Option<Particle> {
        Some(match local_name {
            "presentation" => presentation(),
            "sld" => slide(),
            "sldLayout" => slide_layout(),
            "sldMaster" => slide_master(),
            "notes" => notes_slide(),
            "notesMaster" => notes_master(),
            "cSld" => common_slide_data(),
            "spTree" => shape_tree(),
            "sp" => shape(),
            "pic" => picture(),
            _ => return None,
        })
    }

    pub fn notes_slide() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("cSld", Occurs::ONE),
                Particle::element("clrMapOvr", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }

    pub fn notes_master() -> Particle {
        Particle::sequence(
            vec![
                Particle::element("cSld", Occurs::ONE),
                Particle::element("clrMap", Occurs::ONE),
                Particle::element("hf", Occurs::OPTIONAL),
                Particle::element("notesStyle", Occurs::OPTIONAL),
                Particle::element("extLst", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        )
    }
}

/// Combined particle registry (Word + Spreadsheet + Presentation) for
/// [`ValidationCache::get_constraint`] / SchemaTypeValidator.
pub fn particle_for(local_name: &str) -> Option<Particle> {
    word::particle_for(local_name)
        .or_else(|| spreadsheet::particle_for(local_name))
        .or_else(|| presentation::particle_for(local_name))
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

/// Recursively validate a SpreadsheetML worksheet root with ordered particles.
pub fn validate_spreadsheet_particles(root: &OpenXmlElement) -> Vec<ValidationError> {
    validate_spreadsheet_particles_for_version(root, FileFormatVersions::OFFICE2007)
}

/// Version-aware SpreadsheetML particle walk for `worksheet` / `workbook` roots.
pub fn validate_spreadsheet_particles_for_version(
    root: &OpenXmlElement,
    version: FileFormatVersions,
) -> Vec<ValidationError> {
    let mut context = ValidationContext::new(ValidationSettings::new(version));
    context.set_collect_expected_children(true);
    let root_mc = McContext::new();
    let mut errors = Vec::new();

    match root.local_name.as_str() {
        "workbook" => {
            errors.extend(validate_particle_with_context(
                root,
                &spreadsheet::workbook(),
                "x:workbook",
                &context,
                &root_mc,
            ));
            let children = context.validation_children_with_context(root, &root_mc);
            if let Some(sheets) = children
                .iter()
                .find(|c| c.element.local_name == "sheets")
            {
                errors.extend(validate_particle_with_context(
                    sheets.element,
                    &spreadsheet::sheets(),
                    "x:workbook/x:sheets",
                    &context,
                    &sheets.mc_context,
                ));
            }
        }
        "worksheet" => {
            errors.extend(validate_particle_with_context(
                root,
                &spreadsheet::worksheet(),
                "x:worksheet",
                &context,
                &root_mc,
            ));
            let children = context.validation_children_with_context(root, &root_mc);
            if let Some(sd) = children
                .iter()
                .find(|c| c.element.local_name == "sheetData")
            {
                errors.extend(validate_particle_with_context(
                    sd.element,
                    &spreadsheet::sheet_data(),
                    "x:worksheet/x:sheetData",
                    &context,
                    &sd.mc_context,
                ));
                let rows = context.validation_children_with_context(sd.element, &sd.mc_context);
                for (i, row) in rows
                    .iter()
                    .filter(|c| c.element.local_name == "row")
                    .enumerate()
                {
                    let path = format!("x:worksheet/x:sheetData/x:row[{i}]");
                    errors.extend(validate_particle_with_context(
                        row.element,
                        &spreadsheet::row(),
                        &path,
                        &context,
                        &row.mc_context,
                    ));
                    let cells =
                        context.validation_children_with_context(row.element, &row.mc_context);
                    for (j, cell) in cells
                        .iter()
                        .filter(|c| c.element.local_name == "c")
                        .enumerate()
                    {
                        let cell_path = format!("{path}/x:c[{j}]");
                        errors.extend(validate_particle_with_context(
                            cell.element,
                            &spreadsheet::cell(),
                            &cell_path,
                            &context,
                            &cell.mc_context,
                        ));
                    }
                }
            }
        }
        "sst" => {
            errors.extend(validate_particle_with_context(
                root,
                &spreadsheet::shared_string_table(),
                "x:sst",
                &context,
                &root_mc,
            ));
        }
        "styleSheet" => {
            errors.extend(validate_particle_with_context(
                root,
                &spreadsheet::stylesheet(),
                "x:styleSheet",
                &context,
                &root_mc,
            ));
        }
        _ => {}
    }
    errors
}

/// Recursively validate a PresentationML slide root with ordered particles.
pub fn validate_presentation_particles(root: &OpenXmlElement) -> Vec<ValidationError> {
    validate_presentation_particles_for_version(root, FileFormatVersions::OFFICE2007)
}

/// Version-aware PresentationML particle walk for `sld` / `presentation` roots.
pub fn validate_presentation_particles_for_version(
    root: &OpenXmlElement,
    version: FileFormatVersions,
) -> Vec<ValidationError> {
    let mut context = ValidationContext::new(ValidationSettings::new(version));
    context.set_collect_expected_children(true);
    let root_mc = McContext::new();
    let mut errors = Vec::new();

    match root.local_name.as_str() {
        "presentation" => {
            errors.extend(validate_particle_with_context(
                root,
                &presentation::presentation(),
                "p:presentation",
                &context,
                &root_mc,
            ));
        }
        "sld" | "sldLayout" | "sldMaster" | "notes" | "notesMaster" => {
            let particle = match root.local_name.as_str() {
                "sld" => presentation::slide(),
                "sldLayout" => presentation::slide_layout(),
                "notes" => presentation::notes_slide(),
                "notesMaster" => presentation::notes_master(),
                _ => presentation::slide_master(),
            };
            let path = format!("p:{}", root.local_name);
            errors.extend(validate_particle_with_context(
                root, &particle, &path, &context, &root_mc,
            ));
            let children = context.validation_children_with_context(root, &root_mc);
            if let Some(csld) = children.iter().find(|c| c.element.local_name == "cSld") {
                let csld_path = format!("{path}/p:cSld");
                errors.extend(validate_particle_with_context(
                    csld.element,
                    &presentation::common_slide_data(),
                    &csld_path,
                    &context,
                    &csld.mc_context,
                ));
                let csld_children =
                    context.validation_children_with_context(csld.element, &csld.mc_context);
                if let Some(tree) = csld_children
                    .iter()
                    .find(|c| c.element.local_name == "spTree")
                {
                    let tree_path = format!("{csld_path}/p:spTree");
                    errors.extend(validate_particle_with_context(
                        tree.element,
                        &presentation::shape_tree(),
                        &tree_path,
                        &context,
                        &tree.mc_context,
                    ));
                    let shapes =
                        context.validation_children_with_context(tree.element, &tree.mc_context);
                    for (i, sp) in shapes
                        .iter()
                        .filter(|c| c.element.local_name == "sp")
                        .enumerate()
                    {
                        let sp_path = format!("{tree_path}/p:sp[{i}]");
                        errors.extend(validate_particle_with_context(
                            sp.element,
                            &presentation::shape(),
                            &sp_path,
                            &context,
                            &sp.mc_context,
                        ));
                    }
                    for (i, pic) in shapes
                        .iter()
                        .filter(|c| c.element.local_name == "pic")
                        .enumerate()
                    {
                        let pic_path = format!("{tree_path}/p:pic[{i}]");
                        errors.extend(validate_particle_with_context(
                            pic.element,
                            &presentation::picture(),
                            &pic_path,
                            &context,
                            &pic.mc_context,
                        ));
                    }
                }
            }
        }
        _ => {}
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
    fn word_particle_for_resolves_extended_roots() {
        assert!(word::particle_for("sectPr").is_some());
        assert!(word::particle_for("sdt").is_some());
        assert!(word::particle_for("sdtContent").is_some());
        assert!(word::particle_for("hyperlink").is_some());
        assert!(word::particle_for("drawing").is_some());
        assert!(word::particle_for("styles").is_some());
        assert!(word::particle_for("numbering").is_some());
        assert!(word::particle_for("fonts").is_some());
        assert!(word::particle_for("comments").is_some());
        assert!(word::particle_for("footnotes").is_some());
        assert!(word::particle_for("hdr").is_some());
        assert!(word::particle_for("ftr").is_some());
        assert!(word::particle_for("abstractNum").is_some());
        assert!(word::particle_for("settings").is_some());
        assert!(word::particle_for("webSettings").is_some());
        assert!(word::particle_for("glossaryDocument").is_some());
        assert!(crate::validation::particle::particle_for("sectPr").is_some());
        assert!(crate::validation::particle::particle_for("settings").is_some());
    }

    #[test]
    fn header_particle_accepts_paragraphs() {
        let mut hdr = crate::element::OpenXmlElement::w("hdr");
        hdr.append_child(crate::element::OpenXmlElement::w("p"));
        let errs = validate_particle_for_version(
            &hdr,
            &word::header_footer(),
            "w:hdr",
            FileFormatVersions::OFFICE2007,
        );
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn settings_particle_accepts_common_children() {
        let mut settings = crate::element::OpenXmlElement::w("settings");
        settings.append_child(crate::element::OpenXmlElement::w("defaultTabStop"));
        settings.append_child(crate::element::OpenXmlElement::w("documentProtection"));
        settings.append_child(crate::element::OpenXmlElement::w("compat"));
        let errs = validate_particle_for_version(
            &settings,
            &word::settings(),
            "w:settings",
            FileFormatVersions::OFFICE2007,
        );
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn web_settings_particle_accepts_optimize_and_png() {
        let mut web = crate::element::OpenXmlElement::w("webSettings");
        web.append_child(crate::element::OpenXmlElement::w("optimizeForBrowser"));
        web.append_child(crate::element::OpenXmlElement::w("allowPNG"));
        let errs = validate_particle_for_version(
            &web,
            &word::web_settings(),
            "w:webSettings",
            FileFormatVersions::OFFICE2007,
        );
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn glossary_document_particle_accepts_doc_parts() {
        let mut glossary = crate::element::OpenXmlElement::w("glossaryDocument");
        glossary.append_child(crate::element::OpenXmlElement::w("docParts"));
        let errs = validate_particle_for_version(
            &glossary,
            &word::glossary_document(),
            "w:glossaryDocument",
            FileFormatVersions::OFFICE2007,
        );
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn styles_particle_accepts_doc_defaults_and_styles() {
        let mut styles = crate::element::OpenXmlElement::w("styles");
        styles.append_child(crate::element::OpenXmlElement::w("docDefaults"));
        styles.append_child(crate::element::OpenXmlElement::w("style"));
        let errs = validate_particle_for_version(
            &styles,
            &word::styles(),
            "w:styles",
            FileFormatVersions::OFFICE2007,
        );
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn sdt_particle_accepts_pr_then_content() {
        let mut sdt = crate::element::OpenXmlElement::w("sdt");
        sdt.append_child(crate::element::OpenXmlElement::w("sdtPr"));
        sdt.append_child(crate::element::OpenXmlElement::w("sdtContent"));
        let errs = validate_particle_for_version(
            &sdt,
            &word::sdt(),
            "w:sdt",
            FileFormatVersions::OFFICE2007,
        );
        assert!(errs.is_empty(), "{errs:?}");
    }

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
                || e.id() == Some("Sch_UnexpectedElementContentExpectingComplex")
                || e.message.contains("invalid child")
                || e.message.contains("unexpected")
        }));
    }

    #[test]
    fn dual_body_is_unexpected_not_invalid() {
        // document can contain body, but sequence maxOccurs=1 → Unexpected.
        let doc = document(vec![body(vec![]), body(vec![])]);
        let errs = validate_particle_for_version(
            &doc,
            &word::document(),
            "w:document",
            FileFormatVersions::OFFICE2007,
        );
        assert!(
            errs.iter()
                .any(|e| e.id() == Some("Sch_UnexpectedElementContentExpectingComplex")),
            "{errs:?}"
        );
    }

    #[test]
    fn all_particle_rejects_duplicate_with_sch_all_element() {
        let particle = Particle::all(
            vec![
                Particle::element("a", Occurs::OPTIONAL),
                Particle::element("b", Occurs::OPTIONAL),
            ],
            Occurs::ONE,
        );
        let mut el = crate::element::OpenXmlElement::w("host");
        el.append_child(crate::element::OpenXmlElement::w("a"));
        el.append_child(crate::element::OpenXmlElement::w("a"));
        let errs = validate_particle_for_version(
            &el,
            &particle,
            "w:host",
            FileFormatVersions::OFFICE2007,
        );
        assert!(
            errs.iter().any(|e| e.id() == Some("Sch_AllElement")),
            "{errs:?}"
        );
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

    #[test]
    fn spreadsheet_worksheet_requires_sheet_data() {
        let mut ws = crate::element::OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "worksheet",
        );
        // Missing sheetData.
        let errs = validate_particle_for_version(
            &ws,
            &spreadsheet::worksheet(),
            "x:worksheet",
            FileFormatVersions::OFFICE2007,
        );
        assert!(
            errs.iter().any(|e| {
                e.id() == Some("Sch_IncompleteContentExpectingComplex")
                    || e.message.contains("sheetData")
                    || e.message.contains("incomplete")
            }),
            "{errs:?}"
        );

        ws.append_child(crate::element::OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "sheetData",
        ));
        let ok = validate_particle_for_version(
            &ws,
            &spreadsheet::worksheet(),
            "x:worksheet",
            FileFormatVersions::OFFICE2007,
        );
        assert!(ok.is_empty(), "{ok:?}");
    }

    #[test]
    fn spreadsheet_particle_for_resolves_core_roots() {
        assert!(spreadsheet::particle_for("workbook").is_some());
        assert!(spreadsheet::particle_for("worksheet").is_some());
        assert!(spreadsheet::particle_for("sheetData").is_some());
        assert!(spreadsheet::particle_for("row").is_some());
        assert!(spreadsheet::particle_for("c").is_some());
        assert!(spreadsheet::particle_for("sst").is_some());
        assert!(spreadsheet::particle_for("styleSheet").is_some());
        assert!(spreadsheet::particle_for("fonts").is_some());
        assert!(crate::validation::particle::particle_for("worksheet").is_some());
        assert!(crate::validation::particle::particle_for("document").is_some());
    }

    #[test]
    fn stylesheet_particle_accepts_fonts_then_fills() {
        let mut ss = crate::element::OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "styleSheet",
        );
        ss.append_child(crate::element::OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "fonts",
        ));
        ss.append_child(crate::element::OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "fills",
        ));
        let errs = validate_spreadsheet_particles(&ss);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn validation_cache_resolves_spreadsheet_particles() {
        let mut cache = crate::validation::ValidationCache::new(FileFormatVersions::OFFICE2007);
        assert!(cache.get_constraint("worksheet").is_some());
        assert!(cache.get_constraint("workbook").is_some());
        assert!(cache.get_constraint("document").is_some());
    }

    #[test]
    fn presentation_slide_requires_csld() {
        let sld = crate::element::OpenXmlElement::new(
            "p",
            "http://schemas.openxmlformats.org/presentationml/2006/main",
            "sld",
        );
        let errs = validate_particle_for_version(
            &sld,
            &presentation::slide(),
            "p:sld",
            FileFormatVersions::OFFICE2007,
        );
        assert!(
            errs.iter().any(|e| {
                e.id() == Some("Sch_IncompleteContentExpectingComplex")
                    || e.message.contains("cSld")
                    || e.message.contains("incomplete")
            }),
            "{errs:?}"
        );
    }

    #[test]
    fn presentation_particle_for_resolves_core_roots() {
        assert!(presentation::particle_for("presentation").is_some());
        assert!(presentation::particle_for("sld").is_some());
        assert!(presentation::particle_for("sldMaster").is_some());
        assert!(presentation::particle_for("notes").is_some());
        assert!(presentation::particle_for("notesMaster").is_some());
        assert!(presentation::particle_for("cSld").is_some());
        assert!(presentation::particle_for("spTree").is_some());
        assert!(presentation::particle_for("sp").is_some());
        assert!(crate::validation::particle::particle_for("sld").is_some());
        assert!(crate::validation::particle::particle_for("worksheet").is_some());
    }

    #[test]
    fn validate_spreadsheet_particles_reports_missing_sheet_data() {
        let ws = crate::element::OpenXmlElement::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "worksheet",
        );
        let errs = validate_spreadsheet_particles(&ws);
        assert!(
            errs.iter().any(|e| e.message.contains("sheetData")
                || e.id() == Some("Sch_IncompleteContentExpectingComplex")),
            "{errs:?}"
        );
    }

    #[test]
    fn validate_presentation_particles_reports_missing_csld() {
        let sld = crate::element::OpenXmlElement::new(
            "p",
            "http://schemas.openxmlformats.org/presentationml/2006/main",
            "sld",
        );
        let errs = validate_presentation_particles(&sld);
        assert!(
            errs.iter().any(|e| e.message.contains("cSld")
                || e.id() == Some("Sch_IncompleteContentExpectingComplex")),
            "{errs:?}"
        );
    }
}
