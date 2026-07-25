//! Element load / mutation context (C# `OpenXmlElementContext` shell).
//!
//! Tracks Markup Compatibility settings, load mode, and optional mutation
//! listeners when building or editing element trees.

use crate::file_format::FileFormatVersions;
use crate::packaging::{MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings};
use std::sync::{Arc, Mutex};

/// How deeply to materialize the DOM when loading (C# `OpenXmlLoadMode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OpenXmlLoadMode {
    /// Fully parse the element tree.
    #[default]
    Full,
    /// Parse only outer layers; leave deep content as raw XML when supported.
    Lazy,
}

/// Event payload for element tree mutations (C# `ElementEventArgs` shell).
#[derive(Debug, Clone)]
pub struct ElementEvent {
    pub kind: ElementEventKind,
    /// Qualified-ish path description for diagnostics.
    pub element_name: String,
    pub parent_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementEventKind {
    Inserting,
    Inserted,
    Removing,
    Removed,
    Changing,
    Changed,
}

type Listener = Arc<dyn Fn(&ElementEvent) + Send + Sync>;

/// Loading / editing context attached to a package or document session.
#[derive(Clone)]
pub struct OpenXmlElementContext {
    pub mc_settings: MarkupCompatibilityProcessSettings,
    pub load_mode: OpenXmlLoadMode,
    /// AlternateContent nesting depth (C# `ACBlockLevel`).
    pub ac_block_level: u32,
    listeners: Arc<Mutex<Vec<Listener>>>,
}

impl std::fmt::Debug for OpenXmlElementContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenXmlElementContext")
            .field("load_mode", &self.load_mode)
            .field("ac_block_level", &self.ac_block_level)
            .field("mc_mode", &self.mc_settings.mode)
            .finish()
    }
}

impl Default for OpenXmlElementContext {
    fn default() -> Self {
        Self {
            mc_settings: MarkupCompatibilityProcessSettings::default(),
            load_mode: OpenXmlLoadMode::Full,
            ac_block_level: 0,
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl OpenXmlElementContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mc(mut self, settings: MarkupCompatibilityProcessSettings) -> Self {
        self.mc_settings = settings;
        self
    }

    pub fn with_load_mode(mut self, mode: OpenXmlLoadMode) -> Self {
        self.load_mode = mode;
        self
    }

    pub fn with_office_version(mut self, version: FileFormatVersions) -> Self {
        self.mc_settings.target_file_format_versions = version;
        self
    }

    pub fn process_mc(&self) -> bool {
        self.mc_settings.mode != MarkupCompatibilityProcessMode::NoProcess
    }

    pub fn subscribe<F>(&self, f: F) -> usize
    where
        F: Fn(&ElementEvent) + Send + Sync + 'static,
    {
        let mut g = self.listeners.lock().expect("element context lock");
        g.push(Arc::new(f));
        g.len() - 1
    }

    pub fn raise(&self, event: ElementEvent) {
        let g = self.listeners.lock().expect("element context lock");
        for l in g.iter() {
            l(&event);
        }
    }

    pub fn raise_kind(
        &self,
        kind: ElementEventKind,
        element_name: impl Into<String>,
        parent_name: Option<String>,
    ) {
        self.raise(ElementEvent {
            kind,
            element_name: element_name.into(),
            parent_name,
        });
    }

    pub fn push_ac_block(&mut self) {
        self.ac_block_level = self.ac_block_level.saturating_add(1);
    }

    pub fn pop_ac_block(&mut self) {
        self.ac_block_level = self.ac_block_level.saturating_sub(1);
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.lock().map(|g| g.len()).unwrap_or(0)
    }
}

/// Feature-bag key: store context on a package via `features.set(OpenXmlElementContext::new())`.
pub type ElementContextFeature = OpenXmlElementContext;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn context_events_fire() {
        let ctx = OpenXmlElementContext::new().with_load_mode(OpenXmlLoadMode::Lazy);
        let n = Arc::new(AtomicUsize::new(0));
        let c = n.clone();
        ctx.subscribe(move |e| {
            if e.kind == ElementEventKind::Inserted {
                c.fetch_add(1, Ordering::SeqCst);
            }
        });
        ctx.raise_kind(ElementEventKind::Inserted, "w:p", Some("w:body".into()));
        assert_eq!(n.load(Ordering::SeqCst), 1);
        assert_eq!(ctx.load_mode, OpenXmlLoadMode::Lazy);
    }
}
