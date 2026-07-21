//! Open XML simple types (attribute / leaf value wrappers).
//!
//! Mirrors `DocumentFormat.OpenXml.Framework/SimpleTypes`.

use std::fmt;
use std::str::FromStr;

/// Trait for values that can be stored as Open XML attribute/text content.
pub trait OpenXmlSimpleType: Sized {
    fn as_inner_text(&self) -> String;
    fn from_inner_text(text: &str) -> Option<Self>;
}

/// String simple type.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StringValue(pub String);

impl OpenXmlSimpleType for StringValue {
    fn as_inner_text(&self) -> String {
        self.0.clone()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Some(Self(text.to_string()))
    }
}

impl From<&str> for StringValue {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for StringValue {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for StringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// OOXML on/off value: true/false/0/1/on/off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OnOffValue(pub bool);

impl OpenXmlSimpleType for OnOffValue {
    fn as_inner_text(&self) -> String {
        if self.0 {
            "1".into()
        } else {
            "0".into()
        }
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        match text {
            "true" | "1" | "on" | "True" => Some(Self(true)),
            "false" | "0" | "off" | "False" | "" => Some(Self(false)),
            _ => None,
        }
    }
}

impl From<bool> for OnOffValue {
    fn from(v: bool) -> Self {
        Self(v)
    }
}

/// Boolean value (`true`/`false`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BooleanValue(pub bool);

impl OpenXmlSimpleType for BooleanValue {
    fn as_inner_text(&self) -> String {
        if self.0 {
            "true".into()
        } else {
            "false".into()
        }
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        match text {
            "true" | "1" => Some(Self(true)),
            "false" | "0" => Some(Self(false)),
            _ => None,
        }
    }
}

macro_rules! int_value {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name(pub $ty);

        impl OpenXmlSimpleType for $name {
            fn as_inner_text(&self) -> String {
                self.0.to_string()
            }
            fn from_inner_text(text: &str) -> Option<Self> {
                text.parse().ok().map(Self)
            }
        }

        impl From<$ty> for $name {
            fn from(v: $ty) -> Self {
                Self(v)
            }
        }
    };
}

int_value!(Int32Value, i32);
int_value!(UInt32Value, u32);
int_value!(Int64Value, i64);
int_value!(IntegerValue, i64);
int_value!(Int16Value, i16);
int_value!(UInt16Value, u16);
int_value!(UInt64Value, u64);
int_value!(ByteValue, u8);
int_value!(SByteValue, i8);

/// Hex binary value (e.g. rsid attributes).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HexBinaryValue(pub String);

impl OpenXmlSimpleType for HexBinaryValue {
    fn as_inner_text(&self) -> String {
        self.0.clone()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Some(Self(text.to_string()))
    }
}

impl From<&str> for HexBinaryValue {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Base64 binary value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Base64BinaryValue(pub String);

impl OpenXmlSimpleType for Base64BinaryValue {
    fn as_inner_text(&self) -> String {
        self.0.clone()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Some(Self(text.to_string()))
    }
}

impl From<&str> for Base64BinaryValue {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// DateTime value stored as OOXML string (typically W3C / ISO-8601).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DateTimeValue(pub String);

impl OpenXmlSimpleType for DateTimeValue {
    fn as_inner_text(&self) -> String {
        self.0.clone()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Some(Self(text.to_string()))
    }
}

impl From<&str> for DateTimeValue {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Decimal value (stored as string to preserve precision).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DecimalValue(pub String);

impl OpenXmlSimpleType for DecimalValue {
    fn as_inner_text(&self) -> String {
        self.0.clone()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        Some(Self(text.to_string()))
    }
}

/// Single-precision floating value.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SingleValue(pub f32);

impl OpenXmlSimpleType for SingleValue {
    fn as_inner_text(&self) -> String {
        self.0.to_string()
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        f32::from_str(text).ok().map(Self)
    }
}

impl From<f32> for SingleValue {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

/// Double-precision floating value.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DoubleValue(pub f64);

impl OpenXmlSimpleType for DoubleValue {
    fn as_inner_text(&self) -> String {
        // Avoid unnecessary trailing zeros where possible
        let s = self.0.to_string();
        s
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        f64::from_str(text).ok().map(Self)
    }
}

impl From<f64> for DoubleValue {
    fn from(v: f64) -> Self {
        Self(v)
    }
}

/// True/false value (`true`/`false`/`t`/`f`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TrueFalseValue(pub bool);

impl OpenXmlSimpleType for TrueFalseValue {
    fn as_inner_text(&self) -> String {
        if self.0 {
            "true".into()
        } else {
            "false".into()
        }
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        match text {
            "true" | "t" | "True" | "1" => Some(Self(true)),
            "false" | "f" | "False" | "0" => Some(Self(false)),
            _ => None,
        }
    }
}

/// True/false/blank value (blank → false).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TrueFalseBlankValue(pub bool);

impl OpenXmlSimpleType for TrueFalseBlankValue {
    fn as_inner_text(&self) -> String {
        if self.0 {
            "true".into()
        } else {
            "false".into()
        }
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        match text {
            "true" | "t" | "True" | "1" => Some(Self(true)),
            "false" | "f" | "False" | "0" | "" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Space-separated list of simple values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListValue(pub Vec<String>);

impl OpenXmlSimpleType for ListValue {
    fn as_inner_text(&self) -> String {
        self.0.join(" ")
    }
    fn from_inner_text(text: &str) -> Option<Self> {
        let items = text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        Some(Self(items))
    }
}

impl From<Vec<String>> for ListValue {
    fn from(v: Vec<String>) -> Self {
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_off_parse() {
        assert_eq!(OnOffValue::from_inner_text("1").unwrap().0, true);
        assert_eq!(OnOffValue::from_inner_text("off").unwrap().0, false);
        assert_eq!(OnOffValue(true).as_inner_text(), "1");
    }

    #[test]
    fn int_roundtrip() {
        let v = Int32Value(42);
        assert_eq!(Int32Value::from_inner_text(&v.as_inner_text()).unwrap().0, 42);
    }
}
