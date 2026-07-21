//! Structural equality helpers for [`OpenXmlElement`] trees.

use super::OpenXmlElement;

/// Options controlling deep equality comparison of Open XML elements.
#[derive(Debug, Clone, Copy)]
pub struct EqualityOptions {
    /// Compare namespace URIs (not only local names / prefixes).
    pub compare_namespaces: bool,
    /// Compare attribute values (always by local name).
    pub compare_attributes: bool,
    /// Ignore `mc:` Markup Compatibility attributes when comparing attributes.
    pub ignore_mc_attributes: bool,
    /// Compare text content.
    pub compare_text: bool,
}

impl Default for EqualityOptions {
    fn default() -> Self {
        Self {
            compare_namespaces: true,
            compare_attributes: true,
            ignore_mc_attributes: true,
            compare_text: true,
        }
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
    if a.local_name != b.local_name {
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
            .filter(|attr| !(opts.ignore_mc_attributes && is_mc_attr(attr)))
            .map(|attr| (attr.local_name.as_str(), attr.value.as_str()))
            .collect();
        let mut attrs_b: Vec<_> = b
            .attributes
            .iter()
            .filter(|attr| !(opts.ignore_mc_attributes && is_mc_attr(attr)))
            .map(|attr| (attr.local_name.as_str(), attr.value.as_str()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_trees() {
        let a = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("hi"));
        let b = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r").with_text("hi"));
        assert!(elements_equal(&a, &b));
    }

    #[test]
    fn unequal_text() {
        let a = OpenXmlElement::w("t").with_text("a");
        let b = OpenXmlElement::w("t").with_text("b");
        assert!(!elements_equal(&a, &b));
    }
}
