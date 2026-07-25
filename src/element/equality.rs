//! Structural equality helpers for [`OpenXmlElement`] trees
//! (C# `OpenXmlElementEqualityComparer` / `OpenXmlElementEqualityOptions` shell).

use super::OpenXmlElement;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Options controlling deep equality comparison of Open XML elements.
///
/// Field names lean toward the Rust helpers; C# counterparts:
/// - `include_mc_attributes` ↔ `IncludeMCAttributes` (inverse of ignore)
/// - `compare_prefixes` ↔ inverse of `SkipPrefixComparison`
#[derive(Debug, Clone, Copy)]
pub struct EqualityOptions {
    /// Compare namespace URIs (not only local names / prefixes).
    pub compare_namespaces: bool,
    /// Compare attribute values (always by local name).
    pub compare_attributes: bool,
    /// When false, ignore `mc:` Markup Compatibility attributes (C# `IncludeMCAttributes = false`).
    pub include_mc_attributes: bool,
    /// Compare text content / leaf text.
    pub compare_text: bool,
    /// Compare element prefixes in addition to local names (C# inverse of `SkipPrefixComparison`).
    pub compare_prefixes: bool,
    /// When both sides have `raw_outer_xml` and are otherwise empty of children, compare raw XML
    /// (C# unparsed `RawOuterXml` path when `RequireParsed` is false).
    pub allow_raw_outer_xml: bool,
}

impl Default for EqualityOptions {
    fn default() -> Self {
        Self {
            compare_namespaces: true,
            compare_attributes: true,
            include_mc_attributes: false, // historical Rust default: ignore MC attrs
            compare_text: true,
            compare_prefixes: false,
            allow_raw_outer_xml: true,
        }
    }
}

impl EqualityOptions {
    /// Options closer to C# defaults (`IncludeMCAttributes = true`, no skip prefix).
    pub fn csharp_defaults() -> Self {
        Self {
            compare_namespaces: true,
            compare_attributes: true,
            include_mc_attributes: true,
            compare_text: true,
            compare_prefixes: true,
            allow_raw_outer_xml: true,
        }
    }

    /// Backward-compat alias: `ignore_mc_attributes` inverted.
    pub fn with_ignore_mc_attributes(mut self, ignore: bool) -> Self {
        self.include_mc_attributes = !ignore;
        self
    }
}

/// Deep structural equality of two elements using [`EqualityOptions::default`].
pub fn elements_equal(a: &OpenXmlElement, b: &OpenXmlElement) -> bool {
    elements_equal_with(a, b, &EqualityOptions::default())
}

/// Deep structural equality with custom options.
pub fn elements_equal_with(
    a: &OpenXmlElement,
    b: &OpenXmlElement,
    opts: &EqualityOptions,
) -> bool {
    if opts.allow_raw_outer_xml {
        if let (Some(ra), Some(rb)) = (&a.raw_outer_xml, &b.raw_outer_xml) {
            if a.children.is_empty() && b.children.is_empty() && a.text.is_none() && b.text.is_none()
            {
                return ra == rb;
            }
        }
    }

    if a.local_name != b.local_name {
        return false;
    }
    if a.misc_kind != b.misc_kind {
        return false;
    }
    if opts.compare_prefixes && a.prefix != b.prefix {
        return false;
    }
    if opts.compare_namespaces && a.namespace_uri != b.namespace_uri {
        return false;
    }
    if opts.compare_text && a.text != b.text {
        return false;
    }
    if opts.compare_attributes {
        let mut attrs_a: Vec<_> = a
            .attributes
            .iter()
            .filter(|attr| opts.include_mc_attributes || !is_mc_attr(attr))
            .map(|attr| {
                (
                    attr.prefix.as_deref().unwrap_or(""),
                    attr.local_name.as_str(),
                    attr.value.as_str(),
                )
            })
            .collect();
        let mut attrs_b: Vec<_> = b
            .attributes
            .iter()
            .filter(|attr| opts.include_mc_attributes || !is_mc_attr(attr))
            .map(|attr| {
                (
                    attr.prefix.as_deref().unwrap_or(""),
                    attr.local_name.as_str(),
                    attr.value.as_str(),
                )
            })
            .collect();
        attrs_a.sort_unstable();
        attrs_b.sort_unstable();
        if attrs_a != attrs_b {
            return false;
        }
    }
    if a.children.len() != b.children.len() {
        return false;
    }
    a.children
        .iter()
        .zip(b.children.iter())
        .all(|(ca, cb)| elements_equal_with(ca, cb, opts))
}

/// Structural hash of an element tree under the given options (C# `GetHashCode` shell).
pub fn element_hash_with(el: &OpenXmlElement, opts: &EqualityOptions) -> u64 {
    let mut h = DefaultHasher::new();
    hash_element(el, opts, &mut h);
    h.finish()
}

pub fn element_hash(el: &OpenXmlElement) -> u64 {
    element_hash_with(el, &EqualityOptions::default())
}

fn hash_element(el: &OpenXmlElement, opts: &EqualityOptions, h: &mut DefaultHasher) {
    el.local_name.hash(h);
    if opts.compare_prefixes {
        el.prefix.hash(h);
    }
    if opts.compare_namespaces {
        el.namespace_uri.hash(h);
    }
    if opts.compare_text {
        el.text.hash(h);
    }
    if opts.compare_attributes {
        let mut attrs: Vec<_> = el
            .attributes
            .iter()
            .filter(|attr| opts.include_mc_attributes || !is_mc_attr(attr))
            .map(|attr| {
                (
                    attr.prefix.clone().unwrap_or_default(),
                    attr.local_name.clone(),
                    attr.value.clone(),
                )
            })
            .collect();
        attrs.sort_unstable();
        for (p, n, v) in attrs {
            p.hash(h);
            n.hash(h);
            v.hash(h);
        }
    }
    for c in &el.children {
        hash_element(c, opts, h);
    }
}

fn is_mc_attr(attr: &crate::element::OpenXmlAttribute) -> bool {
    attr.prefix.as_deref() == Some("mc")
        || matches!(
            attr.local_name.as_str(),
            "Ignorable"
                | "ProcessContent"
                | "PreserveElements"
                | "PreserveAttributes"
                | "MustUnderstand"
        )
}

/// Adapter implementing [`PartialEq`] semantics via [`elements_equal_with`].
#[derive(Debug, Clone, Copy)]
pub struct ElementComparer {
    pub options: EqualityOptions,
}

impl ElementComparer {
    pub fn new(options: EqualityOptions) -> Self {
        Self { options }
    }

    pub fn default_comparer() -> Self {
        Self {
            options: EqualityOptions::default(),
        }
    }

    pub fn eq(&self, a: &OpenXmlElement, b: &OpenXmlElement) -> bool {
        elements_equal_with(a, b, &self.options)
    }

    pub fn hash(&self, el: &OpenXmlElement) -> u64 {
        element_hash_with(el, &self.options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlAttribute;

    #[test]
    fn equal_trees() {
        let a = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("hi"));
        let b = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("hi"));
        assert!(elements_equal(&a, &b));
        assert_eq!(element_hash(&a), element_hash(&b));
    }

    #[test]
    fn unequal_text() {
        let a = OpenXmlElement::w("t").with_text("a");
        let b = OpenXmlElement::w("t").with_text("b");
        assert!(!elements_equal(&a, &b));
    }

    #[test]
    fn ignore_mc_by_default() {
        let mut a = OpenXmlElement::w("p");
        a.attributes.push(OpenXmlAttribute {
            prefix: Some("mc".into()),
            namespace_uri: Some("http://schemas.openxmlformats.org/markup-compatibility/2006".into()),
            local_name: "Ignorable".into(),
            value: "w14".into(),
        });
        let b = OpenXmlElement::w("p");
        assert!(elements_equal(&a, &b));
        let strict = EqualityOptions {
            include_mc_attributes: true,
            ..EqualityOptions::default()
        };
        assert!(!elements_equal_with(&a, &b, &strict));
    }

    #[test]
    fn comparer_and_raw_outer() {
        let mut a = OpenXmlElement::w("p");
        a.raw_outer_xml = Some("<w:p/>".into());
        let mut b = OpenXmlElement::w("p");
        b.raw_outer_xml = Some("<w:p/>".into());
        assert!(elements_equal(&a, &b));
        b.raw_outer_xml = Some("<w:p></w:p>".into());
        assert!(!elements_equal(&a, &b));
        let c = ElementComparer::default_comparer();
        assert!(c.eq(&OpenXmlElement::w("x"), &OpenXmlElement::w("x")));
    }
}
