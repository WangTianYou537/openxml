//! Custom file properties (`docProps/custom.xml`).

use crate::element::{parse_element, write_element, OpenXmlElement};
use crate::error::Result;
use crate::namespace::{content_type, ns, rel};
use crate::opc::{OpcPackage, PackUri, RelationshipTargetMode};

const CUSTOM_URI: &str = "/docProps/custom.xml";
const OP: &str = ns::CUSTOM_PROPERTIES.uri;
const VT: &str = ns::DOC_PROPS_VTYPES.uri;

/// Format ID used by Office for user-defined custom properties.
pub const CUSTOM_PROP_FMTID: &str = "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}";

/// A single custom document property (name + typed value).
#[derive(Debug, Clone, PartialEq)]
pub struct CustomProperty {
    pub name: String,
    pub pid: i32,
    pub value: CustomPropertyValue,
}

/// Supported custom property value kinds (subset of vt:*).
#[derive(Debug, Clone, PartialEq)]
pub enum CustomPropertyValue {
    Lpstr(String),
    Lpwstr(String),
    I4(i32),
    R8(f64),
    Bool(bool),
    FileTime(String),
}

impl CustomPropertyValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Lpstr(s) | Self::Lpwstr(s) | Self::FileTime(s) => Some(s),
            _ => None,
        }
    }
}

/// Collection of custom document properties.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CustomProperties {
    pub properties: Vec<CustomProperty>,
}

impl CustomProperties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str) -> Option<&CustomProperty> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Number of custom properties.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Whether there are no custom properties.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Property names in declaration order.
    pub fn names(&self) -> Vec<&str> {
        self.properties.iter().map(|p| p.name.as_str()).collect()
    }

    /// Remove a property by name. Returns whether it was present.
    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.properties.len();
        self.properties.retain(|p| p.name != name);
        self.properties.len() < before
    }

    /// Remove all properties.
    pub fn clear(&mut self) {
        self.properties.clear();
    }

    /// Insert or replace a string custom property.
    pub fn set_string(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = CustomPropertyValue::Lpstr(value.into());
        if let Some(existing) = self.properties.iter_mut().find(|p| p.name == name) {
            existing.value = value;
        } else {
            let pid = self.next_pid();
            self.properties.push(CustomProperty {
                name,
                pid,
                value,
            });
        }
    }

    /// Insert or replace an integer custom property.
    pub fn set_i4(&mut self, name: impl Into<String>, value: i32) {
        let name = name.into();
        let value = CustomPropertyValue::I4(value);
        if let Some(existing) = self.properties.iter_mut().find(|p| p.name == name) {
            existing.value = value;
        } else {
            let pid = self.next_pid();
            self.properties.push(CustomProperty {
                name,
                pid,
                value,
            });
        }
    }

    /// Insert or replace a boolean custom property.
    pub fn set_bool(&mut self, name: impl Into<String>, value: bool) {
        let name = name.into();
        let value = CustomPropertyValue::Bool(value);
        if let Some(existing) = self.properties.iter_mut().find(|p| p.name == name) {
            existing.value = value;
        } else {
            let pid = self.next_pid();
            self.properties.push(CustomProperty {
                name,
                pid,
                value,
            });
        }
    }

    fn next_pid(&self) -> i32 {
        self.properties
            .iter()
            .map(|p| p.pid)
            .max()
            .unwrap_or(1)
            .max(1)
            + 1
    }

    pub fn load_from(package: &OpcPackage) -> Result<Self> {
        let uri = PackUri::new(CUSTOM_URI);
        let Some(data) = package.get_part(&uri) else {
            return Ok(Self::default());
        };
        let root = parse_element(data)?;
        Ok(Self::from_element(&root))
    }

    pub fn save_to(&self, package: &mut OpcPackage) -> Result<()> {
        let xml = write_element(&self.to_element())?;
        let uri = PackUri::new(CUSTOM_URI);
        package.set_part(uri.clone(), content_type::CUSTOM_PROPERTIES, xml);
        let has_rel = package
            .package_relationships()
            .get_by_type(rel::CUSTOM_PROPERTIES)
            .is_some();
        if !has_rel {
            package.add_package_relationship(
                rel::CUSTOM_PROPERTIES,
                &uri,
                RelationshipTargetMode::Internal,
            );
        }
        Ok(())
    }

    pub fn from_element(root: &OpenXmlElement) -> Self {
        let mut properties = Vec::new();
        for child in &root.children {
            if child.local_name != "property" {
                continue;
            }
            let name = child
                .get_attribute_qname("name")
                .or_else(|| child.get_attribute("name"))
                .unwrap_or_default()
                .to_string();
            let pid = child
                .get_attribute_qname("pid")
                .or_else(|| child.get_attribute("pid"))
                .and_then(|s| s.parse().ok())
                .unwrap_or(2);
            let value = child
                .children
                .first()
                .map(value_from_vt)
                .unwrap_or(CustomPropertyValue::Lpstr(String::new()));
            properties.push(CustomProperty { name, pid, value });
        }
        Self { properties }
    }

    pub fn to_element(&self) -> OpenXmlElement {
        let mut root = OpenXmlElement::new("op", OP, "Properties")
            .with_ns_decl("op", OP)
            .with_ns_decl("vt", VT);
        for prop in &self.properties {
            let mut el = OpenXmlElement::new("op", OP, "property")
                .with_attribute("fmtid", CUSTOM_PROP_FMTID)
                .with_attribute("pid", prop.pid.to_string())
                .with_attribute("name", &prop.name);
            el.append_child(value_to_vt(&prop.value));
            root.append_child(el);
        }
        root
    }
}

fn value_from_vt(el: &OpenXmlElement) -> CustomPropertyValue {
    let text = el.inner_text();
    match el.local_name.as_str() {
        "lpstr" => CustomPropertyValue::Lpstr(text),
        "lpwstr" | "bstr" => CustomPropertyValue::Lpwstr(text),
        "i4" | "int" => CustomPropertyValue::I4(text.parse().unwrap_or(0)),
        "r8" | "r4" => CustomPropertyValue::R8(text.parse().unwrap_or(0.0)),
        "bool" => CustomPropertyValue::Bool(matches!(text.as_str(), "true" | "1" | "True")),
        "filetime" | "date" => CustomPropertyValue::FileTime(text),
        _ => CustomPropertyValue::Lpstr(text),
    }
}

fn value_to_vt(value: &CustomPropertyValue) -> OpenXmlElement {
    match value {
        CustomPropertyValue::Lpstr(s) => OpenXmlElement::new("vt", VT, "lpstr").with_text(s),
        CustomPropertyValue::Lpwstr(s) => OpenXmlElement::new("vt", VT, "lpwstr").with_text(s),
        CustomPropertyValue::I4(n) => {
            OpenXmlElement::new("vt", VT, "i4").with_text(n.to_string())
        }
        CustomPropertyValue::R8(n) => {
            OpenXmlElement::new("vt", VT, "r8").with_text(n.to_string())
        }
        CustomPropertyValue::Bool(b) => OpenXmlElement::new("vt", VT, "bool")
            .with_text(if *b { "true" } else { "false" }),
        CustomPropertyValue::FileTime(s) => {
            OpenXmlElement::new("vt", VT, "filetime").with_text(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_properties_roundtrip() {
        let mut props = CustomProperties::new();
        props.set_string("Project", "Alpha");
        props.set_i4("Count", 42);
        props.set_bool("Done", false);
        let el = props.to_element();
        let xml = write_element(&el).unwrap();
        let parsed = parse_element(&xml).unwrap();
        let back = CustomProperties::from_element(&parsed);
        assert_eq!(
            back.get("Project").and_then(|p| p.value.as_str()),
            Some("Alpha")
        );
        assert!(matches!(
            back.get("Count").map(|p| &p.value),
            Some(CustomPropertyValue::I4(42))
        ));
        assert!(matches!(
            back.get("Done").map(|p| &p.value),
            Some(CustomPropertyValue::Bool(false))
        ));
    }
}
