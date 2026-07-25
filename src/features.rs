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

    /// Get feature `T`, panicking if missing (C# `GetRequiredFeature` / `GetRequired`).
    pub fn get_required<T: Any + Send + Sync>(&self) -> &T {
        self.get::<T>().unwrap_or_else(|| {
            panic!(
                "required feature {} is not registered",
                std::any::type_name::<T>()
            )
        })
    }

    /// Get mutable feature `T`, panicking if missing.
    pub fn get_required_mut<T: Any + Send + Sync>(&mut self) -> &mut T {
        let name = std::any::type_name::<T>();
        self.get_mut::<T>()
            .unwrap_or_else(|| panic!("required feature {name} is not registered"))
    }

    /// Get or insert default for `T` (C# `GetOrAddFeature` shell when `T: Default`).
    pub fn get_or_add<T: Any + Send + Sync + Default>(&mut self) -> &mut T {
        if !self.contains::<T>() {
            self.set(T::default());
        }
        self.get_mut::<T>().expect("just inserted")
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

/// Package capability flags (C# `PackageCapabilities`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PackageCapabilities {
    bits: u32,
}

impl PackageCapabilities {
    pub const NONE: Self = Self { bits: 0 };
    pub const SAVE: Self = Self { bits: 1 };
    pub const RELOAD: Self = Self { bits: 1 << 1 };
    pub const CACHED: Self = Self { bits: 1 << 2 };
    pub const LARGE_PART_STREAMS: Self = Self { bits: 1 << 3 };
    pub const MALFORMED_URI: Self = Self { bits: 1 << 4 };

    pub const fn empty() -> Self {
        Self::NONE
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.bits & other.bits) != 0
    }

    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    /// Default in-memory package capabilities: Save | Cached.
    pub fn memory_default() -> Self {
        Self::SAVE.union(Self::CACHED)
    }
}

impl std::ops::BitOr for PackageCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for PackageCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

/// Mutable relationship builder snapshot used by filters (C# `PackageRelationshipBuilder` shell).
#[derive(Debug, Clone)]
pub struct PackageRelationshipBuilder {
    pub id: String,
    pub relationship_type: String,
    pub target: String,
    pub target_mode: String,
    pub source_uri: Option<String>,
}

impl PackageRelationshipBuilder {
    pub fn new(
        id: impl Into<String>,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            relationship_type: relationship_type.into(),
            target: target.into(),
            target_mode: "Internal".into(),
            source_uri: None,
        }
    }

    pub fn with_target_mode(mut self, mode: impl Into<String>) -> Self {
        self.target_mode = mode.into();
        self
    }

    pub fn with_source_uri(mut self, uri: impl Into<String>) -> Self {
        self.source_uri = Some(uri.into());
        self
    }
}

/// Relationship filter pipeline (C# `IRelationshipFilterFeature`).
///
/// Filters run in registration order and may rewrite relationship fields before they are committed.
#[derive(Default)]
pub struct RelationshipFilterFeature {
    filters: Mutex<Vec<Box<dyn Fn(&mut PackageRelationshipBuilder) + Send>>>,
}

impl std::fmt::Debug for RelationshipFilterFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.filters.lock().map(|g| g.len()).unwrap_or(0);
        f.debug_struct("RelationshipFilterFeature")
            .field("filters", &n)
            .finish()
    }
}

impl RelationshipFilterFeature {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a filter (C# `AddFilter`).
    pub fn add_filter<F>(&self, f: F)
    where
        F: Fn(&mut PackageRelationshipBuilder) + Send + 'static,
    {
        if let Ok(mut g) = self.filters.lock() {
            g.push(Box::new(f));
        }
    }

    /// Apply all filters to `builder` in registration order.
    pub fn apply(&self, builder: &mut PackageRelationshipBuilder) {
        if let Ok(g) = self.filters.lock() {
            for f in g.iter() {
                f(builder);
            }
        }
    }

    pub fn filter_count(&self) -> usize {
        self.filters.lock().map(|g| g.len()).unwrap_or(0)
    }
}

/// Package factory feature marker (C# `IPackageFactoryFeature<TPackage>` shell).
///
/// Holds a type-erased document kind name so packages can advertise which factory
/// created them without a full generic factory graph.
#[derive(Debug, Clone, Default)]
pub struct PackageFactoryFeature {
    pub package_kind: String,
}

impl PackageFactoryFeature {
    pub fn new(package_kind: impl Into<String>) -> Self {
        Self {
            package_kind: package_kind.into(),
        }
    }
}

/// Programmatic identifier used when generating part/relationship ids
/// (C# `IProgrammaticIdentifierFeature` shell).
#[derive(Debug)]
pub struct ProgrammaticIdentifierFeature {
    pub identifier: String,
    next: Mutex<u32>,
}

impl Default for ProgrammaticIdentifierFeature {
    fn default() -> Self {
        Self::new("R")
    }
}

impl ProgrammaticIdentifierFeature {
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
            next: Mutex::new(1),
        }
    }

    /// Next id of the form `{identifier}{n}` (hex), e.g. `R00000001`.
    pub fn next_id(&self) -> String {
        let mut g = self.next.lock().unwrap_or_else(|e| e.into_inner());
        let n = *g;
        *g = g.wrapping_add(1);
        format!("{}{:08X}", self.identifier, n)
    }
}

/// Application host type flags (C# `ApplicationType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ApplicationType {
    bits: u8,
}

impl ApplicationType {
    pub const NONE: Self = Self { bits: 0 };
    pub const WORD: Self = Self { bits: 1 };
    pub const EXCEL: Self = Self { bits: 1 << 1 };
    pub const POWERPOINT: Self = Self { bits: 1 << 2 };
    pub const ALL: Self = Self {
        bits: Self::WORD.bits | Self::EXCEL.bits | Self::POWERPOINT.bits,
    };

    pub const fn empty() -> Self {
        Self::NONE
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.bits & other.bits) != 0
    }

    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }
}

impl std::ops::BitOr for ApplicationType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for ApplicationType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

/// Tracks dispose callbacks for package/part close (C# `IDisposableFeature` shell).
#[derive(Default)]
pub struct DisposableFeature {
    callbacks: Mutex<Vec<Box<dyn FnOnce() + Send>>>,
}

impl std::fmt::Debug for DisposableFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.callbacks.lock().map(|g| g.len()).unwrap_or(0);
        f.debug_struct("DisposableFeature")
            .field("pending", &n)
            .finish()
    }
}

impl DisposableFeature {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a callback invoked by [`dispose_all`](Self::dispose_all) (C# `Register`).
    pub fn register<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.callbacks
            .lock()
            .expect("disposable feature lock")
            .push(Box::new(f));
    }

    pub fn pending_count(&self) -> usize {
        self.callbacks.lock().map(|g| g.len()).unwrap_or(0)
    }

    /// Run and clear all registered dispose callbacks (LIFO).
    pub fn dispose_all(&mut self) {
        let mut guard = self.callbacks.lock().expect("disposable feature lock");
        while let Some(cb) = guard.pop() {
            // Drop the lock while running so re-entrant register is safe.
            drop(guard);
            cb();
            guard = self.callbacks.lock().expect("disposable feature lock");
        }
    }
}

/// Main package part metadata (C# `IMainPartFeature` shell).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MainPartFeature {
    pub relationship_type: String,
    pub content_type: String,
    pub part_uri: Option<String>,
}

impl MainPartFeature {
    pub fn new(
        relationship_type: impl Into<String>,
        content_type: impl Into<String>,
        part_uri: Option<String>,
    ) -> Self {
        Self {
            relationship_type: relationship_type.into(),
            content_type: content_type.into(),
            part_uri,
        }
    }

    pub fn with_uri(mut self, uri: impl Into<String>) -> Self {
        self.part_uri = Some(uri.into());
        self
    }
}

/// Document type tag for package builders (C# `IDocumentTypeFeature` shell).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentTypeFeature {
    pub document_type: String,
}

impl DocumentTypeFeature {
    pub fn new(document_type: impl Into<String>) -> Self {
        Self {
            document_type: document_type.into(),
        }
    }
}

/// Tracks schema elements / relationships observed during open (C# `ISchemaTrackingFeature` shell).
#[derive(Debug, Clone, Default)]
pub struct SchemaTrackingFeature {
    pub root_elements: Vec<crate::element::OpenXmlQualifiedName>,
    pub relationships: Vec<String>,
}

impl SchemaTrackingFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track_root(&mut self, qname: crate::element::OpenXmlQualifiedName) {
        if !self.root_elements.iter().any(|q| q == &qname) {
            self.root_elements.push(qname);
        }
    }

    pub fn track_relationship(&mut self, rel: impl Into<String>) {
        let rel = rel.into();
        if !self.relationships.iter().any(|r| r == &rel) {
            self.relationships.push(rel);
        }
    }

    pub fn clear(&mut self) {
        self.root_elements.clear();
        self.relationships.clear();
    }
}

/// Whether a strict-namespace package was observed (C# `IStrictNamespaceFeature` shell).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StrictNamespaceFeature {
    pub found: bool,
}

impl StrictNamespaceFeature {
    pub fn new(found: bool) -> Self {
        Self { found }
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

    #[test]
    fn get_or_add_and_required() {
        let mut f = FeatureCollection::new();
        let g = f.get_or_add::<ParagraphIdGenerator>();
        assert_eq!(g.next_id().len(), 8);
        assert!(f.contains::<ParagraphIdGenerator>());
        let _ = f.get_required::<ParagraphIdGenerator>();
    }

    #[test]
    #[should_panic(expected = "required feature")]
    fn get_required_panics() {
        let f = FeatureCollection::new();
        let _ = f.get_required::<ParagraphIdGenerator>();
    }

    #[test]
    fn package_capabilities_flags() {
        let mut c = PackageCapabilities::memory_default();
        assert!(c.contains(PackageCapabilities::SAVE));
        assert!(c.contains(PackageCapabilities::CACHED));
        assert!(!c.contains(PackageCapabilities::RELOAD));
        c |= PackageCapabilities::RELOAD;
        assert!(c.contains(PackageCapabilities::RELOAD));
        assert!(c.intersects(PackageCapabilities::SAVE | PackageCapabilities::MALFORMED_URI));
    }

    #[test]
    fn application_type_and_disposable() {
        let t = ApplicationType::WORD | ApplicationType::EXCEL;
        assert!(t.contains(ApplicationType::WORD));
        assert!(t.intersects(ApplicationType::EXCEL));
        assert!(!t.contains(ApplicationType::POWERPOINT));
        assert!(ApplicationType::ALL.contains(ApplicationType::POWERPOINT));

        let count = Arc::new(AtomicUsize::new(0));
        let mut d = DisposableFeature::new();
        let c2 = count.clone();
        d.register(move || {
            c2.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(d.pending_count(), 1);
        d.dispose_all();
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert_eq!(d.pending_count(), 0);

        let main = MainPartFeature::new("rel", "ct", None).with_uri("/word/document.xml");
        assert_eq!(main.part_uri.as_deref(), Some("/word/document.xml"));
        let doc = DocumentTypeFeature::new("WordprocessingDocument");
        assert_eq!(doc.document_type, "WordprocessingDocument");

        let mut track = SchemaTrackingFeature::new();
        track.track_root(crate::element::OpenXmlQualifiedName::new(
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "document",
        ));
        track.track_relationship("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument");
        assert_eq!(track.root_elements.len(), 1);
        assert_eq!(track.relationships.len(), 1);
        assert!(!StrictNamespaceFeature::new(false).found);
        assert!(StrictNamespaceFeature::new(true).found);
    }

    #[test]
    fn relationship_filter_and_programmatic_id() {
        let filters = RelationshipFilterFeature::new();
        filters.add_filter(|b| {
            b.relationship_type = "rewritten".into();
        });
        filters.add_filter(|b| {
            b.id = format!("X{}", b.id);
        });
        assert_eq!(filters.filter_count(), 2);
        let mut b = PackageRelationshipBuilder::new("rId1", "orig", "/word/styles.xml");
        filters.apply(&mut b);
        assert_eq!(b.relationship_type, "rewritten");
        assert_eq!(b.id, "XrId1");

        let ids = ProgrammaticIdentifierFeature::new("R");
        assert_eq!(ids.next_id(), "R00000001");
        assert_eq!(ids.next_id(), "R00000002");
        let factory = PackageFactoryFeature::new("WordprocessingDocument");
        assert_eq!(factory.package_kind, "WordprocessingDocument");
    }
}
