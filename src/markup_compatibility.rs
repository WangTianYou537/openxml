//! Markup Compatibility (MC) helpers.
//!
//! Implements:
//! - `mc:AlternateContent` / `mc:Choice` / `mc:Fallback`
//! - `mc:Ignorable` processing (strip / ProcessContent / Preserve*)

use crate::element::OpenXmlElement;
use crate::namespace::ns;
use std::collections::HashSet;

const MC: &str = ns::MARKUP_COMPATIBILITY.uri;

/// `mc:AlternateContent` root with the MC namespace declaration.
pub fn alternate_content(
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("mc", MC, "AlternateContent")
        .with_ns_decl("mc", MC)
        .with_children(children)
}

/// `mc:Choice` with a `Requires` attribute (space-separated namespace prefixes).
pub fn choice(
    requires: &str,
    children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    OpenXmlElement::new("mc", MC, "Choice")
        .with_attribute("Requires", requires)
        .with_children(children)
}

/// `mc:Fallback` content used when no Choice matches.
pub fn fallback(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {
    OpenXmlElement::new("mc", MC, "Fallback").with_children(children)
}

/// Build a complete AlternateContent with one Choice and a Fallback.
pub fn alternate_content_with(
    requires: &str,
    choice_children: impl IntoIterator<Item = OpenXmlElement>,
    fallback_children: impl IntoIterator<Item = OpenXmlElement>,
) -> OpenXmlElement {
    alternate_content(vec![
        choice(requires, choice_children),
        fallback(fallback_children),
    ])
}

/// Select content from an `mc:AlternateContent` element.
pub fn resolve_alternate_content(
    elem: &OpenXmlElement,
    supported_prefixes: &[&str],
) -> Vec<OpenXmlElement> {
    if elem.local_name != "AlternateContent" {
        return elem.children.clone();
    }

    for child in &elem.children {
        if child.local_name == "Choice" {
            let requires = child.get_attribute("Requires").unwrap_or("");
            let ok = requires
                .split_whitespace()
                .all(|p| supported_prefixes.contains(&p));
            if ok {
                return child.children.clone();
            }
        }
    }
    for child in &elem.children {
        if child.local_name == "Fallback" {
            return child.children.clone();
        }
    }
    Vec::new()
}

/// Recursively expand AlternateContent nodes in a tree.
pub fn expand_alternate_content(root: &mut OpenXmlElement, supported_prefixes: &[&str]) {
    for child in &mut root.children {
        expand_alternate_content(child, supported_prefixes);
    }

    let mut new_children = Vec::new();
    for child in std::mem::take(&mut root.children) {
        if child.local_name == "AlternateContent" {
            new_children.extend(resolve_alternate_content(&child, supported_prefixes));
        } else {
            new_children.push(child);
        }
    }
    root.children = new_children;
}

// ---------------------------------------------------------------------------
// Ignorable processing
// ---------------------------------------------------------------------------

/// Parse `mc:Ignorable="w14 w15"` into a set of prefixes.
pub fn parse_ignorable_prefixes(value: &str) -> HashSet<String> {
    value
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Read `mc:Ignorable` from an element's attributes.
pub fn ignorable_from_element(elem: &OpenXmlElement) -> HashSet<String> {
    for a in &elem.attributes {
        let is_ignorable = a.local_name == "Ignorable"
            && (a.prefix.as_deref() == Some("mc") || a.prefix.is_none());
        if is_ignorable {
            return parse_ignorable_prefixes(&a.value);
        }
    }
    HashSet::new()
}

/// Process Markup Compatibility on a tree:
///
/// 1. Expand `mc:AlternateContent` using `supported_prefixes`.
/// 2. For elements in ignorable-but-unsupported namespaces:
///    - if listed in `mc:ProcessContent` (as `pfx:*` or `pfx:local`), keep the
///      element but promote its children (unwrap);
///    - if listed in `mc:PreserveElements`, keep the element as-is;
///    - otherwise strip the element entirely.
/// 3. Strip attributes whose prefix is ignorable and unsupported, unless listed
///    in `mc:PreserveAttributes`.
///
/// Returns the number of nodes/attributes removed or unwrapped.
pub fn process_markup_compatibility(
    root: &mut OpenXmlElement,
    supported_prefixes: &[&str],
) -> usize {
    expand_alternate_content(root, supported_prefixes);
    let mut ctx = McProcessContext::default();
    process_mc_node(root, supported_prefixes, &mut ctx)
}

/// Process MC using a target [`FileFormatVersions`](crate::file_format::FileFormatVersions).
///
/// Supported prefixes are derived from the version matrix (e.g. targeting
/// Office 2010 supports `w` and `w14` but not `w15`).
pub fn process_markup_compatibility_for_version(
    root: &mut OpenXmlElement,
    target: crate::file_format::FileFormatVersions,
) -> usize {
    let prefixes = crate::file_format::supported_prefixes(target);
    process_markup_compatibility(root, &prefixes)
}

#[derive(Default)]
struct McProcessContext {
    ignorable: HashSet<String>,
    /// "prefix:local" or "prefix:*"
    process_content: HashSet<String>,
    preserve_elements: HashSet<String>,
    preserve_attributes: HashSet<String>,
}

fn parse_qname_list(value: &str) -> HashSet<String> {
    value
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn mc_attr(elem: &OpenXmlElement, local: &str) -> Option<String> {
    for a in &elem.attributes {
        if a.local_name == local && (a.prefix.as_deref() == Some("mc") || a.prefix.is_none()) {
            return Some(a.value.clone());
        }
    }
    None
}

fn qname_matches(set: &HashSet<String>, prefix: &str, local: &str) -> bool {
    set.contains(&format!("{prefix}:{local}")) || set.contains(&format!("{prefix}:*"))
}

fn process_mc_node(
    elem: &mut OpenXmlElement,
    supported: &[&str],
    ctx: &mut McProcessContext,
) -> usize {
    let mut removed = 0usize;

    // Push this element's MC attributes
    let mut pushed_ign = Vec::new();
    let mut pushed_pc = Vec::new();
    let mut pushed_pe = Vec::new();
    let mut pushed_pa = Vec::new();

    if let Some(v) = mc_attr(elem, "Ignorable") {
        for p in parse_ignorable_prefixes(&v) {
            if ctx.ignorable.insert(p.clone()) {
                pushed_ign.push(p);
            }
        }
    }
    if let Some(v) = mc_attr(elem, "ProcessContent") {
        for q in parse_qname_list(&v) {
            if ctx.process_content.insert(q.clone()) {
                pushed_pc.push(q);
            }
        }
    }
    if let Some(v) = mc_attr(elem, "PreserveElements") {
        for q in parse_qname_list(&v) {
            if ctx.preserve_elements.insert(q.clone()) {
                pushed_pe.push(q);
            }
        }
    }
    if let Some(v) = mc_attr(elem, "PreserveAttributes") {
        for q in parse_qname_list(&v) {
            if ctx.preserve_attributes.insert(q.clone()) {
                pushed_pa.push(q);
            }
        }
    }

    // Strip ignorable attributes (unless preserved)
    let before = elem.attributes.len();
    elem.attributes.retain(|a| match &a.prefix {
        Some(pfx)
            if ctx.ignorable.contains(pfx.as_str())
                && !supported.contains(&pfx.as_str())
                && !qname_matches(&ctx.preserve_attributes, pfx, &a.local_name) =>
        {
            false
        }
        _ => true,
    });
    removed += before - elem.attributes.len();

    // Process children
    let mut kept = Vec::new();
    for mut child in std::mem::take(&mut elem.children) {
        let pfx = child.prefix.as_str();
        let is_ignorable_ns = !pfx.is_empty()
            && ctx.ignorable.contains(pfx)
            && !supported.contains(&pfx);

        if is_ignorable_ns {
            if qname_matches(&ctx.preserve_elements, pfx, &child.local_name) {
                // keep as-is, still recurse inside
                removed += process_mc_node(&mut child, supported, ctx);
                kept.push(child);
            } else if qname_matches(&ctx.process_content, pfx, &child.local_name) {
                // unwrap: process grandchildren and promote them
                removed += 1; // the wrapper itself
                removed += process_mc_node(&mut child, supported, ctx);
                kept.extend(child.children);
            } else {
                // strip entirely
                removed += 1;
            }
        } else {
            removed += process_mc_node(&mut child, supported, ctx);
            kept.push(child);
        }
    }
    elem.children = kept;

    // Pop
    for p in pushed_ign {
        ctx.ignorable.remove(&p);
    }
    for p in pushed_pc {
        ctx.process_content.remove(&p);
    }
    for p in pushed_pe {
        ctx.preserve_elements.remove(&p);
    }
    for p in pushed_pa {
        ctx.preserve_attributes.remove(&p);
    }

    removed
}

/// Attach `mc:Ignorable` to an element (and ensure mc xmlns is declared).
pub fn with_ignorable(mut elem: OpenXmlElement, prefixes: &str) -> OpenXmlElement {
    let has_mc = elem.namespace_declarations.iter().any(|(p, _)| p == "mc");
    if !has_mc {
        elem.namespace_declarations
            .push(("mc".into(), MC.into()));
    }
    elem.set_attribute_ns("mc", MC, "Ignorable", prefixes);
    elem
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;

    #[test]
    fn resolve_choice() {
        let ac = alternate_content_with(
            "w14",
            vec![OpenXmlElement::w("choiceContent")],
            vec![OpenXmlElement::w("fallbackContent")],
        );
        let resolved = resolve_alternate_content(&ac, &["w14", "w"]);
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].local_name, "choiceContent");

        let resolved = resolve_alternate_content(&ac, &["w"]);
        assert_eq!(resolved[0].local_name, "fallbackContent");
    }

    #[test]
    fn expand_in_tree() {
        let mut root = OpenXmlElement::w("body").with_child(alternate_content_with(
            "w14",
            vec![OpenXmlElement::w("new")],
            vec![OpenXmlElement::w("old")],
        ));
        expand_alternate_content(&mut root, &["w"]);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].local_name, "old");
    }

    #[test]
    fn strip_ignorable_elements() {
        let mut root = with_ignorable(
            OpenXmlElement::w("document")
                .with_ns_decl("w", "http://schemas.openxmlformats.org/wordprocessingml/2006/main")
                .with_ns_decl(
                    "w14",
                    "http://schemas.microsoft.com/office/word/2010/wordml",
                )
                .with_child(OpenXmlElement::w("body"))
                .with_child(OpenXmlElement::new(
                    "w14",
                    "http://schemas.microsoft.com/office/word/2010/wordml",
                    "docId",
                )),
            "w14",
        );
        let n = process_markup_compatibility(&mut root, &["w"]);
        assert!(n >= 1);
        assert!(root.children.iter().all(|c| c.prefix != "w14"));
        assert!(root.child("body").is_some());
    }

    #[test]
    fn process_content_unwrap() {
        let mut root = with_ignorable(
            OpenXmlElement::w("document")
                .with_child(OpenXmlElement::new(
                    "w14",
                    "http://schemas.microsoft.com/office/word/2010/wordml",
                    "wrapper",
                ).with_child(OpenXmlElement::w("inner"))),
            "w14",
        );
        // Also set ProcessContent
        root.set_attribute_ns("mc", MC, "ProcessContent", "w14:*");
        let n = process_markup_compatibility(&mut root, &["w"]);
        assert!(n >= 1);
        // wrapper stripped, inner promoted
        assert!(root.children.iter().any(|c| c.local_name == "inner"));
        assert!(root.children.iter().all(|c| c.local_name != "wrapper"));
    }

    #[test]
    fn preserve_elements() {
        let mut root = with_ignorable(
            OpenXmlElement::w("document").with_child(OpenXmlElement::new(
                "w14",
                "http://schemas.microsoft.com/office/word/2010/wordml",
                "docId",
            )),
            "w14",
        );
        root.set_attribute_ns("mc", MC, "PreserveElements", "w14:docId");
        process_markup_compatibility(&mut root, &["w"]);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].local_name, "docId");
    }

    #[test]
    fn keep_supported_ignorable() {
        let mut root = with_ignorable(
            OpenXmlElement::w("document").with_child(OpenXmlElement::new(
                "w14",
                "http://schemas.microsoft.com/office/word/2010/wordml",
                "docId",
            )),
            "w14",
        );
        process_markup_compatibility(&mut root, &["w", "w14"]);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].prefix, "w14");
    }
}
