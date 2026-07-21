//! Pack URI helpers (OPC part names).

use crate::error::{Error, Result};
use std::path::{Component, Path, PathBuf};

/// An absolute pack URI, always starting with `/` and using `/` separators.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackUri(String);

impl PackUri {
    /// Create a pack URI from a path that may or may not start with `/`.
    pub fn new(path: impl AsRef<str>) -> Self {
        let mut s = path.as_ref().replace('\\', "/");
        if !s.starts_with('/') {
            s.insert(0, '/');
        }
        // Collapse duplicate slashes
        while s.contains("//") {
            s = s.replace("//", "/");
        }
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// ZIP entry name (without leading `/`).
    pub fn zip_name(&self) -> &str {
        self.0.trim_start_matches('/')
    }

    /// Directory portion of the URI (with trailing slash when non-root).
    pub fn parent(&self) -> PackUri {
        let s = self.0.trim_end_matches('/');
        match s.rfind('/') {
            Some(0) | None => PackUri::new("/"),
            Some(i) => PackUri::new(&s[..=i]),
        }
    }

    /// Relationship part URI for this part, e.g. `/word/document.xml` → `/word/_rels/document.xml.rels`.
    pub fn relationship_part_uri(&self) -> PackUri {
        let parent = self.parent();
        let name = self.0.rsplit('/').next().unwrap_or("");
        if parent.as_str() == "/" {
            PackUri::new(format!("/_rels/{name}.rels"))
        } else {
            let p = parent.as_str().trim_end_matches('/');
            PackUri::new(format!("{p}/_rels/{name}.rels"))
        }
    }
}

impl std::fmt::Display for PackUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for PackUri {
    fn from(value: &str) -> Self {
        PackUri::new(value)
    }
}

impl From<String> for PackUri {
    fn from(value: String) -> Self {
        PackUri::new(value)
    }
}

/// Build a pack URI from a path string.
pub fn pack_uri(path: impl AsRef<str>) -> PackUri {
    PackUri::new(path)
}

/// Resolve a relative target against a source part URI.
///
/// OPC relationship targets are relative to the source part.
pub fn resolve_uri(source: &PackUri, target: &str) -> Result<PackUri> {
    if target.starts_with('/') {
        return Ok(PackUri::new(target));
    }

    let base = source.parent();
    let mut parts: Vec<String> = if base.as_str() == "/" {
        Vec::new()
    } else {
        base.as_str()
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    };

    for segment in target.replace('\\', "/").split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err(Error::Package(format!(
                        "cannot resolve target `{target}` against `{source}`"
                    )));
                }
            }
            other => parts.push(other.to_string()),
        }
    }

    Ok(PackUri::new(format!("/{}", parts.join("/"))))
}

/// Make `target` relative to `source` (for writing relationship Target attributes).
pub fn relativize(source: &PackUri, target: &PackUri) -> String {
    let source_dir = source.parent();
    let src_parts: Vec<&str> = if source_dir.as_str() == "/" {
        Vec::new()
    } else {
        source_dir
            .as_str()
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect()
    };
    let tgt_parts: Vec<&str> = target
        .as_str()
        .trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let mut common = 0;
    for i in 0..src_parts.len().min(tgt_parts.len()) {
        if src_parts[i] == tgt_parts[i] {
            common += 1;
        } else {
            break;
        }
    }

    let mut rel: Vec<&str> = Vec::new();
    for _ in common..src_parts.len() {
        rel.push("..");
    }
    for p in &tgt_parts[common..] {
        rel.push(p);
    }
    if rel.is_empty() {
        ".".into()
    } else {
        rel.join("/")
    }
}

/// Normalize a filesystem path for display / error messages.
#[allow(dead_code)]
pub fn normalize_fs_path(path: impl AsRef<Path>) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.as_ref().components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_uri() {
        let u = PackUri::new("/word/document.xml");
        assert_eq!(u.relationship_part_uri().as_str(), "/word/_rels/document.xml.rels");
        let root = PackUri::new("/");
        assert_eq!(root.relationship_part_uri().as_str(), "/_rels/.rels");
    }

    #[test]
    fn resolve_relative() {
        let src = PackUri::new("/word/document.xml");
        assert_eq!(
            resolve_uri(&src, "styles.xml").unwrap().as_str(),
            "/word/styles.xml"
        );
        assert_eq!(
            resolve_uri(&src, "../docProps/core.xml").unwrap().as_str(),
            "/docProps/core.xml"
        );
    }
}
