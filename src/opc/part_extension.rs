//! Content-type → file-extension map (C# `PartExtensionProvider` / `IPartExtensionFeature`).

use std::collections::HashMap;

/// Maps MIME content types to part file extensions (including the leading dot).
#[derive(Debug, Clone)]
pub struct PartExtensionProvider {
    map: HashMap<String, String>,
}

impl Default for PartExtensionProvider {
    fn default() -> Self {
        Self::with_known_extensions()
    }
}

impl PartExtensionProvider {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn with_known_extensions() -> Self {
        let mut p = Self::new();
        p.add_known();
        p
    }

    /// Register or replace the extension for `content_type`.
    ///
    /// `extension` may be with or without a leading dot; stored form always has one.
    pub fn register(&mut self, content_type: impl Into<String>, extension: impl AsRef<str>) {
        let ext = extension.as_ref();
        let ext = if ext.starts_with('.') {
            ext.to_string()
        } else {
            format!(".{ext}")
        };
        self.map.insert(content_type.into(), ext);
    }

    pub fn try_get_extension(&self, content_type: &str) -> Option<&str> {
        self.map.get(content_type).map(|s| s.as_str())
    }

    /// Extension for `content_type`, or `".bin"` if unknown.
    pub fn extension_or_bin(&self, content_type: &str) -> &str {
        self.try_get_extension(content_type).unwrap_or(".bin")
    }

    pub fn contains(&self, content_type: &str) -> bool {
        self.map.contains_key(content_type)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn add_known(&mut self) {
        const KNOWN: &[(&str, &str)] = &[
            ("image/unknown", ".bin"),
            ("image/bmp", ".bmp"),
            ("image/gif", ".gif"),
            ("image/png", ".png"),
            ("image/jp2", ".jp2"),
            ("image/tif", ".tif"),
            ("image/tiff", ".tiff"),
            ("image/xbm", ".xbm"),
            ("image/x-icon", ".ico"),
            ("image/x-pcx", ".pcx"),
            ("image/x-pcz", ".pcz"),
            ("image/x-emz", ".emz"),
            ("image/x-wmz", ".wmz"),
            ("image/jpeg", ".jpeg"),
            ("image/x-emf", ".emf"),
            ("image/x-wmf", ".wmf"),
            ("image/svg+xml", ".svg"),
            ("audio/aiff", ".aiff"),
            ("audio/midi", ".midi"),
            ("audio/mp3", ".mp3"),
            ("audio/mpegurl", ".m3u"),
            ("audio/wav", ".wav"),
            ("audio/x-ms-wma", ".wma"),
            ("audio/mpeg", ".mpeg"),
            ("audio/ogg", ".ogg"),
            ("video/x-ms-asf-plugin", ".asx"),
            ("video/avi", ".avi"),
            ("video/mp4", ".mp4"),
            ("video/mpg", ".mpg"),
            ("video/mpeg", ".mpeg"),
            ("video/ogg", ".ogg"),
            ("video/x-ms-asf", ".asf"),
            ("video/x-ms-wmv", ".wmv"),
            ("video/x-ms-wmx", ".wmx"),
            ("video/x-ms-wvx", ".wvx"),
            ("video/quicktime", ".mov"),
            ("application/vnd.openxmlformats-officedocument.oleObject", ".bin"),
            ("application/vnd.openxmlformats-officedocument.vmlDrawing", ".vml"),
        ];
        for (ct, ext) in KNOWN {
            self.register(*ct, *ext);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_image_and_register() {
        let mut p = PartExtensionProvider::default();
        assert_eq!(p.try_get_extension("image/png"), Some(".png"));
        assert_eq!(p.extension_or_bin("application/x-custom"), ".bin");
        p.register("application/x-custom", "xyz");
        assert_eq!(p.try_get_extension("application/x-custom"), Some(".xyz"));
    }
}
