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
}
