//! Extended file properties (`docProps/app.xml`).

use crate::element::{parse_element, write_element, OpenXmlElement};
use crate::error::Result;
use crate::namespace::{content_type, ns, rel};
use crate::opc::{OpcPackage, PackUri, RelationshipTargetMode};

const APP_URI: &str = "/docProps/app.xml";
const EP: &str = ns::EXTENDED_PROPERTIES.uri;

/// Application-level document properties (App.xml).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtendedProperties {
    pub application: Option<String>,
    pub application_version: Option<String>,
    pub company: Option<String>,
    pub manager: Option<String>,
    pub template: Option<String>,
    pub hyperlink_base: Option<String>,
    pub pages: Option<i32>,
    pub words: Option<i32>,
    pub characters: Option<i32>,
    pub characters_with_spaces: Option<i32>,
    pub lines: Option<i32>,
    pub paragraphs: Option<i32>,
    pub slides: Option<i32>,
    pub notes: Option<i32>,
    pub total_time: Option<i32>,
    pub hidden_slides: Option<i32>,
    pub mm_clips: Option<i32>,
    pub doc_security: Option<i32>,
    pub presentation_format: Option<String>,
    pub scale_crop: Option<bool>,
    pub links_up_to_date: Option<bool>,
    pub shared_doc: Option<bool>,
    pub hyperlinks_changed: Option<bool>,
}

impl ExtendedProperties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from(package: &OpcPackage) -> Result<Self> {
        let uri = PackUri::new(APP_URI);
        let Some(data) = package.get_part(&uri) else {
            return Ok(Self::default());
        };
        let root = parse_element(data)?;
        Ok(Self::from_element(&root))
    }

    pub fn save_to(&self, package: &mut OpcPackage) -> Result<()> {
        let xml = write_element(&self.to_element())?;
        let uri = PackUri::new(APP_URI);
        package.set_part(uri.clone(), content_type::EXTENDED_PROPERTIES, xml);
        let has_rel = package
            .package_relationships()
            .get_by_type(rel::EXTENDED_PROPERTIES)
            .is_some();
        if !has_rel {
            package.add_package_relationship(
                rel::EXTENDED_PROPERTIES,
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
            match child.local_name.as_str() {
                "Application" => props.application = Some(text),
                "AppVersion" => props.application_version = Some(text),
                "Company" => props.company = Some(text),
                "Manager" => props.manager = Some(text),
                "Template" => props.template = Some(text),
                "HyperlinkBase" => props.hyperlink_base = Some(text),
                "PresentationFormat" => props.presentation_format = Some(text),
                "Pages" => props.pages = text.parse().ok(),
                "Words" => props.words = text.parse().ok(),
                "Characters" => props.characters = text.parse().ok(),
                "CharactersWithSpaces" => props.characters_with_spaces = text.parse().ok(),
                "Lines" => props.lines = text.parse().ok(),
                "Paragraphs" => props.paragraphs = text.parse().ok(),
                "Slides" => props.slides = text.parse().ok(),
                "Notes" => props.notes = text.parse().ok(),
                "TotalTime" => props.total_time = text.parse().ok(),
                "HiddenSlides" => props.hidden_slides = text.parse().ok(),
                "MMClips" => props.mm_clips = text.parse().ok(),
                "DocSecurity" => props.doc_security = text.parse().ok(),
                "ScaleCrop" => props.scale_crop = parse_bool(&text),
                "LinksUpToDate" => props.links_up_to_date = parse_bool(&text),
                "SharedDoc" => props.shared_doc = parse_bool(&text),
                "HyperlinksChanged" => props.hyperlinks_changed = parse_bool(&text),
                _ => {}
            }
        }
        props
    }

    pub fn to_element(&self) -> OpenXmlElement {
        // Office writes app.xml with a default namespace (no prefix) + vt for vectors.
        const VT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";
        let mut root = OpenXmlElement::new("", EP, "Properties")
            .with_ns_decl("", EP)
            .with_ns_decl("vt", VT);

        fn text_el(name: &str, value: &str) -> OpenXmlElement {
            OpenXmlElement::new("", EP, name).with_text(value)
        }
        fn int_el(name: &str, value: i32) -> OpenXmlElement {
            text_el(name, &value.to_string())
        }
        fn bool_el(name: &str, value: bool) -> OpenXmlElement {
            text_el(name, if value { "true" } else { "false" })
        }

        if let Some(v) = &self.application {
            root.append_child(text_el("Application", v));
        }
        if let Some(v) = &self.application_version {
            root.append_child(text_el("AppVersion", v));
        }
        if let Some(v) = &self.company {
            root.append_child(text_el("Company", v));
        }
        if let Some(v) = &self.manager {
            root.append_child(text_el("Manager", v));
        }
        if let Some(v) = &self.template {
            root.append_child(text_el("Template", v));
        }
        if let Some(v) = &self.hyperlink_base {
            root.append_child(text_el("HyperlinkBase", v));
        }
        if let Some(v) = &self.presentation_format {
            root.append_child(text_el("PresentationFormat", v));
        }
        if let Some(v) = self.pages {
            root.append_child(int_el("Pages", v));
        }
        if let Some(v) = self.words {
            root.append_child(int_el("Words", v));
        }
        if let Some(v) = self.characters {
            root.append_child(int_el("Characters", v));
        }
        if let Some(v) = self.characters_with_spaces {
            root.append_child(int_el("CharactersWithSpaces", v));
        }
        if let Some(v) = self.lines {
            root.append_child(int_el("Lines", v));
        }
        if let Some(v) = self.paragraphs {
            root.append_child(int_el("Paragraphs", v));
        }
        if let Some(v) = self.slides {
            root.append_child(int_el("Slides", v));
        }
        if let Some(v) = self.notes {
            root.append_child(int_el("Notes", v));
        }
        if let Some(v) = self.total_time {
            root.append_child(int_el("TotalTime", v));
        }
        if let Some(v) = self.hidden_slides {
            root.append_child(int_el("HiddenSlides", v));
        }
        if let Some(v) = self.mm_clips {
            root.append_child(int_el("MMClips", v));
        }
        if let Some(v) = self.doc_security {
            root.append_child(int_el("DocSecurity", v));
        }
        if let Some(v) = self.scale_crop {
            root.append_child(bool_el("ScaleCrop", v));
        }
        if let Some(v) = self.links_up_to_date {
            root.append_child(bool_el("LinksUpToDate", v));
        }
        if let Some(v) = self.shared_doc {
            root.append_child(bool_el("SharedDoc", v));
        }
        if let Some(v) = self.hyperlinks_changed {
            root.append_child(bool_el("HyperlinksChanged", v));
        }
        root
    }
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.trim() {
        "true" | "1" | "True" => Some(true),
        "false" | "0" | "False" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extended_properties_roundtrip() {
        let mut props = ExtendedProperties::new();
        props.application = Some("openxml-rs".into());
        props.company = Some("Acme".into());
        props.pages = Some(3);
        let el = props.to_element();
        let xml = write_element(&el).unwrap();
        let parsed = parse_element(&xml).unwrap();
        let back = ExtendedProperties::from_element(&parsed);
        assert_eq!(back.application.as_deref(), Some("openxml-rs"));
        assert_eq!(back.company.as_deref(), Some("Acme"));
        assert_eq!(back.pages, Some(3));
    }
}
