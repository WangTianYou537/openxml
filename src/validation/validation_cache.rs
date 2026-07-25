//! Validation constraint cache (C# `ValidationCache` shell).
//!
//! C# caches version-built particle constraints. This port keeps a lightweight
//! version-scoped bag; particle tables are still looked up via the static
//! `particle` module rather than per-element metadata.

use crate::file_format::FileFormatVersions;
use std::collections::HashMap;

/// Version-scoped validation cache (C# `ValidationCache`).
#[derive(Debug, Clone)]
pub struct ValidationCache {
    version: FileFormatVersions,
    /// Optional string-keyed memo for expensive version-specific lookups.
    memo: HashMap<String, String>,
}

impl ValidationCache {
    pub fn new(version: FileFormatVersions) -> Self {
        Self {
            version,
            memo: HashMap::new(),
        }
    }

    pub fn version(&self) -> FileFormatVersions {
        self.version
    }

    pub fn set_version(&mut self, version: FileFormatVersions) {
        if version != self.version {
            self.version = version;
            self.memo.clear();
        }
    }

    pub fn clear(&mut self) {
        self.memo.clear();
    }

    /// Memoize a string value for `key` (application / particle helper use).
    pub fn get_or_insert_with<F>(&mut self, key: impl Into<String>, f: F) -> &str
    where
        F: FnOnce() -> String,
    {
        let key = key.into();
        self.memo.entry(key).or_insert_with(f).as_str()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.memo.get(key).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.memo.len()
    }

    pub fn is_empty(&self) -> bool {
        self.memo.is_empty()
    }
}

impl Default for ValidationCache {
    fn default() -> Self {
        Self::new(FileFormatVersions::OFFICE2007)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_memo_and_version_reset() {
        let mut c = ValidationCache::new(FileFormatVersions::OFFICE2010);
        assert_eq!(c.version(), FileFormatVersions::OFFICE2010);
        let v = c.get_or_insert_with("k", || "v".into()).to_string();
        assert_eq!(v, "v");
        assert_eq!(c.get("k"), Some("v"));
        assert_eq!(c.len(), 1);
        c.set_version(FileFormatVersions::OFFICE2016);
        assert!(c.is_empty());
        c.clear();
        assert!(c.is_empty());
    }
}
