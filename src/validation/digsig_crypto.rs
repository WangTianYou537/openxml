//! Digital signature digest integrity (XML-DSig References).
//!
//! Verifies that each `Reference` in signature parts hashes the targeted package
//! part with the declared digest algorithm (SHA-1 / SHA-256 / SHA-384 / SHA-512).
//!
//! Also provides RSA-SHA256 sign/verify of `SignatureValue` over a simplified
//! C14N of `SignedInfo` (sufficient for signatures produced by this crate).
//! Structure wiring is covered by [`super::validate_digital_signatures`].

use super::ValidationError;
use crate::element::{parse_element, OpenXmlElement};
use crate::namespace::rel;
use crate::opc::{OpcPackage, PackUri, RelationshipTargetMode};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use sha1::{Digest as _, Sha1};
use sha2::{Sha256, Sha384, Sha512};

const ORIGIN_URI: &str = "/_xmlsignatures/origin.sigs";

/// Result of digest verification for one signature part.
#[derive(Debug, Clone)]
pub struct DigestCheckResult {
    pub signature_uri: String,
    pub reference_uri: String,
    pub algorithm: String,
    pub ok: bool,
    pub message: String,
}

fn is_sig_rel(r: &crate::opc::Relationship) -> bool {
    r.relationship_type == rel::DIGITAL_SIGNATURE
        || r.relationship_type.ends_with("/digital-signature/signature")
        || r.relationship_type.contains("digital-signature")
}

fn digest_algo_name(uri: &str) -> Option<&'static str> {
    let u = uri.to_ascii_lowercase();
    if u.contains("sha256") || u.ends_with("#sha256") {
        Some("sha256")
    } else if u.contains("sha384") || u.ends_with("#sha384") {
        Some("sha384")
    } else if u.contains("sha512") || u.ends_with("#sha512") {
        Some("sha512")
    } else if u.contains("sha1") || u.ends_with("#sha1") || u.contains("sha-1") {
        Some("sha1")
    } else {
        None
    }
}

fn compute_digest(algo: &str, data: &[u8]) -> Option<Vec<u8>> {
    Some(match algo {
        "sha1" => Sha1::digest(data).to_vec(),
        "sha256" => Sha256::digest(data).to_vec(),
        "sha384" => Sha384::digest(data).to_vec(),
        "sha512" => Sha512::digest(data).to_vec(),
        _ => return None,
    })
}

fn find_child<'a>(el: &'a OpenXmlElement, name: &str) -> Option<&'a OpenXmlElement> {
    el.child(name)
        .or_else(|| el.children.iter().find(|c| c.local_name == name))
}

fn find_descendants<'a>(el: &'a OpenXmlElement, name: &str) -> Vec<&'a OpenXmlElement> {
    std::iter::once(el)
        .chain(el.descendants())
        .filter(|e| e.local_name == name)
        .collect()
}

fn attr_any<'a>(el: &'a OpenXmlElement, name: &str) -> Option<&'a str> {
    el.get_attribute(name)
        .or_else(|| {
            el.attributes
                .iter()
                .find(|a| a.local_name == name)
                .map(|a| a.value.as_str())
        })
}

/// Collect signature part URIs related from the origin.
pub fn signature_part_uris(package: &OpcPackage) -> Vec<PackUri> {
    let origin = PackUri::new(ORIGIN_URI);
    let mut out = Vec::new();
    if let Some(rels) = package.part_relationships(&origin) {
        for r in rels.iter() {
            if r.target_mode == RelationshipTargetMode::External || !is_sig_rel(r) {
                continue;
            }
            if let Ok(uri) = package.resolve_relationship(Some(&origin), r) {
                if package.has_part(&uri) {
                    out.push(uri);
                }
            }
        }
    }
    out
}

/// Resolve a Reference URI relative to the signature part.
fn resolve_ref_uri(sig_uri: &PackUri, reference: &str) -> Option<PackUri> {
    let reference = reference.trim();
    if reference.is_empty() || reference.starts_with('#') {
        // Same-document fragment (SignedProperties etc.) — skip package part digest
        return None;
    }
    if reference.starts_with('/') {
        return Some(PackUri::new(reference));
    }
    // Relative to signature part directory
    crate::opc::resolve_uri(sig_uri, reference).ok()
}

/// Verify package-part digests declared in XML-DSig signature parts.
///
/// For each `Reference` with a package URI and `DigestMethod`/`DigestValue`,
/// recomputes the digest over the part bytes and compares.
pub fn verify_signature_digests(package: &OpcPackage) -> Vec<DigestCheckResult> {
    let mut results = Vec::new();
    for sig_uri in signature_part_uris(package) {
        let Some(bytes) = package.get_part(&sig_uri) else {
            continue;
        };
        let Ok(xml) = std::str::from_utf8(bytes) else {
            results.push(DigestCheckResult {
                signature_uri: sig_uri.as_str().into(),
                reference_uri: String::new(),
                algorithm: String::new(),
                ok: false,
                message: "signature part is not UTF-8 XML".into(),
            });
            continue;
        };
        let Ok(root) = parse_element(xml) else {
            results.push(DigestCheckResult {
                signature_uri: sig_uri.as_str().into(),
                reference_uri: String::new(),
                algorithm: String::new(),
                ok: false,
                message: "failed to parse signature XML".into(),
            });
            continue;
        };

        for reference in find_descendants(&root, "Reference") {
            let Some(uri_attr) = attr_any(reference, "URI") else {
                continue;
            };
            let Some(target) = resolve_ref_uri(&sig_uri, uri_attr) else {
                continue; // fragment or unresolvable
            };
            let method = find_child(reference, "DigestMethod")
                .and_then(|m| attr_any(m, "Algorithm"))
                .unwrap_or("");
            let Some(algo) = digest_algo_name(method) else {
                results.push(DigestCheckResult {
                    signature_uri: sig_uri.as_str().into(),
                    reference_uri: uri_attr.into(),
                    algorithm: method.into(),
                    ok: false,
                    message: format!("unsupported digest algorithm `{method}`"),
                });
                continue;
            };
            let Some(digest_el) = find_child(reference, "DigestValue") else {
                results.push(DigestCheckResult {
                    signature_uri: sig_uri.as_str().into(),
                    reference_uri: uri_attr.into(),
                    algorithm: algo.into(),
                    ok: false,
                    message: "missing DigestValue".into(),
                });
                continue;
            };
            let digest_b64 = digest_el.inner_text();
            let Ok(expected) = B64.decode(digest_b64.trim()) else {
                results.push(DigestCheckResult {
                    signature_uri: sig_uri.as_str().into(),
                    reference_uri: uri_attr.into(),
                    algorithm: algo.into(),
                    ok: false,
                    message: "DigestValue is not valid base64".into(),
                });
                continue;
            };
            let Some(part_bytes) = package.get_part(&target) else {
                results.push(DigestCheckResult {
                    signature_uri: sig_uri.as_str().into(),
                    reference_uri: uri_attr.into(),
                    algorithm: algo.into(),
                    ok: false,
                    message: format!("referenced part `{}` missing", target.as_str()),
                });
                continue;
            };
            let Some(actual) = compute_digest(algo, part_bytes) else {
                continue;
            };
            if actual == expected {
                results.push(DigestCheckResult {
                    signature_uri: sig_uri.as_str().into(),
                    reference_uri: uri_attr.into(),
                    algorithm: algo.into(),
                    ok: true,
                    message: "digest ok".into(),
                });
            } else {
                results.push(DigestCheckResult {
                    signature_uri: sig_uri.as_str().into(),
                    reference_uri: uri_attr.into(),
                    algorithm: algo.into(),
                    ok: false,
                    message: format!(
                        "digest mismatch for `{}` ({algo})",
                        target.as_str()
                    ),
                });
            }
        }
    }
    results
}

/// Convert digest check failures into [`ValidationError`]s.
pub fn validate_signature_digests(package: &OpcPackage) -> Vec<ValidationError> {
    verify_signature_digests(package)
        .into_iter()
        .filter(|r| !r.ok)
        .map(|r| ValidationError {
            path: if r.reference_uri.is_empty() {
                r.signature_uri
            } else {
                format!("{}#{}", r.signature_uri, r.reference_uri)
            },
            message: r.message,
            ..Default::default()
        })
        .collect()
}

/// Build a minimal XML-DSig signature part body with SHA-256 digests of the given parts.
///
/// When `signature_value_b64` is `None`, `SignatureValue` is left empty.
pub fn build_signature_xml(package: &OpcPackage, part_uris: &[PackUri]) -> String {
    build_signature_xml_with_value(package, part_uris, None)
}

/// Build signature XML, optionally filling `SignatureValue` with the given base64 bytes.
pub fn build_signature_xml_with_value(
    package: &OpcPackage,
    part_uris: &[PackUri],
    signature_value_b64: Option<&str>,
) -> String {
    let mut refs = String::new();
    for uri in part_uris {
        let Some(bytes) = package.get_part(uri) else {
            continue;
        };
        let digest = Sha256::digest(bytes);
        let b64 = B64.encode(digest);
        let href = uri.as_str();
        refs.push_str(&format!(
            r#"<Reference URI="{href}"><DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256"/><DigestValue>{b64}</DigestValue></Reference>"#
        ));
    }
    let sig_val = signature_value_b64.unwrap_or("");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Signature xmlns="http://www.w3.org/2000/09/xmldsig#">
  <SignedInfo>
    <CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/>
    <SignatureMethod Algorithm="http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"/>
    {refs}
  </SignedInfo>
  <SignatureValue>{sig_val}</SignatureValue>
</Signature>
"#
    )
}

/// Extract the raw `<SignedInfo>...</SignedInfo>` substring for signing/verification.
///
/// Uses inclusive exclusive-c14n-friendly extraction of the first SignedInfo element.
pub fn extract_signed_info_xml(signature_xml: &str) -> Option<String> {
    let start = signature_xml.find("<SignedInfo")?;
    let after = &signature_xml[start..];
    let end_tag = "</SignedInfo>";
    let end_rel = after.find(end_tag)?;
    Some(after[..end_rel + end_tag.len()].to_string())
}

/// Exclusive-ish C14N subset: strip insignificant whitespace between tags and normalize.
///
/// Full W3C C14N is not implemented; this is sufficient for signatures we produce
/// with our own `build_signature_xml` (deterministic layout, no comments/PI).
pub fn simple_c14n_signed_info(signed_info_xml: &str) -> Vec<u8> {
    // Collapse whitespace between tags; keep attribute order as authored.
    let mut out = String::with_capacity(signed_info_xml.len());
    let mut in_tag = false;
    let mut prev_space = false;
    for ch in signed_info_xml.chars() {
        match ch {
            '<' => {
                in_tag = true;
                prev_space = false;
                out.push(ch);
            }
            '>' => {
                in_tag = false;
                prev_space = false;
                out.push(ch);
            }
            c if c.is_whitespace() && !in_tag => {
                if !prev_space && !out.ends_with('>') && !out.is_empty() {
                    // drop inter-element whitespace entirely for our compact form
                }
                prev_space = true;
            }
            c => {
                prev_space = false;
                out.push(c);
            }
        }
    }
    // Prefer compact form: remove all whitespace between tags
    let compact: String = out
        .split('>')
        .map(|part| {
            if let Some(idx) = part.rfind('<') {
                // text node before next tag — keep trimmed text
                let (text, tag) = part.split_at(idx);
                format!("{}{}", text.trim(), tag)
            } else {
                part.trim_end().to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(">");
    compact.into_bytes()
}

/// RSA-SHA256 sign the canonicalized SignedInfo bytes.
pub fn rsa_sha256_sign(private_key_pem: &str, signed_info_c14n: &[u8]) -> Result<Vec<u8>, String> {
    use rsa::pkcs1v15::SigningKey;
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::signature::{SignatureEncoding, Signer};
    use rsa::RsaPrivateKey;
    use sha2::Sha256;

    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .map_err(|e| format!("parse private key (PKCS#8 PEM): {e}"))?;
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let sig = signing_key.sign(signed_info_c14n);
    Ok(sig.to_bytes().to_vec())
}

/// RSA-SHA256 verify SignatureValue over canonicalized SignedInfo.
pub fn rsa_sha256_verify(
    public_key_pem: &str,
    signed_info_c14n: &[u8],
    signature: &[u8],
) -> Result<(), String> {
    use rsa::pkcs1v15::{Signature, VerifyingKey};
    use rsa::signature::Verifier;
    use rsa::RsaPublicKey;
    use sha2::Sha256;
    use spki::DecodePublicKey;

    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)
        .map_err(|e| format!("parse public key (SPKI PEM): {e}"))?;
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    let sig = Signature::try_from(signature).map_err(|e| format!("bad signature: {e}"))?;
    verifying_key
        .verify(signed_info_c14n, &sig)
        .map_err(|e| format!("verify failed: {e}"))
}

/// Build signature XML with RSA-SHA256 `SignatureValue` over SignedInfo.
pub fn build_signed_signature_xml(
    package: &OpcPackage,
    part_uris: &[PackUri],
    private_key_pem: &str,
) -> Result<String, String> {
    let unsigned = build_signature_xml_with_value(package, part_uris, None);
    let signed_info = extract_signed_info_xml(&unsigned).ok_or("missing SignedInfo")?;
    let c14n = simple_c14n_signed_info(&signed_info);
    let sig = rsa_sha256_sign(private_key_pem, &c14n)?;
    let b64 = B64.encode(&sig);
    Ok(build_signature_xml_with_value(
        package,
        part_uris,
        Some(&b64),
    ))
}

/// Verify RSA-SHA256 SignatureValue in a signature XML string against a public key.
pub fn verify_signature_value(signature_xml: &str, public_key_pem: &str) -> Result<(), String> {
    let signed_info = extract_signed_info_xml(signature_xml).ok_or("missing SignedInfo")?;
    let c14n = simple_c14n_signed_info(&signed_info);
    // extract SignatureValue text
    let start = signature_xml
        .find("<SignatureValue")
        .ok_or("missing SignatureValue")?;
    let after = &signature_xml[start..];
    let gt = after.find('>').ok_or("bad SignatureValue")?;
    let rest = &after[gt + 1..];
    let end = rest.find("</SignatureValue>").ok_or("unclosed SignatureValue")?;
    let b64 = rest[..end].trim();
    if b64.is_empty() {
        return Err("empty SignatureValue".into());
    }
    let sig = B64
        .decode(b64)
        .map_err(|e| format!("SignatureValue base64: {e}"))?;
    rsa_sha256_verify(public_key_pem, &c14n, &sig)
}

/// Compute a hex-encoded digest of raw bytes (helper for tests / callers).
pub fn digest_hex(algo: &str, data: &[u8]) -> Option<String> {
    compute_digest(algo, data).map(|d| {
        d.iter().map(|b| format!("{b:02x}")).collect::<String>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::content_type;
    use crate::opc::OpcPackage;

    #[test]
    fn digest_hex_sha256() {
        let h = digest_hex("sha256", b"abc").unwrap();
        assert_eq!(
            h,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn build_and_verify_signature_digests() {
        let mut pkg = OpcPackage::create();
        pkg.set_part(
            "/word/document.xml",
            content_type::WORD_DOCUMENT,
            b"<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"/>"
                .to_vec(),
        );
        let doc_uri = PackUri::new("/word/document.xml");
        let xml = build_signature_xml(&pkg, &[doc_uri.clone()]);

        // Install origin + signature
        let origin = PackUri::new(ORIGIN_URI);
        pkg.set_part(
            origin.clone(),
            content_type::DIGITAL_SIGNATURE_ORIGIN,
            Vec::new(),
        );
        pkg.add_package_relationship(
            rel::DIGITAL_SIGNATURE_ORIGIN,
            &origin,
            RelationshipTargetMode::Internal,
        );
        let sig_uri = PackUri::new("/_xmlsignatures/sig1.xml");
        pkg.set_part(
            sig_uri.clone(),
            content_type::DIGITAL_SIGNATURE_XML,
            xml.into_bytes(),
        );
        pkg.add_part_relationship(
            &origin,
            rel::DIGITAL_SIGNATURE,
            &sig_uri,
            RelationshipTargetMode::Internal,
        );

        let results = verify_signature_digests(&pkg);
        assert!(
            results.iter().any(|r| r.ok && r.reference_uri.contains("document")),
            "{results:?}"
        );
        assert!(validate_signature_digests(&pkg).is_empty());

        // Tamper with document → digest fails
        pkg.set_part(
            doc_uri,
            content_type::WORD_DOCUMENT,
            b"<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:body/></w:document>"
                .to_vec(),
        );
        let errs = validate_signature_digests(&pkg);
        assert!(
            errs.iter().any(|e| e.message.contains("mismatch")),
            "{errs:?}"
        );
    }

    #[test]
    fn rsa_sign_and_verify_signed_info() {
        use rand::rngs::OsRng;
        use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
        use rsa::{RsaPrivateKey, RsaPublicKey};

        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("keygen");
        let public_key = RsaPublicKey::from(&private_key);
        let priv_pem = private_key
            .to_pkcs8_pem(LineEnding::LF)
            .unwrap()
            .to_string();
        let pub_pem = public_key
            .to_public_key_pem(LineEnding::LF)
            .unwrap();

        let mut pkg = OpcPackage::create();
        pkg.set_part(
            "/word/document.xml",
            content_type::WORD_DOCUMENT,
            b"<w:document/>".to_vec(),
        );
        let xml = build_signed_signature_xml(
            &pkg,
            &[PackUri::new("/word/document.xml")],
            &priv_pem,
        )
        .expect("sign");
        assert!(xml.contains("<SignatureValue>") && !xml.contains("<SignatureValue></SignatureValue>"));
        verify_signature_value(&xml, &pub_pem).expect("verify");

        // Tamper SignatureValue
        let bad = xml.replace("SignatureValue>", "SignatureValue>YQ==</bogus>");
        // force invalid by flipping a char in base64 if present
        let bad = {
            if let Some(start) = xml.find("<SignatureValue>") {
                let s = start + "<SignatureValue>".len();
                let end = xml.find("</SignatureValue>").unwrap();
                let mut b64 = xml[s..end].to_string();
                let flipped: String = b64
                    .chars()
                    .enumerate()
                    .map(|(i, c)| if i == 0 { if c == 'A' { 'B' } else { 'A' } } else { c })
                    .collect();
                format!(
                    "{}{}{}",
                    &xml[..s],
                    flipped,
                    &xml[end..]
                )
            } else {
                bad
            }
        };
        assert!(verify_signature_value(&bad, &pub_pem).is_err());
    }
}
