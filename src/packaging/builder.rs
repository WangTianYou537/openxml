//! Fluent package builders (practical subset of C# `DocumentFormat.OpenXml.Builder`).
//!
//! C# exposes an experimental middleware pipeline (`IPackageBuilder` / `Use`). This
//! module provides a Rust-idiomatic fluent builder for creating Word / Excel / PPT
//! packages with settings, optional main content, and middleware hooks.

use crate::element::OpenXmlElement;
use crate::error::Result;
use crate::packaging::{
    OpenSettings, PresentationDocument, PresentationDocumentType, SpreadsheetDocument,
    SpreadsheetDocumentType, WordprocessingDocument, WordprocessingDocumentType,
};
use crate::wordprocessing::{body, document, paragraph, run, text};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Middleware invoked after a package is constructed (C# `PackageDelegate` shell).
pub type PackageMiddleware<D> = Box<dyn FnOnce(&mut D) -> Result<()> + Send>;

/// Fluent builder for [`WordprocessingDocument`] (C# Word package builder subset).
pub struct WordprocessingDocumentBuilder {
    path: Option<PathBuf>,
    document_type: WordprocessingDocumentType,
    settings: OpenSettings,
    paragraphs: Vec<String>,
    root: Option<OpenXmlElement>,
    properties: HashMap<String, String>,
    middleware: Vec<PackageMiddleware<WordprocessingDocument>>,
}

impl Default for WordprocessingDocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WordprocessingDocumentBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            document_type: WordprocessingDocumentType::Document,
            settings: OpenSettings::default(),
            paragraphs: Vec::new(),
            root: None,
            properties: HashMap::new(),
            middleware: Vec::new(),
        }
    }

    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn document_type(mut self, t: WordprocessingDocumentType) -> Self {
        self.document_type = t;
        self
    }

    pub fn settings(mut self, settings: OpenSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn auto_save(mut self, auto_save: bool) -> Self {
        self.settings.auto_save = auto_save;
        self
    }

    pub fn max_characters_in_part(mut self, limit: u64) -> Self {
        self.settings.max_characters_in_part = limit;
        self
    }

    /// Queue a plain-text paragraph to place in the main document body.
    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.paragraphs.push(text.into());
        self
    }

    /// Replace the main document root entirely (overrides [`paragraph`](Self::paragraph) text).
    pub fn document_root(mut self, root: OpenXmlElement) -> Self {
        self.root = Some(root);
        self
    }

    /// Arbitrary builder property bag (C# `IPackageBuilder.Properties` shell).
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// Register post-create middleware (runs in registration order).
    pub fn use_middleware<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut WordprocessingDocument) -> Result<()> + Send + 'static,
    {
        self.middleware.push(Box::new(f));
        self
    }

    /// Build an in-memory or path-backed document and apply content + middleware.
    pub fn build(self) -> Result<WordprocessingDocument> {
        let mut doc = if let Some(path) = &self.path {
            WordprocessingDocument::create_with_settings(
                path,
                self.document_type,
                self.settings.clone(),
            )?
        } else {
            let mut d = WordprocessingDocument::create_in_memory(self.document_type)?;
            *d.settings_mut() = self.settings.clone();
            d
        };

        let root = if let Some(root) = self.root {
            root
        } else {
            let paras: Vec<_> = self
                .paragraphs
                .iter()
                .map(|t| paragraph(vec![run(vec![text(t.as_str())])]))
                .collect();
            document(vec![body(paras)])
        };
        doc.add_main_document_part().set_document(root);

        for mw in self.middleware {
            mw(&mut doc)?;
        }
        Ok(doc)
    }

    /// Build and save to the configured path (errors if no path was set).
    pub fn build_and_save(self) -> Result<WordprocessingDocument> {
        if self.path.is_none() {
            return Err(crate::error::Error::Package(
                "WordprocessingDocumentBuilder::build_and_save requires path()".into(),
            ));
        }
        let mut doc = self.build()?;
        doc.save()?;
        Ok(doc)
    }
}

/// Fluent builder for [`SpreadsheetDocument`].
pub struct SpreadsheetDocumentBuilder {
    path: Option<PathBuf>,
    document_type: SpreadsheetDocumentType,
    settings: OpenSettings,
    sheet_name: String,
    middleware: Vec<PackageMiddleware<SpreadsheetDocument>>,
}

impl Default for SpreadsheetDocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SpreadsheetDocumentBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            document_type: SpreadsheetDocumentType::Workbook,
            settings: OpenSettings::default(),
            sheet_name: "Sheet1".into(),
            middleware: Vec::new(),
        }
    }

    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn document_type(mut self, t: SpreadsheetDocumentType) -> Self {
        self.document_type = t;
        self
    }

    pub fn settings(mut self, settings: OpenSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn sheet_name(mut self, name: impl Into<String>) -> Self {
        self.sheet_name = name.into();
        self
    }

    pub fn use_middleware<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut SpreadsheetDocument) -> Result<()> + Send + 'static,
    {
        self.middleware.push(Box::new(f));
        self
    }

    pub fn build(self) -> Result<SpreadsheetDocument> {
        let mut doc = if let Some(path) = &self.path {
            SpreadsheetDocument::create_with_settings(path, self.document_type, self.settings)?
        } else {
            let mut d = SpreadsheetDocument::create_in_memory(self.document_type)?;
            *d.settings_mut() = self.settings;
            d
        };
        doc.add_worksheet(&self.sheet_name)?;
        for mw in self.middleware {
            mw(&mut doc)?;
        }
        Ok(doc)
    }
}

/// Fluent builder for [`PresentationDocument`].
pub struct PresentationDocumentBuilder {
    path: Option<PathBuf>,
    document_type: PresentationDocumentType,
    settings: OpenSettings,
    slide_texts: Vec<String>,
    middleware: Vec<PackageMiddleware<PresentationDocument>>,
}

impl Default for PresentationDocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PresentationDocumentBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            document_type: PresentationDocumentType::Presentation,
            settings: OpenSettings::default(),
            slide_texts: Vec::new(),
            middleware: Vec::new(),
        }
    }

    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn document_type(mut self, t: PresentationDocumentType) -> Self {
        self.document_type = t;
        self
    }

    pub fn settings(mut self, settings: OpenSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn slide_text(mut self, text: impl Into<String>) -> Self {
        self.slide_texts.push(text.into());
        self
    }

    pub fn use_middleware<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PresentationDocument) -> Result<()> + Send + 'static,
    {
        self.middleware.push(Box::new(f));
        self
    }

    pub fn build(self) -> Result<PresentationDocument> {
        let mut doc = if let Some(path) = &self.path {
            PresentationDocument::create_with_settings(path, self.document_type, self.settings)?
        } else {
            let mut d = PresentationDocument::create_in_memory(self.document_type)?;
            *d.settings_mut() = self.settings;
            d
        };
        for t in &self.slide_texts {
            doc.add_slide_with_text(t)?;
        }
        for mw in self.middleware {
            mw(&mut doc)?;
        }
        Ok(doc)
    }
}

/// Entry points mirroring C# `*.Create().Build()` style.
pub fn word() -> WordprocessingDocumentBuilder {
    WordprocessingDocumentBuilder::new()
}

pub fn spreadsheet() -> SpreadsheetDocumentBuilder {
    SpreadsheetDocumentBuilder::new()
}

pub fn presentation() -> PresentationDocumentBuilder {
    PresentationDocumentBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_builder_in_memory_paragraphs() {
        let mut doc = word()
            .paragraph("Hello")
            .paragraph("World")
            .auto_save(false)
            .build()
            .unwrap();
        let texts = doc.paragraph_texts().unwrap();
        assert_eq!(texts, vec!["Hello".to_string(), "World".to_string()]);
    }

    #[test]
    fn word_builder_middleware_runs() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        let ran = Arc::new(AtomicBool::new(false));
        let flag = ran.clone();
        let doc = word()
            .paragraph("x")
            .use_middleware(move |_d| {
                flag.store(true, Ordering::SeqCst);
                Ok(())
            })
            .build()
            .unwrap();
        assert!(ran.load(Ordering::SeqCst));
        assert!(doc.main_document_part().is_some());
    }

    #[test]
    fn spreadsheet_builder_adds_sheet() {
        let doc = spreadsheet()
            .sheet_name("Data")
            .build()
            .unwrap();
        assert_eq!(doc.sheet_names(), vec!["Data"]);
    }

    #[test]
    fn presentation_builder_slide_text() {
        let doc = presentation()
            .slide_text("Title A")
            .slide_text("Title B")
            .build()
            .unwrap();
        assert_eq!(doc.slide_count(), 2);
        assert!(doc
            .first_slide_texts()
            .unwrap()
            .iter()
            .any(|t| t.contains("Title A")));
    }

    #[test]
    fn word_clone_to_path_and_bytes() {
        let dir = std::env::temp_dir().join(format!("officexml-clone-{}.docx", std::process::id()));
        let mut doc = word().paragraph("clone-me").auto_save(false).build().unwrap();
        let bytes = doc.clone_to_bytes().unwrap();
        assert!(!bytes.is_empty());
        let mut cloned = doc.clone_to_path(&dir).unwrap();
        assert_eq!(cloned.paragraph_texts().unwrap(), vec!["clone-me".to_string()]);
        let _ = std::fs::remove_file(&dir);
    }
}

