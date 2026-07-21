//! Office Open XML file format versions (mirrors C# `FileFormatVersions`).

use std::fmt;

/// Bitflags for Office Open XML format versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FileFormatVersions(u32);

impl FileFormatVersions {
    pub const NONE: Self = Self(0);
    pub const OFFICE2007: Self = Self(0x1);
    pub const OFFICE2010: Self = Self(0x2);
    pub const OFFICE2013: Self = Self(0x4);
    pub const OFFICE2016: Self = Self(0x8);
    pub const OFFICE2019: Self = Self(0x10);
    pub const OFFICE2021: Self = Self(0x20);
    pub const MICROSOFT365: Self = Self(0x4000_0000);

    /// All known versions.
    pub const ALL: Self = Self(
        Self::OFFICE2007.0
            | Self::OFFICE2010.0
            | Self::OFFICE2013.0
            | Self::OFFICE2016.0
            | Self::OFFICE2019.0
            | Self::OFFICE2021.0
            | Self::MICROSOFT365.0,
    );

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// This version and all later ones (for target processing).
    pub const fn and_later(self) -> Self {
        match self {
            Self::OFFICE2007 => Self::ALL,
            Self::OFFICE2010 => Self(
                Self::OFFICE2010.0
                    | Self::OFFICE2013.0
                    | Self::OFFICE2016.0
                    | Self::OFFICE2019.0
                    | Self::OFFICE2021.0
                    | Self::MICROSOFT365.0,
            ),
            Self::OFFICE2013 => Self(
                Self::OFFICE2013.0
                    | Self::OFFICE2016.0
                    | Self::OFFICE2019.0
                    | Self::OFFICE2021.0
                    | Self::MICROSOFT365.0,
            ),
            Self::OFFICE2016 => Self(
                Self::OFFICE2016.0
                    | Self::OFFICE2019.0
                    | Self::OFFICE2021.0
                    | Self::MICROSOFT365.0,
            ),
            Self::OFFICE2019 => {
                Self(Self::OFFICE2019.0 | Self::OFFICE2021.0 | Self::MICROSOFT365.0)
            }
            Self::OFFICE2021 => Self(Self::OFFICE2021.0 | Self::MICROSOFT365.0),
            Self::MICROSOFT365 => Self::MICROSOFT365,
            _ => self,
        }
    }

    /// Ordinal for comparison (lower = earlier). Unknown → 0.
    pub fn order(self) -> u32 {
        if self.contains(Self::OFFICE2007) && self.0 == Self::OFFICE2007.0 {
            return 1;
        }
        // single-flag ordering
        match self.0 {
            x if x == Self::OFFICE2007.0 => 1,
            x if x == Self::OFFICE2010.0 => 2,
            x if x == Self::OFFICE2013.0 => 3,
            x if x == Self::OFFICE2016.0 => 4,
            x if x == Self::OFFICE2019.0 => 5,
            x if x == Self::OFFICE2021.0 => 6,
            x if x == Self::MICROSOFT365.0 => 7,
            _ => {
                // multi-bit: use lowest set
                if self.intersects(Self::OFFICE2007) {
                    1
                } else if self.intersects(Self::OFFICE2010) {
                    2
                } else if self.intersects(Self::OFFICE2013) {
                    3
                } else if self.intersects(Self::OFFICE2016) {
                    4
                } else if self.intersects(Self::OFFICE2019) {
                    5
                } else if self.intersects(Self::OFFICE2021) {
                    6
                } else if self.intersects(Self::MICROSOFT365) {
                    7
                } else {
                    0
                }
            }
        }
    }

    /// Parse from the `Version` field in `namespaces.json`.
    pub fn from_namespace_version(s: &str) -> Self {
        match s {
            "Office2007" => Self::OFFICE2007,
            "Office2010" => Self::OFFICE2010,
            "Office2013" => Self::OFFICE2013,
            "Office2016" => Self::OFFICE2016,
            "Office2019" => Self::OFFICE2019,
            "Office2021" => Self::OFFICE2021,
            "Microsoft365" => Self::MICROSOFT365,
            _ => Self::OFFICE2007,
        }
    }

    /// Whether content introduced in `intro` is available when targeting `self`.
    pub fn includes_introduction(self, intro: Self) -> bool {
        // Target is a single version or a set; content is available if target
        // includes the intro version or any later version that subsumes it.
        if self == Self::ALL || self.contains(Self::ALL) {
            return true;
        }
        // If target is "and later" style multi-bit, check intersection with intro.and_later()
        if self.intersects(intro.and_later()) {
            return true;
        }
        // Single target version: available if intro.order <= target.order
        intro.order() > 0 && intro.order() <= self.order()
    }
}

impl fmt::Display for FileFormatVersions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::NONE {
            return write!(f, "None");
        }
        let mut parts = Vec::new();
        if self.intersects(Self::OFFICE2007) {
            parts.push("Office2007");
        }
        if self.intersects(Self::OFFICE2010) {
            parts.push("Office2010");
        }
        if self.intersects(Self::OFFICE2013) {
            parts.push("Office2013");
        }
        if self.intersects(Self::OFFICE2016) {
            parts.push("Office2016");
        }
        if self.intersects(Self::OFFICE2019) {
            parts.push("Office2019");
        }
        if self.intersects(Self::OFFICE2021) {
            parts.push("Office2021");
        }
        if self.intersects(Self::MICROSOFT365) {
            parts.push("Microsoft365");
        }
        write!(f, "{}", parts.join("|"))
    }
}

/// Prefix → introduction version for well-known Office namespaces.
///
/// The generated table in [`crate::generated::namespaces::PREFIX_INTRODUCED_IN`]
/// (from `namespaces.json`) is preferred when available; this bootstrap list
/// covers core prefixes used by unit tests and early init.
pub static PREFIX_INTRODUCED_IN: &[(&str, FileFormatVersions)] = &[
    ("w", FileFormatVersions::OFFICE2007),
    ("r", FileFormatVersions::OFFICE2007),
    ("a", FileFormatVersions::OFFICE2007),
    ("p", FileFormatVersions::OFFICE2007),
    ("x", FileFormatVersions::OFFICE2007),
    ("c", FileFormatVersions::OFFICE2007),
    ("xdr", FileFormatVersions::OFFICE2007),
    ("wp", FileFormatVersions::OFFICE2007),
    ("m", FileFormatVersions::OFFICE2007),
    ("mc", FileFormatVersions::OFFICE2007),
    ("cp", FileFormatVersions::OFFICE2007),
    ("dc", FileFormatVersions::OFFICE2007),
    ("dcterms", FileFormatVersions::OFFICE2007),
    ("w14", FileFormatVersions::OFFICE2010),
    ("a14", FileFormatVersions::OFFICE2010),
    ("p14", FileFormatVersions::OFFICE2010),
    ("x14", FileFormatVersions::OFFICE2010),
    ("c14", FileFormatVersions::OFFICE2010),
    ("w15", FileFormatVersions::OFFICE2013),
    ("a15", FileFormatVersions::OFFICE2013),
    ("p15", FileFormatVersions::OFFICE2013),
    ("x15", FileFormatVersions::OFFICE2013),
    ("w16", FileFormatVersions::OFFICE2016),
    ("a16", FileFormatVersions::OFFICE2016),
    ("w16cid", FileFormatVersions::OFFICE2019),
    ("w16se", FileFormatVersions::OFFICE2019),
    ("w16cex", FileFormatVersions::OFFICE2021),
    ("w16du", FileFormatVersions::MICROSOFT365),
];

/// Look up when a namespace prefix was introduced.
///
/// Uses the generated full table when present, otherwise the bootstrap list.
pub fn prefix_introduced_in(prefix: &str) -> FileFormatVersions {
    // Prefer generated table (complete namespaces.json).
    for (p, v) in crate::generated::namespaces::PREFIX_INTRODUCED_IN {
        if *p == prefix {
            return *v;
        }
    }
    PREFIX_INTRODUCED_IN
        .iter()
        .find(|(p, _)| *p == prefix)
        .map(|(_, v)| *v)
        .unwrap_or(FileFormatVersions::OFFICE2007)
}

/// Prefixes considered "supported" when targeting `version`.
pub fn supported_prefixes(version: FileFormatVersions) -> Vec<&'static str> {
    // Use generated full table if it has entries; else bootstrap.
    let table = crate::generated::namespaces::PREFIX_INTRODUCED_IN;
    if !table.is_empty() {
        return table
            .iter()
            .filter(|(_, intro)| version.includes_introduction(*intro))
            .map(|(p, _)| *p)
            .collect();
    }
    PREFIX_INTRODUCED_IN
        .iter()
        .filter(|(_, intro)| version.includes_introduction(*intro))
        .map(|(p, _)| *p)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_later() {
        let v = FileFormatVersions::OFFICE2013.and_later();
        assert!(v.intersects(FileFormatVersions::OFFICE2013));
        assert!(v.intersects(FileFormatVersions::OFFICE2016));
        assert!(!v.intersects(FileFormatVersions::OFFICE2010));
    }

    #[test]
    fn includes_introduction() {
        let target = FileFormatVersions::OFFICE2010;
        assert!(target.includes_introduction(FileFormatVersions::OFFICE2007));
        assert!(target.includes_introduction(FileFormatVersions::OFFICE2010));
        assert!(!target.includes_introduction(FileFormatVersions::OFFICE2013));
    }

    #[test]
    fn supported_prefixes_2010() {
        let p = supported_prefixes(FileFormatVersions::OFFICE2010);
        assert!(p.contains(&"w"));
        assert!(p.contains(&"w14"));
        assert!(!p.contains(&"w15"));
    }
}
