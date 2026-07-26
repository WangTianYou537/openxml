//! Per-element schema validation (C# `SchemaTypeValidator`).
//!
//! Orchestrates, for one element:
//! 1. Reserved / AlternateContent early-out (AC structural checks elsewhere)
//! 2. Attribute undeclared + lexical type checks
//! 3. Leaf empty / leaf-text content checks
//! 4. Composite particle matching via [`ValidationContext::get_particle_constraint`]

use super::{
    validate_attribute_value_types, validate_leaf_content, validate_particle_for_version,
    validate_undeclared_attributes, ValidationContext, ValidationError, ValidationErrorType,
};
use crate::element::OpenXmlElement;
use crate::error::Result;
use crate::file_format::FileFormatVersions;
use crate::markup_compatibility::McContext;
use crate::namespace::ns;

/// Whether the element is an MC reserved structural node
/// (C# `OpenXmlElement.IsReservedElement` / `IsAlternateContent`).
pub fn is_reserved_element(element: &OpenXmlElement) -> bool {
    if element.is_misc_node() || element.is_unknown() {
        return true;
    }
    matches!(
        element.local_name.as_str(),
        "AlternateContent" | "Choice" | "Fallback"
    ) && (element.prefix == "mc"
        || element.namespace_uri == ns::MARKUP_COMPATIBILITY.uri
        || element.namespace_uri.is_empty())
}

/// Whether generated schema metadata marks this as a leaf (empty or text).
/// Covers Word / Spreadsheet / Presentation / DrawingML main schemas.
fn generated_leaf_flags(element: &OpenXmlElement) -> Option<(bool, bool)> {
    macro_rules! try_schema {
        ($mod:path) => {{
            use $mod as schema;
            if let Some(info) = schema::info_by_local_name(&element.local_name) {
                if element.prefix == info.prefix {
                    return Some((info.is_leaf, info.is_leaf_text));
                }
            }
        }};
    }
    match element.prefix.as_str() {
        "w" => try_schema!(crate::generated::wordprocessingml_2006_main),
        "x" => try_schema!(crate::generated::spreadsheetml_2006_main),
        "p" => try_schema!(crate::generated::presentationml_2006_main),
        "a" => try_schema!(crate::generated::drawingml_2006_main),
        _ => {}
    }
    try_schema!(crate::generated::wordprocessingml_2006_main);
    try_schema!(crate::generated::spreadsheetml_2006_main);
    try_schema!(crate::generated::presentationml_2006_main);
    try_schema!(crate::generated::drawingml_2006_main);
    None
}

/// C# `SchemaTypeValidator.Validate` for a single element already on the stack.
///
/// Expects `context` to carry the current element path (and optional MC context).
/// Does **not** recurse into children — callers walk the tree (DocumentValidator
/// / ValidationTraverser).
pub fn validate_schema_type(
    element: &OpenXmlElement,
    context: &mut ValidationContext,
) -> Result<()> {
    if element.is_misc_node() || element.is_unknown() {
        return Ok(());
    }

    // C#: reserved AC/Choice/Fallback — only AlternateContent gets its own
    // validator (handled at the tree level); Choice/Fallback are skipped.
    if is_reserved_element(element) {
        return Ok(());
    }

    let path = if context.current_path().is_empty() {
        element.qualified_name()
    } else {
        context.current_path().to_string()
    };

    // C# CompatibilityRuleAttributesValidator runs at tree scope; attribute
    // undeclared / type / leaf content run per element here.
    let mut mc_context = context
        .mc_context
        .clone()
        .unwrap_or_else(|| McContext::with_exception_on_error(false));

    for error in validate_undeclared_attributes(element, &mut mc_context, &path) {
        if !context.try_add_error(error)? {
            return Ok(());
        }
    }
    for error in validate_attribute_value_types(element, &path) {
        if !context.try_add_error(error)? {
            return Ok(());
        }
    }

    let version = context.cache().version();
    match generated_leaf_flags(element) {
        Some((true, _)) => {
            // C# ValidateEmptyComplexType / ValidateSimpleContextComplexType
            // (simple-content still forbids element children).
            for error in validate_leaf_content(element, &path) {
                if !context.try_add_error(error)? {
                    return Ok(());
                }
            }
        }
        Some((false, _)) | None => {
            // Composite / unknown metadata: particle constraint when available.
            validate_composite_complex_type(element, context, &path, version)?;
        }
    }
    Ok(())
}

/// C# `SchemaTypeValidator.ValidateCompositeComplexType`.
fn validate_composite_complex_type(
    element: &OpenXmlElement,
    context: &mut ValidationContext,
    path: &str,
    version: FileFormatVersions,
) -> Result<()> {
    // Prefer particle from ValidationCache via context (C# GetParticleConstraint).
    let particle = context
        .get_particle_constraint()
        .cloned()
        .or_else(|| super::particle::particle_for(&element.local_name));

    let Some(particle) = particle else {
        // C# empty root complex type: no particle → reject non-misc children
        // when this is a leaf-empty type without a particle.
        if is_empty_root_like(element) {
            for error in validate_empty_root_complex_type(element, path) {
                if !context.try_add_error(error)? {
                    return Ok(());
                }
            }
        }
        return Ok(());
    };

    for error in validate_particle_for_version(element, &particle, path, version) {
        if !context.try_add_error(error)? {
            return Ok(());
        }
    }
    Ok(())
}

/// Heuristic for C# empty part-root complex types (no particle, no children allowed).
fn is_empty_root_like(element: &OpenXmlElement) -> bool {
    matches!(generated_leaf_flags(element), Some((true, false)))
}

/// C# `SchemaTypeValidator.ValidateEmptyRootComplexType` /
/// `ValidateEmptyComplexType` — report one `Sch_InvalidChildinLeafElement`.
pub fn validate_empty_root_complex_type(
    element: &OpenXmlElement,
    path: &str,
) -> Vec<ValidationError> {
    if element.children.iter().any(|child| !child.is_misc_node()) {
        return vec![ValidationError::with_id(
            path,
            "Sch_InvalidChildinLeafElement",
            format!(
                "The element \'{}\' is a leaf element and cannot contain children.",
                element.qualified_name()
            ),
        )
        .with_error_type(ValidationErrorType::Schema)];
    }
    Vec::new()
}

/// Walk `root` preorder with inherited MC context and run [`validate_schema_type`]
/// on each element (C# DocumentValidator schema pass body).
pub fn validate_schema_types_in_tree(
    root: &OpenXmlElement,
    context: &mut ValidationContext,
) -> Result<()> {
    if root.is_misc_node() || root.is_unknown() {
        return Ok(());
    }
    let mut mc = McContext::with_exception_on_error(false);
    walk_schema_types(root, &mut mc, context)
}

fn walk_schema_types(
    element: &OpenXmlElement,
    mc_context: &mut McContext,
    context: &mut ValidationContext,
) -> Result<()> {
    if element.is_misc_node() || element.is_unknown() {
        return Ok(());
    }
    if is_reserved_element(element) {
        // Still walk into AC children for schema of selected content? C# validates
        // AC structure separately; descendants inside Choice/Fallback are validated
        // when selected by the traverser. Keep a simple depth-first walk that
        // visits all non-reserved elements so undeclared attrs inherit Ignorable.
        for child in &element.children {
            walk_schema_types(child, mc_context, context)?;
        }
        return Ok(());
    }

    let attributes =
        crate::markup_compatibility::MarkupCompatibilityAttributes::from_element(element);
    mc_context.push_mc_attributes_for_validation(&attributes, None);

    let path = element.qualified_name();
    context.stack_mut().push_element_path(&path);
    context.current_path = path;
    context.mc_context = Some(mc_context.clone());

    let result = validate_schema_type(element, context);
    context.stack_mut().pop();
    context.mc_context = None;
    result?;
    if context.check_if_cancelled().is_err() {
        mc_context.pop_mc_attributes_for_validation();
        return Ok(());
    }

    for child in &element.children {
        walk_schema_types(child, mc_context, context)?;
        if context.check_if_cancelled().is_err() {
            mc_context.pop_mc_attributes_for_validation();
            return Ok(());
        }
    }

    mc_context.pop_mc_attributes_for_validation();
    Ok(())
}

/// Convenience: collect schema-type errors for a single element without a full context.
pub fn validate_schema_type_standalone(
    element: &OpenXmlElement,
    version: FileFormatVersions,
) -> Vec<ValidationError> {
    let settings = super::ValidationSettings::new(version);
    let mut context = ValidationContext::new(settings);
    context.current_path = element.qualified_name();
    context
        .stack_mut()
        .push_element_path(element.qualified_name());
    let _ = validate_schema_type(element, &mut context);
    context.stack_mut().pop();
    context.errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::OpenXmlElement;
    use crate::file_format::FileFormatVersions;
    use crate::wordprocessing::{body, document, paragraph, run, text};

    #[test]
    fn reserved_ac_is_skipped() {
        let ac = OpenXmlElement::new("mc", ns::MARKUP_COMPATIBILITY.uri, "AlternateContent");
        assert!(is_reserved_element(&ac));
        let errs = validate_schema_type_standalone(&ac, FileFormatVersions::OFFICE2007);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn leaf_text_rejects_element_children() {
        // w:t is leaf-text; adding a child element is invalid.
        let mut t = text("hi");
        t.children
            .push(OpenXmlElement::new("w", ns::WORDPROCESSINGML.uri, "rPr"));
        let errs = validate_schema_type_standalone(&t, FileFormatVersions::OFFICE2007);
        assert!(
            errs.iter()
                .any(|e| e.message.contains("Sch_InvalidChildinLeafElement")),
            "{errs:?}"
        );
    }

    #[test]
    fn document_particle_reports_missing_body() {
        let doc = document(vec![]);
        let errs = validate_schema_type_standalone(&doc, FileFormatVersions::OFFICE2007);
        assert!(
            errs.iter().any(|e| {
                e.message.contains("Sch_IncompleteContent")
                    || e.message.contains("incomplete")
                    || e.message.contains("body")
                    || e.message.contains("expected")
                    || e.message.contains("InvalidElementContent")
            }),
            "{errs:?}"
        );
    }

    #[test]
    fn valid_document_tree_ok() {
        let doc = document(vec![body(vec![paragraph(vec![run(vec![text("hi")])])])]);
        let settings = super::super::ValidationSettings::new(FileFormatVersions::OFFICE2007);
        let mut context = ValidationContext::new(settings);
        validate_schema_types_in_tree(&doc, &mut context).unwrap();
        assert!(
            !context
                .errors
                .iter()
                .any(|e| e.message.contains("Sch_InvalidChildinLeafElement")),
            "{:?}",
            context.errors
        );
    }

    #[test]
    fn undeclared_attribute_reported() {
        let mut p = paragraph(vec![]);
        p.set_attribute("notARealAttr", "x");
        let errs = validate_schema_type_standalone(&p, FileFormatVersions::OFFICE2007);
        assert!(
            errs.iter()
                .any(|e| e.message.contains("Sch_UndeclaredAttribute")
                    || e.message.contains("not declared")
                    || e.message.contains("notARealAttr")),
            "{errs:?}"
        );
    }
}
