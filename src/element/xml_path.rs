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

fn child_info_local_name(name: &str) -> &str {
    // ChildInfo names are `prefix:CT_Type/prefix:localName`.
    let tail = name.rsplit('/').next().unwrap_or(name);
    tail.rsplit(':').next().unwrap_or(tail)
}

/// Schema child-name strings for Word / Spreadsheet / Presentation / Drawing.
///
/// Returns the raw `ChildInfo.name` entries (`prefix:CT_Type/prefix:local`) so
/// callers can match on the local name portion via [`child_info_local_name`].
fn schema_child_names(element: &OpenXmlElement) -> Option<Vec<&'static str>> {
    macro_rules! try_schema {
        ($mod:path) => {{
            use $mod as schema;
            if let Some(info) = schema::info_by_local_name(&element.local_name) {
                if element.prefix == info.prefix {
                    return Some(info.children.iter().map(|c| c.name).collect());
                }
            }
        }};
    }
    match element.prefix.as_str() {
        "w" => try_schema!(crate::generated::wordprocessingml_2006_main),
        "x" => try_schema!(crate::generated::spreadsheetml_2006_main),
        "p" => try_schema!(crate::generated::presentationml_2006_main),
        "a" => try_schema!(crate::generated::drawingml_2006_main),
        _ => {}
    }
    try_schema!(crate::generated::wordprocessingml_2006_main);
    try_schema!(crate::generated::spreadsheetml_2006_main);
    try_schema!(crate::generated::presentationml_2006_main);
    try_schema!(crate::generated::drawingml_2006_main);
    None
}

/// Resolve (prefix, namespace_uri) for a child local name under the parent's schema.
fn resolve_child_element_identity(
    parent: &OpenXmlElement,
    prefix: &str,
    local_name: &str,
) -> Option<(&'static str, &'static str)> {
    macro_rules! try_schema {
        ($mod:path) => {{
            use $mod as schema;
            if let Some(parent_info) = schema::info_by_local_name(&parent.local_name) {
                if parent.prefix == parent_info.prefix {
                    if let Some(info) = schema::info_by_local_name(local_name) {
                        if prefix.is_empty() || info.prefix == prefix {
                            return Some((info.prefix, info.namespace_uri));
                        }
                    }
                }
            }
        }};
    }
    match parent.prefix.as_str() {
        "w" => try_schema!(crate::generated::wordprocessingml_2006_main),
        "x" => try_schema!(crate::generated::spreadsheetml_2006_main),
        "p" => try_schema!(crate::generated::presentationml_2006_main),
        "a" => try_schema!(crate::generated::drawingml_2006_main),
        _ => {}
    }
    try_schema!(crate::generated::wordprocessingml_2006_main);
    try_schema!(crate::generated::spreadsheetml_2006_main);
    try_schema!(crate::generated::presentationml_2006_main);
    try_schema!(crate::generated::drawingml_2006_main);

    // Fallback: use the requested prefix (or parent prefix) via the namespace table.
    let want = if prefix.is_empty() {
        parent.prefix.as_str()
    } else {
        prefix
    };
    let uri = crate::generated::namespaces::uri_for_prefix(want)?;
    let static_prefix = crate::generated::namespaces::prefix_for_uri(uri)?;
    Some((static_prefix, uri))
}

impl OpenXmlElement {
    /// C# `GetXPathIndex` — 1-based index of `child` among same-name siblings
    /// of `self`. Misc nodes are always `1`; unknown children match on
    /// namespace + local name (same rule as known children in this DOM).
    pub fn get_xpath_index(&self, child_index: usize) -> usize {
        xpath_index_among_siblings(self, child_index)
    }

    /// C# `GetAttributeValueEx` — attribute value or `None` (no throw).
    pub fn get_attribute_value_ex(&self, local_name: &str, namespace_uri: &str) -> Option<&str> {
        self.get_attribute_ns(local_name, namespace_uri)
    }

    /// C# `CanContainChild` — whether the schema children table of `self`
    /// (Word / Spreadsheet / Presentation / Drawing metadata) lists `child`'s
    /// local name.
    pub fn can_contain_child(&self, child: &OpenXmlElement) -> bool {
        if child.is_misc_node() || child.is_unknown() {
            return false;
        }
        let Some(children) = schema_child_names(self) else {
            return false;
        };
        children
            .iter()
            .any(|name| child_info_local_name(name) == child.local_name)
    }

    /// C# `TryCreateValidChild` — create an empty child element when the parent
    /// allows it and its namespace prefix is available in `file_format`.
    ///
    /// Looks up child tables across Word / Spreadsheet / Presentation / Drawing.
    pub fn try_create_valid_child(
        &self,
        file_format: crate::file_format::FileFormatVersions,
        prefix: &str,
        local_name: &str,
    ) -> Option<OpenXmlElement> {
        let children = schema_child_names(self)?;
        if !children
            .iter()
            .any(|name| child_info_local_name(name) == local_name)
        {
            return None;
        }
        let (resolved_prefix, namespace_uri) =
            resolve_child_element_identity(self, prefix, local_name)?;
        if !file_format
            .includes_introduction(crate::file_format::prefix_introduced_in(resolved_prefix))
        {
            return None;
        }
        Some(OpenXmlElement::new(resolved_prefix, namespace_uri, local_name))
    }

    /// C# `IsInVersion` — whether this element's namespace prefix is defined in
    /// `version` (misc/unknown elements are never "in version").
    pub fn is_in_version(&self, version: crate::file_format::FileFormatVersions) -> bool {
        if !self.is_known_element() {
            return false;
        }
        if self.prefix.is_empty() {
            return true;
        }
        version.includes_introduction(crate::file_format::prefix_introduced_in(&self.prefix))
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

    #[test]
    fn xpath_index_counts_same_name_siblings() {
        let doc = body(vec![
            paragraph(vec![]),
            paragraph(vec![]),
            paragraph(vec![]),
        ]);
        assert_eq!(doc.get_xpath_index(0), 1);
        assert_eq!(doc.get_xpath_index(2), 3);

        let mut mixed = body(vec![paragraph(vec![])]);
        mixed.append_child(OpenXmlElement::comment("x"));
        mixed.append_child(paragraph(vec![]));
        assert_eq!(mixed.get_xpath_index(1), 1); // misc always 1
        assert_eq!(mixed.get_xpath_index(2), 2);
    }

    #[test]
    fn can_contain_and_create_valid_children() {
        use crate::file_format::FileFormatVersions;

        let p = paragraph(vec![]);
        assert!(p.can_contain_child(&run(vec![])));
        assert!(!p.can_contain_child(&body(vec![])));
        assert!(!p.can_contain_child(&OpenXmlElement::comment("x")));
        assert!(!p.can_contain_child(&OpenXmlElement::unknown("x", "r", "urn:x")));

        let created = p
            .try_create_valid_child(FileFormatVersions::OFFICE2007, "w", "r")
            .expect("w:r allowed in w:p");
        assert_eq!(created.local_name, "r");
        assert_eq!(created.prefix, "w");
        assert!(p
            .try_create_valid_child(FileFormatVersions::OFFICE2007, "w", "body")
            .is_none());
    }

    #[test]
    fn spreadsheet_can_contain_and_create_valid_children() {
        use crate::file_format::FileFormatVersions;

        let row = OpenXmlElement::x("row");
        let cell = OpenXmlElement::x("c");
        assert!(row.can_contain_child(&cell));
        assert!(!row.can_contain_child(&OpenXmlElement::x("sheetData")));

        let created = row
            .try_create_valid_child(FileFormatVersions::OFFICE2007, "x", "c")
            .expect("x:c allowed in x:row");
        assert_eq!(created.local_name, "c");
        assert_eq!(created.prefix, "x");
        assert!(row
            .try_create_valid_child(FileFormatVersions::OFFICE2007, "x", "worksheet")
            .is_none());
    }

    #[test]
    fn presentation_can_contain_csld_on_slide() {
        use crate::file_format::FileFormatVersions;

        let sld = OpenXmlElement::p("sld");
        let csld = OpenXmlElement::p("cSld");
        assert!(sld.can_contain_child(&csld));
        let created = sld
            .try_create_valid_child(FileFormatVersions::OFFICE2007, "p", "cSld")
            .expect("p:cSld allowed in p:sld");
        assert_eq!(created.local_name, "cSld");
        assert_eq!(created.prefix, "p");
    }

    #[test]
    fn element_is_in_version_by_prefix() {
        use crate::file_format::FileFormatVersions;

        assert!(paragraph(vec![]).is_in_version(FileFormatVersions::OFFICE2007));
        let w14 = OpenXmlElement::new(
            "w14",
            "http://schemas.microsoft.com/office/word/2010/wordml",
            "glow",
        );
        assert!(!w14.is_in_version(FileFormatVersions::OFFICE2007));
        assert!(w14.is_in_version(FileFormatVersions::OFFICE2010));
        assert!(!OpenXmlElement::comment("c").is_in_version(FileFormatVersions::OFFICE2007));
        assert!(!OpenXmlElement::unknown("x", "y", "urn:x")
            .is_in_version(FileFormatVersions::OFFICE2007));
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

