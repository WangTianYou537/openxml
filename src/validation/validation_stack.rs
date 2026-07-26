//! Validation stack / state shells (C# `ValidationStack`, `StateManager`, `ValidationErrorEventArgs`).

use super::ValidationError;
use std::any::Any;
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

    pub fn with_package(package_uri: impl Into<String>) -> Self {
        Self {
            package_uri: Some(package_uri.into()),
            ..Default::default()
        }
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

    pub fn with_value(value: impl Into<String>) -> Self {
        Self {
            simple_value: Some(value.into()),
            ..Default::default()
        }
    }

    pub fn with_property(
        property_name: impl Into<String>,
        value: Option<impl Into<String>>,
        is_attribute: bool,
    ) -> Self {
        Self {
            property_name: Some(property_name.into()),
            simple_value: value.map(Into::into),
            is_attribute,
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn copy_from(&mut self, other: Option<&ValidationElement>) {
        if let Some(other) = other {
            self.package_uri = other.package_uri.clone();
            self.part_uri = other.part_uri.clone();
            self.element_path = other.element_path.clone();
            self.property_name = other.property_name.clone();
            self.simple_value = other.simple_value.clone();
            self.is_attribute = other.is_attribute;
        }
    }
}

/// Stack of validation frames (C# `ValidationStack`).
#[derive(Debug, Default)]
pub struct ValidationStack {
    elements: Vec<ValidationElement>,
    available: Vec<ValidationElement>,
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

    /// Push a frame, inheriting the current frame before applying its explicit fields.
    pub fn push(&mut self, frame: ValidationElement) {
        let mut updated = self.available.pop().unwrap_or_default();
        updated.copy_from(self.current());
        if frame.package_uri.is_some() {
            updated.package_uri = frame.package_uri;
        }
        if frame.part_uri.is_some() {
            updated.part_uri = frame.part_uri;
        }
        if frame.element_path.is_some() {
            updated.element_path = frame.element_path;
        }
        if frame.property_name.is_some() {
            updated.property_name = frame.property_name;
            updated.simple_value = frame.simple_value;
            updated.is_attribute = frame.is_attribute;
        } else if frame.simple_value.is_some() {
            updated.simple_value = frame.simple_value;
        }
        self.elements.push(updated);
    }

    pub fn push_package(&mut self, package_uri: impl Into<String>) {
        self.push(ValidationElement::with_package(package_uri));
    }

    pub fn push_element_path(&mut self, path: impl Into<String>) {
        self.push(ValidationElement::with_element_path(path));
    }

    pub fn push_part(&mut self, part_uri: impl Into<String>) {
        self.push(ValidationElement::with_part(part_uri));
    }

    pub fn push_value(&mut self, value: impl Into<String>) {
        let mut frame = self.available.pop().unwrap_or_default();
        frame.copy_from(self.current());
        frame.simple_value = Some(value.into());
        self.elements.push(frame);
    }

    pub fn push_property(
        &mut self,
        property_name: impl Into<String>,
        value: Option<impl Into<String>>,
        is_attribute: bool,
    ) {
        let mut frame = self.available.pop().unwrap_or_default();
        frame.copy_from(self.current());
        frame.property_name = Some(property_name.into());
        frame.simple_value = value.map(Into::into);
        frame.is_attribute = is_attribute;
        self.elements.push(frame);
    }

    fn restore_depth(&mut self, depth: usize) {
        while self.elements.len() > depth {
            let mut frame = self.elements.pop().expect("stack depth checked");
            frame.clear();
            self.available.push(frame);
        }
    }

    pub fn with_frame<R, F>(&mut self, frame: ValidationElement, callback: F) -> R
    where
        F: FnOnce(&mut ValidationStack) -> R,
    {
        let depth = self.depth();
        self.push(frame);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(self)));
        self.restore_depth(depth);
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    pub fn with_value<R, F>(&mut self, value: impl Into<String>, callback: F) -> R
    where
        F: FnOnce(&mut ValidationStack) -> R,
    {
        let value = value.into();
        self.with_frame(ValidationElement::with_value(value), callback)
    }

    pub fn with_property<R, F>(
        &mut self,
        property_name: impl Into<String>,
        value: Option<impl Into<String>>,
        is_attribute: bool,
        callback: F,
    ) -> R
    where
        F: FnOnce(&mut ValidationStack) -> R,
    {
        let property_name = property_name.into();
        let value = value.map(Into::into);
        let depth = self.depth();
        self.push_property(property_name, value, is_attribute);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(self)));
        self.restore_depth(depth);
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    pub fn pop(&mut self) -> Option<ValidationElement> {
        let mut frame = self.elements.pop()?;
        let popped = frame.clone();
        frame.clear();
        self.available.push(frame);
        Some(popped)
    }

    pub fn clear(&mut self) {
        while let Some(mut frame) = self.elements.pop() {
            frame.clear();
            self.available.push(frame);
        }
    }
}

/// Per-pass typed cache (C# `StateManager` shell).
#[derive(Default)]
pub struct StateManager {
    state: HashMap<String, Box<dyn Any + Send>>,
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

    /// Get or insert a value for `key`, panicking when an existing value has another type.
    pub fn get_or_create<T, F>(&mut self, key: impl Into<String>, factory: F) -> &T
    where
        T: Any + Send + 'static,
        F: FnOnce() -> T,
    {
        let key = key.into();
        if !self.state.contains_key(&key) {
            self.state.insert(key.clone(), Box::new(factory()));
        }
        self.state
            .get(&key)
            .and_then(|value| value.downcast_ref::<T>())
            .unwrap_or_else(|| {
                panic!(
                    "state value for key `{key}` has incorrect type; expected `{}`",
                    std::any::type_name::<T>()
                )
            })
    }

    pub fn get<T: Any + Send + 'static>(&self, key: &str) -> Option<&T> {
        self.state
            .get(key)
            .and_then(|value| value.downcast_ref::<T>())
    }

    pub fn get_mut<T: Any + Send + 'static>(&mut self, key: &str) -> Option<&mut T> {
        self.state
            .get_mut(key)
            .and_then(|value| value.downcast_mut::<T>())
    }

    pub fn insert<T: Any + Send + 'static>(
        &mut self,
        key: impl Into<String>,
        value: T,
    ) -> Option<T> {
        self.replace(key, value)
    }

    pub fn replace<T: Any + Send + 'static>(
        &mut self,
        key: impl Into<String>,
        value: T,
    ) -> Option<T> {
        let key = key.into();
        if let Some(previous) = self.state.get(&key) {
            assert!(
                previous.is::<T>(),
                "state value for key `{key}` has incorrect type; expected `{}`",
                std::any::type_name::<T>()
            );
        }
        self.state
            .insert(key, Box::new(value))
            .and_then(|previous| previous.downcast::<T>().ok())
            .map(|previous| *previous)
    }

    pub fn remove<T: Any + Send + 'static>(&mut self, key: &str) -> Option<T> {
        let value = self.state.get(key)?;
        assert!(
            value.is::<T>(),
            "state value for key `{key}` has incorrect type; expected `{}`",
            std::any::type_name::<T>()
        );
        self.state
            .remove(key)
            .and_then(|value| value.downcast::<T>().ok())
            .map(|value| *value)
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
        let mut stack = ValidationStack::new();
        assert!(stack.is_empty());
        stack.push_package("package");
        stack.push_part("/word/document.xml");
        stack.push_property("w:val", Some("one"), true);
        stack.push_element_path("/w:document[1]/w:body[1]");

        let current = stack.current().unwrap();
        assert_eq!(current.package_uri.as_deref(), Some("package"));
        assert_eq!(current.part_uri.as_deref(), Some("/word/document.xml"));
        assert_eq!(
            current.element_path.as_deref(),
            Some("/w:document[1]/w:body[1]")
        );
        assert_eq!(current.property_name.as_deref(), Some("w:val"));
        assert_eq!(current.simple_value.as_deref(), Some("one"));
        assert!(current.is_attribute);

        let popped = stack.pop().unwrap();
        assert_eq!(
            popped.element_path.as_deref(),
            Some("/w:document[1]/w:body[1]")
        );
        assert_eq!(stack.depth(), 3);
        stack.clear();
        assert!(stack.is_empty());

        stack.push_value("reused");
        let current = stack.current().unwrap();
        assert_eq!(current.simple_value.as_deref(), Some("reused"));
        assert!(current.package_uri.is_none());
        assert!(current.property_name.is_none());
        assert!(!current.is_attribute);
    }

    #[test]
    fn validation_stack_scopes_restore_depth() {
        let mut stack = ValidationStack::new();
        stack.push_part("/word/document.xml");
        let result = stack.with_value("outer", |stack| {
            assert_eq!(stack.depth(), 2);
            stack.push_element_path("/w:document[1]");
            assert_eq!(stack.depth(), 3);
            42
        });
        assert_eq!(result, 42);
        assert_eq!(stack.depth(), 1);
        assert!(stack.current().unwrap().simple_value.is_none());

        stack.with_value("inherited", |stack| {
            stack.with_frame(
                ValidationElement::with_property("w:missing", None::<String>, true),
                |stack| {
                    let current = stack.current().unwrap();
                    assert_eq!(current.property_name.as_deref(), Some("w:missing"));
                    assert!(current.simple_value.is_none());
                },
            );
        });

        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            stack.with_property("w:val", Some("bad"), true, |stack| {
                stack.push_value("nested");
                panic!("validation failed");
            });
        }));
        assert!(panic.is_err());
        assert_eq!(stack.depth(), 1);
        let current = stack.current().unwrap();
        assert_eq!(current.part_uri.as_deref(), Some("/word/document.xml"));
        assert!(current.property_name.is_none());
        assert!(current.simple_value.is_none());
    }

    #[test]
    fn validation_element_clear_resets_all_fields() {
        let mut frame = ValidationElement {
            package_uri: Some("package".into()),
            part_uri: Some("part".into()),
            element_path: Some("element".into()),
            is_attribute: true,
            property_name: Some("property".into()),
            simple_value: Some("value".into()),
        };
        frame.clear();
        assert!(frame.package_uri.is_none());
        assert!(frame.part_uri.is_none());
        assert!(frame.element_path.is_none());
        assert!(!frame.is_attribute);
        assert!(frame.property_name.is_none());
        assert!(frame.simple_value.is_none());
    }

    #[test]
    fn state_manager_get_or_create() {
        let mut state = StateManager::new();
        assert_eq!(*state.get_or_create("k", || 42u32), 42);
        assert_eq!(*state.get_or_create("k", || 99u32), 42);
        assert_eq!(state.get::<u32>("k"), Some(&42));

        *state.get_mut::<u32>("k").unwrap() += 1;
        assert_eq!(state.replace("k", 7u32), Some(43));
        assert_eq!(state.insert("k2", "hello".to_string()), None);
        assert_eq!(state.get::<String>("k2").map(String::as_str), Some("hello"));
        assert_eq!(state.remove::<u32>("k"), Some(7));
        assert!(state.get::<u32>("k").is_none());
        assert_eq!(state.len(), 1);
        state.clear();
        assert!(state.is_empty());
    }

    #[test]
    fn state_manager_rejects_a_different_value_type_for_the_same_key() {
        let mut state = StateManager::new();
        state.insert("k", 42u32);
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            state.get_or_create("k", || String::from("wrong"));
        }));
        assert!(panic.is_err());
        assert_eq!(state.len(), 1);
        assert_eq!(state.get::<u32>("k"), Some(&42));
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
