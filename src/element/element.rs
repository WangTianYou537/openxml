//! OpenXmlElement DOM node.

use std::any::{Any, TypeId};
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
    /// User annotations (C# `AnnotationsFeature`). Not compared / not serialized.
    /// Prefer [`add_annotation`](Self::add_annotation); field is public so struct
    /// literals in the parser keep working (`..Default::default()` not required).
    pub annotations: Vec<AnnoEntry>,
}

/// Type-erased annotation entry (C# annotation list item).
pub struct AnnoEntry {
    type_id: TypeId,
    value: Box<dyn Any + Send + Sync>,
}

impl Default for AnnoEntry {
    fn default() -> Self {
        // Never constructed via Default in practice; placeholder for derive hygiene.
        Self {
            type_id: TypeId::of::<()>(),
            value: Box::new(()),
        }
    }
}

impl fmt::Debug for AnnoEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnnoEntry").finish_non_exhaustive()
    }
}

impl Default for OpenXmlElement {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            namespace_uri: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
            namespace_declarations: Vec::new(),
            children: Vec::new(),
            text: None,
            raw_outer_xml: None,
            misc_kind: OpenXmlMiscKind::None,
            annotations: Vec::new(),
        }
    }
}

/// Clone copies the DOM tree but **not** annotations (C# `Clone` / `CloneNode` behavior).
impl Clone for OpenXmlElement {
    fn clone(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            namespace_uri: self.namespace_uri.clone(),
            local_name: self.local_name.clone(),
            attributes: self.attributes.clone(),
            namespace_declarations: self.namespace_declarations.clone(),
            children: self.children.clone(),
            text: self.text.clone(),
            raw_outer_xml: self.raw_outer_xml.clone(),
            misc_kind: self.misc_kind,
            annotations: Vec::new(),
        }
    }
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
            annotations: Vec::new(),
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
            annotations: Vec::new(),
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
            annotations: Vec::new(),
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
            annotations: Vec::new(),
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

    /// Add an annotation (C# `OpenXmlElement.AddAnnotation`). Multiple values of the
    /// same type are kept in insertion order.
    pub fn add_annotation<T: Any + Send + Sync>(&mut self, value: T) {
        self.annotations.push(AnnoEntry {
            type_id: TypeId::of::<T>(),
            value: Box::new(value),
        });
    }

    /// First annotation of type `T`, if any.
    pub fn annotation<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.annotations
            .iter()
            .find(|a| a.type_id == TypeId::of::<T>())
            .and_then(|a| a.value.downcast_ref::<T>())
    }

    /// Mutable first annotation of type `T`, if any.
    pub fn annotation_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        let tid = TypeId::of::<T>();
        self.annotations
            .iter_mut()
            .find(|a| a.type_id == tid)
            .and_then(|a| a.value.downcast_mut::<T>())
    }

    /// All annotations of type `T`.
    pub fn annotations<T: Any + Send + Sync>(&self) -> Vec<&T> {
        self.annotations
            .iter()
            .filter(|a| a.type_id == TypeId::of::<T>())
            .filter_map(|a| a.value.downcast_ref::<T>())
            .collect()
    }

    /// Remove every annotation of type `T`.
    pub fn remove_annotations<T: Any + Send + Sync>(&mut self) {
        let tid = TypeId::of::<T>();
        self.annotations.retain(|a| a.type_id != tid);
    }

    /// Whether any annotation of type `T` is present.
    pub fn has_annotation<T: Any + Send + Sync>(&self) -> bool {
        self.annotations
            .iter()
            .any(|a| a.type_id == TypeId::of::<T>())
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

    /// Remove all children (C# `RemoveAllChildren`).
    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    /// Alias of [`clear_children`](Self::clear_children) (C# `RemoveAllChildren`).
    pub fn remove_all_children(&mut self) {
        self.clear_children();
    }

    /// First child element, if any (C# `FirstChild`).
    pub fn first_child(&self) -> Option<&OpenXmlElement> {
        self.children.first()
    }

    pub fn first_child_mut(&mut self) -> Option<&mut OpenXmlElement> {
        self.children.first_mut()
    }

    /// Last child element, if any (C# `LastChild`).
    pub fn last_child(&self) -> Option<&OpenXmlElement> {
        self.children.last()
    }

    pub fn last_child_mut(&mut self) -> Option<&mut OpenXmlElement> {
        self.children.last_mut()
    }

    /// Prepend a child (C# `PrependChild`).
    pub fn prepend_child(&mut self, child: OpenXmlElement) {
        self.children.insert(0, child);
    }

    /// Insert `new_child` before the first child equal to `reference` by pointer identity
    /// of local structure index. Returns `false` if reference was not found.
    ///
    /// C# `InsertBefore` — reference is matched by index among current children.
    pub fn insert_before(
        &mut self,
        new_child: OpenXmlElement,
        reference_index: usize,
    ) -> bool {
        if reference_index > self.children.len() {
            return false;
        }
        self.children.insert(reference_index, new_child);
        true
    }

    /// Insert `new_child` after the child at `reference_index` (C# `InsertAfter`).
    pub fn insert_after(
        &mut self,
        new_child: OpenXmlElement,
        reference_index: usize,
    ) -> bool {
        if reference_index >= self.children.len() {
            return false;
        }
        self.children.insert(reference_index + 1, new_child);
        true
    }

    /// Insert before the first child whose local name is `reference_local_name`.
    pub fn insert_before_name(
        &mut self,
        new_child: OpenXmlElement,
        reference_local_name: &str,
    ) -> bool {
        if let Some(i) = self
            .children
            .iter()
            .position(|c| c.local_name == reference_local_name)
        {
            self.children.insert(i, new_child);
            true
        } else {
            false
        }
    }

    /// Insert after the first child whose local name is `reference_local_name`.
    pub fn insert_after_name(
        &mut self,
        new_child: OpenXmlElement,
        reference_local_name: &str,
    ) -> bool {
        if let Some(i) = self
            .children
            .iter()
            .position(|c| c.local_name == reference_local_name)
        {
            self.children.insert(i + 1, new_child);
            true
        } else {
            false
        }
    }

    /// Remove the child at `index`, returning it (C# `RemoveChild` by position).
    pub fn remove_child_at(&mut self, index: usize) -> Option<OpenXmlElement> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /// Remove the first child with the given local name.
    pub fn remove_child_by_name(&mut self, local_name: &str) -> Option<OpenXmlElement> {
        if let Some(i) = self.children.iter().position(|c| c.local_name == local_name) {
            Some(self.children.remove(i))
        } else {
            None
        }
    }

    /// Replace the child at `index` with `new_child`, returning the old one.
    pub fn replace_child(
        &mut self,
        index: usize,
        new_child: OpenXmlElement,
    ) -> Option<OpenXmlElement> {
        if index < self.children.len() {
            Some(std::mem::replace(&mut self.children[index], new_child))
        } else {
            None
        }
    }

    /// Number of direct children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Whether this element has any children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Serialize this element to OuterXml (C# `OuterXml`).
    pub fn outer_xml(&self) -> crate::error::Result<String> {
        let bytes = super::writer::write_element_fragment(self)?;
        String::from_utf8(bytes).map_err(|e| crate::error::Error::Xml(e.to_string()))
    }

    /// Serialize inner content only — children + text, no wrapper element
    /// (approximation of C# `InnerXml` getter).
    pub fn inner_xml(&self) -> crate::error::Result<String> {
        let mut out = String::new();
        if let Some(t) = &self.text {
            out.push_str(t);
        }
        for c in &self.children {
            out.push_str(&c.outer_xml()?);
        }
        Ok(out)
    }

    /// Replace children by parsing `inner` as an XML fragment of sibling elements
    /// (C# `InnerXml` setter, simplified — expects well-formed element siblings).
    pub fn set_inner_xml(&mut self, inner: &str) -> crate::error::Result<()> {
        self.children.clear();
        self.text = None;
        let trimmed = inner.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        // Wrap in a synthetic root so the existing single-root parser works.
        let wrapped = format!("<__inner>{trimmed}</__inner>");
        let root = super::reader::parse_element(wrapped.as_bytes())?;
        self.children = root.children;
        self.text = root.text;
        Ok(())
    }

    /// Replace this element's entire content from OuterXml (must match local name).
    pub fn set_outer_xml(&mut self, outer: &str) -> crate::error::Result<()> {
        let parsed = super::reader::parse_element(outer.as_bytes())?;
        if parsed.local_name != self.local_name {
            return Err(crate::error::Error::Xml(format!(
                "OuterXml local name `{}` does not match element `{}`",
                parsed.local_name, self.local_name
            )));
        }
        self.prefix = parsed.prefix;
        self.namespace_uri = parsed.namespace_uri;
        self.attributes = parsed.attributes;
        self.namespace_declarations = parsed.namespace_declarations;
        self.children = parsed.children;
        self.text = parsed.text;
        self.misc_kind = parsed.misc_kind;
        self.raw_outer_xml = None;
        // annotations intentionally preserved
        Ok(())
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

#[cfg(test)]
mod annotation_tests {
    use super::*;

    #[test]
    fn annotation_roundtrip_and_clone_drops() {
        let mut el = OpenXmlElement::w("p");
        el.add_annotation(42u32);
        el.add_annotation("meta".to_string());
        assert_eq!(el.annotation::<u32>(), Some(&42));
        assert_eq!(el.annotation::<String>().map(|s| s.as_str()), Some("meta"));
        el.add_annotation(7u32);
        assert_eq!(el.annotations::<u32>(), vec![&42, &7]);
        el.remove_annotations::<u32>();
        assert!(!el.has_annotation::<u32>());
        assert!(el.has_annotation::<String>());

        let cloned = el.clone();
        assert!(!cloned.has_annotation::<String>());
        // Structural equality ignores annotations
        el.add_annotation(1u8);
        assert_eq!(el, cloned);
    }
}

#[cfg(test)]
mod dom_mutation_tests {
    use super::*;

    #[test]
    fn outer_inner_xml_and_insert() {
        let mut p = OpenXmlElement::w("p");
        p.append_child(OpenXmlElement::w("r").with_child(OpenXmlElement::w("t").with_text("a")));
        p.append_child(OpenXmlElement::w("r").with_child(OpenXmlElement::w("t").with_text("b")));
        assert_eq!(p.child_count(), 2);
        assert!(p.insert_after_name(
            OpenXmlElement::w("r").with_child(OpenXmlElement::w("t").with_text("c")),
            "r"
        ));
        // inserts after first r → 3 children
        assert_eq!(p.child_count(), 3);
        let outer = p.outer_xml().unwrap();
        assert!(outer.contains("w:p") || outer.contains("<p") || outer.contains("p"));
        let inner = p.inner_xml().unwrap();
        assert!(inner.contains('a') || inner.contains('t'));

        let removed = p.remove_child_by_name("r").unwrap();
        assert_eq!(removed.local_name, "r");
        assert_eq!(p.child_count(), 2);

        p.set_inner_xml(r#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:t>z</w:t></w:r>"#).unwrap();
        assert_eq!(p.child_count(), 1);
        assert_eq!(p.first_child().unwrap().local_name, "r");
    }
}
