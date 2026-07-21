//! Lightweight Linq-style navigation over [`OpenXmlElement`] trees.
//!
//! This is **not** a port of `DocumentFormat.OpenXml.Linq` XName tables. It provides
//! small iterator helpers inspired by that API for common queries without Features DI.

use super::OpenXmlElement;

/// Fluent query over an element subtree (self + descendants, or children only).
#[derive(Debug, Clone, Copy)]
pub struct ElementQuery<'a> {
    root: &'a OpenXmlElement,
    include_self: bool,
    descendants: bool,
}

impl<'a> ElementQuery<'a> {
    /// Query over `root` and all its descendants (depth-first).
    pub fn of(root: &'a OpenXmlElement) -> Self {
        Self {
            root,
            include_self: true,
            descendants: true,
        }
    }

    /// Query over direct children only.
    pub fn children(root: &'a OpenXmlElement) -> Self {
        Self {
            root,
            include_self: false,
            descendants: false,
        }
    }

    /// Restrict to elements with the given local name.
    pub fn named(self, local_name: &'a str) -> NamedQuery<'a> {
        NamedQuery {
            inner: self,
            local_name,
            attr: None,
            attr_value: None,
        }
    }

    /// Iterate matching elements.
    pub fn iter(self) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
        let root = self.root;
        let include_self = self.include_self;
        let descendants = self.descendants;
        std::iter::once(root)
            .filter(move |_| include_self)
            .chain(if descendants {
                Box::new(root.descendants()) as Box<dyn Iterator<Item = &'a OpenXmlElement> + 'a>
            } else {
                Box::new(root.children.iter()) as Box<dyn Iterator<Item = &'a OpenXmlElement> + 'a>
            })
    }
}

/// Filtered query by local name and optional attribute predicate.
#[derive(Debug, Clone, Copy)]
pub struct NamedQuery<'a> {
    inner: ElementQuery<'a>,
    local_name: &'a str,
    attr: Option<&'a str>,
    attr_value: Option<&'a str>,
}

impl<'a> NamedQuery<'a> {
    /// Require an attribute with any value.
    pub fn has_attr(mut self, name: &'a str) -> Self {
        self.attr = Some(name);
        self
    }

    /// Require an attribute equal to `value` (local-name match).
    pub fn attr_eq(mut self, name: &'a str, value: &'a str) -> Self {
        self.attr = Some(name);
        self.attr_value = Some(value);
        self
    }

    fn matches(&self, el: &OpenXmlElement) -> bool {
        if el.local_name != self.local_name {
            return false;
        }
        if let Some(name) = self.attr {
            let v = el
                .get_attribute(name)
                .or_else(|| el.get_attribute_qname(&format!("w:{name}")))
                .or_else(|| el.get_attribute_qname(&format!("x:{name}")))
                .or_else(|| el.get_attribute_qname(&format!("p:{name}")));
            match (v, self.attr_value) {
                (None, _) => return false,
                (Some(actual), Some(expected)) => {
                    if actual != expected {
                        return false;
                    }
                }
                (Some(_), None) => {}
            }
        }
        true
    }

    /// Iterate matching elements.
    pub fn iter(self) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
        let local = self.local_name;
        let attr = self.attr;
        let attr_value = self.attr_value;
        self.inner.iter().filter(move |el| {
            let q = NamedQuery {
                inner: ElementQuery {
                    root: el,
                    include_self: true,
                    descendants: false,
                },
                local_name: local,
                attr,
                attr_value,
            };
            q.matches(el)
        })
    }

    /// First match.
    pub fn first(self) -> Option<&'a OpenXmlElement> {
        self.iter().next()
    }

    /// Collect all matches.
    pub fn collect(self) -> Vec<&'a OpenXmlElement> {
        self.iter().collect()
    }

    /// Count matches.
    pub fn count(self) -> usize {
        self.iter().count()
    }

    /// Whether any match exists.
    pub fn any(self) -> bool {
        self.iter().next().is_some()
    }

    /// Concatenated `inner_text` of all matches.
    pub fn texts(self) -> Vec<String> {
        self.iter().map(|e| e.inner_text()).collect()
    }
}

/// Descendants of `root` with local name `local_name` (does not include `root` itself).
pub fn descendants_of<'a>(
    root: &'a OpenXmlElement,
    local_name: &'a str,
) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
    root.descendants()
        .filter(move |e| e.local_name == local_name)
}

/// Direct children named `local_name`.
pub fn elements_of<'a>(
    root: &'a OpenXmlElement,
    local_name: &'a str,
) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
    root.children_by_name(local_name)
}

/// First descendant (not self) with local name.
pub fn first_descendant<'a>(
    root: &'a OpenXmlElement,
    local_name: &'a str,
) -> Option<&'a OpenXmlElement> {
    descendants_of(root, local_name).next()
}

/// Attribute value on the first descendant named `element` with attribute `attr`.
pub fn descendant_attr<'a>(
    root: &'a OpenXmlElement,
    element: &'a str,
    attr: &'a str,
) -> Option<&'a str> {
    first_descendant(root, element).and_then(|e| {
        e.get_attribute(attr).or_else(|| {
            e.attributes
                .iter()
                .find(|a| a.local_name == attr)
                .map(|a| a.value.as_str())
        })
    })
}

impl OpenXmlElement {
    /// Start a Linq-style query over this element and descendants.
    pub fn query(&self) -> ElementQuery<'_> {
        ElementQuery::of(self)
    }

    /// Start a Linq-style query over direct children.
    pub fn query_children(&self) -> ElementQuery<'_> {
        ElementQuery::children(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wordprocessing::{body, document, paragraph, run, text};

    #[test]
    fn query_paragraphs_and_text() {
        let doc = document(vec![body(vec![
            paragraph(vec![run(vec![text("a")])]),
            paragraph(vec![run(vec![text("b")])]),
        ])]);
        let ps: Vec<_> = doc.query().named("p").collect();
        assert_eq!(ps.len(), 2);
        assert_eq!(doc.query().named("t").texts(), vec![String::from("a"), String::from("b")]);
        assert!(doc.query().named("p").any());
        assert_eq!(descendants_of(&doc, "r").count(), 2);
        assert!(first_descendant(&doc, "body").is_some());
    }

    #[test]
    fn attr_filter() {
        let mut p = paragraph(vec![]);
        p.set_attribute("rsidR", "00AB");
        let body_el = body(vec![p]);
        assert_eq!(
            body_el
                .query()
                .named("p")
                .attr_eq("rsidR", "00AB")
                .count(),
            1
        );
        assert_eq!(
            body_el
                .query()
                .named("p")
                .attr_eq("rsidR", "nope")
                .count(),
            0
        );
    }
}
