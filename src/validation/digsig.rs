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
}
