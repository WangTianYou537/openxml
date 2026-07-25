//! Part relationship constraints from generated [`PartInfo`] (C# `IPartConstraintFeature` shell).

use crate::generated::parts::{part_by_name, part_by_relationship_type, PartChildConstraint, PartInfo};
use crate::opc::media_rel;

/// Rule describing whether a child part relationship is allowed (C# `PartConstraintRule`).
#[derive(Debug, Clone, Copy)]
pub struct PartConstraintRule {
    pub relationship_type: &'static str,
    pub part_name: &'static str,
    pub content_type: Option<&'static str>,
    pub max_occurs_greater_than_one: bool,
    pub min_occurs_non_zero: bool,
    pub is_data_part_reference: bool,
}

impl PartConstraintRule {
    pub fn from_child(parent: &PartInfo, child: &PartChildConstraint) -> Option<Self> {
        if child.is_data_part_reference {
            let relationship_type = data_part_relationship_type(child.name)?;
            return Some(Self {
                relationship_type,
                part_name: child.name,
                content_type: None,
                max_occurs_greater_than_one: child.max_occurs_greater_than_one,
                min_occurs_non_zero: child.min_occurs_non_zero,
                is_data_part_reference: true,
            });
        }
        let info = part_by_name(child.name)?;
        Some(Self {
            relationship_type: info.relationship_type,
            part_name: child.name,
            content_type: info.content_type,
            max_occurs_greater_than_one: child.max_occurs_greater_than_one,
            min_occurs_non_zero: child.min_occurs_non_zero,
            is_data_part_reference: false,
        })
    }

    pub fn allows_multiple(&self) -> bool {
        self.max_occurs_greater_than_one
    }

    pub fn required(&self) -> bool {
        self.min_occurs_non_zero
    }
}

fn data_part_relationship_type(child_name: &str) -> Option<&'static str> {
    match child_name {
        "AudioReferenceRelationship" => Some(media_rel::AUDIO),
        "VideoReferenceRelationship" => Some(media_rel::VIDEO),
        "MediaReferenceRelationship" => Some(media_rel::MEDIA),
        _ => None,
    }
}

/// Constraint feature for a parent part type name (e.g. `"MainDocumentPart"`).
#[derive(Debug, Clone, Copy)]
pub struct PartConstraintFeature {
    parent_part_name: &'static str,
}

impl PartConstraintFeature {
    pub fn new(parent_part_name: &'static str) -> Self {
        Self { parent_part_name }
    }

    /// Look up by runtime `&str` when the name is not a `'static` literal.
    ///
    /// Interns against generated [`PartInfo`] names; returns a feature with an empty
    /// rule set when the name is unknown.
    pub fn for_name(parent_part_name: &str) -> Self {
        if let Some(info) = part_by_name(parent_part_name) {
            Self {
                parent_part_name: info.name,
            }
        } else {
            // Unknown — use a sentinel that yields no rules.
            Self {
                parent_part_name: "",
            }
        }
    }

    pub fn parent_part_name(&self) -> &'static str {
        self.parent_part_name
    }

    pub fn parent_info(&self) -> Option<&'static PartInfo> {
        if self.parent_part_name.is_empty() {
            None
        } else {
            part_by_name(self.parent_part_name)
        }
    }

    /// All child rules for this parent.
    pub fn rules(&self) -> Vec<PartConstraintRule> {
        let Some(parent) = self.parent_info() else {
            return Vec::new();
        };
        parent
            .children
            .iter()
            .filter_map(|c| PartConstraintRule::from_child(parent, c))
            .collect()
    }

    /// C# `TryGetRule` by relationship type URI.
    pub fn try_get_rule(&self, relationship_type: &str) -> Option<PartConstraintRule> {
        let parent = self.parent_info()?;
        for child in parent.children {
            if let Some(rule) = PartConstraintRule::from_child(parent, child) {
                if rule.relationship_type == relationship_type {
                    return Some(rule);
                }
            }
        }
        let _ = part_by_relationship_type(relationship_type);
        None
    }

    pub fn is_relationship_allowed(&self, relationship_type: &str) -> bool {
        self.try_get_rule(relationship_type).is_some()
    }

    /// Whether a data-part reference relationship is allowed on this parent.
    pub fn is_data_part_reference_allowed(&self, relationship_type: &str) -> bool {
        self.rules()
            .into_iter()
            .any(|r| r.is_data_part_reference && r.relationship_type == relationship_type)
    }

    /// Validate that adding another instance of `relationship_type` is allowed
    /// given `existing_count` current children of that type.
    pub fn can_add(&self, relationship_type: &str, existing_count: usize) -> Result<(), String> {
        let Some(rule) = self.try_get_rule(relationship_type) else {
            return Err(format!(
                "relationship `{relationship_type}` is not allowed on `{}`",
                self.parent_part_name
            ));
        };
        if !rule.allows_multiple() && existing_count >= 1 {
            return Err(format!(
                "`{}` already has a `{}` (maxOccurs=1)",
                self.parent_part_name, rule.part_name
            ));
        }
        Ok(())
    }

    /// Required child part names that are missing given present relationship types.
    pub fn missing_required<'a>(
        &self,
        present_relationship_types: impl IntoIterator<Item = &'a str>,
    ) -> Vec<&'static str> {
        let present: std::collections::HashSet<&str> =
            present_relationship_types.into_iter().collect();
        self.rules()
            .into_iter()
            .filter(|r| r.required() && !present.contains(r.relationship_type))
            .map(|r| r.part_name)
            .collect()
    }
}

/// Build a constraint feature from a [`PartInfo`] name.
pub fn constraints_for(part_name: &'static str) -> PartConstraintFeature {
    PartConstraintFeature::new(part_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_document_allows_styles_once() {
        let f = PartConstraintFeature::new("MainDocumentPart");
        let styles_rel = part_by_name("StyleDefinitionsPart")
            .unwrap()
            .relationship_type;
        assert!(f.is_relationship_allowed(styles_rel));
        let rule = f.try_get_rule(styles_rel).unwrap();
        assert!(!rule.allows_multiple());
        assert!(f.can_add(styles_rel, 0).is_ok());
        assert!(f.can_add(styles_rel, 1).is_err());
    }

    #[test]
    fn slide_allows_video_data_part_ref() {
        let f = PartConstraintFeature::new("SlidePart");
        assert!(f.is_data_part_reference_allowed(media_rel::VIDEO));
        let rule = f.try_get_rule(media_rel::VIDEO).unwrap();
        assert!(rule.is_data_part_reference);
        assert!(rule.allows_multiple());
    }

    #[test]
    fn for_name_resolves_static() {
        let f = PartConstraintFeature::for_name("MainDocumentPart");
        assert_eq!(f.parent_part_name(), "MainDocumentPart");
        assert!(f.parent_info().is_some());
    }
}
