//! Validation run context (C# `ValidationContext` shell).

use super::{ValidationCache, ValidationError, ValidationSettings};
use crate::element::OpenXmlElement;
use crate::error::{Error, Result};
use crate::file_format::FileFormatVersions;
use crate::markup_compatibility::{
    selected_alternate_content_branch, ElementAction, MarkupCompatibilityAttributes, McContext,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Debug, Clone, Default)]
pub struct ValidationCancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl ValidationCancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// A logical validation child paired with its inherited MC context.
#[derive(Debug, Clone)]
pub struct ValidationChild<'a> {
    pub element: &'a OpenXmlElement,
    pub mc_context: McContext,
}

pub(crate) fn namespace_key(element: &OpenXmlElement) -> &str {
    if element.prefix.is_empty() {
        element.namespace_uri.as_str()
    } else {
        element.prefix.as_str()
    }
}

fn append_validation_children<'a>(
    parent: &'a OpenXmlElement,
    inherited_context: &McContext,
    supported_prefixes: &[&str],
    output: &mut Vec<ValidationChild<'a>>,
) {
    let mut context = inherited_context.clone();
    let attributes = MarkupCompatibilityAttributes::from_element(parent);
    context.push_mc_attributes_for_validation(&attributes, None);

    for child in &parent.children {
        if child.is_misc_node() {
            continue;
        }
        if child.local_name == "AlternateContent" {
            if let Some(branch) = selected_alternate_content_branch(child, supported_prefixes) {
                append_validation_children(branch, &context, supported_prefixes, output);
            }
            continue;
        }

        let known_in_version = child.prefix.is_empty()
            || supported_prefixes.contains(&child.prefix.as_str());
        match context.get_element_action(
            &child.local_name,
            namespace_key(child),
            known_in_version,
            false,
        ) {
            ElementAction::Ignore => {}
            ElementAction::ProcessContent => {
                append_validation_children(child, &context, supported_prefixes, output);
            }
            ElementAction::Normal => output.push(ValidationChild {
                element: child,
                mc_context: context.clone(),
            }),
            ElementAction::AcBlock => unreachable!("AlternateContent handled above"),
        }
    }
}


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
    cancellation_token: ValidationCancellationToken,
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
            cancellation_token: ValidationCancellationToken::new(),
        }
    }

    pub fn with_file_format(version: FileFormatVersions) -> Self {
        Self::new(ValidationSettings::new(version))
    }

    pub fn with_cancellation_token(
        settings: ValidationSettings,
        cancellation_token: ValidationCancellationToken,
    ) -> Self {
        let mut context = Self::new(settings);
        context.cancellation_token = cancellation_token;
        context
    }

    pub fn cancellation_token(&self) -> &ValidationCancellationToken {
        &self.cancellation_token
    }

    pub fn file_format(&self) -> FileFormatVersions {
        self.settings.file_format
    }

    /// Host application type for semantic constraint gating.
    pub fn application_type(&self) -> crate::features::ApplicationType {
        self.settings.application_type
    }

    pub fn set_application_type(&mut self, application_type: crate::features::ApplicationType) {
        self.settings.application_type = application_type;
    }

    pub fn valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn clear(&mut self) {
        self.errors.clear();
    }

    pub fn reset(&mut self) {
        self.errors.clear();
        self.collect_expected_children = false;
        self.expected_children.clear();
        self.mc_context = None;
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

    pub fn with_error_sink<R, F>(&mut self, sink: super::ValidationErrorSink, callback: F) -> R
    where
        F: FnOnce(&mut ValidationContext) -> R,
    {
        let depth = self.stack.depth();
        self.stack.push_error_sink(sink);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(self)));
        while self.stack.depth() > depth {
            self.stack.pop();
        }
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    pub fn state(&self) -> &super::StateManager {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut super::StateManager {
        &mut self.state
    }

    /// Return direct logical children after MC branch selection and content promotion.
    pub fn validation_children<'a>(
        &self,
        parent: &'a OpenXmlElement,
    ) -> Vec<ValidationChild<'a>> {
        let context = self.mc_context.clone().unwrap_or_default();
        self.validation_children_with_context(parent, &context)
    }

    pub(crate) fn validation_children_with_context<'a>(
        &self,
        parent: &'a OpenXmlElement,
        inherited_context: &McContext,
    ) -> Vec<ValidationChild<'a>> {
        let supported_prefixes = crate::file_format::supported_prefixes(self.file_format());
        let mut children = Vec::new();
        append_validation_children(
            parent,
            inherited_context,
            &supported_prefixes,
            &mut children,
        );
        children
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

    pub fn try_create_error(
        &mut self,
        id: &str,
        error_type: super::ValidationErrorType,
        description: impl AsRef<str>,
    ) -> Result<bool> {
        let path = if self.current_path.is_empty() {
            String::new()
        } else {
            self.current_path.clone()
        };
        self.try_add_error(
            super::ValidationError::with_id(path, id, description.as_ref())
                .with_error_type(error_type),
        )
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

    /// Check cancellation before applying the maximum-error budget.
    pub fn check_if_cancelled(&self) -> Result<bool> {
        if self.cancellation_token.is_cancelled() {
            return Err(Error::Cancelled);
        }
        Ok(self.check_max_errors())
    }

    /// True when the max-error budget is exhausted.
    pub fn check_max_errors(&self) -> bool {
        let max = self.settings.max_number_of_errors;
        max > 0 && self.errors.len() >= max
    }

    pub fn try_add_error(&mut self, error: ValidationError) -> Result<bool> {
        if self.check_if_cancelled()? {
            return Ok(false);
        }
        self.route_error(error);
        Ok(true)
    }

    pub fn add_error(&mut self, error: ValidationError) -> bool {
        if self.check_max_errors() {
            return false;
        }
        self.route_error(error);
        true
    }

    fn route_error(&mut self, error: ValidationError) {
        if let Some(sink) = self
            .stack
            .current()
            .and_then(|frame| frame.add_error.clone())
        {
            sink.add(error);
        } else {
            self.errors.push(error);
        }
    }

    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    pub fn into_errors(self) -> Vec<ValidationError> {
        self.errors
    }

    pub fn set_collect_expected_children(&mut self, collect: bool) {
        self.collect_expected_children = collect;
    }

    pub fn with_expected_children_collection<R, F>(&mut self, callback: F) -> R
    where
        F: FnOnce(&mut ValidationContext) -> R,
    {
        let previous = self.collect_expected_children;
        self.collect_expected_children = true;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(self)));
        self.collect_expected_children = previous;
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
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

    /// C# `ValidationContext.GetParticleConstraint` — resolve the version-built
    /// particle for the current stack element's local name (last path segment).
    pub fn get_particle_constraint(&mut self) -> Option<&super::Particle> {
        let local_name = self
            .stack
            .current()
            .and_then(|frame| frame.element_path.as_deref())
            .map(|path| {
                // Paths look like `/w:document[1]/w:body[1]` or bare `body` / `w:body`.
                let last = path.rsplit(['/', ']']).find(|s| !s.is_empty()).unwrap_or(path);
                let name = last.split('[').next().unwrap_or(last);
                name.rsplit(':').next().unwrap_or(name).to_string()
            })?;
        self.cache.get_constraint(&local_name)
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
        assert_eq!(ctx.check_if_cancelled().unwrap(), true);
        ctx.clear();
        assert!(ctx.valid());
    }

    #[test]
    fn cancellation_is_checked_before_error_budget() {
        let token = ValidationCancellationToken::new();
        let mut ctx = ValidationContext::with_cancellation_token(
            ValidationSettings::new(FileFormatVersions::OFFICE2010).with_max_number_of_errors(1),
            token.clone(),
        );
        assert!(!ctx.check_if_cancelled().unwrap());
        token.cancel();
        assert!(matches!(ctx.check_if_cancelled(), Err(Error::Cancelled)));
        assert!(matches!(
            ctx.try_add_error(ValidationError::new("cancelled", "error")),
            Err(Error::Cancelled)
        ));
        assert!(matches!(
            ctx.try_create_error(
                "Sch_Cancelled",
                super::super::ValidationErrorType::Schema,
                "cancelled"
            ),
            Err(Error::Cancelled)
        ));
        assert!(ctx.errors().is_empty());
        assert!(ctx.cancellation_token().is_cancelled());
    }

    #[test]
    fn scoped_error_sink_routes_and_restores() {
        use std::sync::{Arc, Mutex};

        let redirected = Arc::new(Mutex::new(Vec::new()));
        let redirected2 = Arc::clone(&redirected);
        let mut ctx = ValidationContext::default();
        ctx.with_error_sink(
            super::super::ValidationErrorSink::new(move |error| {
                redirected2.lock().unwrap().push(error);
            }),
            |ctx| {
                ctx.stack_mut().push_value("nested");
                assert!(ctx.add_error(ValidationError::new("redirected", "one")));
            },
        );
        assert_eq!(redirected.lock().unwrap().len(), 1);
        assert!(ctx.errors().is_empty());
        assert!(ctx.stack().is_empty());

        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ctx.with_error_sink(super::super::ValidationErrorSink::new(|_| {}), |ctx| {
                ctx.stack_mut().push_value("nested");
                panic!("sink failed");
            });
        }));
        assert!(panic.is_err());
        assert!(ctx.stack().is_empty());
        assert!(ctx.add_error(ValidationError::new("default", "two")));
        assert_eq!(ctx.errors().len(), 1);
    }

    #[test]
    fn expected_children_collection_is_scoped() {
        let mut ctx = ValidationContext::default();
        ctx.push_expected_child("ignored");
        assert!(ctx.expected_children().is_empty());

        ctx.with_expected_children_collection(|ctx| {
            ctx.push_expected_child("w:p");
            assert!(ctx.collect_expected_children);
        });
        assert!(!ctx.collect_expected_children);
        assert_eq!(ctx.expected_children(), &[String::from("w:p")]);

        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ctx.with_expected_children_collection(|_| panic!("collection failed"));
        }));
        assert!(panic.is_err());
        assert!(!ctx.collect_expected_children);
        ctx.clear_expected_children();
        assert!(ctx.expected_children().is_empty());
    }

    #[test]
    fn logical_children_select_versioned_alternate_content() {
        use crate::markup_compatibility::alternate_content_with;
        use crate::namespace::ns;
        use crate::wordprocessing::body;

        let mut document = OpenXmlElement::w("document").with_children(vec![
            OpenXmlElement::comment("before"),
            alternate_content_with(
                "w14",
                vec![OpenXmlElement::w("choiceBody")],
                vec![body(vec![])],
            ),
        ]);
        document.add_namespace_declaration("mc", ns::MARKUP_COMPATIBILITY.uri);

        let office_2007 = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        let children = office_2007.validation_children(&document);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].element.local_name, "body");

        let office_2010 = ValidationContext::with_file_format(FileFormatVersions::OFFICE2010);
        let children = office_2010.validation_children(&document);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].element.local_name, "choiceBody");
    }

    #[test]
    fn logical_children_promote_process_content_and_ignore_unsupported() {
        use crate::namespace::ns;
        use crate::wordprocessing::body;

        let mut document = OpenXmlElement::w("document");
        document.set_attribute_ns("mc", ns::MARKUP_COMPATIBILITY.uri, "Ignorable", "w14 w15");
        document.set_attribute_ns(
            "mc",
            ns::MARKUP_COMPATIBILITY.uri,
            "ProcessContent",
            "w14:wrapper",
        );
        document.append_child(
            OpenXmlElement::new("w15", "urn:w15", "ignored")
                .with_children(vec![OpenXmlElement::w("ignoredChild")]),
        );
        document.append_child(
            OpenXmlElement::new("w14", "urn:w14", "wrapper")
                .with_children(vec![body(vec![])]),
        );

        let office_2007 = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        let children = office_2007.validation_children(&document);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].element.local_name, "body");

        let office_2010 = ValidationContext::with_file_format(FileFormatVersions::OFFICE2010);
        let children = office_2010.validation_children(&document);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].element.local_name, "wrapper");
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
        assert!(ctx.errors().is_empty());
        assert_eq!(ctx.stack().depth(), 2);
        assert_eq!(ctx.state().get::<u32>("count"), Some(&1));
        assert_eq!(ctx.current_path(), "/w:document[1]");
        ctx.reset();
        assert!(ctx.stack().is_empty());
        assert!(ctx.state().is_empty());
        assert!(ctx.current_path().is_empty());
    }

    #[test]
    fn get_particle_constraint_resolves_from_stack_path() {
        let mut ctx = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        ctx.stack_mut().push_element_path("/w:document[1]/w:body[1]");
        let particle = ctx.get_particle_constraint().expect("body particle");
        assert_eq!(
            particle.particle_type(),
            crate::validation::ParticleType::Sequence
        );
        ctx.stack_mut().push_element_path("w:p");
        assert!(ctx.get_particle_constraint().is_some());
        ctx.stack_mut().push_element_path("notReal");
        assert!(ctx.get_particle_constraint().is_none());
    }
}
