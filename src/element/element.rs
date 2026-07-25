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

    /// Fully qualified name (namespace URI + local name) (C# `OpenXmlAttribute.QName`).
    pub fn xml_qualified_name(&self) -> OpenXmlQualifiedName {
        OpenXmlQualifiedName::new(
            self.namespace_uri.clone().unwrap_or_default(),
            self.local_name.clone(),
        )
    }

    /// Whether this attribute matches `local_name` + `namespace_uri`.
    pub fn matches(&self, local_name: &str, namespace_uri: &str) -> bool {
        self.local_name == local_name
            && self.namespace_uri.as_deref().unwrap_or("") == namespace_uri
    }

    /// C# `OpenXmlAttribute(string qualifiedName, string namespaceUri, string? value)`.
    ///
    /// `qualifiedName` may be `prefix:local` or bare `local`.
    pub fn from_qualified_name(
        qualified_name: &str,
        namespace_uri: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        let (prefix, local) = match qualified_name.split_once(':') {
            Some((p, l)) if !p.is_empty() && !l.is_empty() => (Some(p.to_string()), l.to_string()),
            _ => (None, qualified_name.to_string()),
        };
        Self {
            prefix,
            namespace_uri: {
                let ns = namespace_uri.into();
                if ns.is_empty() {
                    None
                } else {
                    Some(ns)
                }
            },
            local_name: local,
            value: value.into(),
        }
    }

    /// C# `OpenXmlAttribute(prefix, localName, namespaceUri, value)`.
    pub fn from_parts(
        prefix: impl Into<String>,
        local_name: impl Into<String>,
        namespace_uri: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        let p = prefix.into();
        let ns = namespace_uri.into();
        Self {
            prefix: if p.is_empty() { None } else { Some(p) },
            namespace_uri: if ns.is_empty() { None } else { Some(ns) },
            local_name: local_name.into(),
            value: value.into(),
        }
    }

    /// Namespace URI string (empty when unset) — C# `NamespaceUri`.
    pub fn namespace_uri_str(&self) -> &str {
        self.namespace_uri.as_deref().unwrap_or("")
    }

    /// Prefix string (empty when unset) — C# `Prefix`.
    pub fn prefix_str(&self) -> &str {
        self.prefix.as_deref().unwrap_or("")
    }
}

impl std::fmt::Display for OpenXmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.qualified_name(), self.value)
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
/// Fully qualified name: namespace URI + local name (C# `OpenXmlQualifiedName` shell).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct OpenXmlQualifiedName {
    pub namespace_uri: String,
    pub local_name: String,
}

impl OpenXmlQualifiedName {
    pub fn new(namespace_uri: impl Into<String>, local_name: impl Into<String>) -> Self {
        Self {
            namespace_uri: namespace_uri.into(),
            local_name: local_name.into(),
        }
    }

    pub fn from_element(elem: &OpenXmlElement) -> Self {
        Self {
            namespace_uri: elem.namespace_uri.clone(),
            local_name: elem.local_name.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.local_name.is_empty() && self.namespace_uri.is_empty()
    }
}

impl std::fmt::Display for OpenXmlQualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.namespace_uri.is_empty() {
            write!(f, "{}", self.local_name)
        } else {
            write!(f, "{}:{}", self.namespace_uri, self.local_name)
        }
    }
}

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

    /// Leaf-like element: ordinary element with no element children
    /// (C# `OpenXmlLeafElement` / `OpenXmlLeafTextElement` shell).
    ///
    /// Misc nodes are not leaves in this sense.
    pub fn is_leaf_element(&self) -> bool {
        !self.is_misc_node() && !self.children.iter().any(|c| !c.is_misc_node())
    }

    /// Leaf text element: leaf with text content (C# `OpenXmlLeafTextElement` shell).
    pub fn is_leaf_text_element(&self) -> bool {
        self.is_leaf_element() && self.text.is_some()
    }

    /// Composite element: has at least one element child (C# `OpenXmlCompositeElement` shell).
    pub fn is_composite_element(&self) -> bool {
        !self.is_misc_node() && self.children.iter().any(|c| !c.is_misc_node())
    }

    /// `xml:space` attribute value when present.
    pub fn xml_space(&self) -> Option<&str> {
        self.get_attribute_ns("space", "http://www.w3.org/XML/1998/namespace")
            .or_else(|| self.get_attribute_qname("xml:space"))
            .or_else(|| self.get_attribute("space"))
    }

    /// Set `xml:space` (typically `"preserve"` or `"default"`).
    pub fn set_xml_space(&mut self, value: impl Into<String>) {
        self.set_attribute_ns("xml", "http://www.w3.org/XML/1998/namespace", "space", value);
    }

    /// Whether `xml:space="preserve"` is set.
    pub fn preserves_space(&self) -> bool {
        self.xml_space() == Some("preserve")
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

    /// Namespace URI + local name pair (C# `OpenXmlQualifiedName`).
    pub fn xml_qualified_name(&self) -> OpenXmlQualifiedName {
        OpenXmlQualifiedName::from_element(self)
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
    /// Whether this element has any attributes (C# `HasAttributes`).
    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    /// Append an attribute treated as an extended/open attribute
    /// (C# `AddExtendedAttribute` — same storage as ordinary attributes in this port).
    pub fn add_extended_attribute(&mut self, attr: OpenXmlAttribute) {
        // Replace same qname if present.
        let q = attr.qualified_name();
        self.attributes.retain(|a| a.qualified_name() != q);
        self.attributes.push(attr);
    }

    /// All attributes as extended list (C# `ExtendedAttributes` enumerator shell).
    pub fn extended_attributes(&self) -> impl Iterator<Item = &OpenXmlAttribute> {
        self.attributes.iter()
    }

    /// Remove an attribute by local name (any prefix).
    pub fn remove_attribute(&mut self, local_name: &str) -> Option<OpenXmlAttribute> {
        if let Some(i) = self.attributes.iter().position(|a| a.local_name == local_name) {
            Some(self.attributes.remove(i))
        } else {
            None
        }
    }

    /// Remove an attribute by local name + namespace URI (C# `RemoveAttribute(local, ns)`).
    pub fn remove_attribute_ns(
        &mut self,
        local_name: &str,
        namespace_uri: &str,
    ) -> Option<OpenXmlAttribute> {
        if let Some(i) = self.attributes.iter().position(|a| {
            a.local_name == local_name
                && a.namespace_uri.as_deref().unwrap_or("") == namespace_uri
        }) {
            Some(self.attributes.remove(i))
        } else if namespace_uri.is_empty() {
            self.remove_attribute(local_name)
        } else {
            None
        }
    }

    /// Set an attribute from a full [`OpenXmlAttribute`] value (C# `SetAttribute`).
    pub fn set_open_xml_attribute(&mut self, attr: OpenXmlAttribute) {
        if attr.local_name.is_empty() {
            return;
        }
        let local = attr.local_name.clone();
        let ns = attr.namespace_uri.clone().unwrap_or_default();
        // Prefer ns+local match; fall back to local-only when ns empty.
        if let Some(i) = self.attributes.iter().position(|a| {
            a.local_name == local && a.namespace_uri.as_deref().unwrap_or("") == ns.as_str()
        }) {
            self.attributes[i] = attr;
        } else if ns.is_empty() {
            if let Some(i) = self.attributes.iter().position(|a| a.local_name == local) {
                self.attributes[i] = attr;
            } else {
                self.attributes.push(attr);
            }
        } else {
            self.attributes.push(attr);
        }
    }

    /// Get a full [`OpenXmlAttribute`] by local name + namespace URI
    /// (C# `GetAttribute(local, ns)` returning the attribute object).
    pub fn get_open_xml_attribute(
        &self,
        local_name: &str,
        namespace_uri: &str,
    ) -> Option<OpenXmlAttribute> {
        self.attributes
            .iter()
            .find(|a| {
                a.local_name == local_name
                    && a.namespace_uri.as_deref().unwrap_or("") == namespace_uri
            })
            .cloned()
            .or_else(|| {
                if namespace_uri.is_empty() {
                    self.attributes
                        .iter()
                        .find(|a| a.local_name == local_name)
                        .cloned()
                } else {
                    None
                }
            })
    }

    /// Markup Compatibility attribute bag for this element (C# `MCAttributes` shell).
    pub fn mc_attributes(&self) -> crate::markup_compatibility::MarkupCompatibilityAttributes {
        crate::markup_compatibility::MarkupCompatibilityAttributes::from_element(self)
    }

    /// Apply a Markup Compatibility attribute bag (C# set `MCAttributes`).
    pub fn set_mc_attributes(
        &mut self,
        attrs: &crate::markup_compatibility::MarkupCompatibilityAttributes,
    ) {
        attrs.apply_to(self);
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

    /// Whether this element has any children (C# `HasChildren`).
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Direct children slice (C# `ChildElements` collection shell).
    pub fn child_elements(&self) -> &[OpenXmlElement] {
        &self.children
    }

    /// Index of the first direct child with `local_name`, if any.
    pub fn child_index(&self, local_name: &str) -> Option<usize> {
        self.children
            .iter()
            .position(|c| c.local_name == local_name)
    }

    /// Index of a child by pointer equality of address within this parent's `children` vec
    /// is not available across clones; use index-based APIs instead.
    ///
    /// Return the child before the one at `index` (C# `PreviousSibling` from parent).
    pub fn previous_sibling_at(&self, index: usize) -> Option<&OpenXmlElement> {
        if index == 0 || index > self.children.len() {
            None
        } else {
            self.children.get(index - 1)
        }
    }

    /// Return the child after the one at `index` (C# `NextSibling` from parent).
    pub fn next_sibling_at(&self, index: usize) -> Option<&OpenXmlElement> {
        self.children.get(index + 1)
    }

    /// Mutable previous sibling of the child at `index`.
    pub fn previous_sibling_at_mut(&mut self, index: usize) -> Option<&mut OpenXmlElement> {
        if index == 0 || index > self.children.len() {
            None
        } else {
            self.children.get_mut(index - 1)
        }
    }

    /// Mutable next sibling of the child at `index`.
    pub fn next_sibling_at_mut(&mut self, index: usize) -> Option<&mut OpenXmlElement> {
        self.children.get_mut(index + 1)
    }

    /// Remove the child at `index` and return it (C# `Remove` when holding parent+index).
    pub fn remove_at(&mut self, index: usize) -> Option<OpenXmlElement> {
        self.remove_child_at(index)
    }

    /// Replace the child at `index` with `new_child` (C# `ReplaceChild` / `ReplaceWith` shell).
    pub fn replace_with_at(
        &mut self,
        index: usize,
        new_child: OpenXmlElement,
    ) -> Option<OpenXmlElement> {
        self.replace_child(index, new_child)
    }

    /// Insert `new_child` immediately before the child at `index` (C# `InsertBeforeSelf`
    /// when `index` identifies "self" among parent's children).
    pub fn insert_before_self_at(&mut self, index: usize, new_child: OpenXmlElement) -> bool {
        self.insert_before(new_child, index)
    }

    /// Insert `new_child` immediately after the child at `index` (C# `InsertAfterSelf`).
    pub fn insert_after_self_at(&mut self, index: usize, new_child: OpenXmlElement) -> bool {
        self.insert_after(new_child, index)
    }

    /// Get-or-add the first child with `local_name` (C# `GetOrAddFirstChild` shell).
    ///
    /// If missing, `factory` builds the new child which is appended.
    pub fn get_or_add_first_child_with<F>(&mut self, local_name: &str, factory: F) -> &mut OpenXmlElement
    where
        F: FnOnce() -> OpenXmlElement,
    {
        if let Some(i) = self.child_index(local_name) {
            return &mut self.children[i];
        }
        self.children.push(factory());
        let last = self.children.len() - 1;
        &mut self.children[last]
    }

    /// Depth-first path of child indices from `self` to the first descendant matching
    /// `pred`. Empty path means `self` itself matched. `None` if not found.
    pub fn find_path(&self, pred: &dyn Fn(&OpenXmlElement) -> bool) -> Option<Vec<usize>> {
        if pred(self) {
            return Some(Vec::new());
        }
        for (i, c) in self.children.iter().enumerate() {
            if let Some(mut sub) = c.find_path(pred) {
                let mut path = vec![i];
                path.append(&mut sub);
                return Some(path);
            }
        }
        None
    }

    /// Resolve a child-index path; empty path returns `self`.
    pub fn get_at_path(&self, path: &[usize]) -> Option<&OpenXmlElement> {
        let mut cur = self;
        for &i in path {
            cur = cur.children.get(i)?;
        }
        Some(cur)
    }

    pub fn get_at_path_mut(&mut self, path: &[usize]) -> Option<&mut OpenXmlElement> {
        let mut cur = self;
        for &i in path {
            cur = cur.children.get_mut(i)?;
        }
        Some(cur)
    }

    /// Remove the descendant at `path` (non-empty). Returns the removed node.
    pub fn remove_at_path(&mut self, path: &[usize]) -> Option<OpenXmlElement> {
        if path.is_empty() {
            return None;
        }
        if path.len() == 1 {
            return self.remove_child_at(path[0]);
        }
        let (last, parent_path) = path.split_last()?;
        self.get_at_path_mut(parent_path)?.remove_child_at(*last)
    }

    /// Ancestors of the node at `path`, nearest parent first (C# `Ancestors` order
    /// when navigating from a known root + child-index path). Does not include the
    /// target; empty `path` yields no ancestors.
    pub fn ancestors_along_path(&self, path: &[usize]) -> Vec<&OpenXmlElement> {
        if path.is_empty() {
            return Vec::new();
        }
        let mut chain: Vec<&OpenXmlElement> = Vec::with_capacity(path.len());
        let mut cur = self;
        // Collect root .. parent (all nodes visited before the final index).
        for &i in &path[..path.len() - 1] {
            chain.push(cur);
            match cur.children.get(i) {
                Some(child) => cur = child,
                None => return Vec::new(),
            }
        }
        chain.push(cur); // parent of target
        chain.reverse(); // nearest parent first, like C# Ancestors()
        chain
    }


    /// Serialize this element to OuterXml (C# `OuterXml`).
    pub fn outer_xml(&self) -> crate::error::Result<String> {
        let bytes = super::writer::write_element_fragment(self)?;
        String::from_utf8(bytes).map_err(|e| crate::error::Error::Xml(e.to_string()))
    }

    /// Write this element (no XML declaration) to `dest` (C# `WriteTo`).
    pub fn write_to<W: std::io::Write>(&self, dest: W) -> crate::error::Result<()> {
        super::writer::write_element_to(self, dest)
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

    /// Shallow clone: attributes and ns decls only, no children or text
    /// (C# `CloneNode(false)` shell — text is treated as content, omitted).
    pub fn clone_node_shallow(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            namespace_uri: self.namespace_uri.clone(),
            local_name: self.local_name.clone(),
            attributes: self.attributes.clone(),
            namespace_declarations: self.namespace_declarations.clone(),
            children: Vec::new(),
            text: None,
            raw_outer_xml: None,
            misc_kind: self.misc_kind,
            annotations: Vec::new(),
        }
    }

    /// All attributes (C# `GetAttributes`).
    pub fn get_attributes(&self) -> &[OpenXmlAttribute] {
        &self.attributes
    }

    /// Replace the attribute list (C# `SetAttributes`).
    pub fn set_attributes(&mut self, attrs: impl IntoIterator<Item = OpenXmlAttribute>) {
        self.attributes = attrs.into_iter().collect();
    }

    /// Remove every attribute (C# `ClearAllAttributes`).
    pub fn clear_all_attributes(&mut self) {
        self.attributes.clear();
    }

    /// Copy attributes and namespace declarations from `other` onto this element
    /// (C# `CopyAttributes` used by `CloneNode`).
    pub fn copy_attributes_from(&mut self, other: &OpenXmlElement) {
        self.attributes = other.attributes.clone();
        self.namespace_declarations = other.namespace_declarations.clone();
    }

    /// Copy only attributes (not namespace declarations).
    pub fn copy_attributes_only_from(&mut self, other: &OpenXmlElement) {
        self.attributes = other.attributes.clone();
    }

    /// Deep- or shallow-copy children from `other` (C# `CopyChildren`).
    ///
    /// When `deep` is true, each child is `clone_node()`; otherwise
    /// `clone_node_shallow()`.
    pub fn copy_children_from(&mut self, other: &OpenXmlElement, deep: bool) {
        self.children = other
            .children
            .iter()
            .map(|c| if deep { c.clone_node() } else { c.clone_node_shallow() })
            .collect();
        if deep {
            self.text = other.text.clone();
        }
    }

    /// Insert a child at `index` (C# `InsertAt`). Clamps to end if out of range.
    pub fn insert_at(&mut self, index: usize, child: OpenXmlElement) {
        let i = index.min(self.children.len());
        self.children.insert(i, child);
    }

    /// Remove all children whose local name equals `local_name`
    /// (C# `RemoveAllChildren<T>` shell by name).
    pub fn remove_all_children_named(&mut self, local_name: &str) {
        self.children.retain(|c| c.local_name != local_name);
    }

    /// Resolve a namespace URI for `prefix` from declarations on this element
    /// (C# `LookupNamespace` on the element — no parent walk in owned trees).
    pub fn lookup_namespace(&self, prefix: &str) -> Option<&str> {
        let key = if prefix == "xmlns" { "" } else { prefix };
        self.namespace_declarations
            .iter()
            .find(|(p, _)| p == key)
            .map(|(_, u)| u.as_str())
            .or_else(|| {
                if !self.prefix.is_empty() && self.prefix == prefix && !self.namespace_uri.is_empty()
                {
                    Some(self.namespace_uri.as_str())
                } else if prefix.is_empty() && !self.namespace_uri.is_empty() {
                    Some(self.namespace_uri.as_str())
                } else {
                    None
                }
            })
    }

    /// Resolve prefix for a namespace URI from this element's declarations
    /// (C# `LookupPrefix` shell).
    pub fn lookup_prefix(&self, namespace_uri: &str) -> Option<&str> {
        self.namespace_declarations
            .iter()
            .find(|(_, u)| u == namespace_uri)
            .map(|(p, _)| p.as_str())
            .or_else(|| {
                if self.namespace_uri == namespace_uri {
                    Some(self.prefix.as_str())
                } else {
                    None
                }
            })
    }

    /// Add or replace a namespace declaration (C# `AddNamespaceDeclaration`).
    pub fn add_namespace_declaration(&mut self, prefix: impl Into<String>, uri: impl Into<String>) {
        let prefix = prefix.into();
        let uri = uri.into();
        if let Some((_, existing)) = self
            .namespace_declarations
            .iter_mut()
            .find(|(p, _)| *p == prefix)
        {
            *existing = uri;
        } else {
            self.namespace_declarations.push((prefix, uri));
        }
    }

    /// Remove a namespace declaration by prefix (C# `RemoveNamespaceDeclaration`).
    pub fn remove_namespace_declaration(&mut self, prefix: &str) -> bool {
        let before = self.namespace_declarations.len();
        self.namespace_declarations.retain(|(p, _)| p != prefix);
        before != self.namespace_declarations.len()
    }

    /// Whether a namespace declaration with `prefix` exists.
    pub fn has_namespace_declaration(&self, prefix: &str) -> bool {
        self.namespace_declarations.iter().any(|(p, _)| p == prefix)
    }

    /// All namespace declarations as `(prefix, uri)` pairs.
    pub fn namespace_declarations(&self) -> &[(String, String)] {
        &self.namespace_declarations
    }

    /// Get attribute by local name + namespace URI (C# `GetAttribute(local, ns)`).
    pub fn get_attribute_ns(&self, local_name: &str, namespace_uri: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|a| {
                a.local_name == local_name
                    && a.namespace_uri.as_deref() == Some(namespace_uri)
            })
            .map(|a| a.value.as_str())
            .or_else(|| {
                // Fallback: match local only when ns empty on attr
                if namespace_uri.is_empty() {
                    self.get_attribute(local_name)
                } else {
                    None
                }
            })
    }

    /// Direct element children only (C# `Elements()` / `ChildElements` shell).
    pub fn elements(&self) -> impl Iterator<Item = &OpenXmlElement> {
        self.children.iter().filter(|c| !c.is_misc_node())
    }

    /// Element children with the given local name (C# `Elements<T>` by name).
    pub fn elements_named<'a>(
        &'a self,
        local_name: &'a str,
    ) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
        self.children
            .iter()
            .filter(move |c| !c.is_misc_node() && c.local_name == local_name)
    }

    /// First element child (skipping misc nodes) — C# `GetFirstChild` shell.
    pub fn get_first_child_element(&self) -> Option<&OpenXmlElement> {
        self.elements().next()
    }

    pub fn get_first_child_named<'a>(&'a self, local_name: &str) -> Option<&'a OpenXmlElement> {
        self.children
            .iter()
            .find(|c| !c.is_misc_node() && c.local_name == local_name)
    }

    /// Siblings before `index` among the parent's children (caller supplies parent slice).
    pub fn elements_before_in_parent<'a>(
        parent_children: &'a [OpenXmlElement],
        index: usize,
    ) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
        parent_children[..index.min(parent_children.len())]
            .iter()
            .filter(|c| !c.is_misc_node())
    }

    /// Siblings after `index` among the parent's children.
    pub fn elements_after_in_parent<'a>(
        parent_children: &'a [OpenXmlElement],
        index: usize,
    ) -> impl Iterator<Item = &'a OpenXmlElement> + 'a {
        let start = (index + 1).min(parent_children.len());
        parent_children[start..]
            .iter()
            .filter(|c| !c.is_misc_node())
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

#[cfg(test)]
mod path_nav_tests {
    use super::*;

    #[test]
    fn path_remove_and_siblings() {
        let mut body = OpenXmlElement::w("body");
        body.append_child(OpenXmlElement::w("p").with_text("1"));
        body.append_child(OpenXmlElement::w("p").with_text("2"));
        body.append_child(OpenXmlElement::w("p").with_text("3"));
        assert_eq!(body.next_sibling_at(0).and_then(|e| e.text_value()), Some("2"));
        assert_eq!(body.previous_sibling_at(2).and_then(|e| e.text_value()), Some("2"));
        assert!(body.insert_after_self_at(0, OpenXmlElement::w("p").with_text("1b")));
        assert_eq!(body.child_count(), 4);

        let path = body
            .find_path(&|e| e.text_value() == Some("2"))
            .expect("path");
        assert_eq!(path, vec![2]);
        let removed = body.remove_at_path(&path).unwrap();
        assert_eq!(removed.text_value(), Some("2"));
        assert_eq!(body.child_count(), 3);

        let p = body.get_or_add_first_child_with("sectPr", || OpenXmlElement::w("sectPr"));
        assert_eq!(p.local_name, "sectPr");
        assert_eq!(body.child_count(), 4);
        // second call returns existing
        let _ = body.get_or_add_first_child_with("sectPr", || OpenXmlElement::w("sectPr"));
        assert_eq!(body.child_count(), 4);
    }
}

#[cfg(test)]
mod element_api_parity_tests {
    use super::*;

    #[test]
    fn clone_node_shallow_and_attrs() {
        let mut el = OpenXmlElement::w("p")
            .with_attribute("rsidR", "00AB")
            .with_ns_decl("w", "http://schemas.openxmlformats.org/wordprocessingml/2006/main")
            .with_child(OpenXmlElement::w("r").with_text("hi"));
        el.set_text("ignore-on-shallow");
        let shallow = el.clone_node_shallow();
        assert!(shallow.children.is_empty());
        assert!(shallow.text.is_none());
        assert_eq!(shallow.get_attribute("rsidR"), Some("00AB"));
        assert_eq!(
            shallow.lookup_namespace("w"),
            Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
        );

        el.clear_all_attributes();
        assert!(el.get_attributes().is_empty());
        el.set_attributes(vec![OpenXmlAttribute {
            prefix: None,
            namespace_uri: None,
            local_name: "a".into(),
            value: "1".into(),
        }]);
        assert_eq!(el.get_attribute("a"), Some("1"));

        el.insert_at(0, OpenXmlElement::w("x"));
        assert_eq!(el.children[0].local_name, "x");
        el.append_child(OpenXmlElement::w("r"));
        el.append_child(OpenXmlElement::comment("c"));
        el.remove_all_children_named("r");
        assert!(el.children.iter().all(|c| c.local_name != "r" || c.is_misc_node()));
        assert_eq!(el.elements().count(), 1); // only x
        assert_eq!(el.get_first_child_named("x").map(|e| e.local_name.as_str()), Some("x"));

        el.add_namespace_declaration("r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships");
        assert!(el.has_namespace_declaration("r"));
        assert_eq!(
            el.lookup_namespace("r"),
            Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships")
        );
        assert!(el.remove_namespace_declaration("r"));
        assert!(!el.has_namespace_declaration("r"));

        assert!(el.has_attributes());
        el.clear_all_attributes();
        assert!(!el.has_attributes());
    }

    #[test]
    fn copy_attributes_from_other() {
        let src = OpenXmlElement::w("p")
            .with_attribute("rsidR", "AA")
            .with_ns_decl("w", "http://schemas.openxmlformats.org/wordprocessingml/2006/main");
        let mut dst = OpenXmlElement::w("p");
        dst.copy_attributes_from(&src);
        assert_eq!(dst.get_attribute("rsidR"), Some("AA"));
        assert_eq!(
            dst.lookup_namespace("w"),
            Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
        );
    }

    #[test]
    fn open_xml_attribute_display() {
        let a = OpenXmlAttribute::with_ns("w", "http://schemas.openxmlformats.org/wordprocessingml/2006/main", "val", "1");
        assert_eq!(a.to_string(), "w:val=\"1\"");
        assert!(a.matches("val", "http://schemas.openxmlformats.org/wordprocessingml/2006/main"));
        assert_eq!(a.xml_qualified_name().local_name, "val");
    }

    #[test]
    fn leaf_composite_and_xml_space() {
        let leaf = OpenXmlElement::w("t").with_text("hi");
        assert!(leaf.is_leaf_element());
        assert!(leaf.is_leaf_text_element());
        assert!(!leaf.is_composite_element());
        let mut p = OpenXmlElement::w("p").with_child(OpenXmlElement::w("r"));
        assert!(p.is_composite_element());
        assert!(!p.is_leaf_element());
        p.set_xml_space("preserve");
        assert!(p.preserves_space());
        assert_eq!(p.xml_space(), Some("preserve"));
    }

    #[test]
    fn attribute_ns_and_open_xml_attribute() {
        let mut el = OpenXmlElement::w("p");
        el.set_attribute_ns(
            "w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "rsidR",
            "00AB",
        );
        el.set_attribute("plain", "1");
        let full = el
            .get_open_xml_attribute(
                "rsidR",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            )
            .expect("rsidR");
        assert_eq!(full.value, "00AB");
        assert_eq!(full.prefix.as_deref(), Some("w"));

        el.set_open_xml_attribute(OpenXmlAttribute::with_ns(
            "w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "rsidR",
            "FF",
        ));
        assert_eq!(
            el.get_attribute_ns(
                "rsidR",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            ),
            Some("FF")
        );
        assert!(el
            .remove_attribute_ns(
                "rsidR",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            )
            .is_some());
        assert!(el
            .get_attribute_ns(
                "rsidR",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            )
            .is_none());
        assert_eq!(el.get_attribute("plain"), Some("1"));
    }

    #[test]
    fn mc_attributes_accessor() {
        use crate::markup_compatibility::MarkupCompatibilityAttributes;
        let mut el = OpenXmlElement::w("p");
        let bag = MarkupCompatibilityAttributes {
            ignorable: Some("w14".into()),
            ..Default::default()
        };
        el.set_mc_attributes(&bag);
        let got = el.mc_attributes();
        assert_eq!(got.ignorable.as_deref(), Some("w14"));
    }

    #[test]
    fn write_to_roundtrip() {
        let el = OpenXmlElement::w("t").with_text("hi");
        let mut buf = Vec::new();
        el.write_to(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("hi"));
        assert!(s.contains("t"));
    }

    #[test]
    fn open_xml_attribute_constructors() {
        let a = OpenXmlAttribute::from_qualified_name(
            "w:val",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "1",
        );
        assert_eq!(a.prefix_str(), "w");
        assert_eq!(a.local_name, "val");
        assert_eq!(
            a.namespace_uri_str(),
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
        );
        let b = OpenXmlAttribute::from_parts("r", "id", "http://ns", "rId1");
        assert_eq!(b.qualified_name(), "r:id");
        assert_eq!(b.value, "rId1");
    }
}
