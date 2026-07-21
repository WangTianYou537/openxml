//! Codegen: read Open-XML-SDK `data/schemas/*.json` and emit Rust modules.
//!
//! Usage:
//!   cargo run --bin openxml-codegen -- \
//!     --data /opt/wp/Open-XML-SDK/data \
//!     --out src/generated
//!
//! Default schemas: word/excel/ppt/drawing 2006 main.
//! Use `--schema all` for every schema JSON.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut data_dir = PathBuf::from("/opt/wp/Open-XML-SDK/data");
    let mut out_dir = PathBuf::from("src/generated");
    let mut schema_filter = String::from(
        "wordprocessingml_2006_main,spreadsheetml_2006_main,presentationml_2006_main,drawingml_2006_main",
    );

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--data" => {
                i += 1;
                data_dir = PathBuf::from(&args[i]);
            }
            "--out" => {
                i += 1;
                out_dir = PathBuf::from(&args[i]);
            }
            "--schema" => {
                i += 1;
                schema_filter = args[i].clone();
            }
            "-h" | "--help" => {
                eprintln!(
                    "Usage: openxml-codegen [--data DIR] [--out DIR] [--schema FILTER[,FILTER…]|all]"
                );
                return;
            }
            other => {
                eprintln!("unknown arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    fs::create_dir_all(&out_dir).expect("create out dir");

    let namespaces = load_namespaces(&data_dir.join("namespaces.json"));
    let schemas_dir = data_dir.join("schemas");
    let filters: Vec<&str> = if schema_filter == "all" {
        vec!["all"]
    } else {
        schema_filter
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let mut schema_modules: BTreeSet<String> = BTreeSet::new();

    for entry in fs::read_dir(&schemas_dir).expect("schemas dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let matched = filters.iter().any(|f| *f == "all" || stem.contains(f));
        if !matched {
            continue;
        }
        eprintln!("generating from {stem} …");
        let module_name = sanitize_module_name(&stem);
        let rust = generate_schema_module(&path, &namespaces, &module_name);
        let out_path = out_dir.join(format!("{module_name}.rs"));
        fs::write(&out_path, rust).expect("write module");
        eprintln!("  wrote {}", out_path.display());
        schema_modules.insert(module_name);
    }

    // Keep previously generated schema modules still on disk.
    if let Ok(dir) = fs::read_dir(&out_dir) {
        for entry in dir.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            if matches!(stem.as_str(), "mod" | "parts" | "namespaces") {
                continue;
            }
            schema_modules.insert(stem);
        }
    }

    let parts_rust = generate_parts_module(&data_dir.join("parts"));
    fs::write(out_dir.join("parts.rs"), parts_rust).expect("write parts");
    let ns_rust = generate_namespaces_module(&namespaces);
    fs::write(out_dir.join("namespaces.rs"), ns_rust).expect("write namespaces");

    let mut mod_rs = String::from(
        "//! Auto-generated from Open-XML-SDK `data/`. Do not edit by hand.\n//!\n//! Regenerate with:\n//! ```sh\n//! cargo run --bin openxml-codegen\n//! ```\n\n",
    );
    for m in &schema_modules {
        mod_rs.push_str(&format!("pub mod {m};\n"));
    }
    mod_rs.push_str("pub mod parts;\n");
    mod_rs.push_str("pub mod namespaces;\n");
    fs::write(out_dir.join("mod.rs"), mod_rs).expect("write mod.rs");

    eprintln!(
        "done: {} schema module(s) + parts + namespaces → {}",
        schema_modules.len(),
        out_dir.display()
    );
}

#[derive(Debug, Clone)]
struct NsEntry {
    prefix: String,
    uri: String,
    version: Option<String>,
}

fn load_namespaces(path: &Path) -> Vec<NsEntry> {
    let raw = fs::read_to_string(path).expect("namespaces.json");
    let value: serde_json::Value = serde_json::from_str(&raw).expect("parse namespaces");
    let mut out = Vec::new();
    for item in value.as_array().unwrap() {
        let prefix = item
            .get("Prefix")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let uri = item
            .get("Uri")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if uri.is_empty() {
            continue;
        }
        let version = item
            .get("Version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        out.push(NsEntry {
            prefix,
            uri,
            version,
        });
    }
    out
}

fn generate_namespaces_module(namespaces: &[NsEntry]) -> String {
    let mut s = String::from(
        "//! Namespace prefix ↔ URI table generated from `data/namespaces.json`.\n\n",
    );
    s.push_str("use crate::file_format::FileFormatVersions;\n\n");
    s.push_str("/// A known Open XML namespace.\n");
    s.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    s.push_str("pub struct KnownNamespace {\n");
    s.push_str("    pub prefix: &'static str,\n");
    s.push_str("    pub uri: &'static str,\n");
    s.push_str("    /// Office version that introduced this namespace, if known.\n");
    s.push_str("    pub version: Option<FileFormatVersions>,\n");
    s.push_str("}\n\n");
    s.push_str("/// All namespaces from the Open XML SDK data set.\n");
    s.push_str("pub static NAMESPACES: &[KnownNamespace] = &[\n");
    for ns in namespaces {
        let ver = match &ns.version {
            Some(v) => match version_const(v) {
                Some(c) => format!("Some(FileFormatVersions::{c})"),
                None => "None".into(),
            },
            None => "None".into(),
        };
        s.push_str(&format!(
            "    KnownNamespace {{ prefix: \"{}\", uri: \"{}\", version: {ver} }},\n",
            escape(&ns.prefix),
            escape(&ns.uri)
        ));
    }
    s.push_str("];\n\n");
    s.push_str("/// Look up a namespace URI by its conventional prefix.\n");
    s.push_str("pub fn uri_for_prefix(prefix: &str) -> Option<&'static str> {\n");
    s.push_str("    NAMESPACES.iter().find(|n| n.prefix == prefix).map(|n| n.uri)\n");
    s.push_str("}\n\n");
    s.push_str("/// Look up the conventional prefix for a namespace URI.\n");
    s.push_str("pub fn prefix_for_uri(uri: &str) -> Option<&'static str> {\n");
    s.push_str("    NAMESPACES.iter().find(|n| n.uri == uri).map(|n| n.prefix)\n");
    s.push_str("}\n\n");
    s.push_str("/// Look up the introduction version for a namespace prefix.\n");
    s.push_str(
        "pub fn version_for_prefix(prefix: &str) -> Option<FileFormatVersions> {\n",
    );
    s.push_str("    NAMESPACES.iter().find(|n| n.prefix == prefix).and_then(|n| n.version)\n");
    s.push_str("}\n\n");
    s.push_str("/// Prefix → introduction version for every versioned namespace in the data set.\n");
    s.push_str("pub static PREFIX_INTRODUCED_IN: &[(&str, FileFormatVersions)] = &[\n");
    // ECMA-376 core prefixes without Version field → Office2007
    let core = [
        "w", "r", "a", "p", "x", "c", "xdr", "wp", "m", "mc", "cp", "dc", "dcterms", "xsi",
    ];
    let mut emitted = BTreeSet::new();
    for p in core {
        s.push_str(&format!(
            "    (\"{p}\", FileFormatVersions::OFFICE2007),\n"
        ));
        emitted.insert(p.to_string());
    }
    for ns in namespaces {
        if ns.prefix.is_empty() || emitted.contains(&ns.prefix) {
            continue;
        }
        if let Some(v) = &ns.version {
            let Some(const_name) = version_const(v) else {
                continue;
            };
            s.push_str(&format!(
                "    (\"{}\", FileFormatVersions::{const_name}),\n",
                escape(&ns.prefix)
            ));
            emitted.insert(ns.prefix.clone());
        }
    }
    s.push_str("];\n");
    s
}

fn version_const(v: &str) -> Option<&'static str> {
    match v {
        "Office2007" => Some("OFFICE2007"),
        "Office2010" => Some("OFFICE2010"),
        "Office2013" => Some("OFFICE2013"),
        "Office2016" => Some("OFFICE2016"),
        "Office2019" => Some("OFFICE2019"),
        "Office2021" => Some("OFFICE2021"),
        "Microsoft365" => Some("MICROSOFT365"),
        _ => None,
    }
}

#[derive(Debug)]
struct SchemaType {
    class_name: String,
    name: String,
    base_class: String,
    is_abstract: bool,
    #[allow(dead_code)]
    is_derived: bool,
    is_leaf: bool,
    is_leaf_text: bool,
    attributes: Vec<Attr>,
    children: Vec<Child>,
}

#[derive(Debug, Clone)]
struct Attr {
    qname: String,
    property_name: Option<String>,
    type_name: String,
}

#[derive(Debug, Clone)]
struct Child {
    name: String,
    property_name: Option<String>,
}

fn generate_schema_module(path: &Path, namespaces: &[NsEntry], module_name: &str) -> String {
    let raw = fs::read_to_string(path).expect("read schema");
    let value: serde_json::Value = serde_json::from_str(&raw).expect("parse schema");
    let target_ns = value
        .get("TargetNamespace")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let prefix = namespaces
        .iter()
        .find(|n| n.uri == target_ns)
        .map(|n| n.prefix.as_str())
        .unwrap_or("w");

    let mut types = Vec::new();
    for t in value
        .get("Types")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
    {
        let class_name = t
            .get("ClassName")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if class_name.is_empty() {
            continue;
        }
        let name = t
            .get("Name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let base_class = t
            .get("BaseClass")
            .and_then(|v| v.as_str())
            .unwrap_or("OpenXmlElement")
            .to_string();
        let is_abstract = t
            .get("IsAbstract")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let is_derived = t
            .get("IsDerived")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let is_leaf = t
            .get("IsLeafElement")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let is_leaf_text = t
            .get("IsLeafText")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let mut attributes = Vec::new();
        if let Some(attrs) = t.get("Attributes").and_then(|v| v.as_array()) {
            for a in attrs {
                attributes.push(Attr {
                    qname: a
                        .get("QName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    property_name: a
                        .get("PropertyName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    type_name: a
                        .get("Type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("StringValue")
                        .to_string(),
                });
            }
        }

        let mut children = Vec::new();
        if let Some(ch) = t.get("Children").and_then(|v| v.as_array()) {
            for c in ch {
                children.push(Child {
                    name: c
                        .get("Name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    property_name: c
                        .get("PropertyName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                });
            }
        }

        types.push(SchemaType {
            class_name,
            name,
            base_class,
            is_abstract,
            is_derived,
            is_leaf,
            is_leaf_text,
            attributes,
            children,
        });
    }

    let by_class: BTreeMap<String, &SchemaType> =
        types.iter().map(|t| (t.class_name.clone(), t)).collect();

    let mut s = String::new();
    s.push_str(&format!(
        "//! Auto-generated from `{}`.\n//! Target namespace: `{target_ns}` (prefix `{prefix}`).\n\n",
        path.file_name().unwrap().to_string_lossy()
    ));
    s.push_str("use crate::element::OpenXmlElement;\n\n");
    s.push_str("/// Target namespace URI for this schema module.\n");
    s.push_str(&format!(
        "pub const NAMESPACE_URI: &str = \"{}\";\n",
        escape(&target_ns)
    ));
    s.push_str("/// Conventional prefix for this schema module.\n");
    s.push_str(&format!(
        "pub const NAMESPACE_PREFIX: &str = \"{}\";\n\n",
        escape(prefix)
    ));

    s.push_str("/// Metadata for a schema element.\n");
    s.push_str("#[derive(Debug, Clone, Copy)]\n");
    s.push_str("pub struct ElementInfo {\n");
    s.push_str("    pub class_name: &'static str,\n");
    s.push_str("    pub local_name: &'static str,\n");
    s.push_str("    pub prefix: &'static str,\n");
    s.push_str("    pub namespace_uri: &'static str,\n");
    s.push_str("    pub is_leaf: bool,\n");
    s.push_str("    pub is_leaf_text: bool,\n");
    s.push_str("    pub attributes: &'static [AttributeInfo],\n");
    s.push_str("    pub children: &'static [ChildInfo],\n");
    s.push_str("}\n\n");

    s.push_str("/// Schema attribute metadata.\n");
    s.push_str("#[derive(Debug, Clone, Copy)]\n");
    s.push_str("pub struct AttributeInfo {\n");
    s.push_str("    pub qname: &'static str,\n");
    s.push_str("    pub property_name: Option<&'static str>,\n");
    s.push_str("    pub type_name: &'static str,\n");
    s.push_str("}\n\n");

    s.push_str("/// Schema child-element metadata.\n");
    s.push_str("#[derive(Debug, Clone, Copy)]\n");
    s.push_str("pub struct ChildInfo {\n");
    s.push_str("    pub name: &'static str,\n");
    s.push_str("    pub property_name: Option<&'static str>,\n");
    s.push_str("}\n\n");

    let mut concrete_elements = Vec::new();
    for t in &types {
        if t.is_abstract {
            continue;
        }
        if let Some((pfx, local)) = parse_element_qname(&t.name) {
            if local.is_empty() {
                continue;
            }
            concrete_elements.push((t, pfx, local));
        }
    }

    fn collect_attrs<'a>(
        class: &str,
        by_class: &BTreeMap<String, &'a SchemaType>,
        out: &mut Vec<&'a Attr>,
        seen: &mut BTreeSet<String>,
    ) {
        if let Some(t) = by_class.get(class) {
            if t.base_class != "OpenXmlElement"
                && t.base_class != "OpenXmlLeafElement"
                && t.base_class != "OpenXmlLeafTextElement"
                && t.base_class != "OpenXmlCompositeElement"
            {
                collect_attrs(&t.base_class, by_class, out, seen);
            }
            for a in &t.attributes {
                if seen.insert(a.qname.clone()) {
                    out.push(a);
                }
            }
        }
    }

    fn collect_children<'a>(
        class: &str,
        by_class: &BTreeMap<String, &'a SchemaType>,
        out: &mut Vec<&'a Child>,
        seen: &mut BTreeSet<String>,
    ) {
        if let Some(t) = by_class.get(class) {
            if t.base_class != "OpenXmlElement"
                && t.base_class != "OpenXmlLeafElement"
                && t.base_class != "OpenXmlLeafTextElement"
                && t.base_class != "OpenXmlCompositeElement"
            {
                collect_children(&t.base_class, by_class, out, seen);
            }
            for c in &t.children {
                if seen.insert(c.name.clone()) {
                    out.push(c);
                }
            }
        }
    }

    for (t, _, _) in &concrete_elements {
        let mut attrs = Vec::new();
        let mut seen = BTreeSet::new();
        collect_attrs(&t.class_name, &by_class, &mut attrs, &mut seen);
        if !attrs.is_empty() {
            let table = format!("ATTRS_{}", to_snake(&t.class_name).to_ascii_uppercase());
            s.push_str(&format!("static {table}: &[AttributeInfo] = &[\n"));
            for a in &attrs {
                let prop = match &a.property_name {
                    Some(p) => format!("Some(\"{}\")", escape(p)),
                    None => "None".into(),
                };
                s.push_str(&format!(
                    "    AttributeInfo {{ qname: \"{}\", property_name: {prop}, type_name: \"{}\" }},\n",
                    escape(&a.qname),
                    escape(&simplify_type_name(&a.type_name)),
                ));
            }
            s.push_str("];\n");
        }

        let mut children = Vec::new();
        let mut seen_c = BTreeSet::new();
        collect_children(&t.class_name, &by_class, &mut children, &mut seen_c);
        if !children.is_empty() {
            let table = format!(
                "CHILDREN_{}",
                to_snake(&t.class_name).to_ascii_uppercase()
            );
            s.push_str(&format!("static {table}: &[ChildInfo] = &[\n"));
            for c in &children {
                let prop = match &c.property_name {
                    Some(p) => format!("Some(\"{}\")", escape(p)),
                    None => "None".into(),
                };
                s.push_str(&format!(
                    "    ChildInfo {{ name: \"{}\", property_name: {prop} }},\n",
                    escape(&c.name),
                ));
            }
            s.push_str("];\n");
        }
    }
    s.push('\n');

    s.push_str("/// All concrete elements in this schema.\n");
    s.push_str("pub static ELEMENTS: &[ElementInfo] = &[\n");
    for (t, pfx, local) in &concrete_elements {
        let is_leaf = t.is_leaf || is_leaf_base(&t.base_class, &by_class);
        let is_leaf_text = t.is_leaf_text || is_leaf_text_base(&t.base_class, &by_class);

        let mut attrs = Vec::new();
        let mut seen = BTreeSet::new();
        collect_attrs(&t.class_name, &by_class, &mut attrs, &mut seen);
        let attr_ref = if attrs.is_empty() {
            "&[]".to_string()
        } else {
            format!("ATTRS_{}", to_snake(&t.class_name).to_ascii_uppercase())
        };

        let mut children = Vec::new();
        let mut seen_c = BTreeSet::new();
        collect_children(&t.class_name, &by_class, &mut children, &mut seen_c);
        let child_ref = if children.is_empty() {
            "&[]".to_string()
        } else {
            format!(
                "CHILDREN_{}",
                to_snake(&t.class_name).to_ascii_uppercase()
            )
        };

        s.push_str(&format!(
            "    ElementInfo {{ class_name: \"{}\", local_name: \"{}\", prefix: \"{}\", namespace_uri: NAMESPACE_URI, is_leaf: {}, is_leaf_text: {}, attributes: {attr_ref}, children: {child_ref} }},\n",
            escape(&t.class_name),
            escape(local),
            escape(if pfx.is_empty() { prefix } else { pfx }),
            is_leaf,
            is_leaf_text,
        ));
    }
    s.push_str("];\n\n");

    s.push_str("/// Look up element metadata by class name.\n");
    s.push_str("pub fn info_by_class(class_name: &str) -> Option<&'static ElementInfo> {\n");
    s.push_str("    ELEMENTS.iter().find(|e| e.class_name == class_name)\n");
    s.push_str("}\n\n");

    s.push_str("/// Look up element metadata by local name (first match).\n");
    s.push_str("pub fn info_by_local_name(local_name: &str) -> Option<&'static ElementInfo> {\n");
    s.push_str("    ELEMENTS.iter().find(|e| e.local_name == local_name)\n");
    s.push_str("}\n\n");

    s.push_str("/// Create an empty element by its schema class name (e.g. `\"Paragraph\"`).\n");
    s.push_str("pub fn create(class_name: &str) -> Option<OpenXmlElement> {\n");
    s.push_str("    let info = info_by_class(class_name)?;\n");
    s.push_str(
        "    Some(OpenXmlElement::new(info.prefix, info.namespace_uri, info.local_name))\n",
    );
    s.push_str("}\n\n");

    s.push_str("// ---------------------------------------------------------------------------\n");
    s.push_str("// Typed constructors\n");
    s.push_str("// ---------------------------------------------------------------------------\n\n");

    let mut seen_fn = BTreeSet::new();
    for (t, pfx, local) in &concrete_elements {
        let fn_name = to_snake(&t.class_name);
        if !seen_fn.insert(fn_name.clone()) {
            continue;
        }
        let pfx = if pfx.is_empty() { prefix } else { pfx };
        let is_leaf_text = t.is_leaf_text || is_leaf_text_base(&t.base_class, &by_class);
        let is_leaf = t.is_leaf || is_leaf_base(&t.base_class, &by_class);

        s.push_str(&format!(
            "/// Create a `<{}:{}>` element (`{}`).\n",
            pfx, local, t.class_name
        ));

        if is_leaf_text {
            s.push_str(&format!(
                "pub fn {fn_name}(value: impl Into<String>) -> OpenXmlElement {{\n"
            ));
            s.push_str(&format!(
                "    OpenXmlElement::new(\"{}\", NAMESPACE_URI, \"{}\").with_text(value)\n",
                escape(pfx),
                escape(local)
            ));
            s.push_str("}\n\n");
        } else if is_leaf {
            s.push_str(&format!("pub fn {fn_name}() -> OpenXmlElement {{\n"));
            s.push_str(&format!(
                "    OpenXmlElement::new(\"{}\", NAMESPACE_URI, \"{}\")\n",
                escape(pfx),
                escape(local)
            ));
            s.push_str("}\n\n");
        } else {
            s.push_str(&format!(
                "pub fn {fn_name}(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {{\n"
            ));
            s.push_str(&format!(
                "    OpenXmlElement::new(\"{}\", NAMESPACE_URI, \"{}\").with_children(children)\n",
                escape(pfx),
                escape(local)
            ));
            s.push_str("}\n\n");
        }

        // Attribute setter helpers
        let mut attrs = Vec::new();
        let mut seen_a = BTreeSet::new();
        collect_attrs(&t.class_name, &by_class, &mut attrs, &mut seen_a);
        let mut emitted_props = BTreeSet::new();
        let named_attr_count = attrs
            .iter()
            .filter(|a| a.property_name.as_ref().map(|p| !p.is_empty()).unwrap_or(false))
            .count();

        for a in &attrs {
            let Some(prop) = &a.property_name else {
                continue;
            };
            if prop.is_empty() {
                continue;
            }
            let prop_snake = to_snake(prop);
            if !emitted_props.insert(prop_snake.clone()) {
                continue;
            }
            let helper = format!("{fn_name}_with_{prop_snake}");
            if !seen_fn.insert(helper.clone()) {
                continue;
            }
            let qname = if a.qname.starts_with(':') {
                a.qname.trim_start_matches(':').to_string()
            } else {
                a.qname.clone()
            };
            s.push_str(&format!(
                "/// Set `{prop}` (`{}`) on a `{}` element.\n",
                escape(&a.qname),
                t.class_name
            ));
            s.push_str(&format!(
                "pub fn {helper}(mut el: OpenXmlElement, value: impl Into<String>) -> OpenXmlElement {{\n"
            ));
            s.push_str(&format!(
                "    el.set_attribute_qname(\"{}\", value);\n",
                escape(&qname)
            ));
            s.push_str("    el\n}\n\n");

            // Convenience constructor for single-attr leaves: bold_val(...)
            if is_leaf && !is_leaf_text && named_attr_count == 1 {
                let ctor = format!("{fn_name}_{prop_snake}");
                if seen_fn.insert(ctor.clone()) {
                    s.push_str(&format!(
                        "/// Create `<{}:{}>` with `{prop}` set.\n",
                        pfx, local
                    ));
                    s.push_str(&format!(
                        "pub fn {ctor}(value: impl Into<String>) -> OpenXmlElement {{\n"
                    ));
                    s.push_str(&format!("    {helper}({fn_name}(), value)\n"));
                    s.push_str("}\n\n");
                }
            }
        }
    }

    if by_class.contains_key("Document") {
        s.push_str("/// Create a `w:document` root with the namespace declaration.\n");
        s.push_str(
            "pub fn document_root(children: impl IntoIterator<Item = OpenXmlElement>) -> OpenXmlElement {\n",
        );
        s.push_str(
            "    OpenXmlElement::new(NAMESPACE_PREFIX, NAMESPACE_URI, \"document\")\n",
        );
        s.push_str("        .with_ns_decl(NAMESPACE_PREFIX, NAMESPACE_URI)\n");
        s.push_str("        .with_children(children)\n");
        s.push_str("}\n\n");
    }

    // Particles (content models)
    s.push_str("// ---------------------------------------------------------------------------\n");
    s.push_str("// Schema particles (content models)\n");
    s.push_str("// ---------------------------------------------------------------------------\n\n");
    s.push_str("use crate::validation::{Occurs, Particle};\n\n");

    let mut particle_count = 0usize;
    let mut particle_fns: Vec<String> = Vec::new();
    // Re-read types with particles from original JSON value
    if let Some(types_arr) = value.get("Types").and_then(|v| v.as_array()) {
        let mut seen = BTreeSet::new();
        for t in types_arr {
            let class_name = t
                .get("ClassName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if class_name.is_empty() || !seen.insert(class_name.to_string()) {
                continue;
            }
            let Some(particle) = t.get("Particle") else {
                continue;
            };
            let Some(expr) = emit_particle_expr(particle, 2) else {
                continue;
            };
            let fn_name = format!("particle_{}", to_snake(class_name));
            if particle_fns.iter().any(|f| f == &fn_name) {
                continue;
            }
            s.push_str(&format!(
                "/// Content model particle for `{class_name}`.\n"
            ));
            s.push_str(&format!("pub fn {fn_name}() -> Particle {{\n"));
            s.push_str(&format!("    {expr}\n"));
            s.push_str("}\n\n");
            particle_fns.push(fn_name);
            particle_count += 1;
        }
    }

    s.push_str("/// Look up a content-model particle by schema class name.\n");
    s.push_str("pub fn particle_for_class(class_name: &str) -> Option<Particle> {\n");
    s.push_str("    match class_name {\n");
    // Rebuild match arms from types again
    if let Some(types_arr) = value.get("Types").and_then(|v| v.as_array()) {
        let mut seen = BTreeSet::new();
        for t in types_arr {
            let class_name = t
                .get("ClassName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if class_name.is_empty() || !seen.insert(class_name.to_string()) {
                continue;
            }
            if t.get("Particle").is_none() {
                continue;
            }
            let fn_name = format!("particle_{}", to_snake(class_name));
            if !particle_fns.iter().any(|f| f == &fn_name) {
                continue;
            }
            s.push_str(&format!(
                "        \"{}\" => Some({fn_name}()),\n",
                escape(class_name)
            ));
        }
    }
    s.push_str("        _ => None,\n");
    s.push_str("    }\n}\n\n");

    // Enums
    s.push_str("// ---------------------------------------------------------------------------\n");
    s.push_str("// Schema enums\n");
    s.push_str("// ---------------------------------------------------------------------------\n\n");

    let mut enum_count = 0usize;
    if let Some(enums) = value.get("Enums").and_then(|v| v.as_array()) {
        let mut seen_enum_names = BTreeSet::new();
        for en in enums {
            let name = en
                .get("Name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if name.is_empty() || !seen_enum_names.insert(name.clone()) {
                continue;
            }
            let Some(facets) = en.get("Facets").and_then(|v| v.as_array()) else {
                continue;
            };
            if facets.is_empty() {
                continue;
            }

            let rust_name = sanitize_type_name(&name);
            let summary = en.get("Summary").and_then(|v| v.as_str()).unwrap_or("");
            if !summary.is_empty() {
                s.push_str(&format!("/// {summary}\n"));
            } else {
                s.push_str(&format!("/// Schema enum `{name}`.\n"));
            }
            s.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
            s.push_str(&format!("pub enum {rust_name} {{\n"));

            let mut variants: Vec<(String, String)> = Vec::new();
            let mut seen_variants = BTreeSet::new();
            for f in facets {
                let value = f.get("Value").and_then(|v| v.as_str()).unwrap_or("");
                if value.is_empty() {
                    continue;
                }
                let mut variant = f
                    .get("Name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| facet_to_variant(value));
                variant = sanitize_type_name(&variant);
                if variant.is_empty() {
                    variant = format!("Value{}", variants.len());
                }
                if variant
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                {
                    variant = format!("N{variant}");
                }
                if is_rust_keyword(&variant.to_ascii_lowercase()) {
                    variant = format!("{variant}_");
                }
                let mut final_name = variant.clone();
                let mut n = 2;
                while !seen_variants.insert(final_name.clone()) {
                    final_name = format!("{variant}{n}");
                    n += 1;
                }
                variants.push((final_name, value.to_string()));
            }
            if variants.is_empty() {
                s.push_str("    #[doc(hidden)]\n    _Empty,\n}\n\n");
                continue;
            }
            for (var, _) in &variants {
                s.push_str(&format!("    {var},\n"));
            }
            s.push_str("}\n\n");

            s.push_str(&format!("impl {rust_name} {{\n"));
            s.push_str("    /// Schema string value.\n");
            s.push_str("    pub fn as_str(self) -> &'static str {\n");
            s.push_str("        match self {\n");
            for (var, val) in &variants {
                s.push_str(&format!(
                    "            Self::{var} => \"{}\",\n",
                    escape(val)
                ));
            }
            s.push_str("        }\n    }\n\n");
            s.push_str("    /// Parse from the schema string value.\n");
            s.push_str("    pub fn from_str(s: &str) -> Option<Self> {\n");
            s.push_str("        match s {\n");
            for (var, val) in &variants {
                s.push_str(&format!(
                    "            \"{}\" => Some(Self::{var}),\n",
                    escape(val)
                ));
            }
            s.push_str("            _ => None,\n");
            s.push_str("        }\n    }\n}\n\n");

            s.push_str(&format!("impl core::fmt::Display for {rust_name} {{\n"));
            s.push_str(
                "    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {\n",
            );
            s.push_str("        f.write_str(self.as_str())\n");
            s.push_str("    }\n}\n\n");

            s.push_str(&format!(
                "impl crate::simple_types::OpenXmlSimpleType for {rust_name} {{\n"
            ));
            s.push_str("    fn as_inner_text(&self) -> String {\n");
            s.push_str("        self.as_str().to_string()\n");
            s.push_str("    }\n");
            s.push_str("    fn from_inner_text(text: &str) -> Option<Self> {\n");
            s.push_str("        Self::from_str(text)\n");
            s.push_str("    }\n}\n\n");

            enum_count += 1;
        }
    }

    s.push_str("/// Number of schema types in the source JSON (including abstract).\n");
    s.push_str(&format!("pub const TYPE_COUNT: usize = {};\n", types.len()));
    s.push_str("/// Number of concrete elements with a local name.\n");
    s.push_str(&format!(
        "pub const ELEMENT_COUNT: usize = {};\n",
        concrete_elements.len()
    ));
    s.push_str("/// Number of generated enums.\n");
    s.push_str(&format!("pub const ENUM_COUNT: usize = {enum_count};\n"));
    s.push_str("/// Number of generated content-model particles.\n");
    s.push_str(&format!(
        "pub const PARTICLE_COUNT: usize = {particle_count};\n"
    ));

    let _ = module_name;
    s
}

/// Emit a Rust expression constructing a [`Particle`] from schema JSON.
fn emit_particle_expr(value: &serde_json::Value, indent: usize) -> Option<String> {
    let pad = " ".repeat(indent);
    let pad2 = " ".repeat(indent + 4);

    // Element form: { "Name": "w:CT_Body/w:body", "Occurs": [...] }
    if let Some(name) = value.get("Name").and_then(|v| v.as_str()) {
        let local = local_from_schema_name(name);
        let occurs = emit_occurs(value.get("Occurs"));
        return Some(format!(
            "Particle::element(\"{local}\", {occurs})"
        ));
    }

    let kind = value.get("Kind").and_then(|v| v.as_str())?;
    let occurs = emit_occurs(value.get("Occurs"));
    let items: Vec<String> = value
        .get("Items")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .filter_map(|it| emit_particle_expr(it, indent + 4))
        .collect();

    let items_expr = if items.is_empty() {
        "vec![]".to_string()
    } else {
        let mut s = String::from("vec![\n");
        for it in &items {
            s.push_str(&format!("{pad2}{it},\n"));
        }
        s.push_str(&format!("{pad}]"));
        s
    };

    let ctor = match kind {
        "Sequence" => "sequence",
        "Choice" => "choice",
        "Group" => "group",
        "All" => "all",
        "Any" | "any" => {
            return Some(format!("Particle::any({occurs})"));
        }
        _ => "group",
    };
    Some(format!("Particle::{ctor}({items_expr}, {occurs})"))
}

fn emit_occurs(occurs: Option<&serde_json::Value>) -> String {
    let Some(arr) = occurs.and_then(|v| v.as_array()) else {
        return "Occurs::STAR".into();
    };
    let Some(first) = arr.first() else {
        return "Occurs::STAR".into();
    };
    let min = first.get("Min").and_then(|v| v.as_u64()).unwrap_or(0);
    let max = first.get("Max").and_then(|v| v.as_u64());
    match (min, max) {
        (0, None) => "Occurs::STAR".into(),
        (1, None) => "Occurs::PLUS".into(),
        (0, Some(1)) => "Occurs::OPTIONAL".into(),
        (1, Some(1)) => "Occurs::ONE".into(),
        (min, Some(max)) => format!("Occurs::new({min}, Some({max}))"),
        (min, None) => format!("Occurs::new({min}, None)"),
    }
}

fn local_from_schema_name(name: &str) -> String {
    let elem = name.split('/').nth(1).unwrap_or(name);
    elem.rsplit(':').next().unwrap_or(elem).to_string()
}

fn is_leaf_base(base: &str, by_class: &BTreeMap<String, &SchemaType>) -> bool {
    if matches!(base, "OpenXmlLeafElement" | "OpenXmlLeafTextElement") {
        return true;
    }
    if let Some(t) = by_class.get(base) {
        if t.is_leaf || t.is_leaf_text {
            return true;
        }
        return is_leaf_base(&t.base_class, by_class);
    }
    false
}

fn is_leaf_text_base(base: &str, by_class: &BTreeMap<String, &SchemaType>) -> bool {
    if base == "OpenXmlLeafTextElement" {
        return true;
    }
    if let Some(t) = by_class.get(base) {
        if t.is_leaf_text {
            return true;
        }
        return is_leaf_text_base(&t.base_class, by_class);
    }
    false
}

fn parse_element_qname(name: &str) -> Option<(String, String)> {
    let elem = name.split('/').nth(1)?;
    if elem.is_empty() {
        return None;
    }
    if let Some((pfx, local)) = elem.split_once(':') {
        Some((pfx.to_string(), local.to_string()))
    } else {
        Some((String::new(), elem.to_string()))
    }
}

fn generate_parts_module(parts_dir: &Path) -> String {
    let mut s = String::from("//! Auto-generated part metadata from `data/parts/*.json`.\n\n");
    s.push_str("/// Metadata for an Open XML part type.\n");
    s.push_str("#[derive(Debug, Clone, Copy)]\n");
    s.push_str("pub struct PartInfo {\n");
    s.push_str("    pub name: &'static str,\n");
    s.push_str("    pub relationship_type: &'static str,\n");
    s.push_str("    pub content_type: Option<&'static str>,\n");
    s.push_str("    pub target: &'static str,\n");
    s.push_str("    pub root_element: Option<&'static str>,\n");
    s.push_str("    pub path_general: &'static str,\n");
    s.push_str("    pub children: &'static [PartChildConstraint],\n");
    s.push_str("}\n\n");

    s.push_str("/// Constraint describing a child part that may be related from a parent part.\n");
    s.push_str("#[derive(Debug, Clone, Copy)]\n");
    s.push_str("pub struct PartChildConstraint {\n");
    s.push_str("    pub name: &'static str,\n");
    s.push_str("    pub api_name: &'static str,\n");
    s.push_str("    pub max_occurs_greater_than_one: bool,\n");
    s.push_str("    pub min_occurs_non_zero: bool,\n");
    s.push_str("    pub has_fixed_content: bool,\n");
    s.push_str("    pub is_data_part_reference: bool,\n");
    s.push_str("}\n\n");

    // First pass: emit child tables
    let mut files: Vec<PathBuf> = fs::read_dir(parts_dir)
        .expect("parts dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();
    files.sort();

    #[derive(Clone)]
    struct PartRec {
        name: String,
        rel: String,
        content_type: Option<String>,
        target: String,
        root_element: Option<String>,
        path_general: String,
        children: Vec<ChildConstraint>,
    }
    #[derive(Clone)]
    struct ChildConstraint {
        name: String,
        api_name: String,
        max_gt_one: bool,
        min_non_zero: bool,
        has_fixed: bool,
        is_data_ref: bool,
    }

    let mut parts: Vec<PartRec> = Vec::new();
    for path in &files {
        let raw = match fs::read_to_string(path) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let value: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let Some(rel) = value.get("RelationshipType").and_then(|v| v.as_str()) else {
            continue;
        };
        let name = value
            .get("Name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            continue;
        }
        let content_type = value
            .get("ContentType")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let target = value
            .get("Target")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let root_element = value
            .get("RootElement")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let path_general = value
            .get("Paths")
            .and_then(|p| p.get("General"))
            .and_then(|v| v.as_str())
            .unwrap_or(".")
            .to_string();

        let mut children = Vec::new();
        if let Some(ch) = value.get("Children").and_then(|v| v.as_array()) {
            for c in ch {
                let cname = c
                    .get("Name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                if cname.is_empty() {
                    continue;
                }
                let api = c
                    .get("ApiName")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&cname)
                    .to_string();
                children.push(ChildConstraint {
                    name: cname,
                    api_name: api,
                    max_gt_one: c
                        .get("MaxOccursGreatThanOne")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    min_non_zero: c
                        .get("MinOccursIsNonZero")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    has_fixed: c
                        .get("HasFixedContent")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    is_data_ref: c
                        .get("IsDataPartReference")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                });
            }
        }

        parts.push(PartRec {
            name,
            rel: rel.to_string(),
            content_type,
            target,
            root_element,
            path_general,
            children,
        });
    }

    for p in &parts {
        if p.children.is_empty() {
            continue;
        }
        let table = format!("CHILDREN_{}", to_snake(&p.name).to_ascii_uppercase());
        s.push_str(&format!(
            "static {table}: &[PartChildConstraint] = &[\n"
        ));
        for c in &p.children {
            s.push_str(&format!(
                "    PartChildConstraint {{ name: \"{}\", api_name: \"{}\", max_occurs_greater_than_one: {}, min_occurs_non_zero: {}, has_fixed_content: {}, is_data_part_reference: {} }},\n",
                escape(&c.name),
                escape(&c.api_name),
                c.max_gt_one,
                c.min_non_zero,
                c.has_fixed,
                c.is_data_ref,
            ));
        }
        s.push_str("];\n");
    }
    s.push('\n');

    s.push_str("/// All known parts.\n");
    s.push_str("pub static PARTS: &[PartInfo] = &[\n");
    for p in &parts {
        let ct = match &p.content_type {
            Some(c) => format!("Some(\"{}\")", escape(c)),
            None => "None".into(),
        };
        let re = match &p.root_element {
            Some(r) => format!("Some(\"{}\")", escape(r)),
            None => "None".into(),
        };
        let children_ref = if p.children.is_empty() {
            "&[]".to_string()
        } else {
            format!("CHILDREN_{}", to_snake(&p.name).to_ascii_uppercase())
        };
        s.push_str(&format!(
            "    PartInfo {{ name: \"{}\", relationship_type: \"{}\", content_type: {ct}, target: \"{}\", root_element: {re}, path_general: \"{}\", children: {children_ref} }},\n",
            escape(&p.name),
            escape(&p.rel),
            escape(&p.target),
            escape(&p.path_general),
        ));
    }
    s.push_str("];\n\n");

    s.push_str("/// Look up part metadata by name (e.g. `\"MainDocumentPart\"`).\n");
    s.push_str("pub fn part_by_name(name: &str) -> Option<&'static PartInfo> {\n");
    s.push_str("    PARTS.iter().find(|p| p.name == name)\n");
    s.push_str("}\n\n");
    s.push_str("/// Look up part metadata by relationship type URI.\n");
    s.push_str(
        "pub fn part_by_relationship_type(relationship_type: &str) -> Option<&'static PartInfo> {\n",
    );
    s.push_str("    PARTS.iter().find(|p| p.relationship_type == relationship_type)\n");
    s.push_str("}\n\n");

    s.push_str("/// Returns true if `child_part_name` is an allowed child of `parent_part_name`.\n");
    s.push_str(
        "pub fn is_allowed_child(parent_part_name: &str, child_part_name: &str) -> bool {\n",
    );
    s.push_str("    part_by_name(parent_part_name)\n");
    s.push_str("        .map(|p| p.children.iter().any(|c| c.name == child_part_name))\n");
    s.push_str("        .unwrap_or(false)\n");
    s.push_str("}\n\n");

    s.push_str("/// Returns true if the parent allows multiple instances of the child part.\n");
    s.push_str("pub fn allows_multiple(parent_part_name: &str, child_part_name: &str) -> bool {\n");
    s.push_str("    part_by_name(parent_part_name)\n");
    s.push_str("        .and_then(|p| p.children.iter().find(|c| c.name == child_part_name))\n");
    s.push_str("        .map(|c| c.max_occurs_greater_than_one)\n");
    s.push_str("        .unwrap_or(false)\n");
    s.push_str("}\n");
    s
}

fn sanitize_module_name(stem: &str) -> String {
    let mut s = stem.to_ascii_lowercase();
    s = s
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    for prefix in [
        "schemas_openxmlformats_org_",
        "schemas_microsoft_com_",
        "purl_oclc_org_",
    ] {
        if let Some(rest) = s.strip_prefix(prefix) {
            s = rest.to_string();
            break;
        }
    }
    if s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        s = format!("n_{s}");
    }
    s
}

fn sanitize_type_name(name: &str) -> String {
    let mut out = String::new();
    let mut cap_next = true;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            if cap_next {
                out.push(c.to_ascii_uppercase());
                cap_next = false;
            } else {
                out.push(c);
            }
        } else {
            cap_next = true;
        }
    }
    if out.is_empty() {
        out.push_str("Unnamed");
    }
    if out
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        out = format!("N{out}");
    }
    out
}

fn facet_to_variant(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut out = first.to_ascii_uppercase().to_string();
    out.extend(chars);
    out.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect()
}

fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "type"
            | "ref"
            | "mod"
            | "use"
            | "fn"
            | "struct"
            | "enum"
            | "const"
            | "static"
            | "crate"
            | "self"
            | "super"
            | "async"
            | "await"
            | "match"
            | "box"
            | "move"
            | "where"
            | "impl"
            | "trait"
            | "pub"
            | "let"
            | "mut"
            | "in"
            | "as"
            | "if"
            | "else"
            | "loop"
            | "while"
            | "for"
            | "return"
            | "break"
            | "continue"
            | "true"
            | "false"
            | "none"
            | "some"
            | "ok"
            | "err"
    )
}

fn simplify_type_name(type_name: &str) -> String {
    if let Some(idx) = type_name.find('<') {
        type_name[..idx].to_string()
    } else if let Some(idx) = type_name.rfind('.') {
        type_name[idx + 1..].to_string()
    } else {
        type_name.to_string()
    }
}

fn to_snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    match out.as_str() {
        "type" | "ref" | "mod" | "use" | "fn" | "struct" | "enum" | "const" | "static"
        | "crate" | "self" | "super" | "async" | "await" | "match" | "box" | "move" | "where"
        | "impl" | "trait" | "pub" | "let" | "mut" | "in" | "as" | "if" | "else" | "loop"
        | "while" | "for" | "return" | "break" | "continue" | "true" | "false" => {
            format!("{out}_")
        }
        _ => out,
    }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
