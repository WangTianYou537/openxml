//! Lightweight feature bag inspired by C# `IFeatureCollection`.
//!
//! The C# SDK uses a full Features DI container with events and ParagraphId
//! generation. This module provides a typed-key bag for attaching optional
//! services to a package without a full DI graph, plus a small package-event
//! hub mirroring `IPackageEventsFeature` / `EventType`.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

/// Package / part lifecycle events (subset of C# `DocumentFormat.OpenXml.Features.EventType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageEventType {
    Creating,
    Created,
    Deleting,
    Deleted,
    Adding,
    Added,
    Removing,
    Removed,
    Closing,
    Closed,
    Saving,
    Saved,
    Reloading,
    Reloaded,
}

/// Event payload for package-level notifications.
#[derive(Debug, Clone)]
pub struct PackageEvent {
    pub event_type: PackageEventType,
    /// Optional part URI related to the event (empty for package-wide events).
    pub part_uri: Option<String>,
}

type Listener = Arc<dyn Fn(&PackageEvent) + Send + Sync>;

/// Observable package event hub (C# `IPackageEventsFeature` shell).
///
/// Part container events live on [`PartEvents`]; part-root DOM events on [`PartRootEvents`].
/// Store in [`FeatureCollection`] via `features.set(PackageEvents::new())`.
#[derive(Clone, Default)]
pub struct PackageEvents {
    listeners: Arc<Mutex<Vec<Listener>>>,
}

impl std::fmt::Debug for PackageEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.listeners.lock().map(|g| g.len()).unwrap_or(0);
        f.debug_struct("PackageEvents")
            .field("listeners", &n)
            .finish()
    }
}

impl PackageEvents {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a listener. Returns an id that can be passed to [`unsubscribe`](Self::unsubscribe).
    pub fn subscribe<F>(&self, f: F) -> usize
    where
        F: Fn(&PackageEvent) + Send + Sync + 'static,
    {
        let mut guard = self.listeners.lock().expect("package events lock");
        guard.push(Arc::new(f));
        guard.len() - 1
    }

    /// Remove listener by index from [`subscribe`](Self::subscribe). No-op if out of range.
    pub fn unsubscribe(&self, id: usize) {
        let mut guard = self.listeners.lock().expect("package events lock");
        if id < guard.len() {
            guard[id] = Arc::new(|_: &PackageEvent| {});
        }
    }

    /// Raise an event to all listeners.
    pub fn raise(&self, event: PackageEvent) {
        let guard = self.listeners.lock().expect("package events lock");
        for listener in guard.iter() {
            listener(&event);
        }
    }

    pub fn raise_type(&self, event_type: PackageEventType) {
        self.raise(PackageEvent {
            event_type,
            part_uri: None,
        });
    }

    pub fn raise_part(&self, event_type: PackageEventType, part_uri: impl Into<String>) {
        self.raise(PackageEvent {
            event_type,
            part_uri: Some(part_uri.into()),
        });
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.lock().map(|g| g.len()).unwrap_or(0)
    }
}

/// Part container lifecycle events (C# `IPartEventsFeature` shell).
///
/// Fired when parts are added or removed from the package graph (not DOM root load/save —
/// that is [`PartRootEvents`]).
#[derive(Clone, Default)]
pub struct PartEvents {
    inner: PackageEvents,
}

impl std::fmt::Debug for PartEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartEvents")
            .field("listeners", &self.inner.listener_count())
            .finish()
    }
}

impl PartEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<F>(&self, f: F) -> usize
    where
        F: Fn(&PackageEvent) + Send + Sync + 'static,
    {
        self.inner.subscribe(f)
    }

    pub fn unsubscribe(&self, id: usize) {
        self.inner.unsubscribe(id);
    }

    pub fn raise(&self, event_type: PackageEventType, part_uri: impl Into<String>) {
        self.inner.raise_part(event_type, part_uri);
    }

    pub fn listener_count(&self) -> usize {
        self.inner.listener_count()
    }
}

/// Part-root lifecycle events (C# `IPartRootEventsFeature` shell).
///
/// Same event types as [`PackageEvents`], but always associated with a part URI
/// (load / unload / save / reload of the part's DOM root).
#[derive(Clone, Default)]
pub struct PartRootEvents {
    inner: PackageEvents,
}

impl std::fmt::Debug for PartRootEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartRootEvents")
            .field("listeners", &self.inner.listener_count())
            .finish()
    }
}

impl PartRootEvents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<F>(&self, f: F) -> usize
    where
        F: Fn(&PackageEvent) + Send + Sync + 'static,
    {
        self.inner.subscribe(f)
    }

    pub fn unsubscribe(&self, id: usize) {
        self.inner.unsubscribe(id);
    }

    pub fn raise(&self, event_type: PackageEventType, part_uri: impl Into<String>) {
        self.inner.raise_part(event_type, part_uri);
    }

    pub fn listener_count(&self) -> usize {
        self.inner.listener_count()
    }
}

/// Package / part container annotations (C# `AnnotationsFeature` on `OpenXmlPartContainer`).
///
/// Stored in [`FeatureCollection`]; also available per-element on [`crate::element::OpenXmlElement`].
#[derive(Default)]
pub struct AnnotationsFeature {
    entries: Vec<AnnoSlot>,
}

struct AnnoSlot {
    type_id: std::any::TypeId,
    value: Box<dyn std::any::Any + Send + Sync>,
}

impl std::fmt::Debug for AnnotationsFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnnotationsFeature")
            .field("len", &self.entries.len())
            .finish()
    }
}

impl AnnotationsFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: std::any::Any + Send + Sync>(&mut self, value: T) {
        self.entries.push(AnnoSlot {
            type_id: std::any::TypeId::of::<T>(),
            value: Box::new(value),
        });
    }

    pub fn get<T: std::any::Any + Send + Sync>(&self) -> Option<&T> {
        self.entries
            .iter()
            .find(|e| e.type_id == std::any::TypeId::of::<T>())
            .and_then(|e| e.value.downcast_ref::<T>())
    }

    pub fn get_all<T: std::any::Any + Send + Sync>(&self) -> Vec<&T> {
        self.entries
            .iter()
            .filter(|e| e.type_id == std::any::TypeId::of::<T>())
            .filter_map(|e| e.value.downcast_ref::<T>())
            .collect()
    }

    pub fn remove<T: std::any::Any + Send + Sync>(&mut self) {
        let tid = std::any::TypeId::of::<T>();
        self.entries.retain(|e| e.type_id != tid);
    }

    pub fn contains<T: std::any::Any + Send + Sync>(&self) -> bool {
        self.entries
            .iter()
            .any(|e| e.type_id == std::any::TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

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

    #[test]
    fn package_events_fire() {
        let events = PackageEvents::new();
        let count = Arc::new(AtomicUsize::new(0));
        let c2 = count.clone();
        events.subscribe(move |e| {
            if e.event_type == PackageEventType::Closing {
                c2.fetch_add(1, Ordering::SeqCst);
            }
        });
        events.raise_type(PackageEventType::Saving);
        events.raise_type(PackageEventType::Closing);
        assert_eq!(count.load(Ordering::SeqCst), 1);
        events.raise_part(PackageEventType::Deleted, "/word/styles.xml");
    }

    #[test]
    fn part_root_events_fire() {
        let events = PartRootEvents::new();
        let count = Arc::new(AtomicUsize::new(0));
        let c2 = count.clone();
        events.subscribe(move |e| {
            if e.event_type == PackageEventType::Reloaded {
                c2.fetch_add(1, Ordering::SeqCst);
            }
        });
        events.raise(PackageEventType::Reloading, "/word/document.xml");
        events.raise(PackageEventType::Reloaded, "/word/document.xml");
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn part_events_fire() {
        let events = PartEvents::new();
        let count = Arc::new(AtomicUsize::new(0));
        let c2 = count.clone();
        events.subscribe(move |e| {
            if e.event_type == PackageEventType::Added {
                c2.fetch_add(1, Ordering::SeqCst);
            }
        });
        events.raise(PackageEventType::Adding, "/word/styles.xml");
        events.raise(PackageEventType::Added, "/word/styles.xml");
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
}
