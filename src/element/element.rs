//! OpenXmlElement DOM node.

use std::fmt;
use std::sync::Arc;

/// A qualified attribute on an Open XML element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenXmlAttribute {
    pub prefix: Option<String>,
    pub namespace_uri: Option<String>,
    pub local_name: String,
    pub value: String,
}

impl OpenXmlAttribute {
    pub fn new(
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            prefix: None,
            namespace_uri: None,
            local_name: local_name.into(),
            value: value.into(),
        }
    }

    pub fn with_ns(
        prefix: impl Into<String>,
        namespace_uri: impl Into<String>,
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            prefix: Some(prefix.into()),
            namespace_uri: Some(namespace_uri.into()),
            local_name: local_name.into(),
            value: value.into(),
        }
    }

    pub fn qualified_name(&self) -> String {
        match &self.prefix {
            Some(p) if !p.is_empty() => format!("{}:{}", p, self.local_name),
            _ => self.local_name.clone(),
        }
    }
}

/// Kind of non-element DOM node (mirrors C# `OpenXmlMiscNode` / `XmlNodeType`).
///
/// Regular elements use [`OpenXmlMiscKind::None`]. Misc nodes are stored in the
/// same tree as elements (C# also subclasses `OpenXmlElement`) so existing
/// traversal APIs keep working; writers emit them as comments / PIs / CDATA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OpenXmlMiscKind {
    /// Ordinary element node.
    #[default]
    None,
    /// XML comment (`<!-- … -->`).
    Comment,
    /// Processing instruction (`<?target data?>`).
    ProcessingInstruction,
    /// CDATA section (stored separately from ordinary text).
    CData,
}

/// An Open XML DOM element.
///
/// This is a general-purpose element node. Strongly-typed wrappers
/// (e.g. `wordprocessing::Paragraph`) build on top of this.
///
/// Non-element nodes (comments, PIs, CDATA) use [`OpenXmlMiscKind`] and
/// well-known `local_name` values (`#comment`, `#pi`, `#cdata-section`).
#[derive(Clone)]
pub struct OpenXmlElement {
    /// Namespace prefix (e.g. `"w"`).
    pub prefix: String,
    /// Namespace URI.
    pub namespace_uri: String,
    /// Local name (e.g. `"p"`). For misc nodes: `#comment`, `#pi`, `#cdata-section`.
    pub local_name: String,
    /// Element attributes (excluding xmlns declarations stored separately).
    pub attributes: Vec<OpenXmlAttribute>,
    /// Namespace declarations declared on this element (`prefix` → `uri`).
    /// Empty prefix means default xmlns.
    pub namespace_declarations: Vec<(String, String)>,
    /// Child elements.
    pub children: Vec<OpenXmlElement>,
    /// Text content for leaf-text elements (e.g. `w:t`), or misc node payload.
    pub text: Option<String>,
    /// Raw outer XML when the element was not fully parsed (optional).
    pub raw_outer_xml: Option<Arc<str>>,
    /// Non-element node kind (comment / PI / CDATA). Default: ordinary element.
    pub misc_kind: OpenXmlMiscKind,
}

impl fmt::Debug for OpenXmlElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenXmlElement")
            .field("name", &self.qualified_name())
            .field("attrs", &self.attributes.len())
            .field("children", &self.children.len())
            .field("text", &self.text)
            .finish()
    }
}

impl OpenXmlElement {
    pub fn new(
        prefix: impl Into<String>,
        namespace_uri: impl Into<String>,
        local_name: impl Into<String>,
    ) -> Self {
        Self {
            prefix: prefix.into(),
            namespace_uri: namespace_uri.into(),
            local_name: local_name.into(),
            attributes: Vec::new(),
            namespace_declarations: Vec::new(),
            children: Vec::new(),
            text: None,
            raw_outer_xml: None,
            misc_kind: OpenXmlMiscKind::None,
        }
    }

    /// XML comment node (`<!-- text -->`). C# `OpenXmlMiscNode(XmlNodeType.Comment)`.
    pub fn comment(text: impl Into<String>) -> Self {
        Self {
            prefix: String::new(),
            namespace_uri: String::new(),
            local_name: "#comment".into(),
            attributes: Vec::new(),
            namespace_declarations: Vec::new(),
            children: Vec::new(),
            text: Some(text.into()),
            raw_outer_xml: None,
            misc_kind: OpenXmlMiscKind::Comment,
        }
    }

    /// Processing instruction (`<?target data?>`).
    pub fn processing_instruction(target: impl Into<String>, data: impl Into<String>) -> Self {
        let target = target.into();
        let data = data.into();
        Self {
            prefix: String::new(),
            namespace_uri: String::new(),
            local_name: "#pi".into(),
            attributes: vec![OpenXmlAttribute::new("target", target)],
            namespace_declarations: Vec::new(),
            children: Vec::new(),
            text: Some(data),
            raw_outer_xml: None,
            misc_kind: OpenXmlMiscKind::ProcessingInstruction,
        }
    }

    /// CDATA section node.
    pub fn cdata(text: impl Into<String>) -> Self {
        Self {
            prefix: String::new(),
            namespace_uri: String::new(),
            local_name: "#cdata-section".into(),
            attributes: Vec::new(),
            namespace_declarations: Vec::new(),
            children: Vec::new(),
            text: Some(text.into()),
            raw_outer_xml: None,
            misc_kind: OpenXmlMiscKind::CData,
        }
    }

    /// Whether this node is a non-element misc node (comment / PI / CDATA).
    pub fn is_misc_node(&self) -> bool {
        self.misc_kind != OpenXmlMiscKind::None
    }

    pub fn misc_kind(&self) -> OpenXmlMiscKind {
        self.misc_kind
    }

    /// PI target, if this is a processing instruction.
    pub fn pi_target(&self) -> Option<&str> {
        if self.misc_kind != OpenXmlMiscKind::ProcessingInstruction {
            return None;
        }
        self.attributes
            .iter()
            .find(|a| a.local_name == "target")
            .map(|a| a.value.as_str())
    }

    /// Create an element in the WordprocessingML namespace.
    pub fn w(local_name: impl Into<String>) -> Self {
        Self::new(
            "w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            local_name,
        )
    }

    /// Create an element in the SpreadsheetML namespace.
    pub fn x(local_name: impl Into<String>) -> Self {
        Self::new(
            "x",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            local_name,
        )
    }

    /// Create an element in the PresentationML namespace.
    pub fn p(local_name: impl Into<String>) -> Self {
        Self::new(
            "p",
            "http://schemas.openxmlformats.org/presentationml/2006/main",
            local_name,
        )
    }

    /// Create an element in the DrawingML main namespace.
    pub fn a(local_name: impl Into<String>) -> Self {
        Self::new(
            "a",
            "http://schemas.openxmlformats.org/drawingml/2006/main",
            local_name,
        )
    }

    pub fn qualified_name(&self) -> String {
        if self.prefix.is_empty() {
            self.local_name.clone()
        } else {
            format!("{}:{}", self.prefix, self.local_name)
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = Some(text.into());
    }

    pub fn text_value(&self) -> Option<&str> {
        self.text.as_deref()
    }

    pub fn with_attr(mut self, attr: OpenXmlAttribute) -> Self {
        self.attributes.push(attr);
        self
    }

    pub fn set_attribute(
        &mut self,
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) {
        let local_name = local_name.into();
        let value = value.into();
        if let Some(a) = self
            .attributes
            .iter_mut()
            .find(|a| a.local_name == local_name && a.prefix.is_none())
        {
            a.value = value;
        } else {
            self.attributes.push(OpenXmlAttribute::new(local_name, value));
        }
    }

    pub fn set_attribute_ns(
        &mut self,
        prefix: impl Into<String>,
        namespace_uri: impl Into<String>,
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) {
        let prefix = prefix.into();
        let local_name = local_name.into();
        let value = value.into();
        let namespace_uri = namespace_uri.into();
        if let Some(a) = self.attributes.iter_mut().find(|a| {
            a.local_name == local_name && a.prefix.as_deref() == Some(prefix.as_str())
        }) {
            a.value = value;
        } else {
            self.attributes.push(OpenXmlAttribute::with_ns(
                prefix,
                namespace_uri,
                local_name,
                value,
            ));
        }
    }

    pub fn get_attribute(&self, local_name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|a| a.local_name == local_name)
            .map(|a| a.value.as_str())
    }

    /// Get an attribute by qualified name (`"w:val"` or `"val"`).
    pub fn get_attribute_qname(&self, qname: &str) -> Option<&str> {
        if let Some((prefix, local)) = qname.split_once(':') {
            self.attributes
                .iter()
                .find(|a| a.local_name == local && a.prefix.as_deref() == Some(prefix))
                .map(|a| a.value.as_str())
        } else {
            self.get_attribute(qname)
        }
    }

    /// Set an attribute by qualified name (`"w:val"` or `"val"`).
    pub fn set_attribute_qname(&mut self, qname: &str, value: impl Into<String>) {
        let value = value.into();
        if let Some((prefix, local)) = qname.split_once(':') {
            // Namespace URI left empty unless known; prefix is enough for serialization.
            self.set_attribute_ns(prefix, "", local, value);
        } else {
            self.set_attribute(qname, value);
        }
    }

    /// Set a typed simple-type attribute by local name.
    pub fn set_simple_attribute<T: crate::simple_types::OpenXmlSimpleType>(
        &mut self,
        local_name: impl Into<String>,
        value: T,
    ) {
        self.set_attribute(local_name, value.as_inner_text());
    }

    /// Get a typed simple-type attribute by local name.
    pub fn get_simple_attribute<T: crate::simple_types::OpenXmlSimpleType>(
        &self,
        local_name: &str,
    ) -> Option<T> {
        self.get_attribute(local_name)
            .and_then(T::from_inner_text)
    }

    /// Set a typed simple-type attribute by qualified name (`"w:val"`).
    pub fn set_simple_attribute_qname<T: crate::simple_types::OpenXmlSimpleType>(
        &mut self,
        qname: &str,
        value: T,
    ) {
        self.set_attribute_qname(qname, value.as_inner_text());
    }

    /// Get a typed simple-type attribute by qualified name.
    pub fn get_simple_attribute_qname<T: crate::simple_types::OpenXmlSimpleType>(
        &self,
        qname: &str,
    ) -> Option<T> {
        self.get_attribute_qname(qname)
            .and_then(T::from_inner_text)
    }

    /// Remove an attribute by local name (any prefix).
    pub fn remove_attribute(&mut self, local_name: &str) -> Option<OpenXmlAttribute> {
        if let Some(i) = self.attributes.iter().position(|a| a.local_name == local_name) {
            Some(self.attributes.remove(i))
        } else {
            None
        }
    }

    /// Builder-style attribute set by local name.
    pub fn with_attribute(
        mut self,
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.set_attribute(local_name, value);
        self
    }

    /// Builder-style attribute set by qualified name.
    pub fn with_attribute_qname(mut self, qname: &str, value: impl Into<String>) -> Self {
        self.set_attribute_qname(qname, value);
        self
    }

    /// Builder-style namespaced attribute set.
    pub fn with_attribute_ns(
        mut self,
        prefix: impl Into<String>,
        namespace_uri: impl Into<String>,
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.set_attribute_ns(prefix, namespace_uri, local_name, value);
        self
    }

    pub fn with_ns_decl(mut self, prefix: impl Into<String>, uri: impl Into<String>) -> Self {
        self.namespace_declarations
            .push((prefix.into(), uri.into()));
        self
    }

    pub fn with_child(mut self, child: OpenXmlElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = OpenXmlElement>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn append_child(&mut self, child: OpenXmlElement) {
        self.children.push(child);
    }

    pub fn append_children(&mut self, children: impl IntoIterator<Item = OpenXmlElement>) {
        self.children.extend(children);
    }

    /// Remove all children.
    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    /// First child with the given local name.
    pub fn child(&self, local_name: &str) -> Option<&OpenXmlElement> {
        self.children.iter().find(|c| c.local_name == local_name)
    }

    pub fn child_mut(&mut self, local_name: &str) -> Option<&mut OpenXmlElement> {
        self.children.iter_mut().find(|c| c.local_name == local_name)
    }

    /// All children with the given local name.
    pub fn children_by_name<'a>(
        &'a self,
        local_name: &'a str,
    ) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
        self.children
            .iter()
            .filter(move |c| c.local_name == local_name)
    }

    /// Depth-first iterator over this element and all descendants.
    pub fn descendants(&self) -> Descendants<'_> {
        Descendants {
            stack: vec![self.children.iter()],
        }
    }

    /// Collect all text content from this element and descendants (concatenated).
    pub fn inner_text(&self) -> String {
        let mut out = String::new();
        self.collect_text(&mut out);
        out
    }

    fn collect_text(&self, out: &mut String) {
        if let Some(t) = &self.text {
            out.push_str(t);
        }
        for c in &self.children {
            c.collect_text(out);
        }
    }

    /// Clone this node deeply.
    pub fn clone_node(&self) -> Self {
        self.clone()
    }
}

/// Iterator over descendants (children, grandchildren, …).
pub struct Descendants<'a> {
    stack: Vec<std::slice::Iter<'a, OpenXmlElement>>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = &'a OpenXmlElement;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iter = self.stack.last_mut()?;
            if let Some(elem) = iter.next() {
                if !elem.children.is_empty() {
                    // Push children for later; return current first
                    // We need to process children after yielding current.
                    // Standard approach: yield elem, then push its children.
                    self.stack.push(elem.children.iter());
                }
                return Some(elem);
            }
            self.stack.pop();
        }
    }
}

impl PartialEq for OpenXmlElement {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix
            && self.namespace_uri == other.namespace_uri
            && self.local_name == other.local_name
            && self.attributes == other.attributes
            && self.namespace_declarations == other.namespace_declarations
            && self.children == other.children
            && self.text == other.text
            && self.misc_kind == other.misc_kind
    }
}

impl Eq for OpenXmlElement {}
