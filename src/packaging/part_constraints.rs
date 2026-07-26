//! Part relationship constraints from generated [`PartInfo`] (C# `IPartConstraintFeature` shell).

use crate::file_format::FileFormatVersions;
use crate::generated::parts::{part_by_name, part_by_relationship_type, PartChildConstraint, PartInfo};
use crate::opc::media_rel;

/// Infer the Office version that introduced a relationship type from its URI
/// (C# `PartConstraintRule.FileFormat` shell when generated tables lack version).
pub fn relationship_introduced_in(relationship_type: &str) -> FileFormatVersions {
    // Prefer explicit year segments used by Microsoft Office relationship URIs.
    if relationship_type.contains("/2024/") || relationship_type.contains("/2025/") {
        return FileFormatVersions::MICROSOFT365;
    }
    if relationship_type.contains("/2021/")
        || relationship_type.contains("/2022/")
        || relationship_type.contains("/2023/")
    {
        return FileFormatVersions::OFFICE2021;
    }
    if relationship_type.contains("/2019/") || relationship_type.contains("/2020/") {
        return FileFormatVersions::OFFICE2019;
    }
    if relationship_type.contains("/2016/")
        || relationship_type.contains("/2017/")
        || relationship_type.contains("/2018/")
    {
        return FileFormatVersions::OFFICE2016;
    }
    if relationship_type.contains("/2013/") || relationship_type.contains("/2014/") {
        return FileFormatVersions::OFFICE2013;
    }
    if relationship_type.contains("/2012/") {
        return FileFormatVersions::OFFICE2013;
    }
    if relationship_type.contains("/2010/") || relationship_type.contains("/2009/") {
        return FileFormatVersions::OFFICE2010;
    }
    FileFormatVersions::OFFICE2007
}

/// Rule describing whether a child part relationship is allowed (C# `PartConstraintRule`).
#[derive(Debug, Clone, Copy)]
pub struct PartConstraintRule {
    pub relationship_type: &'static str,
    pub part_name: &'static str,
    pub content_type: Option<&'static str>,
    pub max_occurs_greater_than_one: bool,
    pub min_occurs_non_zero: bool,
    pub is_data_part_reference: bool,
    /// Office version that introduced this constraint (C# `FileFormat`).
    pub availability: FileFormatVersions,
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
                availability: relationship_introduced_in(relationship_type),
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
            availability: relationship_introduced_in(info.relationship_type),
        })
    }

    pub fn allows_multiple(&self) -> bool {
        self.max_occurs_greater_than_one
    }

    pub fn required(&self) -> bool {
        self.min_occurs_non_zero
    }

    /// Whether this rule applies when validating against `version`
    /// (C# `version.AtLeast(constraintRule.FileFormat)`).
    pub fn applies_to(&self, version: FileFormatVersions) -> bool {
        // C# uses AtLeast; also treat ALL / multi-bit targets via includes_introduction.
        version.at_least(self.availability) || version.includes_introduction(self.availability)
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

    /// Rules that apply when validating against `version`
    /// (C# `version.AtLeast(constraintRule.FileFormat)` filter).
    pub fn rules_for_version(&self, version: FileFormatVersions) -> Vec<PartConstraintRule> {
        self.rules()
            .into_iter()
            .filter(|r| r.applies_to(version))
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

    /// [`try_get_rule`] that only returns rules available in `version`.
    pub fn try_get_rule_for_version(
        &self,
        relationship_type: &str,
        version: FileFormatVersions,
    ) -> Option<PartConstraintRule> {
        self.try_get_rule(relationship_type)
            .filter(|r| r.applies_to(version))
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
        self.missing_required_for_version(
            present_relationship_types,
            FileFormatVersions::ALL,
        )
    }

    /// Required children for `version` that are missing given present relationship types
    /// (C# required-part check with `version.AtLeast(rule.FileFormat)`).
    pub fn missing_required_for_version<'a>(
        &self,
        present_relationship_types: impl IntoIterator<Item = &'a str>,
        version: FileFormatVersions,
    ) -> Vec<&'static str> {
        let present: std::collections::HashSet<&str> =
            present_relationship_types.into_iter().collect();
        self.rules_for_version(version)
            .into_iter()
            .filter(|r| r.required() && !present.contains(r.relationship_type))
            .map(|r| r.part_name)
            .collect()
    }

    /// Rules that describe data-part reference relationships only.
    pub fn data_part_reference_rules(&self) -> Vec<PartConstraintRule> {
        self.rules()
            .into_iter()
            .filter(|r| r.is_data_part_reference)
            .collect()
    }

    /// Non-data-part (ordinary part) child rules.
    pub fn part_rules(&self) -> Vec<PartConstraintRule> {
        self.rules()
            .into_iter()
            .filter(|r| !r.is_data_part_reference)
            .collect()
    }

    /// Required relationship type URIs for this parent.
    pub fn required_relationship_types(&self) -> Vec<&'static str> {
        self.rules()
            .into_iter()
            .filter(|r| r.required())
            .map(|r| r.relationship_type)
            .collect()
    }

    pub fn rule_count(&self) -> usize {
        self.rules().len()
    }

    /// Whether any child rule is a data-part reference.
    pub fn has_data_part_references(&self) -> bool {
        self.rules().into_iter().any(|r| r.is_data_part_reference)
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
    fn relationship_introduced_in_years() {
        assert_eq!(
            relationship_introduced_in(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"
            ),
            FileFormatVersions::OFFICE2007
        );
        assert_eq!(
            relationship_introduced_in(
                "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds"
            ),
            FileFormatVersions::OFFICE2016
        );
        assert_eq!(
            relationship_introduced_in(
                "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView"
            ),
            FileFormatVersions::OFFICE2019
        );
        assert_eq!(
            relationship_introduced_in(
                "http://schemas.microsoft.com/office/2014/relationships/chartEx"
            ),
            FileFormatVersions::OFFICE2013
        );
    }

    #[test]
    fn rules_for_version_filters_future_rules() {
        let f = PartConstraintFeature::new("MainDocumentPart");
        let comments_ids = part_by_name("WordprocessingCommentsIdsPart")
            .unwrap()
            .relationship_type;
        // Rule is present when targeting all / 2016+
        assert!(f
            .try_get_rule_for_version(comments_ids, FileFormatVersions::OFFICE2016)
            .is_some());
        assert!(f
            .try_get_rule_for_version(comments_ids, FileFormatVersions::ALL)
            .is_some());
        // Not applicable for Office 2007
        assert!(f
            .try_get_rule_for_version(comments_ids, FileFormatVersions::OFFICE2007)
            .is_none());
        let styles = part_by_name("StyleDefinitionsPart")
            .unwrap()
            .relationship_type;
        assert!(f
            .try_get_rule_for_version(styles, FileFormatVersions::OFFICE2007)
            .is_some());
    }

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

    #[test]
    fn data_part_and_part_rule_splits() {
        let f = PartConstraintFeature::new("SlidePart");
        assert!(f.has_data_part_references());
        assert!(!f.data_part_reference_rules().is_empty());
        assert!(!f.part_rules().is_empty());
        assert!(f.rule_count() > 0);
        let main = PartConstraintFeature::new("MainDocumentPart");
        // Main document may not require children in all generated tables; just ensure API works.
        let _ = main.required_relationship_types();
        assert!(main.rule_count() > 0);
    }
}
