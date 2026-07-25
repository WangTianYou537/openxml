//! Validation stack / state shells (C# `ValidationStack`, `StateManager`, `ValidationErrorEventArgs`).

use super::ValidationError;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// One frame on the validation stack (C# `ValidationElement` subset).
#[derive(Debug, Clone, Default)]
pub struct ValidationElement {
    pub package_uri: Option<String>,
    pub part_uri: Option<String>,
    pub element_path: Option<String>,
    pub is_attribute: bool,
    pub property_name: Option<String>,
    pub simple_value: Option<String>,
}

impl ValidationElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_part(part_uri: impl Into<String>) -> Self {
        Self {
            part_uri: Some(part_uri.into()),
            ..Default::default()
        }
    }

    pub fn with_element_path(path: impl Into<String>) -> Self {
        Self {
            element_path: Some(path.into()),
            ..Default::default()
        }
    }

    pub fn copy_from(&mut self, other: Option<&ValidationElement>) {
        if let Some(o) = other {
            if self.package_uri.is_none() {
                self.package_uri = o.package_uri.clone();
            }
            if self.part_uri.is_none() {
                self.part_uri = o.part_uri.clone();
            }
            if self.element_path.is_none() {
                self.element_path = o.element_path.clone();
            }
        }
    }
}

/// Stack of validation frames (C# `ValidationStack`).
#[derive(Debug, Default)]
pub struct ValidationStack {
    elements: Vec<ValidationElement>,
}

impl ValidationStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> Option<&ValidationElement> {
        self.elements.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut ValidationElement> {
        self.elements.last_mut()
    }

    pub fn depth(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Push a frame, inheriting unset fields from the current top (C# `Push`).
    pub fn push(&mut self, mut frame: ValidationElement) {
        frame.copy_from(self.current());
        self.elements.push(frame);
    }

    pub fn push_element_path(&mut self, path: impl Into<String>) {
        self.push(ValidationElement::with_element_path(path));
    }

    pub fn push_part(&mut self, part_uri: impl Into<String>) {
        self.push(ValidationElement::with_part(part_uri));
    }

    pub fn pop(&mut self) -> Option<ValidationElement> {
        self.elements.pop()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

/// Per-pass typed cache (C# `StateManager` shell).
#[derive(Default)]
pub struct StateManager {
    state: HashMap<(TypeId, String), Box<dyn Any + Send>>,
}

impl std::fmt::Debug for StateManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateManager")
            .field("entries", &self.state.len())
            .finish()
    }
}

impl StateManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    /// Get or insert a value for `key` (string key + type), creating via `factory` when missing.
    pub fn get_or_create<T, F>(&mut self, key: impl Into<String>, factory: F) -> &T
    where
        T: Any + Send + 'static,
        F: FnOnce() -> T,
    {
        let key = key.into();
        let tid = TypeId::of::<T>();
        if !self.state.contains_key(&(tid, key.clone())) {
            self.state.insert((tid, key.clone()), Box::new(factory()));
        }
        self.state
            .get(&(tid, key))
            .and_then(|b| b.downcast_ref::<T>())
            .expect("type just inserted")
    }

    pub fn get<T: Any + Send + 'static>(&self, key: &str) -> Option<&T> {
        let tid = TypeId::of::<T>();
        self.state
            .get(&(tid, key.to_string()))
            .and_then(|b| b.downcast_ref::<T>())
    }

    pub fn insert<T: Any + Send + 'static>(&mut self, key: impl Into<String>, value: T) {
        self.state
            .insert((TypeId::of::<T>(), key.into()), Box::new(value));
    }
}

/// Validation error event args (C# `ValidationErrorEventArgs`).
#[derive(Debug, Clone)]
pub struct ValidationErrorEventArgs {
    pub validation_error: ValidationError,
}

impl ValidationErrorEventArgs {
    pub fn new(error: ValidationError) -> Self {
        Self {
            validation_error: error,
        }
    }

    pub fn validation_error(&self) -> &ValidationError {
        &self.validation_error
    }

    pub fn set_validation_error(&mut self, error: ValidationError) {
        self.validation_error = error;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::ValidationError;

    #[test]
    fn validation_stack_push_pop_inherits() {
        let mut s = ValidationStack::new();
        assert!(s.is_empty());
        s.push_part("/word/document.xml");
        assert_eq!(s.depth(), 1);
        assert_eq!(
            s.current().and_then(|e| e.part_uri.as_deref()),
            Some("/word/document.xml")
        );
        s.push_element_path("/w:document[1]/w:body[1]");
        assert_eq!(s.depth(), 2);
        // child inherits part_uri
        assert_eq!(
            s.current().and_then(|e| e.part_uri.as_deref()),
            Some("/word/document.xml")
        );
        assert_eq!(
            s.current().and_then(|e| e.element_path.as_deref()),
            Some("/w:document[1]/w:body[1]")
        );
        s.pop();
        assert_eq!(s.depth(), 1);
        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn state_manager_get_or_create() {
        let mut m = StateManager::new();
        let v1 = m.get_or_create("k", || 42u32).clone();
        assert_eq!(v1, 42);
        let v2 = m.get_or_create("k", || 99u32).clone();
        assert_eq!(v2, 42); // factory not re-run
        assert_eq!(m.get::<u32>("k"), Some(&42));
        m.insert("k2", "hello".to_string());
        assert_eq!(m.get::<String>("k2").map(|s| s.as_str()), Some("hello"));
        assert_eq!(m.len(), 2);
        m.clear();
        assert!(m.is_empty());
    }

    #[test]
    fn validation_error_event_args() {
        let e = ValidationError::with_id("/a", "Sch_X", "bad");
        let mut args = ValidationErrorEventArgs::new(e.clone());
        assert_eq!(args.validation_error().id(), Some("Sch_X"));
        args.set_validation_error(ValidationError::new("/b", "y"));
        assert_eq!(args.validation_error().path, "/b");
    }
}
