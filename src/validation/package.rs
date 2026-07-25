//! Package-structure validation (content types + relationship targets).

use super::ValidationError;
use crate::namespace::rel;
use crate::opc::{OpcPackage, PackUri};

/// Validate basic OPC package structure.
///
/// Checks:
/// - package has an officeDocument (main) relationship when `require_main` is true
/// - main part exists and has a content-type override/default
/// - internal relationship targets resolve to existing parts
pub fn validate_package(package: &OpcPackage, require_main: bool) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    let main_rel = package
        .package_relationships()
        .get_by_type(rel::OFFICE_DOCUMENT);

    if require_main && main_rel.is_none() {
        errors.push(ValidationError {
            path: "/_rels/.rels".into(),
            message: "missing package relationship of type officeDocument".into(),
        });
    }

    if let Some(main) = main_rel {
        match resolve_pkg_target(package, &main.target) {
            Ok(uri) => {
                if !package.has_part(&uri) {
                    errors.push(ValidationError {
                        path: uri.as_str().to_string(),
                        message: format!(
                            "officeDocument target `{}` does not exist",
                            main.target
                        ),
                    });
                } else if package
                    .content_types()
                    .content_type_for(uri.as_str())
                    .is_none()
                {
                    errors.push(ValidationError {
                        path: uri.as_str().to_string(),
                        message: "main part has no content type".into(),
                    });
                }
            }
            Err(msg) => errors.push(ValidationError {
                path: "/_rels/.rels".into(),
                message: msg,
            }),
        }
    }

    // Package-level internal relationships
    for rel_item in package.package_relationships().iter() {
        if rel_item.target_mode
            == crate::opc::RelationshipTargetMode::External
        {
            continue;
        }
        // Skip hyperlinks-style absolute URIs that slipped through
        if rel_item.target.contains("://") {
            continue;
        }
        match resolve_pkg_target(package, &rel_item.target) {
            Ok(uri) => {
                if !package.has_part(&uri) && !is_optional_missing_ok(&uri) {
                    errors.push(ValidationError {
                        path: format!("/_rels/.rels#{}", rel_item.id),
                        message: format!(
                            "relationship `{}` targets missing part `{}`",
                            rel_item.id,
                            uri.as_str()
                        ),
                    });
                }
            }
            Err(msg) => errors.push(ValidationError {
                path: format!("/_rels/.rels#{}", rel_item.id),
                message: msg,
            }),
        }
    }

    // Part-level internal relationships
    let part_uris: Vec<PackUri> = package.part_uris();
    for part_uri in part_uris {
        let Some(rels) = package.part_relationships(&part_uri) else {
            continue;
        };
        for rel_item in rels.iter() {
            if rel_item.target_mode
                == crate::opc::RelationshipTargetMode::External
            {
                continue;
            }
            if rel_item.target.contains("://") {
                continue;
            }
            match crate::opc::resolve_uri(&part_uri, &rel_item.target) {
                Ok(target) => {
                    if !package.has_part(&target) {
                        errors.push(ValidationError {
                            path: format!(
                                "{}#_rels#{}",
                                part_uri.as_str(),
                                rel_item.id
                            ),
                            message: format!(
                                "relationship `{}` targets missing part `{}`",
                                rel_item.id,
                                target.as_str()
                            ),
                        });
                    }
                }
                Err(e) => errors.push(ValidationError {
                    path: format!("{}#_rels#{}", part_uri.as_str(), rel_item.id),
                    message: e.to_string(),
                }),
            }
        }
    }

    // Digital signature package structure (no crypto).
    errors.extend(super::validate_digital_signatures(package));
    errors.extend(super::validate_signature_digests(package));

    // Part relationship constraints (C# PackageValidator).
    errors.extend(super::validate_package_constraints(package));

    errors
}

fn resolve_pkg_target(package: &OpcPackage, target: &str) -> std::result::Result<PackUri, String> {
    let root = PackUri::new("/");
    crate::opc::resolve_uri(&root, target).map_err(|e| {
        let _ = package;
        e.to_string()
    })
}

fn is_optional_missing_ok(_uri: &PackUri) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::content_type;
    use crate::opc::RelationshipTargetMode;

    #[test]
    fn empty_package_fails_require_main() {
        let pkg = OpcPackage::create();
        let errs = validate_package(&pkg, true);
        assert!(!errs.is_empty());
    }

    #[test]
    fn valid_minimal_package() {
        let mut pkg = OpcPackage::create();
        let uri = PackUri::new("/word/document.xml");
        pkg.set_part(
            uri.clone(),
            content_type::WORD_DOCUMENT,
            br#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>"#.to_vec(),
        );
        pkg.add_package_relationship(
            rel::OFFICE_DOCUMENT,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let errs = validate_package(&pkg, true);
        assert!(errs.is_empty(), "{errs:?}");
    }
}
