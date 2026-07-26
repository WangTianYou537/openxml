//! MC-aware validating traversal (C# `ValidationTraverser` / `TraversalOptions`).

use super::validation_context::namespace_key;
use super::ValidationContext;
use crate::element::OpenXmlElement;
use crate::error::Result;
use crate::file_format::FileFormatVersions;
use crate::markup_compatibility::{MarkupCompatibilityAttributes, McContext};

/// C# `TraversalOptions` flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TraversalOptions {
    #[default]
    None,
    SelectAlternateContent,
}

fn in_version(element: &OpenXmlElement, version: FileFormatVersions) -> bool {
    element.prefix.is_empty()
        || version.includes_introduction(crate::file_format::prefix_introduced_in(&element.prefix))
}

/// C# `ValidationTraverser.Descendants(element, version, options)`.
///
/// With [`TraversalOptions::SelectAlternateContent`] the walk starts a fresh MC
/// context, includes the root element itself, and follows the C# stack-based
/// order (later siblings first).
pub fn descendants_with_options(
    element: &OpenXmlElement,
    version: FileFormatVersions,
    options: TraversalOptions,
) -> Vec<&OpenXmlElement> {
    match options {
        TraversalOptions::None => element.descendants().collect(),
        TraversalOptions::SelectAlternateContent => {
            let mut mc_context = McContext::with_exception_on_error(false);
            validating_traverse_tree(element, &mut mc_context, version)
        }
    }
}

/// C# static `ValidatingTraverse(inElement, mcContext, version)` — preorder walk
/// that selects AlternateContent branches, promotes unknown ProcessContent
/// children, and skips misc nodes and out-of-version elements.
pub fn validating_traverse_tree<'a>(
    root: &'a OpenXmlElement,
    mc_context: &mut McContext,
    version: FileFormatVersions,
) -> Vec<&'a OpenXmlElement> {
    enum Walk<'a> {
        Enter(&'a OpenXmlElement),
        Leave,
    }

    let mut output = Vec::new();
    let mut stack = vec![Walk::Enter(root)];
    while let Some(step) = stack.pop() {
        let element = match step {
            Walk::Enter(element) => element,
            Walk::Leave => {
                mc_context.pop_mc_attributes_for_validation();
                continue;
            }
        };

        let attributes = MarkupCompatibilityAttributes::from_element(element);
        mc_context.push_mc_attributes_for_validation(&attributes, None);
        stack.push(Walk::Leave);

        if element.is_misc_node() {
            // non-element node: skip
        } else if element.is_unknown() {
            if mc_context.is_process_content(namespace_key(element), &element.local_name) {
                for child in &element.children {
                    stack.push(Walk::Enter(child));
                }
            }
        } else if element.local_name == "AlternateContent" {
            output.push(element);
            if let Ok(Some(selected)) = mc_context.get_content_from_ac_block(element, version) {
                for child in &selected.children {
                    stack.push(Walk::Enter(child));
                }
            }
        } else if element.local_name == "Choice" || element.local_name == "Fallback" {
            // wrong parent (C# Debug.Assert only): skip
        } else {
            if in_version(element, version) {
                output.push(element);
            }
            for child in &element.children {
                stack.push(Walk::Enter(child));
            }
        }
    }
    output
}

/// C# `ValidationTraverser.ValidatingTraverse(validationContext, validateAction)`.
///
/// Walks `root` with the context's MC context and file format, pushing an
/// element-path frame per visited element. Stops silently when the error budget
/// is exhausted and returns [`Error::Cancelled`](crate::error::Error::Cancelled)
/// when the context's cancellation token fires.
pub fn validating_traverse<F>(
    context: &mut ValidationContext,
    root: &OpenXmlElement,
    mut validate_action: F,
) -> Result<()>
where
    F: FnMut(&mut ValidationContext, &OpenXmlElement),
{
    let mut mc_context = context
        .mc_context()
        .cloned()
        .unwrap_or_else(|| McContext::with_exception_on_error(false));
    let version = context.file_format();
    let children = validating_traverse_tree(root, &mut mc_context, version);
    for child in children {
        if context.check_if_cancelled()? {
            return Ok(());
        }
        context.stack_mut().push_element_path(child.qualified_name());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            validate_action(context, child)
        }));
        context.stack_mut().pop();
        if let Err(payload) = result {
            std::panic::resume_unwind(payload);
        }
    }
    Ok(())
}

/// C# `GetFirstChildMc` / `GetNextChildMc` sequence — the logical MC children
/// of `parent` in order: misc nodes skipped, AlternateContent replaced by its
/// selected branch's children, Ignorable-namespace children skipped (with
/// ProcessContent promotion), everything else returned (in-version or not).
pub fn logical_children_mc<'a>(
    parent: &'a OpenXmlElement,
    mc_context: &mut McContext,
    version: FileFormatVersions,
) -> Vec<&'a OpenXmlElement> {
    fn walk<'a>(
        elements: &'a [OpenXmlElement],
        mc_context: &mut McContext,
        version: FileFormatVersions,
        output: &mut Vec<&'a OpenXmlElement>,
    ) {
        for child in elements {
            if child.is_misc_node() {
                continue;
            }
            if !child.is_unknown() && child.local_name != "AlternateContent"
                && in_version(child, version)
            {
                output.push(child);
                continue;
            }

            let attributes = MarkupCompatibilityAttributes::from_element(child);
            mc_context.push_mc_attributes_for_validation(&attributes, None);
            if child.local_name == "AlternateContent" {
                if let Ok(Some(selected)) = mc_context.get_content_from_ac_block(child, version) {
                    walk(&selected.children, mc_context, version, output);
                }
            } else if mc_context.is_ignorable_ns(namespace_key(child)) {
                if mc_context.is_process_content(namespace_key(child), &child.local_name) {
                    walk(&child.children, mc_context, version, output);
                }
                // else: ignorable, skip entirely
            } else {
                output.push(child);
            }
            mc_context.pop_mc_attributes_for_validation();
        }
    }

    let attributes = MarkupCompatibilityAttributes::from_element(parent);
    mc_context.push_mc_attributes_for_validation(&attributes, None);
    let mut output = Vec::new();
    walk(&parent.children, mc_context, version, &mut output);
    mc_context.pop_mc_attributes_for_validation();
    output
}

/// First logical MC child (C# `GetFirstChildMc`).
pub fn get_first_child_mc<'a>(
    parent: &'a OpenXmlElement,
    mc_context: &mut McContext,
    version: FileFormatVersions,
) -> Option<&'a OpenXmlElement> {
    logical_children_mc(parent, mc_context, version).into_iter().next()
}

/// Logical MC child following `child` (C# `GetNextChildMc`), by identity.
pub fn get_next_child_mc<'a>(
    parent: &'a OpenXmlElement,
    child: &OpenXmlElement,
    mc_context: &mut McContext,
    version: FileFormatVersions,
) -> Option<&'a OpenXmlElement> {
    let children = logical_children_mc(parent, mc_context, version);
    let index = children
        .iter()
        .position(|candidate| std::ptr::eq(*candidate, child))?;
    children.get(index + 1).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markup_compatibility::alternate_content_with;
    use crate::validation::{ValidationError, ValidationSettings};

    fn sample_document() -> OpenXmlElement {
        OpenXmlElement::w("document").with_children(vec![
            OpenXmlElement::comment("misc"),
            alternate_content_with(
                "w14",
                vec![OpenXmlElement::new("w14", "urn:w14", "choiceBody")],
                vec![OpenXmlElement::w("body")],
            ),
            OpenXmlElement::w("sectPr"),
        ])
    }

    fn names(elements: &[&OpenXmlElement]) -> Vec<String> {
        elements.iter().map(|e| e.local_name.clone()).collect()
    }

    #[test]
    fn descendants_select_alternate_content_by_version() {
        let document = sample_document();

        let office_2007 = descendants_with_options(
            &document,
            FileFormatVersions::OFFICE2007,
            TraversalOptions::SelectAlternateContent,
        );
        assert_eq!(
            names(&office_2007),
            ["document", "sectPr", "AlternateContent", "body"]
        );

        let office_2010 = descendants_with_options(
            &document,
            FileFormatVersions::OFFICE2010,
            TraversalOptions::SelectAlternateContent,
        );
        assert_eq!(
            names(&office_2010),
            ["document", "sectPr", "AlternateContent", "choiceBody"]
        );

        let plain = descendants_with_options(
            &document,
            FileFormatVersions::OFFICE2007,
            TraversalOptions::None,
        );
        assert_eq!(plain.len(), document.descendants().count());
    }

    #[test]
    fn traverse_promotes_unknown_process_content_and_skips_out_of_version() {
        let mut document = OpenXmlElement::w("document");
        document.set_attribute_ns(
            "mc",
            crate::namespace::ns::MARKUP_COMPATIBILITY.uri,
            "ProcessContent",
            "w14:wrapper",
        );
        let mut wrapper = OpenXmlElement::unknown("w14", "wrapper", "urn:w14");
        wrapper.append_child(OpenXmlElement::w("body"));
        document.append_child(wrapper);
        let mut skipped = OpenXmlElement::unknown("w15", "skipped", "urn:w15");
        skipped.append_child(OpenXmlElement::w("lost"));
        document.append_child(skipped);
        document.append_child(OpenXmlElement::new("w14", "urn:w14", "later"));

        let mut mc_context = McContext::with_exception_on_error(false);
        let visited =
            validating_traverse_tree(&document, &mut mc_context, FileFormatVersions::OFFICE2007);
        assert_eq!(names(&visited), ["document", "body"]);
        assert!(!mc_context.has_ignorable());

        let mut mc_context = McContext::with_exception_on_error(false);
        let visited =
            validating_traverse_tree(&document, &mut mc_context, FileFormatVersions::OFFICE2010);
        assert_eq!(names(&visited), ["document", "later", "body"]);
    }

    #[test]
    fn logical_mc_children_cursor() {
        let document = sample_document();

        let mut mc_context = McContext::with_exception_on_error(false);
        let children_2007 =
            logical_children_mc(&document, &mut mc_context, FileFormatVersions::OFFICE2007);
        assert_eq!(names(&children_2007), ["body", "sectPr"]);

        let mut mc_context = McContext::with_exception_on_error(false);
        let children_2010 =
            logical_children_mc(&document, &mut mc_context, FileFormatVersions::OFFICE2010);
        assert_eq!(names(&children_2010), ["choiceBody", "sectPr"]);

        let mut mc_context = McContext::with_exception_on_error(false);
        let first =
            get_first_child_mc(&document, &mut mc_context, FileFormatVersions::OFFICE2007)
                .unwrap();
        assert_eq!(first.local_name, "body");
        let mut mc_context = McContext::with_exception_on_error(false);
        let next =
            get_next_child_mc(&document, first, &mut mc_context, FileFormatVersions::OFFICE2007)
                .unwrap();
        assert_eq!(next.local_name, "sectPr");
        let mut mc_context = McContext::with_exception_on_error(false);
        assert!(get_next_child_mc(
            &document,
            next,
            &mut mc_context,
            FileFormatVersions::OFFICE2007
        )
        .is_none());

        // Ignorable skip + ProcessContent promotion.
        let mut root = OpenXmlElement::w("document");
        root.set_attribute_ns(
            "mc",
            crate::namespace::ns::MARKUP_COMPATIBILITY.uri,
            "Ignorable",
            "w14 w15",
        );
        root.set_attribute_ns(
            "mc",
            crate::namespace::ns::MARKUP_COMPATIBILITY.uri,
            "ProcessContent",
            "w14:wrapper",
        );
        root.append_child(
            OpenXmlElement::unknown("w14", "wrapper", "urn:w14")
                .with_children(vec![OpenXmlElement::w("body")]),
        );
        root.append_child(
            OpenXmlElement::unknown("w15", "skipped", "urn:w15")
                .with_children(vec![OpenXmlElement::w("lost")]),
        );
        let mut mc_context = McContext::with_exception_on_error(false);
        let children =
            logical_children_mc(&root, &mut mc_context, FileFormatVersions::OFFICE2007);
        assert_eq!(names(&children), ["body"]);
        assert!(!mc_context.has_ignorable());
    }

    #[test]
    fn context_traverse_pushes_frames_and_honors_cancellation() {        let document = sample_document();

        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        let mut visited = Vec::new();
        validating_traverse(&mut context, &document, |context, element| {
            assert_eq!(
                context.stack().current().unwrap().element_path.as_deref(),
                Some(element.qualified_name().as_str())
            );
            visited.push(element.local_name.clone());
        })
        .unwrap();
        assert_eq!(
            visited,
            ["document", "sectPr", "AlternateContent", "body"]
        );
        assert!(context.stack().is_empty());

        let token = crate::validation::ValidationCancellationToken::new();
        let mut context = ValidationContext::with_cancellation_token(
            ValidationSettings::new(FileFormatVersions::OFFICE2007),
            token.clone(),
        );
        token.cancel();
        assert!(matches!(
            validating_traverse(&mut context, &document, |_, _| unreachable!()),
            Err(crate::error::Error::Cancelled)
        ));

        let mut context = ValidationContext::new(
            ValidationSettings::new(FileFormatVersions::OFFICE2007).with_max_number_of_errors(1),
        );
        let mut calls = 0usize;
        validating_traverse(&mut context, &document, |context, _| {
            calls += 1;
            context.add_error(ValidationError::new("/", "stop"));
        })
        .unwrap();
        assert_eq!(calls, 1);
    }
}
