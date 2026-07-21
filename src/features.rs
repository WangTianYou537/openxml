//! Lightweight feature bag inspired by C# `IFeatureCollection`.
//!
//! The C# SDK uses a full Features DI container with events and ParagraphId
//! generation. This module provides a typed-key bag for attaching optional
//! services to a package without a full DI graph.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Type-keyed bag of optional services.
#[derive(Default)]
pub struct FeatureCollection {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl std::fmt::Debug for FeatureCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureCollection")
            .field("len", &self.map.len())
            .finish()
    }
}

impl FeatureCollection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a feature of type `T`.
    pub fn set<T: Any + Send + Sync>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Get a reference to feature `T` if present.
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T>())
    }

    /// Get a mutable reference to feature `T` if present.
    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut::<T>())
    }

    /// Remove feature `T`, returning it if present.
    pub fn remove<T: Any + Send + Sync>(&mut self) -> Option<T> {
        self.map
            .remove(&TypeId::of::<T>())
            .and_then(|b| b.downcast::<T>().ok().map(|b| *b))
    }

    pub fn contains<T: Any + Send + Sync>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

/// Simple monotonic paragraph-id generator (C# `ParagraphIdFeature` shell).
#[derive(Debug, Clone)]
pub struct ParagraphIdGenerator {
    next: u32,
}

impl Default for ParagraphIdGenerator {
    fn default() -> Self {
        Self { next: 1 }
    }
}

impl ParagraphIdGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_start(start: u32) -> Self {
        Self { next: start.max(1) }
    }

    /// Allocate the next 8-hex-digit paragraph id.
    pub fn next_id(&mut self) -> String {
        let id = self.next;
        self.next = self.next.wrapping_add(1).max(1);
        format!("{id:08X}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_bag_roundtrip() {
        let mut f = FeatureCollection::new();
        assert!(f.is_empty());
        f.set(ParagraphIdGenerator::with_start(10));
        assert!(f.contains::<ParagraphIdGenerator>());
        assert_eq!(
            f.get_mut::<ParagraphIdGenerator>().unwrap().next_id(),
            "0000000A"
        );
        assert_eq!(
            f.get_mut::<ParagraphIdGenerator>().unwrap().next_id(),
            "0000000B"
        );
        let mut g = f.remove::<ParagraphIdGenerator>().unwrap();
        assert!(!f.contains::<ParagraphIdGenerator>());
        assert_eq!(g.next_id(), "0000000C");
    }
}
