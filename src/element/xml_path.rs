//! XPath-like location for Open XML elements (C# `XmlPath`).
//!
//! Because Rust DOM nodes are owned (no parent pointers), paths are built either
//! for a standalone root or by walking a known root with a child-index trail.

use super::element::{OpenXmlElement, OpenXmlMiscKind};
use std::fmt;

/// XPath-like information for an element or part (C# `DocumentFormat.OpenXml.XmlPath`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlPath {
    /// XPath string from the part root, e.g. `/w:document[1]/w:body[1]/w:p[2]`.
    pub xpath: String,
    /// Part URI relative to the package root, when known.
    pub part_uri: Option<String>,
    /// Prefix → namespace URI pairs used in [`xpath`](Self::xpath).
    pub namespaces: Vec<(String, String)>,
}

impl fmt::Display for XmlPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(uri) = &self.part_uri {
            write!(f, "{uri}{}", self.xpath)
        } else {
            write!(f, "{}", self.xpath)
        }
    }
}

impl XmlPath {
    /// Path for a part with empty element XPath (C# `XmlPath(OpenXmlPart)`).
    pub fn for_part(part_uri: impl Into<String>) -> Self {
        Self {
            xpath: String::new(),
            part_uri: Some(part_uri.into()),
            namespaces: Vec::new(),
        }
    }

    /// Path treating `element` as the sole root (`/prefix:local[1]`).
    pub fn for_root(element: &OpenXmlElement, part_uri: Option<String>) -> Self {
        let mut namespaces = Vec::new();
        let mut xpath = String::new();
        append_step(&mut xpath, &mut namespaces, element, 1);
        Self {
            xpath,
            part_uri,
            namespaces,
        }
    }

    /// Build path from `root` following zero-based child indices to a descendant.
    ///
    /// `child_indices` empty yields the path of `root` alone.
    /// Returns `None` if any index is out of range.
    pub fn at(
        root: &OpenXmlElement,
        child_indices: &[usize],
        part_uri: Option<String>,
    ) -> Option<Self> {
        let mut namespaces = Vec::new();
        let mut xpath = String::new();
        append_step(&mut xpath, &mut namespaces, root, 1);

        let mut current = root;
        for &idx in child_indices {
            let child = current.children.get(idx)?;
            let xpath_index = xpath_index_among_siblings(current, idx);
            append_step(&mut xpath, &mut namespaces, child, xpath_index);
            current = child;
        }

        Some(Self {
            xpath,
            part_uri,
            namespaces,
        })
    }

    /// C# `XmlPath.GetXPath(element)` when the element is its own tree root.
    pub fn get_xpath(element: &OpenXmlElement) -> Self {
        Self::for_root(element, None)
    }

    pub fn lookup_namespace(&self, prefix: &str) -> Option<&str> {
        self.namespaces
            .iter()
            .find(|(p, _)| p == prefix)
            .map(|(_, u)| u.as_str())
    }

    pub fn lookup_prefix(&self, namespace_uri: &str) -> Option<&str> {
        self.namespaces
            .iter()
            .find(|(_, u)| u == namespace_uri)
            .map(|(p, _)| p.as_str())
    }
}

/// 1-based index among same-name siblings (C# `GetXPathIndex`).
///
/// For misc nodes always returns `1`. Counts prior siblings with the same
/// `(namespace_uri, local_name)` pair.
pub fn xpath_index_among_siblings(parent: &OpenXmlElement, child_index: usize) -> usize {
    let Some(child) = parent.children.get(child_index) else {
        return 1;
    };
    if child.is_misc_node() {
        return 1;
    }
    let mut count = 1;
    for (i, sib) in parent.children.iter().enumerate() {
        if i == child_index {
            break;
        }
        if sib.is_misc_node() {
            continue;
        }
        if sib.namespace_uri == child.namespace_uri && sib.local_name == child.local_name {
            count += 1;
        }
    }
    count
}

fn append_step(
    xpath: &mut String,
    namespaces: &mut Vec<(String, String)>,
    element: &OpenXmlElement,
    index: usize,
) {
    xpath.push('/');
    if element.misc_kind != OpenXmlMiscKind::None {
        // C# appends OuterXml for misc; keep a stable short form.
        match element.misc_kind {
            OpenXmlMiscKind::Comment => xpath.push_str("#comment"),
            OpenXmlMiscKind::ProcessingInstruction => xpath.push_str("#pi"),
            OpenXmlMiscKind::CData => xpath.push_str("#cdata-section"),
            OpenXmlMiscKind::None => {}
        }
        return;
    }

    if !element.prefix.is_empty() {
        if !namespaces.iter().any(|(p, _)| p == &element.prefix) {
            namespaces.push((element.prefix.clone(), element.namespace_uri.clone()));
        }
        xpath.push_str(&element.prefix);
        xpath.push(':');
    } else if !element.namespace_uri.is_empty() {
        // No prefix but has URI — C# writes `uri:local`.
        xpath.push_str(&element.namespace_uri);
        xpath.push(':');
    }
    xpath.push_str(&element.local_name);
    xpath.push('[');
    xpath.push_str(&index.to_string());
    xpath.push(']');
}

impl OpenXmlElement {
    /// [`XmlPath`] for this element as a tree root.
    pub fn xml_path(&self) -> XmlPath {
        XmlPath::for_root(self, None)
    }

    /// [`XmlPath`] for the descendant at `child_indices` (0-based from self).
    pub fn xml_path_at(&self, child_indices: &[usize]) -> Option<XmlPath> {
        XmlPath::at(self, child_indices, None)
    }

    /// [`XmlPath`] including a part URI.
    pub fn xml_path_in_part(&self, part_uri: impl Into<String>) -> XmlPath {
        XmlPath::for_root(self, Some(part_uri.into()))
    }
}

/// Marker annotation for C# `OpenXmlUnknownElement` (elements outside the ECMA schema set).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OpenXmlUnknownMarker;

impl OpenXmlElement {
    /// Create an unknown/extension element (C# `OpenXmlUnknownElement`).
    pub fn unknown(
        prefix: impl Into<String>,
        local_name: impl Into<String>,
        namespace_uri: impl Into<String>,
    ) -> Self {
        let mut el = Self::new(prefix, namespace_uri, local_name);
        el.add_annotation(OpenXmlUnknownMarker);
        el
    }

    /// Parse XML into an element and mark the root as unknown.
    pub fn unknown_from_xml(xml: impl AsRef<[u8]>) -> crate::error::Result<Self> {
        let mut el = super::reader::parse_element(xml)?;
        el.add_annotation(OpenXmlUnknownMarker);
        Ok(el)
    }

    /// Whether this node is marked as an unknown element (C# `is OpenXmlUnknownElement`).
    pub fn is_unknown(&self) -> bool {
        self.has_annotation::<OpenXmlUnknownMarker>()
    }

    /// Mark this element as unknown (does not clear children).
    pub fn mark_unknown(&mut self) {
        if !self.is_unknown() {
            self.add_annotation(OpenXmlUnknownMarker);
        }
    }

    /// True when the element is a normal schema element (not misc, not unknown).
    /// C# `OpenXmlElement.Is` / “not Unknown and not Misc”.
    pub fn is_known_element(&self) -> bool {
        !self.is_misc_node() && !self.is_unknown()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wordprocessing::{body, document, paragraph, run, text};

    #[test]
    fn xpath_nested_paragraphs() {
        let doc = document(vec![body(vec![
            paragraph(vec![run(vec![text("a")])]),
            paragraph(vec![run(vec![text("b")])]),
        ])]);
        let path = XmlPath::at(&doc, &[0, 1], None).unwrap();
        assert!(
            path.xpath.contains("body") && path.xpath.contains("p[2]"),
            "{}",
            path.xpath
        );
        assert_eq!(path.lookup_prefix("http://schemas.openxmlformats.org/wordprocessingml/2006/main").or_else(|| path.lookup_namespace("w").map(|_| "w")), path.lookup_namespace("w").map(|_| "w"));
        assert!(path.namespaces.iter().any(|(p, _)| p == "w"));
    }

    #[test]
    fn unknown_marker() {
        let el = OpenXmlElement::unknown("ex", "foo", "urn:example");
        assert!(el.is_unknown());
        assert!(!el.is_known_element());
        assert!(!el.is_misc_node());
    }

    #[test]
    fn misc_step() {
        let mut root = OpenXmlElement::new("w", "urn:w", "p");
        root.children.push(OpenXmlElement::comment("x"));
        let path = XmlPath::at(&root, &[0], None).unwrap();
        assert!(path.xpath.contains("#comment"), "{}", path.xpath);
    }

    #[test]
    fn for_part_empty_xpath() {
        let p = XmlPath::for_part("/word/document.xml");
        assert!(p.xpath.is_empty());
        assert_eq!(p.part_uri.as_deref(), Some("/word/document.xml"));
    }
}

/// Optional source line/position for a reader cursor (C# `IXmlLineInfo` / `XmlLineInfo`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct XmlLineInfo {
    pub line_number: u64,
    pub line_position: u64,
}

impl XmlLineInfo {
    /// No line information (C# `XmlLineInfo.Empty`).
    pub const EMPTY: Self = Self {
        line_number: 0,
        line_position: 0,
    };

    pub fn new(line_number: u64, line_position: u64) -> Self {
        Self {
            line_number,
            line_position,
        }
    }

    pub fn has_line_info(self) -> bool {
        self.line_number > 0
    }
}

#[cfg(test)]
mod line_info_tests {
    use super::*;

    #[test]
    fn xml_line_info_empty() {
        assert!(!XmlLineInfo::EMPTY.has_line_info());
        assert_eq!(XmlLineInfo::new(3, 10).line_number, 3);
        assert!(XmlLineInfo::new(1, 1).has_line_info());
    }
}

