//! Validation error composition helpers (C# `ValidationContextExtension` +
//! `ValidationResources` message table subset).

use super::{ValidationContext, ValidationError, ValidationErrorType};

/// Message templates for the resource ids used by ported validators
/// (`{0}`/`{1}`/`{2}` positional placeholders, C# `ValidationResources`).
pub fn validation_resource_message(message_id: &str) -> Option<&'static str> {
    Some(match message_id {
        "Sch_InvalidElementContentExpectingComplex" => {
            "The element has invalid child element '{0}'.{1}"
        }
        "Sch_IncompleteContentExpectingComplex" => "The element has incomplete content.{0}",
        "Sch_UndeclaredAttribute" => "The '{0}' attribute is not declared.",
        "Sch_UndeclaredElement" => "The element '{0}' is not declared.",
        "Sch_MissRequiredAttribute" => "The required attribute '{0}' is missing.",
        "Sch_AttributeValueDataTypeDetailed" => "The attribute '{0}' has invalid value '{1}'.{2}",
        "Sch_InvalidChildinLeafElement" => {
            "The element '{0}' is a leaf element and cannot contain children."
        }
        "Sch_MissingPartRootElement" => "The '{0}' part is missing its root element.",
        "MC_ShallContainChoice" => {
            "An AlternateContent element must contain one or more Choice child elements, optionally followed by a Fallback child element."
        }
        "MC_ShallNotContainAlternateContent" => {
            "An AlternateContent element cannot be the child of an AlternateContent element."
        }
        "MC_MissedRequiresAttribute" => {
            "All Choice elements must have a Requires attribute whose value contains a whitespace delimited list of namespace prefixes."
        }
        "MC_InvalidRequiresAttribute" => {
            "The Requires attribute is invalid - The value '{0}' contains an invalid prefix that is not defined."
        }
        "MC_InvalidIgnorableAttribute" => {
            "The Ignorable attribute is invalid - The value '{0}' contains an invalid prefix that is not defined."
        }
        "MC_InvalidMustUnderstandAttribute" => {
            "The MustUnderstand attribute is invalid - The value '{0}' contains an invalid prefix that is not defined."
        }
        "MC_InvalidXmlAttribute" => {
            "The {0} element should not have an xml:lang or xml:space attribute."
        }
        "MC_InvalidXmlAttributeWithProcessContent" => {
            "An element should not have an xml:lang or xml:space attribute and also be identified by a ProcessContent attribute."
        }
        "MC_ErrorOnUnprefixedAttributeName" => {
            "The attribute '{0}' needs to specify a proper prefix when defined on an AlternateContent element."
        }
        "ExceptionError" => "Inner exception: {0}.",
        _ => return None,
    })
}

fn format_positional(template: &str, args: &[&str]) -> String {
    let mut message = template.to_string();
    for (index, arg) in args.iter().enumerate() {
        message = message.replace(&format!("{{{index}}}"), arg);
    }
    // Drop unreplaced placeholders (C# SR.Format with missing args would throw;
    // callers here may omit trailing optional args like expected-children text).
    for index in args.len()..10 {
        message = message.replace(&format!("{{{index}}}"), "");
    }
    message
}

impl ValidationContext {
    /// C# `ComposeValidationError` — build (not add) an error from a message id.
    pub fn compose_validation_error(
        &self,
        error_type: ValidationErrorType,
        element_path: Option<&str>,
        child_path: Option<&str>,
        message_id: &str,
        args: &[&str],
    ) -> ValidationError {
        let description = match validation_resource_message(message_id) {
            Some(template) => format_positional(template, args),
            None => format!("An unknown error occurred. Original message: '{message_id}'"),
        };

        let path = self
            .stack()
            .current()
            .and_then(|frame| frame.part_uri.clone())
            .unwrap_or_else(|| self.current_path().to_string());

        let mut error = ValidationError::with_id(path, message_id, description)
            .with_error_type(error_type);
        if let Some(element_path) = element_path {
            error = error.with_node_path(element_path);
        }
        if let Some(child_path) = child_path {
            error = error.with_related_node_path(child_path);
        }
        error
    }

    /// C# `ComposeSchemaValidationError`.
    pub fn compose_schema_validation_error(
        &self,
        element_path: Option<&str>,
        child_path: Option<&str>,
        message_id: &str,
        args: &[&str],
    ) -> ValidationError {
        self.compose_validation_error(
            ValidationErrorType::Schema,
            element_path,
            child_path,
            message_id,
            args,
        )
    }

    /// C# `ComposeMcValidationError`.
    pub fn compose_mc_validation_error(
        &self,
        element_path: Option<&str>,
        message_id: &str,
        args: &[&str],
    ) -> ValidationError {
        self.compose_validation_error(
            ValidationErrorType::MarkupCompatibility,
            element_path,
            None,
            message_id,
            args,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_format::FileFormatVersions;

    #[test]
    fn composes_known_messages_with_args() {
        let mut context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);
        context.stack_mut().push_part("/word/document.xml");
        context.stack_mut().push_element_path("/w:document[1]");

        let error = context.compose_schema_validation_error(
            Some("/w:document[1]"),
            Some("/w:document[1]/w:bad[1]"),
            "Sch_InvalidElementContentExpectingComplex",
            &["w:bad", " List of possible elements expected: <w:body>."],
        );
        assert_eq!(error.id(), Some("Sch_InvalidElementContentExpectingComplex"));
        assert_eq!(error.error_type(), ValidationErrorType::Schema);
        assert_eq!(error.path, "/word/document.xml");
        assert_eq!(
            error.description(),
            "The element has invalid child element 'w:bad'. List of possible elements expected: <w:body>."
        );
        assert_eq!(error.node_path.as_deref(), Some("/w:document[1]"));
        assert_eq!(
            error.related_node_path.as_deref(),
            Some("/w:document[1]/w:bad[1]")
        );
    }

    #[test]
    fn composes_mc_and_unknown_messages() {
        let context = ValidationContext::with_file_format(FileFormatVersions::OFFICE2007);

        let mc = context.compose_mc_validation_error(None, "MC_ShallContainChoice", &[]);
        assert_eq!(mc.error_type(), ValidationErrorType::MarkupCompatibility);
        assert!(mc.description().starts_with("An AlternateContent element must contain"));

        let trailing = context.compose_schema_validation_error(
            None,
            None,
            "Sch_IncompleteContentExpectingComplex",
            &[],
        );
        assert_eq!(trailing.description(), "The element has incomplete content.");

        let unknown = context.compose_schema_validation_error(None, None, "Nope_NotReal", &[]);
        assert_eq!(
            unknown.description(),
            "An unknown error occurred. Original message: 'Nope_NotReal'"
        );
    }
}
