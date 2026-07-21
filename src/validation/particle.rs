//! Ordered particle matching (sequence / choice / group / all / element).
//!
//! Particles follow the XSD-inspired content models used in the C# Open XML SDK.
//! Both hand-authored core models and schema-generated particles use the same
//! owned [`Particle`] tree so codegen can emit them freely.

use crate::element::OpenXmlElement;
use crate::validation::ValidationError;
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
    /// Any element (wildcard).
    Any { occurs: Occurs },
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
        Particle::Any { occurs }
    }

    fn occurs(&self) -> Occurs {
        match self {
            Particle::Element { occurs, .. }
            | Particle::Sequence { occurs, .. }
            | Particle::Choice { occurs, .. }
            | Particle::Group { occurs, .. }
            | Particle::All { occurs, .. }
            | Particle::Any { occurs } => *occurs,
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

/// Validate that `element`'s children match `particle` in order.
pub fn validate_particle(
    element: &OpenXmlElement,
    particle: &Particle,
    path: &str,
) -> Vec<ValidationError> {
    let children: Vec<&OpenXmlElement> = element
        .children
        .iter()
        .filter(|c| c.local_name != "AlternateContent")
        .collect();

    let result = match_particle(particle, &children, 0);
    let mut errors = Vec::new();

    if result.consumed < children.len() {
        let extra = children[result.consumed];
        errors.push(ValidationError {
            path: format!("{path}/{}", extra.local_name),
            message: format!(
                "unexpected child `<{}>` at position {} under `<{}>` (particle mismatch)",
                extra.local_name, result.consumed, element.local_name
            ),
        });
    }
    for e in result.errors {
        errors.push(ValidationError {
            path: path.to_string(),
            message: e,
        });
    }
    errors
}

fn match_particle(particle: &Particle, children: &[&OpenXmlElement], start: usize) -> MatchResult {
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
        let one = match_once(particle, children, pos);
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

fn match_once(particle: &Particle, children: &[&OpenXmlElement], start: usize) -> MatchResult {
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
        Particle::Any { .. } => {
            if children.get(start).is_some() {
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
                let r = match_particle(item, children, start + total);
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
                let r = match_particle(item, children, start);
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
                    let r = match_particle(item, children, pos);
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
}

/// Recursively validate a Word document using ordered particles.
pub fn validate_word_particles(root: &OpenXmlElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if root.local_name != "document" {
        return errors;
    }
    errors.extend(validate_particle(root, &word::document(), "w:document"));
    if let Some(body) = root.child("body") {
        errors.extend(validate_particle(body, &word::body(), "w:document/w:body"));
        for (i, p) in body.children_by_name("p").enumerate() {
            let path = format!("w:document/w:body/w:p[{i}]");
            errors.extend(validate_particle(p, &word::paragraph(), &path));
            for (j, r) in p.children_by_name("r").enumerate() {
                let rpath = format!("{path}/w:r[{j}]");
                errors.extend(validate_particle(r, &word::run(), &rpath));
            }
        }
        for (i, tbl) in body.children_by_name("tbl").enumerate() {
            let path = format!("w:document/w:body/w:tbl[{i}]");
            errors.extend(validate_particle(tbl, &word::table(), &path));
            for (ri, tr) in tbl.children_by_name("tr").enumerate() {
                let tr_path = format!("{path}/w:tr[{ri}]");
                errors.extend(validate_particle(tr, &word::table_row(), &tr_path));
                for (ci, tc) in tr.children_by_name("tc").enumerate() {
                    let tc_path = format!("{tr_path}/w:tc[{ci}]");
                    errors.extend(validate_particle(tc, &word::table_cell(), &tc_path));
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
        assert!(errs.iter().any(|e| e.message.contains("unexpected")));
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
}
