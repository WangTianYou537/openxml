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

/// Whether content types are fixed for the package (C# `IContentTypeFeature`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentTypeFeature {
    /// When true, content types cannot be changed after open (C# `IsConstant`).
    pub is_constant: bool,
}

impl Default for ContentTypeFeature {
    fn default() -> Self {
        Self { is_constant: false }
    }
}

impl ContentTypeFeature {
    pub fn new(is_constant: bool) -> Self {
        Self { is_constant }
    }

    pub fn constant() -> Self {
        Self { is_constant: true }
    }
}

/// Package-level synchronization lock (C# `ILockFeature.SyncLock` shell).
///
/// Holds a mutex used by callers that need to coordinate concurrent package access.
#[derive(Default)]
pub struct LockFeature {
    lock: Mutex<()>,
}

impl std::fmt::Debug for LockFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LockFeature").finish_non_exhaustive()
    }
}

impl LockFeature {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run `f` while holding the package sync lock.
    pub fn with_lock<R>(&self, f: impl FnOnce() -> R) -> R {
        let _g = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        f()
    }
}

/// Registry of loaded parts by URI (C# `IPartsFeature` shell).
///
/// Tracks which part URIs have been registered without owning the full part DOM.
#[derive(Debug, Default, Clone)]
pub struct PartsFeature {
    uris: Vec<String>,
}

impl PartsFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, uri: impl Into<String>) {
        let uri = uri.into();
        if !self.uris.iter().any(|u| u == &uri) {
            self.uris.push(uri);
        }
    }

    pub fn contains(&self, uri: &str) -> bool {
        self.uris.iter().any(|u| u == uri)
    }

    pub fn try_get(&self, uri: &str) -> bool {
        self.contains(uri)
    }

    pub fn uris(&self) -> &[String] {
        &self.uris
    }

    pub fn len(&self) -> usize {
        self.uris.len()
    }

    pub fn is_empty(&self) -> bool {
        self.uris.is_empty()
    }

    pub fn remove(&mut self, uri: &str) -> bool {
        let before = self.uris.len();
        self.uris.retain(|u| u != uri);
        self.uris.len() != before
    }
}

/// Creates parts from relationship types (C# `IPartFactoryFeature` shell).
///
/// Maps relationship type → part name / content type metadata for dynamic part creation.
#[derive(Debug, Default, Clone)]
pub struct PartFactoryFeature {
    /// relationship_type → part type name (e.g. `"ImagePart"`).
    by_relationship: std::collections::HashMap<String, String>,
}

impl PartFactoryFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, relationship_type: impl Into<String>, part_name: impl Into<String>) {
        self.by_relationship
            .insert(relationship_type.into(), part_name.into());
    }

    /// C# `IPartFactoryFeature.Create` — returns registered part name for `relationship_type`.
    pub fn create(&self, relationship_type: &str) -> Option<&str> {
        self.by_relationship
            .get(relationship_type)
            .map(|s| s.as_str())
    }

    pub fn contains(&self, relationship_type: &str) -> bool {
        self.by_relationship.contains_key(relationship_type)
    }

    pub fn len(&self) -> usize {
        self.by_relationship.len()
    }
}

/// Known data-part relationship types (C# `IKnownDataPartFeature`).
#[derive(Debug, Clone)]
pub struct KnownDataPartFeature {
    types: std::collections::HashSet<String>,
}

impl Default for KnownDataPartFeature {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl KnownDataPartFeature {
    pub fn new() -> Self {
        Self {
            types: std::collections::HashSet::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut f = Self::new();
        // Common Office data/media relationship types
        for t in [
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/media",
            "http://schemas.microsoft.com/office/2007/relationships/media",
        ] {
            f.register(t);
        }
        f
    }

    pub fn register(&mut self, relationship_type: impl Into<String>) {
        self.types.insert(relationship_type.into());
    }

    /// C# `IKnownDataPartFeature.IsKnown`.
    pub fn is_known(&self, relationship_type: &str) -> bool {
        self.types.contains(relationship_type)
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }
}

/// Holds package source bytes when opened from a stream (C# `IPackageStreamFeature` shell).
#[derive(Debug, Clone, Default)]
pub struct PackageStreamFeature {
    pub bytes: Option<Vec<u8>>,
}

impl PackageStreamFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: Some(bytes.into()),
        }
    }

    pub fn set_bytes(&mut self, bytes: impl Into<Vec<u8>>) {
        self.bytes = Some(bytes.into());
    }

    pub fn clear(&mut self) {
        self.bytes = None;
    }
}

/// Current package-part URI context (C# `IPackagePartFeature` shell).
#[derive(Debug, Clone, Default)]
pub struct PackagePartFeature {
    pub part_uri: Option<String>,
}

impl PackagePartFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_uri(uri: impl Into<String>) -> Self {
        Self {
            part_uri: Some(uri.into()),
        }
    }

    pub fn set_uri(&mut self, uri: impl Into<String>) {
        self.part_uri = Some(uri.into());
    }

    pub fn clear(&mut self) {
        self.part_uri = None;
    }
}

/// Part URI allocation feature (C# `IPartUriFeature` shell wrapping [`crate::opc::PartUriHelper`]).
#[derive(Debug, Default)]
pub struct PartUriFeature {
    helper: crate::opc::PartUriHelper,
}

impl PartUriFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_helper(helper: crate::opc::PartUriHelper) -> Self {
        Self { helper }
    }

    pub fn helper(&self) -> &crate::opc::PartUriHelper {
        &self.helper
    }

    pub fn helper_mut(&mut self) -> &mut crate::opc::PartUriHelper {
        &mut self.helper
    }

    pub fn reserve(&mut self, uri: &crate::opc::PackUri) {
        self.helper.reserve(uri);
    }

    pub fn is_reserved(&self, uri: &crate::opc::PackUri) -> bool {
        self.helper.is_reserved(uri)
    }
}

/// Registry of package-level data parts (C# `IDataPartsFeature` shell).
#[derive(Debug, Default, Clone)]
pub struct DataPartsFeature {
    /// Part URIs registered as data/media parts.
    uris: Vec<String>,
}

impl DataPartsFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, uri: impl Into<String>) {
        let uri = uri.into();
        if !self.uris.iter().any(|u| u == &uri) {
            self.uris.push(uri);
        }
    }

    pub fn contains(&self, uri: &str) -> bool {
        self.uris.iter().any(|u| u == uri)
    }

    pub fn try_get(&self, uri: &str) -> bool {
        self.contains(uri)
    }

    pub fn remove(&mut self, uri: &str) -> bool {
        let before = self.uris.len();
        self.uris.retain(|u| u != uri);
        self.uris.len() != before
    }

    pub fn uris(&self) -> &[String] {
        &self.uris
    }

    pub fn len(&self) -> usize {
        self.uris.len()
    }

    pub fn is_empty(&self) -> bool {
        self.uris.is_empty()
    }
}

/// Part relationship id → target URI map for a container (C# `IPartRelationshipsFeature` shell).
#[derive(Debug, Default, Clone)]
pub struct PartRelationshipsFeature {
    /// (relationship id, target part URI)
    entries: Vec<(String, String)>,
}

impl PartRelationshipsFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, id: impl Into<String>, part_uri: impl Into<String>) {
        let id = id.into();
        let part_uri = part_uri.into();
        if let Some(e) = self.entries.iter_mut().find(|(i, _)| i == &id) {
            e.1 = part_uri;
        } else {
            self.entries.push((id, part_uri));
        }
    }

    pub fn contains_id(&self, id: &str) -> bool {
        self.entries.iter().any(|(i, _)| i == id)
    }

    pub fn contains_uri(&self, uri: &str) -> bool {
        self.entries.iter().any(|(_, u)| u == uri)
    }

    pub fn try_get(&self, id: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(i, _)| i == id)
            .map(|(_, u)| u.as_str())
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|(i, _)| i != id);
        self.entries.len() != before
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(i, u)| (i.as_str(), u.as_str()))
    }
}

/// Reference relationship registry (C# `IReferenceRelationshipsFeature` shell).
#[derive(Debug, Default, Clone)]
pub struct ReferenceRelationshipsFeature {
    /// (id, relationship_type, target, is_external)
    items: Vec<(String, String, String, bool)>,
}

impl ReferenceRelationshipsFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        id: impl Into<String>,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
        is_external: bool,
    ) {
        let id = id.into();
        if let Some(item) = self.items.iter_mut().find(|(i, _, _, _)| i == &id) {
            *item = (id, relationship_type.into(), target.into(), is_external);
        } else {
            self.items
                .push((id, relationship_type.into(), target.into(), is_external));
        }
    }

    pub fn try_get(&self, id: &str) -> Option<(&str, &str, bool)> {
        self.items
            .iter()
            .find(|(i, _, _, _)| i == id)
            .map(|(_, t, target, ext)| (t.as_str(), target.as_str(), *ext))
    }

    pub fn contains(&self, id: &str) -> bool {
        self.items.iter().any(|(i, _, _, _)| i == id)
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.items.len();
        self.items.retain(|(i, _, _, _)| i != id);
        self.items.len() != before
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, &str, bool)> {
        self.items
            .iter()
            .map(|(i, t, target, e)| (i.as_str(), t.as_str(), target.as_str(), *e))
    }
}

/// Typed part factory by Rust type name (C# `ITypedPartFactoryFeature` shell).
#[derive(Debug, Default, Clone)]
pub struct TypedPartFactoryFeature {
    /// type name → relationship type
    by_type_name: std::collections::HashMap<String, String>,
}

impl TypedPartFactoryFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, type_name: impl Into<String>, relationship_type: impl Into<String>) {
        self.by_type_name
            .insert(type_name.into(), relationship_type.into());
    }

    pub fn create(&self, type_name: &str) -> Option<&str> {
        self.by_type_name.get(type_name).map(|s| s.as_str())
    }

    pub fn contains(&self, type_name: &str) -> bool {
        self.by_type_name.contains_key(type_name)
    }

    pub fn len(&self) -> usize {
        self.by_type_name.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_type_name.is_empty()
    }
}

/// Target path metadata for a part/container (C# `ITargetFeature` shell).
#[derive(Debug, Clone, Default)]
pub struct TargetFeature {
    pub path: String,
    pub extension: String,
    pub name: String,
}

impl TargetFeature {
    pub fn new(
        path: impl Into<String>,
        extension: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            extension: extension.into(),
            name: name.into(),
        }
    }

    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }
}

/// Root element factory by qualified name (C# `IRootElementFeature` shell).
///
/// Maps `"namespace_uri|local_name"` → type name string for element construction.
#[derive(Debug, Default, Clone)]
pub struct RootElementFeature {
    by_qname: std::collections::HashMap<String, String>,
}

impl RootElementFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        namespace_uri: impl Into<String>,
        local_name: impl Into<String>,
        type_name: impl Into<String>,
    ) {
        let key = format!("{}|{}", namespace_uri.into(), local_name.into());
        self.by_qname.insert(key, type_name.into());
    }

    pub fn try_create(&self, namespace_uri: &str, local_name: &str) -> Option<&str> {
        let key = format!("{namespace_uri}|{local_name}");
        self.by_qname.get(&key).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.by_qname.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_qname.is_empty()
    }
}

/// Save callbacks for containers (C# `ISaveFeature` shell).
///
/// Registered hooks receive the container URI (or empty for package-level save).
#[derive(Default)]
pub struct SaveFeature {
    hooks: Mutex<Vec<Box<dyn Fn(&str) + Send + Sync>>>,
}

impl std::fmt::Debug for SaveFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.hooks.lock().map(|g| g.len()).unwrap_or(0);
        f.debug_struct("SaveFeature").field("hooks", &n).finish()
    }
}

impl SaveFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F>(&self, f: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        if let Ok(mut g) = self.hooks.lock() {
            g.push(Box::new(f));
        }
    }

    /// Invoke all save hooks for `container_uri` (C# `ISaveFeature.Save`).
    pub fn save(&self, container_uri: &str) {
        if let Ok(g) = self.hooks.lock() {
            for h in g.iter() {
                h(container_uri);
            }
        }
    }

    pub fn hook_count(&self) -> usize {
        self.hooks.lock().map(|g| g.len()).unwrap_or(0)
    }
}

/// Package feature shell (C# `IPackageFeature` capabilities + reload token).
#[derive(Debug, Clone)]
pub struct PackageFeature {
    pub capabilities: PackageCapabilities,
    /// Times `reload` has been invoked (shell counter).
    pub reload_count: u32,
    pub path: Option<String>,
}

impl Default for PackageFeature {
    fn default() -> Self {
        Self {
            capabilities: PackageCapabilities::CACHED | PackageCapabilities::RELOAD,
            reload_count: 0,
            path: None,
        }
    }
}

impl PackageFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capabilities(capabilities: PackageCapabilities) -> Self {
        Self {
            capabilities,
            ..Default::default()
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn reload(&mut self) {
        self.reload_count = self.reload_count.saturating_add(1);
    }
}

/// Obsolete / alternate → expected namespace URI map + Strict/Transitional helpers
/// (C# `IOpenXmlNamespaceResolver` shell).
#[derive(Debug, Clone)]
pub struct OpenXmlNamespaceResolverFeature {
    /// Obsolete namespace URI → expected URI.
    extended: Vec<(String, String)>,
}

impl Default for OpenXmlNamespaceResolverFeature {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl OpenXmlNamespaceResolverFeature {
    pub fn new() -> Self {
        Self {
            extended: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let extended = [
            (
                "http://schemas.openxmlformats.org/wordprocessingml/2006/3/main",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            ),
            (
                "http://schemas.openxmlformats.org/wordprocessingml/2006/5/main",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            ),
            (
                "http://schemas.openxmlformats.org/wordprocessingml/2006/6/main",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            ),
            (
                "http://schemas.openxmlformats.org/spreadsheetml/2006/5/main",
                "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            ),
            (
                "http://schemas.openxmlformats.org/spreadsheetml/2006/7/main",
                "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            ),
            (
                "http://schemas.openxmlformats.org/presentationml/2006/3/main",
                "http://schemas.openxmlformats.org/presentationml/2006/main",
            ),
            (
                "http://schemas.openxmlformats.org/drawingml/2006/3/main",
                "http://schemas.openxmlformats.org/drawingml/2006/main",
            ),
            (
                "http://schemas.microsoft.com/office/word/2010/11/wordml",
                "http://schemas.microsoft.com/office/word/2012/wordml",
            ),
        ]
        .into_iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();
        Self { extended }
    }

    pub fn register_extended(
        &mut self,
        obsolete: impl Into<String>,
        expected: impl Into<String>,
    ) {
        let obsolete = obsolete.into();
        let expected = expected.into();
        if let Some(e) = self.extended.iter_mut().find(|(o, _)| o == &obsolete) {
            e.1 = expected;
        } else {
            self.extended.push((obsolete, expected));
        }
    }

    pub fn try_get_extended_namespace(&self, uri: &str) -> Option<&str> {
        self.extended
            .iter()
            .find(|(o, _)| o == uri)
            .map(|(_, e)| e.as_str())
    }

    pub fn try_get_transitional_namespace(&self, uri: &str) -> Option<&'static str> {
        crate::namespace_rewrite::to_transitional_namespace(uri)
    }

    pub fn try_get_transitional_relationship(&self, uri: &str) -> Option<&'static str> {
        crate::namespace_rewrite::to_transitional_relationship(uri)
    }

    /// Normalize obsolete then Strict→Transitional (C# `NormalizeNamespace` shell).
    pub fn normalize_namespace(&self, uri: &str) -> String {
        let base = self
            .try_get_extended_namespace(uri)
            .unwrap_or(uri);
        self.try_get_transitional_namespace(base)
            .unwrap_or(base)
            .to_string()
    }

    /// Best-effort version for well-known Office namespaces (C# `GetVersion` subset).
    pub fn get_version(&self, uri: &str) -> crate::file_format::FileFormatVersions {
        use crate::file_format::FileFormatVersions;
        let n = self.normalize_namespace(uri);
        if n.contains("schemas.openxmlformats.org/wordprocessingml/2006")
            || n.contains("schemas.openxmlformats.org/spreadsheetml/2006")
            || n.contains("schemas.openxmlformats.org/presentationml/2006")
            || n.contains("schemas.openxmlformats.org/drawingml/2006")
            || n.contains("schemas.openxmlformats.org/officeDocument/2006")
        {
            return FileFormatVersions::OFFICE2007;
        }
        if n.contains("/2010/") || n.contains("2010/") {
            return FileFormatVersions::OFFICE2010;
        }
        if n.contains("/2012/") || n.contains("2012/") || n.contains("/2013/") {
            return FileFormatVersions::OFFICE2013;
        }
        if n.contains("/2014/") || n.contains("/2016/") {
            return FileFormatVersions::OFFICE2016;
        }
        if n.contains("/2018/") || n.contains("/2019/") {
            return FileFormatVersions::OFFICE2019;
        }
        if n.contains("/2021/") {
            return FileFormatVersions::OFFICE2021;
        }
        FileFormatVersions::NONE
    }
}

/// Cryptographic-style random fill (C# `IRandomNumberGeneratorFeature` shell).
#[derive(Debug, Default)]
pub struct RandomNumberGeneratorFeature;

impl RandomNumberGeneratorFeature {
    pub fn new() -> Self {
        Self
    }

    /// Fill `buf` with random bytes (thread-local LCG shell; not crypto-grade).
    pub fn get_bytes(&self, buf: &mut [u8]) {
        use std::cell::Cell;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        thread_local! {
            static STATE: Cell<u64> = const { Cell::new(0) };
        }
        STATE.with(|s| {
            let mut state = s.get();
            if state == 0 {
                let mut h = DefaultHasher::new();
                (self as *const Self as usize).hash(&mut h);
                std::thread::current().id().hash(&mut h);
                // Mix in a fixed salt so empty hasher state is unlikely.
                0xC0FFEE_u64.hash(&mut h);
                state = h.finish() | 1;
            }
            for b in buf.iter_mut() {
                // Numerical Recipes LCG
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1);
                *b = (state >> 33) as u8;
            }
            s.set(state);
        });
    }

    pub fn next_u32(&self) -> u32 {
        let mut b = [0u8; 4];
        self.get_bytes(&mut b);
        u32::from_le_bytes(b)
    }

    pub fn next_u64(&self) -> u64 {
        let mut b = [0u8; 8];
        self.get_bytes(&mut b);
        u64::from_le_bytes(b)
    }
}

/// Container-level dispose marker (C# `IContainerDisposableFeature` shell).
#[derive(Default)]
pub struct ContainerDisposableFeature {
    hooks: Mutex<Vec<Box<dyn FnOnce() + Send>>>,
}

impl std::fmt::Debug for ContainerDisposableFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.hooks.lock().map(|g| g.len()).unwrap_or(0);
        f.debug_struct("ContainerDisposableFeature")
            .field("hooks", &n)
            .finish()
    }
}

impl ContainerDisposableFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Ok(mut g) = self.hooks.lock() {
            g.push(Box::new(f));
        }
    }

    pub fn dispose(&self) {
        let hooks: Vec<_> = self
            .hooks
            .lock()
            .map(|mut g| std::mem::take(&mut *g))
            .unwrap_or_default();
        for h in hooks.into_iter().rev() {
            h();
        }
    }

    pub fn pending_count(&self) -> usize {
        self.hooks.lock().map(|g| g.len()).unwrap_or(0)
    }
}

/// Part-scoped element event hub (C# `IElementEventFeature` / `PartElementEventArgs` shell).
#[derive(Debug, Clone)]
pub struct PartElementEvent {
    pub event_type: PackageEventType,
    pub part_uri: String,
    pub element_name: String,
    pub parent_name: Option<String>,
}

type ElementListener = Arc<dyn Fn(&PartElementEvent) + Send + Sync>;

#[derive(Clone, Default)]
pub struct ElementEventsFeature {
    listeners: Arc<Mutex<Vec<ElementListener>>>,
}

impl std::fmt::Debug for ElementEventsFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.listeners.lock().map(|g| g.len()).unwrap_or(0);
        f.debug_struct("ElementEventsFeature")
            .field("listeners", &n)
            .finish()
    }
}

impl ElementEventsFeature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<F>(&self, f: F) -> usize
    where
        F: Fn(&PartElementEvent) + Send + Sync + 'static,
    {
        let mut g = self.listeners.lock().expect("element events lock");
        g.push(Arc::new(f));
        g.len() - 1
    }

    pub fn raise(&self, event: PartElementEvent) {
        let listeners = self
            .listeners
            .lock()
            .map(|g| g.clone())
            .unwrap_or_default();
        for l in listeners {
            l(&event);
        }
    }

    pub fn raise_kind(
        &self,
        event_type: PackageEventType,
        part_uri: impl Into<String>,
        element_name: impl Into<String>,
        parent_name: Option<String>,
    ) {
        self.raise(PartElementEvent {
            event_type,
            part_uri: part_uri.into(),
            element_name: element_name.into(),
            parent_name,
        });
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.lock().map(|g| g.len()).unwrap_or(0)
    }
}

/// Package initializer callbacks (C# `IPackageInitializer` shell).
///
/// Runs registered hooks after a package is constructed (builder path).
#[derive(Default)]
pub struct PackageInitializerFeature {
    hooks: Mutex<Vec<Box<dyn FnOnce() + Send>>>,
}

impl std::fmt::Debug for PackageInitializerFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.hooks.lock().map(|g| g.len()).unwrap_or(0);
        f.debug_struct("PackageInitializerFeature")
            .field("hooks", &n)
            .finish()
    }
}

impl PackageInitializerFeature {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an initializer callback (C# `IPackageInitializer.Initialize` shell).
    pub fn register<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Ok(mut g) = self.hooks.lock() {
            g.push(Box::new(f));
        }
    }

    pub fn run_all(&self) {
        let hooks: Vec<_> = self
            .hooks
            .lock()
            .map(|mut g| std::mem::take(&mut *g))
            .unwrap_or_default();
        for h in hooks {
            h();
        }
    }

    pub fn pending_count(&self) -> usize {
        self.hooks.lock().map(|g| g.len()).unwrap_or(0)
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

    #[test]
    fn content_type_lock_and_parts_features() {
        let ct = ContentTypeFeature::constant();
        assert!(ct.is_constant);
        assert!(!ContentTypeFeature::default().is_constant);

        let lock = LockFeature::new();
        let n = Arc::new(AtomicUsize::new(0));
        let c = n.clone();
        lock.with_lock(|| {
            c.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(n.load(Ordering::SeqCst), 1);

        let mut parts = PartsFeature::new();
        parts.add("/word/document.xml");
        parts.add("/word/document.xml");
        assert_eq!(parts.len(), 1);
        assert!(parts.contains("/word/document.xml"));
        assert!(parts.remove("/word/document.xml"));
        assert!(parts.is_empty());
    }

    #[test]
    fn part_factory_and_known_data_part() {
        let mut factory = PartFactoryFeature::new();
        factory.register(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            "ImagePart",
        );
        assert_eq!(
            factory.create(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
            ),
            Some("ImagePart")
        );
        assert!(factory.contains(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
        ));

        let known = KnownDataPartFeature::with_defaults();
        assert!(known.is_known(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
        ));
        assert!(!known.is_known("http://example/unknown"));
    }

    #[test]
    fn package_stream_part_and_initializer() {
        let mut stream = PackageStreamFeature::from_bytes(b"zip-bytes");
        assert_eq!(stream.bytes.as_deref(), Some(&b"zip-bytes"[..]));
        stream.clear();
        assert!(stream.bytes.is_none());

        let mut part = PackagePartFeature::with_uri("/word/document.xml");
        assert_eq!(part.part_uri.as_deref(), Some("/word/document.xml"));
        part.clear();
        assert!(part.part_uri.is_none());

        let fired = Arc::new(AtomicUsize::new(0));
        let init = PackageInitializerFeature::new();
        let f = fired.clone();
        init.register(move || {
            f.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(init.pending_count(), 1);
        init.run_all();
        assert_eq!(fired.load(Ordering::SeqCst), 1);
        assert_eq!(init.pending_count(), 0);
    }

    #[test]
    fn part_uri_feature_reserves() {
        let mut f = PartUriFeature::new();
        let uri = crate::opc::PackUri::new("/word/styles.xml");
        assert!(!f.is_reserved(&uri));
        f.reserve(&uri);
        assert!(f.is_reserved(&uri));
    }

    #[test]
    fn data_parts_part_rel_ref_rel_typed_factory() {
        let mut dp = DataPartsFeature::new();
        dp.add("/media/image1.png");
        dp.add("/media/image1.png");
        assert_eq!(dp.len(), 1);
        assert!(dp.contains("/media/image1.png"));
        assert!(dp.try_get("/media/image1.png"));
        assert!(dp.remove("/media/image1.png"));
        assert!(dp.is_empty());

        let mut pr = PartRelationshipsFeature::new();
        pr.add("rId1", "/word/styles.xml");
        pr.add("rId1", "/word/styles2.xml");
        assert_eq!(pr.try_get("rId1"), Some("/word/styles2.xml"));
        assert!(pr.contains_id("rId1"));
        assert!(pr.contains_uri("/word/styles2.xml"));
        assert!(pr.remove("rId1"));
        assert!(pr.is_empty());

        let mut rr = ReferenceRelationshipsFeature::new();
        rr.add("rId9", "http://rel/hyperlink", "https://example.com", true);
        assert_eq!(
            rr.try_get("rId9"),
            Some(("http://rel/hyperlink", "https://example.com", true))
        );
        assert!(rr.contains("rId9"));
        assert!(rr.remove("rId9"));
        assert!(rr.is_empty());

        let mut tf = TypedPartFactoryFeature::new();
        tf.register("ImagePart", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image");
        assert_eq!(
            tf.create("ImagePart"),
            Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")
        );
        assert!(tf.contains("ImagePart"));
        assert!(!tf.is_empty());
    }

    #[test]
    fn target_root_save_package_features() {
        let t = TargetFeature::new("/word", "xml", "document");
        assert_eq!(t.path, "/word");
        assert_eq!(t.extension, "xml");
        assert_eq!(t.name, "document");

        let mut root = RootElementFeature::new();
        root.register(
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "document",
            "Document",
        );
        assert_eq!(
            root.try_create(
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "document"
            ),
            Some("Document")
        );
        assert!(root
            .try_create("http://other", "document")
            .is_none());

        let saved = Arc::new(Mutex::new(Vec::<String>::new()));
        let save = SaveFeature::new();
        let s = saved.clone();
        save.register(move |uri| {
            if let Ok(mut g) = s.lock() {
                g.push(uri.to_string());
            }
        });
        assert_eq!(save.hook_count(), 1);
        save.save("/word/document.xml");
        save.save("");
        let got = saved.lock().unwrap().clone();
        assert_eq!(got, vec!["/word/document.xml".to_string(), String::new()]);

        let mut pkg = PackageFeature::with_capabilities(
            PackageCapabilities::SAVE | PackageCapabilities::RELOAD,
        )
        .with_path("doc.docx");
        assert!(pkg.capabilities.contains(PackageCapabilities::SAVE));
        assert_eq!(pkg.path.as_deref(), Some("doc.docx"));
        pkg.reload();
        pkg.reload();
        assert_eq!(pkg.reload_count, 2);
    }

    #[test]
    fn namespace_resolver_random_element_events() {
        let r = OpenXmlNamespaceResolverFeature::with_defaults();
        assert_eq!(
            r.try_get_extended_namespace(
                "http://schemas.openxmlformats.org/wordprocessingml/2006/3/main"
            ),
            Some("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
        );
        assert_eq!(
            r.get_version("http://schemas.openxmlformats.org/wordprocessingml/2006/main"),
            crate::file_format::FileFormatVersions::OFFICE2007
        );
        assert_eq!(
            r.get_version("http://schemas.microsoft.com/office/word/2010/wordml"),
            crate::file_format::FileFormatVersions::OFFICE2010
        );
        assert!(r
            .try_get_transitional_namespace(
                "http://purl.oclc.org/ooxml/wordprocessingml/main"
            )
            .is_some());

        let rng = RandomNumberGeneratorFeature::new();
        let mut a = [0u8; 16];
        let mut b = [0u8; 16];
        rng.get_bytes(&mut a);
        rng.get_bytes(&mut b);
        assert_ne!(a, [0u8; 16]);
        // Not required to differ, but next_u64 should be non-zero with high probability.
        assert_ne!(rng.next_u64(), 0);

        let cd = ContainerDisposableFeature::new();
        let n = Arc::new(AtomicUsize::new(0));
        let c = n.clone();
        cd.register(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(cd.pending_count(), 1);
        cd.dispose();
        assert_eq!(n.load(Ordering::SeqCst), 1);
        assert_eq!(cd.pending_count(), 0);

        let ee = ElementEventsFeature::new();
        let hits = Arc::new(AtomicUsize::new(0));
        let h = hits.clone();
        ee.subscribe(move |e| {
            if e.element_name == "w:p" {
                h.fetch_add(1, Ordering::SeqCst);
            }
        });
        ee.raise_kind(
            PackageEventType::Added,
            "/word/document.xml",
            "w:p",
            Some("w:body".into()),
        );
        assert_eq!(hits.load(Ordering::SeqCst), 1);
        assert_eq!(ee.listener_count(), 1);
    }
}
