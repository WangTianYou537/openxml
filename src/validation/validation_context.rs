//! Validation run context (C# `ValidationContext` shell).

use super::{ValidationCache, ValidationError, ValidationSettings};
use crate::file_format::FileFormatVersions;

/// Mutable state for a validation pass (C# `ValidationContext` subset).
#[derive(Debug)]
pub struct ValidationContext {
    pub settings: ValidationSettings,
    pub cache: ValidationCache,
    pub errors: Vec<ValidationError>,
    /// When true, particle validators may record expected children (C# `CollectExpectedChildren`).
    pub collect_expected_children: bool,
    expected_children: Vec<String>,
    /// Markup Compatibility context for the current pass (C# `MCContext` shell).
    pub mc_context: Option<crate::markup_compatibility::McContext>,
    /// Current element XPath / path for [`create_error`](Self::create_error).
    pub current_path: String,
    /// Validation element stack (C# `ValidationStack`).
    pub stack: super::ValidationStack,
    /// Per-pass typed state cache (C# `StateManager`).
    pub state: super::StateManager,
}

impl ValidationContext {
    pub fn new(settings: ValidationSettings) -> Self {
        let cache = ValidationCache::new(settings.file_format);
        Self {
            settings,
            cache,
            errors: Vec::new(),
            collect_expected_children: false,
            expected_children: Vec::new(),
            mc_context: None,
            current_path: String::new(),
            stack: super::ValidationStack::new(),
            state: super::StateManager::new(),
        }
    }

    pub fn with_file_format(version: FileFormatVersions) -> Self {
        Self::new(ValidationSettings::new(version))
    }

    pub fn file_format(&self) -> FileFormatVersions {
        self.settings.file_format
    }

    pub fn valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn clear(&mut self) {
        self.errors.clear();
        self.expected_children.clear();
        self.current_path.clear();
        self.stack.clear();
        self.state.clear();
    }

    pub fn stack(&self) -> &super::ValidationStack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut super::ValidationStack {
        &mut self.stack
    }

    pub fn state(&self) -> &super::StateManager {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut super::StateManager {
        &mut self.state
    }

    pub fn set_current_path(&mut self, path: impl Into<String>) {
        self.current_path = path.into();
    }

    pub fn current_path(&self) -> &str {
        &self.current_path
    }

    pub fn set_mc_context(&mut self, mc: crate::markup_compatibility::McContext) {
        self.mc_context = Some(mc);
    }

    pub fn clear_mc_context(&mut self) {
        self.mc_context = None;
    }

    pub fn mc_context(&self) -> Option<&crate::markup_compatibility::McContext> {
        self.mc_context.as_ref()
    }

    pub fn max_number_of_errors(&self) -> usize {
        self.settings.max_number_of_errors
    }

    /// C# `ValidationContext.CreateError` shell.
    pub fn create_error(
        &mut self,
        id: &str,
        error_type: super::ValidationErrorType,
        description: impl AsRef<str>,
    ) -> bool {
        let path = if self.current_path.is_empty() {
            String::new()
        } else {
            self.current_path.clone()
        };
        self.add_error(
            super::ValidationError::with_id(path, id, description.as_ref())
                .with_error_type(error_type),
        )
    }

    /// Add a schema error with id (C# `AddError` convenience).
    pub fn add_error_with_id(
        &mut self,
        path: impl Into<String>,
        id: &str,
        description: impl AsRef<str>,
    ) -> bool {
        self.add_error(super::ValidationError::with_id(path, id, description.as_ref()))
    }

    /// True when the max-error budget is exhausted (C# `CheckIfCancelled` error-count half).
    pub fn check_max_errors(&self) -> bool {
        let max = self.settings.max_number_of_errors;
        max > 0 && self.errors.len() >= max
    }

    pub fn add_error(&mut self, error: ValidationError) -> bool {
        if self.check_max_errors() {
            return false;
        }
        self.errors.push(error);
        true
    }

    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    pub fn into_errors(self) -> Vec<ValidationError> {
        self.errors
    }

    pub fn push_expected_child(&mut self, name: impl Into<String>) {
        if self.collect_expected_children {
            self.expected_children.push(name.into());
        }
    }

    pub fn expected_children(&self) -> &[String] {
        &self.expected_children
    }

    pub fn clear_expected_children(&mut self) {
        self.expected_children.clear();
    }

    pub fn cache(&self) -> &ValidationCache {
        &self.cache
    }

    pub fn cache_mut(&mut self) -> &mut ValidationCache {
        &mut self.cache
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new(ValidationSettings::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_error_budget() {
        let mut ctx = ValidationContext::new(
            ValidationSettings::new(FileFormatVersions::OFFICE2010).with_max_number_of_errors(2),
        );
        assert!(ctx.valid());
        assert!(ctx.add_error(ValidationError::new("a", "e1")));
        assert!(ctx.add_error(ValidationError::new("b", "e2")));
        assert!(!ctx.add_error(ValidationError::new("c", "e3")));
        assert_eq!(ctx.errors().len(), 2);
        assert!(ctx.check_max_errors());
        ctx.clear();
        assert!(ctx.valid());
    }

    #[test]
    fn create_error_and_mc_context() {
        let mut ctx = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        ctx.set_current_path("/w:document[1]");
        assert!(ctx.create_error(
            "Sch_InvalidElementContentExpectingComplexType",
            crate::validation::ValidationErrorType::Schema,
            "unexpected child",
        ));
        assert_eq!(ctx.errors().len(), 1);
        assert_eq!(
            ctx.errors()[0].id(),
            Some("Sch_InvalidElementContentExpectingComplexType")
        );
        assert_eq!(ctx.errors()[0].path, "/w:document[1]");
        assert_eq!(
            ctx.errors()[0].error_type(),
            crate::validation::ValidationErrorType::Schema
        );
        assert_eq!(ctx.max_number_of_errors(), ValidationSettings::DEFAULT_MAX_ERRORS);

        let mc = crate::markup_compatibility::McContext::new();
        ctx.set_mc_context(mc);
        assert!(ctx.mc_context().is_some());
        ctx.clear_mc_context();
        assert!(ctx.mc_context().is_none());

        assert!(ctx.add_error_with_id("/x", "Sem_UniqueAttributeValue", "dup"));
        assert_eq!(ctx.errors().len(), 2);

        ctx.stack_mut().push_part("/word/document.xml");
        ctx.stack_mut().push_element_path("/w:document[1]");
        assert_eq!(ctx.stack().depth(), 2);
        let n = ctx.state_mut().get_or_create("count", || 1u32).clone();
        assert_eq!(n, 1);
        ctx.clear();
        assert!(ctx.stack().is_empty());
        assert!(ctx.state().is_empty());
    }
}
