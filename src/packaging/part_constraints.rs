//! Part relationship constraints from generated [`PartInfo`] (C# `IPartConstraintFeature` shell).

use crate::generated::parts::{part_by_name, part_by_relationship_type, PartChildConstraint, PartInfo};

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
        let info = part_by_name(child.name)?;
        Some(Self {
            relationship_type: info.relationship_type,
            part_name: child.name,
            content_type: info.content_type,
            max_occurs_greater_than_one: child.max_occurs_greater_than_one,
            min_occurs_non_zero: child.min_occurs_non_zero,
            is_data_part_reference: child.is_data_part_reference,
        })
    }

    pub fn allows_multiple(&self) -> bool {
        self.max_occurs_greater_than_one
    }

    pub fn required(&self) -> bool {
        self.min_occurs_non_zero
    }
}

/// Constraint feature for a parent part type name (e.g. `"MainDocumentPart"`).
#[derive(Debug, Clone, Copy)]
pub struct PartConstraintFeature {
    pub parent_part_name: &'static str,
}

impl PartConstraintFeature {
    pub fn new(parent_part_name: &'static str) -> Self {
        Self { parent_part_name }
    }

    pub fn parent_info(&self) -> Option<&'static PartInfo> {
        part_by_name(self.parent_part_name)
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
            let info = part_by_name(child.name)?;
            if info.relationship_type == relationship_type {
                return PartConstraintRule::from_child(parent, child);
            }
        }
        // Fallback: any part with this relationship type (ExtendedPart path uses separate checks)
        let _ = part_by_relationship_type(relationship_type);
        None
    }

    pub fn is_relationship_allowed(&self, relationship_type: &str) -> bool {
        self.try_get_rule(relationship_type).is_some()
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
}
