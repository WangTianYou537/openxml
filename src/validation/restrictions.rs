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


/// C# `XsdType` — XML Schema built-in simple types used by attribute validators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XsdType {
    AnySimpleType,
    String,
    NormalizedString,
    Token,
    Base64Binary,
    HexBinary,
    Integer,
    PositiveInteger,
    NegativeInteger,
    NonNegativeInteger,
    NonPositiveInteger,
    Long,
    UnsignedLong,
    Int,
    UnsignedInt,
    Short,
    UnsignedShort,
    Byte,
    UnsignedByte,
    Decimal,
    Float,
    Double,
    Boolean,
    Duration,
    DateTime,
    Date,
    Time,
    GYear,
    GYearMonth,
    GMonth,
    GMonthDay,
    GDay,
    Name,
    QName,
    NCName,
    AnyURI,
    Language,
    Id,
    IdRef,
    IdRefs,
    Entity,
    Entities,
    Notation,
    NmToken,
    NmTokens,
    Enum,
    List,
    Union,
    Redirected,
    SpecialBoolean,
}

impl XsdType {
    /// Map generated attribute `type_name` strings onto an [`XsdType`].
    pub fn from_type_name(type_name: &str) -> Option<Self> {
        Some(match type_name {
            "StringValue" | "String" => Self::String,
            "HexBinaryValue" | "HexBinary" => Self::HexBinary,
            "Base64BinaryValue" | "Base64Binary" => Self::Base64Binary,
            "IntegerValue" | "Integer" => Self::Integer,
            "Int32Value" | "Int" => Self::Int,
            "UInt32Value" | "UnsignedInt" => Self::UnsignedInt,
            "Int16Value" | "Short" => Self::Short,
            "UInt16Value" | "UnsignedShort" => Self::UnsignedShort,
            "ByteValue" | "Byte" | "UnsignedByte" => Self::UnsignedByte,
            "OnOffValue" | "TrueFalseValue" | "TrueFalseBlankValue" | "BooleanValue" => {
                Self::SpecialBoolean
            }
            "DateTimeValue" | "DateTime" => Self::DateTime,
            "EnumValue" | "Enum" => Self::Enum,
            "Token" => Self::Token,
            "QName" | "QnameValue" => Self::QName,
            "AnyURI" | "AnyUriValue" => Self::AnyURI,
            "NCName" => Self::NCName,
            "DecimalValue" | "Decimal" => Self::Decimal,
            "DoubleValue" | "Double" => Self::Double,
            "SingleValue" | "Float" => Self::Float,
            _ => return None,
        })
    }

    /// Lexical validation for a value of this type (C# simple-type restrictions shell).
    pub fn validate_lexical(self, value: &str) -> bool {
        match self {
            Self::Token => verify_token(value),
            Self::QName => is_valid_qname(value),
            Self::NCName => verify_ncname(value),
            Self::AnyURI => validate_any_uri(value),
            Self::HexBinary => {
                !value.is_empty()
                    && value.len() % 2 == 0
                    && value.chars().all(|c| c.is_ascii_hexdigit())
            }
            Self::Base64Binary => value.chars().all(|c| {
                c.is_ascii_alphanumeric()
                    || c == '+'
                    || c == '/'
                    || c == '='
                    || c.is_ascii_whitespace()
            }),
            Self::Integer | Self::Long | Self::Int | Self::Short => value.parse::<i64>().is_ok(),
            Self::NonNegativeInteger | Self::UnsignedLong | Self::UnsignedInt
            | Self::UnsignedShort | Self::UnsignedByte | Self::Byte => {
                value.parse::<u64>().is_ok()
            }
            Self::PositiveInteger => value.parse::<u64>().ok().is_some_and(|v| v > 0),
            Self::NegativeInteger => value.parse::<i64>().ok().is_some_and(|v| v < 0),
            Self::NonPositiveInteger => value.parse::<i64>().ok().is_some_and(|v| v <= 0),
            Self::Decimal | Self::Float | Self::Double => value.parse::<f64>().is_ok(),
            Self::Boolean | Self::SpecialBoolean => {
                matches!(value, "true" | "false" | "1" | "0" | "on" | "off")
            }
            Self::DateTime | Self::Date => {
                value.len() >= 10
                    && value.as_bytes().get(4) == Some(&b'-')
                    && value.as_bytes().get(7) == Some(&b'-')
            }
            Self::Name => verify_ncname(value) || is_valid_qname(value),
            Self::String
            | Self::NormalizedString
            | Self::Enum
            | Self::List
            | Self::Union
            | Self::Redirected
            | Self::AnySimpleType
            | Self::Language
            | Self::Id
            | Self::IdRef
            | Self::IdRefs
            | Self::Entity
            | Self::Entities
            | Self::Notation
            | Self::NmToken
            | Self::NmTokens
            | Self::Duration
            | Self::Time
            | Self::GYear
            | Self::GYearMonth
            | Self::GMonth
            | Self::GMonthDay
            | Self::GDay => true,
        }
    }
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

    #[test]
    fn xsd_type_from_type_name_and_lexical() {
        assert_eq!(XsdType::from_type_name("HexBinaryValue"), Some(XsdType::HexBinary));
        assert_eq!(XsdType::from_type_name("OnOffValue"), Some(XsdType::SpecialBoolean));
        assert!(XsdType::HexBinary.validate_lexical("00AB"));
        assert!(!XsdType::HexBinary.validate_lexical("0G"));
        assert!(XsdType::Token.validate_lexical("a b"));
        assert!(!XsdType::Token.validate_lexical(" a"));
        assert!(XsdType::QName.validate_lexical("w:val"));
        assert!(XsdType::AnyURI.validate_lexical("http://example.com"));
    }
}
