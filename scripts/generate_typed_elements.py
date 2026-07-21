#!/usr/bin/env python3
"""Generate strong-typed OpenXmlElement wrappers from schema JSON.

Usage:
  python3 scripts/generate_typed_elements.py \\
    --data /opt/wp/Open-XML-SDK/data \\
    --out src/generated/typed_elements.rs

Emits thin owned wrappers with attribute get/set and child accessors.
"""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


def rust_ident(s: str) -> str:
    s = re.sub(r"[^0-9A-Za-z_]", "_", s)
    if s and s[0].isdigit():
        s = f"N_{s}"
    # Rust keywords
    keywords = {
        "type", "fn", "mod", "use", "self", "super", "crate", "impl", "let",
        "mut", "ref", "match", "if", "else", "for", "while", "loop", "return",
        "struct", "enum", "trait", "const", "static", "pub", "as", "in", "where",
        "move", "box", "async", "await", "dyn", "true", "false", "abstract",
        "become", "box", "do", "final", "macro", "override", "priv", "typeof",
        "unsized", "virtual", "yield", "try",
    }
    if s.lower() in keywords:
        s = f"{s}_"
    return s


def snake(s: str) -> str:
    # PascalCase / camelCase → snake
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    s = s.replace("-", "_").replace(".", "_")
    return rust_ident(s.lower())


def local_from_name(name: str) -> str | None:
    # "w:CT_P/w:p" → p
    if "/" in name:
        right = name.split("/")[-1]
    else:
        right = name
    if ":" in right:
        return right.split(":")[-1]
    return right or None


def prefix_from_name(name: str) -> str:
    if "/" in name:
        right = name.split("/")[-1]
    else:
        right = name
    if ":" in right:
        return right.split(":")[0]
    return "w"


def ctor_for_prefix(prefix: str) -> str:
    return {
        "w": "OpenXmlElement::w",
        "x": "OpenXmlElement::x",
        "p": "OpenXmlElement::p",
        "a": "OpenXmlElement::a",
        "r": "OpenXmlElement::new",  # relationships-ish
    }.get(prefix, "OpenXmlElement::new")


def load_types(schema_path: Path) -> list[dict]:
    data = json.loads(schema_path.read_text())
    return data.get("Types") or []


def generate(types: list[dict], module_comment: str) -> str:
    # Deduplicate by ClassName (prefer concrete element with local name)
    by_class: dict[str, dict] = {}
    for t in types:
        cn = t.get("ClassName")
        if not cn or t.get("IsAbstract"):
            continue
        loc = local_from_name(t.get("Name", ""))
        if not loc:
            continue
        prev = by_class.get(cn)
        if prev is None or (not prev.get("_local") and loc):
            t = dict(t)
            t["_local"] = loc
            t["_prefix"] = prefix_from_name(t.get("Name", ""))
            by_class[cn] = t

    lines = [
        f"//! {module_comment}",
        "//!",
        "//! Auto-generated strong-typed wrappers over [`OpenXmlElement`].",
        "//! Regenerate: python3 scripts/generate_typed_elements.py",
        "",
        "use crate::element::OpenXmlElement;",
        "",
    ]

    # Emit structs
    for cn in sorted(by_class.keys()):
        t = by_class[cn]
        loc = t["_local"]
        prefix = t["_prefix"]
        rust_name = rust_ident(cn)
        ctor = ctor_for_prefix(prefix)
        summary = (t.get("Summary") or "").replace("\n", " ").strip()
        if summary:
            lines.append(f"/// {summary}")
        lines.append("#[derive(Debug, Clone)]")
        lines.append(f"pub struct {rust_name} {{")
        lines.append("    pub inner: OpenXmlElement,")
        lines.append("}")
        lines.append("")
        lines.append(f"impl {rust_name} {{")
        if ctor == "OpenXmlElement::new":
            lines.append(
                f'    pub fn new() -> Self {{ Self {{ inner: OpenXmlElement::new("{prefix}", "", "{loc}") }} }}'
            )
        else:
            lines.append(
                f'    pub fn new() -> Self {{ Self {{ inner: {ctor}("{loc}") }} }}'
            )
        lines.append(
            f'    pub fn from_element(el: OpenXmlElement) -> Option<Self> {{ if el.local_name == "{loc}" {{ Some(Self {{ inner: el }}) }} else {{ None }} }}'
        )
        lines.append("    pub fn into_inner(self) -> OpenXmlElement { self.inner }")
        lines.append("    pub fn as_element(&self) -> &OpenXmlElement { &self.inner }")
        lines.append(
            "    pub fn as_element_mut(&mut self) -> &mut OpenXmlElement { &mut self.inner }"
        )

        # attributes
        for attr in t.get("Attributes") or []:
            qname = attr.get("QName") or ""
            prop = attr.get("PropertyName") or qname.split(":")[-1]
            method = snake(prop)
            if method in ("new", "from_element", "into_inner", "as_element", "as_element_mut", "default", "text", "set_text"):
                method = f"{method}_attr"
            attr_local = qname.split(":")[-1] if qname else prop
            lines.append(f"    pub fn {method}(&self) -> Option<&str> {{")
            lines.append(
                f'        self.inner.get_attribute("{attr_local}").or_else(|| self.inner.get_attribute_qname("{qname}")).or_else(|| self.inner.attributes.iter().find(|a| a.local_name == "{attr_local}").map(|a| a.value.as_str()))'
            )
            lines.append("    }")
            lines.append(
                f"    pub fn set_{method}(&mut self, value: impl Into<String>) {{"
            )
            if qname and ":" in qname:
                lines.append(
                    f'        self.inner.set_attribute_qname("{qname}", value);'
                )
            else:
                lines.append(
                    f'        self.inner.set_attribute("{attr_local}", value);'
                )
            lines.append("    }")

        # children accessors (first occurrence)
        seen_child_loc = set()
        used_methods = {"new", "from_element", "into_inner", "as_element", "as_element_mut", "default", "text", "set_text"}
        # reserve attribute method names already emitted
        for attr in t.get("Attributes") or []:
            prop = attr.get("PropertyName") or (attr.get("QName") or "").split(":")[-1]
            m = snake(prop)
            if m in ("new", "from_element", "into_inner", "as_element", "as_element_mut", "default", "text", "set_text"):
                m = f"{m}_attr"
            used_methods.add(m)
            used_methods.add(f"set_{m}")
        for ch in t.get("Children") or []:
            cname = ch.get("Name") or ""
            cloc = local_from_name(cname)
            if not cloc or cloc in seen_child_loc:
                continue
            seen_child_loc.add(cloc)
            prop = ch.get("PropertyName") or cloc
            method = snake(prop)
            if not method or method in used_methods:
                method = snake(cloc)
            base = method
            n = 2
            while method in used_methods or f"append_{method}" in used_methods:
                method = f"{base}_{n}"
                n += 1
            used_methods.add(method)
            used_methods.add(f"{method}_mut")
            used_methods.add(f"append_{method}")
            lines.append(
                f'    pub fn {method}(&self) -> Option<&OpenXmlElement> {{ self.inner.child("{cloc}") }}'
            )
            lines.append(
                f'    pub fn {method}_mut(&mut self) -> Option<&mut OpenXmlElement> {{ self.inner.child_mut("{cloc}") }}'
            )
            lines.append(
                f"    pub fn append_{method}(&mut self, child: OpenXmlElement) {{ self.inner.append_child(child); }}"
            )

        if t.get("IsLeafText"):
            lines.append(
                "    pub fn text(&self) -> String { self.inner.inner_text() }"
            )
            lines.append(
                "    pub fn set_text(&mut self, value: impl Into<String>) { self.inner.text = Some(value.into()); }"
            )

        lines.append("}")
        lines.append("")
        lines.append(f"impl Default for {rust_name} {{")
        lines.append("    fn default() -> Self { Self::new() }")
        lines.append("}")
        lines.append("")
        lines.append(f"impl From<{rust_name}> for OpenXmlElement {{")
        lines.append(f"    fn from(v: {rust_name}) -> Self {{ v.inner }}")
        lines.append("}")
        lines.append("")

    lines.append(f"/// Number of generated typed wrappers in this module.")
    lines.append(f"pub const TYPED_ELEMENT_COUNT: usize = {len(by_class)};")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--data", type=Path, default=Path("/opt/wp/Open-XML-SDK/data"))
    ap.add_argument(
        "--out",
        type=Path,
        default=Path(__file__).resolve().parents[1] / "src/generated/typed_elements.rs",
    )
    args = ap.parse_args()
    schemas = [
        (
            "schemas_openxmlformats_org_wordprocessingml_2006_main.json",
            "WordprocessingML 2006 main typed elements",
        ),
        (
            "schemas_openxmlformats_org_spreadsheetml_2006_main.json",
            "SpreadsheetML 2006 main typed elements",
        ),
        (
            "schemas_openxmlformats_org_presentationml_2006_main.json",
            "PresentationML 2006 main typed elements",
        ),
    ]
    all_types: list[dict] = []
    comments = []
    for fname, comment in schemas:
        path = args.data / "schemas" / fname
        if not path.exists():
            print(f"skip missing {path}")
            continue
        ts = load_types(path)
        all_types.extend(ts)
        comments.append(f"{comment} ({len(ts)} schema types)")
        print(f"loaded {len(ts)} from {fname}")

    # ClassName collisions across schemas: suffix later ones
    seen: set[str] = set()
    uniqued = []
    for t in all_types:
        cn = t.get("ClassName")
        if not cn:
            uniqued.append(t)
            continue
        if cn in seen:
            # qualify with local name
            loc = local_from_name(t.get("Name", "")) or "X"
            pref = prefix_from_name(t.get("Name", ""))
            t = dict(t)
            t["ClassName"] = f"{cn}_{pref}_{loc}"
        seen.add(t["ClassName"])
        uniqued.append(t)

    text = generate(uniqued, "; ".join(comments))
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(text)
    # count structs
    n = text.count("pub struct ")
    print(f"wrote {args.out} ({n} structs, {len(text)} bytes)")


if __name__ == "__main__":
    main()
