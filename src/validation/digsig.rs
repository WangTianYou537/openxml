//! Digital signature package structure checks (no crypto).
//!
//! Validates presence and wiring of origin / signature parts. Does **not**
//! verify cryptographic signatures.

use super::ValidationError;
use crate::namespace::{content_type, rel};
use crate::opc::{OpcPackage, PackUri, RelationshipTargetMode};

const ORIGIN_URI: &str = "/_xmlsignatures/origin.sigs";

/// Validate digital-signature package structure when origin is present.
///
/// Checks:
/// - origin part has content type `DIGITAL_SIGNATURE_ORIGIN`
/// - package relationship of type digital signature origin points at origin
/// - each signature relationship from origin targets an existing part
/// - signature parts have XML signature content type when present
pub fn validate_digital_signatures(package: &OpcPackage) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let origin = PackUri::new(ORIGIN_URI);
    let has_origin_part = package.has_part(&origin);
    let origin_rel = package
        .package_relationships()
        .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN);

    if !has_origin_part && origin_rel.is_none() {
        return errors; // no digsig surface — OK
    }

    if has_origin_part {
        let ct = package.content_types().content_type_for(origin.as_str());
        let expected = content_type::DIGITAL_SIGNATURE_ORIGIN;
        if ct.map(|c| !c.eq_ignore_ascii_case(expected)).unwrap_or(true) {
            errors.push(ValidationError {
                path: ORIGIN_URI.into(),
                message: format!(
                    "digital signature origin has content type `{}`, expected `{}`",
                    ct.unwrap_or("<missing>"),
                    expected
                ),
            });
        }
    } else {
        errors.push(ValidationError {
            path: ORIGIN_URI.into(),
            message: "package has digital signature origin relationship but origin part is missing"
                .into(),
        });
    }

    if origin_rel.is_none() && has_origin_part {
        errors.push(ValidationError {
            path: "/_rels/.rels".into(),
            message: "digital signature origin part exists without package relationship".into(),
        });
    }

    // Signature parts related from origin
    if let Some(rels) = package.part_relationships(&origin) {
        for r in rels.iter() {
            if r.target_mode == RelationshipTargetMode::External {
                continue;
            }
            // Accept digital signature relationship type (suffix match)
            let is_sig = r.relationship_type == rel::DIGITAL_SIGNATURE
                || r.relationship_type.ends_with("/digital-signature/signature")
                || r.relationship_type.contains("digital-signature");
            if !is_sig {
                continue;
            }
            match package.resolve_relationship(Some(&origin), r) {
                Ok(uri) => {
                    if !package.has_part(&uri) {
                        errors.push(ValidationError {
                            path: format!("{ORIGIN_URI}#{}", r.id),
                            message: format!(
                                "signature relationship `{}` targets missing part `{}`",
                                r.id,
                                uri.as_str()
                            ),
                        });
                    } else {
                        let ct = package.content_types().content_type_for(uri.as_str());
                        let expected = content_type::DIGITAL_SIGNATURE_XML;
                        // also accept generic xml
                        let ok = ct
                            .map(|c| {
                                c.eq_ignore_ascii_case(expected)
                                    || c.contains("xmldsig")
                                    || c.ends_with("+xml")
                                    || c == "text/xml"
                                    || c == "application/xml"
                            })
                            .unwrap_or(false);
                        if !ok {
                            errors.push(ValidationError {
                                path: uri.as_str().to_string(),
                                message: format!(
                                    "signature part has unexpected content type `{}`",
                                    ct.unwrap_or("<missing>")
                                ),
                            });
                        }
                    }
                }
                Err(e) => errors.push(ValidationError {
                    path: format!("{ORIGIN_URI}#{}", r.id),
                    message: format!("cannot resolve signature relationship: {e}"),
                }),
            }
        }
    }

    errors
}


/// Ensure the digital signature origin part and package relationship exist.
/// Returns the origin URI.
pub fn ensure_digital_signature_origin(package: &mut OpcPackage) -> Result<PackUri, crate::error::Error> {
    let origin = PackUri::new(ORIGIN_URI);
    if !package.has_part(&origin) {
        // origin.sigs is typically empty or minimal binary
        package.set_part(
            origin.clone(),
            content_type::DIGITAL_SIGNATURE_ORIGIN,
            Vec::new(),
        );
    }
    let has_rel = package
        .package_relationships()
        .get_by_type(rel::DIGITAL_SIGNATURE_ORIGIN)
        .is_some();
    if !has_rel {
        package.add_package_relationship(
            rel::DIGITAL_SIGNATURE_ORIGIN,
            &origin,
            RelationshipTargetMode::Internal,
        );
    }
    Ok(origin)
}

/// List signature part URIs related from the origin (if present).
pub fn digital_signature_parts(package: &OpcPackage) -> Vec<PackUri> {
    let origin = PackUri::new(ORIGIN_URI);
    let mut out = Vec::new();
    let Some(rels) = package.part_relationships(&origin) else {
        return out;
    };
    for r in rels.iter() {
        if r.target_mode != RelationshipTargetMode::Internal {
            continue;
        }
        let is_sig = r.relationship_type == rel::DIGITAL_SIGNATURE
            || r.relationship_type.ends_with("/digital-signature/signature")
            || r.relationship_type.contains("digital-signature");
        if !is_sig {
            continue;
        }
        if let Ok(uri) = package.resolve_relationship(Some(&origin), r) {
            if package.has_part(&uri) {
                out.push(uri);
            }
        }
    }
    out
}

/// Whether the package has a digital signature origin part.
pub fn has_digital_signature_origin(package: &OpcPackage) -> bool {
    package.has_part(&PackUri::new(ORIGIN_URI))
}

/// Add a signature XML part under `/_xmlsignatures/` and relate it from origin.
///
/// This stores the provided XML bytes only — it does **not** compute a real signature.
/// Returns `(relationship_id, signature_part_uri)`.
pub fn add_digital_signature_part(
    package: &mut OpcPackage,
    signature_xml: impl Into<Vec<u8>>,
) -> Result<(String, PackUri), crate::error::Error> {
    let origin = ensure_digital_signature_origin(package)?;
    let mut index = 1u32;
    let uri = loop {
        let candidate = PackUri::new(format!("/_xmlsignatures/sig{index}.xml"));
        if !package.has_part(&candidate) {
            break candidate;
        }
        index += 1;
    };
    package.set_part(
        uri.clone(),
        content_type::DIGITAL_SIGNATURE_XML,
        signature_xml.into(),
    );
    let rid = package.add_part_relationship(
        &origin,
        rel::DIGITAL_SIGNATURE,
        &uri,
        RelationshipTargetMode::Internal,
    );
    Ok((rid, uri))
}

/// Remove all digital signature parts and the origin (C# delete digsig surface shell).
pub fn clear_digital_signatures(package: &mut OpcPackage) -> usize {
    let mut n = 0;
    for uri in digital_signature_parts(package) {
        if package.remove_part(&uri).is_some() {
            n += 1;
        }
    }
    let origin = PackUri::new(ORIGIN_URI);
    if package.remove_part(&origin).is_some() {
        n += 1;
    }
    // drop package rels of digsig origin type
    let ids: Vec<String> = package
        .package_relationships()
        .iter()
        .filter(|r| r.relationship_type == rel::DIGITAL_SIGNATURE_ORIGIN)
        .map(|r| r.id.clone())
        .collect();
    for id in ids {
        package.package_relationships_mut().remove(&id);
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opc::OpcPackage;

    #[test]
    fn no_digsig_is_ok() {
        let pkg = OpcPackage::create();
        assert!(validate_digital_signatures(&pkg).is_empty());
    }

    #[test]
    fn origin_without_rel_flagged() {
        let mut pkg = OpcPackage::create();
        pkg.set_part(
            ORIGIN_URI,
            content_type::DIGITAL_SIGNATURE_ORIGIN,
            Vec::<u8>::new(),
        );
        let errs = validate_digital_signatures(&pkg);
        assert!(
            errs.iter().any(|e| e.message.contains("without package relationship")),
            "{errs:?}"
        );
    }

    #[test]
    fn origin_with_rel_ok() {
        let mut pkg = OpcPackage::create();
        let uri = PackUri::new(ORIGIN_URI);
        pkg.set_part(
            uri.clone(),
            content_type::DIGITAL_SIGNATURE_ORIGIN,
            Vec::<u8>::new(),
        );
        pkg.add_package_relationship(
            rel::DIGITAL_SIGNATURE_ORIGIN,
            &uri,
            RelationshipTargetMode::Internal,
        );
        let errs = validate_digital_signatures(&pkg);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn add_and_clear_signature_parts() {
        let mut pkg = OpcPackage::create();
        assert!(!has_digital_signature_origin(&pkg));
        let (rid, uri) = add_digital_signature_part(
            &mut pkg,
            br#"<?xml version="1.0"?><Signature xmlns="http://www.w3.org/2000/09/xmldsig#"/>"#,
        )
        .unwrap();
        assert!(rid.starts_with('r'));
        assert!(has_digital_signature_origin(&pkg));
        assert!(digital_signature_parts(&pkg).contains(&uri));
        let errs = validate_digital_signatures(&pkg);
        assert!(errs.is_empty(), "{errs:?}");
        let n = clear_digital_signatures(&mut pkg);
        assert!(n >= 2);
        assert!(!has_digital_signature_origin(&pkg));
    }
}
