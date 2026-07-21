//! OPC package core properties (`docProps/core.xml`).

use crate::element::{parse_element, write_element, OpenXmlElement};
use crate::error::Result;
use crate::namespace::{content_type, ns, rel};
use crate::opc::{OpcPackage, PackUri, RelationshipTargetMode};

const CORE_URI: &str = "/docProps/core.xml";

/// Dublin Core-style package properties.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageProperties {
    pub title: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub last_modified_by: Option<String>,
    pub revision: Option<String>,
    pub category: Option<String>,
    pub content_status: Option<String>,
    pub language: Option<String>,
    pub version: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

impl PackageProperties {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load from an open OPC package, if the core properties part exists.
    pub fn load_from(package: &OpcPackage) -> Result<Self> {
        let uri = PackUri::new(CORE_URI);
        let Some(data) = package.get_part(&uri) else {
            return Ok(Self::default());
        };
        let root = parse_element(data)?;
        Ok(Self::from_element(&root))
    }

    /// Write core properties into the package (creates part + relationship).
    pub fn save_to(&self, package: &mut OpcPackage) -> Result<()> {
        let xml = write_element(&self.to_element())?;
        let uri = PackUri::new(CORE_URI);
        package.set_part(uri.clone(), content_type::CORE_PROPERTIES, xml);

        // Ensure package relationship exists
        let has_rel = package
            .package_relationships()
            .get_by_type(rel::CORE_PROPERTIES)
            .is_some();
        if !has_rel {
            package.add_package_relationship(
                rel::CORE_PROPERTIES,
                &uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok(())
    }

    pub fn from_element(root: &OpenXmlElement) -> Self {
        let mut props = Self::default();
        for child in &root.children {
            let text = child.inner_text();
            if text.is_empty() && child.text.is_none() {
                // still allow empty assignment for presence
            }
            match (child.prefix.as_str(), child.local_name.as_str()) {
                ("dc", "title") => props.title = Some(text),
                ("dc", "subject") => props.subject = Some(text),
                ("dc", "creator") => props.creator = Some(text),
                ("dc", "description") => props.description = Some(text),
                ("dc", "language") => props.language = Some(text),
                ("cp", "keywords") => props.keywords = Some(text),
                ("cp", "lastModifiedBy") => props.last_modified_by = Some(text),
                ("cp", "revision") => props.revision = Some(text),
                ("cp", "category") => props.category = Some(text),
                ("cp", "contentStatus") => props.content_status = Some(text),
                ("cp", "version") => props.version = Some(text),
                ("dcterms", "created") => props.created = Some(text),
                ("dcterms", "modified") => props.modified = Some(text),
                _ => {}
            }
        }
        props
    }

    pub fn to_element(&self) -> OpenXmlElement {
        let mut root = OpenXmlElement::new("cp", ns::CORE_PROPERTIES.uri, "coreProperties")
            .with_ns_decl("cp", ns::CORE_PROPERTIES.uri)
            .with_ns_decl("dc", ns::DC.uri)
            .with_ns_decl("dcterms", ns::DCTERMS.uri)
            .with_ns_decl("dcmitype", ns::DCMITYPE.uri)
            .with_ns_decl("xsi", ns::XSI.uri);

        fn text_el(prefix: &str, uri: &str, name: &str, value: &str) -> OpenXmlElement {
            OpenXmlElement::new(prefix, uri, name).with_text(value)
        }

        if let Some(v) = &self.title {
            root.append_child(text_el("dc", ns::DC.uri, "title", v));
        }
        if let Some(v) = &self.subject {
            root.append_child(text_el("dc", ns::DC.uri, "subject", v));
        }
        if let Some(v) = &self.creator {
            root.append_child(text_el("dc", ns::DC.uri, "creator", v));
        }
        if let Some(v) = &self.keywords {
            root.append_child(text_el("cp", ns::CORE_PROPERTIES.uri, "keywords", v));
        }
        if let Some(v) = &self.description {
            root.append_child(text_el("dc", ns::DC.uri, "description", v));
        }
        if let Some(v) = &self.last_modified_by {
            root.append_child(text_el(
                "cp",
                ns::CORE_PROPERTIES.uri,
                "lastModifiedBy",
                v,
            ));
        }
        if let Some(v) = &self.revision {
            root.append_child(text_el("cp", ns::CORE_PROPERTIES.uri, "revision", v));
        }
        if let Some(v) = &self.category {
            root.append_child(text_el("cp", ns::CORE_PROPERTIES.uri, "category", v));
        }
        if let Some(v) = &self.content_status {
            root.append_child(text_el("cp", ns::CORE_PROPERTIES.uri, "contentStatus", v));
        }
        if let Some(v) = &self.language {
            root.append_child(text_el("dc", ns::DC.uri, "language", v));
        }
        if let Some(v) = &self.version {
            root.append_child(text_el("cp", ns::CORE_PROPERTIES.uri, "version", v));
        }
        if let Some(v) = &self.created {
            let mut el = text_el("dcterms", ns::DCTERMS.uri, "created", v);
            el.set_attribute_ns("xsi", ns::XSI.uri, "type", "dcterms:W3CDTF");
            root.append_child(el);
        }
        if let Some(v) = &self.modified {
            let mut el = text_el("dcterms", ns::DCTERMS.uri, "modified", v);
            el.set_attribute_ns("xsi", ns::XSI.uri, "type", "dcterms:W3CDTF");
            root.append_child(el);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn properties_roundtrip() {
        let mut props = PackageProperties::new();
        props.title = Some("Hello".into());
        props.creator = Some("Rust".into());
        let el = props.to_element();
        let xml = write_element(&el).unwrap();
        let parsed = parse_element(&xml).unwrap();
        let back = PackageProperties::from_element(&parsed);
        assert_eq!(back.title.as_deref(), Some("Hello"));
        assert_eq!(back.creator.as_deref(), Some("Rust"));
    }
}
