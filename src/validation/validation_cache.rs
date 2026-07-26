//! Validation constraint cache (C# `ValidationCache` shell).
//!
//! C# caches version-built particle constraints. This port keeps a lightweight
//! version-scoped bag and resolves hand-authored particles via
//! [`crate::validation::particle::particle_for`], applying
//! [`Particle::build_for`] for the cache's target version.

use crate::file_format::FileFormatVersions;
use crate::validation::particle::{particle_for, Particle};
use std::collections::HashMap;

/// Version-scoped validation cache (C# `ValidationCache`).
#[derive(Debug, Clone)]
pub struct ValidationCache {
    version: FileFormatVersions,
    /// Optional string-keyed memo for expensive version-specific lookups.
    memo: HashMap<String, String>,
    /// Memo of version-built particles keyed by element local name.
    particles: HashMap<String, Option<Particle>>,
}

impl ValidationCache {
    pub fn new(version: FileFormatVersions) -> Self {
        Self {
            version,
            memo: HashMap::new(),
            particles: HashMap::new(),
        }
    }

    pub fn version(&self) -> FileFormatVersions {
        self.version
    }

    pub fn set_version(&mut self, version: FileFormatVersions) {
        if version != self.version {
            self.version = version;
            self.memo.clear();
            self.particles.clear();
        }
    }

    pub fn clear(&mut self) {
        self.memo.clear();
        self.particles.clear();
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
        self.memo.is_empty() && self.particles.is_empty()
    }

    /// C# `ValidationCache.GetConstraint(element)` — resolve and version-build
    /// the particle for `local_name`, memoizing the result for this cache version.
    pub fn get_constraint(&mut self, local_name: &str) -> Option<&Particle> {
        if !self.particles.contains_key(local_name) {
            let built = particle_for(local_name)
                .and_then(|particle| particle.build_for(self.version));
            self.particles.insert(local_name.to_string(), built);
        }
        self.particles
            .get(local_name)
            .and_then(|entry| entry.as_ref())
    }

    /// Number of memoized particle entries (including negative hits).
    pub fn particle_cache_len(&self) -> usize {
        self.particles.len()
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

    #[test]
    fn get_constraint_resolves_word_particles_and_memos() {
        let mut c = ValidationCache::new(FileFormatVersions::OFFICE2007);
        let p = c.get_constraint("document").expect("document particle");
        assert_eq!(
            p.particle_type(),
            crate::validation::ParticleType::Sequence
        );
        assert_eq!(c.particle_cache_len(), 1);
        // Second lookup hits the memo.
        assert!(c.get_constraint("document").is_some());
        assert_eq!(c.particle_cache_len(), 1);
        assert!(c.get_constraint("not-a-real-element").is_none());
        assert_eq!(c.particle_cache_len(), 2);

        // Version change clears particle memo.
        c.set_version(FileFormatVersions::OFFICE2010);
        assert_eq!(c.particle_cache_len(), 0);
        assert!(c.get_constraint("p").is_some());
    }
}
