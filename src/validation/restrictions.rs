//! Simple-type restriction checks (C# `Validation/Schema/Restrictions`).

/// C# `TokenRestriction.VerifyTOKEN` — no CR/LF/TAB, no leading/trailing space,
/// no internal double spaces. Empty is valid.
pub fn verify_token(token: &str) -> bool {
    if token.is_empty() {
        return true;
    }
    if token.starts_with(' ') || token.ends_with(' ') {
        return false;
    }
    if token.contains(['\n', '\r', '\t']) {
        return false;
    }
    !token.contains("  ")
}

fn is_ncname_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_ncname_char(c: char) -> bool {
    is_ncname_start(c) || c == '-' || c == '.' || c.is_numeric()
}

/// XML `NCName` check (C# `XmlConvert.VerifyNCName` subset — no colon,
/// name-start char then name chars).
pub fn verify_ncname(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if is_ncname_start(first) => chars.all(is_ncname_char),
        _ => false,
    }
}

/// C# `QnameRestriction.IsValidQName` — `(Prefix ':')? LocalPart`, both NCNames.
pub fn is_valid_qname(qname: &str) -> bool {
    if qname.is_empty() {
        return false;
    }
    match qname.find(':') {
        Some(0) => false,
        Some(index) if index == qname.len() - 1 => false,
        Some(index) => {
            verify_ncname(&qname[..index]) && verify_ncname(&qname[index + 1..])
        }
        None => verify_ncname(qname),
    }
}

/// C# `AnyUriRestriction.Validate` — trims whitespace, rejects whitespace-only
/// and `##` values; any other relative or absolute reference is accepted.
pub fn validate_any_uri(uri: &str) -> bool {
    if uri.is_empty() {
        return true;
    }
    let trimmed = uri.trim_matches([' ', '\t', '\n', '\r']);
    if trimmed.is_empty() || trimmed.contains("##") {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_rules() {
        assert!(verify_token(""));
        assert!(verify_token("a b c"));
        assert!(!verify_token(" leading"));
        assert!(!verify_token("trailing "));
        assert!(!verify_token("tab\tinside"));
        assert!(!verify_token("line\nbreak"));
        assert!(!verify_token("double  space"));
    }

    #[test]
    fn ncname_and_qname_rules() {
        assert!(verify_ncname("w"));
        assert!(verify_ncname("_x1-2.3"));
        assert!(!verify_ncname(""));
        assert!(!verify_ncname("1abc"));
        assert!(!verify_ncname("a:b"));
        assert!(!verify_ncname("has space"));

        assert!(is_valid_qname("body"));
        assert!(is_valid_qname("w:body"));
        assert!(!is_valid_qname(""));
        assert!(!is_valid_qname(":body"));
        assert!(!is_valid_qname("w:"));
        assert!(!is_valid_qname("a:b:c"));
        assert!(!is_valid_qname("1w:body"));
        assert!(!is_valid_qname("w:1body"));
    }

    #[test]
    fn any_uri_rules() {
        assert!(validate_any_uri(""));
        assert!(validate_any_uri("relative/path.xml"));
        assert!(validate_any_uri("http://example.com/a?b#c"));
        assert!(validate_any_uri("  spaced  "));
        assert!(!validate_any_uri("   "));
        assert!(!validate_any_uri("bad##fragment"));
    }
}
